#![unstable(reason = "not public", issue = "none", feature = "fd")]

use crate::cmp;
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut, Read};
use crate::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use crate::sys::unsupported;
use crate::sys_common::{AsInner, FromInner, IntoInner};

#[derive(Debug)]
pub struct FileDesc(!);

impl FileDesc {
    pub fn read(&self, _: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn read_vectored(&self, _: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn read_to_end(&self, _: &mut Vec<u8>) -> io::Result<usize> {
        unsupported()
    }

    pub fn read_at(&self, _: &mut [u8], _: u64) -> io::Result<usize> {
        unsupported()
    }

    pub fn read_buf(&self, _: BorrowedCursor<'_>) -> io::Result<()> {
        unsupported()
    }

    pub fn write(&self, _: &[u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn write_vectored(&self, _: &[IoSlice<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_write_vectored(&self) -> bool {
        false
    }

    pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        unsupported()
    }

    pub fn get_cloexec(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn set_cloexec(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<FileDesc> {
        unsupported()
    }
}

impl<'a> Read for &'a FileDesc {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }
}

impl AsInner<OwnedFd> for FileDesc {
    fn as_inner(&self) -> &OwnedFd {
        unimplemented!()
    }
}

impl IntoInner<OwnedFd> for FileDesc {
    fn into_inner(self) -> OwnedFd {
        unimplemented!()
    }
}

impl FromInner<OwnedFd> for FileDesc {
    fn from_inner(owned_fd: OwnedFd) -> Self {
        unimplemented!()
    }
}

impl AsFd for FileDesc {
    fn as_fd(&self) -> BorrowedFd<'_> {
        unimplemented!()
    }
}

impl AsRawFd for FileDesc {
    fn as_raw_fd(&self) -> RawFd {
        unimplemented!()
    }
}

impl IntoRawFd for FileDesc {
    fn into_raw_fd(self) -> RawFd {
        unimplemented!()
    }
}

impl FromRawFd for FileDesc {
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        unimplemented!()
    }
}
