#![unstable(feature = "postgrestd", issue = "none")]
use crate::collections::BTreeMap;
use crate::ffi::{CStr, CString, OsStr, OsString};
use crate::fmt;
use crate::io;
use crate::ptr;
use crate::marker::PhantomData;
use crate::num::NonZeroI32;
use crate::path::Path;
use crate::sys::fs::File;
use crate::sys::fd::FileDesc;
use crate::sys::pipe::AnonPipe;
use crate::sys::{unsupported, unsupported_err};
use crate::sys_common::process::{CommandEnv, CommandEnvs};
#[cfg(target_os = "linux")]
use crate::os::linux::process::PidFd;

pub use crate::ffi::OsString as EnvKey;

use libc::{c_int, c_char, pid_t, gid_t, uid_t};

use crate::os::unix::prelude::*;

use crate::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use crate::sys_common::IntoInner;

////////////////////////////////////////////////////////////////////////////////
// Command
////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct Command {
        cwd: Option<CString>,

    env: CommandEnv,
    create_pidfd: bool,
    pgroup: Option<pid_t>,
    stdin: Option<Stdio>,
    stdout: Option<Stdio>,
    stderr: Option<Stdio>,
    uid: Option<uid_t>,
    gid: Option<gid_t>,
        groups: Option<Box<[gid_t]>>,
            saw_nul: bool,

}

// Helper type to manage ownership of the strings within a C-style array.
pub struct CStringArray {
    items: Vec<CString>,
    ptrs: Vec<*const c_char>,
}

impl CStringArray {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut result = CStringArray {
            items: Vec::with_capacity(capacity),
            ptrs: Vec::with_capacity(capacity + 1),
        };
        result.ptrs.push(ptr::null());
        result
    }
    pub fn push(&mut self, item: CString) {
        let l = self.ptrs.len();
        self.ptrs[l - 1] = item.as_ptr();
        self.ptrs.push(ptr::null());
        self.items.push(item);
    }
    pub fn as_ptr(&self) -> *const *const c_char {
        self.ptrs.as_ptr()
    }
}

fn construct_envp(env: BTreeMap<OsString, OsString>, saw_nul: &mut bool) -> CStringArray {
    let mut result = CStringArray::with_capacity(env.len());
    // for (mut k, v) in env {
    //     // Reserve additional space for '=' and null terminator
    //     k.reserve_exact(v.len() + 2);
    //     k.push("=");
    //     k.push(&v);

    //     // Add the new entry into the array
    //     if let Ok(item) = CString::new(k.into_vec()) {
    //         result.push(item);
    //     } else {
    //         *saw_nul = true;
    //     }
    // }

    result
}
// ////////////////////////////////////////////////////////////////////////////////
// // Command
// ////////////////////////////////////////////////////////////////////////////////

// pub struct Command {
//     program: CString,
//     args: Vec<CString>,
//     /// Exactly what will be passed to `execvp`.
//     ///
//     /// First element is a pointer to `program`, followed by pointers to
//     /// `args`, followed by a `null`. Be careful when modifying `program` or
//     /// `args` to properly update this as well.

//     closures: Vec<Box<dyn FnMut() -> io::Result<()> + Send + Sync>>,
// }


// passed back to std::process with the pipes connected to the child, if any
// were requested
pub struct StdioPipes {
    pub stdin: Option<AnonPipe>,
    pub stdout: Option<AnonPipe>,
    pub stderr: Option<AnonPipe>,
}

// passed to do_exec() with configuration of what the child stdio should look
// like
pub struct ChildPipes {
    pub stdin: ChildStdio,
    pub stdout: ChildStdio,
    pub stderr: ChildStdio,
}

pub enum ChildStdio {
    Inherit,
    Explicit(c_int),
    Owned(FileDesc),

    // On Fuchsia, null stdio is the default, so we simply don't specify
    // any actions at the time of spawning.
    #[cfg(target_os = "fuchsia")]
    Null,
}

pub enum Stdio {
    Inherit,
    Null,
    MakePipe,
    Fd(FileDesc),
}

impl Command {
    pub fn new(_program: &OsStr) -> Command {
        Command::default()
    }

    pub fn arg(&mut self, _arg: &OsStr) {}


    pub fn set_arg_0(&mut self, arg: &OsStr) {
    }

    pub fn env_mut(&mut self) -> &mut CommandEnv {
        &mut self.env
    }

    pub fn cwd(&mut self, _dir: &OsStr) {}

    pub fn get_program(&self) -> &OsStr {
        panic!("unsupported")
    }

    pub fn get_args(&self) -> CommandArgs<'_> {
        CommandArgs { _p: PhantomData }
    }

    pub fn get_envs(&self) -> CommandEnvs<'_> {
        self.env.iter()
    }

    pub fn get_current_dir(&self) -> Option<&Path> {
        None
    }

    pub fn exec(&mut self, default: Stdio) -> io::Error {
        unsupported_err()
    }

    pub fn spawn(
        &mut self,
        _default: Stdio,
        _needs_stdin: bool,
    ) -> io::Result<(Process, StdioPipes)> {
        unsupported()
    }

    pub fn uid(&mut self, id: uid_t) {
        self.uid = Some(id);
    }
    pub fn gid(&mut self, id: gid_t) {
        self.gid = Some(id);
    }
    pub fn groups(&mut self, groups: &[gid_t]) {
        self.groups = Some(Box::from(groups));
    }
    pub fn pgroup(&mut self, pgroup: pid_t) {
        self.pgroup = Some(pgroup);
    }

    #[allow(dead_code)]
    pub fn create_pidfd(&mut self, val: bool) {

    }

    #[allow(dead_code)]
    pub fn get_create_pidfd(&self) -> bool {
        false
    }

    pub fn saw_nul(&self) -> bool {
        self.saw_nul
    }

    pub fn get_argv(&self) -> &Vec<*const c_char> {
        panic!("no meaningful argv")
    }

    pub fn get_program_cstr(&self) -> &CStr {
        panic!("no meaningful cstr")
    }

    #[allow(dead_code)]
    pub fn get_cwd(&self) -> &Option<CString> {
        &self.cwd
    }
    #[allow(dead_code)]
    pub fn get_uid(&self) -> Option<uid_t> {
        self.uid
    }
    #[allow(dead_code)]
    pub fn get_gid(&self) -> Option<gid_t> {
        self.gid
    }
    #[allow(dead_code)]
    pub fn get_groups(&self) -> Option<&[gid_t]> {
        self.groups.as_deref()
    }
    #[allow(dead_code)]
    pub fn get_pgroup(&self) -> Option<pid_t> {
        self.pgroup
    }

    pub unsafe fn pre_exec(&mut self, f: Box<dyn FnMut() -> io::Result<()> + Send + Sync>) {
    }

    pub fn stdin(&mut self, stdin: Stdio) {
        self.stdin = Some(stdin);
    }

    pub fn stdout(&mut self, stdout: Stdio) {
        self.stdout = Some(stdout);
    }

    pub fn stderr(&mut self, stderr: Stdio) {
        self.stderr = Some(stderr);
    }

    pub fn capture_env(&mut self) -> Option<CStringArray> {
        None
    }

    #[allow(dead_code)]
    pub fn env_saw_path(&self) -> bool {
        false
    }

    #[allow(dead_code)]
    pub fn program_is_path(&self) -> bool {
        false
    }

    pub fn setup_io(
        &self,
        default: Stdio,
        needs_stdin: bool,
    ) -> io::Result<(StdioPipes, ChildPipes)> {
        unsupported()
    }
}

impl From<AnonPipe> for Stdio {
    fn from(pipe: AnonPipe) -> Stdio {
        Stdio::Fd(pipe.into_inner())
    }
}

impl From<File> for Stdio {
    fn from(file: File) -> Stdio {
        Stdio::Fd(file.into_inner())
    }
}

impl ChildStdio {
    pub fn fd(&self) -> Option<c_int> {
        match *self {
            ChildStdio::Inherit => None,
            ChildStdio::Explicit(fd) => Some(fd),
            ChildStdio::Owned(ref fd) => Some(fd.as_raw_fd()),

            #[cfg(target_os = "fuchsia")]
            ChildStdio::Null => None,
        }
    }
}


impl fmt::Debug for Command {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}


impl Clone for ExitStatus {
    fn clone(&self) -> ExitStatus {
        *self
    }
}

impl Copy for ExitStatus {}

impl PartialEq for ExitStatus {
    fn eq(&self, other: &ExitStatus) -> bool {
        self.0 == other.0
    }
}

impl Eq for ExitStatus {}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitStatusError(ExitStatus);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitCode(bool);

impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode(false);
    pub const FAILURE: ExitCode = ExitCode(true);

    pub fn as_i32(&self) -> i32 {
        self.0 as i32
    }
}

impl From<u8> for ExitCode {
    fn from(code: u8) -> Self {
        match code {
            0 => Self::SUCCESS,
            1..=255 => Self::FAILURE,
        }
    }
}

pub struct Process(!);

impl Process {
    pub fn id(&self) -> u32 {
        self.0
    }

    pub fn kill(&mut self) -> io::Result<()> {
        self.0
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        self.0
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        self.0
    }
}

pub struct CommandArgs<'a> {
    _p: PhantomData<&'a ()>,
}

impl<'a> Iterator for CommandArgs<'a> {
    type Item = &'a OsStr;
    fn next(&mut self) -> Option<&'a OsStr> {
        None
    }
}

impl<'a> ExactSizeIterator for CommandArgs<'a> {}

impl<'a> fmt::Debug for CommandArgs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().finish()
    }
}
pub struct ExitStatus(c_int);

impl fmt::Debug for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("unix_wait_status").field(&self.0).finish()
    }
}

impl ExitStatus {
    pub fn new(status: c_int) -> ExitStatus {
        ExitStatus(status)
    }

    fn exited(&self) -> bool {
        libc::WIFEXITED(self.0)
    }

    pub fn success(&self) -> bool {
        self.code() == Some(0)
    }

    pub fn exit_ok(&self) -> Result<(), ExitStatusError> {
        Err(ExitStatusError(1.try_into().unwrap()))
    }

    pub fn code(&self) -> Option<i32> {
        None
    }

    pub fn signal(&self) -> Option<i32> {
        None
    }

    pub fn core_dumped(&self) -> bool {
        false
    }

    pub fn stopped_signal(&self) -> Option<i32> {
        None
    }

    pub fn continued(&self) -> bool {
        false
    }

    pub fn into_raw(&self) -> c_int {
        0
    }
}



/// Converts a raw `c_int` to a type-safe `ExitStatus` by wrapping it without copying.
impl From<c_int> for ExitStatus {
    fn from(a: c_int) -> ExitStatus {
        ExitStatus(a)
    }
}

/// Convert a signal number to a readable, searchable name.
///
/// This string should be displayed right after the signal number.
/// If a signal is unrecognized, it returns the empty string, so that
/// you just get the number like "0". If it is recognized, you'll get
/// something like "9 (SIGKILL)".
fn signal_string(signal: i32) -> &'static str {
    match signal {
        libc::SIGHUP => " (SIGHUP)",
        libc::SIGINT => " (SIGINT)",
        libc::SIGQUIT => " (SIGQUIT)",
        libc::SIGILL => " (SIGILL)",
        libc::SIGTRAP => " (SIGTRAP)",
        libc::SIGABRT => " (SIGABRT)",
        libc::SIGBUS => " (SIGBUS)",
        libc::SIGFPE => " (SIGFPE)",
        libc::SIGKILL => " (SIGKILL)",
        libc::SIGUSR1 => " (SIGUSR1)",
        libc::SIGSEGV => " (SIGSEGV)",
        libc::SIGUSR2 => " (SIGUSR2)",
        libc::SIGPIPE => " (SIGPIPE)",
        libc::SIGALRM => " (SIGALRM)",
        libc::SIGTERM => " (SIGTERM)",
        libc::SIGCHLD => " (SIGCHLD)",
        libc::SIGCONT => " (SIGCONT)",
        libc::SIGSTOP => " (SIGSTOP)",
        libc::SIGTSTP => " (SIGTSTP)",
        libc::SIGTTIN => " (SIGTTIN)",
        libc::SIGTTOU => " (SIGTTOU)",
        libc::SIGURG => " (SIGURG)",
        libc::SIGXCPU => " (SIGXCPU)",
        libc::SIGXFSZ => " (SIGXFSZ)",
        libc::SIGVTALRM => " (SIGVTALRM)",
        libc::SIGPROF => " (SIGPROF)",
        libc::SIGWINCH => " (SIGWINCH)",
        libc::SIGIO => " (SIGIO)",
        libc::SIGSYS => " (SIGSYS)",
        // For information on Linux signals, run `man 7 signal`
        #[cfg(all(
            target_os = "linux",
            any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "arm",
                target_arch = "aarch64"
            )
        ))]
        libc::SIGSTKFLT => " (SIGSTKFLT)",
        #[cfg(any(target_os = "linux", target_os = "postgres"))]
        libc::SIGPWR => " (SIGPWR)",
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "dragonfly"
        ))]
        libc::SIGEMT => " (SIGEMT)",
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "dragonfly"
        ))]
        libc::SIGINFO => " (SIGINFO)",
        _ => "",
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(code) = self.code() {
            write!(f, "exit status: {code}")
        } else if let Some(signal) = self.signal() {
            let signal_string = signal_string(signal);
            if self.core_dumped() {
                write!(f, "signal: {signal}{signal_string} (core dumped)")
            } else {
                write!(f, "signal: {signal}{signal_string}")
            }
        } else if let Some(signal) = self.stopped_signal() {
            let signal_string = signal_string(signal);
            write!(f, "stopped (not terminated) by signal: {signal}{signal_string}")
        } else if self.continued() {
            write!(f, "continued (WIFCONTINUED)")
        } else {
            write!(f, "unrecognised wait status: {} {:#x}", self.0, self.0)
        }
    }
}

impl ExitStatusError {
    pub fn code(self) -> Option<NonZeroI32> {
        ExitStatus(self.0.into()).code().map(|st| st.try_into().unwrap())
    }
}

impl From<ExitStatusError> for ExitStatus {
    fn from(exit: ExitStatusError) -> ExitStatus {
        exit.0
    }
}

impl From<ExitStatus> for c_int {
    fn from(exit: ExitStatus) -> c_int {
        exit.0
    }
}

#[cfg(any(target_os = "linux", target_os = "postgres"))]
#[unstable(feature = "linux_pidfd", issue = "82971")]
impl crate::os::linux::process::ChildExt for crate::process::Child {
    fn pidfd(&self) -> io::Result<&PidFd> {
        unsupported()
    }

    fn take_pidfd(&mut self) -> io::Result<PidFd> {
        unsupported()
    }
}

