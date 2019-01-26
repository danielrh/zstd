#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(libc)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* custom memory allocation functions */
    #[no_mangle]
    fn ZSTD_malloc(size: size_t, customMem: ZSTD_customMem)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ZSTD_free(ptr: *mut libc::c_void, customMem: ZSTD_customMem);
    /* typedef'd to ZSTD_DCtx within "zstd.h" */
    /*-*******************************************************
 *  Shared internal functions
 *********************************************************/
    /* ! ZSTD_loadDEntropy() :
 *  dict : must point at beginning of a valid zstd dictionary.
 * @return : size of entropy tables read */
    #[no_mangle]
    fn ZSTD_loadDEntropy(entropy: *mut ZSTD_entropyDTables_t,
                         dict: *const libc::c_void, dictSize: size_t)
     -> size_t;
    /* ! ZSTD_getDictID_fromDict() :
 *  Provides the dictID stored within dictionary.
 *  if @return == 0, the dictionary is not conformant with Zstandard specification.
 *  It can still be loaded, but as a content-only dictionary. */
    #[no_mangle]
    fn ZSTD_getDictID_fromDict(dict: *const libc::c_void, dictSize: size_t)
     -> libc::c_uint;
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
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct unalign32 {
    pub v: U32,
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
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* zstd_ddict.c :
 * concentrates all logic that needs to know the internals of ZSTD_DDict object */
/*-*******************************************************
*  Dependencies
*********************************************************/
/* memcpy, memmove, memset */
/* low level memory routines */
/*-*******************************************************
*  Types
*********************************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_DDict_s {
    pub dictBuffer: *mut libc::c_void,
    pub dictContent: *const libc::c_void,
    pub dictSize: size_t,
    pub entropy: ZSTD_entropyDTables_t,
    pub dictID: U32,
    pub entropyPresent: U32,
    pub cMem: ZSTD_customMem,
}
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
/*= Decompression context
 *  When decompressing many times,
 *  it is recommended to allocate a context only once,
 *  and re-use it for each successive compression operation.
 *  This will make workload friendlier for system's memory.
 *  Use one context per thread for parallel execution. */
pub type ZSTD_DCtx = ZSTD_DCtx_s;
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
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    /* don't use static : performance detrimental  */
    let one: unnamed = unnamed{u: 1i32 as U32,};
    return one.c[0usize] as libc::c_uint;
}
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> U32 {
    return (*(ptr as *const unalign32)).v;
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
/*-****************************************
*  Error codes handling
******************************************/
/* reported already defined on VS 2015 (Rich Geldreich) */
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as
               libc::c_int as libc::c_uint;
}
/* ! ZSTD_createDDict() :
 *  Create a digested dictionary, ready to start decompression operation without startup delay.
 *  dictBuffer can be released after DDict creation, as its content is copied inside DDict. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDDict(mut dict: *const libc::c_void,
                                          mut dictSize: size_t)
 -> *mut ZSTD_DDict {
    let allocator: ZSTD_customMem =
        ZSTD_customMem{customAlloc: None,
                       customFree: None,
                       opaque: 0 as *mut libc::c_void,};
    return ZSTD_createDDict_advanced(dict, dictSize, ZSTD_dlm_byCopy,
                                     ZSTD_dct_auto, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDDict_advanced(mut dict:
                                                       *const libc::c_void,
                                                   mut dictSize: size_t,
                                                   mut dictLoadMethod:
                                                       ZSTD_dictLoadMethod_e,
                                                   mut dictContentType:
                                                       ZSTD_dictContentType_e,
                                                   mut customMem:
                                                       ZSTD_customMem)
 -> *mut ZSTD_DDict {
    if 0 !=
           customMem.customAlloc.is_none() as libc::c_int ^
               customMem.customFree.is_none() as libc::c_int {
        return 0 as *mut ZSTD_DDict
    }
    let ddict: *mut ZSTD_DDict =
        ZSTD_malloc(::std::mem::size_of::<ZSTD_DDict>() as libc::c_ulong,
                    customMem) as *mut ZSTD_DDict;
    if ddict.is_null() { return 0 as *mut ZSTD_DDict }
    (*ddict).cMem = customMem;
    let initResult: size_t =
        ZSTD_initDDict_internal(ddict, dict, dictSize, dictLoadMethod,
                                dictContentType);
    if 0 != ERR_isError(initResult) {
        ZSTD_freeDDict(ddict);
        return 0 as *mut ZSTD_DDict
    }
    return ddict;
}
/* ! ZSTD_freeDDict() :
 *  Function frees memory allocated with ZSTD_createDDict() */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeDDict(mut ddict: *mut ZSTD_DDict)
 -> size_t {
    if ddict.is_null() { return 0i32 as size_t }
    let cMem: ZSTD_customMem = (*ddict).cMem;
    ZSTD_free((*ddict).dictBuffer, cMem);
    ZSTD_free(ddict as *mut libc::c_void, cMem);
    return 0i32 as size_t;
}
unsafe extern "C" fn ZSTD_initDDict_internal(mut ddict: *mut ZSTD_DDict,
                                             mut dict: *const libc::c_void,
                                             mut dictSize: size_t,
                                             mut dictLoadMethod:
                                                 ZSTD_dictLoadMethod_e,
                                             mut dictContentType:
                                                 ZSTD_dictContentType_e)
 -> size_t {
    if dictLoadMethod as libc::c_uint ==
           ZSTD_dlm_byRef as libc::c_int as libc::c_uint || dict.is_null() ||
           0 == dictSize {
        (*ddict).dictBuffer = 0 as *mut libc::c_void;
        (*ddict).dictContent = dict;
        if dict.is_null() { dictSize = 0i32 as size_t }
    } else {
        let internalBuffer: *mut libc::c_void =
            ZSTD_malloc(dictSize, (*ddict).cMem);
        (*ddict).dictBuffer = internalBuffer;
        (*ddict).dictContent = internalBuffer;
        if internalBuffer.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        }
        memcpy(internalBuffer, dict, dictSize);
    }
    (*ddict).dictSize = dictSize;
    (*ddict).entropy.hufTable[0usize] = (12i32 * 0x1000001i32) as HUF_DTable;
    let errcod: size_t = ZSTD_loadEntropy_intoDDict(ddict, dictContentType);
    if 0 != ERR_isError(errcod) { return errcod }
    return 0i32 as size_t;
}
unsafe extern "C" fn ZSTD_loadEntropy_intoDDict(mut ddict: *mut ZSTD_DDict,
                                                mut dictContentType:
                                                    ZSTD_dictContentType_e)
 -> size_t {
    (*ddict).dictID = 0i32 as U32;
    (*ddict).entropyPresent = 0i32 as U32;
    if dictContentType as libc::c_uint ==
           ZSTD_dct_rawContent as libc::c_int as libc::c_uint {
        return 0i32 as size_t
    }
    if (*ddict).dictSize < 8i32 as libc::c_ulong {
        if dictContentType as libc::c_uint ==
               ZSTD_dct_fullDict as libc::c_int as libc::c_uint {
            return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
        }
        return 0i32 as size_t
    }
    let magic: U32 = MEM_readLE32((*ddict).dictContent);
    if magic != 0xec30a437u32 {
        if dictContentType as libc::c_uint ==
               ZSTD_dct_fullDict as libc::c_int as libc::c_uint {
            return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
        }
        return 0i32 as size_t
    }
    (*ddict).dictID =
        MEM_readLE32(((*ddict).dictContent as
                          *const libc::c_char).offset(4isize) as
                         *const libc::c_void);
    let errcod: size_t =
        ZSTD_loadDEntropy(&mut (*ddict).entropy, (*ddict).dictContent,
                          (*ddict).dictSize);
    if 0 != ERR_isError(errcod) {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    (*ddict).entropyPresent = 1i32 as U32;
    return 0i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_DDict(mut ddict: *const ZSTD_DDict)
 -> size_t {
    if ddict.is_null() { return 0i32 as size_t }
    return (::std::mem::size_of::<ZSTD_DDict>() as
                libc::c_ulong).wrapping_add(if !(*ddict).dictBuffer.is_null()
                                               {
                                                (*ddict).dictSize
                                            } else { 0i32 as libc::c_ulong });
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateDDictSize(mut dictSize: size_t,
                                                mut dictLoadMethod:
                                                    ZSTD_dictLoadMethod_e)
 -> size_t {
    return (::std::mem::size_of::<ZSTD_DDict>() as
                libc::c_ulong).wrapping_add(if dictLoadMethod as libc::c_uint
                                                   ==
                                                   ZSTD_dlm_byRef as
                                                       libc::c_int as
                                                       libc::c_uint {
                                                0i32 as libc::c_ulong
                                            } else { dictSize });
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticDDict(mut sBuffer: *mut libc::c_void,
                                              mut sBufferSize: size_t,
                                              mut dict: *const libc::c_void,
                                              mut dictSize: size_t,
                                              mut dictLoadMethod:
                                                  ZSTD_dictLoadMethod_e,
                                              mut dictContentType:
                                                  ZSTD_dictContentType_e)
 -> *const ZSTD_DDict {
    let neededSpace: size_t =
        (::std::mem::size_of::<ZSTD_DDict>() as
             libc::c_ulong).wrapping_add(if dictLoadMethod as libc::c_uint ==
                                                ZSTD_dlm_byRef as libc::c_int
                                                    as libc::c_uint {
                                             0i32 as libc::c_ulong
                                         } else { dictSize });
    let ddict: *mut ZSTD_DDict = sBuffer as *mut ZSTD_DDict;
    if 0 != sBuffer as size_t & 7i32 as libc::c_ulong {
        return 0 as *const ZSTD_DDict
    }
    if sBufferSize < neededSpace { return 0 as *const ZSTD_DDict }
    if dictLoadMethod as libc::c_uint ==
           ZSTD_dlm_byCopy as libc::c_int as libc::c_uint {
        memcpy(ddict.offset(1isize) as *mut libc::c_void, dict, dictSize);
        dict = ddict.offset(1isize) as *const libc::c_void
    }
    if 0 !=
           ERR_isError(ZSTD_initDDict_internal(ddict, dict, dictSize,
                                               ZSTD_dlm_byRef,
                                               dictContentType)) {
        return 0 as *const ZSTD_DDict
    }
    return ddict;
}
/* ! ZSTD_createDDict_byReference() :
 *  Create a digested dictionary, ready to start decompression operation without startup delay.
 *  Dictionary content is referenced, and therefore stays in dictBuffer.
 *  It is important that dictBuffer outlives DDict,
 *  it must remain read accessible throughout the lifetime of DDict */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDDict_byReference(mut dictBuffer:
                                                          *const libc::c_void,
                                                      mut dictSize: size_t)
 -> *mut ZSTD_DDict {
    let allocator: ZSTD_customMem =
        ZSTD_customMem{customAlloc: None,
                       customFree: None,
                       opaque: 0 as *mut libc::c_void,};
    return ZSTD_createDDict_advanced(dictBuffer, dictSize, ZSTD_dlm_byRef,
                                     ZSTD_dct_auto, allocator);
}
/* ! ZSTD_getDictID_fromDDict() :
 *  Provides the dictID of the dictionary loaded into `ddict`.
 *  If @return == 0, the dictionary is not conformant to Zstandard specification, or empty.
 *  Non-conformant dictionaries can still be loaded, but as content-only dictionaries. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDictID_fromDDict(mut ddict:
                                                      *const ZSTD_DDict)
 -> libc::c_uint {
    if ddict.is_null() { return 0i32 as libc::c_uint }
    return ZSTD_getDictID_fromDict((*ddict).dictContent, (*ddict).dictSize);
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
pub unsafe extern "C" fn ZSTD_DDict_dictContent(mut ddict: *const ZSTD_DDict)
 -> *const libc::c_void {
    return (*ddict).dictContent;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DDict_dictSize(mut ddict: *const ZSTD_DDict)
 -> size_t {
    return (*ddict).dictSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_copyDDictParameters(mut dctx: *mut ZSTD_DCtx,
                                                  mut ddict:
                                                      *const ZSTD_DDict) {
    (*dctx).dictID = (*ddict).dictID;
    (*dctx).prefixStart = (*ddict).dictContent;
    (*dctx).virtualStart = (*ddict).dictContent;
    (*dctx).dictEnd =
        ((*ddict).dictContent as
             *const BYTE).offset((*ddict).dictSize as isize) as
            *const libc::c_void;
    (*dctx).previousDstEnd = (*dctx).dictEnd;
    if 0 != (*ddict).entropyPresent {
        (*dctx).litEntropy = 1i32 as U32;
        (*dctx).fseEntropy = 1i32 as U32;
        (*dctx).LLTptr = (*ddict).entropy.LLTable.as_ptr();
        (*dctx).MLTptr = (*ddict).entropy.MLTable.as_ptr();
        (*dctx).OFTptr = (*ddict).entropy.OFTable.as_ptr();
        (*dctx).HUFptr = (*ddict).entropy.hufTable.as_ptr();
        (*dctx).entropy.rep[0usize] = (*ddict).entropy.rep[0usize];
        (*dctx).entropy.rep[1usize] = (*ddict).entropy.rep[1usize];
        (*dctx).entropy.rep[2usize] = (*ddict).entropy.rep[2usize]
    } else {
        (*dctx).litEntropy = 0i32 as U32;
        (*dctx).fseEntropy = 0i32 as U32
    };
}