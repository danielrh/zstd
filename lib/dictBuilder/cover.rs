#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( extern_types , libc , offset_to )]
extern crate libc;
extern "C" {
    pub type POOL_ctx_s;
    pub type _IO_FILE_plus;
    pub type ZSTD_CCtx_s;
    pub type ZSTD_CDict_s;
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
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    static mut __tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut __daylight: libc::c_int;
    #[no_mangle]
    static mut __timezone: libc::c_long;
    #[no_mangle]
    fn ZSTD_compressBound(srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_isError(code: size_t) -> libc::c_uint;
    #[no_mangle]
    fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
    #[no_mangle]
    fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> size_t;
    #[no_mangle]
    fn ZSTD_createCDict(dictBuffer: *const libc::c_void, dictSize: size_t,
                        compressionLevel: libc::c_int) -> *mut ZSTD_CDict;
    #[no_mangle]
    fn ZSTD_freeCDict(CDict: *mut ZSTD_CDict) -> size_t;
    #[no_mangle]
    fn ZSTD_compress_usingCDict(cctx: *mut ZSTD_CCtx, dst: *mut libc::c_void,
                                dstCapacity: size_t, src: *const libc::c_void,
                                srcSize: size_t, cdict: *const ZSTD_CDict)
     -> size_t;
    /* *
 * Parameters for COVER_tryParameters().
 */
    /* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
    /* ! POOL_function :
 *  The function type that can be added to a thread pool.
 */
    /* **************************************
*  Explicit context
***************************************/
    /* !< maximum compression level available */
    /* !< provides readable string from an error code */
    /* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
    /* *********************************
 *  Bulk processing dictionary API
 *********************************/
    /* ! Constructor and Destructor of FSE_DTable.
    Note that its size depends on 'tableLog' */
    /* *
 * COVER_best_t is used for two purposes:
 * 1. Synchronizing threads.
 * 2. Saving the best parameters and dictionary.
 *
 * All of the methods except COVER_best_init() are thread safe if zstd is
 * compiled with multithreaded support.
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
    /* ! POOL_create() :
 *  Create a thread pool with at most `numThreads` threads.
 * `numThreads` must be at least 1.
 *  The maximum number of queued jobs before blocking is `queueSize`.
 * @return : POOL_ctx pointer on success, else NULL.
*/
    #[no_mangle]
    fn POOL_create(numThreads: size_t, queueSize: size_t) -> *mut POOL_ctx;
    /* ! POOL_free() :
 *  Free a thread pool returned by POOL_create().
 */
    #[no_mangle]
    fn POOL_free(ctx: *mut POOL_ctx) -> ();
    /* ! POOL_add() :
 *  Add the job `function(opaque)` to the thread pool. `ctx` must be valid.
 *  Possibly blocks until there is room in the queue.
 *  Note : The function may be executed asynchronously,
 *         therefore, `opaque` must live until function has been completed.
 */
    #[no_mangle]
    fn POOL_add(ctx: *mut POOL_ctx, function: POOL_function,
                opaque: *mut libc::c_void) -> ();
    #[no_mangle]
    fn pthread_mutex_init(__mutex: *mut pthread_mutex_t,
                          __mutexattr: *const pthread_mutexattr_t)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_init(__cond: *mut pthread_cond_t,
                         __cond_attr: *const pthread_condattr_t)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_wait(__cond: *mut pthread_cond_t,
                         __mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
    #[no_mangle]
    fn ZDICT_isError(errorCode: size_t) -> libc::c_uint;
    #[no_mangle]
    fn ZDICT_finalizeDictionary(dictBuffer: *mut libc::c_void,
                                dictBufferCapacity: size_t,
                                dictContent: *const libc::c_void,
                                dictContentSize: size_t,
                                samplesBuffer: *const libc::c_void,
                                samplesSizes: *const size_t,
                                nbSamples: libc::c_uint,
                                parameters: ZDICT_params_t) -> size_t;
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type U32 = uint32_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct COVER_map_pair_t_s {
    pub key: U32,
    pub value: U32,
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
pub type COVER_map_pair_t = COVER_map_pair_t_s;
pub const ZSTD_error_corruption_detected: ERR_enum = 20;
pub const ZSTD_error_frameIndex_tooLarge: ERR_enum = 100;
pub const ZSTD_error_parameter_unsupported: ERR_enum = 40;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: libc::c_uint,
    pub ptr: *const libc::c_char,
    pub start: *const libc::c_char,
    pub limitPtr: *const libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: libc::c_int,
    pub deltaNbBits: U32,
}
pub type __off64_t = libc::c_long;
pub const ZSTD_error_maxCode: ERR_enum = 120;
pub type ptrdiff_t = libc::c_long;
pub const ZSTD_error_init_missing: ERR_enum = 62;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_condattr_t {
    __size: [libc::c_char; 4],
    __align: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZDICT_cover_params_t {
    pub k: libc::c_uint,
    pub d: libc::c_uint,
    pub steps: libc::c_uint,
    pub nbThreads: libc::c_uint,
    pub zParams: ZDICT_params_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_mutex_t {
    __data: __pthread_mutex_s,
    __size: [libc::c_char; 40],
    __align: libc::c_long,
}
pub const ZSTD_error_stage_wrong: ERR_enum = 60;
pub type size_t = libc::c_ulong;
pub const MEM_static_assert: unnamed_1 = 1;
pub type clock_t = __clock_t;
pub const ZSTD_error_workSpace_tooSmall: ERR_enum = 66;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
pub type COVER_tryParameters_data_t = COVER_tryParameters_data_s;
pub const ZSTD_error_maxSymbolValue_tooSmall: ERR_enum = 48;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct COVER_segment_t {
    pub begin: U32,
    pub end: U32,
    pub score: U32,
}
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
pub const ZSTD_error_seekableIO: ERR_enum = 102;
pub const ZSTD_error_memory_allocation: ERR_enum = 64;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
pub type unnamed = libc::c_uint;
pub const ZSTD_error_frameParameter_windowTooLarge: ERR_enum = 16;
pub const ZSTD_error_dictionary_wrong: ERR_enum = 32;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
pub type __clock_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const libc::c_void,
    pub symbolTT: *const libc::c_void,
    pub stateLog: libc::c_uint,
}
pub type POOL_function =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_cond_t {
    __data: unnamed_2,
    __size: [libc::c_char; 48],
    __align: libc::c_longlong,
}
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub const ZSTD_error_maxSymbolValue_tooLarge: ERR_enum = 46;
pub const ZSTD_error_checksum_wrong: ERR_enum = 22;
pub const ZSTD_error_prefix_unknown: ERR_enum = 10;
pub type S16 = int16_t;
pub type BYTE = uint8_t;
pub type ERR_enum = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct COVER_map_s {
    pub data: *mut COVER_map_pair_t,
    pub sizeLog: U32,
    pub size: U32,
    pub sizeMask: U32,
}
pub const MEM_static_assert_0: unnamed = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    u: U32,
    c: [BYTE; 4],
}
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type FSE_CTable = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub const ZSTD_error_version_unsupported: ERR_enum = 12;
pub type COVER_map_t = COVER_map_s;
pub const ZSTD_error_GENERIC: ERR_enum = 1;
pub type _IO_lock_t = ();
pub type BIT_DStream_status = libc::c_uint;
pub const ZSTD_error_dictionary_corrupted: ERR_enum = 30;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub type uint32_t = libc::c_uint;
pub const ZSTD_error_tableLog_tooLarge: ERR_enum = 44;
pub type U16 = uint16_t;
pub type unnamed_1 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct COVER_tryParameters_data_s {
    pub ctx: *const COVER_ctx_t,
    pub best: *mut COVER_best_t,
    pub dictBufferCapacity: size_t,
    pub parameters: ZDICT_cover_params_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub __lock: libc::c_int,
    pub __futex: libc::c_uint,
    pub __total_seq: libc::c_ulonglong,
    pub __wakeup_seq: libc::c_ulonglong,
    pub __woken_seq: libc::c_ulonglong,
    pub __mutex: *mut libc::c_void,
    pub __nwaiters: libc::c_uint,
    pub __broadcast_seq: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DState_t {
    pub state: size_t,
    pub table: *const libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub type U64 = uint64_t;
pub const ZSTD_error_parameter_outOfBound: ERR_enum = 42;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
pub type ZSTD_CDict = ZSTD_CDict_s;
pub type __pthread_list_t = __pthread_internal_list;
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZDICT_params_t {
    pub compressionLevel: libc::c_int,
    pub notificationLevel: libc::c_uint,
    pub dictID: libc::c_uint,
}
pub type int16_t = libc::c_short;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub type FSE_DTable = libc::c_uint;
pub const ZSTD_error_no_error: ERR_enum = 0;
pub type COVER_best_t = COVER_best_s;
pub type ZSTD_ErrorCode = ERR_enum;
pub type uint8_t = libc::c_uchar;
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
pub type uint64_t = libc::c_ulong;
pub const ZSTD_error_dstSize_tooSmall: ERR_enum = 70;
pub type POOL_ctx = POOL_ctx_s;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct COVER_ctx_t {
    pub samples: *const BYTE,
    pub offsets: *mut size_t,
    pub samplesSizes: *const size_t,
    pub nbSamples: size_t,
    pub suffix: *mut U32,
    pub suffixSize: size_t,
    pub freqs: *mut U32,
    pub dmerAt: *mut U32,
    pub d: libc::c_uint,
}
pub type FILE = _IO_FILE;
pub const ZSTD_error_dictionaryCreation_failed: ERR_enum = 34;
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_CStream_t {
    pub bitContainer: size_t,
    pub bitPos: libc::c_uint,
    pub startPtr: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub endPtr: *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct COVER_best_s {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub liveJobs: size_t,
    pub dict: *mut libc::c_void,
    pub dictSize: size_t,
    pub parameters: ZDICT_cover_params_t,
    pub compressedSize: size_t,
}
pub const ZSTD_error_srcSize_wrong: ERR_enum = 72;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_mutexattr_t {
    __size: [libc::c_char; 4],
    __align: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
pub const ZSTD_error_frameParameter_unsupported: ERR_enum = 14;
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
unsafe extern "C" fn BIT_initCStream(mut bitC: *mut BIT_CStream_t,
                                     mut startPtr: *mut libc::c_void,
                                     mut dstCapacity: size_t) -> size_t {
    (*bitC).bitContainer = 0i32 as size_t;
    (*bitC).bitPos = 0i32 as libc::c_uint;
    (*bitC).startPtr = startPtr as *mut libc::c_char;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC).endPtr =
        (*bitC).startPtr.offset(dstCapacity as
                                    isize).offset(-(::std::mem::size_of::<size_t>()
                                                        as libc::c_ulong as
                                                        isize));
    if dstCapacity <= ::std::mem::size_of::<size_t>() as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else { return 0i32 as size_t };
}
unsafe extern "C" fn BIT_addBits(mut bitC: *mut BIT_CStream_t,
                                 mut value: size_t, mut nbBits: libc::c_uint)
 -> () {
    (*bitC).bitContainer |=
        (value & BIT_mask[nbBits as usize] as libc::c_ulong) <<
            (*bitC).bitPos;
    (*bitC).bitPos = (*bitC).bitPos.wrapping_add(nbBits);
}
static mut BIT_mask: [libc::c_uint; 32] =
    unsafe {
        [0i32 as libc::c_uint, 1i32 as libc::c_uint, 3i32 as libc::c_uint,
         7i32 as libc::c_uint, 15i32 as libc::c_uint, 31i32 as libc::c_uint,
         63i32 as libc::c_uint, 127i32 as libc::c_uint,
         255i32 as libc::c_uint, 511i32 as libc::c_uint,
         1023i32 as libc::c_uint, 2047i32 as libc::c_uint,
         4095i32 as libc::c_uint, 8191i32 as libc::c_uint,
         16383i32 as libc::c_uint, 32767i32 as libc::c_uint,
         65535i32 as libc::c_uint, 131071i32 as libc::c_uint,
         262143i32 as libc::c_uint, 524287i32 as libc::c_uint,
         1048575i32 as libc::c_uint, 2097151i32 as libc::c_uint,
         4194303i32 as libc::c_uint, 8388607i32 as libc::c_uint,
         16777215i32 as libc::c_uint, 33554431i32 as libc::c_uint,
         67108863i32 as libc::c_uint, 134217727i32 as libc::c_uint,
         268435455i32 as libc::c_uint, 536870911i32 as libc::c_uint,
         1073741823i32 as libc::c_uint, 2147483647i32 as libc::c_uint]
    };
unsafe extern "C" fn BIT_flushBits(mut bitC: *mut BIT_CStream_t) -> () {
    let nbBytes: size_t = ((*bitC).bitPos >> 3i32) as size_t;
    MEM_writeLEST((*bitC).ptr as *mut libc::c_void, (*bitC).bitContainer);
    (*bitC).ptr = (*bitC).ptr.offset(nbBytes as isize);
    if (*bitC).ptr > (*bitC).endPtr { (*bitC).ptr = (*bitC).endPtr }
    (*bitC).bitPos &= 7i32 as libc::c_uint;
    (*bitC).bitContainer >>= nbBytes.wrapping_mul(8i32 as libc::c_ulong);
}
unsafe extern "C" fn BIT_closeCStream(mut bitC: *mut BIT_CStream_t)
 -> size_t {
    BIT_addBitsFast(bitC, 1i32 as size_t, 1i32 as libc::c_uint);
    BIT_flushBits(bitC);
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0i32 as size_t
    } else {
        return ((*bitC).startPtr.offset_to((*bitC).ptr).expect("bad offset_to")
                    as libc::c_long +
                    ((*bitC).bitPos > 0i32 as libc::c_uint) as libc::c_int as
                        libc::c_long) as size_t
    };
}
unsafe extern "C" fn BIT_addBitsFast(mut bitC: *mut BIT_CStream_t,
                                     mut value: size_t,
                                     mut nbBits: libc::c_uint) -> () {
    (*bitC).bitContainer |= value << (*bitC).bitPos;
    (*bitC).bitPos = (*bitC).bitPos.wrapping_add(nbBits);
}
unsafe extern "C" fn BIT_initDStream(mut bitD: *mut BIT_DStream_t,
                                     mut srcBuffer: *const libc::c_void,
                                     mut srcSize: size_t) -> size_t {
    let mut current_block: u64;
    if srcSize < 1i32 as libc::c_ulong {
        memset(bitD as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<BIT_DStream_t>() as libc::c_ulong);
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else {
        (*bitD).start = srcBuffer as *const libc::c_char;
        (*bitD).limitPtr =
            (*bitD).start.offset(::std::mem::size_of::<size_t>() as
                                     libc::c_ulong as isize);
        if srcSize >= ::std::mem::size_of::<size_t>() as libc::c_ulong {
            (*bitD).ptr =
                (srcBuffer as
                     *const libc::c_char).offset(srcSize as
                                                     isize).offset(-(::std::mem::size_of::<size_t>()
                                                                         as
                                                                         libc::c_ulong
                                                                         as
                                                                         isize));
            (*bitD).bitContainer =
                MEM_readLEST((*bitD).ptr as *const libc::c_void);
            let lastByte: BYTE =
                *(srcBuffer as
                      *const BYTE).offset(srcSize.wrapping_sub(1i32 as
                                                                   libc::c_ulong)
                                              as isize);
            (*bitD).bitsConsumed =
                if 0 != lastByte as libc::c_int {
                    (8i32 as
                         libc::c_uint).wrapping_sub(BIT_highbit32(lastByte as
                                                                      U32))
                } else { 0i32 as libc::c_uint };
            if lastByte as libc::c_int == 0i32 {
                return -(ZSTD_error_GENERIC as libc::c_int) as size_t
            }
        } else {
            (*bitD).ptr = (*bitD).start;
            (*bitD).bitContainer = *((*bitD).start as *const BYTE) as size_t;
            match srcSize {
                7 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(6isize)
                                                              as size_t) <<
                                                             (::std::mem::size_of::<size_t>()
                                                                  as
                                                                  libc::c_ulong).wrapping_mul(8i32
                                                                                                  as
                                                                                                  libc::c_ulong).wrapping_sub(16i32
                                                                                                                                  as
                                                                                                                                  libc::c_ulong))
                            as size_t as size_t;
                    current_block = 10192508258555769664;
                }
                6 => { current_block = 10192508258555769664; }
                5 => { current_block = 7593490379297353364; }
                4 => { current_block = 3906616468301123675; }
                3 => { current_block = 10086016483950629671; }
                2 => { current_block = 15402169207272554803; }
                _ => { current_block = 10886091980245723256; }
            }
            match current_block {
                10192508258555769664 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(5isize)
                                                              as size_t) <<
                                                             (::std::mem::size_of::<size_t>()
                                                                  as
                                                                  libc::c_ulong).wrapping_mul(8i32
                                                                                                  as
                                                                                                  libc::c_ulong).wrapping_sub(24i32
                                                                                                                                  as
                                                                                                                                  libc::c_ulong))
                            as size_t as size_t;
                    current_block = 7593490379297353364;
                }
                _ => { }
            }
            match current_block {
                7593490379297353364 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(4isize)
                                                              as size_t) <<
                                                             (::std::mem::size_of::<size_t>()
                                                                  as
                                                                  libc::c_ulong).wrapping_mul(8i32
                                                                                                  as
                                                                                                  libc::c_ulong).wrapping_sub(32i32
                                                                                                                                  as
                                                                                                                                  libc::c_ulong))
                            as size_t as size_t;
                    current_block = 3906616468301123675;
                }
                _ => { }
            }
            match current_block {
                3906616468301123675 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(3isize)
                                                              as size_t) <<
                                                             24i32) as size_t
                            as size_t;
                    current_block = 10086016483950629671;
                }
                _ => { }
            }
            match current_block {
                10086016483950629671 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(2isize)
                                                              as size_t) <<
                                                             16i32) as size_t
                            as size_t;
                    current_block = 15402169207272554803;
                }
                _ => { }
            }
            match current_block {
                15402169207272554803 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(1isize)
                                                              as size_t) <<
                                                             8i32) as size_t
                            as size_t
                }
                _ => { }
            }
            let lastByte_0: BYTE =
                *(srcBuffer as
                      *const BYTE).offset(srcSize.wrapping_sub(1i32 as
                                                                   libc::c_ulong)
                                              as isize);
            (*bitD).bitsConsumed =
                if 0 != lastByte_0 as libc::c_int {
                    (8i32 as
                         libc::c_uint).wrapping_sub(BIT_highbit32(lastByte_0
                                                                      as U32))
                } else { 0i32 as libc::c_uint };
            if lastByte_0 as libc::c_int == 0i32 {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            } else {
                (*bitD).bitsConsumed =
                    (*bitD).bitsConsumed.wrapping_add(((::std::mem::size_of::<size_t>()
                                                            as
                                                            libc::c_ulong).wrapping_sub(srcSize)
                                                           as
                                                           U32).wrapping_mul(8i32
                                                                                 as
                                                                                 libc::c_uint))
            }
        }
        return srcSize
    };
}
unsafe extern "C" fn BIT_highbit32(mut val: U32) -> libc::c_uint {
    return (31i32 - val.leading_zeros() as i32) as libc::c_uint;
}
unsafe extern "C" fn BIT_readBits(mut bitD: *mut BIT_DStream_t,
                                  mut nbBits: U32) -> size_t {
    let value: size_t = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
unsafe extern "C" fn BIT_lookBits(mut bitD: *const BIT_DStream_t,
                                  mut nbBits: U32) -> size_t {
    let regMask: U32 =
        (::std::mem::size_of::<size_t>() as
             libc::c_ulong).wrapping_mul(8i32 as
                                             libc::c_ulong).wrapping_sub(1i32
                                                                             as
                                                                             libc::c_ulong)
            as U32;
    return (*bitD).bitContainer << ((*bitD).bitsConsumed & regMask) >> 1i32 >>
               (regMask.wrapping_sub(nbBits) & regMask);
}
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t,
                                  mut nbBits: U32) -> () {
    (*bitD).bitsConsumed = (*bitD).bitsConsumed.wrapping_add(nbBits);
}
unsafe extern "C" fn BIT_reloadDStream(mut bitD: *mut BIT_DStream_t)
 -> BIT_DStream_status {
    if (*bitD).bitsConsumed as libc::c_ulong >
           (::std::mem::size_of::<size_t>() as
                libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
        return BIT_DStream_overflow
    } else if (*bitD).ptr >= (*bitD).limitPtr {
        (*bitD).ptr =
            (*bitD).ptr.offset(-(((*bitD).bitsConsumed >> 3i32) as isize));
        (*bitD).bitsConsumed &= 7i32 as libc::c_uint;
        (*bitD).bitContainer =
            MEM_readLEST((*bitD).ptr as *const libc::c_void);
        return BIT_DStream_unfinished
    } else if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as libc::c_ulong) <
               (::std::mem::size_of::<size_t>() as
                    libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
            return BIT_DStream_endOfBuffer
        } else { return BIT_DStream_completed }
    } else {
        let mut nbBytes: U32 = (*bitD).bitsConsumed >> 3i32;
        let mut result: BIT_DStream_status = BIT_DStream_unfinished;
        if (*bitD).ptr.offset(-(nbBytes as isize)) < (*bitD).start {
            nbBytes =
                (*bitD).start.offset_to((*bitD).ptr).expect("bad offset_to")
                    as libc::c_long as U32;
            result = BIT_DStream_endOfBuffer
        }
        (*bitD).ptr = (*bitD).ptr.offset(-(nbBytes as isize));
        (*bitD).bitsConsumed =
            (*bitD).bitsConsumed.wrapping_sub(nbBytes.wrapping_mul(8i32 as
                                                                       libc::c_uint));
        (*bitD).bitContainer =
            MEM_readLEST((*bitD).ptr as *const libc::c_void);
        return result
    };
}
unsafe extern "C" fn BIT_endOfDStream(mut DStream: *const BIT_DStream_t)
 -> libc::c_uint {
    return ((*DStream).ptr == (*DStream).start &&
                (*DStream).bitsConsumed as libc::c_ulong ==
                    (::std::mem::size_of::<size_t>() as
                         libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong))
               as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn BIT_flushBitsFast(mut bitC: *mut BIT_CStream_t) -> () {
    let nbBytes: size_t = ((*bitC).bitPos >> 3i32) as size_t;
    MEM_writeLEST((*bitC).ptr as *mut libc::c_void, (*bitC).bitContainer);
    (*bitC).ptr = (*bitC).ptr.offset(nbBytes as isize);
    (*bitC).bitPos &= 7i32 as libc::c_uint;
    (*bitC).bitContainer >>= nbBytes.wrapping_mul(8i32 as libc::c_ulong);
}
unsafe extern "C" fn BIT_readBitsFast(mut bitD: *mut BIT_DStream_t,
                                      mut nbBits: U32) -> size_t {
    let value: size_t = BIT_lookBitsFast(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
unsafe extern "C" fn BIT_lookBitsFast(mut bitD: *const BIT_DStream_t,
                                      mut nbBits: U32) -> size_t {
    let regMask: U32 =
        (::std::mem::size_of::<size_t>() as
             libc::c_ulong).wrapping_mul(8i32 as
                                             libc::c_ulong).wrapping_sub(1i32
                                                                             as
                                                                             libc::c_ulong)
            as U32;
    return (*bitD).bitContainer << ((*bitD).bitsConsumed & regMask) >>
               (regMask.wrapping_add(1i32 as
                                         libc::c_uint).wrapping_sub(nbBits) &
                    regMask);
}
unsafe extern "C" fn BIT_getUpperBits(mut bitContainer: size_t, start: U32)
 -> size_t {
    return bitContainer >> start;
}
unsafe extern "C" fn BIT_getMiddleBits(mut bitContainer: size_t, start: U32,
                                       nbBits: U32) -> size_t {
    return bitContainer >> start & BIT_mask[nbBits as usize] as libc::c_ulong;
}
unsafe extern "C" fn BIT_getLowerBits(mut bitContainer: size_t, nbBits: U32)
 -> size_t {
    return bitContainer & BIT_mask[nbBits as usize] as libc::c_ulong;
}
unsafe extern "C" fn FSE_initCState(mut statePtr: *mut FSE_CState_t,
                                    mut ct: *const FSE_CTable) -> () {
    let mut ptr: *const libc::c_void = ct as *const libc::c_void;
    let mut u16ptr: *const U16 = ptr as *const U16;
    let tableLog: U32 = MEM_read16(ptr) as U32;
    (*statePtr).value = (1i32 as ptrdiff_t) << tableLog;
    (*statePtr).stateTable = u16ptr.offset(2isize) as *const libc::c_void;
    (*statePtr).symbolTT =
        ct.offset(1isize).offset((if 0 != tableLog {
                                      1i32 <<
                                          tableLog.wrapping_sub(1i32 as
                                                                    libc::c_uint)
                                  } else { 1i32 }) as isize) as
            *const libc::c_void;
    (*statePtr).stateLog = tableLog;
}
unsafe extern "C" fn FSE_encodeSymbol(mut bitC: *mut BIT_CStream_t,
                                      mut statePtr: *mut FSE_CState_t,
                                      mut symbol: U32) -> () {
    let symbolTT: FSE_symbolCompressionTransform =
        *((*statePtr).symbolTT as
              *const FSE_symbolCompressionTransform).offset(symbol as isize);
    let stateTable: *const U16 = (*statePtr).stateTable as *const U16;
    let nbBitsOut: U32 =
        ((*statePtr).value + symbolTT.deltaNbBits as libc::c_long >> 16i32) as
            U32;
    BIT_addBits(bitC, (*statePtr).value as size_t, nbBitsOut);
    (*statePtr).value =
        *stateTable.offset((((*statePtr).value >> nbBitsOut) +
                                symbolTT.deltaFindState as libc::c_long) as
                               isize) as ptrdiff_t;
}
unsafe extern "C" fn FSE_flushCState(mut bitC: *mut BIT_CStream_t,
                                     mut statePtr: *const FSE_CState_t)
 -> () {
    BIT_addBits(bitC, (*statePtr).value as size_t, (*statePtr).stateLog);
    BIT_flushBits(bitC);
}
unsafe extern "C" fn FSE_initDState(mut DStatePtr: *mut FSE_DState_t,
                                    mut bitD: *mut BIT_DStream_t,
                                    mut dt: *const FSE_DTable) -> () {
    let mut ptr: *const libc::c_void = dt as *const libc::c_void;
    let DTableH: *const FSE_DTableHeader = ptr as *const FSE_DTableHeader;
    (*DStatePtr).state =
        BIT_readBits(bitD, (*DTableH).tableLog as libc::c_uint);
    BIT_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1isize) as *const libc::c_void;
}
unsafe extern "C" fn FSE_decodeSymbol(mut DStatePtr: *mut FSE_DState_t,
                                      mut bitD: *mut BIT_DStream_t)
 -> libc::c_uchar {
    let DInfo: FSE_decode_t =
        *((*DStatePtr).table as
              *const FSE_decode_t).offset((*DStatePtr).state as isize);
    let nbBits: U32 = DInfo.nbBits as U32;
    let symbol: BYTE = DInfo.symbol;
    let lowBits: size_t = BIT_readBits(bitD, nbBits);
    (*DStatePtr).state =
        (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
    return symbol;
}
unsafe extern "C" fn FSE_endOfDState(mut DStatePtr: *const FSE_DState_t)
 -> libc::c_uint {
    return ((*DStatePtr).state == 0i32 as libc::c_ulong) as libc::c_int as
               libc::c_uint;
}
/* *<
Let's now decompose FSE_decompress_usingDTable() into its unitary components.
You will decode FSE-encoded symbols from the bitStream,
and also any other bitFields you put in, **in reverse order**.

You will need a few variables to track your bitStream. They are :

BIT_DStream_t DStream;    // Stream context
FSE_DState_t  DState;     // State context. Multiple ones are possible
FSE_DTable*   DTablePtr;  // Decoding table, provided by FSE_buildDTable()

The first thing to do is to init the bitStream.
    errorCode = BIT_initDStream(&DStream, srcBuffer, srcSize);

You should then retrieve your initial state(s)
(in reverse flushing order if you have several ones) :
    errorCode = FSE_initDState(&DState, &DStream, DTablePtr);

You can then decode your data, symbol after symbol.
For information the maximum number of bits read by FSE_decodeSymbol() is 'tableLog'.
Keep in mind that symbols are decoded in reverse order, like a LIFO stack (last in, first out).
    unsigned char symbol = FSE_decodeSymbol(&DState, &DStream);

You can retrieve any bitfield you eventually stored into the bitStream (in reverse order)
Note : maximum allowed nbBits is 25, for 32-bits compatibility
    size_t bitField = BIT_readBits(&DStream, nbBits);

All above operations only read from local register (which size depends on size_t).
Refueling the register from memory is manually performed by the reload method.
    endSignal = FSE_reloadDStream(&DStream);

BIT_reloadDStream() result tells if there is still some more data to read from DStream.
BIT_DStream_unfinished : there is still some data left into the DStream.
BIT_DStream_endOfBuffer : Dstream reached end of buffer. Its container may no longer be completely filled.
BIT_DStream_completed : Dstream reached its exact end, corresponding in general to decompression completed.
BIT_DStream_tooFar : Dstream went too far. Decompression result is corrupted.

When reaching end of buffer (BIT_DStream_endOfBuffer), progress slowly, notably if you decode multiple symbols per loop,
to properly detect the exact end of stream.
After each decoded symbol, check if DStream is fully consumed using this simple test :
    BIT_reloadDStream(&DStream) >= BIT_DStream_completed

When it's done, verify decompression is fully completed, by checking both DStream and the relevant states.
Checking if DStream has reached its end is performed by :
    BIT_endOfDStream(&DStream);
Check also the states. There might be some symbols left there, if some high probability ones (>50%) are possible.
    FSE_endOfDState(&DState);
*/
unsafe extern "C" fn FSE_decodeSymbolFast(mut DStatePtr: *mut FSE_DState_t,
                                          mut bitD: *mut BIT_DStream_t)
 -> libc::c_uchar {
    let DInfo: FSE_decode_t =
        *((*DStatePtr).table as
              *const FSE_decode_t).offset((*DStatePtr).state as isize);
    let nbBits: U32 = DInfo.nbBits as U32;
    let symbol: BYTE = DInfo.symbol;
    let lowBits: size_t = BIT_readBitsFast(bitD, nbBits);
    (*DStatePtr).state =
        (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
    return symbol;
}
unsafe extern "C" fn FSE_initCState2(mut statePtr: *mut FSE_CState_t,
                                     mut ct: *const FSE_CTable,
                                     mut symbol: U32) -> () {
    FSE_initCState(statePtr, ct);
    let symbolTT: FSE_symbolCompressionTransform =
        *((*statePtr).symbolTT as
              *const FSE_symbolCompressionTransform).offset(symbol as isize);
    let mut stateTable: *const U16 = (*statePtr).stateTable as *const U16;
    let mut nbBitsOut: U32 =
        symbolTT.deltaNbBits.wrapping_add((1i32 << 15i32) as libc::c_uint) >>
            16i32;
    (*statePtr).value =
        (nbBitsOut << 16i32).wrapping_sub(symbolTT.deltaNbBits) as ptrdiff_t;
    (*statePtr).value =
        *stateTable.offset((((*statePtr).value >> nbBitsOut) +
                                symbolTT.deltaFindState as libc::c_long) as
                               isize) as ptrdiff_t;
}
unsafe extern "C" fn FSE_getMaxNbBits(mut symbolTTPtr: *const libc::c_void,
                                      mut symbolValue: U32) -> U32 {
    let mut symbolTT: *const FSE_symbolCompressionTransform =
        symbolTTPtr as *const FSE_symbolCompressionTransform;
    return (*symbolTT.offset(symbolValue as
                                 isize)).deltaNbBits.wrapping_add(((1i32 <<
                                                                        16i32)
                                                                       - 1i32)
                                                                      as
                                                                      libc::c_uint)
               >> 16i32;
}
unsafe extern "C" fn FSE_bitCost(mut symbolTTPtr: *const libc::c_void,
                                 mut tableLog: U32, mut symbolValue: U32,
                                 mut accuracyLog: U32) -> U32 {
    let mut symbolTT: *const FSE_symbolCompressionTransform =
        symbolTTPtr as *const FSE_symbolCompressionTransform;
    let minNbBits: U32 =
        (*symbolTT.offset(symbolValue as isize)).deltaNbBits >> 16i32;
    let threshold: U32 =
        minNbBits.wrapping_add(1i32 as libc::c_uint) << 16i32;
    let tableSize: U32 = (1i32 << tableLog) as U32;
    let deltaFromThreshold: U32 =
        threshold.wrapping_sub((*symbolTT.offset(symbolValue as
                                                     isize)).deltaNbBits.wrapping_add(tableSize));
    let normalizedDeltaFromThreshold: U32 =
        deltaFromThreshold << accuracyLog >> tableLog;
    let bitMultiplier: U32 = (1i32 << accuracyLog) as U32;
    return minNbBits.wrapping_add(1i32 as
                                      libc::c_uint).wrapping_mul(bitMultiplier).wrapping_sub(normalizedDeltaFromThreshold);
}
unsafe extern "C" fn FSE_peekSymbol(mut DStatePtr: *const FSE_DState_t)
 -> BYTE {
    let DInfo: FSE_decode_t =
        *((*DStatePtr).table as
              *const FSE_decode_t).offset((*DStatePtr).state as isize);
    return DInfo.symbol;
}
unsafe extern "C" fn FSE_updateState(mut DStatePtr: *mut FSE_DState_t,
                                     mut bitD: *mut BIT_DStream_t) -> () {
    let DInfo: FSE_decode_t =
        *((*DStatePtr).table as
              *const FSE_decode_t).offset((*DStatePtr).state as isize);
    let nbBits: U32 = DInfo.nbBits as U32;
    let lowBits: size_t = BIT_readBits(bitD, nbBits);
    (*DStatePtr).state =
        (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
}
static mut repStartValue: [U32; 3] =
    unsafe { [1i32 as U32, 4i32 as U32, 8i32 as U32] };
static mut ZSTD_fcs_fieldSize: [size_t; 4] =
    unsafe {
        [0i32 as size_t, 2i32 as size_t, 4i32 as size_t, 8i32 as size_t]
    };
static mut ZSTD_did_fieldSize: [size_t; 4] =
    unsafe {
        [0i32 as size_t, 1i32 as size_t, 2i32 as size_t, 4i32 as size_t]
    };
static mut ZSTD_frameIdSize: size_t = unsafe { 4i32 as size_t };
static mut ZSTD_blockHeaderSize: size_t = unsafe { 3i32 as size_t };
static mut LL_bits: [U32; 36] =
    unsafe {
        [0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32,
         0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32,
         0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32,
         0i32 as U32, 1i32 as U32, 1i32 as U32, 1i32 as U32, 1i32 as U32,
         2i32 as U32, 2i32 as U32, 3i32 as U32, 3i32 as U32, 4i32 as U32,
         6i32 as U32, 7i32 as U32, 8i32 as U32, 9i32 as U32, 10i32 as U32,
         11i32 as U32, 12i32 as U32, 13i32 as U32, 14i32 as U32, 15i32 as U32,
         16i32 as U32]
    };
static mut LL_defaultNorm: [S16; 36] =
    unsafe {
        [4i32 as S16, 3i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16,
         2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16,
         2i32 as S16, 2i32 as S16, 2i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16,
         2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16,
         3i32 as S16, 2i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, -1i32 as S16, -1i32 as S16, -1i32 as S16,
         -1i32 as S16]
    };
static mut LL_defaultNormLog: U32 = unsafe { 6i32 as U32 };
static mut ML_bits: [U32; 53] =
    unsafe {
        [0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32,
         0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32,
         0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32,
         0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32,
         0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32,
         0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32,
         0i32 as U32, 0i32 as U32, 1i32 as U32, 1i32 as U32, 1i32 as U32,
         1i32 as U32, 2i32 as U32, 2i32 as U32, 3i32 as U32, 3i32 as U32,
         4i32 as U32, 4i32 as U32, 5i32 as U32, 7i32 as U32, 8i32 as U32,
         9i32 as U32, 10i32 as U32, 11i32 as U32, 12i32 as U32, 13i32 as U32,
         14i32 as U32, 15i32 as U32, 16i32 as U32]
    };
static mut ML_defaultNorm: [S16; 53] =
    unsafe {
        [1i32 as S16, 4i32 as S16, 3i32 as S16, 2i32 as S16, 2i32 as S16,
         2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, -1i32 as S16, -1i32 as S16, -1i32 as S16, -1i32 as S16,
         -1i32 as S16, -1i32 as S16, -1i32 as S16]
    };
static mut ML_defaultNormLog: U32 = unsafe { 6i32 as U32 };
static mut OF_defaultNorm: [S16; 29] =
    unsafe {
        [1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, -1i32 as S16,
         -1i32 as S16, -1i32 as S16, -1i32 as S16, -1i32 as S16]
    };
static mut OF_defaultNormLog: U32 = unsafe { 5i32 as U32 };
unsafe extern "C" fn ZSTD_copy8(mut dst: *mut libc::c_void,
                                mut src: *const libc::c_void) -> () {
    memcpy(dst, src, 8i32 as libc::c_ulong);
}
unsafe extern "C" fn ZSTD_wildcopy(mut dst: *mut libc::c_void,
                                   mut src: *const libc::c_void,
                                   mut length: ptrdiff_t) -> () {
    let mut ip: *const BYTE = src as *const BYTE;
    let mut op: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = op.offset(length as isize);
    loop  {
        ZSTD_copy8(op as *mut libc::c_void, ip as *const libc::c_void);
        op = op.offset(8isize);
        ip = ip.offset(8isize);
        if !(op < oend) { break ; }
    };
}
unsafe extern "C" fn ZSTD_wildcopy_e(mut dst: *mut libc::c_void,
                                     mut src: *const libc::c_void,
                                     mut dstEnd: *mut libc::c_void) -> () {
    let mut ip: *const BYTE = src as *const BYTE;
    let mut op: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = dstEnd as *mut BYTE;
    loop  {
        ZSTD_copy8(op as *mut libc::c_void, ip as *const libc::c_void);
        op = op.offset(8isize);
        ip = ip.offset(8isize);
        if !(op < oend) { break ; }
    };
}
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> U32 {
    return (31i32 - val.leading_zeros() as i32) as U32;
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_cover(mut dictBuffer:
                                                         *mut libc::c_void,
                                                     mut dictBufferCapacity:
                                                         size_t,
                                                     mut samplesBuffer:
                                                         *const libc::c_void,
                                                     mut samplesSizes:
                                                         *const size_t,
                                                     mut nbSamples:
                                                         libc::c_uint,
                                                     mut parameters:
                                                         ZDICT_cover_params_t)
 -> size_t {
    let dict: *mut BYTE = dictBuffer as *mut BYTE;
    let mut ctx: COVER_ctx_t =
        COVER_ctx_t{samples: 0 as *const BYTE,
                    offsets: 0 as *mut size_t,
                    samplesSizes: 0 as *const size_t,
                    nbSamples: 0,
                    suffix: 0 as *mut U32,
                    suffixSize: 0,
                    freqs: 0 as *mut U32,
                    dmerAt: 0 as *mut U32,
                    d: 0,};
    let mut activeDmers: COVER_map_t =
        COVER_map_s{data: 0 as *mut COVER_map_pair_t,
                    sizeLog: 0,
                    size: 0,
                    sizeMask: 0,};
    g_displayLevel = parameters.zParams.notificationLevel as libc::c_int;
    if 0 == COVER_checkParameters(parameters, dictBufferCapacity) {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 28],
                                              &mut [libc::c_char; 28]>(b"Cover parameters incorrect\n\x00")).as_mut_ptr());
            fflush(stderr);
        }
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else if nbSamples == 0i32 as libc::c_uint {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 41],
                                              &mut [libc::c_char; 41]>(b"Cover must have at least one input file\n\x00")).as_mut_ptr());
            fflush(stderr);
        }
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else if dictBufferCapacity < 256i32 as libc::c_ulong {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 40],
                                              &mut [libc::c_char; 40]>(b"dictBufferCapacity must be at least %u\n\x00")).as_mut_ptr(),
                    256i32);
            fflush(stderr);
        }
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else if 0 ==
                  COVER_ctx_init(&mut ctx as *mut COVER_ctx_t, samplesBuffer,
                                 samplesSizes, nbSamples, parameters.d) {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else if 0 ==
                  COVER_map_init(&mut activeDmers as *mut COVER_map_t,
                                 parameters.k.wrapping_sub(parameters.d).wrapping_add(1i32
                                                                                          as
                                                                                          libc::c_uint))
     {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 44],
                                              &mut [libc::c_char; 44]>(b"Failed to allocate dmer map: out of memory\n\x00")).as_mut_ptr());
            fflush(stderr);
        }
        COVER_ctx_destroy(&mut ctx as *mut COVER_ctx_t);
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        if g_displayLevel >= 2i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 21],
                                              &mut [libc::c_char; 21]>(b"Building dictionary\n\x00")).as_mut_ptr());
            fflush(stderr);
        }
        let tail: size_t =
            COVER_buildDictionary(&mut ctx as *mut COVER_ctx_t, ctx.freqs,
                                  &mut activeDmers as *mut COVER_map_t,
                                  dictBuffer, dictBufferCapacity, parameters);
        let dictionarySize: size_t =
            ZDICT_finalizeDictionary(dict as *mut libc::c_void,
                                     dictBufferCapacity,
                                     dict.offset(tail as isize) as
                                         *const libc::c_void,
                                     dictBufferCapacity.wrapping_sub(tail),
                                     samplesBuffer, samplesSizes, nbSamples,
                                     parameters.zParams);
        if 0 == ZSTD_isError(dictionarySize) {
            if g_displayLevel >= 2i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 35],
                                                  &mut [libc::c_char; 35]>(b"Constructed dictionary of size %u\n\x00")).as_mut_ptr(),
                        dictionarySize as U32);
                fflush(stderr);
            }
        }
        COVER_ctx_destroy(&mut ctx as *mut COVER_ctx_t);
        COVER_map_destroy(&mut activeDmers as *mut COVER_map_t);
        return dictionarySize
    };
}
/* *
 * Given the prepared context build the dictionary.
 */
unsafe extern "C" fn COVER_buildDictionary(mut ctx: *const COVER_ctx_t,
                                           mut freqs: *mut U32,
                                           mut activeDmers: *mut COVER_map_t,
                                           mut dictBuffer: *mut libc::c_void,
                                           mut dictBufferCapacity: size_t,
                                           mut parameters:
                                               ZDICT_cover_params_t)
 -> size_t {
    let dict: *mut BYTE = dictBuffer as *mut BYTE;
    let mut tail: size_t = dictBufferCapacity;
    let epochs: U32 =
        if 1i32 as libc::c_uint >
               dictBufferCapacity.wrapping_div(parameters.k as
                                                   libc::c_ulong).wrapping_div(4i32
                                                                                   as
                                                                                   libc::c_ulong)
                   as U32 {
            1i32 as libc::c_uint
        } else {
            dictBufferCapacity.wrapping_div(parameters.k as
                                                libc::c_ulong).wrapping_div(4i32
                                                                                as
                                                                                libc::c_ulong)
                as U32
        };
    let epochSize: U32 =
        (*ctx).suffixSize.wrapping_div(epochs as libc::c_ulong) as U32;
    let mut epoch: size_t = 0;
    if g_displayLevel >= 2i32 {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 44],
                                          &mut [libc::c_char; 44]>(b"Breaking content into %u epochs of size %u\n\x00")).as_mut_ptr(),
                epochs, epochSize);
        fflush(stderr);
    }
    epoch = 0i32 as size_t;
    while tail > 0i32 as libc::c_ulong {
        let epochBegin: U32 =
            epoch.wrapping_mul(epochSize as libc::c_ulong) as U32;
        let epochEnd: U32 = epochBegin.wrapping_add(epochSize);
        let mut segmentSize: size_t = 0;
        let mut segment: COVER_segment_t =
            COVER_selectSegment(ctx, freqs, activeDmers, epochBegin, epochEnd,
                                parameters);
        if segment.score == 0i32 as libc::c_uint { break ; }
        segmentSize =
            if (segment.end.wrapping_sub(segment.begin).wrapping_add(parameters.d).wrapping_sub(1i32
                                                                                                    as
                                                                                                    libc::c_uint)
                    as libc::c_ulong) < tail {
                segment.end.wrapping_sub(segment.begin).wrapping_add(parameters.d).wrapping_sub(1i32
                                                                                                    as
                                                                                                    libc::c_uint)
                    as libc::c_ulong
            } else { tail };
        if segmentSize < parameters.d as libc::c_ulong { break ; }
        tail =
            (tail as libc::c_ulong).wrapping_sub(segmentSize) as size_t as
                size_t;
        memcpy(dict.offset(tail as isize) as *mut libc::c_void,
               (*ctx).samples.offset(segment.begin as isize) as
                   *const libc::c_void, segmentSize);
        if g_displayLevel >= 2i32 {
            if clock() - g_time > refreshRate || g_displayLevel >= 4i32 {
                g_time = clock();
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 13],
                                                  &mut [libc::c_char; 13]>(b"\r%u%%       \x00")).as_mut_ptr(),
                        dictBufferCapacity.wrapping_sub(tail).wrapping_mul(100i32
                                                                               as
                                                                               libc::c_ulong).wrapping_div(dictBufferCapacity)
                            as U32);
                fflush(stderr);
            }
        }
        epoch =
            epoch.wrapping_add(1i32 as
                                   libc::c_ulong).wrapping_rem(epochs as
                                                                   libc::c_ulong)
    }
    if g_displayLevel >= 2i32 {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 7],
                                          &mut [libc::c_char; 7]>(b"\r%79s\r\x00")).as_mut_ptr(),
                (*::std::mem::transmute::<&[u8; 1],
                                          &mut [libc::c_char; 1]>(b"\x00")).as_mut_ptr());
        fflush(stderr);
    }
    return tail;
}
static mut g_displayLevel: libc::c_int = unsafe { 2i32 };
static mut g_time: clock_t = unsafe { 0i32 as clock_t };
static mut refreshRate: clock_t =
    unsafe {
        1000000i32 as clock_t * 15i32 as libc::c_long / 100i32 as libc::c_long
    };
/* *
 * Selects the best segment in an epoch.
 * Segments of are scored according to the function:
 *
 * Let F(d) be the frequency of dmer d.
 * Let S_i be the dmer at position i of segment S which has length k.
 *
 *     Score(S) = F(S_1) + F(S_2) + ... + F(S_{k-d+1})
 *
 * Once the dmer d is in the dictionay we set F(d) = 0.
 */
unsafe extern "C" fn COVER_selectSegment(mut ctx: *const COVER_ctx_t,
                                         mut freqs: *mut U32,
                                         mut activeDmers: *mut COVER_map_t,
                                         mut begin: U32, mut end: U32,
                                         mut parameters: ZDICT_cover_params_t)
 -> COVER_segment_t {
    let k: U32 = parameters.k;
    let d: U32 = parameters.d;
    let dmersInK: U32 = k.wrapping_sub(d).wrapping_add(1i32 as libc::c_uint);
    let mut bestSegment: COVER_segment_t =
        COVER_segment_t{begin: 0i32 as U32,
                        end: 0i32 as U32,
                        score: 0i32 as U32,};
    let mut activeSegment: COVER_segment_t =
        COVER_segment_t{begin: 0, end: 0, score: 0,};
    COVER_map_clear(activeDmers);
    activeSegment.begin = begin;
    activeSegment.end = begin;
    activeSegment.score = 0i32 as U32;
    while activeSegment.end < end {
        let mut newDmer: U32 =
            *(*ctx).dmerAt.offset(activeSegment.end as isize);
        let mut newDmerOcc: *mut U32 = COVER_map_at(activeDmers, newDmer);
        if *newDmerOcc == 0i32 as libc::c_uint {
            activeSegment.score =
                (activeSegment.score as
                     libc::c_uint).wrapping_add(*freqs.offset(newDmer as
                                                                  isize)) as
                    U32 as U32
        }
        activeSegment.end =
            (activeSegment.end as
                 libc::c_uint).wrapping_add(1i32 as libc::c_uint) as U32 as
                U32;
        *newDmerOcc =
            (*newDmerOcc as libc::c_uint).wrapping_add(1i32 as libc::c_uint)
                as U32 as U32;
        if activeSegment.end.wrapping_sub(activeSegment.begin) ==
               dmersInK.wrapping_add(1i32 as libc::c_uint) {
            let mut delDmer: U32 =
                *(*ctx).dmerAt.offset(activeSegment.begin as isize);
            let mut delDmerOcc: *mut U32 = COVER_map_at(activeDmers, delDmer);
            activeSegment.begin =
                (activeSegment.begin as
                     libc::c_uint).wrapping_add(1i32 as libc::c_uint) as U32
                    as U32;
            *delDmerOcc =
                (*delDmerOcc as
                     libc::c_uint).wrapping_sub(1i32 as libc::c_uint) as U32
                    as U32;
            if *delDmerOcc == 0i32 as libc::c_uint {
                COVER_map_remove(activeDmers, delDmer);
                activeSegment.score =
                    (activeSegment.score as
                         libc::c_uint).wrapping_sub(*freqs.offset(delDmer as
                                                                      isize))
                        as U32 as U32
            }
        }
        if !(activeSegment.score > bestSegment.score) { continue ; }
        bestSegment = activeSegment
    }
    let mut newBegin: U32 = bestSegment.end;
    let mut newEnd: U32 = bestSegment.begin;
    let mut pos: U32 = 0;
    pos = bestSegment.begin;
    while pos != bestSegment.end {
        let mut freq: U32 =
            *freqs.offset(*(*ctx).dmerAt.offset(pos as isize) as isize);
        if freq != 0i32 as libc::c_uint {
            newBegin = if newBegin < pos { newBegin } else { pos };
            newEnd = pos.wrapping_add(1i32 as libc::c_uint)
        }
        pos = pos.wrapping_add(1)
    }
    bestSegment.begin = newBegin;
    bestSegment.end = newEnd;
    let mut pos_0: U32 = 0;
    pos_0 = bestSegment.begin;
    while pos_0 != bestSegment.end {
        *freqs.offset(*(*ctx).dmerAt.offset(pos_0 as isize) as isize) =
            0i32 as U32;
        pos_0 = pos_0.wrapping_add(1)
    }
    return bestSegment;
}
/* *
 * Deletes key from the map if present.
 */
unsafe extern "C" fn COVER_map_remove(mut map: *mut COVER_map_t, mut key: U32)
 -> () {
    let mut i: U32 = COVER_map_index(map, key);
    let mut del: *mut COVER_map_pair_t =
        &mut *(*map).data.offset(i as isize) as *mut COVER_map_pair_t;
    let mut shift: U32 = 1i32 as U32;
    if (*del).value == -1i32 as U32 {
        return
    } else {
        i = i.wrapping_add(1i32 as libc::c_uint) & (*map).sizeMask;
        loop  {
            let pos: *mut COVER_map_pair_t =
                &mut *(*map).data.offset(i as isize) as *mut COVER_map_pair_t;
            if (*pos).value == -1i32 as U32 {
                (*del).value = -1i32 as U32;
                return
            } else {
                if i.wrapping_sub(COVER_map_hash(map, (*pos).key)) &
                       (*map).sizeMask >= shift {
                    (*del).key = (*pos).key;
                    (*del).value = (*pos).value;
                    del = pos;
                    shift = 1i32 as U32
                } else { shift = shift.wrapping_add(1) }
                i = i.wrapping_add(1i32 as libc::c_uint) & (*map).sizeMask
            }
        }
    };
}
/* *
 * Helper function that returns the index that a key should be placed into.
 */
unsafe extern "C" fn COVER_map_index(mut map: *mut COVER_map_t, mut key: U32)
 -> U32 {
    let hash: U32 = COVER_map_hash(map, key);
    let mut i: U32 = 0;
    i = hash;
    loop  {
        let mut pos: *mut COVER_map_pair_t =
            &mut *(*map).data.offset(i as isize) as *mut COVER_map_pair_t;
        if (*pos).value == -1i32 as U32 {
            return i
        } else if (*pos).key == key {
            return i
        } else { i = i.wrapping_add(1i32 as libc::c_uint) & (*map).sizeMask }
    };
}
unsafe extern "C" fn COVER_map_hash(mut map: *mut COVER_map_t, mut key: U32)
 -> U32 {
    return key.wrapping_mul(prime4bytes) >>
               (32i32 as libc::c_uint).wrapping_sub((*map).sizeLog);
}
/* *
 * Internal hash function
 */
static mut prime4bytes: U32 = unsafe { 2654435761u32 };
/* *
 * Returns the pointer to the value for key.
 * If key is not in the map, it is inserted and the value is set to 0.
 * The map must not be full.
 */
unsafe extern "C" fn COVER_map_at(mut map: *mut COVER_map_t, mut key: U32)
 -> *mut U32 {
    let mut pos: *mut COVER_map_pair_t =
        &mut *(*map).data.offset(COVER_map_index(map, key) as isize) as
            *mut COVER_map_pair_t;
    if (*pos).value == -1i32 as U32 {
        (*pos).key = key;
        (*pos).value = 0i32 as U32
    }
    return &mut (*pos).value as *mut U32;
}
/* *
 * Clear the map.
 */
unsafe extern "C" fn COVER_map_clear(mut map: *mut COVER_map_t) -> () {
    memset((*map).data as *mut libc::c_void, -1i32 as U32 as libc::c_int,
           ((*map).size as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<COVER_map_pair_t>()
                                                as libc::c_ulong));
}
/* *
 * Destroyes a map that is inited with COVER_map_init().
 */
unsafe extern "C" fn COVER_map_destroy(mut map: *mut COVER_map_t) -> () {
    if !(*map).data.is_null() { free((*map).data as *mut libc::c_void); }
    (*map).data = 0 as *mut COVER_map_pair_t;
    (*map).size = 0i32 as U32;
}
/* *
 * Clean up a context initialized with `COVER_ctx_init()`.
 */
unsafe extern "C" fn COVER_ctx_destroy(mut ctx: *mut COVER_ctx_t) -> () {
    if ctx.is_null() {
        return
    } else {
        if !(*ctx).suffix.is_null() {
            free((*ctx).suffix as *mut libc::c_void);
            (*ctx).suffix = 0 as *mut U32
        }
        if !(*ctx).freqs.is_null() {
            free((*ctx).freqs as *mut libc::c_void);
            (*ctx).freqs = 0 as *mut U32
        }
        if !(*ctx).dmerAt.is_null() {
            free((*ctx).dmerAt as *mut libc::c_void);
            (*ctx).dmerAt = 0 as *mut U32
        }
        if !(*ctx).offsets.is_null() {
            free((*ctx).offsets as *mut libc::c_void);
            (*ctx).offsets = 0 as *mut size_t
        }
        return;
    };
}
/* *
 * Initializes a map of the given size.
 * Returns 1 on success and 0 on failure.
 * The map must be destroyed with COVER_map_destroy().
 * The map is only guaranteed to be large enough to hold size elements.
 */
unsafe extern "C" fn COVER_map_init(mut map: *mut COVER_map_t, mut size: U32)
 -> libc::c_int {
    (*map).sizeLog = ZSTD_highbit32(size).wrapping_add(2i32 as libc::c_uint);
    (*map).size = (1i32 as U32) << (*map).sizeLog;
    (*map).sizeMask = (*map).size.wrapping_sub(1i32 as libc::c_uint);
    (*map).data =
        malloc(((*map).size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<COVER_map_pair_t>()
                                                    as libc::c_ulong)) as
            *mut COVER_map_pair_t;
    if (*map).data.is_null() {
        (*map).sizeLog = 0i32 as U32;
        (*map).size = 0i32 as U32;
        return 0i32
    } else { COVER_map_clear(map); return 1i32 };
}
/* *
 * Prepare a context for dictionary building.
 * The context is only dependent on the parameter `d` and can used multiple
 * times.
 * Returns 1 on success or zero on error.
 * The context must be destroyed with `COVER_ctx_destroy()`.
 */
unsafe extern "C" fn COVER_ctx_init(mut ctx: *mut COVER_ctx_t,
                                    mut samplesBuffer: *const libc::c_void,
                                    mut samplesSizes: *const size_t,
                                    mut nbSamples: libc::c_uint,
                                    mut d: libc::c_uint) -> libc::c_int {
    let samples: *const BYTE = samplesBuffer as *const BYTE;
    let totalSamplesSize: size_t = COVER_sum(samplesSizes, nbSamples);
    if totalSamplesSize <
           if d as libc::c_ulong >
                  ::std::mem::size_of::<U64>() as libc::c_ulong {
               d as libc::c_ulong
           } else { ::std::mem::size_of::<U64>() as libc::c_ulong } ||
           totalSamplesSize >=
               (if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                       8i32 as libc::c_ulong {
                    -1i32 as U32
                } else { (1i32 as U32).wrapping_mul(1u32 << 30i32) }) as
                   size_t {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 64],
                                              &mut [libc::c_char; 64]>(b"Total samples size is too large (%u MB), maximum size is %u MB\n\x00")).as_mut_ptr(),
                    (totalSamplesSize >> 20i32) as U32,
                    if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                           8i32 as libc::c_ulong {
                        -1i32 as U32
                    } else { (1i32 as U32).wrapping_mul(1u32 << 30i32) } >>
                        20i32);
            fflush(stderr);
        }
        return 0i32
    } else {
        memset(ctx as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<COVER_ctx_t>() as libc::c_ulong);
        if g_displayLevel >= 2i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 41],
                                              &mut [libc::c_char; 41]>(b"Training on %u samples of total size %u\n\x00")).as_mut_ptr(),
                    nbSamples, totalSamplesSize as U32);
            fflush(stderr);
        }
        (*ctx).samples = samples;
        (*ctx).samplesSizes = samplesSizes;
        (*ctx).nbSamples = nbSamples as size_t;
        (*ctx).suffixSize =
            totalSamplesSize.wrapping_sub(if d as libc::c_ulong >
                                                 ::std::mem::size_of::<U64>()
                                                     as libc::c_ulong {
                                              d as libc::c_ulong
                                          } else {
                                              ::std::mem::size_of::<U64>() as
                                                  libc::c_ulong
                                          }).wrapping_add(1i32 as
                                                              libc::c_ulong);
        (*ctx).suffix =
            malloc((*ctx).suffixSize.wrapping_mul(::std::mem::size_of::<U32>()
                                                      as libc::c_ulong)) as
                *mut U32;
        (*ctx).dmerAt =
            malloc((*ctx).suffixSize.wrapping_mul(::std::mem::size_of::<U32>()
                                                      as libc::c_ulong)) as
                *mut U32;
        (*ctx).offsets =
            malloc((nbSamples.wrapping_add(1i32 as libc::c_uint) as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>()
                                                        as libc::c_ulong)) as
                *mut size_t;
        if (*ctx).suffix.is_null() || (*ctx).dmerAt.is_null() ||
               (*ctx).offsets.is_null() {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 36],
                                                  &mut [libc::c_char; 36]>(b"Failed to allocate scratch buffers\n\x00")).as_mut_ptr());
                fflush(stderr);
            }
            COVER_ctx_destroy(ctx);
            return 0i32
        } else {
            (*ctx).freqs = 0 as *mut U32;
            (*ctx).d = d;
            let mut i: U32 = 0;
            *(*ctx).offsets.offset(0isize) = 0i32 as size_t;
            i = 1i32 as U32;
            while i <= nbSamples {
                *(*ctx).offsets.offset(i as isize) =
                    (*(*ctx).offsets.offset(i.wrapping_sub(1i32 as
                                                               libc::c_uint)
                                                as
                                                isize)).wrapping_add(*samplesSizes.offset(i.wrapping_sub(1i32
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                              as
                                                                                              isize));
                i = i.wrapping_add(1)
            }
            if g_displayLevel >= 2i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 35],
                                                  &mut [libc::c_char; 35]>(b"Constructing partial suffix array\n\x00")).as_mut_ptr());
                fflush(stderr);
            }
            let mut i_0: U32 = 0;
            i_0 = 0i32 as U32;
            while (i_0 as libc::c_ulong) < (*ctx).suffixSize {
                *(*ctx).suffix.offset(i_0 as isize) = i_0;
                i_0 = i_0.wrapping_add(1)
            }
            g_ctx = ctx;
            qsort((*ctx).suffix as *mut libc::c_void, (*ctx).suffixSize,
                  ::std::mem::size_of::<U32>() as libc::c_ulong,
                  if (*ctx).d <= 8i32 as libc::c_uint {
                      Some(COVER_strict_cmp8)
                  } else { Some(COVER_strict_cmp) });
            if g_displayLevel >= 2i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 23],
                                                  &mut [libc::c_char; 23]>(b"Computing frequencies\n\x00")).as_mut_ptr());
                fflush(stderr);
            }
            COVER_groupBy((*ctx).suffix as *const libc::c_void,
                          (*ctx).suffixSize,
                          ::std::mem::size_of::<U32>() as libc::c_ulong, ctx,
                          if (*ctx).d <= 8i32 as libc::c_uint {
                              Some(COVER_cmp8)
                          } else { Some(COVER_cmp) }, Some(COVER_group));
            (*ctx).freqs = (*ctx).suffix;
            (*ctx).suffix = 0 as *mut U32;
            return 1i32
        }
    };
}
/* *
 * Called on each group of positions with the same dmer.
 * Counts the frequency of each dmer and saves it in the suffix array.
 * Fills `ctx->dmerAt`.
 */
unsafe extern "C" fn COVER_group(mut ctx: *mut COVER_ctx_t,
                                 mut group: *const libc::c_void,
                                 mut groupEnd: *const libc::c_void) -> () {
    let mut grpPtr: *const U32 = group as *const U32;
    let mut grpEnd: *const U32 = groupEnd as *const U32;
    let dmerId: U32 =
        (*ctx).suffix.offset_to(grpPtr).expect("bad offset_to") as
            libc::c_long as U32;
    let mut freq: U32 = 0i32 as U32;
    let mut curOffsetPtr: *const size_t = (*ctx).offsets;
    let mut offsetsEnd: *const size_t =
        (*ctx).offsets.offset((*ctx).nbSamples as isize);
    let mut curSampleEnd: size_t = *(*ctx).offsets.offset(0isize);
    while grpPtr != grpEnd {
        *(*ctx).dmerAt.offset(*grpPtr as isize) = dmerId;
        if !((*grpPtr as libc::c_ulong) < curSampleEnd) {
            freq =
                (freq as libc::c_uint).wrapping_add(1i32 as libc::c_uint) as
                    U32 as U32;
            if grpPtr.offset(1isize) != grpEnd {
                let mut sampleEndPtr: *const size_t =
                    COVER_lower_bound(curOffsetPtr, offsetsEnd,
                                      *grpPtr as size_t);
                curSampleEnd = *sampleEndPtr;
                curOffsetPtr = sampleEndPtr.offset(1isize)
            }
        }
        grpPtr = grpPtr.offset(1isize)
    }
    *(*ctx).suffix.offset(dmerId as isize) = freq;
}
/* *
 * Returns the first pointer in [first, last) whose element does not compare
 * less than value.  If no such element exists it returns last.
 */
unsafe extern "C" fn COVER_lower_bound(mut first: *const size_t,
                                       mut last: *const size_t,
                                       mut value: size_t) -> *const size_t {
    let mut count: size_t =
        first.offset_to(last).expect("bad offset_to") as libc::c_long as
            size_t;
    while count != 0i32 as libc::c_ulong {
        let mut step: size_t = count.wrapping_div(2i32 as libc::c_ulong);
        let mut ptr: *const size_t = first;
        ptr = ptr.offset(step as isize);
        if *ptr < value {
            ptr = ptr.offset(1isize);
            first = ptr;
            count =
                (count as
                     libc::c_ulong).wrapping_sub(step.wrapping_add(1i32 as
                                                                       libc::c_ulong))
                    as size_t as size_t
        } else { count = step }
    }
    return first;
}
/* *
 * Returns -1 if the dmer at lp is less than the dmer at rp.
 * Return 0 if the dmers at lp and rp are equal.
 * Returns 1 if the dmer at lp is greater than the dmer at rp.
 */
unsafe extern "C" fn COVER_cmp(mut ctx: *mut COVER_ctx_t,
                               mut lp: *const libc::c_void,
                               mut rp: *const libc::c_void) -> libc::c_int {
    let lhs: U32 = *(lp as *const U32);
    let rhs: U32 = *(rp as *const U32);
    return memcmp((*ctx).samples.offset(lhs as isize) as *const libc::c_void,
                  (*ctx).samples.offset(rhs as isize) as *const libc::c_void,
                  (*ctx).d as libc::c_ulong);
}
/* *
 * Faster version for d <= 8.
 */
unsafe extern "C" fn COVER_cmp8(mut ctx: *mut COVER_ctx_t,
                                mut lp: *const libc::c_void,
                                mut rp: *const libc::c_void) -> libc::c_int {
    let mask: U64 =
        if (*ctx).d == 8i32 as libc::c_uint {
            -1i32 as U64
        } else {
            ((1i32 as U64) <<
                 (8i32 as
                      libc::c_uint).wrapping_mul((*ctx).d)).wrapping_sub(1i32
                                                                             as
                                                                             libc::c_ulong)
        };
    let lhs: U64 =
        MEM_readLE64((*ctx).samples.offset(*(lp as *const U32) as isize) as
                         *const libc::c_void) & mask;
    let rhs: U64 =
        MEM_readLE64((*ctx).samples.offset(*(rp as *const U32) as isize) as
                         *const libc::c_void) & mask;
    if lhs < rhs { return -1i32 } else { return (lhs > rhs) as libc::c_int };
}
/* *
 * Generic groupBy function.
 * Groups an array sorted by cmp into groups with equivalent values.
 * Calls grp for each group.
 */
unsafe extern "C" fn COVER_groupBy(mut data: *const libc::c_void,
                                   mut count: size_t, mut size: size_t,
                                   mut ctx: *mut COVER_ctx_t,
                                   mut cmp:
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut COVER_ctx_t,
                                                                   _:
                                                                       *const libc::c_void,
                                                                   _:
                                                                       *const libc::c_void)
                                                  -> libc::c_int>,
                                   mut grp:
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut COVER_ctx_t,
                                                                   _:
                                                                       *const libc::c_void,
                                                                   _:
                                                                       *const libc::c_void)
                                                  -> ()>) -> () {
    let mut ptr: *const BYTE = data as *const BYTE;
    let mut num: size_t = 0i32 as size_t;
    while num < count {
        let mut grpEnd: *const BYTE = ptr.offset(size as isize);
        num = num.wrapping_add(1);
        while num < count &&
                  cmp.expect("non-null function pointer")(ctx,
                                                          ptr as
                                                              *const libc::c_void,
                                                          grpEnd as
                                                              *const libc::c_void)
                      == 0i32 {
            grpEnd = grpEnd.offset(size as isize);
            num = num.wrapping_add(1)
        }
        grp.expect("non-null function pointer")(ctx,
                                                ptr as *const libc::c_void,
                                                grpEnd as
                                                    *const libc::c_void);
        ptr = grpEnd
    };
}
/* *
 * Same as COVER_cmp() except ties are broken by pointer value
 * NOTE: g_ctx must be set to call this function.  A global is required because
 * qsort doesn't take an opaque pointer.
 */
unsafe extern "C" fn COVER_strict_cmp(mut lp: *const libc::c_void,
                                      mut rp: *const libc::c_void)
 -> libc::c_int {
    let mut result: libc::c_int = COVER_cmp(g_ctx, lp, rp);
    if result == 0i32 { result = if lp < rp { -1i32 } else { 1i32 } }
    return result;
}
static mut g_ctx: *mut COVER_ctx_t =
    unsafe { 0 as *const COVER_ctx_t as *mut COVER_ctx_t };
/* *
 * Faster version for d <= 8.
 */
unsafe extern "C" fn COVER_strict_cmp8(mut lp: *const libc::c_void,
                                       mut rp: *const libc::c_void)
 -> libc::c_int {
    let mut result: libc::c_int = COVER_cmp8(g_ctx, lp, rp);
    if result == 0i32 { result = if lp < rp { -1i32 } else { 1i32 } }
    return result;
}
/* *
 * Returns the sum of the sample sizes.
 */
unsafe extern "C" fn COVER_sum(mut samplesSizes: *const size_t,
                               mut nbSamples: libc::c_uint) -> size_t {
    let mut sum: size_t = 0i32 as size_t;
    let mut i: size_t = 0;
    i = 0i32 as size_t;
    while i < nbSamples as libc::c_ulong {
        sum =
            (sum as
                 libc::c_ulong).wrapping_add(*samplesSizes.offset(i as isize))
                as size_t as size_t;
        i = i.wrapping_add(1)
    }
    return sum;
}
/* *
 * Check the validity of the parameters.
 * Returns non-zero if the parameters are valid and 0 otherwise.
 */
unsafe extern "C" fn COVER_checkParameters(mut parameters:
                                               ZDICT_cover_params_t,
                                           mut maxDictSize: size_t)
 -> libc::c_int {
    if parameters.d == 0i32 as libc::c_uint ||
           parameters.k == 0i32 as libc::c_uint {
        return 0i32
    } else if parameters.k as libc::c_ulong > maxDictSize {
        return 0i32
    } else if parameters.d > parameters.k {
        return 0i32
    } else { return 1i32 };
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_optimizeTrainFromBuffer_cover(mut dictBuffer:
                                                                 *mut libc::c_void,
                                                             mut dictBufferCapacity:
                                                                 size_t,
                                                             mut samplesBuffer:
                                                                 *const libc::c_void,
                                                             mut samplesSizes:
                                                                 *const size_t,
                                                             mut nbSamples:
                                                                 libc::c_uint,
                                                             mut parameters:
                                                                 *mut ZDICT_cover_params_t)
 -> size_t {
    let nbThreads: libc::c_uint = (*parameters).nbThreads;
    let kMinD: libc::c_uint =
        if (*parameters).d == 0i32 as libc::c_uint {
            6i32 as libc::c_uint
        } else { (*parameters).d };
    let kMaxD: libc::c_uint =
        if (*parameters).d == 0i32 as libc::c_uint {
            8i32 as libc::c_uint
        } else { (*parameters).d };
    let kMinK: libc::c_uint =
        if (*parameters).k == 0i32 as libc::c_uint {
            50i32 as libc::c_uint
        } else { (*parameters).k };
    let kMaxK: libc::c_uint =
        if (*parameters).k == 0i32 as libc::c_uint {
            2000i32 as libc::c_uint
        } else { (*parameters).k };
    let kSteps: libc::c_uint =
        if (*parameters).steps == 0i32 as libc::c_uint {
            40i32 as libc::c_uint
        } else { (*parameters).steps };
    let kStepSize: libc::c_uint =
        if kMaxK.wrapping_sub(kMinK).wrapping_div(kSteps) >
               1i32 as libc::c_uint {
            kMaxK.wrapping_sub(kMinK).wrapping_div(kSteps)
        } else { 1i32 as libc::c_uint };
    let kIterations: libc::c_uint =
        (1i32 as
             libc::c_uint).wrapping_add(kMaxD.wrapping_sub(kMinD).wrapping_div(2i32
                                                                                   as
                                                                                   libc::c_uint)).wrapping_mul((1i32
                                                                                                                    as
                                                                                                                    libc::c_uint).wrapping_add(kMaxK.wrapping_sub(kMinK).wrapping_div(kStepSize)));
    let displayLevel: libc::c_int =
        (*parameters).zParams.notificationLevel as libc::c_int;
    let mut iteration: libc::c_uint = 1i32 as libc::c_uint;
    let mut d: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut best: COVER_best_t =
        COVER_best_s{mutex:
                         pthread_mutex_t{__data:
                                             __pthread_mutex_s{__lock: 0,
                                                               __count: 0,
                                                               __owner: 0,
                                                               __nusers: 0,
                                                               __kind: 0,
                                                               __spins: 0,
                                                               __elision: 0,
                                                               __list:
                                                                   __pthread_internal_list{__prev:
                                                                                               0
                                                                                                   as
                                                                                                   *mut __pthread_internal_list,
                                                                                           __next:
                                                                                               0
                                                                                                   as
                                                                                                   *mut __pthread_internal_list,},},},
                     cond:
                         pthread_cond_t{__data:
                                            unnamed_2{__lock: 0,
                                                      __futex: 0,
                                                      __total_seq: 0,
                                                      __wakeup_seq: 0,
                                                      __woken_seq: 0,
                                                      __mutex:
                                                          0 as
                                                              *mut libc::c_void,
                                                      __nwaiters: 0,
                                                      __broadcast_seq: 0,},},
                     liveJobs: 0,
                     dict: 0 as *mut libc::c_void,
                     dictSize: 0,
                     parameters:
                         ZDICT_cover_params_t{k: 0,
                                              d: 0,
                                              steps: 0,
                                              nbThreads: 0,
                                              zParams:
                                                  ZDICT_params_t{compressionLevel:
                                                                     0,
                                                                 notificationLevel:
                                                                     0,
                                                                 dictID:
                                                                     0,},},
                     compressedSize: 0,};
    let mut pool: *mut POOL_ctx = 0 as *mut POOL_ctx;
    if kMinK < kMaxD || kMaxK < kMinK {
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 22],
                                              &mut [libc::c_char; 22]>(b"Incorrect parameters\n\x00")).as_mut_ptr());
            fflush(stderr);
        }
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else if nbSamples == 0i32 as libc::c_uint {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 41],
                                              &mut [libc::c_char; 41]>(b"Cover must have at least one input file\n\x00")).as_mut_ptr());
            fflush(stderr);
        }
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else if dictBufferCapacity < 256i32 as libc::c_ulong {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 40],
                                              &mut [libc::c_char; 40]>(b"dictBufferCapacity must be at least %u\n\x00")).as_mut_ptr(),
                    256i32);
            fflush(stderr);
        }
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else {
        if nbThreads > 1i32 as libc::c_uint {
            pool = POOL_create(nbThreads as size_t, 1i32 as size_t);
            if pool.is_null() {
                return -(ZSTD_error_memory_allocation as libc::c_int) as
                           size_t
            }
        }
        COVER_best_init(&mut best as *mut COVER_best_t);
        g_displayLevel =
            if displayLevel == 0i32 { 0i32 } else { displayLevel - 1i32 };
        if displayLevel >= 2i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 40],
                                              &mut [libc::c_char; 40]>(b"Trying %u different sets of parameters\n\x00")).as_mut_ptr(),
                    kIterations);
            fflush(stderr);
        }
        d = kMinD;
        while d <= kMaxD {
            let mut ctx: COVER_ctx_t =
                COVER_ctx_t{samples: 0 as *const BYTE,
                            offsets: 0 as *mut size_t,
                            samplesSizes: 0 as *const size_t,
                            nbSamples: 0,
                            suffix: 0 as *mut U32,
                            suffixSize: 0,
                            freqs: 0 as *mut U32,
                            dmerAt: 0 as *mut U32,
                            d: 0,};
            if displayLevel >= 3i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 6],
                                                  &mut [libc::c_char; 6]>(b"d=%u\n\x00")).as_mut_ptr(),
                        d);
                fflush(stderr);
            }
            if 0 ==
                   COVER_ctx_init(&mut ctx as *mut COVER_ctx_t, samplesBuffer,
                                  samplesSizes, nbSamples, d) {
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 30],
                                                      &mut [libc::c_char; 30]>(b"Failed to initialize context\n\x00")).as_mut_ptr());
                    fflush(stderr);
                }
                COVER_best_destroy(&mut best as *mut COVER_best_t);
                POOL_free(pool);
                return -(ZSTD_error_GENERIC as libc::c_int) as size_t
            } else {
                k = kMinK;
                while k <= kMaxK {
                    let mut data: *mut COVER_tryParameters_data_t =
                        malloc(::std::mem::size_of::<COVER_tryParameters_data_t>()
                                   as libc::c_ulong) as
                            *mut COVER_tryParameters_data_t;
                    if displayLevel >= 3i32 {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 6],
                                                          &mut [libc::c_char; 6]>(b"k=%u\n\x00")).as_mut_ptr(),
                                k);
                        fflush(stderr);
                    }
                    if data.is_null() {
                        if displayLevel >= 1i32 {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 31],
                                                              &mut [libc::c_char; 31]>(b"Failed to allocate parameters\n\x00")).as_mut_ptr());
                            fflush(stderr);
                        }
                        COVER_best_destroy(&mut best as *mut COVER_best_t);
                        COVER_ctx_destroy(&mut ctx as *mut COVER_ctx_t);
                        POOL_free(pool);
                        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
                    } else {
                        (*data).ctx = &mut ctx as *mut COVER_ctx_t;
                        (*data).best = &mut best as *mut COVER_best_t;
                        (*data).dictBufferCapacity = dictBufferCapacity;
                        (*data).parameters = *parameters;
                        (*data).parameters.k = k;
                        (*data).parameters.d = d;
                        (*data).parameters.steps = kSteps;
                        (*data).parameters.zParams.notificationLevel =
                            g_displayLevel as libc::c_uint;
                        if 0 ==
                               COVER_checkParameters((*data).parameters,
                                                     dictBufferCapacity) {
                            if g_displayLevel >= 1i32 {
                                fprintf(stderr,
                                        (*::std::mem::transmute::<&[u8; 28],
                                                                  &mut [libc::c_char; 28]>(b"Cover parameters incorrect\n\x00")).as_mut_ptr());
                                fflush(stderr);
                            }
                            free(data as *mut libc::c_void);
                        } else {
                            COVER_best_start(&mut best as *mut COVER_best_t);
                            if !pool.is_null() {
                                POOL_add(pool, Some(COVER_tryParameters),
                                         data as *mut libc::c_void);
                            } else {
                                COVER_tryParameters(data as
                                                        *mut libc::c_void);
                            }
                            if displayLevel >= 2i32 {
                                if clock() - g_time > refreshRate ||
                                       displayLevel >= 4i32 {
                                    g_time = clock();
                                    fprintf(stderr,
                                            (*::std::mem::transmute::<&[u8; 13],
                                                                      &mut [libc::c_char; 13]>(b"\r%u%%       \x00")).as_mut_ptr(),
                                            iteration.wrapping_mul(100i32 as
                                                                       libc::c_uint).wrapping_div(kIterations));
                                    fflush(stderr);
                                }
                            }
                            iteration = iteration.wrapping_add(1)
                        }
                        k = k.wrapping_add(kStepSize)
                    }
                }
                COVER_best_wait(&mut best as *mut COVER_best_t);
                COVER_ctx_destroy(&mut ctx as *mut COVER_ctx_t);
                d = d.wrapping_add(2i32 as libc::c_uint)
            }
        }
        if displayLevel >= 2i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 7],
                                              &mut [libc::c_char; 7]>(b"\r%79s\r\x00")).as_mut_ptr(),
                    (*::std::mem::transmute::<&[u8; 1],
                                              &mut [libc::c_char; 1]>(b"\x00")).as_mut_ptr());
            fflush(stderr);
        }
        let dictSize: size_t = best.dictSize;
        if 0 != ZSTD_isError(best.compressedSize) {
            let compressedSize: size_t = best.compressedSize;
            COVER_best_destroy(&mut best as *mut COVER_best_t);
            POOL_free(pool);
            return compressedSize
        } else {
            *parameters = best.parameters;
            memcpy(dictBuffer, best.dict, dictSize);
            COVER_best_destroy(&mut best as *mut COVER_best_t);
            POOL_free(pool);
            return dictSize
        }
    };
}
/* *
 * Call COVER_best_wait() and then destroy the COVER_best_t.
 */
unsafe extern "C" fn COVER_best_destroy(mut best: *mut COVER_best_t) -> () {
    if best.is_null() {
        return
    } else {
        COVER_best_wait(best);
        if !(*best).dict.is_null() { free((*best).dict); }
        pthread_mutex_destroy(&mut (*best).mutex as *mut pthread_mutex_t);
        pthread_cond_destroy(&mut (*best).cond as *mut pthread_cond_t);
        return;
    };
}
/* *
 * Wait until liveJobs == 0.
 */
unsafe extern "C" fn COVER_best_wait(mut best: *mut COVER_best_t) -> () {
    if best.is_null() {
        return
    } else {
        pthread_mutex_lock(&mut (*best).mutex as *mut pthread_mutex_t);
        while (*best).liveJobs != 0i32 as libc::c_ulong {
            pthread_cond_wait(&mut (*best).cond as *mut pthread_cond_t,
                              &mut (*best).mutex as *mut pthread_mutex_t);
        }
        pthread_mutex_unlock(&mut (*best).mutex as *mut pthread_mutex_t);
        return;
    };
}
/* *
 * Tries a set of parameters and upates the COVER_best_t with the results.
 * This function is thread safe if zstd is compiled with multithreaded support.
 * It takes its parameters as an *OWNING* opaque pointer to support threading.
 */
unsafe extern "C" fn COVER_tryParameters(mut opaque: *mut libc::c_void)
 -> () {
    let data: *mut COVER_tryParameters_data_t =
        opaque as *mut COVER_tryParameters_data_t;
    let ctx: *const COVER_ctx_t = (*data).ctx;
    let parameters: ZDICT_cover_params_t = (*data).parameters;
    let mut dictBufferCapacity: size_t = (*data).dictBufferCapacity;
    let mut totalCompressedSize: size_t =
        -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    let mut activeDmers: COVER_map_t =
        COVER_map_s{data: 0 as *mut COVER_map_pair_t,
                    sizeLog: 0,
                    size: 0,
                    sizeMask: 0,};
    let dict: *mut BYTE = malloc(dictBufferCapacity) as *mut BYTE;
    let mut freqs: *mut U32 =
        malloc((*ctx).suffixSize.wrapping_mul(::std::mem::size_of::<U32>() as
                                                  libc::c_ulong)) as *mut U32;
    if 0 ==
           COVER_map_init(&mut activeDmers as *mut COVER_map_t,
                          parameters.k.wrapping_sub(parameters.d).wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint))
       {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 44],
                                              &mut [libc::c_char; 44]>(b"Failed to allocate dmer map: out of memory\n\x00")).as_mut_ptr());
            fflush(stderr);
        }
    } else if dict.is_null() || freqs.is_null() {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 43],
                                              &mut [libc::c_char; 43]>(b"Failed to allocate buffers: out of memory\n\x00")).as_mut_ptr());
            fflush(stderr);
        }
    } else {
        memcpy(freqs as *mut libc::c_void,
               (*ctx).freqs as *const libc::c_void,
               (*ctx).suffixSize.wrapping_mul(::std::mem::size_of::<U32>() as
                                                  libc::c_ulong));
        let tail: size_t =
            COVER_buildDictionary(ctx, freqs,
                                  &mut activeDmers as *mut COVER_map_t,
                                  dict as *mut libc::c_void,
                                  dictBufferCapacity, parameters);
        dictBufferCapacity =
            ZDICT_finalizeDictionary(dict as *mut libc::c_void,
                                     dictBufferCapacity,
                                     dict.offset(tail as isize) as
                                         *const libc::c_void,
                                     dictBufferCapacity.wrapping_sub(tail),
                                     (*ctx).samples as *const libc::c_void,
                                     (*ctx).samplesSizes,
                                     (*ctx).nbSamples as libc::c_uint,
                                     parameters.zParams);
        if 0 != ZDICT_isError(dictBufferCapacity) {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 31],
                                                  &mut [libc::c_char; 31]>(b"Failed to finalize dictionary\n\x00")).as_mut_ptr());
                fflush(stderr);
            }
        } else {
            let mut cctx: *mut ZSTD_CCtx = 0 as *mut ZSTD_CCtx;
            let mut cdict: *mut ZSTD_CDict = 0 as *mut ZSTD_CDict;
            let mut dst: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut dstCapacity: size_t = 0;
            let mut i: size_t = 0;
            let mut maxSampleSize: size_t = 0i32 as size_t;
            i = 0i32 as size_t;
            while i < (*ctx).nbSamples {
                maxSampleSize =
                    if *(*ctx).samplesSizes.offset(i as isize) > maxSampleSize
                       {
                        *(*ctx).samplesSizes.offset(i as isize)
                    } else { maxSampleSize };
                i = i.wrapping_add(1)
            }
            dstCapacity = ZSTD_compressBound(maxSampleSize);
            dst = malloc(dstCapacity);
            cctx = ZSTD_createCCtx();
            cdict =
                ZSTD_createCDict(dict as *const libc::c_void,
                                 dictBufferCapacity,
                                 parameters.zParams.compressionLevel);
            if !(dst.is_null() || cctx.is_null() || cdict.is_null()) {
                totalCompressedSize = dictBufferCapacity;
                i = 0i32 as size_t;
                while i < (*ctx).nbSamples {
                    let size: size_t =
                        ZSTD_compress_usingCDict(cctx, dst, dstCapacity,
                                                 (*ctx).samples.offset(*(*ctx).offsets.offset(i
                                                                                                  as
                                                                                                  isize)
                                                                           as
                                                                           isize)
                                                     as *const libc::c_void,
                                                 *(*ctx).samplesSizes.offset(i
                                                                                 as
                                                                                 isize),
                                                 cdict);
                    if 0 != ZSTD_isError(size) {
                        totalCompressedSize =
                            -(ZSTD_error_GENERIC as libc::c_int) as size_t;
                        break ;
                    } else {
                        totalCompressedSize =
                            (totalCompressedSize as
                                 libc::c_ulong).wrapping_add(size) as size_t
                                as size_t;
                        i = i.wrapping_add(1)
                    }
                }
            }
            ZSTD_freeCCtx(cctx);
            ZSTD_freeCDict(cdict);
            if !dst.is_null() { free(dst); }
        }
    }
    COVER_best_finish((*data).best, totalCompressedSize, parameters,
                      dict as *mut libc::c_void, dictBufferCapacity);
    free(data as *mut libc::c_void);
    COVER_map_destroy(&mut activeDmers as *mut COVER_map_t);
    if !dict.is_null() { free(dict as *mut libc::c_void); }
    if !freqs.is_null() { free(freqs as *mut libc::c_void); };
}
/* *
 * Called when a thread finishes executing, both on error or success.
 * Decrements liveJobs and signals any waiting threads if liveJobs == 0.
 * If this dictionary is the best so far save it and its parameters.
 */
unsafe extern "C" fn COVER_best_finish(mut best: *mut COVER_best_t,
                                       mut compressedSize: size_t,
                                       mut parameters: ZDICT_cover_params_t,
                                       mut dict: *mut libc::c_void,
                                       mut dictSize: size_t) -> () {
    if best.is_null() {
        return
    } else {
        let mut liveJobs: size_t = 0;
        pthread_mutex_lock(&mut (*best).mutex as *mut pthread_mutex_t);
        (*best).liveJobs = (*best).liveJobs.wrapping_sub(1);
        liveJobs = (*best).liveJobs;
        if compressedSize < (*best).compressedSize {
            if (*best).dict.is_null() || (*best).dictSize < dictSize {
                if !(*best).dict.is_null() { free((*best).dict); }
                (*best).dict = malloc(dictSize);
                if (*best).dict.is_null() {
                    (*best).compressedSize =
                        -(ZSTD_error_GENERIC as libc::c_int) as size_t;
                    (*best).dictSize = 0i32 as size_t;
                    return
                }
            }
            memcpy((*best).dict, dict, dictSize);
            (*best).dictSize = dictSize;
            (*best).parameters = parameters;
            (*best).compressedSize = compressedSize
        }
        pthread_mutex_unlock(&mut (*best).mutex as *mut pthread_mutex_t);
        if liveJobs == 0i32 as libc::c_ulong {
            pthread_cond_broadcast(&mut (*best).cond as *mut pthread_cond_t);
        }
        return;
    };
}
/* *
 * Called when a thread is about to be launched.
 * Increments liveJobs.
 */
unsafe extern "C" fn COVER_best_start(mut best: *mut COVER_best_t) -> () {
    if best.is_null() {
        return
    } else {
        pthread_mutex_lock(&mut (*best).mutex as *mut pthread_mutex_t);
        (*best).liveJobs = (*best).liveJobs.wrapping_add(1);
        pthread_mutex_unlock(&mut (*best).mutex as *mut pthread_mutex_t);
        return;
    };
}
/* *
 * Initialize the `COVER_best_t`.
 */
unsafe extern "C" fn COVER_best_init(mut best: *mut COVER_best_t) -> () {
    if best.is_null() {
        return
    } else {
        pthread_mutex_init(&mut (*best).mutex as *mut pthread_mutex_t,
                           0 as *const pthread_mutexattr_t);
        pthread_cond_init(&mut (*best).cond as *mut pthread_cond_t,
                          0 as *const pthread_condattr_t);
        (*best).liveJobs = 0i32 as size_t;
        (*best).dict = 0 as *mut libc::c_void;
        (*best).dictSize = 0i32 as size_t;
        (*best).compressedSize = -1i32 as size_t;
        memset(&mut (*best).parameters as *mut ZDICT_cover_params_t as
                   *mut libc::c_void, 0i32,
               ::std::mem::size_of::<ZDICT_cover_params_t>() as
                   libc::c_ulong);
        return;
    };
}
