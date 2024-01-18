use std::sync::Arc;

use libosdp::channel::Channel;
use ringbuf::HeapRb;
pub mod device;
pub mod threadbus;

type Result<T> = core::result::Result<T, libosdp::OsdpError>;

pub fn setup() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        .format_timestamp(None)
        .init();
}

pub struct MemoryChannel {
    id: i32,
    sender: ringbuf::Producer<u8, Arc<HeapRb<u8>>>,
    receiver: ringbuf::Consumer<u8, Arc<HeapRb<u8>>>,
}

impl MemoryChannel {
    pub fn new() -> (Self, Self) {
        let rb1 = HeapRb::<u8>::new(1024);
        let (prod1, cons1) = rb1.split();
        let rb2 = HeapRb::<u8>::new(1024);
        let (prod2, cons2) = rb2.split();
        (
            Self {
                id: 0,
                sender: prod1,
                receiver: cons2,
            },
            Self {
                id: 1,
                sender: prod2,
                receiver: cons1,
            },
        )
    }
}

impl libosdp::channel::Write for MemoryChannel {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.sender.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
impl libosdp::channel::Read for MemoryChannel {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.receiver.read(buf)
    }
}

impl Channel for MemoryChannel {
    fn get_id(&self) -> i32 {
        self.id
    }
}
