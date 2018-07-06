#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( extern_types , libc , offset_to , used )]
extern crate libc;
extern "C" {
    pub type _IO_FILE_plus;
    pub type __dirstream;
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
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t,
              __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
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
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
    #[no_mangle]
    fn ZDICT_isError(errorCode: size_t) -> libc::c_uint;
    #[no_mangle]
    fn ZDICT_getErrorName(errorCode: size_t) -> *const libc::c_char;
    #[no_mangle]
    fn ZDICT_trainFromBuffer_cover(dictBuffer: *mut libc::c_void,
                                   dictBufferCapacity: size_t,
                                   samplesBuffer: *const libc::c_void,
                                   samplesSizes: *const size_t,
                                   nbSamples: libc::c_uint,
                                   parameters: ZDICT_cover_params_t)
     -> size_t;
    #[no_mangle]
    fn ZDICT_optimizeTrainFromBuffer_cover(dictBuffer: *mut libc::c_void,
                                           dictBufferCapacity: size_t,
                                           samplesBuffer: *const libc::c_void,
                                           samplesSizes: *const size_t,
                                           nbSamples: libc::c_uint,
                                           parameters:
                                               *mut ZDICT_cover_params_t)
     -> size_t;
    /* ! DiB_trainFromFiles() :
    Train a dictionary from a set of files provided by `fileNamesTable`.
    Resulting dictionary is written into file `dictFileName`.
    `parameters` is optional and can be provided with values set to 0, meaning "default".
    @return : 0 == ok. Any other : error.
*/
    /* ! DiB_fileStats() :
 *  Given a list of files, and a chunkSize (0 == no chunk, whole files)
 *  provides the amount of data to be loaded and the resulting nb of samples.
 *  This is useful primarily for allocation purpose => sample buffer, and sample sizes table.
 */
    /* ! ZDICT_trainFromBuffer_unsafe_legacy() :
    Strictly Internal use only !!
    Same as ZDICT_trainFromBuffer_legacy(), but does not control `samplesBuffer`.
    `samplesBuffer` must be followed by noisy guard band to avoid out-of-buffer reads.
    @return : size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)
              or an error code.
*/
    #[no_mangle]
    fn ZDICT_trainFromBuffer_unsafe_legacy(dictBuffer: *mut libc::c_void,
                                           dictBufferCapacity: size_t,
                                           samplesBuffer: *const libc::c_void,
                                           samplesSizes: *const size_t,
                                           nbSamples: libc::c_uint,
                                           parameters: ZDICT_legacy_params_t)
     -> size_t;
}
pub const _SC_UIO_MAXIOV: unnamed_0 = 60;
pub const _SC_THREAD_SPORADIC_SERVER: unnamed_0 = 161;
pub const _SC_THREAD_STACK_MIN: unnamed_0 = 75;
pub const _SC_LEVEL4_CACHE_ASSOC: unnamed_0 = 198;
pub const _SC_AVPHYS_PAGES: unnamed_0 = 86;
pub const _SC_CLK_TCK: unnamed_0 = 2;
pub const _SC_XOPEN_UNIX: unnamed_0 = 91;
pub const _SC_XOPEN_SHM: unnamed_0 = 94;
pub const _SC_THREAD_PRIO_INHERIT: unnamed_0 = 80;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
pub const _SC_MONOTONIC_CLOCK: unnamed_0 = 149;
pub type uint8_t = libc::c_uchar;
pub const _SC_CHAR_MIN: unnamed_0 = 103;
pub const _SC_MB_LEN_MAX: unnamed_0 = 108;
pub const _SC_TIMEOUTS: unnamed_0 = 164;
pub const ZSTD_error_seekableIO: ERR_enum = 102;
pub const _SC_SPORADIC_SERVER: unnamed_0 = 160;
pub const _SC_2_C_BIND: unnamed_0 = 47;
pub const _SC_TTY_NAME_MAX: unnamed_0 = 72;
pub type __mode_t = libc::c_uint;
pub const _SC_XOPEN_CRYPT: unnamed_0 = 92;
pub const _SC_STREAM_MAX: unnamed_0 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const _SC_PRIORITY_SCHEDULING: unnamed_0 = 10;
pub const _SC_SEM_VALUE_MAX: unnamed_0 = 33;
pub const _SC_ASYNCHRONOUS_IO: unnamed_0 = 12;
pub const _SC_V7_ILP32_OFFBIG: unnamed_0 = 238;
pub type time_t = __time_t;
pub const _SC_SYSTEM_DATABASE_R: unnamed_0 = 163;
pub const _SC_INT_MAX: unnamed_0 = 104;
pub const _SC_LINE_MAX: unnamed_0 = 43;
pub type uint32_t = libc::c_uint;
pub const _SC_FILE_ATTRIBUTES: unnamed_0 = 146;
pub const _SC_SYMLOOP_MAX: unnamed_0 = 173;
pub const _SC_THREADS: unnamed_0 = 67;
pub type FILE = _IO_FILE;
pub const _SC_LEVEL3_CACHE_ASSOC: unnamed_0 = 195;
pub const ZSTD_error_stage_wrong: ERR_enum = 60;
pub const _SC_USHRT_MAX: unnamed_0 = 118;
pub const _SC_BC_DIM_MAX: unnamed_0 = 37;
pub const _SC_SS_REPL_MAX: unnamed_0 = 241;
pub const ZSTD_error_maxSymbolValue_tooLarge: ERR_enum = 46;
pub const _SC_2_CHAR_TERM: unnamed_0 = 95;
pub const _SC_T_IOV_MAX: unnamed_0 = 66;
pub const _SC_BC_SCALE_MAX: unnamed_0 = 38;
pub const _SC_PIPE: unnamed_0 = 145;
pub const _SC_PAGESIZE: unnamed_0 = 30;
pub const _SC_LEVEL4_CACHE_LINESIZE: unnamed_0 = 199;
pub const _SC_2_PBS_LOCATE: unnamed_0 = 170;
pub const _SC_SHELL: unnamed_0 = 157;
pub type UTIL_time_t = timespec;
pub const _SC_DEVICE_IO: unnamed_0 = 140;
pub const ZSTD_error_init_missing: ERR_enum = 62;
pub const _SC_FILE_LOCKING: unnamed_0 = 147;
pub const _SC_TRACE_LOG: unnamed_0 = 184;
pub const _SC_BARRIERS: unnamed_0 = 133;
pub const ZSTD_error_prefix_unknown: ERR_enum = 10;
pub const _SC_LEVEL1_DCACHE_LINESIZE: unnamed_0 = 190;
pub const _SC_XOPEN_XPG4: unnamed_0 = 100;
pub const _SC_XOPEN_XPG3: unnamed_0 = 99;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZDICT_legacy_params_t {
    pub selectivityLevel: libc::c_uint,
    pub zParams: ZDICT_params_t,
}
pub const _SC_SINGLE_PROCESS: unnamed_0 = 151;
pub const _SC_INT_MIN: unnamed_0 = 105;
pub type __dev_t = libc::c_ulong;
pub const _SC_PII_OSI: unnamed_0 = 57;
pub const _SC_MEMLOCK_RANGE: unnamed_0 = 18;
pub const _SC_THREAD_PRIORITY_SCHEDULING: unnamed_0 = 79;
pub const _SC_TRACE_SYS_MAX: unnamed_0 = 244;
pub const _SC_SELECT: unnamed_0 = 59;
pub const _SC_LEVEL2_CACHE_ASSOC: unnamed_0 = 192;
pub const _SC_OPEN_MAX: unnamed_0 = 4;
pub const _SC_PII_INTERNET_DGRAM: unnamed_0 = 62;
pub const ZSTD_error_checksum_wrong: ERR_enum = 22;
pub const _SC_MULTI_PROCESS: unnamed_0 = 150;
pub const _SC_2_PBS: unnamed_0 = 168;
pub const _SC_LEVEL3_CACHE_SIZE: unnamed_0 = 194;
pub const _SC_2_PBS_CHECKPOINT: unnamed_0 = 175;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    u: U32,
    c: [BYTE; 4],
}
pub const _SC_XOPEN_XPG2: unnamed_0 = 98;
pub const _SC_FSYNC: unnamed_0 = 15;
pub const _SC_THREAD_SAFE_FUNCTIONS: unnamed_0 = 68;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: unnamed_0 = 247;
pub const _SC_WORD_BIT: unnamed_0 = 107;
pub const ZSTD_error_frameParameter_windowTooLarge: ERR_enum = 16;
pub type __ino_t = libc::c_ulong;
pub const _SC_UCHAR_MAX: unnamed_0 = 115;
pub const _SC_SSIZE_MAX: unnamed_0 = 110;
pub const _SC_SYNCHRONIZED_IO: unnamed_0 = 14;
pub const _SC_XOPEN_VERSION: unnamed_0 = 89;
pub const ZSTD_error_version_unsupported: ERR_enum = 12;
pub const _SC_CPUTIME: unnamed_0 = 138;
pub const _SC_2_PBS_TRACK: unnamed_0 = 172;
pub const _SC_TYPED_MEMORY_OBJECTS: unnamed_0 = 165;
pub type unnamed_0 = libc::c_uint;
pub const _SC_AIO_LISTIO_MAX: unnamed_0 = 23;
pub const _SC_PII_OSI_COTS: unnamed_0 = 63;
pub const ZSTD_error_maxSymbolValue_tooSmall: ERR_enum = 48;
pub const _SC_CHARCLASS_NAME_MAX: unnamed_0 = 45;
pub const _SC_SCHAR_MIN: unnamed_0 = 112;
pub const _SC_THREAD_THREADS_MAX: unnamed_0 = 76;
pub const _SC_MAPPED_FILES: unnamed_0 = 16;
pub const _SC_PII_OSI_M: unnamed_0 = 65;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: unnamed_0 = 248;
pub const _SC_2_PBS_MESSAGE: unnamed_0 = 171;
pub const _SC_NL_NMAX: unnamed_0 = 122;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
}
pub type _IO_lock_t = ();
pub const _SC_RAW_SOCKETS: unnamed_0 = 236;
pub const _SC_ULONG_MAX: unnamed_0 = 117;
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
pub const _SC_MQ_PRIO_MAX: unnamed_0 = 28;
pub const _SC_NL_ARGMAX: unnamed_0 = 119;
pub const _SC_DEVICE_SPECIFIC_R: unnamed_0 = 142;
pub const _SC_TRACE_INHERIT: unnamed_0 = 183;
pub const _SC_THREAD_CPUTIME: unnamed_0 = 139;
pub type stat_t = stat;
pub const _SC_SHARED_MEMORY_OBJECTS: unnamed_0 = 22;
pub const _SC_TIMER_MAX: unnamed_0 = 35;
pub const _SC_PII: unnamed_0 = 53;
pub const _SC_STREAMS: unnamed_0 = 174;
pub const _SC_PHYS_PAGES: unnamed_0 = 85;
pub const _SC_LEVEL1_DCACHE_ASSOC: unnamed_0 = 189;
pub const _SC_SIGQUEUE_MAX: unnamed_0 = 34;
pub const _SC_IPV6: unnamed_0 = 235;
pub type clockid_t = __clockid_t;
pub const _SC_2_LOCALEDEF: unnamed_0 = 52;
pub const _SC_XOPEN_REALTIME: unnamed_0 = 130;
pub const _SC_ARG_MAX: unnamed_0 = 0;
pub const _SC_2_FORT_DEV: unnamed_0 = 49;
pub const _SC_V6_ILP32_OFF32: unnamed_0 = 176;
pub const _SC_V7_LP64_OFF64: unnamed_0 = 239;
pub const _SC_THREAD_PROCESS_SHARED: unnamed_0 = 82;
pub const _SC_LONG_BIT: unnamed_0 = 106;
pub const _SC_READER_WRITER_LOCKS: unnamed_0 = 153;
pub const _SC_XBS5_ILP32_OFFBIG: unnamed_0 = 126;
pub const _SC_THREAD_ATTR_STACKADDR: unnamed_0 = 77;
pub const _SC_LEVEL1_DCACHE_SIZE: unnamed_0 = 188;
pub const _SC_JOB_CONTROL: unnamed_0 = 7;
pub const ZSTD_error_maxCode: ERR_enum = 120;
pub const ZSTD_error_srcSize_wrong: ERR_enum = 72;
pub const _SC_V6_ILP32_OFFBIG: unnamed_0 = 177;
pub const _SC_PII_INTERNET_STREAM: unnamed_0 = 61;
pub type uint16_t = libc::c_ushort;
pub const ZSTD_error_dictionary_wrong: ERR_enum = 32;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZDICT_params_t {
    pub compressionLevel: libc::c_int,
    pub notificationLevel: libc::c_uint,
    pub dictID: libc::c_uint,
}
pub const _SC_BC_STRING_MAX: unnamed_0 = 39;
pub const _SC_AIO_MAX: unnamed_0 = 24;
pub const _SC_DEVICE_SPECIFIC: unnamed_0 = 141;
pub const _SC_THREAD_KEYS_MAX: unnamed_0 = 74;
pub const _SC_2_FORT_RUN: unnamed_0 = 50;
pub const _SC_NPROCESSORS_ONLN: unnamed_0 = 84;
pub const _SC_LEVEL1_ICACHE_LINESIZE: unnamed_0 = 187;
pub const _SC_THREAD_ATTR_STACKSIZE: unnamed_0 = 78;
pub const _SC_TRACE_NAME_MAX: unnamed_0 = 243;
pub type __off64_t = libc::c_long;
pub const ZSTD_error_dictionaryCreation_failed: ERR_enum = 34;
pub const _SC_POLL: unnamed_0 = 58;
pub const _SC_2_PBS_ACCOUNTING: unnamed_0 = 169;
pub const _SC_V6_LP64_OFF64: unnamed_0 = 178;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub const _SC_SAVED_IDS: unnamed_0 = 8;
pub type BYTE = uint8_t;
pub const _SC_LEVEL4_CACHE_SIZE: unnamed_0 = 197;
pub type size_t = libc::c_ulong;
pub const _SC_SEM_NSEMS_MAX: unnamed_0 = 32;
pub const _SC_NL_TEXTMAX: unnamed_0 = 124;
pub type U64 = uint64_t;
pub const _SC_XOPEN_ENH_I18N: unnamed_0 = 93;
pub const _SC_MEMORY_PROTECTION: unnamed_0 = 19;
pub type U16 = uint16_t;
pub const _SC_LEVEL3_CACHE_LINESIZE: unnamed_0 = 196;
pub const _SC_2_C_DEV: unnamed_0 = 48;
pub const _SC_PII_OSI_CLTS: unnamed_0 = 64;
pub const _SC_CHAR_BIT: unnamed_0 = 101;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: unnamed_0 = 73;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub const _SC_2_UPE: unnamed_0 = 97;
pub const _SC_XBS5_LP64_OFF64: unnamed_0 = 127;
pub const _SC_SIGNALS: unnamed_0 = 158;
pub const _SC_XOPEN_STREAMS: unnamed_0 = 246;
pub const _SC_TRACE_EVENT_FILTER: unnamed_0 = 182;
pub type __clockid_t = libc::c_int;
pub const ZSTD_error_frameParameter_unsupported: ERR_enum = 14;
pub const _SC_RE_DUP_MAX: unnamed_0 = 44;
pub const _SC_NPROCESSORS_CONF: unnamed_0 = 83;
pub const _SC_HOST_NAME_MAX: unnamed_0 = 180;
pub const _SC_NETWORKING: unnamed_0 = 152;
pub type __syscall_slong_t = libc::c_long;
pub const ZSTD_error_frameIndex_tooLarge: ERR_enum = 100;
pub type __gid_t = libc::c_uint;
pub const _SC_NL_SETMAX: unnamed_0 = 123;
pub const ZSTD_error_tableLog_tooLarge: ERR_enum = 44;
pub const MEM_static_assert: unnamed_1 = 1;
pub const _SC_AIO_PRIO_DELTA_MAX: unnamed_0 = 25;
pub const _SC_V7_LPBIG_OFFBIG: unnamed_0 = 240;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
pub const _SC_SEMAPHORES: unnamed_0 = 21;
pub const _SC_LEVEL1_ICACHE_SIZE: unnamed_0 = 185;
pub const _SC_PASS_MAX: unnamed_0 = 88;
pub const _SC_PII_SOCKET: unnamed_0 = 55;
pub const _SC_SHRT_MAX: unnamed_0 = 113;
pub type __blksize_t = libc::c_long;
pub const _SC_PRIORITIZED_IO: unnamed_0 = 13;
pub const _SC_V6_LPBIG_OFFBIG: unnamed_0 = 179;
pub const _SC_LEVEL2_CACHE_LINESIZE: unnamed_0 = 193;
pub const _SC_GETGR_R_SIZE_MAX: unnamed_0 = 69;
pub const _SC_2_SW_DEV: unnamed_0 = 51;
pub const _SC_REGEX_VERSION: unnamed_0 = 156;
pub const _SC_XOPEN_REALTIME_THREADS: unnamed_0 = 131;
pub type __uid_t = libc::c_uint;
pub const _SC_LEVEL1_ICACHE_ASSOC: unnamed_0 = 186;
pub type ZSTD_ErrorCode = ERR_enum;
pub const ZSTD_error_dstSize_tooSmall: ERR_enum = 70;
pub type __blkcnt_t = libc::c_long;
pub type ERR_enum = libc::c_uint;
pub const _SC_USER_GROUPS: unnamed_0 = 166;
pub const _SC_TIMERS: unnamed_0 = 11;
pub const _SC_ATEXIT_MAX: unnamed_0 = 87;
pub const _SC_LOGIN_NAME_MAX: unnamed_0 = 71;
pub const _SC_NL_MSGMAX: unnamed_0 = 121;
pub const _SC_USER_GROUPS_R: unnamed_0 = 167;
pub type ptrdiff_t = libc::c_long;
pub const _SC_SPAWN: unnamed_0 = 159;
pub const _SC_CHILD_MAX: unnamed_0 = 1;
pub const _SC_NL_LANGMAX: unnamed_0 = 120;
pub const _SC_CLOCK_SELECTION: unnamed_0 = 137;
pub const _SC_TRACE_USER_EVENT_MAX: unnamed_0 = 245;
pub const _SC_TRACE_EVENT_NAME_MAX: unnamed_0 = 242;
pub const ZSTD_error_parameter_outOfBound: ERR_enum = 42;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZDICT_cover_params_t {
    pub k: libc::c_uint,
    pub d: libc::c_uint,
    pub steps: libc::c_uint,
    pub nbThreads: libc::c_uint,
    pub zParams: ZDICT_params_t,
}
pub const _SC_SYSTEM_DATABASE: unnamed_0 = 162;
pub const _SC_EXPR_NEST_MAX: unnamed_0 = 42;
pub type __syscall_ulong_t = libc::c_ulong;
pub const _SC_XBS5_LPBIG_OFFBIG: unnamed_0 = 128;
pub const _SC_XBS5_ILP32_OFF32: unnamed_0 = 125;
pub type DIR = __dirstream;
pub const _SC_DELAYTIMER_MAX: unnamed_0 = 26;
pub const _SC_NGROUPS_MAX: unnamed_0 = 3;
pub const _SC_UINT_MAX: unnamed_0 = 116;
pub const ZSTD_error_parameter_unsupported: ERR_enum = 40;
pub const ZSTD_error_memory_allocation: ERR_enum = 64;
pub const _SC_MESSAGE_PASSING: unnamed_0 = 20;
pub const _SC_TZNAME_MAX: unnamed_0 = 6;
pub const _SC_ADVISORY_INFO: unnamed_0 = 132;
pub const _SC_SPIN_LOCKS: unnamed_0 = 154;
pub type U32 = uint32_t;
pub const ZSTD_error_corruption_detected: ERR_enum = 20;
pub const _SC_REALTIME_SIGNALS: unnamed_0 = 9;
pub const ZSTD_error_GENERIC: ERR_enum = 1;
pub const ZSTD_error_no_error: ERR_enum = 0;
pub const _SC_2_VERSION: unnamed_0 = 46;
pub const _SC_BC_BASE_MAX: unnamed_0 = 36;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
pub const _SC_COLL_WEIGHTS_MAX: unnamed_0 = 40;
pub type __time_t = libc::c_long;
pub const _SC_TRACE: unnamed_0 = 181;
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
pub type unnamed_1 = libc::c_uint;
pub const ZSTD_error_workSpace_tooSmall: ERR_enum = 66;
pub const ZSTD_error_dictionary_corrupted: ERR_enum = 30;
pub type __off_t = libc::c_long;
pub const _SC_SHRT_MIN: unnamed_0 = 114;
pub const _SC_FD_MGMT: unnamed_0 = 143;
pub const _SC_V7_ILP32_OFF32: unnamed_0 = 237;
pub const _SC_C_LANG_SUPPORT_R: unnamed_0 = 136;
pub const _SC_C_LANG_SUPPORT: unnamed_0 = 135;
pub type uint64_t = libc::c_ulong;
pub type __nlink_t = libc::c_ulong;
pub const _SC_XOPEN_LEGACY: unnamed_0 = 129;
pub const _SC_MQ_OPEN_MAX: unnamed_0 = 27;
pub const _SC_REGEXP: unnamed_0 = 155;
pub const _SC_FIFO: unnamed_0 = 144;
pub const _SC_EQUIV_CLASS_MAX: unnamed_0 = 41;
pub const _SC_NZERO: unnamed_0 = 109;
pub const _SC_SCHAR_MAX: unnamed_0 = 111;
pub const _SC_FILE_SYSTEM: unnamed_0 = 148;
pub const _SC_PII_INTERNET: unnamed_0 = 56;
pub const _SC_PII_XTI: unnamed_0 = 54;
pub const _SC_MEMLOCK: unnamed_0 = 17;
pub const _SC_IOV_MAX: unnamed_0 = 60;
pub const _SC_XOPEN_XCU_VERSION: unnamed_0 = 90;
pub const _SC_LEVEL2_CACHE_SIZE: unnamed_0 = 191;
pub const _SC_GETPW_R_SIZE_MAX: unnamed_0 = 70;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub const _SC_2_C_VERSION: unnamed_0 = 96;
pub const _SC_CHAR_MAX: unnamed_0 = 102;
pub const _SC_RTSIG_MAX: unnamed_0 = 31;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct fileStats {
    pub totalSizeToLoad: U64,
    pub oneSampleTooLarge: libc::c_uint,
    pub nbSamples: libc::c_uint,
}
pub const _SC_THREAD_PRIO_PROTECT: unnamed_0 = 81;
pub const _SC_VERSION: unnamed_0 = 29;
pub const _SC_BASE: unnamed_0 = 134;
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
    let one: unnamed = unnamed{u: 1i32 as U32,};
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
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as
               libc::c_int as libc::c_uint;
}
unsafe extern "C" fn ERR_getErrorCode(mut code: size_t) -> ERR_enum {
    if 0 == ERR_isError(code) {
        return ZSTD_error_no_error
    } else { return (0i32 as libc::c_ulong).wrapping_sub(code) as ERR_enum };
}
unsafe extern "C" fn ERR_getErrorName(mut code: size_t)
 -> *const libc::c_char {
    return ERR_getErrorString(ERR_getErrorCode(code));
}
#[no_mangle]
pub unsafe extern "C" fn DiB_trainFromFiles(mut dictFileName:
                                                *const libc::c_char,
                                            mut maxDictSize: libc::c_uint,
                                            mut fileNamesTable:
                                                *mut *const libc::c_char,
                                            mut nbFiles: libc::c_uint,
                                            mut chunkSize: size_t,
                                            mut params:
                                                *mut ZDICT_legacy_params_t,
                                            mut coverParams:
                                                *mut ZDICT_cover_params_t,
                                            mut optimizeCover: libc::c_int)
 -> libc::c_int {
    let displayLevel: libc::c_uint =
        if !params.is_null() {
            (*params).zParams.notificationLevel
        } else if !coverParams.is_null() {
            (*coverParams).zParams.notificationLevel
        } else { 0i32 as libc::c_uint };
    let dictBuffer: *mut libc::c_void = malloc(maxDictSize as libc::c_ulong);
    let fs: fileStats =
        DiB_fileStats(fileNamesTable, nbFiles, chunkSize, displayLevel);
    let sampleSizes: *mut size_t =
        malloc((fs.nbSamples as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>()
                                                    as libc::c_ulong)) as
            *mut size_t;
    let memMult: size_t =
        (if !params.is_null() { 11i32 } else { 9i32 }) as size_t;
    let maxMem: size_t =
        DiB_findMaxMem(fs.totalSizeToLoad.wrapping_mul(memMult) as
                           libc::c_ulonglong).wrapping_div(memMult);
    let mut loadedSize: size_t =
        (if (maxMem as libc::c_ulonglong) <
                fs.totalSizeToLoad as libc::c_ulonglong {
             maxMem as libc::c_ulonglong
         } else { fs.totalSizeToLoad as libc::c_ulonglong }) as size_t;
    let srcBuffer: *mut libc::c_void =
        malloc(loadedSize.wrapping_add(32i32 as libc::c_ulong));
    let mut result: libc::c_int = 0i32;
    if sampleSizes.is_null() || srcBuffer.is_null() || dictBuffer.is_null() {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 12],
                                          &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                12i32);
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 37],
                                          &mut [libc::c_char; 37]>(b"not enough memory for DiB_trainFiles\x00")).as_mut_ptr());
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 2],
                                          &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
        exit(12i32);
    } else {
        if 0 != fs.oneSampleTooLarge {
            if displayLevel >= 2i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 45],
                                                  &mut [libc::c_char; 45]>(b"!  Warning : some sample(s) are very large \n\x00")).as_mut_ptr());
            }
            if displayLevel >= 2i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 60],
                                                  &mut [libc::c_char; 60]>(b"!  Note that dictionary is only useful for small samples. \n\x00")).as_mut_ptr());
            }
            if displayLevel >= 2i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 73],
                                                  &mut [libc::c_char; 73]>(b"!  As a consequence, only the first %u bytes of each sample are loaded \n\x00")).as_mut_ptr(),
                        128i32 * (1i32 << 10i32));
            }
        }
        if fs.nbSamples < 5i32 as libc::c_uint {
            if displayLevel >= 2i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 61],
                                                  &mut [libc::c_char; 61]>(b"!  Warning : nb of samples too low for proper processing ! \n\x00")).as_mut_ptr());
            }
            if displayLevel >= 2i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 43],
                                                  &mut [libc::c_char; 43]>(b"!  Please provide _one file per sample_. \n\x00")).as_mut_ptr());
            }
            if displayLevel >= 2i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 91],
                                                  &mut [libc::c_char; 91]>(b"!  Alternatively, split files into fixed-size blocks representative of samples, with -B# \n\x00")).as_mut_ptr());
            }
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                    14i32);
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 22],
                                              &mut [libc::c_char; 22]>(b"nb of samples too low\x00")).as_mut_ptr());
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 2],
                                              &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
            exit(14i32);
        } else {
            if (fs.totalSizeToLoad as libc::c_ulonglong) <
                   (8i32 as libc::c_uint).wrapping_mul(maxDictSize) as
                       libc::c_ulonglong {
                if displayLevel >= 2i32 as libc::c_uint {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 73],
                                                      &mut [libc::c_char; 73]>(b"!  Warning : data size of samples too small for target dictionary size \n\x00")).as_mut_ptr());
                }
                if displayLevel >= 2i32 as libc::c_uint {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 69],
                                                      &mut [libc::c_char; 69]>(b"!  Samples should be about 100x larger than target dictionary size \n\x00")).as_mut_ptr());
                }
            }
            if loadedSize < fs.totalSizeToLoad {
                if displayLevel >= 1i32 as libc::c_uint {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 46],
                                                      &mut [libc::c_char; 46]>(b"Not enough memory; training on %u MB only...\n\x00")).as_mut_ptr(),
                            (loadedSize >> 20i32) as libc::c_uint);
                }
            }
            if displayLevel >= 3i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 23],
                                                  &mut [libc::c_char; 23]>(b"Shuffling input files\n\x00")).as_mut_ptr());
            }
            DiB_shuffle(fileNamesTable, nbFiles);
            nbFiles =
                DiB_loadFiles(srcBuffer, &mut loadedSize as *mut size_t,
                              sampleSizes, fs.nbSamples, fileNamesTable,
                              nbFiles, chunkSize, displayLevel);
            let mut dictSize: size_t = 0;
            if !params.is_null() {
                DiB_fillNoise((srcBuffer as
                                   *mut libc::c_char).offset(loadedSize as
                                                                 isize) as
                                  *mut libc::c_void, 32i32 as size_t);
                dictSize =
                    ZDICT_trainFromBuffer_unsafe_legacy(dictBuffer,
                                                        maxDictSize as size_t,
                                                        srcBuffer,
                                                        sampleSizes,
                                                        fs.nbSamples, *params)
            } else if 0 != optimizeCover {
                dictSize =
                    ZDICT_optimizeTrainFromBuffer_cover(dictBuffer,
                                                        maxDictSize as size_t,
                                                        srcBuffer,
                                                        sampleSizes,
                                                        fs.nbSamples,
                                                        coverParams);
                if 0 == ZDICT_isError(dictSize) {
                    if displayLevel >= 2i32 as libc::c_uint {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 20],
                                                          &mut [libc::c_char; 20]>(b"k=%u\nd=%u\nsteps=%u\n\x00")).as_mut_ptr(),
                                (*coverParams).k, (*coverParams).d,
                                (*coverParams).steps);
                    }
                }
            } else {
                dictSize =
                    ZDICT_trainFromBuffer_cover(dictBuffer,
                                                maxDictSize as size_t,
                                                srcBuffer, sampleSizes,
                                                fs.nbSamples, *coverParams)
            }
            if 0 != ZDICT_isError(dictSize) {
                if displayLevel >= 1i32 as libc::c_uint {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 34],
                                                      &mut [libc::c_char; 34]>(b"dictionary training failed : %s \n\x00")).as_mut_ptr(),
                            ZDICT_getErrorName(dictSize));
                }
                result = 1i32
            } else {
                if displayLevel >= 2i32 as libc::c_uint {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 42],
                                                      &mut [libc::c_char; 42]>(b"Save dictionary of size %u into file %s \n\x00")).as_mut_ptr(),
                            dictSize as U32, dictFileName);
                }
                DiB_saveDict(dictFileName, dictBuffer, dictSize);
            }
            free(srcBuffer);
            free(sampleSizes as *mut libc::c_void);
            free(dictBuffer);
            return result
        }
    };
}
unsafe extern "C" fn DiB_fileStats(mut fileNamesTable:
                                       *mut *const libc::c_char,
                                   mut nbFiles: libc::c_uint,
                                   mut chunkSize: size_t,
                                   mut displayLevel: libc::c_uint)
 -> fileStats {
    let mut fs: fileStats =
        fileStats{totalSizeToLoad: 0, oneSampleTooLarge: 0, nbSamples: 0,};
    let mut n: libc::c_uint = 0;
    memset(&mut fs as *mut fileStats as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<fileStats>() as libc::c_ulong);
    n = 0i32 as libc::c_uint;
    while n < nbFiles {
        let fileSize: U64 =
            UTIL_getFileSize(*fileNamesTable.offset(n as isize));
        let srcSize: U64 =
            if fileSize == -1i32 as U64 {
                0i32 as libc::c_ulong
            } else { fileSize };
        let nbSamples: U32 =
            (if 0 != chunkSize {
                 srcSize.wrapping_add(chunkSize.wrapping_sub(1i32 as
                                                                 libc::c_ulong)).wrapping_div(chunkSize)
             } else { 1i32 as libc::c_ulong }) as U32;
        let chunkToLoad: U64 =
            if 0 != chunkSize {
                if chunkSize < srcSize { chunkSize } else { srcSize }
            } else { srcSize };
        let cappedChunkSize: size_t =
            if chunkToLoad < (128i32 * (1i32 << 10i32)) as libc::c_ulong {
                chunkToLoad
            } else { (128i32 * (1i32 << 10i32)) as libc::c_ulong };
        fs.totalSizeToLoad =
            (fs.totalSizeToLoad as
                 libc::c_ulong).wrapping_add(cappedChunkSize.wrapping_mul(nbSamples
                                                                              as
                                                                              libc::c_ulong))
                as U64 as U64;
        fs.oneSampleTooLarge |=
            (chunkSize > (2i32 * (128i32 * (1i32 << 10i32))) as libc::c_ulong)
                as libc::c_int as libc::c_uint;
        fs.nbSamples = fs.nbSamples.wrapping_add(nbSamples);
        n = n.wrapping_add(1)
    }
    if displayLevel >= 4i32 as libc::c_uint {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 28],
                                          &mut [libc::c_char; 28]>(b"Preparing to load : %u KB \n\x00")).as_mut_ptr(),
                (fs.totalSizeToLoad >> 10i32) as U32);
    }
    return fs;
}
unsafe extern "C" fn DiB_findMaxMem(mut requiredMem: libc::c_ulonglong)
 -> size_t {
    let step: size_t = (8i32 * (1i32 << 20i32)) as size_t;
    let mut testmem: *mut libc::c_void = 0 as *mut libc::c_void;
    requiredMem =
        (requiredMem >> 23i32).wrapping_add(1i32 as libc::c_ulonglong) <<
            23i32;
    requiredMem = requiredMem.wrapping_add(step as libc::c_ulonglong);
    if requiredMem > g_maxMemory as libc::c_ulonglong {
        requiredMem = g_maxMemory as libc::c_ulonglong
    }
    while testmem.is_null() {
        testmem = malloc(requiredMem as size_t);
        requiredMem = requiredMem.wrapping_sub(step as libc::c_ulonglong)
    }
    free(testmem);
    return requiredMem as size_t;
}
static mut g_maxMemory: size_t = 0;
unsafe extern "C" fn DiB_saveDict(mut dictFileName: *const libc::c_char,
                                  mut buff: *const libc::c_void,
                                  mut buffSize: size_t) -> () {
    let f: *mut FILE =
        fopen(dictFileName,
              (*::std::mem::transmute::<&[u8; 3],
                                        &mut [libc::c_char; 3]>(b"wb\x00")).as_mut_ptr());
    if f.is_null() {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 12],
                                          &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                3i32);
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 16],
                                          &mut [libc::c_char; 16]>(b"cannot open %s \x00")).as_mut_ptr(),
                dictFileName);
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 2],
                                          &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
        exit(3i32);
    } else {
        let n: size_t = fwrite(buff, 1i32 as size_t, buffSize, f);
        if n != buffSize {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                    4i32);
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 17],
                                              &mut [libc::c_char; 17]>(b"%s : write error\x00")).as_mut_ptr(),
                    dictFileName);
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 2],
                                              &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
            exit(4i32);
        } else {
            let n_0: size_t = fclose(f) as size_t;
            if n_0 != 0i32 as libc::c_ulong {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 12],
                                                  &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                        5i32);
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 17],
                                                  &mut [libc::c_char; 17]>(b"%s : flush error\x00")).as_mut_ptr(),
                        dictFileName);
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 2],
                                                  &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
                exit(5i32);
            } else { return; }
        }
    };
}
unsafe extern "C" fn DiB_fillNoise(mut buffer: *mut libc::c_void,
                                   mut length: size_t) -> () {
    let prime1: libc::c_uint = 2654435761u32;
    let prime2: libc::c_uint = 2246822519u32;
    let mut acc: libc::c_uint = prime1;
    let mut p: size_t = 0i32 as size_t;
    p = 0i32 as size_t;
    while p < length {
        acc = acc.wrapping_mul(prime2);
        *(buffer as *mut libc::c_uchar).offset(p as isize) =
            (acc >> 21i32) as libc::c_uchar;
        p = p.wrapping_add(1)
    };
}
/* * DiB_loadFiles() :
 *  load samples from files listed in fileNamesTable into buffer.
 *  works even if buffer is too small to load all samples.
 *  Also provides the size of each sample into sampleSizes table
 *  which must be sized correctly, using DiB_fileStats().
 * @return : nb of samples effectively loaded into `buffer`
 * *bufferSizePtr is modified, it provides the amount data loaded within buffer.
 *  sampleSizes is filled with the size of each sample.
 */
unsafe extern "C" fn DiB_loadFiles(mut buffer: *mut libc::c_void,
                                   mut bufferSizePtr: *mut size_t,
                                   mut sampleSizes: *mut size_t,
                                   mut sstSize: libc::c_uint,
                                   mut fileNamesTable:
                                       *mut *const libc::c_char,
                                   mut nbFiles: libc::c_uint,
                                   mut targetChunkSize: size_t,
                                   mut displayLevel: libc::c_uint)
 -> libc::c_uint {
    let buff: *mut libc::c_char = buffer as *mut libc::c_char;
    let mut pos: size_t = 0i32 as size_t;
    let mut nbLoadedChunks: libc::c_uint = 0i32 as libc::c_uint;
    let mut fileIndex: libc::c_uint = 0;
    fileIndex = 0i32 as libc::c_uint;
    while fileIndex < nbFiles {
        let fileName: *const libc::c_char =
            *fileNamesTable.offset(fileIndex as isize);
        let fs64: libc::c_ulonglong =
            UTIL_getFileSize(fileName) as libc::c_ulonglong;
        let mut remainingToLoad: libc::c_ulonglong =
            if fs64 == -1i32 as U64 as libc::c_ulonglong {
                0i32 as libc::c_ulonglong
            } else { fs64 };
        let nbChunks: U32 =
            if 0 != targetChunkSize {
                fs64.wrapping_add(targetChunkSize.wrapping_sub(1i32 as
                                                                   libc::c_ulong)
                                      as
                                      libc::c_ulonglong).wrapping_div(targetChunkSize
                                                                          as
                                                                          libc::c_ulonglong)
                    as U32
            } else { 1i32 as libc::c_uint };
        let chunkSize: U64 =
            (if 0 != targetChunkSize {
                 if (targetChunkSize as libc::c_ulonglong) < fs64 {
                     targetChunkSize as libc::c_ulonglong
                 } else { fs64 }
             } else { fs64 }) as U64;
        let maxChunkSize: size_t =
            if chunkSize < (128i32 * (1i32 << 10i32)) as libc::c_ulong {
                chunkSize
            } else { (128i32 * (1i32 << 10i32)) as libc::c_ulong };
        let mut cnb: U32 = 0;
        let f: *mut FILE =
            fopen(fileName,
                  (*::std::mem::transmute::<&[u8; 3],
                                            &mut [libc::c_char; 3]>(b"rb\x00")).as_mut_ptr());
        if f.is_null() {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                    10i32);
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 26],
                                              &mut [libc::c_char; 26]>(b"zstd: dictBuilder: %s %s \x00")).as_mut_ptr(),
                    fileName, strerror(*__errno_location()));
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 2],
                                              &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
            exit(10i32);
        } else {
            if displayLevel >= 2i32 as libc::c_uint {
                if UTIL_clockSpanMicro(g_displayClock) > g_refreshRate ||
                       displayLevel >= 4i32 as libc::c_uint {
                    g_displayClock = UTIL_getTime();
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 22],
                                                      &mut [libc::c_char; 22]>(b"Loading %s...       \r\x00")).as_mut_ptr(),
                            fileName);
                    if displayLevel >= 4i32 as libc::c_uint {
                        fflush(stderr);
                    }
                }
            }
            cnb = 0i32 as U32;
            while cnb < nbChunks {
                let toLoad: size_t =
                    (if (maxChunkSize as libc::c_ulonglong) < remainingToLoad
                        {
                         maxChunkSize as libc::c_ulonglong
                     } else { remainingToLoad }) as size_t;
                if toLoad > (*bufferSizePtr).wrapping_sub(pos) { break ; }
                let readSize: size_t =
                    fread(buff.offset(pos as isize) as *mut libc::c_void,
                          1i32 as size_t, toLoad, f);
                if readSize != toLoad {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 12],
                                                      &mut [libc::c_char; 12]>(b"Error %i : \x00")).as_mut_ptr(),
                            11i32);
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 14],
                                                      &mut [libc::c_char; 14]>(b"Pb reading %s\x00")).as_mut_ptr(),
                            fileName);
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 2],
                                                      &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
                    exit(11i32);
                } else {
                    pos =
                        (pos as libc::c_ulong).wrapping_add(readSize) as
                            size_t as size_t;
                    let fresh1 = nbLoadedChunks;
                    nbLoadedChunks = nbLoadedChunks.wrapping_add(1);
                    *sampleSizes.offset(fresh1 as isize) = toLoad;
                    remainingToLoad =
                        remainingToLoad.wrapping_sub(targetChunkSize as
                                                         libc::c_ulonglong);
                    if nbLoadedChunks == sstSize {
                        fileIndex = nbFiles;
                        break ;
                    } else {
                        if toLoad < targetChunkSize {
                            fseek(f,
                                  targetChunkSize.wrapping_sub(toLoad) as
                                      libc::c_long, 1i32);
                        }
                        cnb = cnb.wrapping_add(1)
                    }
                }
            }
            fclose(f);
            fileIndex = fileIndex.wrapping_add(1)
        }
    }
    if displayLevel >= 2i32 as libc::c_uint {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 7],
                                          &mut [libc::c_char; 7]>(b"\r%79s\r\x00")).as_mut_ptr(),
                (*::std::mem::transmute::<&[u8; 1],
                                          &mut [libc::c_char; 1]>(b"\x00")).as_mut_ptr());
    }
    *bufferSizePtr = pos;
    if displayLevel >= 4i32 as libc::c_uint {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 17],
                                          &mut [libc::c_char; 17]>(b"loaded : %u KB \n\x00")).as_mut_ptr(),
                (pos >> 10i32) as U32);
    }
    return nbLoadedChunks;
}
static mut g_displayClock: UTIL_time_t =
    unsafe {
        timespec{tv_sec: 0i32 as __time_t,
                 tv_nsec: 0i32 as __syscall_slong_t,}
    };
static mut g_refreshRate: U64 = unsafe { (1000000i32 / 6i32) as U64 };
unsafe extern "C" fn DiB_shuffle(mut fileNamesTable: *mut *const libc::c_char,
                                 mut nbFiles: libc::c_uint) -> () {
    let mut seed: U32 = 4247762216u32;
    let mut i: libc::c_uint = 0;
    i = nbFiles.wrapping_sub(1i32 as libc::c_uint);
    while i > 0i32 as libc::c_uint {
        let j: libc::c_uint =
            DiB_rand(&mut seed as
                         *mut U32).wrapping_rem(i.wrapping_add(1i32 as
                                                                   libc::c_uint));
        let tmp: *const libc::c_char = *fileNamesTable.offset(j as isize);
        let ref mut fresh2 = *fileNamesTable.offset(j as isize);
        *fresh2 = *fileNamesTable.offset(i as isize);
        let ref mut fresh3 = *fileNamesTable.offset(i as isize);
        *fresh3 = tmp;
        i = i.wrapping_sub(1)
    };
}
unsafe extern "C" fn DiB_rand(mut src: *mut U32) -> U32 {
    static mut prime1: U32 = unsafe { 2654435761u32 };
    static mut prime2: U32 = unsafe { 2246822519u32 };
    let mut rand32: U32 = *src;
    rand32 = (rand32 as libc::c_uint).wrapping_mul(prime1) as U32 as U32;
    rand32 ^= prime2;
    rand32 = rand32 << 13i32 | rand32 >> 32i32 - 13i32;
    *src = rand32;
    return rand32 >> 5i32;
}
#[no_mangle]
pub unsafe extern "C" fn DiB_isError(mut errorCode: size_t) -> libc::c_uint {
    return ERR_isError(errorCode);
}
#[no_mangle]
pub unsafe extern "C" fn DiB_getErrorName(mut errorCode: size_t)
 -> *const libc::c_char {
    return ERR_getErrorName(errorCode);
}
unsafe extern "C" fn run_static_initializers() -> () {
    g_maxMemory =
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
            ((512i32 * (1i32 << 20i32)) as size_t) <<
                ::std::mem::size_of::<size_t>() as libc::c_ulong
        }
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn() -> (); 1] =
    [run_static_initializers];
