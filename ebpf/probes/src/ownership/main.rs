#![no_std]
#![no_main]

use redbpf_probes::kprobe::prelude::*;

program!(0xFFFFFFFE, "GPL");

#[kprobe]
fn do_borrow(regs: Registers) {
    
}