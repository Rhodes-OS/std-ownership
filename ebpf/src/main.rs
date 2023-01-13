use redbpf::load::Loader;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut loaded = Loader::load(probe_code()).expect("error on Loader::load");

    let probe = loaded
        .kprobe_mut("do_borrow")
        .expect("error on Loaded::kprobe_mut");

    probe
        .attach_kprobe("do_sys_open", 0)
        .expect("error on KProbe::attach_kprobe");
}

fn probe_code() -> &'static [u8] {
    include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/target/bpf/programs/ownership/ownership.elf"
    ))
}