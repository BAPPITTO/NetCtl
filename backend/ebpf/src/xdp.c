#include <uapi/linux/bpf.h>
#include <uapi/linux/if_ether.h>
#include <uapi/linux/ip.h>
#include <uapi/linux/in.h>
#include <uapi/linux/udp.h>
#include <bpf/bpf_helpers.h>

#define SEC(name) __attribute__((section(name), used))

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 10000);
    __type(key, __u32);    /* Last 4 bytes of destination MAC */
    __type(value, __u32);  /* Rate limit in Kbps */
} qos_rules SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 10000);
    __type(key, __u32);      /* MAC tail */
    __type(value, __u64);    /* Packet count */
} packet_counters SEC(".maps");

SEC("xdp")
int xdp_filter(struct xdp_md *ctx) {
    void *data_end = (void *)(long)ctx->data_end;
    void *data = (void *)(long)ctx->data;

    /* Parse Ethernet header */
    struct ethhdr *eth = data;
    if ((void *)(eth + 1) > data_end)
        return XDP_DROP;

    /* Extract destination MAC (last 4 bytes for lookup) */
    __u32 mac_tail = eth->h_dest[2] << 24 | eth->h_dest[3] << 16 | 
                     eth->h_dest[4] << 8 | eth->h_dest[5];

    /* Check if QoS rule exists for this MAC */
    __u32 *rate_limit = bpf_map_lookup_elem(&qos_rules, &mac_tail);
    if (rate_limit) {
        /* If rate is 0, drop packet (blocking) */
        if (*rate_limit == 0) {
            return XDP_DROP;
        }
        
        /* TODO: Implement token bucket algorithm for rate limiting */
        /* For now, pass through at any configured rate */
    }

    /* Update packet counter */
    __u64 *counter = bpf_map_lookup_elem(&packet_counters, &mac_tail);
    if (counter) {
        __sync_fetch_and_add(counter, 1);
    } else {
        __u64 one = 1;
        bpf_map_update_elem(&packet_counters, &mac_tail, &one, 0);
    }

    return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
