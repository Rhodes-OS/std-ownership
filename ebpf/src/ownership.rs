use redbpf::load::Loader;
use probes::ownership::BorrowResult;
use futures::stream::StreamExt;
use std::ptr;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut loaded = Loader::load(probe_code()).expect("error on Loader::load");

    let probe = loaded
        .kprobe_mut("do_borrow")
        .expect("error on Loaded::kprobe_mut");

    probe
        .attach_kprobe("sys_exit", 0)
        .attach_kprobe("__x64_sys_exit", 0)
        .expect("error on KProbe::attach_kprobe");

    while let Some((map_name, events)) = loaded.events.next().await {
        if map_name == "BORROW_RESULT" {
            for event in events {
                unsafe {
                    let borrow_result = ptr::read(event.as_ptr() as *const BorrowResult);
                    println!("Borrow result: {}, {}", borroww_result.applier, borrow_result.state);
                };
            }
        }
    }
}

fn probe_code() -> &'static [u8] {
    include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/target/bpf/programs/ownership/ownership.elf"
    ))
}