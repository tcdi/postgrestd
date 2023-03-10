use crate::io::{self, IoSlice, IoSliceMut};
use crate::mem;
use crate::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
use crate::sys::fd::FileDesc;
use crate::sys::unsupported;
use crate::sys_common::IntoInner;

////////////////////////////////////////////////////////////////////////////////
// Anonymous pipes
////////////////////////////////////////////////////////////////////////////////

pub struct AnonPipe(!);

pub fn anon_pipe() -> io::Result<(AnonPipe, AnonPipe)> {
    unsupported()
}

impl AnonPipe {
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        unsupported()
    }

    #[inline]
    pub fn is_write_vectored(&self) -> bool {
        false
    }
}

impl IntoInner<FileDesc> for AnonPipe {
    fn into_inner(self) -> FileDesc {
        self.0
    }
}

pub fn read2(p1: AnonPipe, v1: &mut Vec<u8>, p2: AnonPipe, v2: &mut Vec<u8>) -> io::Result<()> {
    unsupported()
}

impl AsRawFd for AnonPipe {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

impl AsFd for AnonPipe {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0
    }
}

impl IntoRawFd for AnonPipe {
    fn into_raw_fd(self) -> RawFd {
        self.0
    }
}

impl FromRawFd for AnonPipe {
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        panic!("unavailable")
    }
}
