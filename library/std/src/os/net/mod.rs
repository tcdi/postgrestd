//! OS-specific networking functionality.

#[cfg(any(target_os = "linux", target_os = "android", doc))]
#[cfg(not(target_family = "postgres"))]
pub(super) mod linux_ext;
