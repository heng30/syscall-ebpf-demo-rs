#![no_std]
#![no_main]
#![allow(warnings)]
#![allow(static_mut_refs)]

use aya_ebpf::macros::map;
use aya_ebpf::maps::HashMap;
use aya_ebpf::{helpers::bpf_get_current_pid_tgid, macros::kprobe, programs::ProbeContext};
use aya_log_ebpf::info;

#[map]
static mut SYSCALL_COUNTS: HashMap<u32, u64> = HashMap::<u32, u64>::with_max_entries(10, 0);

fn increment_syscall_count(ctx: &ProbeContext, syscall_id: u32) {
    unsafe {
        let count = SYSCALL_COUNTS.get_ptr_mut(&syscall_id);
        if let Some(count) = count {
            *count += 1;
        } else {
            SYSCALL_COUNTS
                .insert(&syscall_id, &1, 0)
                .unwrap_or_else(|_| ());
        }

        // FIXME: runtime error, cause by `info!`
        // info!(
        //     ctx,
        //     "Syscall {} called by PID {}",
        //     syscall_id,
        //     bpf_get_current_pid_tgid() >> 32
        // );
    }
}

#[kprobe]
pub fn read_counter(ctx: ProbeContext) -> u32 {
    increment_syscall_count(&ctx, 0);
    0
}

#[kprobe]
pub fn write_counter(ctx: ProbeContext) -> u32 {
    increment_syscall_count(&ctx, 1);
    0
}

#[kprobe]
pub fn open_counter(ctx: ProbeContext) -> u32 {
    increment_syscall_count(&ctx, 2);
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
