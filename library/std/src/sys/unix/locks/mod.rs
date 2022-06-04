cfg_if::cfg_if! {
    if #[cfg(any(
        target_os = "linux",
        target_os = "android",
    ))] {
        mod futex;
        mod futex_rwlock;
        pub(crate) use futex::{Mutex, MovableMutex, MovableCondvar};
        pub(crate) use futex_rwlock::{RwLock, MovableRwLock};
    } else {
        mod pthread_mutex;
        mod pthread_remutex;
        mod pthread_rwlock;
        mod pthread_condvar;
        pub(crate) use pthread_mutex::{Mutex, MovableMutex};
        pub(crate) use pthread_rwlock::{RwLock, MovableRwLock};
        pub(crate) use pthread_condvar::MovableCondvar;
    }
}
