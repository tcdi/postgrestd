use crate::time::Duration;
use crate::sys::cvt;
use crate::fmt;
use crate::mem::MaybeUninit;

const NSEC_PER_SEC: u64 = 1_000_000_000;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemTime {
    pub(in crate::sys) t: Timespec,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(in crate::sys) struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

pub const UNIX_EPOCH: SystemTime = SystemTime{ t: Timespec::zero() };

impl SystemTime {
    pub fn new(tv_sec: i64, tv_nsec: i64) -> SystemTime {
        SystemTime { t: Timespec::new(tv_sec, tv_nsec) }
    }

    pub fn sub_time(&self, other: &SystemTime) -> Result<Duration, Duration> {
        self.t.sub_timespec(&other.t)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime { t: self.t.checked_add_duration(other)? })
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime { t: self.t.checked_sub_duration(other)? })
    }
}

impl Timespec {
    pub const fn zero() -> Timespec {
        Timespec { tv_sec: 0, tv_nsec: 0 }
    }

    fn new(tv_sec: i64, tv_nsec: i64) -> Timespec {
        Timespec { tv_sec, tv_nsec }
    }

    pub fn sub_timespec(&self, other: &Timespec) -> Result<Duration, Duration> {
        if self >= other {
            // NOTE(eddyb) two aspects of this `if`-`else` are required for LLVM
            // to optimize it into a branchless form (see also #75545):
            //
            // 1. `self.tv_sec - other.tv_sec` shows up as a common expression
            //    in both branches, i.e. the `else` must have its `- 1`
            //    subtraction after the common one, not interleaved with it
            //    (it used to be `self.tv_sec - 1 - other.tv_sec`)
            //
            // 2. the `Duration::new` call (or any other additional complexity)
            //    is outside of the `if`-`else`, not duplicated in both branches
            //
            // Ideally this code could be rearranged such that it more
            // directly expresses the lower-cost behavior we want from it.
            let (secs, nsec) = if self.tv_nsec >= other.tv_nsec {
                ((self.tv_sec - other.tv_sec) as u64, (self.tv_nsec - other.tv_nsec) as u32)
            } else {
                (
                    (self.tv_sec - other.tv_sec - 1) as u64,
                    self.tv_nsec as u32 + (NSEC_PER_SEC as u32) - other.tv_nsec as u32,
                )
            };

            Ok(Duration::new(secs, nsec))
        } else {
            match other.sub_timespec(self) {
                Ok(d) => Err(d),
                Err(d) => Ok(d),
            }
        }
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<Timespec> {
        let mut secs = other
            .as_secs()
            .try_into() // <- target type would be `i64`
            .ok()
            .and_then(|secs| self.tv_sec.checked_add(secs))?;

        // Nano calculations can't overflow because nanos are <1B which fit
        // in a u32.
        let mut nsec = other.subsec_nanos() + self.tv_nsec as u32;
        if nsec >= NSEC_PER_SEC as u32 {
            nsec -= NSEC_PER_SEC as u32;
            secs = secs.checked_add(1)?;
        }
        Some(Timespec::new(secs, nsec as i64))
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<Timespec> {
        let mut secs = other
            .as_secs()
            .try_into() // <- target type would be `i64`
            .ok()
            .and_then(|secs| self.tv_sec.checked_sub(secs))?;

        // Similar to above, nanos can't overflow.
        let mut nsec = self.tv_nsec as i32 - other.subsec_nanos() as i32;
        if nsec < 0 {
            nsec += NSEC_PER_SEC as i32;
            secs = secs.checked_sub(1)?;
        }
        Some(Timespec::new(secs, nsec as i64))
    }

    #[allow(dead_code)]
    pub fn to_timespec(&self) -> Option<libc::timespec> {
        Some(libc::timespec {
            tv_sec: self.tv_sec.try_into().ok()?,
            tv_nsec: self.tv_nsec.try_into().ok()?,
        })
    }
}

impl From<libc::timespec> for Timespec {
    fn from(t: libc::timespec) -> Timespec {
        Timespec::new(t.tv_sec as i64, t.tv_nsec as i64)
    }
}

impl fmt::Debug for SystemTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SystemTime")
            .field("tv_sec", &self.t.tv_sec)
            .field("tv_nsec", &self.t.tv_nsec)
            .finish()
    }
}



#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    t: Timespec,
}

impl Instant {
    pub fn now() -> Instant {
        Instant { t: Timespec::now(libc::CLOCK_MONOTONIC) }
    }

    pub fn checked_sub_instant(&self, other: &Instant) -> Option<Duration> {
        self.t.sub_timespec(&other.t).ok()
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<Instant> {
        Some(Instant { t: self.t.checked_add_duration(other)? })
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<Instant> {
        Some(Instant { t: self.t.checked_sub_duration(other)? })
    }
}

impl fmt::Debug for Instant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instant")
            .field("tv_sec", &self.t.tv_sec)
            .field("tv_nsec", &self.t.tv_nsec)
            .finish()
    }
}

impl SystemTime {
    pub fn now() -> SystemTime {
        panic!("not yet bound against postgres for time")
    }
}

#[cfg(not(any(target_os = "dragonfly", target_os = "espidf")))]
#[allow(non_camel_case_types)]
pub type clock_t = libc::c_int;
#[cfg(any(target_os = "dragonfly", target_os = "espidf"))]
pub type clock_t = libc::c_ulong;

impl Timespec {
    pub fn now(clock: clock_t) -> Timespec {
        // Try to use 64-bit time in preparation for Y2038.
        #[cfg(all(target_os = "linux", target_env = "gnu", target_pointer_width = "32"))]
        {
            use crate::sys::weak::weak;

            // __clock_gettime64 was added to 32-bit arches in glibc 2.34,
            // and it handles both vDSO calls and ENOSYS fallbacks itself.
            weak!(fn __clock_gettime64(libc::clockid_t, *mut __timespec64) -> libc::c_int);

            #[repr(C)]
            struct __timespec64 {
                tv_sec: i64,
                #[cfg(target_endian = "big")]
                _padding: i32,
                tv_nsec: i32,
                #[cfg(target_endian = "little")]
                _padding: i32,
            }

            if let Some(clock_gettime64) = __clock_gettime64.get() {
                let mut t = MaybeUninit::uninit();
                cvt(unsafe { clock_gettime64(clock, t.as_mut_ptr()) }).unwrap();
                let t = unsafe { t.assume_init() };
                return Timespec { tv_sec: t.tv_sec, tv_nsec: t.tv_nsec as i64 };
            }
        }

        let mut t = MaybeUninit::uninit();
        cvt(unsafe { libc::clock_gettime(clock, t.as_mut_ptr()) }).unwrap();
        Timespec::from(unsafe { t.assume_init() })
    }
}