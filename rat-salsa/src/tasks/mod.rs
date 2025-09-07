//!
//! Types used for both future tasks and thread tasks.
//!
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Liveness check for background tasks.
/// Used by both the thread-pool and by the tokio async runner.
#[derive(Debug, Default, Clone)]
pub struct Liveness(Arc<AtomicBool>);

impl Liveness {
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }

    pub fn is_alive(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }

    pub fn born(&self) {
        self.0.store(true, Ordering::Release);
    }

    pub fn dead(&self) {
        self.0.store(false, Ordering::Release);
    }
}

/// Cancel background tasks.
#[derive(Debug, Default, Clone)]
pub struct Cancel(Arc<AtomicBool>);

impl Cancel {
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }

    pub fn is_canceled(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }

    pub fn cancel(&self) {
        self.0.store(true, Ordering::Release);
    }
}
