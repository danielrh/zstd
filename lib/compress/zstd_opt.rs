#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(libc, ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* * HUF_getNbBits() :
 *  Read nbBits from CTable symbolTable, for symbol `symbolValue` presumed <= HUF_SYMBOLVALUE_MAX
 *  Note 1 : is not inlined, as HUF_CElt definition is private
 *  Note 2 : const void* used, so that it can provide a statically allocated table as argument (which uses type U32) */
    #[no_mangle]
    fn HUF_getNbBits(symbolTable: *const libc::c_void, symbolValue: U32)
     -> U32;
    #[no_mangle]
    fn ZSTD_resetSeqStore(ssPtr: *mut seqStore_t);
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
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
/*-**************************************************************
*  Basic Types
*****************************************************************/
/* C99 */
pub type BYTE = uint8_t;
pub type U16 = uint16_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    pub u: U32,
    pub c: [BYTE; 4],
}
/* __pack instructions are safer, but compiler specific, hence potentially problematic for some compilers */
/* currently only defined for gcc and icc */
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct unalign16 {
    pub v: U16,
}
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct unalign32 {
    pub v: U32,
}
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct unalign64 {
    pub v: U64,
}
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct unalignArch {
    pub v: size_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_matchState_t {
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
    pub cParams: ZSTD_compressionParameters,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_compressionParameters {
    pub windowLog: libc::c_uint,
    pub chainLog: libc::c_uint,
    pub hashLog: libc::c_uint,
    pub searchLog: libc::c_uint,
    pub minMatch: libc::c_uint,
    pub targetLength: libc::c_uint,
    pub strategy: ZSTD_strategy,
}
/* **************************************
*  Advanced compression API
***************************************/
/* API design :
 *   Parameters are pushed one by one into an existing context,
 *   using ZSTD_CCtx_set*() functions.
 *   Pushed parameters are sticky : they are valid for next compressed frame, and any subsequent frame.
 *   "sticky" parameters are applicable to `ZSTD_compress2()` and `ZSTD_compressStream*()` !
 *   They do not apply to "simple" one-shot variants such as ZSTD_compressCCtx()
 *
 *   It's possible to reset all parameters to "default" using ZSTD_CCtx_reset().
 *
 *   This API supercedes all other "advanced" API entry points in the experimental section.
 *   In the future, we expect to remove from experimental API entry points which are redundant with this API.
 */
/* Compression strategies, listed from fastest to strongest */
pub type ZSTD_strategy = libc::c_uint;
pub const ZSTD_btultra2: ZSTD_strategy = 9;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const ZSTD_btopt: ZSTD_strategy = 7;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const ZSTD_lazy: ZSTD_strategy = 4;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub const ZSTD_fast: ZSTD_strategy = 1;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct optState_t {
    pub litFreq: *mut libc::c_uint,
    pub litLengthFreq: *mut libc::c_uint,
    pub matchLengthFreq: *mut libc::c_uint,
    pub offCodeFreq: *mut libc::c_uint,
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_entropyCTables_t {
    pub huf: ZSTD_hufCTables_t,
    pub fse: ZSTD_fseCTables_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_fseCTables_t {
    pub offcodeCTable: [FSE_CTable; 193],
    pub matchlengthCTable: [FSE_CTable; 363],
    pub litlengthCTable: [FSE_CTable; 329],
    pub offcode_repeatMode: FSE_repeat,
    pub matchlength_repeatMode: FSE_repeat,
    pub litlength_repeatMode: FSE_repeat,
}
/* *< same as FSE_decompress(), using an externally allocated `workSpace` produced with `FSE_DTABLE_SIZE_U32(maxLog)` */
pub type FSE_repeat = libc::c_uint;
/* *< Can use the previous table and it is asumed to be valid */
pub const FSE_repeat_valid: FSE_repeat = 2;
/* *< Can use the previous table but it must be checked */
pub const FSE_repeat_check: FSE_repeat = 1;
/* *< Cannot use the previous table */
pub const FSE_repeat_none: FSE_repeat = 0;
/* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
/* don't allocate that. It's only meant to be more restrictive than void* */
pub type FSE_CTable = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [U32; 256],
    pub repeatMode: HUF_repeat,
}
pub type HUF_repeat = libc::c_uint;
/* *< Can use the previous table and it is assumed to be valid */
pub const HUF_repeat_valid: HUF_repeat = 2;
/* *< Can use the previous table but it must be checked. Note : The previous table must have been constructed by HUF_compress{1, 4}X_repeat */
pub const HUF_repeat_check: HUF_repeat = 1;
/* *< Cannot use the previous table */
pub const HUF_repeat_none: HUF_repeat = 0;
pub type ZSTD_OptPrice_e = libc::c_uint;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_optimal_t {
    pub price: libc::c_int,
    pub off: U32,
    pub mlen: U32,
    pub litlen: U32,
    pub rep: [U32; 3],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_match_t {
    pub off: U32,
    pub len: U32,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_window_t {
    pub nextSrc: *const BYTE,
    pub base: *const BYTE,
    pub dictBase: *const BYTE,
    pub dictLimit: U32,
    pub lowLimit: U32,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct seqStore_t {
    pub sequencesStart: *mut seqDef,
    pub sequences: *mut seqDef,
    pub litStart: *mut BYTE,
    pub lit: *mut BYTE,
    pub llCode: *mut BYTE,
    pub mlCode: *mut BYTE,
    pub ofCode: *mut BYTE,
    pub maxNbSeq: size_t,
    pub maxNbLit: size_t,
    pub longLengthID: U32,
    pub longLengthPos: U32,
}
pub type seqDef = seqDef_s;
/*-*******************************************
*  Private declarations
*********************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct seqDef_s {
    pub offset: U32,
    pub litLength: U16,
    pub matchLength: U16,
}
/* *****************************************
*  FSE symbol compression API
*******************************************/
/*!
   This API consists of small unitary functions, which highly benefit from being inlined.
   Hence their body are included in next section.
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const libc::c_void,
    pub symbolTT: *const libc::c_void,
    pub stateLog: libc::c_uint,
}
/* faster, but works only if nbBits is always >= 1 (otherwise, result will be corrupted) */
/* *****************************************
*  Implementation of inlined functions
*******************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: libc::c_int,
    pub deltaNbBits: U32,
}
pub type ZSTD_dictMode_e = libc::c_uint;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
pub type repcodes_t = repcodes_s;
/*-*******************************
*  Optimal parser
*********************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct repcodes_s {
    pub rep: [U32; 3],
}
unsafe extern "C" fn MEM_64bits() -> libc::c_uint {
    return (::std::mem::size_of::<size_t>() as libc::c_ulong ==
                8i32 as libc::c_ulong) as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    /* don't use static : performance detrimental  */
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
/* MEM_FORCE_MEMORY_ACCESS */
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
}
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return (in_0 as libc::c_ulonglong).swap_bytes() as U64;
}
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const libc::c_void) -> U32 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read32(memPtr)
    } else { return MEM_swap32(MEM_read32(memPtr)) };
}
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const libc::c_void) -> U64 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read64(memPtr)
    } else { return MEM_swap64(MEM_read64(memPtr)) };
}
unsafe extern "C" fn FSE_initCState(mut statePtr: *mut FSE_CState_t,
                                    mut ct: *const FSE_CTable) {
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
/* FSE_getMaxNbBits() :
 * Approximate maximum cost of a symbol, in bits.
 * Fractional get rounded up (i.e : a symbol with a normalized frequency of 3 gives the same result as a frequency of 2)
 * note 1 : assume symbolValue is valid (<= maxSymbolValue)
 * note 2 : if freq[symbolValue]==0, @return a fake cost of tableLog+1 bits */
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
/* Assumption : MaxOff < MaxLL,MaxML */
static mut LL_bits: [U32; 36] =
    [0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32,
     0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32,
     0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32,
     0i32 as U32, 1i32 as U32, 1i32 as U32, 1i32 as U32, 1i32 as U32,
     2i32 as U32, 2i32 as U32, 3i32 as U32, 3i32 as U32, 4i32 as U32,
     6i32 as U32, 7i32 as U32, 8i32 as U32, 9i32 as U32, 10i32 as U32,
     11i32 as U32, 12i32 as U32, 13i32 as U32, 14i32 as U32, 15i32 as U32,
     16i32 as U32];
static mut ML_bits: [U32; 53] =
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
     14i32 as U32, 15i32 as U32, 16i32 as U32];
/*-*******************************************
*  Shared functions to include for inlining
*********************************************/
unsafe extern "C" fn ZSTD_copy8(mut dst: *mut libc::c_void,
                                mut src: *const libc::c_void) {
    memcpy(dst, src, 8i32 as libc::c_ulong);
}
/* ! ZSTD_wildcopy() :
 *  custom version of memcpy(), can overwrite up to WILDCOPY_OVERLENGTH bytes (if length==0) */
unsafe extern "C" fn ZSTD_wildcopy(mut dst: *mut libc::c_void,
                                   mut src: *const libc::c_void,
                                   mut length: ptrdiff_t) {
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
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> U32 {
    return (31i32 - val.leading_zeros() as i32) as U32;
}
unsafe extern "C" fn ZSTD_LLcode(mut litLength: U32) -> U32 {
    static mut LL_Code: [BYTE; 64] =
        [0i32 as BYTE, 1i32 as BYTE, 2i32 as BYTE, 3i32 as BYTE, 4i32 as BYTE,
         5i32 as BYTE, 6i32 as BYTE, 7i32 as BYTE, 8i32 as BYTE, 9i32 as BYTE,
         10i32 as BYTE, 11i32 as BYTE, 12i32 as BYTE, 13i32 as BYTE,
         14i32 as BYTE, 15i32 as BYTE, 16i32 as BYTE, 16i32 as BYTE,
         17i32 as BYTE, 17i32 as BYTE, 18i32 as BYTE, 18i32 as BYTE,
         19i32 as BYTE, 19i32 as BYTE, 20i32 as BYTE, 20i32 as BYTE,
         20i32 as BYTE, 20i32 as BYTE, 21i32 as BYTE, 21i32 as BYTE,
         21i32 as BYTE, 21i32 as BYTE, 22i32 as BYTE, 22i32 as BYTE,
         22i32 as BYTE, 22i32 as BYTE, 22i32 as BYTE, 22i32 as BYTE,
         22i32 as BYTE, 22i32 as BYTE, 23i32 as BYTE, 23i32 as BYTE,
         23i32 as BYTE, 23i32 as BYTE, 23i32 as BYTE, 23i32 as BYTE,
         23i32 as BYTE, 23i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE,
         24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE,
         24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE,
         24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE,
         24i32 as BYTE, 24i32 as BYTE];
    static mut LL_deltaCode: U32 = 19i32 as U32;
    return if litLength > 63i32 as libc::c_uint {
               ZSTD_highbit32(litLength).wrapping_add(LL_deltaCode)
           } else { LL_Code[litLength as usize] as libc::c_uint };
}
/* ZSTD_MLcode() :
 * note : mlBase = matchLength - MINMATCH;
 *        because it's the format it's stored in seqStore->sequences */
unsafe extern "C" fn ZSTD_MLcode(mut mlBase: U32) -> U32 {
    static mut ML_Code: [BYTE; 128] =
        [0i32 as BYTE, 1i32 as BYTE, 2i32 as BYTE, 3i32 as BYTE, 4i32 as BYTE,
         5i32 as BYTE, 6i32 as BYTE, 7i32 as BYTE, 8i32 as BYTE, 9i32 as BYTE,
         10i32 as BYTE, 11i32 as BYTE, 12i32 as BYTE, 13i32 as BYTE,
         14i32 as BYTE, 15i32 as BYTE, 16i32 as BYTE, 17i32 as BYTE,
         18i32 as BYTE, 19i32 as BYTE, 20i32 as BYTE, 21i32 as BYTE,
         22i32 as BYTE, 23i32 as BYTE, 24i32 as BYTE, 25i32 as BYTE,
         26i32 as BYTE, 27i32 as BYTE, 28i32 as BYTE, 29i32 as BYTE,
         30i32 as BYTE, 31i32 as BYTE, 32i32 as BYTE, 32i32 as BYTE,
         33i32 as BYTE, 33i32 as BYTE, 34i32 as BYTE, 34i32 as BYTE,
         35i32 as BYTE, 35i32 as BYTE, 36i32 as BYTE, 36i32 as BYTE,
         36i32 as BYTE, 36i32 as BYTE, 37i32 as BYTE, 37i32 as BYTE,
         37i32 as BYTE, 37i32 as BYTE, 38i32 as BYTE, 38i32 as BYTE,
         38i32 as BYTE, 38i32 as BYTE, 38i32 as BYTE, 38i32 as BYTE,
         38i32 as BYTE, 38i32 as BYTE, 39i32 as BYTE, 39i32 as BYTE,
         39i32 as BYTE, 39i32 as BYTE, 39i32 as BYTE, 39i32 as BYTE,
         39i32 as BYTE, 39i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
         40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
         40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
         40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
         40i32 as BYTE, 40i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
         41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
         41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
         41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
         41i32 as BYTE, 41i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE];
    static mut ML_deltaCode: U32 = 36i32 as U32;
    return if mlBase > 127i32 as libc::c_uint {
               ZSTD_highbit32(mlBase).wrapping_add(ML_deltaCode)
           } else { ML_Code[mlBase as usize] as libc::c_uint };
}
/* ! ZSTD_storeSeq() :
 *  Store a sequence (literal length, literals, offset code and match length code) into seqStore_t.
 *  `offsetCode` : distance to match + 3 (values 1-3 are repCodes).
 *  `mlBase` : matchLength - MINMATCH
*/
unsafe extern "C" fn ZSTD_storeSeq(mut seqStorePtr: *mut seqStore_t,
                                   mut litLength: size_t,
                                   mut literals: *const libc::c_void,
                                   mut offsetCode: U32, mut mlBase: size_t) {
    ZSTD_wildcopy((*seqStorePtr).lit as *mut libc::c_void, literals,
                  litLength as ptrdiff_t);
    (*seqStorePtr).lit = (*seqStorePtr).lit.offset(litLength as isize);
    if litLength > 0xffffi32 as libc::c_ulong {
        (*seqStorePtr).longLengthID = 1i32 as U32;
        (*seqStorePtr).longLengthPos =
            (*seqStorePtr).sequences.wrapping_offset_from((*seqStorePtr).sequencesStart)
                as libc::c_long as U32
    }
    (*(*seqStorePtr).sequences.offset(0isize)).litLength = litLength as U16;
    (*(*seqStorePtr).sequences.offset(0isize)).offset =
        offsetCode.wrapping_add(1i32 as libc::c_uint);
    if mlBase > 0xffffi32 as libc::c_ulong {
        (*seqStorePtr).longLengthID = 2i32 as U32;
        (*seqStorePtr).longLengthPos =
            (*seqStorePtr).sequences.wrapping_offset_from((*seqStorePtr).sequencesStart)
                as libc::c_long as U32
    }
    (*(*seqStorePtr).sequences.offset(0isize)).matchLength = mlBase as U16;
    (*seqStorePtr).sequences = (*seqStorePtr).sequences.offset(1isize);
}
/*-*************************************
*  Match length counter
***************************************/
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
        if 0 != diff { return ZSTD_NbCommonBytes(diff) as size_t }
        pIn =
            pIn.offset(::std::mem::size_of::<size_t>() as libc::c_ulong as
                           isize);
        pMatch =
            pMatch.offset(::std::mem::size_of::<size_t>() as libc::c_ulong as
                              isize);
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
                return pIn.wrapping_offset_from(pStart) as libc::c_long as
                           size_t
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
    return pIn.wrapping_offset_from(pStart) as libc::c_long as size_t;
}
/* * ZSTD_count_2segments() :
 *  can count match length with `ip` & `match` in 2 different segments.
 *  convention : on reaching mEnd, match count continue starting from iStart
 */
unsafe extern "C" fn ZSTD_count_2segments(mut ip: *const BYTE,
                                          mut match_0: *const BYTE,
                                          mut iEnd: *const BYTE,
                                          mut mEnd: *const BYTE,
                                          mut iStart: *const BYTE) -> size_t {
    let vEnd: *const BYTE =
        if ip.offset(mEnd.wrapping_offset_from(match_0) as libc::c_long as
                         isize) < iEnd {
            ip.offset(mEnd.wrapping_offset_from(match_0) as libc::c_long as
                          isize)
        } else { iEnd };
    let matchLength: size_t = ZSTD_count(ip, match_0, vEnd);
    if match_0.offset(matchLength as isize) != mEnd { return matchLength }
    return matchLength.wrapping_add(ZSTD_count(ip.offset(matchLength as
                                                             isize), iStart,
                                               iEnd));
}
/*-*************************************
 *  Hashes
 ***************************************/
static mut prime3bytes: U32 = 506832829u32;
unsafe extern "C" fn ZSTD_hash3(mut u: U32, mut h: U32) -> U32 {
    return (u << 32i32 - 24i32).wrapping_mul(prime3bytes) >>
               (32i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash3Ptr(mut ptr: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash3(MEM_readLE32(ptr), h) as size_t;
}
static mut prime4bytes: U32 = 2654435761u32;
unsafe extern "C" fn ZSTD_hash4(mut u: U32, mut h: U32) -> U32 {
    return u.wrapping_mul(prime4bytes) >>
               (32i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash4Ptr(mut ptr: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash4(MEM_read32(ptr), h) as size_t;
}
static mut prime5bytes: U64 = 889523592379u64 as U64;
unsafe extern "C" fn ZSTD_hash5(mut u: U64, mut h: U32) -> size_t {
    return (u << 64i32 - 40i32).wrapping_mul(prime5bytes) >>
               (64i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash5Ptr(mut p: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash5(MEM_readLE64(p), h);
}
static mut prime6bytes: U64 = 227718039650203u64 as U64;
unsafe extern "C" fn ZSTD_hash6(mut u: U64, mut h: U32) -> size_t {
    return (u << 64i32 - 48i32).wrapping_mul(prime6bytes) >>
               (64i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash6Ptr(mut p: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash6(MEM_readLE64(p), h);
}
static mut prime7bytes: U64 = 58295818150454627u64 as U64;
unsafe extern "C" fn ZSTD_hash7(mut u: U64, mut h: U32) -> size_t {
    return (u << 64i32 - 56i32).wrapping_mul(prime7bytes) >>
               (64i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash7Ptr(mut p: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash7(MEM_readLE64(p), h);
}
static mut prime8bytes: U64 = 0xcf1bbcdcb7a56463u64 as U64;
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
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* used in ZSTD_loadDictionaryContent() */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_updateTree(mut ms: *mut ZSTD_matchState_t,
                                         mut ip: *const BYTE,
                                         mut iend: *const BYTE) {
    ZSTD_updateTree_internal(ms, ip, iend, (*ms).cParams.minMatch,
                             ZSTD_noDict);
}
unsafe extern "C" fn ZSTD_updateTree_internal(mut ms: *mut ZSTD_matchState_t,
                                              ip: *const BYTE,
                                              iend: *const BYTE, mls: U32,
                                              dictMode: ZSTD_dictMode_e) {
    let base: *const BYTE = (*ms).window.base;
    let target: U32 = ip.wrapping_offset_from(base) as libc::c_long as U32;
    let mut idx: U32 = (*ms).nextToUpdate;
    while idx < target {
        idx =
            (idx as
                 libc::c_uint).wrapping_add(ZSTD_insertBt1(ms,
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
/*-*************************************
*  Binary Tree search
***************************************/
/** ZSTD_insertBt1() : add one or multiple positions to tree.
 *  ip : assumed <= iend-8 .
 * @return : nb of positions added */
unsafe extern "C" fn ZSTD_insertBt1(mut ms: *mut ZSTD_matchState_t,
                                    ip: *const BYTE, iend: *const BYTE,
                                    mls: U32, extDict: libc::c_int) -> U32 {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
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
    let current: U32 = ip.wrapping_offset_from(base) as libc::c_long as U32;
    let btLow: U32 =
        if btMask >= current {
            0i32 as libc::c_uint
        } else { current.wrapping_sub(btMask) };
    let mut smallerPtr: *mut U32 =
        bt.offset((2i32 as libc::c_uint).wrapping_mul(current & btMask) as
                      isize);
    let mut largerPtr: *mut U32 = smallerPtr.offset(1isize);
    /* to be nullified at the end */
    let mut dummy32: U32 = 0;
    let windowLow: U32 = (*ms).window.lowLimit;
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
        if !(0 != fresh0 && matchIndex >= windowLow) { break ; }
        let nextPtr: *mut U32 =
            bt.offset((2i32 as libc::c_uint).wrapping_mul(matchIndex & btMask)
                          as isize);
        /* guaranteed minimum nb of common bytes */
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
        if ip.offset(matchLength as isize) == iend {
            /* equal : no way to know if inf or sup */
            /* drop , to guarantee consistency ; miss a bit of compression, but other solutions can corrupt tree */
            break ;
        } else if (*match_0.offset(matchLength as isize) as libc::c_int) <
                      *ip.offset(matchLength as isize) as libc::c_int {
            /* necessarily within buffer */
            *smallerPtr = matchIndex;
            commonLengthSmaller = matchLength;
            if matchIndex <= btLow {
                smallerPtr = &mut dummy32;
                /* beyond tree size, stop searching */
                break ;
            } else {
                smallerPtr = nextPtr.offset(1isize);
                matchIndex = *nextPtr.offset(1isize)
            }
        } else {
            *largerPtr = matchIndex;
            commonLengthLarger = matchLength;
            if matchIndex <= btLow {
                largerPtr = &mut dummy32;
                /* beyond tree size, stop searching */
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
    }
    return matchEndIdx.wrapping_sub(current.wrapping_add(8i32 as
                                                             libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btopt(mut ms:
                                                      *mut ZSTD_matchState_t,
                                                  mut seqStore:
                                                      *mut seqStore_t,
                                                  mut rep: *mut U32,
                                                  mut src:
                                                      *const libc::c_void,
                                                  mut srcSize: size_t)
 -> size_t {
    return ZSTD_compressBlock_opt_generic(ms, seqStore, rep, src, srcSize,
                                          0i32, ZSTD_noDict);
}
/* debug */
unsafe extern "C" fn ZSTD_compressBlock_opt_generic(mut ms:
                                                        *mut ZSTD_matchState_t,
                                                    mut seqStore:
                                                        *mut seqStore_t,
                                                    mut rep: *mut U32,
                                                    mut src:
                                                        *const libc::c_void,
                                                    mut srcSize: size_t,
                                                    optLevel: libc::c_int,
                                                    dictMode: ZSTD_dictMode_e)
 -> size_t {
    let mut current_block: u64;
    let optStatePtr: *mut optState_t = &mut (*ms).opt;
    let istart: *const BYTE = src as *const BYTE;
    let mut ip: *const BYTE = istart;
    let mut anchor: *const BYTE = istart;
    let iend: *const BYTE = istart.offset(srcSize as isize);
    let ilimit: *const BYTE = iend.offset(-8isize);
    let base: *const BYTE = (*ms).window.base;
    let prefixStart: *const BYTE =
        base.offset((*ms).window.dictLimit as isize);
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let sufficient_len: U32 =
        if (*cParams).targetLength < ((1i32 << 12i32) - 1i32) as libc::c_uint
           {
            (*cParams).targetLength
        } else { ((1i32 << 12i32) - 1i32) as libc::c_uint };
    let minMatch: U32 =
        (if (*cParams).minMatch == 3i32 as libc::c_uint {
             3i32
         } else { 4i32 }) as U32;
    let opt: *mut ZSTD_optimal_t = (*optStatePtr).priceTable;
    let matches: *mut ZSTD_match_t = (*optStatePtr).matchTable;
    let mut lastSequence: ZSTD_optimal_t =
        ZSTD_optimal_t{price: 0, off: 0, mlen: 0, litlen: 0, rep: [0; 3],};
    (*ms).nextToUpdate3 = (*ms).nextToUpdate;
    ZSTD_rescaleFreqs(optStatePtr, src as *const BYTE, srcSize, optLevel);
    ip = ip.offset((ip == prefixStart) as libc::c_int as isize);
    /* Match Loop */
    while ip < ilimit {
        let mut cur: U32 = 0;
        let mut last_pos: U32 = 0i32 as U32;
        /* find first match */
        let litlen: U32 =
            ip.wrapping_offset_from(anchor) as libc::c_long as U32;
        let ll0: U32 = (0 == litlen) as libc::c_int as U32;
        let nbMatches: U32 =
            ZSTD_BtGetAllMatches(ms, ip, iend, dictMode, rep, ll0, matches,
                                 minMatch);
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
            /* large match -> immediate encoding */
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
                /* check further positions */
                cur = 1i32 as U32;
                loop  {
                    if !(cur <= last_pos) {
                        current_block = 13454018412769612774;
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
                    /* last match must start at a minimum distance of 8 from oend */
                    if !(inr > ilimit) {
                        if cur == last_pos {
                            current_block = 13454018412769612774;
                            break ;
                        }
                        /*static_test*/
                        if !(optLevel == 0i32 &&
                                 (*opt.offset(cur.wrapping_add(1i32 as
                                                                   libc::c_uint)
                                                  as isize)).price <=
                                     (*opt.offset(cur as isize)).price +
                                         (1i32 << 8i32) / 2i32) {
                            /* skip unpromising positions; about ~+6% speed, -0.01 ratio */
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
                                ZSTD_BtGetAllMatches(ms, inr, iend, dictMode,
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
                                    if cur > (1i32 << 12i32) as libc::c_uint {
                                        cur = 0i32 as U32
                                    }
                                    current_block = 14714495436747744489;
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
                                            /* scan downward */
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
                                                /* early update abort; gets ~+10% speed for about -0.01 ratio loss */
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
                    14714495436747744489 => { }
                    _ => {
                        lastSequence = *opt.offset(last_pos as isize);
                        cur =
                            if last_pos > ZSTD_totalLen(lastSequence) {
                                last_pos.wrapping_sub(ZSTD_totalLen(lastSequence))
                            } else { 0i32 as libc::c_uint }
                    }
                }
            }
            /* cur, last_pos, best_mlen, best_off have to be set */
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
                    /* only literals => must be last "sequence", actually starting a new stream of sequences */
                    ip = anchor.offset(llen as isize)
                } else {
                    /* will finish */
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
    return iend.wrapping_offset_from(anchor) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_setBasePrices(mut optPtr: *mut optState_t,
                                        mut optLevel: libc::c_int) {
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
/*
 * Copyright (c) 2016-present, Przemyslaw Skibinski, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* scaling factor for litFreq, so that frequencies adapt faster to new stats */
/* log factor when using previous stats to init next stats */
/* if srcSize < ZSTD_PREDEF_THRESHOLD, symbols' cost is assumed static, directly determined by pre-defined distributions */
/*-*************************************
*  Price functions for optimal parser
***************************************/
/* approximation at bit level */
/* fractional bit accuracy */
/* opt==approx, ultra==accurate */
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
/* ZSTD_updateStats() :
 * assumption : literals + litLengtn <= iend */
unsafe extern "C" fn ZSTD_updateStats(optPtr: *mut optState_t,
                                      mut litLength: U32,
                                      mut literals: *const BYTE,
                                      mut offsetCode: U32,
                                      mut matchLength: U32) {
    let mut u: U32 = 0;
    u = 0i32 as U32;
    while u < litLength {
        let ref mut fresh1 =
            *(*optPtr).litFreq.offset(*literals.offset(u as isize) as isize);
        *fresh1 = (*fresh1).wrapping_add(2i32 as libc::c_uint);
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
unsafe extern "C" fn ZSTD_updateRep(mut rep: *const U32, offset: U32,
                                    ll0: U32) -> repcodes_t {
    let mut newReps: repcodes_t = repcodes_s{rep: [0; 3],};
    if offset >= 3i32 as libc::c_uint {
        newReps.rep[2usize] = *rep.offset(1isize);
        newReps.rep[1usize] = *rep.offset(0isize);
        newReps.rep[0usize] =
            offset.wrapping_sub((3i32 - 1i32) as libc::c_uint)
    } else {
        let repCode: U32 = offset.wrapping_add(ll0);
        if repCode > 0i32 as libc::c_uint {
            let currentOffset: U32 =
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
/* ZSTD_getMatchPrice() :
 * Provides the cost of the match part (offset + matchLength) of a sequence
 * Must be combined with ZSTD_fullLiteralsCost() to get the full cost of a sequence.
 * optLevel: when <2, favors small offset for decompression speed (improved cache efficiency) */
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
    }
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
    return price;
}
/* ZSTD_litLengthPrice() :
 * cost of literalLength symbol */
unsafe extern "C" fn ZSTD_litLengthPrice(litLength: U32,
                                         optPtr: *const optState_t,
                                         mut optLevel: libc::c_int) -> U32 {
    if (*optPtr).priceType as libc::c_uint ==
           zop_predef as libc::c_int as libc::c_uint {
        return if 0 != optLevel {
                   ZSTD_fracWeight(litLength)
               } else { ZSTD_bitWeight(litLength) }
    }
    let llCode: U32 = ZSTD_LLcode(litLength);
    return LL_bits[llCode as
                       usize].wrapping_mul((1i32 << 8i32) as
                                               libc::c_uint).wrapping_add((*optPtr).litLengthSumBasePrice).wrapping_sub(if 0
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
                                                                                                                        });
}
unsafe extern "C" fn ZSTD_BtGetAllMatches(mut ms: *mut ZSTD_matchState_t,
                                          mut ip: *const BYTE,
                                          iHighLimit: *const BYTE,
                                          dictMode: ZSTD_dictMode_e,
                                          mut rep: *mut U32, ll0: U32,
                                          mut matches: *mut ZSTD_match_t,
                                          lengthToBeat: U32) -> U32 {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let matchLengthSearch: U32 = (*cParams).minMatch;
    if ip < (*ms).window.base.offset((*ms).nextToUpdate as isize) {
        return 0i32 as U32
    }
    ZSTD_updateTree_internal(ms, ip, iHighLimit, matchLengthSearch, dictMode);
    match matchLengthSearch {
        3 => {
            return ZSTD_insertBtAndGetAllMatches(ms, ip, iHighLimit, dictMode,
                                                 rep, ll0, matches,
                                                 lengthToBeat, 3i32 as U32)
        }
        5 => {
            return ZSTD_insertBtAndGetAllMatches(ms, ip, iHighLimit, dictMode,
                                                 rep, ll0, matches,
                                                 lengthToBeat, 5i32 as U32)
        }
        7 | 6 => {
            return ZSTD_insertBtAndGetAllMatches(ms, ip, iHighLimit, dictMode,
                                                 rep, ll0, matches,
                                                 lengthToBeat, 6i32 as U32)
        }
        4 | _ => {
            return ZSTD_insertBtAndGetAllMatches(ms, ip, iHighLimit, dictMode,
                                                 rep, ll0, matches,
                                                 lengthToBeat, 4i32 as U32)
        }
    };
}
unsafe extern "C" fn ZSTD_insertBtAndGetAllMatches(mut ms:
                                                       *mut ZSTD_matchState_t,
                                                   ip: *const BYTE,
                                                   iLimit: *const BYTE,
                                                   dictMode: ZSTD_dictMode_e,
                                                   mut rep: *mut U32,
                                                   ll0: U32,
                                                   mut matches:
                                                       *mut ZSTD_match_t,
                                                   lengthToBeat: U32,
                                                   mls: U32) -> U32 {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let sufficient_len: U32 =
        if (*cParams).targetLength < ((1i32 << 12i32) - 1i32) as libc::c_uint
           {
            (*cParams).targetLength
        } else { ((1i32 << 12i32) - 1i32) as libc::c_uint };
    let base: *const BYTE = (*ms).window.base;
    let current: U32 = ip.wrapping_offset_from(base) as libc::c_long as U32;
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
    /* farthest referenced position of any match => detects repetitive patterns */
    let mut matchEndIdx: U32 =
        current.wrapping_add(8i32 as
                                 libc::c_uint).wrapping_add(1i32 as
                                                                libc::c_uint);
    /* to be nullified at the end */
    let mut dummy32: U32 = 0;
    let mut mnum: U32 = 0i32 as U32;
    let mut nbCompares: U32 = 1u32 << (*cParams).searchLog;
    let mut dms: *const ZSTD_matchState_t =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*ms).dictMatchState
        } else { 0 as *const ZSTD_matchState_t };
    let dmsCParams: *const ZSTD_compressionParameters =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            &(*dms).cParams
        } else { 0 as *const ZSTD_compressionParameters };
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
            dmsEnd.wrapping_offset_from(dmsBase) as libc::c_long as U32
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
    let dmsHashLog: U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*dmsCParams).hashLog
        } else { hashLog };
    let dmsBtLog: U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*dmsCParams).chainLog.wrapping_sub(1i32 as libc::c_uint)
        } else { btLog };
    let dmsBtMask: U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (1u32 << dmsBtLog).wrapping_sub(1i32 as libc::c_uint)
        } else { 0i32 as libc::c_uint };
    let dmsBtLow: U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint &&
               dmsBtMask < dmsHighLimit.wrapping_sub(dmsLowLimit) {
            dmsHighLimit.wrapping_sub(dmsBtMask)
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
                let match_0: *const BYTE = base.offset(matchIndex3 as isize);
                mlen = ZSTD_count(ip, match_0, iLimit)
            } else {
                let match_1: *const BYTE =
                    dictBase.offset(matchIndex3 as isize);
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
        /* guaranteed minimum nb of common bytes */
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
                /* equal : no way to know if inf or sup */
                if dictMode as libc::c_uint ==
                       ZSTD_dictMatchState as libc::c_int as libc::c_uint {
                    nbCompares = 0i32 as U32
                }
                /* drop, to preserve bt consistency (miss a little bit of compression) */
                break ;
            }
        }
        if (*match_2.offset(matchLength as isize) as libc::c_int) <
               *ip.offset(matchLength as isize) as libc::c_int {
            *smallerPtr = matchIndex;
            commonLengthSmaller = matchLength;
            if matchIndex <= btLow {
                smallerPtr = &mut dummy32;
                /* beyond tree size, stop the search */
                break ;
            } else {
                smallerPtr = nextPtr.offset(1isize);
                matchIndex = *nextPtr.offset(1isize)
            }
        } else {
            *largerPtr = matchIndex;
            commonLengthLarger = matchLength;
            if matchIndex <= btLow {
                largerPtr = &mut dummy32;
                /* beyond tree size, stop the search */
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
        let dmsH: size_t =
            ZSTD_hashPtr(ip as *const libc::c_void, dmsHashLog, mls);
        let mut dictMatchIndex: U32 = *(*dms).hashTable.offset(dmsH as isize);
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
                                                                 dmsBtMask) as
                                 isize);
            /* guaranteed minimum nb of common bytes */
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
                    /* equal : no way to know if inf or sup */
                    /* drop, to guarantee consistency (miss a little bit of compression) */
                    break ;
                }
            }
            if dictMatchIndex <= dmsBtLow {
                /* beyond tree size, stop the search */
                break ;
            } else if (*match_3.offset(matchLength_0 as isize) as libc::c_int)
                          < *ip.offset(matchLength_0 as isize) as libc::c_int
             {
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
/* Update hashTable3 up to ip (excluded)
   Assumption : always within prefix (i.e. not within extDict) */
unsafe extern "C" fn ZSTD_insertAndFindFirstIndexHash3(mut ms:
                                                           *mut ZSTD_matchState_t,
                                                       ip: *const BYTE)
 -> U32 {
    let hashTable3: *mut U32 = (*ms).hashTable3;
    let hashLog3: U32 = (*ms).hashLog3;
    let base: *const BYTE = (*ms).window.base;
    let mut idx: U32 = (*ms).nextToUpdate3;
    (*ms).nextToUpdate3 =
        ip.wrapping_offset_from(base) as libc::c_long as U32;
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
/* ZSTD_readMINMATCH() :
 * function safe only for comparisons
 * assumption : memPtr must be at least 4 bytes before end of buffer */
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
/* ZSTD_rawLiteralsCost() :
 * price of literals (only) in specified segment (which length can be 0).
 * does not include price of literalLength symbol */
unsafe extern "C" fn ZSTD_rawLiteralsCost(literals: *const BYTE,
                                          litLength: U32,
                                          optPtr: *const optState_t,
                                          mut optLevel: libc::c_int) -> U32 {
    if litLength == 0i32 as libc::c_uint { return 0i32 as U32 }
    if (*optPtr).priceType as libc::c_uint ==
           zop_predef as libc::c_int as libc::c_uint {
        return litLength.wrapping_mul(6i32 as
                                          libc::c_uint).wrapping_mul((1i32 <<
                                                                          8i32)
                                                                         as
                                                                         libc::c_uint)
    }
    let mut price: U32 = litLength.wrapping_mul((*optPtr).litSumBasePrice);
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
    return price;
}
/* ZSTD_literalsContribution() :
 * creates a fake cost for the literals part of a sequence
 * which can be compared to the ending cost of a match
 * should a new match start at this position */
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
/* ZSTD_litLengthContribution() :
 * @return ( cost(litlength) - cost(0) )
 * this value can then be added to rawLiteralsCost()
 * to provide a cost which is directly comparable to a match ending at same position */
unsafe extern "C" fn ZSTD_litLengthContribution(litLength: U32,
                                                optPtr: *const optState_t,
                                                mut optLevel: libc::c_int)
 -> libc::c_int {
    if (*optPtr).priceType as libc::c_uint >=
           zop_predef as libc::c_int as libc::c_uint {
        return (if 0 != optLevel {
                    ZSTD_fracWeight(litLength)
                } else { ZSTD_bitWeight(litLength) }) as libc::c_int
    }
    let llCode: U32 = ZSTD_LLcode(litLength);
    let contribution: libc::c_int =
        LL_bits[llCode as
                    usize].wrapping_mul((1i32 << 8i32) as
                                            libc::c_uint).wrapping_add(if 0 !=
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
    return contribution;
}
/* ZSTD_rescaleFreqs() :
 * if first block (detected by optPtr->litLengthSum == 0) : init statistics
 *    take hints from dictionary if there is one
 *    or init from zero, using src for literals stats, or flat 1 for match symbols
 * otherwise downscale existing stats, to be used as seed for next block.
 */
unsafe extern "C" fn ZSTD_rescaleFreqs(optPtr: *mut optState_t,
                                       src: *const BYTE, srcSize: size_t,
                                       optLevel: libc::c_int) {
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
                     } else { 1i32 }) as libc::c_uint;
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
            FSE_initCState(&mut llstate,
                           (*(*optPtr).symbolCosts).fse.litlengthCTable.as_ptr());
            (*optPtr).litLengthSum = 0i32 as U32;
            ll = 0i32 as libc::c_uint;
            while ll <= 35i32 as libc::c_uint {
                let scaleLog_0: U32 = 10i32 as U32;
                let bitCost_0: U32 = FSE_getMaxNbBits(llstate.symbolTT, ll);
                *(*optPtr).litLengthFreq.offset(ll as isize) =
                    (if 0 != bitCost_0 {
                         1i32 << scaleLog_0.wrapping_sub(bitCost_0)
                     } else { 1i32 }) as libc::c_uint;
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
            FSE_initCState(&mut mlstate,
                           (*(*optPtr).symbolCosts).fse.matchlengthCTable.as_ptr());
            (*optPtr).matchLengthSum = 0i32 as U32;
            ml = 0i32 as libc::c_uint;
            while ml <= 52i32 as libc::c_uint {
                let scaleLog_1: U32 = 10i32 as U32;
                let bitCost_1: U32 = FSE_getMaxNbBits(mlstate.symbolTT, ml);
                *(*optPtr).matchLengthFreq.offset(ml as isize) =
                    (if 0 != bitCost_1 {
                         1i32 << scaleLog_1.wrapping_sub(bitCost_1)
                     } else { 1i32 }) as libc::c_uint;
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
            FSE_initCState(&mut ofstate,
                           (*(*optPtr).symbolCosts).fse.offcodeCTable.as_ptr());
            (*optPtr).offCodeSum = 0i32 as U32;
            of = 0i32 as libc::c_uint;
            while of <= 31i32 as libc::c_uint {
                let scaleLog_2: U32 = 10i32 as U32;
                let bitCost_2: U32 = FSE_getMaxNbBits(ofstate.symbolTT, of);
                *(*optPtr).offCodeFreq.offset(of as isize) =
                    (if 0 != bitCost_2 {
                         1i32 << scaleLog_2.wrapping_sub(bitCost_2)
                     } else { 1i32 }) as libc::c_uint;
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
            HIST_count_simple((*optPtr).litFreq, &mut lit_0,
                              src as *const libc::c_void, srcSize);
            (*optPtr).litSum =
                ZSTD_downscaleStat((*optPtr).litFreq,
                                   ((1i32 << 8i32) - 1i32) as U32, 1i32);
            let mut ll_0: libc::c_uint = 0;
            ll_0 = 0i32 as libc::c_uint;
            while ll_0 <= 35i32 as libc::c_uint {
                *(*optPtr).litLengthFreq.offset(ll_0 as isize) =
                    1i32 as libc::c_uint;
                ll_0 = ll_0.wrapping_add(1)
            }
            (*optPtr).litLengthSum = (35i32 + 1i32) as U32;
            let mut ml_0: libc::c_uint = 0;
            ml_0 = 0i32 as libc::c_uint;
            while ml_0 <= 52i32 as libc::c_uint {
                *(*optPtr).matchLengthFreq.offset(ml_0 as isize) =
                    1i32 as libc::c_uint;
                ml_0 = ml_0.wrapping_add(1)
            }
            (*optPtr).matchLengthSum = (52i32 + 1i32) as U32;
            let mut of_0: libc::c_uint = 0;
            of_0 = 0i32 as libc::c_uint;
            while of_0 <= 31i32 as libc::c_uint {
                *(*optPtr).offCodeFreq.offset(of_0 as isize) =
                    1i32 as libc::c_uint;
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
/* ZSTD_downscaleStat() :
 * reduce all elements in table by a factor 2^(ZSTD_FREQ_DIV+malus)
 * return the resulting sum of elements */
unsafe extern "C" fn ZSTD_downscaleStat(mut table: *mut libc::c_uint,
                                        mut lastEltIndex: U32,
                                        mut malus: libc::c_int) -> U32 {
    let mut s: U32 = 0;
    let mut sum: U32 = 0i32 as U32;
    s = 0i32 as U32;
    while s < lastEltIndex.wrapping_add(1i32 as libc::c_uint) {
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
                                                    mut src:
                                                        *const libc::c_void,
                                                    mut srcSize: size_t)
 -> size_t {
    return ZSTD_compressBlock_opt_generic(ms, seqStore, rep, src, srcSize,
                                          2i32, ZSTD_noDict);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btultra2(mut ms:
                                                         *mut ZSTD_matchState_t,
                                                     mut seqStore:
                                                         *mut seqStore_t,
                                                     mut rep: *mut U32,
                                                     mut src:
                                                         *const libc::c_void,
                                                     mut srcSize: size_t)
 -> size_t {
    let current: U32 =
        (src as *const BYTE).wrapping_offset_from((*ms).window.base) as
            libc::c_long as U32;
    if (*ms).opt.litLengthSum == 0i32 as libc::c_uint &&
           (*seqStore).sequences == (*seqStore).sequencesStart &&
           (*ms).window.dictLimit == (*ms).window.lowLimit &&
           current == (*ms).window.dictLimit &&
           srcSize > 1024i32 as libc::c_ulong {
        ZSTD_initStats_ultra(ms, seqStore, rep, src, srcSize);
    }
    return ZSTD_compressBlock_opt_generic(ms, seqStore, rep, src, srcSize,
                                          2i32, ZSTD_noDict);
}
/* ZSTD_initStats_ultra():
 * make a first compression pass, just to seed stats with more accurate starting values.
 * only works on first block, with no dictionary and no ldm.
 * this function cannot error, hence its constract must be respected.
 */
unsafe extern "C" fn ZSTD_initStats_ultra(mut ms: *mut ZSTD_matchState_t,
                                          mut seqStore: *mut seqStore_t,
                                          mut rep: *mut U32,
                                          mut src: *const libc::c_void,
                                          mut srcSize: size_t) {
    /* updated rep codes will sink here */
    let mut tmpRep: [U32; 3] = [0; 3];
    memcpy(tmpRep.as_mut_ptr() as *mut libc::c_void,
           rep as *const libc::c_void,
           ::std::mem::size_of::<[U32; 3]>() as libc::c_ulong);
    ZSTD_compressBlock_opt_generic(ms, seqStore, tmpRep.as_mut_ptr(), src,
                                   srcSize, 2i32, ZSTD_noDict);
    ZSTD_resetSeqStore(seqStore);
    (*ms).window.base = (*ms).window.base.offset(-(srcSize as isize));
    (*ms).window.dictLimit =
        ((*ms).window.dictLimit as libc::c_uint).wrapping_add(srcSize as U32)
            as U32 as U32;
    (*ms).window.lowLimit = (*ms).window.dictLimit;
    (*ms).nextToUpdate = (*ms).window.dictLimit;
    (*ms).nextToUpdate3 = (*ms).window.dictLimit;
    ZSTD_upscaleStats(&mut (*ms).opt);
}
/* used in 2-pass strategy */
unsafe extern "C" fn ZSTD_upscaleStats(mut optPtr: *mut optState_t) {
    (*optPtr).litSum =
        ZSTD_upscaleStat((*optPtr).litFreq, ((1i32 << 8i32) - 1i32) as U32,
                         0i32);
    (*optPtr).litLengthSum =
        ZSTD_upscaleStat((*optPtr).litLengthFreq, 35i32 as U32, 0i32);
    (*optPtr).matchLengthSum =
        ZSTD_upscaleStat((*optPtr).matchLengthFreq, 52i32 as U32, 0i32);
    (*optPtr).offCodeSum =
        ZSTD_upscaleStat((*optPtr).offCodeFreq, 31i32 as U32, 0i32);
}
/* used in 2-pass strategy */
unsafe extern "C" fn ZSTD_upscaleStat(mut table: *mut libc::c_uint,
                                      mut lastEltIndex: U32,
                                      mut bonus: libc::c_int) -> U32 {
    let mut s: U32 = 0;
    let mut sum: U32 = 0i32 as U32;
    s = 0i32 as U32;
    while s < lastEltIndex.wrapping_add(1i32 as libc::c_uint) {
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
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btopt_dictMatchState(mut ms:
                                                                     *mut ZSTD_matchState_t,
                                                                 mut seqStore:
                                                                     *mut seqStore_t,
                                                                 mut rep:
                                                                     *mut U32,
                                                                 mut src:
                                                                     *const libc::c_void,
                                                                 mut srcSize:
                                                                     size_t)
 -> size_t {
    return ZSTD_compressBlock_opt_generic(ms, seqStore, rep, src, srcSize,
                                          0i32, ZSTD_dictMatchState);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btultra_dictMatchState(mut ms:
                                                                       *mut ZSTD_matchState_t,
                                                                   mut seqStore:
                                                                       *mut seqStore_t,
                                                                   mut rep:
                                                                       *mut U32,
                                                                   mut src:
                                                                       *const libc::c_void,
                                                                   mut srcSize:
                                                                       size_t)
 -> size_t {
    return ZSTD_compressBlock_opt_generic(ms, seqStore, rep, src, srcSize,
                                          2i32, ZSTD_dictMatchState);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btopt_extDict(mut ms:
                                                              *mut ZSTD_matchState_t,
                                                          mut seqStore:
                                                              *mut seqStore_t,
                                                          mut rep: *mut U32,
                                                          mut src:
                                                              *const libc::c_void,
                                                          mut srcSize: size_t)
 -> size_t {
    return ZSTD_compressBlock_opt_generic(ms, seqStore, rep, src, srcSize,
                                          0i32, ZSTD_extDict);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btultra_extDict(mut ms:
                                                                *mut ZSTD_matchState_t,
                                                            mut seqStore:
                                                                *mut seqStore_t,
                                                            mut rep: *mut U32,
                                                            mut src:
                                                                *const libc::c_void,
                                                            mut srcSize:
                                                                size_t)
 -> size_t {
    return ZSTD_compressBlock_opt_generic(ms, seqStore, rep, src, srcSize,
                                          2i32, ZSTD_extDict);
}