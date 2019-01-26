#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc, ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    pub type __dirstream;
    #[no_mangle]
    fn chown(__file: *const libc::c_char, __owner: __uid_t, __group: __gid_t)
     -> libc::c_int;
    #[no_mangle]
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn utime(__file: *const libc::c_char, __file_times: *const utimbuf)
     -> libc::c_int;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __syscall_ulong_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type unnamed = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: unnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: unnamed = 247;
pub const _SC_XOPEN_STREAMS: unnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: unnamed = 245;
pub const _SC_TRACE_SYS_MAX: unnamed = 244;
pub const _SC_TRACE_NAME_MAX: unnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: unnamed = 242;
pub const _SC_SS_REPL_MAX: unnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: unnamed = 240;
pub const _SC_V7_LP64_OFF64: unnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: unnamed = 238;
pub const _SC_V7_ILP32_OFF32: unnamed = 237;
pub const _SC_RAW_SOCKETS: unnamed = 236;
pub const _SC_IPV6: unnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: unnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: unnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: unnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: unnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: unnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: unnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: unnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: unnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: unnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: unnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: unnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: unnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: unnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: unnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: unnamed = 185;
pub const _SC_TRACE_LOG: unnamed = 184;
pub const _SC_TRACE_INHERIT: unnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: unnamed = 182;
pub const _SC_TRACE: unnamed = 181;
pub const _SC_HOST_NAME_MAX: unnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: unnamed = 179;
pub const _SC_V6_LP64_OFF64: unnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: unnamed = 177;
pub const _SC_V6_ILP32_OFF32: unnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: unnamed = 175;
pub const _SC_STREAMS: unnamed = 174;
pub const _SC_SYMLOOP_MAX: unnamed = 173;
pub const _SC_2_PBS_TRACK: unnamed = 172;
pub const _SC_2_PBS_MESSAGE: unnamed = 171;
pub const _SC_2_PBS_LOCATE: unnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: unnamed = 169;
pub const _SC_2_PBS: unnamed = 168;
pub const _SC_USER_GROUPS_R: unnamed = 167;
pub const _SC_USER_GROUPS: unnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: unnamed = 165;
pub const _SC_TIMEOUTS: unnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: unnamed = 163;
pub const _SC_SYSTEM_DATABASE: unnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: unnamed = 161;
pub const _SC_SPORADIC_SERVER: unnamed = 160;
pub const _SC_SPAWN: unnamed = 159;
pub const _SC_SIGNALS: unnamed = 158;
pub const _SC_SHELL: unnamed = 157;
pub const _SC_REGEX_VERSION: unnamed = 156;
pub const _SC_REGEXP: unnamed = 155;
pub const _SC_SPIN_LOCKS: unnamed = 154;
pub const _SC_READER_WRITER_LOCKS: unnamed = 153;
pub const _SC_NETWORKING: unnamed = 152;
pub const _SC_SINGLE_PROCESS: unnamed = 151;
pub const _SC_MULTI_PROCESS: unnamed = 150;
pub const _SC_MONOTONIC_CLOCK: unnamed = 149;
pub const _SC_FILE_SYSTEM: unnamed = 148;
pub const _SC_FILE_LOCKING: unnamed = 147;
pub const _SC_FILE_ATTRIBUTES: unnamed = 146;
pub const _SC_PIPE: unnamed = 145;
pub const _SC_FIFO: unnamed = 144;
pub const _SC_FD_MGMT: unnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: unnamed = 142;
pub const _SC_DEVICE_SPECIFIC: unnamed = 141;
pub const _SC_DEVICE_IO: unnamed = 140;
pub const _SC_THREAD_CPUTIME: unnamed = 139;
pub const _SC_CPUTIME: unnamed = 138;
pub const _SC_CLOCK_SELECTION: unnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: unnamed = 136;
pub const _SC_C_LANG_SUPPORT: unnamed = 135;
pub const _SC_BASE: unnamed = 134;
pub const _SC_BARRIERS: unnamed = 133;
pub const _SC_ADVISORY_INFO: unnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: unnamed = 131;
pub const _SC_XOPEN_REALTIME: unnamed = 130;
pub const _SC_XOPEN_LEGACY: unnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: unnamed = 128;
pub const _SC_XBS5_LP64_OFF64: unnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: unnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: unnamed = 125;
pub const _SC_NL_TEXTMAX: unnamed = 124;
pub const _SC_NL_SETMAX: unnamed = 123;
pub const _SC_NL_NMAX: unnamed = 122;
pub const _SC_NL_MSGMAX: unnamed = 121;
pub const _SC_NL_LANGMAX: unnamed = 120;
pub const _SC_NL_ARGMAX: unnamed = 119;
pub const _SC_USHRT_MAX: unnamed = 118;
pub const _SC_ULONG_MAX: unnamed = 117;
pub const _SC_UINT_MAX: unnamed = 116;
pub const _SC_UCHAR_MAX: unnamed = 115;
pub const _SC_SHRT_MIN: unnamed = 114;
pub const _SC_SHRT_MAX: unnamed = 113;
pub const _SC_SCHAR_MIN: unnamed = 112;
pub const _SC_SCHAR_MAX: unnamed = 111;
pub const _SC_SSIZE_MAX: unnamed = 110;
pub const _SC_NZERO: unnamed = 109;
pub const _SC_MB_LEN_MAX: unnamed = 108;
pub const _SC_WORD_BIT: unnamed = 107;
pub const _SC_LONG_BIT: unnamed = 106;
pub const _SC_INT_MIN: unnamed = 105;
pub const _SC_INT_MAX: unnamed = 104;
pub const _SC_CHAR_MIN: unnamed = 103;
pub const _SC_CHAR_MAX: unnamed = 102;
pub const _SC_CHAR_BIT: unnamed = 101;
pub const _SC_XOPEN_XPG4: unnamed = 100;
pub const _SC_XOPEN_XPG3: unnamed = 99;
pub const _SC_XOPEN_XPG2: unnamed = 98;
pub const _SC_2_UPE: unnamed = 97;
pub const _SC_2_C_VERSION: unnamed = 96;
pub const _SC_2_CHAR_TERM: unnamed = 95;
pub const _SC_XOPEN_SHM: unnamed = 94;
pub const _SC_XOPEN_ENH_I18N: unnamed = 93;
pub const _SC_XOPEN_CRYPT: unnamed = 92;
pub const _SC_XOPEN_UNIX: unnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: unnamed = 90;
pub const _SC_XOPEN_VERSION: unnamed = 89;
pub const _SC_PASS_MAX: unnamed = 88;
pub const _SC_ATEXIT_MAX: unnamed = 87;
pub const _SC_AVPHYS_PAGES: unnamed = 86;
pub const _SC_PHYS_PAGES: unnamed = 85;
pub const _SC_NPROCESSORS_ONLN: unnamed = 84;
pub const _SC_NPROCESSORS_CONF: unnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: unnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: unnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: unnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: unnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: unnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: unnamed = 77;
pub const _SC_THREAD_THREADS_MAX: unnamed = 76;
pub const _SC_THREAD_STACK_MIN: unnamed = 75;
pub const _SC_THREAD_KEYS_MAX: unnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: unnamed = 73;
pub const _SC_TTY_NAME_MAX: unnamed = 72;
pub const _SC_LOGIN_NAME_MAX: unnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: unnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: unnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: unnamed = 68;
pub const _SC_THREADS: unnamed = 67;
pub const _SC_T_IOV_MAX: unnamed = 66;
pub const _SC_PII_OSI_M: unnamed = 65;
pub const _SC_PII_OSI_CLTS: unnamed = 64;
pub const _SC_PII_OSI_COTS: unnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: unnamed = 62;
pub const _SC_PII_INTERNET_STREAM: unnamed = 61;
pub const _SC_IOV_MAX: unnamed = 60;
pub const _SC_UIO_MAXIOV: unnamed = 60;
pub const _SC_SELECT: unnamed = 59;
pub const _SC_POLL: unnamed = 58;
pub const _SC_PII_OSI: unnamed = 57;
pub const _SC_PII_INTERNET: unnamed = 56;
pub const _SC_PII_SOCKET: unnamed = 55;
pub const _SC_PII_XTI: unnamed = 54;
pub const _SC_PII: unnamed = 53;
pub const _SC_2_LOCALEDEF: unnamed = 52;
pub const _SC_2_SW_DEV: unnamed = 51;
pub const _SC_2_FORT_RUN: unnamed = 50;
pub const _SC_2_FORT_DEV: unnamed = 49;
pub const _SC_2_C_DEV: unnamed = 48;
pub const _SC_2_C_BIND: unnamed = 47;
pub const _SC_2_VERSION: unnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: unnamed = 45;
pub const _SC_RE_DUP_MAX: unnamed = 44;
pub const _SC_LINE_MAX: unnamed = 43;
pub const _SC_EXPR_NEST_MAX: unnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: unnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: unnamed = 40;
pub const _SC_BC_STRING_MAX: unnamed = 39;
pub const _SC_BC_SCALE_MAX: unnamed = 38;
pub const _SC_BC_DIM_MAX: unnamed = 37;
pub const _SC_BC_BASE_MAX: unnamed = 36;
pub const _SC_TIMER_MAX: unnamed = 35;
pub const _SC_SIGQUEUE_MAX: unnamed = 34;
pub const _SC_SEM_VALUE_MAX: unnamed = 33;
pub const _SC_SEM_NSEMS_MAX: unnamed = 32;
pub const _SC_RTSIG_MAX: unnamed = 31;
pub const _SC_PAGESIZE: unnamed = 30;
pub const _SC_VERSION: unnamed = 29;
pub const _SC_MQ_PRIO_MAX: unnamed = 28;
pub const _SC_MQ_OPEN_MAX: unnamed = 27;
pub const _SC_DELAYTIMER_MAX: unnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: unnamed = 25;
pub const _SC_AIO_MAX: unnamed = 24;
pub const _SC_AIO_LISTIO_MAX: unnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: unnamed = 22;
pub const _SC_SEMAPHORES: unnamed = 21;
pub const _SC_MESSAGE_PASSING: unnamed = 20;
pub const _SC_MEMORY_PROTECTION: unnamed = 19;
pub const _SC_MEMLOCK_RANGE: unnamed = 18;
pub const _SC_MEMLOCK: unnamed = 17;
pub const _SC_MAPPED_FILES: unnamed = 16;
pub const _SC_FSYNC: unnamed = 15;
pub const _SC_SYNCHRONIZED_IO: unnamed = 14;
pub const _SC_PRIORITIZED_IO: unnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: unnamed = 12;
pub const _SC_TIMERS: unnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: unnamed = 10;
pub const _SC_REALTIME_SIGNALS: unnamed = 9;
pub const _SC_SAVED_IDS: unnamed = 8;
pub const _SC_JOB_CONTROL: unnamed = 7;
pub const _SC_TZNAME_MAX: unnamed = 6;
pub const _SC_STREAM_MAX: unnamed = 5;
pub const _SC_OPEN_MAX: unnamed = 4;
pub const _SC_NGROUPS_MAX: unnamed = 3;
pub const _SC_CLK_TCK: unnamed = 2;
pub const _SC_CHILD_MAX: unnamed = 1;
pub const _SC_ARG_MAX: unnamed = 0;
pub type ptrdiff_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atime: __time_t,
    pub st_atimensec: __syscall_ulong_t,
    pub st_mtime: __time_t,
    pub st_mtimensec: __syscall_ulong_t,
    pub st_ctime: __time_t,
    pub st_ctimensec: __syscall_ulong_t,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type UTIL_time_t = timespec;
/*-****************************************
*  File functions
******************************************/
pub type stat_t = stat;
pub type DIR = __dirstream;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
/*
 * Copyright (c) 2016-present, Przemyslaw Skibinski, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/*-****************************************
*  Dependencies
******************************************/
/* malloc, realloc, free */
/* size_t, ptrdiff_t */
/* fprintf */
/* stat, utime */
/* stat, chmod */
/* chown, stat */
/* utime */
/* clock_t, clock, CLOCKS_PER_SEC, nanosleep */
/*-************************************************************
* Avoid fseek()'s 2GiB barrier with MSVC, macOS, *BSD, MinGW
***************************************************************/
/* No point defining Large file for 64 bit */
/*-*************************************************
*  Sleep & priority functions: Windows - Posix - others
***************************************************/
/* Unix-like operating system */
/* sleep */
/* necessarily defined in platform.h */
/* setpriority */
/* unknown non-unix operating systen */
/*-*************************************
*  Constants
***************************************/
/*-****************************************
*  Compiler specifics
******************************************/
/* C99 */
/*-****************************************
*  Console log
******************************************/
#[no_mangle]
pub static mut g_utilDisplayLevel: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn UTIL_getSpanTime(mut begin: UTIL_time_t,
                                          mut end: UTIL_time_t)
 -> UTIL_time_t {
    let mut diff: UTIL_time_t = timespec{tv_sec: 0, tv_nsec: 0,};
    if end.tv_nsec < begin.tv_nsec {
        diff.tv_sec = end.tv_sec - 1i32 as libc::c_long - begin.tv_sec;
        diff.tv_nsec =
            (end.tv_nsec as
                 libc::c_ulonglong).wrapping_add(1000000000u64).wrapping_sub(begin.tv_nsec
                                                                                 as
                                                                                 libc::c_ulonglong)
                as __syscall_slong_t
    } else {
        diff.tv_sec = end.tv_sec - begin.tv_sec;
        diff.tv_nsec = end.tv_nsec - begin.tv_nsec
    }
    return diff;
}
/* relies on standard C (note : clock_t measurements can be wrong when using multi-threading) */
#[no_mangle]
pub unsafe extern "C" fn UTIL_getTime() -> UTIL_time_t {
    let mut time_0: UTIL_time_t = timespec{tv_sec: 0, tv_nsec: 0,};
    if 0 != clock_gettime(1i32, &mut time_0) {
        if g_utilDisplayLevel >= 1i32 {
            fprintf(stderr,
                    b"ERROR: Failed to get time\n\x00" as *const u8 as
                        *const libc::c_char);
        }
    }
    return time_0;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_getSpanTimeMicro(mut begin: UTIL_time_t,
                                               mut end: UTIL_time_t) -> U64 {
    let diff: UTIL_time_t = UTIL_getSpanTime(begin, end);
    let mut micro: U64 = 0i32 as U64;
    micro =
        (micro as
             libc::c_ulonglong).wrapping_add(1000000u64.wrapping_mul(diff.tv_sec
                                                                         as
                                                                         libc::c_ulonglong))
            as U64 as U64;
    micro =
        (micro as
             libc::c_ulonglong).wrapping_add((diff.tv_nsec as
                                                  libc::c_ulonglong).wrapping_div(1000u64))
            as U64 as U64;
    return micro;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_getSpanTimeNano(mut begin: UTIL_time_t,
                                              mut end: UTIL_time_t) -> U64 {
    let diff: UTIL_time_t = UTIL_getSpanTime(begin, end);
    let mut nano: U64 = 0i32 as U64;
    nano =
        (nano as
             libc::c_ulonglong).wrapping_add(1000000000u64.wrapping_mul(diff.tv_sec
                                                                            as
                                                                            libc::c_ulonglong))
            as U64 as U64;
    nano =
        (nano as libc::c_ulong).wrapping_add(diff.tv_nsec as libc::c_ulong) as
            U64 as U64;
    return nano;
}
/* returns time span in microseconds */
#[no_mangle]
pub unsafe extern "C" fn UTIL_clockSpanMicro(mut clockStart: UTIL_time_t)
 -> U64 {
    let clockEnd: UTIL_time_t = UTIL_getTime();
    return UTIL_getSpanTimeMicro(clockStart, clockEnd);
}
/* returns time span in microseconds */
#[no_mangle]
pub unsafe extern "C" fn UTIL_clockSpanNano(mut clockStart: UTIL_time_t)
 -> U64 {
    let clockEnd: UTIL_time_t = UTIL_getTime();
    return UTIL_getSpanTimeNano(clockStart, clockEnd);
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_waitForNextTick() {
    let clockStart: UTIL_time_t = UTIL_getTime();
    let mut clockEnd: UTIL_time_t = timespec{tv_sec: 0, tv_nsec: 0,};
    loop  {
        clockEnd = UTIL_getTime();
        if !(UTIL_getSpanTimeNano(clockStart, clockEnd) ==
                 0i32 as libc::c_ulong) {
            break ;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_fileExist(mut filename: *const libc::c_char)
 -> libc::c_int {
    let mut statbuf: stat_t =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atime: 0,
             st_atimensec: 0,
             st_mtime: 0,
             st_mtimensec: 0,
             st_ctime: 0,
             st_ctimensec: 0,
             __glibc_reserved: [0; 3],};
    let stat_error: libc::c_int = stat(filename, &mut statbuf);
    return (0 == stat_error) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_isRegularFile(mut infilename:
                                                *const libc::c_char)
 -> libc::c_int {
    let mut statbuf: stat_t =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atime: 0,
             st_atimensec: 0,
             st_mtime: 0,
             st_mtimensec: 0,
             st_ctime: 0,
             st_ctimensec: 0,
             __glibc_reserved: [0; 3],};
    return UTIL_getFileStat(infilename, &mut statbuf);
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_getFileStat(mut infilename: *const libc::c_char,
                                          mut statbuf: *mut stat_t)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = stat(infilename, statbuf);
    if 0 != r ||
           !((*statbuf).st_mode & 0o170000i32 as libc::c_uint ==
                 0o100000i32 as libc::c_uint) {
        return 0i32
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_setFileStat(mut filename: *const libc::c_char,
                                          mut statbuf: *mut stat_t)
 -> libc::c_int {
    let mut res: libc::c_int = 0i32;
    let mut timebuf: utimbuf = utimbuf{actime: 0, modtime: 0,};
    if 0 == UTIL_isRegularFile(filename) { return -1i32 }
    timebuf.actime = time(0 as *mut time_t);
    timebuf.modtime = (*statbuf).st_mtime;
    res += utime(filename, &mut timebuf);
    res += chown(filename, (*statbuf).st_uid, (*statbuf).st_gid);
    res += chmod(filename, (*statbuf).st_mode & 0o7777i32 as libc::c_uint);
    *__errno_location() = 0i32;
    return -res;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_isDirectory(mut infilename: *const libc::c_char)
 -> U32 {
    let mut r: libc::c_int = 0;
    let mut statbuf: stat_t =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atime: 0,
             st_atimensec: 0,
             st_mtime: 0,
             st_mtimensec: 0,
             st_ctime: 0,
             st_ctimensec: 0,
             __glibc_reserved: [0; 3],};
    r = stat(infilename, &mut statbuf);
    if 0 == r &&
           statbuf.st_mode & 0o170000i32 as libc::c_uint ==
               0o40000i32 as libc::c_uint {
        return 1i32 as U32
    }
    return 0i32 as U32;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_isLink(mut infilename: *const libc::c_char)
 -> U32 {
    return 0i32 as U32;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_getFileSize(mut infilename: *const libc::c_char)
 -> U64 {
    if 0 == UTIL_isRegularFile(infilename) { return -1i32 as U64 }
    let mut r: libc::c_int = 0;
    let mut statbuf: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atime: 0,
             st_atimensec: 0,
             st_mtime: 0,
             st_mtimensec: 0,
             st_ctime: 0,
             st_ctimensec: 0,
             __glibc_reserved: [0; 3],};
    r = stat(infilename, &mut statbuf);
    if 0 != r ||
           !(statbuf.st_mode & 0o170000i32 as libc::c_uint ==
                 0o100000i32 as libc::c_uint) {
        return -1i32 as U64
    }
    return statbuf.st_size as U64;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_getTotalFileSize(fileNamesTable:
                                                   *const *const libc::c_char,
                                               mut nbFiles: libc::c_uint)
 -> U64 {
    let mut total: U64 = 0i32 as U64;
    let mut error: libc::c_int = 0i32;
    let mut n: libc::c_uint = 0;
    n = 0i32 as libc::c_uint;
    while n < nbFiles {
        let size: U64 = UTIL_getFileSize(*fileNamesTable.offset(n as isize));
        error |= (size == -1i32 as U64) as libc::c_int;
        total = (total as libc::c_ulong).wrapping_add(size) as U64 as U64;
        n = n.wrapping_add(1)
    }
    return if 0 != error { -1i32 as U64 } else { total };
}
/*
 * A modified version of realloc().
 * If UTIL_realloc() fails the original block is freed.
*/
unsafe extern "C" fn UTIL_realloc(mut ptr: *mut libc::c_void,
                                  mut size: size_t) -> *mut libc::c_void {
    let mut newptr: *mut libc::c_void = realloc(ptr, size);
    if !newptr.is_null() { return newptr }
    free(ptr);
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_prepareFileList(mut dirName:
                                                  *const libc::c_char,
                                              mut bufStart:
                                                  *mut *mut libc::c_char,
                                              mut pos: *mut size_t,
                                              mut bufEnd:
                                                  *mut *mut libc::c_char,
                                              mut followLinks: libc::c_int)
 -> libc::c_int {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirLength: libc::c_int = 0;
    let mut fnameLength: libc::c_int = 0;
    let mut pathLength: libc::c_int = 0;
    let mut nbFiles: libc::c_int = 0i32;
    dir = opendir(dirName);
    if dir.is_null() {
        if g_utilDisplayLevel >= 1i32 {
            fprintf(stderr,
                    b"Cannot open directory \'%s\': %s\n\x00" as *const u8 as
                        *const libc::c_char, dirName,
                    strerror(*__errno_location()));
        }
        return 0i32
    }
    dirLength = strlen(dirName) as libc::c_int;
    *__errno_location() = 0i32;
    loop  {
        entry = readdir(dir);
        if entry.is_null() { break ; }
        if strcmp((*entry).d_name.as_mut_ptr(),
                  b"..\x00" as *const u8 as *const libc::c_char) == 0i32 ||
               strcmp((*entry).d_name.as_mut_ptr(),
                      b".\x00" as *const u8 as *const libc::c_char) == 0i32 {
            continue ;
        }
        fnameLength = strlen((*entry).d_name.as_mut_ptr()) as libc::c_int;
        path =
            malloc((dirLength + fnameLength + 2i32) as libc::c_ulong) as
                *mut libc::c_char;
        if path.is_null() { closedir(dir); return 0i32 }
        memcpy(path as *mut libc::c_void, dirName as *const libc::c_void,
               dirLength as libc::c_ulong);
        *path.offset(dirLength as isize) = '/' as i32 as libc::c_char;
        memcpy(path.offset(dirLength as isize).offset(1isize) as
                   *mut libc::c_void,
               (*entry).d_name.as_mut_ptr() as *const libc::c_void,
               fnameLength as libc::c_ulong);
        pathLength = dirLength + 1i32 + fnameLength;
        *path.offset(pathLength as isize) = 0i32 as libc::c_char;
        if 0 == followLinks && 0 != UTIL_isLink(path) {
            if g_utilDisplayLevel >= 2i32 {
                fprintf(stderr,
                        b"Warning : %s is a symbolic link, ignoring\n\x00" as
                            *const u8 as *const libc::c_char, path);
            }
        } else {
            if 0 != UTIL_isDirectory(path) {
                nbFiles +=
                    UTIL_prepareFileList(path, bufStart, pos, bufEnd,
                                         followLinks);
                if (*bufStart).is_null() {
                    free(path as *mut libc::c_void);
                    closedir(dir);
                    return 0i32
                }
            } else {
                if (*bufStart).offset(*pos as
                                          isize).offset(pathLength as isize)
                       >= *bufEnd {
                    let mut newListSize: ptrdiff_t =
                        (*bufEnd).wrapping_offset_from(*bufStart) as
                            libc::c_long + (8i32 * 1024i32) as libc::c_long;
                    *bufStart =
                        UTIL_realloc(*bufStart as *mut libc::c_void,
                                     newListSize as size_t) as
                            *mut libc::c_char;
                    *bufEnd = (*bufStart).offset(newListSize as isize);
                    if (*bufStart).is_null() {
                        free(path as *mut libc::c_void);
                        closedir(dir);
                        return 0i32
                    }
                }
                if (*bufStart).offset(*pos as
                                          isize).offset(pathLength as isize) <
                       *bufEnd {
                    memcpy((*bufStart).offset(*pos as isize) as
                               *mut libc::c_void, path as *const libc::c_void,
                           (pathLength + 1i32) as libc::c_ulong);
                    *pos =
                        (*pos as
                             libc::c_ulong).wrapping_add((pathLength + 1i32)
                                                             as libc::c_ulong)
                            as size_t as size_t;
                    nbFiles += 1
                }
            }
            free(path as *mut libc::c_void);
            *__errno_location() = 0i32
        }
    }
    if *__errno_location() != 0i32 {
        if g_utilDisplayLevel >= 1i32 {
            fprintf(stderr,
                    b"readdir(%s) error: %s\n\x00" as *const u8 as
                        *const libc::c_char, dirName,
                    strerror(*__errno_location()));
        }
        free(*bufStart as *mut libc::c_void);
        *bufStart = 0 as *mut libc::c_char
    }
    closedir(dir);
    return nbFiles;
}
/* opendir, readdir require POSIX.1-2001 */
/* opendir, readdir */
/* strerror, memcpy */
/* #ifdef _WIN32 */
/*
 * UTIL_createFileList - takes a list of files and directories (params: inputNames, inputNamesNb), scans directories,
 *                       and returns a new list of files (params: return value, allocatedBuffer, allocatedNamesNb).
 * After finishing usage of the list the structures should be freed with UTIL_freeFileList(params: return value, allocatedBuffer)
 * In case of error UTIL_createFileList returns NULL and UTIL_freeFileList should not be called.
 */
#[no_mangle]
pub unsafe extern "C" fn UTIL_createFileList(mut inputNames:
                                                 *mut *const libc::c_char,
                                             mut inputNamesNb: libc::c_uint,
                                             mut allocatedBuffer:
                                                 *mut *mut libc::c_char,
                                             mut allocatedNamesNb:
                                                 *mut libc::c_uint,
                                             mut followLinks: libc::c_int)
 -> *mut *const libc::c_char {
    let mut pos: size_t = 0;
    let mut i: libc::c_uint = 0;
    let mut nbFiles: libc::c_uint = 0;
    let mut buf: *mut libc::c_char =
        malloc((8i32 * 1024i32) as libc::c_ulong) as *mut libc::c_char;
    let mut bufend: *mut libc::c_char = buf.offset((8i32 * 1024i32) as isize);
    let mut fileTable: *mut *const libc::c_char =
        0 as *mut *const libc::c_char;
    if buf.is_null() { return 0 as *mut *const libc::c_char }
    i = 0i32 as libc::c_uint;
    pos = 0i32 as size_t;
    nbFiles = 0i32 as libc::c_uint;
    while i < inputNamesNb {
        if 0 == UTIL_isDirectory(*inputNames.offset(i as isize)) {
            let len: size_t = strlen(*inputNames.offset(i as isize));
            if buf.offset(pos as isize).offset(len as isize) >= bufend {
                let mut newListSize: ptrdiff_t =
                    bufend.wrapping_offset_from(buf) as libc::c_long +
                        (8i32 * 1024i32) as libc::c_long;
                buf =
                    UTIL_realloc(buf as *mut libc::c_void,
                                 newListSize as size_t) as *mut libc::c_char;
                bufend = buf.offset(newListSize as isize);
                if buf.is_null() { return 0 as *mut *const libc::c_char }
            }
            if buf.offset(pos as isize).offset(len as isize) < bufend {
                memcpy(buf.offset(pos as isize) as *mut libc::c_void,
                       *inputNames.offset(i as isize) as *const libc::c_void,
                       len.wrapping_add(1i32 as libc::c_ulong));
                pos =
                    (pos as
                         libc::c_ulong).wrapping_add(len.wrapping_add(1i32 as
                                                                          libc::c_ulong))
                        as size_t as size_t;
                nbFiles = nbFiles.wrapping_add(1)
            }
        } else {
            nbFiles =
                nbFiles.wrapping_add(UTIL_prepareFileList(*inputNames.offset(i
                                                                                 as
                                                                                 isize),
                                                          &mut buf, &mut pos,
                                                          &mut bufend,
                                                          followLinks) as
                                         libc::c_uint);
            if buf.is_null() { return 0 as *mut *const libc::c_char }
        }
        i = i.wrapping_add(1)
    }
    if nbFiles == 0i32 as libc::c_uint {
        free(buf as *mut libc::c_void);
        return 0 as *mut *const libc::c_char
    }
    fileTable =
        malloc((nbFiles.wrapping_add(1i32 as libc::c_uint) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*const libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *const libc::c_char;
    if fileTable.is_null() {
        free(buf as *mut libc::c_void);
        return 0 as *mut *const libc::c_char
    }
    i = 0i32 as libc::c_uint;
    pos = 0i32 as size_t;
    while i < nbFiles {
        let ref mut fresh0 = *fileTable.offset(i as isize);
        *fresh0 = buf.offset(pos as isize);
        pos =
            (pos as
                 libc::c_ulong).wrapping_add(strlen(*fileTable.offset(i as
                                                                          isize)).wrapping_add(1i32
                                                                                                   as
                                                                                                   libc::c_ulong))
                as size_t as size_t;
        i = i.wrapping_add(1)
    }
    if buf.offset(pos as isize) > bufend {
        free(buf as *mut libc::c_void);
        free(fileTable as *mut libc::c_void);
        return 0 as *mut *const libc::c_char
    }
    *allocatedBuffer = buf;
    *allocatedNamesNb = nbFiles;
    return fileTable;
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_countPhysicalCores() -> libc::c_int {
    let mut current_block: u64;
    static mut numPhysicalCores: libc::c_int = 0i32;
    if numPhysicalCores != 0i32 { return numPhysicalCores }
    numPhysicalCores =
        sysconf(_SC_NPROCESSORS_ONLN as libc::c_int) as libc::c_int;
    if numPhysicalCores == -1i32 {
        numPhysicalCores = 1i32;
        return numPhysicalCores
    }
    /* try to determine if there's hyperthreading */
    let cpuinfo: *mut FILE =
        fopen(b"/proc/cpuinfo\x00" as *const u8 as *const libc::c_char,
              b"r\x00" as *const u8 as *const libc::c_char);
    let mut buff: [libc::c_char; 80] = [0; 80];
    let mut siblings: libc::c_int = 0i32;
    let mut cpu_cores: libc::c_int = 0i32;
    let mut ratio: libc::c_int = 1i32;
    if cpuinfo.is_null() { return numPhysicalCores }
    /* assume the cpu cores/siblings values will be constant across all
         * present processors */
    loop  {
        if !(0 == feof(cpuinfo)) {
            current_block = 18386322304582297246;
            break ;
        }
        if !fgets(buff.as_mut_ptr(), 80i32, cpuinfo).is_null() {
            if strncmp(buff.as_mut_ptr(),
                       b"siblings\x00" as *const u8 as *const libc::c_char,
                       8i32 as libc::c_ulong) == 0i32 {
                let sep: *const libc::c_char =
                    strchr(buff.as_mut_ptr(), ':' as i32);
                if *sep as libc::c_int == '\u{0}' as i32 {
                    /* formatting was broken? */
                    current_block = 10120304801452950108;
                    break ;
                } else { siblings = atoi(sep.offset(1isize)) }
            }
            if !(strncmp(buff.as_mut_ptr(),
                         b"cpu cores\x00" as *const u8 as *const libc::c_char,
                         9i32 as libc::c_ulong) == 0i32) {
                continue ;
            }
            let sep_0: *const libc::c_char =
                strchr(buff.as_mut_ptr(), ':' as i32);
            if *sep_0 as libc::c_int == '\u{0}' as i32 {
                /* formatting was broken? */
                current_block = 10120304801452950108;
                break ;
            } else { cpu_cores = atoi(sep_0.offset(1isize)) }
        } else {
            if !(0 != ferror(cpuinfo)) { continue ; }
            /* fall back on the sysconf value */
            current_block = 10120304801452950108;
            break ;
        }
    }
    match current_block {
        18386322304582297246 => {
            if 0 != siblings && 0 != cpu_cores {
                ratio = siblings / cpu_cores
            }
        }
        _ => { }
    }
    fclose(cpuinfo);
    numPhysicalCores = numPhysicalCores / ratio;
    return numPhysicalCores;
}