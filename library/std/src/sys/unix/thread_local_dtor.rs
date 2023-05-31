#![cfg(target_thread_local)]
#![unstable(feature = "thread_local_internals", issue = "none")]

//! Provides thread-local destructors without an associated "key", which
//! can be more efficient.

// Since what appears to be glibc 2.18 this symbol has been shipped which
// GCC and clang both use to invoke destructors in thread_local globals, so
// let's do the same!
//
// Note, however, that we run on lots older linuxes, as well as cross
// compiling from a newer linux to an older linux, so we also have a
// fallback implementation to use as well.
#[cfg(any(target_os = "linux", target_os = "fuchsia", target_os = "redox"))]
pub unsafe fn register_dtor(t: *mut u8, dtor: unsafe extern "C" fn(*mut u8)) {
    use crate::mem;
    use crate::sys_common::thread_local_dtor::register_dtor_fallback;

    extern "C" {
        #[linkage = "extern_weak"]
        static __dso_handle: *mut u8;
        #[linkage = "extern_weak"]
        static __cxa_thread_atexit_impl: *const libc::c_void;
    }
    if !__cxa_thread_atexit_impl.is_null() {
        type F = unsafe extern "C" fn(
            dtor: unsafe extern "C" fn(*mut u8),
            arg: *mut u8,
            dso_handle: *mut u8,
        ) -> libc::c_int;
        mem::transmute::<*const libc::c_void, F>(__cxa_thread_atexit_impl)(
            dtor,
            t,
            &__dso_handle as *const _ as *mut _,
        );
        return;
    }
    register_dtor_fallback(t, dtor);
}

// This implementation is very similar to register_dtor_fallback in
// sys_common/thread_local.rs. The main difference is that we want to hook into
// macOS's analog of the above linux function, _tlv_atexit. OSX will run the
// registered dtors before any TLS slots get freed, and when the main thread
// exits.
//
// Unfortunately, calling _tlv_atexit while tls dtors are running is UB. The
// workaround below is to register, via _tlv_atexit, a custom DTOR list once per
// thread. thread_local dtors are pushed to the DTOR list without calling
// _tlv_atexit.
#[cfg(target_os = "macos")]
pub unsafe fn register_dtor(t: *mut u8, dtor: unsafe extern "C" fn(*mut u8)) {
    use crate::cell::Cell;
    use crate::mem;
    use crate::ptr;

    #[thread_local]
    static REGISTERED: Cell<bool> = Cell::new(false);

    #[thread_local]
    static mut DTORS: Vec<(*mut u8, unsafe extern "C" fn(*mut u8))> = Vec::new();

    if !REGISTERED.get() {
        _tlv_atexit(run_dtors, ptr::null_mut());
        REGISTERED.set(true);
    }

    extern "C" {
        fn _tlv_atexit(dtor: unsafe extern "C" fn(*mut u8), arg: *mut u8);
    }

    let list = &mut DTORS;
    list.push((t, dtor));

    unsafe extern "C" fn run_dtors(_: *mut u8) {
        let mut list = mem::take(&mut DTORS);
        while !list.is_empty() {
            for (ptr, dtor) in list {
                dtor(ptr);
            }
            list = mem::take(&mut DTORS);
        }
    }
}

#[cfg(any(target_os = "vxworks", target_os = "horizon", target_os = "emscripten"))]
#[cfg_attr(target_family = "wasm", allow(unused))] // might remain unused depending on target details (e.g. wasm32-unknown-emscripten)
pub unsafe fn register_dtor(t: *mut u8, dtor: unsafe extern "C" fn(*mut u8)) {
    use crate::sys_common::thread_local_dtor::register_dtor_fallback;
    register_dtor_fallback(t, dtor);
}
