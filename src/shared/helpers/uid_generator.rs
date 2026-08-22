use std::sync::atomic::{AtomicU64, Ordering};

static counter: AtomicU64 = AtomicU64::new(1);

pub fn NextID() -> u64 {
    counter.fetch_add(1, Ordering::SeqCst)
}
