use crate::fmt;
use crate::hash::{Hash, Hasher};
use crate::path::{Path, PathBuf};
use crate::sys::time::SystemTime;
use crate::sys::unsupported;

use crate::default::Default;
use crate::ffi::{CStr, CString, OsStr, OsString};
use crate::io::{self, Error, IoSlice, IoSliceMut, SeekFrom};
use crate::io::{BorrowedBuf, BorrowedCursor};
use crate::mem;
use crate::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd};
use crate::os::unix::prelude::*;
use crate::ptr;
use crate::sync::Arc;
use crate::sys::fd::FileDesc;
use crate::sys_common::{AsInner, AsInnerMut, FromInner, IntoInner};

impl AsInner<stat64> for FileAttr {
    fn as_inner(&self) -> &libc::stat64 {
        // Should be impossible, but things transmute the return value of this
        // to a raw libc type.
        unimplemented!();
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct FileTimes();

impl FileTimes {
    pub fn set_accessed(&mut self, _t: SystemTime) {}
    pub fn set_modified(&mut self, _t: SystemTime) {}
}

pub struct ReadDir(!);

pub struct DirEntry(!);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FilePermissions {
    mode: mode_t,
}
pub struct FileType(!);

pub struct FileAttr(!);

#[derive(Debug)]
pub struct DirBuilder {}

impl FileAttr {
    pub fn size(&self) -> u64 {
        self.0
    }

    pub fn perm(&self) -> FilePermissions {
        self.0
    }

    pub fn file_type(&self) -> FileType {
        self.0
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        unsupported()
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        unsupported()
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        unsupported()
    }
}

impl Clone for FileAttr {
    fn clone(&self) -> FileAttr {
        self.0
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        false
    }

    pub fn set_readonly(&mut self, _readonly: bool) {}

    pub fn mode(&self) -> u32 {
        unreachable!("can't create this")
    }
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        self.0
    }

    pub fn is_file(&self) -> bool {
        self.0
    }

    pub fn is_symlink(&self) -> bool {
        self.0
    }

    pub fn is(&self, mode: mode_t) -> bool {
        self.0
    }
}

impl Clone for FileType {
    fn clone(&self) -> FileType {
        self.0
    }
}

impl Copy for FileType {}

impl PartialEq for FileType {
    fn eq(&self, _other: &FileType) -> bool {
        self.0
    }
}

impl Eq for FileType {}

impl Hash for FileType {
    fn hash<H: Hasher>(&self, _h: &mut H) {
        self.0
    }
}

impl fmt::Debug for FileType {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        self.0
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        self.0
    }

    pub fn file_name(&self) -> OsString {
        self.0
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        unsupported()
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        unsupported()
    }

    pub fn ino(&self) -> u64 {
        panic!("no sensible answer for ino")
    }

    pub fn file_name_os_str(&self) -> &OsStr {
        self.0
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions::default()
    }

    pub fn read(&mut self, _read: bool) {}
    pub fn write(&mut self, _write: bool) {}
    pub fn append(&mut self, _append: bool) {}
    pub fn truncate(&mut self, _truncate: bool) {}
    pub fn create(&mut self, _create: bool) {}
    pub fn create_new(&mut self, _create_new: bool) {}

    pub fn custom_flags(&mut self, flags: i32) {
        self.custom_flags = flags;
    }
    pub fn mode(&mut self, mode: u32) {}

    fn get_access_mode(&self) -> io::Result<c_int> {
        match (self.read, self.write, self.append) {
            (true, false, false) => Ok(libc::O_RDONLY),
            (false, true, false) => Ok(libc::O_WRONLY),
            (true, true, false) => Ok(libc::O_RDWR),
            (false, _, true) => Ok(libc::O_WRONLY | libc::O_APPEND),
            (true, _, true) => Ok(libc::O_RDWR | libc::O_APPEND),
            (false, false, false) => Err(Error::from_raw_os_error(libc::EINVAL)),
        }
    }

    fn get_creation_mode(&self) -> io::Result<c_int> {
        match (self.write, self.append) {
            (true, false) => {}
            (false, false) => {
                if self.truncate || self.create || self.create_new {
                    return Err(Error::from_raw_os_error(libc::EINVAL));
                }
            }
            (_, true) => {
                if self.truncate && !self.create_new {
                    return Err(Error::from_raw_os_error(libc::EINVAL));
                }
            }
        }

        Ok(match (self.create, self.truncate, self.create_new) {
            (false, false, false) => 0,
            (true, false, false) => libc::O_CREAT,
            (false, true, false) => libc::O_TRUNC,
            (true, true, false) => libc::O_CREAT | libc::O_TRUNC,
            (_, _, true) => libc::O_CREAT | libc::O_EXCL,
        })
    }
}

impl File {
    pub fn open(_path: &Path, _opts: &OpenOptions) -> io::Result<File> {
        unsupported()
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        unsupported()
    }

    pub fn fsync(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn datasync(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        unsupported()
    }

    pub fn read(&self, _buf: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn read_vectored(&self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn read_buf(&self, cursor: BorrowedCursor<'_>) -> io::Result<()> {
        self.0.read_buf(cursor)
    }

    pub fn write(&self, _buf: &[u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn write_vectored(&self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_write_vectored(&self) -> bool {
        false
    }

    pub fn flush(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn seek(&self, _pos: SeekFrom) -> io::Result<u64> {
        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<File> {
        unsupported()
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        unsupported()
    }

    pub fn set_times(&self, _times: FileTimes) -> io::Result<()> {
        unsupported()
    }

    pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        unsupported()
    }

    pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        unsupported()
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder {}
    }

    pub fn mkdir(&self, _p: &Path) -> io::Result<()> {
        unsupported()
    }

    pub fn set_mode(&mut self, _mode: u32) {}
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn readdir(_p: &Path) -> io::Result<ReadDir> {
    unsupported()
}

pub fn unlink(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn rename(_old: &Path, _new: &Path) -> io::Result<()> {
    unsupported()
}

pub fn set_perm(_p: &Path, perm: FilePermissions) -> io::Result<()> {
    unsupported()
}

pub fn rmdir(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn remove_dir_all(_path: &Path) -> io::Result<()> {
    unsupported()
}

pub fn try_exists(_path: &Path) -> io::Result<bool> {
    unsupported()
}

pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn symlink(_original: &Path, _link: &Path) -> io::Result<()> {
    unsupported()
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    unsupported()
}

pub fn stat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn lstat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn canonicalize(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn copy(_from: &Path, _to: &Path) -> io::Result<u64> {
    unsupported()
}

#[cfg(any(
    all(target_os = "linux", target_env = "gnu"),
    target_os = "macos",
    target_os = "ios",
))]
use crate::sys::weak::syscall;
#[cfg(target_os = "macos")]
use crate::sys::weak::weak;

use libc::{c_int, mode_t};

// #[cfg(any(
//     target_os = "macos",
//     target_os = "ios",
//     all(target_os = "linux", target_env = "gnu"),
// ))]
// use libc::c_char;
// #[cfg(any(target_os = "linux", target_os = "emscripten", target_os = "android"))]
// use libc::dirfd;
// #[cfg(any(target_os = "linux", target_os = "emscripten"))]
// use libc::fstatat64;
// #[cfg(any(
//     target_os = "android",
//     target_os = "solaris",
//     target_os = "fuchsia",
//     target_os = "redox",
//     target_os = "illumos"
// ))]
// use libc::readdir as readdir64;
// #[cfg(target_os = "linux")]
// use libc::readdir64;
// #[cfg(any(target_os = "emscripten", target_os = "l4re"))]
// use libc::readdir64_r;
// #[cfg(not(any(
//     target_os = "android",
//     target_os = "linux",
//     target_os = "emscripten",
//     target_os = "solaris",
//     target_os = "illumos",
//     target_os = "l4re",
//     target_os = "fuchsia",
//     target_os = "redox",
// )))]
// use libc::readdir_r as readdir64_r;
// #[cfg(target_os = "android")]
// use libc::{
//     dirent as dirent64, fstat as fstat64, fstatat as fstatat64, ftruncate64, lseek64,
//     lstat as lstat64, off64_t, open as open64, stat as stat64,
// };
// #[cfg(not(any(
//     target_os = "linux",
//     target_os = "emscripten",
//     target_os = "l4re",
//     target_os = "android"
// )))]
// use libc::{
//     dirent as dirent64, fstat as fstat64, ftruncate as ftruncate64, lseek as lseek64,
//     lstat as lstat64, off_t as off64_t, open as open64,
//     stat as stat64,
// };
// #[cfg(any(target_os = "linux", target_os = "emscripten", target_os = "l4re"))]
// use libc::{dirent64, fstat64, ftruncate64, lseek64, lstat64, off64_t, open64, stat64};
#[cfg(target_os = "macos")]
use libc::stat as stat64;
#[cfg(target_os = "linux")]
use libc::stat64;

pub struct File(FileDesc);

struct Dir(*mut libc::DIR);

unsafe impl Send for Dir {}
unsafe impl Sync for Dir {}

// #[cfg(any(
//     target_os = "android",
//     target_os = "linux",
//     target_os = "solaris",
//     target_os = "illumos",
//     target_os = "fuchsia",
//     target_os = "redox",
//     target_os = "postgres",
// ))]
// pub struct DirEntry {
//     dir: Arc<InnerReadDir>,
//     entry: dirent64_min,
//     // We need to store an owned copy of the entry name on platforms that use
//     // readdir() (not readdir_r()), because a) struct dirent may use a flexible
//     // array to store the name, b) it lives only until the next readdir() call.
//     name: CString,
// }

// // Define a minimal subset of fields we need from `dirent64`, especially since
// // we're not using the immediate `d_name` on these targets. Keeping this as an
// // `entry` field in `DirEntry` helps reduce the `cfg` boilerplate elsewhere.
// #[cfg(any(
//     target_os = "android",
//     target_os = "linux",
//     target_os = "postgres",
//     target_os = "solaris",
//     target_os = "illumos",
//     target_os = "fuchsia",
//     target_os = "redox"
// ))]
// struct dirent64_min {
//     d_ino: u64,
//     #[cfg(not(any(target_os = "solaris", target_os = "illumos")))]
//     d_type: u8,
// }

// #[cfg(not(any(
//     target_os = "android",
//     target_os = "linux",
//     target_os = "solaris",
//     target_os = "illumos",
//     target_os = "fuchsia",
//     target_os = "redox",
//     target_os = "postgres",
// )))]
// pub struct DirEntry {
//     dir: Arc<InnerReadDir>,
//     // The full entry includes a fixed-length `d_name`.
//     entry: dirent64,
// }

#[derive(Clone, Debug, Default)]
pub struct OpenOptions {
    // generic
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
    // system-specific
    custom_flags: i32,
    mode: mode_t,
}

impl FromInner<u32> for FilePermissions {
    fn from_inner(mode: u32) -> FilePermissions {
        panic!("oh no! file permissions don't exist here!")
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        let r = unsafe { libc::closedir(self.0) };
        debug_assert_eq!(r, 0);
    }
}

fn cstr(path: &Path) -> io::Result<CString> {
    Ok(CString::new(path.as_os_str().as_bytes())?)
}

impl AsInner<FileDesc> for File {
    fn as_inner(&self) -> &FileDesc {
        &self.0
    }
}

impl IntoInner<!> for ! {
    fn into_inner(self) -> ! {
        match self {}
    }
}

impl AsInner<!> for ! {
    fn as_inner(&self) -> &! {
        match *self {}
    }
}

impl AsInnerMut<FileDesc> for File {
    fn as_inner_mut(&mut self) -> &mut FileDesc {
        &mut self.0
    }
}

impl IntoInner<FileDesc> for File {
    fn into_inner(self) -> FileDesc {
        self.0
    }
}

impl FromInner<FileDesc> for File {
    fn from_inner(file_desc: FileDesc) -> Self {
        Self(file_desc)
    }
}

impl AsFd for File {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl AsRawFd for File {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl IntoRawFd for File {
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}

impl FromRawFd for File {
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(FromRawFd::from_raw_fd(raw_fd))
    }
}

fn open_from(_from: &Path) -> io::Result<(crate::fs::File, crate::fs::Metadata)> {
    unsupported()
}

fn open_to_and_set_permissions(
    _to: &Path,
    _reader_metadata: crate::fs::Metadata,
) -> io::Result<(crate::fs::File, crate::fs::Metadata)> {
    unsupported()
}

pub fn chown(_path: &Path, _uid: u32, _gid: u32) -> io::Result<()> {
    unsupported()
}

pub fn fchown(_fd: c_int, _uid: u32, _gid: u32) -> io::Result<()> {
    unsupported()
}

pub fn lchown(_path: &Path, _uid: u32, _gid: u32) -> io::Result<()> {
    unsupported()
}

pub fn chroot(_dir: &Path) -> io::Result<()> {
    unsupported()
}
