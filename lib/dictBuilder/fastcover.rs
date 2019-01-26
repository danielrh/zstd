#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc)]
extern crate libc;
extern "C" {
    /*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
    /* size_t */
    /* ZSTD_customMem */
    pub type POOL_ctx_s;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn clock() -> clock_t;
    /* ! POOL_create() :
 *  Create a thread pool with at most `numThreads` threads.
 * `numThreads` must be at least 1.
 *  The maximum number of queued jobs before blocking is `queueSize`.
 * @return : POOL_ctx pointer on success, else NULL.
*/
    #[no_mangle]
    fn POOL_create(numThreads: size_t, queueSize: size_t) -> *mut POOL_ctx;
    /* ! POOL_free() :
 *  Free a thread pool returned by POOL_create().
 */
    #[no_mangle]
    fn POOL_free(ctx: *mut POOL_ctx);
    /* ! POOL_add() :
 *  Add the job `function(opaque)` to the thread pool. `ctx` must be valid.
 *  Possibly blocks until there is room in the queue.
 *  Note : The function may be executed asynchronously,
 *         therefore, `opaque` must live until function has been completed.
 */
    #[no_mangle]
    fn POOL_add(ctx: *mut POOL_ctx, function: POOL_function,
                opaque: *mut libc::c_void);
    #[no_mangle]
    fn ZDICT_isError(errorCode: size_t) -> libc::c_uint;
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
    fn ZDICT_finalizeDictionary(dictBuffer: *mut libc::c_void,
                                dictBufferCapacity: size_t,
                                dictContent: *const libc::c_void,
                                dictContentSize: size_t,
                                samplesBuffer: *const libc::c_void,
                                samplesSizes: *const size_t,
                                nbSamples: libc::c_uint,
                                parameters: ZDICT_params_t) -> size_t;
    /* *
 * Returns the sum of the sample sizes.
 */
    #[no_mangle]
    fn COVER_sum(samplesSizes: *const size_t, nbSamples: libc::c_uint)
     -> size_t;
    /* *
 * Call COVER_best_wait() and then destroy the COVER_best_t.
 */
    #[no_mangle]
    fn COVER_best_destroy(best: *mut COVER_best_t);
    /* *
 * Wait until liveJobs == 0.
 */
    #[no_mangle]
    fn COVER_best_wait(best: *mut COVER_best_t);
    /* *
 * Called when a thread finishes executing, both on error or success.
 * Decrements liveJobs and signals any waiting threads if liveJobs == 0.
 * If this dictionary is the best so far save it and its parameters.
 */
    #[no_mangle]
    fn COVER_best_finish(best: *mut COVER_best_t, compressedSize: size_t,
                         parameters: ZDICT_cover_params_t,
                         dict: *mut libc::c_void, dictSize: size_t);
    /* *
 *  Checks total compressed size of a dictionary
 */
    #[no_mangle]
    fn COVER_checkTotalCompressedSize(parameters: ZDICT_cover_params_t,
                                      samplesSizes: *const size_t,
                                      samples: *const BYTE,
                                      offsets: *mut size_t,
                                      nbTrainSamples: size_t,
                                      nbSamples: size_t, dict: *mut BYTE,
                                      dictBufferCapacity: size_t) -> size_t;
    /* *
 * Called when a thread is about to be launched.
 * Increments liveJobs.
 */
    #[no_mangle]
    fn COVER_best_start(best: *mut COVER_best_t);
    /* *
 * Initialize the `COVER_best_t`.
 */
    #[no_mangle]
    fn COVER_best_init(best: *mut COVER_best_t);
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
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct unalign64 {
    pub v: U64,
}
pub type POOL_ctx = POOL_ctx_s;
/* ! POOL_function :
 *  The function type that can be added to a thread pool.
 */
pub type POOL_function
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __pthread_cond_s {
    pub unnamed: unnamed_2,
    pub unnamed_0: unnamed_0,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: unnamed_1,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_1 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: unnamed_3,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_3 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
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
pub type unnamed_4 = libc::c_uint;
/* never EVER use this value directly, it can change in future versions! Use ZSTD_isError() instead */
pub const ZSTD_error_maxCode: unnamed_4 = 120;
pub const ZSTD_error_seekableIO: unnamed_4 = 102;
/* following error codes are __NOT STABLE__, they can be removed or changed in future versions */
pub const ZSTD_error_frameIndex_tooLarge: unnamed_4 = 100;
pub const ZSTD_error_dstBuffer_null: unnamed_4 = 74;
pub const ZSTD_error_srcSize_wrong: unnamed_4 = 72;
pub const ZSTD_error_dstSize_tooSmall: unnamed_4 = 70;
pub const ZSTD_error_workSpace_tooSmall: unnamed_4 = 66;
pub const ZSTD_error_memory_allocation: unnamed_4 = 64;
pub const ZSTD_error_init_missing: unnamed_4 = 62;
pub const ZSTD_error_stage_wrong: unnamed_4 = 60;
pub const ZSTD_error_maxSymbolValue_tooSmall: unnamed_4 = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: unnamed_4 = 46;
pub const ZSTD_error_tableLog_tooLarge: unnamed_4 = 44;
pub const ZSTD_error_parameter_outOfBound: unnamed_4 = 42;
pub const ZSTD_error_parameter_unsupported: unnamed_4 = 40;
pub const ZSTD_error_dictionaryCreation_failed: unnamed_4 = 34;
pub const ZSTD_error_dictionary_wrong: unnamed_4 = 32;
pub const ZSTD_error_dictionary_corrupted: unnamed_4 = 30;
pub const ZSTD_error_checksum_wrong: unnamed_4 = 22;
pub const ZSTD_error_corruption_detected: unnamed_4 = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: unnamed_4 = 16;
pub const ZSTD_error_frameParameter_unsupported: unnamed_4 = 14;
pub const ZSTD_error_version_unsupported: unnamed_4 = 12;
pub const ZSTD_error_prefix_unknown: unnamed_4 = 10;
pub const ZSTD_error_GENERIC: unnamed_4 = 1;
pub const ZSTD_error_no_error: unnamed_4 = 0;
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
/* ! ZDICT_cover_params_t:
 *  k and d are the only required parameters.
 *  For others, value 0 means default.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZDICT_cover_params_t {
    pub k: libc::c_uint,
    pub d: libc::c_uint,
    pub steps: libc::c_uint,
    pub nbThreads: libc::c_uint,
    pub splitPoint: libc::c_double,
    pub zParams: ZDICT_params_t,
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
/*-*************************************
* Acceleration
***************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FASTCOVER_accel_t {
    pub finalize: libc::c_uint,
    pub skip: libc::c_uint,
}
/* accel = 0, should not happen because accel = 0 defaults to accel = 1 */
/* accel = 1 */
/* accel = 2 */
/* accel = 3 */
/* accel = 4 */
/* accel = 5 */
/* accel = 6 */
/* accel = 7 */
/* accel = 8 */
/* accel = 9 */
/* accel = 10 */
/*-*************************************
* Context
***************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FASTCOVER_ctx_t {
    pub samples: *const BYTE,
    pub offsets: *mut size_t,
    pub samplesSizes: *const size_t,
    pub nbSamples: size_t,
    pub nbTrainSamples: size_t,
    pub nbTestSamples: size_t,
    pub nbDmers: size_t,
    pub freqs: *mut U32,
    pub d: libc::c_uint,
    pub f: libc::c_uint,
    pub accelParams: FASTCOVER_accel_t,
}
/* *
 * A segment is a range in the source as well as the score of the segment.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct COVER_segment_t {
    pub begin: U32,
    pub end: U32,
    pub score: U32,
}
pub type COVER_best_t = COVER_best_s;
/* fprintf */
/* malloc, free, qsort */
/* memset */
/* clock */
/* read */
/* *
 * COVER_best_t is used for two purposes:
 * 1. Synchronizing threads.
 * 2. Saving the best parameters and dictionary.
 *
 * All of the methods except COVER_best_init() are thread safe if zstd is
 * compiled with multithreaded support.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct COVER_best_s {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub liveJobs: size_t,
    pub dict: *mut libc::c_void,
    pub dictSize: size_t,
    pub parameters: ZDICT_cover_params_t,
    pub compressedSize: size_t,
}
pub type FASTCOVER_tryParameters_data_t = FASTCOVER_tryParameters_data_s;
/* *
 * Parameters for FASTCOVER_tryParameters().
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FASTCOVER_tryParameters_data_s {
    pub ctx: *const FASTCOVER_ctx_t,
    pub best: *mut COVER_best_t,
    pub dictBufferCapacity: size_t,
    pub parameters: ZDICT_cover_params_t,
}
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    /* don't use static : performance detrimental  */
    let one: unnamed = unnamed{u: 1i32 as U32,};
    return one.c[0usize] as libc::c_uint;
}
unsafe extern "C" fn MEM_read64(mut ptr: *const libc::c_void) -> U64 {
    return (*(ptr as *const unalign64)).v;
}
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return (in_0 as libc::c_ulonglong).swap_bytes() as U64;
}
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const libc::c_void) -> U64 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read64(memPtr)
    } else { return MEM_swap64(MEM_read64(memPtr)) };
}
/*-****************************************
*  Error codes handling
******************************************/
/* reported already defined on VS 2015 (Rich Geldreich) */
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as
               libc::c_int as libc::c_uint;
}
/* ! ZDICT_trainFromBuffer_fastCover():
 *  Train a dictionary from an array of samples using a modified version of COVER algorithm.
 *  Samples must be stored concatenated in a single flat buffer `samplesBuffer`,
 *  supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order.
 *  d and k are required.
 *  All other parameters are optional, will use default values if not provided
 *  The resulting dictionary will be saved into `dictBuffer`.
 * @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)
 *          or an error code, which can be tested with ZDICT_isError().
 *  Note: ZDICT_trainFromBuffer_fastCover() requires about 1 bytes of memory for each input byte and additionally another 6 * 2^f bytes of memory .
 *  Tips: In general, a reasonable dictionary has a size of ~ 100 KB.
 *        It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`.
 *        In general, it's recommended to provide a few thousands samples, though this can vary a lot.
 *        It's recommended that total size of all samples be about ~x100 times the target size of dictionary.
 */
#[no_mangle]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_fastCover(mut dictBuffer:
                                                             *mut libc::c_void,
                                                         mut dictBufferCapacity:
                                                             size_t,
                                                         mut samplesBuffer:
                                                             *const libc::c_void,
                                                         mut samplesSizes:
                                                             *const size_t,
                                                         mut nbSamples:
                                                             libc::c_uint,
                                                         mut parameters:
                                                             ZDICT_fastCover_params_t)
 -> size_t {
    let dict: *mut BYTE = dictBuffer as *mut BYTE;
    let mut ctx: FASTCOVER_ctx_t =
        FASTCOVER_ctx_t{samples: 0 as *const BYTE,
                        offsets: 0 as *mut size_t,
                        samplesSizes: 0 as *const size_t,
                        nbSamples: 0,
                        nbTrainSamples: 0,
                        nbTestSamples: 0,
                        nbDmers: 0,
                        freqs: 0 as *mut U32,
                        d: 0,
                        f: 0,
                        accelParams:
                            FASTCOVER_accel_t{finalize: 0, skip: 0,},};
    let mut coverParams: ZDICT_cover_params_t =
        ZDICT_cover_params_t{k: 0,
                             d: 0,
                             steps: 0,
                             nbThreads: 0,
                             splitPoint: 0.,
                             zParams:
                                 ZDICT_params_t{compressionLevel: 0,
                                                notificationLevel: 0,
                                                dictID: 0,},};
    let mut accelParams: FASTCOVER_accel_t =
        FASTCOVER_accel_t{finalize: 0, skip: 0,};
    g_displayLevel = parameters.zParams.notificationLevel as libc::c_int;
    parameters.splitPoint = 1.0f64;
    parameters.f =
        if parameters.f == 0i32 as libc::c_uint {
            20i32 as libc::c_uint
        } else { parameters.f };
    parameters.accel =
        if parameters.accel == 0i32 as libc::c_uint {
            1i32 as libc::c_uint
        } else { parameters.accel };
    memset(&mut coverParams as *mut ZDICT_cover_params_t as *mut libc::c_void,
           0i32,
           ::std::mem::size_of::<ZDICT_cover_params_t>() as libc::c_ulong);
    FASTCOVER_convertToCoverParams(parameters, &mut coverParams);
    if 0 ==
           FASTCOVER_checkParameters(coverParams, dictBufferCapacity,
                                     parameters.f, parameters.accel) {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"FASTCOVER parameters incorrect\n\x00" as *const u8 as
                        *const libc::c_char);
            fflush(stderr);
        }
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if nbSamples == 0i32 as libc::c_uint {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"FASTCOVER must have at least one input file\n\x00" as
                        *const u8 as *const libc::c_char);
            fflush(stderr);
        }
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if dictBufferCapacity < 256i32 as libc::c_ulong {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"dictBufferCapacity must be at least %u\n\x00" as
                        *const u8 as *const libc::c_char, 256i32);
            fflush(stderr);
        }
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    accelParams = FASTCOVER_defaultAccelParameters[parameters.accel as usize];
    if 0 ==
           FASTCOVER_ctx_init(&mut ctx, samplesBuffer, samplesSizes,
                              nbSamples, coverParams.d, parameters.splitPoint,
                              parameters.f, accelParams) {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Failed to initialize context\n\x00" as *const u8 as
                        *const libc::c_char);
            fflush(stderr);
        }
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if g_displayLevel >= 2i32 {
        fprintf(stderr,
                b"Building dictionary\n\x00" as *const u8 as
                    *const libc::c_char);
        fflush(stderr);
    }
    let mut segmentFreqs: *mut U16 =
        calloc((1i32 as U64) << parameters.f,
               ::std::mem::size_of::<U16>() as libc::c_ulong) as *mut U16;
    let tail: size_t =
        FASTCOVER_buildDictionary(&mut ctx, ctx.freqs, dictBuffer,
                                  dictBufferCapacity, coverParams,
                                  segmentFreqs);
    let nbFinalizeSamples: libc::c_uint =
        ctx.nbTrainSamples.wrapping_mul(ctx.accelParams.finalize as
                                            libc::c_ulong).wrapping_div(100i32
                                                                            as
                                                                            libc::c_ulong)
            as libc::c_uint;
    let dictionarySize: size_t =
        ZDICT_finalizeDictionary(dict as *mut libc::c_void,
                                 dictBufferCapacity,
                                 dict.offset(tail as isize) as
                                     *const libc::c_void,
                                 dictBufferCapacity.wrapping_sub(tail),
                                 samplesBuffer, samplesSizes,
                                 nbFinalizeSamples, coverParams.zParams);
    if 0 == ERR_isError(dictionarySize) {
        if g_displayLevel >= 2i32 {
            fprintf(stderr,
                    b"Constructed dictionary of size %u\n\x00" as *const u8 as
                        *const libc::c_char, dictionarySize as libc::c_uint);
            fflush(stderr);
        }
    }
    FASTCOVER_ctx_destroy(&mut ctx);
    free(segmentFreqs as *mut libc::c_void);
    return dictionarySize;
}
/* *
 * Given the prepared context build the dictionary.
 */
unsafe extern "C" fn FASTCOVER_buildDictionary(mut ctx:
                                                   *const FASTCOVER_ctx_t,
                                               mut freqs: *mut U32,
                                               mut dictBuffer:
                                                   *mut libc::c_void,
                                               mut dictBufferCapacity: size_t,
                                               mut parameters:
                                                   ZDICT_cover_params_t,
                                               mut segmentFreqs: *mut U16)
 -> size_t {
    let dict: *mut BYTE = dictBuffer as *mut BYTE;
    let mut tail: size_t = dictBufferCapacity;
    /* Divide the data up into epochs of equal size.
   * We will select at least one segment from each epoch.
   */
    let epochs: libc::c_uint =
        if 1i32 as libc::c_uint >
               dictBufferCapacity.wrapping_div(parameters.k as libc::c_ulong)
                   as U32 {
            1i32 as libc::c_uint
        } else {
            dictBufferCapacity.wrapping_div(parameters.k as libc::c_ulong) as
                U32
        };
    let epochSize: libc::c_uint =
        (*ctx).nbDmers.wrapping_div(epochs as libc::c_ulong) as U32;
    let mut epoch: size_t = 0;
    if g_displayLevel >= 2i32 {
        fprintf(stderr,
                b"Breaking content into %u epochs of size %u\n\x00" as
                    *const u8 as *const libc::c_char, epochs, epochSize);
        fflush(stderr);
    }
    epoch = 0i32 as size_t;
    while tail > 0i32 as libc::c_ulong {
        let epochBegin: U32 =
            epoch.wrapping_mul(epochSize as libc::c_ulong) as U32;
        let epochEnd: U32 = epochBegin.wrapping_add(epochSize);
        let mut segmentSize: size_t = 0;
        /* Select a segment */
        let mut segment: COVER_segment_t =
            FASTCOVER_selectSegment(ctx, freqs, epochBegin, epochEnd,
                                    parameters, segmentFreqs);
        /* If the segment covers no dmers, then we are out of content */
        if segment.score == 0i32 as libc::c_uint { break ; }
        segmentSize =
            if (segment.end.wrapping_sub(segment.begin).wrapping_add(parameters.d).wrapping_sub(1i32
                                                                                                    as
                                                                                                    libc::c_uint)
                    as libc::c_ulong) < tail {
                segment.end.wrapping_sub(segment.begin).wrapping_add(parameters.d).wrapping_sub(1i32
                                                                                                    as
                                                                                                    libc::c_uint)
                    as libc::c_ulong
            } else { tail };
        if segmentSize < parameters.d as libc::c_ulong { break ; }
        tail =
            (tail as libc::c_ulong).wrapping_sub(segmentSize) as size_t as
                size_t;
        memcpy(dict.offset(tail as isize) as *mut libc::c_void,
               (*ctx).samples.offset(segment.begin as isize) as
                   *const libc::c_void, segmentSize);
        if g_displayLevel >= 2i32 {
            if clock() - g_time > refreshRate || g_displayLevel >= 4i32 {
                g_time = clock();
                fprintf(stderr,
                        b"\r%u%%       \x00" as *const u8 as
                            *const libc::c_char,
                        dictBufferCapacity.wrapping_sub(tail).wrapping_mul(100i32
                                                                               as
                                                                               libc::c_ulong).wrapping_div(dictBufferCapacity)
                            as libc::c_uint);
                fflush(stderr);
            }
        }
        epoch =
            epoch.wrapping_add(1i32 as
                                   libc::c_ulong).wrapping_rem(epochs as
                                                                   libc::c_ulong)
    }
    if g_displayLevel >= 2i32 {
        fprintf(stderr, b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char);
        fflush(stderr);
    }
    return tail;
}
/*-*************************************
*  Dependencies
***************************************/
/* fprintf */
/* malloc, free, qsort */
/* memset */
/* clock */
/* includes zstd.h */
/*-*************************************
*  Constants
***************************************/
/*-*************************************
*  Console display
***************************************/
static mut g_displayLevel: libc::c_int = 2i32;
static mut g_time: clock_t = 0i32 as clock_t;
/* 0 : no display;   1: errors;   2: default;  3: details;  4: debug */
static mut refreshRate: clock_t =
    1000000i32 as __clock_t * 15i32 as libc::c_long / 100i32 as libc::c_long;
/*-*************************************
*  Helper functions
***************************************/
/**
 * Selects the best segment in an epoch.
 * Segments of are scored according to the function:
 *
 * Let F(d) be the frequency of all dmers with hash value d.
 * Let S_i be hash value of the dmer at position i of segment S which has length k.
 *
 *     Score(S) = F(S_1) + F(S_2) + ... + F(S_{k-d+1})
 *
 * Once the dmer with hash value d is in the dictionay we set F(d) = 0.
 */
unsafe extern "C" fn FASTCOVER_selectSegment(mut ctx: *const FASTCOVER_ctx_t,
                                             mut freqs: *mut U32,
                                             mut begin: U32, mut end: U32,
                                             mut parameters:
                                                 ZDICT_cover_params_t,
                                             mut segmentFreqs: *mut U16)
 -> COVER_segment_t {
    /* Constants */
    let k: U32 = parameters.k;
    let d: U32 = parameters.d;
    let f: U32 = (*ctx).f;
    let dmersInK: U32 = k.wrapping_sub(d).wrapping_add(1i32 as libc::c_uint);
    /* Try each segment (activeSegment) and save the best (bestSegment) */
    let mut bestSegment: COVER_segment_t =
        COVER_segment_t{begin: 0i32 as U32,
                        end: 0i32 as U32,
                        score: 0i32 as U32,};
    let mut activeSegment: COVER_segment_t =
        COVER_segment_t{begin: 0, end: 0, score: 0,};
    activeSegment.begin = begin;
    activeSegment.end = begin;
    activeSegment.score = 0i32 as U32;
    while activeSegment.end < end {
        let idx: size_t =
            FASTCOVER_hashPtrToIndex((*ctx).samples.offset(activeSegment.end
                                                               as isize) as
                                         *const libc::c_void, f, d);
        if *segmentFreqs.offset(idx as isize) as libc::c_int == 0i32 {
            activeSegment.score =
                (activeSegment.score as
                     libc::c_uint).wrapping_add(*freqs.offset(idx as isize))
                    as U32 as U32
        }
        activeSegment.end =
            (activeSegment.end as
                 libc::c_uint).wrapping_add(1i32 as libc::c_uint) as U32 as
                U32;
        let ref mut fresh0 = *segmentFreqs.offset(idx as isize);
        *fresh0 = (*fresh0 as libc::c_int + 1i32) as U16;
        if activeSegment.end.wrapping_sub(activeSegment.begin) ==
               dmersInK.wrapping_add(1i32 as libc::c_uint) {
            let delIndex: size_t =
                FASTCOVER_hashPtrToIndex((*ctx).samples.offset(activeSegment.begin
                                                                   as isize)
                                             as *const libc::c_void, f, d);
            let ref mut fresh1 = *segmentFreqs.offset(delIndex as isize);
            *fresh1 = (*fresh1 as libc::c_int - 1i32) as U16;
            if *segmentFreqs.offset(delIndex as isize) as libc::c_int == 0i32
               {
                activeSegment.score =
                    (activeSegment.score as
                         libc::c_uint).wrapping_sub(*freqs.offset(delIndex as
                                                                      isize))
                        as U32 as U32
            }
            activeSegment.begin =
                (activeSegment.begin as
                     libc::c_uint).wrapping_add(1i32 as libc::c_uint) as U32
                    as U32
        }
        if activeSegment.score > bestSegment.score {
            bestSegment = activeSegment
        }
    }
    while activeSegment.begin < end {
        let delIndex_0: size_t =
            FASTCOVER_hashPtrToIndex((*ctx).samples.offset(activeSegment.begin
                                                               as isize) as
                                         *const libc::c_void, f, d);
        let ref mut fresh2 = *segmentFreqs.offset(delIndex_0 as isize);
        *fresh2 = (*fresh2 as libc::c_int - 1i32) as U16;
        activeSegment.begin =
            (activeSegment.begin as
                 libc::c_uint).wrapping_add(1i32 as libc::c_uint) as U32 as
                U32
    }
    let mut pos: U32 = 0;
    pos = bestSegment.begin;
    while pos != bestSegment.end {
        let i: size_t =
            FASTCOVER_hashPtrToIndex((*ctx).samples.offset(pos as isize) as
                                         *const libc::c_void, f, d);
        *freqs.offset(i as isize) = 0i32 as U32;
        pos = pos.wrapping_add(1)
    }
    return bestSegment;
}
/* *
 * Hash the d-byte value pointed to by p and mod 2^f
 */
unsafe extern "C" fn FASTCOVER_hashPtrToIndex(mut p: *const libc::c_void,
                                              mut h: U32, mut d: libc::c_uint)
 -> size_t {
    if d == 6i32 as libc::c_uint {
        return ZSTD_hash6Ptr(p, h) & ((1i32 << h) - 1i32) as libc::c_ulong
    }
    return ZSTD_hash8Ptr(p, h) & ((1i32 << h) - 1i32) as libc::c_ulong;
}
unsafe extern "C" fn ZSTD_hash8Ptr(mut p: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash8(MEM_readLE64(p), h);
}
unsafe extern "C" fn ZSTD_hash8(mut u: U64, mut h: U32) -> size_t {
    return u.wrapping_mul(prime8bytes) >>
               (64i32 as libc::c_uint).wrapping_sub(h);
}
static mut prime8bytes: U64 = 0xcf1bbcdcb7a56463u64 as U64;
unsafe extern "C" fn ZSTD_hash6Ptr(mut p: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash6(MEM_readLE64(p), h);
}
unsafe extern "C" fn ZSTD_hash6(mut u: U64, mut h: U32) -> size_t {
    return (u << 64i32 - 48i32).wrapping_mul(prime6bytes) >>
               (64i32 as libc::c_uint).wrapping_sub(h);
}
/*-*************************************
* Hash Functions
***************************************/
static mut prime6bytes: U64 = 227718039650203u64 as U64;
/* *
 * Clean up a context initialized with `FASTCOVER_ctx_init()`.
 */
unsafe extern "C" fn FASTCOVER_ctx_destroy(mut ctx: *mut FASTCOVER_ctx_t) {
    if ctx.is_null() { return }
    free((*ctx).freqs as *mut libc::c_void);
    (*ctx).freqs = 0 as *mut U32;
    free((*ctx).offsets as *mut libc::c_void);
    (*ctx).offsets = 0 as *mut size_t;
}
/* *
 * Prepare a context for dictionary building.
 * The context is only dependent on the parameter `d` and can used multiple
 * times.
 * Returns 1 on success or zero on error.
 * The context must be destroyed with `FASTCOVER_ctx_destroy()`.
 */
unsafe extern "C" fn FASTCOVER_ctx_init(mut ctx: *mut FASTCOVER_ctx_t,
                                        mut samplesBuffer:
                                            *const libc::c_void,
                                        mut samplesSizes: *const size_t,
                                        mut nbSamples: libc::c_uint,
                                        mut d: libc::c_uint,
                                        mut splitPoint: libc::c_double,
                                        mut f: libc::c_uint,
                                        mut accelParams: FASTCOVER_accel_t)
 -> libc::c_int {
    let samples: *const BYTE = samplesBuffer as *const BYTE;
    let totalSamplesSize: size_t = COVER_sum(samplesSizes, nbSamples);
    /* Split samples into testing and training sets */
    let nbTrainSamples: libc::c_uint =
        if splitPoint < 1.0f64 {
            (nbSamples as libc::c_double * splitPoint) as libc::c_uint
        } else { nbSamples };
    let nbTestSamples: libc::c_uint =
        if splitPoint < 1.0f64 {
            nbSamples.wrapping_sub(nbTrainSamples)
        } else { nbSamples };
    let trainingSamplesSize: size_t =
        if splitPoint < 1.0f64 {
            COVER_sum(samplesSizes, nbTrainSamples)
        } else { totalSamplesSize };
    let testSamplesSize: size_t =
        if splitPoint < 1.0f64 {
            COVER_sum(samplesSizes.offset(nbTrainSamples as isize),
                      nbTestSamples)
        } else { totalSamplesSize };
    if totalSamplesSize <
           if d as libc::c_ulong >
                  ::std::mem::size_of::<U64>() as libc::c_ulong {
               d as libc::c_ulong
           } else { ::std::mem::size_of::<U64>() as libc::c_ulong } ||
           totalSamplesSize >=
               (if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                       8i32 as libc::c_ulong {
                    -1i32 as libc::c_uint
                } else { (1i32 as libc::c_uint).wrapping_mul(1u32 << 30i32) })
                   as size_t {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Total samples size is too large (%u MB), maximum size is %u MB\n\x00"
                        as *const u8 as *const libc::c_char,
                    (totalSamplesSize >> 20i32) as libc::c_uint,
                    if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                           8i32 as libc::c_ulong {
                        -1i32 as libc::c_uint
                    } else {
                        (1i32 as libc::c_uint).wrapping_mul(1u32 << 30i32)
                    } >> 20i32);
            fflush(stderr);
        }
        return 0i32
    }
    if nbTrainSamples < 5i32 as libc::c_uint {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Total number of training samples is %u and is invalid\n\x00"
                        as *const u8 as *const libc::c_char, nbTrainSamples);
            fflush(stderr);
        }
        return 0i32
    }
    if nbTestSamples < 1i32 as libc::c_uint {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Total number of testing samples is %u and is invalid.\n\x00"
                        as *const u8 as *const libc::c_char, nbTestSamples);
            fflush(stderr);
        }
        return 0i32
    }
    memset(ctx as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<FASTCOVER_ctx_t>() as libc::c_ulong);
    if g_displayLevel >= 2i32 {
        fprintf(stderr,
                b"Training on %u samples of total size %u\n\x00" as *const u8
                    as *const libc::c_char, nbTrainSamples,
                trainingSamplesSize as libc::c_uint);
        fflush(stderr);
    }
    if g_displayLevel >= 2i32 {
        fprintf(stderr,
                b"Testing on %u samples of total size %u\n\x00" as *const u8
                    as *const libc::c_char, nbTestSamples,
                testSamplesSize as libc::c_uint);
        fflush(stderr);
    }
    (*ctx).samples = samples;
    (*ctx).samplesSizes = samplesSizes;
    (*ctx).nbSamples = nbSamples as size_t;
    (*ctx).nbTrainSamples = nbTrainSamples as size_t;
    (*ctx).nbTestSamples = nbTestSamples as size_t;
    (*ctx).nbDmers =
        trainingSamplesSize.wrapping_sub(if d as libc::c_ulong >
                                                ::std::mem::size_of::<U64>()
                                                    as libc::c_ulong {
                                             d as libc::c_ulong
                                         } else {
                                             ::std::mem::size_of::<U64>() as
                                                 libc::c_ulong
                                         }).wrapping_add(1i32 as
                                                             libc::c_ulong);
    (*ctx).d = d;
    (*ctx).f = f;
    (*ctx).accelParams = accelParams;
    (*ctx).offsets =
        calloc(nbSamples.wrapping_add(1i32 as libc::c_uint) as libc::c_ulong,
               ::std::mem::size_of::<size_t>() as libc::c_ulong) as
            *mut size_t;
    if (*ctx).offsets.is_null() {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Failed to allocate scratch buffers \n\x00" as *const u8
                        as *const libc::c_char);
            fflush(stderr);
        }
        FASTCOVER_ctx_destroy(ctx);
        return 0i32
    }
    let mut i: U32 = 0;
    *(*ctx).offsets.offset(0isize) = 0i32 as size_t;
    i = 1i32 as U32;
    while i <= nbSamples {
        *(*ctx).offsets.offset(i as isize) =
            (*(*ctx).offsets.offset(i.wrapping_sub(1i32 as libc::c_uint) as
                                        isize)).wrapping_add(*samplesSizes.offset(i.wrapping_sub(1i32
                                                                                                     as
                                                                                                     libc::c_uint)
                                                                                      as
                                                                                      isize));
        i = i.wrapping_add(1)
    }
    (*ctx).freqs =
        calloc((1i32 as U64) << f,
               ::std::mem::size_of::<U32>() as libc::c_ulong) as *mut U32;
    if (*ctx).freqs.is_null() {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Failed to allocate frequency table \n\x00" as *const u8
                        as *const libc::c_char);
            fflush(stderr);
        }
        FASTCOVER_ctx_destroy(ctx);
        return 0i32
    }
    if g_displayLevel >= 2i32 {
        fprintf(stderr,
                b"Computing frequencies\n\x00" as *const u8 as
                    *const libc::c_char);
        fflush(stderr);
    }
    FASTCOVER_computeFrequency((*ctx).freqs, ctx);
    return 1i32;
}
/* *
 * Calculate for frequency of hash value of each dmer in ctx->samples
 */
unsafe extern "C" fn FASTCOVER_computeFrequency(mut freqs: *mut U32,
                                                mut ctx:
                                                    *const FASTCOVER_ctx_t) {
    let f: libc::c_uint = (*ctx).f;
    let d: libc::c_uint = (*ctx).d;
    let skip: libc::c_uint = (*ctx).accelParams.skip;
    let readLength: libc::c_uint =
        if d > 8i32 as libc::c_uint { d } else { 8i32 as libc::c_uint };
    let mut i: size_t = 0;
    i = 0i32 as size_t;
    while i < (*ctx).nbTrainSamples {
        let mut start: size_t = *(*ctx).offsets.offset(i as isize);
        let currSampleEnd: size_t =
            *(*ctx).offsets.offset(i.wrapping_add(1i32 as libc::c_ulong) as
                                       isize);
        while start.wrapping_add(readLength as libc::c_ulong) <= currSampleEnd
              {
            let dmerIndex: size_t =
                FASTCOVER_hashPtrToIndex((*ctx).samples.offset(start as isize)
                                             as *const libc::c_void, f, d);
            let ref mut fresh3 = *freqs.offset(dmerIndex as isize);
            *fresh3 = (*fresh3).wrapping_add(1);
            start =
                start.wrapping_add(skip as
                                       libc::c_ulong).wrapping_add(1i32 as
                                                                       libc::c_ulong)
        }
        i = i.wrapping_add(1)
    };
}
static mut FASTCOVER_defaultAccelParameters: [FASTCOVER_accel_t; 11] =
    [FASTCOVER_accel_t{finalize: 100i32 as libc::c_uint,
                       skip: 0i32 as libc::c_uint,},
     FASTCOVER_accel_t{finalize: 100i32 as libc::c_uint,
                       skip: 0i32 as libc::c_uint,},
     FASTCOVER_accel_t{finalize: 50i32 as libc::c_uint,
                       skip: 1i32 as libc::c_uint,},
     FASTCOVER_accel_t{finalize: 34i32 as libc::c_uint,
                       skip: 2i32 as libc::c_uint,},
     FASTCOVER_accel_t{finalize: 25i32 as libc::c_uint,
                       skip: 3i32 as libc::c_uint,},
     FASTCOVER_accel_t{finalize: 20i32 as libc::c_uint,
                       skip: 4i32 as libc::c_uint,},
     FASTCOVER_accel_t{finalize: 17i32 as libc::c_uint,
                       skip: 5i32 as libc::c_uint,},
     FASTCOVER_accel_t{finalize: 14i32 as libc::c_uint,
                       skip: 6i32 as libc::c_uint,},
     FASTCOVER_accel_t{finalize: 13i32 as libc::c_uint,
                       skip: 7i32 as libc::c_uint,},
     FASTCOVER_accel_t{finalize: 11i32 as libc::c_uint,
                       skip: 8i32 as libc::c_uint,},
     FASTCOVER_accel_t{finalize: 10i32 as libc::c_uint,
                       skip: 9i32 as libc::c_uint,}];
unsafe extern "C" fn FASTCOVER_checkParameters(mut parameters:
                                                   ZDICT_cover_params_t,
                                               mut maxDictSize: size_t,
                                               mut f: libc::c_uint,
                                               mut accel: libc::c_uint)
 -> libc::c_int {
    if parameters.d == 0i32 as libc::c_uint ||
           parameters.k == 0i32 as libc::c_uint {
        return 0i32
    }
    if parameters.d != 6i32 as libc::c_uint &&
           parameters.d != 8i32 as libc::c_uint {
        return 0i32
    }
    if parameters.k as libc::c_ulong > maxDictSize { return 0i32 }
    if parameters.d > parameters.k { return 0i32 }
    if f > 31i32 as libc::c_uint || f == 0i32 as libc::c_uint { return 0i32 }
    if parameters.splitPoint <= 0i32 as libc::c_double ||
           parameters.splitPoint > 1i32 as libc::c_double {
        return 0i32
    }
    if accel > 10i32 as libc::c_uint || accel == 0i32 as libc::c_uint {
        return 0i32
    }
    return 1i32;
}
unsafe extern "C" fn FASTCOVER_convertToCoverParams(mut fastCoverParams:
                                                        ZDICT_fastCover_params_t,
                                                    mut coverParams:
                                                        *mut ZDICT_cover_params_t) {
    (*coverParams).k = fastCoverParams.k;
    (*coverParams).d = fastCoverParams.d;
    (*coverParams).steps = fastCoverParams.steps;
    (*coverParams).nbThreads = fastCoverParams.nbThreads;
    (*coverParams).splitPoint = fastCoverParams.splitPoint;
    (*coverParams).zParams = fastCoverParams.zParams;
}
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
pub unsafe extern "C" fn ZDICT_optimizeTrainFromBuffer_fastCover(mut dictBuffer:
                                                                     *mut libc::c_void,
                                                                 mut dictBufferCapacity:
                                                                     size_t,
                                                                 mut samplesBuffer:
                                                                     *const libc::c_void,
                                                                 mut samplesSizes:
                                                                     *const size_t,
                                                                 mut nbSamples:
                                                                     libc::c_uint,
                                                                 mut parameters:
                                                                     *mut ZDICT_fastCover_params_t)
 -> size_t {
    let mut coverParams: ZDICT_cover_params_t =
        ZDICT_cover_params_t{k: 0,
                             d: 0,
                             steps: 0,
                             nbThreads: 0,
                             splitPoint: 0.,
                             zParams:
                                 ZDICT_params_t{compressionLevel: 0,
                                                notificationLevel: 0,
                                                dictID: 0,},};
    let mut accelParams: FASTCOVER_accel_t =
        FASTCOVER_accel_t{finalize: 0, skip: 0,};
    /* constants */
    let nbThreads: libc::c_uint = (*parameters).nbThreads;
    let splitPoint: libc::c_double =
        if (*parameters).splitPoint <= 0.0f64 {
            0.75f64
        } else { (*parameters).splitPoint };
    let kMinD: libc::c_uint =
        if (*parameters).d == 0i32 as libc::c_uint {
            6i32 as libc::c_uint
        } else { (*parameters).d };
    let kMaxD: libc::c_uint =
        if (*parameters).d == 0i32 as libc::c_uint {
            8i32 as libc::c_uint
        } else { (*parameters).d };
    let kMinK: libc::c_uint =
        if (*parameters).k == 0i32 as libc::c_uint {
            50i32 as libc::c_uint
        } else { (*parameters).k };
    let kMaxK: libc::c_uint =
        if (*parameters).k == 0i32 as libc::c_uint {
            2000i32 as libc::c_uint
        } else { (*parameters).k };
    let kSteps: libc::c_uint =
        if (*parameters).steps == 0i32 as libc::c_uint {
            40i32 as libc::c_uint
        } else { (*parameters).steps };
    let kStepSize: libc::c_uint =
        if kMaxK.wrapping_sub(kMinK).wrapping_div(kSteps) >
               1i32 as libc::c_uint {
            kMaxK.wrapping_sub(kMinK).wrapping_div(kSteps)
        } else { 1i32 as libc::c_uint };
    let kIterations: libc::c_uint =
        (1i32 as
             libc::c_uint).wrapping_add(kMaxD.wrapping_sub(kMinD).wrapping_div(2i32
                                                                                   as
                                                                                   libc::c_uint)).wrapping_mul((1i32
                                                                                                                    as
                                                                                                                    libc::c_uint).wrapping_add(kMaxK.wrapping_sub(kMinK).wrapping_div(kStepSize)));
    let f: libc::c_uint =
        if (*parameters).f == 0i32 as libc::c_uint {
            20i32 as libc::c_uint
        } else { (*parameters).f };
    let accel: libc::c_uint =
        if (*parameters).accel == 0i32 as libc::c_uint {
            1i32 as libc::c_uint
        } else { (*parameters).accel };
    /* Local variables */
    let displayLevel: libc::c_int =
        (*parameters).zParams.notificationLevel as libc::c_int;
    let mut iteration: libc::c_uint = 1i32 as libc::c_uint;
    let mut d: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut best: COVER_best_t =
        COVER_best_s{mutex:
                         pthread_mutex_t{__data:
                                             __pthread_mutex_s{__lock: 0,
                                                               __count: 0,
                                                               __owner: 0,
                                                               __nusers: 0,
                                                               __kind: 0,
                                                               __spins: 0,
                                                               __elision: 0,
                                                               __list:
                                                                   __pthread_internal_list{__prev:
                                                                                               0
                                                                                                   as
                                                                                                   *mut __pthread_internal_list,
                                                                                           __next:
                                                                                               0
                                                                                                   as
                                                                                                   *mut __pthread_internal_list,},},},
                     cond:
                         pthread_cond_t{__data:
                                            __pthread_cond_s{unnamed:
                                                                 unnamed_2{__wseq:
                                                                               0,},
                                                             unnamed_0:
                                                                 unnamed_0{__g1_start:
                                                                               0,},
                                                             __g_refs: [0; 2],
                                                             __g_size: [0; 2],
                                                             __g1_orig_size:
                                                                 0,
                                                             __wrefs: 0,
                                                             __g_signals:
                                                                 [0; 2],},},
                     liveJobs: 0,
                     dict: 0 as *mut libc::c_void,
                     dictSize: 0,
                     parameters:
                         ZDICT_cover_params_t{k: 0,
                                              d: 0,
                                              steps: 0,
                                              nbThreads: 0,
                                              splitPoint: 0.,
                                              zParams:
                                                  ZDICT_params_t{compressionLevel:
                                                                     0,
                                                                 notificationLevel:
                                                                     0,
                                                                 dictID:
                                                                     0,},},
                     compressedSize: 0,};
    let mut pool: *mut POOL_ctx = 0 as *mut POOL_ctx;
    if splitPoint <= 0i32 as libc::c_double ||
           splitPoint > 1i32 as libc::c_double {
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Incorrect splitPoint\n\x00" as *const u8 as
                        *const libc::c_char);
            fflush(stderr);
        }
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if accel == 0i32 as libc::c_uint || accel > 10i32 as libc::c_uint {
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Incorrect accel\n\x00" as *const u8 as
                        *const libc::c_char);
            fflush(stderr);
        }
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if kMinK < kMaxD || kMaxK < kMinK {
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Incorrect k\n\x00" as *const u8 as *const libc::c_char);
            fflush(stderr);
        }
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if nbSamples == 0i32 as libc::c_uint {
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"FASTCOVER must have at least one input file\n\x00" as
                        *const u8 as *const libc::c_char);
            fflush(stderr);
        }
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if dictBufferCapacity < 256i32 as libc::c_ulong {
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"dictBufferCapacity must be at least %u\n\x00" as
                        *const u8 as *const libc::c_char, 256i32);
            fflush(stderr);
        }
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    if nbThreads > 1i32 as libc::c_uint {
        pool = POOL_create(nbThreads as size_t, 1i32 as size_t);
        if pool.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        }
    }
    COVER_best_init(&mut best);
    memset(&mut coverParams as *mut ZDICT_cover_params_t as *mut libc::c_void,
           0i32,
           ::std::mem::size_of::<ZDICT_cover_params_t>() as libc::c_ulong);
    FASTCOVER_convertToCoverParams(*parameters, &mut coverParams);
    accelParams = FASTCOVER_defaultAccelParameters[accel as usize];
    g_displayLevel =
        if displayLevel == 0i32 { 0i32 } else { displayLevel - 1i32 };
    if displayLevel >= 2i32 {
        fprintf(stderr,
                b"Trying %u different sets of parameters\n\x00" as *const u8
                    as *const libc::c_char, kIterations);
        fflush(stderr);
    }
    d = kMinD;
    while d <= kMaxD {
        let mut ctx: FASTCOVER_ctx_t =
            FASTCOVER_ctx_t{samples: 0 as *const BYTE,
                            offsets: 0 as *mut size_t,
                            samplesSizes: 0 as *const size_t,
                            nbSamples: 0,
                            nbTrainSamples: 0,
                            nbTestSamples: 0,
                            nbDmers: 0,
                            freqs: 0 as *mut U32,
                            d: 0,
                            f: 0,
                            accelParams:
                                FASTCOVER_accel_t{finalize: 0, skip: 0,},};
        if displayLevel >= 3i32 {
            fprintf(stderr, b"d=%u\n\x00" as *const u8 as *const libc::c_char,
                    d);
            fflush(stderr);
        }
        if 0 ==
               FASTCOVER_ctx_init(&mut ctx, samplesBuffer, samplesSizes,
                                  nbSamples, d, splitPoint, f, accelParams) {
            if displayLevel >= 1i32 {
                fprintf(stderr,
                        b"Failed to initialize context\n\x00" as *const u8 as
                            *const libc::c_char);
                fflush(stderr);
            }
            COVER_best_destroy(&mut best);
            POOL_free(pool);
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t
        }
        k = kMinK;
        while k <= kMaxK {
            /* Prepare the arguments */
            let mut data: *mut FASTCOVER_tryParameters_data_t =
                malloc(::std::mem::size_of::<FASTCOVER_tryParameters_data_t>()
                           as libc::c_ulong) as
                    *mut FASTCOVER_tryParameters_data_t;
            if displayLevel >= 3i32 {
                fprintf(stderr,
                        b"k=%u\n\x00" as *const u8 as *const libc::c_char, k);
                fflush(stderr);
            }
            if data.is_null() {
                if displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"Failed to allocate parameters\n\x00" as
                                *const u8 as *const libc::c_char);
                    fflush(stderr);
                }
                COVER_best_destroy(&mut best);
                FASTCOVER_ctx_destroy(&mut ctx);
                POOL_free(pool);
                return -(ZSTD_error_GENERIC as libc::c_int) as size_t
            }
            (*data).ctx = &mut ctx;
            (*data).best = &mut best;
            (*data).dictBufferCapacity = dictBufferCapacity;
            (*data).parameters = coverParams;
            (*data).parameters.k = k;
            (*data).parameters.d = d;
            (*data).parameters.splitPoint = splitPoint;
            (*data).parameters.steps = kSteps;
            (*data).parameters.zParams.notificationLevel =
                g_displayLevel as libc::c_uint;
            /* Check the parameters */
            if 0 ==
                   FASTCOVER_checkParameters((*data).parameters,
                                             dictBufferCapacity,
                                             (*(*data).ctx).f, accel) {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"FASTCOVER parameters incorrect\n\x00" as
                                *const u8 as *const libc::c_char);
                    fflush(stderr);
                }
                free(data as *mut libc::c_void);
            } else {
                COVER_best_start(&mut best);
                if !pool.is_null() {
                    POOL_add(pool, Some(FASTCOVER_tryParameters),
                             data as *mut libc::c_void);
                } else { FASTCOVER_tryParameters(data as *mut libc::c_void); }
                if displayLevel >= 2i32 {
                    if clock() - g_time > refreshRate || displayLevel >= 4i32
                       {
                        g_time = clock();
                        fprintf(stderr,
                                b"\r%u%%       \x00" as *const u8 as
                                    *const libc::c_char,
                                iteration.wrapping_mul(100i32 as
                                                           libc::c_uint).wrapping_div(kIterations));
                        fflush(stderr);
                    }
                }
                iteration = iteration.wrapping_add(1)
            }
            k = k.wrapping_add(kStepSize)
        }
        COVER_best_wait(&mut best);
        FASTCOVER_ctx_destroy(&mut ctx);
        d = d.wrapping_add(2i32 as libc::c_uint)
    }
    if displayLevel >= 2i32 {
        fprintf(stderr, b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char);
        fflush(stderr);
    }
    let dictSize: size_t = best.dictSize;
    if 0 != ERR_isError(best.compressedSize) {
        let compressedSize: size_t = best.compressedSize;
        COVER_best_destroy(&mut best);
        POOL_free(pool);
        return compressedSize
    }
    FASTCOVER_convertToFastCoverParams(best.parameters, parameters, f, accel);
    memcpy(dictBuffer, best.dict, dictSize);
    COVER_best_destroy(&mut best);
    POOL_free(pool);
    return dictSize;
}
unsafe extern "C" fn FASTCOVER_convertToFastCoverParams(mut coverParams:
                                                            ZDICT_cover_params_t,
                                                        mut fastCoverParams:
                                                            *mut ZDICT_fastCover_params_t,
                                                        mut f: libc::c_uint,
                                                        mut accel:
                                                            libc::c_uint) {
    (*fastCoverParams).k = coverParams.k;
    (*fastCoverParams).d = coverParams.d;
    (*fastCoverParams).steps = coverParams.steps;
    (*fastCoverParams).nbThreads = coverParams.nbThreads;
    (*fastCoverParams).splitPoint = coverParams.splitPoint;
    (*fastCoverParams).f = f;
    (*fastCoverParams).accel = accel;
    (*fastCoverParams).zParams = coverParams.zParams;
}
/* *
 * Tries a set of parameters and updates the COVER_best_t with the results.
 * This function is thread safe if zstd is compiled with multithreaded support.
 * It takes its parameters as an *OWNING* opaque pointer to support threading.
 */
unsafe extern "C" fn FASTCOVER_tryParameters(mut opaque: *mut libc::c_void) {
    /* Save parameters as local variables */
    let data: *mut FASTCOVER_tryParameters_data_t =
        opaque as *mut FASTCOVER_tryParameters_data_t;
    let ctx: *const FASTCOVER_ctx_t = (*data).ctx;
    let parameters: ZDICT_cover_params_t = (*data).parameters;
    let mut dictBufferCapacity: size_t = (*data).dictBufferCapacity;
    let mut totalCompressedSize: size_t =
        -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    /* Initialize array to keep track of frequency of dmer within activeSegment */
    let mut segmentFreqs: *mut U16 =
        calloc((1i32 as U64) << (*ctx).f,
               ::std::mem::size_of::<U16>() as libc::c_ulong) as *mut U16;
    /* Allocate space for hash table, dict, and freqs */
    let dict: *mut BYTE = malloc(dictBufferCapacity) as *mut BYTE;
    let mut freqs: *mut U32 =
        malloc(((1i32 as U64) <<
                    (*ctx).f).wrapping_mul(::std::mem::size_of::<U32>() as
                                               libc::c_ulong)) as *mut U32;
    if segmentFreqs.is_null() || dict.is_null() || freqs.is_null() {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Failed to allocate buffers: out of memory\n\x00" as
                        *const u8 as *const libc::c_char);
            fflush(stderr);
        }
    } else {
        memcpy(freqs as *mut libc::c_void,
               (*ctx).freqs as *const libc::c_void,
               ((1i32 as U64) <<
                    (*ctx).f).wrapping_mul(::std::mem::size_of::<U32>() as
                                               libc::c_ulong));
        /* Build the dictionary */
        let tail: size_t =
            FASTCOVER_buildDictionary(ctx, freqs, dict as *mut libc::c_void,
                                      dictBufferCapacity, parameters,
                                      segmentFreqs);
        let nbFinalizeSamples: libc::c_uint =
            (*ctx).nbTrainSamples.wrapping_mul((*ctx).accelParams.finalize as
                                                   libc::c_ulong).wrapping_div(100i32
                                                                                   as
                                                                                   libc::c_ulong)
                as libc::c_uint;
        dictBufferCapacity =
            ZDICT_finalizeDictionary(dict as *mut libc::c_void,
                                     dictBufferCapacity,
                                     dict.offset(tail as isize) as
                                         *const libc::c_void,
                                     dictBufferCapacity.wrapping_sub(tail),
                                     (*ctx).samples as *const libc::c_void,
                                     (*ctx).samplesSizes, nbFinalizeSamples,
                                     parameters.zParams);
        if 0 != ZDICT_isError(dictBufferCapacity) {
            if g_displayLevel >= 1i32 {
                fprintf(stderr,
                        b"Failed to finalize dictionary\n\x00" as *const u8 as
                            *const libc::c_char);
                fflush(stderr);
            }
        } else {
            totalCompressedSize =
                COVER_checkTotalCompressedSize(parameters,
                                               (*ctx).samplesSizes,
                                               (*ctx).samples, (*ctx).offsets,
                                               (*ctx).nbTrainSamples,
                                               (*ctx).nbSamples, dict,
                                               dictBufferCapacity)
        }
    }
    COVER_best_finish((*data).best, totalCompressedSize, parameters,
                      dict as *mut libc::c_void, dictBufferCapacity);
    free(data as *mut libc::c_void);
    free(segmentFreqs as *mut libc::c_void);
    free(dict as *mut libc::c_void);
    free(freqs as *mut libc::c_void);
}