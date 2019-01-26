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
pub type ZSTD_dictMode_e = libc::c_uint;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
pub type searchMax_f
    =
    Option<unsafe extern "C" fn(_: *mut ZSTD_matchState_t, _: *const BYTE,
                                _: *const BYTE, _: *mut size_t) -> size_t>;
pub type searchMax_f_0
    =
    Option<unsafe extern "C" fn(_: *mut ZSTD_matchState_t, _: *const BYTE,
                                _: *const BYTE, _: *mut size_t) -> size_t>;
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
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return (in_0 as libc::c_ulonglong).swap_bytes() as U64;
}
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const libc::c_void) -> U64 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read64(memPtr)
    } else { return MEM_swap64(MEM_read64(memPtr)) };
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
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> U32 {
    return (31i32 - val.leading_zeros() as i32) as U32;
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
#[no_mangle]
pub unsafe extern "C" fn ZSTD_insertAndFindFirstIndex(mut ms:
                                                          *mut ZSTD_matchState_t,
                                                      mut ip: *const BYTE)
 -> U32 {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    return ZSTD_insertAndFindFirstIndex_internal(ms, cParams, ip,
                                                 (*ms).cParams.minMatch);
}
/* *********************************
*  Hash Chain
***********************************/
/* Update chains up to ip (excluded)
   Assumption : always within prefix (i.e. not within extDict) */
unsafe extern "C" fn ZSTD_insertAndFindFirstIndex_internal(mut ms:
                                                               *mut ZSTD_matchState_t,
                                                           cParams:
                                                               *const ZSTD_compressionParameters,
                                                           mut ip:
                                                               *const BYTE,
                                                           mls: U32) -> U32 {
    let hashTable: *mut U32 = (*ms).hashTable;
    let hashLog: U32 = (*cParams).hashLog;
    let chainTable: *mut U32 = (*ms).chainTable;
    let chainMask: U32 = ((1i32 << (*cParams).chainLog) - 1i32) as U32;
    let base: *const BYTE = (*ms).window.base;
    let target: U32 = ip.wrapping_offset_from(base) as libc::c_long as U32;
    let mut idx: U32 = (*ms).nextToUpdate;
    while idx < target {
        let h: size_t =
            ZSTD_hashPtr(base.offset(idx as isize) as *const libc::c_void,
                         hashLog, mls);
        *chainTable.offset((idx & chainMask) as isize) =
            *hashTable.offset(h as isize);
        *hashTable.offset(h as isize) = idx;
        idx = idx.wrapping_add(1)
    }
    (*ms).nextToUpdate = target;
    return *hashTable.offset(ZSTD_hashPtr(ip as *const libc::c_void, hashLog,
                                          mls) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btlazy2(mut ms:
                                                        *mut ZSTD_matchState_t,
                                                    mut seqStore:
                                                        *mut seqStore_t,
                                                    mut rep: *mut U32,
                                                    mut src:
                                                        *const libc::c_void,
                                                    mut srcSize: size_t)
 -> size_t {
    return ZSTD_compressBlock_lazy_generic(ms, seqStore, rep, src, srcSize,
                                           1i32 as U32, 2i32 as U32,
                                           ZSTD_noDict);
}
/* *******************************
*  Common parser - lazy strategy
*********************************/
unsafe extern "C" fn ZSTD_compressBlock_lazy_generic(mut ms:
                                                         *mut ZSTD_matchState_t,
                                                     mut seqStore:
                                                         *mut seqStore_t,
                                                     mut rep: *mut U32,
                                                     mut src:
                                                         *const libc::c_void,
                                                     mut srcSize: size_t,
                                                     searchMethod: U32,
                                                     depth: U32,
                                                     dictMode:
                                                         ZSTD_dictMode_e)
 -> size_t {
    let mut current_block: u64;
    let istart: *const BYTE = src as *const BYTE;
    let mut ip: *const BYTE = istart;
    let mut anchor: *const BYTE = istart;
    let iend: *const BYTE = istart.offset(srcSize as isize);
    let ilimit: *const BYTE = iend.offset(-8isize);
    let base: *const BYTE = (*ms).window.base;
    let prefixLowestIndex: U32 = (*ms).window.dictLimit;
    let prefixLowest: *const BYTE = base.offset(prefixLowestIndex as isize);
    let searchMax: searchMax_f =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            if 0 != searchMethod {
                Some(ZSTD_BtFindBestMatch_dictMatchState_selectMLS)
            } else { Some(ZSTD_HcFindBestMatch_dictMatchState_selectMLS) }
        } else if 0 != searchMethod {
            Some(ZSTD_BtFindBestMatch_selectMLS)
        } else { Some(ZSTD_HcFindBestMatch_selectMLS) };
    let mut offset_1: U32 = *rep.offset(0isize);
    let mut offset_2: U32 = *rep.offset(1isize);
    let mut savedOffset: U32 = 0i32 as U32;
    let dms: *const ZSTD_matchState_t = (*ms).dictMatchState;
    let dictLowestIndex: U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*dms).window.dictLimit
        } else { 0i32 as libc::c_uint };
    let dictBase: *const BYTE =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*dms).window.base
        } else { 0 as *const BYTE };
    let dictLowest: *const BYTE =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            dictBase.offset(dictLowestIndex as isize)
        } else { 0 as *const BYTE };
    let dictEnd: *const BYTE =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*dms).window.nextSrc
        } else { 0 as *const BYTE };
    let dictIndexDelta: U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            prefixLowestIndex.wrapping_sub(dictEnd.wrapping_offset_from(dictBase)
                                               as libc::c_long as U32)
        } else { 0i32 as libc::c_uint };
    let dictAndPrefixLength: U32 =
        dictEnd.offset(ip.wrapping_offset_from(prefixLowest) as libc::c_long
                           as isize).wrapping_offset_from(dictLowest) as
            libc::c_long as U32;
    ip =
        ip.offset((dictAndPrefixLength == 0i32 as libc::c_uint) as libc::c_int
                      as isize);
    (*ms).nextToUpdate3 = (*ms).nextToUpdate;
    if dictMode as libc::c_uint == ZSTD_noDict as libc::c_int as libc::c_uint
       {
        let maxRep: U32 =
            ip.wrapping_offset_from(prefixLowest) as libc::c_long as U32;
        if offset_2 > maxRep {
            savedOffset = offset_2;
            offset_2 = 0i32 as U32
        }
        if offset_1 > maxRep {
            savedOffset = offset_1;
            offset_1 = 0i32 as U32
        }
    }
    dictMode as libc::c_uint ==
        ZSTD_dictMatchState as libc::c_int as libc::c_uint;
    /* Match Loop */
    while ip < ilimit {
        let mut matchLength: size_t = 0i32 as size_t;
        let mut offset: size_t = 0i32 as size_t;
        let mut start: *const BYTE = ip.offset(1isize);
        /* check repCode */
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            let repIndex: U32 =
                (ip.wrapping_offset_from(base) as libc::c_long as
                     U32).wrapping_add(1i32 as
                                           libc::c_uint).wrapping_sub(offset_1);
            let mut repMatch: *const BYTE =
                if dictMode as libc::c_uint ==
                       ZSTD_dictMatchState as libc::c_int as libc::c_uint &&
                       repIndex < prefixLowestIndex {
                    dictBase.offset(repIndex.wrapping_sub(dictIndexDelta) as
                                        isize)
                } else { base.offset(repIndex as isize) };
            /* intentional underflow */
            if prefixLowestIndex.wrapping_sub(1i32 as
                                                  libc::c_uint).wrapping_sub(repIndex)
                   >= 3i32 as libc::c_uint &&
                   MEM_read32(repMatch as *const libc::c_void) ==
                       MEM_read32(ip.offset(1isize) as *const libc::c_void) {
                let mut repMatchEnd: *const BYTE =
                    if repIndex < prefixLowestIndex { dictEnd } else { iend };
                matchLength =
                    ZSTD_count_2segments(ip.offset(1isize).offset(4isize),
                                         repMatch.offset(4isize), iend,
                                         repMatchEnd,
                                         prefixLowest).wrapping_add(4i32 as
                                                                        libc::c_ulong);
                if depth == 0i32 as libc::c_uint {
                    current_block = 9533101733188459420;
                } else { current_block = 5494826135382683477; }
            } else { current_block = 5494826135382683477; }
        } else { current_block = 5494826135382683477; }
        match current_block {
            5494826135382683477 => {
                if dictMode as libc::c_uint ==
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
                    matchLength =
                        ZSTD_count(ip.offset(1isize).offset(4isize),
                                   ip.offset(1isize).offset(4isize).offset(-(offset_1
                                                                                 as
                                                                                 isize)),
                                   iend).wrapping_add(4i32 as libc::c_ulong);
                    if depth == 0i32 as libc::c_uint {
                        current_block = 9533101733188459420;
                    } else { current_block = 7245201122033322888; }
                } else { current_block = 7245201122033322888; }
                match current_block {
                    9533101733188459420 => { }
                    _ => {
                        let mut offsetFound: size_t = 999999999i32 as size_t;
                        let ml2: size_t =
                            searchMax.expect("non-null function pointer")(ms,
                                                                          ip,
                                                                          iend,
                                                                          &mut offsetFound);
                        if ml2 > matchLength {
                            matchLength = ml2;
                            start = ip;
                            offset = offsetFound
                        }
                        if matchLength < 4i32 as libc::c_ulong {
                            ip =
                                ip.offset(((ip.wrapping_offset_from(anchor) as
                                                libc::c_long >> 8i32) +
                                               1i32 as libc::c_long) as
                                              isize);
                            continue ;
                        } else {
                            if depth >= 1i32 as libc::c_uint {
                                while ip < ilimit {
                                    ip = ip.offset(1isize);
                                    if dictMode as libc::c_uint ==
                                           ZSTD_noDict as libc::c_int as
                                               libc::c_uint && 0 != offset &&
                                           0 !=
                                               (offset_1 >
                                                    0i32 as libc::c_uint) as
                                                   libc::c_int &
                                                   (MEM_read32(ip as
                                                                   *const libc::c_void)
                                                        ==
                                                        MEM_read32(ip.offset(-(offset_1
                                                                                   as
                                                                                   isize))
                                                                       as
                                                                       *const libc::c_void))
                                                       as libc::c_int {
                                        let mlRep: size_t =
                                            ZSTD_count(ip.offset(4isize),
                                                       ip.offset(4isize).offset(-(offset_1
                                                                                      as
                                                                                      isize)),
                                                       iend).wrapping_add(4i32
                                                                              as
                                                                              libc::c_ulong);
                                        let gain2: libc::c_int =
                                            mlRep.wrapping_mul(3i32 as
                                                                   libc::c_ulong)
                                                as libc::c_int;
                                        let gain1: libc::c_int =
                                            matchLength.wrapping_mul(3i32 as
                                                                         libc::c_ulong).wrapping_sub(ZSTD_highbit32((offset
                                                                                                                         as
                                                                                                                         U32).wrapping_add(1i32
                                                                                                                                               as
                                                                                                                                               libc::c_uint))
                                                                                                         as
                                                                                                         libc::c_ulong).wrapping_add(1i32
                                                                                                                                         as
                                                                                                                                         libc::c_ulong)
                                                as libc::c_int;
                                        if mlRep >= 4i32 as libc::c_ulong &&
                                               gain2 > gain1 {
                                            matchLength = mlRep;
                                            offset = 0i32 as size_t;
                                            start = ip
                                        }
                                    }
                                    if dictMode as libc::c_uint ==
                                           ZSTD_dictMatchState as libc::c_int
                                               as libc::c_uint {
                                        let repIndex_0: U32 =
                                            (ip.wrapping_offset_from(base) as
                                                 libc::c_long as
                                                 U32).wrapping_sub(offset_1);
                                        let mut repMatch_0: *const BYTE =
                                            if repIndex_0 < prefixLowestIndex
                                               {
                                                dictBase.offset(repIndex_0.wrapping_sub(dictIndexDelta)
                                                                    as isize)
                                            } else {
                                                base.offset(repIndex_0 as
                                                                isize)
                                            };
                                        if prefixLowestIndex.wrapping_sub(1i32
                                                                              as
                                                                              libc::c_uint).wrapping_sub(repIndex_0)
                                               >= 3i32 as libc::c_uint &&
                                               MEM_read32(repMatch_0 as
                                                              *const libc::c_void)
                                                   ==
                                                   MEM_read32(ip as
                                                                  *const libc::c_void)
                                           {
                                            let mut repMatchEnd_0:
                                                    *const BYTE =
                                                if repIndex_0 <
                                                       prefixLowestIndex {
                                                    dictEnd
                                                } else { iend };
                                            let mlRep_0: size_t =
                                                ZSTD_count_2segments(ip.offset(4isize),
                                                                     repMatch_0.offset(4isize),
                                                                     iend,
                                                                     repMatchEnd_0,
                                                                     prefixLowest).wrapping_add(4i32
                                                                                                    as
                                                                                                    libc::c_ulong);
                                            let gain2_0: libc::c_int =
                                                mlRep_0.wrapping_mul(3i32 as
                                                                         libc::c_ulong)
                                                    as libc::c_int;
                                            let gain1_0: libc::c_int =
                                                matchLength.wrapping_mul(3i32
                                                                             as
                                                                             libc::c_ulong).wrapping_sub(ZSTD_highbit32((offset
                                                                                                                             as
                                                                                                                             U32).wrapping_add(1i32
                                                                                                                                                   as
                                                                                                                                                   libc::c_uint))
                                                                                                             as
                                                                                                             libc::c_ulong).wrapping_add(1i32
                                                                                                                                             as
                                                                                                                                             libc::c_ulong)
                                                    as libc::c_int;
                                            if mlRep_0 >=
                                                   4i32 as libc::c_ulong &&
                                                   gain2_0 > gain1_0 {
                                                matchLength = mlRep_0;
                                                offset = 0i32 as size_t;
                                                start = ip
                                            }
                                        }
                                    }
                                    let mut offset2: size_t =
                                        999999999i32 as size_t;
                                    let ml2_0: size_t =
                                        searchMax.expect("non-null function pointer")(ms,
                                                                                      ip,
                                                                                      iend,
                                                                                      &mut offset2);
                                    /* raw approx */
                                    let gain2_1: libc::c_int =
                                        ml2_0.wrapping_mul(4i32 as
                                                               libc::c_ulong).wrapping_sub(ZSTD_highbit32((offset2
                                                                                                               as
                                                                                                               U32).wrapping_add(1i32
                                                                                                                                     as
                                                                                                                                     libc::c_uint))
                                                                                               as
                                                                                               libc::c_ulong)
                                            as libc::c_int;
                                    let gain1_1: libc::c_int =
                                        matchLength.wrapping_mul(4i32 as
                                                                     libc::c_ulong).wrapping_sub(ZSTD_highbit32((offset
                                                                                                                     as
                                                                                                                     U32).wrapping_add(1i32
                                                                                                                                           as
                                                                                                                                           libc::c_uint))
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_add(4i32
                                                                                                                                     as
                                                                                                                                     libc::c_ulong)
                                            as libc::c_int;
                                    if ml2_0 >= 4i32 as libc::c_ulong &&
                                           gain2_1 > gain1_1 {
                                        matchLength = ml2_0;
                                        offset = offset2;
                                        start = ip
                                    } else {
                                        /* search a better one */
                                        /* let's find an even better one */
                                        if depth == 2i32 as libc::c_uint &&
                                               ip < ilimit {
                                            ip = ip.offset(1isize);
                                            if dictMode as libc::c_uint ==
                                                   ZSTD_noDict as libc::c_int
                                                       as libc::c_uint &&
                                                   0 != offset &&
                                                   0 !=
                                                       (offset_1 >
                                                            0i32 as
                                                                libc::c_uint)
                                                           as libc::c_int &
                                                           (MEM_read32(ip as
                                                                           *const libc::c_void)
                                                                ==
                                                                MEM_read32(ip.offset(-(offset_1
                                                                                           as
                                                                                           isize))
                                                                               as
                                                                               *const libc::c_void))
                                                               as libc::c_int
                                               {
                                                let mlRep_1: size_t =
                                                    ZSTD_count(ip.offset(4isize),
                                                               ip.offset(4isize).offset(-(offset_1
                                                                                              as
                                                                                              isize)),
                                                               iend).wrapping_add(4i32
                                                                                      as
                                                                                      libc::c_ulong);
                                                let gain2_2: libc::c_int =
                                                    mlRep_1.wrapping_mul(4i32
                                                                             as
                                                                             libc::c_ulong)
                                                        as libc::c_int;
                                                let gain1_2: libc::c_int =
                                                    matchLength.wrapping_mul(4i32
                                                                                 as
                                                                                 libc::c_ulong).wrapping_sub(ZSTD_highbit32((offset
                                                                                                                                 as
                                                                                                                                 U32).wrapping_add(1i32
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint))
                                                                                                                 as
                                                                                                                 libc::c_ulong).wrapping_add(1i32
                                                                                                                                                 as
                                                                                                                                                 libc::c_ulong)
                                                        as libc::c_int;
                                                if mlRep_1 >=
                                                       4i32 as libc::c_ulong
                                                       && gain2_2 > gain1_2 {
                                                    matchLength = mlRep_1;
                                                    offset = 0i32 as size_t;
                                                    start = ip
                                                }
                                            }
                                            if dictMode as libc::c_uint ==
                                                   ZSTD_dictMatchState as
                                                       libc::c_int as
                                                       libc::c_uint {
                                                let repIndex_1: U32 =
                                                    (ip.wrapping_offset_from(base)
                                                         as libc::c_long as
                                                         U32).wrapping_sub(offset_1);
                                                let mut repMatch_1:
                                                        *const BYTE =
                                                    if repIndex_1 <
                                                           prefixLowestIndex {
                                                        dictBase.offset(repIndex_1.wrapping_sub(dictIndexDelta)
                                                                            as
                                                                            isize)
                                                    } else {
                                                        base.offset(repIndex_1
                                                                        as
                                                                        isize)
                                                    };
                                                if prefixLowestIndex.wrapping_sub(1i32
                                                                                      as
                                                                                      libc::c_uint).wrapping_sub(repIndex_1)
                                                       >= 3i32 as libc::c_uint
                                                       &&
                                                       MEM_read32(repMatch_1
                                                                      as
                                                                      *const libc::c_void)
                                                           ==
                                                           MEM_read32(ip as
                                                                          *const libc::c_void)
                                                   {
                                                    let mut repMatchEnd_1:
                                                            *const BYTE =
                                                        if repIndex_1 <
                                                               prefixLowestIndex
                                                           {
                                                            dictEnd
                                                        } else { iend };
                                                    let mlRep_2: size_t =
                                                        ZSTD_count_2segments(ip.offset(4isize),
                                                                             repMatch_1.offset(4isize),
                                                                             iend,
                                                                             repMatchEnd_1,
                                                                             prefixLowest).wrapping_add(4i32
                                                                                                            as
                                                                                                            libc::c_ulong);
                                                    let gain2_3: libc::c_int =
                                                        mlRep_2.wrapping_mul(4i32
                                                                                 as
                                                                                 libc::c_ulong)
                                                            as libc::c_int;
                                                    let gain1_3: libc::c_int =
                                                        matchLength.wrapping_mul(4i32
                                                                                     as
                                                                                     libc::c_ulong).wrapping_sub(ZSTD_highbit32((offset
                                                                                                                                     as
                                                                                                                                     U32).wrapping_add(1i32
                                                                                                                                                           as
                                                                                                                                                           libc::c_uint))
                                                                                                                     as
                                                                                                                     libc::c_ulong).wrapping_add(1i32
                                                                                                                                                     as
                                                                                                                                                     libc::c_ulong)
                                                            as libc::c_int;
                                                    if mlRep_2 >=
                                                           4i32 as
                                                               libc::c_ulong
                                                           &&
                                                           gain2_3 > gain1_3 {
                                                        matchLength = mlRep_2;
                                                        offset =
                                                            0i32 as size_t;
                                                        start = ip
                                                    }
                                                }
                                            }
                                            let mut offset2_0: size_t =
                                                999999999i32 as size_t;
                                            let ml2_1: size_t =
                                                searchMax.expect("non-null function pointer")(ms,
                                                                                              ip,
                                                                                              iend,
                                                                                              &mut offset2_0);
                                            /* raw approx */
                                            let gain2_4: libc::c_int =
                                                ml2_1.wrapping_mul(4i32 as
                                                                       libc::c_ulong).wrapping_sub(ZSTD_highbit32((offset2_0
                                                                                                                       as
                                                                                                                       U32).wrapping_add(1i32
                                                                                                                                             as
                                                                                                                                             libc::c_uint))
                                                                                                       as
                                                                                                       libc::c_ulong)
                                                    as libc::c_int;
                                            let gain1_4: libc::c_int =
                                                matchLength.wrapping_mul(4i32
                                                                             as
                                                                             libc::c_ulong).wrapping_sub(ZSTD_highbit32((offset
                                                                                                                             as
                                                                                                                             U32).wrapping_add(1i32
                                                                                                                                                   as
                                                                                                                                                   libc::c_uint))
                                                                                                             as
                                                                                                             libc::c_ulong).wrapping_add(7i32
                                                                                                                                             as
                                                                                                                                             libc::c_ulong)
                                                    as libc::c_int;
                                            if ml2_1 >= 4i32 as libc::c_ulong
                                                   && gain2_4 > gain1_4 {
                                                matchLength = ml2_1;
                                                offset = offset2_0;
                                                start = ip;
                                                continue ;
                                            }
                                        }
                                        /* nothing found : store previous solution */
                                        break ;
                                    }
                                }
                            }
                            if 0 != offset {
                                if dictMode as libc::c_uint ==
                                       ZSTD_noDict as libc::c_int as
                                           libc::c_uint {
                                    while 0 !=
                                              (start > anchor) as libc::c_int
                                                  &
                                                  (start.offset(-(offset.wrapping_sub((3i32
                                                                                           -
                                                                                           1i32)
                                                                                          as
                                                                                          libc::c_ulong)
                                                                      as
                                                                      isize))
                                                       > prefixLowest) as
                                                      libc::c_int &&
                                              *start.offset(-1i32 as isize) as
                                                  libc::c_int ==
                                                  *start.offset(-(offset.wrapping_sub((3i32
                                                                                           -
                                                                                           1i32)
                                                                                          as
                                                                                          libc::c_ulong)
                                                                      as
                                                                      isize)).offset(-1i32
                                                                                         as
                                                                                         isize)
                                                      as libc::c_int {
                                        start = start.offset(-1isize);
                                        matchLength =
                                            matchLength.wrapping_add(1)
                                    }
                                }
                                if dictMode as libc::c_uint ==
                                       ZSTD_dictMatchState as libc::c_int as
                                           libc::c_uint {
                                    let matchIndex: U32 =
                                        (start.wrapping_offset_from(base) as
                                             libc::c_long as
                                             libc::c_ulong).wrapping_sub(offset.wrapping_sub((3i32
                                                                                                  -
                                                                                                  1i32)
                                                                                                 as
                                                                                                 libc::c_ulong))
                                            as U32;
                                    let mut match_0: *const BYTE =
                                        if matchIndex < prefixLowestIndex {
                                            dictBase.offset(matchIndex as
                                                                isize).offset(-(dictIndexDelta
                                                                                    as
                                                                                    isize))
                                        } else {
                                            base.offset(matchIndex as isize)
                                        };
                                    let mStart: *const BYTE =
                                        if matchIndex < prefixLowestIndex {
                                            dictLowest
                                        } else { prefixLowest };
                                    while start > anchor && match_0 > mStart
                                              &&
                                              *start.offset(-1i32 as isize) as
                                                  libc::c_int ==
                                                  *match_0.offset(-1i32 as
                                                                      isize)
                                                      as libc::c_int {
                                        start = start.offset(-1isize);
                                        match_0 = match_0.offset(-1isize);
                                        matchLength =
                                            matchLength.wrapping_add(1)
                                    }
                                }
                                offset_2 = offset_1;
                                offset_1 =
                                    offset.wrapping_sub((3i32 - 1i32) as
                                                            libc::c_ulong) as
                                        U32
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        /* store sequence */
        let litLength: size_t =
            start.wrapping_offset_from(anchor) as libc::c_long as size_t;
        ZSTD_storeSeq(seqStore, litLength, anchor as *const libc::c_void,
                      offset as U32,
                      matchLength.wrapping_sub(3i32 as libc::c_ulong));
        ip = start.offset(matchLength as isize);
        anchor = ip;
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            while ip <= ilimit {
                let current2: U32 =
                    ip.wrapping_offset_from(base) as libc::c_long as U32;
                let repIndex_2: U32 = current2.wrapping_sub(offset_2);
                let mut repMatch_2: *const BYTE =
                    if dictMode as libc::c_uint ==
                           ZSTD_dictMatchState as libc::c_int as libc::c_uint
                           && repIndex_2 < prefixLowestIndex {
                        dictBase.offset(-(dictIndexDelta as
                                              isize)).offset(repIndex_2 as
                                                                 isize)
                    } else { base.offset(repIndex_2 as isize) };
                /* intentional overflow */
                if !(prefixLowestIndex.wrapping_sub(1i32 as
                                                        libc::c_uint).wrapping_sub(repIndex_2)
                         >= 3i32 as libc::c_uint &&
                         MEM_read32(repMatch_2 as *const libc::c_void) ==
                             MEM_read32(ip as *const libc::c_void)) {
                    break ;
                }
                let repEnd2: *const BYTE =
                    if repIndex_2 < prefixLowestIndex {
                        dictEnd
                    } else { iend };
                matchLength =
                    ZSTD_count_2segments(ip.offset(4isize),
                                         repMatch_2.offset(4isize), iend,
                                         repEnd2,
                                         prefixLowest).wrapping_add(4i32 as
                                                                        libc::c_ulong);
                offset = offset_2 as size_t;
                offset_2 = offset_1;
                offset_1 = offset as U32;
                ZSTD_storeSeq(seqStore, 0i32 as size_t,
                              anchor as *const libc::c_void, 0i32 as U32,
                              matchLength.wrapping_sub(3i32 as
                                                           libc::c_ulong));
                ip = ip.offset(matchLength as isize);
                anchor = ip
            }
        }
        if dictMode as libc::c_uint ==
               ZSTD_noDict as libc::c_int as libc::c_uint {
            while 0 !=
                      (ip <= ilimit) as libc::c_int &
                          (offset_2 > 0i32 as libc::c_uint) as libc::c_int &&
                      MEM_read32(ip as *const libc::c_void) ==
                          MEM_read32(ip.offset(-(offset_2 as isize)) as
                                         *const libc::c_void) {
                matchLength =
                    ZSTD_count(ip.offset(4isize),
                               ip.offset(4isize).offset(-(offset_2 as isize)),
                               iend).wrapping_add(4i32 as libc::c_ulong);
                offset = offset_2 as size_t;
                offset_2 = offset_1;
                offset_1 = offset as U32;
                ZSTD_storeSeq(seqStore, 0i32 as size_t,
                              anchor as *const libc::c_void, 0i32 as U32,
                              matchLength.wrapping_sub(3i32 as
                                                           libc::c_ulong));
                ip = ip.offset(matchLength as isize);
                anchor = ip
            }
        }
    }
    *rep.offset(0isize) = if 0 != offset_1 { offset_1 } else { savedOffset };
    *rep.offset(1isize) = if 0 != offset_2 { offset_2 } else { savedOffset };
    return iend.wrapping_offset_from(anchor) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_HcFindBestMatch_selectMLS(mut ms:
                                                        *mut ZSTD_matchState_t,
                                                    mut ip: *const BYTE,
                                                    iLimit: *const BYTE,
                                                    mut offsetPtr:
                                                        *mut size_t)
 -> size_t {
    match (*ms).cParams.minMatch {
        5 => {
            return ZSTD_HcFindBestMatch_generic(ms, ip, iLimit, offsetPtr,
                                                5i32 as U32, ZSTD_noDict)
        }
        7 | 6 => {
            return ZSTD_HcFindBestMatch_generic(ms, ip, iLimit, offsetPtr,
                                                6i32 as U32, ZSTD_noDict)
        }
        4 | _ => {
            return ZSTD_HcFindBestMatch_generic(ms, ip, iLimit, offsetPtr,
                                                4i32 as U32, ZSTD_noDict)
        }
    };
}
/* inlining is important to hardwire a hot branch (template emulation) */
unsafe extern "C" fn ZSTD_HcFindBestMatch_generic(mut ms:
                                                      *mut ZSTD_matchState_t,
                                                  ip: *const BYTE,
                                                  iLimit: *const BYTE,
                                                  mut offsetPtr: *mut size_t,
                                                  mls: U32,
                                                  dictMode: ZSTD_dictMode_e)
 -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let chainTable: *mut U32 = (*ms).chainTable;
    let chainSize: U32 = (1i32 << (*cParams).chainLog) as U32;
    let chainMask: U32 = chainSize.wrapping_sub(1i32 as libc::c_uint);
    let base: *const BYTE = (*ms).window.base;
    let dictBase: *const BYTE = (*ms).window.dictBase;
    let dictLimit: U32 = (*ms).window.dictLimit;
    let prefixStart: *const BYTE = base.offset(dictLimit as isize);
    let dictEnd: *const BYTE = dictBase.offset(dictLimit as isize);
    let lowLimit: U32 = (*ms).window.lowLimit;
    let current: U32 = ip.wrapping_offset_from(base) as libc::c_long as U32;
    let minChain: U32 =
        if current > chainSize {
            current.wrapping_sub(chainSize)
        } else { 0i32 as libc::c_uint };
    let mut nbAttempts: U32 = 1u32 << (*cParams).searchLog;
    let mut ml: size_t = (4i32 - 1i32) as size_t;
    /* HC4 match finder */
    let mut matchIndex: U32 =
        ZSTD_insertAndFindFirstIndex_internal(ms, cParams, ip, mls);
    while 0 !=
              (matchIndex > lowLimit) as libc::c_int &
                  (nbAttempts > 0i32 as libc::c_uint) as libc::c_int {
        let mut currentMl: size_t = 0i32 as size_t;
        if dictMode as libc::c_uint !=
               ZSTD_extDict as libc::c_int as libc::c_uint ||
               matchIndex >= dictLimit {
            let match_0: *const BYTE = base.offset(matchIndex as isize);
            if *match_0.offset(ml as isize) as libc::c_int ==
                   *ip.offset(ml as isize) as libc::c_int {
                currentMl = ZSTD_count(ip, match_0, iLimit)
            }
        } else {
            let match_1: *const BYTE = dictBase.offset(matchIndex as isize);
            if MEM_read32(match_1 as *const libc::c_void) ==
                   MEM_read32(ip as *const libc::c_void) {
                currentMl =
                    ZSTD_count_2segments(ip.offset(4isize),
                                         match_1.offset(4isize), iLimit,
                                         dictEnd,
                                         prefixStart).wrapping_add(4i32 as
                                                                       libc::c_ulong)
            }
        }
        /* save best solution */
        if currentMl > ml {
            ml = currentMl;
            *offsetPtr =
                current.wrapping_sub(matchIndex).wrapping_add((3i32 - 1i32) as
                                                                  libc::c_uint)
                    as size_t;
            if ip.offset(currentMl as isize) == iLimit {
                /* best possible, avoids read overflow on next attempt */
                break ;
            }
        }
        if matchIndex <= minChain { break ; }
        matchIndex = *chainTable.offset((matchIndex & chainMask) as isize);
        nbAttempts = nbAttempts.wrapping_sub(1)
    }
    if dictMode as libc::c_uint ==
           ZSTD_dictMatchState as libc::c_int as libc::c_uint {
        let dms: *const ZSTD_matchState_t = (*ms).dictMatchState;
        let dmsChainTable: *const U32 = (*dms).chainTable;
        let dmsChainSize: U32 = (1i32 << (*dms).cParams.chainLog) as U32;
        let dmsChainMask: U32 =
            dmsChainSize.wrapping_sub(1i32 as libc::c_uint);
        let dmsLowestIndex: U32 = (*dms).window.dictLimit;
        let dmsBase: *const BYTE = (*dms).window.base;
        let dmsEnd: *const BYTE = (*dms).window.nextSrc;
        let dmsSize: U32 =
            dmsEnd.wrapping_offset_from(dmsBase) as libc::c_long as U32;
        let dmsIndexDelta: U32 = dictLimit.wrapping_sub(dmsSize);
        let dmsMinChain: U32 =
            if dmsSize > dmsChainSize {
                dmsSize.wrapping_sub(dmsChainSize)
            } else { 0i32 as libc::c_uint };
        matchIndex =
            *(*dms).hashTable.offset(ZSTD_hashPtr(ip as *const libc::c_void,
                                                  (*dms).cParams.hashLog, mls)
                                         as isize);
        while 0 !=
                  (matchIndex > dmsLowestIndex) as libc::c_int &
                      (nbAttempts > 0i32 as libc::c_uint) as libc::c_int {
            let mut currentMl_0: size_t = 0i32 as size_t;
            let match_2: *const BYTE = dmsBase.offset(matchIndex as isize);
            if MEM_read32(match_2 as *const libc::c_void) ==
                   MEM_read32(ip as *const libc::c_void) {
                currentMl_0 =
                    ZSTD_count_2segments(ip.offset(4isize),
                                         match_2.offset(4isize), iLimit,
                                         dmsEnd,
                                         prefixStart).wrapping_add(4i32 as
                                                                       libc::c_ulong)
            }
            /* save best solution */
            if currentMl_0 > ml {
                ml = currentMl_0;
                *offsetPtr =
                    current.wrapping_sub(matchIndex.wrapping_add(dmsIndexDelta)).wrapping_add((3i32
                                                                                                   -
                                                                                                   1i32)
                                                                                                  as
                                                                                                  libc::c_uint)
                        as size_t;
                if ip.offset(currentMl_0 as isize) == iLimit {
                    /* best possible, avoids read overflow on next attempt */
                    break ;
                }
            }
            if matchIndex <= dmsMinChain { break ; }
            matchIndex =
                *dmsChainTable.offset((matchIndex & dmsChainMask) as isize);
            nbAttempts = nbAttempts.wrapping_sub(1)
        }
    }
    return ml;
}
unsafe extern "C" fn ZSTD_BtFindBestMatch_selectMLS(mut ms:
                                                        *mut ZSTD_matchState_t,
                                                    mut ip: *const BYTE,
                                                    iLimit: *const BYTE,
                                                    mut offsetPtr:
                                                        *mut size_t)
 -> size_t {
    match (*ms).cParams.minMatch {
        5 => {
            return ZSTD_BtFindBestMatch(ms, ip, iLimit, offsetPtr,
                                        5i32 as U32, ZSTD_noDict)
        }
        7 | 6 => {
            return ZSTD_BtFindBestMatch(ms, ip, iLimit, offsetPtr,
                                        6i32 as U32, ZSTD_noDict)
        }
        4 | _ => {
            return ZSTD_BtFindBestMatch(ms, ip, iLimit, offsetPtr,
                                        4i32 as U32, ZSTD_noDict)
        }
    };
}
/* * ZSTD_BtFindBestMatch() : Tree updater, providing best match */
unsafe extern "C" fn ZSTD_BtFindBestMatch(mut ms: *mut ZSTD_matchState_t,
                                          ip: *const BYTE,
                                          iLimit: *const BYTE,
                                          mut offsetPtr: *mut size_t,
                                          mls: U32, dictMode: ZSTD_dictMode_e)
 -> size_t {
    if ip < (*ms).window.base.offset((*ms).nextToUpdate as isize) {
        return 0i32 as size_t
    }
    ZSTD_updateDUBT(ms, ip, iLimit, mls);
    return ZSTD_DUBT_findBestMatch(ms, ip, iLimit, offsetPtr, mls, dictMode);
}
unsafe extern "C" fn ZSTD_DUBT_findBestMatch(mut ms: *mut ZSTD_matchState_t,
                                             ip: *const BYTE,
                                             iend: *const BYTE,
                                             mut offsetPtr: *mut size_t,
                                             mls: U32,
                                             dictMode: ZSTD_dictMode_e)
 -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable: *mut U32 = (*ms).hashTable;
    let hashLog: U32 = (*cParams).hashLog;
    let h: size_t = ZSTD_hashPtr(ip as *const libc::c_void, hashLog, mls);
    let mut matchIndex: U32 = *hashTable.offset(h as isize);
    let base: *const BYTE = (*ms).window.base;
    let current: U32 = ip.wrapping_offset_from(base) as libc::c_long as U32;
    let windowLow: U32 = (*ms).window.lowLimit;
    let bt: *mut U32 = (*ms).chainTable;
    let btLog: U32 = (*cParams).chainLog.wrapping_sub(1i32 as libc::c_uint);
    let btMask: U32 = ((1i32 << btLog) - 1i32) as U32;
    let btLow: U32 =
        if btMask >= current {
            0i32 as libc::c_uint
        } else { current.wrapping_sub(btMask) };
    let unsortLimit: U32 = if btLow > windowLow { btLow } else { windowLow };
    let mut nextCandidate: *mut U32 =
        bt.offset((2i32 as libc::c_uint).wrapping_mul(matchIndex & btMask) as
                      isize);
    let mut unsortedMark: *mut U32 =
        bt.offset((2i32 as libc::c_uint).wrapping_mul(matchIndex & btMask) as
                      isize).offset(1isize);
    let mut nbCompares: U32 = 1u32 << (*cParams).searchLog;
    let mut nbCandidates: U32 = nbCompares;
    let mut previousCandidate: U32 = 0i32 as U32;
    while matchIndex > unsortLimit && *unsortedMark == 1i32 as libc::c_uint &&
              nbCandidates > 1i32 as libc::c_uint {
        *unsortedMark = previousCandidate;
        previousCandidate = matchIndex;
        matchIndex = *nextCandidate;
        nextCandidate =
            bt.offset((2i32 as libc::c_uint).wrapping_mul(matchIndex & btMask)
                          as isize);
        unsortedMark =
            bt.offset((2i32 as libc::c_uint).wrapping_mul(matchIndex & btMask)
                          as isize).offset(1isize);
        nbCandidates = nbCandidates.wrapping_sub(1)
    }
    if matchIndex > unsortLimit && *unsortedMark == 1i32 as libc::c_uint {
        *unsortedMark = 0i32 as U32;
        *nextCandidate = *unsortedMark
    }
    matchIndex = previousCandidate;
    while 0 != matchIndex {
        let nextCandidateIdxPtr: *mut U32 =
            bt.offset((2i32 as libc::c_uint).wrapping_mul(matchIndex & btMask)
                          as isize).offset(1isize);
        let nextCandidateIdx: U32 = *nextCandidateIdxPtr;
        ZSTD_insertDUBT1(ms, matchIndex, iend, nbCandidates, unsortLimit,
                         dictMode);
        matchIndex = nextCandidateIdx;
        nbCandidates = nbCandidates.wrapping_add(1)
    }
    let mut commonLengthSmaller: size_t = 0i32 as size_t;
    let mut commonLengthLarger: size_t = 0i32 as size_t;
    let dictBase: *const BYTE = (*ms).window.dictBase;
    let dictLimit: U32 = (*ms).window.dictLimit;
    let dictEnd: *const BYTE = dictBase.offset(dictLimit as isize);
    let prefixStart: *const BYTE = base.offset(dictLimit as isize);
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
    let mut bestLength: size_t = 0i32 as size_t;
    matchIndex = *hashTable.offset(h as isize);
    *hashTable.offset(h as isize) = current;
    loop  {
        let fresh0 = nbCompares;
        nbCompares = nbCompares.wrapping_sub(1);
        if !(0 != fresh0 && matchIndex > windowLow) { break ; }
        let nextPtr: *mut U32 =
            bt.offset((2i32 as libc::c_uint).wrapping_mul(matchIndex & btMask)
                          as isize);
        /* guaranteed minimum nb of common bytes */
        let mut matchLength: size_t =
            if commonLengthSmaller < commonLengthLarger {
                commonLengthSmaller
            } else { commonLengthLarger };
        let mut match_0: *const BYTE = 0 as *const BYTE;
        if dictMode as libc::c_uint !=
               ZSTD_extDict as libc::c_int as libc::c_uint ||
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
            if matchLength >
                   matchEndIdx.wrapping_sub(matchIndex) as libc::c_ulong {
                matchEndIdx = matchIndex.wrapping_add(matchLength as U32)
            }
            if 4i32 * matchLength.wrapping_sub(bestLength) as libc::c_int >
                   ZSTD_highbit32(current.wrapping_sub(matchIndex).wrapping_add(1i32
                                                                                    as
                                                                                    libc::c_uint)).wrapping_sub(ZSTD_highbit32((*offsetPtr.offset(0isize)
                                                                                                                                    as
                                                                                                                                    U32).wrapping_add(1i32
                                                                                                                                                          as
                                                                                                                                                          libc::c_uint)))
                       as libc::c_int {
                bestLength = matchLength;
                *offsetPtr =
                    ((3i32 - 1i32) as
                         libc::c_uint).wrapping_add(current).wrapping_sub(matchIndex)
                        as size_t
            }
            if ip.offset(matchLength as isize) == iend {
                /* equal : no way to know if inf or sup */
                if dictMode as libc::c_uint ==
                       ZSTD_dictMatchState as libc::c_int as libc::c_uint {
                    nbCompares = 0i32 as U32
                }
                /* drop, to guarantee consistency (miss a little bit of compression) */
                break ;
            }
        }
        if (*match_0.offset(matchLength as isize) as libc::c_int) <
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
        bestLength =
            ZSTD_DUBT_findBetterDictMatch(ms, ip, iend, offsetPtr, bestLength,
                                          nbCompares, mls, dictMode)
    }
    (*ms).nextToUpdate = matchEndIdx.wrapping_sub(8i32 as libc::c_uint);
    if bestLength >= 3i32 as libc::c_ulong {
        let mIndex: U32 =
            current.wrapping_sub((*offsetPtr as
                                      U32).wrapping_sub((3i32 - 1i32) as
                                                            libc::c_uint));
    }
    return bestLength;
}
unsafe extern "C" fn ZSTD_DUBT_findBetterDictMatch(mut ms:
                                                       *mut ZSTD_matchState_t,
                                                   ip: *const BYTE,
                                                   iend: *const BYTE,
                                                   mut offsetPtr: *mut size_t,
                                                   mut bestLength: size_t,
                                                   mut nbCompares: U32,
                                                   mls: U32,
                                                   dictMode: ZSTD_dictMode_e)
 -> size_t {
    let dms: *const ZSTD_matchState_t = (*ms).dictMatchState;
    let dmsCParams: *const ZSTD_compressionParameters = &(*dms).cParams;
    let dictHashTable: *const U32 = (*dms).hashTable;
    let hashLog: U32 = (*dmsCParams).hashLog;
    let h: size_t = ZSTD_hashPtr(ip as *const libc::c_void, hashLog, mls);
    let mut dictMatchIndex: U32 = *dictHashTable.offset(h as isize);
    let base: *const BYTE = (*ms).window.base;
    let prefixStart: *const BYTE =
        base.offset((*ms).window.dictLimit as isize);
    let current: U32 = ip.wrapping_offset_from(base) as libc::c_long as U32;
    let dictBase: *const BYTE = (*dms).window.base;
    let dictEnd: *const BYTE = (*dms).window.nextSrc;
    let dictHighLimit: U32 =
        (*dms).window.nextSrc.wrapping_offset_from((*dms).window.base) as
            libc::c_long as U32;
    let dictLowLimit: U32 = (*dms).window.lowLimit;
    let dictIndexDelta: U32 =
        (*ms).window.lowLimit.wrapping_sub(dictHighLimit);
    let dictBt: *mut U32 = (*dms).chainTable;
    let btLog: U32 =
        (*dmsCParams).chainLog.wrapping_sub(1i32 as libc::c_uint);
    let btMask: U32 = ((1i32 << btLog) - 1i32) as U32;
    let btLow: U32 =
        if btMask >= dictHighLimit.wrapping_sub(dictLowLimit) {
            dictLowLimit
        } else { dictHighLimit.wrapping_sub(btMask) };
    let mut commonLengthSmaller: size_t = 0i32 as size_t;
    let mut commonLengthLarger: size_t = 0i32 as size_t;
    loop  {
        let fresh1 = nbCompares;
        nbCompares = nbCompares.wrapping_sub(1);
        if !(0 != fresh1 && dictMatchIndex > dictLowLimit) { break ; }
        let nextPtr: *mut U32 =
            dictBt.offset((2i32 as
                               libc::c_uint).wrapping_mul(dictMatchIndex &
                                                              btMask) as
                              isize);
        /* guaranteed minimum nb of common bytes */
        let mut matchLength: size_t =
            if commonLengthSmaller < commonLengthLarger {
                commonLengthSmaller
            } else { commonLengthLarger };
        let mut match_0: *const BYTE =
            dictBase.offset(dictMatchIndex as isize);
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
        if (dictMatchIndex as libc::c_ulong).wrapping_add(matchLength) >=
               dictHighLimit as libc::c_ulong {
            match_0 =
                base.offset(dictMatchIndex as
                                isize).offset(dictIndexDelta as isize)
        }
        if matchLength > bestLength {
            let mut matchIndex: U32 =
                dictMatchIndex.wrapping_add(dictIndexDelta);
            if 4i32 * matchLength.wrapping_sub(bestLength) as libc::c_int >
                   ZSTD_highbit32(current.wrapping_sub(matchIndex).wrapping_add(1i32
                                                                                    as
                                                                                    libc::c_uint)).wrapping_sub(ZSTD_highbit32((*offsetPtr.offset(0isize)
                                                                                                                                    as
                                                                                                                                    U32).wrapping_add(1i32
                                                                                                                                                          as
                                                                                                                                                          libc::c_uint)))
                       as libc::c_int {
                bestLength = matchLength;
                *offsetPtr =
                    ((3i32 - 1i32) as
                         libc::c_uint).wrapping_add(current).wrapping_sub(matchIndex)
                        as size_t
            }
            if ip.offset(matchLength as isize) == iend {
                /* reached end of input : ip[matchLength] is not valid, no way to know if it's larger or smaller than match */
                /* drop, to guarantee consistency (miss a little bit of compression) */
                break ;
            }
        }
        if (*match_0.offset(matchLength as isize) as libc::c_int) <
               *ip.offset(matchLength as isize) as libc::c_int {
            if dictMatchIndex <= btLow {
                /* beyond tree size, stop the search */
                break ;
            } else {
                commonLengthSmaller = matchLength;
                dictMatchIndex = *nextPtr.offset(1isize)
            }
        } else if dictMatchIndex <= btLow {
            /* beyond tree size, stop the search */
            break ;
        } else {
            commonLengthLarger = matchLength;
            dictMatchIndex = *nextPtr.offset(0isize)
        }
    }
    if bestLength >= 3i32 as libc::c_ulong {
        let mIndex: U32 =
            current.wrapping_sub((*offsetPtr as
                                      U32).wrapping_sub((3i32 - 1i32) as
                                                            libc::c_uint));
    }
    return bestLength;
}
/* * ZSTD_insertDUBT1() :
 *  sort one already inserted but unsorted position
 *  assumption : current >= btlow == (current - btmask)
 *  doesn't fail */
unsafe extern "C" fn ZSTD_insertDUBT1(mut ms: *mut ZSTD_matchState_t,
                                      mut current: U32,
                                      mut inputEnd: *const BYTE,
                                      mut nbCompares: U32, mut btLow: U32,
                                      dictMode: ZSTD_dictMode_e) {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let bt: *mut U32 = (*ms).chainTable;
    let btLog: U32 = (*cParams).chainLog.wrapping_sub(1i32 as libc::c_uint);
    let btMask: U32 = ((1i32 << btLog) - 1i32) as U32;
    let mut commonLengthSmaller: size_t = 0i32 as size_t;
    let mut commonLengthLarger: size_t = 0i32 as size_t;
    let base: *const BYTE = (*ms).window.base;
    let dictBase: *const BYTE = (*ms).window.dictBase;
    let dictLimit: U32 = (*ms).window.dictLimit;
    let ip: *const BYTE =
        if current >= dictLimit {
            base.offset(current as isize)
        } else { dictBase.offset(current as isize) };
    let iend: *const BYTE =
        if current >= dictLimit {
            inputEnd
        } else { dictBase.offset(dictLimit as isize) };
    let dictEnd: *const BYTE = dictBase.offset(dictLimit as isize);
    let prefixStart: *const BYTE = base.offset(dictLimit as isize);
    let mut match_0: *const BYTE = 0 as *const BYTE;
    let mut smallerPtr: *mut U32 =
        bt.offset((2i32 as libc::c_uint).wrapping_mul(current & btMask) as
                      isize);
    let mut largerPtr: *mut U32 = smallerPtr.offset(1isize);
    /* this candidate is unsorted : next sorted candidate is reached through *smallerPtr, while *largerPtr contains previous unsorted candidate (which is already saved and can be overwritten) */
    let mut matchIndex: U32 = *smallerPtr;
    /* to be nullified at the end */
    let mut dummy32: U32 = 0;
    let windowLow: U32 = (*ms).window.lowLimit;
    loop  {
        let fresh2 = nbCompares;
        nbCompares = nbCompares.wrapping_sub(1);
        if !(0 != fresh2 && matchIndex > windowLow) { break ; }
        let nextPtr: *mut U32 =
            bt.offset((2i32 as libc::c_uint).wrapping_mul(matchIndex & btMask)
                          as isize);
        /* guaranteed minimum nb of common bytes */
        let mut matchLength: size_t =
            if commonLengthSmaller < commonLengthLarger {
                commonLengthSmaller
            } else { commonLengthLarger };
        if dictMode as libc::c_uint !=
               ZSTD_extDict as libc::c_int as libc::c_uint ||
               (matchIndex as libc::c_ulong).wrapping_add(matchLength) >=
                   dictLimit as libc::c_ulong || current < dictLimit {
            let mBase: *const BYTE =
                if dictMode as libc::c_uint !=
                       ZSTD_extDict as libc::c_int as libc::c_uint ||
                       (matchIndex as libc::c_ulong).wrapping_add(matchLength)
                           >= dictLimit as libc::c_ulong {
                    base
                } else { dictBase };
            match_0 = mBase.offset(matchIndex as isize);
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
/*-*************************************
*  Binary Tree search
***************************************/
unsafe extern "C" fn ZSTD_updateDUBT(mut ms: *mut ZSTD_matchState_t,
                                     mut ip: *const BYTE,
                                     mut iend: *const BYTE, mut mls: U32) {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable: *mut U32 = (*ms).hashTable;
    let hashLog: U32 = (*cParams).hashLog;
    let bt: *mut U32 = (*ms).chainTable;
    let btLog: U32 = (*cParams).chainLog.wrapping_sub(1i32 as libc::c_uint);
    let btMask: U32 = ((1i32 << btLog) - 1i32) as U32;
    let base: *const BYTE = (*ms).window.base;
    let target: U32 = ip.wrapping_offset_from(base) as libc::c_long as U32;
    let mut idx: U32 = (*ms).nextToUpdate;
    idx != target;
    while idx < target {
        let h: size_t =
            ZSTD_hashPtr(base.offset(idx as isize) as *const libc::c_void,
                         hashLog, mls);
        let matchIndex: U32 = *hashTable.offset(h as isize);
        let nextCandidatePtr: *mut U32 =
            bt.offset((2i32 as libc::c_uint).wrapping_mul(idx & btMask) as
                          isize);
        let sortMarkPtr: *mut U32 = nextCandidatePtr.offset(1isize);
        *hashTable.offset(h as isize) = idx;
        *nextCandidatePtr = matchIndex;
        *sortMarkPtr = 1i32 as U32;
        idx = idx.wrapping_add(1)
    }
    (*ms).nextToUpdate = target;
}
unsafe extern "C" fn ZSTD_HcFindBestMatch_dictMatchState_selectMLS(mut ms:
                                                                       *mut ZSTD_matchState_t,
                                                                   mut ip:
                                                                       *const BYTE,
                                                                   iLimit:
                                                                       *const BYTE,
                                                                   mut offsetPtr:
                                                                       *mut size_t)
 -> size_t {
    match (*ms).cParams.minMatch {
        5 => {
            return ZSTD_HcFindBestMatch_generic(ms, ip, iLimit, offsetPtr,
                                                5i32 as U32,
                                                ZSTD_dictMatchState)
        }
        7 | 6 => {
            return ZSTD_HcFindBestMatch_generic(ms, ip, iLimit, offsetPtr,
                                                6i32 as U32,
                                                ZSTD_dictMatchState)
        }
        4 | _ => {
            return ZSTD_HcFindBestMatch_generic(ms, ip, iLimit, offsetPtr,
                                                4i32 as U32,
                                                ZSTD_dictMatchState)
        }
    };
}
unsafe extern "C" fn ZSTD_BtFindBestMatch_dictMatchState_selectMLS(mut ms:
                                                                       *mut ZSTD_matchState_t,
                                                                   mut ip:
                                                                       *const BYTE,
                                                                   iLimit:
                                                                       *const BYTE,
                                                                   mut offsetPtr:
                                                                       *mut size_t)
 -> size_t {
    match (*ms).cParams.minMatch {
        5 => {
            return ZSTD_BtFindBestMatch(ms, ip, iLimit, offsetPtr,
                                        5i32 as U32, ZSTD_dictMatchState)
        }
        7 | 6 => {
            return ZSTD_BtFindBestMatch(ms, ip, iLimit, offsetPtr,
                                        6i32 as U32, ZSTD_dictMatchState)
        }
        4 | _ => {
            return ZSTD_BtFindBestMatch(ms, ip, iLimit, offsetPtr,
                                        4i32 as U32, ZSTD_dictMatchState)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2(mut ms:
                                                      *mut ZSTD_matchState_t,
                                                  mut seqStore:
                                                      *mut seqStore_t,
                                                  mut rep: *mut U32,
                                                  mut src:
                                                      *const libc::c_void,
                                                  mut srcSize: size_t)
 -> size_t {
    return ZSTD_compressBlock_lazy_generic(ms, seqStore, rep, src, srcSize,
                                           0i32 as U32, 2i32 as U32,
                                           ZSTD_noDict);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy(mut ms:
                                                     *mut ZSTD_matchState_t,
                                                 mut seqStore:
                                                     *mut seqStore_t,
                                                 mut rep: *mut U32,
                                                 mut src: *const libc::c_void,
                                                 mut srcSize: size_t)
 -> size_t {
    return ZSTD_compressBlock_lazy_generic(ms, seqStore, rep, src, srcSize,
                                           0i32 as U32, 1i32 as U32,
                                           ZSTD_noDict);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy(mut ms:
                                                       *mut ZSTD_matchState_t,
                                                   mut seqStore:
                                                       *mut seqStore_t,
                                                   mut rep: *mut U32,
                                                   mut src:
                                                       *const libc::c_void,
                                                   mut srcSize: size_t)
 -> size_t {
    return ZSTD_compressBlock_lazy_generic(ms, seqStore, rep, src, srcSize,
                                           0i32 as U32, 0i32 as U32,
                                           ZSTD_noDict);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btlazy2_dictMatchState(mut ms:
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
    return ZSTD_compressBlock_lazy_generic(ms, seqStore, rep, src, srcSize,
                                           1i32 as U32, 2i32 as U32,
                                           ZSTD_dictMatchState);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_dictMatchState(mut ms:
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
    return ZSTD_compressBlock_lazy_generic(ms, seqStore, rep, src, srcSize,
                                           0i32 as U32, 2i32 as U32,
                                           ZSTD_dictMatchState);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_dictMatchState(mut ms:
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
    return ZSTD_compressBlock_lazy_generic(ms, seqStore, rep, src, srcSize,
                                           0i32 as U32, 1i32 as U32,
                                           ZSTD_dictMatchState);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_dictMatchState(mut ms:
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
    return ZSTD_compressBlock_lazy_generic(ms, seqStore, rep, src, srcSize,
                                           0i32 as U32, 0i32 as U32,
                                           ZSTD_dictMatchState);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_extDict(mut ms:
                                                               *mut ZSTD_matchState_t,
                                                           mut seqStore:
                                                               *mut seqStore_t,
                                                           mut rep: *mut U32,
                                                           mut src:
                                                               *const libc::c_void,
                                                           mut srcSize:
                                                               size_t)
 -> size_t {
    return ZSTD_compressBlock_lazy_extDict_generic(ms, seqStore, rep, src,
                                                   srcSize, 0i32 as U32,
                                                   0i32 as U32);
}
unsafe extern "C" fn ZSTD_compressBlock_lazy_extDict_generic(mut ms:
                                                                 *mut ZSTD_matchState_t,
                                                             mut seqStore:
                                                                 *mut seqStore_t,
                                                             mut rep:
                                                                 *mut U32,
                                                             mut src:
                                                                 *const libc::c_void,
                                                             mut srcSize:
                                                                 size_t,
                                                             searchMethod:
                                                                 U32,
                                                             depth: U32)
 -> size_t {
    let istart: *const BYTE = src as *const BYTE;
    let mut ip: *const BYTE = istart;
    let mut anchor: *const BYTE = istart;
    let iend: *const BYTE = istart.offset(srcSize as isize);
    let ilimit: *const BYTE = iend.offset(-8isize);
    let base: *const BYTE = (*ms).window.base;
    let dictLimit: U32 = (*ms).window.dictLimit;
    let lowestIndex: U32 = (*ms).window.lowLimit;
    let prefixStart: *const BYTE = base.offset(dictLimit as isize);
    let dictBase: *const BYTE = (*ms).window.dictBase;
    let dictEnd: *const BYTE = dictBase.offset(dictLimit as isize);
    let dictStart: *const BYTE = dictBase.offset(lowestIndex as isize);
    let mut searchMax: searchMax_f_0 =
        if 0 != searchMethod {
            Some(ZSTD_BtFindBestMatch_extDict_selectMLS)
        } else { Some(ZSTD_HcFindBestMatch_extDict_selectMLS) };
    let mut offset_1: U32 = *rep.offset(0isize);
    let mut offset_2: U32 = *rep.offset(1isize);
    (*ms).nextToUpdate3 = (*ms).nextToUpdate;
    ip = ip.offset((ip == prefixStart) as libc::c_int as isize);
    let mut current_block_48: u64;
    while ip < ilimit {
        let mut matchLength: size_t = 0i32 as size_t;
        let mut offset: size_t = 0i32 as size_t;
        let mut start: *const BYTE = ip.offset(1isize);
        let mut current: U32 =
            ip.wrapping_offset_from(base) as libc::c_long as U32;
        /* check repCode */
        let repIndex: U32 =
            current.wrapping_add(1i32 as libc::c_uint).wrapping_sub(offset_1);
        let repBase: *const BYTE =
            if repIndex < dictLimit { dictBase } else { base };
        let repMatch: *const BYTE = repBase.offset(repIndex as isize);
        /* intentional overflow */
        if 0 !=
               (dictLimit.wrapping_sub(1i32 as
                                           libc::c_uint).wrapping_sub(repIndex)
                    >= 3i32 as libc::c_uint) as libc::c_int &
                   (repIndex > lowestIndex) as libc::c_int {
            if MEM_read32(ip.offset(1isize) as *const libc::c_void) ==
                   MEM_read32(repMatch as *const libc::c_void) {
                /* repcode detected we should take it */
                let repEnd: *const BYTE =
                    if repIndex < dictLimit { dictEnd } else { iend };
                matchLength =
                    ZSTD_count_2segments(ip.offset(1isize).offset(4isize),
                                         repMatch.offset(4isize), iend,
                                         repEnd,
                                         prefixStart).wrapping_add(4i32 as
                                                                       libc::c_ulong);
                if depth == 0i32 as libc::c_uint {
                    current_block_48 = 3274233182728301011;
                } else { current_block_48 = 4495394744059808450; }
            } else { current_block_48 = 4495394744059808450; }
        } else { current_block_48 = 4495394744059808450; }
        match current_block_48 {
            4495394744059808450 => {
                let mut offsetFound: size_t = 999999999i32 as size_t;
                let ml2: size_t =
                    searchMax.expect("non-null function pointer")(ms, ip,
                                                                  iend,
                                                                  &mut offsetFound);
                if ml2 > matchLength {
                    matchLength = ml2;
                    start = ip;
                    offset = offsetFound
                }
                if matchLength < 4i32 as libc::c_ulong {
                    ip =
                        ip.offset(((ip.wrapping_offset_from(anchor) as
                                        libc::c_long >> 8i32) +
                                       1i32 as libc::c_long) as isize);
                    continue ;
                } else {
                    if depth >= 1i32 as libc::c_uint {
                        while ip < ilimit {
                            ip = ip.offset(1isize);
                            current = current.wrapping_add(1);
                            if 0 != offset {
                                let repIndex_0: U32 =
                                    current.wrapping_sub(offset_1);
                                let repBase_0: *const BYTE =
                                    if repIndex_0 < dictLimit {
                                        dictBase
                                    } else { base };
                                let repMatch_0: *const BYTE =
                                    repBase_0.offset(repIndex_0 as isize);
                                if 0 !=
                                       (dictLimit.wrapping_sub(1i32 as
                                                                   libc::c_uint).wrapping_sub(repIndex_0)
                                            >= 3i32 as libc::c_uint) as
                                           libc::c_int &
                                           (repIndex_0 > lowestIndex) as
                                               libc::c_int {
                                    if MEM_read32(ip as *const libc::c_void)
                                           ==
                                           MEM_read32(repMatch_0 as
                                                          *const libc::c_void)
                                       {
                                        let repEnd_0: *const BYTE =
                                            if repIndex_0 < dictLimit {
                                                dictEnd
                                            } else { iend };
                                        let repLength: size_t =
                                            ZSTD_count_2segments(ip.offset(4isize),
                                                                 repMatch_0.offset(4isize),
                                                                 iend,
                                                                 repEnd_0,
                                                                 prefixStart).wrapping_add(4i32
                                                                                               as
                                                                                               libc::c_ulong);
                                        let gain2: libc::c_int =
                                            repLength.wrapping_mul(3i32 as
                                                                       libc::c_ulong)
                                                as libc::c_int;
                                        let gain1: libc::c_int =
                                            matchLength.wrapping_mul(3i32 as
                                                                         libc::c_ulong).wrapping_sub(ZSTD_highbit32((offset
                                                                                                                         as
                                                                                                                         U32).wrapping_add(1i32
                                                                                                                                               as
                                                                                                                                               libc::c_uint))
                                                                                                         as
                                                                                                         libc::c_ulong).wrapping_add(1i32
                                                                                                                                         as
                                                                                                                                         libc::c_ulong)
                                                as libc::c_int;
                                        if repLength >= 4i32 as libc::c_ulong
                                               && gain2 > gain1 {
                                            matchLength = repLength;
                                            offset = 0i32 as size_t;
                                            start = ip
                                        }
                                    }
                                }
                            }
                            /* search match, depth 1 */
                            let mut offset2: size_t = 999999999i32 as size_t;
                            let ml2_0: size_t =
                                searchMax.expect("non-null function pointer")(ms,
                                                                              ip,
                                                                              iend,
                                                                              &mut offset2);
                            /* raw approx */
                            let gain2_0: libc::c_int =
                                ml2_0.wrapping_mul(4i32 as
                                                       libc::c_ulong).wrapping_sub(ZSTD_highbit32((offset2
                                                                                                       as
                                                                                                       U32).wrapping_add(1i32
                                                                                                                             as
                                                                                                                             libc::c_uint))
                                                                                       as
                                                                                       libc::c_ulong)
                                    as libc::c_int;
                            let gain1_0: libc::c_int =
                                matchLength.wrapping_mul(4i32 as
                                                             libc::c_ulong).wrapping_sub(ZSTD_highbit32((offset
                                                                                                             as
                                                                                                             U32).wrapping_add(1i32
                                                                                                                                   as
                                                                                                                                   libc::c_uint))
                                                                                             as
                                                                                             libc::c_ulong).wrapping_add(4i32
                                                                                                                             as
                                                                                                                             libc::c_ulong)
                                    as libc::c_int;
                            if ml2_0 >= 4i32 as libc::c_ulong &&
                                   gain2_0 > gain1_0 {
                                matchLength = ml2_0;
                                offset = offset2;
                                start = ip
                            } else {
                                /* search a better one */
                                /* let's find an even better one */
                                if depth == 2i32 as libc::c_uint &&
                                       ip < ilimit {
                                    ip = ip.offset(1isize);
                                    current = current.wrapping_add(1);
                                    if 0 != offset {
                                        let repIndex_1: U32 =
                                            current.wrapping_sub(offset_1);
                                        let repBase_1: *const BYTE =
                                            if repIndex_1 < dictLimit {
                                                dictBase
                                            } else { base };
                                        let repMatch_1: *const BYTE =
                                            repBase_1.offset(repIndex_1 as
                                                                 isize);
                                        if 0 !=
                                               (dictLimit.wrapping_sub(1i32 as
                                                                           libc::c_uint).wrapping_sub(repIndex_1)
                                                    >= 3i32 as libc::c_uint)
                                                   as libc::c_int &
                                                   (repIndex_1 > lowestIndex)
                                                       as libc::c_int {
                                            if MEM_read32(ip as
                                                              *const libc::c_void)
                                                   ==
                                                   MEM_read32(repMatch_1 as
                                                                  *const libc::c_void)
                                               {
                                                let repEnd_1: *const BYTE =
                                                    if repIndex_1 < dictLimit
                                                       {
                                                        dictEnd
                                                    } else { iend };
                                                let repLength_0: size_t =
                                                    ZSTD_count_2segments(ip.offset(4isize),
                                                                         repMatch_1.offset(4isize),
                                                                         iend,
                                                                         repEnd_1,
                                                                         prefixStart).wrapping_add(4i32
                                                                                                       as
                                                                                                       libc::c_ulong);
                                                let gain2_1: libc::c_int =
                                                    repLength_0.wrapping_mul(4i32
                                                                                 as
                                                                                 libc::c_ulong)
                                                        as libc::c_int;
                                                let gain1_1: libc::c_int =
                                                    matchLength.wrapping_mul(4i32
                                                                                 as
                                                                                 libc::c_ulong).wrapping_sub(ZSTD_highbit32((offset
                                                                                                                                 as
                                                                                                                                 U32).wrapping_add(1i32
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint))
                                                                                                                 as
                                                                                                                 libc::c_ulong).wrapping_add(1i32
                                                                                                                                                 as
                                                                                                                                                 libc::c_ulong)
                                                        as libc::c_int;
                                                if repLength_0 >=
                                                       4i32 as libc::c_ulong
                                                       && gain2_1 > gain1_1 {
                                                    matchLength = repLength_0;
                                                    offset = 0i32 as size_t;
                                                    start = ip
                                                }
                                            }
                                        }
                                    }
                                    /* search match, depth 2 */
                                    let mut offset2_0: size_t =
                                        999999999i32 as size_t;
                                    let ml2_1: size_t =
                                        searchMax.expect("non-null function pointer")(ms,
                                                                                      ip,
                                                                                      iend,
                                                                                      &mut offset2_0);
                                    /* raw approx */
                                    let gain2_2: libc::c_int =
                                        ml2_1.wrapping_mul(4i32 as
                                                               libc::c_ulong).wrapping_sub(ZSTD_highbit32((offset2_0
                                                                                                               as
                                                                                                               U32).wrapping_add(1i32
                                                                                                                                     as
                                                                                                                                     libc::c_uint))
                                                                                               as
                                                                                               libc::c_ulong)
                                            as libc::c_int;
                                    let gain1_2: libc::c_int =
                                        matchLength.wrapping_mul(4i32 as
                                                                     libc::c_ulong).wrapping_sub(ZSTD_highbit32((offset
                                                                                                                     as
                                                                                                                     U32).wrapping_add(1i32
                                                                                                                                           as
                                                                                                                                           libc::c_uint))
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_add(7i32
                                                                                                                                     as
                                                                                                                                     libc::c_ulong)
                                            as libc::c_int;
                                    if ml2_1 >= 4i32 as libc::c_ulong &&
                                           gain2_2 > gain1_2 {
                                        matchLength = ml2_1;
                                        offset = offset2_0;
                                        start = ip;
                                        continue ;
                                    }
                                }
                                /* nothing found : store previous solution */
                                break ;
                            }
                        }
                    }
                    if 0 != offset {
                        let matchIndex: U32 =
                            (start.wrapping_offset_from(base) as libc::c_long
                                 as
                                 libc::c_ulong).wrapping_sub(offset.wrapping_sub((3i32
                                                                                      -
                                                                                      1i32)
                                                                                     as
                                                                                     libc::c_ulong))
                                as U32;
                        let mut match_0: *const BYTE =
                            if matchIndex < dictLimit {
                                dictBase.offset(matchIndex as isize)
                            } else { base.offset(matchIndex as isize) };
                        let mStart: *const BYTE =
                            if matchIndex < dictLimit {
                                dictStart
                            } else { prefixStart };
                        while start > anchor && match_0 > mStart &&
                                  *start.offset(-1i32 as isize) as libc::c_int
                                      ==
                                      *match_0.offset(-1i32 as isize) as
                                          libc::c_int {
                            start = start.offset(-1isize);
                            match_0 = match_0.offset(-1isize);
                            matchLength = matchLength.wrapping_add(1)
                        }
                        offset_2 = offset_1;
                        offset_1 =
                            offset.wrapping_sub((3i32 - 1i32) as
                                                    libc::c_ulong) as U32
                    }
                }
            }
            _ => { }
        }
        /* store sequence */
        let litLength: size_t =
            start.wrapping_offset_from(anchor) as libc::c_long as size_t;
        ZSTD_storeSeq(seqStore, litLength, anchor as *const libc::c_void,
                      offset as U32,
                      matchLength.wrapping_sub(3i32 as libc::c_ulong));
        ip = start.offset(matchLength as isize);
        anchor = ip;
        while ip <= ilimit {
            let repIndex_2: U32 =
                (ip.wrapping_offset_from(base) as libc::c_long -
                     offset_2 as libc::c_long) as U32;
            let repBase_2: *const BYTE =
                if repIndex_2 < dictLimit { dictBase } else { base };
            let repMatch_2: *const BYTE =
                repBase_2.offset(repIndex_2 as isize);
            /* intentional overflow */
            if !(0 !=
                     (dictLimit.wrapping_sub(1i32 as
                                                 libc::c_uint).wrapping_sub(repIndex_2)
                          >= 3i32 as libc::c_uint) as libc::c_int &
                         (repIndex_2 > lowestIndex) as libc::c_int) {
                break ;
            }
            if !(MEM_read32(ip as *const libc::c_void) ==
                     MEM_read32(repMatch_2 as *const libc::c_void)) {
                break ;
            }
            /* repcode detected we should take it */
            let repEnd_2: *const BYTE =
                if repIndex_2 < dictLimit { dictEnd } else { iend };
            matchLength =
                ZSTD_count_2segments(ip.offset(4isize),
                                     repMatch_2.offset(4isize), iend,
                                     repEnd_2,
                                     prefixStart).wrapping_add(4i32 as
                                                                   libc::c_ulong);
            offset = offset_2 as size_t;
            offset_2 = offset_1;
            offset_1 = offset as U32;
            ZSTD_storeSeq(seqStore, 0i32 as size_t,
                          anchor as *const libc::c_void, 0i32 as U32,
                          matchLength.wrapping_sub(3i32 as libc::c_ulong));
            ip = ip.offset(matchLength as isize);
            anchor = ip
        }
    }
    *rep.offset(0isize) = offset_1;
    *rep.offset(1isize) = offset_2;
    return iend.wrapping_offset_from(anchor) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_HcFindBestMatch_extDict_selectMLS(mut ms:
                                                                *mut ZSTD_matchState_t,
                                                            mut ip:
                                                                *const BYTE,
                                                            iLimit:
                                                                *const BYTE,
                                                            mut offsetPtr:
                                                                *mut size_t)
 -> size_t {
    match (*ms).cParams.minMatch {
        5 => {
            return ZSTD_HcFindBestMatch_generic(ms, ip, iLimit, offsetPtr,
                                                5i32 as U32, ZSTD_extDict)
        }
        7 | 6 => {
            return ZSTD_HcFindBestMatch_generic(ms, ip, iLimit, offsetPtr,
                                                6i32 as U32, ZSTD_extDict)
        }
        4 | _ => {
            return ZSTD_HcFindBestMatch_generic(ms, ip, iLimit, offsetPtr,
                                                4i32 as U32, ZSTD_extDict)
        }
    };
}
unsafe extern "C" fn ZSTD_BtFindBestMatch_extDict_selectMLS(mut ms:
                                                                *mut ZSTD_matchState_t,
                                                            mut ip:
                                                                *const BYTE,
                                                            iLimit:
                                                                *const BYTE,
                                                            mut offsetPtr:
                                                                *mut size_t)
 -> size_t {
    match (*ms).cParams.minMatch {
        5 => {
            return ZSTD_BtFindBestMatch(ms, ip, iLimit, offsetPtr,
                                        5i32 as U32, ZSTD_extDict)
        }
        7 | 6 => {
            return ZSTD_BtFindBestMatch(ms, ip, iLimit, offsetPtr,
                                        6i32 as U32, ZSTD_extDict)
        }
        4 | _ => {
            return ZSTD_BtFindBestMatch(ms, ip, iLimit, offsetPtr,
                                        4i32 as U32, ZSTD_extDict)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_extDict(mut ms:
                                                             *mut ZSTD_matchState_t,
                                                         mut seqStore:
                                                             *mut seqStore_t,
                                                         mut rep: *mut U32,
                                                         mut src:
                                                             *const libc::c_void,
                                                         mut srcSize: size_t)
 -> size_t {
    return ZSTD_compressBlock_lazy_extDict_generic(ms, seqStore, rep, src,
                                                   srcSize, 0i32 as U32,
                                                   1i32 as U32);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_extDict(mut ms:
                                                              *mut ZSTD_matchState_t,
                                                          mut seqStore:
                                                              *mut seqStore_t,
                                                          mut rep: *mut U32,
                                                          mut src:
                                                              *const libc::c_void,
                                                          mut srcSize: size_t)
 -> size_t {
    return ZSTD_compressBlock_lazy_extDict_generic(ms, seqStore, rep, src,
                                                   srcSize, 0i32 as U32,
                                                   2i32 as U32);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btlazy2_extDict(mut ms:
                                                                *mut ZSTD_matchState_t,
                                                            mut seqStore:
                                                                *mut seqStore_t,
                                                            mut rep: *mut U32,
                                                            mut src:
                                                                *const libc::c_void,
                                                            mut srcSize:
                                                                size_t)
 -> size_t {
    return ZSTD_compressBlock_lazy_extDict_generic(ms, seqStore, rep, src,
                                                   srcSize, 1i32 as U32,
                                                   2i32 as U32);
}