use crate::sys::locks::Mutex;
use crate::time::Duration;

pub struct Condvar {}

pub type MovableCondvar = Condvar;

impl Condvar {
    pub const fn new() -> Condvar {
        Condvar {}
    }

    // No-ops can be safe, it's fine.
    #[inline]
    pub fn notify_one(&self) {}

    // No-ops can be safe, it's fine.
    #[inline]
    pub fn notify_all(&self) {}

    pub unsafe fn wait(&self, _mutex: &Mutex) {
        panic!("condvar wait not supported")
    }

    pub unsafe fn wait_timeout(&self, _mutex: &Mutex, _dur: Duration) -> bool {
        panic!("condvar wait not supported");
    }
}
