// NetCtl eBPF/XDP packet filtering program
// Kernel-level QoS and packet rate limiting

#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::xdp_action,
    macros::xdp,
    programs::XdpContext,
};
use aya_log_ebpf::info;

#[xdp]
pub fn netctl_xdp(ctx: XdpContext) -> u32 {
    match try_netctl_xdp(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_DROP as u32,
    }
}

fn try_netctl_xdp(_ctx: XdpContext) -> Result<u32, u32> {
    info!(&_ctx, "NetCtl XDP program loaded");
    Ok(xdp_action::XDP_PASS as u32)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
