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
    #[no_mangle]
    fn ZSTD_selectBlockCompressor(strat: ZSTD_strategy,
                                  dictMode: ZSTD_dictMode_e)
     -> ZSTD_blockCompressor;
    /*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
    /* U32 */
    /* ZSTD_CCtx, size_t */
    #[no_mangle]
    fn ZSTD_fillDoubleHashTable(ms: *mut ZSTD_matchState_t,
                                end: *const libc::c_void,
                                dtlm: ZSTD_dictTableLoadMethod_e);
    /*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
    /* U32 */
    #[no_mangle]
    fn ZSTD_fillHashTable(ms: *mut ZSTD_matchState_t,
                          end: *const libc::c_void,
                          dtlm: ZSTD_dictTableLoadMethod_e);
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
pub struct unalignArch {
    pub v: size_t,
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
/*===== dependency =====*/
/* size_t */
/* =====   ZSTDERRORLIB_API : control library symbols visibility   ===== */
/*-*********************************************
 *  Error codes list
 *-*********************************************
 *  Error codes _values_ are pinned down since v1.3.1 only.
 *  Therefore, don't rely on values if you may link to any version < v1.3.1.
 *
 *  Only values < 100 are considered stable.
 *
 *  note 1 : this API shall be used with static linking only.
 *           dynamic linking is not yet officially supported.
 *  note 2 : Prefer relying on the enum than on its value whenever possible
 *           This is the only supported way to use the error list < v1.3.1
 *  note 3 : ZSTD_isError() is always correct, whatever the library version.
 **********************************************/
pub type unnamed_0 = libc::c_uint;
/* never EVER use this value directly, it can change in future versions! Use ZSTD_isError() instead */
pub const ZSTD_error_maxCode: unnamed_0 = 120;
pub const ZSTD_error_seekableIO: unnamed_0 = 102;
/* following error codes are __NOT STABLE__, they can be removed or changed in future versions */
pub const ZSTD_error_frameIndex_tooLarge: unnamed_0 = 100;
pub const ZSTD_error_dstBuffer_null: unnamed_0 = 74;
pub const ZSTD_error_srcSize_wrong: unnamed_0 = 72;
pub const ZSTD_error_dstSize_tooSmall: unnamed_0 = 70;
pub const ZSTD_error_workSpace_tooSmall: unnamed_0 = 66;
pub const ZSTD_error_memory_allocation: unnamed_0 = 64;
pub const ZSTD_error_init_missing: unnamed_0 = 62;
pub const ZSTD_error_stage_wrong: unnamed_0 = 60;
pub const ZSTD_error_maxSymbolValue_tooSmall: unnamed_0 = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: unnamed_0 = 46;
pub const ZSTD_error_tableLog_tooLarge: unnamed_0 = 44;
pub const ZSTD_error_parameter_outOfBound: unnamed_0 = 42;
pub const ZSTD_error_parameter_unsupported: unnamed_0 = 40;
pub const ZSTD_error_dictionaryCreation_failed: unnamed_0 = 34;
pub const ZSTD_error_dictionary_wrong: unnamed_0 = 32;
pub const ZSTD_error_dictionary_corrupted: unnamed_0 = 30;
pub const ZSTD_error_checksum_wrong: unnamed_0 = 22;
pub const ZSTD_error_corruption_detected: unnamed_0 = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: unnamed_0 = 16;
pub const ZSTD_error_frameParameter_unsupported: unnamed_0 = 14;
pub const ZSTD_error_version_unsupported: unnamed_0 = 12;
pub const ZSTD_error_prefix_unknown: unnamed_0 = 10;
pub const ZSTD_error_GENERIC: unnamed_0 = 1;
pub const ZSTD_error_no_error: unnamed_0 = 0;
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
pub struct rawSeqStore_t {
    pub seq: *mut rawSeq,
    pub pos: size_t,
    pub size: size_t,
    pub capacity: size_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct rawSeq {
    pub offset: U32,
    pub litLength: U32,
    pub matchLength: U32,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ldmState_t {
    pub window: ZSTD_window_t,
    pub hashTable: *mut ldmEntry_t,
    pub bucketOffsets: *mut BYTE,
    pub hashPower: U64,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ldmEntry_t {
    pub offset: U32,
    pub checksum: U32,
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ldmParams_t {
    pub enableLdm: U32,
    pub hashLog: U32,
    pub bucketSizeLog: U32,
    pub minMatchLength: U32,
    pub hashRateLog: U32,
    pub windowLog: U32,
}
pub type ZSTD_dictTableLoadMethod_e = libc::c_uint;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
pub type ZSTD_dictMode_e = libc::c_uint;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
pub type ZSTD_blockCompressor
    =
    Option<unsafe extern "C" fn(_: *mut ZSTD_matchState_t, _: *mut seqStore_t,
                                _: *mut U32, _: *const libc::c_void,
                                _: size_t) -> size_t>;
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
unsafe extern "C" fn MEM_readST(mut ptr: *const libc::c_void) -> size_t {
    return (*(ptr as *const unalignArch)).v;
}
/*-****************************************
*  Error codes handling
******************************************/
/* reported already defined on VS 2015 (Rich Geldreich) */
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as
               libc::c_int as libc::c_uint;
}
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
static mut prime8bytes: U64 = 0xcf1bbcdcb7a56463u64 as U64;
/* * ZSTD_rollingHash_append() :
 * Add the buffer to the hash value.
 */
unsafe extern "C" fn ZSTD_rollingHash_append(mut hash: U64,
                                             mut buf: *const libc::c_void,
                                             mut size: size_t) -> U64 {
    let mut istart: *const BYTE = buf as *const BYTE;
    let mut pos: size_t = 0;
    pos = 0i32 as size_t;
    while pos < size {
        hash =
            (hash as libc::c_ulong).wrapping_mul(prime8bytes) as U64 as U64;
        hash =
            (hash as
                 libc::c_ulong).wrapping_add((*istart.offset(pos as isize) as
                                                  libc::c_int + 10i32) as
                                                 libc::c_ulong) as U64 as U64;
        pos = pos.wrapping_add(1)
    }
    return hash;
}
/* * ZSTD_rollingHash_compute() :
 * Compute the rolling hash value of the buffer.
 */
unsafe extern "C" fn ZSTD_rollingHash_compute(mut buf: *const libc::c_void,
                                              mut size: size_t) -> U64 {
    return ZSTD_rollingHash_append(0i32 as U64, buf, size);
}
/* * ZSTD_rollingHash_rotate() :
 * Rotate the rolling hash by one byte.
 */
unsafe extern "C" fn ZSTD_rollingHash_rotate(mut hash: U64,
                                             mut toRemove: BYTE,
                                             mut toAdd: BYTE,
                                             mut primePower: U64) -> U64 {
    hash =
        (hash as
             libc::c_ulong).wrapping_sub(((toRemove as libc::c_int + 10i32) as
                                              libc::c_ulong).wrapping_mul(primePower))
            as U64 as U64;
    hash = (hash as libc::c_ulong).wrapping_mul(prime8bytes) as U64 as U64;
    hash =
        (hash as
             libc::c_ulong).wrapping_add((toAdd as libc::c_int + 10i32) as
                                             libc::c_ulong) as U64 as U64;
    return hash;
}
/* *
 * ZSTD_window_hasExtDict():
 * Returns non-zero if the window has a non-empty extDict.
 */
unsafe extern "C" fn ZSTD_window_hasExtDict(window: ZSTD_window_t) -> U32 {
    return (window.lowLimit < window.dictLimit) as libc::c_int as U32;
}
/* *
 * ZSTD_matchState_dictMode():
 * Inspects the provided matchState and figures out what dictMode should be
 * passed to the compressor.
 */
unsafe extern "C" fn ZSTD_matchState_dictMode(mut ms:
                                                  *const ZSTD_matchState_t)
 -> ZSTD_dictMode_e {
    return (if 0 != ZSTD_window_hasExtDict((*ms).window) {
                ZSTD_extDict as libc::c_int
            } else if !(*ms).dictMatchState.is_null() {
                ZSTD_dictMatchState as libc::c_int
            } else { ZSTD_noDict as libc::c_int }) as ZSTD_dictMode_e;
}
/* *
 * ZSTD_window_needOverflowCorrection():
 * Returns non-zero if the indices are getting too large and need overflow
 * protection.
 */
unsafe extern "C" fn ZSTD_window_needOverflowCorrection(window: ZSTD_window_t,
                                                        mut srcEnd:
                                                            *const libc::c_void)
 -> U32 {
    let current: U32 =
        (srcEnd as *const BYTE).wrapping_offset_from(window.base) as
            libc::c_long as U32;
    return (current >
                (3u32 <<
                     29i32).wrapping_add(1u32 <<
                                             if ::std::mem::size_of::<size_t>()
                                                    as libc::c_ulong ==
                                                    4i32 as libc::c_ulong {
                                                 30i32
                                             } else { 31i32 })) as libc::c_int
               as U32;
}
/* *
 * ZSTD_window_correctOverflow():
 * Reduces the indices to protect from index overflow.
 * Returns the correction made to the indices, which must be applied to every
 * stored index.
 *
 * The least significant cycleLog bits of the indices must remain the same,
 * which may be 0. Every index up to maxDist in the past must be valid.
 * NOTE: (maxDist & cycleMask) must be zero.
 */
unsafe extern "C" fn ZSTD_window_correctOverflow(mut window:
                                                     *mut ZSTD_window_t,
                                                 mut cycleLog: U32,
                                                 mut maxDist: U32,
                                                 mut src: *const libc::c_void)
 -> U32 {
    /* preemptive overflow correction:
     * 1. correction is large enough:
     *    lowLimit > (3<<29) ==> current > 3<<29 + 1<<windowLog
     *    1<<windowLog <= newCurrent < 1<<chainLog + 1<<windowLog
     *
     *    current - newCurrent
     *    > (3<<29 + 1<<windowLog) - (1<<windowLog + 1<<chainLog)
     *    > (3<<29) - (1<<chainLog)
     *    > (3<<29) - (1<<30)             (NOTE: chainLog <= 30)
     *    > 1<<29
     *
     * 2. (ip+ZSTD_CHUNKSIZE_MAX - cctx->base) doesn't overflow:
     *    After correction, current is less than (1<<chainLog + 1<<windowLog).
     *    In 64-bit mode we are safe, because we have 64-bit ptrdiff_t.
     *    In 32-bit mode we are safe, because (chainLog <= 29), so
     *    ip+ZSTD_CHUNKSIZE_MAX - cctx->base < 1<<32.
     * 3. (cctx->lowLimit + 1<<windowLog) < 1<<32:
     *    windowLog <= 31 ==> 3<<29 + 1<<windowLog < 7<<29 < 1<<32.
     */
    let cycleMask: U32 =
        (1u32 << cycleLog).wrapping_sub(1i32 as libc::c_uint);
    let current: U32 =
        (src as *const BYTE).wrapping_offset_from((*window).base) as
            libc::c_long as U32;
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
/* *
 * ZSTD_window_enforceMaxDist():
 * Updates lowLimit so that:
 *    (srcEnd - base) - lowLimit == maxDist + loadedDictEnd
 *
 * This allows a simple check that index >= lowLimit to see if index is valid.
 * This must be called before a block compression call, with srcEnd as the block
 * source end.
 *
 * If loadedDictEndPtr is not NULL, we set it to zero once we update lowLimit.
 * This is because dictionaries are allowed to be referenced as long as the last
 * byte of the dictionary is in the window, but once they are out of range,
 * they cannot be referenced. If loadedDictEndPtr is NULL, we use
 * loadedDictEnd == 0.
 *
 * In normal dict mode, the dict is between lowLimit and dictLimit. In
 * dictMatchState mode, lowLimit and dictLimit are the same, and the dictionary
 * is below them. forceWindow and dictMatchState are therefore incompatible.
 */
unsafe extern "C" fn ZSTD_window_enforceMaxDist(mut window:
                                                    *mut ZSTD_window_t,
                                                mut srcEnd:
                                                    *const libc::c_void,
                                                mut maxDist: U32,
                                                mut loadedDictEndPtr:
                                                    *mut U32,
                                                mut dictMatchStatePtr:
                                                    *mut *const ZSTD_matchState_t) {
    let blockEndIdx: U32 =
        (srcEnd as *const BYTE).wrapping_offset_from((*window).base) as
            libc::c_long as U32;
    let mut loadedDictEnd: U32 =
        if !loadedDictEndPtr.is_null() {
            *loadedDictEndPtr
        } else { 0i32 as libc::c_uint };
    if blockEndIdx > maxDist.wrapping_add(loadedDictEnd) {
        let newLowLimit: U32 = blockEndIdx.wrapping_sub(maxDist);
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
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 */
/*-*************************************
*  Long distance matching
***************************************/
/* *
 * ZSTD_ldm_generateSequences():
 *
 * Generates the sequences using the long distance match finder.
 * Generates long range matching sequences in `sequences`, which parse a prefix
 * of the source. `sequences` must be large enough to store every sequence,
 * which can be checked with `ZSTD_ldm_getMaxNbSeq()`.
 * @returns 0 or an error code.
 *
 * NOTE: The user must have called ZSTD_window_update() for all of the input
 * they have, even if they pass it to ZSTD_ldm_generateSequences() in chunks.
 * NOTE: This function returns an error if it runs out of space to store
 *       sequences.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_ldm_generateSequences(mut ldmState:
                                                        *mut ldmState_t,
                                                    mut sequences:
                                                        *mut rawSeqStore_t,
                                                    mut params:
                                                        *const ldmParams_t,
                                                    mut src:
                                                        *const libc::c_void,
                                                    mut srcSize: size_t)
 -> size_t {
    let maxDist: U32 = 1u32 << (*params).windowLog;
    let istart: *const BYTE = src as *const BYTE;
    let iend: *const BYTE = istart.offset(srcSize as isize);
    let kMaxChunkSize: size_t = (1i32 << 20i32) as size_t;
    let nbChunks: size_t =
        srcSize.wrapping_div(kMaxChunkSize).wrapping_add((srcSize.wrapping_rem(kMaxChunkSize)
                                                              !=
                                                              0i32 as
                                                                  libc::c_ulong)
                                                             as libc::c_int as
                                                             libc::c_ulong);
    let mut chunk: size_t = 0;
    let mut leftoverSize: size_t = 0i32 as size_t;
    chunk = 0i32 as size_t;
    while chunk < nbChunks && (*sequences).size < (*sequences).capacity {
        let chunkStart: *const BYTE =
            istart.offset(chunk.wrapping_mul(kMaxChunkSize) as isize);
        let remaining: size_t =
            iend.wrapping_offset_from(chunkStart) as libc::c_long as size_t;
        let chunkEnd: *const BYTE =
            if remaining < kMaxChunkSize {
                iend
            } else { chunkStart.offset(kMaxChunkSize as isize) };
        let chunkSize: size_t =
            chunkEnd.wrapping_offset_from(chunkStart) as libc::c_long as
                size_t;
        let mut newLeftoverSize: size_t = 0;
        let prevSize: size_t = (*sequences).size;
        if 0 !=
               ZSTD_window_needOverflowCorrection((*ldmState).window,
                                                  chunkEnd as
                                                      *const libc::c_void) {
            let ldmHSize: U32 = 1u32 << (*params).hashLog;
            let correction: U32 =
                ZSTD_window_correctOverflow(&mut (*ldmState).window,
                                            0i32 as U32, maxDist, src);
            ZSTD_ldm_reduceTable((*ldmState).hashTable, ldmHSize, correction);
        }
        ZSTD_window_enforceMaxDist(&mut (*ldmState).window,
                                   chunkEnd as *const libc::c_void, maxDist,
                                   0 as *mut U32,
                                   0 as *mut *const ZSTD_matchState_t);
        newLeftoverSize =
            ZSTD_ldm_generateSequences_internal(ldmState, sequences, params,
                                                chunkStart as
                                                    *const libc::c_void,
                                                chunkSize);
        if 0 != ERR_isError(newLeftoverSize) { return newLeftoverSize }
        if prevSize < (*sequences).size {
            let ref mut fresh0 =
                (*(*sequences).seq.offset(prevSize as isize)).litLength;
            *fresh0 =
                (*fresh0 as libc::c_uint).wrapping_add(leftoverSize as U32) as
                    U32 as U32;
            leftoverSize = newLeftoverSize
        } else {
            leftoverSize =
                (leftoverSize as libc::c_ulong).wrapping_add(chunkSize) as
                    size_t as size_t
        }
        chunk = chunk.wrapping_add(1)
    }
    return 0i32 as size_t;
}
unsafe extern "C" fn ZSTD_ldm_generateSequences_internal(mut ldmState:
                                                             *mut ldmState_t,
                                                         mut rawSeqStore:
                                                             *mut rawSeqStore_t,
                                                         mut params:
                                                             *const ldmParams_t,
                                                         mut src:
                                                             *const libc::c_void,
                                                         mut srcSize: size_t)
 -> size_t {
    /* LDM parameters */
    let extDict: libc::c_int =
        ZSTD_window_hasExtDict((*ldmState).window) as libc::c_int;
    let minMatchLength: U32 = (*params).minMatchLength;
    let hashPower: U64 = (*ldmState).hashPower;
    let hBits: U32 = (*params).hashLog.wrapping_sub((*params).bucketSizeLog);
    let ldmBucketSize: U32 = 1u32 << (*params).bucketSizeLog;
    let hashRateLog: U32 = (*params).hashRateLog;
    let ldmTagMask: U32 =
        (1u32 << (*params).hashRateLog).wrapping_sub(1i32 as libc::c_uint);
    /* Prefix and extDict parameters */
    let dictLimit: U32 = (*ldmState).window.dictLimit;
    let lowestIndex: U32 =
        if 0 != extDict { (*ldmState).window.lowLimit } else { dictLimit };
    let base: *const BYTE = (*ldmState).window.base;
    let dictBase: *const BYTE =
        if 0 != extDict {
            (*ldmState).window.dictBase
        } else { 0 as *const BYTE };
    let dictStart: *const BYTE =
        if 0 != extDict {
            dictBase.offset(lowestIndex as isize)
        } else { 0 as *const BYTE };
    let dictEnd: *const BYTE =
        if 0 != extDict {
            dictBase.offset(dictLimit as isize)
        } else { 0 as *const BYTE };
    let lowPrefixPtr: *const BYTE = base.offset(dictLimit as isize);
    /* Input bounds */
    let istart: *const BYTE = src as *const BYTE;
    let iend: *const BYTE = istart.offset(srcSize as isize);
    let ilimit: *const BYTE =
        iend.offset(-((if minMatchLength > 8i32 as libc::c_uint {
                           minMatchLength
                       } else { 8i32 as libc::c_uint }) as isize));
    /* Input positions */
    let mut anchor: *const BYTE = istart;
    let mut ip: *const BYTE = istart;
    /* Rolling hash */
    let mut lastHashed: *const BYTE = 0 as *const BYTE;
    let mut rollingHash: U64 = 0i32 as U64;
    while ip <= ilimit {
        let mut mLength: size_t = 0;
        let current: U32 =
            ip.wrapping_offset_from(base) as libc::c_long as U32;
        let mut forwardMatchLength: size_t = 0i32 as size_t;
        let mut backwardMatchLength: size_t = 0i32 as size_t;
        let mut bestEntry: *mut ldmEntry_t = 0 as *mut ldmEntry_t;
        if ip != istart {
            rollingHash =
                ZSTD_rollingHash_rotate(rollingHash,
                                        *lastHashed.offset(0isize),
                                        *lastHashed.offset(minMatchLength as
                                                               isize),
                                        hashPower)
        } else {
            rollingHash =
                ZSTD_rollingHash_compute(ip as *const libc::c_void,
                                         minMatchLength as size_t)
        }
        lastHashed = ip;
        /* Do not insert and do not look for a match */
        if ZSTD_ldm_getTag(rollingHash, hBits, hashRateLog) != ldmTagMask {
            ip = ip.offset(1isize)
        } else {
            let bucket: *mut ldmEntry_t =
                ZSTD_ldm_getBucket(ldmState,
                                   ZSTD_ldm_getSmallHash(rollingHash, hBits)
                                       as size_t, *params);
            let mut cur: *mut ldmEntry_t = 0 as *mut ldmEntry_t;
            let mut bestMatchLength: size_t = 0i32 as size_t;
            let checksum: U32 = ZSTD_ldm_getChecksum(rollingHash, hBits);
            let mut current_block_19: u64;
            cur = bucket;
            while cur < bucket.offset(ldmBucketSize as isize) {
                let mut curForwardMatchLength: size_t = 0;
                let mut curBackwardMatchLength: size_t = 0;
                let mut curTotalMatchLength: size_t = 0;
                if !((*cur).checksum != checksum ||
                         (*cur).offset <= lowestIndex) {
                    if 0 != extDict {
                        let curMatchBase: *const BYTE =
                            if (*cur).offset < dictLimit {
                                dictBase
                            } else { base };
                        let pMatch: *const BYTE =
                            curMatchBase.offset((*cur).offset as isize);
                        let matchEnd: *const BYTE =
                            if (*cur).offset < dictLimit {
                                dictEnd
                            } else { iend };
                        let lowMatchPtr: *const BYTE =
                            if (*cur).offset < dictLimit {
                                dictStart
                            } else { lowPrefixPtr };
                        curForwardMatchLength =
                            ZSTD_count_2segments(ip, pMatch, iend, matchEnd,
                                                 lowPrefixPtr);
                        if curForwardMatchLength <
                               minMatchLength as libc::c_ulong {
                            current_block_19 = 15925075030174552612;
                        } else {
                            curBackwardMatchLength =
                                ZSTD_ldm_countBackwardsMatch(ip, anchor,
                                                             pMatch,
                                                             lowMatchPtr);
                            curTotalMatchLength =
                                curForwardMatchLength.wrapping_add(curBackwardMatchLength);
                            current_block_19 = 15004371738079956865;
                        }
                    } else {
                        /* !extDict */
                        let pMatch_0: *const BYTE =
                            base.offset((*cur).offset as isize);
                        curForwardMatchLength =
                            ZSTD_count(ip, pMatch_0, iend);
                        if curForwardMatchLength <
                               minMatchLength as libc::c_ulong {
                            current_block_19 = 15925075030174552612;
                        } else {
                            curBackwardMatchLength =
                                ZSTD_ldm_countBackwardsMatch(ip, anchor,
                                                             pMatch_0,
                                                             lowPrefixPtr);
                            curTotalMatchLength =
                                curForwardMatchLength.wrapping_add(curBackwardMatchLength);
                            current_block_19 = 15004371738079956865;
                        }
                    }
                    match current_block_19 {
                        15925075030174552612 => { }
                        _ => {
                            if curTotalMatchLength > bestMatchLength {
                                bestMatchLength = curTotalMatchLength;
                                forwardMatchLength = curForwardMatchLength;
                                backwardMatchLength = curBackwardMatchLength;
                                bestEntry = cur
                            }
                        }
                    }
                }
                cur = cur.offset(1isize)
            }
            /* No match found -- continue searching */
            if bestEntry.is_null() {
                ZSTD_ldm_makeEntryAndInsertByTag(ldmState, rollingHash, hBits,
                                                 current, *params);
                ip = ip.offset(1isize)
            } else {
                mLength =
                    forwardMatchLength.wrapping_add(backwardMatchLength);
                ip = ip.offset(-(backwardMatchLength as isize));
                let matchIndex: U32 = (*bestEntry).offset;
                let offset: U32 = current.wrapping_sub(matchIndex);
                let seq: *mut rawSeq =
                    (*rawSeqStore).seq.offset((*rawSeqStore).size as isize);
                if (*rawSeqStore).size == (*rawSeqStore).capacity {
                    return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as
                               size_t
                }
                (*seq).litLength =
                    ip.wrapping_offset_from(anchor) as libc::c_long as U32;
                (*seq).matchLength = mLength as U32;
                (*seq).offset = offset;
                (*rawSeqStore).size = (*rawSeqStore).size.wrapping_add(1);
                ZSTD_ldm_makeEntryAndInsertByTag(ldmState, rollingHash, hBits,
                                                 lastHashed.wrapping_offset_from(base)
                                                     as libc::c_long as U32,
                                                 *params);
                if ip.offset(mLength as isize) <= ilimit {
                    rollingHash =
                        ZSTD_ldm_fillLdmHashTable(ldmState, rollingHash,
                                                  lastHashed,
                                                  ip.offset(mLength as isize),
                                                  base, hBits, *params);
                    lastHashed = ip.offset(mLength as isize).offset(-1isize)
                }
                ip = ip.offset(mLength as isize);
                anchor = ip
            }
        }
    }
    return iend.wrapping_offset_from(anchor) as libc::c_long as size_t;
}
/* * ZSTD_ldm_fillLdmHashTable() :
 *
 *  Fills hashTable from (lastHashed + 1) to iend (non-inclusive).
 *  lastHash is the rolling hash that corresponds to lastHashed.
 *
 *  Returns the rolling hash corresponding to position iend-1. */
unsafe extern "C" fn ZSTD_ldm_fillLdmHashTable(mut state: *mut ldmState_t,
                                               mut lastHash: U64,
                                               mut lastHashed: *const BYTE,
                                               mut iend: *const BYTE,
                                               mut base: *const BYTE,
                                               mut hBits: U32,
                                               ldmParams: ldmParams_t)
 -> U64 {
    let mut rollingHash: U64 = lastHash;
    let mut cur: *const BYTE = lastHashed.offset(1isize);
    while cur < iend {
        rollingHash =
            ZSTD_rollingHash_rotate(rollingHash, *cur.offset(-1i32 as isize),
                                    *cur.offset(ldmParams.minMatchLength.wrapping_sub(1i32
                                                                                          as
                                                                                          libc::c_uint)
                                                    as isize),
                                    (*state).hashPower);
        ZSTD_ldm_makeEntryAndInsertByTag(state, rollingHash, hBits,
                                         cur.wrapping_offset_from(base) as
                                             libc::c_long as U32, ldmParams);
        cur = cur.offset(1isize)
    }
    return rollingHash;
}
/* * ZSTD_ldm_makeEntryAndInsertByTag() :
 *
 *  Gets the small hash, checksum, and tag from the rollingHash.
 *
 *  If the tag matches (1 << ldmParams.hashRateLog)-1, then
 *  creates an ldmEntry from the offset, and inserts it into the hash table.
 *
 *  hBits is the length of the small hash, which is the most significant hBits
 *  of rollingHash. The checksum is the next 32 most significant bits, followed
 *  by ldmParams.hashRateLog bits that make up the tag. */
unsafe extern "C" fn ZSTD_ldm_makeEntryAndInsertByTag(mut ldmState:
                                                          *mut ldmState_t,
                                                      rollingHash: U64,
                                                      hBits: U32, offset: U32,
                                                      ldmParams:
                                                          ldmParams_t) {
    let tag: U32 = ZSTD_ldm_getTag(rollingHash, hBits, ldmParams.hashRateLog);
    let tagMask: U32 =
        ((1i32 as U32) <<
             ldmParams.hashRateLog).wrapping_sub(1i32 as libc::c_uint);
    if tag == tagMask {
        let hash: U32 = ZSTD_ldm_getSmallHash(rollingHash, hBits);
        let checksum: U32 = ZSTD_ldm_getChecksum(rollingHash, hBits);
        let mut entry: ldmEntry_t = ldmEntry_t{offset: 0, checksum: 0,};
        entry.offset = offset;
        entry.checksum = checksum;
        ZSTD_ldm_insertEntry(ldmState, hash as size_t, entry, ldmParams);
    };
}
/* * ZSTD_ldm_getSmallHash() :
 *  numBits should be <= 32
 *  If numBits==0, returns 0.
 *  @return : the most significant numBits of value. */
unsafe extern "C" fn ZSTD_ldm_getSmallHash(mut value: U64, mut numBits: U32)
 -> U32 {
    return if numBits == 0i32 as libc::c_uint {
               0i32 as libc::c_uint
           } else {
               (value >> (64i32 as libc::c_uint).wrapping_sub(numBits)) as U32
           };
}
/* * ZSTD_ldm_insertEntry() :
 *  Insert the entry with corresponding hash into the hash table */
unsafe extern "C" fn ZSTD_ldm_insertEntry(mut ldmState: *mut ldmState_t,
                                          hash: size_t, entry: ldmEntry_t,
                                          ldmParams: ldmParams_t) {
    let bucketOffsets: *mut BYTE = (*ldmState).bucketOffsets;
    *ZSTD_ldm_getBucket(ldmState, hash,
                        ldmParams).offset(*bucketOffsets.offset(hash as isize)
                                              as libc::c_int as isize) =
        entry;
    let ref mut fresh1 = *bucketOffsets.offset(hash as isize);
    *fresh1 = (*fresh1).wrapping_add(1);
    let ref mut fresh2 = *bucketOffsets.offset(hash as isize);
    *fresh2 =
        (*fresh2 as libc::c_uint &
             ((1i32 as U32) <<
                  ldmParams.bucketSizeLog).wrapping_sub(1i32 as libc::c_uint))
            as BYTE;
}
/* * ZSTD_ldm_getBucket() :
 *  Returns a pointer to the start of the bucket associated with hash. */
unsafe extern "C" fn ZSTD_ldm_getBucket(mut ldmState: *mut ldmState_t,
                                        mut hash: size_t,
                                        ldmParams: ldmParams_t)
 -> *mut ldmEntry_t {
    return (*ldmState).hashTable.offset((hash << ldmParams.bucketSizeLog) as
                                            isize);
}
/* * ZSTD_ldm_getChecksum() :
 *  numBitsToDiscard should be <= 32
 *  @return : the next most significant 32 bits after numBitsToDiscard */
unsafe extern "C" fn ZSTD_ldm_getChecksum(mut hash: U64,
                                          mut numBitsToDiscard: U32) -> U32 {
    return (hash >>
                ((64i32 - 32i32) as
                     libc::c_uint).wrapping_sub(numBitsToDiscard) &
                0xffffffffu32 as libc::c_ulong) as U32;
}
/* * ZSTD_ldm_getTag() ;
 *  Given the hash, returns the most significant numTagBits bits
 *  after (32 + hbits) bits.
 *
 *  If there are not enough bits remaining, return the last
 *  numTagBits bits. */
unsafe extern "C" fn ZSTD_ldm_getTag(mut hash: U64, mut hbits: U32,
                                     mut numTagBits: U32) -> U32 {
    if (32i32 as libc::c_uint).wrapping_sub(hbits) < numTagBits {
        return (hash &
                    ((1i32 as U32) <<
                         numTagBits).wrapping_sub(1i32 as libc::c_uint) as
                        libc::c_ulong) as U32
    } else {
        return (hash >>
                    (32i32 as
                         libc::c_uint).wrapping_sub(hbits).wrapping_sub(numTagBits)
                    &
                    ((1i32 as U32) <<
                         numTagBits).wrapping_sub(1i32 as libc::c_uint) as
                        libc::c_ulong) as U32
    };
}
/* * ZSTD_ldm_countBackwardsMatch() :
 *  Returns the number of bytes that match backwards before pIn and pMatch.
 *
 *  We count only bytes where pMatch >= pBase and pIn >= pAnchor. */
unsafe extern "C" fn ZSTD_ldm_countBackwardsMatch(mut pIn: *const BYTE,
                                                  mut pAnchor: *const BYTE,
                                                  mut pMatch: *const BYTE,
                                                  mut pBase: *const BYTE)
 -> size_t {
    let mut matchLength: size_t = 0i32 as size_t;
    while pIn > pAnchor && pMatch > pBase &&
              *pIn.offset(-1i32 as isize) as libc::c_int ==
                  *pMatch.offset(-1i32 as isize) as libc::c_int {
        pIn = pIn.offset(-1isize);
        pMatch = pMatch.offset(-1isize);
        matchLength = matchLength.wrapping_add(1)
    }
    return matchLength;
}
/* ! ZSTD_ldm_reduceTable() :
 *  reduce table indexes by `reducerValue` */
unsafe extern "C" fn ZSTD_ldm_reduceTable(table: *mut ldmEntry_t, size: U32,
                                          reducerValue: U32) {
    let mut u: U32 = 0;
    u = 0i32 as U32;
    while u < size {
        if (*table.offset(u as isize)).offset < reducerValue {
            (*table.offset(u as isize)).offset = 0i32 as U32
        } else {
            let ref mut fresh3 = (*table.offset(u as isize)).offset;
            *fresh3 =
                (*fresh3 as libc::c_uint).wrapping_sub(reducerValue) as U32 as
                    U32
        }
        u = u.wrapping_add(1)
    };
}
/* *
 * ZSTD_ldm_blockCompress():
 *
 * Compresses a block using the predefined sequences, along with a secondary
 * block compressor. The literals section of every sequence is passed to the
 * secondary block compressor, and those sequences are interspersed with the
 * predefined sequences. Returns the length of the last literals.
 * Updates `rawSeqStore.pos` to indicate how many sequences have been consumed.
 * `rawSeqStore.seq` may also be updated to split the last sequence between two
 * blocks.
 * @return The length of the last literals.
 *
 * NOTE: The source must be at most the maximum block size, but the predefined
 * sequences can be any size, and may be longer than the block. In the case that
 * they are longer than the block, the last sequences may need to be split into
 * two. We handle that case correctly, and update `rawSeqStore` appropriately.
 * NOTE: This function does not return any errors.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_ldm_blockCompress(mut rawSeqStore:
                                                    *mut rawSeqStore_t,
                                                mut ms:
                                                    *mut ZSTD_matchState_t,
                                                mut seqStore: *mut seqStore_t,
                                                mut rep: *mut U32,
                                                mut src: *const libc::c_void,
                                                mut srcSize: size_t)
 -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let minMatch: libc::c_uint = (*cParams).minMatch;
    let blockCompressor: ZSTD_blockCompressor =
        ZSTD_selectBlockCompressor((*cParams).strategy,
                                   ZSTD_matchState_dictMode(ms));
    /* Input bounds */
    let istart: *const BYTE = src as *const BYTE;
    let iend: *const BYTE = istart.offset(srcSize as isize);
    /* Input positions */
    let mut ip: *const BYTE = istart;
    while (*rawSeqStore).pos < (*rawSeqStore).size && ip < iend {
        /* maybeSplitSequence updates rawSeqStore->pos */
        let sequence: rawSeq =
            maybeSplitSequence(rawSeqStore,
                               iend.wrapping_offset_from(ip) as libc::c_long
                                   as U32, minMatch);
        let mut i: libc::c_int = 0;
        /* End signal */
        if sequence.offset == 0i32 as libc::c_uint { break ; }
        ZSTD_ldm_limitTableUpdate(ms, ip);
        ZSTD_ldm_fillFastTables(ms, ip as *const libc::c_void);
        let newLitLength: size_t =
            blockCompressor.expect("non-null function pointer")(ms, seqStore,
                                                                rep,
                                                                ip as
                                                                    *const libc::c_void,
                                                                sequence.litLength
                                                                    as
                                                                    size_t);
        ip = ip.offset(sequence.litLength as isize);
        i = 3i32 - 1i32;
        while i > 0i32 {
            *rep.offset(i as isize) = *rep.offset((i - 1i32) as isize);
            i -= 1
        }
        *rep.offset(0isize) = sequence.offset;
        ZSTD_storeSeq(seqStore, newLitLength,
                      ip.offset(-(newLitLength as isize)) as
                          *const libc::c_void,
                      sequence.offset.wrapping_add((3i32 - 1i32) as
                                                       libc::c_uint),
                      sequence.matchLength.wrapping_sub(3i32 as libc::c_uint)
                          as size_t);
        ip = ip.offset(sequence.matchLength as isize)
    }
    ZSTD_ldm_limitTableUpdate(ms, ip);
    ZSTD_ldm_fillFastTables(ms, ip as *const libc::c_void);
    return blockCompressor.expect("non-null function pointer")(ms, seqStore,
                                                               rep,
                                                               ip as
                                                                   *const libc::c_void,
                                                               iend.wrapping_offset_from(ip)
                                                                   as
                                                                   libc::c_long
                                                                   as size_t);
}
/* * ZSTD_ldm_fillFastTables() :
 *
 *  Fills the relevant tables for the ZSTD_fast and ZSTD_dfast strategies.
 *  This is similar to ZSTD_loadDictionaryContent.
 *
 *  The tables for the other strategies are filled within their
 *  block compressors. */
unsafe extern "C" fn ZSTD_ldm_fillFastTables(mut ms: *mut ZSTD_matchState_t,
                                             mut end: *const libc::c_void)
 -> size_t {
    let iend: *const BYTE = end as *const BYTE;
    match (*ms).cParams.strategy as libc::c_uint {
        1 => {
            ZSTD_fillHashTable(ms, iend as *const libc::c_void,
                               ZSTD_dtlm_fast);
        }
        2 => {
            ZSTD_fillDoubleHashTable(ms, iend as *const libc::c_void,
                                     ZSTD_dtlm_fast);
        }
        3 | 4 | 5 | 6 | 7 | 8 | 9 | _ => { }
    }
    return 0i32 as size_t;
}
/* * ZSTD_ldm_limitTableUpdate() :
 *
 *  Sets cctx->nextToUpdate to a position corresponding closer to anchor
 *  if it is far way
 *  (after a long match, only update tables a limited amount). */
unsafe extern "C" fn ZSTD_ldm_limitTableUpdate(mut ms: *mut ZSTD_matchState_t,
                                               mut anchor: *const BYTE) {
    let current: U32 =
        anchor.wrapping_offset_from((*ms).window.base) as libc::c_long as U32;
    if current > (*ms).nextToUpdate.wrapping_add(1024i32 as libc::c_uint) {
        (*ms).nextToUpdate =
            current.wrapping_sub(if (512i32 as libc::c_uint) <
                                        current.wrapping_sub((*ms).nextToUpdate).wrapping_sub(1024i32
                                                                                                  as
                                                                                                  libc::c_uint)
                                    {
                                     512i32 as libc::c_uint
                                 } else {
                                     current.wrapping_sub((*ms).nextToUpdate).wrapping_sub(1024i32
                                                                                               as
                                                                                               libc::c_uint)
                                 })
    };
}
/* *
 * If the sequence length is longer than remaining then the sequence is split
 * between this block and the next.
 *
 * Returns the current sequence to handle, or if the rest of the block should
 * be literals, it returns a sequence with offset == 0.
 */
unsafe extern "C" fn maybeSplitSequence(mut rawSeqStore: *mut rawSeqStore_t,
                                        remaining: U32, minMatch: U32)
 -> rawSeq {
    let mut sequence: rawSeq =
        *(*rawSeqStore).seq.offset((*rawSeqStore).pos as isize);
    if remaining >= sequence.litLength.wrapping_add(sequence.matchLength) {
        (*rawSeqStore).pos = (*rawSeqStore).pos.wrapping_add(1);
        return sequence
    }
    if remaining <= sequence.litLength {
        sequence.offset = 0i32 as U32
    } else if remaining <
                  sequence.litLength.wrapping_add(sequence.matchLength) {
        sequence.matchLength = remaining.wrapping_sub(sequence.litLength);
        if sequence.matchLength < minMatch { sequence.offset = 0i32 as U32 }
    }
    ZSTD_ldm_skipSequences(rawSeqStore, remaining as size_t, minMatch);
    return sequence;
}
/* *
 * ZSTD_ldm_skipSequences():
 *
 * Skip past `srcSize` bytes worth of sequences in `rawSeqStore`.
 * Avoids emitting matches less than `minMatch` bytes.
 * Must be called for data with is not passed to ZSTD_ldm_blockCompress().
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_ldm_skipSequences(mut rawSeqStore:
                                                    *mut rawSeqStore_t,
                                                mut srcSize: size_t,
                                                minMatch: U32) {
    while srcSize > 0i32 as libc::c_ulong &&
              (*rawSeqStore).pos < (*rawSeqStore).size {
        let mut seq: *mut rawSeq =
            (*rawSeqStore).seq.offset((*rawSeqStore).pos as isize);
        if srcSize <= (*seq).litLength as libc::c_ulong {
            (*seq).litLength =
                ((*seq).litLength as
                     libc::c_uint).wrapping_sub(srcSize as U32) as U32 as U32;
            return
        }
        srcSize =
            (srcSize as
                 libc::c_ulong).wrapping_sub((*seq).litLength as
                                                 libc::c_ulong) as size_t as
                size_t;
        (*seq).litLength = 0i32 as U32;
        if srcSize < (*seq).matchLength as libc::c_ulong {
            (*seq).matchLength =
                ((*seq).matchLength as
                     libc::c_uint).wrapping_sub(srcSize as U32) as U32 as U32;
            if (*seq).matchLength < minMatch {
                if (*rawSeqStore).pos.wrapping_add(1i32 as libc::c_ulong) <
                       (*rawSeqStore).size {
                    let ref mut fresh4 = (*seq.offset(1isize)).litLength;
                    *fresh4 =
                        (*fresh4 as
                             libc::c_uint).wrapping_add((*seq.offset(0isize)).matchLength)
                            as U32 as U32
                }
                (*rawSeqStore).pos = (*rawSeqStore).pos.wrapping_add(1)
            }
            return
        }
        srcSize =
            (srcSize as
                 libc::c_ulong).wrapping_sub((*seq).matchLength as
                                                 libc::c_ulong) as size_t as
                size_t;
        (*seq).matchLength = 0i32 as U32;
        (*rawSeqStore).pos = (*rawSeqStore).pos.wrapping_add(1)
    };
}
/* * ZSTD_ldm_getTableSize() :
 *  Estimate the space needed for long distance matching tables or 0 if LDM is
 *  disabled.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_ldm_getTableSize(mut params: ldmParams_t)
 -> size_t {
    let ldmHSize: size_t = (1i32 as size_t) << params.hashLog;
    let ldmBucketSizeLog: size_t =
        (if params.bucketSizeLog < params.hashLog {
             params.bucketSizeLog
         } else { params.hashLog }) as size_t;
    let ldmBucketSize: size_t =
        (1i32 as size_t) <<
            (params.hashLog as libc::c_ulong).wrapping_sub(ldmBucketSizeLog);
    let totalSize: size_t =
        ldmBucketSize.wrapping_add(ldmHSize.wrapping_mul(::std::mem::size_of::<ldmEntry_t>()
                                                             as
                                                             libc::c_ulong));
    return if 0 != params.enableLdm {
               totalSize
           } else { 0i32 as libc::c_ulong };
}
/* * ZSTD_ldm_getSeqSpace() :
 *  Return an upper bound on the number of sequences that can be produced by
 *  the long distance matcher, or 0 if LDM is disabled.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_ldm_getMaxNbSeq(mut params: ldmParams_t,
                                              mut maxChunkSize: size_t)
 -> size_t {
    return if 0 != params.enableLdm {
               maxChunkSize.wrapping_div(params.minMatchLength as
                                             libc::c_ulong)
           } else { 0i32 as libc::c_ulong };
}
/* * ZSTD_ldm_adjustParameters() :
 *  If the params->hashRateLog is not set, set it to its default value based on
 *  windowLog and params->hashLog.
 *
 *  Ensures that params->bucketSizeLog is <= params->hashLog (setting it to
 *  params->hashLog if it is not).
 *
 *  Ensures that the minMatchLength >= targetLength during optimal parsing.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_ldm_adjustParameters(mut params:
                                                       *mut ldmParams_t,
                                                   mut cParams:
                                                       *const ZSTD_compressionParameters) {
    (*params).windowLog = (*cParams).windowLog;
    if 0 == (*params).bucketSizeLog { (*params).bucketSizeLog = 3i32 as U32 }
    if 0 == (*params).minMatchLength {
        (*params).minMatchLength = 64i32 as U32
    }
    if (*cParams).strategy as libc::c_uint >=
           ZSTD_btopt as libc::c_int as libc::c_uint {
        let minMatch: U32 =
            if (*cParams).targetLength > (*params).minMatchLength {
                (*cParams).targetLength
            } else { (*params).minMatchLength };
        (*params).minMatchLength = minMatch
    }
    if (*params).hashLog == 0i32 as libc::c_uint {
        (*params).hashLog =
            if 6i32 as libc::c_uint >
                   (*params).windowLog.wrapping_sub(7i32 as libc::c_uint) {
                6i32 as libc::c_uint
            } else { (*params).windowLog.wrapping_sub(7i32 as libc::c_uint) }
    }
    if (*params).hashRateLog == 0i32 as libc::c_uint {
        (*params).hashRateLog =
            if (*params).windowLog < (*params).hashLog {
                0i32 as libc::c_uint
            } else { (*params).windowLog.wrapping_sub((*params).hashLog) }
    }
    (*params).bucketSizeLog =
        if (*params).bucketSizeLog < (*params).hashLog {
            (*params).bucketSizeLog
        } else { (*params).hashLog };
}