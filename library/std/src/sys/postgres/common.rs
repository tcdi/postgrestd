use super::signal;
use crate::ffi::CStr;
use crate::io as std_io;
use crate::sys::stack_overflow;
use crate::sys::thread;
use std_io::ErrorKind;

pub mod memchr {
    pub use core::slice::memchr::{memchr, memrchr};
}

#[cfg(not(target_os = "espidf"))]
// SAFETY: must be called only once during runtime initialization.
// NOTE: this is not guaranteed to run, for example when Rust code is called externally.
pub unsafe fn init(argc: isize, argv: *const *const u8) {
    // The standard streams might be closed on application startup. To prevent
    // std::io::{stdin, stdout,stderr} objects from using other unrelated file
    // resources opened later, we reopen standards streams when they are closed.
    sanitize_standard_fds();

    // By default, some platforms will send a *signal* when an EPIPE error
    // would otherwise be delivered. This runtime doesn't install a SIGPIPE
    // handler, causing it to kill the program, which isn't exactly what we
    // want!
    //
    // Hence, we set SIGPIPE to ignore when the program starts up in order
    // to prevent this problem.
    reset_sigpipe();

    stack_overflow::init();

    // Normally, `thread::spawn` will call `Thread::set_name` but since this thread
    // already exists, we have to call it ourselves. We only do this on macos
    // because some unix-like operating systems such as Linux share process-id and
    // thread-id for the main thread and so renaming the main thread will rename the
    // process and we only want to enable this on platforms we've tested.
    if cfg!(target_os = "macos") {
        thread::Thread::set_name(&CStr::from_bytes_with_nul_unchecked(b"main\0"));
    }

    unsafe fn sanitize_standard_fds() {
        #[cfg(not(miri))]
        // The standard fds are always available in Miri.
        cfg_if::cfg_if! {
            if #[cfg(not(any(
                target_os = "emscripten",
                target_os = "fuchsia",
                target_os = "vxworks",
                // The poll on Darwin doesn't set POLLNVAL for closed fds.
                target_os = "macos",
                target_os = "ios",
                target_os = "redox",
                target_os = "l4re",
            )))] {
                use crate::sys::os::errno;
                let pfds: &mut [_] = &mut [
                    libc::pollfd { fd: 0, events: 0, revents: 0 },
                    libc::pollfd { fd: 1, events: 0, revents: 0 },
                    libc::pollfd { fd: 2, events: 0, revents: 0 },
                ];
                while libc::poll(pfds.as_mut_ptr(), 3, 0) == -1 {
                    if errno() == libc::EINTR {
                        continue;
                    }
                    libc::abort();
                }
                for pfd in pfds {
                    if pfd.revents & libc::POLLNVAL == 0 {
                        continue;
                    }
                    if libc::open("/dev/null\0".as_ptr().cast(), libc::O_RDWR, 0) == -1 {
                        // If the stream is closed but we failed to reopen it, abort the
                        // process. Otherwise we wouldn't preserve the safety of
                        // operations on the corresponding Rust object Stdin, Stdout, or
                        // Stderr.
                        libc::abort();
                    }
                }
            } else if #[cfg(any(target_os = "macos", target_os = "ios", target_os = "redox"))] {
                use crate::sys::os::errno;
                for fd in 0..3 {
                    if libc::fcntl(fd, libc::F_GETFD) == -1 && errno() == libc::EBADF {
                        if libc::open("/dev/null\0".as_ptr().cast(), libc::O_RDWR, 0) == -1 {
                            libc::abort();
                        }
                    }
                }
            }
        }
    }

    unsafe fn reset_sigpipe() {
        #[cfg(not(any(target_os = "emscripten", target_os = "fuchsia")))]
        rtassert!(signal(libc::SIGPIPE, libc::SIG_IGN) != libc::SIG_ERR);
    }
}

// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {
    stack_overflow::cleanup();
}

pub fn unsupported<T>() -> std_io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> std_io::Error {
    std_io::const_io_error!(
        std_io::ErrorKind::Unsupported,
        "operation not supported on this platform",
    )
}

pub fn decode_error_kind(errno: i32) -> ErrorKind {
    use ErrorKind::*;
    match errno as libc::c_int {
        libc::E2BIG => ArgumentListTooLong,
        libc::EADDRINUSE => AddrInUse,
        libc::EADDRNOTAVAIL => AddrNotAvailable,
        libc::EBUSY => ResourceBusy,
        libc::ECONNABORTED => ConnectionAborted,
        libc::ECONNREFUSED => ConnectionRefused,
        libc::ECONNRESET => ConnectionReset,
        libc::EDEADLK => Deadlock,
        libc::EDQUOT => FilesystemQuotaExceeded,
        libc::EEXIST => AlreadyExists,
        libc::EFBIG => FileTooLarge,
        libc::EHOSTUNREACH => HostUnreachable,
        libc::EINTR => Interrupted,
        libc::EINVAL => InvalidInput,
        libc::EISDIR => IsADirectory,
        libc::ELOOP => FilesystemLoop,
        libc::ENOENT => NotFound,
        libc::ENOMEM => OutOfMemory,
        libc::ENOSPC => StorageFull,
        libc::ENOSYS => Unsupported,
        libc::EMLINK => TooManyLinks,
        libc::ENAMETOOLONG => InvalidFilename,
        libc::ENETDOWN => NetworkDown,
        libc::ENETUNREACH => NetworkUnreachable,
        libc::ENOTCONN => NotConnected,
        libc::ENOTDIR => NotADirectory,
        libc::ENOTEMPTY => DirectoryNotEmpty,
        libc::EPIPE => BrokenPipe,
        libc::EROFS => ReadOnlyFilesystem,
        libc::ESPIPE => NotSeekable,
        libc::ESTALE => StaleNetworkFileHandle,
        libc::ETIMEDOUT => TimedOut,
        libc::ETXTBSY => ExecutableFileBusy,
        libc::EXDEV => CrossesDevices,

        libc::EACCES | libc::EPERM => PermissionDenied,

        // These two constants can have the same value on some systems,
        // but different values on others, so we can't use a match
        // clause
        x if x == libc::EAGAIN || x == libc::EWOULDBLOCK => WouldBlock,

        _ => Uncategorized,
    }
}

pub fn abort_internal() -> ! {
    core::intrinsics::abort();
}

pub fn hashmap_random_keys() -> (u64, u64) {
    (1, 2)
}
