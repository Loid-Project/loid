use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn next_id() -> u64 {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}
