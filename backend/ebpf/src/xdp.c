#include <uapi/linux/bpf.h>
#include <uapi/linux/if_ether.h>
#include <uapi/linux/ip.h>
#include <uapi/linux/ipv6.h>
#include <bpf/bpf_helpers.h>

#define SEC(name) __attribute__((section(name), used))

#define MAX_TOKENS 1000000ULL
#define NANOSEC_PER_SEC 1000000000ULL

struct mac_key {
    __u8 addr[6];
};

struct rate_state {
    __u64 tokens;
    __u64 last_ts;
};

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 10000);
    __type(key, struct mac_key);
    __type(value, __u32);
} qos_rules SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 10000);
    __type(key, struct mac_key);
    __type(value, struct rate_state);
} rate_limits SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_HASH);
    __uint(max_entries, 10000);
    __type(key, struct mac_key);
    __type(value, __u64);
} packet_counters SEC(".maps");

struct stats {
    __u64 passed;
    __u64 dropped;
};

struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, struct stats);
} global_stats SEC(".maps");

static __always_inline void update_stats(int passed) {
    __u32 key = 0;
    struct stats *s = bpf_map_lookup_elem(&global_stats, &key);
    if (!s)
        return;
    
    if (passed)
        s->passed++;
    else
        s->dropped++;
}

SEC("xdp")
int xdp_filter(struct xdp_md *ctx) {
    void *data_end = (void *)(long)ctx->data_end;
    void *data = (void *)(long)ctx->data;
    
    struct ethhdr *eth = data;
    if ((void *)(eth + 1) > data_end) {
        update_stats(0);
        return XDP_DROP;
    }
    
    __u16 h_proto = eth->h_proto;
    
    /* Allow only IPv4 and IPv6 */
    if (h_proto != __constant_htons(ETH_P_IP) &&
        h_proto != __constant_htons(ETH_P_IPV6)) {
        update_stats(1);
        return XDP_PASS;
    }
    
    /* Minimal L3 parsing (verifier-safe) */
    if (h_proto == __constant_htons(ETH_P_IP)) {
        struct iphdr *ip = (void *)(eth + 1);
        if ((void *)(ip + 1) > data_end) {
            update_stats(0);
            return XDP_DROP;
        }
    } else {
        struct ipv6hdr *ip6 = (void *)(eth + 1);
        if ((void *)(ip6 + 1) > data_end) {
            update_stats(0);
            return XDP_DROP;
        }
    }
    
    struct mac_key key = {};
    __builtin_memcpy(key.addr, eth->h_dest, 6);
    
    __u32 *rate_kbps = bpf_map_lookup_elem(&qos_rules, &key);
    
    if (rate_kbps) {
        if (*rate_kbps == 0) {
            update_stats(0);
            return XDP_DROP;
        }
        
        __u64 now = bpf_ktime_get_ns();
        
        struct rate_state *state = bpf_map_lookup_elem(&rate_limits, &key);
        
        if (!state) {
            struct rate_state new_state = {
                .tokens = MAX_TOKENS,
                .last_ts = now,
            };
            bpf_map_update_elem(&rate_limits, &key, &new_state, BPF_ANY);
            state = bpf_map_lookup_elem(&rate_limits, &key);
            if (!state) {
                update_stats(1);
                return XDP_PASS;
            }
        }
        
        __u64 elapsed = now - state->last_ts;
        
        __u64 rate_bytes_per_sec = (*rate_kbps * 1000) / 8;
        __u64 tokens_to_add = (rate_bytes_per_sec * elapsed) / NANOSEC_PER_SEC;
        
        if (tokens_to_add > 0) {
            state->tokens += tokens_to_add;
            if (state->tokens > MAX_TOKENS)
                state->tokens = MAX_TOKENS;
            state->last_ts = now;
        }
        
        __u64 pkt_size = data_end - data;
        
        if (state->tokens < pkt_size) {
            update_stats(0);
            return XDP_DROP;
        }
        
        state->tokens -= pkt_size;
    }
    
    __u64 *counter = bpf_map_lookup_elem(&packet_counters, &key);
    if (counter) {
        (*counter)++;
    } else {
        __u64 one = 1;
        bpf_map_update_elem(&packet_counters, &key, &one, BPF_ANY);
    }
    
    update_stats(1);
    return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
