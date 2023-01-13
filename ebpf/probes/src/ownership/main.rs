#![no_std]
#![no_main]

use probes::ownership::*;
use redbpf_probes::kprobe::prelude::*;
use std_ownership::rcm::manager::Manager;

program!(0xFFFFFFFE, "GPL");

#[map]
static mut BORROW_RESULT: PerfMap<BorrowResult> = PerfMap::with_max_entries(1024);

#[kprobe]
fn do_borrow(regs: Registers) {
    let manager = Manager::new();
    unsafe {
        let applier = regs.parm1();
        let resource_id = regs.parm2() as *const u8;
    }
    let borrow_state = manager::borrow(applier, resource_id);
    let borrow_result = BorrowResult::new(applier, borrow_state);
    unsafe {
        if bpf_probe_read_user_str(
            borrow_result.applier.as_mut_ptr() as *mut _,
            borrow_result.state,
            applier as *const _,
        ) <= 0
        {
            bpf_trace_printk(b"error on bpf_probe_read_user_str\0");
            return;
        }
        BORROW_RESULT.insert(regs.ctx, &borrow_result);
    }
}