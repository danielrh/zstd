#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( extern_types , libc , offset_to , used )]
extern crate libc;
extern "C" {
    pub type ZSTD_CCtx_s;
    pub type _IO_FILE_plus;
    pub type __dirstream;
    pub type ZSTD_DCtx_s;
    pub type ZSTD_DDict_s;
    #[no_mangle]
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn chown(__file: *const libc::c_char, __owner: __uid_t, __group: __gid_t)
     -> libc::c_int;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut opterr: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    static mut _IO_2_1_stdin_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stdout_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stderr_: _IO_FILE_plus;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn utime(__file: *const libc::c_char, __file_times: *const utimbuf)
     -> libc::c_int;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    static mut __tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut __daylight: libc::c_int;
    #[no_mangle]
    static mut __timezone: libc::c_long;
    #[no_mangle]
    static mut tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    fn nanosleep(__requested_time: *const timespec,
                 __remaining: *mut timespec) -> libc::c_int;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn setpriority(__which: __priority_which_t, __who: id_t,
                   __prio: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn ZSTD_compressBound(srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_isError(code: size_t) -> libc::c_uint;
    #[no_mangle]
    fn ZSTD_getErrorName(code: size_t) -> *const libc::c_char;
    #[no_mangle]
    fn ZSTD_maxCLevel() -> libc::c_int;
    #[no_mangle]
    fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
    #[no_mangle]
    fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> size_t;
    #[no_mangle]
    fn ZSTD_createDCtx() -> *mut ZSTD_DCtx;
    #[no_mangle]
    fn ZSTD_freeDCtx(dctx: *mut ZSTD_DCtx) -> size_t;
    #[no_mangle]
    fn ZSTD_createDDict(dictBuffer: *const libc::c_void, dictSize: size_t)
     -> *mut ZSTD_DDict;
    #[no_mangle]
    fn ZSTD_freeDDict(ddict: *mut ZSTD_DDict) -> size_t;
    #[no_mangle]
    fn ZSTD_decompress_usingDDict(dctx: *mut ZSTD_DCtx,
                                  dst: *mut libc::c_void, dstCapacity: size_t,
                                  src: *const libc::c_void, srcSize: size_t,
                                  ddict: *const ZSTD_DDict) -> size_t;
    #[no_mangle]
    fn ZSTD_findDecompressedSize(src: *const libc::c_void, srcSize: size_t)
     -> libc::c_ulonglong;
    #[no_mangle]
    fn ZSTD_CCtx_setParameter(cctx: *mut ZSTD_CCtx, param: ZSTD_cParameter,
                              value: libc::c_uint) -> size_t;
    #[no_mangle]
    fn ZSTD_CCtx_loadDictionary(cctx: *mut ZSTD_CCtx,
                                dict: *const libc::c_void, dictSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compress_generic(cctx: *mut ZSTD_CCtx,
                             output: *mut ZSTD_outBuffer,
                             input: *mut ZSTD_inBuffer,
                             endOp: ZSTD_EndDirective) -> size_t;
    #[no_mangle]
    fn RDG_genBuffer(buffer: *mut libc::c_void, size: size_t,
                     matchProba: libc::c_double, litProba: libc::c_double,
                     seed: libc::c_uint) -> ();
    #[no_mangle]
    fn ZSTD_XXH64(input: *const libc::c_void, length: size_t,
                  seed: libc::c_ulonglong) -> XXH64_hash_t;
}
pub const _SC_TRACE_EVENT_NAME_MAX: unnamed_0 = 242;
pub const _SC_NL_MSGMAX: unnamed_0 = 121;
pub type U32 = uint32_t;
pub const _SC_RTSIG_MAX: unnamed_0 = 31;
pub const _SC_XOPEN_REALTIME_THREADS: unnamed_0 = 131;
pub type uint32_t = libc::c_uint;
pub const _SC_THREAD_SPORADIC_SERVER: unnamed_0 = 161;
pub const _SC_TIMER_MAX: unnamed_0 = 35;
pub const _SC_V6_ILP32_OFF32: unnamed_0 = 176;
pub const ZSTD_p_checksumFlag: ZSTD_cParameter = 201;
pub const _SC_XOPEN_UNIX: unnamed_0 = 91;
pub const PRIO_USER: __priority_which = 2;
pub const _SC_LEVEL1_ICACHE_LINESIZE: unnamed_0 = 187;
pub const _SC_THREAD_PRIORITY_SCHEDULING: unnamed_0 = 79;
pub const _SC_MEMORY_PROTECTION: unnamed_0 = 19;
pub const _SC_SPAWN: unnamed_0 = 159;
pub const _SC_LEVEL4_CACHE_SIZE: unnamed_0 = 197;
pub const _SC_2_PBS_ACCOUNTING: unnamed_0 = 169;
pub const _SC_2_PBS_TRACK: unnamed_0 = 172;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type __mode_t = libc::c_uint;
pub const _SC_LEVEL1_ICACHE_SIZE: unnamed_0 = 185;
pub const _SC_CHARCLASS_NAME_MAX: unnamed_0 = 45;
pub const ZSTD_p_ldmHashLog: ZSTD_cParameter = 161;
pub const ZSTD_p_targetLength: ZSTD_cParameter = 106;
pub const _SC_SYMLOOP_MAX: unnamed_0 = 173;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const ZSTD_p_ldmMinMatch: ZSTD_cParameter = 162;
pub const _SC_SELECT: unnamed_0 = 59;
pub const _SC_BC_SCALE_MAX: unnamed_0 = 38;
pub const _SC_DELAYTIMER_MAX: unnamed_0 = 26;
pub const _SC_COLL_WEIGHTS_MAX: unnamed_0 = 40;
pub const _SC_2_PBS: unnamed_0 = 168;
pub const _SC_REGEX_VERSION: unnamed_0 = 156;
pub type unnamed = libc::c_uint;
pub const _SC_CPUTIME: unnamed_0 = 138;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_compressionParameters {
    pub windowLog: libc::c_uint,
    pub chainLog: libc::c_uint,
    pub hashLog: libc::c_uint,
    pub searchLog: libc::c_uint,
    pub searchLength: libc::c_uint,
    pub targetLength: libc::c_uint,
    pub strategy: ZSTD_strategy,
}
pub const _SC_V6_LPBIG_OFFBIG: unnamed_0 = 179;
pub const _SC_JOB_CONTROL: unnamed_0 = 7;
pub type uint16_t = libc::c_ushort;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: unnamed_0 = 247;
pub const _SC_PII_SOCKET: unnamed_0 = 55;
pub const _SC_NL_LANGMAX: unnamed_0 = 120;
pub const _SC_TTY_NAME_MAX: unnamed_0 = 72;
pub const _SC_V7_LP64_OFF64: unnamed_0 = 239;
pub type __dev_t = libc::c_ulong;
pub const _SC_THREAD_ATTR_STACKADDR: unnamed_0 = 77;
pub const ZSTD_fast: ZSTD_strategy = 1;
pub type __id_t = libc::c_uint;
pub type __priority_which = libc::c_uint;
pub const _SC_CHAR_MIN: unnamed_0 = 103;
pub const ZSTD_e_end: ZSTD_EndDirective = 2;
pub const _SC_XOPEN_XPG4: unnamed_0 = 100;
pub const _SC_BASE: unnamed_0 = 134;
pub const _SC_ASYNCHRONOUS_IO: unnamed_0 = 12;
pub type ZSTD_DDict = ZSTD_DDict_s;
pub const _SC_LEVEL2_CACHE_LINESIZE: unnamed_0 = 193;
pub const _SC_MQ_OPEN_MAX: unnamed_0 = 27;
pub const _SC_PII_OSI_M: unnamed_0 = 65;
pub const _SC_RAW_SOCKETS: unnamed_0 = 236;
pub const _SC_POLL: unnamed_0 = 58;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: unnamed_0 = 73;
pub const _SC_TYPED_MEMORY_OBJECTS: unnamed_0 = 165;
pub const _SC_BC_STRING_MAX: unnamed_0 = 39;
pub const _SC_UIO_MAXIOV: unnamed_0 = 60;
pub const _SC_STREAM_MAX: unnamed_0 = 5;
pub const _SC_MQ_PRIO_MAX: unnamed_0 = 28;
pub const _SC_PII_INTERNET_DGRAM: unnamed_0 = 62;
pub const _SC_XBS5_LP64_OFF64: unnamed_0 = 127;
pub const _SC_SYSTEM_DATABASE_R: unnamed_0 = 163;
pub const _SC_CLK_TCK: unnamed_0 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
}
pub type __nlink_t = libc::c_ulong;
pub const ZSTD_p_enableLongDistanceMatching: ZSTD_cParameter = 160;
pub const _SC_PII_OSI_CLTS: unnamed_0 = 64;
pub type __syscall_slong_t = libc::c_long;
pub const _SC_MEMLOCK: unnamed_0 = 17;
pub const _SC_MEMLOCK_RANGE: unnamed_0 = 18;
pub type clockid_t = __clockid_t;
pub const _SC_SIGNALS: unnamed_0 = 158;
pub const _SC_SPIN_LOCKS: unnamed_0 = 154;
pub const ZSTD_p_nbWorkers: ZSTD_cParameter = 400;
pub type __uid_t = libc::c_uint;
pub const _SC_INT_MIN: unnamed_0 = 105;
pub const _SC_2_PBS_MESSAGE: unnamed_0 = 171;
/* *< position where reading stopped. Will be updated. Necessarily 0 <= pos <= size */
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
pub const _SC_V6_LP64_OFF64: unnamed_0 = 178;
pub const _SC_TRACE_SYS_MAX: unnamed_0 = 244;
pub const ZSTD_lazy: ZSTD_strategy = 4;
pub type __gid_t = libc::c_uint;
pub const ZSTD_p_jobSize: ZSTD_cParameter = 401;
pub const _SC_LEVEL1_ICACHE_ASSOC: unnamed_0 = 186;
pub const _SC_SHARED_MEMORY_OBJECTS: unnamed_0 = 22;
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
pub const ZSTD_p_compressionStrategy: ZSTD_cParameter = 107;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: unnamed_0 = 248;
pub type ZSTD_DCtx = ZSTD_DCtx_s;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BMK_result_t {
    pub cSize: size_t,
    pub cSpeed: libc::c_double,
    pub dSpeed: libc::c_double,
}
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
pub const _SC_SEM_VALUE_MAX: unnamed_0 = 33;
pub const _SC_PASS_MAX: unnamed_0 = 88;
pub const _SC_LEVEL4_CACHE_LINESIZE: unnamed_0 = 199;
pub const _SC_SHRT_MIN: unnamed_0 = 114;
pub const _SC_ATEXIT_MAX: unnamed_0 = 87;
pub type unnamed_0 = libc::c_uint;
pub const PRIO_PROCESS: __priority_which = 0;
pub const ZSTD_p_windowLog: ZSTD_cParameter = 101;
pub const _SC_SHELL: unnamed_0 = 157;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub const _SC_TRACE_INHERIT: unnamed_0 = 183;
pub const _SC_EXPR_NEST_MAX: unnamed_0 = 42;
pub const _SC_SCHAR_MIN: unnamed_0 = 112;
pub const _SC_MAPPED_FILES: unnamed_0 = 16;
pub type __ino_t = libc::c_ulong;
pub const _SC_C_LANG_SUPPORT_R: unnamed_0 = 136;
pub const _SC_FSYNC: unnamed_0 = 15;
pub const _SC_MULTI_PROCESS: unnamed_0 = 150;
pub type U64 = uint64_t;
pub const _SC_PIPE: unnamed_0 = 145;
pub const _SC_SSIZE_MAX: unnamed_0 = 110;
pub const ZSTD_p_format: ZSTD_cParameter = 10;
pub const _SC_PRIORITY_SCHEDULING: unnamed_0 = 10;
pub const _SC_USER_GROUPS_R: unnamed_0 = 167;
pub type stat_t = stat;
pub const _SC_MONOTONIC_CLOCK: unnamed_0 = 149;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub const _SC_INT_MAX: unnamed_0 = 104;
pub const _SC_NL_NMAX: unnamed_0 = 122;
pub const _SC_DEVICE_IO: unnamed_0 = 140;
pub const _SC_THREAD_STACK_MIN: unnamed_0 = 75;
pub const PRIO_PGRP: __priority_which = 1;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const ZSTD_p_forceAttachDict: ZSTD_cParameter = 1101;
pub const ZSTD_e_continue: ZSTD_EndDirective = 0;
pub const _SC_XOPEN_VERSION: unnamed_0 = 89;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BMK_return_t {
    pub errorCode: libc::c_int,
    pub result: BMK_result_t,
}
pub const ZSTD_p_forceMaxWindow: ZSTD_cParameter = 1100;
pub const _SC_XOPEN_XPG2: unnamed_0 = 98;
pub const _SC_REALTIME_SIGNALS: unnamed_0 = 9;
pub const _SC_AVPHYS_PAGES: unnamed_0 = 86;
pub const _SC_FILE_ATTRIBUTES: unnamed_0 = 146;
pub const _SC_FD_MGMT: unnamed_0 = 143;
pub const _SC_THREAD_CPUTIME: unnamed_0 = 139;
pub const _SC_SEMAPHORES: unnamed_0 = 21;
pub type size_t = libc::c_ulong;
pub const _SC_LONG_BIT: unnamed_0 = 106;
pub const _SC_LEVEL2_CACHE_SIZE: unnamed_0 = 191;
pub const _SC_DEVICE_SPECIFIC: unnamed_0 = 141;
pub const _SC_2_PBS_CHECKPOINT: unnamed_0 = 175;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub type UTIL_time_t = timespec;
pub const MEM_static_assert: unnamed = 1;
pub const _SC_TRACE: unnamed_0 = 181;
pub const _SC_LEVEL1_DCACHE_SIZE: unnamed_0 = 188;
pub const _SC_V6_ILP32_OFFBIG: unnamed_0 = 177;
pub const _SC_ARG_MAX: unnamed_0 = 0;
pub const _SC_VERSION: unnamed_0 = 29;
/* ***************************
*  Streaming
****************************/
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
pub const _SC_XOPEN_ENH_I18N: unnamed_0 = 93;
pub const _SC_UCHAR_MAX: unnamed_0 = 115;
pub const _SC_STREAMS: unnamed_0 = 174;
pub type BYTE = uint8_t;
pub type uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
pub const _SC_USHRT_MAX: unnamed_0 = 118;
pub const _SC_TZNAME_MAX: unnamed_0 = 6;
/* **************************************
*  Explicit context
***************************************/
/* !< maximum compression level available */
/* !< provides readable string from an error code */
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub const _SC_V7_ILP32_OFFBIG: unnamed_0 = 238;
pub const _SC_CHILD_MAX: unnamed_0 = 1;
pub const _SC_READER_WRITER_LOCKS: unnamed_0 = 153;
pub const _SC_PII_XTI: unnamed_0 = 54;
pub const _SC_THREAD_SAFE_FUNCTIONS: unnamed_0 = 68;
pub const _SC_PII_INTERNET_STREAM: unnamed_0 = 61;
pub const _SC_XBS5_LPBIG_OFFBIG: unnamed_0 = 128;
pub const _SC_2_SW_DEV: unnamed_0 = 51;
pub const _SC_NPROCESSORS_ONLN: unnamed_0 = 84;
pub const _SC_V7_ILP32_OFF32: unnamed_0 = 237;
pub const _SC_SHRT_MAX: unnamed_0 = 113;
pub const _SC_THREAD_PRIO_PROTECT: unnamed_0 = 81;
pub const _SC_AIO_PRIO_DELTA_MAX: unnamed_0 = 25;
pub const _SC_XOPEN_LEGACY: unnamed_0 = 129;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
pub const _SC_LEVEL2_CACHE_ASSOC: unnamed_0 = 192;
pub const _SC_XOPEN_XPG3: unnamed_0 = 99;
pub const _SC_SINGLE_PROCESS: unnamed_0 = 151;
pub const _SC_2_C_BIND: unnamed_0 = 47;
pub type time_t = __time_t;
pub const _SC_PAGESIZE: unnamed_0 = 30;
pub const ZSTD_p_compressionLevel: ZSTD_cParameter = 100;
pub const _SC_EQUIV_CLASS_MAX: unnamed_0 = 41;
pub const _SC_PHYS_PAGES: unnamed_0 = 85;
pub const _SC_NL_SETMAX: unnamed_0 = 123;
pub const _SC_WORD_BIT: unnamed_0 = 107;
/* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
pub type _IO_lock_t = ();
pub const _SC_CLOCK_SELECTION: unnamed_0 = 137;
pub type XXH64_hash_t = libc::c_ulonglong;
pub const _SC_LEVEL4_CACHE_ASSOC: unnamed_0 = 198;
pub const ZSTD_p_ldmHashEveryLog: ZSTD_cParameter = 164;
pub type DIR = __dirstream;
pub const _SC_TIMERS: unnamed_0 = 11;
#[derive ( Copy , Clone )]
#[repr ( C )]
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
pub type ZSTD_EndDirective = libc::c_uint;
pub const _SC_XOPEN_SHM: unnamed_0 = 94;
pub const _SC_OPEN_MAX: unnamed_0 = 4;
pub const _SC_LEVEL1_DCACHE_ASSOC: unnamed_0 = 189;
pub const _SC_FILE_SYSTEM: unnamed_0 = 148;
pub const _SC_GETPW_R_SIZE_MAX: unnamed_0 = 70;
pub const _SC_TRACE_USER_EVENT_MAX: unnamed_0 = 245;
pub const _SC_THREAD_PRIO_INHERIT: unnamed_0 = 80;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const _SC_IPV6: unnamed_0 = 235;
pub const ZSTD_btopt: ZSTD_strategy = 7;
pub const _SC_THREAD_THREADS_MAX: unnamed_0 = 76;
pub type ZSTD_strategy = libc::c_uint;
pub type __syscall_ulong_t = libc::c_ulong;
pub type __blkcnt_t = libc::c_long;
pub type ptrdiff_t = libc::c_long;
pub const _SC_2_C_DEV: unnamed_0 = 48;
pub type __time_t = libc::c_long;
pub type uint64_t = libc::c_ulong;
pub const _SC_SCHAR_MAX: unnamed_0 = 111;
pub const _SC_AIO_MAX: unnamed_0 = 24;
pub const _SC_SYNCHRONIZED_IO: unnamed_0 = 14;
pub const _SC_PRIORITIZED_IO: unnamed_0 = 13;
pub const _SC_ULONG_MAX: unnamed_0 = 117;
pub const _SC_LEVEL1_DCACHE_LINESIZE: unnamed_0 = 190;
pub const _SC_CHAR_MAX: unnamed_0 = 102;
pub const _SC_2_FORT_RUN: unnamed_0 = 50;
pub const _SC_PII_OSI: unnamed_0 = 57;
pub const _SC_PII_OSI_COTS: unnamed_0 = 63;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const _SC_NZERO: unnamed_0 = 109;
pub const _SC_V7_LPBIG_OFFBIG: unnamed_0 = 240;
pub const _SC_SPORADIC_SERVER: unnamed_0 = 160;
pub const _SC_LOGIN_NAME_MAX: unnamed_0 = 71;
pub const _SC_TRACE_LOG: unnamed_0 = 184;
pub const _SC_NGROUPS_MAX: unnamed_0 = 3;
pub const _SC_FILE_LOCKING: unnamed_0 = 147;
pub const ZSTD_p_overlapSizeLog: ZSTD_cParameter = 402;
pub const _SC_SS_REPL_MAX: unnamed_0 = 241;
pub const _SC_MB_LEN_MAX: unnamed_0 = 108;
pub const _SC_LEVEL3_CACHE_ASSOC: unnamed_0 = 195;
pub type id_t = __id_t;
pub const _SC_BC_DIM_MAX: unnamed_0 = 37;
pub const _SC_DEVICE_SPECIFIC_R: unnamed_0 = 142;
pub const _SC_PII_INTERNET: unnamed_0 = 56;
pub const _SC_NPROCESSORS_CONF: unnamed_0 = 83;
pub const _SC_TRACE_EVENT_FILTER: unnamed_0 = 182;
pub const _SC_THREAD_KEYS_MAX: unnamed_0 = 74;
pub const _SC_LINE_MAX: unnamed_0 = 43;
pub const _SC_NL_TEXTMAX: unnamed_0 = 124;
pub type __blksize_t = libc::c_long;
pub const _SC_NL_ARGMAX: unnamed_0 = 119;
pub const _SC_HOST_NAME_MAX: unnamed_0 = 180;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_inBuffer_s {
    pub src: *const libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub const _SC_GETGR_R_SIZE_MAX: unnamed_0 = 69;
pub const ZSTD_p_dictIDFlag: ZSTD_cParameter = 202;
pub const _SC_ADVISORY_INFO: unnamed_0 = 132;
pub const _SC_MESSAGE_PASSING: unnamed_0 = 20;
pub const _SC_2_CHAR_TERM: unnamed_0 = 95;
pub const _SC_XOPEN_CRYPT: unnamed_0 = 92;
pub type FILE = _IO_FILE;
pub const _SC_2_FORT_DEV: unnamed_0 = 49;
pub const _SC_USER_GROUPS: unnamed_0 = 166;
pub const _SC_LEVEL3_CACHE_LINESIZE: unnamed_0 = 196;
pub type __off64_t = libc::c_long;
pub const ZSTD_p_minMatch: ZSTD_cParameter = 105;
pub const _SC_PII: unnamed_0 = 53;
pub const ZSTD_p_chainLog: ZSTD_cParameter = 103;
pub const _SC_THREAD_PROCESS_SHARED: unnamed_0 = 82;
pub const _SC_AIO_LISTIO_MAX: unnamed_0 = 23;
pub const _SC_SYSTEM_DATABASE: unnamed_0 = 162;
pub const _SC_THREADS: unnamed_0 = 67;
pub const _SC_T_IOV_MAX: unnamed_0 = 66;
pub type ZSTD_cParameter = libc::c_uint;
pub const _SC_BC_BASE_MAX: unnamed_0 = 36;
pub const _SC_REGEXP: unnamed_0 = 155;
pub const _SC_BARRIERS: unnamed_0 = 133;
pub type U16 = uint16_t;
pub const ZSTD_e_flush: ZSTD_EndDirective = 1;
pub const ZSTD_p_ldmBucketSizeLog: ZSTD_cParameter = 163;
pub const _SC_UINT_MAX: unnamed_0 = 116;
pub const _SC_NETWORKING: unnamed_0 = 152;
pub const _SC_2_PBS_LOCATE: unnamed_0 = 170;
pub type __priority_which_t = libc::c_int;
pub const _SC_2_LOCALEDEF: unnamed_0 = 52;
pub const ZSTD_p_hashLog: ZSTD_cParameter = 102;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
pub type __clockid_t = libc::c_int;
pub const _SC_CHAR_BIT: unnamed_0 = 101;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct blockParam_t {
    pub srcPtr: *const libc::c_void,
    pub srcSize: size_t,
    pub cPtr: *mut libc::c_void,
    pub cRoom: size_t,
    pub cSize: size_t,
    pub resPtr: *mut libc::c_void,
    pub resSize: size_t,
}
pub const _SC_XBS5_ILP32_OFF32: unnamed_0 = 125;
pub const _SC_SAVED_IDS: unnamed_0 = 8;
pub const _SC_XBS5_ILP32_OFFBIG: unnamed_0 = 126;
pub type __off_t = libc::c_long;
pub const _SC_XOPEN_STREAMS: unnamed_0 = 246;
#[derive ( Copy , Clone )]
#[repr ( C )]
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
pub const ZSTD_p_contentSizeFlag: ZSTD_cParameter = 200;
pub const _SC_SEM_NSEMS_MAX: unnamed_0 = 32;
pub const _SC_XOPEN_XCU_VERSION: unnamed_0 = 90;
pub const _SC_THREAD_ATTR_STACKSIZE: unnamed_0 = 78;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    u: U32,
    c: [BYTE; 4],
}
pub const _SC_RE_DUP_MAX: unnamed_0 = 44;
pub const _SC_IOV_MAX: unnamed_0 = 60;
pub const ZSTD_p_searchLog: ZSTD_cParameter = 104;
pub const _SC_C_LANG_SUPPORT: unnamed_0 = 135;
pub const _SC_2_VERSION: unnamed_0 = 46;
pub const _SC_2_UPE: unnamed_0 = 97;
pub const _SC_XOPEN_REALTIME: unnamed_0 = 130;
pub const _SC_FIFO: unnamed_0 = 144;
pub const _SC_TRACE_NAME_MAX: unnamed_0 = 243;
pub const _SC_LEVEL3_CACHE_SIZE: unnamed_0 = 194;
pub const _SC_TIMEOUTS: unnamed_0 = 164;
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub const _SC_SIGQUEUE_MAX: unnamed_0 = 34;
pub const _SC_2_C_VERSION: unnamed_0 = 96;
unsafe extern "C" fn MEM_check() -> () { }
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::std::mem::size_of::<size_t>() as libc::c_ulong ==
                4i32 as libc::c_ulong) as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn MEM_64bits() -> libc::c_uint {
    return (::std::mem::size_of::<size_t>() as libc::c_ulong ==
                8i32 as libc::c_ulong) as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    let one: unnamed_1 = unnamed_1{u: 1i32 as U32,};
    return one.c[0usize] as libc::c_uint;
}
unsafe extern "C" fn MEM_read16(mut ptr: *const libc::c_void) -> U16 {
    return (*(ptr as *const unalign16)).v;
}
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> U32 {
    return (*(ptr as *const unalign32)).v;
}
unsafe extern "C" fn MEM_read64(mut ptr: *const libc::c_void) -> U64 {
    return (*(ptr as *const unalign64)).v;
}
unsafe extern "C" fn MEM_readST(mut ptr: *const libc::c_void) -> size_t {
    return (*(ptr as *const unalignArch)).v;
}
unsafe extern "C" fn MEM_write16(mut memPtr: *mut libc::c_void,
                                 mut value: U16) -> () {
    (*(memPtr as *mut unalign16)).v = value;
}
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void,
                                 mut value: U32) -> () {
    (*(memPtr as *mut unalign32)).v = value;
}
unsafe extern "C" fn MEM_write64(mut memPtr: *mut libc::c_void,
                                 mut value: U64) -> () {
    (*(memPtr as *mut unalign64)).v = value;
}
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0 << 24i32 & 4278190080u32 |
               in_0 << 8i32 & 16711680i32 as libc::c_uint |
               in_0 >> 8i32 & 65280i32 as libc::c_uint |
               in_0 >> 24i32 & 255i32 as libc::c_uint;
}
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return ((in_0 << 56i32) as libc::c_ulonglong & 18374686479671623680u64 |
                (in_0 << 40i32) as libc::c_ulonglong & 71776119061217280u64 |
                (in_0 << 24i32) as libc::c_ulonglong & 280375465082880u64 |
                (in_0 << 8i32) as libc::c_ulonglong & 1095216660480u64 |
                (in_0 >> 8i32) as libc::c_ulonglong & 4278190080u64 |
                (in_0 >> 24i32) as libc::c_ulonglong & 16711680u64 |
                (in_0 >> 40i32) as libc::c_ulonglong & 65280u64 |
                (in_0 >> 56i32) as libc::c_ulonglong & 255u64) as U64;
}
unsafe extern "C" fn MEM_swapST(mut in_0: size_t) -> size_t {
    if 0 != MEM_32bits() {
        return MEM_swap32(in_0 as U32) as size_t
    } else { return MEM_swap64(in_0) };
}
unsafe extern "C" fn MEM_readLE16(mut memPtr: *const libc::c_void) -> U16 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read16(memPtr)
    } else {
        let mut p: *const BYTE = memPtr as *const BYTE;
        return (*p.offset(0isize) as libc::c_int +
                    ((*p.offset(1isize) as libc::c_int) << 8i32)) as U16
    };
}
unsafe extern "C" fn MEM_writeLE16(mut memPtr: *mut libc::c_void,
                                   mut val: U16) -> () {
    if 0 != MEM_isLittleEndian() {
        MEM_write16(memPtr, val);
    } else {
        let mut p: *mut BYTE = memPtr as *mut BYTE;
        *p.offset(0isize) = val as BYTE;
        *p.offset(1isize) = (val as libc::c_int >> 8i32) as BYTE
    };
}
unsafe extern "C" fn MEM_readLE24(mut memPtr: *const libc::c_void) -> U32 {
    return (MEM_readLE16(memPtr) as libc::c_int +
                ((*(memPtr as *const BYTE).offset(2isize) as libc::c_int) <<
                     16i32)) as U32;
}
unsafe extern "C" fn MEM_writeLE24(mut memPtr: *mut libc::c_void,
                                   mut val: U32) -> () {
    MEM_writeLE16(memPtr, val as U16);
    *(memPtr as *mut BYTE).offset(2isize) = (val >> 16i32) as BYTE;
}
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const libc::c_void) -> U32 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read32(memPtr)
    } else { return MEM_swap32(MEM_read32(memPtr)) };
}
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut libc::c_void,
                                   mut val32: U32) -> () {
    if 0 != MEM_isLittleEndian() {
        MEM_write32(memPtr, val32);
    } else { MEM_write32(memPtr, MEM_swap32(val32)); };
}
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const libc::c_void) -> U64 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read64(memPtr)
    } else { return MEM_swap64(MEM_read64(memPtr)) };
}
unsafe extern "C" fn MEM_writeLE64(mut memPtr: *mut libc::c_void,
                                   mut val64: U64) -> () {
    if 0 != MEM_isLittleEndian() {
        MEM_write64(memPtr, val64);
    } else { MEM_write64(memPtr, MEM_swap64(val64)); };
}
unsafe extern "C" fn MEM_readLEST(mut memPtr: *const libc::c_void) -> size_t {
    if 0 != MEM_32bits() {
        return MEM_readLE32(memPtr) as size_t
    } else { return MEM_readLE64(memPtr) };
}
unsafe extern "C" fn MEM_writeLEST(mut memPtr: *mut libc::c_void,
                                   mut val: size_t) -> () {
    if 0 != MEM_32bits() {
        MEM_writeLE32(memPtr, val as U32);
    } else { MEM_writeLE64(memPtr, val); };
}
unsafe extern "C" fn MEM_readBE32(mut memPtr: *const libc::c_void) -> U32 {
    if 0 != MEM_isLittleEndian() {
        return MEM_swap32(MEM_read32(memPtr))
    } else { return MEM_read32(memPtr) };
}
unsafe extern "C" fn MEM_writeBE32(mut memPtr: *mut libc::c_void,
                                   mut val32: U32) -> () {
    if 0 != MEM_isLittleEndian() {
        MEM_write32(memPtr, MEM_swap32(val32));
    } else { MEM_write32(memPtr, val32); };
}
unsafe extern "C" fn MEM_readBE64(mut memPtr: *const libc::c_void) -> U64 {
    if 0 != MEM_isLittleEndian() {
        return MEM_swap64(MEM_read64(memPtr))
    } else { return MEM_read64(memPtr) };
}
unsafe extern "C" fn MEM_writeBE64(mut memPtr: *mut libc::c_void,
                                   mut val64: U64) -> () {
    if 0 != MEM_isLittleEndian() {
        MEM_write64(memPtr, MEM_swap64(val64));
    } else { MEM_write64(memPtr, val64); };
}
unsafe extern "C" fn MEM_readBEST(mut memPtr: *const libc::c_void) -> size_t {
    if 0 != MEM_32bits() {
        return MEM_readBE32(memPtr) as size_t
    } else { return MEM_readBE64(memPtr) };
}
unsafe extern "C" fn MEM_writeBEST(mut memPtr: *mut libc::c_void,
                                   mut val: size_t) -> () {
    if 0 != MEM_32bits() {
        MEM_writeBE32(memPtr, val as U32);
    } else { MEM_writeBE64(memPtr, val); };
}
static mut g_utilDisplayLevel: libc::c_int = unsafe { 0 };
unsafe extern "C" fn UTIL_getTime() -> UTIL_time_t {
    let mut time_0: UTIL_time_t = timespec{tv_sec: 0, tv_nsec: 0,};
    if 0 != clock_gettime(1i32, &mut time_0 as *mut UTIL_time_t) {
        if g_utilDisplayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 27],
                                              &mut [libc::c_char; 27]>(b"ERROR: Failed to get time\n\x00")).as_mut_ptr());
        }
    }
    return time_0;
}
unsafe extern "C" fn UTIL_getSpanTime(mut begin: UTIL_time_t,
                                      mut end: UTIL_time_t) -> UTIL_time_t {
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
unsafe extern "C" fn UTIL_getSpanTimeMicro(mut begin: UTIL_time_t,
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
unsafe extern "C" fn UTIL_getSpanTimeNano(mut begin: UTIL_time_t,
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
unsafe extern "C" fn UTIL_clockSpanMicro(mut clockStart: UTIL_time_t) -> U64 {
    let clockEnd: UTIL_time_t = UTIL_getTime();
    return UTIL_getSpanTimeMicro(clockStart, clockEnd);
}
unsafe extern "C" fn UTIL_clockSpanNano(mut clockStart: UTIL_time_t) -> U64 {
    let clockEnd: UTIL_time_t = UTIL_getTime();
    return UTIL_getSpanTimeNano(clockStart, clockEnd);
}
unsafe extern "C" fn UTIL_waitForNextTick() -> () {
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
unsafe extern "C" fn UTIL_isRegularFile(mut infilename: *const libc::c_char)
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
    return UTIL_getFileStat(infilename, &mut statbuf as *mut stat_t);
}
unsafe extern "C" fn UTIL_getFileStat(mut infilename: *const libc::c_char,
                                      mut statbuf: *mut stat_t)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = stat(infilename, statbuf);
    if 0 != r ||
           !((*statbuf).st_mode & 61440i32 as libc::c_uint ==
                 32768i32 as libc::c_uint) {
        return 0i32
    } else { return 1i32 };
}
unsafe extern "C" fn UTIL_setFileStat(mut filename: *const libc::c_char,
                                      mut statbuf: *mut stat_t)
 -> libc::c_int {
    let mut res: libc::c_int = 0i32;
    let mut timebuf: utimbuf = utimbuf{actime: 0, modtime: 0,};
    if 0 == UTIL_isRegularFile(filename) {
        return -1i32
    } else {
        timebuf.actime = time(0 as *mut time_t);
        timebuf.modtime = (*statbuf).st_mtime;
        res += utime(filename, &mut timebuf as *mut utimbuf);
        res += chown(filename, (*statbuf).st_uid, (*statbuf).st_gid);
        res += chmod(filename, (*statbuf).st_mode & 4095i32 as libc::c_uint);
        *__errno_location() = 0i32;
        return -res
    };
}
unsafe extern "C" fn UTIL_isDirectory(mut infilename: *const libc::c_char)
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
    r = stat(infilename, &mut statbuf as *mut stat_t);
    if 0 == r &&
           statbuf.st_mode & 61440i32 as libc::c_uint ==
               16384i32 as libc::c_uint {
        return 1i32 as U32
    } else { return 0i32 as U32 };
}
unsafe extern "C" fn UTIL_isLink(mut infilename: *const libc::c_char) -> U32 {
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
    r = lstat(infilename, &mut statbuf as *mut stat_t);
    if 0 == r &&
           statbuf.st_mode & 61440i32 as libc::c_uint ==
               40960i32 as libc::c_uint {
        return 1i32 as U32
    } else { return 0i32 as U32 };
}
unsafe extern "C" fn UTIL_getFileSize(mut infilename: *const libc::c_char)
 -> U64 {
    if 0 == UTIL_isRegularFile(infilename) {
        return -1i32 as U64
    } else {
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
        r = stat(infilename, &mut statbuf as *mut stat);
        if 0 != r ||
               !(statbuf.st_mode & 61440i32 as libc::c_uint ==
                     32768i32 as libc::c_uint) {
            return -1i32 as U64
        } else { return statbuf.st_size as U64 }
    };
}
unsafe extern "C" fn UTIL_getTotalFileSize(fileNamesTable:
                                               *const *const libc::c_char,
                                           mut nbFiles: libc::c_uint) -> U64 {
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
unsafe extern "C" fn UTIL_realloc(mut ptr: *mut libc::c_void,
                                  mut size: size_t) -> *mut libc::c_void {
    let mut newptr: *mut libc::c_void = realloc(ptr, size);
    if !newptr.is_null() {
        return newptr
    } else { free(ptr); return 0 as *mut libc::c_void };
}
unsafe extern "C" fn UTIL_prepareFileList(mut dirName: *const libc::c_char,
                                          mut bufStart:
                                              *mut *mut libc::c_char,
                                          mut pos: *mut size_t,
                                          mut bufEnd: *mut *mut libc::c_char,
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
                    (*::std::mem::transmute::<&[u8; 32],
                                              &mut [libc::c_char; 32]>(b"Cannot open directory \'%s\': %s\n\x00")).as_mut_ptr(),
                    dirName, strerror(*__errno_location()));
        }
        return 0i32
    } else {
        dirLength = strlen(dirName) as libc::c_int;
        *__errno_location() = 0i32;
        loop  {
            entry = readdir(dir);
            if entry.is_null() { break ; }
            if strcmp((*entry).d_name.as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 3],
                                                &mut [libc::c_char; 3]>(b"..\x00")).as_mut_ptr())
                   == 0i32 ||
                   strcmp((*entry).d_name.as_mut_ptr(),
                          (*::std::mem::transmute::<&[u8; 2],
                                                    &mut [libc::c_char; 2]>(b".\x00")).as_mut_ptr())
                       == 0i32 {
                continue ;
            }
            fnameLength = strlen((*entry).d_name.as_mut_ptr()) as libc::c_int;
            path =
                malloc((dirLength + fnameLength + 2i32) as libc::c_ulong) as
                    *mut libc::c_char;
            if path.is_null() {
                closedir(dir);
                return 0i32
            } else {
                memcpy(path as *mut libc::c_void,
                       dirName as *const libc::c_void,
                       dirLength as libc::c_ulong);
                *path.offset(dirLength as isize) = '/' as i32 as libc::c_char;
                memcpy(path.offset(dirLength as isize).offset(1isize) as
                           *mut libc::c_void,
                       (*entry).d_name.as_mut_ptr() as *const libc::c_void,
                       fnameLength as libc::c_ulong);
                pathLength = dirLength + 1i32 + fnameLength;
                *path.offset(pathLength as isize) = 0i32 as libc::c_char;
                if 0 == followLinks && 0 != UTIL_isLink(path) {
                    if !(g_utilDisplayLevel >= 2i32) { continue ; }
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 43],
                                                      &mut [libc::c_char; 43]>(b"Warning : %s is a symbolic link, ignoring\n\x00")).as_mut_ptr(),
                            path);
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
                                                  isize).offset(pathLength as
                                                                    isize) >=
                               *bufEnd {
                            let mut newListSize: ptrdiff_t =
                                (*bufStart).offset_to(*bufEnd).expect("bad offset_to")
                                    as libc::c_long +
                                    (8i32 * 1024i32) as libc::c_long;
                            *bufStart =
                                UTIL_realloc(*bufStart as *mut libc::c_void,
                                             newListSize as size_t) as
                                    *mut libc::c_char;
                            *bufEnd =
                                (*bufStart).offset(newListSize as isize);
                            if (*bufStart).is_null() {
                                free(path as *mut libc::c_void);
                                closedir(dir);
                                return 0i32
                            }
                        }
                        if (*bufStart).offset(*pos as
                                                  isize).offset(pathLength as
                                                                    isize) <
                               *bufEnd {
                            strncpy((*bufStart).offset(*pos as isize), path,
                                    (*bufStart).offset(*pos as
                                                           isize).offset_to(*bufEnd).expect("bad offset_to")
                                        as libc::c_long as libc::c_ulong);
                            *pos =
                                (*pos as
                                     libc::c_ulong).wrapping_add((pathLength +
                                                                      1i32) as
                                                                     libc::c_ulong)
                                    as size_t as size_t;
                            nbFiles += 1
                        }
                    }
                    free(path as *mut libc::c_void);
                    *__errno_location() = 0i32
                }
            }
        }
        if *__errno_location() != 0i32 {
            if g_utilDisplayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 23],
                                                  &mut [libc::c_char; 23]>(b"readdir(%s) error: %s\n\x00")).as_mut_ptr(),
                        dirName, strerror(*__errno_location()));
            }
            free(*bufStart as *mut libc::c_void);
            *bufStart = 0 as *mut libc::c_char
        }
        closedir(dir);
        return nbFiles
    };
}
unsafe extern "C" fn UTIL_createFileList(mut inputNames:
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
    if buf.is_null() {
        return 0 as *mut *const libc::c_char
    } else {
        i = 0i32 as libc::c_uint;
        pos = 0i32 as size_t;
        nbFiles = 0i32 as libc::c_uint;
        while i < inputNamesNb {
            if 0 == UTIL_isDirectory(*inputNames.offset(i as isize)) {
                let len: size_t = strlen(*inputNames.offset(i as isize));
                if buf.offset(pos as isize).offset(len as isize) >= bufend {
                    let mut newListSize: ptrdiff_t =
                        buf.offset_to(bufend).expect("bad offset_to") as
                            libc::c_long + (8i32 * 1024i32) as libc::c_long;
                    buf =
                        UTIL_realloc(buf as *mut libc::c_void,
                                     newListSize as size_t) as
                            *mut libc::c_char;
                    bufend = buf.offset(newListSize as isize);
                    if buf.is_null() { return 0 as *mut *const libc::c_char }
                }
                if buf.offset(pos as isize).offset(len as isize) < bufend {
                    strncpy(buf.offset(pos as isize),
                            *inputNames.offset(i as isize),
                            buf.offset(pos as
                                           isize).offset_to(bufend).expect("bad offset_to")
                                as libc::c_long as libc::c_ulong);
                    pos =
                        (pos as
                             libc::c_ulong).wrapping_add(len.wrapping_add(1i32
                                                                              as
                                                                              libc::c_ulong))
                            as size_t as size_t;
                    nbFiles = nbFiles.wrapping_add(1)
                }
            } else {
                nbFiles =
                    nbFiles.wrapping_add(UTIL_prepareFileList(*inputNames.offset(i
                                                                                     as
                                                                                     isize),
                                                              &mut buf as
                                                                  *mut *mut libc::c_char,
                                                              &mut pos as
                                                                  *mut size_t,
                                                              &mut bufend as
                                                                  *mut *mut libc::c_char,
                                                              followLinks) as
                                             libc::c_uint);
                if buf.is_null() { return 0 as *mut *const libc::c_char }
            }
            i = i.wrapping_add(1)
        }
        if nbFiles == 0i32 as libc::c_uint {
            free(buf as *mut libc::c_void);
            return 0 as *mut *const libc::c_char
        } else {
            fileTable =
                malloc((nbFiles.wrapping_add(1i32 as libc::c_uint) as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<*const libc::c_char>()
                                                            as libc::c_ulong))
                    as *mut *const libc::c_char;
            if fileTable.is_null() {
                free(buf as *mut libc::c_void);
                return 0 as *mut *const libc::c_char
            } else {
                i = 0i32 as libc::c_uint;
                pos = 0i32 as size_t;
                while i < nbFiles {
                    let ref mut fresh0 = *fileTable.offset(i as isize);
                    *fresh0 = buf.offset(pos as isize);
                    pos =
                        (pos as
                             libc::c_ulong).wrapping_add(strlen(*fileTable.offset(i
                                                                                      as
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
                } else {
                    *allocatedBuffer = buf;
                    *allocatedNamesNb = nbFiles;
                    return fileTable
                }
            }
        }
    };
}
unsafe extern "C" fn UTIL_freeFileList(mut filenameTable:
                                           *mut *const libc::c_char,
                                       mut allocatedBuffer: *mut libc::c_char)
 -> () {
    if !allocatedBuffer.is_null() {
        free(allocatedBuffer as *mut libc::c_void);
    }
    if !filenameTable.is_null() { free(filenameTable as *mut libc::c_void); };
}
unsafe extern "C" fn UTIL_countPhysicalCores() -> libc::c_int {
    let mut current_block: u64;
    static mut numPhysicalCores: libc::c_int = unsafe { 0i32 };
    if numPhysicalCores != 0i32 {
        return numPhysicalCores
    } else {
        numPhysicalCores =
            sysconf(_SC_NPROCESSORS_ONLN as libc::c_int) as libc::c_int;
        if numPhysicalCores == -1i32 {
            numPhysicalCores = 1i32;
            return numPhysicalCores
        } else {
            let cpuinfo: *mut FILE =
                fopen((*::std::mem::transmute::<&[u8; 14],
                                                &mut [libc::c_char; 14]>(b"/proc/cpuinfo\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 2],
                                                &mut [libc::c_char; 2]>(b"r\x00")).as_mut_ptr());
            let mut buff: [libc::c_char; 80] = [0; 80];
            let mut siblings: libc::c_int = 0i32;
            let mut cpu_cores: libc::c_int = 0i32;
            let mut ratio: libc::c_int = 1i32;
            if cpuinfo.is_null() {
                return numPhysicalCores
            } else {
                loop  {
                    if !(0 == feof(cpuinfo)) {
                        current_block = 7095457783677275021;
                        break ;
                    }
                    if !fgets(buff.as_mut_ptr(), 80i32, cpuinfo).is_null() {
                        if strncmp(buff.as_mut_ptr(),
                                   (*::std::mem::transmute::<&[u8; 9],
                                                             &mut [libc::c_char; 9]>(b"siblings\x00")).as_mut_ptr(),
                                   8i32 as libc::c_ulong) == 0i32 {
                            let sep: *const libc::c_char =
                                strchr(buff.as_mut_ptr(), ':' as i32);
                            if *sep as libc::c_int == '\u{0}' as i32 {
                                current_block = 867551788711315662;
                                break ;
                            }
                            siblings = atoi(sep.offset(1isize))
                        }
                        if !(strncmp(buff.as_mut_ptr(),
                                     (*::std::mem::transmute::<&[u8; 10],
                                                               &mut [libc::c_char; 10]>(b"cpu cores\x00")).as_mut_ptr(),
                                     9i32 as libc::c_ulong) == 0i32) {
                            continue ;
                        }
                        let sep_0: *const libc::c_char =
                            strchr(buff.as_mut_ptr(), ':' as i32);
                        if *sep_0 as libc::c_int == '\u{0}' as i32 {
                            current_block = 867551788711315662;
                            break ;
                        }
                        cpu_cores = atoi(sep_0.offset(1isize))
                    } else if 0 != ferror(cpuinfo) {
                        current_block = 867551788711315662;
                        break ;
                    }
                }
                match current_block {
                    7095457783677275021 => {
                        if 0 != siblings && 0 != cpu_cores {
                            ratio = siblings / cpu_cores
                        }
                    }
                    _ => { }
                }
                fclose(cpuinfo);
                numPhysicalCores = numPhysicalCores / ratio;
                return numPhysicalCores
            }
        }
    };
}
/* ***************************************************************************************
 * START OF ADVANCED AND EXPERIMENTAL FUNCTIONS
 * The definitions in this section are considered experimental.
 * They should never be used with a dynamic library, as prototypes may change in the future.
 * They are provided for advanced scenarios.
 * Use them only in association with static linking.
 * ***************************************************************************************/
/* ***************************************************************************************
 * START OF ADVANCED AND EXPERIMENTAL FUNCTIONS
 * The definitions in this section are considered experimental.
 * They should never be used with a dynamic library, as prototypes may change in the future.
 * They are provided for advanced scenarios.
 * Use them only in association with static linking.
 * ***************************************************************************************/
/* !< recommended size for output buffer. Guarantee to successfully flush at least one complete block in all circumstances. */
/* !< recommended size for input buffer */
static mut ZSTD_frameHeaderSize_prefix: size_t = unsafe { 5i32 as size_t };
static mut ZSTD_frameHeaderSize_min: size_t = unsafe { 6i32 as size_t };
static mut ZSTD_frameHeaderSize_max: size_t = unsafe { 18i32 as size_t };
static mut ZSTD_skippableHeaderSize: size_t = unsafe { 8i32 as size_t };
static mut ZSTD_defaultCMem: ZSTD_customMem =
    unsafe {
        ZSTD_customMem{customAlloc: None,
                       customFree: None,
                       opaque: 0 as *const libc::c_void as *mut libc::c_void,}
    };
#[no_mangle]
pub unsafe extern "C" fn BMK_benchFiles(mut fileNamesTable:
                                            *mut *const libc::c_char,
                                        mut nbFiles: libc::c_uint,
                                        mut dictFileName: *const libc::c_char,
                                        mut cLevel: libc::c_int,
                                        mut cLevelLast: libc::c_int,
                                        mut compressionParams:
                                            *const ZSTD_compressionParameters,
                                        mut displayLevel: libc::c_int)
 -> libc::c_int {
    BMK_benchFilesFull(fileNamesTable, nbFiles, dictFileName, cLevel,
                       cLevelLast, compressionParams, displayLevel);
    return 0i32;
}
unsafe extern "C" fn BMK_benchFilesFull(mut fileNamesTable:
                                            *mut *const libc::c_char,
                                        mut nbFiles: libc::c_uint,
                                        mut dictFileName: *const libc::c_char,
                                        mut cLevel: libc::c_int,
                                        mut cLevelLast: libc::c_int,
                                        mut compressionParams:
                                            *const ZSTD_compressionParameters,
                                        mut displayLevel: libc::c_int) -> () {
    let compressibility: libc::c_double =
        g_compressibilityDefault as libc::c_double / 100i32 as libc::c_double;
    if cLevel > ZSTD_maxCLevel() { cLevel = ZSTD_maxCLevel() }
    if cLevelLast > ZSTD_maxCLevel() { cLevelLast = ZSTD_maxCLevel() }
    if cLevelLast < cLevel { cLevelLast = cLevel }
    if cLevelLast > cLevel {
        if displayLevel >= 2i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 35],
                                              &mut [libc::c_char; 35]>(b"Benchmarking levels from %d to %d\n\x00")).as_mut_ptr(),
                    cLevel, cLevelLast);
        }
    }
    if nbFiles == 0i32 as libc::c_uint {
        BMK_syntheticTest(cLevel, cLevelLast, compressibility,
                          compressionParams, displayLevel);
    } else {
        BMK_benchFileTable(fileNamesTable, nbFiles, dictFileName, cLevel,
                           cLevelLast, compressionParams, displayLevel);
    };
}
unsafe extern "C" fn BMK_benchFileTable(fileNamesTable:
                                            *const *const libc::c_char,
                                        nbFiles: libc::c_uint,
                                        dictFileName: *const libc::c_char,
                                        cLevel: libc::c_int,
                                        cLevelLast: libc::c_int,
                                        compressionParams:
                                            *const ZSTD_compressionParameters,
                                        mut displayLevel: libc::c_int) -> () {
    let mut srcBuffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut benchedSize: size_t = 0;
    let mut dictBuffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut dictBufferSize: size_t = 0i32 as size_t;
    let fileSizes: *mut size_t =
        malloc((nbFiles as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>()
                                                    as libc::c_ulong)) as
            *mut size_t;
    let totalSizeToLoad: U64 = UTIL_getTotalFileSize(fileNamesTable, nbFiles);
    if fileSizes.is_null() {
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                    12i32);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 32],
                                              &mut [libc::c_char; 32]>(b"not enough memory for fileSizes\x00")).as_mut_ptr());
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 3],
                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
        }
        exit(12i32);
    } else {
        if !dictFileName.is_null() {
            let dictFileSize: U64 = UTIL_getFileSize(dictFileName);
            if dictFileSize > (64i32 * (1i32 << 20i32)) as libc::c_ulong {
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 12],
                                                      &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                            10i32);
                }
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 29],
                                                      &mut [libc::c_char; 29]>(b"dictionary file %s too large\x00")).as_mut_ptr(),
                            dictFileName);
                }
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 3],
                                                      &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                }
                exit(10i32);
            } else {
                dictBufferSize = dictFileSize;
                dictBuffer = malloc(dictBufferSize);
                if dictBuffer.is_null() {
                    if displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                                11i32);
                    }
                    if displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 44],
                                                          &mut [libc::c_char; 44]>(b"not enough memory for dictionary (%u bytes)\x00")).as_mut_ptr(),
                                dictBufferSize as U32);
                    }
                    if displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                    }
                    exit(11i32);
                } else {
                    BMK_loadFiles(dictBuffer, dictBufferSize, fileSizes,
                                  &dictFileName as *const *const libc::c_char,
                                  1i32 as libc::c_uint, displayLevel);
                }
            }
        }
        benchedSize =
            BMK_findMaxMem(totalSizeToLoad.wrapping_mul(3i32 as
                                                            libc::c_ulong)).wrapping_div(3i32
                                                                                             as
                                                                                             libc::c_ulong);
        if benchedSize > totalSizeToLoad { benchedSize = totalSizeToLoad }
        if benchedSize < totalSizeToLoad {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 42],
                                              &mut [libc::c_char; 42]>(b"Not enough memory; testing %u MB only...\n\x00")).as_mut_ptr(),
                    (benchedSize >> 20i32) as U32);
        }
        srcBuffer = malloc(benchedSize);
        if srcBuffer.is_null() {
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 12],
                                                  &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                        12i32);
            }
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 18],
                                                  &mut [libc::c_char; 18]>(b"not enough memory\x00")).as_mut_ptr());
            }
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 3],
                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
            }
            exit(12i32);
        } else {
            BMK_loadFiles(srcBuffer, benchedSize, fileSizes, fileNamesTable,
                          nbFiles, displayLevel);
            if 0 != g_separateFiles {
                let mut srcPtr: *const BYTE = srcBuffer as *const BYTE;
                let mut fileNb: U32 = 0;
                let mut resultarray: *mut BMK_result_t =
                    malloc((::std::mem::size_of::<BMK_result_t>() as
                                libc::c_ulong).wrapping_mul(nbFiles as
                                                                libc::c_ulong))
                        as *mut BMK_result_t;
                if resultarray.is_null() {
                    if displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                                12i32);
                    }
                    if displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 18],
                                                          &mut [libc::c_char; 18]>(b"not enough memory\x00")).as_mut_ptr());
                    }
                    if displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                    }
                    exit(12i32);
                } else {
                    fileNb = 0i32 as U32;
                    while fileNb < nbFiles {
                        let fileSize: size_t =
                            *fileSizes.offset(fileNb as isize);
                        BMK_benchCLevel(srcPtr as *const libc::c_void,
                                        fileSize,
                                        fileSizes.offset(fileNb as isize),
                                        1i32 as libc::c_uint, cLevel,
                                        cLevelLast, compressionParams,
                                        dictBuffer, dictBufferSize,
                                        displayLevel,
                                        *fileNamesTable.offset(fileNb as
                                                                   isize));
                        srcPtr = srcPtr.offset(fileSize as isize);
                        fileNb = fileNb.wrapping_add(1)
                    }
                }
            } else {
                let mut mfName: [libc::c_char; 20] =
                    [0i32 as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0];
                snprintf(mfName.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 20]>() as
                             libc::c_ulong,
                         (*::std::mem::transmute::<&[u8; 10],
                                                   &mut [libc::c_char; 10]>(b" %u files\x00")).as_mut_ptr(),
                         nbFiles);
                let displayName: *const libc::c_char =
                    if nbFiles > 1i32 as libc::c_uint {
                        mfName.as_mut_ptr()
                    } else { *fileNamesTable.offset(0isize) };
                BMK_benchCLevel(srcBuffer, benchedSize, fileSizes, nbFiles,
                                cLevel, cLevelLast, compressionParams,
                                dictBuffer, dictBufferSize, displayLevel,
                                displayName);
            }
            free(srcBuffer);
            free(dictBuffer);
            free(fileSizes as *mut libc::c_void);
            return;
        }
    };
}
unsafe extern "C" fn BMK_benchCLevel(mut srcBuffer: *const libc::c_void,
                                     mut benchedSize: size_t,
                                     mut fileSizes: *const size_t,
                                     mut nbFiles: libc::c_uint,
                                     cLevel: libc::c_int,
                                     cLevelLast: libc::c_int,
                                     mut comprParams:
                                         *const ZSTD_compressionParameters,
                                     mut dictBuffer: *const libc::c_void,
                                     mut dictBufferSize: size_t,
                                     mut displayLevel: libc::c_int,
                                     mut displayName: *const libc::c_char)
 -> () {
    let mut l: libc::c_int = 0;
    let mut pch: *const libc::c_char = strrchr(displayName, '\\' as i32);
    if pch.is_null() { pch = strrchr(displayName, '/' as i32) }
    if !pch.is_null() { displayName = pch.offset(1isize) }
    if 0 != g_realTime {
        if displayLevel >= 2i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 41],
                                              &mut [libc::c_char; 41]>(b"Note : switching to real-time priority \n\x00")).as_mut_ptr());
        }
        setpriority(PRIO_PROCESS as libc::c_int, 0i32 as id_t, -20i32);
    }
    if displayLevel == 1i32 && 0 == g_additionalParam {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 55],
                                          &mut [libc::c_char; 55]>(b"bench %s %s: input %u bytes, %u seconds, %u KB blocks\n\x00")).as_mut_ptr(),
                (*::std::mem::transmute::<&[u8; 6],
                                          &mut [libc::c_char; 6]>(b"1.3.5\x00")).as_mut_ptr(),
                (*::std::mem::transmute::<&[u8; 1],
                                          &mut [libc::c_char; 1]>(b"\x00")).as_mut_ptr(),
                benchedSize as U32, g_nbSeconds,
                (g_blockSize >> 10i32) as U32);
    }
    l = cLevel;
    while l <= cLevelLast {
        if !(l == 0i32) {
            BMK_benchMemCtxless(srcBuffer, benchedSize, fileSizes, nbFiles, l,
                                comprParams, dictBuffer, dictBufferSize,
                                displayLevel, displayName);
        }
        l += 1
    };
}
unsafe extern "C" fn BMK_benchMemCtxless(mut srcBuffer: *const libc::c_void,
                                         mut srcSize: size_t,
                                         mut fileSizes: *const size_t,
                                         mut nbFiles: libc::c_uint,
                                         mut cLevel: libc::c_int,
                                         comprParams:
                                             *const ZSTD_compressionParameters,
                                         mut dictBuffer: *const libc::c_void,
                                         mut dictBufferSize: size_t,
                                         mut displayLevel: libc::c_int,
                                         mut displayName: *const libc::c_char)
 -> () {
    let mut ctx: *mut ZSTD_CCtx = ZSTD_createCCtx();
    let mut dctx: *mut ZSTD_DCtx = ZSTD_createDCtx();
    if ctx.is_null() || dctx.is_null() {
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                    12i32);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 31],
                                              &mut [libc::c_char; 31]>(b"not enough memory for contexts\x00")).as_mut_ptr());
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 3],
                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
        }
        exit(12i32);
    } else {
        BMK_benchMem(srcBuffer, srcSize, fileSizes, nbFiles, cLevel,
                     comprParams, dictBuffer, dictBufferSize, ctx, dctx,
                     displayLevel, displayName);
        ZSTD_freeCCtx(ctx);
        ZSTD_freeDCtx(dctx);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BMK_benchMem(mut srcBuffer: *const libc::c_void,
                                      mut srcSize: size_t,
                                      mut fileSizes: *const size_t,
                                      mut nbFiles: libc::c_uint,
                                      cLevel: libc::c_int,
                                      mut comprParams:
                                          *const ZSTD_compressionParameters,
                                      mut dictBuffer: *const libc::c_void,
                                      mut dictBufferSize: size_t,
                                      mut ctx: *mut ZSTD_CCtx,
                                      mut dctx: *mut ZSTD_DCtx,
                                      mut displayLevel: libc::c_int,
                                      mut displayName: *const libc::c_char)
 -> BMK_return_t {
    let blockSize: size_t =
        if g_blockSize >= 32i32 as libc::c_ulong && 0 == g_decodeOnly {
            g_blockSize
        } else {
            srcSize
        }.wrapping_add((0 == srcSize) as libc::c_int as libc::c_ulong);
    let maxNbBlocks: U32 =
        (srcSize.wrapping_add(blockSize.wrapping_sub(1i32 as
                                                         libc::c_ulong)).wrapping_div(blockSize)
             as U32).wrapping_add(nbFiles);
    let blockTable: *mut blockParam_t =
        malloc((maxNbBlocks as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<blockParam_t>()
                                                    as libc::c_ulong)) as
            *mut blockParam_t;
    let maxCompressedSize: size_t =
        ZSTD_compressBound(srcSize).wrapping_add(maxNbBlocks.wrapping_mul(1024i32
                                                                              as
                                                                              libc::c_uint)
                                                     as libc::c_ulong);
    let compressedBuffer: *mut libc::c_void = malloc(maxCompressedSize);
    let mut resultBuffer: *mut libc::c_void = malloc(srcSize);
    let mut results: BMK_return_t =
        BMK_return_t{errorCode: 0,
                     result:
                         BMK_result_t{cSize: 0, cSpeed: 0., dSpeed: 0.,},};
    let loadedCompressedSize: size_t = srcSize;
    let mut cSize: size_t = 0i32 as size_t;
    let mut ratio: libc::c_double = 0.0f64;
    let mut nbBlocks: U32 = 0;
    if compressedBuffer.is_null() || resultBuffer.is_null() ||
           blockTable.is_null() {
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                    31i32);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 37],
                                              &mut [libc::c_char; 37]>(b"allocation error : not enough memory\x00")).as_mut_ptr());
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 3],
                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
        }
        exit(31i32);
    } else if ctx.is_null() || dctx.is_null() {
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                    31i32);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 30],
                                              &mut [libc::c_char; 30]>(b"error: passed in null context\x00")).as_mut_ptr());
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 3],
                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
        }
        exit(31i32);
    } else {
        if strlen(displayName) > 17i32 as libc::c_ulong {
            displayName =
                displayName.offset(strlen(displayName).wrapping_sub(17i32 as
                                                                        libc::c_ulong)
                                       as isize)
        }
        if g_nbWorkers == 1i32 as libc::c_uint { g_nbWorkers = 0i32 as U32 }
        if 0 != g_decodeOnly {
            let mut srcPtr: *const libc::c_char =
                srcBuffer as *const libc::c_char;
            let mut totalDSize64: U64 = 0i32 as U64;
            let mut fileNb: U32 = 0;
            fileNb = 0i32 as U32;
            while fileNb < nbFiles {
                let fSize64: U64 =
                    ZSTD_findDecompressedSize(srcPtr as *const libc::c_void,
                                              *fileSizes.offset(fileNb as
                                                                    isize)) as
                        U64;
                if fSize64 == 0i32 as libc::c_ulong {
                    if displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                                32i32);
                    }
                    if displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 39],
                                                          &mut [libc::c_char; 39]>(b"Impossible to determine original size \x00")).as_mut_ptr());
                    }
                    if displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                    }
                    exit(32i32);
                } else {
                    totalDSize64 =
                        (totalDSize64 as libc::c_ulong).wrapping_add(fSize64)
                            as U64 as U64;
                    srcPtr =
                        srcPtr.offset(*fileSizes.offset(fileNb as isize) as
                                          isize);
                    fileNb = fileNb.wrapping_add(1)
                }
            }
            let decodedSize: size_t = totalDSize64;
            if totalDSize64 > decodedSize {
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 12],
                                                      &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                            32i32);
                }
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 27],
                                                      &mut [libc::c_char; 27]>(b"original size is too large\x00")).as_mut_ptr());
                }
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 3],
                                                      &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                }
                exit(32i32);
            } else {
                free(resultBuffer);
                resultBuffer = malloc(decodedSize);
                if resultBuffer.is_null() {
                    if displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                                33i32);
                    }
                    if displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 18],
                                                          &mut [libc::c_char; 18]>(b"not enough memory\x00")).as_mut_ptr());
                    }
                    if displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                    }
                    exit(33i32);
                } else {
                    cSize = srcSize;
                    srcSize = decodedSize;
                    ratio =
                        srcSize as libc::c_double / cSize as libc::c_double
                }
            }
        }
        let mut srcPtr_0: *const libc::c_char =
            srcBuffer as *const libc::c_char;
        let mut cPtr: *mut libc::c_char =
            compressedBuffer as *mut libc::c_char;
        let mut resPtr: *mut libc::c_char = resultBuffer as *mut libc::c_char;
        let mut fileNb_0: U32 = 0;
        nbBlocks = 0i32 as U32;
        fileNb_0 = 0i32 as U32;
        while fileNb_0 < nbFiles {
            let mut remaining: size_t = *fileSizes.offset(fileNb_0 as isize);
            let nbBlocksforThisFile: U32 =
                if 0 != g_decodeOnly {
                    1i32 as libc::c_uint
                } else {
                    remaining.wrapping_add(blockSize.wrapping_sub(1i32 as
                                                                      libc::c_ulong)).wrapping_div(blockSize)
                        as U32
                };
            let blockEnd: U32 = nbBlocks.wrapping_add(nbBlocksforThisFile);
            while nbBlocks < blockEnd {
                let thisBlockSize: size_t =
                    if remaining < blockSize { remaining } else { blockSize };
                let ref mut fresh1 =
                    (*blockTable.offset(nbBlocks as isize)).srcPtr;
                *fresh1 = srcPtr_0 as *const libc::c_void;
                (*blockTable.offset(nbBlocks as isize)).srcSize =
                    thisBlockSize;
                let ref mut fresh2 =
                    (*blockTable.offset(nbBlocks as isize)).cPtr;
                *fresh2 = cPtr as *mut libc::c_void;
                (*blockTable.offset(nbBlocks as isize)).cRoom =
                    if 0 != g_decodeOnly {
                        thisBlockSize
                    } else { ZSTD_compressBound(thisBlockSize) };
                (*blockTable.offset(nbBlocks as isize)).cSize =
                    (*blockTable.offset(nbBlocks as isize)).cRoom;
                let ref mut fresh3 =
                    (*blockTable.offset(nbBlocks as isize)).resPtr;
                *fresh3 = resPtr as *mut libc::c_void;
                (*blockTable.offset(nbBlocks as isize)).resSize =
                    if 0 != g_decodeOnly {
                        ZSTD_findDecompressedSize(srcPtr_0 as
                                                      *const libc::c_void,
                                                  thisBlockSize) as size_t
                    } else { thisBlockSize };
                srcPtr_0 = srcPtr_0.offset(thisBlockSize as isize);
                cPtr =
                    cPtr.offset((*blockTable.offset(nbBlocks as isize)).cRoom
                                    as isize);
                resPtr = resPtr.offset(thisBlockSize as isize);
                remaining =
                    (remaining as libc::c_ulong).wrapping_sub(thisBlockSize)
                        as size_t as size_t;
                nbBlocks = nbBlocks.wrapping_add(1)
            }
            fileNb_0 = fileNb_0.wrapping_add(1)
        }
        if 0 != g_decodeOnly {
            memcpy(compressedBuffer, srcBuffer, loadedCompressedSize);
        } else {
            RDG_genBuffer(compressedBuffer, maxCompressedSize, 0.1f64, 0.5f64,
                          1i32 as libc::c_uint);
        }
        let mut fastestC: U64 = -1i64 as U64;
        let mut fastestD: U64 = -1i64 as U64;
        let crcOrig: U64 =
            (if 0 != g_decodeOnly {
                 0i32 as libc::c_ulonglong
             } else {
                 ZSTD_XXH64(srcBuffer, srcSize, 0i32 as libc::c_ulonglong)
             }) as U64;
        let mut coolTime: UTIL_time_t = timespec{tv_sec: 0, tv_nsec: 0,};
        let maxTime: U64 =
            (g_nbSeconds as
                 libc::c_ulonglong).wrapping_mul((1i32 as
                                                      libc::c_ulonglong).wrapping_mul(1000000000u64)).wrapping_add(1i32
                                                                                                                       as
                                                                                                                       libc::c_ulonglong)
                as U64;
        let mut nbDecodeLoops: U32 =
            (((100i32 * (1i32 << 20i32)) as
                  libc::c_ulong).wrapping_div(srcSize.wrapping_add(1i32 as
                                                                       libc::c_ulong))
                 as U32).wrapping_add(1i32 as libc::c_uint);
        let mut nbCompressionLoops: U32 =
            (((2i32 * (1i32 << 20i32)) as
                  libc::c_ulong).wrapping_div(srcSize.wrapping_add(1i32 as
                                                                       libc::c_ulong))
                 as U32).wrapping_add(1i32 as libc::c_uint);
        let mut totalCTime: U64 = 0i32 as U64;
        let mut totalDTime: U64 = 0i32 as U64;
        let mut cCompleted: U32 = g_decodeOnly;
        let mut dCompleted: U32 = 0i32 as U32;
        let marks: [*const libc::c_char; 4] =
            [(*::std::mem::transmute::<&[u8; 3],
                                       &mut [libc::c_char; 3]>(b" |\x00")).as_mut_ptr(),
             (*::std::mem::transmute::<&[u8; 3],
                                       &mut [libc::c_char; 3]>(b" /\x00")).as_mut_ptr(),
             (*::std::mem::transmute::<&[u8; 3],
                                       &mut [libc::c_char; 3]>(b" =\x00")).as_mut_ptr(),
             (*::std::mem::transmute::<&[u8; 2],
                                       &mut [libc::c_char; 2]>(b"\\\x00")).as_mut_ptr()];
        let mut markNb: U32 = 0i32 as U32;
        coolTime = UTIL_getTime();
        if displayLevel >= 2i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 7],
                                              &mut [libc::c_char; 7]>(b"\r%79s\r\x00")).as_mut_ptr(),
                    (*::std::mem::transmute::<&[u8; 1],
                                              &mut [libc::c_char; 1]>(b"\x00")).as_mut_ptr());
        }
        's_286:
            while 0 == cCompleted || 0 == dCompleted {
                if UTIL_clockSpanMicro(coolTime) as libc::c_ulonglong >
                       (70i32 as
                            libc::c_ulonglong).wrapping_mul((1i32 as
                                                                 libc::c_ulonglong).wrapping_mul(1000000u64))
                   {
                    if displayLevel >= 2i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 23],
                                                          &mut [libc::c_char; 23]>(b"\rcooling down ...    \r\x00")).as_mut_ptr());
                    }
                    sleep(10i32 as libc::c_uint);
                    coolTime = UTIL_getTime()
                }
                if 0 == g_decodeOnly {
                    if displayLevel >= 2i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 23],
                                                          &mut [libc::c_char; 23]>(b"%2s-%-17.17s :%10u ->\r\x00")).as_mut_ptr(),
                                marks[markNb as usize], displayName,
                                srcSize as U32);
                    }
                    if 0 == cCompleted {
                        memset(compressedBuffer, 229i32, maxCompressedSize);
                    }
                    let mut t: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
                    t.tv_sec = 0i32 as __time_t;
                    t.tv_nsec =
                        (5i32 as libc::c_ulonglong).wrapping_mul(1000000u64)
                            as __syscall_slong_t;
                    nanosleep(&mut t as *mut timespec, 0 as *mut timespec);
                    UTIL_waitForNextTick();
                    if 0 == cCompleted {
                        let mut nbLoops: U32 = 0i32 as U32;
                        let clockStart: UTIL_time_t = UTIL_getTime();
                        ZSTD_CCtx_setParameter(ctx, ZSTD_p_nbWorkers,
                                               g_nbWorkers);
                        ZSTD_CCtx_setParameter(ctx, ZSTD_p_compressionLevel,
                                               cLevel as libc::c_uint);
                        ZSTD_CCtx_setParameter(ctx,
                                               ZSTD_p_enableLongDistanceMatching,
                                               g_ldmFlag);
                        ZSTD_CCtx_setParameter(ctx, ZSTD_p_ldmMinMatch,
                                               g_ldmMinMatch);
                        ZSTD_CCtx_setParameter(ctx, ZSTD_p_ldmHashLog,
                                               g_ldmHashLog);
                        if g_ldmBucketSizeLog != 9999i32 as libc::c_uint {
                            ZSTD_CCtx_setParameter(ctx,
                                                   ZSTD_p_ldmBucketSizeLog,
                                                   g_ldmBucketSizeLog);
                        }
                        if g_ldmHashEveryLog != 9999i32 as libc::c_uint {
                            ZSTD_CCtx_setParameter(ctx,
                                                   ZSTD_p_ldmHashEveryLog,
                                                   g_ldmHashEveryLog);
                        }
                        ZSTD_CCtx_setParameter(ctx, ZSTD_p_windowLog,
                                               (*comprParams).windowLog);
                        ZSTD_CCtx_setParameter(ctx, ZSTD_p_hashLog,
                                               (*comprParams).hashLog);
                        ZSTD_CCtx_setParameter(ctx, ZSTD_p_chainLog,
                                               (*comprParams).chainLog);
                        ZSTD_CCtx_setParameter(ctx, ZSTD_p_searchLog,
                                               (*comprParams).searchLog);
                        ZSTD_CCtx_setParameter(ctx, ZSTD_p_minMatch,
                                               (*comprParams).searchLength);
                        ZSTD_CCtx_setParameter(ctx, ZSTD_p_targetLength,
                                               (*comprParams).targetLength);
                        ZSTD_CCtx_setParameter(ctx,
                                               ZSTD_p_compressionStrategy,
                                               (*comprParams).strategy as
                                                   libc::c_uint);
                        ZSTD_CCtx_loadDictionary(ctx, dictBuffer,
                                                 dictBufferSize);
                        if 0 == g_nbSeconds {
                            nbCompressionLoops = 1i32 as U32
                        }
                        nbLoops = 0i32 as U32;
                        while nbLoops < nbCompressionLoops {
                            let mut blockNb: U32 = 0;
                            blockNb = 0i32 as U32;
                            while blockNb < nbBlocks {
                                let mut moreToFlush: size_t = 1i32 as size_t;
                                let mut out: ZSTD_outBuffer =
                                    ZSTD_outBuffer_s{dst:
                                                         0 as
                                                             *mut libc::c_void,
                                                     size: 0,
                                                     pos: 0,};
                                let mut in_0: ZSTD_inBuffer =
                                    ZSTD_inBuffer_s{src:
                                                        0 as
                                                            *const libc::c_void,
                                                    size: 0,
                                                    pos: 0,};
                                in_0.src =
                                    (*blockTable.offset(blockNb as
                                                            isize)).srcPtr;
                                in_0.size =
                                    (*blockTable.offset(blockNb as
                                                            isize)).srcSize;
                                in_0.pos = 0i32 as size_t;
                                out.dst =
                                    (*blockTable.offset(blockNb as
                                                            isize)).cPtr;
                                out.size =
                                    (*blockTable.offset(blockNb as
                                                            isize)).cRoom;
                                out.pos = 0i32 as size_t;
                                while 0 != moreToFlush {
                                    moreToFlush =
                                        ZSTD_compress_generic(ctx,
                                                              &mut out as
                                                                  *mut ZSTD_outBuffer,
                                                              &mut in_0 as
                                                                  *mut ZSTD_inBuffer,
                                                              ZSTD_e_end);
                                    if !(0 != ZSTD_isError(moreToFlush)) {
                                        continue ;
                                    }
                                    if displayLevel >= 1i32 {
                                        fprintf(stderr,
                                                (*::std::mem::transmute::<&[u8; 12],
                                                                          &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                                                1i32);
                                    }
                                    if displayLevel >= 1i32 {
                                        fprintf(stderr,
                                                (*::std::mem::transmute::<&[u8; 35],
                                                                          &mut [libc::c_char; 35]>(b"ZSTD_compress_generic() error : %s\x00")).as_mut_ptr(),
                                                ZSTD_getErrorName(moreToFlush));
                                    }
                                    if displayLevel >= 1i32 {
                                        fprintf(stderr,
                                                (*::std::mem::transmute::<&[u8; 3],
                                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                    }
                                    exit(1i32);
                                }
                                (*blockTable.offset(blockNb as isize)).cSize =
                                    out.pos;
                                blockNb = blockNb.wrapping_add(1)
                            }
                            nbLoops = nbLoops.wrapping_add(1)
                        }
                        let loopDuration: U64 =
                            UTIL_clockSpanNano(clockStart);
                        if loopDuration > 0i32 as libc::c_ulong {
                            if loopDuration <
                                   fastestC.wrapping_mul(nbCompressionLoops as
                                                             libc::c_ulong) {
                                fastestC =
                                    loopDuration.wrapping_div(nbCompressionLoops
                                                                  as
                                                                  libc::c_ulong)
                            }
                            nbCompressionLoops =
                                ((1i32 as
                                      libc::c_ulonglong).wrapping_mul(1000000000u64).wrapping_div(fastestC
                                                                                                      as
                                                                                                      libc::c_ulonglong)
                                     as
                                     U32).wrapping_add(1i32 as libc::c_uint)
                        } else {
                            if nbCompressionLoops <
                                   40000000i32 as libc::c_uint {
                            } else {
                                __assert_fail((*::std::mem::transmute::<&[u8; 30],
                                                                        &mut [libc::c_char; 30]>(b"nbCompressionLoops < 40000000\x00")).as_mut_ptr(),
                                              (*::std::mem::transmute::<&[u8; 41],
                                                                        &mut [libc::c_char; 41]>(b"/home/danielrh/dev/zstd/programs/bench.c\x00")).as_mut_ptr(),
                                              369i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 192],
                                                                        &[libc::c_char; 192]>(b"BMK_return_t BMK_benchMem(const void *, size_t, const size_t *, unsigned int, const int, const ZSTD_compressionParameters *, const void *, size_t, ZSTD_CCtx *, ZSTD_DCtx *, int, const char *)\x00")).as_ptr());
                            };
                            nbCompressionLoops =
                                (nbCompressionLoops as
                                     libc::c_uint).wrapping_mul(100i32 as
                                                                    libc::c_uint)
                                    as U32 as U32
                        }
                        totalCTime =
                            (totalCTime as
                                 libc::c_ulong).wrapping_add(loopDuration) as
                                U64 as U64;
                        cCompleted =
                            (totalCTime >= maxTime) as libc::c_int as U32
                    }
                    cSize = 0i32 as size_t;
                    let mut blockNb_0: U32 = 0;
                    blockNb_0 = 0i32 as U32;
                    while blockNb_0 < nbBlocks {
                        cSize =
                            (cSize as
                                 libc::c_ulong).wrapping_add((*blockTable.offset(blockNb_0
                                                                                     as
                                                                                     isize)).cSize)
                                as size_t as size_t;
                        blockNb_0 = blockNb_0.wrapping_add(1)
                    }
                    ratio =
                        srcSize as libc::c_double / cSize as libc::c_double;
                    results.result.cSize = cSize;
                    markNb =
                        markNb.wrapping_add(1i32 as
                                                libc::c_uint).wrapping_rem(4i32
                                                                               as
                                                                               libc::c_uint);
                    let ratioAccuracy: libc::c_int =
                        if ratio < 10.0f64 { 3i32 } else { 2i32 };
                    let compressionSpeed: libc::c_double =
                        srcSize as libc::c_double / fastestC as libc::c_double
                            * 1000i32 as libc::c_double;
                    let cSpeedAccuracy: libc::c_int =
                        if compressionSpeed < 10.0f64 { 2i32 } else { 1i32 };
                    results.result.cSpeed =
                        compressionSpeed * 1000000i32 as libc::c_double;
                    if displayLevel >= 2i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 46],
                                                          &mut [libc::c_char; 46]>(b"%2s-%-17.17s :%10u ->%10u (%5.*f),%6.*f MB/s\r\x00")).as_mut_ptr(),
                                marks[markNb as usize], displayName,
                                srcSize as U32, cSize as U32, ratioAccuracy,
                                ratio, cSpeedAccuracy, compressionSpeed);
                    }
                }
                if 0 == dCompleted { memset(resultBuffer, 214i32, srcSize); }
                let mut t_0: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
                t_0.tv_sec = 0i32 as __time_t;
                t_0.tv_nsec =
                    (5i32 as libc::c_ulonglong).wrapping_mul(1000000u64) as
                        __syscall_slong_t;
                nanosleep(&mut t_0 as *mut timespec, 0 as *mut timespec);
                UTIL_waitForNextTick();
                if 0 == dCompleted {
                    let mut nbLoops_0: U32 = 0i32 as U32;
                    let ddict: *mut ZSTD_DDict =
                        ZSTD_createDDict(dictBuffer, dictBufferSize);
                    let clockStart_0: UTIL_time_t = UTIL_getTime();
                    if ddict.is_null() {
                        if displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 12],
                                                              &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                                    2i32);
                        }
                        if displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 38],
                                                              &mut [libc::c_char; 38]>(b"ZSTD_createDDict() allocation failure\x00")).as_mut_ptr());
                        }
                        if displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 3],
                                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                        }
                        exit(2i32);
                    } else {
                        if 0 == g_nbSeconds { nbDecodeLoops = 1i32 as U32 }
                        nbLoops_0 = 0i32 as U32;
                        while nbLoops_0 < nbDecodeLoops {
                            let mut blockNb_1: U32 = 0;
                            blockNb_1 = 0i32 as U32;
                            while blockNb_1 < nbBlocks {
                                let regenSize: size_t =
                                    ZSTD_decompress_usingDDict(dctx,
                                                               (*blockTable.offset(blockNb_1
                                                                                       as
                                                                                       isize)).resPtr,
                                                               (*blockTable.offset(blockNb_1
                                                                                       as
                                                                                       isize)).resSize,
                                                               (*blockTable.offset(blockNb_1
                                                                                       as
                                                                                       isize)).cPtr,
                                                               (*blockTable.offset(blockNb_1
                                                                                       as
                                                                                       isize)).cSize,
                                                               ddict);
                                if 0 != ZSTD_isError(regenSize) {
                                    if displayLevel >= 1i32 {
                                        fprintf(stderr,
                                                (*::std::mem::transmute::<&[u8; 12],
                                                                          &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                                                2i32);
                                    }
                                    if displayLevel >= 1i32 {
                                        fprintf(stderr,
                                                (*::std::mem::transmute::<&[u8; 67],
                                                                          &mut [libc::c_char; 67]>(b"ZSTD_decompress_usingDDict() failed on block %u of size %u : %s  \n\x00")).as_mut_ptr(),
                                                blockNb_1,
                                                (*blockTable.offset(blockNb_1
                                                                        as
                                                                        isize)).cSize
                                                    as U32,
                                                ZSTD_getErrorName(regenSize));
                                    }
                                    if displayLevel >= 1i32 {
                                        fprintf(stderr,
                                                (*::std::mem::transmute::<&[u8; 3],
                                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                    }
                                    exit(2i32);
                                } else {
                                    (*blockTable.offset(blockNb_1 as
                                                            isize)).resSize =
                                        regenSize;
                                    blockNb_1 = blockNb_1.wrapping_add(1)
                                }
                            }
                            nbLoops_0 = nbLoops_0.wrapping_add(1)
                        }
                        ZSTD_freeDDict(ddict);
                        let loopDuration_0: U64 =
                            UTIL_clockSpanNano(clockStart_0);
                        if loopDuration_0 > 0i32 as libc::c_ulong {
                            if loopDuration_0 <
                                   fastestD.wrapping_mul(nbDecodeLoops as
                                                             libc::c_ulong) {
                                fastestD =
                                    loopDuration_0.wrapping_div(nbDecodeLoops
                                                                    as
                                                                    libc::c_ulong)
                            }
                            nbDecodeLoops =
                                ((1i32 as
                                      libc::c_ulonglong).wrapping_mul(1000000000u64).wrapping_div(fastestD
                                                                                                      as
                                                                                                      libc::c_ulonglong)
                                     as
                                     U32).wrapping_add(1i32 as libc::c_uint)
                        } else {
                            if nbDecodeLoops < 40000000i32 as libc::c_uint {
                            } else {
                                __assert_fail((*::std::mem::transmute::<&[u8; 25],
                                                                        &mut [libc::c_char; 25]>(b"nbDecodeLoops < 40000000\x00")).as_mut_ptr(),
                                              (*::std::mem::transmute::<&[u8; 41],
                                                                        &mut [libc::c_char; 41]>(b"/home/danielrh/dev/zstd/programs/bench.c\x00")).as_mut_ptr(),
                                              428i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 192],
                                                                        &[libc::c_char; 192]>(b"BMK_return_t BMK_benchMem(const void *, size_t, const size_t *, unsigned int, const int, const ZSTD_compressionParameters *, const void *, size_t, ZSTD_CCtx *, ZSTD_DCtx *, int, const char *)\x00")).as_ptr());
                            };
                            nbDecodeLoops =
                                (nbDecodeLoops as
                                     libc::c_uint).wrapping_mul(100i32 as
                                                                    libc::c_uint)
                                    as U32 as U32
                        }
                        totalDTime =
                            (totalDTime as
                                 libc::c_ulong).wrapping_add(loopDuration_0)
                                as U64 as U64;
                        dCompleted =
                            (totalDTime >= maxTime) as libc::c_int as U32
                    }
                }
                markNb =
                    markNb.wrapping_add(1i32 as
                                            libc::c_uint).wrapping_rem(4i32 as
                                                                           libc::c_uint);
                let ratioAccuracy_0: libc::c_int =
                    if ratio < 10.0f64 { 3i32 } else { 2i32 };
                let compressionSpeed_0: libc::c_double =
                    srcSize as libc::c_double / fastestC as libc::c_double *
                        1000i32 as libc::c_double;
                let cSpeedAccuracy_0: libc::c_int =
                    if compressionSpeed_0 < 10.0f64 { 2i32 } else { 1i32 };
                let decompressionSpeed: libc::c_double =
                    srcSize as libc::c_double / fastestD as libc::c_double *
                        1000i32 as libc::c_double;
                results.result.cSpeed =
                    compressionSpeed_0 * 1000000i32 as libc::c_double;
                results.result.dSpeed =
                    decompressionSpeed * 1000000i32 as libc::c_double;
                if displayLevel >= 2i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 59],
                                                      &mut [libc::c_char; 59]>(b"%2s-%-17.17s :%10u ->%10u (%5.*f),%6.*f MB/s ,%6.1f MB/s \r\x00")).as_mut_ptr(),
                            marks[markNb as usize], displayName,
                            srcSize as U32, cSize as U32, ratioAccuracy_0,
                            ratio, cSpeedAccuracy_0, compressionSpeed_0,
                            decompressionSpeed);
                }
                let crcCheck: U64 =
                    ZSTD_XXH64(resultBuffer, srcSize,
                               0i32 as libc::c_ulonglong) as U64;
                if !(0 == g_decodeOnly && crcOrig != crcCheck) { continue ; }
                let mut u: size_t = 0;
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 55],
                                                  &mut [libc::c_char; 55]>(b"!!! WARNING !!! %14s : Invalid Checksum : %x != %x   \n\x00")).as_mut_ptr(),
                        displayName, crcOrig as libc::c_uint,
                        crcCheck as libc::c_uint);
                u = 0i32 as size_t;
                loop  {
                    if !(u < srcSize) { break 's_286 ; }
                    if *(srcBuffer as *const BYTE).offset(u as isize) as
                           libc::c_int !=
                           *(resultBuffer as *const BYTE).offset(u as isize)
                               as libc::c_int {
                        let mut segNb: U32 = 0;
                        let mut bNb: U32 = 0;
                        let mut pos: U32 = 0;
                        let mut bacc: size_t = 0i32 as size_t;
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 26],
                                                          &mut [libc::c_char; 26]>(b"Decoding error at pos %u \x00")).as_mut_ptr(),
                                u as U32);
                        segNb = 0i32 as U32;
                        while segNb < nbBlocks {
                            if bacc.wrapping_add((*blockTable.offset(segNb as
                                                                         isize)).srcSize)
                                   > u {
                                break ;
                            }
                            bacc =
                                (bacc as
                                     libc::c_ulong).wrapping_add((*blockTable.offset(segNb
                                                                                         as
                                                                                         isize)).srcSize)
                                    as size_t as size_t;
                            segNb = segNb.wrapping_add(1)
                        }
                        pos = u.wrapping_sub(bacc) as U32;
                        bNb =
                            pos.wrapping_div((128i32 * (1i32 << 10i32)) as
                                                 libc::c_uint);
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 32],
                                                          &mut [libc::c_char; 32]>(b"(sample %u, block %u, pos %u) \n\x00")).as_mut_ptr(),
                                segNb, bNb, pos);
                        if !(u > 5i32 as libc::c_ulong) { break 's_286 ; }
                        let mut n: libc::c_int = 0;
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 9],
                                                          &mut [libc::c_char; 9]>(b"origin: \x00")).as_mut_ptr());
                        n = -5i32;
                        while n < 0i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 6],
                                                              &mut [libc::c_char; 6]>(b"%02X \x00")).as_mut_ptr(),
                                    *(srcBuffer as
                                          *const BYTE).offset(u.wrapping_add(n
                                                                                 as
                                                                                 libc::c_ulong)
                                                                  as isize) as
                                        libc::c_int);
                            n += 1
                        }
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 10],
                                                          &mut [libc::c_char; 10]>(b" :%02X:  \x00")).as_mut_ptr(),
                                *(srcBuffer as *const BYTE).offset(u as isize)
                                    as libc::c_int);
                        n = 1i32;
                        while n < 3i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 6],
                                                              &mut [libc::c_char; 6]>(b"%02X \x00")).as_mut_ptr(),
                                    *(srcBuffer as
                                          *const BYTE).offset(u.wrapping_add(n
                                                                                 as
                                                                                 libc::c_ulong)
                                                                  as isize) as
                                        libc::c_int);
                            n += 1
                        }
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 9],
                                                          &mut [libc::c_char; 9]>(b"decode: \x00")).as_mut_ptr());
                        n = -5i32;
                        while n < 0i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 6],
                                                              &mut [libc::c_char; 6]>(b"%02X \x00")).as_mut_ptr(),
                                    *(resultBuffer as
                                          *const BYTE).offset(u.wrapping_add(n
                                                                                 as
                                                                                 libc::c_ulong)
                                                                  as isize) as
                                        libc::c_int);
                            n += 1
                        }
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 10],
                                                          &mut [libc::c_char; 10]>(b" :%02X:  \x00")).as_mut_ptr(),
                                *(resultBuffer as
                                      *const BYTE).offset(u as isize) as
                                    libc::c_int);
                        n = 1i32;
                        while n < 3i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 6],
                                                              &mut [libc::c_char; 6]>(b"%02X \x00")).as_mut_ptr(),
                                    *(resultBuffer as
                                          *const BYTE).offset(u.wrapping_add(n
                                                                                 as
                                                                                 libc::c_ulong)
                                                                  as isize) as
                                        libc::c_int);
                            n += 1
                        }
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                        break 's_286 ;
                    } else {
                        if u == srcSize.wrapping_sub(1i32 as libc::c_ulong) {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 24],
                                                              &mut [libc::c_char; 24]>(b"no difference detected\n\x00")).as_mut_ptr());
                        }
                        u = u.wrapping_add(1)
                    }
                }
            }
        if displayLevel == 1i32 {
            let cSpeed: libc::c_double =
                srcSize as libc::c_double / fastestC as libc::c_double *
                    1000i32 as libc::c_double;
            let dSpeed: libc::c_double =
                srcSize as libc::c_double / fastestD as libc::c_double *
                    1000i32 as libc::c_double;
            if 0 != g_additionalParam {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 56],
                                                  &mut [libc::c_char; 56]>(b"-%-3i%11i (%5.3f) %6.2f MB/s %6.1f MB/s  %s (param=%d)\n\x00")).as_mut_ptr(),
                        cLevel, cSize as libc::c_int, ratio, cSpeed, dSpeed,
                        displayName, g_additionalParam);
            } else {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 45],
                                                  &mut [libc::c_char; 45]>(b"-%-3i%11i (%5.3f) %6.2f MB/s %6.1f MB/s  %s\n\x00")).as_mut_ptr(),
                        cLevel, cSize as libc::c_int, ratio, cSpeed, dSpeed,
                        displayName);
            }
        }
        if displayLevel >= 2i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 6],
                                              &mut [libc::c_char; 6]>(b"%2i#\n\x00")).as_mut_ptr(),
                    cLevel);
        }
        free(blockTable as *mut libc::c_void);
        free(compressedBuffer);
        free(resultBuffer);
        results.errorCode = 0i32;
        return results
    };
}
static mut g_blockSize: size_t = unsafe { 0i32 as size_t };
static mut g_decodeOnly: U32 = unsafe { 0i32 as U32 };
static mut g_additionalParam: libc::c_int = unsafe { 0i32 };
static mut g_nbSeconds: U32 = unsafe { 3i32 as U32 };
static mut g_ldmHashEveryLog: U32 = unsafe { 9999i32 as U32 };
static mut g_ldmBucketSizeLog: U32 = unsafe { 9999i32 as U32 };
static mut g_ldmHashLog: U32 = unsafe { 0i32 as U32 };
static mut g_ldmMinMatch: U32 = unsafe { 0i32 as U32 };
static mut g_ldmFlag: U32 = unsafe { 0i32 as U32 };
static mut g_nbWorkers: U32 = unsafe { 0i32 as U32 };
static mut g_realTime: U32 = unsafe { 0i32 as U32 };
static mut g_separateFiles: U32 = unsafe { 0i32 as U32 };
/* ! BMK_loadFiles() :
 *  Loads `buffer` with content of files listed within `fileNamesTable`.
 *  At most, fills `buffer` entirely. */
unsafe extern "C" fn BMK_loadFiles(mut buffer: *mut libc::c_void,
                                   mut bufferSize: size_t,
                                   mut fileSizes: *mut size_t,
                                   fileNamesTable: *const *const libc::c_char,
                                   mut nbFiles: libc::c_uint,
                                   mut displayLevel: libc::c_int) -> () {
    let mut pos: size_t = 0i32 as size_t;
    let mut totalSize: size_t = 0i32 as size_t;
    let mut n: libc::c_uint = 0;
    n = 0i32 as libc::c_uint;
    while n < nbFiles {
        let mut f: *mut FILE = 0 as *mut FILE;
        let mut fileSize: U64 =
            UTIL_getFileSize(*fileNamesTable.offset(n as isize));
        if 0 != UTIL_isDirectory(*fileNamesTable.offset(n as isize)) {
            if displayLevel >= 2i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 33],
                                                  &mut [libc::c_char; 33]>(b"Ignoring %s directory...       \n\x00")).as_mut_ptr(),
                        *fileNamesTable.offset(n as isize));
            }
            *fileSizes.offset(n as isize) = 0i32 as size_t
        } else if fileSize == -1i32 as U64 {
            if displayLevel >= 2i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 43],
                                                  &mut [libc::c_char; 43]>(b"Cannot evaluate size of %s, ignoring ... \n\x00")).as_mut_ptr(),
                        *fileNamesTable.offset(n as isize));
            }
            *fileSizes.offset(n as isize) = 0i32 as size_t
        } else {
            f =
                fopen(*fileNamesTable.offset(n as isize),
                      (*::std::mem::transmute::<&[u8; 3],
                                                &mut [libc::c_char; 3]>(b"rb\x00")).as_mut_ptr());
            if f.is_null() {
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 12],
                                                      &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                            10i32);
                }
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 27],
                                                      &mut [libc::c_char; 27]>(b"impossible to open file %s\x00")).as_mut_ptr(),
                            *fileNamesTable.offset(n as isize));
                }
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 3],
                                                      &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                }
                exit(10i32);
            } else {
                if displayLevel >= 2i32 {
                    if UTIL_clockSpanMicro(g_displayClock) > g_refreshRate ||
                           displayLevel >= 4i32 {
                        g_displayClock = UTIL_getTime();
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 22],
                                                          &mut [libc::c_char; 22]>(b"Loading %s...       \r\x00")).as_mut_ptr(),
                                *fileNamesTable.offset(n as isize));
                        if displayLevel >= 4i32 { fflush(stderr); }
                    }
                }
                if fileSize > bufferSize.wrapping_sub(pos) {
                    fileSize = bufferSize.wrapping_sub(pos);
                    nbFiles = n
                }
                let readSize: size_t =
                    fread((buffer as *mut libc::c_char).offset(pos as isize)
                              as *mut libc::c_void, 1i32 as size_t, fileSize,
                          f);
                if readSize != fileSize {
                    if displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                                11i32);
                    }
                    if displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 18],
                                                          &mut [libc::c_char; 18]>(b"could not read %s\x00")).as_mut_ptr(),
                                *fileNamesTable.offset(n as isize));
                    }
                    if displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                    }
                    exit(11i32);
                } else {
                    pos =
                        (pos as libc::c_ulong).wrapping_add(readSize) as
                            size_t as size_t;
                    *fileSizes.offset(n as isize) = fileSize;
                    totalSize =
                        (totalSize as libc::c_ulong).wrapping_add(fileSize) as
                            size_t as size_t;
                    fclose(f);
                }
            }
        }
        n = n.wrapping_add(1)
    }
    if totalSize == 0i32 as libc::c_ulong {
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                    12i32);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 17],
                                              &mut [libc::c_char; 17]>(b"no data to bench\x00")).as_mut_ptr());
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 3],
                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
        }
        exit(12i32);
    } else { return; };
}
static mut g_displayClock: UTIL_time_t =
    unsafe {
        timespec{tv_sec: 0i32 as __time_t,
                 tv_nsec: 0i32 as __syscall_slong_t,}
    };
static mut g_refreshRate: U64 = unsafe { (1000000i32 / 6i32) as U64 };
unsafe extern "C" fn BMK_findMaxMem(mut requiredMem: U64) -> size_t {
    let step: size_t = (64i32 * (1i32 << 20i32)) as size_t;
    let mut testmem: *mut BYTE = 0 as *mut BYTE;
    requiredMem =
        (requiredMem >> 26i32).wrapping_add(1i32 as libc::c_ulong) << 26i32;
    requiredMem =
        (requiredMem as libc::c_ulong).wrapping_add(step) as U64 as U64;
    if requiredMem > maxMemory { requiredMem = maxMemory }
    loop  {
        testmem = malloc(requiredMem) as *mut BYTE;
        requiredMem =
            (requiredMem as libc::c_ulong).wrapping_sub(step) as U64 as U64;
        if !testmem.is_null() { break ; }
    }
    free(testmem as *mut libc::c_void);
    return requiredMem;
}
static mut maxMemory: size_t = 0;
static mut g_compressibilityDefault: U32 = unsafe { 50i32 as U32 };
unsafe extern "C" fn BMK_syntheticTest(mut cLevel: libc::c_int,
                                       mut cLevelLast: libc::c_int,
                                       mut compressibility: libc::c_double,
                                       mut compressionParams:
                                           *const ZSTD_compressionParameters,
                                       mut displayLevel: libc::c_int) -> () {
    let mut name: [libc::c_char; 20] =
        [0i32 as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0];
    let mut benchedSize: size_t = 10000000i32 as size_t;
    let srcBuffer: *mut libc::c_void = malloc(benchedSize);
    if srcBuffer.is_null() {
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                    21i32);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 18],
                                              &mut [libc::c_char; 18]>(b"not enough memory\x00")).as_mut_ptr());
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 3],
                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
        }
        exit(21i32);
    } else {
        RDG_genBuffer(srcBuffer, benchedSize, compressibility, 0.0f64,
                      0i32 as libc::c_uint);
        snprintf(name.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
                 (*::std::mem::transmute::<&[u8; 16],
                                           &mut [libc::c_char; 16]>(b"Synthetic %2u%%\x00")).as_mut_ptr(),
                 (compressibility * 100i32 as libc::c_double) as
                     libc::c_uint);
        BMK_benchCLevel(srcBuffer, benchedSize,
                        &mut benchedSize as *mut size_t, 1i32 as libc::c_uint,
                        cLevel, cLevelLast, compressionParams,
                        0 as *const libc::c_void, 0i32 as size_t,
                        displayLevel, name.as_mut_ptr());
        free(srcBuffer);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setNbSeconds(mut nbSeconds: libc::c_uint) -> () {
    g_nbSeconds = nbSeconds;
    fprintf(stderr,
            (*::std::mem::transmute::<&[u8; 57],
                                      &mut [libc::c_char; 57]>(b"- test >= %u seconds per compression / decompression - \n\x00")).as_mut_ptr(),
            g_nbSeconds);
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setBlockSize(mut blockSize: size_t) -> () {
    g_blockSize = blockSize;
    if 0 != g_blockSize {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 29],
                                          &mut [libc::c_char; 29]>(b"using blocks of size %u KB \n\x00")).as_mut_ptr(),
                (blockSize >> 10i32) as U32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setNbWorkers(mut nbWorkers: libc::c_uint) -> () {
    g_nbWorkers = nbWorkers;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setRealTime(mut priority: libc::c_uint) -> () {
    g_realTime = (priority > 0i32 as libc::c_uint) as libc::c_int as U32;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setSeparateFiles(mut separate: libc::c_uint)
 -> () {
    g_separateFiles = (separate > 0i32 as libc::c_uint) as libc::c_int as U32;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setAdditionalParam(mut additionalParam:
                                                    libc::c_int) -> () {
    g_additionalParam = additionalParam;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setDecodeOnlyMode(mut decodeFlag: libc::c_uint)
 -> () {
    g_decodeOnly = (decodeFlag > 0i32 as libc::c_uint) as libc::c_int as U32;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setLdmFlag(mut ldmFlag: libc::c_uint) -> () {
    g_ldmFlag = ldmFlag;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setLdmMinMatch(mut ldmMinMatch: libc::c_uint)
 -> () {
    g_ldmMinMatch = ldmMinMatch;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setLdmHashLog(mut ldmHashLog: libc::c_uint)
 -> () {
    g_ldmHashLog = ldmHashLog;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setLdmBucketSizeLog(mut ldmBucketSizeLog:
                                                     libc::c_uint) -> () {
    g_ldmBucketSizeLog = ldmBucketSizeLog;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setLdmHashEveryLog(mut ldmHashEveryLog:
                                                    libc::c_uint) -> () {
    g_ldmHashEveryLog = ldmHashEveryLog;
}
unsafe extern "C" fn run_static_initializers() -> () {
    maxMemory =
        if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
               4i32 as libc::c_ulong {
            (2i32 as
                 libc::c_uint).wrapping_mul(1u32 <<
                                                30i32).wrapping_sub((64i32 *
                                                                         (1i32
                                                                              <<
                                                                              20i32))
                                                                        as
                                                                        libc::c_uint)
                as libc::c_ulong
        } else {
            (1u64 <<
                 (::std::mem::size_of::<size_t>() as
                      libc::c_ulong).wrapping_mul(8i32 as
                                                      libc::c_ulong).wrapping_sub(31i32
                                                                                      as
                                                                                      libc::c_ulong))
                as size_t
        }
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn() -> (); 1] =
    [run_static_initializers];
