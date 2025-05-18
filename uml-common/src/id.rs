use std::fmt::Display;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

static COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Id(usize);

impl Default for Id {
    fn default() -> Self {
        Self(COUNTER.fetch_add(1, Relaxed))
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}
