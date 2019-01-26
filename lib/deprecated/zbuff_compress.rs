#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc)]
extern crate libc;
extern "C" {
    /* **************************************
*  Explicit context
***************************************/
/*= Compression context
 *  When compressing many times,
 *  it is recommended to allocate a context just once, and re-use it for each successive compression operation.
 *  This will make workload friendlier for system's memory.
 *  Use one context per thread for parallel execution in multi-threaded environments. */
    pub type ZSTD_CCtx_s;
    /*===== ZSTD_CStream management functions =====*/
    #[no_mangle]
    fn ZSTD_createCStream() -> *mut ZSTD_CStream;
    #[no_mangle]
    fn ZSTD_freeCStream(zcs: *mut ZSTD_CStream) -> size_t;
    /*===== Streaming compression functions =====*/
    #[no_mangle]
    fn ZSTD_initCStream(zcs: *mut ZSTD_CStream, compressionLevel: libc::c_int)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressStream(zcs: *mut ZSTD_CStream,
                           output: *mut ZSTD_outBuffer,
                           input: *mut ZSTD_inBuffer) -> size_t;
    #[no_mangle]
    fn ZSTD_flushStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer)
     -> size_t;
    #[no_mangle]
    fn ZSTD_endStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer)
     -> size_t;
    /* *< recommended size for input buffer */
    #[no_mangle]
    fn ZSTD_CStreamInSize() -> size_t;
    /* *< recommended size for output buffer. Guarantee to successfully flush at least one complete compressed block in all circumstances. */
    #[no_mangle]
    fn ZSTD_CStreamOutSize() -> size_t;
    #[no_mangle]
    fn ZSTD_initCStream_usingDict(zcs: *mut ZSTD_CStream,
                                  dict: *const libc::c_void, dictSize: size_t,
                                  compressionLevel: libc::c_int) -> size_t;
    #[no_mangle]
    fn ZSTD_createCStream_advanced(customMem: ZSTD_customMem)
     -> *mut ZSTD_CStream;
    #[no_mangle]
    fn ZSTD_initCStream_advanced(zcs: *mut ZSTD_CStream,
                                 dict: *const libc::c_void, dictSize: size_t,
                                 params: ZSTD_parameters,
                                 pledgedSrcSize: libc::c_ulonglong) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type ZSTD_CCtx = ZSTD_CCtx_s;
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
/*-***********************************************************************
*  Streaming compression - HowTo
*
*  A ZSTD_CStream object is required to track streaming operation.
*  Use ZSTD_createCStream() and ZSTD_freeCStream() to create/release resources.
*  ZSTD_CStream objects can be reused multiple times on consecutive compression operations.
*  It is recommended to re-use ZSTD_CStream since it will play nicer with system's memory, by re-using already allocated memory.
*
*  For parallel execution, use one separate ZSTD_CStream per thread.
*
*  note : since v1.3.0, ZSTD_CStream and ZSTD_CCtx are the same thing.
*
*  Parameters are sticky : when starting a new compression on the same context,
*  it will re-use the same sticky parameters as previous compression session.
*  When in doubt, it's recommended to fully initialize the context before usage.
*  Use ZSTD_initCStream() to set the parameter to a selected compression level.
*  Use advanced API (ZSTD_CCtx_setParameter(), etc.) to set more specific parameters.
*
*  Use ZSTD_compressStream() as many times as necessary to consume input stream.
*  The function will automatically update both `pos` fields within `input` and `output`.
*  Note that the function may not consume the entire input,
*  for example, because the output buffer is already full,
*  in which case `input.pos < input.size`.
*  The caller must check if input has been entirely consumed.
*  If not, the caller must make some room to receive more compressed data,
*  and then present again remaining input data.
* @return : a size hint, preferred nb of bytes to use as input for next function call
*           or an error code, which can be tested using ZSTD_isError().
*           Note 1 : it's just a hint, to help latency a little, any value will work fine.
*           Note 2 : size hint is guaranteed to be <= ZSTD_CStreamInSize()
*
*  At any moment, it's possible to flush whatever data might remain stuck within internal buffer,
*  using ZSTD_flushStream(). `output->pos` will be updated.
*  Note that, if `output->size` is too small, a single invocation of ZSTD_flushStream() might not be enough (return code > 0).
*  In which case, make some room to receive more compressed data, and call again ZSTD_flushStream().
*  @return : 0 if internal buffers are entirely flushed,
*            >0 if some data still present within internal buffer (the value is minimal estimation of remaining size),
*            or an error code, which can be tested using ZSTD_isError().
*
*  ZSTD_endStream() instructs to finish a frame.
*  It will perform a flush and write frame epilogue.
*  The epilogue is required for decoders to consider a frame completed.
*  flush() operation is the same, and follows same rules as ZSTD_flushStream().
*  @return : 0 if frame fully completed and fully flushed,
*            >0 if some data still present within internal buffer (the value is minimal estimation of remaining size),
*            or an error code, which can be tested using ZSTD_isError().
*
* *******************************************************************/
/* *< CCtx and CStream are now effectively same object (>= v1.3.0) */
                                 /* Continue to distinguish them for compatibility with older versions <= v1.2.0 */
pub type ZSTD_CStream = ZSTD_CCtx;
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* ***************************************************************
*  NOTES/WARNINGS
******************************************************************/
/* The streaming API defined here is deprecated.
 * Consider migrating towards ZSTD_compressStream() API in `zstd.h`
 * See 'lib/README.md'.
 *****************************************************************/
/* *************************************
*  Dependencies
***************************************/
/* size_t */
/* ***************************************************************
*  Compiler specifics
*****************************************************************/
/* Deprecation warnings */
/* Should these warnings be a problem,
   it is generally possible to disable them,
   typically with -Wno-deprecated-declarations for gcc
   or _CRT_SECURE_NO_WARNINGS in Visual.
   Otherwise, it's also possible to define ZBUFF_DISABLE_DEPRECATE_WARNINGS */
/* C++14 or greater */
/* ZBUFF_DISABLE_DEPRECATE_WARNINGS */
/* *************************************
*  Streaming functions
***************************************/
/* This is the easier "buffered" streaming API,
*  using an internal buffer to lift all restrictions on user-provided buffers
*  which can be any size, any place, for both input and output.
*  ZBUFF and ZSTD are 100% interoperable,
*  frames created by one can be decoded by the other one */
pub type ZBUFF_CCtx = ZSTD_CStream;
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
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_createCCtx() -> *mut ZBUFF_CCtx {
    return ZSTD_createCStream();
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_freeCCtx(mut zbc: *mut ZBUFF_CCtx) -> size_t {
    return ZSTD_freeCStream(zbc);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_compressInit(mut zbc: *mut ZBUFF_CCtx,
                                            mut compressionLevel: libc::c_int)
 -> size_t {
    return ZSTD_initCStream(zbc, compressionLevel);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_compressInitDictionary(mut zbc:
                                                          *mut ZBUFF_CCtx,
                                                      mut dict:
                                                          *const libc::c_void,
                                                      mut dictSize: size_t,
                                                      mut compressionLevel:
                                                          libc::c_int)
 -> size_t {
    return ZSTD_initCStream_usingDict(zbc, dict, dictSize, compressionLevel);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_compressContinue(mut zbc: *mut ZBUFF_CCtx,
                                                mut dst: *mut libc::c_void,
                                                mut dstCapacityPtr:
                                                    *mut size_t,
                                                mut src: *const libc::c_void,
                                                mut srcSizePtr: *mut size_t)
 -> size_t {
    let mut result: size_t = 0;
    let mut outBuff: ZSTD_outBuffer =
        ZSTD_outBuffer_s{dst: 0 as *mut libc::c_void, size: 0, pos: 0,};
    let mut inBuff: ZSTD_inBuffer =
        ZSTD_inBuffer_s{src: 0 as *const libc::c_void, size: 0, pos: 0,};
    outBuff.dst = dst;
    outBuff.pos = 0i32 as size_t;
    outBuff.size = *dstCapacityPtr;
    inBuff.src = src;
    inBuff.pos = 0i32 as size_t;
    inBuff.size = *srcSizePtr;
    result = ZSTD_compressStream(zbc, &mut outBuff, &mut inBuff);
    *dstCapacityPtr = outBuff.pos;
    *srcSizePtr = inBuff.pos;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_compressFlush(mut zbc: *mut ZBUFF_CCtx,
                                             mut dst: *mut libc::c_void,
                                             mut dstCapacityPtr: *mut size_t)
 -> size_t {
    let mut result: size_t = 0;
    let mut outBuff: ZSTD_outBuffer =
        ZSTD_outBuffer_s{dst: 0 as *mut libc::c_void, size: 0, pos: 0,};
    outBuff.dst = dst;
    outBuff.pos = 0i32 as size_t;
    outBuff.size = *dstCapacityPtr;
    result = ZSTD_flushStream(zbc, &mut outBuff);
    *dstCapacityPtr = outBuff.pos;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_compressEnd(mut zbc: *mut ZBUFF_CCtx,
                                           mut dst: *mut libc::c_void,
                                           mut dstCapacityPtr: *mut size_t)
 -> size_t {
    let mut result: size_t = 0;
    let mut outBuff: ZSTD_outBuffer =
        ZSTD_outBuffer_s{dst: 0 as *mut libc::c_void, size: 0, pos: 0,};
    outBuff.dst = dst;
    outBuff.pos = 0i32 as size_t;
    outBuff.size = *dstCapacityPtr;
    result = ZSTD_endStream(zbc, &mut outBuff);
    *dstCapacityPtr = outBuff.pos;
    return result;
}
/* * Functions below provide recommended buffer sizes for Compression or Decompression operations.
*   These sizes are just hints, they tend to offer better latency */
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_recommendedCInSize() -> size_t {
    return ZSTD_CStreamInSize();
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_recommendedCOutSize() -> size_t {
    return ZSTD_CStreamOutSize();
}
/* ZSTD_BUFFERED_H_23987 */
/* ====================================================================================
 * The definitions in this section are considered experimental.
 * They should never be used in association with a dynamic library, as they may change in the future.
 * They are provided for advanced usages.
 * Use them only in association with static linking.
 * ==================================================================================== */
/*--- Dependency ---*/
/* ZSTD_parameters, ZSTD_customMem */
/*--- Custom memory allocator ---*/
/*! ZBUFF_createCCtx_advanced() :
 *  Create a ZBUFF compression context using external alloc and free functions */
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_createCCtx_advanced(mut customMem:
                                                       ZSTD_customMem)
 -> *mut ZBUFF_CCtx {
    return ZSTD_createCStream_advanced(customMem);
}
/*--- Advanced Streaming Initialization ---*/
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_compressInit_advanced(mut zbc: *mut ZBUFF_CCtx,
                                                     mut dict:
                                                         *const libc::c_void,
                                                     mut dictSize: size_t,
                                                     mut params:
                                                         ZSTD_parameters,
                                                     mut pledgedSrcSize:
                                                         libc::c_ulonglong)
 -> size_t {
    if pledgedSrcSize == 0i32 as libc::c_ulonglong {
        pledgedSrcSize = 0u64.wrapping_sub(1i32 as libc::c_ulonglong)
    }
    return ZSTD_initCStream_advanced(zbc, dict, dictSize, params,
                                     pledgedSrcSize);
}