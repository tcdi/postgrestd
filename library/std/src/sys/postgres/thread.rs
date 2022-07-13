use super::unsupported;
use crate::ffi::CStr;
use crate::io;
use crate::num::NonZeroUsize;
use crate::time::Duration;


impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(_stack: usize, _p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        unsupported()
    }

    pub fn yield_now() {
        // do nothing
    }

    pub fn set_name(_name: &CStr) {
        // nope
    }

    pub fn sleep(_dur: Duration) {
        panic!("can't sleep, sql will eat me");
    }

    pub fn join(self) {
    }
}

pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    unsupported()
}

use crate::cmp;
use crate::mem;
use crate::ptr;
use crate::sys::{os, stack_overflow};

#[cfg(all(target_os = "linux", target_env = "gnu"))]
use crate::sys::weak::dlsym;
#[cfg(any(target_os = "solaris", target_os = "illumos"))]
use crate::sys::weak::weak;
#[cfg(not(any(target_os = "l4re", target_os = "vxworks", target_os = "espidf")))]
pub const DEFAULT_MIN_STACK_SIZE: usize = 2 * 1024 * 1024;
#[cfg(target_os = "l4re")]
pub const DEFAULT_MIN_STACK_SIZE: usize = 1024 * 1024;
#[cfg(target_os = "vxworks")]
pub const DEFAULT_MIN_STACK_SIZE: usize = 256 * 1024;
#[cfg(target_os = "espidf")]
pub const DEFAULT_MIN_STACK_SIZE: usize = 0; // 0 indicates that the stack size configured in the ESP-IDF menuconfig system should be used

#[cfg(target_os = "fuchsia")]
mod zircon {
    type zx_handle_t = u32;
    type zx_status_t = i32;
    pub const ZX_PROP_NAME: u32 = 3;

    extern "C" {
        pub fn zx_object_set_property(
            handle: zx_handle_t,
            property: u32,
            value: *const libc::c_void,
            value_size: libc::size_t,
        ) -> zx_status_t;
        pub fn zx_thread_self() -> zx_handle_t;
    }
}

pub struct Thread {
    id: libc::pthread_t,
}

// Some platforms may have pthread_t as a pointer in which case we still want
// a thread to be Send/Sync
unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

impl Thread {
    pub fn id(&self) -> libc::pthread_t {
        self.id
    }

    pub fn into_id(self) -> libc::pthread_t {
        let id = self.id;
        mem::forget(self);
        id
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        let ret = unsafe { libc::pthread_detach(self.id) };
        debug_assert_eq!(ret, 0);
    }
}

#[cfg(all(
    not(target_os = "linux"),
    not(target_os = "postgres"),
    not(target_os = "freebsd"),
    not(target_os = "macos"),
    not(target_os = "netbsd"),
    not(target_os = "openbsd"),
    not(target_os = "solaris")
))]
#[cfg_attr(test, allow(dead_code))]
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

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "solaris",
    target_os = "postgres",
))]
#[cfg_attr(test, allow(dead_code))]
pub mod guard {
    use libc::{mmap, mprotect};
    use libc::{MAP_ANON, MAP_FAILED, MAP_FIXED, MAP_PRIVATE, PROT_NONE, PROT_READ, PROT_WRITE};

    use crate::io;
    use crate::ops::Range;
    use crate::sync::atomic::{AtomicUsize, Ordering};
    use crate::sys::os;

    // This is initialized in init() and only read from after
    static PAGE_SIZE: AtomicUsize = AtomicUsize::new(0);

    pub type Guard = Range<usize>;

    #[cfg(target_os = "solaris")]
    unsafe fn get_stack_start() -> Option<*mut libc::c_void> {
        let mut current_stack: libc::stack_t = crate::mem::zeroed();
        assert_eq!(libc::stack_getbounds(&mut current_stack), 0);
        Some(current_stack.ss_sp)
    }

    #[cfg(target_os = "macos")]
    unsafe fn get_stack_start() -> Option<*mut libc::c_void> {
        let th = libc::pthread_self();
        let stackptr = libc::pthread_get_stackaddr_np(th);
        Some(stackptr.map_addr(|addr| addr - libc::pthread_get_stacksize_np(th)))
    }

    #[cfg(target_os = "openbsd")]
    unsafe fn get_stack_start() -> Option<*mut libc::c_void> {
        let mut current_stack: libc::stack_t = crate::mem::zeroed();
        assert_eq!(libc::pthread_stackseg_np(libc::pthread_self(), &mut current_stack), 0);

        let stack_ptr = current_stack.ss_sp;
        let stackaddr = if libc::pthread_main_np() == 1 {
            // main thread
            stack_ptr.addr() - current_stack.ss_size + PAGE_SIZE.load(Ordering::Relaxed)
        } else {
            // new thread
            stack_ptr.addr() - current_stack.ss_size
        };
        Some(stack_ptr.with_addr(stackaddr))
    }

    #[cfg(any(
        target_os = "android",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "postgres",
        target_os = "netbsd",
        target_os = "l4re"
    ))]
    unsafe fn get_stack_start() -> Option<*mut libc::c_void> {
        let mut ret = None;
        let mut attr: libc::pthread_attr_t = crate::mem::zeroed();
        #[cfg(target_os = "freebsd")]
        assert_eq!(libc::pthread_attr_init(&mut attr), 0);
        #[cfg(target_os = "freebsd")]
        let e = libc::pthread_attr_get_np(libc::pthread_self(), &mut attr);
        #[cfg(not(target_os = "freebsd"))]
        let e = libc::pthread_getattr_np(libc::pthread_self(), &mut attr);
        if e == 0 {
            let mut stackaddr = crate::ptr::null_mut();
            let mut stacksize = 0;
            assert_eq!(libc::pthread_attr_getstack(&attr, &mut stackaddr, &mut stacksize), 0);
            ret = Some(stackaddr);
        }
        if e == 0 || cfg!(target_os = "freebsd") {
            assert_eq!(libc::pthread_attr_destroy(&mut attr), 0);
        }
        ret
    }

    // Precondition: PAGE_SIZE is initialized.
    unsafe fn get_stack_start_aligned() -> Option<*mut libc::c_void> {
        let page_size = PAGE_SIZE.load(Ordering::Relaxed);
        assert!(page_size != 0);
        let stackptr = get_stack_start()?;
        let stackaddr = stackptr.addr();

        // Ensure stackaddr is page aligned! A parent process might
        // have reset RLIMIT_STACK to be non-page aligned. The
        // pthread_attr_getstack() reports the usable stack area
        // stackaddr < stackaddr + stacksize, so if stackaddr is not
        // page-aligned, calculate the fix such that stackaddr <
        // new_page_aligned_stackaddr < stackaddr + stacksize
        let remainder = stackaddr % page_size;
        Some(if remainder == 0 {
            stackptr
        } else {
            stackptr.with_addr(stackaddr + page_size - remainder)
        })
    }

    pub unsafe fn init() -> Option<Guard> {
        let page_size = os::page_size();
        PAGE_SIZE.store(page_size, Ordering::Relaxed);

        if cfg!(any(
            all(target_os = "linux", not(target_env = "musl")),
            target_os = "postgres")) {
            // Linux doesn't allocate the whole stack right away, and
            // the kernel has its own stack-guard mechanism to fault
            // when growing too close to an existing mapping.  If we map
            // our own guard, then the kernel starts enforcing a rather
            // large gap above that, rendering much of the possible
            // stack space useless.  See #43052.
            //
            // Instead, we'll just note where we expect rlimit to start
            // faulting, so our handler can report "stack overflow", and
            // trust that the kernel's own stack guard will work.
            let stackptr = get_stack_start_aligned()?;
            let stackaddr = stackptr.addr();
            Some(stackaddr - page_size..stackaddr)
        } else if cfg!(all(target_os = "linux", target_env = "musl")) {
            // For the main thread, the musl's pthread_attr_getstack
            // returns the current stack size, rather than maximum size
            // it can eventually grow to. It cannot be used to determine
            // the position of kernel's stack guard.
            None
        } else if cfg!(target_os = "freebsd") {
            // FreeBSD's stack autogrows, and optionally includes a guard page
            // at the bottom.  If we try to remap the bottom of the stack
            // ourselves, FreeBSD's guard page moves upwards.  So we'll just use
            // the builtin guard page.
            let stackptr = get_stack_start_aligned()?;
            let guardaddr = stackptr.addr();
            // Technically the number of guard pages is tunable and controlled
            // by the security.bsd.stack_guard_page sysctl, but there are
            // few reasons to change it from the default.  The default value has
            // been 1 ever since FreeBSD 11.1 and 10.4.
            const GUARD_PAGES: usize = 1;
            let guard = guardaddr..guardaddr + GUARD_PAGES * page_size;
            Some(guard)
        } else {
            // Reallocate the last page of the stack.
            // This ensures SIGBUS will be raised on
            // stack overflow.
            // Systems which enforce strict PAX MPROTECT do not allow
            // to mprotect() a mapping with less restrictive permissions
            // than the initial mmap() used, so we mmap() here with
            // read/write permissions and only then mprotect() it to
            // no permissions at all. See issue #50313.
            let stackptr = get_stack_start_aligned()?;
            let result = mmap(
                stackptr,
                page_size,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANON | MAP_FIXED,
                -1,
                0,
            );
            if result != stackptr || result == MAP_FAILED {
                panic!("failed to allocate a guard page: {}", io::Error::last_os_error());
            }

            let result = mprotect(stackptr, page_size, PROT_NONE);
            if result != 0 {
                panic!("failed to protect the guard page: {}", io::Error::last_os_error());
            }

            let guardaddr = stackptr.addr();

            Some(guardaddr..guardaddr + page_size)
        }
    }

    #[cfg(any(target_os = "macos", target_os = "openbsd", target_os = "solaris"))]
    pub unsafe fn current() -> Option<Guard> {
        let stackptr = get_stack_start()?;
        let stackaddr = stackptr.addr();
        Some(stackaddr - PAGE_SIZE.load(Ordering::Relaxed)..stackaddr)
    }

    #[cfg(any(
        target_os = "android",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "postgres",
        target_os = "netbsd",
        target_os = "l4re"
    ))]
    pub unsafe fn current() -> Option<Guard> {
        let mut ret = None;
        let mut attr: libc::pthread_attr_t = crate::mem::zeroed();
        #[cfg(target_os = "freebsd")]
        assert_eq!(libc::pthread_attr_init(&mut attr), 0);
        #[cfg(target_os = "freebsd")]
        let e = libc::pthread_attr_get_np(libc::pthread_self(), &mut attr);
        #[cfg(not(target_os = "freebsd"))]
        let e = libc::pthread_getattr_np(libc::pthread_self(), &mut attr);
        if e == 0 {
            let mut guardsize = 0;
            assert_eq!(libc::pthread_attr_getguardsize(&attr, &mut guardsize), 0);
            if guardsize == 0 {
                if cfg!(all(target_os = "linux", target_env = "musl")) {
                    // musl versions before 1.1.19 always reported guard
                    // size obtained from pthread_attr_get_np as zero.
                    // Use page size as a fallback.
                    guardsize = PAGE_SIZE.load(Ordering::Relaxed);
                } else {
                    panic!("there is no guard page");
                }
            }
            let mut stackptr = crate::ptr::null_mut::<libc::c_void>();
            let mut size = 0;
            assert_eq!(libc::pthread_attr_getstack(&attr, &mut stackptr, &mut size), 0);

            let stackaddr = stackptr.addr();
            ret = if cfg!(any(target_os = "freebsd", target_os = "netbsd")) {
                Some(stackaddr - guardsize..stackaddr)
            } else if cfg!(all(target_os = "linux", target_env = "musl")) {
                Some(stackaddr - guardsize..stackaddr)
            } else if cfg!(any(
                all(target_os = "linux", any(target_env = "gnu", target_env = "uclibc")
            ),
            target_os = "postgres",
        ))
            {
                // glibc used to include the guard area within the stack, as noted in the BUGS
                // section of `man pthread_attr_getguardsize`.  This has been corrected starting
                // with glibc 2.27, and in some distro backports, so the guard is now placed at the
                // end (below) the stack.  There's no easy way for us to know which we have at
                // runtime, so we'll just match any fault in the range right above or below the
                // stack base to call that fault a stack overflow.
                Some(stackaddr - guardsize..stackaddr + guardsize)
            } else {
                Some(stackaddr..stackaddr + guardsize)
            };
        }
        if e == 0 || cfg!(target_os = "freebsd") {
            assert_eq!(libc::pthread_attr_destroy(&mut attr), 0);
        }
        ret
    }
}

// glibc >= 2.15 has a __pthread_get_minstack() function that returns
// PTHREAD_STACK_MIN plus bytes needed for thread-local storage.
// We need that information to avoid blowing up when a small stack
// is created in an application with big thread-local storage requirements.
// See #6233 for rationale and details.
#[cfg(any(
    all(target_os = "linux", target_env = "gnu"),
    target_os = "postgres",
))]
fn min_stack_size(attr: *const libc::pthread_attr_t) -> usize {
    // We use dlsym to avoid an ELF version dependency on GLIBC_PRIVATE. (#23628)
    // We shouldn't really be using such an internal symbol, but there's currently
    // no other way to account for the TLS size.
    dlsym!(fn __pthread_get_minstack(*const libc::pthread_attr_t) -> libc::size_t);

    match __pthread_get_minstack.get() {
        None => libc::PTHREAD_STACK_MIN,
        Some(f) => unsafe { f(attr) },
    }
}

// No point in looking up __pthread_get_minstack() on non-glibc platforms.
#[cfg(all(not(all(target_os = "linux", target_env = "gnu")), 
    not(target_os = "postgres"),
not(target_os = "netbsd")))]
fn min_stack_size(_: *const libc::pthread_attr_t) -> usize {
    libc::PTHREAD_STACK_MIN
}

#[cfg(target_os = "netbsd")]
fn min_stack_size(_: *const libc::pthread_attr_t) -> usize {
    2048 // just a guess
}
