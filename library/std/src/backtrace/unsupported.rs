//! Support for capturing a stack backtrace of an OS thread
//!
//! This module contains the support necessary to capture a stack backtrace of a
//! running OS thread from the OS thread itself. The `Backtrace` type supports
//! capturing a stack trace via the `Backtrace::capture` and
//! `Backtrace::force_capture` functions.
//!
//! A backtrace is typically quite handy to attach to errors (e.g. types
//! implementing `std::error::Error`) to get a causal chain of where an error
//! was generated.
//!
//! ## Accuracy
//!
//! Backtraces are attempted to be as accurate as possible, but no guarantees
//! are provided about the exact accuracy of a backtrace. Instruction pointers,
//! symbol names, filenames, line numbers, etc, may all be incorrect when
//! reported. Accuracy is attempted on a best-effort basis, however, any bug
//! reports are always welcome to indicate areas of improvement!
//!
//! For most platforms a backtrace with a filename/line number requires that
//! programs be compiled with debug information. Without debug information
//! filenames/line numbers will not be reported.
//!
//! ## Platform support
//!
//! Not all platforms that libstd compiles for support capturing backtraces.
//! Some platforms simply do nothing when capturing a backtrace. To check
//! whether the platform supports capturing backtraces you can consult the
//! `BacktraceStatus` enum as a result of `Backtrace::status`.
//!
//! Like above with accuracy platform support is done on a best effort basis.
//! Sometimes libraries might not be available at runtime or something may go
//! wrong which would cause a backtrace to not be captured. Please feel free to
//! report issues with platforms where a backtrace cannot be captured though!
//!
//! ## Environment Variables
//!
//! The `Backtrace::capture` function might not actually capture a backtrace by
//! default. Its behavior is governed by two environment variables:
//!
//! * `RUST_LIB_BACKTRACE` - if this is set to `0` then `Backtrace::capture`
//!   will never capture a backtrace. Any other value set will enable
//!   `Backtrace::capture`.
//!
//! * `RUST_BACKTRACE` - if `RUST_LIB_BACKTRACE` is not set, then this variable
//!   is consulted with the same rules of `RUST_LIB_BACKTRACE`.
//!
//! * If neither of the above env vars are set, then `Backtrace::capture` will
//!   be disabled.
//!
//! Capturing a backtrace can be a quite expensive runtime operation, so the
//! environment variables allow either forcibly disabling this runtime
//! performance hit or allow selectively enabling it in some programs.
//!
//! Note that the `Backtrace::force_capture` function can be used to ignore
//! these environment variables. Also note that the state of environment
//! variables is cached once the first backtrace is created, so altering
//! `RUST_LIB_BACKTRACE` or `RUST_BACKTRACE` at runtime might not actually change
//! how backtraces are captured.

#![stable(feature = "backtrace", since = "1.65.0")]
// Fully no-disabled version of `std::backtrace`, to avoid the various downsides
// inherent in producing backtraces.
//
// FIXME(thom): this is a terrible way to stub this out, and risks accidentally
// drifting from the public backtrace API :(
use crate::fmt;

/// A captured OS thread stack backtrace.
///
/// This type represents a stack backtrace for an OS thread captured at a
/// previous point in time. In some instances the `Backtrace` type may
/// internally be empty due to configuration. For more information see
/// `Backtrace::capture`.
#[stable(feature = "backtrace", since = "1.65.0")]
#[must_use]
pub struct Backtrace {
    inner: Inner,
}

/// The current status of a backtrace, indicating whether it was captured or
/// whether it is empty for some other reason.
#[stable(feature = "backtrace", since = "1.65.0")]
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum BacktraceStatus {
    /// Capturing a backtrace is not supported, likely because it's not
    /// implemented for the current platform.
    #[stable(feature = "backtrace", since = "1.65.0")]
    Unsupported,
    /// Capturing a backtrace has been disabled through either the
    /// `RUST_LIB_BACKTRACE` or `RUST_BACKTRACE` environment variables.
    #[stable(feature = "backtrace", since = "1.65.0")]
    Disabled,
    /// A backtrace has been captured and the `Backtrace` should print
    /// reasonable information when rendered.
    #[stable(feature = "backtrace", since = "1.65.0")]
    Captured,
}

enum Inner {
    Unsupported,
    Disabled,
}

/// A single frame of a backtrace.
#[unstable(feature = "backtrace_frames", issue = "79676")]
pub struct BacktraceFrame {
    _private: (),
}

#[stable(feature = "backtrace", since = "1.65.0")]
impl fmt::Debug for Backtrace {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            Inner::Unsupported => fmt.write_str("<unsupported>"),
            Inner::Disabled => fmt.write_str("<disabled>"),
        }
    }
}

#[unstable(feature = "backtrace_frames", issue = "79676")]
impl fmt::Debug for BacktraceFrame {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_list().finish()
    }
}

impl Backtrace {
    /// Capture a stack backtrace of the current thread.
    ///
    /// This function will capture a stack backtrace of the current OS thread of
    /// execution, returning a `Backtrace` type which can be later used to print
    /// the entire stack trace or render it to a string.
    ///
    /// This function will be a noop if the `RUST_BACKTRACE` or
    /// `RUST_LIB_BACKTRACE` backtrace variables are both not set. If either
    /// environment variable is set and enabled then this function will actually
    /// capture a backtrace. Capturing a backtrace can be both memory intensive
    /// and slow, so these environment variables allow liberally using
    /// `Backtrace::capture` and only incurring a slowdown when the environment
    /// variables are set.
    ///
    /// To forcibly capture a backtrace regardless of environment variables, use
    /// the `Backtrace::force_capture` function.
    #[stable(feature = "backtrace", since = "1.65.0")]
    pub fn capture() -> Backtrace {
        Backtrace { inner: Inner::Unsupported }
    }

    /// Forcibly captures a full backtrace, regardless of environment variable
    /// configuration.
    ///
    /// This function behaves the same as `capture` except that it ignores the
    /// values of the `RUST_BACKTRACE` and `RUST_LIB_BACKTRACE` environment
    /// variables, always capturing a backtrace.
    ///
    /// Note that capturing a backtrace can be an expensive operation on some
    /// platforms, so this should be used with caution in performance-sensitive
    /// parts of code.
    #[stable(feature = "backtrace", since = "1.65.0")]
    #[inline(never)] // want to make sure there's a frame here to remove
    pub fn force_capture() -> Backtrace {
        Backtrace { inner: Inner::Unsupported }
    }

    /// Forcibly captures a disabled backtrace, regardless of environment
    /// variable configuration.
    #[stable(feature = "backtrace", since = "1.65.0")]
    #[rustc_const_stable(feature = "backtrace", since = "1.65.0")]
    pub const fn disabled() -> Backtrace {
        Backtrace { inner: Inner::Disabled }
    }

    /// Returns the status of this backtrace, indicating whether this backtrace
    /// request was unsupported, disabled, or a stack trace was actually
    /// captured.
    #[stable(feature = "backtrace", since = "1.65.0")]
    #[must_use]
    pub fn status(&self) -> BacktraceStatus {
        match self.inner {
            Inner::Unsupported => BacktraceStatus::Unsupported,
            Inner::Disabled => BacktraceStatus::Disabled,
        }
    }
}

impl<'a> Backtrace {
    /// Returns an iterator over the backtrace frames.
    #[must_use]
    #[unstable(feature = "backtrace_frames", issue = "79676")]
    pub fn frames(&'a self) -> &'a [BacktraceFrame] {
        &[]
    }
}

#[stable(feature = "backtrace", since = "1.65.0")]
impl fmt::Display for Backtrace {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            Inner::Unsupported => fmt.write_str("unsupported backtrace"),
            Inner::Disabled => fmt.write_str("disabled backtrace"),
        }
    }
}
