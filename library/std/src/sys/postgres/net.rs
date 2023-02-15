#![allow(non_camel_case_types)]
use crate::fmt;
use crate::prelude::rust_2021::*;
use crate::io::{self, IoSlice, IoSliceMut};
use crate::net::{Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr};
use crate::sys::unsupported;

use crate::os::unix::prelude::*;
pub use crate::sys::{cvt, cvt_r};

use crate::cmp;
use crate::ffi::CStr;
use crate::mem;
use crate::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
use crate::str;
use crate::sys::fd::FileDesc;
use crate::sys_common::net::{getsockopt, setsockopt, sockaddr_to_addr};
use crate::sys_common::{AsInner, FromInner, IntoInner};
use crate::time::{Duration, Instant};

pub extern crate libc as netc;
use netc::{MSG_PEEK, sockaddr, socklen_t};
use netc::c_int;
use core::ffi::c_void;

pub type wrlen_t = netc::size_t;

pub struct TcpStream(!);

impl TcpStream {
    pub fn connect(_: io::Result<&SocketAddr>) -> io::Result<TcpStream> {
        unsupported()
    }

    pub fn connect_timeout(_: &SocketAddr, _: Duration) -> io::Result<TcpStream> {
        unsupported()
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.0
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.0
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0
    }

    pub fn read(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0
    }

    pub fn read_vectored(&self, _: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.0
    }

    pub fn is_read_vectored(&self) -> bool {
        self.0
    }

    pub fn write(&self, _: &[u8]) -> io::Result<usize> {
        self.0
    }

    pub fn write_vectored(&self, _: &[IoSlice<'_>]) -> io::Result<usize> {
        self.0
    }

    pub fn is_write_vectored(&self) -> bool {
        self.0
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn shutdown(&self, _: Shutdown) -> io::Result<()> {
        self.0
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        self.0
    }

    pub fn set_linger(&self, _: Option<Duration>) -> io::Result<()> {
        self.0
    }

    pub fn linger(&self) -> io::Result<Option<Duration>> {
        self.0
    }

    pub fn set_nodelay(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        self.0
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.0
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.0
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        self.0
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

pub struct TcpListener(!);

impl TcpListener {
    pub fn bind(_: io::Result<&SocketAddr>) -> io::Result<TcpListener> {
        unsupported()
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        self.0
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        self.0
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.0
    }

    pub fn set_only_v6(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        self.0
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.0
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        self.0
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

pub struct UdpSocket(!);

impl UdpSocket {
    pub fn bind(_: io::Result<&SocketAddr>) -> io::Result<UdpSocket> {
        unsupported()
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn recv_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.0
    }

    pub fn peek_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.0
    }

    pub fn send_to(&self, _: &[u8], _: &SocketAddr) -> io::Result<usize> {
        self.0
    }

    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        self.0
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.0
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.0
    }

    pub fn set_broadcast(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        self.0
    }

    pub fn set_multicast_loop_v4(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        self.0
    }

    pub fn set_multicast_ttl_v4(&self, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        self.0
    }

    pub fn set_multicast_loop_v6(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        self.0
    }

    pub fn join_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        self.0
    }

    pub fn join_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn leave_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        self.0
    }

    pub fn leave_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.0
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.0
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn recv(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0
    }

    pub fn send(&self, _: &[u8]) -> io::Result<usize> {
        self.0
    }

    pub fn connect(&self, _: io::Result<&SocketAddr>) -> io::Result<()> {
        self.0
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

pub struct LookupHost(!);

impl LookupHost {
    pub fn port(&self) -> u16 {
        self.0
    }
}

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        self.0
    }
}

impl TryFrom<&str> for LookupHost {
    type Error = io::Error;

    fn try_from(_v: &str) -> io::Result<LookupHost> {
        unsupported()
    }
}

impl<'a> TryFrom<(&'a str, u16)> for LookupHost {
    type Error = io::Error;

    fn try_from(_v: (&'a str, u16)) -> io::Result<LookupHost> {
        unsupported()
    }
}

pub struct Socket(FileDesc);

pub fn init() {}

pub fn cvt_gai(err: c_int) -> io::Result<()> {
    if err == 0 {
        return Ok(());
    }

    // We may need to trigger a gnetc workaround. See on_resolver_failure() for details.
    on_resolver_failure();

    #[cfg(not(target_os = "espidf"))]
    if err == netc::EAI_SYSTEM {
        return Err(io::Error::last_os_error());
    }

    #[cfg(not(target_os = "espidf"))]
    let detail = unsafe {
        str::from_utf8(CStr::from_ptr(netc::gai_strerror(err)).to_bytes()).unwrap().to_owned()
    };

    #[cfg(target_os = "espidf")]
    let detail = "";

    Err(io::Error::new(
        io::ErrorKind::Uncategorized,
        &format!("failed to lookup address information: {detail}")[..],
    ))
}

impl Socket {
    pub fn new(addr: &SocketAddr, ty: c_int) -> io::Result<Socket> {
        unsupported()
    }

    pub fn new_raw(fam: c_int, ty: c_int) -> io::Result<Socket> {
        unsupported()
    }

    pub fn new_pair(fam: c_int, ty: c_int) -> io::Result<(Socket, Socket)> {
        unsupported()
    }

    pub fn connect_timeout(&self, addr: &SocketAddr, timeout: Duration) -> io::Result<()> {
        unsupported()
    }

    pub fn accept(&self, storage: *mut sockaddr, len: *mut socklen_t) -> io::Result<Socket> {
        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<Socket> {
        unsupported()
    }

    fn recv_with_flags(&self, buf: &mut [u8], flags: c_int) -> io::Result<usize> {
        unsupported()
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        unsupported()
    }

    #[inline]
    pub fn is_read_vectored(&self) -> bool {
        false
    }

    fn recv_from_with_flags(
        &self,
        buf: &mut [u8],
        flags: c_int,
    ) -> io::Result<(usize, SocketAddr)> {
        unsupported()
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        unsupported()
    }

    #[cfg(any(target_os = "android", target_os = "linux", target_os = "postgres"))]
    pub fn recv_msg(&self, msg: &mut netc::msghdr) -> io::Result<usize> {
        unsupported()
    }

    pub fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        unsupported()
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

    #[cfg(any(target_os = "android", target_os = "linux", target_os = "postgres"))]
    pub fn send_msg(&self, msg: &mut netc::msghdr) -> io::Result<usize> {
        unsupported()
    }

    pub fn set_timeout(&self, dur: Option<Duration>, kind: netc::c_int) -> io::Result<()> {
        unsupported()
    }

    pub fn timeout(&self, kind: netc::c_int) -> io::Result<Option<Duration>> {
        unsupported()
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        unsupported()
    }

    pub fn set_linger(&self, linger: Option<Duration>) -> io::Result<()> {
        unsupported()
    }

    pub fn linger(&self) -> io::Result<Option<Duration>> {
        unsupported()
    }

    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        unsupported()
    }

    #[cfg(any(target_os = "android", target_os = "linux",))]
    pub fn set_quickack(&self, quickack: bool) -> io::Result<()> {
        unsupported()
    }

    #[cfg(any(target_os = "android", target_os = "linux",))]
    pub fn quickack(&self) -> io::Result<bool> {
        unsupported()
    }

    #[cfg(any(target_os = "android", target_os = "linux", target_os = "postgres"))]
    pub fn set_passcred(&self, passcred: bool) -> io::Result<()> {
        unsupported()
    }

    #[cfg(any(target_os = "android", target_os = "linux", target_os = "postgres"))]
    pub fn passcred(&self) -> io::Result<bool> {
        unsupported()
    }

    #[cfg(target_os = "netbsd")]
    pub fn set_passcred(&self, passcred: bool) -> io::Result<()> {
        unsupported()
    }

    #[cfg(target_os = "netbsd")]
    pub fn passcred(&self) -> io::Result<bool> {
        unsupported()
    }

    #[cfg(not(any(target_os = "solaris", target_os = "illumos")))]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        unsupported()
    }

    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        unsupported()
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
    pub fn set_mark(&self, mark: u32) -> io::Result<()> {
        unsupported()
    }


    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unsupported()
    }

    // This is used by sys_common code to abstract over Windows and Unix.
    pub fn as_raw(&self) -> RawFd {
        unimplemented!()
    }
}

impl AsInner<FileDesc> for Socket {
    fn as_inner(&self) -> &FileDesc {
        &self.0
    }
}

impl IntoInner<FileDesc> for Socket {
    fn into_inner(self) -> FileDesc {
        self.0
    }
}

impl FromInner<FileDesc> for Socket {
    fn from_inner(file_desc: FileDesc) -> Self {
        Self(file_desc)
    }
}

impl AsFd for Socket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl IntoRawFd for Socket {
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}

impl FromRawFd for Socket {
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(FromRawFd::from_raw_fd(raw_fd))
    }
}

fn on_resolver_failure() {}
