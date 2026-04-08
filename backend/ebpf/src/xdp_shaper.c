#include <uapi/linux/bpf.h>
#include <uapi/linux/if_ether.h>
#include <uapi/linux/ip.h>
#include <uapi/linux/tcp.h>
#include <uapi/linux/udp.h>
#include <bpf/bpf_helpers.h>

/* Packet classification */
#define PKT_CLASS_VOIP    1
#define PKT_CLASS_VIDEO   2
#define PKT_CLASS_BULK    3
#define PKT_CLASS_OTHER   0

/* Token bucket state */
struct token_bucket {
    __u64 last_update;
    __u64 tokens;
    __u64 bucket_size;
    __u64 rate_bytes_per_sec;
};

/* Flow key: 5-tuple (src_ip, dst_ip, src_port, dst_port, protocol) */
struct flow_key {
    __u32 src_ip;
    __u32 dst_ip;
    __u16 src_port;
    __u16 dst_port;
    __u8 protocol;
    __u8 _pad[3];
} __attribute__((packed));

/* BPF Map: Flow to token bucket mapping */
BPF_HASH(flow_buckets, struct flow_key, struct token_bucket, 10000);

/* BPF Map: Per-protocol statistics */
BPF_ARRAY(pkt_stats, __u64, 4);  // Index by packet class

/* BPF Map: Dropped packets counter */
BPF_ARRAY(dropped_packets, __u64, 4);

/* Classify packet based on port and protocol */
static __always_inline __u8 classify_packet(__u16 dst_port, __u8 protocol) {
    /* VOIP: 5060-5061 (SIP), 4500 (IPSec) */
    if ((dst_port >= 5060 && dst_port <= 5061) || dst_port == 4500) {
        return PKT_CLASS_VOIP;
    }
    
    /* Video: 1935 (RTMP), 1194 (OpenVPN), 6881-6999 (BitTorrent) */
    if (dst_port == 1935 || dst_port == 1194 || (dst_port >= 6881 && dst_port <= 6999)) {
        return PKT_CLASS_VIDEO;
    }
    
    /* Bulk: 21 (FTP), 25 (SMTP), 110 (POP3), 143 (IMAP) */
    if (dst_port == 21 || dst_port == 25 || dst_port == 110 || dst_port == 143) {
        return PKT_CLASS_BULK;
    }
    
    return PKT_CLASS_OTHER;
}

/* Calculate rate limit based on packet class */
static __always_inline __u64 get_rate_limit(__u8 pkt_class) {
    switch (pkt_class) {
        case PKT_CLASS_VOIP:
            return 1000000;  /* 1 Mbps for VOIP */
        case PKT_CLASS_VIDEO:
            return 5000000;  /* 5 Mbps for video */
        case PKT_CLASS_BULK:
            return 500000;   /* 500 Kbps for bulk */
        default:
            return 10000000; /* 10 Mbps for other */
    }
}

/* Update token bucket and decide if packet should pass */
static __always_inline int should_pass_packet(struct flow_key *flow, 
                                              __u32 pkt_size, 
                                              __u64 now) {
    struct token_bucket *tb = bpf_map_lookup_elem(&flow_buckets, flow);
    
    if (!tb) {
        /* New flow: initialize bucket */
        struct token_bucket new_bucket = {
            .last_update = now,
            .tokens = get_rate_limit(classify_packet(flow->dst_port, flow->protocol)),
            .bucket_size = 100000,  /* 100KB bucket */
            .rate_bytes_per_sec = get_rate_limit(classify_packet(flow->dst_port, flow->protocol)),
        };
        bpf_map_update_elem(&flow_buckets, flow, &new_bucket, 0);
        tb = bpf_map_lookup_elem(&flow_buckets, flow);
        if (!tb)
            return 1;  /* Pass if we can't store state */
    }
    
    /* Calculate tokens gained since last update */
    __u64 time_delta = now - tb->last_update;
    if (time_delta > 0) {
        /* Add tokens: rate_per_sec * time_delta / 1_000_000_000 */
        __u64 new_tokens = tb->tokens + (tb->rate_bytes_per_sec * time_delta / 1000000000);
        if (new_tokens > tb->bucket_size) {
            new_tokens = tb->bucket_size;
        }
        tb->tokens = new_tokens;
        tb->last_update = now;
    }
    
    /* Check if we have enough tokens for this packet */
    if (tb->tokens >= pkt_size) {
        tb->tokens -= pkt_size;
        return 1;  /* Pass packet */
    }
    
    return 0;  /* Drop packet */
}

XDP_PROG int xdp_shaper(struct xdp_md *ctx) {
    void *data_end = (void *)(long)ctx->data_end;
    void *data = (void *)(long)ctx->data;
    
    /* Parse Ethernet header */
    struct ethhdr *eth = data;
    if ((void *)(eth + 1) > data_end)
        return XDP_PASS;
    
    if (eth->h_proto != __constant_htons(ETH_P_IP))
        return XDP_PASS;  /* Not IPv4, pass */
    
    /* Parse IP header */
    struct iphdr *ip = (void *)(eth + 1);
    if ((void *)(ip + 1) > data_end)
        return XDP_PASS;
    
    /* Create flow key */
    struct flow_key flow = {
        .src_ip = ip->saddr,
        .dst_ip = ip->daddr,
        .protocol = ip->protocol,
    };
    
    /* Extract port information */
    if (ip->protocol == IPPROTO_TCP) {
        struct tcphdr *tcp = (void *)(ip + 1);
        if ((void *)(tcp + 1) > data_end)
            return XDP_PASS;
        flow.src_port = tcp->source;
        flow.dst_port = tcp->dest;
    } else if (ip->protocol == IPPROTO_UDP) {
        struct udphdr *udp = (void *)(ip + 1);
        if ((void *)(udp + 1) > data_end)
            return XDP_PASS;
        flow.src_port = udp->source;
        flow.dst_port = udp->dest;
    } else {
        return XDP_PASS;  /* Not TCP/UDP, pass */
    }
    
    /* Get current timestamp */
    __u64 now = bpf_ktime_get_ns();
    
    /* Get packet size */
    __u32 pkt_size = (__u32)(data_end - data);
    
    /* Check token bucket */
    if (!should_pass_packet(&flow, pkt_size, now)) {
        /* Packet dropped by rate limiter */
        __u8 pkt_class = classify_packet(flow.dst_port, flow.protocol);
        __u64 *drop_counter = bpf_map_lookup_elem(&dropped_packets, &pkt_class);
        if (drop_counter) {
            __sync_fetch_and_add(drop_counter, 1);
        }
        return XDP_DROP;
    }
    
    /* Update statistics */
    __u8 pkt_class = classify_packet(flow.dst_port, flow.protocol);
    __u64 *stat_counter = bpf_map_lookup_elem(&pkt_stats, &pkt_class);
    if (stat_counter) {
        __sync_fetch_and_add(stat_counter, 1);
    }
    
    return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
