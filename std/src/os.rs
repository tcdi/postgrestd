#![stable(feature = "rust1", since = "1.0.0")]

#[stable(feature = "rust1", since = "1.0.0")]
pub mod raw {
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use core::ffi::{c_void, c_char, c_schar, c_uchar,
        c_double, c_float, c_int, c_long, c_longlong, c_short,
        c_uint, c_ulong, c_ulonglong, c_ushort};
}
