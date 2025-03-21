/// Helper for generating support code for a list of syscalls.
macro_rules! syscall_enum {
    (
        $(#[$outer:meta])*
        $vis:vis enum $Name:ident {
            $(#[$first_inner:meta])*
            $first_syscall:ident = $first_num:expr,
            $(
                $(#[$inner:meta])*
                $syscall:ident = $num:expr,
            )*
        }

        LAST: $last_syscall:ident;
    ) => {
        /// Complete list of Linux syscalls.
        $(#[$outer])*
        #[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
        #[derive(Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
        #[repr(u64)]
        #[non_exhaustive]
        $vis enum $Name {
            $(#[$first_inner])*
            $first_syscall = $first_num,
            $(
                $(#[$inner])*
                $syscall = $num,
            )*
        }

        impl $Name {
            /// Constructs a new syscall from the given ID. If the ID does not
            /// represent a valid syscall, returns `None`.
            pub const fn new(id: usize) -> Option<Self> {
                // TODO: Get rid of this huge match and use the SysnoSet for
                // checking validity.
                match id {
                    $first_num => Some(Self::$first_syscall),
                    $(
                        $num => Some(Self::$syscall),
                    )*
                    _ => None,
                }
            }

            /// Returns the name of the syscall.
            pub const fn name(&self) -> &'static str {
                match self {
                    Self::$first_syscall => core::stringify!($first_syscall),
                    $(
                        Self::$syscall => core::stringify!($syscall),
                    )*
                }
            }

            /// Returns the next syscall in the table. Returns `None` if this is
            /// the last syscall.
            pub const fn next(&self) -> Option<Self> {
                if let Self::$last_syscall = self {
                    return None;
                }

                let mut next_id = self.id() + 1;

                while next_id < Self::last().id() {
                    if let Some(next) = Self::new(next_id as usize) {
                        return Some(next);
                    }

                    next_id += 1;
                }

                None
            }

            /// Returns the first syscall in the table.
            pub const fn first() -> Self {
                Self::$first_syscall
            }

            /// Returns the last syscall in the table.
            pub const fn last() -> Self {
                Self::$last_syscall
            }

            /// Returns the syscall number.
            pub const fn id(&self) -> i32 {
                *self as i32
            }

            /// Returns the length of the syscall table, including any gaps.
            #[deprecated = "Sysno::len() is misleading. Use Sysno::table_size() instead."]
            pub const fn len() -> usize {
                Self::table_size()
            }

            /// Returns the length of the syscall table, including any gaps.
            /// This is not the same thing as the total number of syscalls.
            pub const fn table_size() -> usize {
                (Self::last().id() - Self::first().id()) as usize + 1
            }

            /// Returns an iterator that iterates over all possible syscalls.
            pub fn iter() -> impl Iterator<Item = Self> {
                core::iter::successors(Some(Self::first()), |x| x.next())
            }
        }

        impl core::str::FromStr for $Name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    core::stringify!($first_syscall) => Ok(Self::$first_syscall),
                    $(
                        core::stringify!($syscall) => Ok(Self::$syscall),
                    )*
                    _ => Err(()),
                }
            }
        }

        impl core::fmt::Display for $Name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str(self.name())
            }
        }

        impl core::fmt::Debug for $Name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str(self.name())
            }
        }

        impl From<i64> for $Name {
            fn from(id: i64) -> Self {
                Self::new(id as usize)
                    .unwrap_or_else(|| panic!("invalid syscall: {}", id))
            }
        }

        impl From<u64> for $Name {
            fn from(id: u64) -> Self {
                Self::new(id as usize)
                    .unwrap_or_else(|| panic!("invalid syscall: {}", id))
            }
        }
    }
}

syscall_enum! {
    pub enum Sysno {
        /// See [io_setup(2)](https://man7.org/linux/man-pages/man2/io_setup.2.html) for more info on this syscall.
        io_setup = 0,
        /// See [io_destroy(2)](https://man7.org/linux/man-pages/man2/io_destroy.2.html) for more info on this syscall.
        io_destroy = 1,
        /// See [io_submit(2)](https://man7.org/linux/man-pages/man2/io_submit.2.html) for more info on this syscall.
        io_submit = 2,
        /// See [io_cancel(2)](https://man7.org/linux/man-pages/man2/io_cancel.2.html) for more info on this syscall.
        io_cancel = 3,
        /// See [io_getevents(2)](https://man7.org/linux/man-pages/man2/io_getevents.2.html) for more info on this syscall.
        io_getevents = 4,
        /// See [setxattr(2)](https://man7.org/linux/man-pages/man2/setxattr.2.html) for more info on this syscall.
        setxattr = 5,
        /// See [lsetxattr(2)](https://man7.org/linux/man-pages/man2/lsetxattr.2.html) for more info on this syscall.
        lsetxattr = 6,
        /// See [fsetxattr(2)](https://man7.org/linux/man-pages/man2/fsetxattr.2.html) for more info on this syscall.
        fsetxattr = 7,
        /// See [getxattr(2)](https://man7.org/linux/man-pages/man2/getxattr.2.html) for more info on this syscall.
        getxattr = 8,
        /// See [lgetxattr(2)](https://man7.org/linux/man-pages/man2/lgetxattr.2.html) for more info on this syscall.
        lgetxattr = 9,
        /// See [fgetxattr(2)](https://man7.org/linux/man-pages/man2/fgetxattr.2.html) for more info on this syscall.
        fgetxattr = 10,
        /// See [listxattr(2)](https://man7.org/linux/man-pages/man2/listxattr.2.html) for more info on this syscall.
        listxattr = 11,
        /// See [llistxattr(2)](https://man7.org/linux/man-pages/man2/llistxattr.2.html) for more info on this syscall.
        llistxattr = 12,
        /// See [flistxattr(2)](https://man7.org/linux/man-pages/man2/flistxattr.2.html) for more info on this syscall.
        flistxattr = 13,
        /// See [removexattr(2)](https://man7.org/linux/man-pages/man2/removexattr.2.html) for more info on this syscall.
        removexattr = 14,
        /// See [lremovexattr(2)](https://man7.org/linux/man-pages/man2/lremovexattr.2.html) for more info on this syscall.
        lremovexattr = 15,
        /// See [fremovexattr(2)](https://man7.org/linux/man-pages/man2/fremovexattr.2.html) for more info on this syscall.
        fremovexattr = 16,
        /// See [getcwd(2)](https://man7.org/linux/man-pages/man2/getcwd.2.html) for more info on this syscall.
        getcwd = 17,
        /// See [lookup_dcookie(2)](https://man7.org/linux/man-pages/man2/lookup_dcookie.2.html) for more info on this syscall.
        lookup_dcookie = 18,
        /// See [eventfd2(2)](https://man7.org/linux/man-pages/man2/eventfd2.2.html) for more info on this syscall.
        eventfd2 = 19,
        /// See [epoll_create1(2)](https://man7.org/linux/man-pages/man2/epoll_create1.2.html) for more info on this syscall.
        epoll_create1 = 20,
        /// See [epoll_ctl(2)](https://man7.org/linux/man-pages/man2/epoll_ctl.2.html) for more info on this syscall.
        epoll_ctl = 21,
        /// See [epoll_pwait(2)](https://man7.org/linux/man-pages/man2/epoll_pwait.2.html) for more info on this syscall.
        epoll_pwait = 22,
        /// See [dup(2)](https://man7.org/linux/man-pages/man2/dup.2.html) for more info on this syscall.
        dup = 23,
        /// See [dup3(2)](https://man7.org/linux/man-pages/man2/dup3.2.html) for more info on this syscall.
        dup3 = 24,
        /// See [fcntl(2)](https://man7.org/linux/man-pages/man2/fcntl.2.html) for more info on this syscall.
        fcntl = 25,
        /// See [inotify_init1(2)](https://man7.org/linux/man-pages/man2/inotify_init1.2.html) for more info on this syscall.
        inotify_init1 = 26,
        /// See [inotify_add_watch(2)](https://man7.org/linux/man-pages/man2/inotify_add_watch.2.html) for more info on this syscall.
        inotify_add_watch = 27,
        /// See [inotify_rm_watch(2)](https://man7.org/linux/man-pages/man2/inotify_rm_watch.2.html) for more info on this syscall.
        inotify_rm_watch = 28,
        /// See [ioctl(2)](https://man7.org/linux/man-pages/man2/ioctl.2.html) for more info on this syscall.
        ioctl = 29,
        /// See [ioprio_set(2)](https://man7.org/linux/man-pages/man2/ioprio_set.2.html) for more info on this syscall.
        ioprio_set = 30,
        /// See [ioprio_get(2)](https://man7.org/linux/man-pages/man2/ioprio_get.2.html) for more info on this syscall.
        ioprio_get = 31,
        /// See [flock(2)](https://man7.org/linux/man-pages/man2/flock.2.html) for more info on this syscall.
        flock = 32,
        /// See [mknodat(2)](https://man7.org/linux/man-pages/man2/mknodat.2.html) for more info on this syscall.
        mknodat = 33,
        /// See [mkdirat(2)](https://man7.org/linux/man-pages/man2/mkdirat.2.html) for more info on this syscall.
        mkdirat = 34,
        /// See [unlinkat(2)](https://man7.org/linux/man-pages/man2/unlinkat.2.html) for more info on this syscall.
        unlinkat = 35,
        /// See [symlinkat(2)](https://man7.org/linux/man-pages/man2/symlinkat.2.html) for more info on this syscall.
        symlinkat = 36,
        /// See [linkat(2)](https://man7.org/linux/man-pages/man2/linkat.2.html) for more info on this syscall.
        linkat = 37,
        /// See [renameat(2)](https://man7.org/linux/man-pages/man2/renameat.2.html) for more info on this syscall.
        renameat = 38,
        /// See [umount2(2)](https://man7.org/linux/man-pages/man2/umount2.2.html) for more info on this syscall.
        umount2 = 39,
        /// See [mount(2)](https://man7.org/linux/man-pages/man2/mount.2.html) for more info on this syscall.
        mount = 40,
        /// See [pivot_root(2)](https://man7.org/linux/man-pages/man2/pivot_root.2.html) for more info on this syscall.
        pivot_root = 41,
        /// See [nfsservctl(2)](https://man7.org/linux/man-pages/man2/nfsservctl.2.html) for more info on this syscall.
        nfsservctl = 42,
        /// See [statfs(2)](https://man7.org/linux/man-pages/man2/statfs.2.html) for more info on this syscall.
        statfs = 43,
        /// See [fstatfs(2)](https://man7.org/linux/man-pages/man2/fstatfs.2.html) for more info on this syscall.
        fstatfs = 44,
        /// See [truncate(2)](https://man7.org/linux/man-pages/man2/truncate.2.html) for more info on this syscall.
        truncate = 45,
        /// See [ftruncate(2)](https://man7.org/linux/man-pages/man2/ftruncate.2.html) for more info on this syscall.
        ftruncate = 46,
        /// See [fallocate(2)](https://man7.org/linux/man-pages/man2/fallocate.2.html) for more info on this syscall.
        fallocate = 47,
        /// See [faccessat(2)](https://man7.org/linux/man-pages/man2/faccessat.2.html) for more info on this syscall.
        faccessat = 48,
        /// See [chdir(2)](https://man7.org/linux/man-pages/man2/chdir.2.html) for more info on this syscall.
        chdir = 49,
        /// See [fchdir(2)](https://man7.org/linux/man-pages/man2/fchdir.2.html) for more info on this syscall.
        fchdir = 50,
        /// See [chroot(2)](https://man7.org/linux/man-pages/man2/chroot.2.html) for more info on this syscall.
        chroot = 51,
        /// See [fchmod(2)](https://man7.org/linux/man-pages/man2/fchmod.2.html) for more info on this syscall.
        fchmod = 52,
        /// See [fchmodat(2)](https://man7.org/linux/man-pages/man2/fchmodat.2.html) for more info on this syscall.
        fchmodat = 53,
        /// See [fchownat(2)](https://man7.org/linux/man-pages/man2/fchownat.2.html) for more info on this syscall.
        fchownat = 54,
        /// See [fchown(2)](https://man7.org/linux/man-pages/man2/fchown.2.html) for more info on this syscall.
        fchown = 55,
        /// See [openat(2)](https://man7.org/linux/man-pages/man2/openat.2.html) for more info on this syscall.
        openat = 56,
        /// See [close(2)](https://man7.org/linux/man-pages/man2/close.2.html) for more info on this syscall.
        close = 57,
        /// See [vhangup(2)](https://man7.org/linux/man-pages/man2/vhangup.2.html) for more info on this syscall.
        vhangup = 58,
        /// See [pipe2(2)](https://man7.org/linux/man-pages/man2/pipe2.2.html) for more info on this syscall.
        pipe2 = 59,
        /// See [quotactl(2)](https://man7.org/linux/man-pages/man2/quotactl.2.html) for more info on this syscall.
        quotactl = 60,
        /// See [getdents64(2)](https://man7.org/linux/man-pages/man2/getdents64.2.html) for more info on this syscall.
        getdents64 = 61,
        /// See [lseek(2)](https://man7.org/linux/man-pages/man2/lseek.2.html) for more info on this syscall.
        lseek = 62,
        /// See [read(2)](https://man7.org/linux/man-pages/man2/read.2.html) for more info on this syscall.
        read = 63,
        /// See [write(2)](https://man7.org/linux/man-pages/man2/write.2.html) for more info on this syscall.
        write = 64,
        /// See [readv(2)](https://man7.org/linux/man-pages/man2/readv.2.html) for more info on this syscall.
        readv = 65,
        /// See [writev(2)](https://man7.org/linux/man-pages/man2/writev.2.html) for more info on this syscall.
        writev = 66,
        /// See [pread64(2)](https://man7.org/linux/man-pages/man2/pread64.2.html) for more info on this syscall.
        pread64 = 67,
        /// See [pwrite64(2)](https://man7.org/linux/man-pages/man2/pwrite64.2.html) for more info on this syscall.
        pwrite64 = 68,
        /// See [preadv(2)](https://man7.org/linux/man-pages/man2/preadv.2.html) for more info on this syscall.
        preadv = 69,
        /// See [pwritev(2)](https://man7.org/linux/man-pages/man2/pwritev.2.html) for more info on this syscall.
        pwritev = 70,
        /// See [sendfile(2)](https://man7.org/linux/man-pages/man2/sendfile.2.html) for more info on this syscall.
        sendfile = 71,
        /// See [pselect6(2)](https://man7.org/linux/man-pages/man2/pselect6.2.html) for more info on this syscall.
        pselect6 = 72,
        /// See [ppoll(2)](https://man7.org/linux/man-pages/man2/ppoll.2.html) for more info on this syscall.
        ppoll = 73,
        /// See [signalfd4(2)](https://man7.org/linux/man-pages/man2/signalfd4.2.html) for more info on this syscall.
        signalfd4 = 74,
        /// See [vmsplice(2)](https://man7.org/linux/man-pages/man2/vmsplice.2.html) for more info on this syscall.
        vmsplice = 75,
        /// See [splice(2)](https://man7.org/linux/man-pages/man2/splice.2.html) for more info on this syscall.
        splice = 76,
        /// See [tee(2)](https://man7.org/linux/man-pages/man2/tee.2.html) for more info on this syscall.
        tee = 77,
        /// See [readlinkat(2)](https://man7.org/linux/man-pages/man2/readlinkat.2.html) for more info on this syscall.
        readlinkat = 78,
        /// See [fstatat(2)](https://man7.org/linux/man-pages/man2/fstatat.2.html) for more info on this syscall.
        fstatat = 79,
        /// See [fstat(2)](https://man7.org/linux/man-pages/man2/fstat.2.html) for more info on this syscall.
        fstat = 80,
        /// See [sync(2)](https://man7.org/linux/man-pages/man2/sync.2.html) for more info on this syscall.
        sync = 81,
        /// See [fsync(2)](https://man7.org/linux/man-pages/man2/fsync.2.html) for more info on this syscall.
        fsync = 82,
        /// See [fdatasync(2)](https://man7.org/linux/man-pages/man2/fdatasync.2.html) for more info on this syscall.
        fdatasync = 83,
        /// See [sync_file_range2(2)](https://man7.org/linux/man-pages/man2/sync_file_range2.2.html) for more info on this syscall.
        sync_file_range2 = 84,
        /// See [timerfd_create(2)](https://man7.org/linux/man-pages/man2/timerfd_create.2.html) for more info on this syscall.
        timerfd_create = 85,
        /// See [timerfd_settime(2)](https://man7.org/linux/man-pages/man2/timerfd_settime.2.html) for more info on this syscall.
        timerfd_settime = 86,
        /// See [timerfd_gettime(2)](https://man7.org/linux/man-pages/man2/timerfd_gettime.2.html) for more info on this syscall.
        timerfd_gettime = 87,
        /// See [utimensat(2)](https://man7.org/linux/man-pages/man2/utimensat.2.html) for more info on this syscall.
        utimensat = 88,
        /// See [acct(2)](https://man7.org/linux/man-pages/man2/acct.2.html) for more info on this syscall.
        acct = 89,
        /// See [capget(2)](https://man7.org/linux/man-pages/man2/capget.2.html) for more info on this syscall.
        capget = 90,
        /// See [capset(2)](https://man7.org/linux/man-pages/man2/capset.2.html) for more info on this syscall.
        capset = 91,
        /// See [personality(2)](https://man7.org/linux/man-pages/man2/personality.2.html) for more info on this syscall.
        personality = 92,
        /// See [exit(2)](https://man7.org/linux/man-pages/man2/exit.2.html) for more info on this syscall.
        exit = 93,
        /// See [exit_group(2)](https://man7.org/linux/man-pages/man2/exit_group.2.html) for more info on this syscall.
        exit_group = 94,
        /// See [waitid(2)](https://man7.org/linux/man-pages/man2/waitid.2.html) for more info on this syscall.
        waitid = 95,
        /// See [set_tid_address(2)](https://man7.org/linux/man-pages/man2/set_tid_address.2.html) for more info on this syscall.
        set_tid_address = 96,
        /// See [unshare(2)](https://man7.org/linux/man-pages/man2/unshare.2.html) for more info on this syscall.
        unshare = 97,
        /// See [futex(2)](https://man7.org/linux/man-pages/man2/futex.2.html) for more info on this syscall.
        futex = 98,
        /// See [set_robust_list(2)](https://man7.org/linux/man-pages/man2/set_robust_list.2.html) for more info on this syscall.
        set_robust_list = 99,
        /// See [get_robust_list(2)](https://man7.org/linux/man-pages/man2/get_robust_list.2.html) for more info on this syscall.
        get_robust_list = 100,
        /// See [nanosleep(2)](https://man7.org/linux/man-pages/man2/nanosleep.2.html) for more info on this syscall.
        nanosleep = 101,
        /// See [getitimer(2)](https://man7.org/linux/man-pages/man2/getitimer.2.html) for more info on this syscall.
        getitimer = 102,
        /// See [setitimer(2)](https://man7.org/linux/man-pages/man2/setitimer.2.html) for more info on this syscall.
        setitimer = 103,
        /// See [kexec_load(2)](https://man7.org/linux/man-pages/man2/kexec_load.2.html) for more info on this syscall.
        kexec_load = 104,
        /// See [init_module(2)](https://man7.org/linux/man-pages/man2/init_module.2.html) for more info on this syscall.
        init_module = 105,
        /// See [delete_module(2)](https://man7.org/linux/man-pages/man2/delete_module.2.html) for more info on this syscall.
        delete_module = 106,
        /// See [timer_create(2)](https://man7.org/linux/man-pages/man2/timer_create.2.html) for more info on this syscall.
        timer_create = 107,
        /// See [timer_gettime(2)](https://man7.org/linux/man-pages/man2/timer_gettime.2.html) for more info on this syscall.
        timer_gettime = 108,
        /// See [timer_getoverrun(2)](https://man7.org/linux/man-pages/man2/timer_getoverrun.2.html) for more info on this syscall.
        timer_getoverrun = 109,
        /// See [timer_settime(2)](https://man7.org/linux/man-pages/man2/timer_settime.2.html) for more info on this syscall.
        timer_settime = 110,
        /// See [timer_delete(2)](https://man7.org/linux/man-pages/man2/timer_delete.2.html) for more info on this syscall.
        timer_delete = 111,
        /// See [clock_settime(2)](https://man7.org/linux/man-pages/man2/clock_settime.2.html) for more info on this syscall.
        clock_settime = 112,
        /// See [clock_gettime(2)](https://man7.org/linux/man-pages/man2/clock_gettime.2.html) for more info on this syscall.
        clock_gettime = 113,
        /// See [clock_getres(2)](https://man7.org/linux/man-pages/man2/clock_getres.2.html) for more info on this syscall.
        clock_getres = 114,
        /// See [clock_nanosleep(2)](https://man7.org/linux/man-pages/man2/clock_nanosleep.2.html) for more info on this syscall.
        clock_nanosleep = 115,
        /// See [syslog(2)](https://man7.org/linux/man-pages/man2/syslog.2.html) for more info on this syscall.
        syslog = 116,
        /// See [ptrace(2)](https://man7.org/linux/man-pages/man2/ptrace.2.html) for more info on this syscall.
        ptrace = 117,
        /// See [sched_setparam(2)](https://man7.org/linux/man-pages/man2/sched_setparam.2.html) for more info on this syscall.
        sched_setparam = 118,
        /// See [sched_setscheduler(2)](https://man7.org/linux/man-pages/man2/sched_setscheduler.2.html) for more info on this syscall.
        sched_setscheduler = 119,
        /// See [sched_getscheduler(2)](https://man7.org/linux/man-pages/man2/sched_getscheduler.2.html) for more info on this syscall.
        sched_getscheduler = 120,
        /// See [sched_getparam(2)](https://man7.org/linux/man-pages/man2/sched_getparam.2.html) for more info on this syscall.
        sched_getparam = 121,
        /// See [sched_setaffinity(2)](https://man7.org/linux/man-pages/man2/sched_setaffinity.2.html) for more info on this syscall.
        sched_setaffinity = 122,
        /// See [sched_getaffinity(2)](https://man7.org/linux/man-pages/man2/sched_getaffinity.2.html) for more info on this syscall.
        sched_getaffinity = 123,
        /// See [sched_yield(2)](https://man7.org/linux/man-pages/man2/sched_yield.2.html) for more info on this syscall.
        sched_yield = 124,
        /// See [sched_get_priority_max(2)](https://man7.org/linux/man-pages/man2/sched_get_priority_max.2.html) for more info on this syscall.
        sched_get_priority_max = 125,
        /// See [sched_get_priority_min(2)](https://man7.org/linux/man-pages/man2/sched_get_priority_min.2.html) for more info on this syscall.
        sched_get_priority_min = 126,
        /// See [sched_rr_get_interval(2)](https://man7.org/linux/man-pages/man2/sched_rr_get_interval.2.html) for more info on this syscall.
        sched_rr_get_interval = 127,
        /// See [restart_syscall(2)](https://man7.org/linux/man-pages/man2/restart_syscall.2.html) for more info on this syscall.
        restart_syscall = 128,
        /// See [kill(2)](https://man7.org/linux/man-pages/man2/kill.2.html) for more info on this syscall.
        kill = 129,
        /// See [tkill(2)](https://man7.org/linux/man-pages/man2/tkill.2.html) for more info on this syscall.
        tkill = 130,
        /// See [tgkill(2)](https://man7.org/linux/man-pages/man2/tgkill.2.html) for more info on this syscall.
        tgkill = 131,
        /// See [sigaltstack(2)](https://man7.org/linux/man-pages/man2/sigaltstack.2.html) for more info on this syscall.
        sigaltstack = 132,
        /// See [rt_sigsuspend(2)](https://man7.org/linux/man-pages/man2/rt_sigsuspend.2.html) for more info on this syscall.
        rt_sigsuspend = 133,
        /// See [rt_sigaction(2)](https://man7.org/linux/man-pages/man2/rt_sigaction.2.html) for more info on this syscall.
        rt_sigaction = 134,
        /// See [rt_sigprocmask(2)](https://man7.org/linux/man-pages/man2/rt_sigprocmask.2.html) for more info on this syscall.
        rt_sigprocmask = 135,
        /// See [rt_sigpending(2)](https://man7.org/linux/man-pages/man2/rt_sigpending.2.html) for more info on this syscall.
        rt_sigpending = 136,
        /// See [rt_sigtimedwait(2)](https://man7.org/linux/man-pages/man2/rt_sigtimedwait.2.html) for more info on this syscall.
        rt_sigtimedwait = 137,
        /// See [rt_sigqueueinfo(2)](https://man7.org/linux/man-pages/man2/rt_sigqueueinfo.2.html) for more info on this syscall.
        rt_sigqueueinfo = 138,
        /// See [rt_sigreturn(2)](https://man7.org/linux/man-pages/man2/rt_sigreturn.2.html) for more info on this syscall.
        rt_sigreturn = 139,
        /// See [setpriority(2)](https://man7.org/linux/man-pages/man2/setpriority.2.html) for more info on this syscall.
        setpriority = 140,
        /// See [getpriority(2)](https://man7.org/linux/man-pages/man2/getpriority.2.html) for more info on this syscall.
        getpriority = 141,
        /// See [reboot(2)](https://man7.org/linux/man-pages/man2/reboot.2.html) for more info on this syscall.
        reboot = 142,
        /// See [setregid(2)](https://man7.org/linux/man-pages/man2/setregid.2.html) for more info on this syscall.
        setregid = 143,
        /// See [setgid(2)](https://man7.org/linux/man-pages/man2/setgid.2.html) for more info on this syscall.
        setgid = 144,
        /// See [setreuid(2)](https://man7.org/linux/man-pages/man2/setreuid.2.html) for more info on this syscall.
        setreuid = 145,
        /// See [setuid(2)](https://man7.org/linux/man-pages/man2/setuid.2.html) for more info on this syscall.
        setuid = 146,
        /// See [setresuid(2)](https://man7.org/linux/man-pages/man2/setresuid.2.html) for more info on this syscall.
        setresuid = 147,
        /// See [getresuid(2)](https://man7.org/linux/man-pages/man2/getresuid.2.html) for more info on this syscall.
        getresuid = 148,
        /// See [setresgid(2)](https://man7.org/linux/man-pages/man2/setresgid.2.html) for more info on this syscall.
        setresgid = 149,
        /// See [getresgid(2)](https://man7.org/linux/man-pages/man2/getresgid.2.html) for more info on this syscall.
        getresgid = 150,
        /// See [setfsuid(2)](https://man7.org/linux/man-pages/man2/setfsuid.2.html) for more info on this syscall.
        setfsuid = 151,
        /// See [setfsgid(2)](https://man7.org/linux/man-pages/man2/setfsgid.2.html) for more info on this syscall.
        setfsgid = 152,
        /// See [times(2)](https://man7.org/linux/man-pages/man2/times.2.html) for more info on this syscall.
        times = 153,
        /// See [setpgid(2)](https://man7.org/linux/man-pages/man2/setpgid.2.html) for more info on this syscall.
        setpgid = 154,
        /// See [getpgid(2)](https://man7.org/linux/man-pages/man2/getpgid.2.html) for more info on this syscall.
        getpgid = 155,
        /// See [getsid(2)](https://man7.org/linux/man-pages/man2/getsid.2.html) for more info on this syscall.
        getsid = 156,
        /// See [setsid(2)](https://man7.org/linux/man-pages/man2/setsid.2.html) for more info on this syscall.
        setsid = 157,
        /// See [getgroups(2)](https://man7.org/linux/man-pages/man2/getgroups.2.html) for more info on this syscall.
        getgroups = 158,
        /// See [setgroups(2)](https://man7.org/linux/man-pages/man2/setgroups.2.html) for more info on this syscall.
        setgroups = 159,
        /// See [uname(2)](https://man7.org/linux/man-pages/man2/uname.2.html) for more info on this syscall.
        uname = 160,
        /// See [sethostname(2)](https://man7.org/linux/man-pages/man2/sethostname.2.html) for more info on this syscall.
        sethostname = 161,
        /// See [setdomainname(2)](https://man7.org/linux/man-pages/man2/setdomainname.2.html) for more info on this syscall.
        setdomainname = 162,
        /// See [getrlimit(2)](https://man7.org/linux/man-pages/man2/getrlimit.2.html) for more info on this syscall.
        getrlimit = 163,
        /// See [setrlimit(2)](https://man7.org/linux/man-pages/man2/setrlimit.2.html) for more info on this syscall.
        setrlimit = 164,
        /// See [getrusage(2)](https://man7.org/linux/man-pages/man2/getrusage.2.html) for more info on this syscall.
        getrusage = 165,
        /// See [umask(2)](https://man7.org/linux/man-pages/man2/umask.2.html) for more info on this syscall.
        umask = 166,
        /// See [prctl(2)](https://man7.org/linux/man-pages/man2/prctl.2.html) for more info on this syscall.
        prctl = 167,
        /// See [getcpu(2)](https://man7.org/linux/man-pages/man2/getcpu.2.html) for more info on this syscall.
        getcpu = 168,
        /// See [gettimeofday(2)](https://man7.org/linux/man-pages/man2/gettimeofday.2.html) for more info on this syscall.
        gettimeofday = 169,
        /// See [settimeofday(2)](https://man7.org/linux/man-pages/man2/settimeofday.2.html) for more info on this syscall.
        settimeofday = 170,
        /// See [adjtimex(2)](https://man7.org/linux/man-pages/man2/adjtimex.2.html) for more info on this syscall.
        adjtimex = 171,
        /// See [getpid(2)](https://man7.org/linux/man-pages/man2/getpid.2.html) for more info on this syscall.
        getpid = 172,
        /// See [getppid(2)](https://man7.org/linux/man-pages/man2/getppid.2.html) for more info on this syscall.
        getppid = 173,
        /// See [getuid(2)](https://man7.org/linux/man-pages/man2/getuid.2.html) for more info on this syscall.
        getuid = 174,
        /// See [geteuid(2)](https://man7.org/linux/man-pages/man2/geteuid.2.html) for more info on this syscall.
        geteuid = 175,
        /// See [getgid(2)](https://man7.org/linux/man-pages/man2/getgid.2.html) for more info on this syscall.
        getgid = 176,
        /// See [getegid(2)](https://man7.org/linux/man-pages/man2/getegid.2.html) for more info on this syscall.
        getegid = 177,
        /// See [gettid(2)](https://man7.org/linux/man-pages/man2/gettid.2.html) for more info on this syscall.
        gettid = 178,
        /// See [sysinfo(2)](https://man7.org/linux/man-pages/man2/sysinfo.2.html) for more info on this syscall.
        sysinfo = 179,
        /// See [mq_open(2)](https://man7.org/linux/man-pages/man2/mq_open.2.html) for more info on this syscall.
        mq_open = 180,
        /// See [mq_unlink(2)](https://man7.org/linux/man-pages/man2/mq_unlink.2.html) for more info on this syscall.
        mq_unlink = 181,
        /// See [mq_timedsend(2)](https://man7.org/linux/man-pages/man2/mq_timedsend.2.html) for more info on this syscall.
        mq_timedsend = 182,
        /// See [mq_timedreceive(2)](https://man7.org/linux/man-pages/man2/mq_timedreceive.2.html) for more info on this syscall.
        mq_timedreceive = 183,
        /// See [mq_notify(2)](https://man7.org/linux/man-pages/man2/mq_notify.2.html) for more info on this syscall.
        mq_notify = 184,
        /// See [mq_getsetattr(2)](https://man7.org/linux/man-pages/man2/mq_getsetattr.2.html) for more info on this syscall.
        mq_getsetattr = 185,
        /// See [msgget(2)](https://man7.org/linux/man-pages/man2/msgget.2.html) for more info on this syscall.
        msgget = 186,
        /// See [msgctl(2)](https://man7.org/linux/man-pages/man2/msgctl.2.html) for more info on this syscall.
        msgctl = 187,
        /// See [msgrcv(2)](https://man7.org/linux/man-pages/man2/msgrcv.2.html) for more info on this syscall.
        msgrcv = 188,
        /// See [msgsnd(2)](https://man7.org/linux/man-pages/man2/msgsnd.2.html) for more info on this syscall.
        msgsnd = 189,
        /// See [semget(2)](https://man7.org/linux/man-pages/man2/semget.2.html) for more info on this syscall.
        semget = 190,
        /// See [semctl(2)](https://man7.org/linux/man-pages/man2/semctl.2.html) for more info on this syscall.
        semctl = 191,
        /// See [semtimedop(2)](https://man7.org/linux/man-pages/man2/semtimedop.2.html) for more info on this syscall.
        semtimedop = 192,
        /// See [semop(2)](https://man7.org/linux/man-pages/man2/semop.2.html) for more info on this syscall.
        semop = 193,
        /// See [shmget(2)](https://man7.org/linux/man-pages/man2/shmget.2.html) for more info on this syscall.
        shmget = 194,
        /// See [shmctl(2)](https://man7.org/linux/man-pages/man2/shmctl.2.html) for more info on this syscall.
        shmctl = 195,
        /// See [shmat(2)](https://man7.org/linux/man-pages/man2/shmat.2.html) for more info on this syscall.
        shmat = 196,
        /// See [shmdt(2)](https://man7.org/linux/man-pages/man2/shmdt.2.html) for more info on this syscall.
        shmdt = 197,
        /// See [socket(2)](https://man7.org/linux/man-pages/man2/socket.2.html) for more info on this syscall.
        socket = 198,
        /// See [socketpair(2)](https://man7.org/linux/man-pages/man2/socketpair.2.html) for more info on this syscall.
        socketpair = 199,
        /// See [bind(2)](https://man7.org/linux/man-pages/man2/bind.2.html) for more info on this syscall.
        bind = 200,
        /// See [listen(2)](https://man7.org/linux/man-pages/man2/listen.2.html) for more info on this syscall.
        listen = 201,
        /// See [accept(2)](https://man7.org/linux/man-pages/man2/accept.2.html) for more info on this syscall.
        accept = 202,
        /// See [connect(2)](https://man7.org/linux/man-pages/man2/connect.2.html) for more info on this syscall.
        connect = 203,
        /// See [getsockname(2)](https://man7.org/linux/man-pages/man2/getsockname.2.html) for more info on this syscall.
        getsockname = 204,
        /// See [getpeername(2)](https://man7.org/linux/man-pages/man2/getpeername.2.html) for more info on this syscall.
        getpeername = 205,
        /// See [sendto(2)](https://man7.org/linux/man-pages/man2/sendto.2.html) for more info on this syscall.
        sendto = 206,
        /// See [recvfrom(2)](https://man7.org/linux/man-pages/man2/recvfrom.2.html) for more info on this syscall.
        recvfrom = 207,
        /// See [setsockopt(2)](https://man7.org/linux/man-pages/man2/setsockopt.2.html) for more info on this syscall.
        setsockopt = 208,
        /// See [getsockopt(2)](https://man7.org/linux/man-pages/man2/getsockopt.2.html) for more info on this syscall.
        getsockopt = 209,
        /// See [shutdown(2)](https://man7.org/linux/man-pages/man2/shutdown.2.html) for more info on this syscall.
        shutdown = 210,
        /// See [sendmsg(2)](https://man7.org/linux/man-pages/man2/sendmsg.2.html) for more info on this syscall.
        sendmsg = 211,
        /// See [recvmsg(2)](https://man7.org/linux/man-pages/man2/recvmsg.2.html) for more info on this syscall.
        recvmsg = 212,
        /// See [readahead(2)](https://man7.org/linux/man-pages/man2/readahead.2.html) for more info on this syscall.
        readahead = 213,
        /// See [brk(2)](https://man7.org/linux/man-pages/man2/brk.2.html) for more info on this syscall.
        brk = 214,
        /// See [munmap(2)](https://man7.org/linux/man-pages/man2/munmap.2.html) for more info on this syscall.
        munmap = 215,
        /// See [mremap(2)](https://man7.org/linux/man-pages/man2/mremap.2.html) for more info on this syscall.
        mremap = 216,
        /// See [add_key(2)](https://man7.org/linux/man-pages/man2/add_key.2.html) for more info on this syscall.
        add_key = 217,
        /// See [request_key(2)](https://man7.org/linux/man-pages/man2/request_key.2.html) for more info on this syscall.
        request_key = 218,
        /// See [keyctl(2)](https://man7.org/linux/man-pages/man2/keyctl.2.html) for more info on this syscall.
        keyctl = 219,
        /// See [clone(2)](https://man7.org/linux/man-pages/man2/clone.2.html) for more info on this syscall.
        clone = 220,
        /// See [execve(2)](https://man7.org/linux/man-pages/man2/execve.2.html) for more info on this syscall.
        execve = 221,
        /// See [mmap(2)](https://man7.org/linux/man-pages/man2/mmap.2.html) for more info on this syscall.
        mmap = 222,
        /// See [fadvise64(2)](https://man7.org/linux/man-pages/man2/fadvise64.2.html) for more info on this syscall.
        fadvise64 = 223,
        /// See [swapon(2)](https://man7.org/linux/man-pages/man2/swapon.2.html) for more info on this syscall.
        swapon = 224,
        /// See [swapoff(2)](https://man7.org/linux/man-pages/man2/swapoff.2.html) for more info on this syscall.
        swapoff = 225,
        /// See [mprotect(2)](https://man7.org/linux/man-pages/man2/mprotect.2.html) for more info on this syscall.
        mprotect = 226,
        /// See [msync(2)](https://man7.org/linux/man-pages/man2/msync.2.html) for more info on this syscall.
        msync = 227,
        /// See [mlock(2)](https://man7.org/linux/man-pages/man2/mlock.2.html) for more info on this syscall.
        mlock = 228,
        /// See [munlock(2)](https://man7.org/linux/man-pages/man2/munlock.2.html) for more info on this syscall.
        munlock = 229,
        /// See [mlockall(2)](https://man7.org/linux/man-pages/man2/mlockall.2.html) for more info on this syscall.
        mlockall = 230,
        /// See [munlockall(2)](https://man7.org/linux/man-pages/man2/munlockall.2.html) for more info on this syscall.
        munlockall = 231,
        /// See [mincore(2)](https://man7.org/linux/man-pages/man2/mincore.2.html) for more info on this syscall.
        mincore = 232,
        /// See [madvise(2)](https://man7.org/linux/man-pages/man2/madvise.2.html) for more info on this syscall.
        madvise = 233,
        /// See [remap_file_pages(2)](https://man7.org/linux/man-pages/man2/remap_file_pages.2.html) for more info on this syscall.
        remap_file_pages = 234,
        /// See [mbind(2)](https://man7.org/linux/man-pages/man2/mbind.2.html) for more info on this syscall.
        mbind = 235,
        /// See [get_mempolicy(2)](https://man7.org/linux/man-pages/man2/get_mempolicy.2.html) for more info on this syscall.
        get_mempolicy = 236,
        /// See [set_mempolicy(2)](https://man7.org/linux/man-pages/man2/set_mempolicy.2.html) for more info on this syscall.
        set_mempolicy = 237,
        /// See [migrate_pages(2)](https://man7.org/linux/man-pages/man2/migrate_pages.2.html) for more info on this syscall.
        migrate_pages = 238,
        /// See [move_pages(2)](https://man7.org/linux/man-pages/man2/move_pages.2.html) for more info on this syscall.
        move_pages = 239,
        /// See [rt_tgsigqueueinfo(2)](https://man7.org/linux/man-pages/man2/rt_tgsigqueueinfo.2.html) for more info on this syscall.
        rt_tgsigqueueinfo = 240,
        /// See [perf_event_open(2)](https://man7.org/linux/man-pages/man2/perf_event_open.2.html) for more info on this syscall.
        perf_event_open = 241,
        /// See [accept4(2)](https://man7.org/linux/man-pages/man2/accept4.2.html) for more info on this syscall.
        accept4 = 242,
        /// See [recvmmsg(2)](https://man7.org/linux/man-pages/man2/recvmmsg.2.html) for more info on this syscall.
        recvmmsg = 243,
        /// See [riscv_hwprobe(2)](https://man7.org/linux/man-pages/man2/riscv_hwprobe.2.html) for more info on this syscall.
        riscv_hwprobe = 258,
        /// See [riscv_flush_icache(2)](https://man7.org/linux/man-pages/man2/riscv_flush_icache.2.html) for more info on this syscall.
        riscv_flush_icache = 259,
        /// See [wait4(2)](https://man7.org/linux/man-pages/man2/wait4.2.html) for more info on this syscall.
        wait4 = 260,
        /// See [prlimit64(2)](https://man7.org/linux/man-pages/man2/prlimit64.2.html) for more info on this syscall.
        prlimit64 = 261,
        /// See [fanotify_init(2)](https://man7.org/linux/man-pages/man2/fanotify_init.2.html) for more info on this syscall.
        fanotify_init = 262,
        /// See [fanotify_mark(2)](https://man7.org/linux/man-pages/man2/fanotify_mark.2.html) for more info on this syscall.
        fanotify_mark = 263,
        /// See [name_to_handle_at(2)](https://man7.org/linux/man-pages/man2/name_to_handle_at.2.html) for more info on this syscall.
        name_to_handle_at = 264,
        /// See [open_by_handle_at(2)](https://man7.org/linux/man-pages/man2/open_by_handle_at.2.html) for more info on this syscall.
        open_by_handle_at = 265,
        /// See [clock_adjtime(2)](https://man7.org/linux/man-pages/man2/clock_adjtime.2.html) for more info on this syscall.
        clock_adjtime = 266,
        /// See [syncfs(2)](https://man7.org/linux/man-pages/man2/syncfs.2.html) for more info on this syscall.
        syncfs = 267,
        /// See [setns(2)](https://man7.org/linux/man-pages/man2/setns.2.html) for more info on this syscall.
        setns = 268,
        /// See [sendmmsg(2)](https://man7.org/linux/man-pages/man2/sendmmsg.2.html) for more info on this syscall.
        sendmmsg = 269,
        /// See [process_vm_readv(2)](https://man7.org/linux/man-pages/man2/process_vm_readv.2.html) for more info on this syscall.
        process_vm_readv = 270,
        /// See [process_vm_writev(2)](https://man7.org/linux/man-pages/man2/process_vm_writev.2.html) for more info on this syscall.
        process_vm_writev = 271,
        /// See [kcmp(2)](https://man7.org/linux/man-pages/man2/kcmp.2.html) for more info on this syscall.
        kcmp = 272,
        /// See [finit_module(2)](https://man7.org/linux/man-pages/man2/finit_module.2.html) for more info on this syscall.
        finit_module = 273,
        /// See [sched_setattr(2)](https://man7.org/linux/man-pages/man2/sched_setattr.2.html) for more info on this syscall.
        sched_setattr = 274,
        /// See [sched_getattr(2)](https://man7.org/linux/man-pages/man2/sched_getattr.2.html) for more info on this syscall.
        sched_getattr = 275,
        /// See [renameat2(2)](https://man7.org/linux/man-pages/man2/renameat2.2.html) for more info on this syscall.
        renameat2 = 276,
        /// See [seccomp(2)](https://man7.org/linux/man-pages/man2/seccomp.2.html) for more info on this syscall.
        seccomp = 277,
        /// See [getrandom(2)](https://man7.org/linux/man-pages/man2/getrandom.2.html) for more info on this syscall.
        getrandom = 278,
        /// See [memfd_create(2)](https://man7.org/linux/man-pages/man2/memfd_create.2.html) for more info on this syscall.
        memfd_create = 279,
        /// See [bpf(2)](https://man7.org/linux/man-pages/man2/bpf.2.html) for more info on this syscall.
        bpf = 280,
        /// See [execveat(2)](https://man7.org/linux/man-pages/man2/execveat.2.html) for more info on this syscall.
        execveat = 281,
        /// See [userfaultfd(2)](https://man7.org/linux/man-pages/man2/userfaultfd.2.html) for more info on this syscall.
        userfaultfd = 282,
        /// See [membarrier(2)](https://man7.org/linux/man-pages/man2/membarrier.2.html) for more info on this syscall.
        membarrier = 283,
        /// See [mlock2(2)](https://man7.org/linux/man-pages/man2/mlock2.2.html) for more info on this syscall.
        mlock2 = 284,
        /// See [copy_file_range(2)](https://man7.org/linux/man-pages/man2/copy_file_range.2.html) for more info on this syscall.
        copy_file_range = 285,
        /// See [preadv2(2)](https://man7.org/linux/man-pages/man2/preadv2.2.html) for more info on this syscall.
        preadv2 = 286,
        /// See [pwritev2(2)](https://man7.org/linux/man-pages/man2/pwritev2.2.html) for more info on this syscall.
        pwritev2 = 287,
        /// See [pkey_mprotect(2)](https://man7.org/linux/man-pages/man2/pkey_mprotect.2.html) for more info on this syscall.
        pkey_mprotect = 288,
        /// See [pkey_alloc(2)](https://man7.org/linux/man-pages/man2/pkey_alloc.2.html) for more info on this syscall.
        pkey_alloc = 289,
        /// See [pkey_free(2)](https://man7.org/linux/man-pages/man2/pkey_free.2.html) for more info on this syscall.
        pkey_free = 290,
        /// See [statx(2)](https://man7.org/linux/man-pages/man2/statx.2.html) for more info on this syscall.
        statx = 291,
        /// See [io_pgetevents(2)](https://man7.org/linux/man-pages/man2/io_pgetevents.2.html) for more info on this syscall.
        io_pgetevents = 292,
        /// See [rseq(2)](https://man7.org/linux/man-pages/man2/rseq.2.html) for more info on this syscall.
        rseq = 293,
        /// See [kexec_file_load(2)](https://man7.org/linux/man-pages/man2/kexec_file_load.2.html) for more info on this syscall.
        kexec_file_load = 294,
        /// See [clock_gettime64(2)](https://man7.org/linux/man-pages/man2/clock_gettime64.2.html) for more info on this syscall.
        clock_gettime64 = 403,
        /// See [clock_settime64(2)](https://man7.org/linux/man-pages/man2/clock_settime64.2.html) for more info on this syscall.
        clock_settime64 = 404,
        /// See [clock_adjtime64(2)](https://man7.org/linux/man-pages/man2/clock_adjtime64.2.html) for more info on this syscall.
        clock_adjtime64 = 405,
        /// See [clock_getres_time64(2)](https://man7.org/linux/man-pages/man2/clock_getres_time64.2.html) for more info on this syscall.
        clock_getres_time64 = 406,
        /// See [clock_nanosleep_time64(2)](https://man7.org/linux/man-pages/man2/clock_nanosleep_time64.2.html) for more info on this syscall.
        clock_nanosleep_time64 = 407,
        /// See [timer_gettime64(2)](https://man7.org/linux/man-pages/man2/timer_gettime64.2.html) for more info on this syscall.
        timer_gettime64 = 408,
        /// See [timer_settime64(2)](https://man7.org/linux/man-pages/man2/timer_settime64.2.html) for more info on this syscall.
        timer_settime64 = 409,
        /// See [timerfd_gettime64(2)](https://man7.org/linux/man-pages/man2/timerfd_gettime64.2.html) for more info on this syscall.
        timerfd_gettime64 = 410,
        /// See [timerfd_settime64(2)](https://man7.org/linux/man-pages/man2/timerfd_settime64.2.html) for more info on this syscall.
        timerfd_settime64 = 411,
        /// See [utimensat_time64(2)](https://man7.org/linux/man-pages/man2/utimensat_time64.2.html) for more info on this syscall.
        utimensat_time64 = 412,
        /// See [pselect6_time64(2)](https://man7.org/linux/man-pages/man2/pselect6_time64.2.html) for more info on this syscall.
        pselect6_time64 = 413,
        /// See [ppoll_time64(2)](https://man7.org/linux/man-pages/man2/ppoll_time64.2.html) for more info on this syscall.
        ppoll_time64 = 414,
        /// See [io_pgetevents_time64(2)](https://man7.org/linux/man-pages/man2/io_pgetevents_time64.2.html) for more info on this syscall.
        io_pgetevents_time64 = 416,
        /// See [recvmmsg_time64(2)](https://man7.org/linux/man-pages/man2/recvmmsg_time64.2.html) for more info on this syscall.
        recvmmsg_time64 = 417,
        /// See [mq_timedsend_time64(2)](https://man7.org/linux/man-pages/man2/mq_timedsend_time64.2.html) for more info on this syscall.
        mq_timedsend_time64 = 418,
        /// See [mq_timedreceive_time64(2)](https://man7.org/linux/man-pages/man2/mq_timedreceive_time64.2.html) for more info on this syscall.
        mq_timedreceive_time64 = 419,
        /// See [semtimedop_time64(2)](https://man7.org/linux/man-pages/man2/semtimedop_time64.2.html) for more info on this syscall.
        semtimedop_time64 = 420,
        /// See [rt_sigtimedwait_time64(2)](https://man7.org/linux/man-pages/man2/rt_sigtimedwait_time64.2.html) for more info on this syscall.
        rt_sigtimedwait_time64 = 421,
        /// See [futex_time64(2)](https://man7.org/linux/man-pages/man2/futex_time64.2.html) for more info on this syscall.
        futex_time64 = 422,
        /// See [sched_rr_get_interval_time64(2)](https://man7.org/linux/man-pages/man2/sched_rr_get_interval_time64.2.html) for more info on this syscall.
        sched_rr_get_interval_time64 = 423,
        /// See [pidfd_send_signal(2)](https://man7.org/linux/man-pages/man2/pidfd_send_signal.2.html) for more info on this syscall.
        pidfd_send_signal = 424,
        /// See [io_uring_setup(2)](https://man7.org/linux/man-pages/man2/io_uring_setup.2.html) for more info on this syscall.
        io_uring_setup = 425,
        /// See [io_uring_enter(2)](https://man7.org/linux/man-pages/man2/io_uring_enter.2.html) for more info on this syscall.
        io_uring_enter = 426,
        /// See [io_uring_register(2)](https://man7.org/linux/man-pages/man2/io_uring_register.2.html) for more info on this syscall.
        io_uring_register = 427,
        /// See [open_tree(2)](https://man7.org/linux/man-pages/man2/open_tree.2.html) for more info on this syscall.
        open_tree = 428,
        /// See [move_mount(2)](https://man7.org/linux/man-pages/man2/move_mount.2.html) for more info on this syscall.
        move_mount = 429,
        /// See [fsopen(2)](https://man7.org/linux/man-pages/man2/fsopen.2.html) for more info on this syscall.
        fsopen = 430,
        /// See [fsconfig(2)](https://man7.org/linux/man-pages/man2/fsconfig.2.html) for more info on this syscall.
        fsconfig = 431,
        /// See [fsmount(2)](https://man7.org/linux/man-pages/man2/fsmount.2.html) for more info on this syscall.
        fsmount = 432,
        /// See [fspick(2)](https://man7.org/linux/man-pages/man2/fspick.2.html) for more info on this syscall.
        fspick = 433,
        /// See [pidfd_open(2)](https://man7.org/linux/man-pages/man2/pidfd_open.2.html) for more info on this syscall.
        pidfd_open = 434,
        /// See [clone3(2)](https://man7.org/linux/man-pages/man2/clone3.2.html) for more info on this syscall.
        clone3 = 435,
        /// See [close_range(2)](https://man7.org/linux/man-pages/man2/close_range.2.html) for more info on this syscall.
        close_range = 436,
        /// See [openat2(2)](https://man7.org/linux/man-pages/man2/openat2.2.html) for more info on this syscall.
        openat2 = 437,
        /// See [pidfd_getfd(2)](https://man7.org/linux/man-pages/man2/pidfd_getfd.2.html) for more info on this syscall.
        pidfd_getfd = 438,
        /// See [faccessat2(2)](https://man7.org/linux/man-pages/man2/faccessat2.2.html) for more info on this syscall.
        faccessat2 = 439,
        /// See [process_madvise(2)](https://man7.org/linux/man-pages/man2/process_madvise.2.html) for more info on this syscall.
        process_madvise = 440,
        /// See [epoll_pwait2(2)](https://man7.org/linux/man-pages/man2/epoll_pwait2.2.html) for more info on this syscall.
        epoll_pwait2 = 441,
        /// See [mount_setattr(2)](https://man7.org/linux/man-pages/man2/mount_setattr.2.html) for more info on this syscall.
        mount_setattr = 442,
        /// See [quotactl_fd(2)](https://man7.org/linux/man-pages/man2/quotactl_fd.2.html) for more info on this syscall.
        quotactl_fd = 443,
        /// See [landlock_create_ruleset(2)](https://man7.org/linux/man-pages/man2/landlock_create_ruleset.2.html) for more info on this syscall.
        landlock_create_ruleset = 444,
        /// See [landlock_add_rule(2)](https://man7.org/linux/man-pages/man2/landlock_add_rule.2.html) for more info on this syscall.
        landlock_add_rule = 445,
        /// See [landlock_restrict_self(2)](https://man7.org/linux/man-pages/man2/landlock_restrict_self.2.html) for more info on this syscall.
        landlock_restrict_self = 446,
        /// See [memfd_secret(2)](https://man7.org/linux/man-pages/man2/memfd_secret.2.html) for more info on this syscall.
        memfd_secret = 447,
        /// See [process_mrelease(2)](https://man7.org/linux/man-pages/man2/process_mrelease.2.html) for more info on this syscall.
        process_mrelease = 448,
        /// See [futex_waitv(2)](https://man7.org/linux/man-pages/man2/futex_waitv.2.html) for more info on this syscall.
        futex_waitv = 449,
        /// See [set_mempolicy_home_node(2)](https://man7.org/linux/man-pages/man2/set_mempolicy_home_node.2.html) for more info on this syscall.
        set_mempolicy_home_node = 450,
        /// See [cachestat(2)](https://man7.org/linux/man-pages/man2/cachestat.2.html) for more info on this syscall.
        cachestat = 451,
        /// See [fchmodat2(2)](https://man7.org/linux/man-pages/man2/fchmodat2.2.html) for more info on this syscall.
        fchmodat2 = 452,
        /// See [map_shadow_stack(2)](https://man7.org/linux/man-pages/man2/map_shadow_stack.2.html) for more info on this syscall.
        map_shadow_stack = 453,
        /// See [futex_wake(2)](https://man7.org/linux/man-pages/man2/futex_wake.2.html) for more info on this syscall.
        futex_wake = 454,
        /// See [futex_wait(2)](https://man7.org/linux/man-pages/man2/futex_wait.2.html) for more info on this syscall.
        futex_wait = 455,
        /// See [futex_requeue(2)](https://man7.org/linux/man-pages/man2/futex_requeue.2.html) for more info on this syscall.
        futex_requeue = 456,
        /// See [statmount(2)](https://man7.org/linux/man-pages/man2/statmount.2.html) for more info on this syscall.
        statmount = 457,
        /// See [listmount(2)](https://man7.org/linux/man-pages/man2/listmount.2.html) for more info on this syscall.
        listmount = 458,
        /// See [lsm_get_self_attr(2)](https://man7.org/linux/man-pages/man2/lsm_get_self_attr.2.html) for more info on this syscall.
        lsm_get_self_attr = 459,
        /// See [lsm_set_self_attr(2)](https://man7.org/linux/man-pages/man2/lsm_set_self_attr.2.html) for more info on this syscall.
        lsm_set_self_attr = 460,
        /// See [lsm_list_modules(2)](https://man7.org/linux/man-pages/man2/lsm_list_modules.2.html) for more info on this syscall.
        lsm_list_modules = 461,
    }
    LAST: lsm_list_modules;
}
