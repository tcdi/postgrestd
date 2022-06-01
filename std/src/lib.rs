#![no_std]
// These are required just for reexports.
#![feature(
    assert_matches,
    async_iterator,
    core_ffi_c,
    core_intrinsics,
    core_panic,
    once_cell,
    portable_simd,
    unicode_internals
)]

extern crate alloc as inner_alloc;
pub use core::{
    any, arch, array, ascii, assert_matches, async_iter, cell, char, clone, cmp, convert, default,
    f32, f64, ffi, future, hash, hint, i128, i16, i32, i64, i8, intrinsics, isize, iter, lazy,
    marker, mem, num, ops, option, panic, panicking, pin, primitive, ptr, result, simd, time, u128,
    u16, u32, u64, u8, unicode, usize,
};

pub mod alloc {
    pub use crate::inner_alloc::alloc::*;
}

pub use inner_alloc::{borrow, boxed, collections, fmt, rc, slice, str, string, sync, task, vec};

pub mod backtrace;
pub mod error;
pub mod io;
pub mod os;
pub mod path;
pub mod prelude;
