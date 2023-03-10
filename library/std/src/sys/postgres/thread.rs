use super::unsupported;
use crate::ffi::CStr;
use crate::io;
use crate::num::NonZeroUsize;
use crate::time::Duration;

// Used from some functions that are never called.
pub const DEFAULT_MIN_STACK_SIZE: usize = 2 * 1024 * 1024;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(_stack: usize, _p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        unsupported()
    }

    pub fn yield_now() {
        // do nothing
    }

    pub fn set_name(_: &CStr) {
        // Do nothing
    }

    pub fn sleep(dur: Duration) {
        let mut secs = dur.as_secs();
        let mut nsecs = dur.subsec_nanos() as _;

        // If we're awoken with a signal then the return value will be -1 and
        // nanosleep will fill in `ts` with the remaining time.
        unsafe {
            while secs > 0 || nsecs > 0 {
                let mut ts = libc::timespec {
                    tv_sec: cmp::min(libc::time_t::MAX as u64, secs) as libc::time_t,
                    tv_nsec: nsecs,
                };
                secs -= ts.tv_sec as u64;
                let ts_ptr = &mut ts as *mut _;
                if libc::nanosleep(ts_ptr, ts_ptr) == -1 {
                    // `postgres` does not have `errno()`.
                    assert_eq!(os::real_errno_use_carefully(), libc::EINTR);
                    secs += ts.tv_sec as u64;
                    nsecs = ts.tv_nsec;
                } else {
                    nsecs = 0;
                }
            }
        }
    }

    pub fn join(self) {
        unreachable!();
    }
}

pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    unsupported()
}

use crate::cmp;
use crate::mem;
use crate::ptr;
use crate::sys::os;

#[cfg(all(target_os = "linux", target_env = "gnu"))]
use crate::sys::weak::dlsym;
#[cfg(any(target_os = "solaris", target_os = "illumos"))]
use crate::sys::weak::weak;

// Note: `struct Thread(!)` doesn't work (`std::thread::current()` exists)
pub struct Thread {
    id: libc::pthread_t,
}

// Some platforms may have pthread_t as a pointer in which case we still want
// a thread to be Send/Sync
unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

impl Thread {
    pub fn id(&self) -> libc::pthread_t {
        unimplemented!("unsupported")
    }
    pub fn into_id(self) -> libc::pthread_t {
        unimplemented!("unsupported")
    }
}

#[cfg(target_family = "postgres")]
impl Drop for Thread {
    fn drop(&mut self) {
        unreachable!();
    }
}

pub mod guard {
    use crate::ops::Range;
    pub type Guard = Range<usize>;
    pub unsafe fn current() -> Option<Guard> {
        None
    }
    pub unsafe fn init() -> Option<Guard> {
        None
    }
}
