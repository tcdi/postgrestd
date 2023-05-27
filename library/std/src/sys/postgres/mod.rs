#![allow(unused)]
#[path = "../unix/weak.rs"]
#[macro_use]
pub mod weak;

pub mod alloc;
pub mod args;
#[path = "../unix/cmath.rs"]
pub mod cmath;
pub mod env;
pub mod fs;
pub mod io;
pub mod locks;
pub mod net;
pub mod os;
#[path = "../unix/os_str.rs"]
pub mod os_str;
#[path = "../unix/path.rs"]
pub mod path;
pub mod pipe;
pub mod process;
pub mod rand;
pub mod stdio;
pub mod thread;
#[cfg(target_thread_local)]
pub mod thread_local_dtor;
pub mod thread_local_key;
pub mod time;

pub use rand::hashmap_random_keys;

mod common;
pub use common::*;

pub mod fd;

use crate::ffi::CStr;
use crate::io::ErrorKind;

#[doc(hidden)]
pub trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
    ($($t:ident)*) => ($(impl IsMinusOne for $t {
        fn is_minus_one(&self) -> bool {
            *self == -1
        }
    })*)
}

impl_is_minus_one! { i8 i16 i32 i64 isize }

pub fn real_last_os_error_use_carefully() -> crate::io::Error {
    crate::io::Error::from_raw_os_error(os::real_errno_use_carefully())
}
// TODO: audit these and think about them more...
pub use {cvt_unsup as cvt, cvt_unsup_r as cvt_r};

pub fn cvt_unsup<T: IsMinusOne>(t: T) -> crate::io::Result<T> {
    if t.is_minus_one() {
        unsupported()
    } else {
        Ok(t)
    }
}

pub fn cvt_unsup_r<T, F>(mut f: F) -> crate::io::Result<T>
where
    T: IsMinusOne,
    F: FnMut() -> T,
{
    loop {
        if f().is_minus_one() {
            let e = real_last_os_error_use_carefully();
            if e.kind() != ErrorKind::Interrupted {
                return unsupported();
            }
        }
    }
}

// #[allow(dead_code)] // Not used on all platforms.
// pub fn cvt_nz(error: libc::c_int) -> crate::io::Result<()> {
//     if error == 0 {
//         Ok(())
//     } else {
//         Err(crate::io::Error::from_raw_os_error(error))
//     }
// }

pub use libc::strlen;
