use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(1);

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Id(usize);

impl Default for Id {
    fn default() -> Self {
        Self(GLOBAL_THREAD_COUNT.fetch_add(1, Relaxed))
    }
}
