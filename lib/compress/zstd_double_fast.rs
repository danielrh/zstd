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
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const libc::c_void,
    pub symbolTT: *const libc::c_void,
    pub stateLog: libc::c_uint,
}
pub type ZSTD_dictTableLoadMethod_e = libc::c_uint;
pub const ZSTD_lazy: ZSTD_strategy = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    u: U32,
    c: [BYTE; 4],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct seqStore_t {
    pub sequencesStart: *mut seqDef,
    pub sequences: *mut seqDef,
    pub litStart: *mut BYTE,
    pub lit: *mut BYTE,
    pub llCode: *mut BYTE,
    pub mlCode: *mut BYTE,
    pub ofCode: *mut BYTE,
    pub longLengthID: U32,
    pub longLengthPos: U32,
}
pub type BIT_DStream_status = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
/* *< Cannot use the previous table */
pub const FSE_repeat_check: FSE_repeat = 1;
pub type BYTE = uint8_t;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
/* ! Constructor and Destructor of FSE_DTable.
    Note that its size depends on 'tableLog' */
pub type FSE_DTable = libc::c_uint;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub type S16 = int16_t;
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
pub type unnamed_0 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
/* *< Can use the previous table but it must be checked */
pub const FSE_repeat_valid: FSE_repeat = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_fseCTables_t {
    pub offcodeCTable: [FSE_CTable; 193],
    pub matchlengthCTable: [FSE_CTable; 363],
    pub litlengthCTable: [FSE_CTable; 329],
    pub offcode_repeatMode: FSE_repeat,
    pub matchlength_repeatMode: FSE_repeat,
    pub litlength_repeatMode: FSE_repeat,
}
pub const HUF_repeat_none: HUF_repeat = 0;
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
pub type ZSTD_matchState_t = ZSTD_matchState_t_0;
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
pub type HUF_repeat = libc::c_uint;
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub type ERR_enum = ZSTD_ErrorCode;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
pub type unnamed_1 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub type ptrdiff_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DState_t {
    pub state: size_t,
    pub table: *const libc::c_void,
}
pub const MEM_static_assert: unnamed_1 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
pub const ZSTD_btopt: ZSTD_strategy = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_hufCTables_t {
    pub CTable: [U32; 256],
    pub repeatMode: HUF_repeat,
}
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: libc::c_int,
    pub deltaNbBits: U32,
}
pub const MEM_static_assert_0: unnamed_0 = 1;
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
pub type U32 = uint32_t;
/* *< Cannot use the previous table */
pub const HUF_repeat_check: HUF_repeat = 1;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub type U16 = uint16_t;
pub const FSE_repeat_none: FSE_repeat = 0;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct seqDef_s {
    pub offset: U32,
    pub litLength: U16,
    pub matchLength: U16,
}
pub type ZSTD_ErrorCode = libc::c_uint;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_optimal_t {
    pub price: libc::c_int,
    pub off: U32,
    pub mlen: U32,
    pub litlen: U32,
    pub rep: [U32; 3],
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
pub const ZSTD_fast: ZSTD_strategy = 1;
pub type ZSTD_OptPrice_e = libc::c_uint;
pub type ZSTD_dictMode_e = libc::c_uint;
/* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub type ZSTD_strategy = libc::c_uint;
pub type U64 = uint64_t;
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_window_t {
    pub nextSrc: *const BYTE,
    pub base: *const BYTE,
    pub dictBase: *const BYTE,
    pub dictLimit: U32,
    pub lowLimit: U32,
}
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
pub type int16_t = libc::c_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_match_t {
    pub off: U32,
    pub len: U32,
}
/* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
pub type FSE_CTable = libc::c_uint;
pub type seqDef = seqDef_s;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_CStream_t {
    pub bitContainer: size_t,
    pub bitPos: libc::c_uint,
    pub startPtr: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub endPtr: *mut libc::c_char,
}
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_matchState_t_0 {
    pub window: ZSTD_window_t,
    pub loadedDictEnd: U32,
    pub nextToUpdate: U32,
    pub nextToUpdate3: U32,
    pub hashLog3: U32,
    pub hashTable: *mut U32,
    pub hashTable3: *mut U32,
    pub chainTable: *mut U32,
    pub opt: optState_t,
    pub dictMatchState: *const ZSTD_matchState_t,
}
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
/* *< Can use the previous table but it must be checked. Note : The previous table must have been constructed by HUF_compress{1, 4}X_repeat */
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
pub type size_t = libc::c_ulong;
pub type uint8_t = libc::c_uchar;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub type FSE_repeat = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_entropyCTables_t {
    pub huf: ZSTD_hufCTables_t,
    pub fse: ZSTD_fseCTables_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct optState_t {
    pub litFreq: *mut U32,
    pub litLengthFreq: *mut U32,
    pub matchLengthFreq: *mut U32,
    pub offCodeFreq: *mut U32,
    pub matchTable: *mut ZSTD_match_t,
    pub priceTable: *mut ZSTD_optimal_t,
    pub litSum: U32,
    pub litLengthSum: U32,
    pub matchLengthSum: U32,
    pub offCodeSum: U32,
    pub litSumBasePrice: U32,
    pub litLengthSumBasePrice: U32,
    pub matchLengthSumBasePrice: U32,
    pub offCodeSumBasePrice: U32,
    pub priceType: ZSTD_OptPrice_e,
    pub symbolCosts: *const ZSTD_entropyCTables_t,
}
pub type uint64_t = libc::c_ulong;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
pub type uint16_t = libc::c_ushort;
pub const ZSTD_dfast: ZSTD_strategy = 2;
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
                    current_block = 11516302844859794045;
                }
                6 => { current_block = 11516302844859794045; }
                5 => { current_block = 1908961402461318613; }
                4 => { current_block = 6284263362753553522; }
                3 => { current_block = 12372378619745344700; }
                2 => { current_block = 7459387687377790050; }
                _ => { current_block = 10886091980245723256; }
            }
            match current_block {
                11516302844859794045 => {
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
                    current_block = 1908961402461318613;
                }
                _ => { }
            }
            match current_block {
                1908961402461318613 => {
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
                    current_block = 6284263362753553522;
                }
                _ => { }
            }
            match current_block {
                6284263362753553522 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(3isize)
                                                              as size_t) <<
                                                             24i32) as size_t
                            as size_t;
                    current_block = 12372378619745344700;
                }
                _ => { }
            }
            match current_block {
                12372378619745344700 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(2isize)
                                                              as size_t) <<
                                                             16i32) as size_t
                            as size_t;
                    current_block = 7459387687377790050;
                }
                _ => { }
            }
            match current_block {
                7459387687377790050 => {
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
unsafe extern "C" fn ZSTD_LLcode(mut litLength: U32) -> U32 {
    static mut LL_Code: [BYTE; 64] =
        unsafe {
            [0i32 as BYTE, 1i32 as BYTE, 2i32 as BYTE, 3i32 as BYTE,
             4i32 as BYTE, 5i32 as BYTE, 6i32 as BYTE, 7i32 as BYTE,
             8i32 as BYTE, 9i32 as BYTE, 10i32 as BYTE, 11i32 as BYTE,
             12i32 as BYTE, 13i32 as BYTE, 14i32 as BYTE, 15i32 as BYTE,
             16i32 as BYTE, 16i32 as BYTE, 17i32 as BYTE, 17i32 as BYTE,
             18i32 as BYTE, 18i32 as BYTE, 19i32 as BYTE, 19i32 as BYTE,
             20i32 as BYTE, 20i32 as BYTE, 20i32 as BYTE, 20i32 as BYTE,
             21i32 as BYTE, 21i32 as BYTE, 21i32 as BYTE, 21i32 as BYTE,
             22i32 as BYTE, 22i32 as BYTE, 22i32 as BYTE, 22i32 as BYTE,
             22i32 as BYTE, 22i32 as BYTE, 22i32 as BYTE, 22i32 as BYTE,
             23i32 as BYTE, 23i32 as BYTE, 23i32 as BYTE, 23i32 as BYTE,
             23i32 as BYTE, 23i32 as BYTE, 23i32 as BYTE, 23i32 as BYTE,
             24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE,
             24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE,
             24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE,
             24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE]
        };
    static mut LL_deltaCode: U32 = unsafe { 19i32 as U32 };
    return if litLength > 63i32 as libc::c_uint {
               ZSTD_highbit32(litLength).wrapping_add(LL_deltaCode)
           } else { LL_Code[litLength as usize] as libc::c_uint };
}
unsafe extern "C" fn ZSTD_MLcode(mut mlBase: U32) -> U32 {
    static mut ML_Code: [BYTE; 128] =
        unsafe {
            [0i32 as BYTE, 1i32 as BYTE, 2i32 as BYTE, 3i32 as BYTE,
             4i32 as BYTE, 5i32 as BYTE, 6i32 as BYTE, 7i32 as BYTE,
             8i32 as BYTE, 9i32 as BYTE, 10i32 as BYTE, 11i32 as BYTE,
             12i32 as BYTE, 13i32 as BYTE, 14i32 as BYTE, 15i32 as BYTE,
             16i32 as BYTE, 17i32 as BYTE, 18i32 as BYTE, 19i32 as BYTE,
             20i32 as BYTE, 21i32 as BYTE, 22i32 as BYTE, 23i32 as BYTE,
             24i32 as BYTE, 25i32 as BYTE, 26i32 as BYTE, 27i32 as BYTE,
             28i32 as BYTE, 29i32 as BYTE, 30i32 as BYTE, 31i32 as BYTE,
             32i32 as BYTE, 32i32 as BYTE, 33i32 as BYTE, 33i32 as BYTE,
             34i32 as BYTE, 34i32 as BYTE, 35i32 as BYTE, 35i32 as BYTE,
             36i32 as BYTE, 36i32 as BYTE, 36i32 as BYTE, 36i32 as BYTE,
             37i32 as BYTE, 37i32 as BYTE, 37i32 as BYTE, 37i32 as BYTE,
             38i32 as BYTE, 38i32 as BYTE, 38i32 as BYTE, 38i32 as BYTE,
             38i32 as BYTE, 38i32 as BYTE, 38i32 as BYTE, 38i32 as BYTE,
             39i32 as BYTE, 39i32 as BYTE, 39i32 as BYTE, 39i32 as BYTE,
             39i32 as BYTE, 39i32 as BYTE, 39i32 as BYTE, 39i32 as BYTE,
             40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
             40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
             40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
             40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
             41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
             41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
             41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
             41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE]
        };
    static mut ML_deltaCode: U32 = unsafe { 36i32 as U32 };
    return if mlBase > 127i32 as libc::c_uint {
               ZSTD_highbit32(mlBase).wrapping_add(ML_deltaCode)
           } else { ML_Code[mlBase as usize] as libc::c_uint };
}
unsafe extern "C" fn ZSTD_storeSeq(mut seqStorePtr: *mut seqStore_t,
                                   mut litLength: size_t,
                                   mut literals: *const libc::c_void,
                                   mut offsetCode: U32, mut mlBase: size_t)
 -> () {
    ZSTD_wildcopy((*seqStorePtr).lit as *mut libc::c_void, literals,
                  litLength as ptrdiff_t);
    (*seqStorePtr).lit = (*seqStorePtr).lit.offset(litLength as isize);
    if litLength > 65535i32 as libc::c_ulong {
        (*seqStorePtr).longLengthID = 1i32 as U32;
        (*seqStorePtr).longLengthPos =
            (*seqStorePtr).sequencesStart.offset_to((*seqStorePtr).sequences).expect("bad offset_to")
                as libc::c_long as U32
    }
    (*(*seqStorePtr).sequences.offset(0isize)).litLength = litLength as U16;
    (*(*seqStorePtr).sequences.offset(0isize)).offset =
        offsetCode.wrapping_add(1i32 as libc::c_uint);
    if mlBase > 65535i32 as libc::c_ulong {
        (*seqStorePtr).longLengthID = 2i32 as U32;
        (*seqStorePtr).longLengthPos =
            (*seqStorePtr).sequencesStart.offset_to((*seqStorePtr).sequences).expect("bad offset_to")
                as libc::c_long as U32
    }
    (*(*seqStorePtr).sequences.offset(0isize)).matchLength = mlBase as U16;
    (*seqStorePtr).sequences = (*seqStorePtr).sequences.offset(1isize);
}
unsafe extern "C" fn ZSTD_NbCommonBytes(mut val: size_t) -> libc::c_uint {
    if 0 != MEM_isLittleEndian() {
        if 0 != MEM_64bits() {
            return ((val as libc::c_ulonglong).trailing_zeros() as i32 >>
                        3i32) as libc::c_uint
        } else {
            return ((val as U32).trailing_zeros() as i32 >> 3i32) as
                       libc::c_uint
        }
    } else if 0 != MEM_64bits() {
        return ((val as libc::c_ulonglong).leading_zeros() as i32 >> 3i32) as
                   libc::c_uint
    } else {
        return ((val as U32).leading_zeros() as i32 >> 3i32) as libc::c_uint
    };
}
unsafe extern "C" fn ZSTD_count(mut pIn: *const BYTE, mut pMatch: *const BYTE,
                                pInLimit: *const BYTE) -> size_t {
    let pStart: *const BYTE = pIn;
    let pInLoopLimit: *const BYTE =
        pInLimit.offset(-((::std::mem::size_of::<size_t>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong)
                              as isize));
    if pIn < pInLoopLimit {
        let diff: size_t =
            MEM_readST(pMatch as *const libc::c_void) ^
                MEM_readST(pIn as *const libc::c_void);
        if 0 != diff {
            return ZSTD_NbCommonBytes(diff) as size_t
        } else {
            pIn =
                pIn.offset(::std::mem::size_of::<size_t>() as libc::c_ulong as
                               isize);
            pMatch =
                pMatch.offset(::std::mem::size_of::<size_t>() as libc::c_ulong
                                  as isize);
            while pIn < pInLoopLimit {
                let diff_0: size_t =
                    MEM_readST(pMatch as *const libc::c_void) ^
                        MEM_readST(pIn as *const libc::c_void);
                if 0 == diff_0 {
                    pIn =
                        pIn.offset(::std::mem::size_of::<size_t>() as
                                       libc::c_ulong as isize);
                    pMatch =
                        pMatch.offset(::std::mem::size_of::<size_t>() as
                                          libc::c_ulong as isize)
                } else {
                    pIn = pIn.offset(ZSTD_NbCommonBytes(diff_0) as isize);
                    return pStart.offset_to(pIn).expect("bad offset_to") as
                               libc::c_long as size_t
                }
            }
        }
    }
    if 0 != MEM_64bits() && pIn < pInLimit.offset(-3isize) &&
           MEM_read32(pMatch as *const libc::c_void) ==
               MEM_read32(pIn as *const libc::c_void) {
        pIn = pIn.offset(4isize);
        pMatch = pMatch.offset(4isize)
    }
    if pIn < pInLimit.offset(-1isize) &&
           MEM_read16(pMatch as *const libc::c_void) as libc::c_int ==
               MEM_read16(pIn as *const libc::c_void) as libc::c_int {
        pIn = pIn.offset(2isize);
        pMatch = pMatch.offset(2isize)
    }
    if pIn < pInLimit && *pMatch as libc::c_int == *pIn as libc::c_int {
        pIn = pIn.offset(1isize)
    }
    return pStart.offset_to(pIn).expect("bad offset_to") as libc::c_long as
               size_t;
}
unsafe extern "C" fn ZSTD_count_2segments(mut ip: *const BYTE,
                                          mut match_0: *const BYTE,
                                          mut iEnd: *const BYTE,
                                          mut mEnd: *const BYTE,
                                          mut iStart: *const BYTE) -> size_t {
    let vEnd: *const BYTE =
        if ip.offset(match_0.offset_to(mEnd).expect("bad offset_to") as
                         libc::c_long as isize) < iEnd {
            ip.offset(match_0.offset_to(mEnd).expect("bad offset_to") as
                          libc::c_long as isize)
        } else { iEnd };
    let matchLength: size_t = ZSTD_count(ip, match_0, vEnd);
    if match_0.offset(matchLength as isize) != mEnd {
        return matchLength
    } else {
        return matchLength.wrapping_add(ZSTD_count(ip.offset(matchLength as
                                                                 isize),
                                                   iStart, iEnd))
    };
}
static mut prime3bytes: U32 = unsafe { 506832829u32 };
unsafe extern "C" fn ZSTD_hash3(mut u: U32, mut h: U32) -> U32 {
    return (u << 32i32 - 24i32).wrapping_mul(prime3bytes) >>
               (32i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash3Ptr(mut ptr: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash3(MEM_readLE32(ptr), h) as size_t;
}
static mut prime4bytes: U32 = unsafe { 2654435761u32 };
unsafe extern "C" fn ZSTD_hash4(mut u: U32, mut h: U32) -> U32 {
    return u.wrapping_mul(prime4bytes) >>
               (32i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash4Ptr(mut ptr: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash4(MEM_read32(ptr), h) as size_t;
}
static mut prime5bytes: U64 = unsafe { 889523592379u64 as U64 };
unsafe extern "C" fn ZSTD_hash5(mut u: U64, mut h: U32) -> size_t {
    return (u << 64i32 - 40i32).wrapping_mul(prime5bytes) >>
               (64i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash5Ptr(mut p: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash5(MEM_readLE64(p), h);
}
static mut prime6bytes: U64 = unsafe { 227718039650203u64 as U64 };
unsafe extern "C" fn ZSTD_hash6(mut u: U64, mut h: U32) -> size_t {
    return (u << 64i32 - 48i32).wrapping_mul(prime6bytes) >>
               (64i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash6Ptr(mut p: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash6(MEM_readLE64(p), h);
}
static mut prime7bytes: U64 = unsafe { 58295818150454627u64 as U64 };
unsafe extern "C" fn ZSTD_hash7(mut u: U64, mut h: U32) -> size_t {
    return (u << 64i32 - 56i32).wrapping_mul(prime7bytes) >>
               (64i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash7Ptr(mut p: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash7(MEM_readLE64(p), h);
}
static mut prime8bytes: U64 = unsafe { 14923729446382167139u64 as U64 };
unsafe extern "C" fn ZSTD_hash8(mut u: U64, mut h: U32) -> size_t {
    return u.wrapping_mul(prime8bytes) >>
               (64i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash8Ptr(mut p: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash8(MEM_readLE64(p), h);
}
unsafe extern "C" fn ZSTD_hashPtr(mut p: *const libc::c_void, mut hBits: U32,
                                  mut mls: U32) -> size_t {
    match mls {
        5 => { return ZSTD_hash5Ptr(p, hBits) }
        6 => { return ZSTD_hash6Ptr(p, hBits) }
        7 => { return ZSTD_hash7Ptr(p, hBits) }
        8 => { return ZSTD_hash8Ptr(p, hBits) }
        4 | _ => { return ZSTD_hash4Ptr(p, hBits) }
    };
}
unsafe extern "C" fn ZSTD_window_clear(mut window: *mut ZSTD_window_t) -> () {
    let endT: size_t =
        (*window).base.offset_to((*window).nextSrc).expect("bad offset_to") as
            libc::c_long as size_t;
    let end: U32 = endT as U32;
    (*window).lowLimit = end;
    (*window).dictLimit = end;
}
unsafe extern "C" fn ZSTD_window_hasExtDict(window: ZSTD_window_t) -> U32 {
    return (window.lowLimit < window.dictLimit) as libc::c_int as U32;
}
unsafe extern "C" fn ZSTD_matchState_dictMode(mut ms:
                                                  *const ZSTD_matchState_t)
 -> ZSTD_dictMode_e {
    return (if 0 != ZSTD_window_hasExtDict((*ms).window) {
                ZSTD_extDict as libc::c_int
            } else if !(*ms).dictMatchState.is_null() {
                ZSTD_dictMatchState as libc::c_int
            } else { ZSTD_noDict as libc::c_int }) as ZSTD_dictMode_e;
}
unsafe extern "C" fn ZSTD_window_needOverflowCorrection(window: ZSTD_window_t,
                                                        mut srcEnd:
                                                            *const libc::c_void)
 -> U32 {
    let current: U32 =
        window.base.offset_to(srcEnd as *const BYTE).expect("bad offset_to")
            as libc::c_long as U32;
    return (current >
                (3u32 <<
                     29i32).wrapping_add(1u32 <<
                                             (if ::std::mem::size_of::<size_t>()
                                                     as libc::c_ulong ==
                                                     4i32 as libc::c_ulong {
                                                  30i32
                                              } else { 31i32 }) as
                                                 libc::c_uint)) as libc::c_int
               as U32;
}
unsafe extern "C" fn ZSTD_window_correctOverflow(mut window:
                                                     *mut ZSTD_window_t,
                                                 mut cycleLog: U32,
                                                 mut maxDist: U32,
                                                 mut src: *const libc::c_void)
 -> U32 {
    let cycleMask: U32 =
        (1u32 << cycleLog).wrapping_sub(1i32 as libc::c_uint);
    let current: U32 =
        (*window).base.offset_to(src as *const BYTE).expect("bad offset_to")
            as libc::c_long as U32;
    let newCurrent: U32 = (current & cycleMask).wrapping_add(maxDist);
    let correction: U32 = current.wrapping_sub(newCurrent);
    (*window).base = (*window).base.offset(correction as isize);
    (*window).dictBase = (*window).dictBase.offset(correction as isize);
    (*window).lowLimit =
        ((*window).lowLimit as libc::c_uint).wrapping_sub(correction) as U32
            as U32;
    (*window).dictLimit =
        ((*window).dictLimit as libc::c_uint).wrapping_sub(correction) as U32
            as U32;
    return correction;
}
unsafe extern "C" fn ZSTD_window_enforceMaxDist(mut window:
                                                    *mut ZSTD_window_t,
                                                mut srcEnd:
                                                    *const libc::c_void,
                                                mut maxDist: U32,
                                                mut loadedDictEndPtr:
                                                    *mut U32,
                                                mut dictMatchStatePtr:
                                                    *mut *const ZSTD_matchState_t)
 -> () {
    let current: U32 =
        (*window).base.offset_to(srcEnd as
                                     *const BYTE).expect("bad offset_to") as
            libc::c_long as U32;
    let mut loadedDictEnd: U32 =
        if !loadedDictEndPtr.is_null() {
            *loadedDictEndPtr
        } else { 0i32 as libc::c_uint };
    if current > maxDist.wrapping_add(loadedDictEnd) {
        let newLowLimit: U32 = current.wrapping_sub(maxDist);
        if (*window).lowLimit < newLowLimit {
            (*window).lowLimit = newLowLimit
        }
        if (*window).dictLimit < (*window).lowLimit {
            (*window).dictLimit = (*window).lowLimit
        }
        if !loadedDictEndPtr.is_null() { *loadedDictEndPtr = 0i32 as U32 }
        if !dictMatchStatePtr.is_null() {
            *dictMatchStatePtr = 0 as *const ZSTD_matchState_t
        }
    };
}
unsafe extern "C" fn ZSTD_window_update(mut window: *mut ZSTD_window_t,
                                        mut src: *const libc::c_void,
                                        mut srcSize: size_t) -> U32 {
    let ip: *const BYTE = src as *const BYTE;
    let mut contiguous: U32 = 1i32 as U32;
    if src != (*window).nextSrc as *const libc::c_void {
        let distanceFromBase: size_t =
            (*window).base.offset_to((*window).nextSrc).expect("bad offset_to")
                as libc::c_long as size_t;
        (*window).lowLimit = (*window).dictLimit;
        (*window).dictLimit = distanceFromBase as U32;
        (*window).dictBase = (*window).base;
        (*window).base = ip.offset(-(distanceFromBase as isize));
        if (*window).dictLimit.wrapping_sub((*window).lowLimit) <
               8i32 as libc::c_uint {
            (*window).lowLimit = (*window).dictLimit
        }
        contiguous = 0i32 as U32
    }
    (*window).nextSrc = ip.offset(srcSize as isize);
    if 0 !=
           (ip.offset(srcSize as isize) >
                (*window).dictBase.offset((*window).lowLimit as isize)) as
               libc::c_int &
               (ip < (*window).dictBase.offset((*window).dictLimit as isize))
                   as libc::c_int {
        let highInputIdx: ptrdiff_t =
            (*window).dictBase.offset_to(ip.offset(srcSize as
                                                       isize)).expect("bad offset_to")
                as libc::c_long;
        let lowLimitMax: U32 =
            if highInputIdx > (*window).dictLimit as ptrdiff_t {
                (*window).dictLimit
            } else { highInputIdx as U32 };
        (*window).lowLimit = lowLimitMax
    }
    return contiguous;
}
unsafe extern "C" fn ZSTD_fWeight(mut rawStat: U32) -> libc::c_double {
    let fp_accuracy: U32 = 8i32 as U32;
    let fp_multiplier: U32 = (1i32 << fp_accuracy) as U32;
    let stat: U32 = rawStat.wrapping_add(1i32 as libc::c_uint);
    let hb: U32 = ZSTD_highbit32(stat);
    let BWeight: U32 = hb.wrapping_mul(fp_multiplier);
    let FWeight: U32 = stat << fp_accuracy >> hb;
    let weight: U32 = BWeight.wrapping_add(FWeight);
    return weight as libc::c_double / fp_multiplier as libc::c_double;
}
unsafe extern "C" fn ZSTD_debugTable(mut table: *const U32, mut max: U32)
 -> () {
    let mut u: libc::c_uint = 0;
    let mut sum: libc::c_uint = 0;
    u = 0i32 as libc::c_uint;
    sum = 0i32 as libc::c_uint;
    while u <= max {
        sum = sum.wrapping_add(*table.offset(u as isize));
        u = u.wrapping_add(1)
    }
    u = 0i32 as libc::c_uint;
    while u <= max { u = u.wrapping_add(1) };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_fillDoubleHashTable(mut ms:
                                                      *mut ZSTD_matchState_t,
                                                  mut cParams:
                                                      *const ZSTD_compressionParameters,
                                                  mut end:
                                                      *const libc::c_void,
                                                  mut dtlm:
                                                      ZSTD_dictTableLoadMethod_e)
 -> () {
    let hashLarge: *mut U32 = (*ms).hashTable;
    let hBitsL: U32 = (*cParams).hashLog;
    let mls: U32 = (*cParams).searchLength;
    let hashSmall: *mut U32 = (*ms).chainTable;
    let hBitsS: U32 = (*cParams).chainLog;
    let base: *const BYTE = (*ms).window.base;
    let mut ip: *const BYTE = base.offset((*ms).nextToUpdate as isize);
    let iend: *const BYTE = (end as *const BYTE).offset(-8isize);
    let fastHashFillStep: U32 = 3i32 as U32;
    while ip.offset(fastHashFillStep as isize).offset(-1isize) <= iend {
        let current: U32 =
            base.offset_to(ip).expect("bad offset_to") as libc::c_long as U32;
        let mut i: U32 = 0;
        i = 0i32 as U32;
        while i < fastHashFillStep {
            let smHash: size_t =
                ZSTD_hashPtr(ip.offset(i as isize) as *const libc::c_void,
                             hBitsS, mls);
            let lgHash: size_t =
                ZSTD_hashPtr(ip.offset(i as isize) as *const libc::c_void,
                             hBitsL, 8i32 as U32);
            if i == 0i32 as libc::c_uint {
                *hashSmall.offset(smHash as isize) = current.wrapping_add(i)
            }
            if i == 0i32 as libc::c_uint ||
                   *hashLarge.offset(lgHash as isize) == 0i32 as libc::c_uint
               {
                *hashLarge.offset(lgHash as isize) = current.wrapping_add(i)
            }
            if dtlm as libc::c_uint ==
                   ZSTD_dtlm_fast as libc::c_int as libc::c_uint {
                break ;
            }
            i = i.wrapping_add(1)
        }
        ip = ip.offset(fastHashFillStep as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_doubleFast(mut ms:
                                                           *mut ZSTD_matchState_t,
                                                       mut seqStore:
                                                           *mut seqStore_t,
                                                       mut rep: *mut U32,
                                                       mut cParams:
                                                           *const ZSTD_compressionParameters,
                                                       mut src:
                                                           *const libc::c_void,
                                                       mut srcSize: size_t)
 -> size_t {
    let mls: U32 = (*cParams).searchLength;
    match mls {
        5 => {
            return ZSTD_compressBlock_doubleFast_generic(ms, seqStore, rep,
                                                         cParams, src,
                                                         srcSize, 5i32 as U32,
                                                         ZSTD_noDict)
        }
        6 => {
            return ZSTD_compressBlock_doubleFast_generic(ms, seqStore, rep,
                                                         cParams, src,
                                                         srcSize, 6i32 as U32,
                                                         ZSTD_noDict)
        }
        7 => {
            return ZSTD_compressBlock_doubleFast_generic(ms, seqStore, rep,
                                                         cParams, src,
                                                         srcSize, 7i32 as U32,
                                                         ZSTD_noDict)
        }
        4 | _ => {
            return ZSTD_compressBlock_doubleFast_generic(ms, seqStore, rep,
                                                         cParams, src,
                                                         srcSize, 4i32 as U32,
                                                         ZSTD_noDict)
        }
    };
}
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_generic(mut ms:
                                                               *mut ZSTD_matchState_t,
                                                           mut seqStore:
                                                               *mut seqStore_t,
                                                           mut rep: *mut U32,
                                                           mut cParams:
                                                               *const ZSTD_compressionParameters,
                                                           mut src:
                                                               *const libc::c_void,
                                                           mut srcSize:
                                                               size_t,
                                                           mls: U32,
                                                           dictMode:
                                                               ZSTD_dictMode_e)
 -> size_t {
    let mut current_block: u64;
    let hashLong: *mut U32 = (*ms).hashTable;
    let hBitsL: U32 = (*cParams).hashLog;
    let hashSmall: *mut U32 = (*ms).chainTable;
    let hBitsS: U32 = (*cParams).chainLog;
    let base: *const BYTE = (*ms).window.base;
    let istart: *const BYTE = src as *const BYTE;
    let mut ip: *const BYTE = istart;
    let mut anchor: *const BYTE = istart;
    let prefixLowestIndex: U32 = (*ms).window.dictLimit;
    let prefixLowest: *const BYTE = base.offset(prefixLowestIndex as isize);
    let iend: *const BYTE = istart.offset(srcSize as isize);
    let ilimit: *const BYTE = iend.offset(-8isize);
    let mut offset_1: U32 = *rep.offset(0isize);
    let mut offset_2: U32 = *rep.offset(1isize);
    let mut offsetSaved: U32 = 0i32 as U32;
    let dms: *const ZSTD_matchState_t = (*ms).dictMatchState;
    let dictHashLong: *const U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*dms).hashTable
        } else { 0 as *mut U32 };
    let dictHashSmall: *const U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*dms).chainTable
        } else { 0 as *mut U32 };
    let dictStartIndex: U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*dms).window.dictLimit
        } else { 0i32 as libc::c_uint };
    let dictBase: *const BYTE =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*dms).window.base
        } else { 0 as *const BYTE };
    let dictStart: *const BYTE =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            dictBase.offset(dictStartIndex as isize)
        } else { 0 as *const BYTE };
    let dictEnd: *const BYTE =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*dms).window.nextSrc
        } else { 0 as *const BYTE };
    let dictIndexDelta: U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            prefixLowestIndex.wrapping_sub(dictBase.offset_to(dictEnd).expect("bad offset_to")
                                               as libc::c_long as U32)
        } else { 0i32 as libc::c_uint };
    let dictAndPrefixLength: U32 =
        dictStart.offset_to(dictEnd.offset(prefixLowest.offset_to(ip).expect("bad offset_to")
                                               as libc::c_long as
                                               isize)).expect("bad offset_to")
            as libc::c_long as U32;
    ip =
        ip.offset((dictAndPrefixLength == 0i32 as libc::c_uint) as libc::c_int
                      as isize);
    if dictMode as libc::c_uint == ZSTD_noDict as libc::c_int as libc::c_uint
       {
        let maxRep: U32 =
            prefixLowest.offset_to(ip).expect("bad offset_to") as libc::c_long
                as U32;
        if offset_2 > maxRep {
            offsetSaved = offset_2;
            offset_2 = 0i32 as U32
        }
        if offset_1 > maxRep {
            offsetSaved = offset_1;
            offset_1 = 0i32 as U32
        }
    }
    dictMode as libc::c_uint ==
        ZSTD_dictMatchState as libc::c_int as libc::c_uint;
    while ip < ilimit {
        let mut mLength: size_t = 0;
        let mut offset: U32 = 0;
        let h2: size_t =
            ZSTD_hashPtr(ip as *const libc::c_void, hBitsL, 8i32 as U32);
        let h: size_t = ZSTD_hashPtr(ip as *const libc::c_void, hBitsS, mls);
        let current: U32 =
            base.offset_to(ip).expect("bad offset_to") as libc::c_long as U32;
        let matchIndexL: U32 = *hashLong.offset(h2 as isize);
        let mut matchIndexS: U32 = *hashSmall.offset(h as isize);
        let mut matchLong: *const BYTE = base.offset(matchIndexL as isize);
        let mut match_0: *const BYTE = base.offset(matchIndexS as isize);
        let repIndex: U32 =
            current.wrapping_add(1i32 as libc::c_uint).wrapping_sub(offset_1);
        let mut repMatch: *const BYTE =
            if dictMode as libc::c_uint ==
                   ZSTD_dictMatchState as libc::c_int as libc::c_uint &&
                   repIndex < prefixLowestIndex {
                dictBase.offset(repIndex.wrapping_sub(dictIndexDelta) as
                                    isize)
            } else { base.offset(repIndex as isize) };
        let ref mut fresh0 = *hashSmall.offset(h as isize);
        *fresh0 = current;
        *hashLong.offset(h2 as isize) = *fresh0;
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint &&
               prefixLowestIndex.wrapping_sub(1i32 as
                                                  libc::c_uint).wrapping_sub(repIndex)
                   >= 3i32 as libc::c_uint &&
               MEM_read32(repMatch as *const libc::c_void) ==
                   MEM_read32(ip.offset(1isize) as *const libc::c_void) {
            let mut repMatchEnd: *const BYTE =
                if repIndex < prefixLowestIndex { dictEnd } else { iend };
            mLength =
                ZSTD_count_2segments(ip.offset(1isize).offset(4isize),
                                     repMatch.offset(4isize), iend,
                                     repMatchEnd,
                                     prefixLowest).wrapping_add(4i32 as
                                                                    libc::c_ulong);
            ip = ip.offset(1isize);
            ZSTD_storeSeq(seqStore,
                          anchor.offset_to(ip).expect("bad offset_to") as
                              libc::c_long as size_t,
                          anchor as *const libc::c_void, 0i32 as U32,
                          mLength.wrapping_sub(3i32 as libc::c_ulong));
        } else if dictMode as libc::c_uint ==
                      ZSTD_noDict as libc::c_int as libc::c_uint &&
                      0 !=
                          (offset_1 > 0i32 as libc::c_uint) as libc::c_int &
                              (MEM_read32(ip.offset(1isize).offset(-(offset_1
                                                                         as
                                                                         isize))
                                              as *const libc::c_void) ==
                                   MEM_read32(ip.offset(1isize) as
                                                  *const libc::c_void)) as
                                  libc::c_int {
            mLength =
                ZSTD_count(ip.offset(1isize).offset(4isize),
                           ip.offset(1isize).offset(4isize).offset(-(offset_1
                                                                         as
                                                                         isize)),
                           iend).wrapping_add(4i32 as libc::c_ulong);
            ip = ip.offset(1isize);
            ZSTD_storeSeq(seqStore,
                          anchor.offset_to(ip).expect("bad offset_to") as
                              libc::c_long as size_t,
                          anchor as *const libc::c_void, 0i32 as U32,
                          mLength.wrapping_sub(3i32 as libc::c_ulong));
        } else {
            if matchIndexL > prefixLowestIndex &&
                   MEM_read64(matchLong as *const libc::c_void) ==
                       MEM_read64(ip as *const libc::c_void) {
                mLength =
                    ZSTD_count(ip.offset(8isize), matchLong.offset(8isize),
                               iend).wrapping_add(8i32 as libc::c_ulong);
                offset =
                    matchLong.offset_to(ip).expect("bad offset_to") as
                        libc::c_long as U32;
                while 0 !=
                          (ip > anchor) as libc::c_int &
                              (matchLong > prefixLowest) as libc::c_int &&
                          *ip.offset(-1i32 as isize) as libc::c_int ==
                              *matchLong.offset(-1i32 as isize) as libc::c_int
                      {
                    ip = ip.offset(-1isize);
                    matchLong = matchLong.offset(-1isize);
                    mLength = mLength.wrapping_add(1)
                }
            } else {
                if dictMode as libc::c_uint ==
                       ZSTD_dictMatchState as libc::c_int as libc::c_uint {
                    let dictMatchIndexL: U32 =
                        *dictHashLong.offset(h2 as isize);
                    let mut dictMatchL: *const BYTE =
                        dictBase.offset(dictMatchIndexL as isize);
                    if dictMatchL > dictStart &&
                           MEM_read64(dictMatchL as *const libc::c_void) ==
                               MEM_read64(ip as *const libc::c_void) {
                        mLength =
                            ZSTD_count_2segments(ip.offset(8isize),
                                                 dictMatchL.offset(8isize),
                                                 iend, dictEnd,
                                                 prefixLowest).wrapping_add(8i32
                                                                                as
                                                                                libc::c_ulong);
                        offset =
                            current.wrapping_sub(dictMatchIndexL).wrapping_sub(dictIndexDelta);
                        while 0 !=
                                  (ip > anchor) as libc::c_int &
                                      (dictMatchL > dictStart) as libc::c_int
                                  &&
                                  *ip.offset(-1i32 as isize) as libc::c_int ==
                                      *dictMatchL.offset(-1i32 as isize) as
                                          libc::c_int {
                            ip = ip.offset(-1isize);
                            dictMatchL = dictMatchL.offset(-1isize);
                            mLength = mLength.wrapping_add(1)
                        }
                        current_block = 11015529106535645207;
                    } else { current_block = 13242334135786603907; }
                } else { current_block = 13242334135786603907; }
                match current_block {
                    11015529106535645207 => { }
                    _ => {
                        if !(matchIndexS > prefixLowestIndex &&
                                 MEM_read32(match_0 as *const libc::c_void) ==
                                     MEM_read32(ip as *const libc::c_void)) {
                            if dictMode as libc::c_uint ==
                                   ZSTD_dictMatchState as libc::c_int as
                                       libc::c_uint {
                                let dictMatchIndexS: U32 =
                                    *dictHashSmall.offset(h as isize);
                                match_0 =
                                    dictBase.offset(dictMatchIndexS as isize);
                                matchIndexS =
                                    dictMatchIndexS.wrapping_add(dictIndexDelta);
                                if match_0 > dictStart &&
                                       MEM_read32(match_0 as
                                                      *const libc::c_void) ==
                                           MEM_read32(ip as
                                                          *const libc::c_void)
                                   {
                                    current_block = 15345278821338558188;
                                } else {
                                    current_block = 15768484401365413375;
                                }
                            } else { current_block = 15768484401365413375; }
                            match current_block {
                                15345278821338558188 => { }
                                _ => {
                                    ip =
                                        ip.offset(((anchor.offset_to(ip).expect("bad offset_to")
                                                        as libc::c_long >>
                                                        8i32) +
                                                       1i32 as libc::c_long)
                                                      as isize);
                                    continue ;
                                }
                            }
                        }
                        let hl3: size_t =
                            ZSTD_hashPtr(ip.offset(1isize) as
                                             *const libc::c_void, hBitsL,
                                         8i32 as U32);
                        let matchIndexL3: U32 =
                            *hashLong.offset(hl3 as isize);
                        let mut matchL3: *const BYTE =
                            base.offset(matchIndexL3 as isize);
                        *hashLong.offset(hl3 as isize) =
                            current.wrapping_add(1i32 as libc::c_uint);
                        if matchIndexL3 > prefixLowestIndex &&
                               MEM_read64(matchL3 as *const libc::c_void) ==
                                   MEM_read64(ip.offset(1isize) as
                                                  *const libc::c_void) {
                            mLength =
                                ZSTD_count(ip.offset(9isize),
                                           matchL3.offset(8isize),
                                           iend).wrapping_add(8i32 as
                                                                  libc::c_ulong);
                            ip = ip.offset(1isize);
                            offset =
                                matchL3.offset_to(ip).expect("bad offset_to")
                                    as libc::c_long as U32;
                            while 0 !=
                                      (ip > anchor) as libc::c_int &
                                          (matchL3 > prefixLowest) as
                                              libc::c_int &&
                                      *ip.offset(-1i32 as isize) as
                                          libc::c_int ==
                                          *matchL3.offset(-1i32 as isize) as
                                              libc::c_int {
                                ip = ip.offset(-1isize);
                                matchL3 = matchL3.offset(-1isize);
                                mLength = mLength.wrapping_add(1)
                            }
                        } else {
                            if dictMode as libc::c_uint ==
                                   ZSTD_dictMatchState as libc::c_int as
                                       libc::c_uint {
                                let dictMatchIndexL3: U32 =
                                    *dictHashLong.offset(hl3 as isize);
                                let mut dictMatchL3: *const BYTE =
                                    dictBase.offset(dictMatchIndexL3 as
                                                        isize);
                                if dictMatchL3 > dictStart &&
                                       MEM_read64(dictMatchL3 as
                                                      *const libc::c_void) ==
                                           MEM_read64(ip.offset(1isize) as
                                                          *const libc::c_void)
                                   {
                                    mLength =
                                        ZSTD_count_2segments(ip.offset(1isize).offset(8isize),
                                                             dictMatchL3.offset(8isize),
                                                             iend, dictEnd,
                                                             prefixLowest).wrapping_add(8i32
                                                                                            as
                                                                                            libc::c_ulong);
                                    ip = ip.offset(1isize);
                                    offset =
                                        current.wrapping_add(1i32 as
                                                                 libc::c_uint).wrapping_sub(dictMatchIndexL3).wrapping_sub(dictIndexDelta);
                                    while 0 !=
                                              (ip > anchor) as libc::c_int &
                                                  (dictMatchL3 > dictStart) as
                                                      libc::c_int &&
                                              *ip.offset(-1i32 as isize) as
                                                  libc::c_int ==
                                                  *dictMatchL3.offset(-1i32 as
                                                                          isize)
                                                      as libc::c_int {
                                        ip = ip.offset(-1isize);
                                        dictMatchL3 =
                                            dictMatchL3.offset(-1isize);
                                        mLength = mLength.wrapping_add(1)
                                    }
                                    current_block = 11015529106535645207;
                                } else {
                                    current_block = 8845338526596852646;
                                }
                            } else { current_block = 8845338526596852646; }
                            match current_block {
                                11015529106535645207 => { }
                                _ => {
                                    if dictMode as libc::c_uint ==
                                           ZSTD_dictMatchState as libc::c_int
                                               as libc::c_uint &&
                                           matchIndexS < prefixLowestIndex {
                                        mLength =
                                            ZSTD_count_2segments(ip.offset(4isize),
                                                                 match_0.offset(4isize),
                                                                 iend,
                                                                 dictEnd,
                                                                 prefixLowest).wrapping_add(4i32
                                                                                                as
                                                                                                libc::c_ulong);
                                        offset =
                                            current.wrapping_sub(matchIndexS);
                                        while 0 !=
                                                  (ip > anchor) as libc::c_int
                                                      &
                                                      (match_0 > dictStart) as
                                                          libc::c_int &&
                                                  *ip.offset(-1i32 as isize)
                                                      as libc::c_int ==
                                                      *match_0.offset(-1i32 as
                                                                          isize)
                                                          as libc::c_int {
                                            ip = ip.offset(-1isize);
                                            match_0 = match_0.offset(-1isize);
                                            mLength = mLength.wrapping_add(1)
                                        }
                                    } else {
                                        mLength =
                                            ZSTD_count(ip.offset(4isize),
                                                       match_0.offset(4isize),
                                                       iend).wrapping_add(4i32
                                                                              as
                                                                              libc::c_ulong);
                                        offset =
                                            match_0.offset_to(ip).expect("bad offset_to")
                                                as libc::c_long as U32;
                                        while 0 !=
                                                  (ip > anchor) as libc::c_int
                                                      &
                                                      (match_0 > prefixLowest)
                                                          as libc::c_int &&
                                                  *ip.offset(-1i32 as isize)
                                                      as libc::c_int ==
                                                      *match_0.offset(-1i32 as
                                                                          isize)
                                                          as libc::c_int {
                                            ip = ip.offset(-1isize);
                                            match_0 = match_0.offset(-1isize);
                                            mLength = mLength.wrapping_add(1)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            offset_2 = offset_1;
            offset_1 = offset;
            ZSTD_storeSeq(seqStore,
                          anchor.offset_to(ip).expect("bad offset_to") as
                              libc::c_long as size_t,
                          anchor as *const libc::c_void,
                          offset.wrapping_add((3i32 - 1i32) as libc::c_uint),
                          mLength.wrapping_sub(3i32 as libc::c_ulong));
        }
        ip = ip.offset(mLength as isize);
        anchor = ip;
        if !(ip <= ilimit) { continue ; }
        let ref mut fresh1 =
            *hashSmall.offset(ZSTD_hashPtr(base.offset(current as
                                                           isize).offset(2isize)
                                               as *const libc::c_void, hBitsS,
                                           mls) as isize);
        *fresh1 = current.wrapping_add(2i32 as libc::c_uint);
        *hashLong.offset(ZSTD_hashPtr(base.offset(current as
                                                      isize).offset(2isize) as
                                          *const libc::c_void, hBitsL,
                                      8i32 as U32) as isize) = *fresh1;
        let ref mut fresh2 =
            *hashSmall.offset(ZSTD_hashPtr(ip.offset(-2isize) as
                                               *const libc::c_void, hBitsS,
                                           mls) as isize);
        *fresh2 =
            base.offset_to(ip.offset(-2isize)).expect("bad offset_to") as
                libc::c_long as U32;
        *hashLong.offset(ZSTD_hashPtr(ip.offset(-2isize) as
                                          *const libc::c_void, hBitsL,
                                      8i32 as U32) as isize) = *fresh2;
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            while ip <= ilimit {
                let current2: U32 =
                    base.offset_to(ip).expect("bad offset_to") as libc::c_long
                        as U32;
                let repIndex2: U32 = current2.wrapping_sub(offset_2);
                let mut repMatch2: *const BYTE =
                    if dictMode as libc::c_uint ==
                           ZSTD_dictMatchState as libc::c_int as libc::c_uint
                           && repIndex2 < prefixLowestIndex {
                        dictBase.offset(-(dictIndexDelta as
                                              isize)).offset(repIndex2 as
                                                                 isize)
                    } else { base.offset(repIndex2 as isize) };
                if !(prefixLowestIndex.wrapping_sub(1i32 as
                                                        libc::c_uint).wrapping_sub(repIndex2)
                         >= 3i32 as libc::c_uint &&
                         MEM_read32(repMatch2 as *const libc::c_void) ==
                             MEM_read32(ip as *const libc::c_void)) {
                    break ;
                }
                let repEnd2: *const BYTE =
                    if repIndex2 < prefixLowestIndex {
                        dictEnd
                    } else { iend };
                let repLength2: size_t =
                    ZSTD_count_2segments(ip.offset(4isize),
                                         repMatch2.offset(4isize), iend,
                                         repEnd2,
                                         prefixLowest).wrapping_add(4i32 as
                                                                        libc::c_ulong);
                let mut tmpOffset: U32 = offset_2;
                offset_2 = offset_1;
                offset_1 = tmpOffset;
                ZSTD_storeSeq(seqStore, 0i32 as size_t,
                              anchor as *const libc::c_void, 0i32 as U32,
                              repLength2.wrapping_sub(3i32 as libc::c_ulong));
                *hashSmall.offset(ZSTD_hashPtr(ip as *const libc::c_void,
                                               hBitsS, mls) as isize) =
                    current2;
                *hashLong.offset(ZSTD_hashPtr(ip as *const libc::c_void,
                                              hBitsL, 8i32 as U32) as isize) =
                    current2;
                ip = ip.offset(repLength2 as isize);
                anchor = ip
            }
        }
        if !(dictMode as libc::c_uint ==
                 ZSTD_noDict as libc::c_int as libc::c_uint) {
            continue ;
        }
        while ip <= ilimit &&
                  0 !=
                      (offset_2 > 0i32 as libc::c_uint) as libc::c_int &
                          (MEM_read32(ip as *const libc::c_void) ==
                               MEM_read32(ip.offset(-(offset_2 as isize)) as
                                              *const libc::c_void)) as
                              libc::c_int {
            let rLength: size_t =
                ZSTD_count(ip.offset(4isize),
                           ip.offset(4isize).offset(-(offset_2 as isize)),
                           iend).wrapping_add(4i32 as libc::c_ulong);
            let tmpOff: U32 = offset_2;
            offset_2 = offset_1;
            offset_1 = tmpOff;
            *hashSmall.offset(ZSTD_hashPtr(ip as *const libc::c_void, hBitsS,
                                           mls) as isize) =
                base.offset_to(ip).expect("bad offset_to") as libc::c_long as
                    U32;
            *hashLong.offset(ZSTD_hashPtr(ip as *const libc::c_void, hBitsL,
                                          8i32 as U32) as isize) =
                base.offset_to(ip).expect("bad offset_to") as libc::c_long as
                    U32;
            ZSTD_storeSeq(seqStore, 0i32 as size_t,
                          anchor as *const libc::c_void, 0i32 as U32,
                          rLength.wrapping_sub(3i32 as libc::c_ulong));
            ip = ip.offset(rLength as isize);
            anchor = ip
        }
    }
    *rep.offset(0isize) = if 0 != offset_1 { offset_1 } else { offsetSaved };
    *rep.offset(1isize) = if 0 != offset_2 { offset_2 } else { offsetSaved };
    return anchor.offset_to(iend).expect("bad offset_to") as libc::c_long as
               size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_doubleFast_dictMatchState(mut ms:
                                                                          *mut ZSTD_matchState_t,
                                                                      mut seqStore:
                                                                          *mut seqStore_t,
                                                                      mut rep:
                                                                          *mut U32,
                                                                      mut cParams:
                                                                          *const ZSTD_compressionParameters,
                                                                      mut src:
                                                                          *const libc::c_void,
                                                                      mut srcSize:
                                                                          size_t)
 -> size_t {
    let mls: U32 = (*cParams).searchLength;
    match mls {
        5 => {
            return ZSTD_compressBlock_doubleFast_generic(ms, seqStore, rep,
                                                         cParams, src,
                                                         srcSize, 5i32 as U32,
                                                         ZSTD_dictMatchState)
        }
        6 => {
            return ZSTD_compressBlock_doubleFast_generic(ms, seqStore, rep,
                                                         cParams, src,
                                                         srcSize, 6i32 as U32,
                                                         ZSTD_dictMatchState)
        }
        7 => {
            return ZSTD_compressBlock_doubleFast_generic(ms, seqStore, rep,
                                                         cParams, src,
                                                         srcSize, 7i32 as U32,
                                                         ZSTD_dictMatchState)
        }
        4 | _ => {
            return ZSTD_compressBlock_doubleFast_generic(ms, seqStore, rep,
                                                         cParams, src,
                                                         srcSize, 4i32 as U32,
                                                         ZSTD_dictMatchState)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_doubleFast_extDict(mut ms:
                                                                   *mut ZSTD_matchState_t,
                                                               mut seqStore:
                                                                   *mut seqStore_t,
                                                               mut rep:
                                                                   *mut U32,
                                                               mut cParams:
                                                                   *const ZSTD_compressionParameters,
                                                               mut src:
                                                                   *const libc::c_void,
                                                               mut srcSize:
                                                                   size_t)
 -> size_t {
    let mls: U32 = (*cParams).searchLength;
    match mls {
        5 => {
            return ZSTD_compressBlock_doubleFast_extDict_generic(ms, seqStore,
                                                                 rep, cParams,
                                                                 src, srcSize,
                                                                 5i32 as U32)
        }
        6 => {
            return ZSTD_compressBlock_doubleFast_extDict_generic(ms, seqStore,
                                                                 rep, cParams,
                                                                 src, srcSize,
                                                                 6i32 as U32)
        }
        7 => {
            return ZSTD_compressBlock_doubleFast_extDict_generic(ms, seqStore,
                                                                 rep, cParams,
                                                                 src, srcSize,
                                                                 7i32 as U32)
        }
        4 | _ => {
            return ZSTD_compressBlock_doubleFast_extDict_generic(ms, seqStore,
                                                                 rep, cParams,
                                                                 src, srcSize,
                                                                 4i32 as U32)
        }
    };
}
unsafe extern "C" fn ZSTD_compressBlock_doubleFast_extDict_generic(mut ms:
                                                                       *mut ZSTD_matchState_t,
                                                                   mut seqStore:
                                                                       *mut seqStore_t,
                                                                   mut rep:
                                                                       *mut U32,
                                                                   mut cParams:
                                                                       *const ZSTD_compressionParameters,
                                                                   mut src:
                                                                       *const libc::c_void,
                                                                   mut srcSize:
                                                                       size_t,
                                                                   mls: U32)
 -> size_t {
    let mut repMatchEnd: *const BYTE = 0 as *const BYTE;
    let hashLong: *mut U32 = (*ms).hashTable;
    let hBitsL: U32 = (*cParams).hashLog;
    let hashSmall: *mut U32 = (*ms).chainTable;
    let hBitsS: U32 = (*cParams).chainLog;
    let istart: *const BYTE = src as *const BYTE;
    let mut ip: *const BYTE = istart;
    let mut anchor: *const BYTE = istart;
    let iend: *const BYTE = istart.offset(srcSize as isize);
    let ilimit: *const BYTE = iend.offset(-8isize);
    let prefixStartIndex: U32 = (*ms).window.dictLimit;
    let base: *const BYTE = (*ms).window.base;
    let prefixStart: *const BYTE = base.offset(prefixStartIndex as isize);
    let dictStartIndex: U32 = (*ms).window.lowLimit;
    let dictBase: *const BYTE = (*ms).window.dictBase;
    let dictStart: *const BYTE = dictBase.offset(dictStartIndex as isize);
    let dictEnd: *const BYTE = dictBase.offset(prefixStartIndex as isize);
    let mut offset_1: U32 = *rep.offset(0isize);
    let mut offset_2: U32 = *rep.offset(1isize);
    while ip < ilimit {
        let hSmall: size_t =
            ZSTD_hashPtr(ip as *const libc::c_void, hBitsS, mls);
        let matchIndex: U32 = *hashSmall.offset(hSmall as isize);
        let matchBase: *const BYTE =
            if matchIndex < prefixStartIndex { dictBase } else { base };
        let mut match_0: *const BYTE = matchBase.offset(matchIndex as isize);
        let hLong: size_t =
            ZSTD_hashPtr(ip as *const libc::c_void, hBitsL, 8i32 as U32);
        let matchLongIndex: U32 = *hashLong.offset(hLong as isize);
        let matchLongBase: *const BYTE =
            if matchLongIndex < prefixStartIndex { dictBase } else { base };
        let mut matchLong: *const BYTE =
            matchLongBase.offset(matchLongIndex as isize);
        let current: U32 =
            base.offset_to(ip).expect("bad offset_to") as libc::c_long as U32;
        let repIndex: U32 =
            current.wrapping_add(1i32 as libc::c_uint).wrapping_sub(offset_1);
        let repBase: *const BYTE =
            if repIndex < prefixStartIndex { dictBase } else { base };
        let repMatch: *const BYTE = repBase.offset(repIndex as isize);
        let mut mLength: size_t = 0;
        let ref mut fresh3 = *hashLong.offset(hLong as isize);
        *fresh3 = current;
        *hashSmall.offset(hSmall as isize) = *fresh3;
        if 0 !=
               (prefixStartIndex.wrapping_sub(1i32 as
                                                  libc::c_uint).wrapping_sub(repIndex)
                    >= 3i32 as libc::c_uint) as libc::c_int &
                   (repIndex > dictStartIndex) as libc::c_int &&
               MEM_read32(repMatch as *const libc::c_void) ==
                   MEM_read32(ip.offset(1isize) as *const libc::c_void) {
            repMatchEnd =
                if repIndex < prefixStartIndex { dictEnd } else { iend };
            mLength =
                ZSTD_count_2segments(ip.offset(1isize).offset(4isize),
                                     repMatch.offset(4isize), iend,
                                     repMatchEnd,
                                     prefixStart).wrapping_add(4i32 as
                                                                   libc::c_ulong);
            ip = ip.offset(1isize);
            ZSTD_storeSeq(seqStore,
                          anchor.offset_to(ip).expect("bad offset_to") as
                              libc::c_long as size_t,
                          anchor as *const libc::c_void, 0i32 as U32,
                          mLength.wrapping_sub(3i32 as libc::c_ulong));
        } else if matchLongIndex > dictStartIndex &&
                      MEM_read64(matchLong as *const libc::c_void) ==
                          MEM_read64(ip as *const libc::c_void) {
            let matchEnd: *const BYTE =
                if matchLongIndex < prefixStartIndex {
                    dictEnd
                } else { iend };
            let lowMatchPtr: *const BYTE =
                if matchLongIndex < prefixStartIndex {
                    dictStart
                } else { prefixStart };
            let mut offset: U32 = 0;
            mLength =
                ZSTD_count_2segments(ip.offset(8isize),
                                     matchLong.offset(8isize), iend, matchEnd,
                                     prefixStart).wrapping_add(8i32 as
                                                                   libc::c_ulong);
            offset = current.wrapping_sub(matchLongIndex);
            while 0 !=
                      (ip > anchor) as libc::c_int &
                          (matchLong > lowMatchPtr) as libc::c_int &&
                      *ip.offset(-1i32 as isize) as libc::c_int ==
                          *matchLong.offset(-1i32 as isize) as libc::c_int {
                ip = ip.offset(-1isize);
                matchLong = matchLong.offset(-1isize);
                mLength = mLength.wrapping_add(1)
            }
            offset_2 = offset_1;
            offset_1 = offset;
            ZSTD_storeSeq(seqStore,
                          anchor.offset_to(ip).expect("bad offset_to") as
                              libc::c_long as size_t,
                          anchor as *const libc::c_void,
                          offset.wrapping_add((3i32 - 1i32) as libc::c_uint),
                          mLength.wrapping_sub(3i32 as libc::c_ulong));
        } else if matchIndex > dictStartIndex &&
                      MEM_read32(match_0 as *const libc::c_void) ==
                          MEM_read32(ip as *const libc::c_void) {
            let h3: size_t =
                ZSTD_hashPtr(ip.offset(1isize) as *const libc::c_void, hBitsL,
                             8i32 as U32);
            let matchIndex3: U32 = *hashLong.offset(h3 as isize);
            let match3Base: *const BYTE =
                if matchIndex3 < prefixStartIndex { dictBase } else { base };
            let mut match3: *const BYTE =
                match3Base.offset(matchIndex3 as isize);
            let mut offset_0: U32 = 0;
            *hashLong.offset(h3 as isize) =
                current.wrapping_add(1i32 as libc::c_uint);
            if matchIndex3 > dictStartIndex &&
                   MEM_read64(match3 as *const libc::c_void) ==
                       MEM_read64(ip.offset(1isize) as *const libc::c_void) {
                let matchEnd_0: *const BYTE =
                    if matchIndex3 < prefixStartIndex {
                        dictEnd
                    } else { iend };
                let lowMatchPtr_0: *const BYTE =
                    if matchIndex3 < prefixStartIndex {
                        dictStart
                    } else { prefixStart };
                mLength =
                    ZSTD_count_2segments(ip.offset(9isize),
                                         match3.offset(8isize), iend,
                                         matchEnd_0,
                                         prefixStart).wrapping_add(8i32 as
                                                                       libc::c_ulong);
                ip = ip.offset(1isize);
                offset_0 =
                    current.wrapping_add(1i32 as
                                             libc::c_uint).wrapping_sub(matchIndex3);
                while 0 !=
                          (ip > anchor) as libc::c_int &
                              (match3 > lowMatchPtr_0) as libc::c_int &&
                          *ip.offset(-1i32 as isize) as libc::c_int ==
                              *match3.offset(-1i32 as isize) as libc::c_int {
                    ip = ip.offset(-1isize);
                    match3 = match3.offset(-1isize);
                    mLength = mLength.wrapping_add(1)
                }
            } else {
                let matchEnd_1: *const BYTE =
                    if matchIndex < prefixStartIndex {
                        dictEnd
                    } else { iend };
                let lowMatchPtr_1: *const BYTE =
                    if matchIndex < prefixStartIndex {
                        dictStart
                    } else { prefixStart };
                mLength =
                    ZSTD_count_2segments(ip.offset(4isize),
                                         match_0.offset(4isize), iend,
                                         matchEnd_1,
                                         prefixStart).wrapping_add(4i32 as
                                                                       libc::c_ulong);
                offset_0 = current.wrapping_sub(matchIndex);
                while 0 !=
                          (ip > anchor) as libc::c_int &
                              (match_0 > lowMatchPtr_1) as libc::c_int &&
                          *ip.offset(-1i32 as isize) as libc::c_int ==
                              *match_0.offset(-1i32 as isize) as libc::c_int {
                    ip = ip.offset(-1isize);
                    match_0 = match_0.offset(-1isize);
                    mLength = mLength.wrapping_add(1)
                }
            }
            offset_2 = offset_1;
            offset_1 = offset_0;
            ZSTD_storeSeq(seqStore,
                          anchor.offset_to(ip).expect("bad offset_to") as
                              libc::c_long as size_t,
                          anchor as *const libc::c_void,
                          offset_0.wrapping_add((3i32 - 1i32) as
                                                    libc::c_uint),
                          mLength.wrapping_sub(3i32 as libc::c_ulong));
        } else {
            ip =
                ip.offset(((anchor.offset_to(ip).expect("bad offset_to") as
                                libc::c_long >> 8i32) + 1i32 as libc::c_long)
                              as isize);
            continue ;
        }
        ip = ip.offset(mLength as isize);
        anchor = ip;
        if !(ip <= ilimit) { continue ; }
        *hashSmall.offset(ZSTD_hashPtr(base.offset(current as
                                                       isize).offset(2isize)
                                           as *const libc::c_void, hBitsS,
                                       mls) as isize) =
            current.wrapping_add(2i32 as libc::c_uint);
        *hashLong.offset(ZSTD_hashPtr(base.offset(current as
                                                      isize).offset(2isize) as
                                          *const libc::c_void, hBitsL,
                                      8i32 as U32) as isize) =
            current.wrapping_add(2i32 as libc::c_uint);
        *hashSmall.offset(ZSTD_hashPtr(ip.offset(-2isize) as
                                           *const libc::c_void, hBitsS, mls)
                              as isize) =
            base.offset_to(ip.offset(-2isize)).expect("bad offset_to") as
                libc::c_long as U32;
        *hashLong.offset(ZSTD_hashPtr(ip.offset(-2isize) as
                                          *const libc::c_void, hBitsL,
                                      8i32 as U32) as isize) =
            base.offset_to(ip.offset(-2isize)).expect("bad offset_to") as
                libc::c_long as U32;
        while ip <= ilimit {
            let current2: U32 =
                base.offset_to(ip).expect("bad offset_to") as libc::c_long as
                    U32;
            let repIndex2: U32 = current2.wrapping_sub(offset_2);
            let mut repMatch2: *const BYTE =
                if repIndex2 < prefixStartIndex {
                    dictBase.offset(repIndex2 as isize)
                } else { base.offset(repIndex2 as isize) };
            if !(0 !=
                     (prefixStartIndex.wrapping_sub(1i32 as
                                                        libc::c_uint).wrapping_sub(repIndex2)
                          >= 3i32 as libc::c_uint) as libc::c_int &
                         (repIndex2 > dictStartIndex) as libc::c_int &&
                     MEM_read32(repMatch2 as *const libc::c_void) ==
                         MEM_read32(ip as *const libc::c_void)) {
                break ;
            }
            let repEnd2: *const BYTE =
                if repIndex2 < prefixStartIndex { dictEnd } else { iend };
            let repLength2: size_t =
                ZSTD_count_2segments(ip.offset(4isize),
                                     repMatch2.offset(4isize), iend, repEnd2,
                                     prefixStart).wrapping_add(4i32 as
                                                                   libc::c_ulong);
            let tmpOffset: U32 = offset_2;
            offset_2 = offset_1;
            offset_1 = tmpOffset;
            ZSTD_storeSeq(seqStore, 0i32 as size_t,
                          anchor as *const libc::c_void, 0i32 as U32,
                          repLength2.wrapping_sub(3i32 as libc::c_ulong));
            *hashSmall.offset(ZSTD_hashPtr(ip as *const libc::c_void, hBitsS,
                                           mls) as isize) = current2;
            *hashLong.offset(ZSTD_hashPtr(ip as *const libc::c_void, hBitsL,
                                          8i32 as U32) as isize) = current2;
            ip = ip.offset(repLength2 as isize);
            anchor = ip
        }
    }
    *rep.offset(0isize) = offset_1;
    *rep.offset(1isize) = offset_2;
    return anchor.offset_to(iend).expect("bad offset_to") as libc::c_long as
               size_t;
}
