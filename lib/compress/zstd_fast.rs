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
pub type ZSTD_dictTableLoadMethod_e = libc::c_uint;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
pub type ZSTD_dictMode_e = libc::c_uint;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
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
/* U32 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_fillHashTable(mut ms: *mut ZSTD_matchState_t,
                                            mut end: *const libc::c_void,
                                            mut dtlm:
                                                ZSTD_dictTableLoadMethod_e) {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable: *mut U32 = (*ms).hashTable;
    let hBits: U32 = (*cParams).hashLog;
    let mls: U32 = (*cParams).minMatch;
    let base: *const BYTE = (*ms).window.base;
    let mut ip: *const BYTE = base.offset((*ms).nextToUpdate as isize);
    let iend: *const BYTE = (end as *const BYTE).offset(-8isize);
    let fastHashFillStep: U32 = 3i32 as U32;
    while ip.offset(fastHashFillStep as isize) < iend.offset(2isize) {
        let current: U32 =
            ip.wrapping_offset_from(base) as libc::c_long as U32;
        let hash0: size_t =
            ZSTD_hashPtr(ip as *const libc::c_void, hBits, mls);
        *hashTable.offset(hash0 as isize) = current;
        if !(dtlm as libc::c_uint ==
                 ZSTD_dtlm_fast as libc::c_int as libc::c_uint) {
            let mut p: U32 = 0;
            p = 1i32 as U32;
            while p < fastHashFillStep {
                let hash: size_t =
                    ZSTD_hashPtr(ip.offset(p as isize) as *const libc::c_void,
                                 hBits, mls);
                if *hashTable.offset(hash as isize) == 0i32 as libc::c_uint {
                    *hashTable.offset(hash as isize) = current.wrapping_add(p)
                }
                p = p.wrapping_add(1)
            }
        }
        ip = ip.offset(fastHashFillStep as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_fast(mut ms:
                                                     *mut ZSTD_matchState_t,
                                                 mut seqStore:
                                                     *mut seqStore_t,
                                                 mut rep: *mut U32,
                                                 mut src: *const libc::c_void,
                                                 mut srcSize: size_t)
 -> size_t {
    let mut cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let mls: U32 = (*cParams).minMatch;
    match mls {
        5 => {
            return ZSTD_compressBlock_fast_generic(ms, seqStore, rep, src,
                                                   srcSize, 5i32 as U32,
                                                   ZSTD_noDict)
        }
        6 => {
            return ZSTD_compressBlock_fast_generic(ms, seqStore, rep, src,
                                                   srcSize, 6i32 as U32,
                                                   ZSTD_noDict)
        }
        7 => {
            return ZSTD_compressBlock_fast_generic(ms, seqStore, rep, src,
                                                   srcSize, 7i32 as U32,
                                                   ZSTD_noDict)
        }
        4 | _ => {
            return ZSTD_compressBlock_fast_generic(ms, seqStore, rep, src,
                                                   srcSize, 4i32 as U32,
                                                   ZSTD_noDict)
        }
    };
}
unsafe extern "C" fn ZSTD_compressBlock_fast_generic(mut ms:
                                                         *mut ZSTD_matchState_t,
                                                     mut seqStore:
                                                         *mut seqStore_t,
                                                     mut rep: *mut U32,
                                                     mut src:
                                                         *const libc::c_void,
                                                     mut srcSize: size_t,
                                                     mls: U32,
                                                     dictMode:
                                                         ZSTD_dictMode_e)
 -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable: *mut U32 = (*ms).hashTable;
    let hlog: U32 = (*cParams).hashLog;
    /* support stepSize of 0 */
    let stepSize: U32 =
        (*cParams).targetLength.wrapping_add((0 == (*cParams).targetLength) as
                                                 libc::c_int as libc::c_uint);
    let base: *const BYTE = (*ms).window.base;
    let istart: *const BYTE = src as *const BYTE;
    let mut ip: *const BYTE = istart;
    let mut anchor: *const BYTE = istart;
    let prefixStartIndex: U32 = (*ms).window.dictLimit;
    let prefixStart: *const BYTE = base.offset(prefixStartIndex as isize);
    let iend: *const BYTE = istart.offset(srcSize as isize);
    let ilimit: *const BYTE = iend.offset(-8isize);
    let mut offset_1: U32 = *rep.offset(0isize);
    let mut offset_2: U32 = *rep.offset(1isize);
    let mut offsetSaved: U32 = 0i32 as U32;
    let dms: *const ZSTD_matchState_t = (*ms).dictMatchState;
    let dictCParams: *const ZSTD_compressionParameters =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            &(*dms).cParams
        } else { 0 as *const ZSTD_compressionParameters };
    let dictHashTable: *const U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*dms).hashTable
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
            prefixStartIndex.wrapping_sub(dictEnd.wrapping_offset_from(dictBase)
                                              as libc::c_long as U32)
        } else { 0i32 as libc::c_uint };
    let dictAndPrefixLength: U32 =
        dictEnd.offset(ip.wrapping_offset_from(prefixStart) as libc::c_long as
                           isize).wrapping_offset_from(dictStart) as
            libc::c_long as U32;
    let dictHLog: U32 =
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint {
            (*dictCParams).hashLog
        } else { hlog };
    ip =
        ip.offset((dictAndPrefixLength == 0i32 as libc::c_uint) as libc::c_int
                      as isize);
    if dictMode as libc::c_uint == ZSTD_noDict as libc::c_int as libc::c_uint
       {
        let maxRep: U32 =
            ip.wrapping_offset_from(prefixStart) as libc::c_long as U32;
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
        /* < instead of <=, because repcode check at (ip+1) */
        let mut mLength: size_t = 0;
        let h: size_t = ZSTD_hashPtr(ip as *const libc::c_void, hlog, mls);
        let current: U32 =
            ip.wrapping_offset_from(base) as libc::c_long as U32;
        let matchIndex: U32 = *hashTable.offset(h as isize);
        let mut match_0: *const BYTE = base.offset(matchIndex as isize);
        let repIndex: U32 =
            current.wrapping_add(1i32 as libc::c_uint).wrapping_sub(offset_1);
        let mut repMatch: *const BYTE =
            if dictMode as libc::c_uint ==
                   ZSTD_dictMatchState as libc::c_int as libc::c_uint &&
                   repIndex < prefixStartIndex {
                dictBase.offset(repIndex.wrapping_sub(dictIndexDelta) as
                                    isize)
            } else { base.offset(repIndex as isize) };
        *hashTable.offset(h as isize) = current;
        if dictMode as libc::c_uint ==
               ZSTD_dictMatchState as libc::c_int as libc::c_uint &&
               prefixStartIndex.wrapping_sub(1i32 as
                                                 libc::c_uint).wrapping_sub(repIndex)
                   >= 3i32 as libc::c_uint &&
               MEM_read32(repMatch as *const libc::c_void) ==
                   MEM_read32(ip.offset(1isize) as *const libc::c_void) {
            let repMatchEnd: *const BYTE =
                if repIndex < prefixStartIndex { dictEnd } else { iend };
            mLength =
                ZSTD_count_2segments(ip.offset(1isize).offset(4isize),
                                     repMatch.offset(4isize), iend,
                                     repMatchEnd,
                                     prefixStart).wrapping_add(4i32 as
                                                                   libc::c_ulong);
            ip = ip.offset(1isize);
            ZSTD_storeSeq(seqStore,
                          ip.wrapping_offset_from(anchor) as libc::c_long as
                              size_t, anchor as *const libc::c_void,
                          0i32 as U32,
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
                          ip.wrapping_offset_from(anchor) as libc::c_long as
                              size_t, anchor as *const libc::c_void,
                          0i32 as U32,
                          mLength.wrapping_sub(3i32 as libc::c_ulong));
        } else if matchIndex <= prefixStartIndex {
            if dictMode as libc::c_uint ==
                   ZSTD_dictMatchState as libc::c_int as libc::c_uint {
                let dictHash: size_t =
                    ZSTD_hashPtr(ip as *const libc::c_void, dictHLog, mls);
                let dictMatchIndex: U32 =
                    *dictHashTable.offset(dictHash as isize);
                let mut dictMatch: *const BYTE =
                    dictBase.offset(dictMatchIndex as isize);
                if dictMatchIndex <= dictStartIndex ||
                       MEM_read32(dictMatch as *const libc::c_void) !=
                           MEM_read32(ip as *const libc::c_void) {
                    ip =
                        ip.offset(((ip.wrapping_offset_from(anchor) as
                                        libc::c_long >> 8i32) +
                                       stepSize as libc::c_long) as isize);
                    continue ;
                } else {
                    let offset: U32 =
                        current.wrapping_sub(dictMatchIndex).wrapping_sub(dictIndexDelta);
                    mLength =
                        ZSTD_count_2segments(ip.offset(4isize),
                                             dictMatch.offset(4isize), iend,
                                             dictEnd,
                                             prefixStart).wrapping_add(4i32 as
                                                                           libc::c_ulong);
                    while 0 !=
                              (ip > anchor) as libc::c_int &
                                  (dictMatch > dictStart) as libc::c_int &&
                              *ip.offset(-1i32 as isize) as libc::c_int ==
                                  *dictMatch.offset(-1i32 as isize) as
                                      libc::c_int {
                        ip = ip.offset(-1isize);
                        dictMatch = dictMatch.offset(-1isize);
                        mLength = mLength.wrapping_add(1)
                    }
                    offset_2 = offset_1;
                    offset_1 = offset;
                    ZSTD_storeSeq(seqStore,
                                  ip.wrapping_offset_from(anchor) as
                                      libc::c_long as size_t,
                                  anchor as *const libc::c_void,
                                  offset.wrapping_add((3i32 - 1i32) as
                                                          libc::c_uint),
                                  mLength.wrapping_sub(3i32 as
                                                           libc::c_ulong));
                }
            } else {
                ip =
                    ip.offset(((ip.wrapping_offset_from(anchor) as
                                    libc::c_long >> 8i32) +
                                   stepSize as libc::c_long) as isize);
                continue ;
            }
        } else if MEM_read32(match_0 as *const libc::c_void) !=
                      MEM_read32(ip as *const libc::c_void) {
            ip =
                ip.offset(((ip.wrapping_offset_from(anchor) as libc::c_long >>
                                8i32) + stepSize as libc::c_long) as isize);
            continue ;
        } else {
            let offset_0: U32 =
                ip.wrapping_offset_from(match_0) as libc::c_long as U32;
            mLength =
                ZSTD_count(ip.offset(4isize), match_0.offset(4isize),
                           iend).wrapping_add(4i32 as libc::c_ulong);
            while 0 !=
                      (ip > anchor) as libc::c_int &
                          (match_0 > prefixStart) as libc::c_int &&
                      *ip.offset(-1i32 as isize) as libc::c_int ==
                          *match_0.offset(-1i32 as isize) as libc::c_int {
                ip = ip.offset(-1isize);
                match_0 = match_0.offset(-1isize);
                mLength = mLength.wrapping_add(1)
            }
            offset_2 = offset_1;
            offset_1 = offset_0;
            ZSTD_storeSeq(seqStore,
                          ip.wrapping_offset_from(anchor) as libc::c_long as
                              size_t, anchor as *const libc::c_void,
                          offset_0.wrapping_add((3i32 - 1i32) as
                                                    libc::c_uint),
                          mLength.wrapping_sub(3i32 as libc::c_ulong));
        }
        ip = ip.offset(mLength as isize);
        anchor = ip;
        if ip <= ilimit {
            *hashTable.offset(ZSTD_hashPtr(base.offset(current as
                                                           isize).offset(2isize)
                                               as *const libc::c_void, hlog,
                                           mls) as isize) =
                current.wrapping_add(2i32 as libc::c_uint);
            *hashTable.offset(ZSTD_hashPtr(ip.offset(-2isize) as
                                               *const libc::c_void, hlog, mls)
                                  as isize) =
                ip.offset(-2isize).wrapping_offset_from(base) as libc::c_long
                    as U32;
            if dictMode as libc::c_uint ==
                   ZSTD_dictMatchState as libc::c_int as libc::c_uint {
                while ip <= ilimit {
                    let current2: U32 =
                        ip.wrapping_offset_from(base) as libc::c_long as U32;
                    let repIndex2: U32 = current2.wrapping_sub(offset_2);
                    let mut repMatch2: *const BYTE =
                        if repIndex2 < prefixStartIndex {
                            dictBase.offset(-(dictIndexDelta as
                                                  isize)).offset(repIndex2 as
                                                                     isize)
                        } else { base.offset(repIndex2 as isize) };
                    /* intentional overflow */
                    if !(prefixStartIndex.wrapping_sub(1i32 as
                                                           libc::c_uint).wrapping_sub(repIndex2)
                             >= 3i32 as libc::c_uint &&
                             MEM_read32(repMatch2 as *const libc::c_void) ==
                                 MEM_read32(ip as *const libc::c_void)) {
                        break ;
                    }
                    let repEnd2: *const BYTE =
                        if repIndex2 < prefixStartIndex {
                            dictEnd
                        } else { iend };
                    let repLength2: size_t =
                        ZSTD_count_2segments(ip.offset(4isize),
                                             repMatch2.offset(4isize), iend,
                                             repEnd2,
                                             prefixStart).wrapping_add(4i32 as
                                                                           libc::c_ulong);
                    /* swap offset_2 <=> offset_1 */
                    let mut tmpOffset: U32 = offset_2;
                    offset_2 = offset_1;
                    offset_1 = tmpOffset;
                    ZSTD_storeSeq(seqStore, 0i32 as size_t,
                                  anchor as *const libc::c_void, 0i32 as U32,
                                  repLength2.wrapping_sub(3i32 as
                                                              libc::c_ulong));
                    *hashTable.offset(ZSTD_hashPtr(ip as *const libc::c_void,
                                                   hlog, mls) as isize) =
                        current2;
                    ip = ip.offset(repLength2 as isize);
                    anchor = ip
                }
            }
            if dictMode as libc::c_uint ==
                   ZSTD_noDict as libc::c_int as libc::c_uint {
                while ip <= ilimit &&
                          0 !=
                              (offset_2 > 0i32 as libc::c_uint) as libc::c_int
                                  &
                                  (MEM_read32(ip as *const libc::c_void) ==
                                       MEM_read32(ip.offset(-(offset_2 as
                                                                  isize)) as
                                                      *const libc::c_void)) as
                                      libc::c_int {
                    /* store sequence */
                    let rLength: size_t =
                        ZSTD_count(ip.offset(4isize),
                                   ip.offset(4isize).offset(-(offset_2 as
                                                                  isize)),
                                   iend).wrapping_add(4i32 as libc::c_ulong);
                    /* swap offset_2 <=> offset_1 */
                    let tmpOff: U32 = offset_2;
                    offset_2 = offset_1;
                    offset_1 = tmpOff;
                    *hashTable.offset(ZSTD_hashPtr(ip as *const libc::c_void,
                                                   hlog, mls) as isize) =
                        ip.wrapping_offset_from(base) as libc::c_long as U32;
                    ZSTD_storeSeq(seqStore, 0i32 as size_t,
                                  anchor as *const libc::c_void, 0i32 as U32,
                                  rLength.wrapping_sub(3i32 as
                                                           libc::c_ulong));
                    ip = ip.offset(rLength as isize);
                    anchor = ip
                }
            }
        }
    }
    *rep.offset(0isize) = if 0 != offset_1 { offset_1 } else { offsetSaved };
    *rep.offset(1isize) = if 0 != offset_2 { offset_2 } else { offsetSaved };
    return iend.wrapping_offset_from(anchor) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_fast_dictMatchState(mut ms:
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
    let mut cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let mls: U32 = (*cParams).minMatch;
    match mls {
        5 => {
            return ZSTD_compressBlock_fast_generic(ms, seqStore, rep, src,
                                                   srcSize, 5i32 as U32,
                                                   ZSTD_dictMatchState)
        }
        6 => {
            return ZSTD_compressBlock_fast_generic(ms, seqStore, rep, src,
                                                   srcSize, 6i32 as U32,
                                                   ZSTD_dictMatchState)
        }
        7 => {
            return ZSTD_compressBlock_fast_generic(ms, seqStore, rep, src,
                                                   srcSize, 7i32 as U32,
                                                   ZSTD_dictMatchState)
        }
        4 | _ => {
            return ZSTD_compressBlock_fast_generic(ms, seqStore, rep, src,
                                                   srcSize, 4i32 as U32,
                                                   ZSTD_dictMatchState)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_fast_extDict(mut ms:
                                                             *mut ZSTD_matchState_t,
                                                         mut seqStore:
                                                             *mut seqStore_t,
                                                         mut rep: *mut U32,
                                                         mut src:
                                                             *const libc::c_void,
                                                         mut srcSize: size_t)
 -> size_t {
    let mut cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let mls: U32 = (*cParams).minMatch;
    match mls {
        5 => {
            return ZSTD_compressBlock_fast_extDict_generic(ms, seqStore, rep,
                                                           src, srcSize,
                                                           5i32 as U32)
        }
        6 => {
            return ZSTD_compressBlock_fast_extDict_generic(ms, seqStore, rep,
                                                           src, srcSize,
                                                           6i32 as U32)
        }
        7 => {
            return ZSTD_compressBlock_fast_extDict_generic(ms, seqStore, rep,
                                                           src, srcSize,
                                                           7i32 as U32)
        }
        4 | _ => {
            return ZSTD_compressBlock_fast_extDict_generic(ms, seqStore, rep,
                                                           src, srcSize,
                                                           4i32 as U32)
        }
    };
}
unsafe extern "C" fn ZSTD_compressBlock_fast_extDict_generic(mut ms:
                                                                 *mut ZSTD_matchState_t,
                                                             mut seqStore:
                                                                 *mut seqStore_t,
                                                             mut rep:
                                                                 *mut U32,
                                                             mut src:
                                                                 *const libc::c_void,
                                                             mut srcSize:
                                                                 size_t,
                                                             mls: U32)
 -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable: *mut U32 = (*ms).hashTable;
    let hlog: U32 = (*cParams).hashLog;
    /* support stepSize of 0 */
    let stepSize: U32 =
        (*cParams).targetLength.wrapping_add((0 == (*cParams).targetLength) as
                                                 libc::c_int as libc::c_uint);
    let base: *const BYTE = (*ms).window.base;
    let dictBase: *const BYTE = (*ms).window.dictBase;
    let istart: *const BYTE = src as *const BYTE;
    let mut ip: *const BYTE = istart;
    let mut anchor: *const BYTE = istart;
    let dictStartIndex: U32 = (*ms).window.lowLimit;
    let dictStart: *const BYTE = dictBase.offset(dictStartIndex as isize);
    let prefixStartIndex: U32 = (*ms).window.dictLimit;
    let prefixStart: *const BYTE = base.offset(prefixStartIndex as isize);
    let dictEnd: *const BYTE = dictBase.offset(prefixStartIndex as isize);
    let iend: *const BYTE = istart.offset(srcSize as isize);
    let ilimit: *const BYTE = iend.offset(-8isize);
    let mut offset_1: U32 = *rep.offset(0isize);
    let mut offset_2: U32 = *rep.offset(1isize);
    while ip < ilimit {
        /* < instead of <=, because (ip+1) */
        let h: size_t = ZSTD_hashPtr(ip as *const libc::c_void, hlog, mls);
        let matchIndex: U32 = *hashTable.offset(h as isize);
        let matchBase: *const BYTE =
            if matchIndex < prefixStartIndex { dictBase } else { base };
        let mut match_0: *const BYTE = matchBase.offset(matchIndex as isize);
        let current: U32 =
            ip.wrapping_offset_from(base) as libc::c_long as U32;
        let repIndex: U32 =
            current.wrapping_add(1i32 as libc::c_uint).wrapping_sub(offset_1);
        let repBase: *const BYTE =
            if repIndex < prefixStartIndex { dictBase } else { base };
        let repMatch: *const BYTE = repBase.offset(repIndex as isize);
        let mut mLength: size_t = 0;
        *hashTable.offset(h as isize) = current;
        /* intentional underflow */
        if 0 !=
               (prefixStartIndex.wrapping_sub(1i32 as
                                                  libc::c_uint).wrapping_sub(repIndex)
                    >= 3i32 as libc::c_uint) as libc::c_int &
                   (repIndex > dictStartIndex) as libc::c_int &&
               MEM_read32(repMatch as *const libc::c_void) ==
                   MEM_read32(ip.offset(1isize) as *const libc::c_void) {
            let mut repMatchEnd: *const BYTE =
                if repIndex < prefixStartIndex { dictEnd } else { iend };
            mLength =
                ZSTD_count_2segments(ip.offset(1isize).offset(4isize),
                                     repMatch.offset(4isize), iend,
                                     repMatchEnd,
                                     prefixStart).wrapping_add(4i32 as
                                                                   libc::c_ulong);
            ip = ip.offset(1isize);
            ZSTD_storeSeq(seqStore,
                          ip.wrapping_offset_from(anchor) as libc::c_long as
                              size_t, anchor as *const libc::c_void,
                          0i32 as U32,
                          mLength.wrapping_sub(3i32 as libc::c_ulong));
        } else if matchIndex < dictStartIndex ||
                      MEM_read32(match_0 as *const libc::c_void) !=
                          MEM_read32(ip as *const libc::c_void) {
            ip =
                ip.offset(((ip.wrapping_offset_from(anchor) as libc::c_long >>
                                8i32) + stepSize as libc::c_long) as isize);
            continue ;
        } else {
            let mut matchEnd: *const BYTE =
                if matchIndex < prefixStartIndex { dictEnd } else { iend };
            let mut lowMatchPtr: *const BYTE =
                if matchIndex < prefixStartIndex {
                    dictStart
                } else { prefixStart };
            let mut offset: U32 = 0;
            mLength =
                ZSTD_count_2segments(ip.offset(4isize),
                                     match_0.offset(4isize), iend, matchEnd,
                                     prefixStart).wrapping_add(4i32 as
                                                                   libc::c_ulong);
            while 0 !=
                      (ip > anchor) as libc::c_int &
                          (match_0 > lowMatchPtr) as libc::c_int &&
                      *ip.offset(-1i32 as isize) as libc::c_int ==
                          *match_0.offset(-1i32 as isize) as libc::c_int {
                ip = ip.offset(-1isize);
                match_0 = match_0.offset(-1isize);
                mLength = mLength.wrapping_add(1)
            }
            offset = current.wrapping_sub(matchIndex);
            offset_2 = offset_1;
            offset_1 = offset;
            ZSTD_storeSeq(seqStore,
                          ip.wrapping_offset_from(anchor) as libc::c_long as
                              size_t, anchor as *const libc::c_void,
                          offset.wrapping_add((3i32 - 1i32) as libc::c_uint),
                          mLength.wrapping_sub(3i32 as libc::c_ulong));
        }
        ip = ip.offset(mLength as isize);
        anchor = ip;
        if ip <= ilimit {
            *hashTable.offset(ZSTD_hashPtr(base.offset(current as
                                                           isize).offset(2isize)
                                               as *const libc::c_void, hlog,
                                           mls) as isize) =
                current.wrapping_add(2i32 as libc::c_uint);
            *hashTable.offset(ZSTD_hashPtr(ip.offset(-2isize) as
                                               *const libc::c_void, hlog, mls)
                                  as isize) =
                ip.offset(-2isize).wrapping_offset_from(base) as libc::c_long
                    as U32;
            while ip <= ilimit {
                let current2: U32 =
                    ip.wrapping_offset_from(base) as libc::c_long as U32;
                let repIndex2: U32 = current2.wrapping_sub(offset_2);
                let mut repMatch2: *const BYTE =
                    if repIndex2 < prefixStartIndex {
                        dictBase.offset(repIndex2 as isize)
                    } else { base.offset(repIndex2 as isize) };
                /* intentional overflow */
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
                                         repMatch2.offset(4isize), iend,
                                         repEnd2,
                                         prefixStart).wrapping_add(4i32 as
                                                                       libc::c_ulong);
                /* swap offset_2 <=> offset_1 */
                let mut tmpOffset: U32 = offset_2;
                offset_2 = offset_1;
                offset_1 = tmpOffset;
                ZSTD_storeSeq(seqStore, 0i32 as size_t,
                              anchor as *const libc::c_void, 0i32 as U32,
                              repLength2.wrapping_sub(3i32 as libc::c_ulong));
                *hashTable.offset(ZSTD_hashPtr(ip as *const libc::c_void,
                                               hlog, mls) as isize) =
                    current2;
                ip = ip.offset(repLength2 as isize);
                anchor = ip
            }
        }
    }
    *rep.offset(0isize) = offset_1;
    *rep.offset(1isize) = offset_2;
    return iend.wrapping_offset_from(anchor) as libc::c_long as size_t;
}