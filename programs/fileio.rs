#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( extern_types , libc , offset_to , used )]
extern crate libc;
extern "C" {
    pub type ZSTD_CCtx_s;
    pub type __dirstream;
    pub type ZSTD_DCtx_s;
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn chown(__file: *const libc::c_char, __owner: __uid_t, __group: __gid_t)
     -> libc::c_int;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
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
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
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
    fn getchar() -> libc::c_int;
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
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
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
    fn clock() -> clock_t;
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
    fn ZSTD_getFrameContentSize(src: *const libc::c_void, srcSize: size_t)
     -> libc::c_ulonglong;
    #[no_mangle]
    fn ZSTD_isError(code: size_t) -> libc::c_uint;
    #[no_mangle]
    fn ZSTD_getErrorName(code: size_t) -> *const libc::c_char;
    #[no_mangle]
    fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
    #[no_mangle]
    fn ZSTD_freeCStream(zcs: *mut ZSTD_CStream) -> size_t;
    #[no_mangle]
    fn ZSTD_CStreamInSize() -> size_t;
    #[no_mangle]
    fn ZSTD_CStreamOutSize() -> size_t;
    #[no_mangle]
    fn ZSTD_createDStream() -> *mut ZSTD_DStream;
    #[no_mangle]
    fn ZSTD_freeDStream(zds: *mut ZSTD_DStream) -> size_t;
    #[no_mangle]
    fn ZSTD_decompressStream(zds: *mut ZSTD_DStream,
                             output: *mut ZSTD_outBuffer,
                             input: *mut ZSTD_inBuffer) -> size_t;
    #[no_mangle]
    fn ZSTD_DStreamInSize() -> size_t;
    #[no_mangle]
    fn ZSTD_DStreamOutSize() -> size_t;
    #[no_mangle]
    fn ZSTD_frameHeaderSize(src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_isFrame(buffer: *const libc::c_void, size: size_t)
     -> libc::c_uint;
    #[no_mangle]
    fn ZSTD_getFrameProgression(cctx: *const ZSTD_CCtx)
     -> ZSTD_frameProgression;
    #[no_mangle]
    fn ZSTD_setDStreamParameter(zds: *mut ZSTD_DStream,
                                paramType: ZSTD_DStreamParameter_e,
                                paramValue: libc::c_uint) -> size_t;
    #[no_mangle]
    fn ZSTD_initDStream_usingDict(zds: *mut ZSTD_DStream,
                                  dict: *const libc::c_void, dictSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_resetDStream(zds: *mut ZSTD_DStream) -> size_t;
    #[no_mangle]
    fn ZSTD_getFrameHeader(zfhPtr: *mut ZSTD_frameHeader,
                           src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_CCtx_setParameter(cctx: *mut ZSTD_CCtx, param: ZSTD_cParameter,
                              value: libc::c_uint) -> size_t;
    #[no_mangle]
    fn ZSTD_CCtx_setPledgedSrcSize(cctx: *mut ZSTD_CCtx,
                                   pledgedSrcSize: libc::c_ulonglong)
     -> size_t;
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
    fn signal(__sig: libc::c_int, __handler: __sighandler_t)
     -> __sighandler_t;
    #[no_mangle]
    fn ZSTD_getErrorCode(functionResult: size_t) -> ZSTD_ErrorCode;
}
pub const _SC_SSIZE_MAX: unnamed_1 = 110;
pub const _SC_LINE_MAX: unnamed_1 = 43;
pub const _SC_CPUTIME: unnamed_1 = 138;
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
pub const _SC_NPROCESSORS_CONF: unnamed_1 = 83;
pub const _SC_TZNAME_MAX: unnamed_1 = 6;
pub const _SC_SEM_NSEMS_MAX: unnamed_1 = 32;
pub const _SC_PII_SOCKET: unnamed_1 = 55;
pub type DIR = __dirstream;
pub const _SC_LEVEL1_ICACHE_ASSOC: unnamed_1 = 186;
pub type UTIL_time_t = timespec;
pub const _SC_POLL: unnamed_1 = 58;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: unnamed_1 = 248;
pub const _SC_2_FORT_DEV: unnamed_1 = 49;
pub const _SC_NL_NMAX: unnamed_1 = 122;
pub const _SC_2_PBS_LOCATE: unnamed_1 = 170;
pub const _SC_TRACE_INHERIT: unnamed_1 = 183;
pub const ZSTD_p_searchLog: ZSTD_cParameter = 104;
pub const _SC_FD_MGMT: unnamed_1 = 143;
pub const _SC_AVPHYS_PAGES: unnamed_1 = 86;
pub const _SC_PASS_MAX: unnamed_1 = 88;
pub const _SC_JOB_CONTROL: unnamed_1 = 7;
pub const ZSTD_p_ldmHashEveryLog: ZSTD_cParameter = 164;
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub const _SC_THREAD_PRIO_PROTECT: unnamed_1 = 81;
pub const ZSTD_p_contentSizeFlag: ZSTD_cParameter = 200;
pub const _SC_2_PBS_TRACK: unnamed_1 = 172;
pub const _SC_COLL_WEIGHTS_MAX: unnamed_1 = 40;
pub const _SC_LEVEL2_CACHE_SIZE: unnamed_1 = 191;
pub const _SC_V6_ILP32_OFFBIG: unnamed_1 = 177;
pub const _SC_VERSION: unnamed_1 = 29;
pub const _SC_NL_SETMAX: unnamed_1 = 123;
pub const _SC_LEVEL4_CACHE_SIZE: unnamed_1 = 197;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub const _SC_CHARCLASS_NAME_MAX: unnamed_1 = 45;
pub const ZSTD_p_compressionStrategy: ZSTD_cParameter = 107;
pub const _SC_2_CHAR_TERM: unnamed_1 = 95;
pub const _SC_SAVED_IDS: unnamed_1 = 8;
pub const _SC_V7_LPBIG_OFFBIG: unnamed_1 = 240;
pub const _SC_SCHAR_MIN: unnamed_1 = 112;
pub type __mode_t = libc::c_uint;
pub const _SC_LEVEL1_DCACHE_LINESIZE: unnamed_1 = 190;
pub const _SC_CHAR_MIN: unnamed_1 = 103;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cRess_t {
    pub srcFile: *mut FILE,
    pub dstFile: *mut FILE,
    pub srcBuffer: *mut libc::c_void,
    pub srcBufferSize: size_t,
    pub dstBuffer: *mut libc::c_void,
    pub dstBufferSize: size_t,
    pub cctx: *mut ZSTD_CStream,
}
pub type __ino_t = libc::c_ulong;
pub const _SC_NL_LANGMAX: unnamed_1 = 120;
pub const _SC_LONG_BIT: unnamed_1 = 106;
pub const _SC_LEVEL2_CACHE_ASSOC: unnamed_1 = 192;
pub const ZSTD_p_checksumFlag: ZSTD_cParameter = 201;
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
/* *< position where reading stopped. Will be updated. Necessarily 0 <= pos <= size */
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
pub const _SC_CHAR_MAX: unnamed_1 = 102;
pub const DStream_p_maxWindowSize: ZSTD_DStreamParameter_e = 0;
pub const _SC_XOPEN_VERSION: unnamed_1 = 89;
pub const _SC_2_SW_DEV: unnamed_1 = 51;
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
pub const _SC_EQUIV_CLASS_MAX: unnamed_1 = 41;
pub const ZSTD_p_forceMaxWindow: ZSTD_cParameter = 1100;
pub const _SC_XOPEN_ENH_I18N: unnamed_1 = 93;
pub type size_t = libc::c_ulong;
pub const _SC_PII_INTERNET_STREAM: unnamed_1 = 61;
pub const _SC_FILE_LOCKING: unnamed_1 = 147;
pub const _SC_DEVICE_IO: unnamed_1 = 140;
pub const ZSTD_p_minMatch: ZSTD_cParameter = 105;
pub const _SC_SELECT: unnamed_1 = 59;
pub const _SC_PII_XTI: unnamed_1 = 54;
pub const ZSTD_p_targetLength: ZSTD_cParameter = 106;
pub const _SC_REGEX_VERSION: unnamed_1 = 156;
pub const _SC_BARRIERS: unnamed_1 = 133;
pub const _SC_CLK_TCK: unnamed_1 = 2;
pub type clockid_t = __clockid_t;
pub type uint8_t = libc::c_uchar;
pub const _SC_TTY_NAME_MAX: unnamed_1 = 72;
pub const ZSTD_p_chainLog: ZSTD_cParameter = 103;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
pub const ZSTD_p_dictIDFlag: ZSTD_cParameter = 202;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
pub const ZSTD_p_forceAttachDict: ZSTD_cParameter = 1101;
pub const _SC_MQ_OPEN_MAX: unnamed_1 = 27;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: unnamed_1 = 73;
pub const ZSTD_p_enableLongDistanceMatching: ZSTD_cParameter = 160;
pub const _SC_V6_LP64_OFF64: unnamed_1 = 178;
pub const _SC_TIMEOUTS: unnamed_1 = 164;
pub const _SC_AIO_LISTIO_MAX: unnamed_1 = 23;
pub const _SC_ASYNCHRONOUS_IO: unnamed_1 = 12;
pub const _SC_PII_INTERNET: unnamed_1 = 56;
pub const _SC_CHAR_BIT: unnamed_1 = 101;
pub const ZSTD_btopt: ZSTD_strategy = 7;
/* *< position where writing stopped. Will be updated. Necessarily 0 <= pos <= size */
pub type ZSTD_CStream = ZSTD_CCtx;
pub const _SC_THREAD_PROCESS_SHARED: unnamed_1 = 82;
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
pub type ZSTD_EndDirective = libc::c_uint;
pub const _SC_XOPEN_UNIX: unnamed_1 = 91;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub const _SC_BC_SCALE_MAX: unnamed_1 = 38;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type uint64_t = libc::c_ulong;
pub type stat_t = stat;
pub const _SC_SCHAR_MAX: unnamed_1 = 111;
pub const _SC_LEVEL4_CACHE_ASSOC: unnamed_1 = 198;
pub const _SC_2_PBS_CHECKPOINT: unnamed_1 = 175;
pub const _SC_UCHAR_MAX: unnamed_1 = 115;
pub const _SC_XOPEN_LEGACY: unnamed_1 = 129;
pub const FIO_lz4Compression: FIO_compressionType_t = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const ZSTD_p_format: ZSTD_cParameter = 10;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub type __dev_t = libc::c_ulong;
pub type ZSTD_DCtx = ZSTD_DCtx_s;
pub const _SC_THREADS: unnamed_1 = 67;
pub const _SC_TRACE_USER_EVENT_MAX: unnamed_1 = 245;
pub const _SC_SHELL: unnamed_1 = 157;
pub type __time_t = libc::c_long;
pub const _SC_PII_OSI: unnamed_1 = 57;
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
pub const _SC_AIO_PRIO_DELTA_MAX: unnamed_1 = 25;
pub const _SC_LEVEL4_CACHE_LINESIZE: unnamed_1 = 199;
pub const ZSTD_skippableFrame: ZSTD_frameType_e = 1;
pub type __clock_t = libc::c_long;
pub const FIO_gzipCompression: FIO_compressionType_t = 1;
pub const _SC_THREAD_PRIO_INHERIT: unnamed_1 = 80;
pub const _SC_SYSTEM_DATABASE_R: unnamed_1 = 163;
pub const ZSTD_e_continue: ZSTD_EndDirective = 0;
pub type uint16_t = libc::c_ushort;
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
pub const _SC_LEVEL1_ICACHE_LINESIZE: unnamed_1 = 187;
pub const _SC_SYNCHRONIZED_IO: unnamed_1 = 14;
pub const _SC_XOPEN_CRYPT: unnamed_1 = 92;
pub const _SC_NL_MSGMAX: unnamed_1 = 121;
pub const _SC_THREAD_KEYS_MAX: unnamed_1 = 74;
pub const _SC_CLOCK_SELECTION: unnamed_1 = 137;
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
pub const _SC_XOPEN_XPG2: unnamed_1 = 98;
pub const _SC_2_UPE: unnamed_1 = 97;
pub const _SC_PII_OSI_COTS: unnamed_1 = 63;
pub const _SC_FIFO: unnamed_1 = 144;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_frameProgression {
    pub ingested: libc::c_ulonglong,
    pub consumed: libc::c_ulonglong,
    pub produced: libc::c_ulonglong,
}
pub type U32 = uint32_t;
pub const ZSTD_p_jobSize: ZSTD_cParameter = 401;
pub const _SC_2_VERSION: unnamed_1 = 46;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub const _SC_V7_ILP32_OFF32: unnamed_1 = 237;
pub type uint32_t = libc::c_uint;
pub const _SC_PII_OSI_CLTS: unnamed_1 = 64;
pub const ZSTD_p_hashLog: ZSTD_cParameter = 102;
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
pub const _SC_LOGIN_NAME_MAX: unnamed_1 = 71;
pub const _SC_2_PBS_MESSAGE: unnamed_1 = 171;
pub const _SC_AIO_MAX: unnamed_1 = 24;
pub const _SC_LEVEL1_DCACHE_SIZE: unnamed_1 = 188;
pub const ZSTD_e_flush: ZSTD_EndDirective = 1;
pub type ZSTD_ErrorCode = libc::c_uint;
/* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
pub const _SC_LEVEL1_DCACHE_ASSOC: unnamed_1 = 189;
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
pub const _SC_STREAMS: unnamed_1 = 174;
pub type __clockid_t = libc::c_int;
pub const _SC_MULTI_PROCESS: unnamed_1 = 150;
pub const _SC_THREAD_PRIORITY_SCHEDULING: unnamed_1 = 79;
pub const _SC_SIGQUEUE_MAX: unnamed_1 = 34;
pub const _SC_MESSAGE_PASSING: unnamed_1 = 20;
pub const _SC_TRACE_EVENT_FILTER: unnamed_1 = 182;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct fileInfo_t {
    pub decompressedSize: U64,
    pub compressedSize: U64,
    pub windowSize: U64,
    pub numActualFrames: libc::c_int,
    pub numSkippableFrames: libc::c_int,
    pub decompUnavailable: libc::c_int,
    pub usesCheck: libc::c_int,
    pub nbFiles: U32,
}
pub const _SC_SYMLOOP_MAX: unnamed_1 = 173;
pub const _SC_REGEXP: unnamed_1 = 155;
pub const _SC_2_FORT_RUN: unnamed_1 = 50;
pub const _SC_STREAM_MAX: unnamed_1 = 5;
pub const _SC_PHYS_PAGES: unnamed_1 = 85;
pub const _SC_IPV6: unnamed_1 = 235;
pub const _SC_BC_DIM_MAX: unnamed_1 = 37;
pub const _SC_THREAD_CPUTIME: unnamed_1 = 139;
pub const _SC_PII: unnamed_1 = 53;
pub const _SC_DEVICE_SPECIFIC: unnamed_1 = 141;
pub const _SC_XOPEN_STREAMS: unnamed_1 = 246;
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
pub const _SC_RE_DUP_MAX: unnamed_1 = 44;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct dRess_t {
    pub srcBuffer: *mut libc::c_void,
    pub srcBufferSize: size_t,
    pub srcBufferLoaded: size_t,
    pub dstBuffer: *mut libc::c_void,
    pub dstBufferSize: size_t,
    pub dctx: *mut ZSTD_DStream,
    pub dstFile: *mut FILE,
}
pub const _SC_NGROUPS_MAX: unnamed_1 = 3;
pub const _SC_MB_LEN_MAX: unnamed_1 = 108;
pub const _SC_THREAD_STACK_MIN: unnamed_1 = 75;
pub const _SC_ULONG_MAX: unnamed_1 = 117;
pub const _SC_USHRT_MAX: unnamed_1 = 118;
pub const _SC_REALTIME_SIGNALS: unnamed_1 = 9;
pub const _SC_EXPR_NEST_MAX: unnamed_1 = 42;
pub type FIO_compressionType_t = libc::c_uint;
pub const _SC_USER_GROUPS: unnamed_1 = 166;
pub const ZSTD_p_compressionLevel: ZSTD_cParameter = 100;
pub const _SC_C_LANG_SUPPORT_R: unnamed_1 = 136;
pub const _SC_UINT_MAX: unnamed_1 = 116;
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
pub const _SC_XOPEN_SHM: unnamed_1 = 94;
pub const _SC_CHILD_MAX: unnamed_1 = 1;
pub const _SC_NPROCESSORS_ONLN: unnamed_1 = 84;
pub const _SC_SEM_VALUE_MAX: unnamed_1 = 33;
pub const _SC_READER_WRITER_LOCKS: unnamed_1 = 153;
pub const _SC_MQ_PRIO_MAX: unnamed_1 = 28;
pub const _SC_LEVEL3_CACHE_ASSOC: unnamed_1 = 195;
pub const _SC_BASE: unnamed_1 = 134;
pub const _SC_V6_LPBIG_OFFBIG: unnamed_1 = 179;
pub const _SC_MONOTONIC_CLOCK: unnamed_1 = 149;
pub type U64 = uint64_t;
pub const _SC_XOPEN_XPG3: unnamed_1 = 99;
pub const _SC_THREAD_ATTR_STACKADDR: unnamed_1 = 77;
pub const ZSTD_lazy: ZSTD_strategy = 4;
pub type __blkcnt_t = libc::c_long;
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
pub const _SC_SIGNALS: unnamed_1 = 158;
pub const _SC_SHARED_MEMORY_OBJECTS: unnamed_1 = 22;
pub const _SC_FSYNC: unnamed_1 = 15;
pub const _SC_XBS5_LP64_OFF64: unnamed_1 = 127;
pub const _SC_PRIORITIZED_IO: unnamed_1 = 13;
pub const ZSTD_fast: ZSTD_strategy = 1;
pub const _SC_TRACE_SYS_MAX: unnamed_1 = 244;
pub const _SC_THREAD_SAFE_FUNCTIONS: unnamed_1 = 68;
pub const _SC_2_C_VERSION: unnamed_1 = 96;
pub type U16 = uint16_t;
pub const _SC_SS_REPL_MAX: unnamed_1 = 241;
pub const _SC_IOV_MAX: unnamed_1 = 60;
pub const _SC_2_LOCALEDEF: unnamed_1 = 52;
pub const _SC_DEVICE_SPECIFIC_R: unnamed_1 = 142;
pub type ZSTD_cParameter = libc::c_uint;
pub const _SC_PRIORITY_SCHEDULING: unnamed_1 = 10;
pub const _SC_BC_BASE_MAX: unnamed_1 = 36;
pub const _SC_XOPEN_XPG4: unnamed_1 = 100;
pub const _SC_NL_ARGMAX: unnamed_1 = 119;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const _SC_GETGR_R_SIZE_MAX: unnamed_1 = 69;
pub const _SC_INT_MAX: unnamed_1 = 104;
pub const _SC_GETPW_R_SIZE_MAX: unnamed_1 = 70;
pub const _SC_C_LANG_SUPPORT: unnamed_1 = 135;
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
pub const ZSTD_p_ldmMinMatch: ZSTD_cParameter = 162;
pub const _SC_PIPE: unnamed_1 = 145;
pub const FIO_xzCompression: FIO_compressionType_t = 2;
pub const _SC_TYPED_MEMORY_OBJECTS: unnamed_1 = 165;
pub const ZSTD_frame: ZSTD_frameType_e = 0;
pub const _SC_MEMLOCK_RANGE: unnamed_1 = 18;
pub const FIO_lzmaCompression: FIO_compressionType_t = 3;
pub const _SC_SHRT_MAX: unnamed_1 = 113;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub const _SC_TRACE: unnamed_1 = 181;
pub const _SC_THREAD_ATTR_STACKSIZE: unnamed_1 = 78;
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
pub const _SC_XBS5_LPBIG_OFFBIG: unnamed_1 = 128;
pub const ZSTD_p_overlapSizeLog: ZSTD_cParameter = 402;
pub const _SC_FILE_SYSTEM: unnamed_1 = 148;
pub const _SC_SPIN_LOCKS: unnamed_1 = 154;
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const _SC_HOST_NAME_MAX: unnamed_1 = 180;
pub const _SC_XBS5_ILP32_OFF32: unnamed_1 = 125;
pub const _SC_LEVEL2_CACHE_LINESIZE: unnamed_1 = 193;
pub const _SC_V6_ILP32_OFF32: unnamed_1 = 176;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_inBuffer_s {
    pub src: *const libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type unnamed = libc::c_uint;
pub const _SC_SYSTEM_DATABASE: unnamed_1 = 162;
/* **************************************
*  Explicit context
***************************************/
/* !< maximum compression level available */
/* !< provides readable string from an error code */
pub type ZSTD_CCtx = ZSTD_CCtx_s;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    u: U32,
    c: [BYTE; 4],
}
pub const _SC_UIO_MAXIOV: unnamed_1 = 60;
pub const _SC_MEMORY_PROTECTION: unnamed_1 = 19;
pub const _SC_TIMER_MAX: unnamed_1 = 35;
pub type __syscall_ulong_t = libc::c_ulong;
pub const _SC_THREAD_SPORADIC_SERVER: unnamed_1 = 161;
pub type __off_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
pub type unnamed_1 = libc::c_uint;
pub type time_t = __time_t;
pub type clock_t = __clock_t;
pub const _SC_ATEXIT_MAX: unnamed_1 = 87;
pub type __blksize_t = libc::c_long;
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub type ptrdiff_t = libc::c_long;
pub const _SC_LEVEL3_CACHE_SIZE: unnamed_1 = 194;
pub const _SC_INT_MIN: unnamed_1 = 105;
pub const FIO_zstdCompression: FIO_compressionType_t = 0;
pub const _SC_WORD_BIT: unnamed_1 = 107;
pub const _SC_2_PBS_ACCOUNTING: unnamed_1 = 169;
pub type __nlink_t = libc::c_ulong;
pub const _SC_PII_OSI_M: unnamed_1 = 65;
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
pub type __uid_t = libc::c_uint;
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
pub const _SC_NL_TEXTMAX: unnamed_1 = 124;
pub const _SC_XOPEN_REALTIME: unnamed_1 = 130;
pub const _SC_ADVISORY_INFO: unnamed_1 = 132;
pub const ZSTD_p_ldmBucketSizeLog: ZSTD_cParameter = 163;
pub const _SC_TRACE_NAME_MAX: unnamed_1 = 243;
/* ***************************
*  Streaming
****************************/
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub const _SC_V7_ILP32_OFFBIG: unnamed_1 = 238;
pub const _SC_NETWORKING: unnamed_1 = 152;
pub const ZSTD_p_nbWorkers: ZSTD_cParameter = 400;
pub const _SC_SPAWN: unnamed_1 = 159;
pub const _SC_RTSIG_MAX: unnamed_1 = 31;
pub const _SC_TRACE_LOG: unnamed_1 = 184;
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
}
pub type __gid_t = libc::c_uint;
pub const _SC_ARG_MAX: unnamed_1 = 0;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
pub const _SC_LEVEL3_CACHE_LINESIZE: unnamed_1 = 196;
pub const _SC_V7_LP64_OFF64: unnamed_1 = 239;
pub type ZSTD_frameType_e = libc::c_uint;
pub const _SC_THREAD_THREADS_MAX: unnamed_1 = 76;
pub const ZSTD_e_end: ZSTD_EndDirective = 2;
pub const _SC_OPEN_MAX: unnamed_1 = 4;
pub const _SC_XOPEN_XCU_VERSION: unnamed_1 = 90;
pub const _SC_T_IOV_MAX: unnamed_1 = 66;
pub const _SC_LEVEL1_ICACHE_SIZE: unnamed_1 = 185;
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
pub const _SC_SINGLE_PROCESS: unnamed_1 = 151;
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
pub const _SC_SEMAPHORES: unnamed_1 = 21;
pub const _SC_TIMERS: unnamed_1 = 11;
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_frameHeader {
    pub frameContentSize: libc::c_ulonglong,
    pub windowSize: libc::c_ulonglong,
    pub blockSizeMax: libc::c_uint,
    pub frameType: ZSTD_frameType_e,
    pub headerSize: libc::c_uint,
    pub dictID: libc::c_uint,
    pub checksumFlag: libc::c_uint,
}
pub const _SC_BC_STRING_MAX: unnamed_1 = 39;
pub type ZSTD_DStreamParameter_e = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: unnamed_1 = 247;
pub const _SC_2_C_DEV: unnamed_1 = 48;
pub const _SC_SHRT_MIN: unnamed_1 = 114;
pub const _SC_PAGESIZE: unnamed_1 = 30;
pub const _SC_TRACE_EVENT_NAME_MAX: unnamed_1 = 242;
pub const _SC_FILE_ATTRIBUTES: unnamed_1 = 146;
pub const _SC_RAW_SOCKETS: unnamed_1 = 236;
pub const _SC_MAPPED_FILES: unnamed_1 = 16;
pub type FILE = _IO_FILE;
pub const _SC_XOPEN_REALTIME_THREADS: unnamed_1 = 131;
pub const _SC_PII_INTERNET_DGRAM: unnamed_1 = 62;
pub const _SC_2_PBS: unnamed_1 = 168;
pub const _SC_USER_GROUPS_R: unnamed_1 = 167;
pub type ZSTD_strategy = libc::c_uint;
pub const _SC_DELAYTIMER_MAX: unnamed_1 = 26;
pub type BYTE = uint8_t;
pub const _SC_XBS5_ILP32_OFFBIG: unnamed_1 = 126;
pub type _IO_lock_t = ();
pub const _SC_2_C_BIND: unnamed_1 = 47;
pub const _SC_NZERO: unnamed_1 = 109;
pub const ZSTD_p_ldmHashLog: ZSTD_cParameter = 161;
pub type __off64_t = libc::c_long;
pub const ZSTD_p_windowLog: ZSTD_cParameter = 101;
pub const _SC_MEMLOCK: unnamed_1 = 17;
pub const _SC_SPORADIC_SERVER: unnamed_1 = 160;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
pub const MEM_static_assert: unnamed = 1;
/* *< recommended size for output buffer. Guarantee to successfully flush at least one complete compressed block in all circumstances. */
/* *< recommended size for input buffer */
pub type ZSTD_DStream = ZSTD_DCtx;
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
    let one: unnamed_0 = unnamed_0{u: 1i32 as U32,};
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
                                current_block = 1757844130948290377;
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
                            current_block = 1757844130948290377;
                            break ;
                        }
                        cpu_cores = atoi(sep_0.offset(1isize))
                    } else if 0 != ferror(cpuinfo) {
                        current_block = 1757844130948290377;
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
pub unsafe extern "C" fn FIO_setCompressionType(mut compressionType:
                                                    FIO_compressionType_t)
 -> () {
    g_compressionType = compressionType;
}
static mut g_compressionType: FIO_compressionType_t =
    unsafe { FIO_zstdCompression };
#[no_mangle]
pub unsafe extern "C" fn FIO_overwriteMode() -> () {
    g_overwrite = 1i32 as U32;
}
static mut g_overwrite: U32 = unsafe { 0i32 as U32 };
#[no_mangle]
pub unsafe extern "C" fn FIO_setNotificationLevel(mut level: libc::c_uint)
 -> () {
    g_displayLevel = level as libc::c_int;
}
static mut g_displayLevel: libc::c_int = unsafe { 2i32 };
#[no_mangle]
pub unsafe extern "C" fn FIO_setSparseWrite(mut sparse: libc::c_uint) -> () {
    g_sparseFileSupport = sparse;
}
static mut g_sparseFileSupport: U32 = unsafe { 1i32 as U32 };
/* *< 0: no sparse; 1: disable on stdout; 2: always enabled */
#[no_mangle]
pub unsafe extern "C" fn FIO_setDictIDFlag(mut dictIDFlag: libc::c_uint)
 -> () {
    g_dictIDFlag = dictIDFlag;
}
static mut g_dictIDFlag: U32 = unsafe { 1i32 as U32 };
#[no_mangle]
pub unsafe extern "C" fn FIO_setChecksumFlag(mut checksumFlag: libc::c_uint)
 -> () {
    g_checksumFlag = checksumFlag;
}
static mut g_checksumFlag: U32 = unsafe { 1i32 as U32 };
#[no_mangle]
pub unsafe extern "C" fn FIO_setRemoveSrcFile(mut flag: libc::c_uint) -> () {
    g_removeSrcFile = (flag > 0i32 as libc::c_uint) as libc::c_int as U32;
}
static mut g_removeSrcFile: U32 = unsafe { 0i32 as U32 };
#[no_mangle]
pub unsafe extern "C" fn FIO_setMemLimit(mut memLimit: libc::c_uint) -> () {
    g_memLimit = memLimit;
}
static mut g_memLimit: U32 = unsafe { 0i32 as U32 };
#[no_mangle]
pub unsafe extern "C" fn FIO_setNbWorkers(mut nbWorkers: libc::c_uint) -> () {
    if nbWorkers > 0i32 as libc::c_uint {
        if g_displayLevel >= 2i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 37],
                                              &mut [libc::c_char; 37]>(b"Note : multi-threading is disabled \n\x00")).as_mut_ptr());
        }
    }
    g_nbWorkers = nbWorkers;
}
static mut g_nbWorkers: U32 = unsafe { 1i32 as U32 };
#[no_mangle]
pub unsafe extern "C" fn FIO_setBlockSize(mut blockSize: libc::c_uint) -> () {
    if 0 != blockSize && g_nbWorkers == 0i32 as libc::c_uint {
        if g_displayLevel >= 2i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 54],
                                              &mut [libc::c_char; 54]>(b"Setting block size is useless in single-thread mode \n\x00")).as_mut_ptr());
        }
    }
    g_blockSize = blockSize;
}
static mut g_blockSize: U32 = unsafe { 0i32 as U32 };
#[no_mangle]
pub unsafe extern "C" fn FIO_setOverlapLog(mut overlapLog: libc::c_uint)
 -> () {
    if 0 != overlapLog && g_nbWorkers == 0i32 as libc::c_uint {
        if g_displayLevel >= 2i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 54],
                                              &mut [libc::c_char; 54]>(b"Setting overlapLog is useless in single-thread mode \n\x00")).as_mut_ptr());
        }
    }
    g_overlapLog = overlapLog;
}
static mut g_overlapLog: U32 = unsafe { 9999i32 as U32 };
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmFlag(mut ldmFlag: libc::c_uint) -> () {
    g_ldmFlag = (ldmFlag > 0i32 as libc::c_uint) as libc::c_int as U32;
}
static mut g_ldmFlag: U32 = unsafe { 0i32 as U32 };
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmHashLog(mut ldmHashLog: libc::c_uint)
 -> () {
    g_ldmHashLog = ldmHashLog;
}
static mut g_ldmHashLog: U32 = unsafe { 0i32 as U32 };
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmMinMatch(mut ldmMinMatch: libc::c_uint)
 -> () {
    g_ldmMinMatch = ldmMinMatch;
}
static mut g_ldmMinMatch: U32 = unsafe { 0i32 as U32 };
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmBucketSizeLog(mut ldmBucketSizeLog:
                                                     libc::c_uint) -> () {
    g_ldmBucketSizeLog = ldmBucketSizeLog;
}
static mut g_ldmBucketSizeLog: U32 = unsafe { 9999i32 as U32 };
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmHashEveryLog(mut ldmHashEveryLog:
                                                    libc::c_uint) -> () {
    g_ldmHashEveryLog = ldmHashEveryLog;
}
static mut g_ldmHashEveryLog: U32 = unsafe { 9999i32 as U32 };
/* * FIO_compressFilename() :
    @return : 0 == ok;  1 == pb with src file. */
#[no_mangle]
pub unsafe extern "C" fn FIO_compressFilename(mut dstFileName:
                                                  *const libc::c_char,
                                              mut srcFileName:
                                                  *const libc::c_char,
                                              mut dictFileName:
                                                  *const libc::c_char,
                                              mut compressionLevel:
                                                  libc::c_int,
                                              mut comprParams:
                                                  *mut ZSTD_compressionParameters)
 -> libc::c_int {
    let start: clock_t = clock();
    let fileSize: U64 = UTIL_getFileSize(srcFileName);
    let srcSize: U64 =
        (if fileSize == -1i32 as U64 {
             0u64.wrapping_sub(1i32 as libc::c_ulonglong)
         } else { fileSize as libc::c_ulonglong }) as U64;
    let ress: cRess_t =
        FIO_createCResources(dictFileName, compressionLevel, srcSize,
                             comprParams);
    let result: libc::c_int =
        FIO_compressFilename_dstFile(ress, dstFileName, srcFileName,
                                     compressionLevel);
    let seconds: libc::c_double =
        (clock() - start) as libc::c_double /
            1000000i32 as clock_t as libc::c_double;
    if g_displayLevel >= 4i32 {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 24],
                                          &mut [libc::c_char; 24]>(b"Completed in %.2f sec \n\x00")).as_mut_ptr(),
                seconds);
    }
    FIO_freeCResources(ress);
    return result;
}
unsafe extern "C" fn FIO_createCResources(mut dictFileName:
                                              *const libc::c_char,
                                          mut cLevel: libc::c_int,
                                          mut srcSize: U64,
                                          mut comprParams:
                                              *mut ZSTD_compressionParameters)
 -> cRess_t {
    let mut ress: cRess_t =
        cRess_t{srcFile: 0 as *mut FILE,
                dstFile: 0 as *mut FILE,
                srcBuffer: 0 as *mut libc::c_void,
                srcBufferSize: 0,
                dstBuffer: 0 as *mut libc::c_void,
                dstBufferSize: 0,
                cctx: 0 as *mut ZSTD_CStream,};
    memset(&mut ress as *mut cRess_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<cRess_t>() as libc::c_ulong);
    if g_displayLevel >= 6i32 {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 23],
                                          &mut [libc::c_char; 23]>(b"FIO_createCResources \n\x00")).as_mut_ptr());
    }
    ress.cctx = ZSTD_createCCtx();
    if ress.cctx.is_null() {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 7],
                                              &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                    30i32);
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 42],
                                              &mut [libc::c_char; 42]>(b"allocation error : can\'t create ZSTD_CCtx\x00")).as_mut_ptr());
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 3],
                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
        }
        exit(30i32);
    } else {
        ress.srcBufferSize = ZSTD_CStreamInSize();
        ress.srcBuffer = malloc(ress.srcBufferSize);
        ress.dstBufferSize = ZSTD_CStreamOutSize();
        ress.dstBuffer = malloc(ress.dstBufferSize);
        if ress.srcBuffer.is_null() || ress.dstBuffer.is_null() {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 7],
                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 12],
                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                        31i32);
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 37],
                                                  &mut [libc::c_char; 37]>(b"allocation error : not enough memory\x00")).as_mut_ptr());
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 3],
                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
            }
            exit(31i32);
        } else {
            let mut dictBuffer: *mut libc::c_void = 0 as *mut libc::c_void;
            let dictBuffSize: size_t =
                FIO_createDictBuffer(&mut dictBuffer as
                                         *mut *mut libc::c_void,
                                     dictFileName);
            if !dictFileName.is_null() && dictBuffer.is_null() {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 7],
                                                      &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 12],
                                                      &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                            32i32);
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 43],
                                                      &mut [libc::c_char; 43]>(b"allocation error : can\'t create dictBuffer\x00")).as_mut_ptr());
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 3],
                                                      &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                }
                exit(32i32);
            } else {
                let mut err: size_t = 0;
                err =
                    ZSTD_CCtx_setParameter(ress.cctx, ZSTD_p_contentSizeFlag,
                                           1i32 as libc::c_uint);
                if 0 != ZSTD_isError(err) {
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 7],
                                                          &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                11i32);
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                ZSTD_getErrorName(err));
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                    }
                    exit(11i32);
                } else {
                    let mut err_0: size_t = 0;
                    err_0 =
                        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_p_dictIDFlag,
                                               g_dictIDFlag);
                    if 0 != ZSTD_isError(err_0) {
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 7],
                                                              &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 12],
                                                              &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                    11i32);
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 3],
                                                              &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                    ZSTD_getErrorName(err_0));
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 3],
                                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                        }
                        exit(11i32);
                    } else {
                        let mut err_1: size_t = 0;
                        err_1 =
                            ZSTD_CCtx_setParameter(ress.cctx,
                                                   ZSTD_p_checksumFlag,
                                                   g_checksumFlag);
                        if 0 != ZSTD_isError(err_1) {
                            if g_displayLevel >= 1i32 {
                                fprintf(stderr,
                                        (*::std::mem::transmute::<&[u8; 7],
                                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                            }
                            if g_displayLevel >= 1i32 {
                                fprintf(stderr,
                                        (*::std::mem::transmute::<&[u8; 12],
                                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                        11i32);
                            }
                            if g_displayLevel >= 1i32 {
                                fprintf(stderr,
                                        (*::std::mem::transmute::<&[u8; 3],
                                                                  &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                        ZSTD_getErrorName(err_1));
                            }
                            if g_displayLevel >= 1i32 {
                                fprintf(stderr,
                                        (*::std::mem::transmute::<&[u8; 3],
                                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                            }
                            exit(11i32);
                        } else {
                            let mut err_2: size_t = 0;
                            err_2 =
                                ZSTD_CCtx_setParameter(ress.cctx,
                                                       ZSTD_p_compressionLevel,
                                                       cLevel as
                                                           libc::c_uint);
                            if 0 != ZSTD_isError(err_2) {
                                if g_displayLevel >= 1i32 {
                                    fprintf(stderr,
                                            (*::std::mem::transmute::<&[u8; 7],
                                                                      &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                }
                                if g_displayLevel >= 1i32 {
                                    fprintf(stderr,
                                            (*::std::mem::transmute::<&[u8; 12],
                                                                      &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                            11i32);
                                }
                                if g_displayLevel >= 1i32 {
                                    fprintf(stderr,
                                            (*::std::mem::transmute::<&[u8; 3],
                                                                      &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                            ZSTD_getErrorName(err_2));
                                }
                                if g_displayLevel >= 1i32 {
                                    fprintf(stderr,
                                            (*::std::mem::transmute::<&[u8; 3],
                                                                      &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                }
                                exit(11i32);
                            } else {
                                let mut err_3: size_t = 0;
                                err_3 =
                                    ZSTD_CCtx_setParameter(ress.cctx,
                                                           ZSTD_p_enableLongDistanceMatching,
                                                           g_ldmFlag);
                                if 0 != ZSTD_isError(err_3) {
                                    if g_displayLevel >= 1i32 {
                                        fprintf(stderr,
                                                (*::std::mem::transmute::<&[u8; 7],
                                                                          &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                    }
                                    if g_displayLevel >= 1i32 {
                                        fprintf(stderr,
                                                (*::std::mem::transmute::<&[u8; 12],
                                                                          &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                                11i32);
                                    }
                                    if g_displayLevel >= 1i32 {
                                        fprintf(stderr,
                                                (*::std::mem::transmute::<&[u8; 3],
                                                                          &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                                ZSTD_getErrorName(err_3));
                                    }
                                    if g_displayLevel >= 1i32 {
                                        fprintf(stderr,
                                                (*::std::mem::transmute::<&[u8; 3],
                                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                    }
                                    exit(11i32);
                                } else {
                                    let mut err_4: size_t = 0;
                                    err_4 =
                                        ZSTD_CCtx_setParameter(ress.cctx,
                                                               ZSTD_p_ldmHashLog,
                                                               g_ldmHashLog);
                                    if 0 != ZSTD_isError(err_4) {
                                        if g_displayLevel >= 1i32 {
                                            fprintf(stderr,
                                                    (*::std::mem::transmute::<&[u8; 7],
                                                                              &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                        }
                                        if g_displayLevel >= 1i32 {
                                            fprintf(stderr,
                                                    (*::std::mem::transmute::<&[u8; 12],
                                                                              &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                                    11i32);
                                        }
                                        if g_displayLevel >= 1i32 {
                                            fprintf(stderr,
                                                    (*::std::mem::transmute::<&[u8; 3],
                                                                              &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                                    ZSTD_getErrorName(err_4));
                                        }
                                        if g_displayLevel >= 1i32 {
                                            fprintf(stderr,
                                                    (*::std::mem::transmute::<&[u8; 3],
                                                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                        }
                                        exit(11i32);
                                    } else {
                                        let mut err_5: size_t = 0;
                                        err_5 =
                                            ZSTD_CCtx_setParameter(ress.cctx,
                                                                   ZSTD_p_ldmMinMatch,
                                                                   g_ldmMinMatch);
                                        if 0 != ZSTD_isError(err_5) {
                                            if g_displayLevel >= 1i32 {
                                                fprintf(stderr,
                                                        (*::std::mem::transmute::<&[u8; 7],
                                                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                            }
                                            if g_displayLevel >= 1i32 {
                                                fprintf(stderr,
                                                        (*::std::mem::transmute::<&[u8; 12],
                                                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                                        11i32);
                                            }
                                            if g_displayLevel >= 1i32 {
                                                fprintf(stderr,
                                                        (*::std::mem::transmute::<&[u8; 3],
                                                                                  &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                                        ZSTD_getErrorName(err_5));
                                            }
                                            if g_displayLevel >= 1i32 {
                                                fprintf(stderr,
                                                        (*::std::mem::transmute::<&[u8; 3],
                                                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                            }
                                            exit(11i32);
                                        } else {
                                            if g_ldmBucketSizeLog !=
                                                   9999i32 as libc::c_uint {
                                                let mut err_6: size_t = 0;
                                                err_6 =
                                                    ZSTD_CCtx_setParameter(ress.cctx,
                                                                           ZSTD_p_ldmBucketSizeLog,
                                                                           g_ldmBucketSizeLog);
                                                if 0 != ZSTD_isError(err_6) {
                                                    if g_displayLevel >= 1i32
                                                       {
                                                        fprintf(stderr,
                                                                (*::std::mem::transmute::<&[u8; 7],
                                                                                          &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                                    }
                                                    if g_displayLevel >= 1i32
                                                       {
                                                        fprintf(stderr,
                                                                (*::std::mem::transmute::<&[u8; 12],
                                                                                          &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                                                11i32);
                                                    }
                                                    if g_displayLevel >= 1i32
                                                       {
                                                        fprintf(stderr,
                                                                (*::std::mem::transmute::<&[u8; 3],
                                                                                          &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                                                ZSTD_getErrorName(err_6));
                                                    }
                                                    if g_displayLevel >= 1i32
                                                       {
                                                        fprintf(stderr,
                                                                (*::std::mem::transmute::<&[u8; 3],
                                                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                                    }
                                                    exit(11i32);
                                                }
                                            }
                                            if g_ldmHashEveryLog !=
                                                   9999i32 as libc::c_uint {
                                                let mut err_7: size_t = 0;
                                                err_7 =
                                                    ZSTD_CCtx_setParameter(ress.cctx,
                                                                           ZSTD_p_ldmHashEveryLog,
                                                                           g_ldmHashEveryLog);
                                                if 0 != ZSTD_isError(err_7) {
                                                    if g_displayLevel >= 1i32
                                                       {
                                                        fprintf(stderr,
                                                                (*::std::mem::transmute::<&[u8; 7],
                                                                                          &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                                    }
                                                    if g_displayLevel >= 1i32
                                                       {
                                                        fprintf(stderr,
                                                                (*::std::mem::transmute::<&[u8; 12],
                                                                                          &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                                                11i32);
                                                    }
                                                    if g_displayLevel >= 1i32
                                                       {
                                                        fprintf(stderr,
                                                                (*::std::mem::transmute::<&[u8; 3],
                                                                                          &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                                                ZSTD_getErrorName(err_7));
                                                    }
                                                    if g_displayLevel >= 1i32
                                                       {
                                                        fprintf(stderr,
                                                                (*::std::mem::transmute::<&[u8; 3],
                                                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                                    }
                                                    exit(11i32);
                                                }
                                            }
                                            let mut err_8: size_t = 0;
                                            err_8 =
                                                ZSTD_CCtx_setParameter(ress.cctx,
                                                                       ZSTD_p_windowLog,
                                                                       (*comprParams).windowLog);
                                            if 0 != ZSTD_isError(err_8) {
                                                if g_displayLevel >= 1i32 {
                                                    fprintf(stderr,
                                                            (*::std::mem::transmute::<&[u8; 7],
                                                                                      &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                                }
                                                if g_displayLevel >= 1i32 {
                                                    fprintf(stderr,
                                                            (*::std::mem::transmute::<&[u8; 12],
                                                                                      &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                                            11i32);
                                                }
                                                if g_displayLevel >= 1i32 {
                                                    fprintf(stderr,
                                                            (*::std::mem::transmute::<&[u8; 3],
                                                                                      &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                                            ZSTD_getErrorName(err_8));
                                                }
                                                if g_displayLevel >= 1i32 {
                                                    fprintf(stderr,
                                                            (*::std::mem::transmute::<&[u8; 3],
                                                                                      &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                                }
                                                exit(11i32);
                                            } else {
                                                let mut err_9: size_t = 0;
                                                err_9 =
                                                    ZSTD_CCtx_setParameter(ress.cctx,
                                                                           ZSTD_p_chainLog,
                                                                           (*comprParams).chainLog);
                                                if 0 != ZSTD_isError(err_9) {
                                                    if g_displayLevel >= 1i32
                                                       {
                                                        fprintf(stderr,
                                                                (*::std::mem::transmute::<&[u8; 7],
                                                                                          &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                                    }
                                                    if g_displayLevel >= 1i32
                                                       {
                                                        fprintf(stderr,
                                                                (*::std::mem::transmute::<&[u8; 12],
                                                                                          &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                                                11i32);
                                                    }
                                                    if g_displayLevel >= 1i32
                                                       {
                                                        fprintf(stderr,
                                                                (*::std::mem::transmute::<&[u8; 3],
                                                                                          &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                                                ZSTD_getErrorName(err_9));
                                                    }
                                                    if g_displayLevel >= 1i32
                                                       {
                                                        fprintf(stderr,
                                                                (*::std::mem::transmute::<&[u8; 3],
                                                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                                    }
                                                    exit(11i32);
                                                } else {
                                                    let mut err_10: size_t =
                                                        0;
                                                    err_10 =
                                                        ZSTD_CCtx_setParameter(ress.cctx,
                                                                               ZSTD_p_hashLog,
                                                                               (*comprParams).hashLog);
                                                    if 0 !=
                                                           ZSTD_isError(err_10)
                                                       {
                                                        if g_displayLevel >=
                                                               1i32 {
                                                            fprintf(stderr,
                                                                    (*::std::mem::transmute::<&[u8; 7],
                                                                                              &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                                        }
                                                        if g_displayLevel >=
                                                               1i32 {
                                                            fprintf(stderr,
                                                                    (*::std::mem::transmute::<&[u8; 12],
                                                                                              &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                                                    11i32);
                                                        }
                                                        if g_displayLevel >=
                                                               1i32 {
                                                            fprintf(stderr,
                                                                    (*::std::mem::transmute::<&[u8; 3],
                                                                                              &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                                                    ZSTD_getErrorName(err_10));
                                                        }
                                                        if g_displayLevel >=
                                                               1i32 {
                                                            fprintf(stderr,
                                                                    (*::std::mem::transmute::<&[u8; 3],
                                                                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                                        }
                                                        exit(11i32);
                                                    } else {
                                                        let mut err_11:
                                                                size_t = 0;
                                                        err_11 =
                                                            ZSTD_CCtx_setParameter(ress.cctx,
                                                                                   ZSTD_p_searchLog,
                                                                                   (*comprParams).searchLog);
                                                        if 0 !=
                                                               ZSTD_isError(err_11)
                                                           {
                                                            if g_displayLevel
                                                                   >= 1i32 {
                                                                fprintf(stderr,
                                                                        (*::std::mem::transmute::<&[u8; 7],
                                                                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                                            }
                                                            if g_displayLevel
                                                                   >= 1i32 {
                                                                fprintf(stderr,
                                                                        (*::std::mem::transmute::<&[u8; 12],
                                                                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                                                        11i32);
                                                            }
                                                            if g_displayLevel
                                                                   >= 1i32 {
                                                                fprintf(stderr,
                                                                        (*::std::mem::transmute::<&[u8; 3],
                                                                                                  &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                                                        ZSTD_getErrorName(err_11));
                                                            }
                                                            if g_displayLevel
                                                                   >= 1i32 {
                                                                fprintf(stderr,
                                                                        (*::std::mem::transmute::<&[u8; 3],
                                                                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                                            }
                                                            exit(11i32);
                                                        } else {
                                                            let mut err_12:
                                                                    size_t =
                                                                0;
                                                            err_12 =
                                                                ZSTD_CCtx_setParameter(ress.cctx,
                                                                                       ZSTD_p_minMatch,
                                                                                       (*comprParams).searchLength);
                                                            if 0 !=
                                                                   ZSTD_isError(err_12)
                                                               {
                                                                if g_displayLevel
                                                                       >= 1i32
                                                                   {
                                                                    fprintf(stderr,
                                                                            (*::std::mem::transmute::<&[u8; 7],
                                                                                                      &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                                                }
                                                                if g_displayLevel
                                                                       >= 1i32
                                                                   {
                                                                    fprintf(stderr,
                                                                            (*::std::mem::transmute::<&[u8; 12],
                                                                                                      &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                                                            11i32);
                                                                }
                                                                if g_displayLevel
                                                                       >= 1i32
                                                                   {
                                                                    fprintf(stderr,
                                                                            (*::std::mem::transmute::<&[u8; 3],
                                                                                                      &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                                                            ZSTD_getErrorName(err_12));
                                                                }
                                                                if g_displayLevel
                                                                       >= 1i32
                                                                   {
                                                                    fprintf(stderr,
                                                                            (*::std::mem::transmute::<&[u8; 3],
                                                                                                      &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                                                }
                                                                exit(11i32);
                                                            } else {
                                                                let mut err_13:
                                                                        size_t =
                                                                    0;
                                                                err_13 =
                                                                    ZSTD_CCtx_setParameter(ress.cctx,
                                                                                           ZSTD_p_targetLength,
                                                                                           (*comprParams).targetLength);
                                                                if 0 !=
                                                                       ZSTD_isError(err_13)
                                                                   {
                                                                    if g_displayLevel
                                                                           >=
                                                                           1i32
                                                                       {
                                                                        fprintf(stderr,
                                                                                (*::std::mem::transmute::<&[u8; 7],
                                                                                                          &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                                                    }
                                                                    if g_displayLevel
                                                                           >=
                                                                           1i32
                                                                       {
                                                                        fprintf(stderr,
                                                                                (*::std::mem::transmute::<&[u8; 12],
                                                                                                          &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                                                                11i32);
                                                                    }
                                                                    if g_displayLevel
                                                                           >=
                                                                           1i32
                                                                       {
                                                                        fprintf(stderr,
                                                                                (*::std::mem::transmute::<&[u8; 3],
                                                                                                          &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                                                                ZSTD_getErrorName(err_13));
                                                                    }
                                                                    if g_displayLevel
                                                                           >=
                                                                           1i32
                                                                       {
                                                                        fprintf(stderr,
                                                                                (*::std::mem::transmute::<&[u8; 3],
                                                                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                                                    }
                                                                    exit(11i32);
                                                                } else {
                                                                    let mut err_14:
                                                                            size_t =
                                                                        0;
                                                                    err_14 =
                                                                        ZSTD_CCtx_setParameter(ress.cctx,
                                                                                               ZSTD_p_compressionStrategy,
                                                                                               (*comprParams).strategy
                                                                                                   as
                                                                                                   U32);
                                                                    if 0 !=
                                                                           ZSTD_isError(err_14)
                                                                       {
                                                                        if g_displayLevel
                                                                               >=
                                                                               1i32
                                                                           {
                                                                            fprintf(stderr,
                                                                                    (*::std::mem::transmute::<&[u8; 7],
                                                                                                              &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                                                        }
                                                                        if g_displayLevel
                                                                               >=
                                                                               1i32
                                                                           {
                                                                            fprintf(stderr,
                                                                                    (*::std::mem::transmute::<&[u8; 12],
                                                                                                              &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                                                                    11i32);
                                                                        }
                                                                        if g_displayLevel
                                                                               >=
                                                                               1i32
                                                                           {
                                                                            fprintf(stderr,
                                                                                    (*::std::mem::transmute::<&[u8; 3],
                                                                                                              &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                                                                    ZSTD_getErrorName(err_14));
                                                                        }
                                                                        if g_displayLevel
                                                                               >=
                                                                               1i32
                                                                           {
                                                                            fprintf(stderr,
                                                                                    (*::std::mem::transmute::<&[u8; 3],
                                                                                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                                                        }
                                                                        exit(11i32);
                                                                    } else {
                                                                        let mut err_15:
                                                                                size_t =
                                                                            0;
                                                                        err_15
                                                                            =
                                                                            ZSTD_CCtx_setPledgedSrcSize(ress.cctx,
                                                                                                        srcSize
                                                                                                            as
                                                                                                            libc::c_ulonglong);
                                                                        if 0
                                                                               !=
                                                                               ZSTD_isError(err_15)
                                                                           {
                                                                            if g_displayLevel
                                                                                   >=
                                                                                   1i32
                                                                               {
                                                                                fprintf(stderr,
                                                                                        (*::std::mem::transmute::<&[u8; 7],
                                                                                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                                                            }
                                                                            if g_displayLevel
                                                                                   >=
                                                                                   1i32
                                                                               {
                                                                                fprintf(stderr,
                                                                                        (*::std::mem::transmute::<&[u8; 12],
                                                                                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                                                                        11i32);
                                                                            }
                                                                            if g_displayLevel
                                                                                   >=
                                                                                   1i32
                                                                               {
                                                                                fprintf(stderr,
                                                                                        (*::std::mem::transmute::<&[u8; 3],
                                                                                                                  &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                                                                        ZSTD_getErrorName(err_15));
                                                                            }
                                                                            if g_displayLevel
                                                                                   >=
                                                                                   1i32
                                                                               {
                                                                                fprintf(stderr,
                                                                                        (*::std::mem::transmute::<&[u8; 3],
                                                                                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                                                            }
                                                                            exit(11i32);
                                                                        } else {
                                                                            let mut err_16:
                                                                                    size_t =
                                                                                0;
                                                                            err_16
                                                                                =
                                                                                ZSTD_CCtx_loadDictionary(ress.cctx,
                                                                                                         dictBuffer,
                                                                                                         dictBuffSize);
                                                                            if 0
                                                                                   !=
                                                                                   ZSTD_isError(err_16)
                                                                               {
                                                                                if g_displayLevel
                                                                                       >=
                                                                                       1i32
                                                                                   {
                                                                                    fprintf(stderr,
                                                                                            (*::std::mem::transmute::<&[u8; 7],
                                                                                                                      &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                                                                }
                                                                                if g_displayLevel
                                                                                       >=
                                                                                       1i32
                                                                                   {
                                                                                    fprintf(stderr,
                                                                                            (*::std::mem::transmute::<&[u8; 12],
                                                                                                                      &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                                                                            11i32);
                                                                                }
                                                                                if g_displayLevel
                                                                                       >=
                                                                                       1i32
                                                                                   {
                                                                                    fprintf(stderr,
                                                                                            (*::std::mem::transmute::<&[u8; 3],
                                                                                                                      &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                                                                            ZSTD_getErrorName(err_16));
                                                                                }
                                                                                if g_displayLevel
                                                                                       >=
                                                                                       1i32
                                                                                   {
                                                                                    fprintf(stderr,
                                                                                            (*::std::mem::transmute::<&[u8; 3],
                                                                                                                      &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                                                                }
                                                                                exit(11i32);
                                                                            } else {
                                                                                let mut err_17:
                                                                                        size_t =
                                                                                    0;
                                                                                err_17
                                                                                    =
                                                                                    ZSTD_CCtx_setPledgedSrcSize(ress.cctx,
                                                                                                                0u64.wrapping_sub(1i32
                                                                                                                                      as
                                                                                                                                      libc::c_ulonglong));
                                                                                if 0
                                                                                       !=
                                                                                       ZSTD_isError(err_17)
                                                                                   {
                                                                                    if g_displayLevel
                                                                                           >=
                                                                                           1i32
                                                                                       {
                                                                                        fprintf(stderr,
                                                                                                (*::std::mem::transmute::<&[u8; 7],
                                                                                                                          &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                                                                                    }
                                                                                    if g_displayLevel
                                                                                           >=
                                                                                           1i32
                                                                                       {
                                                                                        fprintf(stderr,
                                                                                                (*::std::mem::transmute::<&[u8; 12],
                                                                                                                          &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                                                                                11i32);
                                                                                    }
                                                                                    if g_displayLevel
                                                                                           >=
                                                                                           1i32
                                                                                       {
                                                                                        fprintf(stderr,
                                                                                                (*::std::mem::transmute::<&[u8; 3],
                                                                                                                          &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                                                                                ZSTD_getErrorName(err_17));
                                                                                    }
                                                                                    if g_displayLevel
                                                                                           >=
                                                                                           1i32
                                                                                       {
                                                                                        fprintf(stderr,
                                                                                                (*::std::mem::transmute::<&[u8; 3],
                                                                                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                                                                                    }
                                                                                    exit(11i32);
                                                                                } else {
                                                                                    free(dictBuffer);
                                                                                    return ress
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
/* ! FIO_createDictBuffer() :
 *  creates a buffer, pointed by `*bufferPtr`,
 *  loads `filename` content into it, up to DICTSIZE_MAX bytes.
 * @return : loaded size
 *  if fileName==NULL, returns 0 and a NULL pointer
 */
unsafe extern "C" fn FIO_createDictBuffer(mut bufferPtr:
                                              *mut *mut libc::c_void,
                                          mut fileName: *const libc::c_char)
 -> size_t {
    let mut fileHandle: *mut FILE = 0 as *mut FILE;
    let mut fileSize: U64 = 0;
    *bufferPtr = 0 as *mut libc::c_void;
    if fileName.is_null() {
        return 0i32 as size_t
    } else {
        if g_displayLevel >= 4i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 27],
                                              &mut [libc::c_char; 27]>(b"Loading %s as dictionary \n\x00")).as_mut_ptr(),
                    fileName);
        }
        fileHandle =
            fopen(fileName,
                  (*::std::mem::transmute::<&[u8; 3],
                                            &mut [libc::c_char; 3]>(b"rb\x00")).as_mut_ptr());
        if fileHandle.is_null() {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 7],
                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 12],
                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                        31i32);
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 7],
                                                  &mut [libc::c_char; 7]>(b"%s: %s\x00")).as_mut_ptr(),
                        fileName, strerror(*__errno_location()));
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 3],
                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
            }
            exit(31i32);
        } else {
            fileSize = UTIL_getFileSize(fileName);
            if fileSize > (32i32 * (1i32 << 20i32)) as libc::c_ulong {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 7],
                                                      &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 12],
                                                      &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                            32i32);
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 42],
                                                      &mut [libc::c_char; 42]>(b"Dictionary file %s is too large (> %u MB)\x00")).as_mut_ptr(),
                            fileName, 32i32 * (1i32 << 20i32) >> 20i32);
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 3],
                                                      &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                }
                exit(32i32);
            } else {
                *bufferPtr = malloc(fileSize);
                if (*bufferPtr).is_null() {
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 7],
                                                          &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                34i32);
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                strerror(*__errno_location()));
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                    }
                    exit(34i32);
                } else {
                    let readSize: size_t =
                        fread(*bufferPtr, 1i32 as size_t, fileSize,
                              fileHandle);
                    if readSize != fileSize {
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 7],
                                                              &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 12],
                                                              &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                    35i32);
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 33],
                                                              &mut [libc::c_char; 33]>(b"Error reading dictionary file %s\x00")).as_mut_ptr(),
                                    fileName);
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 3],
                                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                        }
                        exit(35i32);
                    } else { fclose(fileHandle); return fileSize }
                }
            }
        }
    };
}
/* ! FIO_compressFilename_dstFile() :
 *  @return : 0 : compression completed correctly,
 *            1 : pb
 */
unsafe extern "C" fn FIO_compressFilename_dstFile(mut ress: cRess_t,
                                                  mut dstFileName:
                                                      *const libc::c_char,
                                                  mut srcFileName:
                                                      *const libc::c_char,
                                                  mut compressionLevel:
                                                      libc::c_int)
 -> libc::c_int {
    let mut result: libc::c_int = 0;
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
    let mut stat_result: libc::c_int = 0i32;
    if g_displayLevel >= 6i32 {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 46],
                                          &mut [libc::c_char; 46]>(b"FIO_compressFilename_dstFile: opening dst: %s\x00")).as_mut_ptr(),
                dstFileName);
    }
    ress.dstFile = FIO_openDstFile(dstFileName);
    if ress.dstFile.is_null() {
        return 1i32
    } else {
        addHandler(dstFileName);
        if 0 !=
               strcmp(srcFileName,
                      (*::std::mem::transmute::<&[u8; 10],
                                                &mut [libc::c_char; 10]>(b"/*stdin*\\\x00")).as_mut_ptr())
               &&
               0 != UTIL_getFileStat(srcFileName, &mut statbuf as *mut stat_t)
           {
            stat_result = 1i32
        }
        result =
            FIO_compressFilename_srcFile(ress, dstFileName, srcFileName,
                                         compressionLevel);
        clearHandler();
        if 0 != fclose(ress.dstFile) {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 15],
                                                  &mut [libc::c_char; 15]>(b"zstd: %s: %s \n\x00")).as_mut_ptr(),
                        dstFileName, strerror(*__errno_location()));
            }
            result = 1i32
        }
        if result != 0i32 &&
               0 !=
                   strcmp(dstFileName,
                          (*::std::mem::transmute::<&[u8; 10],
                                                    &mut [libc::c_char; 10]>(b"/dev/null\x00")).as_mut_ptr())
               &&
               0 !=
                   strcmp(dstFileName,
                          (*::std::mem::transmute::<&[u8; 11],
                                                    &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr())
           {
            FIO_remove(dstFileName);
        } else if 0 !=
                      strcmp(dstFileName,
                             (*::std::mem::transmute::<&[u8; 11],
                                                       &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr())
                      &&
                      0 !=
                          strcmp(dstFileName,
                                 (*::std::mem::transmute::<&[u8; 10],
                                                           &mut [libc::c_char; 10]>(b"/dev/null\x00")).as_mut_ptr())
                      && 0 != stat_result {
            UTIL_setFileStat(dstFileName, &mut statbuf as *mut stat_t);
        }
        return result
    };
}
/* * FIO_remove() :
 * @result : Unlink `fileName`, even if it's read-only */
unsafe extern "C" fn FIO_remove(mut path: *const libc::c_char)
 -> libc::c_int {
    if 0 == UTIL_isRegularFile(path) {
        if g_displayLevel >= 2i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 46],
                                              &mut [libc::c_char; 46]>(b"zstd: Refusing to remove non-regular file %s\n\x00")).as_mut_ptr(),
                    path);
        }
        return 0i32
    } else { return remove(path) };
}
unsafe extern "C" fn clearHandler() -> () {
    if !g_artefact.is_null() { signal(2i32, None); }
    g_artefact = 0 as *const libc::c_char;
}
static mut g_artefact: *const libc::c_char =
    unsafe { 0 as *const libc::c_char };
/* ! FIO_compressFilename_srcFile() :
 *  note : ress.destFile already opened
 *  @return : 0 : compression completed correctly,
 *            1 : missing or pb opening srcFileName
 */
unsafe extern "C" fn FIO_compressFilename_srcFile(mut ress: cRess_t,
                                                  mut dstFileName:
                                                      *const libc::c_char,
                                                  mut srcFileName:
                                                      *const libc::c_char,
                                                  mut compressionLevel:
                                                      libc::c_int)
 -> libc::c_int {
    let mut result: libc::c_int = 0;
    if 0 != UTIL_isDirectory(srcFileName) {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 37],
                                              &mut [libc::c_char; 37]>(b"zstd: %s is a directory -- ignored \n\x00")).as_mut_ptr(),
                    srcFileName);
        }
        return 1i32
    } else {
        ress.srcFile = FIO_openSrcFile(srcFileName);
        if ress.srcFile.is_null() {
            return 1i32
        } else {
            result =
                FIO_compressFilename_internal(ress, dstFileName, srcFileName,
                                              compressionLevel);
            fclose(ress.srcFile);
            if 0 != g_removeSrcFile && 0 == result &&
                   0 !=
                       strcmp(srcFileName,
                              (*::std::mem::transmute::<&[u8; 10],
                                                        &mut [libc::c_char; 10]>(b"/*stdin*\\\x00")).as_mut_ptr())
               {
                clearHandler();
                if 0 != FIO_remove(srcFileName) {
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 7],
                                                          &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                1i32);
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 13],
                                                          &mut [libc::c_char; 13]>(b"zstd: %s: %s\x00")).as_mut_ptr(),
                                srcFileName, strerror(*__errno_location()));
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                    }
                    exit(1i32);
                }
            }
            return result
        }
    };
}
/* ! FIO_compressFilename_internal() :
 *  same as FIO_compressFilename_extRess(), with `ress.desFile` already opened.
 *  @return : 0 : compression completed correctly,
 *            1 : missing or pb opening srcFileName
 */
unsafe extern "C" fn FIO_compressFilename_internal(mut ress: cRess_t,
                                                   mut dstFileName:
                                                       *const libc::c_char,
                                                   mut srcFileName:
                                                       *const libc::c_char,
                                                   mut compressionLevel:
                                                       libc::c_int)
 -> libc::c_int {
    let mut readsize: U64 = 0i32 as U64;
    let mut compressedfilesize: U64 = 0i32 as U64;
    let fileSize: U64 = UTIL_getFileSize(srcFileName);
    if g_displayLevel >= 5i32 {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 15],
                                          &mut [libc::c_char; 15]>(b"%s: %u bytes \n\x00")).as_mut_ptr(),
                srcFileName, fileSize as U32);
    }
    match g_compressionType as libc::c_uint {
        1 => {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 7],
                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 12],
                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                        20i32);
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 97],
                                                  &mut [libc::c_char; 97]>(b"zstd: %s: file cannot be compressed as gzip (zstd compiled without ZSTD_GZCOMPRESS) -- ignored \n\x00")).as_mut_ptr(),
                        srcFileName);
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 3],
                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
            }
            exit(20i32);
        }
        2 | 3 => {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 7],
                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 12],
                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                        20i32);
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 102],
                                                  &mut [libc::c_char; 102]>(b"zstd: %s: file cannot be compressed as xz/lzma (zstd compiled without ZSTD_LZMACOMPRESS) -- ignored \n\x00")).as_mut_ptr(),
                        srcFileName);
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 3],
                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
            }
            exit(20i32);
        }
        4 => {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 7],
                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 12],
                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                        20i32);
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 97],
                                                  &mut [libc::c_char; 97]>(b"zstd: %s: file cannot be compressed as lz4 (zstd compiled without ZSTD_LZ4COMPRESS) -- ignored \n\x00")).as_mut_ptr(),
                        srcFileName);
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 3],
                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
            }
            exit(20i32);
        }
        0 | _ => {
            compressedfilesize =
                FIO_compressZstdFrame(&mut ress as *mut cRess_t, srcFileName,
                                      fileSize, compressionLevel,
                                      &mut readsize as *mut U64) as U64;
            if g_displayLevel >= 2i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 7],
                                                  &mut [libc::c_char; 7]>(b"\r%79s\r\x00")).as_mut_ptr(),
                        (*::std::mem::transmute::<&[u8; 1],
                                                  &mut [libc::c_char; 1]>(b"\x00")).as_mut_ptr());
            }
            if g_displayLevel >= 2i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 46],
                                                  &mut [libc::c_char; 46]>(b"%-20s :%6.2f%%   (%6llu => %6llu bytes, %s) \n\x00")).as_mut_ptr(),
                        srcFileName,
                        compressedfilesize as libc::c_double /
                            readsize.wrapping_add((0 == readsize) as
                                                      libc::c_int as
                                                      libc::c_ulong) as
                                libc::c_double * 100i32 as libc::c_double,
                        readsize as libc::c_ulonglong,
                        compressedfilesize as libc::c_ulonglong, dstFileName);
            }
            return 0i32
        }
    };
}
/* ! FIO_compressFilename_internal() :
 *  same as FIO_compressFilename_extRess(), with `ress.desFile` already opened.
 *  @return : 0 : compression completed correctly,
 *            1 : missing or pb opening srcFileName
 */
unsafe extern "C" fn FIO_compressZstdFrame(mut ressPtr: *const cRess_t,
                                           mut srcFileName:
                                               *const libc::c_char,
                                           mut fileSize: U64,
                                           mut compressionLevel: libc::c_int,
                                           mut readsize: *mut U64)
 -> libc::c_ulonglong {
    let ress: cRess_t = *ressPtr;
    let srcFile: *mut FILE = ress.srcFile;
    let dstFile: *mut FILE = ress.dstFile;
    let mut compressedfilesize: U64 = 0i32 as U64;
    let mut directive: ZSTD_EndDirective = ZSTD_e_continue;
    if g_displayLevel >= 6i32 {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 32],
                                          &mut [libc::c_char; 32]>(b"compression using zstd format \n\x00")).as_mut_ptr());
    }
    if fileSize != -1i32 as U64 {
        let mut err: size_t = 0;
        err =
            ZSTD_CCtx_setPledgedSrcSize(ress.cctx,
                                        fileSize as libc::c_ulonglong);
        if 0 != ZSTD_isError(err) {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 7],
                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 12],
                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                        11i32);
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 3],
                                                  &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                        ZSTD_getErrorName(err));
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 3],
                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
            }
            exit(11i32);
        }
    }
    loop  {
        let mut result: size_t = 0;
        let inSize: size_t =
            fread(ress.srcBuffer, 1i32 as size_t, ress.srcBufferSize,
                  srcFile);
        let mut inBuff: ZSTD_inBuffer =
            ZSTD_inBuffer_s{src: ress.srcBuffer,
                            size: inSize,
                            pos: 0i32 as size_t,};
        if g_displayLevel >= 6i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 29],
                                              &mut [libc::c_char; 29]>(b"fread %u bytes from source \n\x00")).as_mut_ptr(),
                    inSize as U32);
        }
        *readsize =
            (*readsize as libc::c_ulong).wrapping_add(inSize) as U64 as U64;
        if inSize == 0i32 as libc::c_ulong || *readsize == fileSize {
            directive = ZSTD_e_end
        }
        result = 1i32 as size_t;
        while inBuff.pos != inBuff.size ||
                  directive as libc::c_uint ==
                      ZSTD_e_end as libc::c_int as libc::c_uint &&
                      result != 0i32 as libc::c_ulong {
            let mut outBuff: ZSTD_outBuffer =
                ZSTD_outBuffer_s{dst: ress.dstBuffer,
                                 size: ress.dstBufferSize,
                                 pos: 0i32 as size_t,};
            result =
                ZSTD_compress_generic(ress.cctx,
                                      &mut outBuff as *mut ZSTD_outBuffer,
                                      &mut inBuff as *mut ZSTD_inBuffer,
                                      directive);
            if 0 != ZSTD_isError(result) {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 7],
                                                      &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 12],
                                                      &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                            11i32);
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 3],
                                                      &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                            ZSTD_getErrorName(result));
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 3],
                                                      &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                }
                exit(11i32);
            } else {
                if g_displayLevel >= 6i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 88],
                                                      &mut [libc::c_char; 88]>(b"ZSTD_compress_generic(end:%u) => intput pos(%u)<=(%u)size ; output generated %u bytes \n\x00")).as_mut_ptr(),
                            directive as U32, inBuff.pos as U32,
                            inBuff.size as U32, outBuff.pos as U32);
                }
                if 0 != outBuff.pos {
                    let sizeCheck: size_t =
                        fwrite(ress.dstBuffer, 1i32 as size_t, outBuff.pos,
                               dstFile);
                    if sizeCheck != outBuff.pos {
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 7],
                                                              &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 12],
                                                              &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                    25i32);
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 44],
                                                              &mut [libc::c_char; 44]>(b"Write error : cannot write compressed block\x00")).as_mut_ptr());
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 3],
                                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                        }
                        exit(25i32);
                    } else {
                        compressedfilesize =
                            (compressedfilesize as
                                 libc::c_ulong).wrapping_add(outBuff.pos) as
                                U64 as U64
                    }
                }
                if !(UTIL_clockSpanMicro(g_displayClock) > g_refreshRate) {
                    continue ;
                }
                let zfp: ZSTD_frameProgression =
                    ZSTD_getFrameProgression(ress.cctx);
                let cShare: libc::c_double =
                    zfp.produced as libc::c_double /
                        zfp.consumed.wrapping_add((0 == zfp.consumed) as
                                                      libc::c_int as
                                                      libc::c_ulonglong) as
                            libc::c_double * 100i32 as libc::c_double;
                if g_displayLevel >= 3i32 {
                    if !(g_displayLevel >= 3i32) { continue ; }
                    if !(UTIL_clockSpanMicro(g_displayClock) > g_refreshRate
                             || g_displayLevel >= 4i32) {
                        continue ;
                    }
                    g_displayClock = UTIL_getTime();
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 74],
                                                      &mut [libc::c_char; 74]>(b"\r(L%i) Buffered :%4u MB - Consumed :%4u MB - Compressed :%4u MB => %.2f%%\x00")).as_mut_ptr(),
                            compressionLevel,
                            (zfp.ingested.wrapping_sub(zfp.consumed) >> 20i32)
                                as U32, (zfp.consumed >> 20i32) as U32,
                            (zfp.produced >> 20i32) as U32, cShare);
                    if !(g_displayLevel >= 4i32) { continue ; }
                    fflush(stderr);
                } else {
                    if g_displayLevel >= 2i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &mut [libc::c_char; 12]>(b"\rRead : %u \x00")).as_mut_ptr(),
                                (zfp.consumed >> 20i32) as U32);
                    }
                    if fileSize != -1i32 as U64 {
                        if g_displayLevel >= 2i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 6],
                                                              &mut [libc::c_char; 6]>(b"/ %u \x00")).as_mut_ptr(),
                                    (fileSize >> 20i32) as U32);
                        }
                    }
                    if g_displayLevel >= 2i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 15],
                                                          &mut [libc::c_char; 15]>(b"MB ==> %2.f%% \x00")).as_mut_ptr(),
                                cShare);
                    }
                    g_displayClock = UTIL_getTime()
                }
            }
        }
        if !(directive as libc::c_uint !=
                 ZSTD_e_end as libc::c_int as libc::c_uint) {
            break ;
        }
    }
    return compressedfilesize as libc::c_ulonglong;
}
static mut g_displayClock: UTIL_time_t =
    unsafe {
        timespec{tv_sec: 0i32 as __time_t,
                 tv_nsec: 0i32 as __syscall_slong_t,}
    };
static mut g_refreshRate: U64 = unsafe { (1000000i32 / 6i32) as U64 };
/* * FIO_openSrcFile() :
 *  condition : `srcFileName` must be non-NULL.
 * @result : FILE* to `srcFileName`, or NULL if it fails */
unsafe extern "C" fn FIO_openSrcFile(mut srcFileName: *const libc::c_char)
 -> *mut FILE {
    if 0 ==
           strcmp(srcFileName,
                  (*::std::mem::transmute::<&[u8; 10],
                                            &mut [libc::c_char; 10]>(b"/*stdin*\\\x00")).as_mut_ptr())
       {
        if g_displayLevel >= 4i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 23],
                                              &mut [libc::c_char; 23]>(b"Using stdin for input\n\x00")).as_mut_ptr());
        }
        return stdin
    } else if 0 == UTIL_isRegularFile(srcFileName) {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 44],
                                              &mut [libc::c_char; 44]>(b"zstd: %s is not a regular file -- ignored \n\x00")).as_mut_ptr(),
                    srcFileName);
        }
        return 0 as *mut FILE
    } else {
        let f: *mut FILE =
            fopen(srcFileName,
                  (*::std::mem::transmute::<&[u8; 3],
                                            &mut [libc::c_char; 3]>(b"rb\x00")).as_mut_ptr());
        if f.is_null() {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 15],
                                                  &mut [libc::c_char; 15]>(b"zstd: %s: %s \n\x00")).as_mut_ptr(),
                        srcFileName, strerror(*__errno_location()));
            }
        }
        return f
    };
}
unsafe extern "C" fn addHandler(mut dstFileName: *const libc::c_char) -> () {
    if 0 != UTIL_isRegularFile(dstFileName) {
        g_artefact = dstFileName;
        signal(2i32, Some(INThandler));
    } else { g_artefact = 0 as *const libc::c_char };
}
unsafe extern "C" fn INThandler(mut sig: libc::c_int) -> () {
    signal(sig,
           ::std::mem::transmute::<libc::intptr_t,
                                   __sighandler_t>(1i32 as libc::intptr_t));
    if !g_artefact.is_null() { remove(g_artefact); }
    fprintf(stderr,
            (*::std::mem::transmute::<&[u8; 2],
                                      &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
    exit(2i32);
}
/* * FIO_openDstFile() :
 *  condition : `dstFileName` must be non-NULL.
 * @result : FILE* to `dstFileName`, or NULL if it fails */
unsafe extern "C" fn FIO_openDstFile(mut dstFileName: *const libc::c_char)
 -> *mut FILE {
    if 0 ==
           strcmp(dstFileName,
                  (*::std::mem::transmute::<&[u8; 11],
                                            &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr())
       {
        if g_displayLevel >= 4i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 25],
                                              &mut [libc::c_char; 25]>(b"Using stdout for output\n\x00")).as_mut_ptr());
        }
        if g_sparseFileSupport == 1i32 as libc::c_uint {
            g_sparseFileSupport = 0i32 as U32;
            if g_displayLevel >= 4i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 73],
                                                  &mut [libc::c_char; 73]>(b"Sparse File Support is automatically disabled on stdout ; try --sparse \n\x00")).as_mut_ptr());
            }
        }
        return stdout
    } else {
        if g_sparseFileSupport == 1i32 as libc::c_uint {
            g_sparseFileSupport = 1i32 as U32
        }
        if 0 != UTIL_isRegularFile(dstFileName) {
            let mut fCheck: *mut FILE = 0 as *mut FILE;
            if 0 ==
                   strcmp(dstFileName,
                          (*::std::mem::transmute::<&[u8; 10],
                                                    &mut [libc::c_char; 10]>(b"/dev/null\x00")).as_mut_ptr())
               {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 7],
                                                      &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 12],
                                                      &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                            40i32);
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 34],
                                                      &mut [libc::c_char; 34]>(b"%s is unexpectedly a regular file\x00")).as_mut_ptr(),
                            dstFileName);
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 3],
                                                      &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                }
                exit(40i32);
            } else {
                fCheck =
                    fopen(dstFileName,
                          (*::std::mem::transmute::<&[u8; 3],
                                                    &mut [libc::c_char; 3]>(b"rb\x00")).as_mut_ptr());
                if !fCheck.is_null() {
                    fclose(fCheck);
                    if 0 == g_overwrite {
                        if g_displayLevel <= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 44],
                                                              &mut [libc::c_char; 44]>(b"zstd: %s already exists; not overwritten  \n\x00")).as_mut_ptr(),
                                    dstFileName);
                            return 0 as *mut FILE
                        } else {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 44],
                                                              &mut [libc::c_char; 44]>(b"zstd: %s already exists; overwrite (y/N) ? \x00")).as_mut_ptr(),
                                    dstFileName);
                            let mut ch: libc::c_int = getchar();
                            if ch != 'Y' as i32 && ch != 'y' as i32 {
                                fprintf(stderr,
                                        (*::std::mem::transmute::<&[u8; 23],
                                                                  &mut [libc::c_char; 23]>(b"    not overwritten  \n\x00")).as_mut_ptr());
                                return 0 as *mut FILE
                            } else {
                                while ch != -1i32 && ch != '\n' as i32 {
                                    ch = getchar()
                                }
                            }
                        }
                    }
                    FIO_remove(dstFileName);
                }
            }
        }
        let f: *mut FILE =
            fopen(dstFileName,
                  (*::std::mem::transmute::<&[u8; 3],
                                            &mut [libc::c_char; 3]>(b"wb\x00")).as_mut_ptr());
        if f.is_null() {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 14],
                                                  &mut [libc::c_char; 14]>(b"zstd: %s: %s\n\x00")).as_mut_ptr(),
                        dstFileName, strerror(*__errno_location()));
            }
        }
        return f
    };
}
unsafe extern "C" fn FIO_freeCResources(mut ress: cRess_t) -> () {
    free(ress.srcBuffer);
    free(ress.dstBuffer);
    ZSTD_freeCStream(ress.cctx);
}
/* * FIO_decompressFilename() :
    @return : 0 == ok;  1 == pb with src file. */
#[no_mangle]
pub unsafe extern "C" fn FIO_decompressFilename(mut dstFileName:
                                                    *const libc::c_char,
                                                mut srcFileName:
                                                    *const libc::c_char,
                                                mut dictFileName:
                                                    *const libc::c_char)
 -> libc::c_int {
    let ress: dRess_t = FIO_createDResources(dictFileName);
    let decodingError: libc::c_int =
        FIO_decompressDstFile(ress, dstFileName, srcFileName);
    FIO_freeDResources(ress);
    return decodingError;
}
unsafe extern "C" fn FIO_createDResources(mut dictFileName:
                                              *const libc::c_char)
 -> dRess_t {
    let mut ress: dRess_t =
        dRess_t{srcBuffer: 0 as *mut libc::c_void,
                srcBufferSize: 0,
                srcBufferLoaded: 0,
                dstBuffer: 0 as *mut libc::c_void,
                dstBufferSize: 0,
                dctx: 0 as *mut ZSTD_DStream,
                dstFile: 0 as *mut FILE,};
    memset(&mut ress as *mut dRess_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<dRess_t>() as libc::c_ulong);
    ress.dctx = ZSTD_createDStream();
    if ress.dctx.is_null() {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 7],
                                              &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                    60i32);
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 26],
                                              &mut [libc::c_char; 26]>(b"Can\'t create ZSTD_DStream\x00")).as_mut_ptr());
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 3],
                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
        }
        exit(60i32);
    } else {
        let mut err: size_t = 0;
        err =
            ZSTD_setDStreamParameter(ress.dctx, DStream_p_maxWindowSize,
                                     g_memLimit);
        if 0 != ZSTD_isError(err) {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 7],
                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 12],
                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                        11i32);
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 3],
                                                  &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                        ZSTD_getErrorName(err));
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 3],
                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
            }
            exit(11i32);
        } else {
            ress.srcBufferSize = ZSTD_DStreamInSize();
            ress.srcBuffer = malloc(ress.srcBufferSize);
            ress.dstBufferSize = ZSTD_DStreamOutSize();
            ress.dstBuffer = malloc(ress.dstBufferSize);
            if ress.srcBuffer.is_null() || ress.dstBuffer.is_null() {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 7],
                                                      &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 12],
                                                      &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                            61i32);
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 37],
                                                      &mut [libc::c_char; 37]>(b"Allocation error : not enough memory\x00")).as_mut_ptr());
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 3],
                                                      &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                }
                exit(61i32);
            } else {
                let mut dictBuffer: *mut libc::c_void =
                    0 as *mut libc::c_void;
                let dictBufferSize: size_t =
                    FIO_createDictBuffer(&mut dictBuffer as
                                             *mut *mut libc::c_void,
                                         dictFileName);
                let mut err_0: size_t = 0;
                err_0 =
                    ZSTD_initDStream_usingDict(ress.dctx, dictBuffer,
                                               dictBufferSize);
                if 0 != ZSTD_isError(err_0) {
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 7],
                                                          &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                11i32);
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                                ZSTD_getErrorName(err_0));
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                    }
                    exit(11i32);
                } else { free(dictBuffer); return ress }
            }
        }
    };
}
/* * FIO_decompressFile_extRess() :
    decompress `srcFileName` into `dstFileName`
    @return : 0 : OK
              1 : operation aborted (src not available, dst already taken, etc.)
*/
unsafe extern "C" fn FIO_decompressDstFile(mut ress: dRess_t,
                                           mut dstFileName:
                                               *const libc::c_char,
                                           mut srcFileName:
                                               *const libc::c_char)
 -> libc::c_int {
    let mut result: libc::c_int = 0;
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
    let mut stat_result: libc::c_int = 0i32;
    ress.dstFile = FIO_openDstFile(dstFileName);
    if ress.dstFile.is_null() {
        return 1i32
    } else {
        addHandler(dstFileName);
        if 0 !=
               strcmp(srcFileName,
                      (*::std::mem::transmute::<&[u8; 10],
                                                &mut [libc::c_char; 10]>(b"/*stdin*\\\x00")).as_mut_ptr())
               &&
               0 != UTIL_getFileStat(srcFileName, &mut statbuf as *mut stat_t)
           {
            stat_result = 1i32
        }
        result = FIO_decompressSrcFile(ress, dstFileName, srcFileName);
        clearHandler();
        if 0 != fclose(ress.dstFile) {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 15],
                                                  &mut [libc::c_char; 15]>(b"zstd: %s: %s \n\x00")).as_mut_ptr(),
                        dstFileName, strerror(*__errno_location()));
            }
            result = 1i32
        }
        if result != 0i32 &&
               0 !=
                   strcmp(dstFileName,
                          (*::std::mem::transmute::<&[u8; 10],
                                                    &mut [libc::c_char; 10]>(b"/dev/null\x00")).as_mut_ptr())
               &&
               0 !=
                   strcmp(dstFileName,
                          (*::std::mem::transmute::<&[u8; 11],
                                                    &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr())
           {
            FIO_remove(dstFileName);
        } else if 0 !=
                      strcmp(dstFileName,
                             (*::std::mem::transmute::<&[u8; 11],
                                                       &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr())
                      &&
                      0 !=
                          strcmp(dstFileName,
                                 (*::std::mem::transmute::<&[u8; 10],
                                                           &mut [libc::c_char; 10]>(b"/dev/null\x00")).as_mut_ptr())
                      && 0 != stat_result {
            UTIL_setFileStat(dstFileName, &mut statbuf as *mut stat_t);
        }
        signal(2i32, None);
        return result
    };
}
/* * FIO_decompressSrcFile() :
    Decompression `srcFileName` into `ress.dstFile`
    @return : 0 : OK
              1 : operation not started
*/
unsafe extern "C" fn FIO_decompressSrcFile(mut ress: dRess_t,
                                           mut dstFileName:
                                               *const libc::c_char,
                                           mut srcFileName:
                                               *const libc::c_char)
 -> libc::c_int {
    let mut srcFile: *mut FILE = 0 as *mut FILE;
    let mut result: libc::c_int = 0;
    if 0 != UTIL_isDirectory(srcFileName) {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 37],
                                              &mut [libc::c_char; 37]>(b"zstd: %s is a directory -- ignored \n\x00")).as_mut_ptr(),
                    srcFileName);
        }
        return 1i32
    } else {
        srcFile = FIO_openSrcFile(srcFileName);
        if srcFile.is_null() {
            return 1i32
        } else {
            result =
                FIO_decompressFrames(ress, srcFile, dstFileName, srcFileName);
            if 0 != fclose(srcFile) {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 15],
                                                      &mut [libc::c_char; 15]>(b"zstd: %s: %s \n\x00")).as_mut_ptr(),
                            srcFileName, strerror(*__errno_location()));
                }
                return 1i32
            } else {
                if 0 != g_removeSrcFile && result == 0i32 &&
                       0 !=
                           strcmp(srcFileName,
                                  (*::std::mem::transmute::<&[u8; 10],
                                                            &mut [libc::c_char; 10]>(b"/*stdin*\\\x00")).as_mut_ptr())
                   {
                    clearHandler();
                    if 0 != FIO_remove(srcFileName) {
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 15],
                                                              &mut [libc::c_char; 15]>(b"zstd: %s: %s \n\x00")).as_mut_ptr(),
                                    srcFileName,
                                    strerror(*__errno_location()));
                        }
                        return 1i32
                    }
                }
                return result
            }
        }
    };
}
/* * FIO_decompressFrames() :
 *  Find and decode frames inside srcFile
 *  srcFile presumed opened and valid
 * @return : 0 : OK
 *           1 : error
 */
unsafe extern "C" fn FIO_decompressFrames(mut ress: dRess_t,
                                          mut srcFile: *mut FILE,
                                          mut dstFileName:
                                              *const libc::c_char,
                                          mut srcFileName:
                                              *const libc::c_char)
 -> libc::c_int {
    let mut readSomething: libc::c_uint = 0i32 as libc::c_uint;
    let mut filesize: libc::c_ulonglong = 0i32 as libc::c_ulonglong;
    loop  {
        let toRead: size_t = 4i32 as size_t;
        let buf: *const BYTE = ress.srcBuffer as *const BYTE;
        if ress.srcBufferLoaded < toRead {
            ress.srcBufferLoaded =
                (ress.srcBufferLoaded as
                     libc::c_ulong).wrapping_add(fread((ress.srcBuffer as
                                                            *mut libc::c_char).offset(ress.srcBufferLoaded
                                                                                          as
                                                                                          isize)
                                                           as
                                                           *mut libc::c_void,
                                                       1i32 as size_t,
                                                       toRead.wrapping_sub(ress.srcBufferLoaded),
                                                       srcFile)) as size_t as
                    size_t
        }
        if ress.srcBufferLoaded == 0i32 as libc::c_ulong {
            if !(readSomething == 0i32 as libc::c_uint) { break ; }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 35],
                                                  &mut [libc::c_char; 35]>(b"zstd: %s: unexpected end of file \n\x00")).as_mut_ptr(),
                        srcFileName);
            }
            return 1i32
        } else {
            readSomething = 1i32 as libc::c_uint;
            if ress.srcBufferLoaded < toRead {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 27],
                                                      &mut [libc::c_char; 27]>(b"zstd: %s: unknown header \n\x00")).as_mut_ptr(),
                            srcFileName);
                }
                return 1i32
            } else if 0 !=
                          ZSTD_isFrame(buf as *const libc::c_void,
                                       ress.srcBufferLoaded) {
                let frameSize: libc::c_ulonglong =
                    FIO_decompressZstdFrame(&mut ress as *mut dRess_t,
                                            srcFile, srcFileName,
                                            filesize as U64);
                if frameSize == -2i32 as libc::c_ulonglong {
                    return 1i32
                } else { filesize = filesize.wrapping_add(frameSize) }
            } else if *buf.offset(0isize) as libc::c_int == 31i32 &&
                          *buf.offset(1isize) as libc::c_int == 139i32 {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 90],
                                                      &mut [libc::c_char; 90]>(b"zstd: %s: gzip file cannot be uncompressed (zstd compiled without HAVE_ZLIB) -- ignored \n\x00")).as_mut_ptr(),
                            srcFileName);
                }
                return 1i32
            } else if *buf.offset(0isize) as libc::c_int == 253i32 &&
                          *buf.offset(1isize) as libc::c_int == 55i32 ||
                          *buf.offset(0isize) as libc::c_int == 93i32 &&
                              *buf.offset(1isize) as libc::c_int == 0i32 {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 93],
                                                      &mut [libc::c_char; 93]>(b"zstd: %s: xz/lzma file cannot be uncompressed (zstd compiled without HAVE_LZMA) -- ignored \n\x00")).as_mut_ptr(),
                            srcFileName);
                }
                return 1i32
            } else if MEM_readLE32(buf as *const libc::c_void) ==
                          407708164i32 as libc::c_uint {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 88],
                                                      &mut [libc::c_char; 88]>(b"zstd: %s: lz4 file cannot be uncompressed (zstd compiled without HAVE_LZ4) -- ignored \n\x00")).as_mut_ptr(),
                            srcFileName);
                }
                return 1i32
            } else if 0 != g_overwrite &&
                          0 ==
                              strcmp(dstFileName,
                                     (*::std::mem::transmute::<&[u8; 11],
                                                               &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr())
             {
                return FIO_passThrough(ress.dstFile, srcFile, ress.srcBuffer,
                                       ress.srcBufferSize,
                                       ress.srcBufferLoaded) as libc::c_int
            } else {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 31],
                                                      &mut [libc::c_char; 31]>(b"zstd: %s: unsupported format \n\x00")).as_mut_ptr(),
                            srcFileName);
                }
                return 1i32
            }
        }
    }
    if g_displayLevel >= 2i32 {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 7],
                                          &mut [libc::c_char; 7]>(b"\r%79s\r\x00")).as_mut_ptr(),
                (*::std::mem::transmute::<&[u8; 1],
                                          &mut [libc::c_char; 1]>(b"\x00")).as_mut_ptr());
    }
    if g_displayLevel >= 2i32 {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 20],
                                          &mut [libc::c_char; 20]>(b"%-20s: %llu bytes \n\x00")).as_mut_ptr(),
                srcFileName, filesize);
    }
    return 0i32;
}
/* * FIO_passThrough() : just copy input into output, for compatibility with gzip -df mode
    @return : 0 (no error) */
unsafe extern "C" fn FIO_passThrough(mut foutput: *mut FILE,
                                     mut finput: *mut FILE,
                                     mut buffer: *mut libc::c_void,
                                     mut bufferSize: size_t,
                                     mut alreadyLoaded: size_t)
 -> libc::c_uint {
    let blockSize: size_t =
        if ((64i32 * (1i32 << 10i32)) as libc::c_ulong) < bufferSize {
            (64i32 * (1i32 << 10i32)) as libc::c_ulong
        } else { bufferSize };
    let mut readFromInput: size_t = 1i32 as size_t;
    let mut storedSkips: libc::c_uint = 0i32 as libc::c_uint;
    let sizeCheck: size_t =
        fwrite(buffer, 1i32 as size_t, alreadyLoaded, foutput);
    if sizeCheck != alreadyLoaded {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 27],
                                              &mut [libc::c_char; 27]>(b"Pass-through write error \n\x00")).as_mut_ptr());
        }
        return 1i32 as libc::c_uint
    } else {
        while 0 != readFromInput {
            readFromInput = fread(buffer, 1i32 as size_t, blockSize, finput);
            storedSkips =
                FIO_fwriteSparse(foutput, buffer, readFromInput, storedSkips)
        }
        FIO_fwriteSparseEnd(foutput, storedSkips);
        return 0i32 as libc::c_uint
    };
}
unsafe extern "C" fn FIO_fwriteSparseEnd(mut file: *mut FILE,
                                         mut storedSkips: libc::c_uint)
 -> () {
    let fresh1 = storedSkips;
    storedSkips = storedSkips.wrapping_sub(1);
    if fresh1 > 0i32 as libc::c_uint {
        let seekResult: libc::c_int =
            fseek(file, storedSkips as libc::c_long, 1i32);
        if seekResult != 0i32 {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 7],
                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 12],
                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                        69i32);
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 31],
                                                  &mut [libc::c_char; 31]>(b"Final skip error (sparse file)\x00")).as_mut_ptr());
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 3],
                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
            }
            exit(69i32);
        } else {
            let lastZeroByte: [libc::c_char; 1] = [0i32 as libc::c_char];
            let sizeCheck: size_t =
                fwrite(lastZeroByte.as_ptr() as *const libc::c_void,
                       1i32 as size_t, 1i32 as size_t, file);
            if sizeCheck != 1i32 as libc::c_ulong {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 7],
                                                      &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 12],
                                                      &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                            69i32);
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 37],
                                                      &mut [libc::c_char; 37]>(b"Write error : cannot write last zero\x00")).as_mut_ptr());
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 3],
                                                      &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                }
                exit(69i32);
            }
        }
    };
}
/* * FIO_fwriteSparse() :
*   @return : storedSkips, to be provided to next call to FIO_fwriteSparse() of LZ4IO_fwriteSparseEnd() */
unsafe extern "C" fn FIO_fwriteSparse(mut file: *mut FILE,
                                      mut buffer: *const libc::c_void,
                                      mut bufferSize: size_t,
                                      mut storedSkips: libc::c_uint)
 -> libc::c_uint {
    let bufferT: *const size_t = buffer as *const size_t;
    let mut bufferSizeT: size_t =
        bufferSize.wrapping_div(::std::mem::size_of::<size_t>() as
                                    libc::c_ulong);
    let bufferTEnd: *const size_t = bufferT.offset(bufferSizeT as isize);
    let mut ptrT: *const size_t = bufferT;
    static mut segmentSizeT: size_t = 0;
    if 0 == g_sparseFileSupport {
        let sizeCheck: size_t =
            fwrite(buffer, 1i32 as size_t, bufferSize, file);
        if sizeCheck != bufferSize {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 7],
                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 12],
                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                        70i32);
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 41],
                                                  &mut [libc::c_char; 41]>(b"Write error : cannot write decoded block\x00")).as_mut_ptr());
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 3],
                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
            }
            exit(70i32);
        } else { return 0i32 as libc::c_uint }
    } else {
        if storedSkips > (1i32 as libc::c_uint).wrapping_mul(1u32 << 30i32) {
            let seekResult: libc::c_int =
                fseek(file,
                      (1i32 as libc::c_uint).wrapping_mul(1u32 << 30i32) as
                          libc::c_long, 1i32);
            if seekResult != 0i32 {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 7],
                                                      &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 12],
                                                      &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                            71i32);
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 38],
                                                      &mut [libc::c_char; 38]>(b"1 GB skip error (sparse file support)\x00")).as_mut_ptr());
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 3],
                                                      &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                }
                exit(71i32);
            } else {
                storedSkips =
                    storedSkips.wrapping_sub((1i32 as
                                                  libc::c_uint).wrapping_mul(1u32
                                                                                 <<
                                                                                 30i32))
            }
        }
        while ptrT < bufferTEnd {
            let mut seg0SizeT: size_t = segmentSizeT;
            let mut nb0T: size_t = 0;
            if seg0SizeT > bufferSizeT { seg0SizeT = bufferSizeT }
            bufferSizeT =
                (bufferSizeT as libc::c_ulong).wrapping_sub(seg0SizeT) as
                    size_t as size_t;
            nb0T = 0i32 as size_t;
            while nb0T < seg0SizeT &&
                      *ptrT.offset(nb0T as isize) == 0i32 as libc::c_ulong {
                nb0T = nb0T.wrapping_add(1)
            }
            storedSkips =
                storedSkips.wrapping_add(nb0T.wrapping_mul(::std::mem::size_of::<size_t>()
                                                               as
                                                               libc::c_ulong)
                                             as libc::c_uint);
            if nb0T != seg0SizeT {
                let seekResult_0: libc::c_int =
                    fseek(file, storedSkips as libc::c_long, 1i32);
                if 0 != seekResult_0 {
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 7],
                                                          &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                72i32);
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 36],
                                                          &mut [libc::c_char; 36]>(b"Sparse skip error ; try --no-sparse\x00")).as_mut_ptr());
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                    }
                    exit(72i32);
                } else {
                    storedSkips = 0i32 as libc::c_uint;
                    seg0SizeT =
                        (seg0SizeT as libc::c_ulong).wrapping_sub(nb0T) as
                            size_t as size_t;
                    ptrT = ptrT.offset(nb0T as isize);
                    let sizeCheck_0: size_t =
                        fwrite(ptrT as *const libc::c_void,
                               ::std::mem::size_of::<size_t>() as
                                   libc::c_ulong, seg0SizeT, file);
                    if sizeCheck_0 != seg0SizeT {
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 7],
                                                              &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 12],
                                                              &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                    73i32);
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 41],
                                                              &mut [libc::c_char; 41]>(b"Write error : cannot write decoded block\x00")).as_mut_ptr());
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 3],
                                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                        }
                        exit(73i32);
                    }
                }
            }
            ptrT = ptrT.offset(seg0SizeT as isize)
        }
        static mut maskT: size_t = 0;
        if 0 != bufferSize & maskT {
            let restStart: *const libc::c_char =
                bufferTEnd as *const libc::c_char;
            let mut restPtr: *const libc::c_char = restStart;
            let mut restSize: size_t = bufferSize & maskT;
            let restEnd: *const libc::c_char =
                restStart.offset(restSize as isize);
            while restPtr < restEnd && *restPtr as libc::c_int == 0i32 {
                restPtr = restPtr.offset(1isize)
            }
            storedSkips =
                storedSkips.wrapping_add(restStart.offset_to(restPtr).expect("bad offset_to")
                                             as libc::c_long as libc::c_uint);
            if restPtr != restEnd {
                let mut seekResult_1: libc::c_int =
                    fseek(file, storedSkips as libc::c_long, 1i32);
                if 0 != seekResult_1 {
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 7],
                                                          &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                74i32);
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 36],
                                                          &mut [libc::c_char; 36]>(b"Sparse skip error ; try --no-sparse\x00")).as_mut_ptr());
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                    }
                    exit(74i32);
                } else {
                    storedSkips = 0i32 as libc::c_uint;
                    let sizeCheck_1: size_t =
                        fwrite(restPtr as *const libc::c_void, 1i32 as size_t,
                               restPtr.offset_to(restEnd).expect("bad offset_to")
                                   as libc::c_long as size_t, file);
                    if sizeCheck_1 !=
                           restPtr.offset_to(restEnd).expect("bad offset_to")
                               as libc::c_long as size_t {
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 7],
                                                              &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 12],
                                                              &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                    75i32);
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 48],
                                                              &mut [libc::c_char; 48]>(b"Write error : cannot write decoded end of block\x00")).as_mut_ptr());
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 3],
                                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                        }
                        exit(75i32);
                    }
                }
            }
        }
        return storedSkips
    };
}
/* * FIO_decompressFrame() :
 *  @return : size of decoded zstd frame, or an error code
*/
#[no_mangle]
pub unsafe extern "C" fn FIO_decompressZstdFrame(mut ress: *mut dRess_t,
                                                 mut finput: *mut FILE,
                                                 mut srcFileName:
                                                     *const libc::c_char,
                                                 mut alreadyDecoded: U64)
 -> libc::c_ulonglong {
    let mut frameSize: U64 = 0i32 as U64;
    let mut storedSkips: U32 = 0i32 as U32;
    let srcFileLength: size_t = strlen(srcFileName);
    if srcFileLength > 20i32 as libc::c_ulong {
        srcFileName =
            srcFileName.offset(srcFileLength.wrapping_sub(20i32 as
                                                              libc::c_ulong)
                                   as isize)
    }
    ZSTD_resetDStream((*ress).dctx);
    let toDecode: size_t = 18i32 as size_t;
    if (*ress).srcBufferLoaded < toDecode {
        let toRead: size_t = toDecode.wrapping_sub((*ress).srcBufferLoaded);
        let startPosition: *mut libc::c_void =
            ((*ress).srcBuffer as
                 *mut libc::c_char).offset((*ress).srcBufferLoaded as isize)
                as *mut libc::c_void;
        (*ress).srcBufferLoaded =
            ((*ress).srcBufferLoaded as
                 libc::c_ulong).wrapping_add(fread(startPosition,
                                                   1i32 as size_t, toRead,
                                                   finput)) as size_t as
                size_t
    }
    loop  {
        let mut inBuff: ZSTD_inBuffer =
            ZSTD_inBuffer_s{src: (*ress).srcBuffer,
                            size: (*ress).srcBufferLoaded,
                            pos: 0i32 as size_t,};
        let mut outBuff: ZSTD_outBuffer =
            ZSTD_outBuffer_s{dst: (*ress).dstBuffer,
                             size: (*ress).dstBufferSize,
                             pos: 0i32 as size_t,};
        let readSizeHint: size_t =
            ZSTD_decompressStream((*ress).dctx,
                                  &mut outBuff as *mut ZSTD_outBuffer,
                                  &mut inBuff as *mut ZSTD_inBuffer);
        if 0 != ZSTD_isError(readSizeHint) {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 32],
                                                  &mut [libc::c_char; 32]>(b"%s : Decoding error (36) : %s \n\x00")).as_mut_ptr(),
                        srcFileName, ZSTD_getErrorName(readSizeHint));
            }
            FIO_zstdErrorHelp(ress, readSizeHint, srcFileName);
            return -2i32 as libc::c_ulonglong
        } else {
            storedSkips =
                FIO_fwriteSparse((*ress).dstFile, (*ress).dstBuffer,
                                 outBuff.pos, storedSkips);
            frameSize =
                (frameSize as libc::c_ulong).wrapping_add(outBuff.pos) as U64
                    as U64;
            if g_displayLevel >= 2i32 {
                if UTIL_clockSpanMicro(g_displayClock) > g_refreshRate ||
                       g_displayLevel >= 4i32 {
                    g_displayClock = UTIL_getTime();
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 26],
                                                      &mut [libc::c_char; 26]>(b"\r%-20.20s : %u MB...     \x00")).as_mut_ptr(),
                            srcFileName,
                            (alreadyDecoded.wrapping_add(frameSize) >> 20i32)
                                as U32);
                    if g_displayLevel >= 4i32 { fflush(stderr); }
                }
            }
            if inBuff.pos > 0i32 as libc::c_ulong {
                memmove((*ress).srcBuffer,
                        ((*ress).srcBuffer as
                             *mut libc::c_char).offset(inBuff.pos as isize) as
                            *const libc::c_void,
                        inBuff.size.wrapping_sub(inBuff.pos));
                (*ress).srcBufferLoaded =
                    ((*ress).srcBufferLoaded as
                         libc::c_ulong).wrapping_sub(inBuff.pos) as size_t as
                        size_t
            }
            if readSizeHint == 0i32 as libc::c_ulong { break ; }
            if inBuff.size != inBuff.pos {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 57],
                                                      &mut [libc::c_char; 57]>(b"%s : Decoding error (37) : should consume entire input \n\x00")).as_mut_ptr(),
                            srcFileName);
                }
                return -2i32 as libc::c_ulonglong
            } else {
                let toDecode_0: size_t =
                    if readSizeHint < (*ress).srcBufferSize {
                        readSizeHint
                    } else { (*ress).srcBufferSize };
                if !((*ress).srcBufferLoaded < toDecode_0) { continue ; }
                let toRead_0: size_t =
                    toDecode_0.wrapping_sub((*ress).srcBufferLoaded);
                let startPosition_0: *mut libc::c_void =
                    ((*ress).srcBuffer as
                         *mut libc::c_char).offset((*ress).srcBufferLoaded as
                                                       isize) as
                        *mut libc::c_void;
                let readSize: size_t =
                    fread(startPosition_0, 1i32 as size_t, toRead_0, finput);
                if readSize == 0i32 as libc::c_ulong {
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 39],
                                                          &mut [libc::c_char; 39]>(b"%s : Read error (39) : premature end \n\x00")).as_mut_ptr(),
                                srcFileName);
                    }
                    return -2i32 as libc::c_ulonglong
                } else {
                    (*ress).srcBufferLoaded =
                        ((*ress).srcBufferLoaded as
                             libc::c_ulong).wrapping_add(readSize) as size_t
                            as size_t
                }
            }
        }
    }
    FIO_fwriteSparseEnd((*ress).dstFile, storedSkips);
    return frameSize as libc::c_ulonglong;
}
unsafe extern "C" fn FIO_zstdErrorHelp(mut ress: *mut dRess_t,
                                       mut err: size_t,
                                       mut srcFileName: *const libc::c_char)
 -> () {
    let mut header: ZSTD_frameHeader =
        ZSTD_frameHeader{frameContentSize: 0,
                         windowSize: 0,
                         blockSizeMax: 0,
                         frameType: ZSTD_frame,
                         headerSize: 0,
                         dictID: 0,
                         checksumFlag: 0,};
    if ZSTD_getErrorCode(err) as libc::c_uint !=
           ZSTD_error_frameParameter_windowTooLarge as libc::c_int as
               libc::c_uint {
        return
    } else {
        err =
            ZSTD_getFrameHeader(&mut header as *mut ZSTD_frameHeader,
                                (*ress).srcBuffer, (*ress).srcBufferLoaded);
        if err == 0i32 as libc::c_ulong {
            let windowSize: libc::c_ulonglong = header.windowSize;
            let windowLog: U32 =
                FIO_highbit64(windowSize).wrapping_add((windowSize &
                                                            windowSize.wrapping_sub(1i32
                                                                                        as
                                                                                        libc::c_ulonglong)
                                                            !=
                                                            0i32 as
                                                                libc::c_ulonglong)
                                                           as libc::c_int as
                                                           libc::c_uint);
            let windowMB: U32 =
                (windowSize >>
                     20i32).wrapping_add((windowSize &
                                              (1i32 * (1i32 << 20i32) - 1i32)
                                                  as libc::c_ulonglong !=
                                              0i32 as libc::c_ulonglong) as
                                             libc::c_int as libc::c_ulonglong)
                    as U32;
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 50],
                                                  &mut [libc::c_char; 50]>(b"%s : Window size larger than maximum : %llu > %u\n\x00")).as_mut_ptr(),
                        srcFileName, windowSize, g_memLimit);
            }
            if windowLog <=
                   (if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                           4i32 as libc::c_ulong {
                        30i32
                    } else { 31i32 }) as libc::c_uint {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 37],
                                                      &mut [libc::c_char; 37]>(b"%s : Use --long=%u or --memory=%uMB\n\x00")).as_mut_ptr(),
                            srcFileName, windowLog, windowMB);
                }
                return
            }
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 66],
                                              &mut [libc::c_char; 66]>(b"%s : Window log larger than ZSTD_WINDOWLOG_MAX=%u; not supported\n\x00")).as_mut_ptr(),
                    srcFileName,
                    (if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                            4i32 as libc::c_ulong {
                         30i32
                     } else { 31i32 }) as libc::c_uint);
        }
        return;
    };
}
unsafe extern "C" fn FIO_highbit64(mut v: libc::c_ulonglong) -> libc::c_uint {
    let mut count: libc::c_uint = 0i32 as libc::c_uint;
    v >>= 1i32;
    while 0 != v { v >>= 1i32; count = count.wrapping_add(1) }
    return count;
}
unsafe extern "C" fn FIO_freeDResources(mut ress: dRess_t) -> () {
    let mut err: size_t = 0;
    err = ZSTD_freeDStream(ress.dctx);
    if 0 != ZSTD_isError(err) {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 7],
                                              &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                    11i32);
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 3],
                                              &mut [libc::c_char; 3]>(b"%s\x00")).as_mut_ptr(),
                    ZSTD_getErrorName(err));
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 3],
                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
        }
        exit(11i32);
    } else { free(ress.srcBuffer); free(ress.dstBuffer); return; };
}
#[no_mangle]
pub unsafe extern "C" fn FIO_listMultipleFiles(mut numFiles: libc::c_uint,
                                               mut filenameTable:
                                                   *mut *const libc::c_char,
                                               mut displayLevel: libc::c_int)
 -> libc::c_int {
    if 0 == isatty(fileno(stdin)) {
        fprintf(stdout,
                (*::std::mem::transmute::<&[u8; 59],
                                          &mut [libc::c_char; 59]>(b"zstd: --list does not support reading from standard input\n\x00")).as_mut_ptr());
        return 1i32
    } else if numFiles == 0i32 as libc::c_uint {
        fprintf(stdout,
                (*::std::mem::transmute::<&[u8; 16],
                                          &mut [libc::c_char; 16]>(b"No files given\n\x00")).as_mut_ptr());
        return 0i32
    } else {
        if displayLevel <= 2i32 {
            fprintf(stdout,
                    (*::std::mem::transmute::<&[u8; 65],
                                              &mut [libc::c_char; 65]>(b"Frames  Skips  Compressed  Uncompressed  Ratio  Check  Filename\n\x00")).as_mut_ptr());
        }
        let mut error: libc::c_int = 0i32;
        let mut u: libc::c_uint = 0;
        let mut total: fileInfo_t =
            fileInfo_t{decompressedSize: 0,
                       compressedSize: 0,
                       windowSize: 0,
                       numActualFrames: 0,
                       numSkippableFrames: 0,
                       decompUnavailable: 0,
                       usesCheck: 0,
                       nbFiles: 0,};
        memset(&mut total as *mut fileInfo_t as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<fileInfo_t>() as libc::c_ulong);
        total.usesCheck = 1i32;
        u = 0i32 as libc::c_uint;
        while u < numFiles {
            error |=
                FIO_listFile(&mut total as *mut fileInfo_t,
                             *filenameTable.offset(u as isize), displayLevel);
            u = u.wrapping_add(1)
        }
        if numFiles > 1i32 as libc::c_uint && displayLevel <= 2i32 {
            let unit: libc::c_uint =
                (if total.compressedSize <
                        (1i32 * (1i32 << 20i32)) as libc::c_ulong {
                     1i32 * (1i32 << 10i32)
                 } else { 1i32 * (1i32 << 20i32) }) as libc::c_uint;
            let unitStr: *const libc::c_char =
                if total.compressedSize <
                       (1i32 * (1i32 << 20i32)) as libc::c_ulong {
                    (*::std::mem::transmute::<&[u8; 3],
                                              &mut [libc::c_char; 3]>(b"KB\x00")).as_mut_ptr()
                } else {
                    (*::std::mem::transmute::<&[u8; 3],
                                              &mut [libc::c_char; 3]>(b"MB\x00")).as_mut_ptr()
                };
            let compressedSizeUnit: libc::c_double =
                total.compressedSize as libc::c_double /
                    unit as libc::c_double;
            let decompressedSizeUnit: libc::c_double =
                total.decompressedSize as libc::c_double /
                    unit as libc::c_double;
            let ratio: libc::c_double =
                if total.compressedSize == 0i32 as libc::c_ulong {
                    0i32 as libc::c_double
                } else {
                    total.decompressedSize as libc::c_double /
                        total.compressedSize as libc::c_double
                };
            let checkString: *const libc::c_char =
                if 0 != total.usesCheck {
                    (*::std::mem::transmute::<&[u8; 6],
                                              &mut [libc::c_char; 6]>(b"XXH64\x00")).as_mut_ptr()
                } else {
                    (*::std::mem::transmute::<&[u8; 1],
                                              &mut [libc::c_char; 1]>(b"\x00")).as_mut_ptr()
                };
            fprintf(stdout,
                    (*::std::mem::transmute::<&[u8; 68],
                                              &mut [libc::c_char; 68]>(b"----------------------------------------------------------------- \n\x00")).as_mut_ptr());
            if 0 != total.decompUnavailable {
                fprintf(stdout,
                        (*::std::mem::transmute::<&[u8; 57],
                                                  &mut [libc::c_char; 57]>(b"%6d  %5d  %7.2f %2s                       %5s  %u files\n\x00")).as_mut_ptr(),
                        total.numSkippableFrames + total.numActualFrames,
                        total.numSkippableFrames, compressedSizeUnit, unitStr,
                        checkString, total.nbFiles);
            } else {
                fprintf(stdout,
                        (*::std::mem::transmute::<&[u8; 54],
                                                  &mut [libc::c_char; 54]>(b"%6d  %5d  %7.2f %2s  %9.2f %2s  %5.3f  %5s  %u files\n\x00")).as_mut_ptr(),
                        total.numSkippableFrames + total.numActualFrames,
                        total.numSkippableFrames, compressedSizeUnit, unitStr,
                        decompressedSizeUnit, unitStr, ratio, checkString,
                        total.nbFiles);
            }
        }
        return error
    };
}
unsafe extern "C" fn FIO_listFile(mut total: *mut fileInfo_t,
                                  mut inFileName: *const libc::c_char,
                                  mut displayLevel: libc::c_int)
 -> libc::c_int {
    let mut info: fileInfo_t =
        fileInfo_t{decompressedSize: 0,
                   compressedSize: 0,
                   windowSize: 0,
                   numActualFrames: 0,
                   numSkippableFrames: 0,
                   decompUnavailable: 0,
                   usesCheck: 0,
                   nbFiles: 0,};
    memset(&mut info as *mut fileInfo_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<fileInfo_t>() as libc::c_ulong);
    let error: libc::c_int =
        getFileInfo(&mut info as *mut fileInfo_t, inFileName);
    if error == 1i32 {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 44],
                                          &mut [libc::c_char; 44]>(b"An error occurred while getting file info \n\x00")).as_mut_ptr());
    } else if error == 2i32 {
        fprintf(stdout,
                (*::std::mem::transmute::<&[u8; 33],
                                          &mut [libc::c_char; 33]>(b"File %s not compressed by zstd \n\x00")).as_mut_ptr(),
                inFileName);
        if displayLevel > 2i32 {
            fprintf(stdout,
                    (*::std::mem::transmute::<&[u8; 2],
                                              &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
        }
        return 1i32
    } else if error == 3i32 {
        if displayLevel > 2i32 {
            fprintf(stdout,
                    (*::std::mem::transmute::<&[u8; 2],
                                              &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
        }
        return 1i32
    }
    displayInfo(inFileName, &mut info as *mut fileInfo_t, displayLevel);
    *total = FIO_addFInfo(*total, info);
    return error;
}
unsafe extern "C" fn getFileInfo(mut info: *mut fileInfo_t,
                                 mut srcFileName: *const libc::c_char)
 -> libc::c_int {
    let isAFile: libc::c_int = UTIL_isRegularFile(srcFileName);
    if 0 == isAFile {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 25],
                                          &mut [libc::c_char; 25]>(b"Error : %s is not a file\x00")).as_mut_ptr(),
                srcFileName);
        return 3i32
    } else { return getFileInfo_fileConfirmed(info, srcFileName) };
}
/* * getFileInfo() :
 *  Reads information from file, stores in *info
 * @return : 0 if successful
 *           1 for frame analysis error
 *           2 for file not compressed with zstd
 *           3 for cases in which file could not be opened.
 */
unsafe extern "C" fn getFileInfo_fileConfirmed(mut info: *mut fileInfo_t,
                                               mut inFileName:
                                                   *const libc::c_char)
 -> libc::c_int {
    let mut detectError: libc::c_int = 0i32;
    let srcFile: *mut FILE = FIO_openSrcFile(inFileName);
    if srcFile.is_null() {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 38],
                                          &mut [libc::c_char; 38]>(b"Error: could not open source file %s\n\x00")).as_mut_ptr(),
                inFileName);
        return 3i32
    } else {
        (*info).compressedSize = UTIL_getFileSize(inFileName);
        loop  {
            let mut headerBuffer: [BYTE; 18] = [0; 18];
            let numBytesRead: size_t =
                fread(headerBuffer.as_mut_ptr() as *mut libc::c_void,
                      1i32 as size_t,
                      ::std::mem::size_of::<[BYTE; 18]>() as libc::c_ulong,
                      srcFile);
            if numBytesRead < ZSTD_frameHeaderSize_min {
                if 0 != feof(srcFile) && numBytesRead == 0i32 as libc::c_ulong
                       && (*info).compressedSize > 0i32 as libc::c_ulong &&
                       (*info).compressedSize != -1i32 as U64 {
                    break ;
                }
                if 0 != feof(srcFile) {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 50],
                                                      &mut [libc::c_char; 50]>(b"Error: reached end of file with incomplete frame\n\x00")).as_mut_ptr());
                    detectError = 2i32;
                    break ;
                } else {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 56],
                                                      &mut [libc::c_char; 56]>(b"Error: did not reach end of file but ran out of frames\n\x00")).as_mut_ptr());
                    detectError = 1i32;
                    break ;
                }
            } else {
                let magicNumber: U32 =
                    MEM_readLE32(headerBuffer.as_mut_ptr() as
                                     *const libc::c_void);
                if magicNumber == 4247762216u32 {
                    let mut header: ZSTD_frameHeader =
                        ZSTD_frameHeader{frameContentSize: 0,
                                         windowSize: 0,
                                         blockSizeMax: 0,
                                         frameType: ZSTD_frame,
                                         headerSize: 0,
                                         dictID: 0,
                                         checksumFlag: 0,};
                    let frameContentSize: U64 =
                        ZSTD_getFrameContentSize(headerBuffer.as_mut_ptr() as
                                                     *const libc::c_void,
                                                 numBytesRead) as U64;
                    if frameContentSize as libc::c_ulonglong ==
                           0u64.wrapping_sub(2i32 as libc::c_ulonglong) ||
                           frameContentSize as libc::c_ulonglong ==
                               0u64.wrapping_sub(1i32 as libc::c_ulonglong) {
                        (*info).decompUnavailable = 1i32
                    } else {
                        (*info).decompressedSize =
                            ((*info).decompressedSize as
                                 libc::c_ulong).wrapping_add(frameContentSize)
                                as U64 as U64
                    }
                    if ZSTD_getFrameHeader(&mut header as
                                               *mut ZSTD_frameHeader,
                                           headerBuffer.as_mut_ptr() as
                                               *const libc::c_void,
                                           numBytesRead) !=
                           0i32 as libc::c_ulong {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 38],
                                                          &mut [libc::c_char; 38]>(b"Error: could not decode frame header\n\x00")).as_mut_ptr());
                        detectError = 1i32;
                        break ;
                    } else {
                        (*info).windowSize = header.windowSize as U64;
                        let headerSize: size_t =
                            ZSTD_frameHeaderSize(headerBuffer.as_mut_ptr() as
                                                     *const libc::c_void,
                                                 numBytesRead);
                        if 0 != ZSTD_isError(headerSize) {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 46],
                                                              &mut [libc::c_char; 46]>(b"Error: could not determine frame header size\n\x00")).as_mut_ptr());
                            detectError = 1i32;
                            break ;
                        } else {
                            let ret: libc::c_int =
                                fseek(srcFile,
                                      headerSize as libc::c_long -
                                          numBytesRead as libc::c_long, 1i32);
                            if ret != 0i32 {
                                fprintf(stderr,
                                        (*::std::mem::transmute::<&[u8; 46],
                                                                  &mut [libc::c_char; 46]>(b"Error: could not move to end of frame header\n\x00")).as_mut_ptr());
                                detectError = 1i32;
                                break ;
                            } else {
                                let mut lastBlock: libc::c_int = 0i32;
                                loop  {
                                    let mut blockHeaderBuffer: [BYTE; 3] =
                                        [0; 3];
                                    let readBytes: size_t =
                                        fread(blockHeaderBuffer.as_mut_ptr()
                                                  as *mut libc::c_void,
                                              1i32 as size_t, 3i32 as size_t,
                                              srcFile);
                                    if readBytes != 3i32 as libc::c_ulong {
                                        fprintf(stderr,
                                                (*::std::mem::transmute::<&[u8; 46],
                                                                          &mut [libc::c_char; 46]>(b"There was a problem reading the block header\n\x00")).as_mut_ptr());
                                        detectError = 1i32;
                                        break ;
                                    } else {
                                        let blockHeader: U32 =
                                            MEM_readLE24(blockHeaderBuffer.as_mut_ptr()
                                                             as
                                                             *const libc::c_void);
                                        let blockTypeID: U32 =
                                            blockHeader >> 1i32 &
                                                3i32 as libc::c_uint;
                                        let isRLE: U32 =
                                            (blockTypeID ==
                                                 1i32 as libc::c_uint) as
                                                libc::c_int as U32;
                                        let isWrongBlock: U32 =
                                            (blockTypeID ==
                                                 3i32 as libc::c_uint) as
                                                libc::c_int as U32;
                                        let blockSize: libc::c_long =
                                            if 0 != isRLE {
                                                1i32 as libc::c_long
                                            } else {
                                                (blockHeader >> 3i32) as
                                                    libc::c_long
                                            };
                                        if 0 != isWrongBlock {
                                            fprintf(stderr,
                                                    (*::std::mem::transmute::<&[u8; 32],
                                                                              &mut [libc::c_char; 32]>(b"Error: unsupported block type \n\x00")).as_mut_ptr());
                                            detectError = 1i32;
                                            break ;
                                        } else {
                                            lastBlock =
                                                (blockHeader &
                                                     1i32 as libc::c_uint) as
                                                    libc::c_int;
                                            let ret_0: libc::c_int =
                                                fseek(srcFile, blockSize,
                                                      1i32);
                                            if ret_0 != 0i32 {
                                                fprintf(stderr,
                                                        (*::std::mem::transmute::<&[u8; 39],
                                                                                  &mut [libc::c_char; 39]>(b"Error: could not skip to end of block\n\x00")).as_mut_ptr());
                                                detectError = 1i32;
                                                break ;
                                            } else if !(lastBlock != 1i32) {
                                                break ;
                                            }
                                        }
                                    }
                                }
                                if 0 != detectError { break ; }
                                let frameHeaderDescriptor: BYTE =
                                    headerBuffer[4usize];
                                let contentChecksumFlag: libc::c_int =
                                    (frameHeaderDescriptor as libc::c_int &
                                         1i32 << 2i32) >> 2i32;
                                if 0 != contentChecksumFlag {
                                    let ret_1: libc::c_int =
                                        fseek(srcFile, 4i32 as libc::c_long,
                                              1i32);
                                    (*info).usesCheck = 1i32;
                                    if ret_1 != 0i32 {
                                        fprintf(stderr,
                                                (*::std::mem::transmute::<&[u8; 37],
                                                                          &mut [libc::c_char; 37]>(b"Error: could not skip past checksum\n\x00")).as_mut_ptr());
                                        detectError = 1i32;
                                        break ;
                                    }
                                }
                                (*info).numActualFrames += 1
                            }
                        }
                    }
                } else if magicNumber & 4294967280u32 == 407710288u32 {
                    let frameSize: U32 =
                        MEM_readLE32(headerBuffer.as_mut_ptr().offset(4isize)
                                         as *const libc::c_void);
                    let seek: libc::c_long =
                        ((8i32 as libc::c_uint).wrapping_add(frameSize) as
                             libc::c_ulong).wrapping_sub(numBytesRead) as
                            libc::c_long;
                    let ret_2: libc::c_int = fseek(srcFile, seek, 1i32);
                    if ret_2 != 0i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 46],
                                                          &mut [libc::c_char; 46]>(b"Error: could not find end of skippable frame\n\x00")).as_mut_ptr());
                        detectError = 1i32;
                        break ;
                    } else { (*info).numSkippableFrames += 1 }
                } else { detectError = 2i32; break ; }
            }
        }
        fclose(srcFile);
        (*info).nbFiles = 1i32 as U32;
        return detectError
    };
}
unsafe extern "C" fn FIO_addFInfo(mut fi1: fileInfo_t, mut fi2: fileInfo_t)
 -> fileInfo_t {
    let mut total: fileInfo_t =
        fileInfo_t{decompressedSize: 0,
                   compressedSize: 0,
                   windowSize: 0,
                   numActualFrames: 0,
                   numSkippableFrames: 0,
                   decompUnavailable: 0,
                   usesCheck: 0,
                   nbFiles: 0,};
    memset(&mut total as *mut fileInfo_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<fileInfo_t>() as libc::c_ulong);
    total.numActualFrames = fi1.numActualFrames + fi2.numActualFrames;
    total.numSkippableFrames =
        fi1.numSkippableFrames + fi2.numSkippableFrames;
    total.compressedSize =
        fi1.compressedSize.wrapping_add(fi2.compressedSize);
    total.decompressedSize =
        fi1.decompressedSize.wrapping_add(fi2.decompressedSize);
    total.decompUnavailable = fi1.decompUnavailable | fi2.decompUnavailable;
    total.usesCheck = fi1.usesCheck & fi2.usesCheck;
    total.nbFiles = fi1.nbFiles.wrapping_add(fi2.nbFiles);
    return total;
}
unsafe extern "C" fn displayInfo(mut inFileName: *const libc::c_char,
                                 mut info: *const fileInfo_t,
                                 mut displayLevel: libc::c_int) -> () {
    let unit: libc::c_uint =
        (if (*info).compressedSize < (1i32 * (1i32 << 20i32)) as libc::c_ulong
            {
             1i32 * (1i32 << 10i32)
         } else { 1i32 * (1i32 << 20i32) }) as libc::c_uint;
    let unitStr: *const libc::c_char =
        if (*info).compressedSize < (1i32 * (1i32 << 20i32)) as libc::c_ulong
           {
            (*::std::mem::transmute::<&[u8; 3],
                                      &mut [libc::c_char; 3]>(b"KB\x00")).as_mut_ptr()
        } else {
            (*::std::mem::transmute::<&[u8; 3],
                                      &mut [libc::c_char; 3]>(b"MB\x00")).as_mut_ptr()
        };
    let windowSizeUnit: libc::c_double =
        (*info).windowSize as libc::c_double / unit as libc::c_double;
    let compressedSizeUnit: libc::c_double =
        (*info).compressedSize as libc::c_double / unit as libc::c_double;
    let decompressedSizeUnit: libc::c_double =
        (*info).decompressedSize as libc::c_double / unit as libc::c_double;
    let ratio: libc::c_double =
        if (*info).compressedSize == 0i32 as libc::c_ulong {
            0i32 as libc::c_double
        } else {
            (*info).decompressedSize as libc::c_double /
                (*info).compressedSize as libc::c_double
        };
    let checkString: *const libc::c_char =
        if 0 != (*info).usesCheck {
            (*::std::mem::transmute::<&[u8; 6],
                                      &mut [libc::c_char; 6]>(b"XXH64\x00")).as_mut_ptr()
        } else {
            (*::std::mem::transmute::<&[u8; 5],
                                      &mut [libc::c_char; 5]>(b"None\x00")).as_mut_ptr()
        };
    if displayLevel <= 2i32 {
        if 0 == (*info).decompUnavailable {
            fprintf(stdout,
                    (*::std::mem::transmute::<&[u8; 48],
                                              &mut [libc::c_char; 48]>(b"%6d  %5d  %7.2f %2s  %9.2f %2s  %5.3f  %5s  %s\n\x00")).as_mut_ptr(),
                    (*info).numSkippableFrames + (*info).numActualFrames,
                    (*info).numSkippableFrames, compressedSizeUnit, unitStr,
                    decompressedSizeUnit, unitStr, ratio, checkString,
                    inFileName);
        } else {
            fprintf(stdout,
                    (*::std::mem::transmute::<&[u8; 51],
                                              &mut [libc::c_char; 51]>(b"%6d  %5d  %7.2f %2s                       %5s  %s\n\x00")).as_mut_ptr(),
                    (*info).numSkippableFrames + (*info).numActualFrames,
                    (*info).numSkippableFrames, compressedSizeUnit, unitStr,
                    checkString, inFileName);
        }
    } else {
        fprintf(stdout,
                (*::std::mem::transmute::<&[u8; 5],
                                          &mut [libc::c_char; 5]>(b"%s \n\x00")).as_mut_ptr(),
                inFileName);
        fprintf(stdout,
                (*::std::mem::transmute::<&[u8; 24],
                                          &mut [libc::c_char; 24]>(b"# Zstandard Frames: %d\n\x00")).as_mut_ptr(),
                (*info).numActualFrames);
        if 0 != (*info).numSkippableFrames {
            fprintf(stdout,
                    (*::std::mem::transmute::<&[u8; 24],
                                              &mut [libc::c_char; 24]>(b"# Skippable Frames: %d\n\x00")).as_mut_ptr(),
                    (*info).numSkippableFrames);
        }
        fprintf(stdout,
                (*::std::mem::transmute::<&[u8; 32],
                                          &mut [libc::c_char; 32]>(b"Window Size: %.2f %2s (%llu B)\n\x00")).as_mut_ptr(),
                windowSizeUnit, unitStr,
                (*info).windowSize as libc::c_ulonglong);
        fprintf(stdout,
                (*::std::mem::transmute::<&[u8; 36],
                                          &mut [libc::c_char; 36]>(b"Compressed Size: %.2f %2s (%llu B)\n\x00")).as_mut_ptr(),
                compressedSizeUnit, unitStr,
                (*info).compressedSize as libc::c_ulonglong);
        if 0 == (*info).decompUnavailable {
            fprintf(stdout,
                    (*::std::mem::transmute::<&[u8; 38],
                                              &mut [libc::c_char; 38]>(b"Decompressed Size: %.2f %2s (%llu B)\n\x00")).as_mut_ptr(),
                    decompressedSizeUnit, unitStr,
                    (*info).decompressedSize as libc::c_ulonglong);
            fprintf(stdout,
                    (*::std::mem::transmute::<&[u8; 13],
                                              &mut [libc::c_char; 13]>(b"Ratio: %.4f\n\x00")).as_mut_ptr(),
                    ratio);
        }
        fprintf(stdout,
                (*::std::mem::transmute::<&[u8; 11],
                                          &mut [libc::c_char; 11]>(b"Check: %s\n\x00")).as_mut_ptr(),
                checkString);
        fprintf(stdout,
                (*::std::mem::transmute::<&[u8; 2],
                                          &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
    };
}
/* * FIO_compressMultipleFilenames() :
    @return : nb of missing files */
#[no_mangle]
pub unsafe extern "C" fn FIO_compressMultipleFilenames(mut inFileNamesTable:
                                                           *mut *const libc::c_char,
                                                       mut nbFiles:
                                                           libc::c_uint,
                                                       mut outFileName:
                                                           *const libc::c_char,
                                                       mut suffix:
                                                           *const libc::c_char,
                                                       mut dictFileName:
                                                           *const libc::c_char,
                                                       mut compressionLevel:
                                                           libc::c_int,
                                                       mut comprParams:
                                                           *mut ZSTD_compressionParameters)
 -> libc::c_int {
    let mut missed_files: libc::c_int = 0i32;
    let mut dfnSize: size_t = 30i32 as size_t;
    let mut dstFileName: *mut libc::c_char =
        malloc(30i32 as libc::c_ulong) as *mut libc::c_char;
    let suffixSize: size_t =
        if !suffix.is_null() {
            strlen(suffix)
        } else { 0i32 as libc::c_ulong };
    let firstFileSize: U64 =
        UTIL_getFileSize(*inFileNamesTable.offset(0isize));
    let firstSrcSize: U64 =
        (if firstFileSize == -1i32 as U64 {
             0u64.wrapping_sub(1i32 as libc::c_ulonglong)
         } else { firstFileSize as libc::c_ulonglong }) as U64;
    let srcSize: U64 =
        (if nbFiles != 1i32 as libc::c_uint {
             0u64.wrapping_sub(1i32 as libc::c_ulonglong)
         } else { firstSrcSize as libc::c_ulonglong }) as U64;
    let mut ress: cRess_t =
        FIO_createCResources(dictFileName, compressionLevel, srcSize,
                             comprParams);
    if dstFileName.is_null() {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 7],
                                              &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                    27i32);
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 65],
                                              &mut [libc::c_char; 65]>(b"FIO_compressMultipleFilenames : allocation error for dstFileName\x00")).as_mut_ptr());
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 3],
                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
        }
        exit(27i32);
    } else if outFileName.is_null() && suffix.is_null() {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 7],
                                              &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 12],
                                              &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                    28i32);
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 44],
                                              &mut [libc::c_char; 44]>(b"FIO_compressMultipleFilenames : dst unknown\x00")).as_mut_ptr());
        }
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 3],
                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
        }
        exit(28i32);
    } else {
        if !outFileName.is_null() {
            let mut u: libc::c_uint = 0;
            ress.dstFile = FIO_openDstFile(outFileName);
            if ress.dstFile.is_null() {
                missed_files = nbFiles as libc::c_int
            } else {
                u = 0i32 as libc::c_uint;
                while u < nbFiles {
                    missed_files +=
                        FIO_compressFilename_srcFile(ress, outFileName,
                                                     *inFileNamesTable.offset(u
                                                                                  as
                                                                                  isize),
                                                     compressionLevel);
                    u = u.wrapping_add(1)
                }
                if 0 != fclose(ress.dstFile) {
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 7],
                                                          &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                29i32);
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 43],
                                                          &mut [libc::c_char; 43]>(b"Write error : cannot properly close stdout\x00")).as_mut_ptr());
                    }
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 3],
                                                          &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                    }
                    exit(29i32);
                }
            }
        } else {
            let mut u_0: libc::c_uint = 0;
            u_0 = 0i32 as libc::c_uint;
            while u_0 < nbFiles {
                let ifnSize: size_t =
                    strlen(*inFileNamesTable.offset(u_0 as isize));
                if dfnSize <=
                       ifnSize.wrapping_add(suffixSize).wrapping_add(1i32 as
                                                                         libc::c_ulong)
                   {
                    free(dstFileName as *mut libc::c_void);
                    dfnSize = ifnSize.wrapping_add(20i32 as libc::c_ulong);
                    dstFileName = malloc(dfnSize) as *mut libc::c_char;
                    if dstFileName.is_null() {
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 7],
                                                              &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 12],
                                                              &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                    30i32);
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 9],
                                                              &mut [libc::c_char; 9]>(b"zstd: %s\x00")).as_mut_ptr(),
                                    strerror(*__errno_location()));
                        }
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 3],
                                                              &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                        }
                        exit(30i32);
                    }
                }
                strcpy(dstFileName, *inFileNamesTable.offset(u_0 as isize));
                strcat(dstFileName, suffix);
                missed_files +=
                    FIO_compressFilename_dstFile(ress, dstFileName,
                                                 *inFileNamesTable.offset(u_0
                                                                              as
                                                                              isize),
                                                 compressionLevel);
                u_0 = u_0.wrapping_add(1)
            }
        }
        FIO_freeCResources(ress);
        free(dstFileName as *mut libc::c_void);
        return missed_files
    };
}
/* * FIO_decompressMultipleFilenames() :
    @return : nb of missing or skipped files */
#[no_mangle]
pub unsafe extern "C" fn FIO_decompressMultipleFilenames(mut srcNamesTable:
                                                             *mut *const libc::c_char,
                                                         mut nbFiles:
                                                             libc::c_uint,
                                                         mut outFileName:
                                                             *const libc::c_char,
                                                         mut dictFileName:
                                                             *const libc::c_char)
 -> libc::c_int {
    let mut skippedFiles: libc::c_int = 0i32;
    let mut missingFiles: libc::c_int = 0i32;
    let mut ress: dRess_t = FIO_createDResources(dictFileName);
    if !outFileName.is_null() {
        let mut u: libc::c_uint = 0;
        ress.dstFile = FIO_openDstFile(outFileName);
        if ress.dstFile.is_null() {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 7],
                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 12],
                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                        71i32);
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 15],
                                                  &mut [libc::c_char; 15]>(b"cannot open %s\x00")).as_mut_ptr(),
                        outFileName);
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 3],
                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
            }
            exit(71i32);
        } else {
            u = 0i32 as libc::c_uint;
            while u < nbFiles {
                missingFiles +=
                    FIO_decompressSrcFile(ress, outFileName,
                                          *srcNamesTable.offset(u as isize));
                u = u.wrapping_add(1)
            }
            if 0 != fclose(ress.dstFile) {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 7],
                                                      &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 12],
                                                      &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                            72i32);
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 48],
                                                      &mut [libc::c_char; 48]>(b"Write error : cannot properly close output file\x00")).as_mut_ptr());
                }
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 3],
                                                      &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                }
                exit(72i32);
            }
        }
    } else {
        let mut suffixSize: size_t = 0;
        let mut dfnSize: size_t = 30i32 as size_t;
        let mut u_0: libc::c_uint = 0;
        let mut dstFileName: *mut libc::c_char =
            malloc(30i32 as libc::c_ulong) as *mut libc::c_char;
        if dstFileName.is_null() {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 7],
                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 12],
                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                        73i32);
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 34],
                                                  &mut [libc::c_char; 34]>(b"not enough memory for dstFileName\x00")).as_mut_ptr());
            }
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 3],
                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
            }
            exit(73i32);
        } else {
            u_0 = 0i32 as libc::c_uint;
            while u_0 < nbFiles {
                let srcFileName: *const libc::c_char =
                    *srcNamesTable.offset(u_0 as isize);
                let suffixPtr: *const libc::c_char =
                    strrchr(srcFileName, '.' as i32);
                let sfnSize: size_t = strlen(srcFileName);
                if suffixPtr.is_null() {
                    if g_displayLevel >= 1i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 38],
                                                          &mut [libc::c_char; 38]>(b"zstd: %s: unknown suffix -- ignored \n\x00")).as_mut_ptr(),
                                srcFileName);
                    }
                    skippedFiles += 1
                } else {
                    suffixSize = strlen(suffixPtr);
                    if dfnSize.wrapping_add(suffixSize) <=
                           sfnSize.wrapping_add(1i32 as libc::c_ulong) {
                        free(dstFileName as *mut libc::c_void);
                        dfnSize =
                            sfnSize.wrapping_add(20i32 as libc::c_ulong);
                        dstFileName = malloc(dfnSize) as *mut libc::c_char;
                        if dstFileName.is_null() {
                            if g_displayLevel >= 1i32 {
                                fprintf(stderr,
                                        (*::std::mem::transmute::<&[u8; 7],
                                                                  &mut [libc::c_char; 7]>(b"zstd: \x00")).as_mut_ptr());
                            }
                            if g_displayLevel >= 1i32 {
                                fprintf(stderr,
                                        (*::std::mem::transmute::<&[u8; 12],
                                                                  &mut [libc::c_char; 12]>(b"error %i : \x00")).as_mut_ptr(),
                                        74i32);
                            }
                            if g_displayLevel >= 1i32 {
                                fprintf(stderr,
                                        (*::std::mem::transmute::<&[u8; 34],
                                                                  &mut [libc::c_char; 34]>(b"not enough memory for dstFileName\x00")).as_mut_ptr());
                            }
                            if g_displayLevel >= 1i32 {
                                fprintf(stderr,
                                        (*::std::mem::transmute::<&[u8; 3],
                                                                  &mut [libc::c_char; 3]>(b" \n\x00")).as_mut_ptr());
                            }
                            exit(74i32);
                        }
                    }
                    if sfnSize <= suffixSize ||
                           0 !=
                               strcmp(suffixPtr,
                                      (*::std::mem::transmute::<&[u8; 4],
                                                                &mut [libc::c_char; 4]>(b".gz\x00")).as_mut_ptr())
                               &&
                               0 !=
                                   strcmp(suffixPtr,
                                          (*::std::mem::transmute::<&[u8; 4],
                                                                    &mut [libc::c_char; 4]>(b".xz\x00")).as_mut_ptr())
                               &&
                               0 !=
                                   strcmp(suffixPtr,
                                          (*::std::mem::transmute::<&[u8; 5],
                                                                    &mut [libc::c_char; 5]>(b".zst\x00")).as_mut_ptr())
                               &&
                               0 !=
                                   strcmp(suffixPtr,
                                          (*::std::mem::transmute::<&[u8; 6],
                                                                    &mut [libc::c_char; 6]>(b".lzma\x00")).as_mut_ptr())
                               &&
                               0 !=
                                   strcmp(suffixPtr,
                                          (*::std::mem::transmute::<&[u8; 5],
                                                                    &mut [libc::c_char; 5]>(b".lz4\x00")).as_mut_ptr())
                       {
                        let mut suffixlist: *const libc::c_char =
                            (*::std::mem::transmute::<&[u8; 5],
                                                      &mut [libc::c_char; 5]>(b".zst\x00")).as_mut_ptr();
                        if g_displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 52],
                                                              &mut [libc::c_char; 52]>(b"zstd: %s: unknown suffix (%s expected) -- ignored \n\x00")).as_mut_ptr(),
                                    srcFileName, suffixlist);
                        }
                        skippedFiles += 1
                    } else {
                        memcpy(dstFileName as *mut libc::c_void,
                               srcFileName as *const libc::c_void,
                               sfnSize.wrapping_sub(suffixSize));
                        *dstFileName.offset(sfnSize.wrapping_sub(suffixSize)
                                                as isize) =
                            '\u{0}' as i32 as libc::c_char;
                        missingFiles +=
                            FIO_decompressDstFile(ress, dstFileName,
                                                  srcFileName)
                    }
                }
                u_0 = u_0.wrapping_add(1)
            }
            free(dstFileName as *mut libc::c_void);
        }
    }
    FIO_freeDResources(ress);
    return missingFiles + skippedFiles;
}
unsafe extern "C" fn run_static_initializers() -> () {
    segmentSizeT =
        ((32i32 * (1i32 << 10i32)) as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<size_t>() as
                                             libc::c_ulong);
    maskT =
        (::std::mem::size_of::<size_t>() as
             libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong)
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn() -> (); 1] =
    [run_static_initializers];
