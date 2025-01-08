use linkme::distributed_slice;

#[distributed_slice]
pub static BOOT: [fn()];
