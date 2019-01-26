#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc)]
extern crate libc;
extern "C" {
    /* ====  Benchmark any function, returning intermediate results  ==== */
    /* state information tracking benchmark session */
    pub type BMK_timedFnState_s;
    /* **************************************
*  Explicit context
***************************************/
/*= Compression context
 *  When compressing many times,
 *  it is recommended to allocate a context just once, and re-use it for each successive compression operation.
 *  This will make workload friendlier for system's memory.
 *  Use one context per thread for parallel execution in multi-threaded environments. */
    pub type ZSTD_CCtx_s;
    /*= Decompression context
 *  When decompressing many times,
 *  it is recommended to allocate a context only once,
 *  and re-use it for each successive compression operation.
 *  This will make workload friendlier for system's memory.
 *  Use one context per thread for parallel execution. */
    pub type ZSTD_DCtx_s;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn setpriority(__which: __priority_which_t, __who: id_t,
                   __prio: libc::c_int) -> libc::c_int;
    /* relies on standard C (note : clock_t measurements can be wrong when using multi-threading) */
    #[no_mangle]
    fn UTIL_getTime() -> UTIL_time_t;
    /* returns time span in microseconds */
    #[no_mangle]
    fn UTIL_clockSpanMicro(clockStart: UTIL_time_t) -> U64;
    #[no_mangle]
    fn UTIL_isDirectory(infilename: *const libc::c_char) -> U32;
    #[no_mangle]
    fn UTIL_getFileSize(infilename: *const libc::c_char) -> U64;
    #[no_mangle]
    fn UTIL_getTotalFileSize(fileNamesTable: *const *const libc::c_char,
                             nbFiles: libc::c_uint) -> U64;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    /* check first if the benchmark was successful or not */
    #[no_mangle]
    fn BMK_isSuccessful_runOutcome(outcome: BMK_runOutcome_t) -> libc::c_int;
    /* If the benchmark was successful, extract the result.
 * note : this function will abort() program execution if benchmark failed !
 *        always check if benchmark was successful first !
 */
    #[no_mangle]
    fn BMK_extract_runTime(outcome: BMK_runOutcome_t) -> BMK_runTime_t;
    /* BMK_benchTimedFn() :
 * Similar to BMK_benchFunction(), most arguments being identical.
 * Automatically determines `nbLoops` so that each result is regularly produced at interval of about run_ms.
 * Note : minimum `nbLoops` is 1, therefore a run may last more than run_ms, and possibly even more than total_ms.
 * Usage - initialize timedFnState, select benchmark duration (total_ms) and each measurement duration (run_ms)
 *         call BMK_benchTimedFn() repetitively, each measurement is supposed to last about run_ms
 *         Check if total time budget is spent or exceeded, using BMK_isCompleted_TimedFn()
 */
    #[no_mangle]
    fn BMK_benchTimedFn(timedFnState: *mut BMK_timedFnState_t,
                        params: BMK_benchParams_t) -> BMK_runOutcome_t;
    /* Tells if duration of all benchmark runs has exceeded total_ms
 */
    #[no_mangle]
    fn BMK_isCompleted_TimedFn(timedFnState: *const BMK_timedFnState_t)
     -> libc::c_int;
    /* BMK_createTimedFnState() and BMK_resetTimedFnState() :
 * Create/Set BMK_timedFnState_t for next benchmark session,
 * which shall last a minimum of total_ms milliseconds,
 * producing intermediate results, paced at interval of (approximately) run_ms.
 */
    #[no_mangle]
    fn BMK_createTimedFnState(total_ms: libc::c_uint, run_ms: libc::c_uint)
     -> *mut BMK_timedFnState_t;
    #[no_mangle]
    fn BMK_freeTimedFnState(state: *mut BMK_timedFnState_t);
    /*======  Helper functions  ======*/
    /* margin, from 64 to 0 */
    /* this formula ensures that bound(A) + bound(B) <= bound(A+B) as long as A and B >= 128 KB */
    #[no_mangle]
    fn ZSTD_compressBound(srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_isError(code: size_t) -> libc::c_uint;
    #[no_mangle]
    fn ZSTD_getErrorName(code: size_t) -> *const libc::c_char;
    /* !< maximum compression level available */
    #[no_mangle]
    fn ZSTD_maxCLevel() -> libc::c_int;
    #[no_mangle]
    fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
    #[no_mangle]
    fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> size_t;
    #[no_mangle]
    fn ZSTD_createDCtx() -> *mut ZSTD_DCtx;
    #[no_mangle]
    fn ZSTD_freeDCtx(dctx: *mut ZSTD_DCtx) -> size_t;
    #[no_mangle]
    fn ZSTD_decompressStream(zds: *mut ZSTD_DStream,
                             output: *mut ZSTD_outBuffer,
                             input: *mut ZSTD_inBuffer) -> size_t;
    /* ===   Memory management   === */
    /* ! ZSTD_sizeof_*() :
 *  These functions give the _current_ memory usage of selected object.
 *  Note that object memory usage can evolve (increase or decrease) over time. */
    #[no_mangle]
    fn ZSTD_sizeof_CCtx(cctx: *const ZSTD_CCtx) -> size_t;
    /* ! ZSTD_CCtx_setParameter() :
 *  Set one compression parameter, selected by enum ZSTD_cParameter.
 *  All parameters have valid bounds. Bounds can be queried using ZSTD_cParam_getBounds().
 *  Providing a value beyond bound will either clamp it, or trigger an error (depending on parameter).
 *  Setting a parameter is generally only possible during frame initialization (before starting compression).
 *  Exception : when using multi-threading mode (nbWorkers >= 1),
 *              the following parameters can be updated _during_ compression (within same frame):
 *              => compressionLevel, hashLog, chainLog, searchLog, minMatch, targetLength and strategy.
 *              new parameters will be active for next job only (after a flush()).
 * @return : an error code (which can be tested using ZSTD_isError()).
 */
    #[no_mangle]
    fn ZSTD_CCtx_setParameter(cctx: *mut ZSTD_CCtx, param: ZSTD_cParameter,
                              value: libc::c_int) -> size_t;
    /* ! ZSTD_CCtx_loadDictionary() :
 *  Create an internal CDict from `dict` buffer.
 *  Decompression will have to use same dictionary.
 * @result : 0, or an error code (which can be tested with ZSTD_isError()).
 *  Special: Loading a NULL (or 0-size) dictionary invalidates previous dictionary,
 *           meaning "return to no-dictionary mode".
 *  Note 1 : Dictionary is sticky, it will be used for all future compressed frames.
 *           To return to "no-dictionary" situation, load a NULL dictionary (or reset parameters).
 *  Note 2 : Loading a dictionary involves building tables.
 *           It's also a CPU consuming operation, with non-negligible impact on latency.
 *           Tables are dependent on compression parameters, and for this reason,
 *           compression parameters can no longer be changed after loading a dictionary.
 *  Note 3 :`dict` content will be copied internally.
 *           Use experimental ZSTD_CCtx_loadDictionary_byReference() to reference content instead.
 *           In such a case, dictionary buffer must outlive its users.
 *  Note 4 : Use ZSTD_CCtx_loadDictionary_advanced()
 *           to precisely select how dictionary content must be interpreted. */
    #[no_mangle]
    fn ZSTD_CCtx_loadDictionary(cctx: *mut ZSTD_CCtx,
                                dict: *const libc::c_void, dictSize: size_t)
     -> size_t;
    /* ! ZSTD_CCtx_reset() :
 *  There are 2 different things that can be reset, independently or jointly :
 *  - The session : will stop compressing current frame, and make CCtx ready to start a new one.
 *                  Useful after an error, or to interrupt any ongoing compression.
 *                  Any internal data not yet flushed is cancelled.
 *                  Compression parameters and dictionary remain unchanged.
 *                  They will be used to compress next frame.
 *                  Resetting session never fails.
 *  - The parameters : changes all parameters back to "default".
 *                  This removes any reference to any dictionary too.
 *                  Parameters can only be changed between 2 sessions (i.e. no compression is currently ongoing)
 *                  otherwise the reset fails, and function returns an error value (which can be tested using ZSTD_isError())
 *  - Both : similar to resetting the session, followed by resetting parameters.
 */
    #[no_mangle]
    fn ZSTD_CCtx_reset(cctx: *mut ZSTD_CCtx, reset: ZSTD_ResetDirective)
     -> size_t;
    /* ! ZSTD_compress2() :
 *  Behave the same as ZSTD_compressCCtx(), but compression parameters are set using the advanced API.
 *  ZSTD_compress2() always starts a new frame.
 *  Should cctx hold data from a previously unfinished frame, everything about it is forgotten.
 *  - Compression parameters are pushed into CCtx before starting compression, using ZSTD_CCtx_set*()
 *  - The function is always blocking, returns when compression is completed.
 *  Hint : compression runs faster if `dstCapacity` >=  `ZSTD_compressBound(srcSize)`.
 * @return : compressed size written into `dst` (<= `dstCapacity),
 *           or an error code if it fails (which can be tested using ZSTD_isError()).
 */
    #[no_mangle]
    fn ZSTD_compress2(cctx: *mut ZSTD_CCtx, dst: *mut libc::c_void,
                      dstCapacity: size_t, src: *const libc::c_void,
                      srcSize: size_t) -> size_t;
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
    fn ZSTD_DCtx_loadDictionary(dctx: *mut ZSTD_DCtx,
                                dict: *const libc::c_void, dictSize: size_t)
     -> size_t;
    /* ! ZSTD_DCtx_reset() :
 *  Return a DCtx to clean state.
 *  Session and parameters can be reset jointly or separately.
 *  Parameters can only be reset when no active frame is being decompressed.
 * @return : 0, or an error code, which can be tested with ZSTD_isError()
 */
    #[no_mangle]
    fn ZSTD_DCtx_reset(dctx: *mut ZSTD_DCtx, reset: ZSTD_ResetDirective)
     -> size_t;
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
    fn ZSTD_findDecompressedSize(src: *const libc::c_void, srcSize: size_t)
     -> libc::c_ulonglong;
    #[no_mangle]
    fn RDG_genBuffer(buffer: *mut libc::c_void, size: size_t,
                     matchProba: libc::c_double, litProba: libc::c_double,
                     seed: libc::c_uint);
    #[no_mangle]
    fn ZSTD_XXH64(input: *const libc::c_void, length: size_t,
                  seed: libc::c_ulonglong) -> XXH64_hash_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __id_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
/*-**************************************************************
*  Basic Types
*****************************************************************/
/* C99 */
pub type BYTE = uint8_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type __priority_which = libc::c_uint;
pub const PRIO_USER: __priority_which = 2;
pub const PRIO_PGRP: __priority_which = 1;
pub const PRIO_PROCESS: __priority_which = 0;
pub type id_t = __id_t;
pub type __priority_which_t = libc::c_int;
pub type UTIL_time_t = timespec;
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* benchfn :
 * benchmark any function on a set of input
 * providing result in nanoSecPerRun
 * or detecting and returning an error
 */
/* ===  Dependencies  === */
/* size_t */
/* ====  Benchmark any function, iterated on a set of blocks  ==== */
/* BMK_runTime_t: valid result return type */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BMK_runTime_t {
    pub nanoSecPerRun: libc::c_ulonglong,
    pub sumOfReturn: size_t,
}
/* BMK_runOutcome_t:
 * type expressing the outcome of a benchmark run by BMK_benchFunction(),
 * which can be either valid or invalid.
 * benchmark outcome can be invalid if errorFn is provided.
 * BMK_runOutcome_t must be considered "opaque" : never access its members directly.
 * Instead, use its assigned methods :
 * BMK_isSuccessful_runOutcome, BMK_extract_runTime, BMK_extract_errorResult.
 * The structure is only described here to allow its allocation on stack. */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BMK_runOutcome_t {
    pub internal_never_ever_use_directly: BMK_runTime_t,
    pub error_result_never_ever_use_directly: size_t,
    pub error_tag_never_ever_use_directly: libc::c_int,
}
/* prototypes for benchmarked functions */
pub type BMK_benchFn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: size_t,
                                _: *mut libc::c_void, _: size_t,
                                _: *mut libc::c_void) -> size_t>;
pub type BMK_initFn_t
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> size_t>;
pub type BMK_errorFn_t
    =
    Option<unsafe extern "C" fn(_: size_t) -> libc::c_uint>;
/* BMK_benchFunction() parameters are provided through following structure.
 * This is preferable for readability,
 * as the number of parameters required is pretty large.
 * No initializer is provided, because it doesn't make sense to provide some "default" :
 * all parameters should be specified by the caller */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BMK_benchParams_t {
    pub benchFn: BMK_benchFn_t,
    pub benchPayload: *mut libc::c_void,
    pub initFn: BMK_initFn_t,
    pub initPayload: *mut libc::c_void,
    pub errorFn: BMK_errorFn_t,
    pub blockCount: size_t,
    pub srcBuffers: *const *const libc::c_void,
    pub srcSizes: *const size_t,
    pub dstBuffers: *const *mut libc::c_void,
    pub dstCapacities: *const size_t,
    pub blockResults: *mut size_t,
}
pub type BMK_timedFnState_t = BMK_timedFnState_s;
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type ZSTD_DCtx = ZSTD_DCtx_s;
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
pub type ZSTD_cParameter = libc::c_uint;
pub const ZSTD_c_experimentalParam4: ZSTD_cParameter = 1001;
pub const ZSTD_c_experimentalParam3: ZSTD_cParameter = 1000;
pub const ZSTD_c_experimentalParam2: ZSTD_cParameter = 10;
/* note : additional experimental parameters are also available
     * within the experimental section of the API.
     * At the time of this writing, they include :
     * ZSTD_c_rsyncable
     * ZSTD_c_format
     * ZSTD_c_forceMaxWindow
     * ZSTD_c_forceAttachDict
     * Because they are not stable, it's necessary to define ZSTD_STATIC_LINKING_ONLY to access them.
     * note : never ever use experimentalParam? names directly;
     *        also, the enums values themselves are unstable and can still change.
     */
pub const ZSTD_c_experimentalParam1: ZSTD_cParameter = 500;
/* Control the overlap size, as a fraction of window size.
                              * The overlap size is an amount of data reloaded from previous job at the beginning of a new job.
                              * It helps preserve compression ratio, while each job is compressed in parallel.
                              * This value is enforced only when nbWorkers >= 1.
                              * Larger values increase compression ratio, but decrease speed.
                              * Possible values range from 0 to 9 :
                              * - 0 means "default" : value will be determined by the library, depending on strategy
                              * - 1 means "no overlap"
                              * - 9 means "full overlap", using a full window size.
                              * Each intermediate rank increases/decreases load size by a factor 2 :
                              * 9: full window;  8: w/2;  7: w/4;  6: w/8;  5:w/16;  4: w/32;  3:w/64;  2:w/128;  1:no overlap;  0:default
                              * default value varies between 6 and 9, depending on strategy */
pub const ZSTD_c_overlapLog: ZSTD_cParameter = 402;
/* Size of a compression job. This value is enforced only when nbWorkers >= 1.
                              * Each compression job is completed in parallel, so this value can indirectly impact the nb of active threads.
                              * 0 means default, which is dynamically determined based on compression parameters.
                              * Job size must be a minimum of overlap size, or 1 MB, whichever is largest.
                              * The minimum size is automatically and transparently enforced */
pub const ZSTD_c_jobSize: ZSTD_cParameter = 401;
/* multi-threading parameters */
    /* These parameters are only useful if multi-threading is enabled (compiled with build macro ZSTD_MULTITHREAD).
     * They return an error otherwise. */
/* Select how many threads will be spawned to compress in parallel.
                              * When nbWorkers >= 1, triggers asynchronous mode when used with ZSTD_compressStream*() :
                              * ZSTD_compressStream*() consumes input and flush output if possible, but immediately gives back control to caller,
                              * while compression work is performed in parallel, within worker threads.
                              * (note : a strong exception to this rule is when first invocation of ZSTD_compressStream2() sets ZSTD_e_end :
                              *  in which case, ZSTD_compressStream2() delegates to ZSTD_compress2(), which is always a blocking call).
                              * More workers improve speed, but also increase memory usage.
                              * Default value is `0`, aka "single-threaded mode" : no worker is spawned, compression is performed inside Caller's thread, all invocations are blocking */
pub const ZSTD_c_nbWorkers: ZSTD_cParameter = 400;
/* When applicable, dictionary's ID is written into frame header (default:1) */
pub const ZSTD_c_dictIDFlag: ZSTD_cParameter = 202;
/* A 32-bits checksum of content is written at end of frame (default:0) */
pub const ZSTD_c_checksumFlag: ZSTD_cParameter = 201;
/* frame parameters */
/* Content size will be written into frame header _whenever known_ (default:1)
                              * Content size must be known at the beginning of compression.
                              * This is automatically the case when using ZSTD_compress2(),
                              * For streaming variants, content size must be provided with ZSTD_CCtx_setPledgedSrcSize() */
pub const ZSTD_c_contentSizeFlag: ZSTD_cParameter = 200;
/* Frequency of inserting/looking up entries into the LDM hash table.
                              * Must be clamped between 0 and (ZSTD_WINDOWLOG_MAX - ZSTD_HASHLOG_MIN).
                              * Default is MAX(0, (windowLog - ldmHashLog)), optimizing hash table usage.
                              * Larger values improve compression speed.
                              * Deviating far from default value will likely result in a compression ratio decrease.
                              * Special: value 0 means "automatically determine hashRateLog". */
pub const ZSTD_c_ldmHashRateLog: ZSTD_cParameter = 164;
/* Log size of each bucket in the LDM hash table for collision resolution.
                              * Larger values improve collision resolution but decrease compression speed.
                              * The maximum value is ZSTD_LDM_BUCKETSIZELOG_MAX.
                              * Special: value 0 means "use default value" (default: 3). */
pub const ZSTD_c_ldmBucketSizeLog: ZSTD_cParameter = 163;
/* Minimum match size for long distance matcher.
                              * Larger/too small values usually decrease compression ratio.
                              * Must be clamped between ZSTD_LDM_MINMATCH_MIN and ZSTD_LDM_MINMATCH_MAX.
                              * Special: value 0 means "use default value" (default: 64). */
pub const ZSTD_c_ldmMinMatch: ZSTD_cParameter = 162;
/* Size of the table for long distance matching, as a power of 2.
                              * Larger values increase memory usage and compression ratio,
                              * but decrease compression speed.
                              * Must be clamped between ZSTD_HASHLOG_MIN and ZSTD_HASHLOG_MAX
                              * default: windowlog - 7.
                              * Special: value 0 means "automatically determine hashlog". */
pub const ZSTD_c_ldmHashLog: ZSTD_cParameter = 161;
/* LDM mode parameters */
/* Enable long distance matching.
                                     * This parameter is designed to improve compression ratio
                                     * for large inputs, by finding large matches at long distance.
                                     * It increases memory usage and window size.
                                     * Note: enabling this parameter increases default ZSTD_c_windowLog to 128 MB
                                     * except when expressly set to a different value. */
pub const ZSTD_c_enableLongDistanceMatching: ZSTD_cParameter = 160;
/* See ZSTD_strategy enum definition.
                              * The higher the value of selected strategy, the more complex it is,
                              * resulting in stronger and slower compression.
                              * Special: value 0 means "use default strategy". */
pub const ZSTD_c_strategy: ZSTD_cParameter = 107;
/* Impact of this field depends on strategy.
                              * For strategies btopt, btultra & btultra2:
                              *     Length of Match considered "good enough" to stop search.
                              *     Larger values make compression stronger, and slower.
                              * For strategy fast:
                              *     Distance between match sampling.
                              *     Larger values make compression faster, and weaker.
                              * Special: value 0 means "use default targetLength". */
pub const ZSTD_c_targetLength: ZSTD_cParameter = 106;
/* Minimum size of searched matches.
                              * Note that Zstandard can still find matches of smaller size,
                              * it just tweaks its search algorithm to look for this size and larger.
                              * Larger values increase compression and decompression speed, but decrease ratio.
                              * Must be clamped between ZSTD_MINMATCH_MIN and ZSTD_MINMATCH_MAX.
                              * Note that currently, for all strategies < btopt, effective minimum is 4.
                              *                    , for all strategies > fast, effective maximum is 6.
                              * Special: value 0 means "use default minMatchLength". */
pub const ZSTD_c_minMatch: ZSTD_cParameter = 105;
/* Number of search attempts, as a power of 2.
                              * More attempts result in better and slower compression.
                              * This parameter is useless when using "fast" and "dFast" strategies.
                              * Special: value 0 means "use default searchLog". */
pub const ZSTD_c_searchLog: ZSTD_cParameter = 104;
/* Size of the multi-probe search table, as a power of 2.
                              * Resulting memory usage is (1 << (chainLog+2)).
                              * Must be clamped between ZSTD_CHAINLOG_MIN and ZSTD_CHAINLOG_MAX.
                              * Larger tables result in better and slower compression.
                              * This parameter is useless when using "fast" strategy.
                              * It's still useful when using "dfast" strategy,
                              * in which case it defines a secondary probe table.
                              * Special: value 0 means "use default chainLog". */
pub const ZSTD_c_chainLog: ZSTD_cParameter = 103;
/* Size of the initial probe table, as a power of 2.
                              * Resulting memory usage is (1 << (hashLog+2)).
                              * Must be clamped between ZSTD_HASHLOG_MIN and ZSTD_HASHLOG_MAX.
                              * Larger tables improve compression ratio of strategies <= dFast,
                              * and improve speed of strategies > dFast.
                              * Special: value 0 means "use default hashLog". */
pub const ZSTD_c_hashLog: ZSTD_cParameter = 102;
/* Maximum allowed back-reference distance, expressed as power of 2.
                              * Must be clamped between ZSTD_WINDOWLOG_MIN and ZSTD_WINDOWLOG_MAX.
                              * Special: value 0 means "use default windowLog".
                              * Note: Using a windowLog greater than ZSTD_WINDOWLOG_LIMIT_DEFAULT
                              *       requires explicitly allowing such window size at decompression stage if using streaming. */
pub const ZSTD_c_windowLog: ZSTD_cParameter = 101;
/* compression parameters */
/* Update all compression parameters according to pre-defined cLevel table
                              * Default level is ZSTD_CLEVEL_DEFAULT==3.
                              * Special: value 0 means default, which is controlled by ZSTD_CLEVEL_DEFAULT.
                              * Note 1 : it's possible to pass a negative compression level.
                              * Note 2 : setting a level sets all default values of other compression parameters */
pub const ZSTD_c_compressionLevel: ZSTD_cParameter = 100;
pub type ZSTD_ResetDirective = libc::c_uint;
pub const ZSTD_reset_session_and_parameters: ZSTD_ResetDirective = 3;
pub const ZSTD_reset_parameters: ZSTD_ResetDirective = 2;
pub const ZSTD_reset_session_only: ZSTD_ResetDirective = 1;
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
pub type XXH64_hash_t = libc::c_ulonglong;
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* benchzstd :
  * benchmark Zstandard compression / decompression
  * over a set of files or buffers
  * and display progress result and final summary
  */
/* ===  Dependencies  === */
/* size_t */
/* ZSTD_compressionParameters */
/* ===  Constants  === */
/* ===  Benchmark functions  === */
/* Creates a variant `typeName`, able to express "error or valid result".
 * Functions with return type `typeName`
 * must first check if result is valid, using BMK_isSuccessful_*(),
 * and only then can extract `baseType`.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BMK_benchResult_t {
    pub cSize: size_t,
    pub cSpeed: libc::c_ulonglong,
    pub dSpeed: libc::c_ulonglong,
    pub cMem: size_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BMK_benchOutcome_t {
    pub internal_never_use_directly: BMK_benchResult_t,
    pub tag: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BMK_advancedParams_t {
    pub mode: BMK_mode_t,
    pub nbSeconds: libc::c_uint,
    pub blockSize: size_t,
    pub nbWorkers: libc::c_uint,
    pub realTime: libc::c_uint,
    pub additionalParam: libc::c_int,
    pub ldmFlag: libc::c_uint,
    pub ldmMinMatch: libc::c_uint,
    pub ldmHashLog: libc::c_uint,
    pub ldmBucketSizeLog: libc::c_uint,
    pub ldmHashRateLog: libc::c_uint,
}
pub type BMK_mode_t = libc::c_uint;
pub const BMK_compressOnly: BMK_mode_t = 2;
pub const BMK_decodeOnly: BMK_mode_t = 1;
pub const BMK_both: BMK_mode_t = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BMK_initDCtxArgs {
    pub dctx: *mut ZSTD_DCtx,
    pub dictBuffer: *const libc::c_void,
    pub dictBufferSize: size_t,
}
pub const ZSTD_error_dstSize_tooSmall: unnamed = 70;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BMK_initCCtxArgs {
    pub cctx: *mut ZSTD_CCtx,
    pub dictBuffer: *const libc::c_void,
    pub dictBufferSize: size_t,
    pub cLevel: libc::c_int,
    pub comprParams: *const ZSTD_compressionParameters,
    pub adv: *const BMK_advancedParams_t,
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
pub type unnamed = libc::c_uint;
/* never EVER use this value directly, it can change in future versions! Use ZSTD_isError() instead */
pub const ZSTD_error_maxCode: unnamed = 120;
pub const ZSTD_error_seekableIO: unnamed = 102;
/* following error codes are __NOT STABLE__, they can be removed or changed in future versions */
pub const ZSTD_error_frameIndex_tooLarge: unnamed = 100;
pub const ZSTD_error_dstBuffer_null: unnamed = 74;
pub const ZSTD_error_srcSize_wrong: unnamed = 72;
pub const ZSTD_error_workSpace_tooSmall: unnamed = 66;
pub const ZSTD_error_memory_allocation: unnamed = 64;
pub const ZSTD_error_init_missing: unnamed = 62;
pub const ZSTD_error_stage_wrong: unnamed = 60;
pub const ZSTD_error_maxSymbolValue_tooSmall: unnamed = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: unnamed = 46;
pub const ZSTD_error_tableLog_tooLarge: unnamed = 44;
pub const ZSTD_error_parameter_outOfBound: unnamed = 42;
pub const ZSTD_error_parameter_unsupported: unnamed = 40;
pub const ZSTD_error_dictionaryCreation_failed: unnamed = 34;
pub const ZSTD_error_dictionary_wrong: unnamed = 32;
pub const ZSTD_error_dictionary_corrupted: unnamed = 30;
pub const ZSTD_error_checksum_wrong: unnamed = 22;
pub const ZSTD_error_corruption_detected: unnamed = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: unnamed = 16;
pub const ZSTD_error_frameParameter_unsupported: unnamed = 14;
pub const ZSTD_error_version_unsupported: unnamed = 12;
pub const ZSTD_error_prefix_unknown: unnamed = 10;
pub const ZSTD_error_GENERIC: unnamed = 1;
pub const ZSTD_error_no_error: unnamed = 0;
/* check first if the return structure represents an error or a valid result */
#[no_mangle]
pub unsafe extern "C" fn BMK_isSuccessful_benchOutcome(mut outcome:
                                                           BMK_benchOutcome_t)
 -> libc::c_int {
    return (outcome.tag == 0i32) as libc::c_int;
}
/* extract result from variant type.
 * note : this function will abort() program execution if result is not valid
 *        check result validity first, by using BMK_isSuccessful_benchOutcome()
 */
#[no_mangle]
pub unsafe extern "C" fn BMK_extract_benchResult(mut outcome:
                                                     BMK_benchOutcome_t)
 -> BMK_benchResult_t {
    if outcome.tag == 0i32 {
    } else {
        __assert_fail(b"outcome.tag == 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/benchzstd.c\x00"
                          as *const u8 as *const libc::c_char,
                      265i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"BMK_benchResult_t BMK_extract_benchResult(BMK_benchOutcome_t)\x00")).as_ptr());
    };
    return outcome.internal_never_use_directly;
}
/* ! BMK_benchFiles() -- called by zstdcli */
/*  Loads files from fileNamesTable into memory,
 *  and an optional dictionary from dictFileName (can be NULL),
 *  then uses benchMem().
 *  fileNamesTable - name of files to benchmark.
 *  nbFiles - number of files (size of fileNamesTable), must be > 0.
 *  dictFileName - name of dictionary file to load.
 *  cLevel - compression level to benchmark, errors if invalid.
 *  compressionParams - advanced compression Parameters.
 *  displayLevel - what gets printed:
 *      0 : no display;
 *      1 : errors;
 *      2 : + result + interaction + warnings;
 *      3 : + information;
 *      4 : + debug
 * @return:
 *      a variant, which expresses either an error, or a valid result.
 *      Use BMK_isSuccessful_benchOutcome() to check if function was successful.
 *      If yes, extract the valid result with BMK_extract_benchResult(),
 *      it will contain :
 *          .cSpeed: compression speed in bytes per second,
 *          .dSpeed: decompression speed in bytes per second,
 *          .cSize : compressed size, in bytes
 *          .cMem  : memory budget required for the compression context
 */
#[no_mangle]
pub unsafe extern "C" fn BMK_benchFiles(mut fileNamesTable:
                                            *const *const libc::c_char,
                                        mut nbFiles: libc::c_uint,
                                        mut dictFileName: *const libc::c_char,
                                        mut cLevel: libc::c_int,
                                        mut compressionParams:
                                            *const ZSTD_compressionParameters,
                                        mut displayLevel: libc::c_int)
 -> BMK_benchOutcome_t {
    let adv: BMK_advancedParams_t = BMK_initAdvancedParams();
    return BMK_benchFilesAdvanced(fileNamesTable, nbFiles, dictFileName,
                                  cLevel, compressionParams, displayLevel,
                                  &adv);
}
/* returns default parameters used by nonAdvanced functions */
#[no_mangle]
pub unsafe extern "C" fn BMK_initAdvancedParams() -> BMK_advancedParams_t {
    let res: BMK_advancedParams_t =
        BMK_advancedParams_t{mode: BMK_both,
                             nbSeconds: 3i32 as libc::c_uint,
                             blockSize: 0i32 as size_t,
                             nbWorkers: 0i32 as libc::c_uint,
                             realTime: 0i32 as libc::c_uint,
                             additionalParam: 0i32,
                             ldmFlag: 0i32 as libc::c_uint,
                             ldmMinMatch: 0i32 as libc::c_uint,
                             ldmHashLog: 0i32 as libc::c_uint,
                             ldmBucketSizeLog: 0i32 as libc::c_uint,
                             ldmHashRateLog: 0i32 as libc::c_uint,};
    return res;
}
/* ! BMK_benchFilesAdvanced():
 *  Same as BMK_benchFiles(),
 *  with more controls, provided through advancedParams_t structure */
#[no_mangle]
pub unsafe extern "C" fn BMK_benchFilesAdvanced(mut fileNamesTable:
                                                    *const *const libc::c_char,
                                                mut nbFiles: libc::c_uint,
                                                mut dictFileName:
                                                    *const libc::c_char,
                                                mut cLevel: libc::c_int,
                                                mut compressionParams:
                                                    *const ZSTD_compressionParameters,
                                                mut displayLevel: libc::c_int,
                                                mut adv:
                                                    *const BMK_advancedParams_t)
 -> BMK_benchOutcome_t {
    let mut current_block: u64;
    let mut srcBuffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut benchedSize: size_t = 0;
    let mut dictBuffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut dictBufferSize: size_t = 0i32 as size_t;
    let mut fileSizes: *mut size_t = 0 as *mut size_t;
    let mut res: BMK_benchOutcome_t =
        BMK_benchOutcome_t{internal_never_use_directly:
                               BMK_benchResult_t{cSize: 0,
                                                 cSpeed: 0,
                                                 dSpeed: 0,
                                                 cMem: 0,},
                           tag: 0,};
    let totalSizeToLoad: U64 = UTIL_getTotalFileSize(fileNamesTable, nbFiles);
    if 0 == nbFiles {
        let mut r: BMK_benchOutcome_t =
            BMK_benchOutcome_t{internal_never_use_directly:
                                   BMK_benchResult_t{cSize: 0,
                                                     cSpeed: 0,
                                                     dSpeed: 0,
                                                     cMem: 0,},
                               tag: 0,};
        memset(&mut r as *mut BMK_benchOutcome_t as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong);
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    14i32);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"No Files to Benchmark\x00" as *const u8 as
                        *const libc::c_char);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        r.tag = 14i32;
        return r
    }
    if cLevel > ZSTD_maxCLevel() {
        let mut r_0: BMK_benchOutcome_t =
            BMK_benchOutcome_t{internal_never_use_directly:
                                   BMK_benchResult_t{cSize: 0,
                                                     cSpeed: 0,
                                                     dSpeed: 0,
                                                     cMem: 0,},
                               tag: 0,};
        memset(&mut r_0 as *mut BMK_benchOutcome_t as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong);
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    15i32);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Invalid Compression Level\x00" as *const u8 as
                        *const libc::c_char);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        r_0.tag = 15i32;
        return r_0
    }
    fileSizes =
        calloc(nbFiles as libc::c_ulong,
               ::std::mem::size_of::<size_t>() as libc::c_ulong) as
            *mut size_t;
    if fileSizes.is_null() {
        let mut r_1: BMK_benchOutcome_t =
            BMK_benchOutcome_t{internal_never_use_directly:
                                   BMK_benchResult_t{cSize: 0,
                                                     cSpeed: 0,
                                                     dSpeed: 0,
                                                     cMem: 0,},
                               tag: 0,};
        memset(&mut r_1 as *mut BMK_benchOutcome_t as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong);
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    12i32);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"not enough memory for fileSizes\x00" as *const u8 as
                        *const libc::c_char);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        r_1.tag = 12i32;
        return r_1
    }
    /* Load dictionary */
    if !dictFileName.is_null() {
        let dictFileSize: U64 = UTIL_getFileSize(dictFileName);
        if dictFileSize == -1i32 as U64 {
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error loading %s : %s \n\x00" as *const u8 as
                            *const libc::c_char, dictFileName,
                        strerror(*__errno_location()));
            }
            free(fileSizes as *mut libc::c_void);
            let mut r_2: BMK_benchOutcome_t =
                BMK_benchOutcome_t{internal_never_use_directly:
                                       BMK_benchResult_t{cSize: 0,
                                                         cSpeed: 0,
                                                         dSpeed: 0,
                                                         cMem: 0,},
                                   tag: 0,};
            memset(&mut r_2 as *mut BMK_benchOutcome_t as *mut libc::c_void,
                   0i32,
                   ::std::mem::size_of::<BMK_benchOutcome_t>() as
                       libc::c_ulong);
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 9i32);
            }
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b"benchmark aborted\x00" as *const u8 as
                            *const libc::c_char);
            }
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            r_2.tag = 9i32;
            return r_2
        }
        if dictFileSize > (64i32 * (1i32 << 20i32)) as libc::c_ulong {
            free(fileSizes as *mut libc::c_void);
            let mut r_3: BMK_benchOutcome_t =
                BMK_benchOutcome_t{internal_never_use_directly:
                                       BMK_benchResult_t{cSize: 0,
                                                         cSpeed: 0,
                                                         dSpeed: 0,
                                                         cMem: 0,},
                                   tag: 0,};
            memset(&mut r_3 as *mut BMK_benchOutcome_t as *mut libc::c_void,
                   0i32,
                   ::std::mem::size_of::<BMK_benchOutcome_t>() as
                       libc::c_ulong);
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 10i32);
            }
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b"dictionary file %s too large\x00" as *const u8 as
                            *const libc::c_char, dictFileName);
            }
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            r_3.tag = 10i32;
            return r_3
        }
        dictBufferSize = dictFileSize;
        dictBuffer = malloc(dictBufferSize);
        if dictBuffer.is_null() {
            free(fileSizes as *mut libc::c_void);
            let mut r_4: BMK_benchOutcome_t =
                BMK_benchOutcome_t{internal_never_use_directly:
                                       BMK_benchResult_t{cSize: 0,
                                                         cSpeed: 0,
                                                         dSpeed: 0,
                                                         cMem: 0,},
                                   tag: 0,};
            memset(&mut r_4 as *mut BMK_benchOutcome_t as *mut libc::c_void,
                   0i32,
                   ::std::mem::size_of::<BMK_benchOutcome_t>() as
                       libc::c_ulong);
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 11i32);
            }
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b"not enough memory for dictionary (%u bytes)\x00" as
                            *const u8 as *const libc::c_char,
                        dictBufferSize as libc::c_uint);
            }
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            r_4.tag = 11i32;
            return r_4
        }
        let errorCode: libc::c_int =
            BMK_loadFiles(dictBuffer, dictBufferSize, fileSizes,
                          &mut dictFileName, 1i32 as libc::c_uint,
                          displayLevel);
        /*?*/
        /*?*/
        if 0 != errorCode {
            res = BMK_benchOutcome_error();
            current_block = 7097867847565071335;
        } else { current_block = 10661265453436690952; }
    } else { current_block = 10661265453436690952; }
    match current_block {
        10661265453436690952 => {
            benchedSize =
                BMK_findMaxMem(totalSizeToLoad.wrapping_mul(3i32 as
                                                                libc::c_ulong)).wrapping_div(3i32
                                                                                                 as
                                                                                                 libc::c_ulong);
            if benchedSize > totalSizeToLoad { benchedSize = totalSizeToLoad }
            if benchedSize < totalSizeToLoad {
                fprintf(stderr,
                        b"Not enough memory; testing %u MB only...\n\x00" as
                            *const u8 as *const libc::c_char,
                        (benchedSize >> 20i32) as libc::c_uint);
            }
            srcBuffer =
                if 0 != benchedSize {
                    malloc(benchedSize)
                } else { 0 as *mut libc::c_void };
            if srcBuffer.is_null() {
                free(dictBuffer);
                free(fileSizes as *mut libc::c_void);
                let mut r_5: BMK_benchOutcome_t =
                    BMK_benchOutcome_t{internal_never_use_directly:
                                           BMK_benchResult_t{cSize: 0,
                                                             cSpeed: 0,
                                                             dSpeed: 0,
                                                             cMem: 0,},
                                       tag: 0,};
                memset(&mut r_5 as *mut BMK_benchOutcome_t as
                           *mut libc::c_void, 0i32,
                       ::std::mem::size_of::<BMK_benchOutcome_t>() as
                           libc::c_ulong);
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 12i32);
                }
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"not enough memory\x00" as *const u8 as
                                *const libc::c_char);
                }
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                r_5.tag = 12i32;
                return r_5
            }
            /* Load input buffer */
            let errorCode_0: libc::c_int =
                BMK_loadFiles(srcBuffer, benchedSize, fileSizes,
                              fileNamesTable, nbFiles, displayLevel);
            if 0 != errorCode_0 {
                res = BMK_benchOutcome_error()
            } else {
                let mut mfName: [libc::c_char; 20] =
                    [0i32 as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0, 0, 0];
                snprintf(mfName.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 20]>() as
                             libc::c_ulong,
                         b" %u files\x00" as *const u8 as *const libc::c_char,
                         nbFiles);
                let displayName: *const libc::c_char =
                    if nbFiles > 1i32 as libc::c_uint {
                        mfName.as_mut_ptr()
                    } else { *fileNamesTable.offset(0isize) };
                res =
                    BMK_benchCLevel(srcBuffer, benchedSize, fileSizes,
                                    nbFiles, cLevel, compressionParams,
                                    dictBuffer, dictBufferSize, displayLevel,
                                    displayName, adv)
            }
        }
        _ => { }
    }
    free(srcBuffer);
    free(dictBuffer);
    free(fileSizes as *mut libc::c_void);
    return res;
}
unsafe extern "C" fn BMK_benchCLevel(mut srcBuffer: *const libc::c_void,
                                     mut benchedSize: size_t,
                                     mut fileSizes: *const size_t,
                                     mut nbFiles: libc::c_uint,
                                     mut cLevel: libc::c_int,
                                     mut comprParams:
                                         *const ZSTD_compressionParameters,
                                     mut dictBuffer: *const libc::c_void,
                                     mut dictBufferSize: size_t,
                                     mut displayLevel: libc::c_int,
                                     mut displayName: *const libc::c_char,
                                     adv: *const BMK_advancedParams_t)
 -> BMK_benchOutcome_t {
    /* Windows */
    let mut pch: *const libc::c_char = strrchr(displayName, '\\' as i32);
    if pch.is_null() { pch = strrchr(displayName, '/' as i32) }
    if !pch.is_null() { displayName = pch.offset(1isize) }
    if 0 != (*adv).realTime {
        if displayLevel >= 2i32 {
            fprintf(stderr,
                    b"Note : switching to real-time priority \n\x00" as
                        *const u8 as *const libc::c_char);
        }
        setpriority(PRIO_PROCESS as libc::c_int, 0i32 as id_t, -20i32);
    }
    if displayLevel == 1i32 && 0 == (*adv).additionalParam {
        fprintf(stderr,
                b"bench %s %s: input %u bytes, %u seconds, %u KB blocks\n\x00"
                    as *const u8 as *const libc::c_char,
                b"1.3.8\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char,
                benchedSize as libc::c_uint, (*adv).nbSeconds,
                ((*adv).blockSize >> 10i32) as libc::c_uint);
    }
    return BMK_benchMemAdvanced(srcBuffer, benchedSize,
                                0 as *mut libc::c_void, 0i32 as size_t,
                                fileSizes, nbFiles, cLevel, comprParams,
                                dictBuffer, dictBufferSize, displayLevel,
                                displayName, adv);
}
/* BMK_benchMemAdvanced() : same as BMK_benchMem()
 * with following additional options :
 * dstBuffer - destination buffer to write compressed output in, NULL if none provided.
 * dstCapacity - capacity of destination buffer, give 0 if dstBuffer = NULL
 * adv = see advancedParams_t
 */
#[no_mangle]
pub unsafe extern "C" fn BMK_benchMemAdvanced(mut srcBuffer:
                                                  *const libc::c_void,
                                              mut srcSize: size_t,
                                              mut dstBuffer:
                                                  *mut libc::c_void,
                                              mut dstCapacity: size_t,
                                              mut fileSizes: *const size_t,
                                              mut nbFiles: libc::c_uint,
                                              mut cLevel: libc::c_int,
                                              mut comprParams:
                                                  *const ZSTD_compressionParameters,
                                              mut dictBuffer:
                                                  *const libc::c_void,
                                              mut dictBufferSize: size_t,
                                              mut displayLevel: libc::c_int,
                                              mut displayName:
                                                  *const libc::c_char,
                                              mut adv:
                                                  *const BMK_advancedParams_t)
 -> BMK_benchOutcome_t {
    /* must be both NULL or none */
    let dstParamsError: libc::c_int =
        dstBuffer.is_null() as libc::c_int ^
            (0 == dstCapacity) as libc::c_int;
    /* avoid div by 0 */
    let blockSize: size_t =
        if (*adv).blockSize >= 32i32 as libc::c_ulong &&
               (*adv).mode as libc::c_uint !=
                   BMK_decodeOnly as libc::c_int as libc::c_uint {
            (*adv).blockSize
        } else {
            srcSize
        }.wrapping_add((0 == srcSize) as libc::c_int as libc::c_ulong);
    let maxNbBlocks: U32 =
        (srcSize.wrapping_add(blockSize.wrapping_sub(1i32 as
                                                         libc::c_ulong)).wrapping_div(blockSize)
             as U32).wrapping_add(nbFiles);
    /* these are the blockTable parameters, just split up */
    let srcPtrs: *mut *const libc::c_void =
        malloc((maxNbBlocks as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_void>()
                                                    as libc::c_ulong)) as
            *mut *const libc::c_void;
    let srcSizes: *mut size_t =
        malloc((maxNbBlocks as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>()
                                                    as libc::c_ulong)) as
            *mut size_t;
    let cPtrs: *mut *mut libc::c_void =
        malloc((maxNbBlocks as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_void>()
                                                    as libc::c_ulong)) as
            *mut *mut libc::c_void;
    let cSizes: *mut size_t =
        malloc((maxNbBlocks as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>()
                                                    as libc::c_ulong)) as
            *mut size_t;
    let cCapacities: *mut size_t =
        malloc((maxNbBlocks as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>()
                                                    as libc::c_ulong)) as
            *mut size_t;
    let resPtrs: *mut *mut libc::c_void =
        malloc((maxNbBlocks as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_void>()
                                                    as libc::c_ulong)) as
            *mut *mut libc::c_void;
    let resSizes: *mut size_t =
        malloc((maxNbBlocks as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>()
                                                    as libc::c_ulong)) as
            *mut size_t;
    let mut timeStateCompress: *mut BMK_timedFnState_t =
        BMK_createTimedFnState((*adv).nbSeconds.wrapping_mul(1000i32 as
                                                                 libc::c_uint),
                               1000i32 as libc::c_uint);
    let mut timeStateDecompress: *mut BMK_timedFnState_t =
        BMK_createTimedFnState((*adv).nbSeconds.wrapping_mul(1000i32 as
                                                                 libc::c_uint),
                               1000i32 as libc::c_uint);
    let cctx: *mut ZSTD_CCtx = ZSTD_createCCtx();
    let dctx: *mut ZSTD_DCtx = ZSTD_createDCtx();
    let maxCompressedSize: size_t =
        if 0 != dstCapacity {
            dstCapacity
        } else {
            ZSTD_compressBound(srcSize).wrapping_add(maxNbBlocks.wrapping_mul(1024i32
                                                                                  as
                                                                                  libc::c_uint)
                                                         as libc::c_ulong)
        };
    let internalDstBuffer: *mut libc::c_void =
        if !dstBuffer.is_null() {
            0 as *mut libc::c_void
        } else { malloc(maxCompressedSize) };
    let compressedBuffer: *mut libc::c_void =
        if !dstBuffer.is_null() { dstBuffer } else { internalDstBuffer };
    /* error by default */
    let mut outcome: BMK_benchOutcome_t = BMK_benchOutcome_error();
    let mut resultBuffer: *mut libc::c_void =
        if 0 != srcSize { malloc(srcSize) } else { 0 as *mut libc::c_void };
    let mut allocationincomplete: libc::c_int =
        (srcPtrs.is_null() || srcSizes.is_null() || cPtrs.is_null() ||
             cSizes.is_null() || cCapacities.is_null() || resPtrs.is_null() ||
             resSizes.is_null() || timeStateCompress.is_null() ||
             timeStateDecompress.is_null() || cctx.is_null() || dctx.is_null()
             || compressedBuffer.is_null() || resultBuffer.is_null()) as
            libc::c_int;
    if 0 == allocationincomplete && 0 == dstParamsError {
        outcome =
            BMK_benchMemAdvancedNoAlloc(srcPtrs, srcSizes, cPtrs, cCapacities,
                                        cSizes, resPtrs, resSizes,
                                        &mut resultBuffer, compressedBuffer,
                                        maxCompressedSize, timeStateCompress,
                                        timeStateDecompress, srcBuffer,
                                        srcSize, fileSizes, nbFiles, cLevel,
                                        comprParams, dictBuffer,
                                        dictBufferSize, cctx, dctx,
                                        displayLevel, displayName, adv)
    }
    BMK_freeTimedFnState(timeStateCompress);
    BMK_freeTimedFnState(timeStateDecompress);
    ZSTD_freeCCtx(cctx);
    ZSTD_freeDCtx(dctx);
    free(internalDstBuffer);
    free(resultBuffer);
    free(srcPtrs as *mut libc::c_void);
    free(srcSizes as *mut libc::c_void);
    free(cPtrs as *mut libc::c_void);
    free(cSizes as *mut libc::c_void);
    free(cCapacities as *mut libc::c_void);
    free(resPtrs as *mut libc::c_void);
    free(resSizes as *mut libc::c_void);
    if 0 != allocationincomplete {
        let mut r: BMK_benchOutcome_t =
            BMK_benchOutcome_t{internal_never_use_directly:
                                   BMK_benchResult_t{cSize: 0,
                                                     cSpeed: 0,
                                                     dSpeed: 0,
                                                     cMem: 0,},
                               tag: 0,};
        memset(&mut r as *mut BMK_benchOutcome_t as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong);
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    31i32);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"allocation error : not enough memory\x00" as *const u8
                        as *const libc::c_char);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        r.tag = 31i32;
        return r
    }
    if 0 != dstParamsError {
        let mut r_0: BMK_benchOutcome_t =
            BMK_benchOutcome_t{internal_never_use_directly:
                                   BMK_benchResult_t{cSize: 0,
                                                     cSpeed: 0,
                                                     dSpeed: 0,
                                                     cMem: 0,},
                               tag: 0,};
        memset(&mut r_0 as *mut BMK_benchOutcome_t as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong);
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    32i32);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Dst parameters not coherent\x00" as *const u8 as
                        *const libc::c_char);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        r_0.tag = 32i32;
        return r_0
    }
    return outcome;
}
unsafe extern "C" fn BMK_benchOutcome_error() -> BMK_benchOutcome_t {
    let mut b: BMK_benchOutcome_t =
        BMK_benchOutcome_t{internal_never_use_directly:
                               BMK_benchResult_t{cSize: 0,
                                                 cSpeed: 0,
                                                 dSpeed: 0,
                                                 cMem: 0,},
                           tag: 0,};
    memset(&mut b as *mut BMK_benchOutcome_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong);
    b.tag = 1i32;
    return b;
}
/* benchMem with no allocation */
unsafe extern "C" fn BMK_benchMemAdvancedNoAlloc(mut srcPtrs:
                                                     *mut *const libc::c_void,
                                                 mut srcSizes: *mut size_t,
                                                 mut cPtrs:
                                                     *mut *mut libc::c_void,
                                                 mut cCapacities: *mut size_t,
                                                 mut cSizes: *mut size_t,
                                                 mut resPtrs:
                                                     *mut *mut libc::c_void,
                                                 mut resSizes: *mut size_t,
                                                 mut resultBufferPtr:
                                                     *mut *mut libc::c_void,
                                                 mut compressedBuffer:
                                                     *mut libc::c_void,
                                                 mut maxCompressedSize:
                                                     size_t,
                                                 mut timeStateCompress:
                                                     *mut BMK_timedFnState_t,
                                                 mut timeStateDecompress:
                                                     *mut BMK_timedFnState_t,
                                                 mut srcBuffer:
                                                     *const libc::c_void,
                                                 mut srcSize: size_t,
                                                 mut fileSizes: *const size_t,
                                                 mut nbFiles: libc::c_uint,
                                                 cLevel: libc::c_int,
                                                 mut comprParams:
                                                     *const ZSTD_compressionParameters,
                                                 mut dictBuffer:
                                                     *const libc::c_void,
                                                 mut dictBufferSize: size_t,
                                                 mut cctx: *mut ZSTD_CCtx,
                                                 mut dctx: *mut ZSTD_DCtx,
                                                 mut displayLevel:
                                                     libc::c_int,
                                                 mut displayName:
                                                     *const libc::c_char,
                                                 mut adv:
                                                     *const BMK_advancedParams_t)
 -> BMK_benchOutcome_t {
    /* avoid div by 0 */
    let blockSize: size_t =
        if (*adv).blockSize >= 32i32 as libc::c_ulong &&
               (*adv).mode as libc::c_uint !=
                   BMK_decodeOnly as libc::c_int as libc::c_uint {
            (*adv).blockSize
        } else {
            srcSize
        }.wrapping_add((0 == srcSize) as libc::c_int as libc::c_ulong);
    let mut benchResult: BMK_benchResult_t =
        BMK_benchResult_t{cSize: 0, cSpeed: 0, dSpeed: 0, cMem: 0,};
    let loadedCompressedSize: size_t = srcSize;
    let mut cSize: size_t = 0i32 as size_t;
    let mut ratio: libc::c_double = 0.0f64;
    let mut nbBlocks: U32 = 0;
    if !cctx.is_null() {
    } else {
        __assert_fail(b"cctx != NULL\x00" as *const u8 as *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/benchzstd.c\x00"
                          as *const u8 as *const libc::c_char,
                      313i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 385],
                                                &[libc::c_char; 385]>(b"BMK_benchOutcome_t BMK_benchMemAdvancedNoAlloc(const void **, size_t *, void **, size_t *, size_t *, void **, size_t *, void **, void *, size_t, BMK_timedFnState_t *, BMK_timedFnState_t *, const void *, size_t, const size_t *, unsigned int, const int, const ZSTD_compressionParameters *, const void *, size_t, ZSTD_CCtx *, ZSTD_DCtx *, int, const char *, const BMK_advancedParams_t *)\x00")).as_ptr());
    };
    if !dctx.is_null() {
    } else {
        __assert_fail(b"dctx != NULL\x00" as *const u8 as *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/benchzstd.c\x00"
                          as *const u8 as *const libc::c_char,
                      313i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 385],
                                                &[libc::c_char; 385]>(b"BMK_benchOutcome_t BMK_benchMemAdvancedNoAlloc(const void **, size_t *, void **, size_t *, size_t *, void **, size_t *, void **, void *, size_t, BMK_timedFnState_t *, BMK_timedFnState_t *, const void *, size_t, const size_t *, unsigned int, const int, const ZSTD_compressionParameters *, const void *, size_t, ZSTD_CCtx *, ZSTD_DCtx *, int, const char *, const BMK_advancedParams_t *)\x00")).as_ptr());
    };
    memset(&mut benchResult as *mut BMK_benchResult_t as *mut libc::c_void,
           0i32, ::std::mem::size_of::<BMK_benchResult_t>() as libc::c_ulong);
    if strlen(displayName) > 17i32 as libc::c_ulong {
        displayName =
            displayName.offset(strlen(displayName).wrapping_sub(17i32 as
                                                                    libc::c_ulong)
                                   as isize)
    }
    if (*adv).mode as libc::c_uint ==
           BMK_decodeOnly as libc::c_int as libc::c_uint {
        let mut srcPtr: *const libc::c_char =
            srcBuffer as *const libc::c_char;
        let mut totalDSize64: U64 = 0i32 as U64;
        let mut fileNb: U32 = 0;
        fileNb = 0i32 as U32;
        while fileNb < nbFiles {
            let fSize64: U64 =
                ZSTD_findDecompressedSize(srcPtr as *const libc::c_void,
                                          *fileSizes.offset(fileNb as isize))
                    as U64;
            if fSize64 == 0i32 as libc::c_ulong {
                let mut r: BMK_benchOutcome_t =
                    BMK_benchOutcome_t{internal_never_use_directly:
                                           BMK_benchResult_t{cSize: 0,
                                                             cSpeed: 0,
                                                             dSpeed: 0,
                                                             cMem: 0,},
                                       tag: 0,};
                memset(&mut r as *mut BMK_benchOutcome_t as *mut libc::c_void,
                       0i32,
                       ::std::mem::size_of::<BMK_benchOutcome_t>() as
                           libc::c_ulong);
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 32i32);
                }
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"Impossible to determine original size \x00" as
                                *const u8 as *const libc::c_char);
                }
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                r.tag = 32i32;
                return r
            }
            totalDSize64 =
                (totalDSize64 as libc::c_ulong).wrapping_add(fSize64) as U64
                    as U64;
            srcPtr =
                srcPtr.offset(*fileSizes.offset(fileNb as isize) as isize);
            fileNb = fileNb.wrapping_add(1)
        }
        let decodedSize: size_t = totalDSize64;
        if decodedSize == totalDSize64 {
        } else {
            __assert_fail(b"(U64)decodedSize == totalDSize64\x00" as *const u8
                              as *const libc::c_char,
                          b"/home/danielrh/dev/zstd-c2rust/programs/benchzstd.c\x00"
                              as *const u8 as *const libc::c_char,
                          329i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 385],
                                                    &[libc::c_char; 385]>(b"BMK_benchOutcome_t BMK_benchMemAdvancedNoAlloc(const void **, size_t *, void **, size_t *, size_t *, void **, size_t *, void **, void *, size_t, BMK_timedFnState_t *, BMK_timedFnState_t *, const void *, size_t, const size_t *, unsigned int, const int, const ZSTD_compressionParameters *, const void *, size_t, ZSTD_CCtx *, ZSTD_DCtx *, int, const char *, const BMK_advancedParams_t *)\x00")).as_ptr());
        };
        free(*resultBufferPtr);
        *resultBufferPtr = malloc(decodedSize);
        if (*resultBufferPtr).is_null() {
            let mut r_0: BMK_benchOutcome_t =
                BMK_benchOutcome_t{internal_never_use_directly:
                                       BMK_benchResult_t{cSize: 0,
                                                         cSpeed: 0,
                                                         dSpeed: 0,
                                                         cMem: 0,},
                                   tag: 0,};
            memset(&mut r_0 as *mut BMK_benchOutcome_t as *mut libc::c_void,
                   0i32,
                   ::std::mem::size_of::<BMK_benchOutcome_t>() as
                       libc::c_ulong);
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 33i32);
            }
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b"not enough memory\x00" as *const u8 as
                            *const libc::c_char);
            }
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            r_0.tag = 33i32;
            return r_0
        }
        if totalDSize64 > decodedSize {
            free(*resultBufferPtr);
            let mut r_1: BMK_benchOutcome_t =
                BMK_benchOutcome_t{internal_never_use_directly:
                                       BMK_benchResult_t{cSize: 0,
                                                         cSpeed: 0,
                                                         dSpeed: 0,
                                                         cMem: 0,},
                                   tag: 0,};
            memset(&mut r_1 as *mut BMK_benchOutcome_t as *mut libc::c_void,
                   0i32,
                   ::std::mem::size_of::<BMK_benchOutcome_t>() as
                       libc::c_ulong);
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 32i32);
            }
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b"original size is too large\x00" as *const u8 as
                            *const libc::c_char);
            }
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            r_1.tag = 32i32;
            return r_1
        }
        cSize = srcSize;
        srcSize = decodedSize;
        ratio = srcSize as libc::c_double / cSize as libc::c_double
    }
    let mut srcPtr_0: *const libc::c_char = srcBuffer as *const libc::c_char;
    let mut cPtr: *mut libc::c_char = compressedBuffer as *mut libc::c_char;
    let mut resPtr: *mut libc::c_char = *resultBufferPtr as *mut libc::c_char;
    let mut fileNb_0: U32 = 0;
    nbBlocks = 0i32 as U32;
    fileNb_0 = 0i32 as U32;
    while fileNb_0 < nbFiles {
        let mut remaining: size_t = *fileSizes.offset(fileNb_0 as isize);
        let nbBlocksforThisFile: U32 =
            if (*adv).mode as libc::c_uint ==
                   BMK_decodeOnly as libc::c_int as libc::c_uint {
                1i32 as libc::c_uint
            } else {
                remaining.wrapping_add(blockSize.wrapping_sub(1i32 as
                                                                  libc::c_ulong)).wrapping_div(blockSize)
                    as U32
            };
        let blockEnd: U32 = nbBlocks.wrapping_add(nbBlocksforThisFile);
        while nbBlocks < blockEnd {
            let thisBlockSize: size_t =
                if remaining < blockSize { remaining } else { blockSize };
            let ref mut fresh0 = *srcPtrs.offset(nbBlocks as isize);
            *fresh0 = srcPtr_0 as *const libc::c_void;
            *srcSizes.offset(nbBlocks as isize) = thisBlockSize;
            let ref mut fresh1 = *cPtrs.offset(nbBlocks as isize);
            *fresh1 = cPtr as *mut libc::c_void;
            *cCapacities.offset(nbBlocks as isize) =
                if (*adv).mode as libc::c_uint ==
                       BMK_decodeOnly as libc::c_int as libc::c_uint {
                    thisBlockSize
                } else { ZSTD_compressBound(thisBlockSize) };
            let ref mut fresh2 = *resPtrs.offset(nbBlocks as isize);
            *fresh2 = resPtr as *mut libc::c_void;
            *resSizes.offset(nbBlocks as isize) =
                if (*adv).mode as libc::c_uint ==
                       BMK_decodeOnly as libc::c_int as libc::c_uint {
                    ZSTD_findDecompressedSize(srcPtr_0 as *const libc::c_void,
                                              thisBlockSize) as size_t
                } else { thisBlockSize };
            srcPtr_0 = srcPtr_0.offset(thisBlockSize as isize);
            cPtr =
                cPtr.offset(*cCapacities.offset(nbBlocks as isize) as isize);
            resPtr = resPtr.offset(thisBlockSize as isize);
            remaining =
                (remaining as libc::c_ulong).wrapping_sub(thisBlockSize) as
                    size_t as size_t;
            if (*adv).mode as libc::c_uint ==
                   BMK_decodeOnly as libc::c_int as libc::c_uint {
                if nbBlocks == 0i32 as libc::c_uint {
                } else {
                    __assert_fail(b"nbBlocks==0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"/home/danielrh/dev/zstd-c2rust/programs/benchzstd.c\x00"
                                      as *const u8 as *const libc::c_char,
                                  367i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 385],
                                                            &[libc::c_char; 385]>(b"BMK_benchOutcome_t BMK_benchMemAdvancedNoAlloc(const void **, size_t *, void **, size_t *, size_t *, void **, size_t *, void **, void *, size_t, BMK_timedFnState_t *, BMK_timedFnState_t *, const void *, size_t, const size_t *, unsigned int, const int, const ZSTD_compressionParameters *, const void *, size_t, ZSTD_CCtx *, ZSTD_DCtx *, int, const char *, const BMK_advancedParams_t *)\x00")).as_ptr());
                };
                *cSizes.offset(nbBlocks as isize) = thisBlockSize;
                benchResult.cSize = thisBlockSize
            }
            nbBlocks = nbBlocks.wrapping_add(1)
        }
        fileNb_0 = fileNb_0.wrapping_add(1)
    }
    if (*adv).mode as libc::c_uint ==
           BMK_decodeOnly as libc::c_int as libc::c_uint {
        memcpy(compressedBuffer, srcBuffer, loadedCompressedSize);
    } else {
        RDG_genBuffer(compressedBuffer, maxCompressedSize, 0.10f64, 0.50f64,
                      1i32 as libc::c_uint);
    }
    let crcOrig: U64 =
        (if (*adv).mode as libc::c_uint ==
                BMK_decodeOnly as libc::c_int as libc::c_uint {
             0i32 as libc::c_ulonglong
         } else { ZSTD_XXH64(srcBuffer, srcSize, 0i32 as libc::c_ulonglong) })
            as U64;
    let mut marks: [*const libc::c_char; 4] =
        [b" |\x00" as *const u8 as *const libc::c_char,
         b" /\x00" as *const u8 as *const libc::c_char,
         b" =\x00" as *const u8 as *const libc::c_char,
         b" \\\x00" as *const u8 as *const libc::c_char];
    let mut markNb: U32 = 0i32 as U32;
    let mut compressionCompleted: libc::c_int =
        ((*adv).mode as libc::c_uint ==
             BMK_decodeOnly as libc::c_int as libc::c_uint) as libc::c_int;
    let mut decompressionCompleted: libc::c_int =
        ((*adv).mode as libc::c_uint ==
             BMK_compressOnly as libc::c_int as libc::c_uint) as libc::c_int;
    let mut cbp: BMK_benchParams_t =
        BMK_benchParams_t{benchFn: None,
                          benchPayload: 0 as *mut libc::c_void,
                          initFn: None,
                          initPayload: 0 as *mut libc::c_void,
                          errorFn: None,
                          blockCount: 0,
                          srcBuffers: 0 as *const *const libc::c_void,
                          srcSizes: 0 as *const size_t,
                          dstBuffers: 0 as *const *mut libc::c_void,
                          dstCapacities: 0 as *const size_t,
                          blockResults: 0 as *mut size_t,};
    let mut dbp: BMK_benchParams_t =
        BMK_benchParams_t{benchFn: None,
                          benchPayload: 0 as *mut libc::c_void,
                          initFn: None,
                          initPayload: 0 as *mut libc::c_void,
                          errorFn: None,
                          blockCount: 0,
                          srcBuffers: 0 as *const *const libc::c_void,
                          srcSizes: 0 as *const size_t,
                          dstBuffers: 0 as *const *mut libc::c_void,
                          dstCapacities: 0 as *const size_t,
                          blockResults: 0 as *mut size_t,};
    let mut cctxprep: BMK_initCCtxArgs =
        BMK_initCCtxArgs{cctx: 0 as *mut ZSTD_CCtx,
                         dictBuffer: 0 as *const libc::c_void,
                         dictBufferSize: 0,
                         cLevel: 0,
                         comprParams: 0 as *const ZSTD_compressionParameters,
                         adv: 0 as *const BMK_advancedParams_t,};
    let mut dctxprep: BMK_initDCtxArgs =
        BMK_initDCtxArgs{dctx: 0 as *mut ZSTD_DCtx,
                         dictBuffer: 0 as *const libc::c_void,
                         dictBufferSize: 0,};
    cbp.benchFn = Some(local_defaultCompress);
    cbp.benchPayload = cctx as *mut libc::c_void;
    cbp.initFn = Some(local_initCCtx);
    cbp.initPayload =
        &mut cctxprep as *mut BMK_initCCtxArgs as *mut libc::c_void;
    cbp.errorFn = Some(ZSTD_isError);
    cbp.blockCount = nbBlocks as size_t;
    cbp.srcBuffers = srcPtrs;
    cbp.srcSizes = srcSizes;
    cbp.dstBuffers = cPtrs;
    cbp.dstCapacities = cCapacities;
    cbp.blockResults = cSizes;
    cctxprep.cctx = cctx;
    cctxprep.dictBuffer = dictBuffer;
    cctxprep.dictBufferSize = dictBufferSize;
    cctxprep.cLevel = cLevel;
    cctxprep.comprParams = comprParams;
    cctxprep.adv = adv;
    dbp.benchFn = Some(local_defaultDecompress);
    dbp.benchPayload = dctx as *mut libc::c_void;
    dbp.initFn = Some(local_initDCtx);
    dbp.initPayload =
        &mut dctxprep as *mut BMK_initDCtxArgs as *mut libc::c_void;
    dbp.errorFn = Some(ZSTD_isError);
    dbp.blockCount = nbBlocks as size_t;
    dbp.srcBuffers = cPtrs as *const *const libc::c_void;
    dbp.srcSizes = cSizes;
    dbp.dstBuffers = resPtrs;
    dbp.dstCapacities = resSizes;
    dbp.blockResults = 0 as *mut size_t;
    dctxprep.dctx = dctx;
    dctxprep.dictBuffer = dictBuffer;
    dctxprep.dictBufferSize = dictBufferSize;
    if displayLevel >= 2i32 {
        fprintf(stderr, b"\r%70s\r\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char);
    }
    if displayLevel >= 2i32 {
        fprintf(stderr,
                b"%2s-%-17.17s :%10u ->\r\x00" as *const u8 as
                    *const libc::c_char, marks[markNb as usize], displayName,
                srcSize as libc::c_uint);
    }
    while !(0 != compressionCompleted && 0 != decompressionCompleted) {
        if 0 == compressionCompleted {
            let cOutcome: BMK_runOutcome_t =
                BMK_benchTimedFn(timeStateCompress, cbp);
            if 0 == BMK_isSuccessful_runOutcome(cOutcome) {
                return BMK_benchOutcome_error()
            }
            let cResult: BMK_runTime_t = BMK_extract_runTime(cOutcome);
            cSize = cResult.sumOfReturn;
            ratio = srcSize as libc::c_double / cSize as libc::c_double;
            let mut newResult: BMK_benchResult_t =
                BMK_benchResult_t{cSize: 0, cSpeed: 0, dSpeed: 0, cMem: 0,};
            newResult.cSpeed =
                (srcSize as
                     libc::c_ulonglong).wrapping_mul((1i32 as
                                                          libc::c_ulonglong).wrapping_mul(1000000000u64)).wrapping_div(cResult.nanoSecPerRun);
            benchResult.cSize = cSize;
            if newResult.cSpeed > benchResult.cSpeed {
                benchResult.cSpeed = newResult.cSpeed
            }
            let ratioAccuracy: libc::c_int =
                if ratio < 10.0f64 { 3i32 } else { 2i32 };
            if displayLevel >= 2i32 {
                fprintf(stderr,
                        b"%2s-%-17.17s :%10u ->%10u (%5.*f),%6.*f MB/s\r\x00"
                            as *const u8 as *const libc::c_char,
                        marks[markNb as usize], displayName,
                        srcSize as libc::c_uint, cSize as libc::c_uint,
                        ratioAccuracy, ratio,
                        if benchResult.cSpeed <
                               (10i32 * (1i32 << 20i32)) as libc::c_ulonglong
                           {
                            2i32
                        } else { 1i32 },
                        benchResult.cSpeed as libc::c_double /
                            1000000i32 as libc::c_double);
            }
            compressionCompleted = BMK_isCompleted_TimedFn(timeStateCompress)
        }
        if 0 == decompressionCompleted {
            let dOutcome: BMK_runOutcome_t =
                BMK_benchTimedFn(timeStateDecompress, dbp);
            if 0 == BMK_isSuccessful_runOutcome(dOutcome) {
                return BMK_benchOutcome_error()
            }
            let dResult: BMK_runTime_t = BMK_extract_runTime(dOutcome);
            let newDSpeed: U64 =
                (srcSize as
                     libc::c_ulonglong).wrapping_mul((1i32 as
                                                          libc::c_ulonglong).wrapping_mul(1000000000u64)).wrapping_div(dResult.nanoSecPerRun)
                    as U64;
            if newDSpeed as libc::c_ulonglong > benchResult.dSpeed {
                benchResult.dSpeed = newDSpeed as libc::c_ulonglong
            }
            let ratioAccuracy_0: libc::c_int =
                if ratio < 10.0f64 { 3i32 } else { 2i32 };
            if displayLevel >= 2i32 {
                fprintf(stderr,
                        b"%2s-%-17.17s :%10u ->%10u (%5.*f),%6.*f MB/s ,%6.1f MB/s \r\x00"
                            as *const u8 as *const libc::c_char,
                        marks[markNb as usize], displayName,
                        srcSize as libc::c_uint,
                        benchResult.cSize as libc::c_uint, ratioAccuracy_0,
                        ratio,
                        if benchResult.cSpeed <
                               (10i32 * (1i32 << 20i32)) as libc::c_ulonglong
                           {
                            2i32
                        } else { 1i32 },
                        benchResult.cSpeed as libc::c_double /
                            1000000i32 as libc::c_double,
                        benchResult.dSpeed as libc::c_double /
                            1000000i32 as libc::c_double);
            }
            decompressionCompleted =
                BMK_isCompleted_TimedFn(timeStateDecompress)
        }
        markNb =
            markNb.wrapping_add(1i32 as
                                    libc::c_uint).wrapping_rem(4i32 as
                                                                   libc::c_uint)
    }
    let mut resultBuffer: *const BYTE = *resultBufferPtr as *const BYTE;
    let crcCheck: U64 =
        ZSTD_XXH64(resultBuffer as *const libc::c_void, srcSize,
                   0i32 as libc::c_ulonglong) as U64;
    if (*adv).mode as libc::c_uint == BMK_both as libc::c_int as libc::c_uint
           && crcOrig != crcCheck {
        let mut u: size_t = 0;
        fprintf(stderr,
                b"!!! WARNING !!! %14s : Invalid Checksum : %x != %x   \n\x00"
                    as *const u8 as *const libc::c_char, displayName,
                crcOrig as libc::c_uint, crcCheck as libc::c_uint);
        u = 0i32 as size_t;
        while u < srcSize {
            if *(srcBuffer as *const BYTE).offset(u as isize) as libc::c_int
                   != *resultBuffer.offset(u as isize) as libc::c_int {
                let mut segNb: libc::c_uint = 0;
                let mut bNb: libc::c_uint = 0;
                let mut pos: libc::c_uint = 0;
                let mut bacc: size_t = 0i32 as size_t;
                fprintf(stderr,
                        b"Decoding error at pos %u \x00" as *const u8 as
                            *const libc::c_char, u as libc::c_uint);
                segNb = 0i32 as libc::c_uint;
                while segNb < nbBlocks {
                    if bacc.wrapping_add(*srcSizes.offset(segNb as isize)) > u
                       {
                        break ;
                    }
                    bacc =
                        (bacc as
                             libc::c_ulong).wrapping_add(*srcSizes.offset(segNb
                                                                              as
                                                                              isize))
                            as size_t as size_t;
                    segNb = segNb.wrapping_add(1)
                }
                pos = u.wrapping_sub(bacc) as U32;
                bNb =
                    pos.wrapping_div((128i32 * (1i32 << 10i32)) as
                                         libc::c_uint);
                fprintf(stderr,
                        b"(sample %u, block %u, pos %u) \n\x00" as *const u8
                            as *const libc::c_char, segNb, bNb, pos);
                if u > 5i32 as libc::c_ulong {
                    let mut n: libc::c_int = 0;
                    fprintf(stderr,
                            b"origin: \x00" as *const u8 as
                                *const libc::c_char);
                    n = -5i32;
                    while n < 0i32 {
                        fprintf(stderr,
                                b"%02X \x00" as *const u8 as
                                    *const libc::c_char,
                                *(srcBuffer as
                                      *const BYTE).offset(u.wrapping_add(n as
                                                                             libc::c_ulong)
                                                              as isize) as
                                    libc::c_int);
                        n += 1
                    }
                    fprintf(stderr,
                            b" :%02X:  \x00" as *const u8 as
                                *const libc::c_char,
                            *(srcBuffer as *const BYTE).offset(u as isize) as
                                libc::c_int);
                    n = 1i32;
                    while n < 3i32 {
                        fprintf(stderr,
                                b"%02X \x00" as *const u8 as
                                    *const libc::c_char,
                                *(srcBuffer as
                                      *const BYTE).offset(u.wrapping_add(n as
                                                                             libc::c_ulong)
                                                              as isize) as
                                    libc::c_int);
                        n += 1
                    }
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                    fprintf(stderr,
                            b"decode: \x00" as *const u8 as
                                *const libc::c_char);
                    n = -5i32;
                    while n < 0i32 {
                        fprintf(stderr,
                                b"%02X \x00" as *const u8 as
                                    *const libc::c_char,
                                *resultBuffer.offset(u.wrapping_add(n as
                                                                        libc::c_ulong)
                                                         as isize) as
                                    libc::c_int);
                        n += 1
                    }
                    fprintf(stderr,
                            b" :%02X:  \x00" as *const u8 as
                                *const libc::c_char,
                            *resultBuffer.offset(u as isize) as libc::c_int);
                    n = 1i32;
                    while n < 3i32 {
                        fprintf(stderr,
                                b"%02X \x00" as *const u8 as
                                    *const libc::c_char,
                                *resultBuffer.offset(u.wrapping_add(n as
                                                                        libc::c_ulong)
                                                         as isize) as
                                    libc::c_int);
                        n += 1
                    }
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                break ;
            } else {
                if u == srcSize.wrapping_sub(1i32 as libc::c_ulong) {
                    fprintf(stderr,
                            b"no difference detected\n\x00" as *const u8 as
                                *const libc::c_char);
                }
                u = u.wrapping_add(1)
            }
        }
    }
    if displayLevel == 1i32 {
        let cSpeed: libc::c_double =
            benchResult.cSpeed as libc::c_double /
                1000000i32 as libc::c_double;
        let dSpeed: libc::c_double =
            benchResult.dSpeed as libc::c_double /
                1000000i32 as libc::c_double;
        if 0 != (*adv).additionalParam {
            fprintf(stderr,
                    b"-%-3i%11i (%5.3f) %6.2f MB/s %6.1f MB/s  %s (param=%d)\n\x00"
                        as *const u8 as *const libc::c_char, cLevel,
                    cSize as libc::c_int, ratio, cSpeed, dSpeed, displayName,
                    (*adv).additionalParam);
        } else {
            fprintf(stderr,
                    b"-%-3i%11i (%5.3f) %6.2f MB/s %6.1f MB/s  %s\n\x00" as
                        *const u8 as *const libc::c_char, cLevel,
                    cSize as libc::c_int, ratio, cSpeed, dSpeed, displayName);
        }
    }
    if displayLevel >= 2i32 {
        fprintf(stderr, b"%2i#\n\x00" as *const u8 as *const libc::c_char,
                cLevel);
    }
    benchResult.cMem =
        (1u64 <<
             (*comprParams).windowLog).wrapping_add(ZSTD_sizeof_CCtx(cctx) as
                                                        libc::c_ulonglong) as
            size_t;
    return BMK_benchOutcome_setValidResult(benchResult);
}
unsafe extern "C" fn BMK_benchOutcome_setValidResult(mut result:
                                                         BMK_benchResult_t)
 -> BMK_benchOutcome_t {
    let mut b: BMK_benchOutcome_t =
        BMK_benchOutcome_t{internal_never_use_directly:
                               BMK_benchResult_t{cSize: 0,
                                                 cSpeed: 0,
                                                 dSpeed: 0,
                                                 cMem: 0,},
                           tag: 0,};
    b.tag = 0i32;
    b.internal_never_use_directly = result;
    return b;
}
unsafe extern "C" fn local_initDCtx(mut payload: *mut libc::c_void)
 -> size_t {
    let mut ag: *mut BMK_initDCtxArgs = payload as *mut BMK_initDCtxArgs;
    BMK_initDCtx((*ag).dctx, (*ag).dictBuffer, (*ag).dictBufferSize);
    return 0i32 as size_t;
}
unsafe extern "C" fn BMK_initDCtx(mut dctx: *mut ZSTD_DCtx,
                                  mut dictBuffer: *const libc::c_void,
                                  mut dictBufferSize: size_t) {
    let zerr: size_t =
        ZSTD_DCtx_reset(dctx, ZSTD_reset_session_and_parameters);
    if 0 != ZSTD_isError(zerr) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_DCtx_reset(dctx, ZSTD_reset_session_and_parameters)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    }
    let zerr_0: size_t =
        ZSTD_DCtx_loadDictionary(dctx, dictBuffer, dictBufferSize);
    if 0 != ZSTD_isError(zerr_0) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_DCtx_loadDictionary(dctx, dictBuffer, dictBufferSize)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_0));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    };
}
/* `addArgs` is the context */
unsafe extern "C" fn local_defaultDecompress(mut srcBuffer:
                                                 *const libc::c_void,
                                             mut srcSize: size_t,
                                             mut dstBuffer: *mut libc::c_void,
                                             mut dstCapacity: size_t,
                                             mut addArgs: *mut libc::c_void)
 -> size_t {
    let mut moreToFlush: size_t = 1i32 as size_t;
    let dctx: *mut ZSTD_DCtx = addArgs as *mut ZSTD_DCtx;
    let mut in_0: ZSTD_inBuffer =
        ZSTD_inBuffer_s{src: 0 as *const libc::c_void, size: 0, pos: 0,};
    let mut out: ZSTD_outBuffer =
        ZSTD_outBuffer_s{dst: 0 as *mut libc::c_void, size: 0, pos: 0,};
    in_0.src = srcBuffer;
    in_0.size = srcSize;
    in_0.pos = 0i32 as size_t;
    out.dst = dstBuffer;
    out.size = dstCapacity;
    out.pos = 0i32 as size_t;
    while 0 != moreToFlush {
        if out.pos == out.size {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
        }
        moreToFlush = ZSTD_decompressStream(dctx, &mut out, &mut in_0);
        if 0 != ZSTD_isError(moreToFlush) { return moreToFlush }
    }
    return out.pos;
}
unsafe extern "C" fn local_initCCtx(mut payload: *mut libc::c_void)
 -> size_t {
    let mut ag: *mut BMK_initCCtxArgs = payload as *mut BMK_initCCtxArgs;
    BMK_initCCtx((*ag).cctx, (*ag).dictBuffer, (*ag).dictBufferSize,
                 (*ag).cLevel, (*ag).comprParams, (*ag).adv);
    return 0i32 as size_t;
}
unsafe extern "C" fn BMK_initCCtx(mut ctx: *mut ZSTD_CCtx,
                                  mut dictBuffer: *const libc::c_void,
                                  mut dictBufferSize: size_t,
                                  mut cLevel: libc::c_int,
                                  mut comprParams:
                                      *const ZSTD_compressionParameters,
                                  mut adv: *const BMK_advancedParams_t) {
    ZSTD_CCtx_reset(ctx, ZSTD_reset_session_and_parameters);
    if (*adv).nbWorkers == 1i32 as libc::c_uint {
        let zerr: size_t =
            ZSTD_CCtx_setParameter(ctx, ZSTD_c_nbWorkers, 0i32);
        if 0 != ZSTD_isError(zerr) {
            fprintf(stderr,
                    b"Error : \x00" as *const u8 as *const libc::c_char);
            fprintf(stderr,
                    b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_nbWorkers, 0)\x00" as
                        *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(zerr));
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
            exit(1i32);
        }
    } else {
        let zerr_0: size_t =
            ZSTD_CCtx_setParameter(ctx, ZSTD_c_nbWorkers,
                                   (*adv).nbWorkers as libc::c_int);
        if 0 != ZSTD_isError(zerr_0) {
            fprintf(stderr,
                    b"Error : \x00" as *const u8 as *const libc::c_char);
            fprintf(stderr,
                    b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_nbWorkers, adv->nbWorkers)\x00"
                        as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(zerr_0));
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
            exit(1i32);
        }
    }
    let zerr_1: size_t =
        ZSTD_CCtx_setParameter(ctx, ZSTD_c_compressionLevel, cLevel);
    if 0 != ZSTD_isError(zerr_1) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_compressionLevel, cLevel)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_1));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    }
    let zerr_2: size_t =
        ZSTD_CCtx_setParameter(ctx, ZSTD_c_enableLongDistanceMatching,
                               (*adv).ldmFlag as libc::c_int);
    if 0 != ZSTD_isError(zerr_2) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_enableLongDistanceMatching, adv->ldmFlag)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_2));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    }
    let zerr_3: size_t =
        ZSTD_CCtx_setParameter(ctx, ZSTD_c_ldmMinMatch,
                               (*adv).ldmMinMatch as libc::c_int);
    if 0 != ZSTD_isError(zerr_3) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_ldmMinMatch, adv->ldmMinMatch)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_3));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    }
    let zerr_4: size_t =
        ZSTD_CCtx_setParameter(ctx, ZSTD_c_ldmHashLog,
                               (*adv).ldmHashLog as libc::c_int);
    if 0 != ZSTD_isError(zerr_4) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_ldmHashLog, adv->ldmHashLog)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_4));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    }
    let zerr_5: size_t =
        ZSTD_CCtx_setParameter(ctx, ZSTD_c_ldmBucketSizeLog,
                               (*adv).ldmBucketSizeLog as libc::c_int);
    if 0 != ZSTD_isError(zerr_5) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_ldmBucketSizeLog, adv->ldmBucketSizeLog)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_5));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    }
    let zerr_6: size_t =
        ZSTD_CCtx_setParameter(ctx, ZSTD_c_ldmHashRateLog,
                               (*adv).ldmHashRateLog as libc::c_int);
    if 0 != ZSTD_isError(zerr_6) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_ldmHashRateLog, adv->ldmHashRateLog)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_6));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    }
    let zerr_7: size_t =
        ZSTD_CCtx_setParameter(ctx, ZSTD_c_windowLog,
                               (*comprParams).windowLog as libc::c_int);
    if 0 != ZSTD_isError(zerr_7) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_windowLog, comprParams->windowLog)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_7));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    }
    let zerr_8: size_t =
        ZSTD_CCtx_setParameter(ctx, ZSTD_c_hashLog,
                               (*comprParams).hashLog as libc::c_int);
    if 0 != ZSTD_isError(zerr_8) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_hashLog, comprParams->hashLog)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_8));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    }
    let zerr_9: size_t =
        ZSTD_CCtx_setParameter(ctx, ZSTD_c_chainLog,
                               (*comprParams).chainLog as libc::c_int);
    if 0 != ZSTD_isError(zerr_9) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_chainLog, comprParams->chainLog)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_9));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    }
    let zerr_10: size_t =
        ZSTD_CCtx_setParameter(ctx, ZSTD_c_searchLog,
                               (*comprParams).searchLog as libc::c_int);
    if 0 != ZSTD_isError(zerr_10) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_searchLog, comprParams->searchLog)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_10));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    }
    let zerr_11: size_t =
        ZSTD_CCtx_setParameter(ctx, ZSTD_c_minMatch,
                               (*comprParams).minMatch as libc::c_int);
    if 0 != ZSTD_isError(zerr_11) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_minMatch, comprParams->minMatch)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_11));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    }
    let zerr_12: size_t =
        ZSTD_CCtx_setParameter(ctx, ZSTD_c_targetLength,
                               (*comprParams).targetLength as libc::c_int);
    if 0 != ZSTD_isError(zerr_12) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_targetLength, comprParams->targetLength)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_12));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    }
    let zerr_13: size_t =
        ZSTD_CCtx_setParameter(ctx, ZSTD_c_strategy,
                               (*comprParams).strategy as libc::c_int);
    if 0 != ZSTD_isError(zerr_13) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_c_strategy, comprParams->strategy)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_13));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    }
    let zerr_14: size_t =
        ZSTD_CCtx_loadDictionary(ctx, dictBuffer, dictBufferSize);
    if 0 != ZSTD_isError(zerr_14) {
        fprintf(stderr, b"Error : \x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"%s failed : %s\x00" as *const u8 as *const libc::c_char,
                b"ZSTD_CCtx_loadDictionary(ctx, dictBuffer, dictBufferSize)\x00"
                    as *const u8 as *const libc::c_char,
                ZSTD_getErrorName(zerr_14));
        fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        exit(1i32);
    };
}
/* `addArgs` is the context */
unsafe extern "C" fn local_defaultCompress(mut srcBuffer: *const libc::c_void,
                                           mut srcSize: size_t,
                                           mut dstBuffer: *mut libc::c_void,
                                           mut dstSize: size_t,
                                           mut addArgs: *mut libc::c_void)
 -> size_t {
    let cctx: *mut ZSTD_CCtx = addArgs as *mut ZSTD_CCtx;
    return ZSTD_compress2(cctx, dstBuffer, dstSize, srcBuffer, srcSize);
}
/* ! BMK_loadFiles() :
 *  Loads `buffer` with content of files listed within `fileNamesTable`.
 *  At most, fills `buffer` entirely. */
unsafe extern "C" fn BMK_loadFiles(mut buffer: *mut libc::c_void,
                                   mut bufferSize: size_t,
                                   mut fileSizes: *mut size_t,
                                   mut fileNamesTable:
                                       *const *const libc::c_char,
                                   mut nbFiles: libc::c_uint,
                                   mut displayLevel: libc::c_int)
 -> libc::c_int {
    let mut pos: size_t = 0i32 as size_t;
    let mut totalSize: size_t = 0i32 as size_t;
    let mut n: libc::c_uint = 0;
    n = 0i32 as libc::c_uint;
    while n < nbFiles {
        let mut f: *mut FILE = 0 as *mut FILE;
        let mut fileSize: U64 =
            UTIL_getFileSize(*fileNamesTable.offset(n as isize));
        if 0 != UTIL_isDirectory(*fileNamesTable.offset(n as isize)) {
            if displayLevel >= 2i32 {
                fprintf(stderr,
                        b"Ignoring %s directory...       \n\x00" as *const u8
                            as *const libc::c_char,
                        *fileNamesTable.offset(n as isize));
            }
            *fileSizes.offset(n as isize) = 0i32 as size_t
        } else if fileSize == -1i32 as U64 {
            if displayLevel >= 2i32 {
                fprintf(stderr,
                        b"Cannot evaluate size of %s, ignoring ... \n\x00" as
                            *const u8 as *const libc::c_char,
                        *fileNamesTable.offset(n as isize));
            }
            *fileSizes.offset(n as isize) = 0i32 as size_t
        } else {
            f =
                fopen(*fileNamesTable.offset(n as isize),
                      b"rb\x00" as *const u8 as *const libc::c_char);
            if f.is_null() {
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 10i32);
                }
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"impossible to open file %s\x00" as *const u8 as
                                *const libc::c_char,
                            *fileNamesTable.offset(n as isize));
                }
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                return 10i32
            }
            if displayLevel >= 2i32 {
                if UTIL_clockSpanMicro(g_displayClock) > g_refreshRate ||
                       displayLevel >= 4i32 {
                    g_displayClock = UTIL_getTime();
                    fprintf(stderr,
                            b"Loading %s...       \r\x00" as *const u8 as
                                *const libc::c_char,
                            *fileNamesTable.offset(n as isize));
                    if displayLevel >= 4i32 { fflush(stderr); }
                }
            }
            if fileSize > bufferSize.wrapping_sub(pos) {
                fileSize = bufferSize.wrapping_sub(pos);
                nbFiles = n
            }
            let readSize: size_t =
                fread((buffer as *mut libc::c_char).offset(pos as isize) as
                          *mut libc::c_void, 1i32 as size_t, fileSize, f);
            if readSize != fileSize {
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 11i32);
                }
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"could not read %s\x00" as *const u8 as
                                *const libc::c_char,
                            *fileNamesTable.offset(n as isize));
                }
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                return 11i32
            }
            pos =
                (pos as libc::c_ulong).wrapping_add(readSize) as size_t as
                    size_t;
            *fileSizes.offset(n as isize) = fileSize;
            totalSize =
                (totalSize as libc::c_ulong).wrapping_add(fileSize) as size_t
                    as size_t;
            fclose(f);
        }
        n = n.wrapping_add(1)
    }
    if totalSize == 0i32 as libc::c_ulong {
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    12i32);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"no data to bench\x00" as *const u8 as
                        *const libc::c_char);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        return 12i32
    }
    return 0i32;
}
static mut g_displayClock: UTIL_time_t =
    timespec{tv_sec: 0i32 as __time_t, tv_nsec: 0i32 as __syscall_slong_t,};
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* **************************************
*  Tuning parameters
****************************************/
/* default minimum time per test */
/* *************************************
*  Includes
***************************************/
/* malloc, free */
/* memset, strerror */
/* fprintf, fopen */
/* assert */
/* *************************************
*  Constants
***************************************/
/* 1 second */
/* 1 second */
/* 70 seconds */
/* *************************************
*  console display
***************************************/
/* 0 : no display;   1: errors;   2 : + result + interaction + warnings;   3 : + progression;   4 : + information */
static mut g_refreshRate: U64 = (1000000i32 / 6i32) as U64;
unsafe extern "C" fn BMK_findMaxMem(mut requiredMem: U64) -> size_t {
    let step: size_t = (64i32 * (1i32 << 20i32)) as size_t;
    let mut testmem: *mut BYTE = 0 as *mut BYTE;
    let mut maxMemory: size_t =
        if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
               4i32 as libc::c_ulong {
            (2i32 as
                 libc::c_uint).wrapping_mul(1u32 <<
                                                30i32).wrapping_sub((64i32 *
                                                                         (1i32
                                                                              <<
                                                                              20i32))
                                                                        as
                                                                        libc::c_uint)
                as libc::c_ulong
        } else {
            (1u64 <<
                 (::std::mem::size_of::<size_t>() as
                      libc::c_ulong).wrapping_mul(8i32 as
                                                      libc::c_ulong).wrapping_sub(31i32
                                                                                      as
                                                                                      libc::c_ulong))
                as size_t
        };
    requiredMem =
        (requiredMem >> 26i32).wrapping_add(1i32 as libc::c_ulong) << 26i32;
    requiredMem =
        (requiredMem as libc::c_ulong).wrapping_add(step) as U64 as U64;
    if requiredMem > maxMemory { requiredMem = maxMemory }
    loop  {
        testmem = malloc(requiredMem) as *mut BYTE;
        requiredMem =
            (requiredMem as libc::c_ulong).wrapping_sub(step) as U64 as U64;
        if !(testmem.is_null() && requiredMem > 0i32 as libc::c_ulong) {
            break ;
        }
    }
    free(testmem as *mut libc::c_void);
    return requiredMem;
}
/* ! BMK_syntheticTest() -- called from zstdcli */
/*  Generates a sample with datagen, using compressibility argument */
/*  cLevel - compression level to benchmark, errors if invalid
 *  compressibility - determines compressibility of sample
 *  compressionParams - basic compression Parameters
 *  displayLevel - see benchFiles
 *  adv - see advanced_Params_t
 * @return:
 *      a variant, which expresses either an error, or a valid result.
 *      Use BMK_isSuccessful_benchOutcome() to check if function was successful.
 *      If yes, extract the valid result with BMK_extract_benchResult(),
 *      it will contain :
 *          .cSpeed: compression speed in bytes per second,
 *          .dSpeed: decompression speed in bytes per second,
 *          .cSize : compressed size, in bytes
 *          .cMem  : memory budget required for the compression context
 */
#[no_mangle]
pub unsafe extern "C" fn BMK_syntheticTest(mut cLevel: libc::c_int,
                                           mut compressibility:
                                               libc::c_double,
                                           mut compressionParams:
                                               *const ZSTD_compressionParameters,
                                           mut displayLevel: libc::c_int,
                                           mut adv:
                                               *const BMK_advancedParams_t)
 -> BMK_benchOutcome_t {
    let mut name: [libc::c_char; 20] =
        [0i32 as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0];
    let benchedSize: size_t = 10000000i32 as size_t;
    let mut srcBuffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut res: BMK_benchOutcome_t =
        BMK_benchOutcome_t{internal_never_use_directly:
                               BMK_benchResult_t{cSize: 0,
                                                 cSpeed: 0,
                                                 dSpeed: 0,
                                                 cMem: 0,},
                           tag: 0,};
    if cLevel > ZSTD_maxCLevel() {
        let mut r: BMK_benchOutcome_t =
            BMK_benchOutcome_t{internal_never_use_directly:
                                   BMK_benchResult_t{cSize: 0,
                                                     cSpeed: 0,
                                                     dSpeed: 0,
                                                     cMem: 0,},
                               tag: 0,};
        memset(&mut r as *mut BMK_benchOutcome_t as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong);
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    15i32);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Invalid Compression Level\x00" as *const u8 as
                        *const libc::c_char);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        r.tag = 15i32;
        return r
    }
    srcBuffer = malloc(benchedSize);
    if srcBuffer.is_null() {
        let mut r_0: BMK_benchOutcome_t =
            BMK_benchOutcome_t{internal_never_use_directly:
                                   BMK_benchResult_t{cSize: 0,
                                                     cSpeed: 0,
                                                     dSpeed: 0,
                                                     cMem: 0,},
                               tag: 0,};
        memset(&mut r_0 as *mut BMK_benchOutcome_t as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<BMK_benchOutcome_t>() as libc::c_ulong);
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    21i32);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"not enough memory\x00" as *const u8 as
                        *const libc::c_char);
        }
        if displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        r_0.tag = 21i32;
        return r_0
    }
    RDG_genBuffer(srcBuffer, benchedSize, compressibility, 0.0f64,
                  0i32 as libc::c_uint);
    snprintf(name.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
             b"Synthetic %2u%%\x00" as *const u8 as *const libc::c_char,
             (compressibility * 100i32 as libc::c_double) as libc::c_uint);
    res =
        BMK_benchCLevel(srcBuffer, benchedSize, &benchedSize,
                        1i32 as libc::c_uint, cLevel, compressionParams,
                        0 as *const libc::c_void, 0i32 as size_t,
                        displayLevel, name.as_mut_ptr(), adv);
    free(srcBuffer);
    return res;
}
/* ===  Benchmark Zstandard in a memory-to-memory scenario  === */
/* * BMK_benchMem() -- core benchmarking function, called in paramgrill
 *  applies ZSTD_compress_generic() and ZSTD_decompress_generic() on data in srcBuffer
 *  with specific compression parameters provided by other arguments using benchFunction
 *  (cLevel, comprParams + adv in advanced Mode) */
/*  srcBuffer - data source, expected to be valid compressed data if in Decode Only Mode
 *  srcSize - size of data in srcBuffer
 *  fileSizes - srcBuffer is considered cut into 1+ segments, to compress separately.
 *              note : sum(fileSizes) must be == srcSize.  (<== ensure it's properly checked)
 *  nbFiles - nb of segments
 *  cLevel - compression level
 *  comprParams - basic compression parameters
 *  dictBuffer - a dictionary if used, null otherwise
 *  dictBufferSize - size of dictBuffer, 0 otherwise
 *  diplayLevel - see BMK_benchFiles
 *  displayName - name used by display
 * @return:
 *      a variant, which expresses either an error, or a valid result.
 *      Use BMK_isSuccessful_benchOutcome() to check if function was successful.
 *      If yes, extract the valid result with BMK_extract_benchResult(),
 *      it will contain :
 *          .cSpeed: compression speed in bytes per second,
 *          .dSpeed: decompression speed in bytes per second,
 *          .cSize : compressed size, in bytes
 *          .cMem  : memory budget required for the compression context
 */
#[no_mangle]
pub unsafe extern "C" fn BMK_benchMem(mut srcBuffer: *const libc::c_void,
                                      mut srcSize: size_t,
                                      mut fileSizes: *const size_t,
                                      mut nbFiles: libc::c_uint,
                                      mut cLevel: libc::c_int,
                                      mut comprParams:
                                          *const ZSTD_compressionParameters,
                                      mut dictBuffer: *const libc::c_void,
                                      mut dictBufferSize: size_t,
                                      mut displayLevel: libc::c_int,
                                      mut displayName: *const libc::c_char)
 -> BMK_benchOutcome_t {
    let adv: BMK_advancedParams_t = BMK_initAdvancedParams();
    return BMK_benchMemAdvanced(srcBuffer, srcSize, 0 as *mut libc::c_void,
                                0i32 as size_t, fileSizes, nbFiles, cLevel,
                                comprParams, dictBuffer, dictBufferSize,
                                displayLevel, displayName, &adv);
}