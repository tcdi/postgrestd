#![cfg_attr(not(feature = "restricted-std"), stable(feature = "rust1", since = "1.0.0"))]
#![cfg_attr(feature = "restricted-std", unstable(feature = "restricted_std", issue = "none"))]
#![no_std]
// These are required just for reexports.
#![feature(
    assert_matches,
    async_iterator,
    core_ffi_c,
    core_panic,
    once_cell,
    unicode_internals
)]

#![feature(prelude_import)]

// Library features (core):
#![feature(array_error_internals)]
#![feature(atomic_mut_ptr)]
#![feature(char_error_internals)]
#![feature(char_internals)]
#![feature(core_c_str)]
#![feature(core_intrinsics)]
#![feature(cstr_from_bytes_until_nul)]
#![feature(cstr_internals)]
#![feature(duration_checked_float)]
#![feature(duration_constants)]
#![feature(exact_size_is_empty)]
#![feature(extend_one)]
#![feature(float_minimum_maximum)]
#![feature(hasher_prefixfree_extras)]
#![feature(hashmap_internals)]
#![feature(int_error_internals)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_write_slice)]
#![feature(mixed_integer_ops)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(panic_can_unwind)]
#![feature(panic_info_message)]
#![feature(panic_internals)]
#![feature(portable_simd)]
#![feature(prelude_2024)]
#![feature(ptr_as_uninit)]
#![feature(raw_os_nonzero)]
#![feature(slice_internals)]
#![feature(slice_ptr_get)]
#![feature(std_internals)]
#![feature(str_internals)]
#![feature(strict_provenance)]
//
// Library features (alloc):
#![feature(alloc_layout_extra)]
#![feature(alloc_c_string)]
#![feature(allocator_api)]
#![feature(get_mut_unchecked)]
#![feature(map_try_insert)]
#![feature(new_uninit)]
#![feature(thin_box)]
#![feature(try_reserve_kind)]
#![feature(vec_into_raw_parts)]
#![feature(slice_concat_trait)]

// #![feature(alloc_error_handler)]
// #![feature(allocator_internals)]
#![feature(allow_internal_unsafe)]
#![feature(allow_internal_unstable)]
// #![feature(box_syntax)]
// #![feature(c_unwind)]
// #![feature(cfg_target_thread_local)]
#![feature(concat_idents)]
// #![feature(const_mut_refs)]
// #![feature(const_trait_impl)]
// #![feature(decl_macro)]
// #![feature(deprecated_suggestion)]
// #![feature(doc_cfg)]
// #![feature(doc_cfg_hide)]
// #![feature(doc_masked)]
#![feature(doc_notable_trait)]
// #![feature(dropck_eyepatch)]
// #![feature(exhaustive_patterns)]
// #![feature(intra_doc_pointers)]
// #![feature(lang_items)]
// #![feature(let_chains)]
// #![feature(let_else)]
// #![feature(linkage)]
#![feature(min_specialization)]
// #![feature(must_not_suspend)]
// #![feature(needs_panic_runtime)]
// #![feature(negative_impls)]
// #![feature(never_type)]
// #![feature(nll)]
// #![feature(platform_intrinsics)]
// #![feature(prelude_import)]
#![feature(rustc_attrs)]
// #![feature(rustdoc_internals)]
#![feature(staged_api)]
// #![feature(thread_local)]
// #![feature(try_blocks)]

// #![feature(assert_matches)]
// #![feature(async_iterator)]
#![feature(c_variadic)]
#![feature(cfg_accessible)]
#![feature(cfg_eval)]
#![feature(concat_bytes)]
// #![feature(const_format_args)]
// #![feature(core_ffi_c)]
// #![feature(core_panic)]
#![feature(custom_test_frameworks)]
#![feature(edition_panic)]
#![feature(format_args_nl)]
#![feature(log_syntax)]
// #![feature(once_cell)]
// #![feature(saturating_int_impl)]
#![feature(stdsimd)]
#![feature(test)]
#![feature(trace_macros)]

extern crate alloc as inner_alloc;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::{
    any, arch, array, ascii, assert_matches, async_iter, cell, char, clone, cmp, convert, default,
    f32, f64, ffi, future, hash, hint, i128, i16, i32, i64, i8, intrinsics, isize, iter, lazy,
    marker, mem, num, ops, option, panic, panicking, pin, primitive, ptr, result, simd, time, u128,
    u16, u32, u64, u8, unicode, usize,
};

#[stable(feature = "rust1", since = "1.0.0")]
pub mod alloc { 
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use crate::inner_alloc::alloc::*;
}

#[stable(feature = "rust1", since = "1.0.0")]
pub use inner_alloc::{borrow, boxed, collections, fmt, rc, slice, str, string, sync, task, vec};

pub mod backtrace;
pub mod error;
pub mod io;
pub mod os;
pub mod path;
pub mod prelude;

mod sys;
mod sys_common;

#[prelude_import]
#[allow(unused)]
use prelude::rust_2021::*;
