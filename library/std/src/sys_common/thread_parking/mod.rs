cfg_if::cfg_if! {
    if #[cfg(all(
        not(target_family = "postgres"),
        any(
            target_os = "linux",
            target_os = "android",
            all(target_arch = "wasm32", target_feature = "atomics"),
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "dragonfly",
            target_os = "fuchsia",
            target_os = "hermit",
        ))
    )] {
        mod futex;
        pub use futex::Parker;
    } else if #[cfg(all(not(target_family = "postgres"), any(
        target_os = "netbsd",
        all(target_vendor = "fortanix", target_env = "sgx"),
        target_os = "solid_asp3",
    )))] {
        mod id;
        pub use id::Parker;
    } else {
        pub use crate::sys::thread_parking::Parker;
    }
}
