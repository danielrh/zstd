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
    /* ! Constructor and Destructor of FSE_DTable.
    Note that its size depends on 'tableLog' */
    /* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
    /* *< Cannot use the previous table */
    /* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
    /* *< Can use the previous table but it must be checked. Note : The previous table must have been constructed by HUF_compress{1, 4}X_repeat */
    /* *< Cannot use the previous table */
    /* *< Can use the previous table but it must be checked */
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
    /* * HUF_getNbBits() :
 *  Read nbBits from CTable symbolTable, for symbol `symbolValue` presumed <= HUF_SYMBOLVALUE_MAX
 *  Note 1 : is not inlined, as HUF_CElt definition is private
 *  Note 2 : const void* used, so that it can provide a statically allocated table as argument (which uses type U32) */
    #[no_mangle]
    fn HUF_getNbBits(symbolTable: *const libc::c_void, symbolValue: U32)
     -> U32;
    /* ! HIST_count_simple() :
 *  Same as HIST_countFast(), this function is unsafe,
 *  and will segfault if any value within `src` is `> *maxSymbolValuePtr`.
 *  It is also a bit slower for large inputs.
 *  However, it does not need any additional memory (not even on stack).
 * @return : count of the most frequent symbol.
 *  Note this function doesn't produce any error (i.e. it must succeed).
 */
    #[no_mangle]
    fn HIST_count_simple(count: *mut libc::c_uint,
                         maxSymbolValuePtr: *mut libc::c_uint,
                         src: *const libc::c_void, srcSize: size_t)
     -> libc::c_uint;
}
pub const zop_dynamic: ZSTD_OptPrice_e = 0;
pub type S16 = int16_t;
pub type BIT_DStream_status = libc::c_uint;
pub type HUF_repeat = libc::c_uint;
pub type BYTE = uint8_t;
pub type ZSTD_strategy = libc::c_uint;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const ZSTD_error_dictionary_wrong: ERR_enum = 32;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DState_t {
    pub state: size_t,
    pub table: *const libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: libc::c_int,
    pub deltaNbBits: U32,
}
pub const ZSTD_error_maxSymbolValue_tooSmall: ERR_enum = 48;
pub type FSE_DTable = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct seqDef_s {
    pub offset: U32,
    pub litLength: U16,
    pub matchLength: U16,
}
pub const MEM_static_assert: unnamed_0 = 1;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub type U32 = uint32_t;
pub type FSE_CTable = libc::c_uint;
pub type uint64_t = libc::c_ulong;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const libc::c_void,
    pub symbolTT: *const libc::c_void,
    pub stateLog: libc::c_uint,
}
pub type U64 = uint64_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct repcodes_s {
    pub rep: [U32; 3],
}
pub type ptrdiff_t = libc::c_long;
pub const ZSTD_error_init_missing: ERR_enum = 62;
pub type U16 = uint16_t;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const ZSTD_greedy: ZSTD_strategy = 3;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
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
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub type uint16_t = libc::c_ushort;
pub const ZSTD_error_frameParameter_windowTooLarge: ERR_enum = 16;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub type ZSTD_OptPrice_e = libc::c_uint;
pub const ZSTD_error_parameter_unsupported: ERR_enum = 40;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_window_t {
    pub nextSrc: *const BYTE,
    pub base: *const BYTE,
    pub dictBase: *const BYTE,
    pub dictLimit: U32,
    pub lowLimit: U32,
}
pub type int16_t = libc::c_short;
pub type ZSTD_dictMode_e = libc::c_uint;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
pub const ZSTD_error_no_error: ERR_enum = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
pub const HUF_repeat_check: HUF_repeat = 1;
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
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_error_memory_allocation: ERR_enum = 64;
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub const MEM_static_assert_0: unnamed_1 = 1;
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type ERR_enum = libc::c_uint;
pub const ZSTD_lazy: ZSTD_strategy = 4;
pub const ZSTD_error_version_unsupported: ERR_enum = 12;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_CStream_t {
    pub bitContainer: size_t,
    pub bitPos: libc::c_uint,
    pub startPtr: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub endPtr: *mut libc::c_char,
}
pub const ZSTD_btopt: ZSTD_strategy = 7;
pub type repcodes_t = repcodes_s;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_hufCTables_t {
    pub CTable: [U32; 256],
    pub repeatMode: HUF_repeat,
}
pub const ZSTD_error_frameIndex_tooLarge: ERR_enum = 100;
pub const ZSTD_error_corruption_detected: ERR_enum = 20;
pub type seqDef = seqDef_s;
pub const ZSTD_error_dstSize_tooSmall: ERR_enum = 70;
pub const ZSTD_error_dictionary_corrupted: ERR_enum = 30;
pub const ZSTD_error_tableLog_tooLarge: ERR_enum = 44;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub type uint32_t = libc::c_uint;
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
pub const ZSTD_error_parameter_outOfBound: ERR_enum = 42;
pub const ZSTD_error_dictionaryCreation_failed: ERR_enum = 34;
pub const ZSTD_error_GENERIC: ERR_enum = 1;
pub const FSE_repeat_none: FSE_repeat = 0;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub const ZSTD_fast: ZSTD_strategy = 1;
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
pub const HUF_repeat_valid: HUF_repeat = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    u: U32,
    c: [BYTE; 4],
}
pub const ZSTD_error_stage_wrong: ERR_enum = 60;
pub const ZSTD_error_maxCode: ERR_enum = 120;
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
pub struct ZSTD_match_t {
    pub off: U32,
    pub len: U32,
}
pub type FSE_repeat = libc::c_uint;
pub const ZSTD_error_checksum_wrong: ERR_enum = 22;
pub type ZSTD_matchState_t = ZSTD_matchState_t_0;
pub type unnamed_0 = libc::c_uint;
pub const ZSTD_error_srcSize_wrong: ERR_enum = 72;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_entropyCTables_t {
    pub huf: ZSTD_hufCTables_t,
    pub fse: ZSTD_fseCTables_t,
}
pub const FSE_repeat_check: FSE_repeat = 1;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub type ZSTD_ErrorCode = ERR_enum;
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
pub type uint8_t = libc::c_uchar;
pub const ZSTD_error_maxSymbolValue_tooLarge: ERR_enum = 46;
pub const ZSTD_error_frameParameter_unsupported: ERR_enum = 14;
pub type size_t = libc::c_ulong;
pub const ZSTD_error_prefix_unknown: ERR_enum = 10;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_optimal_t {
    pub price: libc::c_int,
    pub off: U32,
    pub mlen: U32,
    pub litlen: U32,
    pub rep: [U32; 3],
}
pub const ZSTD_error_workSpace_tooSmall: ERR_enum = 66;
pub type unnamed_1 = libc::c_uint;
pub const FSE_repeat_valid: FSE_repeat = 2;
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
pub const ZSTD_error_seekableIO: ERR_enum = 102;
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
pub unsafe extern "C" fn ZSTD_updateTree(mut ms: *mut ZSTD_matchState_t,
                                         mut cParams:
                                             *const ZSTD_compressionParameters,
                                         mut ip: *const BYTE,
                                         mut iend: *const BYTE) -> () {
    ZSTD_updateTree_internal(ms, cParams, ip, iend, (*cParams).searchLength,
                             ZSTD_noDict);
}
unsafe extern "C" fn ZSTD_updateTree_internal(mut ms: *mut ZSTD_matchState_t,
                                              mut cParams:
                                                  *const ZSTD_compressionParameters,
                                              ip: *const BYTE,
                                              iend: *const BYTE, mls: U32,
                                              dictMode: ZSTD_dictMode_e)
 -> () {
    let base: *const BYTE = (*ms).window.base;
    let target: U32 =
        base.offset_to(ip).expect("bad offset_to") as libc::c_long as U32;
    let mut idx: U32 = (*ms).nextToUpdate;
    while idx < target {
        idx =
            (idx as
                 libc::c_uint).wrapping_add(ZSTD_insertBt1(ms, cParams,
                                                           base.offset(idx as
                                                                           isize),
                                                           iend, mls,
                                                           (dictMode as
                                                                libc::c_uint
                                                                ==
                                                                ZSTD_extDict
                                                                    as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                                               as
                                                               libc::c_int))
                as U32 as U32
    }
    (*ms).nextToUpdate = target;
}
/* * ZSTD_insertBt1() : add one or multiple positions to tree.
 *  ip : assumed <= iend-8 .
 * @return : nb of positions added */
unsafe extern "C" fn ZSTD_insertBt1(mut ms: *mut ZSTD_matchState_t,
                                    mut cParams:
                                        *const ZSTD_compressionParameters,
                                    ip: *const BYTE, iend: *const BYTE,
                                    mls: U32, extDict: libc::c_int) -> U32 {
    let hashTable: *mut U32 = (*ms).hashTable;
    let hashLog: U32 = (*cParams).hashLog;
    let h: size_t = ZSTD_hashPtr(ip as *const libc::c_void, hashLog, mls);
    let bt: *mut U32 = (*ms).chainTable;
    let btLog: U32 = (*cParams).chainLog.wrapping_sub(1i32 as libc::c_uint);
    let btMask: U32 = ((1i32 << btLog) - 1i32) as U32;
    let mut matchIndex: U32 = *hashTable.offset(h as isize);
    let mut commonLengthSmaller: size_t = 0i32 as size_t;
    let mut commonLengthLarger: size_t = 0i32 as size_t;
    let base: *const BYTE = (*ms).window.base;
    let dictBase: *const BYTE = (*ms).window.dictBase;
    let dictLimit: U32 = (*ms).window.dictLimit;
    let dictEnd: *const BYTE = dictBase.offset(dictLimit as isize);
    let prefixStart: *const BYTE = base.offset(dictLimit as isize);
    let mut match_0: *const BYTE = 0 as *const BYTE;
    let current: U32 =
        base.offset_to(ip).expect("bad offset_to") as libc::c_long as U32;
    let btLow: U32 =
        if btMask >= current {
            0i32 as libc::c_uint
        } else { current.wrapping_sub(btMask) };
    let mut smallerPtr: *mut U32 =
        bt.offset((2i32 as libc::c_uint).wrapping_mul(current & btMask) as
                      isize);
    let mut largerPtr: *mut U32 = smallerPtr.offset(1isize);
    let mut dummy32: U32 = 0;
    let windowLow: U32 = (*ms).window.lowLimit;
    let matchLow: U32 =
        if 0 != windowLow { windowLow } else { 1i32 as libc::c_uint };
    let mut matchEndIdx: U32 =
        current.wrapping_add(8i32 as
                                 libc::c_uint).wrapping_add(1i32 as
                                                                libc::c_uint);
    let mut bestLength: size_t = 8i32 as size_t;
    let mut nbCompares: U32 = 1u32 << (*cParams).searchLog;
    *hashTable.offset(h as isize) = current;
    loop  {
        let fresh0 = nbCompares;
        nbCompares = nbCompares.wrapping_sub(1);
        if !(0 != fresh0 && matchIndex >= matchLow) { break ; }
        let nextPtr: *mut U32 =
            bt.offset((2i32 as libc::c_uint).wrapping_mul(matchIndex & btMask)
                          as isize);
        let mut matchLength: size_t =
            if commonLengthSmaller < commonLengthLarger {
                commonLengthSmaller
            } else { commonLengthLarger };
        if 0 == extDict ||
               (matchIndex as libc::c_ulong).wrapping_add(matchLength) >=
                   dictLimit as libc::c_ulong {
            match_0 = base.offset(matchIndex as isize);
            matchLength =
                (matchLength as
                     libc::c_ulong).wrapping_add(ZSTD_count(ip.offset(matchLength
                                                                          as
                                                                          isize),
                                                            match_0.offset(matchLength
                                                                               as
                                                                               isize),
                                                            iend)) as size_t
                    as size_t
        } else {
            match_0 = dictBase.offset(matchIndex as isize);
            matchLength =
                (matchLength as
                     libc::c_ulong).wrapping_add(ZSTD_count_2segments(ip.offset(matchLength
                                                                                    as
                                                                                    isize),
                                                                      match_0.offset(matchLength
                                                                                         as
                                                                                         isize),
                                                                      iend,
                                                                      dictEnd,
                                                                      prefixStart))
                    as size_t as size_t;
            if (matchIndex as libc::c_ulong).wrapping_add(matchLength) >=
                   dictLimit as libc::c_ulong {
                match_0 = base.offset(matchIndex as isize)
            }
        }
        if matchLength > bestLength {
            bestLength = matchLength;
            if matchLength >
                   matchEndIdx.wrapping_sub(matchIndex) as libc::c_ulong {
                matchEndIdx = matchIndex.wrapping_add(matchLength as U32)
            }
        }
        if ip.offset(matchLength as isize) == iend { break ; }
        if (*match_0.offset(matchLength as isize) as libc::c_int) <
               *ip.offset(matchLength as isize) as libc::c_int {
            *smallerPtr = matchIndex;
            commonLengthSmaller = matchLength;
            if matchIndex <= btLow {
                smallerPtr = &mut dummy32 as *mut U32;
                break ;
            } else {
                smallerPtr = nextPtr.offset(1isize);
                matchIndex = *nextPtr.offset(1isize)
            }
        } else {
            *largerPtr = matchIndex;
            commonLengthLarger = matchLength;
            if matchIndex <= btLow {
                largerPtr = &mut dummy32 as *mut U32;
                break ;
            } else {
                largerPtr = nextPtr;
                matchIndex = *nextPtr.offset(0isize)
            }
        }
    }
    *largerPtr = 0i32 as U32;
    *smallerPtr = *largerPtr;
    if bestLength > 384i32 as libc::c_ulong {
        return if (192i32 as libc::c_uint) <
                      bestLength.wrapping_sub(384i32 as libc::c_ulong) as U32
                  {
                   192i32 as libc::c_uint
               } else {
                   bestLength.wrapping_sub(384i32 as libc::c_ulong) as U32
               }
    } else {
        return matchEndIdx.wrapping_sub(current.wrapping_add(8i32 as
                                                                 libc::c_uint))
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btopt(mut ms:
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
    return ZSTD_compressBlock_opt_generic(ms, seqStore, rep, cParams, src,
                                          srcSize, 0i32, ZSTD_noDict);
}
unsafe extern "C" fn ZSTD_compressBlock_opt_generic(mut ms:
                                                        *mut ZSTD_matchState_t,
                                                    mut seqStore:
                                                        *mut seqStore_t,
                                                    mut rep: *mut U32,
                                                    mut cParams:
                                                        *const ZSTD_compressionParameters,
                                                    mut src:
                                                        *const libc::c_void,
                                                    mut srcSize: size_t,
                                                    optLevel: libc::c_int,
                                                    dictMode: ZSTD_dictMode_e)
 -> size_t {
    let mut current_block: u64;
    let optStatePtr: *mut optState_t = &mut (*ms).opt as *mut optState_t;
    let istart: *const BYTE = src as *const BYTE;
    let mut ip: *const BYTE = istart;
    let mut anchor: *const BYTE = istart;
    let iend: *const BYTE = istart.offset(srcSize as isize);
    let ilimit: *const BYTE = iend.offset(-8isize);
    let base: *const BYTE = (*ms).window.base;
    let prefixStart: *const BYTE =
        base.offset((*ms).window.dictLimit as isize);
    let sufficient_len: U32 =
        if (*cParams).targetLength < ((1i32 << 12i32) - 1i32) as libc::c_uint
           {
            (*cParams).targetLength
        } else { ((1i32 << 12i32) - 1i32) as libc::c_uint };
    let minMatch: U32 =
        (if (*cParams).searchLength == 3i32 as libc::c_uint {
             3i32
         } else { 4i32 }) as U32;
    let opt: *mut ZSTD_optimal_t = (*optStatePtr).priceTable;
    let matches: *mut ZSTD_match_t = (*optStatePtr).matchTable;
    let mut lastSequence: ZSTD_optimal_t =
        ZSTD_optimal_t{price: 0, off: 0, mlen: 0, litlen: 0, rep: [0; 3],};
    (*ms).nextToUpdate3 = (*ms).nextToUpdate;
    ZSTD_rescaleFreqs(optStatePtr, src as *const BYTE, srcSize, optLevel);
    ip = ip.offset((ip == prefixStart) as libc::c_int as isize);
    while ip < ilimit {
        let mut cur: U32 = 0;
        let mut last_pos: U32 = 0i32 as U32;
        let litlen: U32 =
            anchor.offset_to(ip).expect("bad offset_to") as libc::c_long as
                U32;
        let ll0: U32 = (0 == litlen) as libc::c_int as U32;
        let nbMatches: U32 =
            ZSTD_BtGetAllMatches(ms, cParams, ip, iend, dictMode, rep, ll0,
                                 matches, minMatch);
        if 0 == nbMatches {
            ip = ip.offset(1isize)
        } else {
            let mut i: U32 = 0;
            i = 0i32 as U32;
            while i < 3i32 as libc::c_uint {
                (*opt.offset(0isize)).rep[i as usize] =
                    *rep.offset(i as isize);
                i = i.wrapping_add(1)
            }
            (*opt.offset(0isize)).mlen = 0i32 as U32;
            (*opt.offset(0isize)).litlen = litlen;
            (*opt.offset(0isize)).price =
                ZSTD_literalsContribution(anchor, litlen, optStatePtr,
                                          optLevel);
            let maxML: U32 =
                (*matches.offset(nbMatches.wrapping_sub(1i32 as libc::c_uint)
                                     as isize)).len;
            let maxOffset: U32 =
                (*matches.offset(nbMatches.wrapping_sub(1i32 as libc::c_uint)
                                     as isize)).off;
            if maxML > sufficient_len {
                lastSequence.litlen = litlen;
                lastSequence.mlen = maxML;
                lastSequence.off = maxOffset;
                cur = 0i32 as U32;
                last_pos = ZSTD_totalLen(lastSequence)
            } else {
                let literalsPrice: U32 =
                    ((*opt.offset(0isize)).price as
                         libc::c_uint).wrapping_add(ZSTD_litLengthPrice(0i32
                                                                            as
                                                                            U32,
                                                                        optStatePtr,
                                                                        optLevel));
                let mut pos: U32 = 0;
                let mut matchNb: U32 = 0;
                pos = 1i32 as U32;
                while pos < minMatch {
                    (*opt.offset(pos as isize)).price = 1i32 << 30i32;
                    pos = pos.wrapping_add(1)
                }
                matchNb = 0i32 as U32;
                while matchNb < nbMatches {
                    let offset: U32 = (*matches.offset(matchNb as isize)).off;
                    let end: U32 = (*matches.offset(matchNb as isize)).len;
                    let repHistory: repcodes_t =
                        ZSTD_updateRep(rep as *const U32, offset, ll0);
                    while pos <= end {
                        let matchPrice: U32 =
                            ZSTD_getMatchPrice(offset, pos, optStatePtr,
                                               optLevel);
                        let sequencePrice: U32 =
                            literalsPrice.wrapping_add(matchPrice);
                        (*opt.offset(pos as isize)).mlen = pos;
                        (*opt.offset(pos as isize)).off = offset;
                        (*opt.offset(pos as isize)).litlen = litlen;
                        (*opt.offset(pos as isize)).price =
                            sequencePrice as libc::c_int;
                        memcpy((*opt.offset(pos as isize)).rep.as_mut_ptr() as
                                   *mut libc::c_void,
                               &repHistory as *const repcodes_t as
                                   *const libc::c_void,
                               ::std::mem::size_of::<repcodes_t>() as
                                   libc::c_ulong);
                        pos = pos.wrapping_add(1)
                    }
                    matchNb = matchNb.wrapping_add(1)
                }
                last_pos = pos.wrapping_sub(1i32 as libc::c_uint);
                cur = 1i32 as U32;
                loop  {
                    if !(cur <= last_pos) {
                        current_block = 7172762164747879670;
                        break ;
                    }
                    let inr: *const BYTE = ip.offset(cur as isize);
                    let litlen_0: U32 =
                        if (*opt.offset(cur.wrapping_sub(1i32 as libc::c_uint)
                                            as isize)).mlen ==
                               0i32 as libc::c_uint {
                            (*opt.offset(cur.wrapping_sub(1i32 as
                                                              libc::c_uint) as
                                             isize)).litlen.wrapping_add(1i32
                                                                             as
                                                                             libc::c_uint)
                        } else { 1i32 as libc::c_uint };
                    let price: libc::c_int =
                        ((*opt.offset(cur.wrapping_sub(1i32 as libc::c_uint)
                                          as isize)).price as
                             libc::c_uint).wrapping_add(ZSTD_rawLiteralsCost(ip.offset(cur
                                                                                           as
                                                                                           isize).offset(-1isize),
                                                                             1i32
                                                                                 as
                                                                                 U32,
                                                                             optStatePtr,
                                                                             optLevel)).wrapping_add(ZSTD_litLengthPrice(litlen_0,
                                                                                                                         optStatePtr,
                                                                                                                         optLevel)).wrapping_sub(ZSTD_litLengthPrice(litlen_0.wrapping_sub(1i32
                                                                                                                                                                                               as
                                                                                                                                                                                               libc::c_uint),
                                                                                                                                                                     optStatePtr,
                                                                                                                                                                     optLevel))
                            as libc::c_int;
                    if price <= (*opt.offset(cur as isize)).price {
                        (*opt.offset(cur as isize)).mlen = 0i32 as U32;
                        (*opt.offset(cur as isize)).off = 0i32 as U32;
                        (*opt.offset(cur as isize)).litlen = litlen_0;
                        (*opt.offset(cur as isize)).price = price;
                        memcpy((*opt.offset(cur as isize)).rep.as_mut_ptr() as
                                   *mut libc::c_void,
                               (*opt.offset(cur.wrapping_sub(1i32 as
                                                                 libc::c_uint)
                                                as isize)).rep.as_mut_ptr() as
                                   *const libc::c_void,
                               ::std::mem::size_of::<[U32; 3]>() as
                                   libc::c_ulong);
                    }
                    if !(inr > ilimit) {
                        if cur == last_pos {
                            current_block = 7172762164747879670;
                            break ;
                        }
                        if !(optLevel == 0i32 &&
                                 (*opt.offset(cur.wrapping_add(1i32 as
                                                                   libc::c_uint)
                                                  as isize)).price <=
                                     (*opt.offset(cur as isize)).price +
                                         (1i32 << 8i32) / 2i32) {
                            let ll0_0: U32 =
                                ((*opt.offset(cur as isize)).mlen !=
                                     0i32 as libc::c_uint) as libc::c_int as
                                    U32;
                            let litlen_1: U32 =
                                if (*opt.offset(cur as isize)).mlen ==
                                       0i32 as libc::c_uint {
                                    (*opt.offset(cur as isize)).litlen
                                } else { 0i32 as libc::c_uint };
                            let previousPrice: U32 =
                                (*opt.offset(cur as isize)).price as U32;
                            let basePrice: U32 =
                                previousPrice.wrapping_add(ZSTD_litLengthPrice(0i32
                                                                                   as
                                                                                   U32,
                                                                               optStatePtr,
                                                                               optLevel));
                            let nbMatches_0: U32 =
                                ZSTD_BtGetAllMatches(ms, cParams, inr, iend,
                                                     dictMode,
                                                     (*opt.offset(cur as
                                                                      isize)).rep.as_mut_ptr(),
                                                     ll0_0, matches,
                                                     minMatch);
                            let mut matchNb_0: U32 = 0;
                            if !(0 == nbMatches_0) {
                                let maxML_0: U32 =
                                    (*matches.offset(nbMatches_0.wrapping_sub(1i32
                                                                                  as
                                                                                  libc::c_uint)
                                                         as isize)).len;
                                if maxML_0 > sufficient_len ||
                                       cur.wrapping_add(maxML_0) >=
                                           (1i32 << 12i32) as libc::c_uint {
                                    lastSequence.mlen = maxML_0;
                                    lastSequence.off =
                                        (*matches.offset(nbMatches_0.wrapping_sub(1i32
                                                                                      as
                                                                                      libc::c_uint)
                                                             as isize)).off;
                                    lastSequence.litlen = litlen_1;
                                    cur =
                                        (cur as
                                             libc::c_uint).wrapping_sub(if (*opt.offset(cur
                                                                                            as
                                                                                            isize)).mlen
                                                                               ==
                                                                               0i32
                                                                                   as
                                                                                   libc::c_uint
                                                                           {
                                                                            (*opt.offset(cur
                                                                                             as
                                                                                             isize)).litlen
                                                                        } else {
                                                                            0i32
                                                                                as
                                                                                libc::c_uint
                                                                        }) as
                                            U32 as U32;
                                    last_pos =
                                        cur.wrapping_add(ZSTD_totalLen(lastSequence));
                                    if !(cur >
                                             (1i32 << 12i32) as libc::c_uint)
                                       {
                                        current_block = 10380409671385728102;
                                        break ;
                                    }
                                    cur = 0i32 as U32;
                                    current_block = 10380409671385728102;
                                    break ;
                                } else {
                                    matchNb_0 = 0i32 as U32;
                                    while matchNb_0 < nbMatches_0 {
                                        let offset_0: U32 =
                                            (*matches.offset(matchNb_0 as
                                                                 isize)).off;
                                        let repHistory_0: repcodes_t =
                                            ZSTD_updateRep((*opt.offset(cur as
                                                                            isize)).rep.as_mut_ptr()
                                                               as *const U32,
                                                           offset_0, ll0_0);
                                        let lastML: U32 =
                                            (*matches.offset(matchNb_0 as
                                                                 isize)).len;
                                        let startML: U32 =
                                            if matchNb_0 >
                                                   0i32 as libc::c_uint {
                                                (*matches.offset(matchNb_0.wrapping_sub(1i32
                                                                                            as
                                                                                            libc::c_uint)
                                                                     as
                                                                     isize)).len.wrapping_add(1i32
                                                                                                  as
                                                                                                  libc::c_uint)
                                            } else { minMatch };
                                        let mut mlen: U32 = 0;
                                        mlen = lastML;
                                        while mlen >= startML {
                                            let pos_0: U32 =
                                                cur.wrapping_add(mlen);
                                            let price_0: libc::c_int =
                                                basePrice.wrapping_add(ZSTD_getMatchPrice(offset_0,
                                                                                          mlen,
                                                                                          optStatePtr,
                                                                                          optLevel))
                                                    as libc::c_int;
                                            if pos_0 > last_pos ||
                                                   price_0 <
                                                       (*opt.offset(pos_0 as
                                                                        isize)).price
                                               {
                                                while last_pos < pos_0 {
                                                    (*opt.offset(last_pos.wrapping_add(1i32
                                                                                           as
                                                                                           libc::c_uint)
                                                                     as
                                                                     isize)).price
                                                        = 1i32 << 30i32;
                                                    last_pos =
                                                        last_pos.wrapping_add(1)
                                                }
                                                (*opt.offset(pos_0 as
                                                                 isize)).mlen
                                                    = mlen;
                                                (*opt.offset(pos_0 as
                                                                 isize)).off =
                                                    offset_0;
                                                (*opt.offset(pos_0 as
                                                                 isize)).litlen
                                                    = litlen_1;
                                                (*opt.offset(pos_0 as
                                                                 isize)).price
                                                    = price_0;
                                                memcpy((*opt.offset(pos_0 as
                                                                        isize)).rep.as_mut_ptr()
                                                           as
                                                           *mut libc::c_void,
                                                       &repHistory_0 as
                                                           *const repcodes_t
                                                           as
                                                           *const libc::c_void,
                                                       ::std::mem::size_of::<repcodes_t>()
                                                           as libc::c_ulong);
                                            } else if optLevel == 0i32 {
                                                break ;
                                            }
                                            mlen = mlen.wrapping_sub(1)
                                        }
                                        matchNb_0 = matchNb_0.wrapping_add(1)
                                    }
                                }
                            }
                        }
                    }
                    cur = cur.wrapping_add(1)
                }
                match current_block {
                    10380409671385728102 => { }
                    _ => {
                        lastSequence = *opt.offset(last_pos as isize);
                        cur =
                            if last_pos > ZSTD_totalLen(lastSequence) {
                                last_pos.wrapping_sub(ZSTD_totalLen(lastSequence))
                            } else { 0i32 as libc::c_uint }
                    }
                }
            }
            let storeEnd: U32 = cur.wrapping_add(1i32 as libc::c_uint);
            let mut storeStart: U32 = storeEnd;
            let mut seqPos: U32 = cur;
            *opt.offset(storeEnd as isize) = lastSequence;
            while seqPos > 0i32 as libc::c_uint {
                let backDist: U32 =
                    ZSTD_totalLen(*opt.offset(seqPos as isize));
                storeStart = storeStart.wrapping_sub(1);
                *opt.offset(storeStart as isize) =
                    *opt.offset(seqPos as isize);
                seqPos =
                    if seqPos > backDist {
                        seqPos.wrapping_sub(backDist)
                    } else { 0i32 as libc::c_uint }
            }
            let mut storePos: U32 = 0;
            storePos = storeStart;
            while storePos <= storeEnd {
                let llen: U32 = (*opt.offset(storePos as isize)).litlen;
                let mlen_0: U32 = (*opt.offset(storePos as isize)).mlen;
                let offCode: U32 = (*opt.offset(storePos as isize)).off;
                let advance: U32 = llen.wrapping_add(mlen_0);
                if mlen_0 == 0i32 as libc::c_uint {
                    ip = anchor.offset(llen as isize)
                } else {
                    if offCode >= 3i32 as libc::c_uint {
                        *rep.offset(2isize) = *rep.offset(1isize);
                        *rep.offset(1isize) = *rep.offset(0isize);
                        *rep.offset(0isize) =
                            offCode.wrapping_sub((3i32 - 1i32) as
                                                     libc::c_uint)
                    } else {
                        let repCode: U32 =
                            offCode.wrapping_add((llen ==
                                                      0i32 as libc::c_uint) as
                                                     libc::c_int as
                                                     libc::c_uint);
                        if 0 != repCode {
                            let currentOffset: U32 =
                                if repCode == 3i32 as libc::c_uint {
                                    (*rep.offset(0isize)).wrapping_sub(1i32 as
                                                                           libc::c_uint)
                                } else { *rep.offset(repCode as isize) };
                            if repCode >= 2i32 as libc::c_uint {
                                *rep.offset(2isize) = *rep.offset(1isize)
                            }
                            *rep.offset(1isize) = *rep.offset(0isize);
                            *rep.offset(0isize) = currentOffset
                        }
                    }
                    ZSTD_updateStats(optStatePtr, llen, anchor, offCode,
                                     mlen_0);
                    ZSTD_storeSeq(seqStore, llen as size_t,
                                  anchor as *const libc::c_void, offCode,
                                  mlen_0.wrapping_sub(3i32 as libc::c_uint) as
                                      size_t);
                    anchor = anchor.offset(advance as isize);
                    ip = anchor
                }
                storePos = storePos.wrapping_add(1)
            }
            ZSTD_setBasePrices(optStatePtr, optLevel);
        }
    }
    return anchor.offset_to(iend).expect("bad offset_to") as libc::c_long as
               size_t;
}
unsafe extern "C" fn ZSTD_setBasePrices(mut optPtr: *mut optState_t,
                                        mut optLevel: libc::c_int) -> () {
    (*optPtr).litSumBasePrice =
        if 0 != optLevel {
            ZSTD_fracWeight((*optPtr).litSum)
        } else { ZSTD_bitWeight((*optPtr).litSum) };
    (*optPtr).litLengthSumBasePrice =
        if 0 != optLevel {
            ZSTD_fracWeight((*optPtr).litLengthSum)
        } else { ZSTD_bitWeight((*optPtr).litLengthSum) };
    (*optPtr).matchLengthSumBasePrice =
        if 0 != optLevel {
            ZSTD_fracWeight((*optPtr).matchLengthSum)
        } else { ZSTD_bitWeight((*optPtr).matchLengthSum) };
    (*optPtr).offCodeSumBasePrice =
        if 0 != optLevel {
            ZSTD_fracWeight((*optPtr).offCodeSum)
        } else { ZSTD_bitWeight((*optPtr).offCodeSum) };
}
unsafe extern "C" fn ZSTD_bitWeight(mut stat: U32) -> U32 {
    return ZSTD_highbit32(stat.wrapping_add(1i32 as
                                                libc::c_uint)).wrapping_mul((1i32
                                                                                 <<
                                                                                 8i32)
                                                                                as
                                                                                libc::c_uint);
}
unsafe extern "C" fn ZSTD_fracWeight(mut rawStat: U32) -> U32 {
    let stat: U32 = rawStat.wrapping_add(1i32 as libc::c_uint);
    let hb: U32 = ZSTD_highbit32(stat);
    let BWeight: U32 = hb.wrapping_mul((1i32 << 8i32) as libc::c_uint);
    let FWeight: U32 = stat << 8i32 >> hb;
    let weight: U32 = BWeight.wrapping_add(FWeight);
    return weight;
}
unsafe extern "C" fn ZSTD_updateStats(optPtr: *mut optState_t,
                                      mut litLength: U32,
                                      mut literals: *const BYTE,
                                      mut offsetCode: U32,
                                      mut matchLength: U32) -> () {
    let mut u: U32 = 0;
    u = 0i32 as U32;
    while u < litLength {
        let ref mut fresh1 =
            *(*optPtr).litFreq.offset(*literals.offset(u as isize) as isize);
        *fresh1 =
            (*fresh1 as libc::c_uint).wrapping_add(2i32 as libc::c_uint) as
                U32 as U32;
        u = u.wrapping_add(1)
    }
    (*optPtr).litSum =
        ((*optPtr).litSum as
             libc::c_uint).wrapping_add(litLength.wrapping_mul(2i32 as
                                                                   libc::c_uint))
            as U32 as U32;
    let llCode: U32 = ZSTD_LLcode(litLength);
    let ref mut fresh2 = *(*optPtr).litLengthFreq.offset(llCode as isize);
    *fresh2 = (*fresh2).wrapping_add(1);
    (*optPtr).litLengthSum = (*optPtr).litLengthSum.wrapping_add(1);
    let offCode: U32 =
        ZSTD_highbit32(offsetCode.wrapping_add(1i32 as libc::c_uint));
    let ref mut fresh3 = *(*optPtr).offCodeFreq.offset(offCode as isize);
    *fresh3 = (*fresh3).wrapping_add(1);
    (*optPtr).offCodeSum = (*optPtr).offCodeSum.wrapping_add(1);
    let mlBase: U32 = matchLength.wrapping_sub(3i32 as libc::c_uint);
    let mlCode: U32 = ZSTD_MLcode(mlBase);
    let ref mut fresh4 = *(*optPtr).matchLengthFreq.offset(mlCode as isize);
    *fresh4 = (*fresh4).wrapping_add(1);
    (*optPtr).matchLengthSum = (*optPtr).matchLengthSum.wrapping_add(1);
}
unsafe extern "C" fn ZSTD_totalLen(mut sol: ZSTD_optimal_t) -> U32 {
    return sol.litlen.wrapping_add(sol.mlen);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_updateRep(mut rep: *const U32, offset: U32,
                                        ll0: U32) -> repcodes_t {
    let mut currentOffset: U32 = 0;
    let mut newReps: repcodes_t = repcodes_s{rep: [0; 3],};
    if offset >= 3i32 as libc::c_uint {
        newReps.rep[2usize] = *rep.offset(1isize);
        newReps.rep[1usize] = *rep.offset(0isize);
        newReps.rep[0usize] =
            offset.wrapping_sub((3i32 - 1i32) as libc::c_uint)
    } else {
        let repCode: U32 = offset.wrapping_add(ll0);
        if repCode > 0i32 as libc::c_uint {
            currentOffset =
                if repCode == 3i32 as libc::c_uint {
                    (*rep.offset(0isize)).wrapping_sub(1i32 as libc::c_uint)
                } else { *rep.offset(repCode as isize) };
            newReps.rep[2usize] =
                if repCode >= 2i32 as libc::c_uint {
                    *rep.offset(1isize)
                } else { *rep.offset(2isize) };
            newReps.rep[1usize] = *rep.offset(0isize);
            newReps.rep[0usize] = currentOffset
        } else {
            memcpy(&mut newReps as *mut repcodes_t as *mut libc::c_void,
                   rep as *const libc::c_void,
                   ::std::mem::size_of::<repcodes_t>() as libc::c_ulong);
        }
    }
    return newReps;
}
unsafe extern "C" fn ZSTD_getMatchPrice(offset: U32, matchLength: U32,
                                        optPtr: *const optState_t,
                                        optLevel: libc::c_int) -> U32 {
    let mut price: U32 = 0;
    let offCode: U32 =
        ZSTD_highbit32(offset.wrapping_add(1i32 as libc::c_uint));
    let mlBase: U32 = matchLength.wrapping_sub(3i32 as libc::c_uint);
    if (*optPtr).priceType as libc::c_uint ==
           zop_predef as libc::c_int as libc::c_uint {
        return if 0 != optLevel {
                   ZSTD_fracWeight(mlBase)
               } else {
                   ZSTD_bitWeight(mlBase)
               }.wrapping_add((16i32 as
                                   libc::c_uint).wrapping_add(offCode).wrapping_mul((1i32
                                                                                         <<
                                                                                         8i32)
                                                                                        as
                                                                                        libc::c_uint))
    } else {
        price =
            offCode.wrapping_mul((1i32 << 8i32) as
                                     libc::c_uint).wrapping_add((*optPtr).offCodeSumBasePrice.wrapping_sub(if 0
                                                                                                                  !=
                                                                                                                  optLevel
                                                                                                              {
                                                                                                               ZSTD_fracWeight(*(*optPtr).offCodeFreq.offset(offCode
                                                                                                                                                                 as
                                                                                                                                                                 isize))
                                                                                                           } else {
                                                                                                               ZSTD_bitWeight(*(*optPtr).offCodeFreq.offset(offCode
                                                                                                                                                                as
                                                                                                                                                                isize))
                                                                                                           }));
        if optLevel < 2i32 && offCode >= 20i32 as libc::c_uint {
            price =
                (price as
                     libc::c_uint).wrapping_add(offCode.wrapping_sub(19i32 as
                                                                         libc::c_uint).wrapping_mul(2i32
                                                                                                        as
                                                                                                        libc::c_uint).wrapping_mul((1i32
                                                                                                                                        <<
                                                                                                                                        8i32)
                                                                                                                                       as
                                                                                                                                       libc::c_uint))
                    as U32 as U32
        }
        let mlCode: U32 = ZSTD_MLcode(mlBase);
        price =
            (price as
                 libc::c_uint).wrapping_add(ML_bits[mlCode as
                                                        usize].wrapping_mul((1i32
                                                                                 <<
                                                                                 8i32)
                                                                                as
                                                                                libc::c_uint).wrapping_add((*optPtr).matchLengthSumBasePrice.wrapping_sub(if 0
                                                                                                                                                                 !=
                                                                                                                                                                 optLevel
                                                                                                                                                             {
                                                                                                                                                              ZSTD_fracWeight(*(*optPtr).matchLengthFreq.offset(mlCode
                                                                                                                                                                                                                    as
                                                                                                                                                                                                                    isize))
                                                                                                                                                          } else {
                                                                                                                                                              ZSTD_bitWeight(*(*optPtr).matchLengthFreq.offset(mlCode
                                                                                                                                                                                                                   as
                                                                                                                                                                                                                   isize))
                                                                                                                                                          })))
                as U32 as U32;
        price =
            (price as
                 libc::c_uint).wrapping_add(((1i32 << 8i32) / 5i32) as
                                                libc::c_uint) as U32 as U32;
        return price
    };
}
unsafe extern "C" fn ZSTD_litLengthPrice(litLength: U32,
                                         optPtr: *const optState_t,
                                         mut optLevel: libc::c_int) -> U32 {
    if (*optPtr).priceType as libc::c_uint ==
           zop_predef as libc::c_int as libc::c_uint {
        return if 0 != optLevel {
                   ZSTD_fracWeight(litLength)
               } else { ZSTD_bitWeight(litLength) }
    } else {
        let llCode: U32 = ZSTD_LLcode(litLength);
        return LL_bits[llCode as
                           usize].wrapping_mul((1i32 << 8i32) as
                                                   libc::c_uint).wrapping_add((*optPtr).litLengthSumBasePrice.wrapping_sub(if 0
                                                                                                                                  !=
                                                                                                                                  optLevel
                                                                                                                              {
                                                                                                                               ZSTD_fracWeight(*(*optPtr).litLengthFreq.offset(llCode
                                                                                                                                                                                   as
                                                                                                                                                                                   isize))
                                                                                                                           } else {
                                                                                                                               ZSTD_bitWeight(*(*optPtr).litLengthFreq.offset(llCode
                                                                                                                                                                                  as
                                                                                                                                                                                  isize))
                                                                                                                           }))
    };
}
unsafe extern "C" fn ZSTD_BtGetAllMatches(mut ms: *mut ZSTD_matchState_t,
                                          mut cParams:
                                              *const ZSTD_compressionParameters,
                                          mut ip: *const BYTE,
                                          iHighLimit: *const BYTE,
                                          dictMode: ZSTD_dictMode_e,
                                          mut rep: *mut U32, ll0: U32,
                                          mut matches: *mut ZSTD_match_t,
                                          lengthToBeat: U32) -> U32 {
    let matchLengthSearch: U32 = (*cParams).searchLength;
    if ip < (*ms).window.base.offset((*ms).nextToUpdate as isize) {
        return 0i32 as U32
    } else {
        ZSTD_updateTree_internal(ms, cParams, ip, iHighLimit,
                                 matchLengthSearch, dictMode);
        match matchLengthSearch {
            3 => {
                return ZSTD_insertBtAndGetAllMatches(ms, cParams, ip,
                                                     iHighLimit, dictMode,
                                                     rep, ll0, matches,
                                                     lengthToBeat,
                                                     3i32 as U32)
            }
            5 => {
                return ZSTD_insertBtAndGetAllMatches(ms, cParams, ip,
                                                     iHighLimit, dictMode,
                                                     rep, ll0, matches,
                                                     lengthToBeat,
                                                     5i32 as U32)
            }
            7 | 6 => {
                return ZSTD_insertBtAndGetAllMatches(ms, cParams, ip,
                                                     iHighLimit, dictMode,
                                                     rep, ll0, matches,
                                                     lengthToBeat,
                                                     6i32 as U32)
            }
            4 | _ => {
                return ZSTD_insertBtAndGetAllMatches(ms, cParams, ip,
                                                     iHighLimit, dictMode,
                                                     rep, ll0, matches,
                                                     lengthToBeat,
                                                     4i32 as U32)
            }
        }
    };
}
unsafe extern "C" fn ZSTD_insertBtAndGetAllMatches(mut ms:
                                                       *mut ZSTD_matchState_t,
                                                   mut cParams:
                                                       *const ZSTD_compressionParameters,
                                                   ip: *const BYTE,
                                                   iLimit: *const BYTE,
                                                   dictMode: ZSTD_dictMode_e,
                                                   mut rep: *mut U32,
                                                   ll0: U32,
                                                   mut matches:
                                                       *mut ZSTD_match_t,
                                                   lengthToBeat: U32,
                                                   mls: U32) -> U32 {
    let mut match_0: *const BYTE = 0 as *const BYTE;
    let mut match_1: *const BYTE = 0 as *const BYTE;
    let sufficient_len: U32 =
        if (*cParams).targetLength < ((1i32 << 12i32) - 1i32) as libc::c_uint
           {
            (*cParams).targetLength
        } else { ((1i32 << 12i32) - 1i32) as libc::c_uint };
    let base: *const BYTE = (*ms).window.base;
    let current: U32 =
        base.offset_to(ip).expect("bad offset_to") as libc::c_long as U32;
    let hashLog: U32 = (*cParams).hashLog;
    let minMatch: U32 =
        (if mls == 3i32 as libc::c_uint { 3i32 } else { 4i32 }) as U32;
    let hashTable: *mut U32 = (*ms).hashTable;
    let h: size_t = ZSTD_hashPtr(ip as *const libc::c_void, hashLog, mls);
    let mut matchIndex: U32 = *hashTable.offset(h as isize);
    let bt: *mut U32 = (*ms).chainTable;
    let btLog: U32 = (*cParams).chainLog.wrapping_sub(1i32 as libc::c_uint);
    let btMask: U32 = (1u32 << btLog).wrapping_sub(1i32 as libc::c_uint);
    let mut commonLengthSmaller: size_t = 0i32 as size_t;
    let mut commonLengthLarger: size_t = 0i32 as size_t;
    let dictBase: *const BYTE = (*ms).window.dictBase;
    let dictLimit: U32 = (*ms).window.dictLimit;
    let dictEnd: *const BYTE = dictBase.offset(dictLimit as isize);
    let prefixStart: *const BYTE = base.offset(dictLimit as isize);
    let btLow: U32 =
        if btMask >= current {
            0i32 as libc::c_uint
        } else { current.wrapping_sub(btMask) };
    let windowLow: U32 = (*ms).window.lowLimit;
    let matchLow: U32 =
        if 0 != windowLow { windowLow } else { 1i32 as libc::c_uint };
    let mut smallerPtr: *mut U32 =
        bt.offset((2i32 as libc::c_uint).wrapping_mul(current & btMask) as
                      isize);
    let mut largerPtr: *mut U32 =
        bt.offset((2i32 as libc::c_uint).wrapping_mul(current & btMask) as
                      isize).offset(1isize);
    let mut matchEndIdx: U32 =
        current.wrapping_add(8i32 as
                                 libc::c_uint).wrapping_add(1i32 as
                                                                libc::c_uint);
    let mut dummy32: U32 = 0;
    let mut mnum: U32 = 0i32 as U32;
    let mut nbCompares: U32 = 1u32 << (*cParams).searchLog;
    let mut dms: *const ZSTD_matchState_t =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*ms).dictMatchState
        } else { 0 as *const ZSTD_matchState_t };
    let dmsBase: *const BYTE =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*dms).window.base
        } else { 0 as *const BYTE };
    let dmsEnd: *const BYTE =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*dms).window.nextSrc
        } else { 0 as *const BYTE };
    let dmsHighLimit: U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            dmsBase.offset_to(dmsEnd).expect("bad offset_to") as libc::c_long
                as U32
        } else { 0i32 as libc::c_uint };
    let dmsLowLimit: U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*dms).window.lowLimit
        } else { 0i32 as libc::c_uint };
    let dmsIndexDelta: U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            windowLow.wrapping_sub(dmsHighLimit)
        } else { 0i32 as libc::c_uint };
    let dmsBtLow: U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint &&
               btMask < dmsHighLimit.wrapping_sub(dmsLowLimit) {
            dmsHighLimit.wrapping_sub(btMask)
        } else { dmsLowLimit };
    let mut bestLength: size_t =
        lengthToBeat.wrapping_sub(1i32 as libc::c_uint) as size_t;
    let lastR: U32 = (3i32 as libc::c_uint).wrapping_add(ll0);
    let mut repCode: U32 = 0;
    repCode = ll0;
    while repCode < lastR {
        let repOffset: U32 =
            if repCode == 3i32 as libc::c_uint {
                (*rep.offset(0isize)).wrapping_sub(1i32 as libc::c_uint)
            } else { *rep.offset(repCode as isize) };
        let repIndex: U32 = current.wrapping_sub(repOffset);
        let mut repLen: U32 = 0i32 as U32;
        if repOffset.wrapping_sub(1i32 as libc::c_uint) <
               current.wrapping_sub(dictLimit) {
            if ZSTD_readMINMATCH(ip as *const libc::c_void, minMatch) ==
                   ZSTD_readMINMATCH(ip.offset(-(repOffset as isize)) as
                                         *const libc::c_void, minMatch) {
                repLen =
                    (ZSTD_count(ip.offset(minMatch as isize),
                                ip.offset(minMatch as
                                              isize).offset(-(repOffset as
                                                                  isize)),
                                iLimit) as U32).wrapping_add(minMatch)
            }
        } else {
            let repMatch: *const BYTE =
                if dictMode as libc::c_uint ==
                       ZSTD_dictMatchState as libc::c_int as libc::c_uint {
                    dmsBase.offset(repIndex as
                                       isize).offset(-(dmsIndexDelta as
                                                           isize))
                } else { dictBase.offset(repIndex as isize) };
            if dictMode as libc::c_uint ==
                   ZSTD_extDict as libc::c_int as libc::c_uint &&
                   0 !=
                       (repOffset.wrapping_sub(1i32 as libc::c_uint) <
                            current.wrapping_sub(windowLow)) as libc::c_int &
                           (dictLimit.wrapping_sub(1i32 as
                                                       libc::c_uint).wrapping_sub(repIndex)
                                >= 3i32 as libc::c_uint) as libc::c_int &&
                   ZSTD_readMINMATCH(ip as *const libc::c_void, minMatch) ==
                       ZSTD_readMINMATCH(repMatch as *const libc::c_void,
                                         minMatch) {
                repLen =
                    (ZSTD_count_2segments(ip.offset(minMatch as isize),
                                          repMatch.offset(minMatch as isize),
                                          iLimit, dictEnd, prefixStart) as
                         U32).wrapping_add(minMatch)
            }
            if dictMode as libc::c_uint ==
                   ZSTD_dictMatchState as libc::c_int as libc::c_uint &&
                   0 !=
                       (repOffset.wrapping_sub(1i32 as libc::c_uint) <
                            current.wrapping_sub(dmsLowLimit.wrapping_add(dmsIndexDelta)))
                           as libc::c_int &
                           (dictLimit.wrapping_sub(1i32 as
                                                       libc::c_uint).wrapping_sub(repIndex)
                                >= 3i32 as libc::c_uint) as libc::c_int &&
                   ZSTD_readMINMATCH(ip as *const libc::c_void, minMatch) ==
                       ZSTD_readMINMATCH(repMatch as *const libc::c_void,
                                         minMatch) {
                repLen =
                    (ZSTD_count_2segments(ip.offset(minMatch as isize),
                                          repMatch.offset(minMatch as isize),
                                          iLimit, dmsEnd, prefixStart) as
                         U32).wrapping_add(minMatch)
            }
        }
        if repLen as libc::c_ulong > bestLength {
            bestLength = repLen as size_t;
            (*matches.offset(mnum as isize)).off = repCode.wrapping_sub(ll0);
            (*matches.offset(mnum as isize)).len = repLen;
            mnum = mnum.wrapping_add(1);
            if 0 !=
                   (repLen > sufficient_len) as libc::c_int |
                       (ip.offset(repLen as isize) == iLimit) as libc::c_int {
                return mnum
            }
        }
        repCode = repCode.wrapping_add(1)
    }
    if mls == 3i32 as libc::c_uint && bestLength < mls as libc::c_ulong {
        let matchIndex3: U32 = ZSTD_insertAndFindFirstIndexHash3(ms, ip);
        if 0 !=
               (matchIndex3 >= matchLow) as libc::c_int &
                   (current.wrapping_sub(matchIndex3) <
                        (1i32 << 18i32) as libc::c_uint) as libc::c_int {
            let mut mlen: size_t = 0;
            if dictMode as libc::c_uint ==
                   ZSTD_noDict as libc::c_int as libc::c_uint ||
                   dictMode as libc::c_uint ==
                       ZSTD_dictMatchState as libc::c_int as libc::c_uint ||
                   matchIndex3 >= dictLimit {
                match_0 = base.offset(matchIndex3 as isize);
                mlen = ZSTD_count(ip, match_0, iLimit)
            } else {
                match_1 = dictBase.offset(matchIndex3 as isize);
                mlen =
                    ZSTD_count_2segments(ip, match_1, iLimit, dictEnd,
                                         prefixStart)
            }
            if mlen >= mls as libc::c_ulong {
                bestLength = mlen;
                (*matches.offset(0isize)).off =
                    current.wrapping_sub(matchIndex3).wrapping_add((3i32 -
                                                                        1i32)
                                                                       as
                                                                       libc::c_uint);
                (*matches.offset(0isize)).len = mlen as U32;
                mnum = 1i32 as U32;
                if 0 !=
                       (mlen > sufficient_len as libc::c_ulong) as libc::c_int
                           |
                           (ip.offset(mlen as isize) == iLimit) as libc::c_int
                   {
                    (*ms).nextToUpdate =
                        current.wrapping_add(1i32 as libc::c_uint);
                    return 1i32 as U32
                }
            }
        }
    }
    *hashTable.offset(h as isize) = current;
    loop  {
        let fresh5 = nbCompares;
        nbCompares = nbCompares.wrapping_sub(1);
        if !(0 != fresh5 && matchIndex >= matchLow) { break ; }
        let nextPtr: *mut U32 =
            bt.offset((2i32 as libc::c_uint).wrapping_mul(matchIndex & btMask)
                          as isize);
        let mut matchLength: size_t =
            if commonLengthSmaller < commonLengthLarger {
                commonLengthSmaller
            } else { commonLengthLarger };
        let mut match_2: *const BYTE = 0 as *const BYTE;
        if dictMode as libc::c_uint ==
               ZSTD_noDict as libc::c_int as libc::c_uint ||
               dictMode as libc::c_uint ==
                   ZSTD_dictMatchState as libc::c_int as libc::c_uint ||
               (matchIndex as libc::c_ulong).wrapping_add(matchLength) >=
                   dictLimit as libc::c_ulong {
            match_2 = base.offset(matchIndex as isize);
            matchLength =
                (matchLength as
                     libc::c_ulong).wrapping_add(ZSTD_count(ip.offset(matchLength
                                                                          as
                                                                          isize),
                                                            match_2.offset(matchLength
                                                                               as
                                                                               isize),
                                                            iLimit)) as size_t
                    as size_t
        } else {
            match_2 = dictBase.offset(matchIndex as isize);
            matchLength =
                (matchLength as
                     libc::c_ulong).wrapping_add(ZSTD_count_2segments(ip.offset(matchLength
                                                                                    as
                                                                                    isize),
                                                                      match_2.offset(matchLength
                                                                                         as
                                                                                         isize),
                                                                      iLimit,
                                                                      dictEnd,
                                                                      prefixStart))
                    as size_t as size_t;
            if (matchIndex as libc::c_ulong).wrapping_add(matchLength) >=
                   dictLimit as libc::c_ulong {
                match_2 = base.offset(matchIndex as isize)
            }
        }
        if matchLength > bestLength {
            if matchLength >
                   matchEndIdx.wrapping_sub(matchIndex) as libc::c_ulong {
                matchEndIdx = matchIndex.wrapping_add(matchLength as U32)
            }
            bestLength = matchLength;
            (*matches.offset(mnum as isize)).off =
                current.wrapping_sub(matchIndex).wrapping_add((3i32 - 1i32) as
                                                                  libc::c_uint);
            (*matches.offset(mnum as isize)).len = matchLength as U32;
            mnum = mnum.wrapping_add(1);
            if 0 !=
                   (matchLength > (1i32 << 12i32) as libc::c_ulong) as
                       libc::c_int |
                       (ip.offset(matchLength as isize) == iLimit) as
                           libc::c_int {
                if !(dictMode as libc::c_uint ==
                         ZSTD_dictMatchState as libc::c_int as libc::c_uint) {
                    break ;
                }
                nbCompares = 0i32 as U32;
                break ;
            }
        }
        if (*match_2.offset(matchLength as isize) as libc::c_int) <
               *ip.offset(matchLength as isize) as libc::c_int {
            *smallerPtr = matchIndex;
            commonLengthSmaller = matchLength;
            if matchIndex <= btLow {
                smallerPtr = &mut dummy32 as *mut U32;
                break ;
            } else {
                smallerPtr = nextPtr.offset(1isize);
                matchIndex = *nextPtr.offset(1isize)
            }
        } else {
            *largerPtr = matchIndex;
            commonLengthLarger = matchLength;
            if matchIndex <= btLow {
                largerPtr = &mut dummy32 as *mut U32;
                break ;
            } else {
                largerPtr = nextPtr;
                matchIndex = *nextPtr.offset(0isize)
            }
        }
    }
    *largerPtr = 0i32 as U32;
    *smallerPtr = *largerPtr;
    if dictMode as libc::c_uint ==
           ZSTD_dictMatchState as libc::c_int as libc::c_uint &&
           0 != nbCompares {
        let mut dictMatchIndex: U32 = *(*dms).hashTable.offset(h as isize);
        let dmsBt: *const U32 = (*dms).chainTable;
        commonLengthLarger = 0i32 as size_t;
        commonLengthSmaller = commonLengthLarger;
        loop  {
            let fresh6 = nbCompares;
            nbCompares = nbCompares.wrapping_sub(1);
            if !(0 != fresh6 && dictMatchIndex > dmsLowLimit) { break ; }
            let nextPtr_0: *const U32 =
                dmsBt.offset((2i32 as
                                  libc::c_uint).wrapping_mul(dictMatchIndex &
                                                                 btMask) as
                                 isize);
            let mut matchLength_0: size_t =
                if commonLengthSmaller < commonLengthLarger {
                    commonLengthSmaller
                } else { commonLengthLarger };
            let mut match_3: *const BYTE =
                dmsBase.offset(dictMatchIndex as isize);
            matchLength_0 =
                (matchLength_0 as
                     libc::c_ulong).wrapping_add(ZSTD_count_2segments(ip.offset(matchLength_0
                                                                                    as
                                                                                    isize),
                                                                      match_3.offset(matchLength_0
                                                                                         as
                                                                                         isize),
                                                                      iLimit,
                                                                      dmsEnd,
                                                                      prefixStart))
                    as size_t as size_t;
            if (dictMatchIndex as libc::c_ulong).wrapping_add(matchLength_0)
                   >= dmsHighLimit as libc::c_ulong {
                match_3 =
                    base.offset(dictMatchIndex as
                                    isize).offset(dmsIndexDelta as isize)
            }
            if matchLength_0 > bestLength {
                matchIndex = dictMatchIndex.wrapping_add(dmsIndexDelta);
                if matchLength_0 >
                       matchEndIdx.wrapping_sub(matchIndex) as libc::c_ulong {
                    matchEndIdx =
                        matchIndex.wrapping_add(matchLength_0 as U32)
                }
                bestLength = matchLength_0;
                (*matches.offset(mnum as isize)).off =
                    current.wrapping_sub(matchIndex).wrapping_add((3i32 -
                                                                       1i32)
                                                                      as
                                                                      libc::c_uint);
                (*matches.offset(mnum as isize)).len = matchLength_0 as U32;
                mnum = mnum.wrapping_add(1);
                if 0 !=
                       (matchLength_0 > (1i32 << 12i32) as libc::c_ulong) as
                           libc::c_int |
                           (ip.offset(matchLength_0 as isize) == iLimit) as
                               libc::c_int {
                    break ;
                }
            }
            if dictMatchIndex <= dmsBtLow { break ; }
            if (*match_3.offset(matchLength_0 as isize) as libc::c_int) <
                   *ip.offset(matchLength_0 as isize) as libc::c_int {
                commonLengthSmaller = matchLength_0;
                dictMatchIndex = *nextPtr_0.offset(1isize)
            } else {
                commonLengthLarger = matchLength_0;
                dictMatchIndex = *nextPtr_0.offset(0isize)
            }
        }
    }
    (*ms).nextToUpdate = matchEndIdx.wrapping_sub(8i32 as libc::c_uint);
    return mnum;
}
unsafe extern "C" fn ZSTD_insertAndFindFirstIndexHash3(mut ms:
                                                           *mut ZSTD_matchState_t,
                                                       ip: *const BYTE)
 -> U32 {
    let hashTable3: *mut U32 = (*ms).hashTable3;
    let hashLog3: U32 = (*ms).hashLog3;
    let base: *const BYTE = (*ms).window.base;
    let mut idx: U32 = (*ms).nextToUpdate3;
    (*ms).nextToUpdate3 =
        base.offset_to(ip).expect("bad offset_to") as libc::c_long as U32;
    let target: U32 = (*ms).nextToUpdate3;
    let hash3: size_t = ZSTD_hash3Ptr(ip as *const libc::c_void, hashLog3);
    while idx < target {
        *hashTable3.offset(ZSTD_hash3Ptr(base.offset(idx as isize) as
                                             *const libc::c_void, hashLog3) as
                               isize) = idx;
        idx = idx.wrapping_add(1)
    }
    return *hashTable3.offset(hash3 as isize);
}
unsafe extern "C" fn ZSTD_readMINMATCH(mut memPtr: *const libc::c_void,
                                       mut length: U32) -> U32 {
    match length {
        3 => {
            if 0 != MEM_isLittleEndian() {
                return MEM_read32(memPtr) << 8i32
            } else { return MEM_read32(memPtr) >> 8i32 }
        }
        4 | _ => { return MEM_read32(memPtr) }
    };
}
unsafe extern "C" fn ZSTD_rawLiteralsCost(literals: *const BYTE,
                                          litLength: U32,
                                          optPtr: *const optState_t,
                                          mut optLevel: libc::c_int) -> U32 {
    if litLength == 0i32 as libc::c_uint {
        return 0i32 as U32
    } else if (*optPtr).priceType as libc::c_uint ==
                  zop_predef as libc::c_int as libc::c_uint {
        return litLength.wrapping_mul(6i32 as
                                          libc::c_uint).wrapping_mul((1i32 <<
                                                                          8i32)
                                                                         as
                                                                         libc::c_uint)
    } else {
        let mut price: U32 =
            litLength.wrapping_mul((*optPtr).litSumBasePrice);
        let mut u: U32 = 0;
        u = 0i32 as U32;
        while u < litLength {
            price =
                (price as
                     libc::c_uint).wrapping_sub(if 0 != optLevel {
                                                    ZSTD_fracWeight(*(*optPtr).litFreq.offset(*literals.offset(u
                                                                                                                   as
                                                                                                                   isize)
                                                                                                  as
                                                                                                  isize))
                                                } else {
                                                    ZSTD_bitWeight(*(*optPtr).litFreq.offset(*literals.offset(u
                                                                                                                  as
                                                                                                                  isize)
                                                                                                 as
                                                                                                 isize))
                                                }) as U32 as U32;
            u = u.wrapping_add(1)
        }
        return price
    };
}
unsafe extern "C" fn ZSTD_literalsContribution(literals: *const BYTE,
                                               litLength: U32,
                                               optPtr: *const optState_t,
                                               mut optLevel: libc::c_int)
 -> libc::c_int {
    let contribution: libc::c_int =
        ZSTD_rawLiteralsCost(literals, litLength, optPtr,
                             optLevel).wrapping_add(ZSTD_litLengthContribution(litLength,
                                                                               optPtr,
                                                                               optLevel)
                                                        as libc::c_uint) as
            libc::c_int;
    return contribution;
}
unsafe extern "C" fn ZSTD_litLengthContribution(litLength: U32,
                                                optPtr: *const optState_t,
                                                mut optLevel: libc::c_int)
 -> libc::c_int {
    if (*optPtr).priceType as libc::c_uint >=
           zop_predef as libc::c_int as libc::c_uint {
        return (if 0 != optLevel {
                    ZSTD_fracWeight(litLength)
                } else { ZSTD_bitWeight(litLength) }) as libc::c_int
    } else {
        let llCode: U32 = ZSTD_LLcode(litLength);
        let contribution: libc::c_int =
            LL_bits[llCode as
                        usize].wrapping_mul((1i32 << 8i32) as
                                                libc::c_uint).wrapping_add(if 0
                                                                                  !=
                                                                                  optLevel
                                                                              {
                                                                               ZSTD_fracWeight(*(*optPtr).litLengthFreq.offset(0isize))
                                                                           } else {
                                                                               ZSTD_bitWeight(*(*optPtr).litLengthFreq.offset(0isize))
                                                                           }).wrapping_sub(if 0
                                                                                                  !=
                                                                                                  optLevel
                                                                                              {
                                                                                               ZSTD_fracWeight(*(*optPtr).litLengthFreq.offset(llCode
                                                                                                                                                   as
                                                                                                                                                   isize))
                                                                                           } else {
                                                                                               ZSTD_bitWeight(*(*optPtr).litLengthFreq.offset(llCode
                                                                                                                                                  as
                                                                                                                                                  isize))
                                                                                           })
                as libc::c_int;
        return contribution
    };
}
unsafe extern "C" fn ZSTD_rescaleFreqs(optPtr: *mut optState_t,
                                       src: *const BYTE, srcSize: size_t,
                                       mut optLevel: libc::c_int) -> () {
    (*optPtr).priceType = zop_dynamic;
    if (*optPtr).litLengthSum == 0i32 as libc::c_uint {
        if srcSize <= 1024i32 as libc::c_ulong {
            (*optPtr).priceType = zop_predef
        }
        if (*(*optPtr).symbolCosts).huf.repeatMode as libc::c_uint ==
               HUF_repeat_valid as libc::c_int as libc::c_uint {
            (*optPtr).priceType = zop_dynamic;
            (*optPtr).litSum = 0i32 as U32;
            let mut lit: libc::c_uint = 0;
            lit = 0i32 as libc::c_uint;
            while lit <= ((1i32 << 8i32) - 1i32) as libc::c_uint {
                let scaleLog: U32 = 11i32 as U32;
                let bitCost: U32 =
                    HUF_getNbBits((*(*optPtr).symbolCosts).huf.CTable.as_ptr()
                                      as *const libc::c_void, lit);
                *(*optPtr).litFreq.offset(lit as isize) =
                    (if 0 != bitCost {
                         1i32 << scaleLog.wrapping_sub(bitCost)
                     } else { 1i32 }) as U32;
                (*optPtr).litSum =
                    ((*optPtr).litSum as
                         libc::c_uint).wrapping_add(*(*optPtr).litFreq.offset(lit
                                                                                  as
                                                                                  isize))
                        as U32 as U32;
                lit = lit.wrapping_add(1)
            }
            let mut ll: libc::c_uint = 0;
            let mut llstate: FSE_CState_t =
                FSE_CState_t{value: 0,
                             stateTable: 0 as *const libc::c_void,
                             symbolTT: 0 as *const libc::c_void,
                             stateLog: 0,};
            FSE_initCState(&mut llstate as *mut FSE_CState_t,
                           (*(*optPtr).symbolCosts).fse.litlengthCTable.as_ptr());
            (*optPtr).litLengthSum = 0i32 as U32;
            ll = 0i32 as libc::c_uint;
            while ll <= 35i32 as libc::c_uint {
                let scaleLog_0: U32 = 10i32 as U32;
                let bitCost_0: U32 = FSE_getMaxNbBits(llstate.symbolTT, ll);
                *(*optPtr).litLengthFreq.offset(ll as isize) =
                    (if 0 != bitCost_0 {
                         1i32 << scaleLog_0.wrapping_sub(bitCost_0)
                     } else { 1i32 }) as U32;
                (*optPtr).litLengthSum =
                    ((*optPtr).litLengthSum as
                         libc::c_uint).wrapping_add(*(*optPtr).litLengthFreq.offset(ll
                                                                                        as
                                                                                        isize))
                        as U32 as U32;
                ll = ll.wrapping_add(1)
            }
            let mut ml: libc::c_uint = 0;
            let mut mlstate: FSE_CState_t =
                FSE_CState_t{value: 0,
                             stateTable: 0 as *const libc::c_void,
                             symbolTT: 0 as *const libc::c_void,
                             stateLog: 0,};
            FSE_initCState(&mut mlstate as *mut FSE_CState_t,
                           (*(*optPtr).symbolCosts).fse.matchlengthCTable.as_ptr());
            (*optPtr).matchLengthSum = 0i32 as U32;
            ml = 0i32 as libc::c_uint;
            while ml <= 52i32 as libc::c_uint {
                let scaleLog_1: U32 = 10i32 as U32;
                let bitCost_1: U32 = FSE_getMaxNbBits(mlstate.symbolTT, ml);
                *(*optPtr).matchLengthFreq.offset(ml as isize) =
                    (if 0 != bitCost_1 {
                         1i32 << scaleLog_1.wrapping_sub(bitCost_1)
                     } else { 1i32 }) as U32;
                (*optPtr).matchLengthSum =
                    ((*optPtr).matchLengthSum as
                         libc::c_uint).wrapping_add(*(*optPtr).matchLengthFreq.offset(ml
                                                                                          as
                                                                                          isize))
                        as U32 as U32;
                ml = ml.wrapping_add(1)
            }
            let mut of: libc::c_uint = 0;
            let mut ofstate: FSE_CState_t =
                FSE_CState_t{value: 0,
                             stateTable: 0 as *const libc::c_void,
                             symbolTT: 0 as *const libc::c_void,
                             stateLog: 0,};
            FSE_initCState(&mut ofstate as *mut FSE_CState_t,
                           (*(*optPtr).symbolCosts).fse.offcodeCTable.as_ptr());
            (*optPtr).offCodeSum = 0i32 as U32;
            of = 0i32 as libc::c_uint;
            while of <= 31i32 as libc::c_uint {
                let scaleLog_2: U32 = 10i32 as U32;
                let bitCost_2: U32 = FSE_getMaxNbBits(ofstate.symbolTT, of);
                *(*optPtr).offCodeFreq.offset(of as isize) =
                    (if 0 != bitCost_2 {
                         1i32 << scaleLog_2.wrapping_sub(bitCost_2)
                     } else { 1i32 }) as U32;
                (*optPtr).offCodeSum =
                    ((*optPtr).offCodeSum as
                         libc::c_uint).wrapping_add(*(*optPtr).offCodeFreq.offset(of
                                                                                      as
                                                                                      isize))
                        as U32 as U32;
                of = of.wrapping_add(1)
            }
        } else {
            let mut lit_0: libc::c_uint =
                ((1i32 << 8i32) - 1i32) as libc::c_uint;
            HIST_count_simple((*optPtr).litFreq,
                              &mut lit_0 as *mut libc::c_uint,
                              src as *const libc::c_void, srcSize);
            (*optPtr).litSum =
                ZSTD_downscaleStat((*optPtr).litFreq,
                                   ((1i32 << 8i32) - 1i32) as U32, 1i32);
            let mut ll_0: libc::c_uint = 0;
            ll_0 = 0i32 as libc::c_uint;
            while ll_0 <= 35i32 as libc::c_uint {
                *(*optPtr).litLengthFreq.offset(ll_0 as isize) = 1i32 as U32;
                ll_0 = ll_0.wrapping_add(1)
            }
            (*optPtr).litLengthSum = (35i32 + 1i32) as U32;
            let mut ml_0: libc::c_uint = 0;
            ml_0 = 0i32 as libc::c_uint;
            while ml_0 <= 52i32 as libc::c_uint {
                *(*optPtr).matchLengthFreq.offset(ml_0 as isize) =
                    1i32 as U32;
                ml_0 = ml_0.wrapping_add(1)
            }
            (*optPtr).matchLengthSum = (52i32 + 1i32) as U32;
            let mut of_0: libc::c_uint = 0;
            of_0 = 0i32 as libc::c_uint;
            while of_0 <= 31i32 as libc::c_uint {
                *(*optPtr).offCodeFreq.offset(of_0 as isize) = 1i32 as U32;
                of_0 = of_0.wrapping_add(1)
            }
            (*optPtr).offCodeSum = (31i32 + 1i32) as U32
        }
    } else {
        (*optPtr).litSum =
            ZSTD_downscaleStat((*optPtr).litFreq,
                               ((1i32 << 8i32) - 1i32) as U32, 1i32);
        (*optPtr).litLengthSum =
            ZSTD_downscaleStat((*optPtr).litLengthFreq, 35i32 as U32, 0i32);
        (*optPtr).matchLengthSum =
            ZSTD_downscaleStat((*optPtr).matchLengthFreq, 52i32 as U32, 0i32);
        (*optPtr).offCodeSum =
            ZSTD_downscaleStat((*optPtr).offCodeFreq, 31i32 as U32, 0i32)
    }
    ZSTD_setBasePrices(optPtr, optLevel);
}
unsafe extern "C" fn ZSTD_downscaleStat(mut table: *mut U32,
                                        mut lastEltIndex: U32,
                                        mut malus: libc::c_int) -> U32 {
    let mut s: U32 = 0;
    let mut sum: U32 = 0i32 as U32;
    s = 0i32 as U32;
    while s <= lastEltIndex {
        *table.offset(s as isize) =
            (1i32 as
                 libc::c_uint).wrapping_add(*table.offset(s as isize) >>
                                                4i32 + malus);
        sum =
            (sum as libc::c_uint).wrapping_add(*table.offset(s as isize)) as
                U32 as U32;
        s = s.wrapping_add(1)
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btultra(mut ms:
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
    return ZSTD_compressBlock_opt_generic(ms, seqStore, rep, cParams, src,
                                          srcSize, 2i32, ZSTD_noDict);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btopt_dictMatchState(mut ms:
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
    return ZSTD_compressBlock_opt_generic(ms, seqStore, rep, cParams, src,
                                          srcSize, 0i32, ZSTD_dictMatchState);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btultra_dictMatchState(mut ms:
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
    return ZSTD_compressBlock_opt_generic(ms, seqStore, rep, cParams, src,
                                          srcSize, 2i32, ZSTD_dictMatchState);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btopt_extDict(mut ms:
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
    return ZSTD_compressBlock_opt_generic(ms, seqStore, rep, cParams, src,
                                          srcSize, 0i32, ZSTD_extDict);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btultra_extDict(mut ms:
                                                                *mut ZSTD_matchState_t,
                                                            mut seqStore:
                                                                *mut seqStore_t,
                                                            mut rep: *mut U32,
                                                            mut cParams:
                                                                *const ZSTD_compressionParameters,
                                                            mut src:
                                                                *const libc::c_void,
                                                            mut srcSize:
                                                                size_t)
 -> size_t {
    return ZSTD_compressBlock_opt_generic(ms, seqStore, rep, cParams, src,
                                          srcSize, 2i32, ZSTD_extDict);
}
unsafe extern "C" fn ZSTD_fCost(mut price: U32) -> libc::c_double {
    return price as libc::c_double /
               ((1i32 << 8i32) * 8i32) as libc::c_double;
}
unsafe extern "C" fn ZSTD_upscaleStat(mut table: *mut U32,
                                      mut lastEltIndex: U32,
                                      mut bonus: libc::c_int) -> U32 {
    let mut s: U32 = 0;
    let mut sum: U32 = 0i32 as U32;
    s = 0i32 as U32;
    while s <= lastEltIndex {
        *table.offset(s as isize) <<= 4i32 + bonus;
        let ref mut fresh7 = *table.offset(s as isize);
        *fresh7 = (*fresh7).wrapping_sub(1);
        sum =
            (sum as libc::c_uint).wrapping_add(*table.offset(s as isize)) as
                U32 as U32;
        s = s.wrapping_add(1)
    }
    return sum;
}
unsafe extern "C" fn ZSTD_upscaleStats(mut optPtr: *mut optState_t) -> () {
    (*optPtr).litSum =
        ZSTD_upscaleStat((*optPtr).litFreq, ((1i32 << 8i32) - 1i32) as U32,
                         0i32);
    (*optPtr).litLengthSum =
        ZSTD_upscaleStat((*optPtr).litLengthFreq, 35i32 as U32, 1i32);
    (*optPtr).matchLengthSum =
        ZSTD_upscaleStat((*optPtr).matchLengthFreq, 52i32 as U32, 1i32);
    (*optPtr).offCodeSum =
        ZSTD_upscaleStat((*optPtr).offCodeFreq, 31i32 as U32, 1i32);
}
