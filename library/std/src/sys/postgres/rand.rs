/// Mostly based on `sys/unix/rand.rs`, although that impl can't really be used
/// because:
/// 1. We've killed `File`'s ability to actually read from the system.
/// 2. Our `errno()` function is a liar, and `real_errno_use_carefully()` must
///    be used instead.
///
/// On the bright side, we don't support versions of Linux without `getrandom`
/// or versions of macOS without `getentropy`. This doesn't actually help much
/// in practice though, since the fallback `/dev/urandom` path is hit in cases
/// like early boot and whatnot. Note that we're single threaded so
/// `hashmap_random_keys` is not called multiple times, the cache in
/// `RandomState`'s thread local is enough for us. This does not simplify very
/// much for us, sadly.
///
/// Technically we could also use `pg_strong_random` to implement this. This
/// would be less code, but we'd be screwed if it ever fired a PG error (we
/// can't use pgx-pg-sys's sjlj catching). The current impl of
/// `pg_strong_random` doesn't seem to ever do that, but given that it *is* a
/// fallible function, I'm not that comfortable with relying on that.

pub fn hashmap_random_keys() -> (u64, u64) {
    const KEY_LEN: usize = core::mem::size_of::<u64>();

    let mut v = [0u8; KEY_LEN * 2];
    if !try_fill_bytes(&mut v) {
        if !urandom_fill_bytes(&mut v) {
            let syscall = if cfg!(target_os = "linux") { "getrandom" } else { "getentropy" };
            // Panic to inform the user that they've misconfigured something,
            // rather than falling back to something insecure.
            panic!("failed to initialize hashmap keys: could not use {syscall} or /dev/urandom");
        }
    }

    let key1 = v[0..KEY_LEN].try_into().unwrap();
    let key2 = v[KEY_LEN..].try_into().unwrap();

    (u64::from_ne_bytes(key1), u64::from_ne_bytes(key2))
}

#[cfg(target_os = "linux")]
fn try_fill_bytes(v: &mut [u8]) -> bool {
    use crate::sync::atomic::{AtomicBool, Ordering};
    use crate::sys::os::real_errno_use_carefully;

    let mut read = 0;
    while read < v.len() {
        let result = getrandom(&mut v[read..]);
        if result == -1 {
            let err = real_errno_use_carefully() as libc::c_int;
            if err == libc::EINTR {
                continue;
            } else if err == libc::ENOSYS || err == libc::EPERM || err == libc::EAGAIN {
                // Don't bother remembering, we only come here once in
                // postgrestd anyway (no threads).
                return false;
            }
        } else {
            read += result as usize;
        }
    }
    true
}

#[cfg(target_os = "linux")]
fn getrandom(buf: &mut [u8]) -> libc::ssize_t {
    use super::weak::syscall;
    use crate::sync::atomic::{AtomicBool, Ordering};
    use crate::sys::os::real_errno_use_carefully;

    // A weak symbol allows interposition, e.g. for perf measurements that want to
    // disable randomness for consistency. Otherwise, we'll try a raw syscall.
    // (`getrandom` was added in glibc 2.25, musl 1.1.20, android API level 28)
    syscall! {
        fn getrandom(
            buffer: *mut libc::c_void,
            length: libc::size_t,
            flags: libc::c_uint
        ) -> libc::ssize_t
    }

    // This provides the best quality random numbers available at the given moment
    // without ever blocking, and is preferable to falling back to /dev/urandom.
    static GRND_INSECURE_AVAILABLE: AtomicBool = AtomicBool::new(true);
    if GRND_INSECURE_AVAILABLE.load(Ordering::Relaxed) {
        let ret = unsafe { getrandom(buf.as_mut_ptr().cast(), buf.len(), libc::GRND_INSECURE) };
        if ret == -1 && (real_errno_use_carefully() as libc::c_int) == libc::EINVAL {
            GRND_INSECURE_AVAILABLE.store(false, Ordering::Relaxed);
        } else {
            return ret;
        }
    }

    unsafe { getrandom(buf.as_mut_ptr().cast(), buf.len(), libc::GRND_NONBLOCK) }
}

#[cfg(target_os = "macos")]
fn try_fill_bytes(v: &mut [u8]) -> bool {
    use crate::sys::os::real_errno_use_carefully;
    extern "C" {
        // Present on macOS 10.12+
        fn getentropy(p: *mut libc::c_void, sz: libc::size_t) -> libc::c_int;
    }
    for chunk in v.chunks_mut(256) {
        if unsafe { getentropy(chunk.as_mut_ptr().cast::<libc::c_void>(), chunk.len()) } == -1 {
            return false;
        }
    }
    true
}

// Unfortunately even if we only support kernel versions that have getentropy,
// we may have to fall back to `/dev/urandom` when either seccomp has disabled
// the syscall, or if it's early boot and the kernel doesn't have (the
// relatively recent) `GRND_INSECURE` flag.
fn urandom_fill_bytes(v: &mut [u8]) -> bool {
    use crate::sys::os::real_errno_use_carefully;
    // Can't use `File` for any of this, because we've disabled `File`'s ability to do I/O.
    let fd =
        unsafe { libc::open(b"/dev/urandom\0".as_ptr().cast::<libc::c_char>(), libc::O_RDONLY, 0) };
    if fd < 0 {
        return false;
    }

    let mut read = 0;
    while read < v.len() {
        let remaining = &mut v[read..];
        let result = unsafe { libc::read(fd, remaining.as_mut_ptr().cast(), remaining.len()) };
        if result <= 0 {
            if real_errno_use_carefully() == libc::EINTR {
                continue;
            }
            break;
        } else {
            read += result as usize;
        }
    }
    unsafe {
        libc::close(fd);
    }
    true
}
