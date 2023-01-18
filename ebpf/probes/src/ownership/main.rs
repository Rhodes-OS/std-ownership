#![no_std]
#![no_main]

use probes::ownership::*;
use redbpf_probes::kprobe::prelude::*;
use std_ownership::rcm::center::ResourceCenter;

program!(0xFFFFFFFE, "GPL");

#[map(link_section = "maps/borrow_result")]
static mut BORROW_RESULT: PerfMap<BorrowResult> = PerfMap::with_max_entries(1024);

#[kprobe]
fn do_borrow(regs: Registers) {
    let rc = ResourceCenter::build();
    unsafe {
        let applier = regs.parm1();
        let resource_id = regs.parm2() as *const u8;
        let data = regs.parm3() as *const &[u8];
    }
    let borrow_state = rc::borrow(applier, resource_id, data);
    let borrow_result = BorrowResult::new(applier, borrow_state);
    unsafe {
        BORROW_RESULT.insert(regs.ctx, &borrow_result);
    }
}