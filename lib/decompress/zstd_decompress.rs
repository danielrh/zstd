#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc, ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    pub type ZSTD_DDict_s;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* !
Tutorial :
----------
The first step is to count all symbols. FSE_count() does this job very fast.
Result will be saved into 'count', a table of unsigned int, which must be already allocated, and have 'maxSymbolValuePtr[0]+1' cells.
'src' is a table of bytes of size 'srcSize'. All values within 'src' MUST be <= maxSymbolValuePtr[0]
maxSymbolValuePtr[0] will be updated, with its real value (necessarily <= original value)
FSE_count() will return the number of occurrence of the most frequent symbol.
This can be used to know if there is a single symbol within 'src', and to quickly evaluate its compressibility.
If there is an error, the function will return an ErrorCode (which can be tested using FSE_isError()).

The next step is to normalize the frequencies.
FSE_normalizeCount() will ensure that sum of frequencies is == 2 ^'tableLog'.
It also guarantees a minimum of 1 to any Symbol with frequency >= 1.
You can use 'tableLog'==0 to mean "use default tableLog value".
If you are unsure of which tableLog value to use, you can ask FSE_optimalTableLog(),
which will provide the optimal valid tableLog given sourceSize, maxSymbolValue, and a user-defined maximum (0 means "default").

The result of FSE_normalizeCount() will be saved into a table,
called 'normalizedCounter', which is a table of signed short.
'normalizedCounter' must be already allocated, and have at least 'maxSymbolValue+1' cells.
The return value is tableLog if everything proceeded as expected.
It is 0 if there is a single symbol within distribution.
If there is an error (ex: invalid tableLog value), the function will return an ErrorCode (which can be tested using FSE_isError()).

'normalizedCounter' can be saved in a compact manner to a memory area using FSE_writeNCount().
'buffer' must be already allocated.
For guaranteed success, buffer size must be at least FSE_headerBound().
The result of the function is the number of bytes written into 'buffer'.
If there is an error, the function will return an ErrorCode (which can be tested using FSE_isError(); ex : buffer size too small).

'normalizedCounter' can then be used to create the compression table 'CTable'.
The space required by 'CTable' must be already allocated, using FSE_createCTable().
You can then use FSE_buildCTable() to fill 'CTable'.
If there is an error, both functions will return an ErrorCode (which can be tested using FSE_isError()).

'CTable' can then be used to compress 'src', with FSE_compress_usingCTable().
Similar to FSE_count(), the convention is that 'src' is assumed to be a table of char of size 'srcSize'
The function returns the size of compressed data (without header), necessarily <= `dstCapacity`.
If it returns '0', compressed data could not fit into 'dst'.
If there is an error, the function will return an ErrorCode (which can be tested using FSE_isError()).
*/
    /* *** DECOMPRESSION *** */
    /* ! FSE_readNCount():
    Read compactly saved 'normalizedCounter' from 'rBuffer'.
    @return : size read from 'rBuffer',
              or an errorCode, which can be tested using FSE_isError().
              maxSymbolValuePtr[0] and tableLogPtr[0] will also be updated with their respective values */
    #[no_mangle]
    fn FSE_readNCount(normalizedCounter: *mut libc::c_short,
                      maxSymbolValuePtr: *mut libc::c_uint,
                      tableLogPtr: *mut libc::c_uint,
                      rBuffer: *const libc::c_void, rBuffSize: size_t)
     -> size_t;
    /* ! ZSTD_getErrorCode() :
    convert a `size_t` function result into a `ZSTD_ErrorCode` enum type,
    which can be used to compare with enum list published above */
    #[no_mangle]
    fn ZSTD_getErrorCode(functionResult: size_t) -> ZSTD_ErrorCode;
    #[no_mangle]
    fn HUF_readDTableX2_wksp(DTable: *mut HUF_DTable,
                             src: *const libc::c_void, srcSize: size_t,
                             workSpace: *mut libc::c_void, wkspSize: size_t)
     -> size_t;
    /* custom memory allocation functions */
    #[no_mangle]
    fn ZSTD_malloc(size: size_t, customMem: ZSTD_customMem)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ZSTD_free(ptr: *mut libc::c_void, customMem: ZSTD_customMem);
    /* ! ZSTD_freeDDict() :
 *  Function frees memory allocated with ZSTD_createDDict() */
    #[no_mangle]
    fn ZSTD_freeDDict(ddict: *mut ZSTD_DDict) -> size_t;
    #[no_mangle]
    fn ZSTD_XXH64_digest(statePtr: *const XXH64_state_t) -> XXH64_hash_t;
    /* ! ZSTD_getcBlockSize() :
 *  Provides the size of compressed block from block header `src` */
/* Used by: decompress, fullbench (does not get its definition from here) */
    #[no_mangle]
    fn ZSTD_getcBlockSize(src: *const libc::c_void, srcSize: size_t,
                          bpPtr: *mut blockProperties_t) -> size_t;
    #[no_mangle]
    fn ZSTD_XXH64_update(statePtr: *mut XXH64_state_t,
                         input: *const libc::c_void, length: size_t)
     -> XXH_errorcode;
    /*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
    /*-*******************************************************
 *  Dependencies
 *********************************************************/
    /* size_t */
    /* blockProperties_t, and some public functions */
    /* ZSTD_seqSymbol */
    /* ===   Prototypes   === */
    /* note: prototypes already published within `zstd.h` :
 * ZSTD_decompressBlock()
 */
    /* note: prototypes already published within `zstd_internal.h` :
 * ZSTD_getcBlockSize()
 * ZSTD_decodeSeqHeaders()
 */
    /* ZSTD_decompressBlock_internal() :
 * decompress block, starting at `src`,
 * into destination buffer `dst`.
 * @return : decompressed block size,
 *           or an error code (which can be tested using ZSTD_isError())
 */
    #[no_mangle]
    fn ZSTD_decompressBlock_internal(dctx: *mut ZSTD_DCtx,
                                     dst: *mut libc::c_void,
                                     dstCapacity: size_t,
                                     src: *const libc::c_void,
                                     srcSize: size_t, frame: libc::c_int)
     -> size_t;
    #[no_mangle]
    fn ZSTD_XXH64_reset(statePtr: *mut XXH64_state_t, seed: libc::c_ulonglong)
     -> XXH_errorcode;
    /* ZSTD_buildFSETable() :
 * generate FSE decoding table for one symbol (ll, ml or off)
 * this function must be called with valid parameters only
 * (dt is large enough, normalizedCounter distribution total is a power of 2, max is within range, etc.)
 * in which case it cannot fail.
 * Internal use only.
 */
    #[no_mangle]
    fn ZSTD_buildFSETable(dt: *mut ZSTD_seqSymbol,
                          normalizedCounter: *const libc::c_short,
                          maxSymbolValue: libc::c_uint, baseValue: *const U32,
                          nbAdditionalBits: *const U32,
                          tableLog: libc::c_uint);
    #[no_mangle]
    fn ZSTD_copyDDictParameters(dctx: *mut ZSTD_DCtx,
                                ddict: *const ZSTD_DDict);
    #[no_mangle]
    fn ZSTD_DDict_dictSize(ddict: *const ZSTD_DDict) -> size_t;
    /*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
    /*-*******************************************************
 *  Dependencies
 *********************************************************/
    /* size_t */
    /*-*******************************************************
 *  Interface
 *********************************************************/
    /* note: several prototypes are already published in `zstd.h` :
 * ZSTD_createDDict()
 * ZSTD_createDDict_byReference()
 * ZSTD_createDDict_advanced()
 * ZSTD_freeDDict()
 * ZSTD_initStaticDDict()
 * ZSTD_sizeof_DDict()
 * ZSTD_estimateDDictSize()
 * ZSTD_getDictID_fromDict()
 */
    #[no_mangle]
    fn ZSTD_DDict_dictContent(ddict: *const ZSTD_DDict)
     -> *const libc::c_void;
    #[no_mangle]
    fn ZSTD_createDDict_advanced(dict: *const libc::c_void, dictSize: size_t,
                                 dictLoadMethod: ZSTD_dictLoadMethod_e,
                                 dictContentType: ZSTD_dictContentType_e,
                                 customMem: ZSTD_customMem)
     -> *mut ZSTD_DDict;
    #[no_mangle]
    fn ZSTD_sizeof_DDict(ddict: *const ZSTD_DDict) -> size_t;
}
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
/*
 * Copyright (c) 2018-present, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* *
 * Implementation taken from folly/CpuId.h
 * https://github.com/facebook/folly/blob/master/folly/CpuId.h
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_cpuid_t {
    pub f1c: U32,
    pub f1d: U32,
    pub f7b: U32,
    pub f7c: U32,
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
/* HUF_H_298734234 */
/* ******************************************************************
 *  WARNING !!
 *  The following section contains advanced and experimental definitions
 *  which shall never be used in the context of a dynamic library,
 *  because they are not guaranteed to remain stable in the future.
 *  Only consider them in association with static linking.
 * *****************************************************************/
/* *** Dependencies *** */
/* U32 */
/* *** Constants *** */
/* max runtime value of tableLog (due to static allocation); can be modified up to HUF_ABSOLUTEMAX_TABLELOG */
/* default tableLog value when none specified */
/* absolute limit of HUF_MAX_TABLELOG. Beyond that value, code does not work */
/* ****************************************
*  Static allocation
******************************************/
/* HUF buffer bounds */
/* only true when incompressible is pre-filtered with fast heuristic */
/* Macro version, useful for static allocation */
/* static allocation of HUF's Compression Table */
/* Use tables of U32, for proper alignment */
/* no final ; */
/* static allocation of HUF's DTable */
pub type HUF_DTable = U32;
/*= Decompression context
 *  When decompressing many times,
 *  it is recommended to allocate a context only once,
 *  and re-use it for each successive compression operation.
 *  This will make workload friendlier for system's memory.
 *  Use one context per thread for parallel execution. */
pub type ZSTD_DCtx = ZSTD_DCtx_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_DCtx_s {
    pub LLTptr: *const ZSTD_seqSymbol,
    pub MLTptr: *const ZSTD_seqSymbol,
    pub OFTptr: *const ZSTD_seqSymbol,
    pub HUFptr: *const HUF_DTable,
    pub entropy: ZSTD_entropyDTables_t,
    pub workspace: [U32; 512],
    pub previousDstEnd: *const libc::c_void,
    pub prefixStart: *const libc::c_void,
    pub virtualStart: *const libc::c_void,
    pub dictEnd: *const libc::c_void,
    pub expected: size_t,
    pub fParams: ZSTD_frameHeader,
    pub decodedSize: U64,
    pub bType: blockType_e,
    pub stage: ZSTD_dStage,
    pub litEntropy: U32,
    pub fseEntropy: U32,
    pub xxhState: XXH64_state_t,
    pub headerSize: size_t,
    pub format: ZSTD_format_e,
    pub litPtr: *const BYTE,
    pub customMem: ZSTD_customMem,
    pub litSize: size_t,
    pub rleSize: size_t,
    pub staticSize: size_t,
    pub bmi2: libc::c_int,
    pub ddictLocal: *mut ZSTD_DDict,
    pub ddict: *const ZSTD_DDict,
    pub dictID: U32,
    pub ddictIsCold: libc::c_int,
    pub streamStage: ZSTD_dStreamStage,
    pub inBuff: *mut libc::c_char,
    pub inBuffSize: size_t,
    pub inPos: size_t,
    pub maxWindowSize: size_t,
    pub outBuff: *mut libc::c_char,
    pub outBuffSize: size_t,
    pub outStart: size_t,
    pub outEnd: size_t,
    pub lhSize: size_t,
    pub legacyContext: *mut libc::c_void,
    pub previousLegacyVersion: U32,
    pub legacyVersion: U32,
    pub hostageByte: U32,
    pub noForwardProgress: libc::c_int,
    pub litBuffer: [BYTE; 131080],
    pub headerBuffer: [BYTE; 18],
}
pub type ZSTD_dStreamStage = libc::c_uint;
pub const zdss_flush: ZSTD_dStreamStage = 4;
pub const zdss_load: ZSTD_dStreamStage = 3;
pub const zdss_read: ZSTD_dStreamStage = 2;
pub const zdss_loadHeader: ZSTD_dStreamStage = 1;
pub const zdss_init: ZSTD_dStreamStage = 0;
pub type ZSTD_DDict = ZSTD_DDict_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
pub type ZSTD_freeFunction
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
/* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
pub type ZSTD_allocFunction
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
pub type ZSTD_format_e = libc::c_uint;
/* Variant of zstd frame format, without initial 4-bytes magic number.
                                 * Useful to save 4 bytes per generated frame.
                                 * Decoder cannot recognise automatically this format, requiring this instruction. */
pub const ZSTD_f_zstd1_magicless: ZSTD_format_e = 1;
/* Opened question : should we have a format ZSTD_f_auto ?
     * Today, it would mean exactly the same as ZSTD_f_zstd1.
     * But, in the future, should several formats become supported,
     * on the compression side, it would mean "default format".
     * On the decompression side, it would mean "automatic format detection",
     * so that ZSTD_f_zstd1 would mean "accept *only* zstd frames".
     * Since meaning is a little different, another option could be to define different enums for compression and decompression.
     * This question could be kept for later, when there are actually multiple formats to support,
     * but there is also the question of pinning enum values, and pinning value `0` is especially important */
/* zstd frame format, specified in zstd_compression_format.md (default) */
pub const ZSTD_f_zstd1: ZSTD_format_e = 0;
/* incomplete type */
pub type XXH64_state_t = XXH64_state_s;
/* typedef'd to XXH32_state_t */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XXH64_state_s {
    pub total_len: libc::c_ulonglong,
    pub v1: libc::c_ulonglong,
    pub v2: libc::c_ulonglong,
    pub v3: libc::c_ulonglong,
    pub v4: libc::c_ulonglong,
    pub mem64: [libc::c_ulonglong; 4],
    pub memsize: libc::c_uint,
    pub reserved: [libc::c_uint; 2],
}
pub type ZSTD_dStage = libc::c_uint;
pub const ZSTDds_skipFrame: ZSTD_dStage = 7;
pub const ZSTDds_decodeSkippableHeader: ZSTD_dStage = 6;
pub const ZSTDds_checkChecksum: ZSTD_dStage = 5;
pub const ZSTDds_decompressLastBlock: ZSTD_dStage = 4;
pub const ZSTDds_decompressBlock: ZSTD_dStage = 3;
pub const ZSTDds_decodeBlockHeader: ZSTD_dStage = 2;
pub const ZSTDds_decodeFrameHeader: ZSTD_dStage = 1;
pub const ZSTDds_getFrameHeaderSize: ZSTD_dStage = 0;
pub type blockType_e = libc::c_uint;
pub const bt_reserved: blockType_e = 3;
pub const bt_compressed: blockType_e = 2;
pub const bt_rle: blockType_e = 1;
pub const bt_raw: blockType_e = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_frameHeader {
    pub frameContentSize: libc::c_ulonglong,
    pub windowSize: libc::c_ulonglong,
    pub blockSizeMax: libc::c_uint,
    pub frameType: ZSTD_frameType_e,
    pub headerSize: libc::c_uint,
    pub dictID: libc::c_uint,
    pub checksumFlag: libc::c_uint,
}
/*-
  Buffer-less streaming decompression (synchronous mode)

  A ZSTD_DCtx object is required to track streaming operations.
  Use ZSTD_createDCtx() / ZSTD_freeDCtx() to manage it.
  A ZSTD_DCtx object can be re-used multiple times.

  First typical operation is to retrieve frame parameters, using ZSTD_getFrameHeader().
  Frame header is extracted from the beginning of compressed frame, so providing only the frame's beginning is enough.
  Data fragment must be large enough to ensure successful decoding.
 `ZSTD_frameHeaderSize_max` bytes is guaranteed to always be large enough.
  @result : 0 : successful decoding, the `ZSTD_frameHeader` structure is correctly filled.
           >0 : `srcSize` is too small, please provide at least @result bytes on next attempt.
           errorCode, which can be tested using ZSTD_isError().

  It fills a ZSTD_frameHeader structure with important information to correctly decode the frame,
  such as the dictionary ID, content size, or maximum back-reference distance (`windowSize`).
  Note that these values could be wrong, either because of data corruption, or because a 3rd party deliberately spoofs false information.
  As a consequence, check that values remain within valid application range.
  For example, do not allocate memory blindly, check that `windowSize` is within expectation.
  Each application can set its own limits, depending on local restrictions.
  For extended interoperability, it is recommended to support `windowSize` of at least 8 MB.

  ZSTD_decompressContinue() needs previous data blocks during decompression, up to `windowSize` bytes.
  ZSTD_decompressContinue() is very sensitive to contiguity,
  if 2 blocks don't follow each other, make sure that either the compressor breaks contiguity at the same place,
  or that previous contiguous segment is large enough to properly handle maximum back-reference distance.
  There are multiple ways to guarantee this condition.

  The most memory efficient way is to use a round buffer of sufficient size.
  Sufficient size is determined by invoking ZSTD_decodingBufferSize_min(),
  which can @return an error code if required value is too large for current system (in 32-bits mode).
  In a round buffer methodology, ZSTD_decompressContinue() decompresses each block next to previous one,
  up to the moment there is not enough room left in the buffer to guarantee decoding another full block,
  which maximum size is provided in `ZSTD_frameHeader` structure, field `blockSizeMax`.
  At which point, decoding can resume from the beginning of the buffer.
  Note that already decoded data stored in the buffer should be flushed before being overwritten.

  There are alternatives possible, for example using two or more buffers of size `windowSize` each, though they consume more memory.

  Finally, if you control the compression process, you can also ignore all buffer size rules,
  as long as the encoder and decoder progress in "lock-step",
  aka use exactly the same buffer sizes, break contiguity at the same place, etc.

  Once buffers are setup, start decompression, with ZSTD_decompressBegin().
  If decompression requires a dictionary, use ZSTD_decompressBegin_usingDict() or ZSTD_decompressBegin_usingDDict().

  Then use ZSTD_nextSrcSizeToDecompress() and ZSTD_decompressContinue() alternatively.
  ZSTD_nextSrcSizeToDecompress() tells how many bytes to provide as 'srcSize' to ZSTD_decompressContinue().
  ZSTD_decompressContinue() requires this _exact_ amount of bytes, or it will fail.

 @result of ZSTD_decompressContinue() is the number of bytes regenerated within 'dst' (necessarily <= dstCapacity).
  It can be zero : it just means ZSTD_decompressContinue() has decoded some metadata item.
  It can also be an error code, which can be tested with ZSTD_isError().

  A frame is fully decoded when ZSTD_nextSrcSizeToDecompress() returns zero.
  Context can then be reset to start a new decompression.

  Note : it's possible to know if next input to present is a header or a block, using ZSTD_nextInputType().
  This information is not required to properly decode a frame.

  == Special case : skippable frames ==

  Skippable frames allow integration of user-defined data into a flow of concatenated frames.
  Skippable frames will be ignored (skipped) by decompressor.
  The format of skippable frames is as follows :
  a) Skippable frame ID - 4 Bytes, Little endian format, any value from 0x184D2A50 to 0x184D2A5F
  b) Frame Size - 4 Bytes, Little endian format, unsigned 32-bits
  c) Frame Content - any content (User Data) of length equal to Frame Size
  For skippable frames ZSTD_getFrameHeader() returns zfhPtr->frameType==ZSTD_skippableFrame.
  For skippable frames ZSTD_decompressContinue() always returns 0 : it only skips the content.
*/
/*=====   Buffer-less streaming decompression functions  =====*/
pub type ZSTD_frameType_e = libc::c_uint;
pub const ZSTD_skippableFrame: ZSTD_frameType_e = 1;
pub const ZSTD_frame: ZSTD_frameType_e = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_entropyDTables_t {
    pub LLTable: [ZSTD_seqSymbol; 513],
    pub OFTable: [ZSTD_seqSymbol; 257],
    pub MLTable: [ZSTD_seqSymbol; 513],
    pub hufTable: [HUF_DTable; 4097],
    pub rep: [U32; 3],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_seqSymbol {
    pub nextState: U16,
    pub nbAdditionalBits: BYTE,
    pub nbBits: BYTE,
    pub baseValue: U32,
}
pub type XXH64_hash_t = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct blockProperties_t {
    pub blockType: blockType_e,
    pub lastBlock: U32,
    pub origSize: U32,
}
/*
   xxHash - Extremely Fast Hash algorithm
   Header File
   Copyright (C) 2012-2016, Yann Collet.

   BSD 2-Clause License (http://www.opensource.org/licenses/bsd-license.php)

   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions are
   met:

       * Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.
       * Redistributions in binary form must reproduce the above
   copyright notice, this list of conditions and the following disclaimer
   in the documentation and/or other materials provided with the
   distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
   OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
   SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
   LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
   DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
   THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
   (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
   OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

   You can contact the author at :
   - xxHash source repository : https://github.com/Cyan4973/xxHash
*/
/* Notice extracted from xxHash homepage :

xxHash is an extremely fast Hash algorithm, running at RAM speed limits.
It also successfully passes all tests from the SMHasher suite.

Comparison (single thread, Windows Seven 32 bits, using SMHasher on a Core 2 Duo @3GHz)

Name            Speed       Q.Score   Author
xxHash          5.4 GB/s     10
CrapWow         3.2 GB/s      2       Andrew
MumurHash 3a    2.7 GB/s     10       Austin Appleby
SpookyHash      2.0 GB/s     10       Bob Jenkins
SBox            1.4 GB/s      9       Bret Mulvey
Lookup3         1.2 GB/s      9       Bob Jenkins
SuperFastHash   1.2 GB/s      1       Paul Hsieh
CityHash64      1.05 GB/s    10       Pike & Alakuijala
FNV             0.55 GB/s     5       Fowler, Noll, Vo
CRC32           0.43 GB/s     9
MD5-32          0.33 GB/s    10       Ronald L. Rivest
SHA1-32         0.28 GB/s    10

Q.Score is a measure of quality of the hash function.
It depends on successfully passing SMHasher test set.
10 is a perfect score.

A 64-bits version, named XXH64, is available since r35.
It offers much better speed, but for 64-bits applications only.
Name     Speed on 64 bits    Speed on 32 bits
XXH64       13.8 GB/s            1.9 GB/s
XXH32        6.8 GB/s            6.0 GB/s
*/
/* ****************************
*  Definitions
******************************/
/* size_t */
pub type XXH_errorcode = libc::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
/* ***************************
*  Streaming
****************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_inBuffer_s {
    pub src: *const libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
/*-***************************************************************************
*  Streaming decompression - HowTo
*
*  A ZSTD_DStream object is required to track streaming operations.
*  Use ZSTD_createDStream() and ZSTD_freeDStream() to create/release resources.
*  ZSTD_DStream objects can be re-used multiple times.
*
*  Use ZSTD_initDStream() to start a new decompression operation.
* @return : recommended first input size
*  Alternatively, use advanced API to set specific properties.
*
*  Use ZSTD_decompressStream() repetitively to consume your input.
*  The function will update both `pos` fields.
*  If `input.pos < input.size`, some input has not been consumed.
*  It's up to the caller to present again remaining data.
*  The function tries to flush all data decoded immediately, respecting output buffer size.
*  If `output.pos < output.size`, decoder has flushed everything it could.
*  But if `output.pos == output.size`, there might be some data left within internal buffers.,
*  In which case, call ZSTD_decompressStream() again to flush whatever remains in the buffer.
*  Note : with no additional input provided, amount of data flushed is necessarily <= ZSTD_BLOCKSIZE_MAX.
* @return : 0 when a frame is completely decoded and fully flushed,
*        or an error code, which can be tested using ZSTD_isError(),
*        or any other value > 0, which means there is still some decoding or flushing to do to complete current frame :
*                                the return value is a suggested next input size (just a hint for better latency)
*                                that will never request more than the remaining frame size.
* *******************************************************************************/
/* *< DCtx and DStream are now effectively same object (>= v1.3.0) */
                                 /* For compatibility with versions <= v1.2.0, prefer differentiating them. */
pub type ZSTD_DStream = ZSTD_DCtx;
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
pub const ZSTDnit_block: ZSTD_nextInputType_e = 2;
pub type ZSTD_nextInputType_e = libc::c_uint;
pub const ZSTDnit_skippableFrame: ZSTD_nextInputType_e = 5;
pub const ZSTDnit_checksum: ZSTD_nextInputType_e = 4;
pub const ZSTDnit_lastBlock: ZSTD_nextInputType_e = 3;
pub const ZSTDnit_blockHeader: ZSTD_nextInputType_e = 1;
pub const ZSTDnit_frameHeader: ZSTD_nextInputType_e = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_bounds {
    pub error: size_t,
    pub lowerBound: libc::c_int,
    pub upperBound: libc::c_int,
}
pub type ZSTD_ResetDirective = libc::c_uint;
pub const ZSTD_reset_session_and_parameters: ZSTD_ResetDirective = 3;
pub const ZSTD_reset_parameters: ZSTD_ResetDirective = 2;
pub const ZSTD_reset_session_only: ZSTD_ResetDirective = 1;
/* ============================== */
/*   Advanced decompression API   */
/* ============================== */
/* The advanced API pushes parameters one by one into an existing DCtx context.
 * Parameters are sticky, and remain valid for all following frames
 * using the same DCtx context.
 * It's possible to reset parameters to default values using ZSTD_DCtx_reset().
 * Note : This API is compatible with existing ZSTD_decompressDCtx() and ZSTD_decompressStream().
 *        Therefore, no new decompression function is necessary.
 */
pub type ZSTD_dParameter = libc::c_uint;
/* note : additional experimental parameters are also available
     * within the experimental section of the API.
     * At the time of this writing, they include :
     * ZSTD_c_format
     * Because they are not stable, it's necessary to define ZSTD_STATIC_LINKING_ONLY to access them.
     * note : never ever use experimentalParam? names directly
     */
pub const ZSTD_d_experimentalParam1: ZSTD_dParameter = 1000;
/* Select a size limit (in power of 2) beyond which
                              * the streaming API will refuse to allocate memory buffer
                              * in order to protect the host from unreasonable memory requirements.
                              * This parameter is only useful in streaming mode, since no internal buffer is allocated in single-pass mode.
                              * By default, a decompression context accepts window sizes <= (1 << ZSTD_WINDOWLOG_LIMIT_DEFAULT) */
pub const ZSTD_d_windowLogMax: ZSTD_dParameter = 100;
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
/* MEM_FORCE_MEMORY_ACCESS */
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
}
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return (in_0 as libc::c_ulonglong).swap_bytes() as U64;
}
/*=== Little endian r/w ===*/
unsafe extern "C" fn MEM_readLE16(mut memPtr: *const libc::c_void) -> U16 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read16(memPtr)
    } else {
        let mut p: *const BYTE = memPtr as *const BYTE;
        return (*p.offset(0isize) as libc::c_int +
                    ((*p.offset(1isize) as libc::c_int) << 8i32)) as U16
    };
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
unsafe extern "C" fn ZSTD_cpuid_bmi2(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 8i32 != 0i32 as libc::c_uint) as libc::c_int;
}
/*-****************************************
*  Error codes handling
******************************************/
/* reported already defined on VS 2015 (Rich Geldreich) */
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as
               libc::c_int as libc::c_uint;
}
/* ! ZSTD_decompress() :
 *  `compressedSize` : must be the _exact_ size of some number of compressed and/or skippable frames.
 *  `dstCapacity` is an upper bound of originalSize to regenerate.
 *  If user cannot imply a maximum upper bound, it's better to use streaming mode to decompress data.
 *  @return : the number of bytes decompressed into `dst` (<= `dstCapacity`),
 *            or an errorCode if it fails (which can be tested using ZSTD_isError()). */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompress(mut dst: *mut libc::c_void,
                                         mut dstCapacity: size_t,
                                         mut src: *const libc::c_void,
                                         mut srcSize: size_t) -> size_t {
    let mut regenSize: size_t = 0;
    let dctx: *mut ZSTD_DCtx = ZSTD_createDCtx();
    if dctx.is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    }
    regenSize = ZSTD_decompressDCtx(dctx, dst, dstCapacity, src, srcSize);
    ZSTD_freeDCtx(dctx);
    return regenSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDCtx() -> *mut ZSTD_DCtx {
    return ZSTD_createDCtx_advanced(ZSTD_defaultCMem);
}
/* *< this constant defers to stdlib's functions */
static mut ZSTD_defaultCMem: ZSTD_customMem =
    ZSTD_customMem{customAlloc: None,
                   customFree: None,
                   opaque: 0 as *const libc::c_void as *mut libc::c_void,};
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDCtx_advanced(mut customMem:
                                                      ZSTD_customMem)
 -> *mut ZSTD_DCtx {
    if 0 !=
           customMem.customAlloc.is_none() as libc::c_int ^
               customMem.customFree.is_none() as libc::c_int {
        return 0 as *mut ZSTD_DCtx
    }
    let dctx: *mut ZSTD_DCtx =
        ZSTD_malloc(::std::mem::size_of::<ZSTD_DCtx>() as libc::c_ulong,
                    customMem) as *mut ZSTD_DCtx;
    if dctx.is_null() { return 0 as *mut ZSTD_DCtx }
    (*dctx).customMem = customMem;
    ZSTD_initDCtx_internal(dctx);
    return dctx;
}
unsafe extern "C" fn ZSTD_initDCtx_internal(mut dctx: *mut ZSTD_DCtx) {
    (*dctx).format = ZSTD_f_zstd1;
    (*dctx).staticSize = 0i32 as size_t;
    (*dctx).maxWindowSize =
        ((1i32 as U32) << 27i32).wrapping_add(1i32 as libc::c_uint) as size_t;
    (*dctx).ddict = 0 as *const ZSTD_DDict;
    (*dctx).ddictLocal = 0 as *mut ZSTD_DDict;
    (*dctx).dictEnd = 0 as *const libc::c_void;
    (*dctx).ddictIsCold = 0i32;
    (*dctx).inBuff = 0 as *mut libc::c_char;
    (*dctx).inBuffSize = 0i32 as size_t;
    (*dctx).outBuffSize = 0i32 as size_t;
    (*dctx).streamStage = zdss_init;
    (*dctx).legacyContext = 0 as *mut libc::c_void;
    (*dctx).previousLegacyVersion = 0i32 as U32;
    (*dctx).noForwardProgress = 0i32;
    (*dctx).bmi2 = ZSTD_cpuid_bmi2(ZSTD_cpuid());
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeDCtx(mut dctx: *mut ZSTD_DCtx) -> size_t {
    if dctx.is_null() { return 0i32 as size_t }
    if 0 != (*dctx).staticSize {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    }
    let cMem: ZSTD_customMem = (*dctx).customMem;
    ZSTD_freeDDict((*dctx).ddictLocal);
    (*dctx).ddictLocal = 0 as *mut ZSTD_DDict;
    ZSTD_free((*dctx).inBuff as *mut libc::c_void, cMem);
    (*dctx).inBuff = 0 as *mut libc::c_char;
    ZSTD_free(dctx as *mut libc::c_void, cMem);
    return 0i32 as size_t;
}
/* ! ZSTD_decompressDCtx() :
 *  Same as ZSTD_decompress(),
 *  requires an allocated ZSTD_DCtx.
 *  Compatible with sticky parameters.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressDCtx(mut dctx: *mut ZSTD_DCtx,
                                             mut dst: *mut libc::c_void,
                                             mut dstCapacity: size_t,
                                             mut src: *const libc::c_void,
                                             mut srcSize: size_t) -> size_t {
    return ZSTD_decompress_usingDict(dctx, dst, dstCapacity, src, srcSize,
                                     0 as *const libc::c_void,
                                     0i32 as size_t);
}
/* ! ZSTD_decompress_usingDict() :
 *  Decompression using a known Dictionary.
 *  Dictionary must be identical to the one used during compression.
 *  Note : This function loads the dictionary, resulting in significant startup delay.
 *         It's intended for a dictionary used only once.
 *  Note : When `dict == NULL || dictSize < 8` no dictionary is used. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompress_usingDict(mut dctx: *mut ZSTD_DCtx,
                                                   mut dst: *mut libc::c_void,
                                                   mut dstCapacity: size_t,
                                                   mut src:
                                                       *const libc::c_void,
                                                   mut srcSize: size_t,
                                                   mut dict:
                                                       *const libc::c_void,
                                                   mut dictSize: size_t)
 -> size_t {
    return ZSTD_decompressMultiFrame(dctx, dst, dstCapacity, src, srcSize,
                                     dict, dictSize, 0 as *const ZSTD_DDict);
}
unsafe extern "C" fn ZSTD_decompressMultiFrame(mut dctx: *mut ZSTD_DCtx,
                                               mut dst: *mut libc::c_void,
                                               mut dstCapacity: size_t,
                                               mut src: *const libc::c_void,
                                               mut srcSize: size_t,
                                               mut dict: *const libc::c_void,
                                               mut dictSize: size_t,
                                               mut ddict: *const ZSTD_DDict)
 -> size_t {
    let dststart: *mut libc::c_void = dst;
    let mut moreThan1Frame: libc::c_int = 0i32;
    if !ddict.is_null() {
        dict = ZSTD_DDict_dictContent(ddict);
        dictSize = ZSTD_DDict_dictSize(ddict)
    }
    while srcSize >= 5i32 as libc::c_ulong {
        let magicNumber: U32 = MEM_readLE32(src);
        if magicNumber & 0xfffffff0u32 == 0x184d2a50i32 as libc::c_uint {
            let skippableSize: size_t = readSkippableFrameSize(src, srcSize);
            if 0 != ERR_isError(skippableSize) { return skippableSize }
            if srcSize < skippableSize {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
            }
            src =
                (src as *const BYTE).offset(skippableSize as isize) as
                    *const libc::c_void;
            srcSize =
                (srcSize as libc::c_ulong).wrapping_sub(skippableSize) as
                    size_t as size_t
        } else {
            if !ddict.is_null() {
                let errcod: size_t =
                    ZSTD_decompressBegin_usingDDict(dctx, ddict);
                if 0 != ERR_isError(errcod) { return errcod }
            } else {
                let errcod_0: size_t =
                    ZSTD_decompressBegin_usingDict(dctx, dict, dictSize);
                if 0 != ERR_isError(errcod_0) { return errcod_0 }
            }
            ZSTD_checkContinuity(dctx, dst);
            let res: size_t =
                ZSTD_decompressFrame(dctx, dst, dstCapacity, &mut src,
                                     &mut srcSize);
            if ZSTD_getErrorCode(res) as libc::c_uint ==
                   ZSTD_error_prefix_unknown as libc::c_int as libc::c_uint &&
                   moreThan1Frame == 1i32 {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
            }
            if 0 != ERR_isError(res) { return res }
            dst =
                (dst as *mut BYTE).offset(res as isize) as *mut libc::c_void;
            dstCapacity =
                (dstCapacity as libc::c_ulong).wrapping_sub(res) as size_t as
                    size_t;
            moreThan1Frame = 1i32
        }
    }
    if 0 != srcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    return (dst as *mut BYTE).wrapping_offset_from(dststart as *mut BYTE) as
               libc::c_long as size_t;
}
/* ! ZSTD_decompressFrame() :
 * @dctx must be properly initialized
 *  will update *srcPtr and *srcSizePtr,
 *  to make *srcPtr progress by one frame. */
unsafe extern "C" fn ZSTD_decompressFrame(mut dctx: *mut ZSTD_DCtx,
                                          mut dst: *mut libc::c_void,
                                          mut dstCapacity: size_t,
                                          mut srcPtr:
                                              *mut *const libc::c_void,
                                          mut srcSizePtr: *mut size_t)
 -> size_t {
    let mut ip: *const BYTE = *srcPtr as *const BYTE;
    let ostart: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = ostart.offset(dstCapacity as isize);
    let mut op: *mut BYTE = ostart;
    let mut remainingSrcSize: size_t = *srcSizePtr;
    if remainingSrcSize <
           (6i32 as libc::c_ulong).wrapping_add(ZSTD_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    let frameHeaderSize: size_t =
        ZSTD_frameHeaderSize(ip as *const libc::c_void, 5i32 as size_t);
    if 0 != ERR_isError(frameHeaderSize) { return frameHeaderSize }
    if remainingSrcSize < frameHeaderSize.wrapping_add(ZSTD_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    let errcod: size_t =
        ZSTD_decodeFrameHeader(dctx, ip as *const libc::c_void,
                               frameHeaderSize);
    if 0 != ERR_isError(errcod) { return errcod }
    ip = ip.offset(frameHeaderSize as isize);
    remainingSrcSize =
        (remainingSrcSize as libc::c_ulong).wrapping_sub(frameHeaderSize) as
            size_t as size_t;
    loop  {
        let mut decodedSize: size_t = 0;
        let mut blockProperties: blockProperties_t =
            blockProperties_t{blockType: bt_raw, lastBlock: 0, origSize: 0,};
        let cBlockSize: size_t =
            ZSTD_getcBlockSize(ip as *const libc::c_void, remainingSrcSize,
                               &mut blockProperties);
        if 0 != ERR_isError(cBlockSize) { return cBlockSize }
        ip = ip.offset(ZSTD_blockHeaderSize as isize);
        remainingSrcSize =
            (remainingSrcSize as
                 libc::c_ulong).wrapping_sub(ZSTD_blockHeaderSize) as size_t
                as size_t;
        if cBlockSize > remainingSrcSize {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
        }
        match blockProperties.blockType as libc::c_uint {
            2 => {
                decodedSize =
                    ZSTD_decompressBlock_internal(dctx,
                                                  op as *mut libc::c_void,
                                                  oend.wrapping_offset_from(op)
                                                      as libc::c_long as
                                                      size_t,
                                                  ip as *const libc::c_void,
                                                  cBlockSize, 1i32)
            }
            0 => {
                decodedSize =
                    ZSTD_copyRawBlock(op as *mut libc::c_void,
                                      oend.wrapping_offset_from(op) as
                                          libc::c_long as size_t,
                                      ip as *const libc::c_void, cBlockSize)
            }
            1 => {
                decodedSize =
                    ZSTD_setRleBlock(op as *mut libc::c_void,
                                     oend.wrapping_offset_from(op) as
                                         libc::c_long as size_t, *ip,
                                     blockProperties.origSize as size_t)
            }
            3 | _ => {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            }
        }
        if 0 != ERR_isError(decodedSize) { return decodedSize }
        if 0 != (*dctx).fParams.checksumFlag {
            ZSTD_XXH64_update(&mut (*dctx).xxhState,
                              op as *const libc::c_void, decodedSize);
        }
        op = op.offset(decodedSize as isize);
        ip = ip.offset(cBlockSize as isize);
        remainingSrcSize =
            (remainingSrcSize as libc::c_ulong).wrapping_sub(cBlockSize) as
                size_t as size_t;
        if 0 != blockProperties.lastBlock { break ; }
    }
    if (*dctx).fParams.frameContentSize !=
           0u64.wrapping_sub(1i32 as libc::c_ulonglong) {
        if op.wrapping_offset_from(ostart) as libc::c_long as U64 as
               libc::c_ulonglong != (*dctx).fParams.frameContentSize {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        }
    }
    if 0 != (*dctx).fParams.checksumFlag {
        let checkCalc: U32 = ZSTD_XXH64_digest(&mut (*dctx).xxhState) as U32;
        let mut checkRead: U32 = 0;
        if remainingSrcSize < 4i32 as libc::c_ulong {
            return -(ZSTD_error_checksum_wrong as libc::c_int) as size_t
        }
        checkRead = MEM_readLE32(ip as *const libc::c_void);
        if checkRead != checkCalc {
            return -(ZSTD_error_checksum_wrong as libc::c_int) as size_t
        }
        ip = ip.offset(4isize);
        remainingSrcSize =
            (remainingSrcSize as
                 libc::c_ulong).wrapping_sub(4i32 as libc::c_ulong) as size_t
                as size_t
    }
    *srcPtr = ip as *const libc::c_void;
    *srcSizePtr = remainingSrcSize;
    return op.wrapping_offset_from(ostart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_setRleBlock(mut dst: *mut libc::c_void,
                                      mut dstCapacity: size_t, mut b: BYTE,
                                      mut regenSize: size_t) -> size_t {
    if dst.is_null() {
        if regenSize == 0i32 as libc::c_ulong { return 0i32 as size_t }
        return -(ZSTD_error_dstBuffer_null as libc::c_int) as size_t
    }
    if regenSize > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    memset(dst, b as libc::c_int, regenSize);
    return regenSize;
}
unsafe extern "C" fn ZSTD_copyRawBlock(mut dst: *mut libc::c_void,
                                       mut dstCapacity: size_t,
                                       mut src: *const libc::c_void,
                                       mut srcSize: size_t) -> size_t {
    if dst.is_null() {
        if srcSize == 0i32 as libc::c_ulong { return 0i32 as size_t }
        return -(ZSTD_error_dstBuffer_null as libc::c_int) as size_t
    }
    if srcSize > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    memcpy(dst, src, srcSize);
    return srcSize;
}
/* magic number size */
/* C standard doesn't allow `static const` variable to be init using another `static const` variable */
static mut ZSTD_blockHeaderSize: size_t = 3i32 as size_t;
/* ! ZSTD_frameHeaderSize() :
 *  srcSize must be >= ZSTD_FRAMEHEADERSIZE_PREFIX.
 * @return : size of the Frame Header,
 *           or an error code (if srcSize is too small) */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_frameHeaderSize(mut src: *const libc::c_void,
                                              mut srcSize: size_t) -> size_t {
    return ZSTD_frameHeaderSize_internal(src, srcSize, ZSTD_f_zstd1);
}
/* * ZSTD_frameHeaderSize_internal() :
 *  srcSize must be large enough to reach header size fields.
 *  note : only works for formats ZSTD_f_zstd1 and ZSTD_f_zstd1_magicless.
 * @return : size of the Frame Header
 *           or an error code, which can be tested with ZSTD_isError() */
unsafe extern "C" fn ZSTD_frameHeaderSize_internal(mut src:
                                                       *const libc::c_void,
                                                   mut srcSize: size_t,
                                                   mut format: ZSTD_format_e)
 -> size_t {
    let minInputSize: size_t = ZSTD_startingInputLength(format);
    if srcSize < minInputSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    let fhd: BYTE =
        *(src as
              *const BYTE).offset(minInputSize.wrapping_sub(1i32 as
                                                                libc::c_ulong)
                                      as isize);
    let dictID: U32 = (fhd as libc::c_int & 3i32) as U32;
    let singleSegment: U32 = (fhd as libc::c_int >> 5i32 & 1i32) as U32;
    let fcsId: U32 = (fhd as libc::c_int >> 6i32) as U32;
    return minInputSize.wrapping_add((0 == singleSegment) as libc::c_int as
                                         libc::c_ulong).wrapping_add(ZSTD_did_fieldSize[dictID
                                                                                            as
                                                                                            usize]).wrapping_add(ZSTD_fcs_fieldSize[fcsId
                                                                                                                                        as
                                                                                                                                        usize]).wrapping_add((0
                                                                                                                                                                  !=
                                                                                                                                                                  singleSegment
                                                                                                                                                                  &&
                                                                                                                                                                  0
                                                                                                                                                                      ==
                                                                                                                                                                      fcsId)
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_int
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_ulong);
}
unsafe extern "C" fn ZSTD_startingInputLength(mut format: ZSTD_format_e)
 -> size_t {
    let startingInputLength: size_t =
        (if format as libc::c_uint ==
                ZSTD_f_zstd1_magicless as libc::c_int as libc::c_uint {
             5i32 - 4i32
         } else { 5i32 }) as size_t;
    return startingInputLength;
}
static mut ZSTD_fcs_fieldSize: [size_t; 4] =
    [0i32 as size_t, 2i32 as size_t, 4i32 as size_t, 8i32 as size_t];
static mut ZSTD_did_fieldSize: [size_t; 4] =
    [0i32 as size_t, 1i32 as size_t, 2i32 as size_t, 4i32 as size_t];
/* * ZSTD_decodeFrameHeader() :
 * `headerSize` must be the size provided by ZSTD_frameHeaderSize().
 * @return : 0 if success, or an error code, which can be tested using ZSTD_isError() */
unsafe extern "C" fn ZSTD_decodeFrameHeader(mut dctx: *mut ZSTD_DCtx,
                                            mut src: *const libc::c_void,
                                            mut headerSize: size_t)
 -> size_t {
    let result: size_t =
        ZSTD_getFrameHeader_advanced(&mut (*dctx).fParams, src, headerSize,
                                     (*dctx).format);
    if 0 != ERR_isError(result) { return result }
    if result > 0i32 as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    if 0 != (*dctx).fParams.dictID && (*dctx).dictID != (*dctx).fParams.dictID
       {
        return -(ZSTD_error_dictionary_wrong as libc::c_int) as size_t
    }
    if 0 != (*dctx).fParams.checksumFlag {
        ZSTD_XXH64_reset(&mut (*dctx).xxhState, 0i32 as libc::c_ulonglong);
    }
    return 0i32 as size_t;
}
/* ! ZSTD_getFrameHeader_advanced() :
 *  same as ZSTD_getFrameHeader(),
 *  with added capability to select a format (like ZSTD_f_zstd1_magicless) */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getFrameHeader_advanced(mut zfhPtr:
                                                          *mut ZSTD_frameHeader,
                                                      mut src:
                                                          *const libc::c_void,
                                                      mut srcSize: size_t,
                                                      mut format:
                                                          ZSTD_format_e)
 -> size_t {
    let mut ip: *const BYTE = src as *const BYTE;
    let minInputSize: size_t = ZSTD_startingInputLength(format);
    memset(zfhPtr as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZSTD_frameHeader>() as libc::c_ulong);
    if srcSize < minInputSize { return minInputSize }
    if src == 0 as *mut libc::c_void {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if format as libc::c_uint !=
           ZSTD_f_zstd1_magicless as libc::c_int as libc::c_uint &&
           MEM_readLE32(src) != 0xfd2fb528u32 {
        if MEM_readLE32(src) & 0xfffffff0u32 == 0x184d2a50i32 as libc::c_uint
           {
            if srcSize < 8i32 as libc::c_ulong { return 8i32 as size_t }
            memset(zfhPtr as *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<ZSTD_frameHeader>() as
                       libc::c_ulong);
            (*zfhPtr).frameContentSize =
                MEM_readLE32((src as *const libc::c_char).offset(4isize) as
                                 *const libc::c_void) as libc::c_ulonglong;
            (*zfhPtr).frameType = ZSTD_skippableFrame;
            return 0i32 as size_t
        }
        return -(ZSTD_error_prefix_unknown as libc::c_int) as size_t
    }
    let fhsize: size_t = ZSTD_frameHeaderSize_internal(src, srcSize, format);
    if srcSize < fhsize { return fhsize }
    (*zfhPtr).headerSize = fhsize as U32;
    let fhdByte: BYTE =
        *ip.offset(minInputSize.wrapping_sub(1i32 as libc::c_ulong) as isize);
    let mut pos: size_t = minInputSize;
    let dictIDSizeCode: U32 = (fhdByte as libc::c_int & 3i32) as U32;
    let checksumFlag: U32 = (fhdByte as libc::c_int >> 2i32 & 1i32) as U32;
    let singleSegment: U32 = (fhdByte as libc::c_int >> 5i32 & 1i32) as U32;
    let fcsID: U32 = (fhdByte as libc::c_int >> 6i32) as U32;
    let mut windowSize: U64 = 0i32 as U64;
    let mut dictID: U32 = 0i32 as U32;
    let mut frameContentSize: U64 =
        0u64.wrapping_sub(1i32 as libc::c_ulonglong) as U64;
    if fhdByte as libc::c_int & 0x8i32 != 0i32 {
        return -(ZSTD_error_frameParameter_unsupported as libc::c_int) as
                   size_t
    }
    if 0 == singleSegment {
        let fresh0 = pos;
        pos = pos.wrapping_add(1);
        let wlByte: BYTE = *ip.offset(fresh0 as isize);
        let windowLog: U32 = ((wlByte as libc::c_int >> 3i32) + 10i32) as U32;
        if windowLog >
               (if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                       4i32 as libc::c_ulong {
                    30i32
                } else { 31i32 }) as libc::c_uint {
            return -(ZSTD_error_frameParameter_windowTooLarge as libc::c_int)
                       as size_t
        }
        windowSize = (1u64 << windowLog) as U64;
        windowSize =
            (windowSize as
                 libc::c_ulong).wrapping_add((windowSize >>
                                                  3i32).wrapping_mul((wlByte
                                                                          as
                                                                          libc::c_int
                                                                          &
                                                                          7i32)
                                                                         as
                                                                         libc::c_ulong))
                as U64 as U64
    }
    match dictIDSizeCode {
        1 => {
            dictID = *ip.offset(pos as isize) as U32;
            pos = pos.wrapping_add(1)
        }
        2 => {
            dictID =
                MEM_readLE16(ip.offset(pos as isize) as *const libc::c_void)
                    as U32;
            pos =
                (pos as libc::c_ulong).wrapping_add(2i32 as libc::c_ulong) as
                    size_t as size_t
        }
        3 => {
            dictID =
                MEM_readLE32(ip.offset(pos as isize) as *const libc::c_void);
            pos =
                (pos as libc::c_ulong).wrapping_add(4i32 as libc::c_ulong) as
                    size_t as size_t
        }
        0 | _ => { }
    }
    match fcsID {
        1 => {
            frameContentSize =
                (MEM_readLE16(ip.offset(pos as isize) as *const libc::c_void)
                     as libc::c_int + 256i32) as U64
        }
        2 => {
            frameContentSize =
                MEM_readLE32(ip.offset(pos as isize) as *const libc::c_void)
                    as U64
        }
        3 => {
            frameContentSize =
                MEM_readLE64(ip.offset(pos as isize) as *const libc::c_void)
        }
        0 | _ => {
            if 0 != singleSegment {
                frameContentSize = *ip.offset(pos as isize) as U64
            }
        }
    }
    if 0 != singleSegment { windowSize = frameContentSize }
    (*zfhPtr).frameType = ZSTD_frame;
    (*zfhPtr).frameContentSize = frameContentSize as libc::c_ulonglong;
    (*zfhPtr).windowSize = windowSize as libc::c_ulonglong;
    (*zfhPtr).blockSizeMax =
        (if windowSize < (1i32 << 17i32) as libc::c_ulong {
             windowSize
         } else { (1i32 << 17i32) as libc::c_ulong }) as libc::c_uint;
    (*zfhPtr).dictID = dictID;
    (*zfhPtr).checksumFlag = checksumFlag;
    return 0i32 as size_t;
}
/* ! ZSTD_checkContinuity() :
 *  check if next `dst` follows previous position, where decompression ended.
 *  If yes, do nothing (continue on current segment).
 *  If not, classify previous segment as "external dictionary", and start a new segment.
 *  This function cannot fail. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_checkContinuity(mut dctx: *mut ZSTD_DCtx,
                                              mut dst: *const libc::c_void) {
    if dst != (*dctx).previousDstEnd {
        (*dctx).dictEnd = (*dctx).previousDstEnd;
        (*dctx).virtualStart =
            (dst as
                 *const libc::c_char).offset(-(((*dctx).previousDstEnd as
                                                    *const libc::c_char).wrapping_offset_from((*dctx).prefixStart
                                                                                                  as
                                                                                                  *const libc::c_char)
                                                   as libc::c_long as isize))
                as *const libc::c_void;
        (*dctx).prefixStart = dst;
        (*dctx).previousDstEnd = dst
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBegin_usingDict(mut dctx:
                                                            *mut ZSTD_DCtx,
                                                        mut dict:
                                                            *const libc::c_void,
                                                        mut dictSize: size_t)
 -> size_t {
    let errcod: size_t = ZSTD_decompressBegin(dctx);
    if 0 != ERR_isError(errcod) { return errcod }
    if !dict.is_null() && 0 != dictSize {
        let errcod_0: size_t =
            ZSTD_decompress_insertDictionary(dctx, dict, dictSize);
        if 0 != ERR_isError(errcod_0) {
            return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
        }
    }
    return 0i32 as size_t;
}
unsafe extern "C" fn ZSTD_decompress_insertDictionary(mut dctx:
                                                          *mut ZSTD_DCtx,
                                                      mut dict:
                                                          *const libc::c_void,
                                                      mut dictSize: size_t)
 -> size_t {
    if dictSize < 8i32 as libc::c_ulong {
        return ZSTD_refDictContent(dctx, dict, dictSize)
    }
    let magic: U32 = MEM_readLE32(dict);
    if magic != 0xec30a437u32 {
        return ZSTD_refDictContent(dctx, dict, dictSize)
    }
    (*dctx).dictID =
        MEM_readLE32((dict as *const libc::c_char).offset(4isize) as
                         *const libc::c_void);
    let eSize: size_t =
        ZSTD_loadDEntropy(&mut (*dctx).entropy, dict, dictSize);
    if 0 != ERR_isError(eSize) {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    dict =
        (dict as *const libc::c_char).offset(eSize as isize) as
            *const libc::c_void;
    dictSize =
        (dictSize as libc::c_ulong).wrapping_sub(eSize) as size_t as size_t;
    (*dctx).fseEntropy = 1i32 as U32;
    (*dctx).litEntropy = (*dctx).fseEntropy;
    return ZSTD_refDictContent(dctx, dict, dictSize);
}
unsafe extern "C" fn ZSTD_refDictContent(mut dctx: *mut ZSTD_DCtx,
                                         mut dict: *const libc::c_void,
                                         mut dictSize: size_t) -> size_t {
    (*dctx).dictEnd = (*dctx).previousDstEnd;
    (*dctx).virtualStart =
        (dict as
             *const libc::c_char).offset(-(((*dctx).previousDstEnd as
                                                *const libc::c_char).wrapping_offset_from((*dctx).prefixStart
                                                                                              as
                                                                                              *const libc::c_char)
                                               as libc::c_long as isize)) as
            *const libc::c_void;
    (*dctx).prefixStart = dict;
    (*dctx).previousDstEnd =
        (dict as *const libc::c_char).offset(dictSize as isize) as
            *const libc::c_void;
    return 0i32 as size_t;
}
/* typedef'd to ZSTD_DCtx within "zstd.h" */
/*-*******************************************************
 *  Shared internal functions
 *********************************************************/
/* ! ZSTD_loadDEntropy() :
 *  dict : must point at beginning of a valid zstd dictionary.
 * @return : size of entropy tables read */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_loadDEntropy(mut entropy:
                                               *mut ZSTD_entropyDTables_t,
                                           dict: *const libc::c_void,
                                           dictSize: size_t) -> size_t {
    let mut dictPtr: *const BYTE = dict as *const BYTE;
    let dictEnd: *const BYTE = dictPtr.offset(dictSize as isize);
    if dictSize <= 8i32 as libc::c_ulong {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    dictPtr = dictPtr.offset(8isize);
    let workspace: *mut libc::c_void =
        &mut (*entropy).LLTable as *mut [ZSTD_seqSymbol; 513] as
            *mut libc::c_void;
    let workspaceSize: size_t =
        (::std::mem::size_of::<[ZSTD_seqSymbol; 513]>() as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<[ZSTD_seqSymbol; 257]>()
                                             as
                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<[ZSTD_seqSymbol; 513]>()
                                                                             as
                                                                             libc::c_ulong);
    let hSize: size_t =
        HUF_readDTableX2_wksp((*entropy).hufTable.as_mut_ptr(),
                              dictPtr as *const libc::c_void,
                              dictEnd.wrapping_offset_from(dictPtr) as
                                  libc::c_long as size_t, workspace,
                              workspaceSize);
    if 0 != ERR_isError(hSize) {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    dictPtr = dictPtr.offset(hSize as isize);
    let mut offcodeNCount: [libc::c_short; 32] = [0; 32];
    let mut offcodeMaxValue: libc::c_uint = 31i32 as libc::c_uint;
    let mut offcodeLog: libc::c_uint = 0;
    let offcodeHeaderSize: size_t =
        FSE_readNCount(offcodeNCount.as_mut_ptr(), &mut offcodeMaxValue,
                       &mut offcodeLog, dictPtr as *const libc::c_void,
                       dictEnd.wrapping_offset_from(dictPtr) as libc::c_long
                           as size_t);
    if 0 != ERR_isError(offcodeHeaderSize) {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    if offcodeMaxValue > 31i32 as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    if offcodeLog > 8i32 as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    ZSTD_buildFSETable((*entropy).OFTable.as_mut_ptr(),
                       offcodeNCount.as_mut_ptr(), offcodeMaxValue,
                       OF_base.as_ptr(), OF_bits.as_ptr(), offcodeLog);
    dictPtr = dictPtr.offset(offcodeHeaderSize as isize);
    let mut matchlengthNCount: [libc::c_short; 53] = [0; 53];
    let mut matchlengthMaxValue: libc::c_uint = 52i32 as libc::c_uint;
    let mut matchlengthLog: libc::c_uint = 0;
    let matchlengthHeaderSize: size_t =
        FSE_readNCount(matchlengthNCount.as_mut_ptr(),
                       &mut matchlengthMaxValue, &mut matchlengthLog,
                       dictPtr as *const libc::c_void,
                       dictEnd.wrapping_offset_from(dictPtr) as libc::c_long
                           as size_t);
    if 0 != ERR_isError(matchlengthHeaderSize) {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    if matchlengthMaxValue > 52i32 as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    if matchlengthLog > 9i32 as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    ZSTD_buildFSETable((*entropy).MLTable.as_mut_ptr(),
                       matchlengthNCount.as_mut_ptr(), matchlengthMaxValue,
                       ML_base.as_ptr(), ML_bits.as_ptr(), matchlengthLog);
    dictPtr = dictPtr.offset(matchlengthHeaderSize as isize);
    let mut litlengthNCount: [libc::c_short; 36] = [0; 36];
    let mut litlengthMaxValue: libc::c_uint = 35i32 as libc::c_uint;
    let mut litlengthLog: libc::c_uint = 0;
    let litlengthHeaderSize: size_t =
        FSE_readNCount(litlengthNCount.as_mut_ptr(), &mut litlengthMaxValue,
                       &mut litlengthLog, dictPtr as *const libc::c_void,
                       dictEnd.wrapping_offset_from(dictPtr) as libc::c_long
                           as size_t);
    if 0 != ERR_isError(litlengthHeaderSize) {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    if litlengthMaxValue > 35i32 as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    if litlengthLog > 9i32 as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    ZSTD_buildFSETable((*entropy).LLTable.as_mut_ptr(),
                       litlengthNCount.as_mut_ptr(), litlengthMaxValue,
                       LL_base.as_ptr(), LL_bits.as_ptr(), litlengthLog);
    dictPtr = dictPtr.offset(litlengthHeaderSize as isize);
    if dictPtr.offset(12isize) > dictEnd {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    let mut i: libc::c_int = 0;
    let dictContentSize: size_t =
        dictEnd.wrapping_offset_from(dictPtr.offset(12isize)) as libc::c_long
            as size_t;
    i = 0i32;
    while i < 3i32 {
        let rep: U32 = MEM_readLE32(dictPtr as *const libc::c_void);
        dictPtr = dictPtr.offset(4isize);
        if rep == 0i32 as libc::c_uint ||
               rep as libc::c_ulong >= dictContentSize {
            return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
        }
        (*entropy).rep[i as usize] = rep;
        i += 1
    }
    return dictPtr.wrapping_offset_from(dict as *const BYTE) as libc::c_long
               as size_t;
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
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* zstd_decompress_internal:
 * objects and definitions shared within lib/decompress modules */
/*-*******************************************************
 *  Dependencies
 *********************************************************/
/* BYTE, U16, U32 */
/* ZSTD_seqSymbol */
/*-*******************************************************
 *  Constants
 *********************************************************/
static mut LL_base: [U32; 36] =
    [0i32 as U32, 1i32 as U32, 2i32 as U32, 3i32 as U32, 4i32 as U32,
     5i32 as U32, 6i32 as U32, 7i32 as U32, 8i32 as U32, 9i32 as U32,
     10i32 as U32, 11i32 as U32, 12i32 as U32, 13i32 as U32, 14i32 as U32,
     15i32 as U32, 16i32 as U32, 18i32 as U32, 20i32 as U32, 22i32 as U32,
     24i32 as U32, 28i32 as U32, 32i32 as U32, 40i32 as U32, 48i32 as U32,
     64i32 as U32, 0x80i32 as U32, 0x100i32 as U32, 0x200i32 as U32,
     0x400i32 as U32, 0x800i32 as U32, 0x1000i32 as U32, 0x2000i32 as U32,
     0x4000i32 as U32, 0x8000i32 as U32, 0x10000i32 as U32];
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
static mut ML_base: [U32; 53] =
    [3i32 as U32, 4i32 as U32, 5i32 as U32, 6i32 as U32, 7i32 as U32,
     8i32 as U32, 9i32 as U32, 10i32 as U32, 11i32 as U32, 12i32 as U32,
     13i32 as U32, 14i32 as U32, 15i32 as U32, 16i32 as U32, 17i32 as U32,
     18i32 as U32, 19i32 as U32, 20i32 as U32, 21i32 as U32, 22i32 as U32,
     23i32 as U32, 24i32 as U32, 25i32 as U32, 26i32 as U32, 27i32 as U32,
     28i32 as U32, 29i32 as U32, 30i32 as U32, 31i32 as U32, 32i32 as U32,
     33i32 as U32, 34i32 as U32, 35i32 as U32, 37i32 as U32, 39i32 as U32,
     41i32 as U32, 43i32 as U32, 47i32 as U32, 51i32 as U32, 59i32 as U32,
     67i32 as U32, 83i32 as U32, 99i32 as U32, 0x83i32 as U32,
     0x103i32 as U32, 0x203i32 as U32, 0x403i32 as U32, 0x803i32 as U32,
     0x1003i32 as U32, 0x2003i32 as U32, 0x4003i32 as U32, 0x8003i32 as U32,
     0x10003i32 as U32];
static mut OF_bits: [U32; 32] =
    [0i32 as U32, 1i32 as U32, 2i32 as U32, 3i32 as U32, 4i32 as U32,
     5i32 as U32, 6i32 as U32, 7i32 as U32, 8i32 as U32, 9i32 as U32,
     10i32 as U32, 11i32 as U32, 12i32 as U32, 13i32 as U32, 14i32 as U32,
     15i32 as U32, 16i32 as U32, 17i32 as U32, 18i32 as U32, 19i32 as U32,
     20i32 as U32, 21i32 as U32, 22i32 as U32, 23i32 as U32, 24i32 as U32,
     25i32 as U32, 26i32 as U32, 27i32 as U32, 28i32 as U32, 29i32 as U32,
     30i32 as U32, 31i32 as U32];
static mut OF_base: [U32; 32] =
    [0i32 as U32, 1i32 as U32, 1i32 as U32, 5i32 as U32, 0xdi32 as U32,
     0x1di32 as U32, 0x3di32 as U32, 0x7di32 as U32, 0xfdi32 as U32,
     0x1fdi32 as U32, 0x3fdi32 as U32, 0x7fdi32 as U32, 0xffdi32 as U32,
     0x1ffdi32 as U32, 0x3ffdi32 as U32, 0x7ffdi32 as U32, 0xfffdi32 as U32,
     0x1fffdi32 as U32, 0x3fffdi32 as U32, 0x7fffdi32 as U32,
     0xffffdi32 as U32, 0x1ffffdi32 as U32, 0x3ffffdi32 as U32,
     0x7ffffdi32 as U32, 0xfffffdi32 as U32, 0x1fffffdi32 as U32,
     0x3fffffdi32 as U32, 0x7fffffdi32 as U32, 0xffffffdi32 as U32,
     0x1ffffffdi32 as U32, 0x3ffffffdi32 as U32, 0x7ffffffdi32 as U32];
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBegin(mut dctx: *mut ZSTD_DCtx)
 -> size_t {
    (*dctx).expected = ZSTD_startingInputLength((*dctx).format);
    (*dctx).stage = ZSTDds_getFrameHeaderSize;
    (*dctx).decodedSize = 0i32 as U64;
    (*dctx).previousDstEnd = 0 as *const libc::c_void;
    (*dctx).prefixStart = 0 as *const libc::c_void;
    (*dctx).virtualStart = 0 as *const libc::c_void;
    (*dctx).dictEnd = 0 as *const libc::c_void;
    (*dctx).entropy.hufTable[0usize] = (12i32 * 0x1000001i32) as HUF_DTable;
    (*dctx).fseEntropy = 0i32 as U32;
    (*dctx).litEntropy = (*dctx).fseEntropy;
    (*dctx).dictID = 0i32 as U32;
    memcpy((*dctx).entropy.rep.as_mut_ptr() as *mut libc::c_void,
           repStartValue.as_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[U32; 3]>() as libc::c_ulong);
    (*dctx).LLTptr = (*dctx).entropy.LLTable.as_mut_ptr();
    (*dctx).MLTptr = (*dctx).entropy.MLTable.as_mut_ptr();
    (*dctx).OFTptr = (*dctx).entropy.OFTable.as_mut_ptr();
    (*dctx).HUFptr = (*dctx).entropy.hufTable.as_mut_ptr();
    return 0i32 as size_t;
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
/* this module contains definitions which must be identical
 * across compression, decompression and dictBuilder.
 * It also contains a few functions useful to at least 2 of them
 * and which benefit from being inlined */
/*-*************************************
*  Dependencies
***************************************/
/* assert, DEBUGLOG, RAWLOG, g_debuglevel */
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
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBegin_usingDDict(mut dctx:
                                                             *mut ZSTD_DCtx,
                                                         mut ddict:
                                                             *const ZSTD_DDict)
 -> size_t {
    if !ddict.is_null() {
        let dictStart: *const libc::c_char =
            ZSTD_DDict_dictContent(ddict) as *const libc::c_char;
        let dictSize: size_t = ZSTD_DDict_dictSize(ddict);
        let dictEnd: *const libc::c_void =
            dictStart.offset(dictSize as isize) as *const libc::c_void;
        (*dctx).ddictIsCold = ((*dctx).dictEnd != dictEnd) as libc::c_int
    }
    let errcod: size_t = ZSTD_decompressBegin(dctx);
    if 0 != ERR_isError(errcod) { return errcod }
    if !ddict.is_null() { ZSTD_copyDDictParameters(dctx, ddict); }
    return 0i32 as size_t;
}
unsafe extern "C" fn readSkippableFrameSize(mut src: *const libc::c_void,
                                            mut srcSize: size_t) -> size_t {
    let skippableHeaderSize: size_t = 8i32 as size_t;
    let mut sizeU32: U32 = 0;
    if srcSize < 8i32 as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    sizeU32 =
        MEM_readLE32((src as *const BYTE).offset(4isize) as
                         *const libc::c_void);
    if sizeU32.wrapping_add(8i32 as libc::c_uint) < sizeU32 {
        return -(ZSTD_error_frameParameter_unsupported as libc::c_int) as
                   size_t
    }
    return skippableHeaderSize.wrapping_add(sizeU32 as libc::c_ulong);
}
/* ! ZSTD_getFrameContentSize() : requires v1.3.0+
 *  `src` should point to the start of a ZSTD encoded frame.
 *  `srcSize` must be at least as large as the frame header.
 *            hint : any size >= `ZSTD_frameHeaderSize_max` is large enough.
 *  @return : - decompressed size of `src` frame content, if known
 *            - ZSTD_CONTENTSIZE_UNKNOWN if the size cannot be determined
 *            - ZSTD_CONTENTSIZE_ERROR if an error occurred (e.g. invalid magic number, srcSize too small)
 *   note 1 : a 0 return value means the frame is valid but "empty".
 *   note 2 : decompressed size is an optional field, it may not be present, typically in streaming mode.
 *            When `return==ZSTD_CONTENTSIZE_UNKNOWN`, data to decompress could be any size.
 *            In which case, it's necessary to use streaming mode to decompress data.
 *            Optionally, application can rely on some implicit limit,
 *            as ZSTD_decompress() only needs an upper bound of decompressed size.
 *            (For example, data could be necessarily cut into blocks <= 16 KB).
 *   note 3 : decompressed size is always present when compression is completed using single-pass functions,
 *            such as ZSTD_compress(), ZSTD_compressCCtx() ZSTD_compress_usingDict() or ZSTD_compress_usingCDict().
 *   note 4 : decompressed size can be very large (64-bits value),
 *            potentially larger than what local system can handle as a single memory segment.
 *            In which case, it's necessary to use streaming mode to decompress data.
 *   note 5 : If source is untrusted, decompressed size could be wrong or intentionally modified.
 *            Always ensure return value fits within application's authorized limits.
 *            Each application can set its own limits.
 *   note 6 : This function replaces ZSTD_getDecompressedSize() */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getFrameContentSize(mut src:
                                                      *const libc::c_void,
                                                  mut srcSize: size_t)
 -> libc::c_ulonglong {
    let mut zfh: ZSTD_frameHeader =
        ZSTD_frameHeader{frameContentSize: 0,
                         windowSize: 0,
                         blockSizeMax: 0,
                         frameType: ZSTD_frame,
                         headerSize: 0,
                         dictID: 0,
                         checksumFlag: 0,};
    if ZSTD_getFrameHeader(&mut zfh, src, srcSize) != 0i32 as libc::c_ulong {
        return 0u64.wrapping_sub(2i32 as libc::c_ulonglong)
    }
    if zfh.frameType as libc::c_uint ==
           ZSTD_skippableFrame as libc::c_int as libc::c_uint {
        return 0i32 as libc::c_ulonglong
    } else { return zfh.frameContentSize };
}
/* * ZSTD_getFrameHeader() :
 *  decode Frame Header, or requires larger `srcSize`.
 * @return : 0, `zfhPtr` is correctly filled,
 *          >0, `srcSize` is too small, value is wanted `srcSize` amount,
 *           or an error code, which can be tested using ZSTD_isError() */
/* *< doesn't consume input */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getFrameHeader(mut zfhPtr:
                                                 *mut ZSTD_frameHeader,
                                             mut src: *const libc::c_void,
                                             mut srcSize: size_t) -> size_t {
    return ZSTD_getFrameHeader_advanced(zfhPtr, src, srcSize, ZSTD_f_zstd1);
}
/* ! ZSTD_getDecompressedSize() :
 *  NOTE: This function is now obsolete, in favor of ZSTD_getFrameContentSize().
 *  Both functions work the same way, but ZSTD_getDecompressedSize() blends
 *  "empty", "unknown" and "error" results to the same return value (0),
 *  while ZSTD_getFrameContentSize() gives them separate return values.
 * @return : decompressed size of `src` frame content _if known and not empty_, 0 otherwise. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDecompressedSize(mut src:
                                                      *const libc::c_void,
                                                  mut srcSize: size_t)
 -> libc::c_ulonglong {
    let ret: libc::c_ulonglong = ZSTD_getFrameContentSize(src, srcSize);
    return if ret >= 0u64.wrapping_sub(2i32 as libc::c_ulonglong) {
               0i32 as libc::c_ulonglong
           } else { ret };
}
/* ! ZSTD_decompress_usingDDict() :
 *  Decompression using a digested Dictionary.
 *  Recommended when same dictionary is used multiple times. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompress_usingDDict(mut dctx: *mut ZSTD_DCtx,
                                                    mut dst:
                                                        *mut libc::c_void,
                                                    mut dstCapacity: size_t,
                                                    mut src:
                                                        *const libc::c_void,
                                                    mut srcSize: size_t,
                                                    mut ddict:
                                                        *const ZSTD_DDict)
 -> size_t {
    return ZSTD_decompressMultiFrame(dctx, dst, dstCapacity, src, srcSize,
                                     0 as *const libc::c_void, 0i32 as size_t,
                                     ddict);
}
/*===== ZSTD_DStream management functions =====*/
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDStream() -> *mut ZSTD_DStream {
    return ZSTD_createDStream_advanced(ZSTD_defaultCMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDStream_advanced(mut customMem:
                                                         ZSTD_customMem)
 -> *mut ZSTD_DStream {
    return ZSTD_createDCtx_advanced(customMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeDStream(mut zds: *mut ZSTD_DStream)
 -> size_t {
    return ZSTD_freeDCtx(zds);
}
/*===== Streaming decompression functions =====*/
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initDStream(mut zds: *mut ZSTD_DStream)
 -> size_t {
    return ZSTD_initDStream_usingDict(zds, 0 as *const libc::c_void,
                                      0i32 as size_t);
}
/*=====   Advanced Streaming decompression functions  =====*/
/* *< note: no dictionary will be used if dict == NULL or dictSize < 8 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initDStream_usingDict(mut zds:
                                                        *mut ZSTD_DStream,
                                                    mut dict:
                                                        *const libc::c_void,
                                                    mut dictSize: size_t)
 -> size_t {
    (*zds).streamStage = zdss_init;
    (*zds).noForwardProgress = 0i32;
    let errcod: size_t = ZSTD_DCtx_loadDictionary(zds, dict, dictSize);
    if 0 != ERR_isError(errcod) { return errcod }
    return 5i32 as size_t;
}
/* ! ZSTD_DCtx_loadDictionary() :
 *  Create an internal DDict from dict buffer,
 *  to be used to decompress next frames.
 *  The dictionary remains valid for all future frames, until explicitly invalidated.
 * @result : 0, or an error code (which can be tested with ZSTD_isError()).
 *  Special : Adding a NULL (or 0-size) dictionary invalidates any previous dictionary,
 *            meaning "return to no-dictionary mode".
 *  Note 1 : Loading a dictionary involves building tables,
 *           which has a non-negligible impact on CPU usage and latency.
 *           It's recommended to "load once, use many times", to amortize the cost
 *  Note 2 :`dict` content will be copied internally, so `dict` can be released after loading.
 *           Use ZSTD_DCtx_loadDictionary_byReference() to reference dictionary content instead.
 *  Note 3 : Use ZSTD_DCtx_loadDictionary_advanced() to take control of
 *           how dictionary content is loaded and interpreted.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_loadDictionary(mut dctx: *mut ZSTD_DCtx,
                                                  mut dict:
                                                      *const libc::c_void,
                                                  mut dictSize: size_t)
 -> size_t {
    return ZSTD_DCtx_loadDictionary_advanced(dctx, dict, dictSize,
                                             ZSTD_dlm_byCopy, ZSTD_dct_auto);
}
/* ! ZSTD_DCtx_loadDictionary_advanced() :
 *  Same as ZSTD_DCtx_loadDictionary(),
 *  but gives direct control over
 *  how to load the dictionary (by copy ? by reference ?)
 *  and how to interpret it (automatic ? force raw mode ? full mode only ?). */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_loadDictionary_advanced(mut dctx:
                                                               *mut ZSTD_DCtx,
                                                           mut dict:
                                                               *const libc::c_void,
                                                           mut dictSize:
                                                               size_t,
                                                           mut dictLoadMethod:
                                                               ZSTD_dictLoadMethod_e,
                                                           mut dictContentType:
                                                               ZSTD_dictContentType_e)
 -> size_t {
    if (*dctx).streamStage as libc::c_uint !=
           zdss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    }
    ZSTD_freeDDict((*dctx).ddictLocal);
    if !dict.is_null() && dictSize >= 8i32 as libc::c_ulong {
        (*dctx).ddictLocal =
            ZSTD_createDDict_advanced(dict, dictSize, dictLoadMethod,
                                      dictContentType, (*dctx).customMem);
        if (*dctx).ddictLocal.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        }
    } else { (*dctx).ddictLocal = 0 as *mut ZSTD_DDict }
    (*dctx).ddict = (*dctx).ddictLocal;
    return 0i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressStream(mut zds: *mut ZSTD_DStream,
                                               mut output:
                                                   *mut ZSTD_outBuffer,
                                               mut input: *mut ZSTD_inBuffer)
 -> size_t {
    let istart: *const libc::c_char =
        ((*input).src as *const libc::c_char).offset((*input).pos as isize);
    let iend: *const libc::c_char =
        ((*input).src as *const libc::c_char).offset((*input).size as isize);
    let mut ip: *const libc::c_char = istart;
    let ostart: *mut libc::c_char =
        ((*output).dst as *mut libc::c_char).offset((*output).pos as isize);
    let oend: *mut libc::c_char =
        ((*output).dst as *mut libc::c_char).offset((*output).size as isize);
    let mut op: *mut libc::c_char = ostart;
    let mut someMoreWork: U32 = 1i32 as U32;
    if (*input).pos > (*input).size {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    if (*output).pos > (*output).size {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    while 0 != someMoreWork {
        let mut current_block_131: u64;
        match (*zds).streamStage as libc::c_uint {
            0 => {
                ZSTD_resetDStream(zds);
                /* fall-through */
                current_block_131 = 11307063007268554308;
            }
            1 => { current_block_131 = 11307063007268554308; }
            2 => { current_block_131 = 1658462350791934405; }
            3 => { current_block_131 = 13740693533991687037; }
            4 => { current_block_131 = 850175865728824115; }
            _ => { return -(ZSTD_error_GENERIC as libc::c_int) as size_t }
        }
        match current_block_131 {
            11307063007268554308 => {
                let hSize: size_t =
                    ZSTD_getFrameHeader_advanced(&mut (*zds).fParams,
                                                 (*zds).headerBuffer.as_mut_ptr()
                                                     as *const libc::c_void,
                                                 (*zds).lhSize,
                                                 (*zds).format);
                if 0 != ERR_isError(hSize) { return hSize }
                if hSize != 0i32 as libc::c_ulong {
                    /* need more input */
                    /* if hSize!=0, hSize > zds->lhSize */
                    let toLoad: size_t = hSize.wrapping_sub((*zds).lhSize);
                    let remainingInput: size_t =
                        iend.wrapping_offset_from(ip) as libc::c_long as
                            size_t;
                    if toLoad > remainingInput {
                        if remainingInput > 0i32 as libc::c_ulong {
                            memcpy((*zds).headerBuffer.as_mut_ptr().offset((*zds).lhSize
                                                                               as
                                                                               isize)
                                       as *mut libc::c_void,
                                   ip as *const libc::c_void, remainingInput);
                            (*zds).lhSize =
                                ((*zds).lhSize as
                                     libc::c_ulong).wrapping_add(remainingInput)
                                    as size_t as size_t
                        }
                        (*input).pos = (*input).size;
                        return if 6i32 as libc::c_ulong > hSize {
                                   6i32 as libc::c_ulong
                               } else {
                                   hSize
                               }.wrapping_sub((*zds).lhSize).wrapping_add(ZSTD_blockHeaderSize)
                    }
                    memcpy((*zds).headerBuffer.as_mut_ptr().offset((*zds).lhSize
                                                                       as
                                                                       isize)
                               as *mut libc::c_void,
                           ip as *const libc::c_void, toLoad);
                    (*zds).lhSize = hSize;
                    ip = ip.offset(toLoad as isize);
                    current_block_131 = 1745632252074978848;
                } else {
                    /* check for single-pass mode opportunity */
                    /* skippable frame if == 0 */
                    if 0 != (*zds).fParams.frameContentSize &&
                           0 != (*zds).fParams.windowSize &&
                           oend.wrapping_offset_from(op) as libc::c_long as
                               size_t as libc::c_ulonglong >=
                               (*zds).fParams.frameContentSize {
                        let cSize: size_t =
                            ZSTD_findFrameCompressedSize(istart as
                                                             *const libc::c_void,
                                                         iend.wrapping_offset_from(istart)
                                                             as libc::c_long
                                                             as size_t);
                        if cSize <=
                               iend.wrapping_offset_from(istart) as
                                   libc::c_long as size_t {
                            /* shortcut : using single-pass mode */
                            let decompressedSize: size_t =
                                ZSTD_decompress_usingDDict(zds,
                                                           op as
                                                               *mut libc::c_void,
                                                           oend.wrapping_offset_from(op)
                                                               as libc::c_long
                                                               as size_t,
                                                           istart as
                                                               *const libc::c_void,
                                                           cSize,
                                                           (*zds).ddict);
                            if 0 != ERR_isError(decompressedSize) {
                                return decompressedSize
                            }
                            ip = istart.offset(cSize as isize);
                            op = op.offset(decompressedSize as isize);
                            (*zds).expected = 0i32 as size_t;
                            (*zds).streamStage = zdss_init;
                            someMoreWork = 0i32 as U32;
                            current_block_131 = 1745632252074978848;
                        } else { current_block_131 = 2500484646272006982; }
                    } else { current_block_131 = 2500484646272006982; }
                    match current_block_131 {
                        1745632252074978848 => { }
                        _ => {
                            let errcod: size_t =
                                ZSTD_decompressBegin_usingDDict(zds,
                                                                (*zds).ddict);
                            if 0 != ERR_isError(errcod) { return errcod }
                            if MEM_readLE32((*zds).headerBuffer.as_mut_ptr()
                                                as *const libc::c_void) &
                                   0xfffffff0u32 ==
                                   0x184d2a50i32 as libc::c_uint {
                                (*zds).expected =
                                    MEM_readLE32((*zds).headerBuffer.as_mut_ptr().offset(4isize)
                                                     as *const libc::c_void)
                                        as size_t;
                                (*zds).stage = ZSTDds_skipFrame
                            } else {
                                let errcod_0: size_t =
                                    ZSTD_decodeFrameHeader(zds,
                                                           (*zds).headerBuffer.as_mut_ptr()
                                                               as
                                                               *const libc::c_void,
                                                           (*zds).lhSize);
                                if 0 != ERR_isError(errcod_0) {
                                    return errcod_0
                                }
                                (*zds).expected = ZSTD_blockHeaderSize;
                                (*zds).stage = ZSTDds_decodeBlockHeader
                            }
                            (*zds).fParams.windowSize =
                                if (*zds).fParams.windowSize >
                                       (1u32 << 10i32) as libc::c_ulonglong {
                                    (*zds).fParams.windowSize
                                } else {
                                    (1u32 << 10i32) as libc::c_ulonglong
                                };
                            if (*zds).fParams.windowSize >
                                   (*zds).maxWindowSize as libc::c_ulonglong {
                                return -(ZSTD_error_frameParameter_windowTooLarge
                                             as libc::c_int) as size_t
                            }
                            let neededInBuffSize: size_t =
                                (if (*zds).fParams.blockSizeMax >
                                        4i32 as libc::c_uint {
                                     (*zds).fParams.blockSizeMax
                                 } else { 4i32 as libc::c_uint }) as size_t;
                            let neededOutBuffSize: size_t =
                                ZSTD_decodingBufferSize_min((*zds).fParams.windowSize,
                                                            (*zds).fParams.frameContentSize);
                            if (*zds).inBuffSize < neededInBuffSize ||
                                   (*zds).outBuffSize < neededOutBuffSize {
                                let bufferSize: size_t =
                                    neededInBuffSize.wrapping_add(neededOutBuffSize);
                                if 0 != (*zds).staticSize {
                                    if bufferSize >
                                           (*zds).staticSize.wrapping_sub(::std::mem::size_of::<ZSTD_DCtx>()
                                                                              as
                                                                              libc::c_ulong)
                                       {
                                        return -(ZSTD_error_memory_allocation
                                                     as libc::c_int) as size_t
                                    }
                                } else {
                                    ZSTD_free((*zds).inBuff as
                                                  *mut libc::c_void,
                                              (*zds).customMem);
                                    (*zds).inBuffSize = 0i32 as size_t;
                                    (*zds).outBuffSize = 0i32 as size_t;
                                    (*zds).inBuff =
                                        ZSTD_malloc(bufferSize,
                                                    (*zds).customMem) as
                                            *mut libc::c_char;
                                    if (*zds).inBuff.is_null() {
                                        return -(ZSTD_error_memory_allocation
                                                     as libc::c_int) as size_t
                                    }
                                }
                                (*zds).inBuffSize = neededInBuffSize;
                                (*zds).outBuff =
                                    (*zds).inBuff.offset((*zds).inBuffSize as
                                                             isize);
                                (*zds).outBuffSize = neededOutBuffSize
                            }
                            (*zds).streamStage = zdss_read;
                            /* fall-through */
                            current_block_131 = 1658462350791934405;
                        }
                    }
                }
            }
            _ => { }
        }
        match current_block_131 {
            1658462350791934405 => {
                let neededInSize: size_t = ZSTD_nextSrcSizeToDecompress(zds);
                if neededInSize == 0i32 as libc::c_ulong {
                    /* end of frame */
                    (*zds).streamStage = zdss_init;
                    someMoreWork = 0i32 as U32;
                    current_block_131 = 1745632252074978848;
                } else if iend.wrapping_offset_from(ip) as libc::c_long as
                              size_t >= neededInSize {
                    /* decode directly from src */
                    let isSkipFrame: libc::c_int = ZSTD_isSkipFrame(zds);
                    let decodedSize: size_t =
                        ZSTD_decompressContinue(zds,
                                                (*zds).outBuff.offset((*zds).outStart
                                                                          as
                                                                          isize)
                                                    as *mut libc::c_void,
                                                if 0 != isSkipFrame {
                                                    0i32 as libc::c_ulong
                                                } else {
                                                    (*zds).outBuffSize.wrapping_sub((*zds).outStart)
                                                }, ip as *const libc::c_void,
                                                neededInSize);
                    if 0 != ERR_isError(decodedSize) { return decodedSize }
                    ip = ip.offset(neededInSize as isize);
                    if !(0 == decodedSize && 0 == isSkipFrame) {
                        /* this was just a header */
                        (*zds).outEnd =
                            (*zds).outStart.wrapping_add(decodedSize);
                        (*zds).streamStage = zdss_flush
                    }
                    current_block_131 = 1745632252074978848;
                } else if ip == iend {
                    someMoreWork = 0i32 as U32;
                    /* no more input */
                    current_block_131 = 1745632252074978848;
                } else {
                    (*zds).streamStage = zdss_load;
                    /* fall-through */
                    current_block_131 = 13740693533991687037;
                }
            }
            _ => { }
        }
        match current_block_131 {
            13740693533991687037 => {
                let neededInSize_0: size_t =
                    ZSTD_nextSrcSizeToDecompress(zds);
                let toLoad_0: size_t =
                    neededInSize_0.wrapping_sub((*zds).inPos);
                let isSkipFrame_0: libc::c_int = ZSTD_isSkipFrame(zds);
                let mut loadedSize: size_t = 0;
                if 0 != isSkipFrame_0 {
                    loadedSize =
                        if toLoad_0 <
                               iend.wrapping_offset_from(ip) as libc::c_long
                                   as size_t {
                            toLoad_0
                        } else {
                            iend.wrapping_offset_from(ip) as libc::c_long as
                                size_t
                        }
                } else {
                    if toLoad_0 > (*zds).inBuffSize.wrapping_sub((*zds).inPos)
                       {
                        return -(ZSTD_error_corruption_detected as
                                     libc::c_int) as size_t
                    }
                    loadedSize =
                        ZSTD_limitCopy((*zds).inBuff.offset((*zds).inPos as
                                                                isize) as
                                           *mut libc::c_void, toLoad_0,
                                       ip as *const libc::c_void,
                                       iend.wrapping_offset_from(ip) as
                                           libc::c_long as size_t)
                }
                ip = ip.offset(loadedSize as isize);
                (*zds).inPos =
                    ((*zds).inPos as libc::c_ulong).wrapping_add(loadedSize)
                        as size_t as size_t;
                if loadedSize < toLoad_0 {
                    someMoreWork = 0i32 as U32;
                    /* not enough input, wait for more */
                    current_block_131 = 1745632252074978848;
                } else {
                    /* decode loaded input */
                    let decodedSize_0: size_t =
                        ZSTD_decompressContinue(zds,
                                                (*zds).outBuff.offset((*zds).outStart
                                                                          as
                                                                          isize)
                                                    as *mut libc::c_void,
                                                (*zds).outBuffSize.wrapping_sub((*zds).outStart),
                                                (*zds).inBuff as
                                                    *const libc::c_void,
                                                neededInSize_0);
                    if 0 != ERR_isError(decodedSize_0) {
                        return decodedSize_0
                    }
                    (*zds).inPos = 0i32 as size_t;
                    if 0 == decodedSize_0 && 0 == isSkipFrame_0 {
                        (*zds).streamStage = zdss_read;
                        /* this was just a header */
                        current_block_131 = 1745632252074978848;
                    } else {
                        (*zds).outEnd =
                            (*zds).outStart.wrapping_add(decodedSize_0);
                        (*zds).streamStage = zdss_flush;
                        /* fall-through */
                        current_block_131 = 850175865728824115;
                    }
                }
            }
            _ => { }
        }
        match current_block_131 {
            850175865728824115 => {
                let toFlushSize: size_t =
                    (*zds).outEnd.wrapping_sub((*zds).outStart);
                let flushedSize: size_t =
                    ZSTD_limitCopy(op as *mut libc::c_void,
                                   oend.wrapping_offset_from(op) as
                                       libc::c_long as size_t,
                                   (*zds).outBuff.offset((*zds).outStart as
                                                             isize) as
                                       *const libc::c_void, toFlushSize);
                op = op.offset(flushedSize as isize);
                (*zds).outStart =
                    ((*zds).outStart as
                         libc::c_ulong).wrapping_add(flushedSize) as size_t as
                        size_t;
                if flushedSize == toFlushSize {
                    /* flush completed */
                    (*zds).streamStage = zdss_read;
                    if ((*zds).outBuffSize as libc::c_ulonglong) <
                           (*zds).fParams.frameContentSize &&
                           (*zds).outStart.wrapping_add((*zds).fParams.blockSizeMax
                                                            as libc::c_ulong)
                               > (*zds).outBuffSize {
                        (*zds).outEnd = 0i32 as size_t;
                        (*zds).outStart = (*zds).outEnd
                    }
                } else { someMoreWork = 0i32 as U32 }
            }
            _ => { }
        }
    }
    (*input).pos =
        ip.wrapping_offset_from((*input).src as *const libc::c_char) as
            libc::c_long as size_t;
    (*output).pos =
        op.wrapping_offset_from((*output).dst as *mut libc::c_char) as
            libc::c_long as size_t;
    if ip == istart && op == ostart {
        (*zds).noForwardProgress += 1;
        if (*zds).noForwardProgress >= 16i32 {
            if op == oend {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
            }
            if ip == iend {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
            }
        }
    } else { (*zds).noForwardProgress = 0i32 }
    let mut nextSrcSizeHint: size_t = ZSTD_nextSrcSizeToDecompress(zds);
    if 0 == nextSrcSizeHint {
        if (*zds).outEnd == (*zds).outStart {
            if 0 != (*zds).hostageByte {
                if (*input).pos >= (*input).size {
                    (*zds).streamStage = zdss_read;
                    return 1i32 as size_t
                }
                (*input).pos = (*input).pos.wrapping_add(1)
            }
            return 0i32 as size_t
        }
        if 0 == (*zds).hostageByte {
            (*input).pos = (*input).pos.wrapping_sub(1);
            (*zds).hostageByte = 1i32 as U32
        }
        return 1i32 as size_t
    }
    nextSrcSizeHint =
        (nextSrcSizeHint as
             libc::c_ulong).wrapping_add(ZSTD_blockHeaderSize.wrapping_mul((ZSTD_nextInputType(zds)
                                                                                as
                                                                                libc::c_uint
                                                                                ==
                                                                                ZSTDnit_block
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong))
            as size_t as size_t;
    nextSrcSizeHint =
        (nextSrcSizeHint as libc::c_ulong).wrapping_sub((*zds).inPos) as
            size_t as size_t;
    return nextSrcSizeHint;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_nextSrcSizeToDecompress(mut dctx:
                                                          *mut ZSTD_DCtx)
 -> size_t {
    return (*dctx).expected;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_nextInputType(mut dctx: *mut ZSTD_DCtx)
 -> ZSTD_nextInputType_e {
    match (*dctx).stage as libc::c_uint {
        2 => { return ZSTDnit_blockHeader }
        3 => { return ZSTDnit_block }
        4 => { return ZSTDnit_lastBlock }
        5 => { return ZSTDnit_checksum }
        6 | 7 => { return ZSTDnit_skippableFrame }
        0 | 1 | _ => { return ZSTDnit_frameHeader }
    };
}
/* *****   Decompression   ***** */
unsafe extern "C" fn ZSTD_limitCopy(mut dst: *mut libc::c_void,
                                    mut dstCapacity: size_t,
                                    mut src: *const libc::c_void,
                                    mut srcSize: size_t) -> size_t {
    let length: size_t =
        if dstCapacity < srcSize { dstCapacity } else { srcSize };
    memcpy(dst, src, length);
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressContinue(mut dctx: *mut ZSTD_DCtx,
                                                 mut dst: *mut libc::c_void,
                                                 mut dstCapacity: size_t,
                                                 mut src: *const libc::c_void,
                                                 mut srcSize: size_t)
 -> size_t {
    if srcSize != (*dctx).expected {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    if 0 != dstCapacity { ZSTD_checkContinuity(dctx, dst); }
    match (*dctx).stage as libc::c_uint {
        0 => {
            if (*dctx).format as libc::c_uint ==
                   ZSTD_f_zstd1 as libc::c_int as libc::c_uint {
                if MEM_readLE32(src) & 0xfffffff0u32 ==
                       0x184d2a50i32 as libc::c_uint {
                    memcpy((*dctx).headerBuffer.as_mut_ptr() as
                               *mut libc::c_void, src, srcSize);
                    (*dctx).expected =
                        (8i32 as libc::c_ulong).wrapping_sub(srcSize);
                    (*dctx).stage = ZSTDds_decodeSkippableHeader;
                    return 0i32 as size_t
                }
            }
            (*dctx).headerSize =
                ZSTD_frameHeaderSize_internal(src, srcSize, (*dctx).format);
            if 0 != ERR_isError((*dctx).headerSize) {
                return (*dctx).headerSize
            }
            memcpy((*dctx).headerBuffer.as_mut_ptr() as *mut libc::c_void,
                   src, srcSize);
            (*dctx).expected = (*dctx).headerSize.wrapping_sub(srcSize);
            (*dctx).stage = ZSTDds_decodeFrameHeader;
            return 0i32 as size_t
        }
        1 => {
            memcpy((*dctx).headerBuffer.as_mut_ptr().offset((*dctx).headerSize.wrapping_sub(srcSize)
                                                                as isize) as
                       *mut libc::c_void, src, srcSize);
            let errcod: size_t =
                ZSTD_decodeFrameHeader(dctx,
                                       (*dctx).headerBuffer.as_mut_ptr() as
                                           *const libc::c_void,
                                       (*dctx).headerSize);
            if 0 != ERR_isError(errcod) { return errcod }
            (*dctx).expected = ZSTD_blockHeaderSize;
            (*dctx).stage = ZSTDds_decodeBlockHeader;
            return 0i32 as size_t
        }
        2 => {
            let mut bp: blockProperties_t =
                blockProperties_t{blockType: bt_raw,
                                  lastBlock: 0,
                                  origSize: 0,};
            let cBlockSize: size_t =
                ZSTD_getcBlockSize(src, ZSTD_blockHeaderSize, &mut bp);
            if 0 != ERR_isError(cBlockSize) { return cBlockSize }
            (*dctx).expected = cBlockSize;
            (*dctx).bType = bp.blockType;
            (*dctx).rleSize = bp.origSize as size_t;
            if 0 != cBlockSize {
                (*dctx).stage =
                    (if 0 != bp.lastBlock {
                         ZSTDds_decompressLastBlock as libc::c_int
                     } else { ZSTDds_decompressBlock as libc::c_int }) as
                        ZSTD_dStage;
                return 0i32 as size_t
            }
            if 0 != bp.lastBlock {
                if 0 != (*dctx).fParams.checksumFlag {
                    (*dctx).expected = 4i32 as size_t;
                    (*dctx).stage = ZSTDds_checkChecksum
                } else {
                    (*dctx).expected = 0i32 as size_t;
                    (*dctx).stage = ZSTDds_getFrameHeaderSize
                }
            } else {
                (*dctx).expected = ZSTD_blockHeaderSize;
                (*dctx).stage = ZSTDds_decodeBlockHeader
            }
            return 0i32 as size_t
        }
        4 | 3 => {
            let mut rSize: size_t = 0;
            match (*dctx).bType as libc::c_uint {
                2 => {
                    rSize =
                        ZSTD_decompressBlock_internal(dctx, dst, dstCapacity,
                                                      src, srcSize, 1i32)
                }
                0 => {
                    rSize = ZSTD_copyRawBlock(dst, dstCapacity, src, srcSize)
                }
                1 => {
                    rSize =
                        ZSTD_setRleBlock(dst, dstCapacity,
                                         *(src as *const BYTE),
                                         (*dctx).rleSize)
                }
                3 | _ => {
                    return -(ZSTD_error_corruption_detected as libc::c_int) as
                               size_t
                }
            }
            if 0 != ERR_isError(rSize) { return rSize }
            (*dctx).decodedSize =
                ((*dctx).decodedSize as libc::c_ulong).wrapping_add(rSize) as
                    U64 as U64;
            if 0 != (*dctx).fParams.checksumFlag {
                ZSTD_XXH64_update(&mut (*dctx).xxhState, dst, rSize);
            }
            if (*dctx).stage as libc::c_uint ==
                   ZSTDds_decompressLastBlock as libc::c_int as libc::c_uint {
                if (*dctx).fParams.frameContentSize !=
                       0u64.wrapping_sub(1i32 as libc::c_ulonglong) {
                    if (*dctx).decodedSize as libc::c_ulonglong !=
                           (*dctx).fParams.frameContentSize {
                        return -(ZSTD_error_corruption_detected as
                                     libc::c_int) as size_t
                    }
                }
                if 0 != (*dctx).fParams.checksumFlag {
                    (*dctx).expected = 4i32 as size_t;
                    (*dctx).stage = ZSTDds_checkChecksum
                } else {
                    (*dctx).expected = 0i32 as size_t;
                    (*dctx).stage = ZSTDds_getFrameHeaderSize
                }
            } else {
                (*dctx).stage = ZSTDds_decodeBlockHeader;
                (*dctx).expected = ZSTD_blockHeaderSize;
                (*dctx).previousDstEnd =
                    (dst as *mut libc::c_char).offset(rSize as isize) as
                        *const libc::c_void
            }
            return rSize
        }
        5 => {
            let h32: U32 = ZSTD_XXH64_digest(&mut (*dctx).xxhState) as U32;
            let check32: U32 = MEM_readLE32(src);
            if check32 != h32 {
                return -(ZSTD_error_checksum_wrong as libc::c_int) as size_t
            }
            (*dctx).expected = 0i32 as size_t;
            (*dctx).stage = ZSTDds_getFrameHeaderSize;
            return 0i32 as size_t
        }
        6 => {
            memcpy((*dctx).headerBuffer.as_mut_ptr().offset((8i32 as
                                                                 libc::c_ulong).wrapping_sub(srcSize)
                                                                as isize) as
                       *mut libc::c_void, src, srcSize);
            (*dctx).expected =
                MEM_readLE32((*dctx).headerBuffer.as_mut_ptr().offset(4isize)
                                 as *const libc::c_void) as size_t;
            (*dctx).stage = ZSTDds_skipFrame;
            return 0i32 as size_t
        }
        7 => {
            (*dctx).expected = 0i32 as size_t;
            (*dctx).stage = ZSTDds_getFrameHeaderSize;
            return 0i32 as size_t
        }
        _ => { return -(ZSTD_error_GENERIC as libc::c_int) as size_t }
    };
}
unsafe extern "C" fn ZSTD_isSkipFrame(mut dctx: *mut ZSTD_DCtx)
 -> libc::c_int {
    return ((*dctx).stage as libc::c_uint ==
                ZSTDds_skipFrame as libc::c_int as libc::c_uint) as
               libc::c_int;
}
/* *< when frame content size is not known, pass in frameContentSize == ZSTD_CONTENTSIZE_UNKNOWN */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decodingBufferSize_min(mut windowSize:
                                                         libc::c_ulonglong,
                                                     mut frameContentSize:
                                                         libc::c_ulonglong)
 -> size_t {
    let blockSize: size_t =
        (if windowSize < (1i32 << 17i32) as libc::c_ulonglong {
             windowSize
         } else { (1i32 << 17i32) as libc::c_ulonglong }) as size_t;
    let neededRBSize: libc::c_ulonglong =
        windowSize.wrapping_add(blockSize as
                                    libc::c_ulonglong).wrapping_add((8i32 *
                                                                         2i32)
                                                                        as
                                                                        libc::c_ulonglong);
    let neededSize: libc::c_ulonglong =
        if frameContentSize < neededRBSize {
            frameContentSize
        } else { neededRBSize };
    let minRBSize: size_t = neededSize as size_t;
    if minRBSize as libc::c_ulonglong != neededSize {
        return -(ZSTD_error_frameParameter_windowTooLarge as libc::c_int) as
                   size_t
    }
    return minRBSize;
}
/* ===   frame size   === */
/* ! ZSTD_findFrameCompressedSize() :
 * `src` should point to the start of a ZSTD frame or skippable frame.
 * `srcSize` must be >= first frame size
 * @return : the compressed size of the first frame starting at `src`,
 *           suitable to pass as `srcSize` to `ZSTD_decompress` or similar,
 *        or an error code if input is invalid */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_findFrameCompressedSize(mut src:
                                                          *const libc::c_void,
                                                      mut srcSize: size_t)
 -> size_t {
    if srcSize >= 8i32 as libc::c_ulong &&
           MEM_readLE32(src) & 0xfffffff0u32 == 0x184d2a50i32 as libc::c_uint
       {
        return readSkippableFrameSize(src, srcSize)
    } else {
        let mut ip: *const BYTE = src as *const BYTE;
        let ipstart: *const BYTE = ip;
        let mut remainingSize: size_t = srcSize;
        let mut zfh: ZSTD_frameHeader =
            ZSTD_frameHeader{frameContentSize: 0,
                             windowSize: 0,
                             blockSizeMax: 0,
                             frameType: ZSTD_frame,
                             headerSize: 0,
                             dictID: 0,
                             checksumFlag: 0,};
        let ret: size_t = ZSTD_getFrameHeader(&mut zfh, src, srcSize);
        if 0 != ERR_isError(ret) { return ret }
        if ret > 0i32 as libc::c_ulong {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
        }
        ip = ip.offset(zfh.headerSize as isize);
        remainingSize =
            (remainingSize as
                 libc::c_ulong).wrapping_sub(zfh.headerSize as libc::c_ulong)
                as size_t as size_t;
        loop  {
            let mut blockProperties: blockProperties_t =
                blockProperties_t{blockType: bt_raw,
                                  lastBlock: 0,
                                  origSize: 0,};
            let cBlockSize: size_t =
                ZSTD_getcBlockSize(ip as *const libc::c_void, remainingSize,
                                   &mut blockProperties);
            if 0 != ERR_isError(cBlockSize) { return cBlockSize }
            if ZSTD_blockHeaderSize.wrapping_add(cBlockSize) > remainingSize {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
            }
            ip =
                ip.offset(ZSTD_blockHeaderSize.wrapping_add(cBlockSize) as
                              isize);
            remainingSize =
                (remainingSize as
                     libc::c_ulong).wrapping_sub(ZSTD_blockHeaderSize.wrapping_add(cBlockSize))
                    as size_t as size_t;
            if 0 != blockProperties.lastBlock { break ; }
        }
        if 0 != zfh.checksumFlag {
            if remainingSize < 4i32 as libc::c_ulong {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
            }
            ip = ip.offset(4isize)
        }
        return ip.wrapping_offset_from(ipstart) as libc::c_long as size_t
    };
}
/* *< re-use decompression parameters from previous init; saves dictionary loading */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_resetDStream(mut dctx: *mut ZSTD_DStream)
 -> size_t {
    (*dctx).streamStage = zdss_loadHeader;
    (*dctx).outEnd = 0i32 as size_t;
    (*dctx).outStart = (*dctx).outEnd;
    (*dctx).inPos = (*dctx).outStart;
    (*dctx).lhSize = (*dctx).inPos;
    (*dctx).legacyVersion = 0i32 as U32;
    (*dctx).hostageByte = 0i32 as U32;
    return 5i32 as size_t;
}
/* !< recommended size for input buffer */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DStreamInSize() -> size_t {
    return ((1i32 << 17i32) as
                libc::c_ulong).wrapping_add(ZSTD_blockHeaderSize);
}
/* !< recommended size for output buffer. Guarantee to successfully flush at least one complete block in all circumstances. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DStreamOutSize() -> size_t {
    return (1i32 << 17i32) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_DCtx(mut dctx: *const ZSTD_DCtx)
 -> size_t {
    if dctx.is_null() { return 0i32 as size_t }
    return (::std::mem::size_of::<ZSTD_DCtx>() as
                libc::c_ulong).wrapping_add(ZSTD_sizeof_DDict((*dctx).ddictLocal)).wrapping_add((*dctx).inBuffSize).wrapping_add((*dctx).outBuffSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_DStream(mut dctx: *const ZSTD_DStream)
 -> size_t {
    return ZSTD_sizeof_DCtx(dctx);
}
/* ! ZSTD_dParam_getBounds() :
 *  All parameters must belong to an interval with lower and upper bounds,
 *  otherwise they will either trigger an error or be automatically clamped.
 * @return : a structure, ZSTD_bounds, which contains
 *         - an error status field, which must be tested using ZSTD_isError()
 *         - both lower and upper bounds, inclusive
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_dParam_getBounds(mut dParam: ZSTD_dParameter)
 -> ZSTD_bounds {
    let mut bounds: ZSTD_bounds =
        ZSTD_bounds{error: 0i32 as size_t,
                    lowerBound: 0i32,
                    upperBound: 0i32,};
    match dParam as libc::c_uint {
        100 => {
            bounds.lowerBound = 10i32;
            bounds.upperBound =
                if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                       4i32 as libc::c_ulong {
                    30i32
                } else { 31i32 };
            return bounds
        }
        1000 => {
            bounds.lowerBound = ZSTD_f_zstd1 as libc::c_int;
            bounds.upperBound = ZSTD_f_zstd1_magicless as libc::c_int;
            return bounds
        }
        _ => { }
    }
    bounds.error =
        -(ZSTD_error_parameter_unsupported as libc::c_int) as size_t;
    return bounds;
}
/* ! ZSTD_DCtx_setParameter() :
 *  Set one compression parameter, selected by enum ZSTD_dParameter.
 *  All parameters have valid bounds. Bounds can be queried using ZSTD_dParam_getBounds().
 *  Providing a value beyond bound will either clamp it, or trigger an error (depending on parameter).
 *  Setting a parameter is only possible during frame initialization (before starting decompression).
 * @return : 0, or an error code (which can be tested using ZSTD_isError()).
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_setParameter(mut dctx: *mut ZSTD_DCtx,
                                                mut dParam: ZSTD_dParameter,
                                                mut value: libc::c_int)
 -> size_t {
    if (*dctx).streamStage as libc::c_uint !=
           zdss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    }
    match dParam as libc::c_uint {
        100 => {
            if 0 == ZSTD_dParam_withinBounds(ZSTD_d_windowLogMax, value) {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as
                           size_t
            }
            (*dctx).maxWindowSize = (1i32 as size_t) << value;
            return 0i32 as size_t
        }
        1000 => {
            if 0 == ZSTD_dParam_withinBounds(ZSTD_d_experimentalParam1, value)
               {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as
                           size_t
            }
            (*dctx).format = value as ZSTD_format_e;
            return 0i32 as size_t
        }
        _ => { }
    }
    return -(ZSTD_error_parameter_unsupported as libc::c_int) as size_t;
}
/* ZSTD_dParam_withinBounds:
 * @return 1 if value is within dParam bounds,
 * 0 otherwise */
unsafe extern "C" fn ZSTD_dParam_withinBounds(mut dParam: ZSTD_dParameter,
                                              mut value: libc::c_int)
 -> libc::c_int {
    let bounds: ZSTD_bounds = ZSTD_dParam_getBounds(dParam);
    if 0 != ERR_isError(bounds.error) { return 0i32 }
    if value < bounds.lowerBound { return 0i32 }
    if value > bounds.upperBound { return 0i32 }
    return 1i32;
}
/* ! ZSTD_DCtx_refDDict() :
 *  Reference a prepared dictionary, to be used to decompress next frames.
 *  The dictionary remains active for decompression of future frames using same DCtx.
 * @result : 0, or an error code (which can be tested with ZSTD_isError()).
 *  Note 1 : Currently, only one dictionary can be managed.
 *           Referencing a new dictionary effectively "discards" any previous one.
 *  Special: referencing a NULL DDict means "return to no-dictionary mode".
 *  Note 2 : DDict is just referenced, its lifetime must outlive its usage from DCtx.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_refDDict(mut dctx: *mut ZSTD_DCtx,
                                            mut ddict: *const ZSTD_DDict)
 -> size_t {
    if (*dctx).streamStage as libc::c_uint !=
           zdss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    }
    (*dctx).ddict = ddict;
    return 0i32 as size_t;
}
/* ! ZSTD_DCtx_refPrefix() :
 *  Reference a prefix (single-usage dictionary) to decompress next frame.
 *  This is the reverse operation of ZSTD_CCtx_refPrefix(),
 *  and must use the same prefix as the one used during compression.
 *  Prefix is **only used once**. Reference is discarded at end of frame.
 *  End of frame is reached when ZSTD_decompressStream() returns 0.
 * @result : 0, or an error code (which can be tested with ZSTD_isError()).
 *  Note 1 : Adding any prefix (including NULL) invalidates any previously set prefix or dictionary
 *  Note 2 : Prefix buffer is referenced. It **must** outlive decompression.
 *           Prefix buffer must remain unmodified up to the end of frame,
 *           reached when ZSTD_decompressStream() returns 0.
 *  Note 3 : By default, the prefix is treated as raw content (ZSTD_dm_rawContent).
 *           Use ZSTD_CCtx_refPrefix_advanced() to alter dictMode (Experimental section)
 *  Note 4 : Referencing a raw content prefix has almost no cpu nor memory cost.
 *           A full dictionary is more costly, as it requires building tables.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_refPrefix(mut dctx: *mut ZSTD_DCtx,
                                             mut prefix: *const libc::c_void,
                                             mut prefixSize: size_t)
 -> size_t {
    return ZSTD_DCtx_refPrefix_advanced(dctx, prefix, prefixSize,
                                        ZSTD_dct_rawContent);
}
/* ! ZSTD_DCtx_refPrefix_advanced() :
 *  Same as ZSTD_DCtx_refPrefix(), but gives finer control over
 *  how to interpret prefix content (automatic ? force raw mode (default) ? full mode only ?) */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_refPrefix_advanced(mut dctx:
                                                          *mut ZSTD_DCtx,
                                                      mut prefix:
                                                          *const libc::c_void,
                                                      mut prefixSize: size_t,
                                                      mut dictContentType:
                                                          ZSTD_dictContentType_e)
 -> size_t {
    return ZSTD_DCtx_loadDictionary_advanced(dctx, prefix, prefixSize,
                                             ZSTD_dlm_byRef, dictContentType);
}
/* ! ZSTD_DCtx_reset() :
 *  Return a DCtx to clean state.
 *  Session and parameters can be reset jointly or separately.
 *  Parameters can only be reset when no active frame is being decompressed.
 * @return : 0, or an error code, which can be tested with ZSTD_isError()
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_reset(mut dctx: *mut ZSTD_DCtx,
                                         mut reset: ZSTD_ResetDirective)
 -> size_t {
    if reset as libc::c_uint ==
           ZSTD_reset_session_only as libc::c_int as libc::c_uint ||
           reset as libc::c_uint ==
               ZSTD_reset_session_and_parameters as libc::c_int as
                   libc::c_uint {
        ZSTD_initDStream(dctx);
    }
    if reset as libc::c_uint ==
           ZSTD_reset_parameters as libc::c_int as libc::c_uint ||
           reset as libc::c_uint ==
               ZSTD_reset_session_and_parameters as libc::c_int as
                   libc::c_uint {
        if (*dctx).streamStage as libc::c_uint !=
               zdss_init as libc::c_int as libc::c_uint {
            return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
        }
        (*dctx).format = ZSTD_f_zstd1;
        (*dctx).maxWindowSize =
            ((1i32 as U32) << 27i32).wrapping_add(1i32 as libc::c_uint) as
                size_t
    }
    return 0i32 as size_t;
}
/* **************************************
*  Frame size functions
***************************************/
/* ! ZSTD_findDecompressedSize() :
 *  `src` should point the start of a series of ZSTD encoded and/or skippable frames
 *  `srcSize` must be the _exact_ size of this series
 *       (i.e. there should be a frame boundary exactly at `srcSize` bytes after `src`)
 *  @return : - decompressed size of all data in all successive frames
 *            - if the decompressed size cannot be determined: ZSTD_CONTENTSIZE_UNKNOWN
 *            - if an error occurred: ZSTD_CONTENTSIZE_ERROR
 *
 *   note 1 : decompressed size is an optional field, that may not be present, especially in streaming mode.
 *            When `return==ZSTD_CONTENTSIZE_UNKNOWN`, data to decompress could be any size.
 *            In which case, it's necessary to use streaming mode to decompress data.
 *   note 2 : decompressed size is always present when compression is done with ZSTD_compress()
 *   note 3 : decompressed size can be very large (64-bits value),
 *            potentially larger than what local system can handle as a single memory segment.
 *            In which case, it's necessary to use streaming mode to decompress data.
 *   note 4 : If source is untrusted, decompressed size could be wrong or intentionally modified.
 *            Always ensure result fits within application's authorized limits.
 *            Each application can set its own limits.
 *   note 5 : ZSTD_findDecompressedSize handles multiple frames, and so it must traverse the input to
 *            read each contained frame header.  This is fast as most of the data is skipped,
 *            however it does mean that all frame data must be present and valid. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_findDecompressedSize(mut src:
                                                       *const libc::c_void,
                                                   mut srcSize: size_t)
 -> libc::c_ulonglong {
    let mut totalDstSize: libc::c_ulonglong = 0i32 as libc::c_ulonglong;
    while srcSize >= 5i32 as libc::c_ulong {
        let magicNumber: U32 = MEM_readLE32(src);
        if magicNumber & 0xfffffff0u32 == 0x184d2a50i32 as libc::c_uint {
            let skippableSize: size_t = readSkippableFrameSize(src, srcSize);
            if 0 != ERR_isError(skippableSize) {
                return skippableSize as libc::c_ulonglong
            }
            if srcSize < skippableSize {
                return 0u64.wrapping_sub(2i32 as libc::c_ulonglong)
            }
            src =
                (src as *const BYTE).offset(skippableSize as isize) as
                    *const libc::c_void;
            srcSize =
                (srcSize as libc::c_ulong).wrapping_sub(skippableSize) as
                    size_t as size_t
        } else {
            let ret: libc::c_ulonglong =
                ZSTD_getFrameContentSize(src, srcSize);
            if ret >= 0u64.wrapping_sub(2i32 as libc::c_ulonglong) {
                return ret
            }
            if totalDstSize.wrapping_add(ret) < totalDstSize {
                return 0u64.wrapping_sub(2i32 as libc::c_ulonglong)
            }
            totalDstSize = totalDstSize.wrapping_add(ret);
            let frameSrcSize: size_t =
                ZSTD_findFrameCompressedSize(src, srcSize);
            if 0 != ERR_isError(frameSrcSize) {
                return 0u64.wrapping_sub(2i32 as libc::c_ulonglong)
            }
            src =
                (src as *const BYTE).offset(frameSrcSize as isize) as
                    *const libc::c_void;
            srcSize =
                (srcSize as libc::c_ulong).wrapping_sub(frameSrcSize) as
                    size_t as size_t
        }
    }
    if 0 != srcSize { return 0u64.wrapping_sub(2i32 as libc::c_ulonglong) }
    return totalDstSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateDCtxSize() -> size_t {
    return ::std::mem::size_of::<ZSTD_DCtx>() as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateDStreamSize(mut windowSize: size_t)
 -> size_t {
    let blockSize: size_t =
        if windowSize < (1i32 << 17i32) as libc::c_ulong {
            windowSize
        } else { (1i32 << 17i32) as libc::c_ulong };
    /* no block can be larger */
    let inBuffSize: size_t = blockSize;
    let outBuffSize: size_t =
        ZSTD_decodingBufferSize_min(windowSize as libc::c_ulonglong,
                                    0u64.wrapping_sub(1i32 as
                                                          libc::c_ulonglong));
    return ZSTD_estimateDCtxSize().wrapping_add(inBuffSize).wrapping_add(outBuffSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateDStreamSize_fromFrame(mut src:
                                                                *const libc::c_void,
                                                            mut srcSize:
                                                                size_t)
 -> size_t {
    /* note : should be user-selectable, but requires an additional parameter (or a dctx) */
    let windowSizeMax: U32 =
        1u32 <<
            if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                   4i32 as libc::c_ulong {
                30i32
            } else { 31i32 };
    let mut zfh: ZSTD_frameHeader =
        ZSTD_frameHeader{frameContentSize: 0,
                         windowSize: 0,
                         blockSizeMax: 0,
                         frameType: ZSTD_frame,
                         headerSize: 0,
                         dictID: 0,
                         checksumFlag: 0,};
    let err: size_t = ZSTD_getFrameHeader(&mut zfh, src, srcSize);
    if 0 != ERR_isError(err) { return err }
    if err > 0i32 as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    if zfh.windowSize > windowSizeMax as libc::c_ulonglong {
        return -(ZSTD_error_frameParameter_windowTooLarge as libc::c_int) as
                   size_t
    }
    return ZSTD_estimateDStreamSize(zfh.windowSize as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticDCtx(mut workspace: *mut libc::c_void,
                                             mut workspaceSize: size_t)
 -> *mut ZSTD_DCtx {
    let dctx: *mut ZSTD_DCtx = workspace as *mut ZSTD_DCtx;
    if 0 != workspace as size_t & 7i32 as libc::c_ulong {
        return 0 as *mut ZSTD_DCtx
    }
    if workspaceSize < ::std::mem::size_of::<ZSTD_DCtx>() as libc::c_ulong {
        return 0 as *mut ZSTD_DCtx
    }
    ZSTD_initDCtx_internal(dctx);
    (*dctx).staticSize = workspaceSize;
    (*dctx).inBuff = dctx.offset(1isize) as *mut libc::c_char;
    return dctx;
}
/* *< same as ZSTD_initStaticDCtx() */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticDStream(mut workspace:
                                                    *mut libc::c_void,
                                                mut workspaceSize: size_t)
 -> *mut ZSTD_DStream {
    return ZSTD_initStaticDCtx(workspace, workspaceSize);
}
/* **************************************
*  Advanced decompression functions
***************************************/
/* ! ZSTD_isFrame() :
 *  Tells if the content of `buffer` starts with a valid Frame Identifier.
 *  Note : Frame Identifier is 4 bytes. If `size < 4`, @return will always be 0.
 *  Note 2 : Legacy Frame Identifiers are considered valid only if Legacy Support is enabled.
 *  Note 3 : Skippable Frame Identifiers are considered valid. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_isFrame(mut buffer: *const libc::c_void,
                                      mut size: size_t) -> libc::c_uint {
    if size < 4i32 as libc::c_ulong { return 0i32 as libc::c_uint }
    let magic: U32 = MEM_readLE32(buffer);
    if magic == 0xfd2fb528u32 { return 1i32 as libc::c_uint }
    if magic & 0xfffffff0u32 == 0x184d2a50i32 as libc::c_uint {
        return 1i32 as libc::c_uint
    }
    return 0i32 as libc::c_uint;
}
/* ! ZSTD_getDictID_fromDict() :
 *  Provides the dictID stored within dictionary.
 *  if @return == 0, the dictionary is not conformant with Zstandard specification.
 *  It can still be loaded, but as a content-only dictionary. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDictID_fromDict(mut dict:
                                                     *const libc::c_void,
                                                 mut dictSize: size_t)
 -> libc::c_uint {
    if dictSize < 8i32 as libc::c_ulong { return 0i32 as libc::c_uint }
    if MEM_readLE32(dict) != 0xec30a437u32 { return 0i32 as libc::c_uint }
    return MEM_readLE32((dict as *const libc::c_char).offset(4isize) as
                            *const libc::c_void);
}
/* ! ZSTD_getDictID_fromFrame() :
 *  Provides the dictID required to decompressed the frame stored within `src`.
 *  If @return == 0, the dictID could not be decoded.
 *  This could for one of the following reasons :
 *  - The frame does not require a dictionary to be decoded (most common case).
 *  - The frame was built with dictID intentionally removed. Whatever dictionary is necessary is a hidden information.
 *    Note : this use case also happens when using a non-conformant dictionary.
 *  - `srcSize` is too small, and as a result, the frame header could not be decoded (only possible if `srcSize < ZSTD_FRAMEHEADERSIZE_MAX`).
 *  - This is not a Zstandard frame.
 *  When identifying the exact failure cause, it's possible to use ZSTD_getFrameHeader(), which will provide a more precise error code. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDictID_fromFrame(mut src:
                                                      *const libc::c_void,
                                                  mut srcSize: size_t)
 -> libc::c_uint {
    let mut zfp: ZSTD_frameHeader =
        ZSTD_frameHeader{frameContentSize: 0i32 as libc::c_ulonglong,
                         windowSize: 0i32 as libc::c_ulonglong,
                         blockSizeMax: 0i32 as libc::c_uint,
                         frameType: ZSTD_frame,
                         headerSize: 0i32 as libc::c_uint,
                         dictID: 0i32 as libc::c_uint,
                         checksumFlag: 0i32 as libc::c_uint,};
    let hError: size_t = ZSTD_getFrameHeader(&mut zfp, src, srcSize);
    if 0 != ERR_isError(hError) { return 0i32 as libc::c_uint }
    return zfp.dictID;
}
/* ! ZSTD_DCtx_loadDictionary_byReference() :
 *  Same as ZSTD_DCtx_loadDictionary(),
 *  but references `dict` content instead of copying it into `dctx`.
 *  This saves memory if `dict` remains around.,
 *  However, it's imperative that `dict` remains accessible (and unmodified) while being used, so it must outlive decompression. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_loadDictionary_byReference(mut dctx:
                                                                  *mut ZSTD_DCtx,
                                                              mut dict:
                                                                  *const libc::c_void,
                                                              mut dictSize:
                                                                  size_t)
 -> size_t {
    return ZSTD_DCtx_loadDictionary_advanced(dctx, dict, dictSize,
                                             ZSTD_dlm_byRef, ZSTD_dct_auto);
}
/* ! ZSTD_DCtx_setMaxWindowSize() :
 *  Refuses allocating internal buffers for frames requiring a window size larger than provided limit.
 *  This protects a decoder context from reserving too much memory for itself (potential attack scenario).
 *  This parameter is only useful in streaming mode, since no internal buffer is allocated in single-pass mode.
 *  By default, a decompression context accepts all window sizes <= (1 << ZSTD_WINDOWLOG_LIMIT_DEFAULT)
 * @return : 0, or an error code (which can be tested using ZSTD_isError()).
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_setMaxWindowSize(mut dctx: *mut ZSTD_DCtx,
                                                    mut maxWindowSize: size_t)
 -> size_t {
    let bounds: ZSTD_bounds = ZSTD_dParam_getBounds(ZSTD_d_windowLogMax);
    let min: size_t = (1i32 as size_t) << bounds.lowerBound;
    let max: size_t = (1i32 as size_t) << bounds.upperBound;
    if (*dctx).streamStage as libc::c_uint !=
           zdss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    }
    if maxWindowSize < min {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t
    }
    if maxWindowSize > max {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t
    }
    (*dctx).maxWindowSize = maxWindowSize;
    return 0i32 as size_t;
}
/* ZSTD_d_format
 * experimental parameter,
 * allowing selection between ZSTD_format_e input compression formats
 */
/* ! ZSTD_DCtx_setFormat() :
 *  Instruct the decoder context about what kind of data to decode next.
 *  This instruction is mandatory to decode data without a fully-formed header,
 *  such ZSTD_f_zstd1_magicless for example.
 * @return : 0, or an error code (which can be tested using ZSTD_isError()). */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_setFormat(mut dctx: *mut ZSTD_DCtx,
                                             mut format: ZSTD_format_e)
 -> size_t {
    return ZSTD_DCtx_setParameter(dctx, ZSTD_d_experimentalParam1,
                                  format as libc::c_int);
}
/* ! ZSTD_decompressStream_simpleArgs() :
 *  Same as ZSTD_decompressStream(),
 *  but using only integral types as arguments.
 *  This can be helpful for binders from dynamic languages
 *  which have troubles handling structures containing memory pointers.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressStream_simpleArgs(mut dctx:
                                                              *mut ZSTD_DCtx,
                                                          mut dst:
                                                              *mut libc::c_void,
                                                          mut dstCapacity:
                                                              size_t,
                                                          mut dstPos:
                                                              *mut size_t,
                                                          mut src:
                                                              *const libc::c_void,
                                                          mut srcSize: size_t,
                                                          mut srcPos:
                                                              *mut size_t)
 -> size_t {
    let mut output: ZSTD_outBuffer =
        ZSTD_outBuffer_s{dst: dst, size: dstCapacity, pos: *dstPos,};
    let mut input: ZSTD_inBuffer =
        ZSTD_inBuffer_s{src: src, size: srcSize, pos: *srcPos,};
    /* ZSTD_compress_generic() will check validity of dstPos and srcPos */
    let cErr: size_t = ZSTD_decompressStream(dctx, &mut output, &mut input);
    *dstPos = output.pos;
    *srcPos = input.pos;
    return cErr;
}
/* *< note : ddict is referenced, it must outlive decompression session */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initDStream_usingDDict(mut dctx:
                                                         *mut ZSTD_DStream,
                                                     mut ddict:
                                                         *const ZSTD_DDict)
 -> size_t {
    let initResult: size_t = ZSTD_initDStream(dctx);
    (*dctx).ddict = ddict;
    return initResult;
}
/* misc */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_copyDCtx(mut dstDCtx: *mut ZSTD_DCtx,
                                       mut srcDCtx: *const ZSTD_DCtx) {
    let toCopy: size_t =
        (&mut (*dstDCtx).inBuff as *mut *mut libc::c_char as
             *mut libc::c_char).wrapping_offset_from(dstDCtx as
                                                         *mut libc::c_char) as
            libc::c_long as size_t;
    memcpy(dstDCtx as *mut libc::c_void, srcDCtx as *const libc::c_void,
           toCopy);
}
/* *< insert uncompressed block into `dctx` history. Useful for multi-blocks decompression. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_insertBlock(mut dctx: *mut ZSTD_DCtx,
                                          mut blockStart: *const libc::c_void,
                                          mut blockSize: size_t) -> size_t {
    ZSTD_checkContinuity(dctx, blockStart);
    (*dctx).previousDstEnd =
        (blockStart as *const libc::c_char).offset(blockSize as isize) as
            *const libc::c_void;
    return blockSize;
}