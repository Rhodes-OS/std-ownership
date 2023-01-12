use std_ownership::rcm::checker::buffer_checker::BufferChecker;
use std_ownership::resource::buffer::Buffer;
use std_ownership::api::Resource;

#[test]
fn init_buffer_checker() {
    let buffer_checker = BufferChecker::new(Buffer::new(8));
    assert_eq!(buffer_checker.buffer().size(), 8);
}