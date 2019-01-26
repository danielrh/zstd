#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc, ptr_wrapping_offset_from)]
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
    /* **********************************
 *  Bulk processing dictionary API
 **********************************/
    pub type ZSTD_CDict_s;
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
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn clock() -> clock_t;
    /*======  Helper functions  ======*/
    /* margin, from 64 to 0 */
    /* this formula ensures that bound(A) + bound(B) <= bound(A+B) as long as A and B >= 128 KB */
    #[no_mangle]
    fn ZSTD_compressBound(srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
    #[no_mangle]
    fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> size_t;
    /* ! ZSTD_createCDict() :
 *  When compressing multiple messages / blocks using the same dictionary, it's recommended to load it only once.
 *  ZSTD_createCDict() will create a digested dictionary, ready to start future compression operations without startup cost.
 *  ZSTD_CDict can be created once and shared by multiple threads concurrently, since its usage is read-only.
 * `dictBuffer` can be released after ZSTD_CDict creation, because its content is copied within CDict.
 *  Consider experimental function `ZSTD_createCDict_byReference()` if you prefer to not duplicate `dictBuffer` content.
 *  Note : A ZSTD_CDict can be created from an empty dictBuffer, but it is inefficient when used to compress small data. */
    #[no_mangle]
    fn ZSTD_createCDict(dictBuffer: *const libc::c_void, dictSize: size_t,
                        compressionLevel: libc::c_int) -> *mut ZSTD_CDict;
    /* ! ZSTD_freeCDict() :
 *  Function frees memory allocated by ZSTD_createCDict(). */
    #[no_mangle]
    fn ZSTD_freeCDict(CDict: *mut ZSTD_CDict) -> size_t;
    /* ! ZSTD_compress_usingCDict() :
 *  Compression using a digested Dictionary.
 *  Recommended when same dictionary is used multiple times.
 *  Note : compression level is _decided at dictionary creation time_,
 *     and frame parameters are hardcoded (dictID=yes, contentSize=yes, checksum=no) */
    #[no_mangle]
    fn ZSTD_compress_usingCDict(cctx: *mut ZSTD_CCtx, dst: *mut libc::c_void,
                                dstCapacity: size_t, src: *const libc::c_void,
                                srcSize: size_t, cdict: *const ZSTD_CDict)
     -> size_t;
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
    fn pthread_mutex_init(__mutex: *mut pthread_mutex_t,
                          __mutexattr: *const pthread_mutexattr_t)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_init(__cond: *mut pthread_cond_t,
                         __cond_attr: *const pthread_condattr_t)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_wait(__cond: *mut pthread_cond_t,
                         __mutex: *mut pthread_mutex_t) -> libc::c_int;
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
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
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
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
pub type clock_t = __clock_t;
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
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type ZSTD_CDict = ZSTD_CDict_s;
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
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
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
pub type COVER_map_t = COVER_map_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct COVER_map_s {
    pub data: *mut COVER_map_pair_t,
    pub sizeLog: U32,
    pub size: U32,
    pub sizeMask: U32,
}
pub type COVER_map_pair_t = COVER_map_pair_t_s;
/*-*************************************
* Hash table
***************************************
* A small specialized hash map for storing activeDmers.
* The map does not resize, so if it becomes full it will loop forever.
* Thus, the map must be large enough to store every value.
* The map implements linear probing and keeps its load less than 0.5.
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct COVER_map_pair_t_s {
    pub key: U32,
    pub value: U32,
}
/*-*************************************
* Context
***************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct COVER_ctx_t {
    pub samples: *const BYTE,
    pub offsets: *mut size_t,
    pub samplesSizes: *const size_t,
    pub nbSamples: size_t,
    pub nbTrainSamples: size_t,
    pub nbTestSamples: size_t,
    pub suffix: *mut U32,
    pub suffixSize: size_t,
    pub freqs: *mut U32,
    pub dmerAt: *mut U32,
    pub d: libc::c_uint,
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
pub type COVER_tryParameters_data_t = COVER_tryParameters_data_s;
/* *
 * Parameters for COVER_tryParameters().
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct COVER_tryParameters_data_s {
    pub ctx: *const COVER_ctx_t,
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
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> U32 {
    return (31i32 - val.leading_zeros() as i32) as U32;
}
/* ! ZDICT_trainFromBuffer_cover():
 *  Train a dictionary from an array of samples using the COVER algorithm.
 *  Samples must be stored concatenated in a single flat buffer `samplesBuffer`,
 *  supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order.
 *  The resulting dictionary will be saved into `dictBuffer`.
 * @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)
 *          or an error code, which can be tested with ZDICT_isError().
 *  Note: ZDICT_trainFromBuffer_cover() requires about 9 bytes of memory for each input byte.
 *  Tips: In general, a reasonable dictionary has a size of ~ 100 KB.
 *        It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`.
 *        In general, it's recommended to provide a few thousands samples, though this can vary a lot.
 *        It's recommended that total size of all samples be about ~x100 times the target size of dictionary.
 */
#[no_mangle]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_cover(mut dictBuffer:
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
                                                         ZDICT_cover_params_t)
 -> size_t {
    let dict: *mut BYTE = dictBuffer as *mut BYTE;
    let mut ctx: COVER_ctx_t =
        COVER_ctx_t{samples: 0 as *const BYTE,
                    offsets: 0 as *mut size_t,
                    samplesSizes: 0 as *const size_t,
                    nbSamples: 0,
                    nbTrainSamples: 0,
                    nbTestSamples: 0,
                    suffix: 0 as *mut U32,
                    suffixSize: 0,
                    freqs: 0 as *mut U32,
                    dmerAt: 0 as *mut U32,
                    d: 0,};
    let mut activeDmers: COVER_map_t =
        COVER_map_s{data: 0 as *mut COVER_map_pair_t,
                    sizeLog: 0,
                    size: 0,
                    sizeMask: 0,};
    parameters.splitPoint = 1.0f64;
    g_displayLevel = parameters.zParams.notificationLevel as libc::c_int;
    if 0 == COVER_checkParameters(parameters, dictBufferCapacity) {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Cover parameters incorrect\n\x00" as *const u8 as
                        *const libc::c_char);
            fflush(stderr);
        }
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if nbSamples == 0i32 as libc::c_uint {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Cover must have at least one input file\n\x00" as
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
    if 0 ==
           COVER_ctx_init(&mut ctx, samplesBuffer, samplesSizes, nbSamples,
                          parameters.d, parameters.splitPoint) {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if 0 ==
           COVER_map_init(&mut activeDmers,
                          parameters.k.wrapping_sub(parameters.d).wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint))
       {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Failed to allocate dmer map: out of memory\n\x00" as
                        *const u8 as *const libc::c_char);
            fflush(stderr);
        }
        COVER_ctx_destroy(&mut ctx);
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if g_displayLevel >= 2i32 {
        fprintf(stderr,
                b"Building dictionary\n\x00" as *const u8 as
                    *const libc::c_char);
        fflush(stderr);
    }
    let tail: size_t =
        COVER_buildDictionary(&mut ctx, ctx.freqs, &mut activeDmers,
                              dictBuffer, dictBufferCapacity, parameters);
    let dictionarySize: size_t =
        ZDICT_finalizeDictionary(dict as *mut libc::c_void,
                                 dictBufferCapacity,
                                 dict.offset(tail as isize) as
                                     *const libc::c_void,
                                 dictBufferCapacity.wrapping_sub(tail),
                                 samplesBuffer, samplesSizes, nbSamples,
                                 parameters.zParams);
    if 0 == ERR_isError(dictionarySize) {
        if g_displayLevel >= 2i32 {
            fprintf(stderr,
                    b"Constructed dictionary of size %u\n\x00" as *const u8 as
                        *const libc::c_char, dictionarySize as libc::c_uint);
            fflush(stderr);
        }
    }
    COVER_ctx_destroy(&mut ctx);
    COVER_map_destroy(&mut activeDmers);
    return dictionarySize;
}
/* *
 * Given the prepared context build the dictionary.
 */
unsafe extern "C" fn COVER_buildDictionary(mut ctx: *const COVER_ctx_t,
                                           mut freqs: *mut U32,
                                           mut activeDmers: *mut COVER_map_t,
                                           mut dictBuffer: *mut libc::c_void,
                                           mut dictBufferCapacity: size_t,
                                           mut parameters:
                                               ZDICT_cover_params_t)
 -> size_t {
    let dict: *mut BYTE = dictBuffer as *mut BYTE;
    let mut tail: size_t = dictBufferCapacity;
    /* Divide the data up into epochs of equal size.
   * We will select at least one segment from each epoch.
   */
    let epochs: libc::c_uint =
        if 1i32 as libc::c_uint >
               dictBufferCapacity.wrapping_div(parameters.k as
                                                   libc::c_ulong).wrapping_div(4i32
                                                                                   as
                                                                                   libc::c_ulong)
                   as U32 {
            1i32 as libc::c_uint
        } else {
            dictBufferCapacity.wrapping_div(parameters.k as
                                                libc::c_ulong).wrapping_div(4i32
                                                                                as
                                                                                libc::c_ulong)
                as U32
        };
    let epochSize: libc::c_uint =
        (*ctx).suffixSize.wrapping_div(epochs as libc::c_ulong) as U32;
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
            COVER_selectSegment(ctx, freqs, activeDmers, epochBegin, epochEnd,
                                parameters);
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
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* *****************************************************************************
 * Constructs a dictionary using a heuristic based on the following paper:
 *
 * Liao, Petri, Moffat, Wirth
 * Effective Construction of Relative Lempel-Ziv Dictionaries
 * Published in WWW 2016.
 *
 * Adapted from code originally written by @ot (Giuseppe Ottaviano).
 ******************************************************************************/
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
/* *
 * Selects the best segment in an epoch.
 * Segments of are scored according to the function:
 *
 * Let F(d) be the frequency of dmer d.
 * Let S_i be the dmer at position i of segment S which has length k.
 *
 *     Score(S) = F(S_1) + F(S_2) + ... + F(S_{k-d+1})
 *
 * Once the dmer d is in the dictionay we set F(d) = 0.
 */
unsafe extern "C" fn COVER_selectSegment(mut ctx: *const COVER_ctx_t,
                                         mut freqs: *mut U32,
                                         mut activeDmers: *mut COVER_map_t,
                                         mut begin: U32, mut end: U32,
                                         mut parameters: ZDICT_cover_params_t)
 -> COVER_segment_t {
    /* Constants */
    let k: U32 = parameters.k;
    let d: U32 = parameters.d;
    let dmersInK: U32 = k.wrapping_sub(d).wrapping_add(1i32 as libc::c_uint);
    /* Try each segment (activeSegment) and save the best (bestSegment) */
    let mut bestSegment: COVER_segment_t =
        COVER_segment_t{begin: 0i32 as U32,
                        end: 0i32 as U32,
                        score: 0i32 as U32,};
    let mut activeSegment: COVER_segment_t =
        COVER_segment_t{begin: 0, end: 0, score: 0,};
    COVER_map_clear(activeDmers);
    activeSegment.begin = begin;
    activeSegment.end = begin;
    activeSegment.score = 0i32 as U32;
    while activeSegment.end < end {
        let mut newDmer: U32 =
            *(*ctx).dmerAt.offset(activeSegment.end as isize);
        let mut newDmerOcc: *mut U32 = COVER_map_at(activeDmers, newDmer);
        if *newDmerOcc == 0i32 as libc::c_uint {
            activeSegment.score =
                (activeSegment.score as
                     libc::c_uint).wrapping_add(*freqs.offset(newDmer as
                                                                  isize)) as
                    U32 as U32
        }
        activeSegment.end =
            (activeSegment.end as
                 libc::c_uint).wrapping_add(1i32 as libc::c_uint) as U32 as
                U32;
        *newDmerOcc =
            (*newDmerOcc as libc::c_uint).wrapping_add(1i32 as libc::c_uint)
                as U32 as U32;
        if activeSegment.end.wrapping_sub(activeSegment.begin) ==
               dmersInK.wrapping_add(1i32 as libc::c_uint) {
            let mut delDmer: U32 =
                *(*ctx).dmerAt.offset(activeSegment.begin as isize);
            let mut delDmerOcc: *mut U32 = COVER_map_at(activeDmers, delDmer);
            activeSegment.begin =
                (activeSegment.begin as
                     libc::c_uint).wrapping_add(1i32 as libc::c_uint) as U32
                    as U32;
            *delDmerOcc =
                (*delDmerOcc as
                     libc::c_uint).wrapping_sub(1i32 as libc::c_uint) as U32
                    as U32;
            if *delDmerOcc == 0i32 as libc::c_uint {
                COVER_map_remove(activeDmers, delDmer);
                activeSegment.score =
                    (activeSegment.score as
                         libc::c_uint).wrapping_sub(*freqs.offset(delDmer as
                                                                      isize))
                        as U32 as U32
            }
        }
        if activeSegment.score > bestSegment.score {
            bestSegment = activeSegment
        }
    }
    let mut newBegin: U32 = bestSegment.end;
    let mut newEnd: U32 = bestSegment.begin;
    let mut pos: U32 = 0;
    pos = bestSegment.begin;
    while pos != bestSegment.end {
        let mut freq: U32 =
            *freqs.offset(*(*ctx).dmerAt.offset(pos as isize) as isize);
        if freq != 0i32 as libc::c_uint {
            newBegin = if newBegin < pos { newBegin } else { pos };
            newEnd = pos.wrapping_add(1i32 as libc::c_uint)
        }
        pos = pos.wrapping_add(1)
    }
    bestSegment.begin = newBegin;
    bestSegment.end = newEnd;
    let mut pos_0: U32 = 0;
    pos_0 = bestSegment.begin;
    while pos_0 != bestSegment.end {
        *freqs.offset(*(*ctx).dmerAt.offset(pos_0 as isize) as isize) =
            0i32 as U32;
        pos_0 = pos_0.wrapping_add(1)
    }
    return bestSegment;
}
/* *
 * Deletes key from the map if present.
 */
unsafe extern "C" fn COVER_map_remove(mut map: *mut COVER_map_t,
                                      mut key: U32) {
    let mut i: U32 = COVER_map_index(map, key);
    let mut del: *mut COVER_map_pair_t =
        &mut *(*map).data.offset(i as isize) as *mut COVER_map_pair_t;
    let mut shift: U32 = 1i32 as U32;
    if (*del).value == -1i32 as U32 { return }
    i = i.wrapping_add(1i32 as libc::c_uint) & (*map).sizeMask;
    loop  {
        let pos: *mut COVER_map_pair_t =
            &mut *(*map).data.offset(i as isize) as *mut COVER_map_pair_t;
        if (*pos).value == -1i32 as U32 {
            (*del).value = -1i32 as U32;
            return
        }
        if i.wrapping_sub(COVER_map_hash(map, (*pos).key)) & (*map).sizeMask
               >= shift {
            (*del).key = (*pos).key;
            (*del).value = (*pos).value;
            del = pos;
            shift = 1i32 as U32
        } else { shift = shift.wrapping_add(1) }
        i = i.wrapping_add(1i32 as libc::c_uint) & (*map).sizeMask
    };
}
/* *
 * Helper function that returns the index that a key should be placed into.
 */
unsafe extern "C" fn COVER_map_index(mut map: *mut COVER_map_t, mut key: U32)
 -> U32 {
    let hash: U32 = COVER_map_hash(map, key);
    let mut i: U32 = 0;
    i = hash;
    loop  {
        let mut pos: *mut COVER_map_pair_t =
            &mut *(*map).data.offset(i as isize) as *mut COVER_map_pair_t;
        if (*pos).value == -1i32 as U32 { return i }
        if (*pos).key == key { return i }
        i = i.wrapping_add(1i32 as libc::c_uint) & (*map).sizeMask
    };
}
unsafe extern "C" fn COVER_map_hash(mut map: *mut COVER_map_t, mut key: U32)
 -> U32 {
    return key.wrapping_mul(prime4bytes) >>
               (32i32 as libc::c_uint).wrapping_sub((*map).sizeLog);
}
/* *
 * Internal hash function
 */
static mut prime4bytes: U32 = 2654435761u32;
/* *
 * Returns the pointer to the value for key.
 * If key is not in the map, it is inserted and the value is set to 0.
 * The map must not be full.
 */
unsafe extern "C" fn COVER_map_at(mut map: *mut COVER_map_t, mut key: U32)
 -> *mut U32 {
    let mut pos: *mut COVER_map_pair_t =
        &mut *(*map).data.offset(COVER_map_index(map, key) as isize) as
            *mut COVER_map_pair_t;
    if (*pos).value == -1i32 as U32 {
        (*pos).key = key;
        (*pos).value = 0i32 as U32
    }
    return &mut (*pos).value;
}
/* *
 * Clear the map.
 */
unsafe extern "C" fn COVER_map_clear(mut map: *mut COVER_map_t) {
    memset((*map).data as *mut libc::c_void, -1i32 as U32 as libc::c_int,
           ((*map).size as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<COVER_map_pair_t>()
                                                as libc::c_ulong));
}
/* *
 * Destroys a map that is inited with COVER_map_init().
 */
unsafe extern "C" fn COVER_map_destroy(mut map: *mut COVER_map_t) {
    if !(*map).data.is_null() { free((*map).data as *mut libc::c_void); }
    (*map).data = 0 as *mut COVER_map_pair_t;
    (*map).size = 0i32 as U32;
}
/* *
 * Clean up a context initialized with `COVER_ctx_init()`.
 */
unsafe extern "C" fn COVER_ctx_destroy(mut ctx: *mut COVER_ctx_t) {
    if ctx.is_null() { return }
    if !(*ctx).suffix.is_null() {
        free((*ctx).suffix as *mut libc::c_void);
        (*ctx).suffix = 0 as *mut U32
    }
    if !(*ctx).freqs.is_null() {
        free((*ctx).freqs as *mut libc::c_void);
        (*ctx).freqs = 0 as *mut U32
    }
    if !(*ctx).dmerAt.is_null() {
        free((*ctx).dmerAt as *mut libc::c_void);
        (*ctx).dmerAt = 0 as *mut U32
    }
    if !(*ctx).offsets.is_null() {
        free((*ctx).offsets as *mut libc::c_void);
        (*ctx).offsets = 0 as *mut size_t
    };
}
/* *
 * Initializes a map of the given size.
 * Returns 1 on success and 0 on failure.
 * The map must be destroyed with COVER_map_destroy().
 * The map is only guaranteed to be large enough to hold size elements.
 */
unsafe extern "C" fn COVER_map_init(mut map: *mut COVER_map_t, mut size: U32)
 -> libc::c_int {
    (*map).sizeLog = ZSTD_highbit32(size).wrapping_add(2i32 as libc::c_uint);
    (*map).size = (1i32 as U32) << (*map).sizeLog;
    (*map).sizeMask = (*map).size.wrapping_sub(1i32 as libc::c_uint);
    (*map).data =
        malloc(((*map).size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<COVER_map_pair_t>()
                                                    as libc::c_ulong)) as
            *mut COVER_map_pair_t;
    if (*map).data.is_null() {
        (*map).sizeLog = 0i32 as U32;
        (*map).size = 0i32 as U32;
        return 0i32
    }
    COVER_map_clear(map);
    return 1i32;
}
/* *
 * Prepare a context for dictionary building.
 * The context is only dependent on the parameter `d` and can used multiple
 * times.
 * Returns 1 on success or zero on error.
 * The context must be destroyed with `COVER_ctx_destroy()`.
 */
unsafe extern "C" fn COVER_ctx_init(mut ctx: *mut COVER_ctx_t,
                                    mut samplesBuffer: *const libc::c_void,
                                    mut samplesSizes: *const size_t,
                                    mut nbSamples: libc::c_uint,
                                    mut d: libc::c_uint,
                                    mut splitPoint: libc::c_double)
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
                    b"Total number of training samples is %u and is invalid.\x00"
                        as *const u8 as *const libc::c_char, nbTrainSamples);
            fflush(stderr);
        }
        return 0i32
    }
    if nbTestSamples < 1i32 as libc::c_uint {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Total number of testing samples is %u and is invalid.\x00"
                        as *const u8 as *const libc::c_char, nbTestSamples);
            fflush(stderr);
        }
        return 0i32
    }
    memset(ctx as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<COVER_ctx_t>() as libc::c_ulong);
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
    (*ctx).suffixSize =
        trainingSamplesSize.wrapping_sub(if d as libc::c_ulong >
                                                ::std::mem::size_of::<U64>()
                                                    as libc::c_ulong {
                                             d as libc::c_ulong
                                         } else {
                                             ::std::mem::size_of::<U64>() as
                                                 libc::c_ulong
                                         }).wrapping_add(1i32 as
                                                             libc::c_ulong);
    (*ctx).suffix =
        malloc((*ctx).suffixSize.wrapping_mul(::std::mem::size_of::<U32>() as
                                                  libc::c_ulong)) as *mut U32;
    (*ctx).dmerAt =
        malloc((*ctx).suffixSize.wrapping_mul(::std::mem::size_of::<U32>() as
                                                  libc::c_ulong)) as *mut U32;
    (*ctx).offsets =
        malloc((nbSamples.wrapping_add(1i32 as libc::c_uint) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>()
                                                    as libc::c_ulong)) as
            *mut size_t;
    if (*ctx).suffix.is_null() || (*ctx).dmerAt.is_null() ||
           (*ctx).offsets.is_null() {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Failed to allocate scratch buffers\n\x00" as *const u8
                        as *const libc::c_char);
            fflush(stderr);
        }
        COVER_ctx_destroy(ctx);
        return 0i32
    }
    (*ctx).freqs = 0 as *mut U32;
    (*ctx).d = d;
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
    if g_displayLevel >= 2i32 {
        fprintf(stderr,
                b"Constructing partial suffix array\n\x00" as *const u8 as
                    *const libc::c_char);
        fflush(stderr);
    }
    let mut i_0: U32 = 0;
    i_0 = 0i32 as U32;
    while (i_0 as libc::c_ulong) < (*ctx).suffixSize {
        *(*ctx).suffix.offset(i_0 as isize) = i_0;
        i_0 = i_0.wrapping_add(1)
    }
    g_ctx = ctx;
    qsort((*ctx).suffix as *mut libc::c_void, (*ctx).suffixSize,
          ::std::mem::size_of::<U32>() as libc::c_ulong,
          if (*ctx).d <= 8i32 as libc::c_uint {
              Some(COVER_strict_cmp8)
          } else { Some(COVER_strict_cmp) });
    if g_displayLevel >= 2i32 {
        fprintf(stderr,
                b"Computing frequencies\n\x00" as *const u8 as
                    *const libc::c_char);
        fflush(stderr);
    }
    COVER_groupBy((*ctx).suffix as *const libc::c_void, (*ctx).suffixSize,
                  ::std::mem::size_of::<U32>() as libc::c_ulong, ctx,
                  if (*ctx).d <= 8i32 as libc::c_uint {
                      Some(COVER_cmp8)
                  } else { Some(COVER_cmp) }, Some(COVER_group));
    (*ctx).freqs = (*ctx).suffix;
    (*ctx).suffix = 0 as *mut U32;
    return 1i32;
}
/*-*************************************
*  Cover functions
***************************************/
/* *
 * Called on each group of positions with the same dmer.
 * Counts the frequency of each dmer and saves it in the suffix array.
 * Fills `ctx->dmerAt`.
 */
unsafe extern "C" fn COVER_group(mut ctx: *mut COVER_ctx_t,
                                 mut group: *const libc::c_void,
                                 mut groupEnd: *const libc::c_void) {
    /* The group consists of all the positions with the same first d bytes. */
    let mut grpPtr: *const U32 = group as *const U32;
    let mut grpEnd: *const U32 = groupEnd as *const U32;
    /* The dmerId is how we will reference this dmer.
   * This allows us to map the whole dmer space to a much smaller space, the
   * size of the suffix array.
   */
    let dmerId: U32 =
        grpPtr.wrapping_offset_from((*ctx).suffix) as libc::c_long as U32;
    /* Count the number of samples this dmer shows up in */
    let mut freq: U32 = 0i32 as U32;
    /* Details */
    let mut curOffsetPtr: *const size_t = (*ctx).offsets;
    let mut offsetsEnd: *const size_t =
        (*ctx).offsets.offset((*ctx).nbSamples as isize);
    /* Once *grpPtr >= curSampleEnd this occurrence of the dmer is in a
   * different sample than the last.
   */
    let mut curSampleEnd: size_t = *(*ctx).offsets.offset(0isize);
    while grpPtr != grpEnd {
        *(*ctx).dmerAt.offset(*grpPtr as isize) = dmerId;
        /* Dictionaries only help for the first reference to the dmer.
     * After that zstd can reference the match from the previous reference.
     * So only count each dmer once for each sample it is in.
     */
        if !((*grpPtr as libc::c_ulong) < curSampleEnd) {
            freq =
                (freq as libc::c_uint).wrapping_add(1i32 as libc::c_uint) as
                    U32 as U32;
            if grpPtr.offset(1isize) != grpEnd {
                let mut sampleEndPtr: *const size_t =
                    COVER_lower_bound(curOffsetPtr, offsetsEnd,
                                      *grpPtr as size_t);
                curSampleEnd = *sampleEndPtr;
                curOffsetPtr = sampleEndPtr.offset(1isize)
            }
        }
        grpPtr = grpPtr.offset(1isize)
    }
    *(*ctx).suffix.offset(dmerId as isize) = freq;
}
/* *
 * Returns the first pointer in [first, last) whose element does not compare
 * less than value.  If no such element exists it returns last.
 */
unsafe extern "C" fn COVER_lower_bound(mut first: *const size_t,
                                       mut last: *const size_t,
                                       mut value: size_t) -> *const size_t {
    let mut count: size_t =
        last.wrapping_offset_from(first) as libc::c_long as size_t;
    while count != 0i32 as libc::c_ulong {
        let mut step: size_t = count.wrapping_div(2i32 as libc::c_ulong);
        let mut ptr: *const size_t = first;
        ptr = ptr.offset(step as isize);
        if *ptr < value {
            ptr = ptr.offset(1isize);
            first = ptr;
            count =
                (count as
                     libc::c_ulong).wrapping_sub(step.wrapping_add(1i32 as
                                                                       libc::c_ulong))
                    as size_t as size_t
        } else { count = step }
    }
    return first;
}
/* *
 * Returns -1 if the dmer at lp is less than the dmer at rp.
 * Return 0 if the dmers at lp and rp are equal.
 * Returns 1 if the dmer at lp is greater than the dmer at rp.
 */
unsafe extern "C" fn COVER_cmp(mut ctx: *mut COVER_ctx_t,
                               mut lp: *const libc::c_void,
                               mut rp: *const libc::c_void) -> libc::c_int {
    let lhs: U32 = *(lp as *const U32);
    let rhs: U32 = *(rp as *const U32);
    return memcmp((*ctx).samples.offset(lhs as isize) as *const libc::c_void,
                  (*ctx).samples.offset(rhs as isize) as *const libc::c_void,
                  (*ctx).d as libc::c_ulong);
}
/* *
 * Faster version for d <= 8.
 */
unsafe extern "C" fn COVER_cmp8(mut ctx: *mut COVER_ctx_t,
                                mut lp: *const libc::c_void,
                                mut rp: *const libc::c_void) -> libc::c_int {
    let mask: U64 =
        if (*ctx).d == 8i32 as libc::c_uint {
            -1i32 as U64
        } else {
            ((1i32 as U64) <<
                 (8i32 as
                      libc::c_uint).wrapping_mul((*ctx).d)).wrapping_sub(1i32
                                                                             as
                                                                             libc::c_ulong)
        };
    let lhs: U64 =
        MEM_readLE64((*ctx).samples.offset(*(lp as *const U32) as isize) as
                         *const libc::c_void) & mask;
    let rhs: U64 =
        MEM_readLE64((*ctx).samples.offset(*(rp as *const U32) as isize) as
                         *const libc::c_void) & mask;
    if lhs < rhs { return -1i32 }
    return (lhs > rhs) as libc::c_int;
}
/* *
 * Generic groupBy function.
 * Groups an array sorted by cmp into groups with equivalent values.
 * Calls grp for each group.
 */
unsafe extern "C" fn COVER_groupBy(mut data: *const libc::c_void,
                                   mut count: size_t, mut size: size_t,
                                   mut ctx: *mut COVER_ctx_t,
                                   mut cmp:
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut COVER_ctx_t,
                                                                   _:
                                                                       *const libc::c_void,
                                                                   _:
                                                                       *const libc::c_void)
                                                  -> libc::c_int>,
                                   mut grp:
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut COVER_ctx_t,
                                                                   _:
                                                                       *const libc::c_void,
                                                                   _:
                                                                       *const libc::c_void)
                                                  -> ()>) {
    let mut ptr: *const BYTE = data as *const BYTE;
    let mut num: size_t = 0i32 as size_t;
    while num < count {
        let mut grpEnd: *const BYTE = ptr.offset(size as isize);
        num = num.wrapping_add(1);
        while num < count &&
                  cmp.expect("non-null function pointer")(ctx,
                                                          ptr as
                                                              *const libc::c_void,
                                                          grpEnd as
                                                              *const libc::c_void)
                      == 0i32 {
            grpEnd = grpEnd.offset(size as isize);
            num = num.wrapping_add(1)
        }
        grp.expect("non-null function pointer")(ctx,
                                                ptr as *const libc::c_void,
                                                grpEnd as
                                                    *const libc::c_void);
        ptr = grpEnd
    };
}
/* *
 * Same as COVER_cmp() except ties are broken by pointer value
 * NOTE: g_ctx must be set to call this function.  A global is required because
 * qsort doesn't take an opaque pointer.
 */
unsafe extern "C" fn COVER_strict_cmp(mut lp: *const libc::c_void,
                                      mut rp: *const libc::c_void)
 -> libc::c_int {
    let mut result: libc::c_int = COVER_cmp(g_ctx, lp, rp);
    if result == 0i32 { result = if lp < rp { -1i32 } else { 1i32 } }
    return result;
}
/* We need a global context for qsort... */
static mut g_ctx: *mut COVER_ctx_t =
    0 as *const COVER_ctx_t as *mut COVER_ctx_t;
/* *
 * Faster version for d <= 8.
 */
unsafe extern "C" fn COVER_strict_cmp8(mut lp: *const libc::c_void,
                                       mut rp: *const libc::c_void)
 -> libc::c_int {
    let mut result: libc::c_int = COVER_cmp8(g_ctx, lp, rp);
    if result == 0i32 { result = if lp < rp { -1i32 } else { 1i32 } }
    return result;
}
/* *
 * Returns the sum of the sample sizes.
 */
#[no_mangle]
pub unsafe extern "C" fn COVER_sum(mut samplesSizes: *const size_t,
                                   mut nbSamples: libc::c_uint) -> size_t {
    let mut sum: size_t = 0i32 as size_t;
    let mut i: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    while i < nbSamples {
        sum =
            (sum as
                 libc::c_ulong).wrapping_add(*samplesSizes.offset(i as isize))
                as size_t as size_t;
        i = i.wrapping_add(1)
    }
    return sum;
}
/* *
 * Check the validity of the parameters.
 * Returns non-zero if the parameters are valid and 0 otherwise.
 */
unsafe extern "C" fn COVER_checkParameters(mut parameters:
                                               ZDICT_cover_params_t,
                                           mut maxDictSize: size_t)
 -> libc::c_int {
    if parameters.d == 0i32 as libc::c_uint ||
           parameters.k == 0i32 as libc::c_uint {
        return 0i32
    }
    if parameters.k as libc::c_ulong > maxDictSize { return 0i32 }
    if parameters.d > parameters.k { return 0i32 }
    if parameters.splitPoint <= 0i32 as libc::c_double ||
           parameters.splitPoint > 1i32 as libc::c_double {
        return 0i32
    }
    return 1i32;
}
/* ! ZDICT_optimizeTrainFromBuffer_cover():
 * The same requirements as above hold for all the parameters except `parameters`.
 * This function tries many parameter combinations and picks the best parameters.
 * `*parameters` is filled with the best parameters found,
 * dictionary constructed with those parameters is stored in `dictBuffer`.
 *
 * All of the parameters d, k, steps are optional.
 * If d is non-zero then we don't check multiple values of d, otherwise we check d = {6, 8}.
 * if steps is zero it defaults to its default value.
 * If k is non-zero then we don't check multiple values of k, otherwise we check steps values in [50, 2000].
 *
 * @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)
 *           or an error code, which can be tested with ZDICT_isError().
 *           On success `*parameters` contains the parameters selected.
 * Note: ZDICT_optimizeTrainFromBuffer_cover() requires about 8 bytes of memory for each input byte and additionally another 5 bytes of memory for each byte of memory for each thread.
 */
#[no_mangle]
pub unsafe extern "C" fn ZDICT_optimizeTrainFromBuffer_cover(mut dictBuffer:
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
                                                                 *mut ZDICT_cover_params_t)
 -> size_t {
    /* constants */
    let nbThreads: libc::c_uint = (*parameters).nbThreads;
    let splitPoint: libc::c_double =
        if (*parameters).splitPoint <= 0.0f64 {
            1.0f64
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
                    b"Incorrect parameters\n\x00" as *const u8 as
                        *const libc::c_char);
            fflush(stderr);
        }
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if kMinK < kMaxD || kMaxK < kMinK {
        if displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Incorrect parameters\n\x00" as *const u8 as
                        *const libc::c_char);
            fflush(stderr);
        }
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if nbSamples == 0i32 as libc::c_uint {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Cover must have at least one input file\n\x00" as
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
    if nbThreads > 1i32 as libc::c_uint {
        pool = POOL_create(nbThreads as size_t, 1i32 as size_t);
        if pool.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        }
    }
    COVER_best_init(&mut best);
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
        let mut ctx: COVER_ctx_t =
            COVER_ctx_t{samples: 0 as *const BYTE,
                        offsets: 0 as *mut size_t,
                        samplesSizes: 0 as *const size_t,
                        nbSamples: 0,
                        nbTrainSamples: 0,
                        nbTestSamples: 0,
                        suffix: 0 as *mut U32,
                        suffixSize: 0,
                        freqs: 0 as *mut U32,
                        dmerAt: 0 as *mut U32,
                        d: 0,};
        if displayLevel >= 3i32 {
            fprintf(stderr, b"d=%u\n\x00" as *const u8 as *const libc::c_char,
                    d);
            fflush(stderr);
        }
        if 0 ==
               COVER_ctx_init(&mut ctx, samplesBuffer, samplesSizes,
                              nbSamples, d, splitPoint) {
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
            let mut data: *mut COVER_tryParameters_data_t =
                malloc(::std::mem::size_of::<COVER_tryParameters_data_t>() as
                           libc::c_ulong) as *mut COVER_tryParameters_data_t;
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
                COVER_ctx_destroy(&mut ctx);
                POOL_free(pool);
                return -(ZSTD_error_GENERIC as libc::c_int) as size_t
            }
            (*data).ctx = &mut ctx;
            (*data).best = &mut best;
            (*data).dictBufferCapacity = dictBufferCapacity;
            (*data).parameters = *parameters;
            (*data).parameters.k = k;
            (*data).parameters.d = d;
            (*data).parameters.splitPoint = splitPoint;
            (*data).parameters.steps = kSteps;
            (*data).parameters.zParams.notificationLevel =
                g_displayLevel as libc::c_uint;
            /* Check the parameters */
            if 0 ==
                   COVER_checkParameters((*data).parameters,
                                         dictBufferCapacity) {
                if g_displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"Cover parameters incorrect\n\x00" as *const u8
                                as *const libc::c_char);
                    fflush(stderr);
                }
                free(data as *mut libc::c_void);
            } else {
                COVER_best_start(&mut best);
                if !pool.is_null() {
                    POOL_add(pool, Some(COVER_tryParameters),
                             data as *mut libc::c_void);
                } else { COVER_tryParameters(data as *mut libc::c_void); }
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
        COVER_ctx_destroy(&mut ctx);
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
    *parameters = best.parameters;
    memcpy(dictBuffer, best.dict, dictSize);
    COVER_best_destroy(&mut best);
    POOL_free(pool);
    return dictSize;
}
/* *
 * Call COVER_best_wait() and then destroy the COVER_best_t.
 */
#[no_mangle]
pub unsafe extern "C" fn COVER_best_destroy(mut best: *mut COVER_best_t) {
    if best.is_null() { return }
    COVER_best_wait(best);
    if !(*best).dict.is_null() { free((*best).dict); }
    pthread_mutex_destroy(&mut (*best).mutex);
    pthread_cond_destroy(&mut (*best).cond);
}
/* *
 * Wait until liveJobs == 0.
 */
#[no_mangle]
pub unsafe extern "C" fn COVER_best_wait(mut best: *mut COVER_best_t) {
    if best.is_null() { return }
    pthread_mutex_lock(&mut (*best).mutex);
    while (*best).liveJobs != 0i32 as libc::c_ulong {
        pthread_cond_wait(&mut (*best).cond, &mut (*best).mutex);
    }
    pthread_mutex_unlock(&mut (*best).mutex);
}
/* *
 * Tries a set of parameters and updates the COVER_best_t with the results.
 * This function is thread safe if zstd is compiled with multithreaded support.
 * It takes its parameters as an *OWNING* opaque pointer to support threading.
 */
unsafe extern "C" fn COVER_tryParameters(mut opaque: *mut libc::c_void) {
    /* Save parameters as local variables */
    let data: *mut COVER_tryParameters_data_t =
        opaque as *mut COVER_tryParameters_data_t;
    let ctx: *const COVER_ctx_t = (*data).ctx;
    let parameters: ZDICT_cover_params_t = (*data).parameters;
    let mut dictBufferCapacity: size_t = (*data).dictBufferCapacity;
    let mut totalCompressedSize: size_t =
        -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    /* Allocate space for hash table, dict, and freqs */
    let mut activeDmers: COVER_map_t =
        COVER_map_s{data: 0 as *mut COVER_map_pair_t,
                    sizeLog: 0,
                    size: 0,
                    sizeMask: 0,};
    let dict: *mut BYTE = malloc(dictBufferCapacity) as *mut BYTE;
    let mut freqs: *mut U32 =
        malloc((*ctx).suffixSize.wrapping_mul(::std::mem::size_of::<U32>() as
                                                  libc::c_ulong)) as *mut U32;
    if 0 ==
           COVER_map_init(&mut activeDmers,
                          parameters.k.wrapping_sub(parameters.d).wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint))
       {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Failed to allocate dmer map: out of memory\n\x00" as
                        *const u8 as *const libc::c_char);
            fflush(stderr);
        }
    } else if dict.is_null() || freqs.is_null() {
        if g_displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Failed to allocate buffers: out of memory\n\x00" as
                        *const u8 as *const libc::c_char);
            fflush(stderr);
        }
    } else {
        memcpy(freqs as *mut libc::c_void,
               (*ctx).freqs as *const libc::c_void,
               (*ctx).suffixSize.wrapping_mul(::std::mem::size_of::<U32>() as
                                                  libc::c_ulong));
        /* Build the dictionary */
        let tail: size_t =
            COVER_buildDictionary(ctx, freqs, &mut activeDmers,
                                  dict as *mut libc::c_void,
                                  dictBufferCapacity, parameters);
        dictBufferCapacity =
            ZDICT_finalizeDictionary(dict as *mut libc::c_void,
                                     dictBufferCapacity,
                                     dict.offset(tail as isize) as
                                         *const libc::c_void,
                                     dictBufferCapacity.wrapping_sub(tail),
                                     (*ctx).samples as *const libc::c_void,
                                     (*ctx).samplesSizes,
                                     (*ctx).nbTrainSamples as libc::c_uint,
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
    COVER_map_destroy(&mut activeDmers);
    if !dict.is_null() { free(dict as *mut libc::c_void); }
    if !freqs.is_null() { free(freqs as *mut libc::c_void); };
}
/* *
 * Called when a thread finishes executing, both on error or success.
 * Decrements liveJobs and signals any waiting threads if liveJobs == 0.
 * If this dictionary is the best so far save it and its parameters.
 */
#[no_mangle]
pub unsafe extern "C" fn COVER_best_finish(mut best: *mut COVER_best_t,
                                           mut compressedSize: size_t,
                                           mut parameters:
                                               ZDICT_cover_params_t,
                                           mut dict: *mut libc::c_void,
                                           mut dictSize: size_t) {
    if best.is_null() { return }
    let mut liveJobs: size_t = 0;
    pthread_mutex_lock(&mut (*best).mutex);
    (*best).liveJobs = (*best).liveJobs.wrapping_sub(1);
    liveJobs = (*best).liveJobs;
    if compressedSize < (*best).compressedSize {
        if (*best).dict.is_null() || (*best).dictSize < dictSize {
            if !(*best).dict.is_null() { free((*best).dict); }
            (*best).dict = malloc(dictSize);
            if (*best).dict.is_null() {
                (*best).compressedSize =
                    -(ZSTD_error_GENERIC as libc::c_int) as size_t;
                (*best).dictSize = 0i32 as size_t;
                pthread_cond_signal(&mut (*best).cond);
                pthread_mutex_unlock(&mut (*best).mutex);
                return
            }
        }
        memcpy((*best).dict, dict, dictSize);
        (*best).dictSize = dictSize;
        (*best).parameters = parameters;
        (*best).compressedSize = compressedSize
    }
    if liveJobs == 0i32 as libc::c_ulong {
        pthread_cond_broadcast(&mut (*best).cond);
    }
    pthread_mutex_unlock(&mut (*best).mutex);
}
/* *
 *  Checks total compressed size of a dictionary
 */
#[no_mangle]
pub unsafe extern "C" fn COVER_checkTotalCompressedSize(parameters:
                                                            ZDICT_cover_params_t,
                                                        mut samplesSizes:
                                                            *const size_t,
                                                        mut samples:
                                                            *const BYTE,
                                                        mut offsets:
                                                            *mut size_t,
                                                        mut nbTrainSamples:
                                                            size_t,
                                                        mut nbSamples: size_t,
                                                        dict: *mut BYTE,
                                                        mut dictBufferCapacity:
                                                            size_t)
 -> size_t {
    let mut totalCompressedSize: size_t =
        -(ZSTD_error_GENERIC as libc::c_int) as size_t;
    /* Pointers */
    let mut cctx: *mut ZSTD_CCtx = 0 as *mut ZSTD_CCtx;
    let mut cdict: *mut ZSTD_CDict = 0 as *mut ZSTD_CDict;
    let mut dst: *mut libc::c_void = 0 as *mut libc::c_void;
    /* Local variables */
    let mut dstCapacity: size_t = 0;
    let mut i: size_t = 0;
    let mut maxSampleSize: size_t = 0i32 as size_t;
    i =
        if parameters.splitPoint < 1.0f64 {
            nbTrainSamples
        } else { 0i32 as libc::c_ulong };
    while i < nbSamples {
        maxSampleSize =
            if *samplesSizes.offset(i as isize) > maxSampleSize {
                *samplesSizes.offset(i as isize)
            } else { maxSampleSize };
        i = i.wrapping_add(1)
    }
    dstCapacity = ZSTD_compressBound(maxSampleSize);
    dst = malloc(dstCapacity);
    cctx = ZSTD_createCCtx();
    cdict =
        ZSTD_createCDict(dict as *const libc::c_void, dictBufferCapacity,
                         parameters.zParams.compressionLevel);
    if !(dst.is_null() || cctx.is_null() || cdict.is_null()) {
        totalCompressedSize = dictBufferCapacity;
        i =
            if parameters.splitPoint < 1.0f64 {
                nbTrainSamples
            } else { 0i32 as libc::c_ulong };
        while i < nbSamples {
            let size: size_t =
                ZSTD_compress_usingCDict(cctx, dst, dstCapacity,
                                         samples.offset(*offsets.offset(i as
                                                                            isize)
                                                            as isize) as
                                             *const libc::c_void,
                                         *samplesSizes.offset(i as isize),
                                         cdict);
            if 0 != ERR_isError(size) {
                totalCompressedSize =
                    -(ZSTD_error_GENERIC as libc::c_int) as size_t;
                break ;
            } else {
                totalCompressedSize =
                    (totalCompressedSize as libc::c_ulong).wrapping_add(size)
                        as size_t as size_t;
                i = i.wrapping_add(1)
            }
        }
    }
    ZSTD_freeCCtx(cctx);
    ZSTD_freeCDict(cdict);
    if !dst.is_null() { free(dst); }
    return totalCompressedSize;
}
/* *
 * Called when a thread is about to be launched.
 * Increments liveJobs.
 */
#[no_mangle]
pub unsafe extern "C" fn COVER_best_start(mut best: *mut COVER_best_t) {
    if best.is_null() { return }
    pthread_mutex_lock(&mut (*best).mutex);
    (*best).liveJobs = (*best).liveJobs.wrapping_add(1);
    pthread_mutex_unlock(&mut (*best).mutex);
}
/* *
 * Initialize the `COVER_best_t`.
 */
#[no_mangle]
pub unsafe extern "C" fn COVER_best_init(mut best: *mut COVER_best_t) {
    if best.is_null() { return }
    pthread_mutex_init(&mut (*best).mutex, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(&mut (*best).cond, 0 as *const pthread_condattr_t);
    (*best).liveJobs = 0i32 as size_t;
    (*best).dict = 0 as *mut libc::c_void;
    (*best).dictSize = 0i32 as size_t;
    (*best).compressedSize = -1i32 as size_t;
    memset(&mut (*best).parameters as *mut ZDICT_cover_params_t as
               *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZDICT_cover_params_t>() as libc::c_ulong);
}