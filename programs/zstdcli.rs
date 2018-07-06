#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( const_slice_as_ptr , extern_types , libc , offset_to )]
extern crate libc;
extern "C" {
    pub type __dirstream;
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
    fn fclose(__stream: *mut FILE) -> libc::c_int;
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
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fileno(__stream: *mut FILE) -> libc::c_int;
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
    fn ZSTD_maxCLevel() -> libc::c_int;
    #[no_mangle]
    fn FIO_setCompressionType(compressionType: FIO_compressionType_t) -> ();
    #[no_mangle]
    fn FIO_overwriteMode() -> ();
    #[no_mangle]
    fn FIO_setNotificationLevel(level: libc::c_uint) -> ();
    #[no_mangle]
    fn FIO_setSparseWrite(sparse: libc::c_uint) -> ();
    /* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
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
    /* *< 0: no sparse; 1: disable on stdout; 2: always enabled */
    #[no_mangle]
    fn FIO_setDictIDFlag(dictIDFlag: libc::c_uint) -> ();
    #[no_mangle]
    fn FIO_setChecksumFlag(checksumFlag: libc::c_uint) -> ();
    #[no_mangle]
    fn FIO_setRemoveSrcFile(flag: libc::c_uint) -> ();
    #[no_mangle]
    fn FIO_setMemLimit(memLimit: libc::c_uint) -> ();
    #[no_mangle]
    fn FIO_setNbWorkers(nbWorkers: libc::c_uint) -> ();
    #[no_mangle]
    fn FIO_setBlockSize(blockSize: libc::c_uint) -> ();
    #[no_mangle]
    fn FIO_setOverlapLog(overlapLog: libc::c_uint) -> ();
    #[no_mangle]
    fn FIO_setLdmFlag(ldmFlag: libc::c_uint) -> ();
    #[no_mangle]
    fn FIO_setLdmHashLog(ldmHashLog: libc::c_uint) -> ();
    #[no_mangle]
    fn FIO_setLdmMinMatch(ldmMinMatch: libc::c_uint) -> ();
    #[no_mangle]
    fn FIO_setLdmBucketSizeLog(ldmBucketSizeLog: libc::c_uint) -> ();
    #[no_mangle]
    fn FIO_setLdmHashEveryLog(ldmHashEveryLog: libc::c_uint) -> ();
    /* * FIO_compressFilename() :
    @return : 0 == ok;  1 == pb with src file. */
    #[no_mangle]
    fn FIO_compressFilename(outfilename: *const libc::c_char,
                            infilename: *const libc::c_char,
                            dictFileName: *const libc::c_char,
                            compressionLevel: libc::c_int,
                            comprParams: *mut ZSTD_compressionParameters)
     -> libc::c_int;
    /* * FIO_decompressFilename() :
    @return : 0 == ok;  1 == pb with src file. */
    #[no_mangle]
    fn FIO_decompressFilename(outfilename: *const libc::c_char,
                              infilename: *const libc::c_char,
                              dictFileName: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn FIO_listMultipleFiles(numFiles: libc::c_uint,
                             filenameTable: *mut *const libc::c_char,
                             displayLevel: libc::c_int) -> libc::c_int;
    /* * FIO_compressMultipleFilenames() :
    @return : nb of missing files */
    #[no_mangle]
    fn FIO_compressMultipleFilenames(srcNamesTable: *mut *const libc::c_char,
                                     nbFiles: libc::c_uint,
                                     outFileName: *const libc::c_char,
                                     suffix: *const libc::c_char,
                                     dictFileName: *const libc::c_char,
                                     compressionLevel: libc::c_int,
                                     comprParams:
                                         *mut ZSTD_compressionParameters)
     -> libc::c_int;
    /* * FIO_decompressMultipleFilenames() :
    @return : nb of missing or skipped files */
    #[no_mangle]
    fn FIO_decompressMultipleFilenames(srcNamesTable:
                                           *mut *const libc::c_char,
                                       nbFiles: libc::c_uint,
                                       outFileName: *const libc::c_char,
                                       dictFileName: *const libc::c_char)
     -> libc::c_int;
}
pub const _SC_2_PBS: unnamed_0 = 168;
pub type __blksize_t = libc::c_long;
pub const _SC_RE_DUP_MAX: unnamed_0 = 44;
pub const _SC_SEMAPHORES: unnamed_0 = 21;
pub const _SC_2_C_DEV: unnamed_0 = 48;
pub const _SC_V7_LPBIG_OFFBIG: unnamed_0 = 240;
pub const _SC_XOPEN_SHM: unnamed_0 = 94;
pub const _SC_THREAD_SAFE_FUNCTIONS: unnamed_0 = 68;
pub type __uid_t = libc::c_uint;
pub const _SC_READER_WRITER_LOCKS: unnamed_0 = 153;
pub const _SC_INT_MAX: unnamed_0 = 104;
pub const _SC_V7_ILP32_OFFBIG: unnamed_0 = 238;
pub const _SC_LEVEL3_CACHE_ASSOC: unnamed_0 = 195;
pub const _SC_TYPED_MEMORY_OBJECTS: unnamed_0 = 165;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub const _SC_MONOTONIC_CLOCK: unnamed_0 = 149;
pub const _SC_GETGR_R_SIZE_MAX: unnamed_0 = 69;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: unnamed_0 = 73;
pub const _SC_NPROCESSORS_CONF: unnamed_0 = 83;
pub type time_t = __time_t;
pub const _SC_POLL: unnamed_0 = 58;
pub type unnamed = libc::c_uint;
pub const _SC_THREAD_THREADS_MAX: unnamed_0 = 76;
pub const _SC_SYSTEM_DATABASE_R: unnamed_0 = 163;
pub const _SC_PII_XTI: unnamed_0 = 54;
pub const _SC_USER_GROUPS_R: unnamed_0 = 167;
pub const _SC_SELECT: unnamed_0 = 59;
pub const _SC_SPAWN: unnamed_0 = 159;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type BYTE = uint8_t;
pub const _SC_SHRT_MIN: unnamed_0 = 114;
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
pub const _SC_LEVEL2_CACHE_SIZE: unnamed_0 = 191;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub type U16 = uint16_t;
pub const _SC_TRACE: unnamed_0 = 181;
pub const zom_decompress: zstd_operation_mode = 1;
pub const _SC_SYMLOOP_MAX: unnamed_0 = 173;
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub type DIR = __dirstream;
pub type clockid_t = __clockid_t;
pub const FIO_zstdCompression: FIO_compressionType_t = 0;
pub const _SC_INT_MIN: unnamed_0 = 105;
pub const _SC_2_PBS_LOCATE: unnamed_0 = 170;
pub const _SC_2_PBS_TRACK: unnamed_0 = 172;
pub const _SC_NL_ARGMAX: unnamed_0 = 119;
pub const _SC_DEVICE_SPECIFIC: unnamed_0 = 141;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub const _SC_2_UPE: unnamed_0 = 97;
pub const _SC_FILE_SYSTEM: unnamed_0 = 148;
pub const _SC_TRACE_EVENT_FILTER: unnamed_0 = 182;
pub type UTIL_time_t = timespec;
pub const _SC_C_LANG_SUPPORT_R: unnamed_0 = 136;
pub const _SC_ULONG_MAX: unnamed_0 = 117;
pub const _SC_AIO_MAX: unnamed_0 = 24;
pub const _SC_FIFO: unnamed_0 = 144;
pub const _SC_BARRIERS: unnamed_0 = 133;
pub const _SC_LEVEL3_CACHE_SIZE: unnamed_0 = 194;
pub const _SC_SIGNALS: unnamed_0 = 158;
pub const _SC_MESSAGE_PASSING: unnamed_0 = 20;
pub const _SC_AIO_LISTIO_MAX: unnamed_0 = 23;
pub const ZSTD_lazy: ZSTD_strategy = 4;
pub const _SC_XOPEN_XPG2: unnamed_0 = 98;
pub const _SC_PAGESIZE: unnamed_0 = 30;
pub const _SC_XOPEN_VERSION: unnamed_0 = 89;
pub const _SC_MEMORY_PROTECTION: unnamed_0 = 19;
pub const _SC_XBS5_ILP32_OFF32: unnamed_0 = 125;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: unnamed_0 = 248;
pub const _SC_TZNAME_MAX: unnamed_0 = 6;
pub const _SC_HOST_NAME_MAX: unnamed_0 = 180;
pub type uint64_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
}
pub const _SC_TRACE_EVENT_NAME_MAX: unnamed_0 = 242;
pub const _SC_LONG_BIT: unnamed_0 = 106;
pub const _SC_ARG_MAX: unnamed_0 = 0;
pub const _SC_AVPHYS_PAGES: unnamed_0 = 86;
pub const _SC_THREADS: unnamed_0 = 67;
pub const _SC_PII_SOCKET: unnamed_0 = 55;
pub const _SC_2_LOCALEDEF: unnamed_0 = 52;
pub const _SC_BASE: unnamed_0 = 134;
pub const _SC_SHARED_MEMORY_OBJECTS: unnamed_0 = 22;
pub const _SC_2_PBS_CHECKPOINT: unnamed_0 = 175;
pub const _SC_MAPPED_FILES: unnamed_0 = 16;
pub const _SC_PII_OSI_CLTS: unnamed_0 = 64;
pub type uint16_t = libc::c_ushort;
pub const _SC_THREAD_PRIO_INHERIT: unnamed_0 = 80;
pub const _SC_DEVICE_SPECIFIC_R: unnamed_0 = 142;
pub const _SC_V6_LP64_OFF64: unnamed_0 = 178;
pub const _SC_SSIZE_MAX: unnamed_0 = 110;
pub type __blkcnt_t = libc::c_long;
pub const FIO_xzCompression: FIO_compressionType_t = 2;
pub const _SC_XOPEN_ENH_I18N: unnamed_0 = 93;
pub const _SC_NETWORKING: unnamed_0 = 152;
pub const _SC_UINT_MAX: unnamed_0 = 116;
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
pub type stat_t = stat;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
pub const _SC_IPV6: unnamed_0 = 235;
pub const _SC_PHYS_PAGES: unnamed_0 = 85;
pub const zom_test: zstd_operation_mode = 2;
pub type U32 = uint32_t;
pub const _SC_VERSION: unnamed_0 = 29;
pub const _SC_2_FORT_DEV: unnamed_0 = 49;
pub const _SC_PII: unnamed_0 = 53;
pub const _SC_NPROCESSORS_ONLN: unnamed_0 = 84;
pub const _SC_SPIN_LOCKS: unnamed_0 = 154;
pub const _SC_LEVEL1_DCACHE_LINESIZE: unnamed_0 = 190;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const _SC_THREAD_PRIO_PROTECT: unnamed_0 = 81;
pub const _SC_XOPEN_UNIX: unnamed_0 = 91;
pub const _SC_RAW_SOCKETS: unnamed_0 = 236;
pub const zom_compress: zstd_operation_mode = 0;
pub const _SC_MEMLOCK_RANGE: unnamed_0 = 18;
pub const _SC_UIO_MAXIOV: unnamed_0 = 60;
pub const zom_bench: zstd_operation_mode = 3;
pub const _SC_UCHAR_MAX: unnamed_0 = 115;
pub const _SC_V6_ILP32_OFFBIG: unnamed_0 = 177;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
pub type __nlink_t = libc::c_ulong;
pub const _SC_LINE_MAX: unnamed_0 = 43;
pub const _SC_PII_OSI_COTS: unnamed_0 = 63;
pub const _SC_NL_LANGMAX: unnamed_0 = 120;
pub const _SC_PASS_MAX: unnamed_0 = 88;
pub const _SC_TIMERS: unnamed_0 = 11;
pub const _SC_TRACE_LOG: unnamed_0 = 184;
pub type uint32_t = libc::c_uint;
pub const _SC_SHRT_MAX: unnamed_0 = 113;
pub const _SC_LEVEL1_DCACHE_SIZE: unnamed_0 = 188;
pub const _SC_2_PBS_ACCOUNTING: unnamed_0 = 169;
pub type unnamed_0 = libc::c_uint;
pub type __syscall_slong_t = libc::c_long;
pub const _SC_LEVEL3_CACHE_LINESIZE: unnamed_0 = 196;
pub const _SC_2_SW_DEV: unnamed_0 = 51;
pub const _SC_AIO_PRIO_DELTA_MAX: unnamed_0 = 25;
pub const _SC_XOPEN_REALTIME: unnamed_0 = 130;
pub const _SC_THREAD_CPUTIME: unnamed_0 = 139;
pub const _SC_PIPE: unnamed_0 = 145;
pub const _SC_TRACE_NAME_MAX: unnamed_0 = 243;
pub const _SC_REALTIME_SIGNALS: unnamed_0 = 9;
pub const _SC_PII_OSI: unnamed_0 = 57;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
pub const _SC_CHAR_BIT: unnamed_0 = 101;
pub type __time_t = libc::c_long;
pub const _SC_TIMER_MAX: unnamed_0 = 35;
pub const _SC_TRACE_INHERIT: unnamed_0 = 183;
pub const _SC_XOPEN_XPG3: unnamed_0 = 99;
pub const _SC_NZERO: unnamed_0 = 109;
pub const _SC_V6_ILP32_OFF32: unnamed_0 = 176;
pub const ZSTD_btopt: ZSTD_strategy = 7;
pub const _SC_FILE_ATTRIBUTES: unnamed_0 = 146;
pub const _SC_LEVEL1_ICACHE_SIZE: unnamed_0 = 185;
pub const _SC_TRACE_USER_EVENT_MAX: unnamed_0 = 245;
pub const _SC_PRIORITIZED_IO: unnamed_0 = 13;
pub const _SC_OPEN_MAX: unnamed_0 = 4;
pub const _SC_XOPEN_STREAMS: unnamed_0 = 246;
pub type __ino_t = libc::c_ulong;
pub const _SC_V7_ILP32_OFF32: unnamed_0 = 237;
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub const _SC_PII_INTERNET_STREAM: unnamed_0 = 61;
pub const _SC_LEVEL4_CACHE_ASSOC: unnamed_0 = 198;
pub const _SC_WORD_BIT: unnamed_0 = 107;
pub const _SC_LEVEL4_CACHE_LINESIZE: unnamed_0 = 199;
pub const _SC_SEM_VALUE_MAX: unnamed_0 = 33;
pub const _SC_XOPEN_CRYPT: unnamed_0 = 92;
pub const _SC_CHAR_MIN: unnamed_0 = 103;
pub type U64 = uint64_t;
pub const _SC_RTSIG_MAX: unnamed_0 = 31;
pub const _SC_BC_DIM_MAX: unnamed_0 = 37;
pub type __clockid_t = libc::c_int;
pub const _SC_DELAYTIMER_MAX: unnamed_0 = 26;
pub type __off64_t = libc::c_long;
pub const _SC_ASYNCHRONOUS_IO: unnamed_0 = 12;
pub const _SC_NL_MSGMAX: unnamed_0 = 121;
pub const _SC_USER_GROUPS: unnamed_0 = 166;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: unnamed_0 = 247;
pub const _SC_BC_STRING_MAX: unnamed_0 = 39;
pub const _SC_LEVEL4_CACHE_SIZE: unnamed_0 = 197;
pub type __gid_t = libc::c_uint;
pub const _SC_TTY_NAME_MAX: unnamed_0 = 72;
pub const _SC_PII_OSI_M: unnamed_0 = 65;
pub type __syscall_ulong_t = libc::c_ulong;
pub const _SC_THREAD_ATTR_STACKSIZE: unnamed_0 = 78;
pub const _SC_XOPEN_XPG4: unnamed_0 = 100;
pub const _SC_LEVEL2_CACHE_ASSOC: unnamed_0 = 192;
pub const _SC_SPORADIC_SERVER: unnamed_0 = 160;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
pub type __off_t = libc::c_long;
pub const _SC_SIGQUEUE_MAX: unnamed_0 = 34;
pub const _SC_GETPW_R_SIZE_MAX: unnamed_0 = 70;
pub const _SC_XOPEN_LEGACY: unnamed_0 = 129;
pub type FILE = _IO_FILE;
pub const _SC_XBS5_LP64_OFF64: unnamed_0 = 127;
pub const _SC_DEVICE_IO: unnamed_0 = 140;
pub const _SC_XOPEN_REALTIME_THREADS: unnamed_0 = 131;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const _SC_REGEXP: unnamed_0 = 155;
pub type size_t = libc::c_ulong;
pub const _SC_MQ_PRIO_MAX: unnamed_0 = 28;
pub const _SC_LEVEL2_CACHE_LINESIZE: unnamed_0 = 193;
pub const _SC_XBS5_ILP32_OFFBIG: unnamed_0 = 126;
pub const _SC_THREAD_SPORADIC_SERVER: unnamed_0 = 161;
pub const _SC_BC_SCALE_MAX: unnamed_0 = 38;
pub const _SC_ADVISORY_INFO: unnamed_0 = 132;
pub const _SC_2_PBS_MESSAGE: unnamed_0 = 171;
pub const _SC_THREAD_ATTR_STACKADDR: unnamed_0 = 77;
pub const _SC_SAVED_IDS: unnamed_0 = 8;
pub const FIO_gzipCompression: FIO_compressionType_t = 1;
pub const _SC_LEVEL1_ICACHE_ASSOC: unnamed_0 = 186;
pub const _SC_NL_TEXTMAX: unnamed_0 = 124;
pub const _SC_REGEX_VERSION: unnamed_0 = 156;
pub const _SC_THREAD_STACK_MIN: unnamed_0 = 75;
pub const _SC_LOGIN_NAME_MAX: unnamed_0 = 71;
pub const _SC_CLK_TCK: unnamed_0 = 2;
pub const _SC_EQUIV_CLASS_MAX: unnamed_0 = 41;
pub const _SC_JOB_CONTROL: unnamed_0 = 7;
pub const _SC_LEVEL1_DCACHE_ASSOC: unnamed_0 = 189;
pub const _SC_USHRT_MAX: unnamed_0 = 118;
pub type FIO_compressionType_t = libc::c_uint;
pub const _SC_2_VERSION: unnamed_0 = 46;
pub const _SC_LEVEL1_ICACHE_LINESIZE: unnamed_0 = 187;
pub const _SC_STREAMS: unnamed_0 = 174;
pub const _SC_MB_LEN_MAX: unnamed_0 = 108;
pub const _SC_PII_INTERNET_DGRAM: unnamed_0 = 62;
pub const _SC_THREAD_PRIORITY_SCHEDULING: unnamed_0 = 79;
pub const _SC_SYSTEM_DATABASE: unnamed_0 = 162;
pub const _SC_PII_INTERNET: unnamed_0 = 56;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub const _SC_MEMLOCK: unnamed_0 = 17;
pub const _SC_C_LANG_SUPPORT: unnamed_0 = 135;
pub const _SC_XOPEN_XCU_VERSION: unnamed_0 = 90;
pub const _SC_COLL_WEIGHTS_MAX: unnamed_0 = 40;
pub const _SC_IOV_MAX: unnamed_0 = 60;
pub const _SC_SCHAR_MAX: unnamed_0 = 111;
pub const _SC_SHELL: unnamed_0 = 157;
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
pub type ptrdiff_t = libc::c_long;
pub type _IO_lock_t = ();
pub const _SC_STREAM_MAX: unnamed_0 = 5;
pub const zom_train: zstd_operation_mode = 4;
pub type uint8_t = libc::c_uchar;
pub const _SC_2_C_VERSION: unnamed_0 = 96;
pub const _SC_THREAD_KEYS_MAX: unnamed_0 = 74;
pub type zstd_operation_mode = libc::c_uint;
pub const _SC_SINGLE_PROCESS: unnamed_0 = 151;
pub const MEM_static_assert: unnamed = 1;
pub const FIO_lzmaCompression: FIO_compressionType_t = 3;
pub const _SC_FSYNC: unnamed_0 = 15;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    u: U32,
    c: [BYTE; 4],
}
pub const zom_list: zstd_operation_mode = 5;
pub const _SC_CHILD_MAX: unnamed_0 = 1;
pub type __mode_t = libc::c_uint;
pub const FIO_lz4Compression: FIO_compressionType_t = 4;
pub const _SC_CPUTIME: unnamed_0 = 138;
pub const _SC_NL_SETMAX: unnamed_0 = 123;
pub const _SC_EXPR_NEST_MAX: unnamed_0 = 42;
pub const _SC_PRIORITY_SCHEDULING: unnamed_0 = 10;
pub const _SC_BC_BASE_MAX: unnamed_0 = 36;
pub const _SC_TIMEOUTS: unnamed_0 = 164;
pub const _SC_CHAR_MAX: unnamed_0 = 102;
pub const ZSTD_fast: ZSTD_strategy = 1;
pub const _SC_SEM_NSEMS_MAX: unnamed_0 = 32;
pub const _SC_MULTI_PROCESS: unnamed_0 = 150;
pub const _SC_2_FORT_RUN: unnamed_0 = 50;
pub const _SC_CHARCLASS_NAME_MAX: unnamed_0 = 45;
pub const _SC_NL_NMAX: unnamed_0 = 122;
pub const _SC_2_CHAR_TERM: unnamed_0 = 95;
pub type ZSTD_strategy = libc::c_uint;
pub const _SC_SYNCHRONIZED_IO: unnamed_0 = 14;
pub const _SC_CLOCK_SELECTION: unnamed_0 = 137;
pub const _SC_FD_MGMT: unnamed_0 = 143;
pub const _SC_T_IOV_MAX: unnamed_0 = 66;
pub const _SC_THREAD_PROCESS_SHARED: unnamed_0 = 82;
pub const _SC_NGROUPS_MAX: unnamed_0 = 3;
pub const _SC_SCHAR_MIN: unnamed_0 = 112;
pub type __dev_t = libc::c_ulong;
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
pub const _SC_TRACE_SYS_MAX: unnamed_0 = 244;
pub const _SC_V6_LPBIG_OFFBIG: unnamed_0 = 179;
pub const _SC_2_C_BIND: unnamed_0 = 47;
pub const _SC_SS_REPL_MAX: unnamed_0 = 241;
pub const _SC_ATEXIT_MAX: unnamed_0 = 87;
pub const _SC_XBS5_LPBIG_OFFBIG: unnamed_0 = 128;
pub const _SC_V7_LP64_OFF64: unnamed_0 = 239;
pub const _SC_FILE_LOCKING: unnamed_0 = 147;
pub const _SC_MQ_OPEN_MAX: unnamed_0 = 27;
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
static mut g_defaultDictName: *const libc::c_char =
    unsafe {
        [100, 105, 99, 116, 105, 111, 110, 97, 114, 121, 0].as_ptr() as
            *const _
    };
static mut g_defaultMaxDictSize: libc::c_uint =
    unsafe { (110i32 * (1i32 << 10i32)) as libc::c_uint };
static mut g_defaultDictCLevel: libc::c_int = unsafe { 3i32 };
static mut g_defaultSelectivityLevel: libc::c_uint =
    unsafe { 9i32 as libc::c_uint };
static mut g_defaultMaxWindowLog: libc::c_uint =
    unsafe { 27i32 as libc::c_uint };
static mut g_overlapLog: U32 = unsafe { 9999i32 as U32 };
static mut g_ldmHashLog: U32 = unsafe { 0i32 as U32 };
static mut g_ldmMinMatch: U32 = unsafe { 0i32 as U32 };
static mut g_ldmHashEveryLog: U32 = unsafe { 9999i32 as U32 };
static mut g_ldmBucketSizeLog: U32 = unsafe { 9999i32 as U32 };
static mut g_displayLevel: libc::c_int = unsafe { 2i32 };
static mut g_displayOut: *mut FILE = unsafe { 0 as *const FILE as *mut FILE };
unsafe extern "C" fn usage(mut programName: *const libc::c_char)
 -> libc::c_int {
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 10],
                                      &mut [libc::c_char; 10]>(b"Usage : \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 38],
                                      &mut [libc::c_char; 38]>(b"      %s [args] [FILE(s)] [-o file] \n\x00")).as_mut_ptr(),
            programName);
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 2],
                                      &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 23],
                                      &mut [libc::c_char; 23]>(b"FILE    : a filename \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 65],
                                      &mut [libc::c_char; 65]>(b"          with no FILE, or when FILE is - , read standard input\n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 14],
                                      &mut [libc::c_char; 14]>(b"Arguments : \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 52],
                                      &mut [libc::c_char; 52]>(b" -#     : # compression level (1-%d, default: %d) \n\x00")).as_mut_ptr(),
            19i32, 3i32);
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 26],
                                      &mut [libc::c_char; 26]>(b" -d     : decompression \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 37],
                                      &mut [libc::c_char; 37]>(b" -D file: use `file` as Dictionary \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 61],
                                      &mut [libc::c_char; 61]>(b" -o file: result stored into `file` (only if 1 input file) \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 70],
                                      &mut [libc::c_char; 70]>(b" -f     : overwrite output without prompting and (de)compress links \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 66],
                                      &mut [libc::c_char; 66]>(b"--rm    : remove source file(s) after successful de/compression \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 46],
                                      &mut [libc::c_char; 46]>(b" -k     : preserve source file(s) (default) \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 44],
                                      &mut [libc::c_char; 44]>(b" -h/-H  : display help/long help and exit \n\x00")).as_mut_ptr());
    return 0i32;
}
unsafe extern "C" fn usage_advanced(mut programName: *const libc::c_char)
 -> libc::c_int {
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 30],
                                      &mut [libc::c_char; 30]>(b"*** %s %i-bits %s, by %s ***\n\x00")).as_mut_ptr(),
            (*::std::mem::transmute::<&[u8; 28],
                                      &mut [libc::c_char; 28]>(b"zstd command line interface\x00")).as_mut_ptr(),
            (::std::mem::size_of::<size_t>() as
                 libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) as
                libc::c_int,
            (*::std::mem::transmute::<&[u8; 7],
                                      &mut [libc::c_char; 7]>(b"v1.3.5\x00")).as_mut_ptr(),
            (*::std::mem::transmute::<&[u8; 12],
                                      &mut [libc::c_char; 12]>(b"Yann Collet\x00")).as_mut_ptr());
    usage(programName);
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 2],
                                      &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 23],
                                      &mut [libc::c_char; 23]>(b"Advanced arguments : \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 44],
                                      &mut [libc::c_char; 44]>(b" -V     : display Version number and exit \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 70],
                                      &mut [libc::c_char; 70]>(b" -v     : verbose mode; specify multiple times to increase verbosity\n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 67],
                                      &mut [libc::c_char; 67]>(b" -q     : suppress warnings; specify twice to suppress errors too\n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 69],
                                      &mut [libc::c_char; 69]>(b" -c     : force write to standard output, even if it is the console\n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 58],
                                      &mut [libc::c_char; 58]>(b" -l     : print information about zstd compressed files \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 68],
                                      &mut [libc::c_char; 68]>(b"--ultra : enable levels beyond %i, up to %i (requires more memory)\n\x00")).as_mut_ptr(),
            19i32, ZSTD_maxCLevel());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 79],
                                      &mut [libc::c_char; 79]>(b"--long[=#]: enable long distance matching with given window log (default: %u)\n\x00")).as_mut_ptr(),
            g_defaultMaxWindowLog);
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 66],
                                      &mut [libc::c_char; 66]>(b"--fast[=#]: switch to ultra fast compression level (default: %u)\n\x00")).as_mut_ptr(),
            1i32);
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 71],
                                      &mut [libc::c_char; 71]>(b"--no-dictID : don\'t write dictID into header (dictionary compression)\n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 52],
                                      &mut [libc::c_char; 52]>(b"--[no-]check : integrity check (default: enabled) \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 47],
                                      &mut [libc::c_char; 47]>(b" -r     : operate recursively on directories \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 63],
                                      &mut [libc::c_char; 63]>(b"--format=zstd : compress files to the .zstd format (default) \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 43],
                                      &mut [libc::c_char; 43]>(b"--test  : test compressed file integrity \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 76],
                                      &mut [libc::c_char; 76]>(b"--[no-]sparse : sparse mode (default: enabled on file, disabled on stdout)\n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 55],
                                      &mut [libc::c_char; 55]>(b" -M#    : Set a memory usage limit for decompression \n\x00")).as_mut_ptr());
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 58],
                                      &mut [libc::c_char; 58]>(b"--      : All arguments after \"--\" are treated as files \n\x00")).as_mut_ptr());
    return 0i32;
}
unsafe extern "C" fn badusage(mut programName: *const libc::c_char)
 -> libc::c_int {
    if g_displayLevel >= 1i32 {
        fprintf(g_displayOut,
                (*::std::mem::transmute::<&[u8; 22],
                                          &mut [libc::c_char; 22]>(b"Incorrect parameters\n\x00")).as_mut_ptr());
    }
    if g_displayLevel >= 2i32 { usage(programName); }
    return 1i32;
}
unsafe extern "C" fn waitEnter() -> () {
    let mut unused: libc::c_int = 0;
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 28],
                                      &mut [libc::c_char; 28]>(b"Press enter to continue...\n\x00")).as_mut_ptr());
    unused = getchar();
}
unsafe extern "C" fn lastNameFromPath(mut path: *const libc::c_char)
 -> *const libc::c_char {
    let mut name: *const libc::c_char = path;
    if !strrchr(name, '/' as i32).is_null() {
        name = strrchr(name, '/' as i32).offset(1isize)
    }
    if !strrchr(name, '\\' as i32).is_null() {
        name = strrchr(name, '\\' as i32).offset(1isize)
    }
    return name;
}
/* ! exeNameMatch() :
    @return : a non-zero value if exeName matches test, excluding the extension
   */
unsafe extern "C" fn exeNameMatch(mut exeName: *const libc::c_char,
                                  mut test: *const libc::c_char)
 -> libc::c_int {
    return (0 == strncmp(exeName, test, strlen(test)) &&
                (*exeName.offset(strlen(test) as isize) as libc::c_int ==
                     '\u{0}' as i32 ||
                     *exeName.offset(strlen(test) as isize) as libc::c_int ==
                         '.' as i32)) as libc::c_int;
}
unsafe extern "C" fn errorOut(mut msg: *const libc::c_char) -> () {
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 5],
                                      &mut [libc::c_char; 5]>(b"%s \n\x00")).as_mut_ptr(),
            msg);
    exit(1i32);
}
/* ! readU32FromChar() :
 * @return : unsigned integer value read from input in `char` format.
 *  allows and interprets K, KB, KiB, M, MB and MiB suffix.
 *  Will also modify `*stringPtr`, advancing it to position where it stopped reading.
 *  Note : function will exit() program if digit sequence overflows */
unsafe extern "C" fn readU32FromChar(mut stringPtr: *mut *const libc::c_char)
 -> libc::c_uint {
    let errorMsg: [libc::c_char; 31] =
        *::std::mem::transmute::<&[u8; 31],
                                 &[libc::c_char; 31]>(b"error: numeric value too large\x00");
    let mut result: libc::c_uint = 0i32 as libc::c_uint;
    while **stringPtr as libc::c_int >= '0' as i32 &&
              **stringPtr as libc::c_int <= '9' as i32 {
        let max: libc::c_uint =
            (-1i32 as
                 libc::c_uint).wrapping_div(10i32 as
                                                libc::c_uint).wrapping_sub(1i32
                                                                               as
                                                                               libc::c_uint);
        if result > max { errorOut(errorMsg.as_ptr()); }
        result = result.wrapping_mul(10i32 as libc::c_uint);
        result =
            result.wrapping_add((**stringPtr as libc::c_int - '0' as i32) as
                                    libc::c_uint);
        *stringPtr = (*stringPtr).offset(1isize)
    }
    if **stringPtr as libc::c_int == 'K' as i32 ||
           **stringPtr as libc::c_int == 'M' as i32 {
        let maxK: libc::c_uint = -1i32 as libc::c_uint >> 10i32;
        if result > maxK { errorOut(errorMsg.as_ptr()); }
        result <<= 10i32;
        if **stringPtr as libc::c_int == 'M' as i32 {
            if result > maxK { errorOut(errorMsg.as_ptr()); }
            result <<= 10i32
        }
        *stringPtr = (*stringPtr).offset(1isize);
        if **stringPtr as libc::c_int == 'i' as i32 {
            *stringPtr = (*stringPtr).offset(1isize)
        }
        if **stringPtr as libc::c_int == 'B' as i32 {
            *stringPtr = (*stringPtr).offset(1isize)
        }
    }
    return result;
}
/* * longCommandWArg() :
 *  check if *stringPtr is the same as longCommand.
 *  If yes, @return 1 and advances *stringPtr to the position which immediately follows longCommand.
 * @return 0 and doesn't modify *stringPtr otherwise.
 */
unsafe extern "C" fn longCommandWArg(mut stringPtr: *mut *const libc::c_char,
                                     mut longCommand: *const libc::c_char)
 -> libc::c_uint {
    let comSize: size_t = strlen(longCommand);
    let result: libc::c_int =
        (0 == strncmp(*stringPtr, longCommand, comSize)) as libc::c_int;
    if 0 != result { *stringPtr = (*stringPtr).offset(comSize as isize) }
    return result as libc::c_uint;
}
/* * parseCompressionParameters() :
 *  reads compression parameters from *stringPtr (e.g. "--zstd=wlog=23,clog=23,hlog=22,slog=6,slen=3,tlen=48,strat=6") into *params
 *  @return 1 means that compression parameters were correct
 *  @return 0 in case of malformed parameters
 */
unsafe extern "C" fn parseCompressionParameters(mut stringPtr:
                                                    *const libc::c_char,
                                                mut params:
                                                    *mut ZSTD_compressionParameters)
 -> libc::c_uint {
    loop  {
        if 0 !=
               longCommandWArg(&mut stringPtr as *mut *const libc::c_char,
                               (*::std::mem::transmute::<&[u8; 11],
                                                         &mut [libc::c_char; 11]>(b"windowLog=\x00")).as_mut_ptr())
               ||
               0 !=
                   longCommandWArg(&mut stringPtr as *mut *const libc::c_char,
                                   (*::std::mem::transmute::<&[u8; 6],
                                                             &mut [libc::c_char; 6]>(b"wlog=\x00")).as_mut_ptr())
           {
            (*params).windowLog =
                readU32FromChar(&mut stringPtr as *mut *const libc::c_char);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr as
                                          *mut *const libc::c_char,
                                      (*::std::mem::transmute::<&[u8; 10],
                                                                &mut [libc::c_char; 10]>(b"chainLog=\x00")).as_mut_ptr())
                      ||
                      0 !=
                          longCommandWArg(&mut stringPtr as
                                              *mut *const libc::c_char,
                                          (*::std::mem::transmute::<&[u8; 6],
                                                                    &mut [libc::c_char; 6]>(b"clog=\x00")).as_mut_ptr())
         {
            (*params).chainLog =
                readU32FromChar(&mut stringPtr as *mut *const libc::c_char);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr as
                                          *mut *const libc::c_char,
                                      (*::std::mem::transmute::<&[u8; 9],
                                                                &mut [libc::c_char; 9]>(b"hashLog=\x00")).as_mut_ptr())
                      ||
                      0 !=
                          longCommandWArg(&mut stringPtr as
                                              *mut *const libc::c_char,
                                          (*::std::mem::transmute::<&[u8; 6],
                                                                    &mut [libc::c_char; 6]>(b"hlog=\x00")).as_mut_ptr())
         {
            (*params).hashLog =
                readU32FromChar(&mut stringPtr as *mut *const libc::c_char);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr as
                                          *mut *const libc::c_char,
                                      (*::std::mem::transmute::<&[u8; 11],
                                                                &mut [libc::c_char; 11]>(b"searchLog=\x00")).as_mut_ptr())
                      ||
                      0 !=
                          longCommandWArg(&mut stringPtr as
                                              *mut *const libc::c_char,
                                          (*::std::mem::transmute::<&[u8; 6],
                                                                    &mut [libc::c_char; 6]>(b"slog=\x00")).as_mut_ptr())
         {
            (*params).searchLog =
                readU32FromChar(&mut stringPtr as *mut *const libc::c_char);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr as
                                          *mut *const libc::c_char,
                                      (*::std::mem::transmute::<&[u8; 14],
                                                                &mut [libc::c_char; 14]>(b"searchLength=\x00")).as_mut_ptr())
                      ||
                      0 !=
                          longCommandWArg(&mut stringPtr as
                                              *mut *const libc::c_char,
                                          (*::std::mem::transmute::<&[u8; 6],
                                                                    &mut [libc::c_char; 6]>(b"slen=\x00")).as_mut_ptr())
         {
            (*params).searchLength =
                readU32FromChar(&mut stringPtr as *mut *const libc::c_char);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr as
                                          *mut *const libc::c_char,
                                      (*::std::mem::transmute::<&[u8; 14],
                                                                &mut [libc::c_char; 14]>(b"targetLength=\x00")).as_mut_ptr())
                      ||
                      0 !=
                          longCommandWArg(&mut stringPtr as
                                              *mut *const libc::c_char,
                                          (*::std::mem::transmute::<&[u8; 6],
                                                                    &mut [libc::c_char; 6]>(b"tlen=\x00")).as_mut_ptr())
         {
            (*params).targetLength =
                readU32FromChar(&mut stringPtr as *mut *const libc::c_char);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr as
                                          *mut *const libc::c_char,
                                      (*::std::mem::transmute::<&[u8; 10],
                                                                &mut [libc::c_char; 10]>(b"strategy=\x00")).as_mut_ptr())
                      ||
                      0 !=
                          longCommandWArg(&mut stringPtr as
                                              *mut *const libc::c_char,
                                          (*::std::mem::transmute::<&[u8; 7],
                                                                    &mut [libc::c_char; 7]>(b"strat=\x00")).as_mut_ptr())
         {
            (*params).strategy =
                readU32FromChar(&mut stringPtr as *mut *const libc::c_char) as
                    ZSTD_strategy;
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr as
                                          *mut *const libc::c_char,
                                      (*::std::mem::transmute::<&[u8; 12],
                                                                &mut [libc::c_char; 12]>(b"overlapLog=\x00")).as_mut_ptr())
                      ||
                      0 !=
                          longCommandWArg(&mut stringPtr as
                                              *mut *const libc::c_char,
                                          (*::std::mem::transmute::<&[u8; 7],
                                                                    &mut [libc::c_char; 7]>(b"ovlog=\x00")).as_mut_ptr())
         {
            g_overlapLog =
                readU32FromChar(&mut stringPtr as *mut *const libc::c_char);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr as
                                          *mut *const libc::c_char,
                                      (*::std::mem::transmute::<&[u8; 12],
                                                                &mut [libc::c_char; 12]>(b"ldmHashLog=\x00")).as_mut_ptr())
                      ||
                      0 !=
                          longCommandWArg(&mut stringPtr as
                                              *mut *const libc::c_char,
                                          (*::std::mem::transmute::<&[u8; 9],
                                                                    &mut [libc::c_char; 9]>(b"ldmhlog=\x00")).as_mut_ptr())
         {
            g_ldmHashLog =
                readU32FromChar(&mut stringPtr as *mut *const libc::c_char);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr as
                                          *mut *const libc::c_char,
                                      (*::std::mem::transmute::<&[u8; 17],
                                                                &mut [libc::c_char; 17]>(b"ldmSearchLength=\x00")).as_mut_ptr())
                      ||
                      0 !=
                          longCommandWArg(&mut stringPtr as
                                              *mut *const libc::c_char,
                                          (*::std::mem::transmute::<&[u8; 9],
                                                                    &mut [libc::c_char; 9]>(b"ldmslen=\x00")).as_mut_ptr())
         {
            g_ldmMinMatch =
                readU32FromChar(&mut stringPtr as *mut *const libc::c_char);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr as
                                          *mut *const libc::c_char,
                                      (*::std::mem::transmute::<&[u8; 18],
                                                                &mut [libc::c_char; 18]>(b"ldmBucketSizeLog=\x00")).as_mut_ptr())
                      ||
                      0 !=
                          longCommandWArg(&mut stringPtr as
                                              *mut *const libc::c_char,
                                          (*::std::mem::transmute::<&[u8; 9],
                                                                    &mut [libc::c_char; 9]>(b"ldmblog=\x00")).as_mut_ptr())
         {
            g_ldmBucketSizeLog =
                readU32FromChar(&mut stringPtr as *mut *const libc::c_char);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr as
                                          *mut *const libc::c_char,
                                      (*::std::mem::transmute::<&[u8; 17],
                                                                &mut [libc::c_char; 17]>(b"ldmHashEveryLog=\x00")).as_mut_ptr())
                      ||
                      0 !=
                          longCommandWArg(&mut stringPtr as
                                              *mut *const libc::c_char,
                                          (*::std::mem::transmute::<&[u8; 11],
                                                                    &mut [libc::c_char; 11]>(b"ldmhevery=\x00")).as_mut_ptr())
         {
            g_ldmHashEveryLog =
                readU32FromChar(&mut stringPtr as *mut *const libc::c_char);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else {
            if g_displayLevel >= 4i32 {
                fprintf(g_displayOut,
                        (*::std::mem::transmute::<&[u8; 32],
                                                  &mut [libc::c_char; 32]>(b"invalid compression parameter \n\x00")).as_mut_ptr());
            }
            return 0i32 as libc::c_uint
        }
    }
    if g_displayLevel >= 4i32 {
        fprintf(g_displayOut,
                (*::std::mem::transmute::<&[u8; 54],
                                          &mut [libc::c_char; 54]>(b"windowLog=%d, chainLog=%d, hashLog=%d, searchLog=%d \n\x00")).as_mut_ptr(),
                (*params).windowLog, (*params).chainLog, (*params).hashLog,
                (*params).searchLog);
    }
    if g_displayLevel >= 4i32 {
        fprintf(g_displayOut,
                (*::std::mem::transmute::<&[u8; 48],
                                          &mut [libc::c_char; 48]>(b"searchLength=%d, targetLength=%d, strategy=%d \n\x00")).as_mut_ptr(),
                (*params).searchLength, (*params).targetLength,
                (*params).strategy as libc::c_uint);
    }
    if *stringPtr.offset(0isize) as libc::c_int != 0i32 {
        return 0i32 as libc::c_uint
    } else { return 1i32 as libc::c_uint };
}
unsafe extern "C" fn printVersion() -> () {
    fprintf(g_displayOut,
            (*::std::mem::transmute::<&[u8; 30],
                                      &mut [libc::c_char; 30]>(b"*** %s %i-bits %s, by %s ***\n\x00")).as_mut_ptr(),
            (*::std::mem::transmute::<&[u8; 28],
                                      &mut [libc::c_char; 28]>(b"zstd command line interface\x00")).as_mut_ptr(),
            (::std::mem::size_of::<size_t>() as
                 libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) as
                libc::c_int,
            (*::std::mem::transmute::<&[u8; 7],
                                      &mut [libc::c_char; 7]>(b"v1.3.5\x00")).as_mut_ptr(),
            (*::std::mem::transmute::<&[u8; 12],
                                      &mut [libc::c_char; 12]>(b"Yann Collet\x00")).as_mut_ptr());
    if g_displayLevel >= 3i32 {
        fprintf(g_displayOut,
                (*::std::mem::transmute::<&[u8; 19],
                                          &mut [libc::c_char; 19]>(b"*** supports: zstd\x00")).as_mut_ptr());
    }
    if g_displayLevel >= 3i32 {
        fprintf(g_displayOut,
                (*::std::mem::transmute::<&[u8; 2],
                                          &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
    }
    if g_displayLevel >= 4i32 {
        fprintf(g_displayOut,
                (*::std::mem::transmute::<&[u8; 31],
                                          &mut [libc::c_char; 31]>(b"_POSIX_C_SOURCE defined: %ldL\n\x00")).as_mut_ptr(),
                200112i64);
    }
    if g_displayLevel >= 4i32 {
        fprintf(g_displayOut,
                (*::std::mem::transmute::<&[u8; 31],
                                          &mut [libc::c_char; 31]>(b"_POSIX_VERSION defined: %ldL \n\x00")).as_mut_ptr(),
                200112i64);
    }
    if g_displayLevel >= 4i32 {
        fprintf(g_displayOut,
                (*::std::mem::transmute::<&[u8; 38],
                                          &mut [libc::c_char; 38]>(b"PLATFORM_POSIX_VERSION defined: %ldL\n\x00")).as_mut_ptr(),
                200112i64);
    };
}
#[export_name = "main"]
pub unsafe extern "C" fn main_0(mut argCount: libc::c_int,
                                mut argv: *mut *const libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut argNb: libc::c_int = 0;
    let mut followLinks: libc::c_int = 0i32;
    let mut forceStdout: libc::c_int = 0i32;
    let mut lastCommand: libc::c_int = 0i32;
    let mut ldmFlag: libc::c_int = 0i32;
    let mut main_pause: libc::c_int = 0i32;
    let mut nbWorkers: libc::c_int = 0i32;
    let mut nextArgumentIsOutFileName: libc::c_int = 0i32;
    let mut nextArgumentIsMaxDict: libc::c_int = 0i32;
    let mut nextArgumentIsDictID: libc::c_int = 0i32;
    let mut nextArgumentsAreFiles: libc::c_int = 0i32;
    let mut nextEntryIsDictionary: libc::c_int = 0i32;
    let mut operationResult: libc::c_int = 0i32;
    let mut separateFiles: libc::c_int = 0i32;
    let mut setRealTimePrio: libc::c_int = 0i32;
    let mut singleThread: libc::c_int = 0i32;
    let mut ultra: libc::c_int = 0i32;
    let mut bench_nbSeconds: libc::c_uint = 3i32 as libc::c_uint;
    let mut blockSize: size_t = 0i32 as size_t;
    let mut operation: zstd_operation_mode = zom_compress;
    let mut compressionParams: ZSTD_compressionParameters =
        ZSTD_compressionParameters{windowLog: 0,
                                   chainLog: 0,
                                   hashLog: 0,
                                   searchLog: 0,
                                   searchLength: 0,
                                   targetLength: 0,
                                   strategy: 0 as ZSTD_strategy,};
    let mut cLevel: libc::c_int = 3i32;
    let mut cLevelLast: libc::c_int = -1000000000i32;
    let mut recursive: libc::c_uint = 0i32 as libc::c_uint;
    let mut memLimit: libc::c_uint = 0i32 as libc::c_uint;
    let mut filenameTable: *mut *const libc::c_char =
        malloc((argCount as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*const libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *const libc::c_char;
    let mut filenameIdx: libc::c_uint = 0i32 as libc::c_uint;
    let mut programName: *const libc::c_char = *argv.offset(0isize);
    let mut outFileName: *const libc::c_char = 0 as *const libc::c_char;
    let mut dictFileName: *const libc::c_char = 0 as *const libc::c_char;
    let mut suffix: *const libc::c_char =
        (*::std::mem::transmute::<&[u8; 5],
                                  &mut [libc::c_char; 5]>(b".zst\x00")).as_mut_ptr();
    let mut maxDictSize: libc::c_uint = g_defaultMaxDictSize;
    let mut dictID: libc::c_uint = 0i32 as libc::c_uint;
    let mut dictCLevel: libc::c_int = g_defaultDictCLevel;
    let mut dictSelect: libc::c_uint = g_defaultSelectivityLevel;
    let mut extendedFileList: *mut *const libc::c_char =
        0 as *mut *const libc::c_char;
    let mut fileNamesBuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fileNamesNb: libc::c_uint = 0;
    if filenameTable.is_null() {
        fprintf(g_displayOut,
                (*::std::mem::transmute::<&[u8; 11],
                                          &mut [libc::c_char; 11]>(b"zstd: %s \n\x00")).as_mut_ptr(),
                strerror(*__errno_location()));
        exit(1i32);
    } else {
        let ref mut fresh1 = *filenameTable.offset(0isize);
        *fresh1 =
            (*::std::mem::transmute::<&[u8; 10],
                                      &mut [libc::c_char; 10]>(b"/*stdin*\\\x00")).as_mut_ptr();
        g_displayOut = stderr;
        programName = lastNameFromPath(programName);
        if 0 !=
               exeNameMatch(programName,
                            (*::std::mem::transmute::<&[u8; 7],
                                                      &mut [libc::c_char; 7]>(b"zstdmt\x00")).as_mut_ptr())
           {
            nbWorkers = 0i32
        }
        if 0 !=
               exeNameMatch(programName,
                            (*::std::mem::transmute::<&[u8; 7],
                                                      &mut [libc::c_char; 7]>(b"unzstd\x00")).as_mut_ptr())
           {
            operation = zom_decompress
        }
        if 0 !=
               exeNameMatch(programName,
                            (*::std::mem::transmute::<&[u8; 8],
                                                      &mut [libc::c_char; 8]>(b"zstdcat\x00")).as_mut_ptr())
           {
            operation = zom_decompress;
            forceStdout = 1i32;
            FIO_overwriteMode();
            outFileName =
                (*::std::mem::transmute::<&[u8; 11],
                                          &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr();
            g_displayLevel = 1i32
        }
        if 0 !=
               exeNameMatch(programName,
                            (*::std::mem::transmute::<&[u8; 5],
                                                      &mut [libc::c_char; 5]>(b"zcat\x00")).as_mut_ptr())
           {
            operation = zom_decompress;
            forceStdout = 1i32;
            FIO_overwriteMode();
            outFileName =
                (*::std::mem::transmute::<&[u8; 11],
                                          &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr();
            g_displayLevel = 1i32
        }
        if 0 !=
               exeNameMatch(programName,
                            (*::std::mem::transmute::<&[u8; 5],
                                                      &mut [libc::c_char; 5]>(b"gzip\x00")).as_mut_ptr())
           {
            suffix =
                (*::std::mem::transmute::<&[u8; 4],
                                          &mut [libc::c_char; 4]>(b".gz\x00")).as_mut_ptr();
            FIO_setCompressionType(FIO_gzipCompression);
            FIO_setRemoveSrcFile(1i32 as libc::c_uint);
        }
        if 0 !=
               exeNameMatch(programName,
                            (*::std::mem::transmute::<&[u8; 7],
                                                      &mut [libc::c_char; 7]>(b"gunzip\x00")).as_mut_ptr())
           {
            operation = zom_decompress;
            FIO_setRemoveSrcFile(1i32 as libc::c_uint);
        }
        if 0 !=
               exeNameMatch(programName,
                            (*::std::mem::transmute::<&[u8; 6],
                                                      &mut [libc::c_char; 6]>(b"gzcat\x00")).as_mut_ptr())
           {
            operation = zom_decompress;
            forceStdout = 1i32;
            FIO_overwriteMode();
            outFileName =
                (*::std::mem::transmute::<&[u8; 11],
                                          &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr();
            g_displayLevel = 1i32
        }
        if 0 !=
               exeNameMatch(programName,
                            (*::std::mem::transmute::<&[u8; 5],
                                                      &mut [libc::c_char; 5]>(b"lzma\x00")).as_mut_ptr())
           {
            suffix =
                (*::std::mem::transmute::<&[u8; 6],
                                          &mut [libc::c_char; 6]>(b".lzma\x00")).as_mut_ptr();
            FIO_setCompressionType(FIO_lzmaCompression);
            FIO_setRemoveSrcFile(1i32 as libc::c_uint);
        }
        if 0 !=
               exeNameMatch(programName,
                            (*::std::mem::transmute::<&[u8; 7],
                                                      &mut [libc::c_char; 7]>(b"unlzma\x00")).as_mut_ptr())
           {
            operation = zom_decompress;
            FIO_setCompressionType(FIO_lzmaCompression);
            FIO_setRemoveSrcFile(1i32 as libc::c_uint);
        }
        if 0 !=
               exeNameMatch(programName,
                            (*::std::mem::transmute::<&[u8; 3],
                                                      &mut [libc::c_char; 3]>(b"xz\x00")).as_mut_ptr())
           {
            suffix =
                (*::std::mem::transmute::<&[u8; 4],
                                          &mut [libc::c_char; 4]>(b".xz\x00")).as_mut_ptr();
            FIO_setCompressionType(FIO_xzCompression);
            FIO_setRemoveSrcFile(1i32 as libc::c_uint);
        }
        if 0 !=
               exeNameMatch(programName,
                            (*::std::mem::transmute::<&[u8; 5],
                                                      &mut [libc::c_char; 5]>(b"unxz\x00")).as_mut_ptr())
           {
            operation = zom_decompress;
            FIO_setCompressionType(FIO_xzCompression);
            FIO_setRemoveSrcFile(1i32 as libc::c_uint);
        }
        if 0 !=
               exeNameMatch(programName,
                            (*::std::mem::transmute::<&[u8; 4],
                                                      &mut [libc::c_char; 4]>(b"lz4\x00")).as_mut_ptr())
           {
            suffix =
                (*::std::mem::transmute::<&[u8; 5],
                                          &mut [libc::c_char; 5]>(b".lz4\x00")).as_mut_ptr();
            FIO_setCompressionType(FIO_lz4Compression);
        }
        if 0 !=
               exeNameMatch(programName,
                            (*::std::mem::transmute::<&[u8; 6],
                                                      &mut [libc::c_char; 6]>(b"unlz4\x00")).as_mut_ptr())
           {
            operation = zom_decompress;
            FIO_setCompressionType(FIO_lz4Compression);
        }
        memset(&mut compressionParams as *mut ZSTD_compressionParameters as
                   *mut libc::c_void, 0i32,
               ::std::mem::size_of::<ZSTD_compressionParameters>() as
                   libc::c_ulong);
        argNb = 1i32;
        's_143:
            loop  {
                if !(argNb < argCount) {
                    current_block = 16924917904204750491;
                    break ;
                }
                let mut argument: *const libc::c_char =
                    *argv.offset(argNb as isize);
                if !argument.is_null() {
                    if nextArgumentsAreFiles == 0i32 {
                        if 0 ==
                               strcmp(argument,
                                      (*::std::mem::transmute::<&[u8; 2],
                                                                &mut [libc::c_char; 2]>(b"-\x00")).as_mut_ptr())
                           {
                            if 0 == filenameIdx {
                                filenameIdx = 1i32 as libc::c_uint;
                                let ref mut fresh2 =
                                    *filenameTable.offset(0isize);
                                *fresh2 =
                                    (*::std::mem::transmute::<&[u8; 10],
                                                              &mut [libc::c_char; 10]>(b"/*stdin*\\\x00")).as_mut_ptr();
                                outFileName =
                                    (*::std::mem::transmute::<&[u8; 11],
                                                              &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr();
                                g_displayLevel -=
                                    (g_displayLevel == 2i32) as libc::c_int;
                                current_block = 14359455889292382949;
                            } else { current_block = 2569451025026770673; }
                        } else { current_block = 2569451025026770673; }
                        match current_block {
                            14359455889292382949 => { }
                            _ => {
                                if *argument.offset(0isize) as libc::c_int ==
                                       '-' as i32 {
                                    if *argument.offset(1isize) as libc::c_int
                                           == '-' as i32 {
                                        if 0 ==
                                               strcmp(argument,
                                                      (*::std::mem::transmute::<&[u8; 3],
                                                                                &mut [libc::c_char; 3]>(b"--\x00")).as_mut_ptr())
                                           {
                                            nextArgumentsAreFiles = 1i32;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 7],
                                                                                       &mut [libc::c_char; 7]>(b"--list\x00")).as_mut_ptr())
                                         {
                                            operation = zom_list;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 11],
                                                                                       &mut [libc::c_char; 11]>(b"--compress\x00")).as_mut_ptr())
                                         {
                                            operation = zom_compress;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 13],
                                                                                       &mut [libc::c_char; 13]>(b"--decompress\x00")).as_mut_ptr())
                                         {
                                            operation = zom_decompress;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 13],
                                                                                       &mut [libc::c_char; 13]>(b"--uncompress\x00")).as_mut_ptr())
                                         {
                                            operation = zom_decompress;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 8],
                                                                                       &mut [libc::c_char; 8]>(b"--force\x00")).as_mut_ptr())
                                         {
                                            FIO_overwriteMode();
                                            forceStdout = 1i32;
                                            followLinks = 1i32;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 10],
                                                                                       &mut [libc::c_char; 10]>(b"--version\x00")).as_mut_ptr())
                                         {
                                            g_displayOut = stdout;
                                            fprintf(g_displayOut,
                                                    (*::std::mem::transmute::<&[u8; 30],
                                                                              &mut [libc::c_char; 30]>(b"*** %s %i-bits %s, by %s ***\n\x00")).as_mut_ptr(),
                                                    (*::std::mem::transmute::<&[u8; 28],
                                                                              &mut [libc::c_char; 28]>(b"zstd command line interface\x00")).as_mut_ptr(),
                                                    (::std::mem::size_of::<size_t>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8i32
                                                                                         as
                                                                                         libc::c_ulong)
                                                        as libc::c_int,
                                                    (*::std::mem::transmute::<&[u8; 7],
                                                                              &mut [libc::c_char; 7]>(b"v1.3.5\x00")).as_mut_ptr(),
                                                    (*::std::mem::transmute::<&[u8; 12],
                                                                              &mut [libc::c_char; 12]>(b"Yann Collet\x00")).as_mut_ptr());
                                            operationResult = 0i32;
                                            current_block =
                                                579604380461068407;
                                            break ;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 7],
                                                                                       &mut [libc::c_char; 7]>(b"--help\x00")).as_mut_ptr())
                                         {
                                            g_displayOut = stdout;
                                            operationResult =
                                                usage_advanced(programName);
                                            current_block =
                                                579604380461068407;
                                            break ;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 10],
                                                                                       &mut [libc::c_char; 10]>(b"--verbose\x00")).as_mut_ptr())
                                         {
                                            g_displayLevel += 1;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 8],
                                                                                       &mut [libc::c_char; 8]>(b"--quiet\x00")).as_mut_ptr())
                                         {
                                            g_displayLevel -= 1;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 9],
                                                                                       &mut [libc::c_char; 9]>(b"--stdout\x00")).as_mut_ptr())
                                         {
                                            forceStdout = 1i32;
                                            outFileName =
                                                (*::std::mem::transmute::<&[u8; 11],
                                                                          &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr();
                                            g_displayLevel -=
                                                (g_displayLevel == 2i32) as
                                                    libc::c_int;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 8],
                                                                                       &mut [libc::c_char; 8]>(b"--ultra\x00")).as_mut_ptr())
                                         {
                                            ultra = 1i32;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 8],
                                                                                       &mut [libc::c_char; 8]>(b"--check\x00")).as_mut_ptr())
                                         {
                                            FIO_setChecksumFlag(2i32 as
                                                                    libc::c_uint);
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 11],
                                                                                       &mut [libc::c_char; 11]>(b"--no-check\x00")).as_mut_ptr())
                                         {
                                            FIO_setChecksumFlag(0i32 as
                                                                    libc::c_uint);
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 9],
                                                                                       &mut [libc::c_char; 9]>(b"--sparse\x00")).as_mut_ptr())
                                         {
                                            FIO_setSparseWrite(2i32 as
                                                                   libc::c_uint);
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 12],
                                                                                       &mut [libc::c_char; 12]>(b"--no-sparse\x00")).as_mut_ptr())
                                         {
                                            FIO_setSparseWrite(0i32 as
                                                                   libc::c_uint);
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 7],
                                                                                       &mut [libc::c_char; 7]>(b"--test\x00")).as_mut_ptr())
                                         {
                                            operation = zom_test;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 8],
                                                                                       &mut [libc::c_char; 8]>(b"--train\x00")).as_mut_ptr())
                                         {
                                            operation = zom_train;
                                            outFileName = g_defaultDictName;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 10],
                                                                                       &mut [libc::c_char; 10]>(b"--maxdict\x00")).as_mut_ptr())
                                         {
                                            nextArgumentIsMaxDict = 1i32;
                                            lastCommand = 1i32;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 9],
                                                                                       &mut [libc::c_char; 9]>(b"--dictID\x00")).as_mut_ptr())
                                         {
                                            nextArgumentIsDictID = 1i32;
                                            lastCommand = 1i32;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 12],
                                                                                       &mut [libc::c_char; 12]>(b"--no-dictID\x00")).as_mut_ptr())
                                         {
                                            FIO_setDictIDFlag(0i32 as
                                                                  libc::c_uint);
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 7],
                                                                                       &mut [libc::c_char; 7]>(b"--keep\x00")).as_mut_ptr())
                                         {
                                            FIO_setRemoveSrcFile(0i32 as
                                                                     libc::c_uint);
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 5],
                                                                                       &mut [libc::c_char; 5]>(b"--rm\x00")).as_mut_ptr())
                                         {
                                            FIO_setRemoveSrcFile(1i32 as
                                                                     libc::c_uint);
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 14],
                                                                                       &mut [libc::c_char; 14]>(b"--priority=rt\x00")).as_mut_ptr())
                                         {
                                            setRealTimePrio = 1i32;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 16],
                                                                                       &mut [libc::c_char; 16]>(b"--single-thread\x00")).as_mut_ptr())
                                         {
                                            nbWorkers = 0i32;
                                            singleThread = 1i32;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 ==
                                                      strcmp(argument,
                                                             (*::std::mem::transmute::<&[u8; 14],
                                                                                       &mut [libc::c_char; 14]>(b"--format=zstd\x00")).as_mut_ptr())
                                         {
                                            suffix =
                                                (*::std::mem::transmute::<&[u8; 5],
                                                                          &mut [libc::c_char; 5]>(b".zst\x00")).as_mut_ptr();
                                            FIO_setCompressionType(FIO_zstdCompression);
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 !=
                                                      longCommandWArg(&mut argument
                                                                          as
                                                                          *mut *const libc::c_char,
                                                                      (*::std::mem::transmute::<&[u8; 11],
                                                                                                &mut [libc::c_char; 11]>(b"--threads=\x00")).as_mut_ptr())
                                         {
                                            nbWorkers =
                                                readU32FromChar(&mut argument
                                                                    as
                                                                    *mut *const libc::c_char)
                                                    as libc::c_int;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 !=
                                                      longCommandWArg(&mut argument
                                                                          as
                                                                          *mut *const libc::c_char,
                                                                      (*::std::mem::transmute::<&[u8; 12],
                                                                                                &mut [libc::c_char; 12]>(b"--memlimit=\x00")).as_mut_ptr())
                                         {
                                            memLimit =
                                                readU32FromChar(&mut argument
                                                                    as
                                                                    *mut *const libc::c_char);
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 !=
                                                      longCommandWArg(&mut argument
                                                                          as
                                                                          *mut *const libc::c_char,
                                                                      (*::std::mem::transmute::<&[u8; 10],
                                                                                                &mut [libc::c_char; 10]>(b"--memory=\x00")).as_mut_ptr())
                                         {
                                            memLimit =
                                                readU32FromChar(&mut argument
                                                                    as
                                                                    *mut *const libc::c_char);
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 !=
                                                      longCommandWArg(&mut argument
                                                                          as
                                                                          *mut *const libc::c_char,
                                                                      (*::std::mem::transmute::<&[u8; 23],
                                                                                                &mut [libc::c_char; 23]>(b"--memlimit-decompress=\x00")).as_mut_ptr())
                                         {
                                            memLimit =
                                                readU32FromChar(&mut argument
                                                                    as
                                                                    *mut *const libc::c_char);
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 !=
                                                      longCommandWArg(&mut argument
                                                                          as
                                                                          *mut *const libc::c_char,
                                                                      (*::std::mem::transmute::<&[u8; 14],
                                                                                                &mut [libc::c_char; 14]>(b"--block-size=\x00")).as_mut_ptr())
                                         {
                                            blockSize =
                                                readU32FromChar(&mut argument
                                                                    as
                                                                    *mut *const libc::c_char)
                                                    as size_t;
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 !=
                                                      longCommandWArg(&mut argument
                                                                          as
                                                                          *mut *const libc::c_char,
                                                                      (*::std::mem::transmute::<&[u8; 11],
                                                                                                &mut [libc::c_char; 11]>(b"--maxdict=\x00")).as_mut_ptr())
                                         {
                                            maxDictSize =
                                                readU32FromChar(&mut argument
                                                                    as
                                                                    *mut *const libc::c_char);
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 !=
                                                      longCommandWArg(&mut argument
                                                                          as
                                                                          *mut *const libc::c_char,
                                                                      (*::std::mem::transmute::<&[u8; 10],
                                                                                                &mut [libc::c_char; 10]>(b"--dictID=\x00")).as_mut_ptr())
                                         {
                                            dictID =
                                                readU32FromChar(&mut argument
                                                                    as
                                                                    *mut *const libc::c_char);
                                            current_block =
                                                14359455889292382949;
                                        } else if 0 !=
                                                      longCommandWArg(&mut argument
                                                                          as
                                                                          *mut *const libc::c_char,
                                                                      (*::std::mem::transmute::<&[u8; 8],
                                                                                                &mut [libc::c_char; 8]>(b"--zstd=\x00")).as_mut_ptr())
                                         {
                                            if 0 ==
                                                   parseCompressionParameters(argument,
                                                                              &mut compressionParams
                                                                                  as
                                                                                  *mut ZSTD_compressionParameters)
                                               {
                                                operationResult =
                                                    badusage(programName);
                                                current_block =
                                                    579604380461068407;
                                                break ;
                                            } else {
                                                current_block =
                                                    14359455889292382949;
                                            }
                                        } else if 0 !=
                                                      longCommandWArg(&mut argument
                                                                          as
                                                                          *mut *const libc::c_char,
                                                                      (*::std::mem::transmute::<&[u8; 7],
                                                                                                &mut [libc::c_char; 7]>(b"--long\x00")).as_mut_ptr())
                                         {
                                            let mut ldmWindowLog:
                                                    libc::c_uint =
                                                0i32 as libc::c_uint;
                                            ldmFlag = 1i32;
                                            if *argument as libc::c_int ==
                                                   '=' as i32 {
                                                argument =
                                                    argument.offset(1isize);
                                                ldmWindowLog =
                                                    readU32FromChar(&mut argument
                                                                        as
                                                                        *mut *const libc::c_char)
                                            } else if *argument as libc::c_int
                                                          != 0i32 {
                                                operationResult =
                                                    badusage(programName);
                                                current_block =
                                                    579604380461068407;
                                                break ;
                                            }
                                            if compressionParams.windowLog ==
                                                   0i32 as libc::c_uint {
                                                compressionParams.windowLog =
                                                    ldmWindowLog;
                                                current_block =
                                                    14359455889292382949;
                                            } else {
                                                current_block =
                                                    14359455889292382949;
                                            }
                                        } else if 0 !=
                                                      longCommandWArg(&mut argument
                                                                          as
                                                                          *mut *const libc::c_char,
                                                                      (*::std::mem::transmute::<&[u8; 7],
                                                                                                &mut [libc::c_char; 7]>(b"--fast\x00")).as_mut_ptr())
                                         {
                                            if *argument as libc::c_int ==
                                                   '=' as i32 {
                                                let mut fastLevel: U32 = 0;
                                                argument =
                                                    argument.offset(1isize);
                                                fastLevel =
                                                    readU32FromChar(&mut argument
                                                                        as
                                                                        *mut *const libc::c_char);
                                                if 0 != fastLevel {
                                                    cLevel =
                                                        -(fastLevel as
                                                              libc::c_int);
                                                    dictCLevel = cLevel
                                                } else {
                                                    operationResult =
                                                        badusage(programName);
                                                    current_block =
                                                        579604380461068407;
                                                    break ;
                                                }
                                            } else if *argument as libc::c_int
                                                          != 0i32 {
                                                operationResult =
                                                    badusage(programName);
                                                current_block =
                                                    579604380461068407;
                                                break ;
                                            } else { cLevel = -1i32 }
                                            current_block =
                                                14359455889292382949;
                                        } else {
                                            current_block =
                                                11385396242402735691;
                                        }
                                    } else {
                                        current_block = 11385396242402735691;
                                    }
                                    match current_block {
                                        14359455889292382949 => { }
                                        _ => {
                                            argument =
                                                argument.offset(1isize);
                                            while *argument.offset(0isize) as
                                                      libc::c_int != 0i32 {
                                                if 0 != lastCommand {
                                                    fprintf(g_displayOut,
                                                            (*::std::mem::transmute::<&[u8; 47],
                                                                                      &mut [libc::c_char; 47]>(b"error : command must be followed by argument \n\x00")).as_mut_ptr());
                                                    operationResult = 1i32;
                                                    current_block =
                                                        579604380461068407;
                                                    break 's_143 ;
                                                } else if *argument as
                                                              libc::c_int >=
                                                              '0' as i32 &&
                                                              *argument as
                                                                  libc::c_int
                                                                  <=
                                                                  '9' as i32 {
                                                    cLevel =
                                                        readU32FromChar(&mut argument
                                                                            as
                                                                            *mut *const libc::c_char)
                                                            as libc::c_int;
                                                    dictCLevel = cLevel
                                                } else {
                                                    match *argument.offset(0isize)
                                                              as libc::c_int {
                                                        86 => {
                                                            g_displayOut =
                                                                stdout;
                                                            printVersion();
                                                            operationResult =
                                                                0i32;
                                                            current_block =
                                                                579604380461068407;
                                                            break 's_143 ;
                                                        }
                                                        72 | 104 => {
                                                            g_displayOut =
                                                                stdout;
                                                            operationResult =
                                                                usage_advanced(programName);
                                                            current_block =
                                                                579604380461068407;
                                                            break 's_143 ;
                                                        }
                                                        122 => {
                                                            operation =
                                                                zom_compress;
                                                            argument =
                                                                argument.offset(1isize)
                                                        }
                                                        100 => {
                                                            operation =
                                                                zom_decompress;
                                                            argument =
                                                                argument.offset(1isize)
                                                        }
                                                        99 => {
                                                            forceStdout =
                                                                1i32;
                                                            outFileName =
                                                                (*::std::mem::transmute::<&[u8; 11],
                                                                                          &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr();
                                                            argument =
                                                                argument.offset(1isize)
                                                        }
                                                        68 => {
                                                            nextEntryIsDictionary
                                                                = 1i32;
                                                            lastCommand =
                                                                1i32;
                                                            argument =
                                                                argument.offset(1isize)
                                                        }
                                                        102 => {
                                                            FIO_overwriteMode();
                                                            forceStdout =
                                                                1i32;
                                                            followLinks =
                                                                1i32;
                                                            argument =
                                                                argument.offset(1isize)
                                                        }
                                                        118 => {
                                                            g_displayLevel +=
                                                                1;
                                                            argument =
                                                                argument.offset(1isize)
                                                        }
                                                        113 => {
                                                            g_displayLevel -=
                                                                1;
                                                            argument =
                                                                argument.offset(1isize)
                                                        }
                                                        107 => {
                                                            FIO_setRemoveSrcFile(0i32
                                                                                     as
                                                                                     libc::c_uint);
                                                            argument =
                                                                argument.offset(1isize)
                                                        }
                                                        67 => {
                                                            FIO_setChecksumFlag(2i32
                                                                                    as
                                                                                    libc::c_uint);
                                                            argument =
                                                                argument.offset(1isize)
                                                        }
                                                        116 => {
                                                            operation =
                                                                zom_test;
                                                            argument =
                                                                argument.offset(1isize)
                                                        }
                                                        111 => {
                                                            nextArgumentIsOutFileName
                                                                = 1i32;
                                                            lastCommand =
                                                                1i32;
                                                            argument =
                                                                argument.offset(1isize)
                                                        }
                                                        77 => {
                                                            argument =
                                                                argument.offset(1isize);
                                                            memLimit =
                                                                readU32FromChar(&mut argument
                                                                                    as
                                                                                    *mut *const libc::c_char)
                                                        }
                                                        108 => {
                                                            operation =
                                                                zom_list;
                                                            argument =
                                                                argument.offset(1isize)
                                                        }
                                                        114 => {
                                                            recursive =
                                                                1i32 as
                                                                    libc::c_uint;
                                                            argument =
                                                                argument.offset(1isize)
                                                        }
                                                        84 => {
                                                            argument =
                                                                argument.offset(1isize);
                                                            nbWorkers =
                                                                readU32FromChar(&mut argument
                                                                                    as
                                                                                    *mut *const libc::c_char)
                                                                    as
                                                                    libc::c_int
                                                        }
                                                        115 => {
                                                            argument =
                                                                argument.offset(1isize);
                                                            dictSelect =
                                                                readU32FromChar(&mut argument
                                                                                    as
                                                                                    *mut *const libc::c_char)
                                                        }
                                                        112 => {
                                                            argument =
                                                                argument.offset(1isize);
                                                            main_pause = 1i32
                                                        }
                                                        _ => {
                                                            operationResult =
                                                                badusage(programName);
                                                            current_block =
                                                                579604380461068407;
                                                            break 's_143 ;
                                                        }
                                                    }
                                                }
                                            }
                                            current_block =
                                                14359455889292382949;
                                        }
                                    }
                                } else if 0 != nextArgumentIsMaxDict {
                                    nextArgumentIsMaxDict = 0i32;
                                    lastCommand = 0i32;
                                    maxDictSize =
                                        readU32FromChar(&mut argument as
                                                            *mut *const libc::c_char);
                                    current_block = 14359455889292382949;
                                } else if 0 != nextArgumentIsDictID {
                                    nextArgumentIsDictID = 0i32;
                                    lastCommand = 0i32;
                                    dictID =
                                        readU32FromChar(&mut argument as
                                                            *mut *const libc::c_char);
                                    current_block = 14359455889292382949;
                                } else {
                                    current_block = 17788412896529399552;
                                }
                            }
                        }
                    } else { current_block = 17788412896529399552; }
                    match current_block {
                        14359455889292382949 => { }
                        _ => {
                            if 0 != nextEntryIsDictionary {
                                nextEntryIsDictionary = 0i32;
                                lastCommand = 0i32;
                                dictFileName = argument
                            } else if 0 != nextArgumentIsOutFileName {
                                nextArgumentIsOutFileName = 0i32;
                                lastCommand = 0i32;
                                outFileName = argument;
                                if 0 ==
                                       strcmp(outFileName,
                                              (*::std::mem::transmute::<&[u8; 2],
                                                                        &mut [libc::c_char; 2]>(b"-\x00")).as_mut_ptr())
                                   {
                                    outFileName =
                                        (*::std::mem::transmute::<&[u8; 11],
                                                                  &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr()
                                }
                            } else {
                                let fresh3 = filenameIdx;
                                filenameIdx = filenameIdx.wrapping_add(1);
                                let ref mut fresh4 =
                                    *filenameTable.offset(fresh3 as isize);
                                *fresh4 = argument
                            }
                        }
                    }
                }
                argNb += 1
            }
        match current_block {
            16924917904204750491 => {
                if 0 != lastCommand {
                    fprintf(g_displayOut,
                            (*::std::mem::transmute::<&[u8; 47],
                                                      &mut [libc::c_char; 47]>(b"error : command must be followed by argument \n\x00")).as_mut_ptr());
                    operationResult = 1i32
                } else {
                    if g_displayLevel >= 3i32 {
                        fprintf(g_displayOut,
                                (*::std::mem::transmute::<&[u8; 30],
                                                          &mut [libc::c_char; 30]>(b"*** %s %i-bits %s, by %s ***\n\x00")).as_mut_ptr(),
                                (*::std::mem::transmute::<&[u8; 28],
                                                          &mut [libc::c_char; 28]>(b"zstd command line interface\x00")).as_mut_ptr(),
                                (::std::mem::size_of::<size_t>() as
                                     libc::c_ulong).wrapping_mul(8i32 as
                                                                     libc::c_ulong)
                                    as libc::c_int,
                                (*::std::mem::transmute::<&[u8; 7],
                                                          &mut [libc::c_char; 7]>(b"v1.3.5\x00")).as_mut_ptr(),
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &mut [libc::c_char; 12]>(b"Yann Collet\x00")).as_mut_ptr());
                    }
                    g_utilDisplayLevel = g_displayLevel;
                    if 0 == followLinks {
                        let mut u: libc::c_uint = 0;
                        u = 0i32 as libc::c_uint;
                        fileNamesNb = 0i32 as libc::c_uint;
                        while u < filenameIdx {
                            if 0 !=
                                   UTIL_isLink(*filenameTable.offset(u as
                                                                         isize))
                               {
                                if g_displayLevel >= 2i32 {
                                    fprintf(g_displayOut,
                                            (*::std::mem::transmute::<&[u8; 43],
                                                                      &mut [libc::c_char; 43]>(b"Warning : %s is a symbolic link, ignoring\n\x00")).as_mut_ptr(),
                                            *filenameTable.offset(u as
                                                                      isize));
                                }
                            } else {
                                let fresh5 = fileNamesNb;
                                fileNamesNb = fileNamesNb.wrapping_add(1);
                                let ref mut fresh6 =
                                    *filenameTable.offset(fresh5 as isize);
                                *fresh6 = *filenameTable.offset(u as isize)
                            }
                            u = u.wrapping_add(1)
                        }
                        filenameIdx = fileNamesNb
                    }
                    if 0 != recursive {
                        extendedFileList =
                            UTIL_createFileList(filenameTable, filenameIdx,
                                                &mut fileNamesBuf as
                                                    *mut *mut libc::c_char,
                                                &mut fileNamesNb as
                                                    *mut libc::c_uint,
                                                followLinks);
                        if !extendedFileList.is_null() {
                            let mut u_0: libc::c_uint = 0;
                            u_0 = 0i32 as libc::c_uint;
                            while u_0 < fileNamesNb {
                                if g_displayLevel >= 4i32 {
                                    fprintf(g_displayOut,
                                            (*::std::mem::transmute::<&[u8; 7],
                                                                      &mut [libc::c_char; 7]>(b"%u %s\n\x00")).as_mut_ptr(),
                                            u_0,
                                            *extendedFileList.offset(u_0 as
                                                                         isize));
                                }
                                u_0 = u_0.wrapping_add(1)
                            }
                            free(filenameTable as *mut libc::c_void);
                            filenameTable = extendedFileList;
                            filenameIdx = fileNamesNb
                        }
                    }
                    if operation as libc::c_uint ==
                           zom_list as libc::c_int as libc::c_uint {
                        let ret: libc::c_int =
                            FIO_listMultipleFiles(filenameIdx, filenameTable,
                                                  g_displayLevel);
                        operationResult = ret
                    } else if !(operation as libc::c_uint ==
                                    zom_bench as libc::c_int as libc::c_uint)
                     {
                        if !(operation as libc::c_uint ==
                                 zom_train as libc::c_int as libc::c_uint) {
                            if operation as libc::c_uint ==
                                   zom_test as libc::c_int as libc::c_uint {
                                outFileName =
                                    (*::std::mem::transmute::<&[u8; 10],
                                                              &mut [libc::c_char; 10]>(b"/dev/null\x00")).as_mut_ptr();
                                FIO_setRemoveSrcFile(0i32 as libc::c_uint);
                            }
                            filenameIdx =
                                filenameIdx.wrapping_add((0 == filenameIdx) as
                                                             libc::c_int as
                                                             libc::c_uint);
                            if 0 ==
                                   strcmp(*filenameTable.offset(0isize),
                                          (*::std::mem::transmute::<&[u8; 10],
                                                                    &mut [libc::c_char; 10]>(b"/*stdin*\\\x00")).as_mut_ptr())
                                   && outFileName.is_null() {
                                outFileName =
                                    (*::std::mem::transmute::<&[u8; 11],
                                                              &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr()
                            }
                            if 0 ==
                                   strcmp(*filenameTable.offset(0isize),
                                          (*::std::mem::transmute::<&[u8; 10],
                                                                    &mut [libc::c_char; 10]>(b"/*stdin*\\\x00")).as_mut_ptr())
                                   && 0 != isatty(fileno(stdin)) {
                                operationResult = badusage(programName)
                            } else if !outFileName.is_null() &&
                                          0 ==
                                              strcmp(outFileName,
                                                     (*::std::mem::transmute::<&[u8; 11],
                                                                               &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr())
                                          && 0 != isatty(fileno(stdout)) &&
                                          0 ==
                                              strcmp(*filenameTable.offset(0isize),
                                                     (*::std::mem::transmute::<&[u8; 10],
                                                                               &mut [libc::c_char; 10]>(b"/*stdin*\\\x00")).as_mut_ptr())
                                          && 0 == forceStdout &&
                                          operation as libc::c_uint !=
                                              zom_decompress as libc::c_int as
                                                  libc::c_uint {
                                operationResult = badusage(programName)
                            } else {
                                let maxCLevel: libc::c_int =
                                    if 0 != ultra {
                                        ZSTD_maxCLevel()
                                    } else { 19i32 };
                                if cLevel > maxCLevel {
                                    if g_displayLevel >= 2i32 {
                                        fprintf(g_displayOut,
                                                (*::std::mem::transmute::<&[u8; 61],
                                                                          &mut [libc::c_char; 61]>(b"Warning : compression level higher than max, reduced to %i \n\x00")).as_mut_ptr(),
                                                maxCLevel);
                                    }
                                    cLevel = maxCLevel
                                }
                                if 0 ==
                                       strcmp(*filenameTable.offset(0isize),
                                              (*::std::mem::transmute::<&[u8; 10],
                                                                        &mut [libc::c_char; 10]>(b"/*stdin*\\\x00")).as_mut_ptr())
                                       && !outFileName.is_null() &&
                                       0 ==
                                           strcmp(outFileName,
                                                  (*::std::mem::transmute::<&[u8; 11],
                                                                            &mut [libc::c_char; 11]>(b"/*stdout*\\\x00")).as_mut_ptr())
                                       && g_displayLevel == 2i32 {
                                    g_displayLevel = 1i32
                                }
                                if 0 !=
                                       (filenameIdx > 1i32 as libc::c_uint) as
                                           libc::c_int &
                                           (g_displayLevel == 2i32) as
                                               libc::c_int {
                                    g_displayLevel = 1i32
                                }
                                FIO_setNotificationLevel(g_displayLevel as
                                                             libc::c_uint);
                                if operation as libc::c_uint ==
                                       zom_compress as libc::c_int as
                                           libc::c_uint {
                                    FIO_setNbWorkers(nbWorkers as
                                                         libc::c_uint);
                                    FIO_setBlockSize(blockSize as U32);
                                    FIO_setLdmFlag(ldmFlag as libc::c_uint);
                                    FIO_setLdmHashLog(g_ldmHashLog);
                                    FIO_setLdmMinMatch(g_ldmMinMatch);
                                    if g_ldmBucketSizeLog !=
                                           9999i32 as libc::c_uint {
                                        FIO_setLdmBucketSizeLog(g_ldmBucketSizeLog);
                                    }
                                    if g_ldmHashEveryLog !=
                                           9999i32 as libc::c_uint {
                                        FIO_setLdmHashEveryLog(g_ldmHashEveryLog);
                                    }
                                    if g_overlapLog != 9999i32 as libc::c_uint
                                       {
                                        FIO_setOverlapLog(g_overlapLog);
                                    }
                                    if filenameIdx == 1i32 as libc::c_uint &&
                                           !outFileName.is_null() {
                                        operationResult =
                                            FIO_compressFilename(outFileName,
                                                                 *filenameTable.offset(0isize),
                                                                 dictFileName,
                                                                 cLevel,
                                                                 &mut compressionParams
                                                                     as
                                                                     *mut ZSTD_compressionParameters)
                                    } else {
                                        operationResult =
                                            FIO_compressMultipleFilenames(filenameTable,
                                                                          filenameIdx,
                                                                          outFileName,
                                                                          suffix,
                                                                          dictFileName,
                                                                          cLevel,
                                                                          &mut compressionParams
                                                                              as
                                                                              *mut ZSTD_compressionParameters)
                                    }
                                } else {
                                    if memLimit == 0i32 as libc::c_uint {
                                        if compressionParams.windowLog ==
                                               0i32 as libc::c_uint {
                                            memLimit =
                                                (1i32 as U32) <<
                                                    g_defaultMaxWindowLog
                                        } else {
                                            memLimit =
                                                (1i32 as U32) <<
                                                    (compressionParams.windowLog
                                                         &
                                                         31i32 as
                                                             libc::c_uint)
                                        }
                                    }
                                    FIO_setMemLimit(memLimit);
                                    if filenameIdx == 1i32 as libc::c_uint &&
                                           !outFileName.is_null() {
                                        operationResult =
                                            FIO_decompressFilename(outFileName,
                                                                   *filenameTable.offset(0isize),
                                                                   dictFileName)
                                    } else {
                                        operationResult =
                                            FIO_decompressMultipleFilenames(filenameTable,
                                                                            filenameIdx,
                                                                            outFileName,
                                                                            dictFileName)
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        if 0 != main_pause { waitEnter(); }
        if !extendedFileList.is_null() {
            UTIL_freeFileList(extendedFileList, fileNamesBuf);
        } else { free(filenameTable as *mut libc::c_void); }
        return operationResult
    };
}
