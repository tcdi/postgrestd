use crate::borrow::Cow;
use crate::env;
use crate::fmt;
use crate::io;
use crate::io::prelude::*;
use crate::path::{self, Path, PathBuf};
use crate::sync::{Mutex, PoisonError};

// pub fn lock() -> impl Drop {
//     static LOCK: Mutex<()> = Mutex::new(());
//     LOCK.lock().unwrap_or_else(PoisonError::into_inner)
// }

#[inline(never)]
pub fn __rust_begin_short_backtrace<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let result = f();

    // prevent this frame from being tail-call optimised away
    crate::hint::black_box(());

    result
}

#[inline(never)]
pub fn __rust_end_short_backtrace<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let result = f();

    // prevent this frame from being tail-call optimised away
    crate::hint::black_box(());

    result
}
