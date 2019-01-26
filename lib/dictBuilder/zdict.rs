#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc, ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    pub type HUF_CElt_s;
    /* **************************************
*  Explicit context
***************************************/
/*= Compression context
 *  When compressing many times,
 *  it is recommended to allocate a context just once, and re-use it for each successive compression operation.
 *  This will make workload friendlier for system's memory.
 *  Use one context per thread for parallel execution in multi-threaded environments. */
    pub type ZSTD_CCtx_s;
    /* **********************************
 *  Bulk processing dictionary API
 **********************************/
    pub type ZSTD_CDict_s;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
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
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn clock() -> clock_t;
    /* ! FSE_normalizeCount():
    normalize counts so that sum(count[]) == Power_of_2 (2^tableLog)
    'normalizedCounter' is a table of short, of minimum size (maxSymbolValue+1).
    @return : tableLog,
              or an errorCode, which can be tested using FSE_isError() */
    #[no_mangle]
    fn FSE_normalizeCount(normalizedCounter: *mut libc::c_short,
                          tableLog: libc::c_uint, count: *const libc::c_uint,
                          srcSize: size_t, maxSymbolValue: libc::c_uint)
     -> size_t;
    /* ! FSE_writeNCount():
    Compactly save 'normalizedCounter' into 'buffer'.
    @return : size of the compressed table,
              or an errorCode, which can be tested using FSE_isError(). */
    #[no_mangle]
    fn FSE_writeNCount(buffer: *mut libc::c_void, bufferSize: size_t,
                       normalizedCounter: *const libc::c_short,
                       maxSymbolValue: libc::c_uint, tableLog: libc::c_uint)
     -> size_t;
    #[no_mangle]
    fn HUF_buildCTable(CTable: *mut HUF_CElt, count: *const libc::c_uint,
                       maxSymbolValue: libc::c_uint, maxNbBits: libc::c_uint)
     -> size_t;
    #[no_mangle]
    fn HUF_writeCTable(dst: *mut libc::c_void, maxDstSize: size_t,
                       CTable: *const HUF_CElt, maxSymbolValue: libc::c_uint,
                       huffLog: libc::c_uint) -> size_t;
    /*-****************************************
*  Error Strings
******************************************/
    #[no_mangle]
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
    #[no_mangle]
    fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
    #[no_mangle]
    fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> size_t;
    /* ! ZSTD_freeCDict() :
 *  Function frees memory allocated by ZSTD_createCDict(). */
    #[no_mangle]
    fn ZSTD_freeCDict(CDict: *mut ZSTD_CDict) -> size_t;
    #[no_mangle]
    fn ZSTD_createCDict_advanced(dict: *const libc::c_void, dictSize: size_t,
                                 dictLoadMethod: ZSTD_dictLoadMethod_e,
                                 dictContentType: ZSTD_dictContentType_e,
                                 cParams: ZSTD_compressionParameters,
                                 customMem: ZSTD_customMem)
     -> *mut ZSTD_CDict;
    /* ! ZSTD_getParams() :
 *  same as ZSTD_getCParams(), but @return a full `ZSTD_parameters` object instead of sub-component `ZSTD_compressionParameters`.
 *  All fields of `ZSTD_frameParameters` are set to default : contentSize=1, checksum=0, noDictID=0 */
    #[no_mangle]
    fn ZSTD_getParams(compressionLevel: libc::c_int,
                      estimatedSrcSize: libc::c_ulonglong, dictSize: size_t)
     -> ZSTD_parameters;
    #[no_mangle]
    fn ZSTD_compressBegin_usingCDict(cctx: *mut ZSTD_CCtx,
                                     cdict: *const ZSTD_CDict) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock(cctx: *mut ZSTD_CCtx, dst: *mut libc::c_void,
                          dstCapacity: size_t, src: *const libc::c_void,
                          srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_XXH64(input: *const libc::c_void, length: size_t,
                  seed: libc::c_ulonglong) -> XXH64_hash_t;
    #[no_mangle]
    fn ZSTD_getSeqStore(ctx: *const ZSTD_CCtx) -> *const seqStore_t;
    #[no_mangle]
    fn ZSTD_seqToCodes(seqStorePtr: *const seqStore_t);
    /*
 * divsufsort.h for libdivsufsort-lite
 * Copyright (c) 2003-2008 Yuta Mori All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation
 * files (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use,
 * copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following
 * conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 * OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
 * HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */
    /* __cplusplus */
    /*- Prototypes -*/
    /* *
 * Constructs the suffix array of a given string.
 * @param T [0..n-1] The input string.
 * @param SA [0..n-1] The output array of suffixes.
 * @param n The length of the given string.
 * @param openMP enables OpenMP optimization.
 * @return 0 if no error occurred, -1 or -2 otherwise.
 */
    #[no_mangle]
    fn divsufsort(T: *const libc::c_uchar, SA: *mut libc::c_int,
                  n: libc::c_int, openMP: libc::c_int) -> libc::c_int;
    /* ! ZDICT_optimizeTrainFromBuffer_fastCover():
 * The same requirements as above hold for all the parameters except `parameters`.
 * This function tries many parameter combinations (specifically, k and d combinations)
 * and picks the best parameters. `*parameters` is filled with the best parameters found,
 * dictionary constructed with those parameters is stored in `dictBuffer`.
 * All of the parameters d, k, steps, f, and accel are optional.
 * If d is non-zero then we don't check multiple values of d, otherwise we check d = {6, 8}.
 * if steps is zero it defaults to its default value.
 * If k is non-zero then we don't check multiple values of k, otherwise we check steps values in [50, 2000].
 * If f is zero, default value of 20 is used.
 * If accel is zero, default value of 1 is used.
 *
 * @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)
 *           or an error code, which can be tested with ZDICT_isError().
 *           On success `*parameters` contains the parameters selected.
 * Note: ZDICT_optimizeTrainFromBuffer_fastCover() requires about 1 byte of memory for each input byte and additionally another 6 * 2^f bytes of memory for each thread.
 */
    #[no_mangle]
    fn ZDICT_optimizeTrainFromBuffer_fastCover(dictBuffer: *mut libc::c_void,
                                               dictBufferCapacity: size_t,
                                               samplesBuffer:
                                                   *const libc::c_void,
                                               samplesSizes: *const size_t,
                                               nbSamples: libc::c_uint,
                                               parameters:
                                                   *mut ZDICT_fastCover_params_t)
     -> size_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr(C)]
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
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type clock_t = __clock_t;
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
/* incomplete type */
pub type HUF_CElt = HUF_CElt_s;
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
pub type ZSTD_ErrorCode = libc::c_uint;
/* never EVER use this value directly, it can change in future versions! Use ZSTD_isError() instead */
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
/* following error codes are __NOT STABLE__, they can be removed or changed in future versions */
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
pub const ZSTD_error_dstBuffer_null: ZSTD_ErrorCode = 74;
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* Note : this module is expected to remain private, do not expose it */
/* ****************************************
*  Dependencies
******************************************/
/* size_t */
/* ****************************************
*  Compiler-specific
******************************************/
/* C99 */
/*-****************************************
*  Customization (error_public.h)
******************************************/
pub type ERR_enum = ZSTD_ErrorCode;
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type ZSTD_CDict = ZSTD_CDict_s;
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
pub struct ZSTD_compressionParameters {
    pub windowLog: libc::c_uint,
    pub chainLog: libc::c_uint,
    pub hashLog: libc::c_uint,
    pub searchLog: libc::c_uint,
    pub minMatch: libc::c_uint,
    pub targetLength: libc::c_uint,
    pub strategy: ZSTD_strategy,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_frameParameters {
    pub contentSizeFlag: libc::c_int,
    pub checksumFlag: libc::c_int,
    pub noDictIDFlag: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_parameters {
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
}
pub type ZSTD_dictContentType_e = libc::c_uint;
/* refuses to load a dictionary if it does not respect Zstandard's specification, starting with ZSTD_MAGIC_DICTIONARY */
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
/* ensures dictionary is always loaded as rawContent, even if it starts with ZSTD_MAGIC_DICTIONARY */
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
/* dictionary is "full" when starting with ZSTD_MAGIC_DICTIONARY, otherwise it is "rawContent" */
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;
pub type ZSTD_dictLoadMethod_e = libc::c_uint;
/* *< Reference dictionary content -- the dictionary buffer must outlive its users. */
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
/* *< Copy dictionary content internally */
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
/* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
pub type ZSTD_allocFunction
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
pub type ZSTD_freeFunction
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
pub type XXH64_hash_t = libc::c_ulonglong;
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
pub type seqDef = seqDef_s;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZDICT_fastCover_params_t {
    pub k: libc::c_uint,
    pub d: libc::c_uint,
    pub f: libc::c_uint,
    pub steps: libc::c_uint,
    pub nbThreads: libc::c_uint,
    pub splitPoint: libc::c_double,
    pub accel: libc::c_uint,
    pub zParams: ZDICT_params_t,
}
/* ====================================================================================
 * The definitions in this section are considered experimental.
 * They should never be used with a dynamic library, as they may change in the future.
 * They are provided for advanced usages.
 * Use them only in association with static linking.
 * ==================================================================================== */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZDICT_params_t {
    pub compressionLevel: libc::c_int,
    pub notificationLevel: libc::c_uint,
    pub dictID: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct EStats_ress_t {
    pub dict: *mut ZSTD_CDict,
    pub zc: *mut ZSTD_CCtx,
    pub workPlace: *mut libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct offsetCount_t {
    pub offset: U32,
    pub count: U32,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZDICT_legacy_params_t {
    pub selectivityLevel: libc::c_uint,
    pub zParams: ZDICT_params_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct dictItem {
    pub pos: U32,
    pub length: U32,
    pub savings: U32,
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
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void,
                                 mut value: U32) {
    (*(memPtr as *mut unalign32)).v = value;
}
/* MEM_FORCE_MEMORY_ACCESS */
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
}
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const libc::c_void) -> U32 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read32(memPtr)
    } else { return MEM_swap32(MEM_read32(memPtr)) };
}
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut libc::c_void,
                                   mut val32: U32) {
    if 0 != MEM_isLittleEndian() {
        MEM_write32(memPtr, val32);
    } else { MEM_write32(memPtr, MEM_swap32(val32)); };
}
/*-****************************************
*  Error codes handling
******************************************/
/* reported already defined on VS 2015 (Rich Geldreich) */
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as
               libc::c_int as libc::c_uint;
}
unsafe extern "C" fn ERR_getErrorCode(mut code: size_t) -> ERR_enum {
    if 0 == ERR_isError(code) { return ZSTD_error_no_error }
    return (0i32 as libc::c_ulong).wrapping_sub(code) as ERR_enum;
}
unsafe extern "C" fn ERR_getErrorName(mut code: size_t)
 -> *const libc::c_char {
    return ERR_getErrorString(ERR_getErrorCode(code));
}
/* *< this constant defers to stdlib's functions */
static mut ZSTD_defaultCMem: ZSTD_customMem =
    ZSTD_customMem{customAlloc: None,
                   customFree: None,
                   opaque: 0 as *const libc::c_void as *mut libc::c_void,};
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* this module contains definitions which must be identical
 * across compression, decompression and dictBuilder.
 * It also contains a few functions useful to at least 2 of them
 * and which benefit from being inlined */
/*-*************************************
*  Dependencies
***************************************/
/* XXH64_state_t */
/* ---- static assert (debug) --- */
/* for inlining */
/*-*************************************
*  shared macros
***************************************/
/* check and Forward error code */
/* check and send Error code */
/*-*************************************
*  Common constants
***************************************/
/* number of repcodes */
static mut repStartValue: [U32; 3] = [1i32 as U32, 4i32 as U32, 8i32 as U32];
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> U32 {
    return (31i32 - val.leading_zeros() as i32) as U32;
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
/*======  Dependencies  ======*/
/* size_t */
/* =====   ZDICTLIB_API : control library symbols visibility   ===== */
/* ! ZDICT_trainFromBuffer():
 *  Train a dictionary from an array of samples.
 *  Redirect towards ZDICT_optimizeTrainFromBuffer_fastCover() single-threaded, with d=8, steps=4,
 *  f=20, and accel=1.
 *  Samples must be stored concatenated in a single flat buffer `samplesBuffer`,
 *  supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order.
 *  The resulting dictionary will be saved into `dictBuffer`.
 * @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)
 *          or an error code, which can be tested with ZDICT_isError().
 *  Note: ZDICT_trainFromBuffer() requires about 9 bytes of memory for each input byte.
 *  Tips: In general, a reasonable dictionary has a size of ~ 100 KB.
 *        It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`.
 *        In general, it's recommended to provide a few thousands samples, though this can vary a lot.
 *        It's recommended that total size of all samples be about ~x100 times the target size of dictionary.
 */
#[no_mangle]
pub unsafe extern "C" fn ZDICT_trainFromBuffer(mut dictBuffer:
                                                   *mut libc::c_void,
                                               mut dictBufferCapacity: size_t,
                                               mut samplesBuffer:
                                                   *const libc::c_void,
                                               mut samplesSizes:
                                                   *const size_t,
                                               mut nbSamples: libc::c_uint)
 -> size_t {
    let mut params: ZDICT_fastCover_params_t =
        ZDICT_fastCover_params_t{k: 0,
                                 d: 0,
                                 f: 0,
                                 steps: 0,
                                 nbThreads: 0,
                                 splitPoint: 0.,
                                 accel: 0,
                                 zParams:
                                     ZDICT_params_t{compressionLevel: 0,
                                                    notificationLevel: 0,
                                                    dictID: 0,},};
    memset(&mut params as *mut ZDICT_fastCover_params_t as *mut libc::c_void,
           0i32,
           ::std::mem::size_of::<ZDICT_fastCover_params_t>() as
               libc::c_ulong);
    params.d = 8i32 as libc::c_uint;
    params.steps = 4i32 as libc::c_uint;
    params.zParams.compressionLevel = 3i32;
    return ZDICT_optimizeTrainFromBuffer_fastCover(dictBuffer,
                                                   dictBufferCapacity,
                                                   samplesBuffer,
                                                   samplesSizes, nbSamples,
                                                   &mut params);
}
/*======   Helper functions   ======*/
/* *< extracts dictID; @return zero if error (not a valid dictionary) */
#[no_mangle]
pub unsafe extern "C" fn ZDICT_getDictID(mut dictBuffer: *const libc::c_void,
                                         mut dictSize: size_t)
 -> libc::c_uint {
    if dictSize < 8i32 as libc::c_ulong { return 0i32 as libc::c_uint }
    if MEM_readLE32(dictBuffer) != 0xec30a437u32 {
        return 0i32 as libc::c_uint
    }
    return MEM_readLE32((dictBuffer as *const libc::c_char).offset(4isize) as
                            *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_isError(mut errorCode: size_t)
 -> libc::c_uint {
    return ERR_isError(errorCode);
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_getErrorName(mut errorCode: size_t)
 -> *const libc::c_char {
    return ERR_getErrorName(errorCode);
}
/* ! ZDICT_finalizeDictionary():
 * Given a custom content as a basis for dictionary, and a set of samples,
 * finalize dictionary by adding headers and statistics.
 *
 * Samples must be stored concatenated in a flat buffer `samplesBuffer`,
 * supplied with an array of sizes `samplesSizes`, providing the size of each sample in order.
 *
 * dictContentSize must be >= ZDICT_CONTENTSIZE_MIN bytes.
 * maxDictSize must be >= dictContentSize, and must be >= ZDICT_DICTSIZE_MIN bytes.
 *
 * @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`),
 *           or an error code, which can be tested by ZDICT_isError().
 * Note: ZDICT_finalizeDictionary() will push notifications into stderr if instructed to, using notificationLevel>0.
 * Note 2: dictBuffer and dictContent can overlap
 */
#[no_mangle]
pub unsafe extern "C" fn ZDICT_finalizeDictionary(mut dictBuffer:
                                                      *mut libc::c_void,
                                                  mut dictBufferCapacity:
                                                      size_t,
                                                  mut customDictContent:
                                                      *const libc::c_void,
                                                  mut dictContentSize: size_t,
                                                  mut samplesBuffer:
                                                      *const libc::c_void,
                                                  mut samplesSizes:
                                                      *const size_t,
                                                  mut nbSamples: libc::c_uint,
                                                  mut params: ZDICT_params_t)
 -> size_t {
    let mut hSize: size_t = 0;
    /* should prove large enough for all entropy headers */
    let mut header: [BYTE; 256] = [0; 256];
    let compressionLevel: libc::c_int =
        if params.compressionLevel == 0i32 {
            g_compressionLevel_default
        } else { params.compressionLevel };
    let notificationLevel: U32 = params.notificationLevel;
    if dictBufferCapacity < dictContentSize {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    if dictContentSize < 128i32 as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    if dictBufferCapacity < 256i32 as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    MEM_writeLE32(header.as_mut_ptr() as *mut libc::c_void, 0xec30a437u32);
    let randomID: U64 =
        ZSTD_XXH64(customDictContent, dictContentSize,
                   0i32 as libc::c_ulonglong) as U64;
    let compliantID: U32 =
        randomID.wrapping_rem((1u32 <<
                                   31i32).wrapping_sub(32768i32 as
                                                           libc::c_uint) as
                                  libc::c_ulong).wrapping_add(32768i32 as
                                                                  libc::c_ulong)
            as U32;
    let dictID: U32 =
        if 0 != params.dictID { params.dictID } else { compliantID };
    MEM_writeLE32(header.as_mut_ptr().offset(4isize) as *mut libc::c_void,
                  dictID);
    hSize = 8i32 as size_t;
    if notificationLevel >= 2i32 as libc::c_uint {
        fprintf(stderr, b"\r%70s\r\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char);
        fflush(stderr);
    }
    if notificationLevel >= 2i32 as libc::c_uint {
        fprintf(stderr,
                b"statistics ... \n\x00" as *const u8 as *const libc::c_char);
        fflush(stderr);
    }
    let eSize: size_t =
        ZDICT_analyzeEntropy(header.as_mut_ptr().offset(hSize as isize) as
                                 *mut libc::c_void,
                             (256i32 as libc::c_ulong).wrapping_sub(hSize),
                             compressionLevel as libc::c_uint, samplesBuffer,
                             samplesSizes, nbSamples, customDictContent,
                             dictContentSize, notificationLevel);
    if 0 != ZDICT_isError(eSize) { return eSize }
    hSize = (hSize as libc::c_ulong).wrapping_add(eSize) as size_t as size_t;
    if hSize.wrapping_add(dictContentSize) > dictBufferCapacity {
        dictContentSize = dictBufferCapacity.wrapping_sub(hSize)
    }
    let dictSize: size_t = hSize.wrapping_add(dictContentSize);
    let mut dictEnd: *mut libc::c_char =
        (dictBuffer as *mut libc::c_char).offset(dictSize as isize);
    memmove(dictEnd.offset(-(dictContentSize as isize)) as *mut libc::c_void,
            customDictContent, dictContentSize);
    memcpy(dictBuffer, header.as_mut_ptr() as *const libc::c_void, hSize);
    return dictSize;
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
/*-**************************************
*  Tuning parameters
****************************************/
/* minimum nb of apparition to be selected in dictionary */
/*-**************************************
*  Compiler Options
****************************************/
/* Unix Large Files support (>4GB) */
/* Sun Solaris 32-bits requires specific definitions */
/* No point defining Large file for 64 bit */
/*-*************************************
*  Dependencies
***************************************/
/* malloc, free */
/* memset */
/* fprintf, fopen, ftello64 */
/* clock */
/*-*************************************
*  Constants
***************************************/
static mut g_compressionLevel_default: libc::c_int = 3i32;
/* only applicable to first block */
unsafe extern "C" fn ZDICT_analyzeEntropy(mut dstBuffer: *mut libc::c_void,
                                          mut maxDstSize: size_t,
                                          mut compressionLevel: libc::c_uint,
                                          mut srcBuffer: *const libc::c_void,
                                          mut fileSizes: *const size_t,
                                          mut nbFiles: libc::c_uint,
                                          mut dictBuffer: *const libc::c_void,
                                          mut dictBufferSize: size_t,
                                          mut notificationLevel: libc::c_uint)
 -> size_t {
    let mut countLit: [libc::c_uint; 256] = [0; 256];
    let mut hufTablehb: [U32; 256] = [0; 256];
    let mut hufTablehv: *mut libc::c_void =
        &mut hufTablehb as *mut [U32; 256] as *mut libc::c_void;
    let mut hufTable: *mut HUF_CElt = hufTablehv as *mut HUF_CElt;
    let mut offcodeCount: [libc::c_uint; 31] = [0; 31];
    let mut offcodeNCount: [libc::c_short; 31] = [0; 31];
    let mut offcodeMax: U32 =
        ZSTD_highbit32(dictBufferSize.wrapping_add((128i32 * (1i32 << 10i32))
                                                       as libc::c_ulong) as
                           U32);
    let mut matchLengthCount: [libc::c_uint; 53] = [0; 53];
    let mut matchLengthNCount: [libc::c_short; 53] = [0; 53];
    let mut litLengthCount: [libc::c_uint; 36] = [0; 36];
    let mut litLengthNCount: [libc::c_short; 36] = [0; 36];
    let mut repOffset: [U32; 1024] = [0; 1024];
    let mut bestRepOffset: [offsetCount_t; 4] =
        [offsetCount_t{offset: 0, count: 0,}; 4];
    let mut esr: EStats_ress_t =
        EStats_ress_t{dict: 0 as *mut ZSTD_CDict,
                      zc: 0 as *mut ZSTD_CCtx,
                      workPlace: 0 as *mut libc::c_void,};
    let mut params: ZSTD_parameters =
        ZSTD_parameters{cParams:
                            ZSTD_compressionParameters{windowLog: 0,
                                                       chainLog: 0,
                                                       hashLog: 0,
                                                       searchLog: 0,
                                                       minMatch: 0,
                                                       targetLength: 0,
                                                       strategy:
                                                           0 as
                                                               ZSTD_strategy,},
                        fParams:
                            ZSTD_frameParameters{contentSizeFlag: 0,
                                                 checksumFlag: 0,
                                                 noDictIDFlag: 0,},};
    let mut u: U32 = 0;
    let mut huffLog: U32 = 11i32 as U32;
    let mut Offlog: U32 = 8i32 as U32;
    let mut mlLog: U32 = 9i32 as U32;
    let mut llLog: U32 = 9i32 as U32;
    let mut total: U32 = 0;
    let mut pos: size_t = 0i32 as size_t;
    let mut errorCode: size_t = 0;
    let mut eSize: size_t = 0i32 as size_t;
    let totalSrcSize: size_t = ZDICT_totalSampleSize(fileSizes, nbFiles);
    let averageSampleSize: size_t =
        totalSrcSize.wrapping_div(nbFiles.wrapping_add((0 == nbFiles) as
                                                           libc::c_int as
                                                           libc::c_uint) as
                                      libc::c_ulong);
    let mut dstPtr: *mut BYTE = dstBuffer as *mut BYTE;
    if offcodeMax > 30i32 as libc::c_uint {
        eSize =
            -(ZSTD_error_dictionaryCreation_failed as libc::c_int) as size_t
    } else {
        /* too large dictionary */
        u = 0i32 as U32;
        while u < 256i32 as libc::c_uint {
            countLit[u as usize] = 1i32 as libc::c_uint;
            u = u.wrapping_add(1)
        }
        u = 0i32 as U32;
        while u <= offcodeMax {
            offcodeCount[u as usize] = 1i32 as libc::c_uint;
            u = u.wrapping_add(1)
        }
        u = 0i32 as U32;
        while u <= 52i32 as libc::c_uint {
            matchLengthCount[u as usize] = 1i32 as libc::c_uint;
            u = u.wrapping_add(1)
        }
        u = 0i32 as U32;
        while u <= 35i32 as libc::c_uint {
            litLengthCount[u as usize] = 1i32 as libc::c_uint;
            u = u.wrapping_add(1)
        }
        memset(repOffset.as_mut_ptr() as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<[U32; 1024]>() as libc::c_ulong);
        repOffset[8usize] = 1i32 as U32;
        repOffset[4usize] = repOffset[8usize];
        repOffset[1usize] = repOffset[4usize];
        memset(bestRepOffset.as_mut_ptr() as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<[offsetCount_t; 4]>() as libc::c_ulong);
        if compressionLevel == 0i32 as libc::c_uint {
            compressionLevel = g_compressionLevel_default as libc::c_uint
        }
        params =
            ZSTD_getParams(compressionLevel as libc::c_int,
                           averageSampleSize as libc::c_ulonglong,
                           dictBufferSize);
        esr.dict =
            ZSTD_createCDict_advanced(dictBuffer, dictBufferSize,
                                      ZSTD_dlm_byRef, ZSTD_dct_rawContent,
                                      params.cParams, ZSTD_defaultCMem);
        esr.zc = ZSTD_createCCtx();
        esr.workPlace = malloc((1i32 << 17i32) as libc::c_ulong);
        if esr.dict.is_null() || esr.zc.is_null() || esr.workPlace.is_null() {
            eSize = -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
            if notificationLevel >= 1i32 as libc::c_uint {
                fprintf(stderr,
                        b"Not enough memory \n\x00" as *const u8 as
                            *const libc::c_char);
                fflush(stderr);
            }
        } else {
            u = 0i32 as U32;
            while u < nbFiles {
                ZDICT_countEStats(esr, params, countLit.as_mut_ptr(),
                                  offcodeCount.as_mut_ptr(),
                                  matchLengthCount.as_mut_ptr(),
                                  litLengthCount.as_mut_ptr(),
                                  repOffset.as_mut_ptr(),
                                  (srcBuffer as
                                       *const libc::c_char).offset(pos as
                                                                       isize)
                                      as *const libc::c_void,
                                  *fileSizes.offset(u as isize),
                                  notificationLevel);
                pos =
                    (pos as
                         libc::c_ulong).wrapping_add(*fileSizes.offset(u as
                                                                           isize))
                        as size_t as size_t;
                u = u.wrapping_add(1)
            }
            /* analyze, build stats, starting with literals */
            let mut maxNbBits: size_t =
                HUF_buildCTable(hufTable, countLit.as_mut_ptr(),
                                255i32 as libc::c_uint, huffLog);
            if 0 != ERR_isError(maxNbBits) {
                eSize = -(ZSTD_error_GENERIC as libc::c_int) as size_t;
                if notificationLevel >= 1i32 as libc::c_uint {
                    fprintf(stderr,
                            b" HUF_buildCTable error \n\x00" as *const u8 as
                                *const libc::c_char);
                    fflush(stderr);
                }
            } else {
                if maxNbBits == 8i32 as libc::c_ulong {
                    if notificationLevel >= 2i32 as libc::c_uint {
                        fprintf(stderr,
                                b"warning : pathological dataset : literals are not compressible : samples are noisy or too regular \n\x00"
                                    as *const u8 as *const libc::c_char);
                        fflush(stderr);
                    }
                    ZDICT_flatLit(countLit.as_mut_ptr());
                    maxNbBits =
                        HUF_buildCTable(hufTable, countLit.as_mut_ptr(),
                                        255i32 as libc::c_uint, huffLog)
                }
                huffLog = maxNbBits as U32;
                let mut offset: U32 = 0;
                offset = 1i32 as U32;
                while offset < 1024i32 as libc::c_uint {
                    ZDICT_insertSortCount(bestRepOffset.as_mut_ptr(), offset,
                                          repOffset[offset as usize]);
                    offset = offset.wrapping_add(1)
                }
                total = 0i32 as U32;
                u = 0i32 as U32;
                while u <= offcodeMax {
                    total =
                        (total as
                             libc::c_uint).wrapping_add(offcodeCount[u as
                                                                         usize])
                            as U32 as U32;
                    u = u.wrapping_add(1)
                }
                errorCode =
                    FSE_normalizeCount(offcodeNCount.as_mut_ptr(), Offlog,
                                       offcodeCount.as_mut_ptr(),
                                       total as size_t, offcodeMax);
                if 0 != ERR_isError(errorCode) {
                    eSize = -(ZSTD_error_GENERIC as libc::c_int) as size_t;
                    if notificationLevel >= 1i32 as libc::c_uint {
                        fprintf(stderr,
                                b"FSE_normalizeCount error with offcodeCount \n\x00"
                                    as *const u8 as *const libc::c_char);
                        fflush(stderr);
                    }
                } else {
                    Offlog = errorCode as U32;
                    total = 0i32 as U32;
                    u = 0i32 as U32;
                    while u <= 52i32 as libc::c_uint {
                        total =
                            (total as
                                 libc::c_uint).wrapping_add(matchLengthCount[u
                                                                                 as
                                                                                 usize])
                                as U32 as U32;
                        u = u.wrapping_add(1)
                    }
                    errorCode =
                        FSE_normalizeCount(matchLengthNCount.as_mut_ptr(),
                                           mlLog,
                                           matchLengthCount.as_mut_ptr(),
                                           total as size_t,
                                           52i32 as libc::c_uint);
                    if 0 != ERR_isError(errorCode) {
                        eSize =
                            -(ZSTD_error_GENERIC as libc::c_int) as size_t;
                        if notificationLevel >= 1i32 as libc::c_uint {
                            fprintf(stderr,
                                    b"FSE_normalizeCount error with matchLengthCount \n\x00"
                                        as *const u8 as *const libc::c_char);
                            fflush(stderr);
                        }
                    } else {
                        mlLog = errorCode as U32;
                        total = 0i32 as U32;
                        u = 0i32 as U32;
                        while u <= 35i32 as libc::c_uint {
                            total =
                                (total as
                                     libc::c_uint).wrapping_add(litLengthCount[u
                                                                                   as
                                                                                   usize])
                                    as U32 as U32;
                            u = u.wrapping_add(1)
                        }
                        errorCode =
                            FSE_normalizeCount(litLengthNCount.as_mut_ptr(),
                                               llLog,
                                               litLengthCount.as_mut_ptr(),
                                               total as size_t,
                                               35i32 as libc::c_uint);
                        if 0 != ERR_isError(errorCode) {
                            eSize =
                                -(ZSTD_error_GENERIC as libc::c_int) as
                                    size_t;
                            if notificationLevel >= 1i32 as libc::c_uint {
                                fprintf(stderr,
                                        b"FSE_normalizeCount error with litLengthCount \n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                                fflush(stderr);
                            }
                        } else {
                            llLog = errorCode as U32;
                            /* write result to buffer */
                            let hhSize: size_t =
                                HUF_writeCTable(dstPtr as *mut libc::c_void,
                                                maxDstSize, hufTable,
                                                255i32 as libc::c_uint,
                                                huffLog);
                            if 0 != ERR_isError(hhSize) {
                                eSize =
                                    -(ZSTD_error_GENERIC as libc::c_int) as
                                        size_t;
                                if notificationLevel >= 1i32 as libc::c_uint {
                                    fprintf(stderr,
                                            b"HUF_writeCTable error \n\x00" as
                                                *const u8 as
                                                *const libc::c_char);
                                    fflush(stderr);
                                }
                            } else {
                                dstPtr = dstPtr.offset(hhSize as isize);
                                maxDstSize =
                                    (maxDstSize as
                                         libc::c_ulong).wrapping_sub(hhSize)
                                        as size_t as size_t;
                                eSize =
                                    (eSize as
                                         libc::c_ulong).wrapping_add(hhSize)
                                        as size_t as size_t;
                                let ohSize: size_t =
                                    FSE_writeNCount(dstPtr as
                                                        *mut libc::c_void,
                                                    maxDstSize,
                                                    offcodeNCount.as_mut_ptr(),
                                                    30i32 as libc::c_uint,
                                                    Offlog);
                                if 0 != ERR_isError(ohSize) {
                                    eSize =
                                        -(ZSTD_error_GENERIC as libc::c_int)
                                            as size_t;
                                    if notificationLevel >=
                                           1i32 as libc::c_uint {
                                        fprintf(stderr,
                                                b"FSE_writeNCount error with offcodeNCount \n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char);
                                        fflush(stderr);
                                    }
                                } else {
                                    dstPtr = dstPtr.offset(ohSize as isize);
                                    maxDstSize =
                                        (maxDstSize as
                                             libc::c_ulong).wrapping_sub(ohSize)
                                            as size_t as size_t;
                                    eSize =
                                        (eSize as
                                             libc::c_ulong).wrapping_add(ohSize)
                                            as size_t as size_t;
                                    let mhSize: size_t =
                                        FSE_writeNCount(dstPtr as
                                                            *mut libc::c_void,
                                                        maxDstSize,
                                                        matchLengthNCount.as_mut_ptr(),
                                                        52i32 as libc::c_uint,
                                                        mlLog);
                                    if 0 != ERR_isError(mhSize) {
                                        eSize =
                                            -(ZSTD_error_GENERIC as
                                                  libc::c_int) as size_t;
                                        if notificationLevel >=
                                               1i32 as libc::c_uint {
                                            fprintf(stderr,
                                                    b"FSE_writeNCount error with matchLengthNCount \n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char);
                                            fflush(stderr);
                                        }
                                    } else {
                                        dstPtr =
                                            dstPtr.offset(mhSize as isize);
                                        maxDstSize =
                                            (maxDstSize as
                                                 libc::c_ulong).wrapping_sub(mhSize)
                                                as size_t as size_t;
                                        eSize =
                                            (eSize as
                                                 libc::c_ulong).wrapping_add(mhSize)
                                                as size_t as size_t;
                                        let lhSize: size_t =
                                            FSE_writeNCount(dstPtr as
                                                                *mut libc::c_void,
                                                            maxDstSize,
                                                            litLengthNCount.as_mut_ptr(),
                                                            35i32 as
                                                                libc::c_uint,
                                                            llLog);
                                        if 0 != ERR_isError(lhSize) {
                                            eSize =
                                                -(ZSTD_error_GENERIC as
                                                      libc::c_int) as size_t;
                                            if notificationLevel >=
                                                   1i32 as libc::c_uint {
                                                fprintf(stderr,
                                                        b"FSE_writeNCount error with litlengthNCount \n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                                fflush(stderr);
                                            }
                                        } else {
                                            dstPtr =
                                                dstPtr.offset(lhSize as
                                                                  isize);
                                            maxDstSize =
                                                (maxDstSize as
                                                     libc::c_ulong).wrapping_sub(lhSize)
                                                    as size_t as size_t;
                                            eSize =
                                                (eSize as
                                                     libc::c_ulong).wrapping_add(lhSize)
                                                    as size_t as size_t;
                                            if maxDstSize <
                                                   12i32 as libc::c_ulong {
                                                eSize =
                                                    -(ZSTD_error_GENERIC as
                                                          libc::c_int) as
                                                        size_t;
                                                if notificationLevel >=
                                                       1i32 as libc::c_uint {
                                                    fprintf(stderr,
                                                            b"not enough space to write RepOffsets \n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                                    fflush(stderr);
                                                }
                                            } else {
                                                MEM_writeLE32(dstPtr.offset(0isize)
                                                                  as
                                                                  *mut libc::c_void,
                                                              repStartValue[0usize]);
                                                MEM_writeLE32(dstPtr.offset(4isize)
                                                                  as
                                                                  *mut libc::c_void,
                                                              repStartValue[1usize]);
                                                MEM_writeLE32(dstPtr.offset(8isize)
                                                                  as
                                                                  *mut libc::c_void,
                                                              repStartValue[2usize]);
                                                eSize =
                                                    (eSize as
                                                         libc::c_ulong).wrapping_add(12i32
                                                                                         as
                                                                                         libc::c_ulong)
                                                        as size_t as size_t
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
    ZSTD_freeCDict(esr.dict);
    ZSTD_freeCCtx(esr.zc);
    free(esr.workPlace);
    return eSize;
}
unsafe extern "C" fn ZDICT_insertSortCount(mut table: *mut offsetCount_t,
                                           mut val: U32, mut count: U32) {
    let mut u: U32 = 0;
    (*table.offset(3isize)).offset = val;
    (*table.offset(3isize)).count = count;
    u = 3i32 as U32;
    while u > 0i32 as libc::c_uint {
        let mut tmp: offsetCount_t = offsetCount_t{offset: 0, count: 0,};
        if (*table.offset(u.wrapping_sub(1i32 as libc::c_uint) as
                              isize)).count >=
               (*table.offset(u as isize)).count {
            break ;
        }
        tmp = *table.offset(u.wrapping_sub(1i32 as libc::c_uint) as isize);
        *table.offset(u.wrapping_sub(1i32 as libc::c_uint) as isize) =
            *table.offset(u as isize);
        *table.offset(u as isize) = tmp;
        u = u.wrapping_sub(1)
    };
}
/* ZDICT_flatLit() :
 * rewrite `countLit` to contain a mostly flat but still compressible distribution of literals.
 * necessary to avoid generating a non-compressible distribution that HUF_writeCTable() cannot encode.
 */
unsafe extern "C" fn ZDICT_flatLit(mut countLit: *mut libc::c_uint) {
    let mut u: libc::c_int = 0;
    u = 1i32;
    while u < 256i32 {
        *countLit.offset(u as isize) = 2i32 as libc::c_uint;
        u += 1
    }
    *countLit.offset(0isize) = 4i32 as libc::c_uint;
    *countLit.offset(253isize) = 1i32 as libc::c_uint;
    *countLit.offset(254isize) = 1i32 as libc::c_uint;
}
unsafe extern "C" fn ZDICT_countEStats(mut esr: EStats_ress_t,
                                       mut params: ZSTD_parameters,
                                       mut countLit: *mut libc::c_uint,
                                       mut offsetcodeCount: *mut libc::c_uint,
                                       mut matchlengthCount:
                                           *mut libc::c_uint,
                                       mut litlengthCount: *mut libc::c_uint,
                                       mut repOffsets: *mut U32,
                                       mut src: *const libc::c_void,
                                       mut srcSize: size_t,
                                       mut notificationLevel: U32) {
    let blockSizeMax: size_t =
        (if 1i32 << 17i32 < 1i32 << params.cParams.windowLog {
             1i32 << 17i32
         } else { 1i32 << params.cParams.windowLog }) as size_t;
    let mut cSize: size_t = 0;
    if srcSize > blockSizeMax { srcSize = blockSizeMax }
    let errorCode: size_t = ZSTD_compressBegin_usingCDict(esr.zc, esr.dict);
    if 0 != ERR_isError(errorCode) {
        if notificationLevel >= 1i32 as libc::c_uint {
            fprintf(stderr,
                    b"warning : ZSTD_compressBegin_usingCDict failed \n\x00"
                        as *const u8 as *const libc::c_char);
            fflush(stderr);
        }
        return
    }
    cSize =
        ZSTD_compressBlock(esr.zc, esr.workPlace, (1i32 << 17i32) as size_t,
                           src, srcSize);
    if 0 != ERR_isError(cSize) {
        if notificationLevel >= 3i32 as libc::c_uint {
            fprintf(stderr,
                    b"warning : could not compress sample size %u \n\x00" as
                        *const u8 as *const libc::c_char,
                    srcSize as libc::c_uint);
            fflush(stderr);
        }
        return
    }
    if 0 != cSize {
        let seqStorePtr: *const seqStore_t = ZSTD_getSeqStore(esr.zc);
        let mut bytePtr: *const BYTE = 0 as *const BYTE;
        bytePtr = (*seqStorePtr).litStart;
        while bytePtr < (*seqStorePtr).lit {
            let ref mut fresh0 = *countLit.offset(*bytePtr as isize);
            *fresh0 = (*fresh0).wrapping_add(1);
            bytePtr = bytePtr.offset(1isize)
        }
        let nbSeq: U32 =
            (*seqStorePtr).sequences.wrapping_offset_from((*seqStorePtr).sequencesStart)
                as libc::c_long as U32;
        ZSTD_seqToCodes(seqStorePtr);
        let mut codePtr: *const BYTE = (*seqStorePtr).ofCode;
        let mut u: U32 = 0;
        u = 0i32 as U32;
        while u < nbSeq {
            let ref mut fresh1 =
                *offsetcodeCount.offset(*codePtr.offset(u as isize) as isize);
            *fresh1 = (*fresh1).wrapping_add(1);
            u = u.wrapping_add(1)
        }
        let mut codePtr_0: *const BYTE = (*seqStorePtr).mlCode;
        let mut u_0: U32 = 0;
        u_0 = 0i32 as U32;
        while u_0 < nbSeq {
            let ref mut fresh2 =
                *matchlengthCount.offset(*codePtr_0.offset(u_0 as isize) as
                                             isize);
            *fresh2 = (*fresh2).wrapping_add(1);
            u_0 = u_0.wrapping_add(1)
        }
        let mut codePtr_1: *const BYTE = (*seqStorePtr).llCode;
        let mut u_1: U32 = 0;
        u_1 = 0i32 as U32;
        while u_1 < nbSeq {
            let ref mut fresh3 =
                *litlengthCount.offset(*codePtr_1.offset(u_1 as isize) as
                                           isize);
            *fresh3 = (*fresh3).wrapping_add(1);
            u_1 = u_1.wrapping_add(1)
        }
        if nbSeq >= 2i32 as libc::c_uint {
            let seq: *const seqDef = (*seqStorePtr).sequencesStart;
            let mut offset1: U32 =
                (*seq.offset(0isize)).offset.wrapping_sub(3i32 as
                                                              libc::c_uint);
            let mut offset2: U32 =
                (*seq.offset(1isize)).offset.wrapping_sub(3i32 as
                                                              libc::c_uint);
            if offset1 >= 1024i32 as libc::c_uint { offset1 = 0i32 as U32 }
            if offset2 >= 1024i32 as libc::c_uint { offset2 = 0i32 as U32 }
            let ref mut fresh4 = *repOffsets.offset(offset1 as isize);
            *fresh4 =
                (*fresh4 as libc::c_uint).wrapping_add(3i32 as libc::c_uint)
                    as U32 as U32;
            let ref mut fresh5 = *repOffsets.offset(offset2 as isize);
            *fresh5 =
                (*fresh5 as libc::c_uint).wrapping_add(1i32 as libc::c_uint)
                    as U32 as U32
        }
    };
}
unsafe extern "C" fn ZDICT_totalSampleSize(mut fileSizes: *const size_t,
                                           mut nbFiles: libc::c_uint)
 -> size_t {
    let mut total: size_t = 0i32 as size_t;
    let mut u: libc::c_uint = 0;
    u = 0i32 as libc::c_uint;
    while u < nbFiles {
        total =
            (total as
                 libc::c_ulong).wrapping_add(*fileSizes.offset(u as isize)) as
                size_t as size_t;
        u = u.wrapping_add(1)
    }
    return total;
}
/* ! ZDICT_trainFromBuffer_legacy():
 *  Train a dictionary from an array of samples.
 *  Samples must be stored concatenated in a single flat buffer `samplesBuffer`,
 *  supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order.
 *  The resulting dictionary will be saved into `dictBuffer`.
 * `parameters` is optional and can be provided with values set to 0 to mean "default".
 * @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)
 *          or an error code, which can be tested with ZDICT_isError().
 *  Tips: In general, a reasonable dictionary has a size of ~ 100 KB.
 *        It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`.
 *        In general, it's recommended to provide a few thousands samples, though this can vary a lot.
 *        It's recommended that total size of all samples be about ~x100 times the target size of dictionary.
 *  Note: ZDICT_trainFromBuffer_legacy() will send notifications into stderr if instructed to, using notificationLevel>0.
 */
#[no_mangle]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_legacy(mut dictBuffer:
                                                          *mut libc::c_void,
                                                      mut dictBufferCapacity:
                                                          size_t,
                                                      mut samplesBuffer:
                                                          *const libc::c_void,
                                                      mut samplesSizes:
                                                          *const size_t,
                                                      mut nbSamples:
                                                          libc::c_uint,
                                                      mut params:
                                                          ZDICT_legacy_params_t)
 -> size_t {
    let mut result: size_t = 0;
    let mut newBuff: *mut libc::c_void = 0 as *mut libc::c_void;
    let sBuffSize: size_t = ZDICT_totalSampleSize(samplesSizes, nbSamples);
    if sBuffSize < (128i32 * 4i32) as libc::c_ulong { return 0i32 as size_t }
    newBuff = malloc(sBuffSize.wrapping_add(32i32 as libc::c_ulong));
    if newBuff.is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    }
    memcpy(newBuff, samplesBuffer, sBuffSize);
    ZDICT_fillNoise((newBuff as *mut libc::c_char).offset(sBuffSize as isize)
                        as *mut libc::c_void, 32i32 as size_t);
    result =
        ZDICT_trainFromBuffer_unsafe_legacy(dictBuffer, dictBufferCapacity,
                                            newBuff, samplesSizes, nbSamples,
                                            params);
    free(newBuff);
    return result;
}
/* Hidden declaration for dbio.c */
#[no_mangle]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_unsafe_legacy(mut dictBuffer:
                                                                 *mut libc::c_void,
                                                             mut maxDictSize:
                                                                 size_t,
                                                             mut samplesBuffer:
                                                                 *const libc::c_void,
                                                             mut samplesSizes:
                                                                 *const size_t,
                                                             mut nbSamples:
                                                                 libc::c_uint,
                                                             mut params:
                                                                 ZDICT_legacy_params_t)
 -> size_t {
    let dictListSize: U32 =
        if if 10000i32 as libc::c_uint > nbSamples {
               10000i32 as libc::c_uint
           } else { nbSamples } >
               maxDictSize.wrapping_div(16i32 as libc::c_ulong) as U32 {
            if 10000i32 as libc::c_uint > nbSamples {
                10000i32 as libc::c_uint
            } else { nbSamples }
        } else { maxDictSize.wrapping_div(16i32 as libc::c_ulong) as U32 };
    let dictList: *mut dictItem =
        malloc((dictListSize as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<dictItem>()
                                                    as libc::c_ulong)) as
            *mut dictItem;
    let selectivity: libc::c_uint =
        if params.selectivityLevel == 0i32 as libc::c_uint {
            g_selectivity_default
        } else { params.selectivityLevel };
    let minRep: libc::c_uint =
        if selectivity > 30i32 as libc::c_uint {
            4i32 as libc::c_uint
        } else { nbSamples >> selectivity };
    let targetDictSize: size_t = maxDictSize;
    let samplesBuffSize: size_t =
        ZDICT_totalSampleSize(samplesSizes, nbSamples);
    let mut dictSize: size_t = 0i32 as size_t;
    let notificationLevel: U32 = params.zParams.notificationLevel;
    if dictList.is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    }
    if maxDictSize < 256i32 as libc::c_ulong {
        free(dictList as *mut libc::c_void);
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    if samplesBuffSize < (128i32 * 4i32) as libc::c_ulong {
        free(dictList as *mut libc::c_void);
        return -(ZSTD_error_dictionaryCreation_failed as libc::c_int) as
                   size_t
    }
    ZDICT_initDictItem(dictList);
    ZDICT_trainBuffer_legacy(dictList, dictListSize, samplesBuffer,
                             samplesBuffSize, samplesSizes, nbSamples, minRep,
                             notificationLevel);
    if params.zParams.notificationLevel >= 3i32 as libc::c_uint {
        let nb: libc::c_uint =
            if (25i32 as libc::c_uint) < (*dictList.offset(0isize)).pos {
                25i32 as libc::c_uint
            } else { (*dictList.offset(0isize)).pos };
        let dictContentSize: libc::c_uint = ZDICT_dictSize(dictList);
        let mut u: libc::c_uint = 0;
        if notificationLevel >= 3i32 as libc::c_uint {
            fprintf(stderr,
                    b"\n %u segments found, of total size %u \n\x00" as
                        *const u8 as *const libc::c_char,
                    (*dictList.offset(0isize)).pos.wrapping_sub(1i32 as
                                                                    libc::c_uint),
                    dictContentSize);
            fflush(stderr);
        }
        if notificationLevel >= 3i32 as libc::c_uint {
            fprintf(stderr,
                    b"list %u best segments \n\x00" as *const u8 as
                        *const libc::c_char,
                    nb.wrapping_sub(1i32 as libc::c_uint));
            fflush(stderr);
        }
        u = 1i32 as libc::c_uint;
        while u < nb {
            let pos: libc::c_uint = (*dictList.offset(u as isize)).pos;
            let length: libc::c_uint = (*dictList.offset(u as isize)).length;
            let printedLength: U32 =
                if (40i32 as libc::c_uint) < length {
                    40i32 as libc::c_uint
                } else { length };
            if pos as libc::c_ulong > samplesBuffSize ||
                   pos.wrapping_add(length) as libc::c_ulong > samplesBuffSize
               {
                free(dictList as *mut libc::c_void);
                return -(ZSTD_error_GENERIC as libc::c_int) as size_t
            }
            if notificationLevel >= 3i32 as libc::c_uint {
                fprintf(stderr,
                        b"%3u:%3u bytes at pos %8u, savings %7u bytes |\x00"
                            as *const u8 as *const libc::c_char, u, length,
                        pos, (*dictList.offset(u as isize)).savings);
                fflush(stderr);
            }
            ZDICT_printHex((samplesBuffer as
                                *const libc::c_char).offset(pos as isize) as
                               *const libc::c_void, printedLength as size_t);
            if notificationLevel >= 3i32 as libc::c_uint {
                fprintf(stderr,
                        b"| \n\x00" as *const u8 as *const libc::c_char);
                fflush(stderr);
            }
            u = u.wrapping_add(1)
        }
    }
    let mut dictContentSize_0: libc::c_uint = ZDICT_dictSize(dictList);
    if dictContentSize_0 < 128i32 as libc::c_uint {
        free(dictList as *mut libc::c_void);
        return -(ZSTD_error_dictionaryCreation_failed as libc::c_int) as
                   size_t
    }
    if (dictContentSize_0 as libc::c_ulong) <
           targetDictSize.wrapping_div(4i32 as libc::c_ulong) {
        if notificationLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    b"!  warning : selected content significantly smaller than requested (%u < %u) \n\x00"
                        as *const u8 as *const libc::c_char,
                    dictContentSize_0, maxDictSize as libc::c_uint);
            fflush(stderr);
        }
        if samplesBuffSize <
               (10i32 as libc::c_ulong).wrapping_mul(targetDictSize) {
            if notificationLevel >= 2i32 as libc::c_uint {
                fprintf(stderr,
                        b"!  consider increasing the number of samples (total size : %u MB)\n\x00"
                            as *const u8 as *const libc::c_char,
                        (samplesBuffSize >> 20i32) as libc::c_uint);
                fflush(stderr);
            }
        }
        if minRep > 4i32 as libc::c_uint {
            if notificationLevel >= 2i32 as libc::c_uint {
                fprintf(stderr,
                        b"!  consider increasing selectivity to produce larger dictionary (-s%u) \n\x00"
                            as *const u8 as *const libc::c_char,
                        selectivity.wrapping_add(1i32 as libc::c_uint));
                fflush(stderr);
            }
            if notificationLevel >= 2i32 as libc::c_uint {
                fprintf(stderr,
                        b"!  note : larger dictionaries are not necessarily better, test its efficiency on samples \n\x00"
                            as *const u8 as *const libc::c_char);
                fflush(stderr);
            }
        }
    }
    if dictContentSize_0 as libc::c_ulong >
           targetDictSize.wrapping_mul(3i32 as libc::c_ulong) &&
           nbSamples > (2i32 * 4i32) as libc::c_uint &&
           selectivity > 1i32 as libc::c_uint {
        let mut proposedSelectivity: libc::c_uint =
            selectivity.wrapping_sub(1i32 as libc::c_uint);
        while nbSamples >> proposedSelectivity <= 4i32 as libc::c_uint {
            proposedSelectivity = proposedSelectivity.wrapping_sub(1)
        }
        if notificationLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    b"!  note : calculated dictionary significantly larger than requested (%u > %u) \n\x00"
                        as *const u8 as *const libc::c_char,
                    dictContentSize_0, maxDictSize as libc::c_uint);
            fflush(stderr);
        }
        if notificationLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    b"!  consider increasing dictionary size, or produce denser dictionary (-s%u) \n\x00"
                        as *const u8 as *const libc::c_char,
                    proposedSelectivity);
            fflush(stderr);
        }
        if notificationLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    b"!  always test dictionary efficiency on real samples \n\x00"
                        as *const u8 as *const libc::c_char);
            fflush(stderr);
        }
    }
    let max: U32 = (*dictList).pos;
    let mut currentSize: U32 = 0i32 as U32;
    let mut n: U32 = 0;
    n = 1i32 as U32;
    while n < max {
        currentSize =
            (currentSize as
                 libc::c_uint).wrapping_add((*dictList.offset(n as
                                                                  isize)).length)
                as U32 as U32;
        if currentSize as libc::c_ulong > targetDictSize {
            currentSize =
                (currentSize as
                     libc::c_uint).wrapping_sub((*dictList.offset(n as
                                                                      isize)).length)
                    as U32 as U32;
            break ;
        } else { n = n.wrapping_add(1) }
    }
    (*dictList).pos = n;
    dictContentSize_0 = currentSize;
    let mut u_0: U32 = 0;
    let mut ptr: *mut BYTE =
        (dictBuffer as *mut BYTE).offset(maxDictSize as isize);
    u_0 = 1i32 as U32;
    while u_0 < (*dictList).pos {
        let mut l: U32 = (*dictList.offset(u_0 as isize)).length;
        ptr = ptr.offset(-(l as isize));
        if ptr < dictBuffer as *mut BYTE {
            free(dictList as *mut libc::c_void);
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t
        }
        memcpy(ptr as *mut libc::c_void,
               (samplesBuffer as
                    *const libc::c_char).offset((*dictList.offset(u_0 as
                                                                      isize)).pos
                                                    as isize) as
                   *const libc::c_void, l as libc::c_ulong);
        u_0 = u_0.wrapping_add(1)
    }
    dictSize =
        ZDICT_addEntropyTablesFromBuffer_advanced(dictBuffer,
                                                  dictContentSize_0 as size_t,
                                                  maxDictSize, samplesBuffer,
                                                  samplesSizes, nbSamples,
                                                  params.zParams);
    free(dictList as *mut libc::c_void);
    return dictSize;
}
unsafe extern "C" fn ZDICT_dictSize(mut dictList: *const dictItem) -> U32 {
    let mut u: U32 = 0;
    let mut dictSize: U32 = 0i32 as U32;
    u = 1i32 as U32;
    while u < (*dictList.offset(0isize)).pos {
        dictSize =
            (dictSize as
                 libc::c_uint).wrapping_add((*dictList.offset(u as
                                                                  isize)).length)
                as U32 as U32;
        u = u.wrapping_add(1)
    }
    return dictSize;
}
unsafe extern "C" fn ZDICT_addEntropyTablesFromBuffer_advanced(mut dictBuffer:
                                                                   *mut libc::c_void,
                                                               mut dictContentSize:
                                                                   size_t,
                                                               mut dictBufferCapacity:
                                                                   size_t,
                                                               mut samplesBuffer:
                                                                   *const libc::c_void,
                                                               mut samplesSizes:
                                                                   *const size_t,
                                                               mut nbSamples:
                                                                   libc::c_uint,
                                                               mut params:
                                                                   ZDICT_params_t)
 -> size_t {
    let compressionLevel: libc::c_int =
        if params.compressionLevel == 0i32 {
            g_compressionLevel_default
        } else { params.compressionLevel };
    let notificationLevel: U32 = params.notificationLevel;
    let mut hSize: size_t = 8i32 as size_t;
    if notificationLevel >= 2i32 as libc::c_uint {
        fprintf(stderr, b"\r%70s\r\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char);
        fflush(stderr);
    }
    if notificationLevel >= 2i32 as libc::c_uint {
        fprintf(stderr,
                b"statistics ... \n\x00" as *const u8 as *const libc::c_char);
        fflush(stderr);
    }
    let eSize: size_t =
        ZDICT_analyzeEntropy((dictBuffer as
                                  *mut libc::c_char).offset(hSize as isize) as
                                 *mut libc::c_void,
                             dictBufferCapacity.wrapping_sub(hSize),
                             compressionLevel as libc::c_uint, samplesBuffer,
                             samplesSizes, nbSamples,
                             (dictBuffer as
                                  *mut libc::c_char).offset(dictBufferCapacity
                                                                as
                                                                isize).offset(-(dictContentSize
                                                                                    as
                                                                                    isize))
                                 as *const libc::c_void, dictContentSize,
                             notificationLevel);
    if 0 != ZDICT_isError(eSize) { return eSize }
    hSize = (hSize as libc::c_ulong).wrapping_add(eSize) as size_t as size_t;
    MEM_writeLE32(dictBuffer, 0xec30a437u32);
    let randomID: U64 =
        ZSTD_XXH64((dictBuffer as
                        *mut libc::c_char).offset(dictBufferCapacity as
                                                      isize).offset(-(dictContentSize
                                                                          as
                                                                          isize))
                       as *const libc::c_void, dictContentSize,
                   0i32 as libc::c_ulonglong) as U64;
    let compliantID: U32 =
        randomID.wrapping_rem((1u32 <<
                                   31i32).wrapping_sub(32768i32 as
                                                           libc::c_uint) as
                                  libc::c_ulong).wrapping_add(32768i32 as
                                                                  libc::c_ulong)
            as U32;
    let dictID: U32 =
        if 0 != params.dictID { params.dictID } else { compliantID };
    MEM_writeLE32((dictBuffer as *mut libc::c_char).offset(4isize) as
                      *mut libc::c_void, dictID);
    if hSize.wrapping_add(dictContentSize) < dictBufferCapacity {
        memmove((dictBuffer as *mut libc::c_char).offset(hSize as isize) as
                    *mut libc::c_void,
                (dictBuffer as
                     *mut libc::c_char).offset(dictBufferCapacity as
                                                   isize).offset(-(dictContentSize
                                                                       as
                                                                       isize))
                    as *const libc::c_void, dictContentSize);
    }
    return if dictBufferCapacity < hSize.wrapping_add(dictContentSize) {
               dictBufferCapacity
           } else { hSize.wrapping_add(dictContentSize) };
}
static mut g_selectivity_default: U32 = 9i32 as U32;
unsafe extern "C" fn ZDICT_printHex(mut ptr: *const libc::c_void,
                                    mut length: size_t) {
    let b: *const BYTE = ptr as *const BYTE;
    let mut u: size_t = 0;
    u = 0i32 as size_t;
    while u < length {
        let mut c: BYTE = *b.offset(u as isize);
        if (c as libc::c_int) < 32i32 || c as libc::c_int > 126i32 {
            c = '.' as i32 as BYTE
        }
        fprintf(stderr, b"%c\x00" as *const u8 as *const libc::c_char,
                c as libc::c_int);
        fflush(stderr);
        u = u.wrapping_add(1)
    };
}
unsafe extern "C" fn ZDICT_trainBuffer_legacy(mut dictList: *mut dictItem,
                                              mut dictListSize: U32,
                                              buffer: *const libc::c_void,
                                              mut bufferSize: size_t,
                                              mut fileSizes: *const size_t,
                                              mut nbFiles: libc::c_uint,
                                              mut minRatio: libc::c_uint,
                                              mut notificationLevel: U32)
 -> size_t {
    let suffix0: *mut libc::c_int =
        malloc(bufferSize.wrapping_add(2i32 as
                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                           as
                                                                           libc::c_ulong))
            as *mut libc::c_int;
    let suffix: *mut libc::c_int = suffix0.offset(1isize);
    let mut reverseSuffix: *mut U32 =
        malloc(bufferSize.wrapping_mul(::std::mem::size_of::<U32>() as
                                           libc::c_ulong)) as *mut U32;
    /* +16 for overflow security */
    let mut doneMarks: *mut BYTE =
        malloc(bufferSize.wrapping_add(16i32 as
                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<BYTE>()
                                                                           as
                                                                           libc::c_ulong))
            as *mut BYTE;
    let mut filePos: *mut U32 =
        malloc((nbFiles as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<U32>()
                                                    as libc::c_ulong)) as
            *mut U32;
    let mut result: size_t = 0i32 as size_t;
    let mut displayClock: clock_t = 0i32 as clock_t;
    let refreshRate: clock_t =
        1000000i32 as __clock_t * 3i32 as libc::c_long /
            10i32 as libc::c_long;
    if notificationLevel >= 2i32 as libc::c_uint {
        fprintf(stderr, b"\r%70s\r\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char);
        fflush(stderr);
    }
    if suffix0.is_null() || reverseSuffix.is_null() || doneMarks.is_null() ||
           filePos.is_null() {
        result = -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    } else {
        if minRatio < 4i32 as libc::c_uint { minRatio = 4i32 as libc::c_uint }
        memset(doneMarks as *mut libc::c_void, 0i32,
               bufferSize.wrapping_add(16i32 as libc::c_ulong));
        if bufferSize > (2000u32 << 20i32) as libc::c_ulong {
            if notificationLevel >= 3i32 as libc::c_uint {
                fprintf(stderr,
                        b"sample set too large : reduced to %u MB ...\n\x00"
                            as *const u8 as *const libc::c_char,
                        2000u32 << 20i32 >> 20i32);
                fflush(stderr);
            }
        }
        while bufferSize > (2000u32 << 20i32) as libc::c_ulong {
            nbFiles = nbFiles.wrapping_sub(1);
            bufferSize =
                (bufferSize as
                     libc::c_ulong).wrapping_sub(*fileSizes.offset(nbFiles as
                                                                       isize))
                    as size_t as size_t
        }
        if notificationLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    b"sorting %u files of total size %u MB ...\n\x00" as
                        *const u8 as *const libc::c_char, nbFiles,
                    (bufferSize >> 20i32) as libc::c_uint);
            fflush(stderr);
        }
        let divSuftSortResult: libc::c_int =
            divsufsort(buffer as *const libc::c_uchar, suffix,
                       bufferSize as libc::c_int, 0i32);
        if divSuftSortResult != 0i32 {
            result = -(ZSTD_error_GENERIC as libc::c_int) as size_t
        } else {
            *suffix.offset(bufferSize as isize) = bufferSize as libc::c_int;
            *suffix0.offset(0isize) = bufferSize as libc::c_int;
            let mut pos: size_t = 0;
            pos = 0i32 as size_t;
            while pos < bufferSize {
                *reverseSuffix.offset(*suffix.offset(pos as isize) as isize) =
                    pos as U32;
                pos = pos.wrapping_add(1)
            }
            *filePos.offset(0isize) = 0i32 as U32;
            pos = 1i32 as size_t;
            while pos < nbFiles as libc::c_ulong {
                *filePos.offset(pos as isize) =
                    (*filePos.offset(pos.wrapping_sub(1i32 as libc::c_ulong)
                                         as isize) as
                         libc::c_ulong).wrapping_add(*fileSizes.offset(pos.wrapping_sub(1i32
                                                                                            as
                                                                                            libc::c_ulong)
                                                                           as
                                                                           isize))
                        as U32;
                pos = pos.wrapping_add(1)
            }
            if notificationLevel >= 2i32 as libc::c_uint {
                fprintf(stderr,
                        b"finding patterns ... \n\x00" as *const u8 as
                            *const libc::c_char);
                fflush(stderr);
            }
            if notificationLevel >= 3i32 as libc::c_uint {
                fprintf(stderr,
                        b"minimum ratio : %u \n\x00" as *const u8 as
                            *const libc::c_char, minRatio);
                fflush(stderr);
            }
            let mut cursor: U32 = 0;
            cursor = 0i32 as U32;
            while (cursor as libc::c_ulong) < bufferSize {
                let mut solution: dictItem =
                    dictItem{pos: 0, length: 0, savings: 0,};
                if 0 != *doneMarks.offset(cursor as isize) {
                    cursor = cursor.wrapping_add(1)
                } else {
                    solution =
                        ZDICT_analyzePos(doneMarks, suffix,
                                         *reverseSuffix.offset(cursor as
                                                                   isize),
                                         buffer, minRatio, notificationLevel);
                    if solution.length == 0i32 as libc::c_uint {
                        cursor = cursor.wrapping_add(1)
                    } else {
                        ZDICT_insertDictItem(dictList, dictListSize, solution,
                                             buffer);
                        cursor =
                            (cursor as
                                 libc::c_uint).wrapping_add(solution.length)
                                as U32 as U32;
                        if notificationLevel >= 2i32 as libc::c_uint {
                            if ZDICT_clockSpan(displayClock) > refreshRate {
                                displayClock = clock();
                                fprintf(stderr,
                                        b"\r%4.2f %% \r\x00" as *const u8 as
                                            *const libc::c_char,
                                        cursor as libc::c_double /
                                            bufferSize as libc::c_double *
                                            100i32 as libc::c_double);
                                fflush(stderr);
                                if notificationLevel >= 4i32 as libc::c_uint {
                                    fflush(stderr);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(suffix0 as *mut libc::c_void);
    free(reverseSuffix as *mut libc::c_void);
    free(doneMarks as *mut libc::c_void);
    free(filePos as *mut libc::c_void);
    return result;
}
/*-*************************************
*  Console display
***************************************/
/* 0 : no display;   1: errors;   2: default;  3: details;  4: debug */
unsafe extern "C" fn ZDICT_clockSpan(mut nPrevious: clock_t) -> clock_t {
    return clock() - nPrevious;
}
unsafe extern "C" fn ZDICT_insertDictItem(mut table: *mut dictItem,
                                          mut maxSize: U32, mut elt: dictItem,
                                          mut buffer: *const libc::c_void) {
    /* merge if possible */
    let mut mergeId: U32 = ZDICT_tryMerge(table, elt, 0i32 as U32, buffer);
    if 0 != mergeId {
        let mut newMerge: U32 = 1i32 as U32;
        while 0 != newMerge {
            newMerge =
                ZDICT_tryMerge(table, *table.offset(mergeId as isize),
                               mergeId, buffer);
            if 0 != newMerge { ZDICT_removeDictItem(table, mergeId); }
            mergeId = newMerge
        }
        return
    }
    let mut current: U32 = 0;
    let mut nextElt: U32 = (*table).pos;
    if nextElt >= maxSize {
        nextElt = maxSize.wrapping_sub(1i32 as libc::c_uint)
    }
    current = nextElt.wrapping_sub(1i32 as libc::c_uint);
    while (*table.offset(current as isize)).savings < elt.savings {
        *table.offset(current.wrapping_add(1i32 as libc::c_uint) as isize) =
            *table.offset(current as isize);
        current = current.wrapping_sub(1)
    }
    *table.offset(current.wrapping_add(1i32 as libc::c_uint) as isize) = elt;
    (*table).pos = nextElt.wrapping_add(1i32 as libc::c_uint);
}
/* ! ZDICT_tryMerge() :
    check if dictItem can be merged, do it if possible
    @return : id of destination elt, 0 if not merged
*/
unsafe extern "C" fn ZDICT_tryMerge(mut table: *mut dictItem,
                                    mut elt: dictItem, mut eltNbToSkip: U32,
                                    mut buffer: *const libc::c_void) -> U32 {
    let tableSize: U32 = (*table).pos;
    let eltEnd: U32 = elt.pos.wrapping_add(elt.length);
    let buf: *const libc::c_char = buffer as *const libc::c_char;
    /* tail overlap */
    let mut u: U32 = 0;
    u = 1i32 as U32;
    while u < tableSize {
        if !(u == eltNbToSkip) {
            if (*table.offset(u as isize)).pos > elt.pos &&
                   (*table.offset(u as isize)).pos <= eltEnd {
                let addedLength: U32 =
                    (*table.offset(u as isize)).pos.wrapping_sub(elt.pos);
                let ref mut fresh6 = (*table.offset(u as isize)).length;
                *fresh6 =
                    (*fresh6 as libc::c_uint).wrapping_add(addedLength) as U32
                        as U32;
                (*table.offset(u as isize)).pos = elt.pos;
                let ref mut fresh7 = (*table.offset(u as isize)).savings;
                *fresh7 =
                    (*fresh7 as
                         libc::c_uint).wrapping_add(elt.savings.wrapping_mul(addedLength).wrapping_div(elt.length))
                        as U32 as U32;
                let ref mut fresh8 = (*table.offset(u as isize)).savings;
                *fresh8 =
                    (*fresh8 as
                         libc::c_uint).wrapping_add(elt.length.wrapping_div(8i32
                                                                                as
                                                                                libc::c_uint))
                        as U32 as U32;
                elt = *table.offset(u as isize);
                while u > 1i32 as libc::c_uint &&
                          (*table.offset(u.wrapping_sub(1i32 as libc::c_uint)
                                             as isize)).savings < elt.savings
                      {
                    *table.offset(u as isize) =
                        *table.offset(u.wrapping_sub(1i32 as libc::c_uint) as
                                          isize);
                    u = u.wrapping_sub(1)
                }
                *table.offset(u as isize) = elt;
                return u
            }
        }
        u = u.wrapping_add(1)
    }
    u = 1i32 as U32;
    while u < tableSize {
        if !(u == eltNbToSkip) {
            if (*table.offset(u as
                                  isize)).pos.wrapping_add((*table.offset(u as
                                                                              isize)).length)
                   >= elt.pos && (*table.offset(u as isize)).pos < elt.pos {
                let addedLength_0: libc::c_int =
                    (eltEnd as libc::c_int as
                         libc::c_uint).wrapping_sub((*table.offset(u as
                                                                       isize)).pos.wrapping_add((*table.offset(u
                                                                                                                   as
                                                                                                                   isize)).length))
                        as libc::c_int;
                let ref mut fresh9 = (*table.offset(u as isize)).savings;
                *fresh9 =
                    (*fresh9 as
                         libc::c_uint).wrapping_add(elt.length.wrapping_div(8i32
                                                                                as
                                                                                libc::c_uint))
                        as U32 as U32;
                if addedLength_0 > 0i32 {
                    let ref mut fresh10 = (*table.offset(u as isize)).length;
                    *fresh10 =
                        (*fresh10 as
                             libc::c_uint).wrapping_add(addedLength_0 as
                                                            libc::c_uint) as
                            U32 as U32;
                    let ref mut fresh11 = (*table.offset(u as isize)).savings;
                    *fresh11 =
                        (*fresh11 as
                             libc::c_uint).wrapping_add(elt.savings.wrapping_mul(addedLength_0
                                                                                     as
                                                                                     libc::c_uint).wrapping_div(elt.length))
                            as U32 as U32
                }
                elt = *table.offset(u as isize);
                while u > 1i32 as libc::c_uint &&
                          (*table.offset(u.wrapping_sub(1i32 as libc::c_uint)
                                             as isize)).savings < elt.savings
                      {
                    *table.offset(u as isize) =
                        *table.offset(u.wrapping_sub(1i32 as libc::c_uint) as
                                          isize);
                    u = u.wrapping_sub(1)
                }
                *table.offset(u as isize) = elt;
                return u
            }
            if MEM_read64(buf.offset((*table.offset(u as isize)).pos as isize)
                              as *const libc::c_void) ==
                   MEM_read64(buf.offset(elt.pos as isize).offset(1isize) as
                                  *const libc::c_void) {
                if 0 !=
                       isIncluded(buf.offset((*table.offset(u as isize)).pos
                                                 as isize) as
                                      *const libc::c_void,
                                  buf.offset(elt.pos as isize).offset(1isize)
                                      as *const libc::c_void,
                                  (*table.offset(u as isize)).length as
                                      size_t) {
                    let addedLength_1: size_t =
                        (if elt.length as libc::c_int -
                                (*table.offset(u as isize)).length as
                                    libc::c_int > 1i32 {
                             elt.length as libc::c_int -
                                 (*table.offset(u as isize)).length as
                                     libc::c_int
                         } else { 1i32 }) as size_t;
                    (*table.offset(u as isize)).pos = elt.pos;
                    let ref mut fresh12 = (*table.offset(u as isize)).savings;
                    *fresh12 =
                        (*fresh12 as
                             libc::c_uint).wrapping_add((elt.savings as
                                                             libc::c_ulong).wrapping_mul(addedLength_1).wrapping_div(elt.length
                                                                                                                         as
                                                                                                                         libc::c_ulong)
                                                            as U32) as U32 as
                            U32;
                    (*table.offset(u as isize)).length =
                        if elt.length <
                               (*table.offset(u as
                                                  isize)).length.wrapping_add(1i32
                                                                                  as
                                                                                  libc::c_uint)
                           {
                            elt.length
                        } else {
                            (*table.offset(u as
                                               isize)).length.wrapping_add(1i32
                                                                               as
                                                                               libc::c_uint)
                        };
                    return u
                }
            }
        }
        u = u.wrapping_add(1)
    }
    return 0i32 as U32;
}
unsafe extern "C" fn isIncluded(mut in_0: *const libc::c_void,
                                mut container: *const libc::c_void,
                                mut length: size_t) -> libc::c_int {
    let ip: *const libc::c_char = in_0 as *const libc::c_char;
    let into: *const libc::c_char = container as *const libc::c_char;
    let mut u: size_t = 0;
    u = 0i32 as size_t;
    while u < length {
        /* works because end of buffer is a noisy guard band */
        if *ip.offset(u as isize) as libc::c_int !=
               *into.offset(u as isize) as libc::c_int {
            break ;
        }
        u = u.wrapping_add(1)
    }
    return (u == length) as libc::c_int;
}
unsafe extern "C" fn ZDICT_removeDictItem(mut table: *mut dictItem,
                                          mut id: U32) {
    /* convention : table[0].pos stores nb of elts */
    let max: U32 = (*table.offset(0isize)).pos;
    let mut u: U32 = 0;
    if 0 == id { return }
    u = id;
    while u < max.wrapping_sub(1i32 as libc::c_uint) {
        *table.offset(u as isize) =
            *table.offset(u.wrapping_add(1i32 as libc::c_uint) as isize);
        u = u.wrapping_add(1)
    }
    (*table).pos = (*table).pos.wrapping_sub(1);
}
/* heuristic determined experimentally */
/* heuristic determined experimentally */
unsafe extern "C" fn ZDICT_analyzePos(mut doneMarks: *mut BYTE,
                                      mut suffix: *const libc::c_int,
                                      mut start: U32,
                                      mut buffer: *const libc::c_void,
                                      mut minRatio: U32,
                                      mut notificationLevel: U32)
 -> dictItem {
    let mut lengthList: [U32; 64] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut cumulLength: [U32; 64] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut savings: [U32; 64] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut b: *const BYTE = buffer as *const BYTE;
    let mut maxLength: size_t = 64i32 as size_t;
    let mut pos: size_t = *suffix.offset(start as isize) as size_t;
    let mut end: U32 = start;
    let mut solution: dictItem = dictItem{pos: 0, length: 0, savings: 0,};
    memset(&mut solution as *mut dictItem as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<dictItem>() as libc::c_ulong);
    *doneMarks.offset(pos as isize) = 1i32 as BYTE;
    if MEM_read16(b.offset(pos as isize).offset(0isize) as
                      *const libc::c_void) as libc::c_int ==
           MEM_read16(b.offset(pos as isize).offset(2isize) as
                          *const libc::c_void) as libc::c_int ||
           MEM_read16(b.offset(pos as isize).offset(1isize) as
                          *const libc::c_void) as libc::c_int ==
               MEM_read16(b.offset(pos as isize).offset(3isize) as
                              *const libc::c_void) as libc::c_int ||
           MEM_read16(b.offset(pos as isize).offset(2isize) as
                          *const libc::c_void) as libc::c_int ==
               MEM_read16(b.offset(pos as isize).offset(4isize) as
                              *const libc::c_void) as libc::c_int {
        let pattern16: U16 =
            MEM_read16(b.offset(pos as isize).offset(4isize) as
                           *const libc::c_void);
        let mut u: U32 = 0;
        let mut patternEnd: U32 = 6i32 as U32;
        while MEM_read16(b.offset(pos as isize).offset(patternEnd as isize) as
                             *const libc::c_void) as libc::c_int ==
                  pattern16 as libc::c_int {
            patternEnd =
                (patternEnd as
                     libc::c_uint).wrapping_add(2i32 as libc::c_uint) as U32
                    as U32
        }
        if *b.offset(pos.wrapping_add(patternEnd as libc::c_ulong) as isize)
               as libc::c_int ==
               *b.offset(pos.wrapping_add(patternEnd as
                                              libc::c_ulong).wrapping_sub(1i32
                                                                              as
                                                                              libc::c_ulong)
                             as isize) as libc::c_int {
            patternEnd = patternEnd.wrapping_add(1)
        }
        u = 1i32 as U32;
        while u < patternEnd {
            *doneMarks.offset(pos.wrapping_add(u as libc::c_ulong) as isize) =
                1i32 as BYTE;
            u = u.wrapping_add(1)
        }
        return solution
    }
    let mut length: size_t = 0;
    loop  {
        end = end.wrapping_add(1);
        length =
            ZDICT_count(b.offset(pos as isize) as *const libc::c_void,
                        b.offset(*suffix.offset(end as isize) as isize) as
                            *const libc::c_void);
        if !(length >= 7i32 as libc::c_ulong) { break ; }
    }
    let mut length_0: size_t = 0;
    loop  {
        length_0 =
            ZDICT_count(b.offset(pos as isize) as *const libc::c_void,
                        b.offset(*suffix.offset(start as
                                                    isize).offset(-1isize) as
                                     isize) as *const libc::c_void);
        if length_0 >= 7i32 as libc::c_ulong { start = start.wrapping_sub(1) }
        if !(length_0 >= 7i32 as libc::c_ulong) { break ; }
    }
    if end.wrapping_sub(start) < minRatio {
        let mut idx: U32 = 0;
        idx = start;
        while idx < end {
            *doneMarks.offset(*suffix.offset(idx as isize) as isize) =
                1i32 as BYTE;
            idx = idx.wrapping_add(1)
        }
        return solution
    }
    let mut i: libc::c_int = 0;
    let mut mml: U32 = 0;
    let mut refinedStart: U32 = start;
    let mut refinedEnd: U32 = end;
    if notificationLevel >= 4i32 as libc::c_uint {
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        fflush(stderr);
    }
    if notificationLevel >= 4i32 as libc::c_uint {
        fprintf(stderr,
                b"found %3u matches of length >= %i at pos %7u  \x00" as
                    *const u8 as *const libc::c_char, end.wrapping_sub(start),
                7i32, pos as libc::c_uint);
        fflush(stderr);
    }
    if notificationLevel >= 4i32 as libc::c_uint {
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        fflush(stderr);
    }
    mml = 7i32 as U32;
    loop  {
        let mut currentChar: BYTE = 0i32 as BYTE;
        let mut currentCount: U32 = 0i32 as U32;
        let mut currentID: U32 = refinedStart;
        let mut id: U32 = 0;
        let mut selectedCount: U32 = 0i32 as U32;
        let mut selectedID: U32 = currentID;
        id = refinedStart;
        while id < refinedEnd {
            if *b.offset((*suffix.offset(id as isize) as
                              libc::c_uint).wrapping_add(mml) as isize) as
                   libc::c_int != currentChar as libc::c_int {
                if currentCount > selectedCount {
                    selectedCount = currentCount;
                    selectedID = currentID
                }
                currentID = id;
                currentChar =
                    *b.offset((*suffix.offset(id as isize) as
                                   libc::c_uint).wrapping_add(mml) as isize);
                currentCount = 0i32 as U32
            }
            currentCount = currentCount.wrapping_add(1);
            id = id.wrapping_add(1)
        }
        if currentCount > selectedCount {
            selectedCount = currentCount;
            selectedID = currentID
        }
        if selectedCount < minRatio { break ; }
        refinedStart = selectedID;
        refinedEnd = refinedStart.wrapping_add(selectedCount);
        mml = mml.wrapping_add(1)
    }
    start = refinedStart;
    pos = *suffix.offset(refinedStart as isize) as size_t;
    end = start;
    memset(lengthList.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[U32; 64]>() as libc::c_ulong);
    let mut length_1: size_t = 0;
    loop  {
        end = end.wrapping_add(1);
        length_1 =
            ZDICT_count(b.offset(pos as isize) as *const libc::c_void,
                        b.offset(*suffix.offset(end as isize) as isize) as
                            *const libc::c_void);
        if length_1 >= 64i32 as libc::c_ulong {
            length_1 = (64i32 - 1i32) as size_t
        }
        lengthList[length_1 as usize] =
            lengthList[length_1 as usize].wrapping_add(1);
        if !(length_1 >= 7i32 as libc::c_ulong) { break ; }
    }
    let mut length_2: size_t = 7i32 as size_t;
    while 0 !=
              (length_2 >= 7i32 as libc::c_ulong) as libc::c_int &
                  (start > 0i32 as libc::c_uint) as libc::c_int {
        length_2 =
            ZDICT_count(b.offset(pos as isize) as *const libc::c_void,
                        b.offset(*suffix.offset(start.wrapping_sub(1i32 as
                                                                       libc::c_uint)
                                                    as isize) as isize) as
                            *const libc::c_void);
        if length_2 >= 64i32 as libc::c_ulong {
            length_2 = (64i32 - 1i32) as size_t
        }
        lengthList[length_2 as usize] =
            lengthList[length_2 as usize].wrapping_add(1);
        if length_2 >= 7i32 as libc::c_ulong { start = start.wrapping_sub(1) }
    }
    memset(cumulLength.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[U32; 64]>() as libc::c_ulong);
    cumulLength[maxLength.wrapping_sub(1i32 as libc::c_ulong) as usize] =
        lengthList[maxLength.wrapping_sub(1i32 as libc::c_ulong) as usize];
    i = maxLength.wrapping_sub(2i32 as libc::c_ulong) as libc::c_int;
    while i >= 0i32 {
        cumulLength[i as usize] =
            cumulLength[(i + 1i32) as
                            usize].wrapping_add(lengthList[i as usize]);
        i -= 1
    }
    i = 64i32 - 1i32;
    while i >= 7i32 {
        if cumulLength[i as usize] >= minRatio { break ; }
        i -= 1
    }
    maxLength = i as size_t;
    let mut l: U32 = maxLength as U32;
    let c: BYTE =
        *b.offset(pos.wrapping_add(maxLength).wrapping_sub(1i32 as
                                                               libc::c_ulong)
                      as isize);
    while *b.offset(pos.wrapping_add(l as
                                         libc::c_ulong).wrapping_sub(2i32 as
                                                                         libc::c_ulong)
                        as isize) as libc::c_int == c as libc::c_int {
        l = l.wrapping_sub(1)
    }
    maxLength = l as size_t;
    if maxLength < 7i32 as libc::c_ulong { return solution }
    savings[5usize] = 0i32 as U32;
    i = 7i32;
    while i <= maxLength as libc::c_int {
        savings[i as usize] =
            savings[(i - 1i32) as
                        usize].wrapping_add(lengthList[i as
                                                           usize].wrapping_mul((i
                                                                                    -
                                                                                    3i32)
                                                                                   as
                                                                                   libc::c_uint));
        i += 1
    }
    if notificationLevel >= 4i32 as libc::c_uint {
        fprintf(stderr,
                b"Selected dict at position %u, of length %u : saves %u (ratio: %.2f)  \n\x00"
                    as *const u8 as *const libc::c_char, pos as libc::c_uint,
                maxLength as libc::c_uint, savings[maxLength as usize],
                savings[maxLength as usize] as libc::c_double /
                    maxLength as libc::c_double);
        fflush(stderr);
    }
    solution.pos = pos as U32;
    solution.length = maxLength as U32;
    solution.savings = savings[maxLength as usize];
    let mut id_0: U32 = 0;
    id_0 = start;
    while id_0 < end {
        let mut p: U32 = 0;
        let mut pEnd: U32 = 0;
        let mut length_3: U32 = 0;
        let testedPos: U32 = *suffix.offset(id_0 as isize) as U32;
        if testedPos as libc::c_ulong == pos {
            length_3 = solution.length
        } else {
            length_3 =
                ZDICT_count(b.offset(pos as isize) as *const libc::c_void,
                            b.offset(testedPos as isize) as
                                *const libc::c_void) as U32;
            if length_3 > solution.length { length_3 = solution.length }
        }
        pEnd = testedPos.wrapping_add(length_3);
        p = testedPos;
        while p < pEnd {
            *doneMarks.offset(p as isize) = 1i32 as BYTE;
            p = p.wrapping_add(1)
        }
        id_0 = id_0.wrapping_add(1)
    }
    return solution;
}
/* ! ZDICT_count() :
    Count the nb of common bytes between 2 pointers.
    Note : this function presumes end of buffer followed by noisy guard band.
*/
unsafe extern "C" fn ZDICT_count(mut pIn: *const libc::c_void,
                                 mut pMatch: *const libc::c_void) -> size_t {
    let pStart: *const libc::c_char = pIn as *const libc::c_char;
    loop  {
        let diff: size_t = MEM_readST(pMatch) ^ MEM_readST(pIn);
        if 0 == diff {
            pIn =
                (pIn as
                     *const libc::c_char).offset(::std::mem::size_of::<size_t>()
                                                     as libc::c_ulong as
                                                     isize) as
                    *const libc::c_void;
            pMatch =
                (pMatch as
                     *const libc::c_char).offset(::std::mem::size_of::<size_t>()
                                                     as libc::c_ulong as
                                                     isize) as
                    *const libc::c_void
        } else {
            pIn =
                (pIn as
                     *const libc::c_char).offset(ZDICT_NbCommonBytes(diff) as
                                                     isize) as
                    *const libc::c_void;
            return (pIn as *const libc::c_char).wrapping_offset_from(pStart)
                       as libc::c_long as size_t
        }
    };
}
/*-********************************************************
*  Dictionary training functions
**********************************************************/
unsafe extern "C" fn ZDICT_NbCommonBytes(mut val: size_t) -> libc::c_uint {
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
unsafe extern "C" fn ZDICT_initDictItem(mut d: *mut dictItem) {
    (*d).pos = 1i32 as U32;
    (*d).length = 0i32 as U32;
    (*d).savings = -1i32 as U32;
}
unsafe extern "C" fn ZDICT_fillNoise(mut buffer: *mut libc::c_void,
                                     mut length: size_t) {
    let prime1: libc::c_uint = 2654435761u32;
    let prime2: libc::c_uint = 2246822519u32;
    let mut acc: libc::c_uint = prime1;
    let mut p: size_t = 0i32 as size_t;
    p = 0i32 as size_t;
    while p < length {
        acc = acc.wrapping_mul(prime2);
        *(buffer as *mut libc::c_uchar).offset(p as isize) =
            (acc >> 21i32) as libc::c_uchar;
        p = p.wrapping_add(1)
    };
}
/* Deprecation warnings */
/* It is generally possible to disable deprecation warnings from compiler,
   for example with -Wno-deprecated-declarations for gcc
   or _CRT_SECURE_NO_WARNINGS in Visual.
   Otherwise, it's also possible to manually define ZDICT_DISABLE_DEPRECATE_WARNINGS */
/* C++14 or greater */
/* ZDICT_DISABLE_DEPRECATE_WARNINGS */
#[no_mangle]
pub unsafe extern "C" fn ZDICT_addEntropyTablesFromBuffer(mut dictBuffer:
                                                              *mut libc::c_void,
                                                          mut dictContentSize:
                                                              size_t,
                                                          mut dictBufferCapacity:
                                                              size_t,
                                                          mut samplesBuffer:
                                                              *const libc::c_void,
                                                          mut samplesSizes:
                                                              *const size_t,
                                                          mut nbSamples:
                                                              libc::c_uint)
 -> size_t {
    let mut params: ZDICT_params_t =
        ZDICT_params_t{compressionLevel: 0, notificationLevel: 0, dictID: 0,};
    memset(&mut params as *mut ZDICT_params_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZDICT_params_t>() as libc::c_ulong);
    return ZDICT_addEntropyTablesFromBuffer_advanced(dictBuffer,
                                                     dictContentSize,
                                                     dictBufferCapacity,
                                                     samplesBuffer,
                                                     samplesSizes, nbSamples,
                                                     params);
}