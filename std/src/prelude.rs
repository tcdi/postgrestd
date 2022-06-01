pub mod v1 {
    pub use crate::borrow::ToOwned;
    pub use crate::boxed::Box;
    pub use crate::clone::Clone;
    pub use crate::cmp::{Eq, Ord, PartialEq, PartialOrd};
    pub use crate::convert::{AsMut, AsRef, From, Into};
    pub use crate::default::Default;
    pub use crate::iter::{DoubleEndedIterator, ExactSizeIterator, Extend, IntoIterator, Iterator};
    pub use crate::marker::{Copy, Send, Sized, Sync, Unpin};
    pub use crate::mem::drop;
    pub use crate::ops::{Drop, Fn, FnMut, FnOnce};
    pub use crate::option::Option::{self, None, Some};
    pub use crate::result::Result::{self, Err, Ok};
    pub use crate::string::{String, ToString};
    pub use crate::vec::Vec;
    pub use core::prelude::v1::*;
}

pub mod rust_2015 {
    pub use super::v1::*;
}
pub mod rust_2018 {
    pub use super::v1::*;
}
pub mod rust_2021 {
    pub use super::v1::*;
    pub use crate::convert::{TryFrom, TryInto};
    pub use crate::iter::IntoIterator;
    pub use core::prelude::rust_2021::*;
}
