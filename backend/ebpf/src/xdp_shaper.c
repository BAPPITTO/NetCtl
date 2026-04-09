#include <uapi/linux/bpf.h>
#include <uapi/linux/if_ether.h>
#include <uapi/linux/ip.h>
#include <uapi/linux/ipv6.h>
#include <uapi/linux/tcp.h>
#include <uapi/linux/udp.h>
#include <bpf/bpf_helpers.h>

#define SEC(name) __attribute__((section(name), used))

#define NANOSEC_PER_SEC 1000000000ULL
#define BURST_SECONDS 1

/* Packet classification */
#define PKT_CLASS_OTHER 0
#define PKT_CLASS_VOIP 1
#define PKT_CLASS_VIDEO 2
#define PKT_CLASS_BULK 3

struct token_bucket {
    __u64 last_update;
    __u64 tokens;
    __u64 bucket_size;
    __u64 rate_bytes_per_sec;
};

struct flow_key {
    __u32 src_ip;
    __u32 dst_ip;
    __u16 src_port;
    __u16 dst_port;
    __u8 protocol;
    __u8 _pad[3];
};

/* LRU flow table */
struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __uint(max_entries, 10000);
    __type(key, struct flow_key);
    __type(value, struct token_bucket);
} flow_buckets SEC(".maps");

/* Per-CPU stats */
struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 4);
    __type(key, __u32);
    __type(value, __u64);
} pkt_stats SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 4);
    __type(key, __u32);
    __type(value, __u64);
} dropped_packets SEC(".maps");

/* Classify packet */
static __always_inline __u8 classify_packet(__u16 dst_port) {
    if ((dst_port >= 5060 && dst_port <= 5061) || dst_port == 4500)
        return PKT_CLASS_VOIP;
    
    if (dst_port == 1935 || dst_port == 1194 ||
        (dst_port >= 6881 && dst_port <= 6999))
        return PKT_CLASS_VIDEO;
    
    if (dst_port == 21 || dst_port == 25 ||
        dst_port == 110 || dst_port == 143)
        return PKT_CLASS_BULK;
    
    return PKT_CLASS_OTHER;
}

/* Rate limits (bytes/sec) */
static __always_inline __u64 get_rate_limit(__u8 cls) {
    switch (cls) {
        case PKT_CLASS_VOIP: return 125000; /* 1 Mbps */
        case PKT_CLASS_VIDEO: return 625000; /* 5 Mbps */
        case PKT_CLASS_BULK: return 62500; /* 500 Kbps */
        default: return 1250000; /* 10 Mbps */
    }
}

static __always_inline int should_pass_packet(struct flow_key *flow,
                                              __u32 pkt_size,
                                              __u64 now) {
    struct token_bucket *tb = bpf_map_lookup_elem(&flow_buckets, flow);
    
    __u8 cls = classify_packet(flow->dst_port);
    __u64 rate = get_rate_limit(cls);
    __u64 bucket_size = rate * BURST_SECONDS;
    
    if (!tb) {
        struct token_bucket new_tb = {
            .last_update = now,
            .tokens = bucket_size,
            .bucket_size = bucket_size,
            .rate_bytes_per_sec = rate,
        };
        bpf_map_update_elem(&flow_buckets, flow, &new_tb, BPF_ANY);
        return 1;
    }
    
    __u64 delta = now - tb->last_update;
    
    if (delta > 0) {
        __u64 added = (tb->rate_bytes_per_sec * delta) / NANOSEC_PER_SEC;
        tb->tokens = tb->tokens + added;
        if (tb->tokens > tb->bucket_size)
            tb->tokens = tb->bucket_size;
        tb->last_update = now;
    }
    
    if (tb->tokens < pkt_size)
        return 0;
    
    tb->tokens -= pkt_size;
    return 1;
}

SEC("xdp")
int xdp_shaper(struct xdp_md *ctx) {
    void *data_end = (void *)(long)ctx->data_end;
    void *data = (void *)(long)ctx->data;
    
    struct ethhdr *eth = data;
    if ((void *)(eth + 1) > data_end)
        return XDP_PASS;
    
    __u16 proto = eth->h_proto;
    
    if (proto != __constant_htons(ETH_P_IP) &&
        proto != __constant_htons(ETH_P_IPV6))
        return XDP_PASS;
    
    struct flow_key flow = {};
    __u8 ip_proto = 0;
    
    if (proto == __constant_htons(ETH_P_IP)) {
        struct iphdr *ip = (void *)(eth + 1);
        if ((void *)(ip + 1) > data_end)
            return XDP_PASS;
        
        flow.src_ip = ip->saddr;
        flow.dst_ip = ip->daddr;
        ip_proto = ip->protocol;
        
    } else {
        struct ipv6hdr *ip6 = (void *)(eth + 1);
        if ((void *)(ip6 + 1) > data_end)
            return XDP_PASS;
        
        ip_proto = ip6->nexthdr;
        /* NOTE: Not storing IPv6 addresses in key (intentional simplification) */
    }
    
    flow.protocol = ip_proto;
    
    if (ip_proto == IPPROTO_TCP) {
        struct tcphdr *tcp = (void *)((void *)eth + sizeof(*eth) + sizeof(struct iphdr));
        if ((void *)(tcp + 1) > data_end)
            return XDP_PASS;
        
        flow.src_port = tcp->source;
        flow.dst_port = tcp->dest;
        
    } else if (ip_proto == IPPROTO_UDP) {
        struct udphdr *udp = (void *)((void *)eth + sizeof(*eth) + sizeof(struct iphdr));
        if ((void *)(udp + 1) > data_end)
            return XDP_PASS;
        
        flow.src_port = udp->source;
        flow.dst_port = udp->dest;
        
    } else {
        return XDP_PASS;
    }
    
    __u64 now = bpf_ktime_get_ns();
    __u32 pkt_size = data_end - data;
    
    __u8 cls = classify_packet(flow.dst_port);
    
    if (!should_pass_packet(&flow, pkt_size, now)) {
        __u64 *d = bpf_map_lookup_elem(&dropped_packets, &cls);
        if (d) (*d)++;
        return XDP_DROP;
    }
    
    __u64 *s = bpf_map_lookup_elem(&pkt_stats, &cls);
    if (s) (*s)++;
    
    return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
