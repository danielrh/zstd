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
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
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
    /* BMI2 variants.
 * If the CPU has BMI2 support, pass bmi2=1, otherwise pass bmi2=0.
 */
    #[no_mangle]
    fn HUF_decompress1X_usingDTable_bmi2(dst: *mut libc::c_void,
                                         maxDstSize: size_t,
                                         cSrc: *const libc::c_void,
                                         cSrcSize: size_t,
                                         DTable: *const HUF_DTable,
                                         bmi2: libc::c_int) -> size_t;
    #[no_mangle]
    fn HUF_decompress1X1_DCtx_wksp_bmi2(dctx: *mut HUF_DTable,
                                        dst: *mut libc::c_void,
                                        dstSize: size_t,
                                        cSrc: *const libc::c_void,
                                        cSrcSize: size_t,
                                        workSpace: *mut libc::c_void,
                                        wkspSize: size_t, bmi2: libc::c_int)
     -> size_t;
    #[no_mangle]
    fn HUF_decompress4X_usingDTable_bmi2(dst: *mut libc::c_void,
                                         maxDstSize: size_t,
                                         cSrc: *const libc::c_void,
                                         cSrcSize: size_t,
                                         DTable: *const HUF_DTable,
                                         bmi2: libc::c_int) -> size_t;
    #[no_mangle]
    fn HUF_decompress4X_hufOnly_wksp_bmi2(dctx: *mut HUF_DTable,
                                          dst: *mut libc::c_void,
                                          dstSize: size_t,
                                          cSrc: *const libc::c_void,
                                          cSrcSize: size_t,
                                          workSpace: *mut libc::c_void,
                                          wkspSize: size_t, bmi2: libc::c_int)
     -> size_t;
    /* ! ZSTD_checkContinuity() :
 *  check if next `dst` follows previous position, where decompression ended.
 *  If yes, do nothing (continue on current segment).
 *  If not, classify previous segment as "external dictionary", and start a new segment.
 *  This function cannot fail. */
    #[no_mangle]
    fn ZSTD_checkContinuity(dctx: *mut ZSTD_DCtx, dst: *const libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int16_t = __int16_t;
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
pub type S16 = int16_t;
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
/* Start with initCStream, providing the size of buffer to write into.
*  bitStream will never write outside of this buffer.
*  `dstCapacity` must be >= sizeof(bitD->bitContainer), otherwise @return will be an error code.
*
*  bits are first added to a local register.
*  Local register is size_t, hence 64-bits on 64-bits systems, or 32-bits on 32-bits systems.
*  Writing data into memory is an explicit operation, performed by the flushBits function.
*  Hence keep track how many bits are potentially stored into local register to avoid register overflow.
*  After a flushBits, a maximum of 7 bits might still be stored into local register.
*
*  Avoid storing elements of more than 24 bits if you want compatibility with 32-bits bitstream readers.
*
*  Last operation is to close the bitStream.
*  The function returns the final size of CStream in bytes.
*  If data couldn't fit into `dstBuffer`, it will return a 0 ( == not storable)
*/
/*-********************************************
*  bitStream decoding API (read backward)
**********************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BIT_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: libc::c_uint,
    pub ptr: *const libc::c_char,
    pub start: *const libc::c_char,
    pub limitPtr: *const libc::c_char,
}
pub type BIT_DStream_status = libc::c_uint;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
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
/*= Decompression context
 *  When decompressing many times,
 *  it is recommended to allocate a context only once,
 *  and re-use it for each successive compression operation.
 *  This will make workload friendlier for system's memory.
 *  Use one context per thread for parallel execution. */
pub type ZSTD_DCtx = ZSTD_DCtx_s;
/* We need to add at most (ZSTD_WINDOWLOG_MAX_32 - 1) bits to read the maximum
 * offset bits. But we can only read at most (STREAM_ACCUMULATOR_MIN_32 - 1)
 * bits before reloading. This value is the maximum number of bytes we read
 * after reloading when we are decoding long offets.
 */
pub type ZSTD_longOffset_e = libc::c_uint;
pub const ZSTD_lo_isLongOffset: ZSTD_longOffset_e = 1;
pub const ZSTD_lo_isRegularOffset: ZSTD_longOffset_e = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct seqState_t {
    pub DStream: BIT_DStream_t,
    pub stateLL: ZSTD_fseState,
    pub stateOffb: ZSTD_fseState,
    pub stateML: ZSTD_fseState,
    pub prevOffset: [size_t; 3],
    pub prefixStart: *const BYTE,
    pub dictEnd: *const BYTE,
    pub pos: size_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_fseState {
    pub state: size_t,
    pub table: *const ZSTD_seqSymbol,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct seq_t {
    pub litLength: size_t,
    pub matchLength: size_t,
    pub offset: size_t,
    pub match_0: *const BYTE,
}
/*-*******************************************************
 *  Decompression types
 *********************************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_seqSymbol_header {
    pub fastMode: U32,
    pub tableLog: U32,
}
/* nbSeq==0 */
/*litCSize*/
/* RLE or RAW */
/* nbSeq==0 */
/* for a non-null block */
pub type symbolEncodingType_e = libc::c_uint;
pub const set_repeat: symbolEncodingType_e = 3;
pub const set_compressed: symbolEncodingType_e = 2;
pub const set_rle: symbolEncodingType_e = 1;
pub const set_basic: symbolEncodingType_e = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct blockProperties_t {
    pub blockType: blockType_e,
    pub lastBlock: U32,
    pub origSize: U32,
}
/*-**************************************************************
*  Memory I/O
*****************************************************************/
/* MEM_FORCE_MEMORY_ACCESS :
 * By default, access to unaligned memory is controlled by `memcpy()`, which is safe and portable.
 * Unfortunately, on some target/compiler combinations, the generated assembly is sub-optimal.
 * The below switch allow to select different access method for improved performance.
 * Method 0 (default) : use `memcpy()`. Safe and portable.
 * Method 1 : `__packed` statement. It depends on compiler extension (i.e., not portable).
 *            This method is safe if your compiler supports it, and *generally* as fast or faster than `memcpy`.
 * Method 2 : direct access. This method is portable but violate C standard.
 *            It can generate buggy code on targets depending on alignment.
 *            In some circumstances, it's the only known way to get the most performance (i.e. GCC + ARMv6)
 * See http://fastcompression.blogspot.fr/2015/08/accessing-unaligned-memory.html for details.
 * Prefer these methods in priority order (0 > 1 > 2)
 */
/* can be defined externally, on command line for example */
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::std::mem::size_of::<size_t>() as libc::c_ulong ==
                4i32 as libc::c_ulong) as libc::c_int as libc::c_uint;
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
unsafe extern "C" fn MEM_readLE24(mut memPtr: *const libc::c_void) -> U32 {
    return (MEM_readLE16(memPtr) as libc::c_int +
                ((*(memPtr as *const BYTE).offset(2isize) as libc::c_int) <<
                     16i32)) as U32;
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
unsafe extern "C" fn MEM_readLEST(mut memPtr: *const libc::c_void) -> size_t {
    if 0 != MEM_32bits() {
        return MEM_readLE32(memPtr) as size_t
    } else { return MEM_readLE64(memPtr) };
}
/*-****************************************
*  Error codes handling
******************************************/
/* reported already defined on VS 2015 (Rich Geldreich) */
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as
               libc::c_int as libc::c_uint;
}
/* Software version */
/*=====    Local Constants   =====*/
static mut BIT_mask: [libc::c_uint; 32] =
    [0i32 as libc::c_uint, 1i32 as libc::c_uint, 3i32 as libc::c_uint,
     7i32 as libc::c_uint, 0xfi32 as libc::c_uint, 0x1fi32 as libc::c_uint,
     0x3fi32 as libc::c_uint, 0x7fi32 as libc::c_uint,
     0xffi32 as libc::c_uint, 0x1ffi32 as libc::c_uint,
     0x3ffi32 as libc::c_uint, 0x7ffi32 as libc::c_uint,
     0xfffi32 as libc::c_uint, 0x1fffi32 as libc::c_uint,
     0x3fffi32 as libc::c_uint, 0x7fffi32 as libc::c_uint,
     0xffffi32 as libc::c_uint, 0x1ffffi32 as libc::c_uint,
     0x3ffffi32 as libc::c_uint, 0x7ffffi32 as libc::c_uint,
     0xfffffi32 as libc::c_uint, 0x1fffffi32 as libc::c_uint,
     0x3fffffi32 as libc::c_uint, 0x7fffffi32 as libc::c_uint,
     0xffffffi32 as libc::c_uint, 0x1ffffffi32 as libc::c_uint,
     0x3ffffffi32 as libc::c_uint, 0x7ffffffi32 as libc::c_uint,
     0xfffffffi32 as libc::c_uint, 0x1fffffffi32 as libc::c_uint,
     0x3fffffffi32 as libc::c_uint, 0x7fffffffi32 as libc::c_uint];
/* 1,2,4,8 would be better for bitmap combinations, but slows down performance a bit ... :( */
unsafe extern "C" fn BIT_initDStream(mut bitD: *mut BIT_DStream_t,
                                     mut srcBuffer: *const libc::c_void,
                                     mut srcSize: size_t) -> size_t {
    if srcSize < 1i32 as libc::c_ulong {
        memset(bitD as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<BIT_DStream_t>() as libc::c_ulong);
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    (*bitD).start = srcBuffer as *const libc::c_char;
    (*bitD).limitPtr =
        (*bitD).start.offset(::std::mem::size_of::<size_t>() as libc::c_ulong
                                 as isize);
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
        let mut current_block_20: u64;
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
                /* fall-through */
                current_block_20 = 841891325399266334;
            }
            6 => { current_block_20 = 841891325399266334; }
            5 => { current_block_20 = 1701641549668001212; }
            4 => { current_block_20 = 4465339460277708311; }
            3 => { current_block_20 = 12924973157245942766; }
            2 => { current_block_20 = 7257324432027208013; }
            _ => { current_block_20 = 14576567515993809846; }
        }
        match current_block_20 {
            841891325399266334 => {
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
                /* fall-through */
                current_block_20 = 1701641549668001212;
            }
            _ => { }
        }
        match current_block_20 {
            1701641549668001212 => {
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
                /* fall-through */
                current_block_20 = 4465339460277708311;
            }
            _ => { }
        }
        match current_block_20 {
            4465339460277708311 => {
                (*bitD).bitContainer =
                    ((*bitD).bitContainer as
                         libc::c_ulong).wrapping_add((*(srcBuffer as
                                                            *const BYTE).offset(3isize)
                                                          as size_t) << 24i32)
                        as size_t as size_t;
                /* fall-through */
                current_block_20 = 12924973157245942766;
            }
            _ => { }
        }
        match current_block_20 {
            12924973157245942766 => {
                (*bitD).bitContainer =
                    ((*bitD).bitContainer as
                         libc::c_ulong).wrapping_add((*(srcBuffer as
                                                            *const BYTE).offset(2isize)
                                                          as size_t) << 16i32)
                        as size_t as size_t;
                /* fall-through */
                current_block_20 = 7257324432027208013;
            }
            _ => { }
        }
        match current_block_20 {
            7257324432027208013 => {
                (*bitD).bitContainer =
                    ((*bitD).bitContainer as
                         libc::c_ulong).wrapping_add((*(srcBuffer as
                                                            *const BYTE).offset(1isize)
                                                          as size_t) << 8i32)
                        as size_t as size_t
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
                     libc::c_uint).wrapping_sub(BIT_highbit32(lastByte_0 as
                                                                  U32))
            } else { 0i32 as libc::c_uint };
        if lastByte_0 as libc::c_int == 0i32 {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        }
        (*bitD).bitsConsumed =
            (*bitD).bitsConsumed.wrapping_add(((::std::mem::size_of::<size_t>()
                                                    as
                                                    libc::c_ulong).wrapping_sub(srcSize)
                                                   as
                                                   U32).wrapping_mul(8i32 as
                                                                         libc::c_uint))
    }
    return srcSize;
}
/* faster, but works only if nbBits >= 1 */
/*-**************************************************************
*  Internal functions
****************************************************************/
unsafe extern "C" fn BIT_highbit32(mut val: U32) -> libc::c_uint {
    return (31i32 - val.leading_zeros() as i32) as libc::c_uint;
}
unsafe extern "C" fn BIT_readBits(mut bitD: *mut BIT_DStream_t,
                                  mut nbBits: libc::c_uint) -> size_t {
    let value: size_t = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
/* ! BIT_lookBits() :
 *  Provides next n bits from local register.
 *  local register is not modified.
 *  On 32-bits, maxNbBits==24.
 *  On 64-bits, maxNbBits==56.
 * @return : value extracted */
unsafe extern "C" fn BIT_lookBits(mut bitD: *const BIT_DStream_t,
                                  mut nbBits: U32) -> size_t {
    return BIT_getMiddleBits((*bitD).bitContainer,
                             (::std::mem::size_of::<size_t>() as
                                  libc::c_ulong).wrapping_mul(8i32 as
                                                                  libc::c_ulong).wrapping_sub((*bitD).bitsConsumed
                                                                                                  as
                                                                                                  libc::c_ulong).wrapping_sub(nbBits
                                                                                                                                  as
                                                                                                                                  libc::c_ulong)
                                 as U32, nbBits);
}
unsafe extern "C" fn BIT_getMiddleBits(mut bitContainer: size_t, start: U32,
                                       nbBits: U32) -> size_t {
    let regMask: U32 =
        (::std::mem::size_of::<size_t>() as
             libc::c_ulong).wrapping_mul(8i32 as
                                             libc::c_ulong).wrapping_sub(1i32
                                                                             as
                                                                             libc::c_ulong)
            as U32;
    return bitContainer >> (start & regMask) &
               BIT_mask[nbBits as usize] as libc::c_ulong;
}
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t,
                                  mut nbBits: U32) {
    (*bitD).bitsConsumed = (*bitD).bitsConsumed.wrapping_add(nbBits);
}
unsafe extern "C" fn BIT_reloadDStream(mut bitD: *mut BIT_DStream_t)
 -> BIT_DStream_status {
    if (*bitD).bitsConsumed as libc::c_ulong >
           (::std::mem::size_of::<size_t>() as
                libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
        return BIT_DStream_overflow
    }
    if (*bitD).ptr >= (*bitD).limitPtr {
        (*bitD).ptr =
            (*bitD).ptr.offset(-(((*bitD).bitsConsumed >> 3i32) as isize));
        (*bitD).bitsConsumed &= 7i32 as libc::c_uint;
        (*bitD).bitContainer =
            MEM_readLEST((*bitD).ptr as *const libc::c_void);
        return BIT_DStream_unfinished
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as libc::c_ulong) <
               (::std::mem::size_of::<size_t>() as
                    libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
            return BIT_DStream_endOfBuffer
        }
        return BIT_DStream_completed
    }
    let mut nbBytes: U32 = (*bitD).bitsConsumed >> 3i32;
    let mut result: BIT_DStream_status = BIT_DStream_unfinished;
    if (*bitD).ptr.offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes =
            (*bitD).ptr.wrapping_offset_from((*bitD).start) as libc::c_long as
                U32;
        result = BIT_DStream_endOfBuffer
    }
    (*bitD).ptr = (*bitD).ptr.offset(-(nbBytes as isize));
    (*bitD).bitsConsumed =
        (*bitD).bitsConsumed.wrapping_sub(nbBytes.wrapping_mul(8i32 as
                                                                   libc::c_uint));
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const libc::c_void);
    return result;
}
/* unsafe version; does not check buffer overflow */
unsafe extern "C" fn BIT_readBitsFast(mut bitD: *mut BIT_DStream_t,
                                      mut nbBits: libc::c_uint) -> size_t {
    let value: size_t = BIT_lookBitsFast(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
/* ! BIT_lookBitsFast() :
 *  unsafe version; only works if nbBits >= 1 */
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
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBlock(mut dctx: *mut ZSTD_DCtx,
                                              mut dst: *mut libc::c_void,
                                              mut dstCapacity: size_t,
                                              mut src: *const libc::c_void,
                                              mut srcSize: size_t) -> size_t {
    let mut dSize: size_t = 0;
    ZSTD_checkContinuity(dctx, dst);
    dSize =
        ZSTD_decompressBlock_internal(dctx, dst, dstCapacity, src, srcSize,
                                      0i32);
    (*dctx).previousDstEnd =
        (dst as *mut libc::c_char).offset(dSize as isize) as
            *const libc::c_void;
    return dSize;
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
pub unsafe extern "C" fn ZSTD_decompressBlock_internal(mut dctx:
                                                           *mut ZSTD_DCtx,
                                                       mut dst:
                                                           *mut libc::c_void,
                                                       mut dstCapacity:
                                                           size_t,
                                                       mut src:
                                                           *const libc::c_void,
                                                       mut srcSize: size_t,
                                                       frame: libc::c_int)
 -> size_t {
    let mut ip: *const BYTE = src as *const BYTE;
    /* isLongOffset must be true if there are long offsets.
     * Offsets are long if they are larger than 2^STREAM_ACCUMULATOR_MIN.
     * We don't expect that to be the case in 64-bit mode.
     * In block mode, window size is not known, so we have to be conservative.
     * (note: but it could be evaluated from current-lowLimit)
     */
    let isLongOffset: ZSTD_longOffset_e =
        (0 != MEM_32bits() &&
             (0 == frame ||
                  (*dctx).fParams.windowSize >
                      1u64 <<
                          (if 0 != MEM_32bits() { 25i32 } else { 57i32 }) as
                              U32)) as libc::c_int as ZSTD_longOffset_e;
    if srcSize >= (1i32 << 17i32) as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    let litCSize: size_t = ZSTD_decodeLiteralsBlock(dctx, src, srcSize);
    if 0 != ERR_isError(litCSize) { return litCSize }
    ip = ip.offset(litCSize as isize);
    srcSize =
        (srcSize as libc::c_ulong).wrapping_sub(litCSize) as size_t as size_t;
    let mut usePrefetchDecoder: libc::c_int = (*dctx).ddictIsCold;
    let mut nbSeq: libc::c_int = 0;
    let seqHSize: size_t =
        ZSTD_decodeSeqHeaders(dctx, &mut nbSeq, ip as *const libc::c_void,
                              srcSize);
    if 0 != ERR_isError(seqHSize) { return seqHSize }
    ip = ip.offset(seqHSize as isize);
    srcSize =
        (srcSize as libc::c_ulong).wrapping_sub(seqHSize) as size_t as size_t;
    if 0 == usePrefetchDecoder &&
           (0 == frame ||
                (*dctx).fParams.windowSize >
                    (1i32 << 24i32) as libc::c_ulonglong) && nbSeq > 4i32 {
        let shareLongOffsets: U32 = ZSTD_getLongOffsetsShare((*dctx).OFTptr);
        let minShare: U32 =
            (if 0 != MEM_64bits() { 7i32 } else { 20i32 }) as U32;
        usePrefetchDecoder = (shareLongOffsets >= minShare) as libc::c_int
    }
    (*dctx).ddictIsCold = 0i32;
    if 0 != usePrefetchDecoder {
        return ZSTD_decompressSequencesLong(dctx, dst, dstCapacity,
                                            ip as *const libc::c_void,
                                            srcSize, nbSeq, isLongOffset)
    }
    return ZSTD_decompressSequences(dctx, dst, dstCapacity,
                                    ip as *const libc::c_void, srcSize, nbSeq,
                                    isLongOffset);
}
unsafe extern "C" fn ZSTD_decompressSequences(mut dctx: *mut ZSTD_DCtx,
                                              mut dst: *mut libc::c_void,
                                              mut maxDstSize: size_t,
                                              mut seqStart:
                                                  *const libc::c_void,
                                              mut seqSize: size_t,
                                              mut nbSeq: libc::c_int,
                                              isLongOffset: ZSTD_longOffset_e)
 -> size_t {
    if 0 != (*dctx).bmi2 {
        return ZSTD_decompressSequences_bmi2(dctx, dst, maxDstSize, seqStart,
                                             seqSize, nbSeq, isLongOffset)
    }
    return ZSTD_decompressSequences_default(dctx, dst, maxDstSize, seqStart,
                                            seqSize, nbSeq, isLongOffset);
}
unsafe extern "C" fn ZSTD_decompressSequences_default(mut dctx:
                                                          *mut ZSTD_DCtx,
                                                      mut dst:
                                                          *mut libc::c_void,
                                                      mut maxDstSize: size_t,
                                                      mut seqStart:
                                                          *const libc::c_void,
                                                      mut seqSize: size_t,
                                                      mut nbSeq: libc::c_int,
                                                      isLongOffset:
                                                          ZSTD_longOffset_e)
 -> size_t {
    return ZSTD_decompressSequences_body(dctx, dst, maxDstSize, seqStart,
                                         seqSize, nbSeq, isLongOffset);
}
unsafe extern "C" fn ZSTD_decompressSequences_body(mut dctx: *mut ZSTD_DCtx,
                                                   mut dst: *mut libc::c_void,
                                                   mut maxDstSize: size_t,
                                                   mut seqStart:
                                                       *const libc::c_void,
                                                   mut seqSize: size_t,
                                                   mut nbSeq: libc::c_int,
                                                   isLongOffset:
                                                       ZSTD_longOffset_e)
 -> size_t {
    let mut ip: *const BYTE = seqStart as *const BYTE;
    let iend: *const BYTE = ip.offset(seqSize as isize);
    let ostart: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = ostart.offset(maxDstSize as isize);
    let mut op: *mut BYTE = ostart;
    let mut litPtr: *const BYTE = (*dctx).litPtr;
    let litEnd: *const BYTE = litPtr.offset((*dctx).litSize as isize);
    let prefixStart: *const BYTE = (*dctx).prefixStart as *const BYTE;
    let vBase: *const BYTE = (*dctx).virtualStart as *const BYTE;
    let dictEnd: *const BYTE = (*dctx).dictEnd as *const BYTE;
    if 0 != nbSeq {
        let mut seqState: seqState_t =
            seqState_t{DStream:
                           BIT_DStream_t{bitContainer: 0,
                                         bitsConsumed: 0,
                                         ptr: 0 as *const libc::c_char,
                                         start: 0 as *const libc::c_char,
                                         limitPtr: 0 as *const libc::c_char,},
                       stateLL:
                           ZSTD_fseState{state: 0,
                                         table: 0 as *const ZSTD_seqSymbol,},
                       stateOffb:
                           ZSTD_fseState{state: 0,
                                         table: 0 as *const ZSTD_seqSymbol,},
                       stateML:
                           ZSTD_fseState{state: 0,
                                         table: 0 as *const ZSTD_seqSymbol,},
                       prevOffset: [0; 3],
                       prefixStart: 0 as *const BYTE,
                       dictEnd: 0 as *const BYTE,
                       pos: 0,};
        (*dctx).fseEntropy = 1i32 as U32;
        let mut i: U32 = 0;
        i = 0i32 as U32;
        while i < 3i32 as libc::c_uint {
            seqState.prevOffset[i as usize] =
                (*dctx).entropy.rep[i as usize] as size_t;
            i = i.wrapping_add(1)
        }
        let errcod: size_t =
            BIT_initDStream(&mut seqState.DStream, ip as *const libc::c_void,
                            iend.wrapping_offset_from(ip) as libc::c_long as
                                size_t);
        if 0 != ERR_isError(errcod) {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        }
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream,
                          (*dctx).LLTptr);
        ZSTD_initFseState(&mut seqState.stateOffb, &mut seqState.DStream,
                          (*dctx).OFTptr);
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream,
                          (*dctx).MLTptr);
        while BIT_reloadDStream(&mut seqState.DStream) as libc::c_uint <=
                  BIT_DStream_completed as libc::c_int as libc::c_uint &&
                  0 != nbSeq {
            nbSeq -= 1;
            let sequence: seq_t =
                ZSTD_decodeSequence(&mut seqState, isLongOffset);
            let oneSeqSize: size_t =
                ZSTD_execSequence(op, oend, sequence, &mut litPtr, litEnd,
                                  prefixStart, vBase, dictEnd);
            if 0 != ERR_isError(oneSeqSize) { return oneSeqSize }
            op = op.offset(oneSeqSize as isize)
        }
        if 0 != nbSeq {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        }
        let mut i_0: U32 = 0;
        i_0 = 0i32 as U32;
        while i_0 < 3i32 as libc::c_uint {
            (*dctx).entropy.rep[i_0 as usize] =
                seqState.prevOffset[i_0 as usize] as U32;
            i_0 = i_0.wrapping_add(1)
        }
    }
    let lastLLSize: size_t =
        litEnd.wrapping_offset_from(litPtr) as libc::c_long as size_t;
    if lastLLSize > oend.wrapping_offset_from(op) as libc::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    memcpy(op as *mut libc::c_void, litPtr as *const libc::c_void,
           lastLLSize);
    op = op.offset(lastLLSize as isize);
    return op.wrapping_offset_from(ostart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_decodeSequence(mut seqState: *mut seqState_t,
                                         longOffsets: ZSTD_longOffset_e)
 -> seq_t {
    let mut seq: seq_t =
        seq_t{litLength: 0,
              matchLength: 0,
              offset: 0,
              match_0: 0 as *const BYTE,};
    let llBits: U32 =
        (*(*seqState).stateLL.table.offset((*seqState).stateLL.state as
                                               isize)).nbAdditionalBits as
            U32;
    let mlBits: U32 =
        (*(*seqState).stateML.table.offset((*seqState).stateML.state as
                                               isize)).nbAdditionalBits as
            U32;
    let ofBits: U32 =
        (*(*seqState).stateOffb.table.offset((*seqState).stateOffb.state as
                                                 isize)).nbAdditionalBits as
            U32;
    let totalBits: U32 = llBits.wrapping_add(mlBits).wrapping_add(ofBits);
    let llBase: U32 =
        (*(*seqState).stateLL.table.offset((*seqState).stateLL.state as
                                               isize)).baseValue;
    let mlBase: U32 =
        (*(*seqState).stateML.table.offset((*seqState).stateML.state as
                                               isize)).baseValue;
    let ofBase: U32 =
        (*(*seqState).stateOffb.table.offset((*seqState).stateOffb.state as
                                                 isize)).baseValue;
    let mut offset: size_t = 0;
    if 0 == ofBits {
        offset = 0i32 as size_t
    } else if 0 != MEM_32bits() && 0 != longOffsets as libc::c_uint &&
                  ofBits >= 25i32 as libc::c_uint {
        let extraBits: U32 =
            ofBits.wrapping_sub(if ofBits <
                                       (32i32 as
                                            libc::c_uint).wrapping_sub((*seqState).DStream.bitsConsumed)
                                   {
                                    ofBits
                                } else {
                                    (32i32 as
                                         libc::c_uint).wrapping_sub((*seqState).DStream.bitsConsumed)
                                });
        offset =
            (ofBase as
                 libc::c_ulong).wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream,
                                                              ofBits.wrapping_sub(extraBits))
                                                 << extraBits);
        BIT_reloadDStream(&mut (*seqState).DStream);
        if 0 != extraBits {
            offset =
                (offset as
                     libc::c_ulong).wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream,
                                                                  extraBits))
                    as size_t as size_t
        }
    } else {
        offset =
            (ofBase as
                 libc::c_ulong).wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream,
                                                              ofBits));
        if 0 != MEM_32bits() { BIT_reloadDStream(&mut (*seqState).DStream); }
    }
    if ofBits <= 1i32 as libc::c_uint {
        offset =
            (offset as
                 libc::c_ulong).wrapping_add((llBase == 0i32 as libc::c_uint)
                                                 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t;
        if 0 != offset {
            let mut temp: size_t =
                if offset == 3i32 as libc::c_ulong {
                    (*seqState).prevOffset[0usize].wrapping_sub(1i32 as
                                                                    libc::c_ulong)
                } else { (*seqState).prevOffset[offset as usize] };
            temp =
                (temp as
                     libc::c_ulong).wrapping_add((0 == temp) as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t;
            if offset != 1i32 as libc::c_ulong {
                (*seqState).prevOffset[2usize] =
                    (*seqState).prevOffset[1usize]
            }
            (*seqState).prevOffset[1usize] = (*seqState).prevOffset[0usize];
            offset = temp;
            (*seqState).prevOffset[0usize] = offset
        } else { offset = (*seqState).prevOffset[0usize] }
    } else {
        (*seqState).prevOffset[2usize] = (*seqState).prevOffset[1usize];
        (*seqState).prevOffset[1usize] = (*seqState).prevOffset[0usize];
        (*seqState).prevOffset[0usize] = offset
    }
    seq.offset = offset;
    seq.matchLength =
        (mlBase as
             libc::c_ulong).wrapping_add(if mlBits > 0i32 as libc::c_uint {
                                             BIT_readBitsFast(&mut (*seqState).DStream,
                                                              mlBits)
                                         } else { 0i32 as libc::c_ulong });
    if 0 != MEM_32bits() &&
           mlBits.wrapping_add(llBits) >=
               (25i32 - if 30i32 > 25i32 { 30i32 - 25i32 } else { 0i32 }) as
                   libc::c_uint {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    if 0 != MEM_64bits() &&
           totalBits >= (57i32 - (9i32 + 9i32 + 8i32)) as libc::c_uint {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    seq.litLength =
        (llBase as
             libc::c_ulong).wrapping_add(if llBits > 0i32 as libc::c_uint {
                                             BIT_readBitsFast(&mut (*seqState).DStream,
                                                              llBits)
                                         } else { 0i32 as libc::c_ulong });
    if 0 != MEM_32bits() { BIT_reloadDStream(&mut (*seqState).DStream); }
    ZSTD_updateFseState(&mut (*seqState).stateLL, &mut (*seqState).DStream);
    ZSTD_updateFseState(&mut (*seqState).stateML, &mut (*seqState).DStream);
    if 0 != MEM_32bits() { BIT_reloadDStream(&mut (*seqState).DStream); }
    ZSTD_updateFseState(&mut (*seqState).stateOffb, &mut (*seqState).DStream);
    return seq;
}
unsafe extern "C" fn ZSTD_updateFseState(mut DStatePtr: *mut ZSTD_fseState,
                                         mut bitD: *mut BIT_DStream_t) {
    let DInfo: ZSTD_seqSymbol =
        *(*DStatePtr).table.offset((*DStatePtr).state as isize);
    let nbBits: U32 = DInfo.nbBits as U32;
    let lowBits: size_t = BIT_readBits(bitD, nbBits);
    (*DStatePtr).state =
        (DInfo.nextState as libc::c_ulong).wrapping_add(lowBits);
}
unsafe extern "C" fn ZSTD_execSequence(mut op: *mut BYTE, oend: *mut BYTE,
                                       mut sequence: seq_t,
                                       mut litPtr: *mut *const BYTE,
                                       litLimit: *const BYTE,
                                       prefixStart: *const BYTE,
                                       virtualStart: *const BYTE,
                                       dictEnd: *const BYTE) -> size_t {
    let oLitEnd: *mut BYTE = op.offset(sequence.litLength as isize);
    let sequenceLength: size_t =
        sequence.litLength.wrapping_add(sequence.matchLength);
    /* risk : address space overflow (32-bits) */
    let oMatchEnd: *mut BYTE = op.offset(sequenceLength as isize);
    let oend_w: *mut BYTE = oend.offset(-8isize);
    let iLitEnd: *const BYTE = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const BYTE =
        oLitEnd.offset(-(sequence.offset as isize));
    if oMatchEnd > oend {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    if iLitEnd > litLimit {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    }
    if oLitEnd > oend_w {
        return ZSTD_execSequenceLast7(op, oend, sequence, litPtr, litLimit,
                                      prefixStart, virtualStart, dictEnd)
    }
    ZSTD_copy8(op as *mut libc::c_void, *litPtr as *const libc::c_void);
    if sequence.litLength > 8i32 as libc::c_ulong {
        ZSTD_wildcopy(op.offset(8isize) as *mut libc::c_void,
                      (*litPtr).offset(8isize) as *const libc::c_void,
                      sequence.litLength.wrapping_sub(8i32 as libc::c_ulong)
                          as ptrdiff_t);
    }
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset >
           oLitEnd.wrapping_offset_from(prefixStart) as libc::c_long as size_t
       {
        if sequence.offset >
               oLitEnd.wrapping_offset_from(virtualStart) as libc::c_long as
                   size_t {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        }
        match_0 =
            dictEnd.offset(match_0.wrapping_offset_from(prefixStart) as
                               libc::c_long as isize);
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            memmove(oLitEnd as *mut libc::c_void,
                    match_0 as *const libc::c_void, sequence.matchLength);
            return sequenceLength
        }
        let length1: size_t =
            dictEnd.wrapping_offset_from(match_0) as libc::c_long as size_t;
        memmove(oLitEnd as *mut libc::c_void, match_0 as *const libc::c_void,
                length1);
        op = oLitEnd.offset(length1 as isize);
        sequence.matchLength =
            (sequence.matchLength as libc::c_ulong).wrapping_sub(length1) as
                size_t as size_t;
        match_0 = prefixStart;
        if op > oend_w || sequence.matchLength < 3i32 as libc::c_ulong {
            let mut i: U32 = 0;
            i = 0i32 as U32;
            while (i as libc::c_ulong) < sequence.matchLength {
                *op.offset(i as isize) = *match_0.offset(i as isize);
                i = i.wrapping_add(1)
            }
            return sequenceLength
        }
    }
    if sequence.offset < 8i32 as libc::c_ulong {
        static mut dec32table: [U32; 8] =
            [0i32 as U32, 1i32 as U32, 2i32 as U32, 1i32 as U32, 4i32 as U32,
             4i32 as U32, 4i32 as U32, 4i32 as U32];
        static mut dec64table: [libc::c_int; 8] =
            [8i32, 8i32, 8i32, 7i32, 8i32, 9i32, 10i32, 11i32];
        let sub2: libc::c_int = dec64table[sequence.offset as usize];
        *op.offset(0isize) = *match_0.offset(0isize);
        *op.offset(1isize) = *match_0.offset(1isize);
        *op.offset(2isize) = *match_0.offset(2isize);
        *op.offset(3isize) = *match_0.offset(3isize);
        match_0 =
            match_0.offset(dec32table[sequence.offset as usize] as isize);
        ZSTD_copy4(op.offset(4isize) as *mut libc::c_void,
                   match_0 as *const libc::c_void);
        match_0 = match_0.offset(-(sub2 as isize))
    } else {
        ZSTD_copy8(op as *mut libc::c_void, match_0 as *const libc::c_void);
    }
    op = op.offset(8isize);
    match_0 = match_0.offset(8isize);
    if oMatchEnd > oend.offset(-((16i32 - 3i32) as isize)) {
        if op < oend_w {
            ZSTD_wildcopy(op as *mut libc::c_void,
                          match_0 as *const libc::c_void,
                          oend_w.wrapping_offset_from(op) as libc::c_long);
            match_0 =
                match_0.offset(oend_w.wrapping_offset_from(op) as libc::c_long
                                   as isize);
            op = oend_w
        }
        while op < oMatchEnd {
            let fresh1 = op;
            op = op.offset(1);
            let fresh0 = match_0;
            match_0 = match_0.offset(1);
            *fresh1 = *fresh0
        }
    } else {
        ZSTD_wildcopy(op as *mut libc::c_void, match_0 as *const libc::c_void,
                      sequence.matchLength as ptrdiff_t -
                          8i32 as libc::c_long);
    }
    return sequenceLength;
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
/*-*******************************************
*  Shared functions to include for inlining
*********************************************/
unsafe extern "C" fn ZSTD_copy8(mut dst: *mut libc::c_void,
                                mut src: *const libc::c_void) {
    memcpy(dst, src, 8i32 as libc::c_ulong);
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
/* zstd_decompress_block :
 * this module takes care of decompressing _compressed_ block */
/*-*******************************************************
*  Dependencies
*********************************************************/
/* memcpy, memmove, memset */
/* low level memory routines */
/*_*******************************************************
*  Macros
**********************************************************/
/* These two optional macros force the use one way or another of the two
 * ZSTD_decompressSequences implementations. You can't force in both directions
 * at the same time.
 */
/*_*******************************************************
*  Memory operations
**********************************************************/
unsafe extern "C" fn ZSTD_copy4(mut dst: *mut libc::c_void,
                                mut src: *const libc::c_void) {
    memcpy(dst, src, 4i32 as libc::c_ulong);
}
/* ZSTD_execSequenceLast7():
 * exceptional case : decompress a match starting within last 7 bytes of output buffer.
 * requires more careful checks, to ensure there is no overflow.
 * performance does not matter though.
 * note : this case is supposed to be never generated "naturally" by reference encoder,
 *        since in most cases it needs at least 8 bytes to look for a match.
 *        but it's allowed by the specification. */
unsafe extern "C" fn ZSTD_execSequenceLast7(mut op: *mut BYTE,
                                            oend: *mut BYTE,
                                            mut sequence: seq_t,
                                            mut litPtr: *mut *const BYTE,
                                            litLimit: *const BYTE,
                                            base: *const BYTE,
                                            vBase: *const BYTE,
                                            dictEnd: *const BYTE) -> size_t {
    let oLitEnd: *mut BYTE = op.offset(sequence.litLength as isize);
    let sequenceLength: size_t =
        sequence.litLength.wrapping_add(sequence.matchLength);
    /* risk : address space overflow (32-bits) */
    let oMatchEnd: *mut BYTE = op.offset(sequenceLength as isize);
    let iLitEnd: *const BYTE = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const BYTE =
        oLitEnd.offset(-(sequence.offset as isize));
    if oMatchEnd > oend {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    if iLitEnd > litLimit {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    }
    while op < oLitEnd {
        let fresh3 = op;
        op = op.offset(1);
        let fresh2 = *litPtr;
        *litPtr = (*litPtr).offset(1);
        *fresh3 = *fresh2
    }
    if sequence.offset >
           oLitEnd.wrapping_offset_from(base) as libc::c_long as size_t {
        if sequence.offset >
               oLitEnd.wrapping_offset_from(vBase) as libc::c_long as size_t {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        }
        match_0 =
            dictEnd.offset(-(base.wrapping_offset_from(match_0) as
                                 libc::c_long as isize));
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            memmove(oLitEnd as *mut libc::c_void,
                    match_0 as *const libc::c_void, sequence.matchLength);
            return sequenceLength
        }
        let length1: size_t =
            dictEnd.wrapping_offset_from(match_0) as libc::c_long as size_t;
        memmove(oLitEnd as *mut libc::c_void, match_0 as *const libc::c_void,
                length1);
        op = oLitEnd.offset(length1 as isize);
        sequence.matchLength =
            (sequence.matchLength as libc::c_ulong).wrapping_sub(length1) as
                size_t as size_t;
        match_0 = base
    }
    while op < oMatchEnd {
        let fresh5 = op;
        op = op.offset(1);
        let fresh4 = match_0;
        match_0 = match_0.offset(1);
        *fresh5 = *fresh4
    }
    return sequenceLength;
}
unsafe extern "C" fn ZSTD_initFseState(mut DStatePtr: *mut ZSTD_fseState,
                                       mut bitD: *mut BIT_DStream_t,
                                       mut dt: *const ZSTD_seqSymbol) {
    let mut ptr: *const libc::c_void = dt as *const libc::c_void;
    let DTableH: *const ZSTD_seqSymbol_header =
        ptr as *const ZSTD_seqSymbol_header;
    (*DStatePtr).state = BIT_readBits(bitD, (*DTableH).tableLog);
    BIT_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1isize);
}
/* ZSTD_FORCE_DECOMPRESS_SEQUENCES_SHORT */
unsafe extern "C" fn ZSTD_decompressSequences_bmi2(mut dctx: *mut ZSTD_DCtx,
                                                   mut dst: *mut libc::c_void,
                                                   mut maxDstSize: size_t,
                                                   mut seqStart:
                                                       *const libc::c_void,
                                                   mut seqSize: size_t,
                                                   mut nbSeq: libc::c_int,
                                                   isLongOffset:
                                                       ZSTD_longOffset_e)
 -> size_t {
    return ZSTD_decompressSequences_body(dctx, dst, maxDstSize, seqStart,
                                         seqSize, nbSeq, isLongOffset);
}
/* ZSTD_FORCE_DECOMPRESS_SEQUENCES_LONG */
/* ZSTD_decompressSequencesLong() :
 * decompression function triggered when a minimum share of offsets is considered "long",
 * aka out of cache.
 * note : "long" definition seems overloaded here, sometimes meaning "wider than bitstream register", and sometimes mearning "farther than memory cache distance".
 * This function will try to mitigate main memory latency through the use of prefetching */
unsafe extern "C" fn ZSTD_decompressSequencesLong(mut dctx: *mut ZSTD_DCtx,
                                                  mut dst: *mut libc::c_void,
                                                  mut maxDstSize: size_t,
                                                  mut seqStart:
                                                      *const libc::c_void,
                                                  mut seqSize: size_t,
                                                  mut nbSeq: libc::c_int,
                                                  isLongOffset:
                                                      ZSTD_longOffset_e)
 -> size_t {
    if 0 != (*dctx).bmi2 {
        return ZSTD_decompressSequencesLong_bmi2(dctx, dst, maxDstSize,
                                                 seqStart, seqSize, nbSeq,
                                                 isLongOffset)
    }
    return ZSTD_decompressSequencesLong_default(dctx, dst, maxDstSize,
                                                seqStart, seqSize, nbSeq,
                                                isLongOffset);
}
unsafe extern "C" fn ZSTD_decompressSequencesLong_default(mut dctx:
                                                              *mut ZSTD_DCtx,
                                                          mut dst:
                                                              *mut libc::c_void,
                                                          mut maxDstSize:
                                                              size_t,
                                                          mut seqStart:
                                                              *const libc::c_void,
                                                          mut seqSize: size_t,
                                                          mut nbSeq:
                                                              libc::c_int,
                                                          isLongOffset:
                                                              ZSTD_longOffset_e)
 -> size_t {
    return ZSTD_decompressSequencesLong_body(dctx, dst, maxDstSize, seqStart,
                                             seqSize, nbSeq, isLongOffset);
}
unsafe extern "C" fn ZSTD_decompressSequencesLong_body(mut dctx:
                                                           *mut ZSTD_DCtx,
                                                       mut dst:
                                                           *mut libc::c_void,
                                                       mut maxDstSize: size_t,
                                                       mut seqStart:
                                                           *const libc::c_void,
                                                       mut seqSize: size_t,
                                                       mut nbSeq: libc::c_int,
                                                       isLongOffset:
                                                           ZSTD_longOffset_e)
 -> size_t {
    let mut ip: *const BYTE = seqStart as *const BYTE;
    let iend: *const BYTE = ip.offset(seqSize as isize);
    let ostart: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = ostart.offset(maxDstSize as isize);
    let mut op: *mut BYTE = ostart;
    let mut litPtr: *const BYTE = (*dctx).litPtr;
    let litEnd: *const BYTE = litPtr.offset((*dctx).litSize as isize);
    let prefixStart: *const BYTE = (*dctx).prefixStart as *const BYTE;
    let dictStart: *const BYTE = (*dctx).virtualStart as *const BYTE;
    let dictEnd: *const BYTE = (*dctx).dictEnd as *const BYTE;
    if 0 != nbSeq {
        let mut sequences: [seq_t; 4] =
            [seq_t{litLength: 0,
                   matchLength: 0,
                   offset: 0,
                   match_0: 0 as *const BYTE,}; 4];
        let seqAdvance: libc::c_int = if nbSeq < 4i32 { nbSeq } else { 4i32 };
        let mut seqState: seqState_t =
            seqState_t{DStream:
                           BIT_DStream_t{bitContainer: 0,
                                         bitsConsumed: 0,
                                         ptr: 0 as *const libc::c_char,
                                         start: 0 as *const libc::c_char,
                                         limitPtr: 0 as *const libc::c_char,},
                       stateLL:
                           ZSTD_fseState{state: 0,
                                         table: 0 as *const ZSTD_seqSymbol,},
                       stateOffb:
                           ZSTD_fseState{state: 0,
                                         table: 0 as *const ZSTD_seqSymbol,},
                       stateML:
                           ZSTD_fseState{state: 0,
                                         table: 0 as *const ZSTD_seqSymbol,},
                       prevOffset: [0; 3],
                       prefixStart: 0 as *const BYTE,
                       dictEnd: 0 as *const BYTE,
                       pos: 0,};
        let mut seqNb: libc::c_int = 0;
        (*dctx).fseEntropy = 1i32 as U32;
        let mut i: libc::c_int = 0;
        i = 0i32;
        while i < 3i32 {
            seqState.prevOffset[i as usize] =
                (*dctx).entropy.rep[i as usize] as size_t;
            i += 1
        }
        seqState.prefixStart = prefixStart;
        seqState.pos =
            op.wrapping_offset_from(prefixStart) as libc::c_long as size_t;
        seqState.dictEnd = dictEnd;
        let errcod: size_t =
            BIT_initDStream(&mut seqState.DStream, ip as *const libc::c_void,
                            iend.wrapping_offset_from(ip) as libc::c_long as
                                size_t);
        if 0 != ERR_isError(errcod) {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        }
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream,
                          (*dctx).LLTptr);
        ZSTD_initFseState(&mut seqState.stateOffb, &mut seqState.DStream,
                          (*dctx).OFTptr);
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream,
                          (*dctx).MLTptr);
        seqNb = 0i32;
        while BIT_reloadDStream(&mut seqState.DStream) as libc::c_uint <=
                  BIT_DStream_completed as libc::c_int as libc::c_uint &&
                  seqNb < seqAdvance {
            sequences[seqNb as usize] =
                ZSTD_decodeSequenceLong(&mut seqState, isLongOffset);
            seqNb += 1
        }
        if seqNb < seqAdvance {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        }
        while BIT_reloadDStream(&mut seqState.DStream) as libc::c_uint <=
                  BIT_DStream_completed as libc::c_int as libc::c_uint &&
                  seqNb < nbSeq {
            let sequence: seq_t =
                ZSTD_decodeSequenceLong(&mut seqState, isLongOffset);
            let oneSeqSize: size_t =
                ZSTD_execSequenceLong(op, oend,
                                      sequences[(seqNb - 4i32 & 4i32 - 1i32)
                                                    as usize], &mut litPtr,
                                      litEnd, prefixStart, dictStart,
                                      dictEnd);
            if 0 != ERR_isError(oneSeqSize) { return oneSeqSize }
            sequences[(seqNb & 4i32 - 1i32) as usize] = sequence;
            op = op.offset(oneSeqSize as isize);
            seqNb += 1
        }
        if seqNb < nbSeq {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        }
        seqNb -= seqAdvance;
        while seqNb < nbSeq {
            let oneSeqSize_0: size_t =
                ZSTD_execSequenceLong(op, oend,
                                      sequences[(seqNb & 4i32 - 1i32) as
                                                    usize], &mut litPtr,
                                      litEnd, prefixStart, dictStart,
                                      dictEnd);
            if 0 != ERR_isError(oneSeqSize_0) { return oneSeqSize_0 }
            op = op.offset(oneSeqSize_0 as isize);
            seqNb += 1
        }
        let mut i_0: U32 = 0;
        i_0 = 0i32 as U32;
        while i_0 < 3i32 as libc::c_uint {
            (*dctx).entropy.rep[i_0 as usize] =
                seqState.prevOffset[i_0 as usize] as U32;
            i_0 = i_0.wrapping_add(1)
        }
    }
    let lastLLSize: size_t =
        litEnd.wrapping_offset_from(litPtr) as libc::c_long as size_t;
    if lastLLSize > oend.wrapping_offset_from(op) as libc::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    memcpy(op as *mut libc::c_void, litPtr as *const libc::c_void,
           lastLLSize);
    op = op.offset(lastLLSize as isize);
    return op.wrapping_offset_from(ostart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_execSequenceLong(mut op: *mut BYTE, oend: *mut BYTE,
                                           mut sequence: seq_t,
                                           mut litPtr: *mut *const BYTE,
                                           litLimit: *const BYTE,
                                           prefixStart: *const BYTE,
                                           dictStart: *const BYTE,
                                           dictEnd: *const BYTE) -> size_t {
    let oLitEnd: *mut BYTE = op.offset(sequence.litLength as isize);
    let sequenceLength: size_t =
        sequence.litLength.wrapping_add(sequence.matchLength);
    /* risk : address space overflow (32-bits) */
    let oMatchEnd: *mut BYTE = op.offset(sequenceLength as isize);
    let oend_w: *mut BYTE = oend.offset(-8isize);
    let iLitEnd: *const BYTE = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const BYTE = sequence.match_0;
    if oMatchEnd > oend {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    if iLitEnd > litLimit {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    }
    if oLitEnd > oend_w {
        return ZSTD_execSequenceLast7(op, oend, sequence, litPtr, litLimit,
                                      prefixStart, dictStart, dictEnd)
    }
    ZSTD_copy8(op as *mut libc::c_void, *litPtr as *const libc::c_void);
    if sequence.litLength > 8i32 as libc::c_ulong {
        ZSTD_wildcopy(op.offset(8isize) as *mut libc::c_void,
                      (*litPtr).offset(8isize) as *const libc::c_void,
                      sequence.litLength.wrapping_sub(8i32 as libc::c_ulong)
                          as ptrdiff_t);
    }
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset >
           oLitEnd.wrapping_offset_from(prefixStart) as libc::c_long as size_t
       {
        if sequence.offset >
               oLitEnd.wrapping_offset_from(dictStart) as libc::c_long as
                   size_t {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        }
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            memmove(oLitEnd as *mut libc::c_void,
                    match_0 as *const libc::c_void, sequence.matchLength);
            return sequenceLength
        }
        let length1: size_t =
            dictEnd.wrapping_offset_from(match_0) as libc::c_long as size_t;
        memmove(oLitEnd as *mut libc::c_void, match_0 as *const libc::c_void,
                length1);
        op = oLitEnd.offset(length1 as isize);
        sequence.matchLength =
            (sequence.matchLength as libc::c_ulong).wrapping_sub(length1) as
                size_t as size_t;
        match_0 = prefixStart;
        if op > oend_w || sequence.matchLength < 3i32 as libc::c_ulong {
            let mut i: U32 = 0;
            i = 0i32 as U32;
            while (i as libc::c_ulong) < sequence.matchLength {
                *op.offset(i as isize) = *match_0.offset(i as isize);
                i = i.wrapping_add(1)
            }
            return sequenceLength
        }
    }
    if sequence.offset < 8i32 as libc::c_ulong {
        static mut dec32table: [U32; 8] =
            [0i32 as U32, 1i32 as U32, 2i32 as U32, 1i32 as U32, 4i32 as U32,
             4i32 as U32, 4i32 as U32, 4i32 as U32];
        static mut dec64table: [libc::c_int; 8] =
            [8i32, 8i32, 8i32, 7i32, 8i32, 9i32, 10i32, 11i32];
        let sub2: libc::c_int = dec64table[sequence.offset as usize];
        *op.offset(0isize) = *match_0.offset(0isize);
        *op.offset(1isize) = *match_0.offset(1isize);
        *op.offset(2isize) = *match_0.offset(2isize);
        *op.offset(3isize) = *match_0.offset(3isize);
        match_0 =
            match_0.offset(dec32table[sequence.offset as usize] as isize);
        ZSTD_copy4(op.offset(4isize) as *mut libc::c_void,
                   match_0 as *const libc::c_void);
        match_0 = match_0.offset(-(sub2 as isize))
    } else {
        ZSTD_copy8(op as *mut libc::c_void, match_0 as *const libc::c_void);
    }
    op = op.offset(8isize);
    match_0 = match_0.offset(8isize);
    if oMatchEnd > oend.offset(-((16i32 - 3i32) as isize)) {
        if op < oend_w {
            ZSTD_wildcopy(op as *mut libc::c_void,
                          match_0 as *const libc::c_void,
                          oend_w.wrapping_offset_from(op) as libc::c_long);
            match_0 =
                match_0.offset(oend_w.wrapping_offset_from(op) as libc::c_long
                                   as isize);
            op = oend_w
        }
        while op < oMatchEnd {
            let fresh7 = op;
            op = op.offset(1);
            let fresh6 = match_0;
            match_0 = match_0.offset(1);
            *fresh7 = *fresh6
        }
    } else {
        ZSTD_wildcopy(op as *mut libc::c_void, match_0 as *const libc::c_void,
                      sequence.matchLength as ptrdiff_t -
                          8i32 as libc::c_long);
    }
    return sequenceLength;
}
/* ZSTD_FORCE_DECOMPRESS_SEQUENCES_LONG */
unsafe extern "C" fn ZSTD_decodeSequenceLong(mut seqState: *mut seqState_t,
                                             longOffsets: ZSTD_longOffset_e)
 -> seq_t {
    let mut seq: seq_t =
        seq_t{litLength: 0,
              matchLength: 0,
              offset: 0,
              match_0: 0 as *const BYTE,};
    let llBits: U32 =
        (*(*seqState).stateLL.table.offset((*seqState).stateLL.state as
                                               isize)).nbAdditionalBits as
            U32;
    let mlBits: U32 =
        (*(*seqState).stateML.table.offset((*seqState).stateML.state as
                                               isize)).nbAdditionalBits as
            U32;
    let ofBits: U32 =
        (*(*seqState).stateOffb.table.offset((*seqState).stateOffb.state as
                                                 isize)).nbAdditionalBits as
            U32;
    let totalBits: U32 = llBits.wrapping_add(mlBits).wrapping_add(ofBits);
    let llBase: U32 =
        (*(*seqState).stateLL.table.offset((*seqState).stateLL.state as
                                               isize)).baseValue;
    let mlBase: U32 =
        (*(*seqState).stateML.table.offset((*seqState).stateML.state as
                                               isize)).baseValue;
    let ofBase: U32 =
        (*(*seqState).stateOffb.table.offset((*seqState).stateOffb.state as
                                                 isize)).baseValue;
    let mut offset: size_t = 0;
    if 0 == ofBits {
        offset = 0i32 as size_t
    } else if 0 != MEM_32bits() && 0 != longOffsets as libc::c_uint {
        let extraBits: U32 =
            ofBits.wrapping_sub(if ofBits < (25i32 - 1i32) as libc::c_uint {
                                    ofBits
                                } else { (25i32 - 1i32) as libc::c_uint });
        offset =
            (ofBase as
                 libc::c_ulong).wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream,
                                                              ofBits.wrapping_sub(extraBits))
                                                 << extraBits);
        if 0 != MEM_32bits() || 0 != extraBits {
            BIT_reloadDStream(&mut (*seqState).DStream);
        }
        if 0 != extraBits {
            offset =
                (offset as
                     libc::c_ulong).wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream,
                                                                  extraBits))
                    as size_t as size_t
        }
    } else {
        offset =
            (ofBase as
                 libc::c_ulong).wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream,
                                                              ofBits));
        if 0 != MEM_32bits() { BIT_reloadDStream(&mut (*seqState).DStream); }
    }
    if ofBits <= 1i32 as libc::c_uint {
        offset =
            (offset as
                 libc::c_ulong).wrapping_add((llBase == 0i32 as libc::c_uint)
                                                 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t;
        if 0 != offset {
            let mut temp: size_t =
                if offset == 3i32 as libc::c_ulong {
                    (*seqState).prevOffset[0usize].wrapping_sub(1i32 as
                                                                    libc::c_ulong)
                } else { (*seqState).prevOffset[offset as usize] };
            temp =
                (temp as
                     libc::c_ulong).wrapping_add((0 == temp) as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t;
            if offset != 1i32 as libc::c_ulong {
                (*seqState).prevOffset[2usize] =
                    (*seqState).prevOffset[1usize]
            }
            (*seqState).prevOffset[1usize] = (*seqState).prevOffset[0usize];
            offset = temp;
            (*seqState).prevOffset[0usize] = offset
        } else { offset = (*seqState).prevOffset[0usize] }
    } else {
        (*seqState).prevOffset[2usize] = (*seqState).prevOffset[1usize];
        (*seqState).prevOffset[1usize] = (*seqState).prevOffset[0usize];
        (*seqState).prevOffset[0usize] = offset
    }
    seq.offset = offset;
    seq.matchLength =
        (mlBase as
             libc::c_ulong).wrapping_add(if mlBits > 0i32 as libc::c_uint {
                                             BIT_readBitsFast(&mut (*seqState).DStream,
                                                              mlBits)
                                         } else { 0i32 as libc::c_ulong });
    if 0 != MEM_32bits() &&
           mlBits.wrapping_add(llBits) >=
               (25i32 - if 30i32 > 25i32 { 30i32 - 25i32 } else { 0i32 }) as
                   libc::c_uint {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    if 0 != MEM_64bits() &&
           totalBits >= (57i32 - (9i32 + 9i32 + 8i32)) as libc::c_uint {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    seq.litLength =
        (llBase as
             libc::c_ulong).wrapping_add(if llBits > 0i32 as libc::c_uint {
                                             BIT_readBitsFast(&mut (*seqState).DStream,
                                                              llBits)
                                         } else { 0i32 as libc::c_ulong });
    if 0 != MEM_32bits() { BIT_reloadDStream(&mut (*seqState).DStream); }
    let pos: size_t = (*seqState).pos.wrapping_add(seq.litLength);
    let matchBase: *const BYTE =
        if seq.offset > pos {
            (*seqState).dictEnd
        } else { (*seqState).prefixStart };
    seq.match_0 =
        matchBase.offset(pos as isize).offset(-(seq.offset as isize));
    (*seqState).pos = pos.wrapping_add(seq.matchLength);
    ZSTD_updateFseState(&mut (*seqState).stateLL, &mut (*seqState).DStream);
    ZSTD_updateFseState(&mut (*seqState).stateML, &mut (*seqState).DStream);
    if 0 != MEM_32bits() { BIT_reloadDStream(&mut (*seqState).DStream); }
    ZSTD_updateFseState(&mut (*seqState).stateOffb, &mut (*seqState).DStream);
    return seq;
}
/* ZSTD_FORCE_DECOMPRESS_SEQUENCES_LONG */
unsafe extern "C" fn ZSTD_decompressSequencesLong_bmi2(mut dctx:
                                                           *mut ZSTD_DCtx,
                                                       mut dst:
                                                           *mut libc::c_void,
                                                       mut maxDstSize: size_t,
                                                       mut seqStart:
                                                           *const libc::c_void,
                                                       mut seqSize: size_t,
                                                       mut nbSeq: libc::c_int,
                                                       isLongOffset:
                                                           ZSTD_longOffset_e)
 -> size_t {
    return ZSTD_decompressSequencesLong_body(dctx, dst, maxDstSize, seqStart,
                                             seqSize, nbSeq, isLongOffset);
}
/* ZSTD_FORCE_DECOMPRESS_SEQUENCES_SHORT */
/* ZSTD_getLongOffsetsShare() :
 * condition : offTable must be valid
 * @return : "share" of long offsets (arbitrarily defined as > (1<<23))
 *           compared to maximum possible of (1<<OffFSELog) */
unsafe extern "C" fn ZSTD_getLongOffsetsShare(mut offTable:
                                                  *const ZSTD_seqSymbol)
 -> libc::c_uint {
    let mut ptr: *const libc::c_void = offTable as *const libc::c_void;
    let tableLog: U32 =
        (*(ptr as *const ZSTD_seqSymbol_header).offset(0isize)).tableLog;
    let mut table: *const ZSTD_seqSymbol = offTable.offset(1isize);
    let max: U32 = (1i32 << tableLog) as U32;
    let mut u: U32 = 0;
    let mut total: U32 = 0i32 as U32;
    u = 0i32 as U32;
    while u < max {
        if (*table.offset(u as isize)).nbAdditionalBits as libc::c_int > 22i32
           {
            total =
                (total as libc::c_uint).wrapping_add(1i32 as libc::c_uint) as
                    U32 as U32
        }
        u = u.wrapping_add(1)
    }
    total <<= (8i32 as libc::c_uint).wrapping_sub(tableLog);
    return total;
}
/* ! ZSTD_decodeSeqHeaders() :
 *  decode sequence header from src */
/* Used by: decompress, fullbench (does not get its definition from here) */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decodeSeqHeaders(mut dctx: *mut ZSTD_DCtx,
                                               mut nbSeqPtr: *mut libc::c_int,
                                               mut src: *const libc::c_void,
                                               mut srcSize: size_t)
 -> size_t {
    let istart: *const BYTE = src as *const BYTE;
    let iend: *const BYTE = istart.offset(srcSize as isize);
    let mut ip: *const BYTE = istart;
    let mut nbSeq: libc::c_int = 0;
    if srcSize < 1i32 as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    let fresh8 = ip;
    ip = ip.offset(1);
    nbSeq = *fresh8 as libc::c_int;
    if 0 == nbSeq {
        *nbSeqPtr = 0i32;
        if srcSize != 1i32 as libc::c_ulong {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
        }
        return 1i32 as size_t
    }
    if nbSeq > 0x7fi32 {
        if nbSeq == 0xffi32 {
            if ip.offset(2isize) > iend {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
            }
            nbSeq =
                MEM_readLE16(ip as *const libc::c_void) as libc::c_int +
                    0x7f00i32;
            ip = ip.offset(2isize)
        } else {
            if ip >= iend {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
            }
            let fresh9 = ip;
            ip = ip.offset(1);
            nbSeq = (nbSeq - 0x80i32 << 8i32) + *fresh9 as libc::c_int
        }
    }
    *nbSeqPtr = nbSeq;
    if ip.offset(4isize) > iend {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    let LLtype: symbolEncodingType_e =
        (*ip as libc::c_int >> 6i32) as symbolEncodingType_e;
    let OFtype: symbolEncodingType_e =
        (*ip as libc::c_int >> 4i32 & 3i32) as symbolEncodingType_e;
    let MLtype: symbolEncodingType_e =
        (*ip as libc::c_int >> 2i32 & 3i32) as symbolEncodingType_e;
    ip = ip.offset(1isize);
    let llhSize: size_t =
        ZSTD_buildSeqTable((*dctx).entropy.LLTable.as_mut_ptr(),
                           &mut (*dctx).LLTptr, LLtype, 35i32 as libc::c_uint,
                           9i32 as U32, ip as *const libc::c_void,
                           iend.wrapping_offset_from(ip) as libc::c_long as
                               size_t, LL_base.as_ptr(), LL_bits.as_ptr(),
                           LL_defaultDTable.as_ptr(), (*dctx).fseEntropy,
                           (*dctx).ddictIsCold, nbSeq);
    if 0 != ERR_isError(llhSize) {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    }
    ip = ip.offset(llhSize as isize);
    let ofhSize: size_t =
        ZSTD_buildSeqTable((*dctx).entropy.OFTable.as_mut_ptr(),
                           &mut (*dctx).OFTptr, OFtype, 31i32 as libc::c_uint,
                           8i32 as U32, ip as *const libc::c_void,
                           iend.wrapping_offset_from(ip) as libc::c_long as
                               size_t, OF_base.as_ptr(), OF_bits.as_ptr(),
                           OF_defaultDTable.as_ptr(), (*dctx).fseEntropy,
                           (*dctx).ddictIsCold, nbSeq);
    if 0 != ERR_isError(ofhSize) {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    }
    ip = ip.offset(ofhSize as isize);
    let mlhSize: size_t =
        ZSTD_buildSeqTable((*dctx).entropy.MLTable.as_mut_ptr(),
                           &mut (*dctx).MLTptr, MLtype, 52i32 as libc::c_uint,
                           9i32 as U32, ip as *const libc::c_void,
                           iend.wrapping_offset_from(ip) as libc::c_long as
                               size_t, ML_base.as_ptr(), ML_bits.as_ptr(),
                           ML_defaultDTable.as_ptr(), (*dctx).fseEntropy,
                           (*dctx).ddictIsCold, nbSeq);
    if 0 != ERR_isError(mlhSize) {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    }
    ip = ip.offset(mlhSize as isize);
    return ip.wrapping_offset_from(istart) as libc::c_long as size_t;
}
/* header : fastMode, tableLog */
/* nextState, nbAddBits, nbBits, baseVal */
/* OF_defaultDTable */
/* Default FSE distribution table for Match Lengths */
static mut ML_defaultDTable: [ZSTD_seqSymbol; 65] =
    [ZSTD_seqSymbol{nextState: 1i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 1i32 as BYTE,
                    baseValue: 6i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 3i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 4i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 5i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 6i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 8i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 9i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 11i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 13i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 16i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 19i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 22i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 25i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 28i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 31i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 34i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 37i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 41i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 2i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 47i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 3i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 59i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 4i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 83i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 7i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 131i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 9i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 515i32 as U32,},
     ZSTD_seqSymbol{nextState: 16i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 4i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 5i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 6i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 7i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 9i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 10i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 12i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 15i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 18i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 21i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 24i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 27i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 30i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 33i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 35i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 39i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 2i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 43i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 3i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 51i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 4i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 67i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 5i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 99i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 8i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 259i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 4i32 as U32,},
     ZSTD_seqSymbol{nextState: 48i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 4i32 as U32,},
     ZSTD_seqSymbol{nextState: 16i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 5i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 7i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 8i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 10i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 11i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 14i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 17i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 20i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 23i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 26i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 29i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 32i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 16i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 65539i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 15i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 32771i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 14i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 16387i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 13i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 8195i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 12i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 4099i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 11i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 2051i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 10i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 1027i32 as U32,}];
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
/* ! ZSTD_buildSeqTable() :
 * @return : nb bytes read from src,
 *           or an error code if it fails */
unsafe extern "C" fn ZSTD_buildSeqTable(mut DTableSpace: *mut ZSTD_seqSymbol,
                                        mut DTablePtr:
                                            *mut *const ZSTD_seqSymbol,
                                        mut type_0: symbolEncodingType_e,
                                        mut max: libc::c_uint,
                                        mut maxLog: U32,
                                        mut src: *const libc::c_void,
                                        mut srcSize: size_t,
                                        mut baseValue: *const U32,
                                        mut nbAdditionalBits: *const U32,
                                        mut defaultTable:
                                            *const ZSTD_seqSymbol,
                                        mut flagRepeatTable: U32,
                                        mut ddictIsCold: libc::c_int,
                                        mut nbSeq: libc::c_int) -> size_t {
    match type_0 as libc::c_uint {
        1 => {
            if 0 == srcSize {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
            }
            if *(src as *const BYTE) as libc::c_uint > max {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            }
            let symbol: U32 = *(src as *const BYTE) as U32;
            let baseline: U32 = *baseValue.offset(symbol as isize);
            let nbBits: U32 = *nbAdditionalBits.offset(symbol as isize);
            ZSTD_buildSeqTable_rle(DTableSpace, baseline, nbBits);
            *DTablePtr = DTableSpace;
            return 1i32 as size_t
        }
        0 => { *DTablePtr = defaultTable; return 0i32 as size_t }
        3 => {
            if 0 == flagRepeatTable {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            }
            if 0 != ddictIsCold && nbSeq > 24i32 {
                let pStart: *const libc::c_void =
                    *DTablePtr as *const libc::c_void;
                let pSize: size_t =
                    (::std::mem::size_of::<ZSTD_seqSymbol>() as
                         libc::c_ulong).wrapping_mul((1i32 + (1i32 << maxLog))
                                                         as libc::c_ulong);
                let _ptr: *const libc::c_char = pStart as *const libc::c_char;
                let _size: size_t = pSize;
                let mut _pos: size_t = 0;
                _pos = 0i32 as size_t;
                while _pos < _size {
                    _pos =
                        (_pos as
                             libc::c_ulong).wrapping_add(64i32 as
                                                             libc::c_ulong) as
                            size_t as size_t
                }
            }
            return 0i32 as size_t
        }
        2 => {
            let mut tableLog: libc::c_uint = 0;
            let mut norm: [S16; 53] = [0; 53];
            let headerSize: size_t =
                FSE_readNCount(norm.as_mut_ptr(), &mut max, &mut tableLog,
                               src, srcSize);
            if 0 != ERR_isError(headerSize) {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            }
            if tableLog > maxLog {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            }
            ZSTD_buildFSETable(DTableSpace, norm.as_mut_ptr(), max, baseValue,
                               nbAdditionalBits, tableLog);
            *DTablePtr = DTableSpace;
            return headerSize
        }
        _ => { return -(ZSTD_error_GENERIC as libc::c_int) as size_t }
    };
}
/* ZSTD_buildFSETable() :
 * generate FSE decoding table for one symbol (ll, ml or off)
 * this function must be called with valid parameters only
 * (dt is large enough, normalizedCounter distribution total is a power of 2, max is within range, etc.)
 * in which case it cannot fail.
 * Internal use only.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_buildFSETable(mut dt: *mut ZSTD_seqSymbol,
                                            mut normalizedCounter:
                                                *const libc::c_short,
                                            mut maxSymbolValue: libc::c_uint,
                                            mut baseValue: *const U32,
                                            mut nbAdditionalBits: *const U32,
                                            mut tableLog: libc::c_uint) {
    let tableDecode: *mut ZSTD_seqSymbol = dt.offset(1isize);
    let mut symbolNext: [U16; 53] = [0; 53];
    let maxSV1: U32 = maxSymbolValue.wrapping_add(1i32 as libc::c_uint);
    let tableSize: U32 = (1i32 << tableLog) as U32;
    let mut highThreshold: U32 = tableSize.wrapping_sub(1i32 as libc::c_uint);
    let mut DTableH: ZSTD_seqSymbol_header =
        ZSTD_seqSymbol_header{fastMode: 0, tableLog: 0,};
    DTableH.tableLog = tableLog;
    DTableH.fastMode = 1i32 as U32;
    let largeLimit: S16 =
        (1i32 << tableLog.wrapping_sub(1i32 as libc::c_uint)) as S16;
    let mut s: U32 = 0;
    s = 0i32 as U32;
    while s < maxSV1 {
        if *normalizedCounter.offset(s as isize) as libc::c_int == -1i32 {
            let fresh10 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh10 as isize)).baseValue = s;
            symbolNext[s as usize] = 1i32 as U16
        } else {
            if *normalizedCounter.offset(s as isize) as libc::c_int >=
                   largeLimit as libc::c_int {
                DTableH.fastMode = 0i32 as U32
            }
            symbolNext[s as usize] =
                *normalizedCounter.offset(s as isize) as U16
        }
        s = s.wrapping_add(1)
    }
    memcpy(dt as *mut libc::c_void,
           &mut DTableH as *mut ZSTD_seqSymbol_header as *const libc::c_void,
           ::std::mem::size_of::<ZSTD_seqSymbol_header>() as libc::c_ulong);
    let tableMask: U32 = tableSize.wrapping_sub(1i32 as libc::c_uint);
    let step: U32 =
        (tableSize >>
             1i32).wrapping_add(tableSize >>
                                    3i32).wrapping_add(3i32 as libc::c_uint);
    let mut s_0: U32 = 0;
    let mut position: U32 = 0i32 as U32;
    s_0 = 0i32 as U32;
    while s_0 < maxSV1 {
        let mut i: libc::c_int = 0;
        i = 0i32;
        while i < *normalizedCounter.offset(s_0 as isize) as libc::c_int {
            (*tableDecode.offset(position as isize)).baseValue = s_0;
            position = position.wrapping_add(step) & tableMask;
            while position > highThreshold {
                position = position.wrapping_add(step) & tableMask
            }
            i += 1
        }
        s_0 = s_0.wrapping_add(1)
    }
    let mut u: U32 = 0;
    u = 0i32 as U32;
    while u < tableSize {
        let symbol: U32 = (*tableDecode.offset(u as isize)).baseValue;
        let fresh11 = symbolNext[symbol as usize];
        symbolNext[symbol as usize] =
            symbolNext[symbol as usize].wrapping_add(1);
        let nextState: U32 = fresh11 as U32;
        (*tableDecode.offset(u as isize)).nbBits =
            tableLog.wrapping_sub(BIT_highbit32(nextState)) as BYTE;
        (*tableDecode.offset(u as isize)).nextState =
            (nextState <<
                 (*tableDecode.offset(u as isize)).nbBits as
                     libc::c_int).wrapping_sub(tableSize) as U16;
        (*tableDecode.offset(u as isize)).nbAdditionalBits =
            *nbAdditionalBits.offset(symbol as isize) as BYTE;
        (*tableDecode.offset(u as isize)).baseValue =
            *baseValue.offset(symbol as isize);
        u = u.wrapping_add(1)
    };
}
/* header : fastMode, tableLog */
/* nextState, nbAddBits, nbBits, baseVal */
/* ML_defaultDTable */
unsafe extern "C" fn ZSTD_buildSeqTable_rle(mut dt: *mut ZSTD_seqSymbol,
                                            mut baseValue: U32,
                                            mut nbAddBits: U32) {
    let mut ptr: *mut libc::c_void = dt as *mut libc::c_void;
    let DTableH: *mut ZSTD_seqSymbol_header =
        ptr as *mut ZSTD_seqSymbol_header;
    let cell: *mut ZSTD_seqSymbol = dt.offset(1isize);
    (*DTableH).tableLog = 0i32 as U32;
    (*DTableH).fastMode = 0i32 as U32;
    (*cell).nbBits = 0i32 as BYTE;
    (*cell).nextState = 0i32 as U16;
    (*cell).nbAdditionalBits = nbAddBits as BYTE;
    (*cell).baseValue = baseValue;
}
/* header : fastMode, tableLog */
/* nextState, nbAddBits, nbBits, baseVal */
/* LL_defaultDTable */
/* Default FSE distribution table for Offset Codes */
static mut OF_defaultDTable: [ZSTD_seqSymbol; 33] =
    [ZSTD_seqSymbol{nextState: 1i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 1i32 as BYTE,
                    baseValue: 5i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 0i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 6i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 61i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 9i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 509i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 15i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 32765i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 21i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 2097149i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 3i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 5i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 7i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 125i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 12i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 4093i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 18i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 262141i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 23i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 8388605i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 5i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 29i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 8i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 253i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 14i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 16381i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 20i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 1048573i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 2i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 1i32 as U32,},
     ZSTD_seqSymbol{nextState: 16i32 as U16,
                    nbAdditionalBits: 7i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 125i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 11i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 2045i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 17i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 131069i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 22i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 4194301i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 4i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 13i32 as U32,},
     ZSTD_seqSymbol{nextState: 16i32 as U16,
                    nbAdditionalBits: 8i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 253i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 13i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 8189i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 19i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 524285i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 1i32 as U32,},
     ZSTD_seqSymbol{nextState: 16i32 as U16,
                    nbAdditionalBits: 6i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 61i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 10i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 1021i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 16i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 65533i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 28i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 268435453i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 27i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 134217725i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 26i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 67108861i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 25i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 33554429i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 24i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 16777213i32 as U32,}];
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
/* Default FSE distribution tables.
 * These are pre-calculated FSE decoding tables using default distributions as defined in specification :
 * https://github.com/facebook/zstd/blob/master/doc/zstd_compression_format.md#default-distributions
 * They were generated programmatically with following method :
 * - start from default distributions, present in /lib/common/zstd_internal.h
 * - generate tables normally, using ZSTD_buildFSETable()
 * - printout the content of tables
 * - pretify output, report below, test with fuzzer to ensure it's correct */
/* Default FSE distribution table for Literal Lengths */
static mut LL_defaultDTable: [ZSTD_seqSymbol; 65] =
    [ZSTD_seqSymbol{nextState: 1i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 1i32 as BYTE,
                    baseValue: 6i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 0i32 as U32,},
     ZSTD_seqSymbol{nextState: 16i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 0i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 1i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 3i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 4i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 6i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 7i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 9i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 10i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 12i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 14i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 16i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 20i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 22i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 2i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 28i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 3i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 32i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 4i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 48i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 6i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 64i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 7i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 128i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 8i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 256i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 10i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 1024i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 12i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 4096i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 0i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 1i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 2i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 4i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 5i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 7i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 8i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 10i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 11i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 13i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 16i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 18i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 22i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 2i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 24i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 3i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 32i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 3i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 40i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 6i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 64i32 as U32,},
     ZSTD_seqSymbol{nextState: 16i32 as U16,
                    nbAdditionalBits: 6i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 64i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 7i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 128i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 9i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 512i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 11i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 2048i32 as U32,},
     ZSTD_seqSymbol{nextState: 48i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 0i32 as U32,},
     ZSTD_seqSymbol{nextState: 16i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 4i32 as BYTE,
                    baseValue: 1i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 2i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 3i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 5i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 6i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 8i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 9i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 11i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 12i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 0i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 15i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 18i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 1i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 20i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 2i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 24i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 2i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 28i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 3i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 40i32 as U32,},
     ZSTD_seqSymbol{nextState: 32i32 as U16,
                    nbAdditionalBits: 4i32 as BYTE,
                    nbBits: 5i32 as BYTE,
                    baseValue: 48i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 16i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 65536i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 15i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 32768i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 14i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 16384i32 as U32,},
     ZSTD_seqSymbol{nextState: 0i32 as U16,
                    nbAdditionalBits: 13i32 as BYTE,
                    nbBits: 6i32 as BYTE,
                    baseValue: 8192i32 as U32,}];
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
/* Hidden declaration for fullbench */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decodeLiteralsBlock(mut dctx: *mut ZSTD_DCtx,
                                                  mut src:
                                                      *const libc::c_void,
                                                  mut srcSize: size_t)
 -> size_t {
    if srcSize < (1i32 + 1i32 + 1i32) as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    }
    let istart: *const BYTE = src as *const BYTE;
    let litEncType: symbolEncodingType_e =
        (*istart.offset(0isize) as libc::c_int & 3i32) as
            symbolEncodingType_e;
    match litEncType as libc::c_uint {
        3 => {
            if (*dctx).litEntropy == 0i32 as libc::c_uint {
                return -(ZSTD_error_dictionary_corrupted as libc::c_int) as
                           size_t
            }
        }
        2 => { }
        0 => {
            /* fall-through */
            let mut litSize_0: size_t = 0;
            let mut lhSize_0: size_t = 0;
            let lhlCode_0: U32 =
                (*istart.offset(0isize) as libc::c_int >> 2i32 & 3i32) as U32;
            let mut current_block_57: u64;
            match lhlCode_0 {
                0 | 2 => {
                    /* note : default is impossible, since lhlCode into [0..3] */
                    current_block_57 = 7550579355280491942;
                }
                1 => {
                    lhSize_0 = 2i32 as size_t;
                    litSize_0 =
                        (MEM_readLE16(istart as *const libc::c_void) as
                             libc::c_int >> 4i32) as size_t;
                    current_block_57 = 2631791190359682872;
                }
                3 => {
                    lhSize_0 = 3i32 as size_t;
                    litSize_0 =
                        (MEM_readLE24(istart as *const libc::c_void) >> 4i32)
                            as size_t;
                    current_block_57 = 2631791190359682872;
                }
                _ => { current_block_57 = 7550579355280491942; }
            }
            match current_block_57 {
                7550579355280491942 => {
                    lhSize_0 = 1i32 as size_t;
                    litSize_0 =
                        (*istart.offset(0isize) as libc::c_int >> 3i32) as
                            size_t
                }
                _ => { }
            }
            if lhSize_0.wrapping_add(litSize_0).wrapping_add(8i32 as
                                                                 libc::c_ulong)
                   > srcSize {
                if litSize_0.wrapping_add(lhSize_0) > srcSize {
                    return -(ZSTD_error_corruption_detected as libc::c_int) as
                               size_t
                }
                memcpy((*dctx).litBuffer.as_mut_ptr() as *mut libc::c_void,
                       istart.offset(lhSize_0 as isize) as
                           *const libc::c_void, litSize_0);
                (*dctx).litPtr = (*dctx).litBuffer.as_mut_ptr();
                (*dctx).litSize = litSize_0;
                memset((*dctx).litBuffer.as_mut_ptr().offset((*dctx).litSize
                                                                 as isize) as
                           *mut libc::c_void, 0i32, 8i32 as libc::c_ulong);
                return lhSize_0.wrapping_add(litSize_0)
            }
            (*dctx).litPtr = istart.offset(lhSize_0 as isize);
            (*dctx).litSize = litSize_0;
            return lhSize_0.wrapping_add(litSize_0)
        }
        1 => {
            let lhlCode_1: U32 =
                (*istart.offset(0isize) as libc::c_int >> 2i32 & 3i32) as U32;
            let mut litSize_1: size_t = 0;
            let mut lhSize_1: size_t = 0;
            let mut current_block_79: u64;
            match lhlCode_1 {
                0 | 2 => {
                    /* note : default is impossible, since lhlCode into [0..3] */
                    current_block_79 = 12152551618251980297;
                }
                1 => {
                    lhSize_1 = 2i32 as size_t;
                    litSize_1 =
                        (MEM_readLE16(istart as *const libc::c_void) as
                             libc::c_int >> 4i32) as size_t;
                    current_block_79 = 9505035279996566320;
                }
                3 => {
                    lhSize_1 = 3i32 as size_t;
                    litSize_1 =
                        (MEM_readLE24(istart as *const libc::c_void) >> 4i32)
                            as size_t;
                    if srcSize < 4i32 as libc::c_ulong {
                        return -(ZSTD_error_corruption_detected as
                                     libc::c_int) as size_t
                    }
                    current_block_79 = 9505035279996566320;
                }
                _ => { current_block_79 = 12152551618251980297; }
            }
            match current_block_79 {
                12152551618251980297 => {
                    lhSize_1 = 1i32 as size_t;
                    litSize_1 =
                        (*istart.offset(0isize) as libc::c_int >> 3i32) as
                            size_t
                }
                _ => { }
            }
            if litSize_1 > (1i32 << 17i32) as libc::c_ulong {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            }
            memset((*dctx).litBuffer.as_mut_ptr() as *mut libc::c_void,
                   *istart.offset(lhSize_1 as isize) as libc::c_int,
                   litSize_1.wrapping_add(8i32 as libc::c_ulong));
            (*dctx).litPtr = (*dctx).litBuffer.as_mut_ptr();
            (*dctx).litSize = litSize_1;
            return lhSize_1.wrapping_add(1i32 as libc::c_ulong)
        }
        _ => {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        }
    }
    if srcSize < 5i32 as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    }
    let mut lhSize: size_t = 0;
    let mut litSize: size_t = 0;
    let mut litCSize: size_t = 0;
    let mut singleStream: U32 = 0i32 as U32;
    let lhlCode: U32 =
        (*istart.offset(0isize) as libc::c_int >> 2i32 & 3i32) as U32;
    let lhc: U32 = MEM_readLE32(istart as *const libc::c_void);
    let mut hufSuccess: size_t = 0;
    let mut current_block_15: u64;
    match lhlCode {
        0 | 1 => {
            /* note : default is impossible, since lhlCode into [0..3] */
            current_block_15 = 16000071871159355311;
        }
        2 => {
            lhSize = 4i32 as size_t;
            litSize = (lhc >> 4i32 & 0x3fffi32 as libc::c_uint) as size_t;
            litCSize = (lhc >> 18i32) as size_t;
            current_block_15 = 11298138898191919651;
        }
        3 => {
            lhSize = 5i32 as size_t;
            litSize = (lhc >> 4i32 & 0x3ffffi32 as libc::c_uint) as size_t;
            litCSize =
                (lhc >>
                     22i32).wrapping_add(((*istart.offset(4isize) as
                                               libc::c_int) << 10i32) as
                                             libc::c_uint) as size_t;
            current_block_15 = 11298138898191919651;
        }
        _ => { current_block_15 = 16000071871159355311; }
    }
    match current_block_15 {
        16000071871159355311 => {
            singleStream = (0 == lhlCode) as libc::c_int as U32;
            lhSize = 3i32 as size_t;
            litSize = (lhc >> 4i32 & 0x3ffi32 as libc::c_uint) as size_t;
            litCSize = (lhc >> 14i32 & 0x3ffi32 as libc::c_uint) as size_t
        }
        _ => { }
    }
    if litSize > (1i32 << 17i32) as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    }
    if litCSize.wrapping_add(lhSize) > srcSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    }
    if 0 != (*dctx).ddictIsCold && litSize > 768i32 as libc::c_ulong {
        let _ptr: *const libc::c_char = (*dctx).HUFptr as *const libc::c_char;
        let _size: size_t =
            ::std::mem::size_of::<[HUF_DTable; 4097]>() as libc::c_ulong;
        let mut _pos: size_t = 0;
        _pos = 0i32 as size_t;
        while _pos < _size {
            _pos =
                (_pos as libc::c_ulong).wrapping_add(64i32 as libc::c_ulong)
                    as size_t as size_t
        }
    }
    if litEncType as libc::c_uint == set_repeat as libc::c_int as libc::c_uint
       {
        if 0 != singleStream {
            hufSuccess =
                HUF_decompress1X_usingDTable_bmi2((*dctx).litBuffer.as_mut_ptr()
                                                      as *mut libc::c_void,
                                                  litSize,
                                                  istart.offset(lhSize as
                                                                    isize) as
                                                      *const libc::c_void,
                                                  litCSize, (*dctx).HUFptr,
                                                  (*dctx).bmi2)
        } else {
            hufSuccess =
                HUF_decompress4X_usingDTable_bmi2((*dctx).litBuffer.as_mut_ptr()
                                                      as *mut libc::c_void,
                                                  litSize,
                                                  istart.offset(lhSize as
                                                                    isize) as
                                                      *const libc::c_void,
                                                  litCSize, (*dctx).HUFptr,
                                                  (*dctx).bmi2)
        }
    } else if 0 != singleStream {
        hufSuccess =
            HUF_decompress1X1_DCtx_wksp_bmi2((*dctx).entropy.hufTable.as_mut_ptr(),
                                             (*dctx).litBuffer.as_mut_ptr() as
                                                 *mut libc::c_void, litSize,
                                             istart.offset(lhSize as isize) as
                                                 *const libc::c_void,
                                             litCSize,
                                             (*dctx).workspace.as_mut_ptr() as
                                                 *mut libc::c_void,
                                             ::std::mem::size_of::<[U32; 512]>()
                                                 as libc::c_ulong,
                                             (*dctx).bmi2)
    } else {
        hufSuccess =
            HUF_decompress4X_hufOnly_wksp_bmi2((*dctx).entropy.hufTable.as_mut_ptr(),
                                               (*dctx).litBuffer.as_mut_ptr()
                                                   as *mut libc::c_void,
                                               litSize,
                                               istart.offset(lhSize as isize)
                                                   as *const libc::c_void,
                                               litCSize,
                                               (*dctx).workspace.as_mut_ptr()
                                                   as *mut libc::c_void,
                                               ::std::mem::size_of::<[U32; 512]>()
                                                   as libc::c_ulong,
                                               (*dctx).bmi2)
    }
    if 0 != ERR_isError(hufSuccess) {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    }
    (*dctx).litPtr = (*dctx).litBuffer.as_mut_ptr();
    (*dctx).litSize = litSize;
    (*dctx).litEntropy = 1i32 as U32;
    if litEncType as libc::c_uint ==
           set_compressed as libc::c_int as libc::c_uint {
        (*dctx).HUFptr = (*dctx).entropy.hufTable.as_mut_ptr()
    }
    memset((*dctx).litBuffer.as_mut_ptr().offset((*dctx).litSize as isize) as
               *mut libc::c_void, 0i32, 8i32 as libc::c_ulong);
    return litCSize.wrapping_add(lhSize);
}
/* magic number size */
/* C standard doesn't allow `static const` variable to be init using another `static const` variable */
static mut ZSTD_blockHeaderSize: size_t = 3i32 as size_t;
/* ! ZSTD_getcBlockSize() :
 *  Provides the size of compressed block from block header `src` */
/* Used by: decompress, fullbench (does not get its definition from here) */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getcBlockSize(mut src: *const libc::c_void,
                                            mut srcSize: size_t,
                                            mut bpPtr: *mut blockProperties_t)
 -> size_t {
    if srcSize < ZSTD_blockHeaderSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    let cBlockHeader: U32 = MEM_readLE24(src);
    let cSize: U32 = cBlockHeader >> 3i32;
    (*bpPtr).lastBlock = cBlockHeader & 1i32 as libc::c_uint;
    (*bpPtr).blockType =
        (cBlockHeader >> 1i32 & 3i32 as libc::c_uint) as blockType_e;
    (*bpPtr).origSize = cSize;
    if (*bpPtr).blockType as libc::c_uint ==
           bt_rle as libc::c_int as libc::c_uint {
        return 1i32 as size_t
    }
    if (*bpPtr).blockType as libc::c_uint ==
           bt_reserved as libc::c_int as libc::c_uint {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    }
    return cSize as size_t;
}