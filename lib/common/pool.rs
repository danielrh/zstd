#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( libc , offset_to )]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
    #[no_mangle]
    fn ZSTD_malloc(size: size_t, customMem: ZSTD_customMem)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ZSTD_calloc(size: size_t, customMem: ZSTD_customMem)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ZSTD_free(ptr: *mut libc::c_void, customMem: ZSTD_customMem) -> ();
    #[no_mangle]
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_wait(__cond: *mut pthread_cond_t,
                         __mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_create(__newthread: *mut pthread_t,
                      __attr: *const pthread_attr_t_0,
                      __start_routine:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> *mut libc::c_void>,
                      __arg: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_init(__cond: *mut pthread_cond_t,
                         __cond_attr: *const pthread_condattr_t)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_init(__mutex: *mut pthread_mutex_t,
                          __mutexattr: *const pthread_mutexattr_t)
     -> libc::c_int;
    #[no_mangle]
    static mut __tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut __daylight: libc::c_int;
    #[no_mangle]
    static mut __timezone: libc::c_long;
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_cond_t {
    __data: unnamed_2,
    __size: [libc::c_char; 48],
    __align: libc::c_longlong,
}
pub type BYTE = uint8_t;
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const libc::c_void,
    pub symbolTT: *const libc::c_void,
    pub stateLog: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct POOL_job_s {
    pub function: POOL_function,
    pub opaque: *mut libc::c_void,
}
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DState_t {
    pub state: size_t,
    pub table: *const libc::c_void,
}
pub type BIT_DStream_status = libc::c_uint;
/* ! POOL_function :
 *  The function type that can be added to a thread pool.
 */
pub type POOL_function =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_mutex_t {
    __data: __pthread_mutex_s,
    __size: [libc::c_char; 40],
    __align: libc::c_long,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_mutexattr_t {
    __size: [libc::c_char; 4],
    __align: libc::c_int,
}
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
pub type U16 = uint16_t;
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
pub const MEM_static_assert: unnamed_0 = 1;
pub type U64 = uint64_t;
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    u: U32,
    c: [BYTE; 4],
}
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
pub type ptrdiff_t = libc::c_long;
/* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
pub type uint64_t = libc::c_ulong;
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
pub type ZSTD_ErrorCode = libc::c_uint;
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_attr_t {
    __size: [libc::c_char; 56],
    __align: libc::c_long,
}
pub type S16 = int16_t;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub type uint8_t = libc::c_uchar;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
pub type pthread_t = libc::c_ulong;
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: libc::c_uint,
    pub ptr: *const libc::c_char,
    pub start: *const libc::c_char,
    pub limitPtr: *const libc::c_char,
}
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub type uint16_t = libc::c_ushort;
pub type unnamed_0 = libc::c_uint;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_condattr_t {
    __size: [libc::c_char; 4],
    __align: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub type size_t = libc::c_ulong;
pub type pthread_attr_t_0 = pthread_attr_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct POOL_ctx_s {
    pub customMem: ZSTD_customMem,
    pub threads: *mut pthread_t,
    pub threadCapacity: size_t,
    pub threadLimit: size_t,
    pub queue: *mut POOL_job,
    pub queueHead: size_t,
    pub queueTail: size_t,
    pub queueSize: size_t,
    pub numThreadsBusy: size_t,
    pub queueEmpty: libc::c_int,
    pub queueMutex: pthread_mutex_t,
    pub queuePushCond: pthread_cond_t,
    pub queuePopCond: pthread_cond_t,
    pub shutdown: libc::c_int,
}
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
/* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
pub type FSE_CTable = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
pub type __pthread_list_t = __pthread_internal_list;
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
pub type ERR_enum = ZSTD_ErrorCode;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
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
pub struct FSE_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub type int16_t = libc::c_short;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub const MEM_static_assert_0: unnamed_1 = 1;
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
pub type U32 = uint32_t;
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
/* ! Constructor and Destructor of FSE_DTable.
    Note that its size depends on 'tableLog' */
pub type FSE_DTable = libc::c_uint;
pub type POOL_ctx = POOL_ctx_s;
pub type unnamed_1 = libc::c_uint;
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
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
pub type POOL_job = POOL_job_s;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: libc::c_int,
    pub deltaNbBits: U32,
}
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
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
                    current_block = 11952241882418547642;
                }
                6 => { current_block = 11952241882418547642; }
                5 => { current_block = 3383678736627173723; }
                4 => { current_block = 8289819411585916169; }
                3 => { current_block = 14159273637582461377; }
                2 => { current_block = 10592393544436638764; }
                _ => { current_block = 10886091980245723256; }
            }
            match current_block {
                11952241882418547642 => {
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
                    current_block = 3383678736627173723;
                }
                _ => { }
            }
            match current_block {
                3383678736627173723 => {
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
                    current_block = 8289819411585916169;
                }
                _ => { }
            }
            match current_block {
                8289819411585916169 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(3isize)
                                                              as size_t) <<
                                                             24i32) as size_t
                            as size_t;
                    current_block = 14159273637582461377;
                }
                _ => { }
            }
            match current_block {
                14159273637582461377 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(2isize)
                                                              as size_t) <<
                                                             16i32) as size_t
                            as size_t;
                    current_block = 10592393544436638764;
                }
                _ => { }
            }
            match current_block {
                10592393544436638764 => {
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
/* ! POOL_create() :
 *  Create a thread pool with at most `numThreads` threads.
 * `numThreads` must be at least 1.
 *  The maximum number of queued jobs before blocking is `queueSize`.
 * @return : POOL_ctx pointer on success, else NULL.
*/
#[no_mangle]
pub unsafe extern "C" fn POOL_create(mut numThreads: size_t,
                                     mut queueSize: size_t) -> *mut POOL_ctx {
    return POOL_create_advanced(numThreads, queueSize, ZSTD_defaultCMem);
}
#[no_mangle]
pub unsafe extern "C" fn POOL_create_advanced(mut numThreads: size_t,
                                              mut queueSize: size_t,
                                              mut customMem: ZSTD_customMem)
 -> *mut POOL_ctx {
    let mut ctx: *mut POOL_ctx = 0 as *mut POOL_ctx;
    if 0 == numThreads {
        return 0 as *mut POOL_ctx
    } else {
        ctx =
            ZSTD_calloc(::std::mem::size_of::<POOL_ctx>() as libc::c_ulong,
                        customMem) as *mut POOL_ctx;
        if ctx.is_null() {
            return 0 as *mut POOL_ctx
        } else {
            (*ctx).queueSize = queueSize.wrapping_add(1i32 as libc::c_ulong);
            (*ctx).queue =
                ZSTD_malloc((*ctx).queueSize.wrapping_mul(::std::mem::size_of::<POOL_job>()
                                                              as
                                                              libc::c_ulong),
                            customMem) as *mut POOL_job;
            (*ctx).queueHead = 0i32 as size_t;
            (*ctx).queueTail = 0i32 as size_t;
            (*ctx).numThreadsBusy = 0i32 as size_t;
            (*ctx).queueEmpty = 1i32;
            pthread_mutex_init(&mut (*ctx).queueMutex as *mut pthread_mutex_t,
                               0 as *const pthread_mutexattr_t);
            pthread_cond_init(&mut (*ctx).queuePushCond as
                                  *mut pthread_cond_t,
                              0 as *const pthread_condattr_t);
            pthread_cond_init(&mut (*ctx).queuePopCond as *mut pthread_cond_t,
                              0 as *const pthread_condattr_t);
            (*ctx).shutdown = 0i32;
            (*ctx).threads =
                ZSTD_malloc(numThreads.wrapping_mul(::std::mem::size_of::<pthread_t>()
                                                        as libc::c_ulong),
                            customMem) as *mut pthread_t;
            (*ctx).threadCapacity = 0i32 as size_t;
            (*ctx).customMem = customMem;
            if (*ctx).threads.is_null() || (*ctx).queue.is_null() {
                POOL_free(ctx);
                return 0 as *mut POOL_ctx
            } else {
                let mut i: size_t = 0;
                i = 0i32 as size_t;
                while i < numThreads {
                    if 0 !=
                           pthread_create(&mut *(*ctx).threads.offset(i as
                                                                          isize)
                                              as *mut pthread_t,
                                          0 as *const pthread_attr_t_0,
                                          Some(POOL_thread),
                                          ctx as *mut libc::c_void) {
                        (*ctx).threadCapacity = i;
                        POOL_free(ctx);
                        return 0 as *mut POOL_ctx
                    } else { i = i.wrapping_add(1) }
                }
                (*ctx).threadCapacity = numThreads;
                (*ctx).threadLimit = numThreads;
                return ctx
            }
        }
    };
}
/* ! POOL_free() :
 *  Free a thread pool returned by POOL_create().
 */
#[no_mangle]
pub unsafe extern "C" fn POOL_free(mut ctx: *mut POOL_ctx) -> () {
    if ctx.is_null() {
        return
    } else {
        POOL_join(ctx);
        pthread_mutex_destroy(&mut (*ctx).queueMutex as *mut pthread_mutex_t);
        pthread_cond_destroy(&mut (*ctx).queuePushCond as
                                 *mut pthread_cond_t);
        pthread_cond_destroy(&mut (*ctx).queuePopCond as *mut pthread_cond_t);
        ZSTD_free((*ctx).queue as *mut libc::c_void, (*ctx).customMem);
        ZSTD_free((*ctx).threads as *mut libc::c_void, (*ctx).customMem);
        ZSTD_free(ctx as *mut libc::c_void, (*ctx).customMem);
        return;
    };
}
/* ! POOL_join() :
    Shutdown the queue, wake any sleeping threads, and join all of the threads.
*/
unsafe extern "C" fn POOL_join(mut ctx: *mut POOL_ctx) -> () {
    pthread_mutex_lock(&mut (*ctx).queueMutex as *mut pthread_mutex_t);
    (*ctx).shutdown = 1i32;
    pthread_mutex_unlock(&mut (*ctx).queueMutex as *mut pthread_mutex_t);
    pthread_cond_broadcast(&mut (*ctx).queuePushCond as *mut pthread_cond_t);
    pthread_cond_broadcast(&mut (*ctx).queuePopCond as *mut pthread_cond_t);
    let mut i: size_t = 0;
    i = 0i32 as size_t;
    while i < (*ctx).threadCapacity {
        pthread_join(*(*ctx).threads.offset(i as isize),
                     0 as *mut *mut libc::c_void);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn POOL_thread(mut opaque: *mut libc::c_void)
 -> *mut libc::c_void {
    let ctx: *mut POOL_ctx = opaque as *mut POOL_ctx;
    if ctx.is_null() {
        return 0 as *mut libc::c_void
    } else {
        loop  {
            pthread_mutex_lock(&mut (*ctx).queueMutex as
                                   *mut pthread_mutex_t);
            while 0 != (*ctx).queueEmpty ||
                      (*ctx).numThreadsBusy >= (*ctx).threadLimit {
                if 0 != (*ctx).shutdown {
                    pthread_mutex_unlock(&mut (*ctx).queueMutex as
                                             *mut pthread_mutex_t);
                    return opaque
                } else {
                    pthread_cond_wait(&mut (*ctx).queuePopCond as
                                          *mut pthread_cond_t,
                                      &mut (*ctx).queueMutex as
                                          *mut pthread_mutex_t);
                }
            }
            let job: POOL_job =
                *(*ctx).queue.offset((*ctx).queueHead as isize);
            (*ctx).queueHead =
                (*ctx).queueHead.wrapping_add(1i32 as
                                                  libc::c_ulong).wrapping_rem((*ctx).queueSize);
            (*ctx).numThreadsBusy = (*ctx).numThreadsBusy.wrapping_add(1);
            (*ctx).queueEmpty =
                ((*ctx).queueHead == (*ctx).queueTail) as libc::c_int;
            pthread_mutex_unlock(&mut (*ctx).queueMutex as
                                     *mut pthread_mutex_t);
            pthread_cond_signal(&mut (*ctx).queuePushCond as
                                    *mut pthread_cond_t);
            job.function.expect("non-null function pointer")(job.opaque);
            pthread_mutex_lock(&mut (*ctx).queueMutex as
                                   *mut pthread_mutex_t);
            (*ctx).numThreadsBusy = (*ctx).numThreadsBusy.wrapping_sub(1);
            if (*ctx).queueSize == 1i32 as libc::c_ulong {
                pthread_cond_signal(&mut (*ctx).queuePushCond as
                                        *mut pthread_cond_t);
            }
            pthread_mutex_unlock(&mut (*ctx).queueMutex as
                                     *mut pthread_mutex_t);
        }
    };
}
/* ! POOL_resize() :
 *  Expands or shrinks pool's number of threads.
 *  This is more efficient than releasing + creating a new context,
 *  since it tries to preserve and re-use existing threads.
 * `numThreads` must be at least 1.
 * @return : 0 when resize was successful,
 *           !0 (typically 1) if there is an error.
 *    note : only numThreads can be resized, queueSize remains unchanged.
 */
#[no_mangle]
pub unsafe extern "C" fn POOL_resize(mut ctx: *mut POOL_ctx,
                                     mut numThreads: size_t) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if ctx.is_null() {
        return 1i32
    } else {
        pthread_mutex_lock(&mut (*ctx).queueMutex as *mut pthread_mutex_t);
        result = POOL_resize_internal(ctx, numThreads);
        pthread_cond_broadcast(&mut (*ctx).queuePopCond as
                                   *mut pthread_cond_t);
        pthread_mutex_unlock(&mut (*ctx).queueMutex as *mut pthread_mutex_t);
        return result
    };
}
unsafe extern "C" fn POOL_resize_internal(mut ctx: *mut POOL_ctx,
                                          mut numThreads: size_t)
 -> libc::c_int {
    if numThreads <= (*ctx).threadCapacity {
        if 0 == numThreads {
            return 1i32
        } else { (*ctx).threadLimit = numThreads; return 0i32 }
    } else {
        let threadPool: *mut pthread_t =
            ZSTD_malloc(numThreads.wrapping_mul(::std::mem::size_of::<pthread_t>()
                                                    as libc::c_ulong),
                        (*ctx).customMem) as *mut pthread_t;
        if threadPool.is_null() {
            return 1i32
        } else {
            memcpy(threadPool as *mut libc::c_void,
                   (*ctx).threads as *const libc::c_void,
                   (*ctx).threadCapacity.wrapping_mul(::std::mem::size_of::<pthread_t>()
                                                          as libc::c_ulong));
            ZSTD_free((*ctx).threads as *mut libc::c_void, (*ctx).customMem);
            (*ctx).threads = threadPool;
            let mut threadId: size_t = 0;
            threadId = (*ctx).threadCapacity;
            while threadId < numThreads {
                if 0 !=
                       pthread_create(&mut *threadPool.offset(threadId as
                                                                  isize) as
                                          *mut pthread_t,
                                      0 as *const pthread_attr_t_0,
                                      Some(POOL_thread),
                                      ctx as *mut libc::c_void) {
                    (*ctx).threadCapacity = threadId;
                    return 1i32
                } else { threadId = threadId.wrapping_add(1) }
            }
            (*ctx).threadCapacity = numThreads;
            (*ctx).threadLimit = numThreads;
            return 0i32
        }
    };
}
/* ! POOL_sizeof() :
 * @return threadpool memory usage
 *  note : compatible with NULL (returns 0 in this case)
 */
#[no_mangle]
pub unsafe extern "C" fn POOL_sizeof(mut ctx: *mut POOL_ctx) -> size_t {
    if ctx.is_null() {
        return 0i32 as size_t
    } else {
        return (::std::mem::size_of::<POOL_ctx>() as
                    libc::c_ulong).wrapping_add((*ctx).queueSize.wrapping_mul(::std::mem::size_of::<POOL_job>()
                                                                                  as
                                                                                  libc::c_ulong)).wrapping_add((*ctx).threadCapacity.wrapping_mul(::std::mem::size_of::<pthread_t>()
                                                                                                                                                      as
                                                                                                                                                      libc::c_ulong))
    };
}
/* ! POOL_add() :
 *  Add the job `function(opaque)` to the thread pool. `ctx` must be valid.
 *  Possibly blocks until there is room in the queue.
 *  Note : The function may be executed asynchronously,
 *         therefore, `opaque` must live until function has been completed.
 */
#[no_mangle]
pub unsafe extern "C" fn POOL_add(mut ctx: *mut POOL_ctx,
                                  mut function: POOL_function,
                                  mut opaque: *mut libc::c_void) -> () {
    pthread_mutex_lock(&mut (*ctx).queueMutex as *mut pthread_mutex_t);
    while 0 != isQueueFull(ctx) && 0 == (*ctx).shutdown {
        pthread_cond_wait(&mut (*ctx).queuePushCond as *mut pthread_cond_t,
                          &mut (*ctx).queueMutex as *mut pthread_mutex_t);
    }
    POOL_add_internal(ctx, function, opaque);
    pthread_mutex_unlock(&mut (*ctx).queueMutex as *mut pthread_mutex_t);
}
unsafe extern "C" fn POOL_add_internal(mut ctx: *mut POOL_ctx,
                                       mut function: POOL_function,
                                       mut opaque: *mut libc::c_void) -> () {
    let job: POOL_job = POOL_job_s{function: function, opaque: opaque,};
    if 0 != (*ctx).shutdown {
        return
    } else {
        (*ctx).queueEmpty = 0i32;
        *(*ctx).queue.offset((*ctx).queueTail as isize) = job;
        (*ctx).queueTail =
            (*ctx).queueTail.wrapping_add(1i32 as
                                              libc::c_ulong).wrapping_rem((*ctx).queueSize);
        pthread_cond_signal(&mut (*ctx).queuePopCond as *mut pthread_cond_t);
        return;
    };
}
/* *
 * Returns 1 if the queue is full and 0 otherwise.
 *
 * When queueSize is 1 (pool was created with an intended queueSize of 0),
 * then a queue is empty if there is a thread free _and_ no job is waiting.
 */
unsafe extern "C" fn isQueueFull(mut ctx: *const POOL_ctx) -> libc::c_int {
    if (*ctx).queueSize > 1i32 as libc::c_ulong {
        return ((*ctx).queueHead ==
                    (*ctx).queueTail.wrapping_add(1i32 as
                                                      libc::c_ulong).wrapping_rem((*ctx).queueSize))
                   as libc::c_int
    } else {
        return ((*ctx).numThreadsBusy == (*ctx).threadLimit ||
                    0 == (*ctx).queueEmpty) as libc::c_int
    };
}
/* ! POOL_tryAdd() :
 *  Add the job `function(opaque)` to thread pool _if_ a worker is available.
 *  Returns immediately even if not (does not block).
 * @return : 1 if successful, 0 if not.
 */
#[no_mangle]
pub unsafe extern "C" fn POOL_tryAdd(mut ctx: *mut POOL_ctx,
                                     mut function: POOL_function,
                                     mut opaque: *mut libc::c_void)
 -> libc::c_int {
    pthread_mutex_lock(&mut (*ctx).queueMutex as *mut pthread_mutex_t);
    if 0 != isQueueFull(ctx) {
        pthread_mutex_unlock(&mut (*ctx).queueMutex as *mut pthread_mutex_t);
        return 0i32
    } else {
        POOL_add_internal(ctx, function, opaque);
        pthread_mutex_unlock(&mut (*ctx).queueMutex as *mut pthread_mutex_t);
        return 1i32
    };
}
