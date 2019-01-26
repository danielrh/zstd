#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc, ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    /*======  Helper functions  ======*/
    /* margin, from 64 to 0 */
    /* this formula ensures that bound(A) + bound(B) <= bound(A+B) as long as A and B >= 128 KB */
    #[no_mangle]
    fn ZSTD_compressBound(srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> size_t;
    /* ! ZSTD_freeCDict() :
 *  Function frees memory allocated by ZSTD_createCDict(). */
    #[no_mangle]
    fn ZSTD_freeCDict(CDict: *mut ZSTD_CDict) -> size_t;
    #[no_mangle]
    fn ZSTD_flushStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer)
     -> size_t;
    #[no_mangle]
    fn ZSTD_endStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer)
     -> size_t;
    /* ===   Memory management   === */
    /* ! ZSTD_sizeof_*() :
 *  These functions give the _current_ memory usage of selected object.
 *  Note that object memory usage can evolve (increase or decrease) over time. */
    #[no_mangle]
    fn ZSTD_sizeof_CCtx(cctx: *const ZSTD_CCtx) -> size_t;
    #[no_mangle]
    fn ZSTD_sizeof_CDict(cdict: *const ZSTD_CDict) -> size_t;
    #[no_mangle]
    fn ZSTD_createCCtx_advanced(customMem: ZSTD_customMem) -> *mut ZSTD_CCtx;
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
    /* ! ZSTD_compress_usingCDict_advanced() :
 *  Same as ZSTD_compress_usingCDict(), with fine-tune control over frame parameters */
    #[no_mangle]
    fn ZSTD_compress_usingCDict_advanced(cctx: *mut ZSTD_CCtx,
                                         dst: *mut libc::c_void,
                                         dstCapacity: size_t,
                                         src: *const libc::c_void,
                                         srcSize: size_t,
                                         cdict: *const ZSTD_CDict,
                                         fParams: ZSTD_frameParameters)
     -> size_t;
    /* ! ZSTD_CCtxParam_setParameter() :
 *  Similar to ZSTD_CCtx_setParameter.
 *  Set one compression parameter, selected by enum ZSTD_cParameter.
 *  Parameters must be applied to a ZSTD_CCtx using ZSTD_CCtx_setParametersUsingCCtxParams().
 * @result : 0, or an error code (which can be tested with ZSTD_isError()).
 */
    #[no_mangle]
    fn ZSTD_CCtxParam_setParameter(params: *mut ZSTD_CCtx_params,
                                   param: ZSTD_cParameter, value: libc::c_int)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressContinue(cctx: *mut ZSTD_CCtx, dst: *mut libc::c_void,
                             dstCapacity: size_t, src: *const libc::c_void,
                             srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressEnd(cctx: *mut ZSTD_CCtx, dst: *mut libc::c_void,
                        dstCapacity: size_t, src: *const libc::c_void,
                        srcSize: size_t) -> size_t;
    #[no_mangle]
    fn POOL_create_advanced(numThreads: size_t, queueSize: size_t,
                            customMem: ZSTD_customMem) -> *mut POOL_ctx;
    /* ! POOL_free() :
 *  Free a thread pool returned by POOL_create().
 */
    #[no_mangle]
    fn POOL_free(ctx: *mut POOL_ctx);
    /* ! POOL_resize() :
 *  Expands or shrinks pool's number of threads.
 *  This is more efficient than releasing + creating a new context,
 *  since it tries to preserve and re-use existing threads.
 * `numThreads` must be at least 1.
 * @return : 0 when resize was successful,
 *           !0 (typically 1) if there is an error.
 *    note : only numThreads can be resized, queueSize remains unchanged.
 */
    #[no_mangle]
    fn POOL_resize(ctx: *mut POOL_ctx, numThreads: size_t) -> libc::c_int;
    /* ! POOL_sizeof() :
 * @return threadpool memory usage
 *  note : compatible with NULL (returns 0 in this case)
 */
    #[no_mangle]
    fn POOL_sizeof(ctx: *mut POOL_ctx) -> size_t;
    /* ! POOL_add() :
 *  Add the job `function(opaque)` to the thread pool. `ctx` must be valid.
 *  Possibly blocks until there is room in the queue.
 *  Note : The function may be executed asynchronously,
 *         therefore, `opaque` must live until function has been completed.
 */
    #[no_mangle]
    fn POOL_add(ctx: *mut POOL_ctx, function: POOL_function,
                opaque: *mut libc::c_void);
    /* ! POOL_tryAdd() :
 *  Add the job `function(opaque)` to thread pool _if_ a worker is available.
 *  Returns immediately even if not (does not block).
 * @return : 1 if successful, 0 if not.
 */
    #[no_mangle]
    fn POOL_tryAdd(ctx: *mut POOL_ctx, function: POOL_function,
                   opaque: *mut libc::c_void) -> libc::c_int;
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
    fn ZSTD_XXH64_reset(statePtr: *mut XXH64_state_t, seed: libc::c_ulonglong)
     -> XXH_errorcode;
    #[no_mangle]
    fn ZSTD_XXH64_update(statePtr: *mut XXH64_state_t,
                         input: *const libc::c_void, length: size_t)
     -> XXH_errorcode;
    #[no_mangle]
    fn ZSTD_XXH64_digest(statePtr: *const XXH64_state_t) -> XXH64_hash_t;
    /* custom memory allocation functions */
    #[no_mangle]
    fn ZSTD_malloc(size: size_t, customMem: ZSTD_customMem)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ZSTD_calloc(size: size_t, customMem: ZSTD_customMem)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ZSTD_free(ptr: *mut libc::c_void, customMem: ZSTD_customMem);
    /* Software version */
    /* ZSTD_invalidateRepCodes() :
 * ensures next compression will not use repcodes from previous block.
 * Note : only works with regular variant;
 *        do not use with extDict variant ! */
    #[no_mangle]
    fn ZSTD_invalidateRepCodes(cctx: *mut ZSTD_CCtx);
    /* ZSTD_referenceExternalSequences() :
 * Must be called before starting a compression operation.
 * seqs must parse a prefix of the source.
 * This cannot be used when long range matching is enabled.
 * Zstd will use these sequences, and pass the literals to a secondary block
 * compressor.
 * @return : An error code on failure.
 * NOTE: seqs are not verified! Invalid sequences can cause out-of-bounds memory
 * access and data corruption.
 */
    #[no_mangle]
    fn ZSTD_referenceExternalSequences(cctx: *mut ZSTD_CCtx, seq: *mut rawSeq,
                                       nbSeq: size_t) -> size_t;
    /*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 */
    /* ldmParams_t, U32 */
    /*-*************************************
*  Long distance matching
***************************************/
    /* *
 * ZSTD_ldm_generateSequences():
 *
 * Generates the sequences using the long distance match finder.
 * Generates long range matching sequences in `sequences`, which parse a prefix
 * of the source. `sequences` must be large enough to store every sequence,
 * which can be checked with `ZSTD_ldm_getMaxNbSeq()`.
 * @returns 0 or an error code.
 *
 * NOTE: The user must have called ZSTD_window_update() for all of the input
 * they have, even if they pass it to ZSTD_ldm_generateSequences() in chunks.
 * NOTE: This function returns an error if it runs out of space to store
 *       sequences.
 */
    #[no_mangle]
    fn ZSTD_ldm_generateSequences(ldms: *mut ldmState_t,
                                  sequences: *mut rawSeqStore_t,
                                  params: *const ldmParams_t,
                                  src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    /* ZSTD_compressBegin_advanced_internal() :
 * Private use only. To be called from zstdmt_compress.c. */
    #[no_mangle]
    fn ZSTD_compressBegin_advanced_internal(cctx: *mut ZSTD_CCtx,
                                            dict: *const libc::c_void,
                                            dictSize: size_t,
                                            dictContentType:
                                                ZSTD_dictContentType_e,
                                            dtlm: ZSTD_dictTableLoadMethod_e,
                                            cdict: *const ZSTD_CDict,
                                            params: ZSTD_CCtx_params,
                                            pledgedSrcSize: libc::c_ulonglong)
     -> size_t;
    /* * ZSTD_ldm_getSeqSpace() :
 *  Return an upper bound on the number of sequences that can be produced by
 *  the long distance matcher, or 0 if LDM is disabled.
 */
    #[no_mangle]
    fn ZSTD_ldm_getMaxNbSeq(params: ldmParams_t, maxChunkSize: size_t)
     -> size_t;
    /* * ZSTD_ldm_adjustParameters() :
 *  If the params->hashRateLog is not set, set it to its default value based on
 *  windowLog and params->hashLog.
 *
 *  Ensures that params->bucketSizeLog is <= params->hashLog (setting it to
 *  params->hashLog if it is not).
 *
 *  Ensures that the minMatchLength >= targetLength during optimal parsing.
 */
    #[no_mangle]
    fn ZSTD_ldm_adjustParameters(params: *mut ldmParams_t,
                                 cParams: *const ZSTD_compressionParameters);
    /* ZSTD_compress_advanced_internal() :
 * Private use only. To be called from zstdmt_compress.c. */
    #[no_mangle]
    fn ZSTD_compress_advanced_internal(cctx: *mut ZSTD_CCtx,
                                       dst: *mut libc::c_void,
                                       dstCapacity: size_t,
                                       src: *const libc::c_void,
                                       srcSize: size_t,
                                       dict: *const libc::c_void,
                                       dictSize: size_t,
                                       params: ZSTD_CCtx_params) -> size_t;
    /* ! ZSTD_initCStream_internal() :
 *  Private use only. Init streaming operation.
 *  expects params to be valid.
 *  must receive dict, or cdict, or none, but not both.
 *  @return : 0, or an error code */
    #[no_mangle]
    fn ZSTD_initCStream_internal(zcs: *mut ZSTD_CStream,
                                 dict: *const libc::c_void, dictSize: size_t,
                                 cdict: *const ZSTD_CDict,
                                 params: ZSTD_CCtx_params,
                                 pledgedSrcSize: libc::c_ulonglong) -> size_t;
    /* ZSTD_writeLastEmptyBlock() :
 * output an empty Block with end-of-frame mark to complete a frame
 * @return : size of data written into `dst` (== ZSTD_blockHeaderSize (defined in zstd_internal.h))
 *           or an error code if `dstCapcity` is too small (<ZSTD_blockHeaderSize)
 */
    #[no_mangle]
    fn ZSTD_writeLastEmptyBlock(dst: *mut libc::c_void, dstCapacity: size_t)
     -> size_t;
    /* ! ZSTD_compressStream_generic() :
 *  Private use only. To be called from zstdmt_compress.c in single-thread mode. */
    #[no_mangle]
    fn ZSTD_compressStream_generic(zcs: *mut ZSTD_CStream,
                                   output: *mut ZSTD_outBuffer,
                                   input: *mut ZSTD_inBuffer,
                                   flushMode: ZSTD_EndDirective) -> size_t;
    /* ! ZSTD_getCParamsFromCDict() :
 *  as the name implies */
    #[no_mangle]
    fn ZSTD_getCParamsFromCDict(cdict: *const ZSTD_CDict)
     -> ZSTD_compressionParameters;
    /* debug functions */
    /* ==============================================================
 * Private declarations
 * These prototypes shall only be called from within lib/compress
 * ============================================================== */
    /* ZSTD_getCParamsFromCCtxParams() :
 * cParams are built depending on compressionLevel, src size hints,
 * LDM and manually set compression parameters.
 */
    #[no_mangle]
    fn ZSTD_getCParamsFromCCtxParams(CCtxParams: *const ZSTD_CCtx_params,
                                     srcSizeHint: U64, dictSize: size_t)
     -> ZSTD_compressionParameters;
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
/* typedef'd to ZSTD_CCtx_params within "zstd.h" */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_CCtx_s {
    pub stage: ZSTD_compressionStage_e,
    pub cParamsChanged: libc::c_int,
    pub bmi2: libc::c_int,
    pub requestedParams: ZSTD_CCtx_params,
    pub appliedParams: ZSTD_CCtx_params,
    pub dictID: U32,
    pub workSpaceOversizedDuration: libc::c_int,
    pub workSpace: *mut libc::c_void,
    pub workSpaceSize: size_t,
    pub blockSize: size_t,
    pub pledgedSrcSizePlusOne: libc::c_ulonglong,
    pub consumedSrcSize: libc::c_ulonglong,
    pub producedCSize: libc::c_ulonglong,
    pub xxhState: XXH64_state_t,
    pub customMem: ZSTD_customMem,
    pub staticSize: size_t,
    pub seqStore: seqStore_t,
    pub ldmState: ldmState_t,
    pub ldmSequences: *mut rawSeq,
    pub maxNbLdmSequences: size_t,
    pub externSeqStore: rawSeqStore_t,
    pub blockState: ZSTD_blockState_t,
    pub entropyWorkspace: *mut U32,
    pub inBuff: *mut libc::c_char,
    pub inBuffSize: size_t,
    pub inToCompress: size_t,
    pub inBuffPos: size_t,
    pub inBuffTarget: size_t,
    pub outBuff: *mut libc::c_char,
    pub outBuffSize: size_t,
    pub outBuffContentSize: size_t,
    pub outBuffFlushedSize: size_t,
    pub streamStage: ZSTD_cStreamStage,
    pub frameEnded: U32,
    pub cdictLocal: *mut ZSTD_CDict,
    pub cdict: *const ZSTD_CDict,
    pub prefixDict: ZSTD_prefixDict,
    pub mtctx: *mut ZSTDMT_CCtx,
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
/* Note : This is an internal API.
 *        Some methods are still exposed (ZSTDLIB_API),
 *        because it used to be the only way to invoke MT compression.
 *        Now, it's recommended to use ZSTD_compress_generic() instead.
 *        These methods will stop being exposed in a future version */
/* ===   Dependencies   === */
/* size_t */
/* ZSTD_parameters */
/* ===   Constants   === */
/* ===   Memory management   === */
pub type ZSTDMT_CCtx = ZSTDMT_CCtx_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTDMT_CCtx_s {
    pub factory: *mut POOL_ctx,
    pub jobs: *mut ZSTDMT_jobDescription,
    pub bufPool: *mut ZSTDMT_bufferPool,
    pub cctxPool: *mut ZSTDMT_CCtxPool,
    pub seqPool: *mut ZSTDMT_seqPool,
    pub params: ZSTD_CCtx_params,
    pub targetSectionSize: size_t,
    pub targetPrefixSize: size_t,
    pub jobReady: libc::c_int,
    pub inBuff: inBuff_t,
    pub roundBuff: roundBuff_t,
    pub serial: serialState_t,
    pub rsync: rsyncState_t,
    pub singleBlockingThread: libc::c_uint,
    pub jobIDMask: libc::c_uint,
    pub doneJobID: libc::c_uint,
    pub nextJobID: libc::c_uint,
    pub frameEnded: libc::c_uint,
    pub allJobsCompleted: libc::c_uint,
    pub frameContentSize: libc::c_ulonglong,
    pub consumed: libc::c_ulonglong,
    pub produced: libc::c_ulonglong,
    pub cMem: ZSTD_customMem,
    pub cdictLocal: *mut ZSTD_CDict,
    pub cdict: *const ZSTD_CDict,
}
pub type ZSTD_CDict = ZSTD_CDict_s;
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
pub struct rsyncState_t {
    pub hash: U64,
    pub hitMask: U64,
    pub primePower: U64,
}
pub type U64 = uint64_t;
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct serialState_t {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub params: ZSTD_CCtx_params,
    pub ldmState: ldmState_t,
    pub xxhState: XXH64_state_t,
    pub nextJobID: libc::c_uint,
    pub ldmWindowMutex: pthread_mutex_t,
    pub ldmWindowCond: pthread_cond_t,
    pub ldmWindow: ZSTD_window_t,
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
pub type U32 = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
/*-**************************************************************
*  Basic Types
*****************************************************************/
/* C99 */
pub type BYTE = uint8_t;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __pthread_cond_s {
    pub unnamed: unnamed_1,
    pub unnamed_0: unnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: unnamed_2,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_2 {
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
pub type __pthread_list_t = __pthread_internal_list;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ldmState_t {
    pub window: ZSTD_window_t,
    pub hashTable: *mut ldmEntry_t,
    pub bucketOffsets: *mut BYTE,
    pub hashPower: U64,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ldmEntry_t {
    pub offset: U32,
    pub checksum: U32,
}
/* ***************************************************************************************
 *   experimental API (static linking only)
 ****************************************************************************************
 * The following symbols and constants
 * are not planned to join "stable API" status in the near future.
 * They can still change in future versions.
 * Some of them are planned to remain in the static_only section indefinitely.
 * Some of them might be removed in the future (especially when redundant with existing stable functions)
 * ***************************************************************************************/
/* minimum input size required to query frame header size */
/* can be useful for static allocation */
/* compression parameter bounds */
/* only for ZSTD_fast, other strategies are limited to 6 */
/* only for ZSTD_btopt+, faster strategies are limited to 4 */
/* note : comparing this constant to an unsigned results in a tautological test */
/* by default, the streaming decoder will refuse any frame
                                           * requiring larger than (1<<ZSTD_WINDOWLOG_LIMIT_DEFAULT) window size,
                                           * to preserve host's memory from unreasonable requirements.
                                           * This limit can be overriden using ZSTD_DCtx_setParameter(,ZSTD_d_windowLogMax,).
                                           * The limit does not apply for one-pass decoders (such as ZSTD_decompress()), since no additional memory is allocated */
/* LDM parameter bounds */
/* internal */
/* ---  Advanced types  --- */
pub type ZSTD_CCtx_params = ZSTD_CCtx_params_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_CCtx_params_s {
    pub format: ZSTD_format_e,
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
    pub compressionLevel: libc::c_int,
    pub forceWindow: libc::c_int,
    pub attachDictPref: ZSTD_dictAttachPref_e,
    pub nbWorkers: libc::c_int,
    pub jobSize: size_t,
    pub overlapLog: libc::c_int,
    pub rsyncable: libc::c_int,
    pub ldmParams: ldmParams_t,
    pub customMem: ZSTD_customMem,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ldmParams_t {
    pub enableLdm: U32,
    pub hashLog: U32,
    pub bucketSizeLog: U32,
    pub minMatchLength: U32,
    pub hashRateLog: U32,
    pub windowLog: U32,
}
pub type ZSTD_dictAttachPref_e = libc::c_uint;
/* Always copy the dictionary. */
pub const ZSTD_dictForceCopy: ZSTD_dictAttachPref_e = 2;
/* Never copy the dictionary. */
pub const ZSTD_dictForceAttach: ZSTD_dictAttachPref_e = 1;
/* Note: this enum and the behavior it controls are effectively internal
     * implementation details of the compressor. They are expected to continue
     * to evolve and should be considered only in the context of extremely
     * advanced performance tuning.
     *
     * Zstd currently supports the use of a CDict in two ways:
     *
     * - The contents of the CDict can be copied into the working context. This
     *   means that the compression can search both the dictionary and input
     *   while operating on a single set of internal tables. This makes
     *   the compression faster per-byte of input. However, the initial copy of
     *   the CDict's tables incurs a fixed cost at the beginning of the
     *   compression. For small compressions (< 8 KB), that copy can dominate
     *   the cost of the compression.
     *
     * - The CDict's tables can be used in-place. In this model, compression is
     *   slower per input byte, because the compressor has to search two sets of
     *   tables. However, this model incurs no start-up cost (as long as the
     *   working context's tables can be reused). For small inputs, this can be
     *   faster than copying the CDict's tables.
     *
     * Zstd has a simple internal heuristic that selects which strategy to use
     * at the beginning of a compression. However, if experimentation shows that
     * Zstd is making poor choices, it is possible to override that choice with
     * this enum.
     */
/* Use the default heuristic. */
pub const ZSTD_dictDefaultAttach: ZSTD_dictAttachPref_e = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_frameParameters {
    pub contentSizeFlag: libc::c_int,
    pub checksumFlag: libc::c_int,
    pub noDictIDFlag: libc::c_int,
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct roundBuff_t {
    pub buffer: *mut BYTE,
    pub capacity: size_t,
    pub pos: size_t,
}
/* ------------------------------------------ */
/* =====   Multi-threaded compression   ===== */
/* ------------------------------------------ */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct inBuff_t {
    pub prefix: range_t,
    pub buffer: buffer_t,
    pub filled: size_t,
}
pub type buffer_t = buffer_s;
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* ======   Compiler specifics   ====== */
/* ======   Constants   ====== */
/* ======   Dependencies   ====== */
/* memcpy, memset */
/* INT_MAX, UINT_MAX */
/* Guards code to support resizing the SeqPool.
 * We will want to resize the SeqPool to save memory in the future.
 * Until then, comment the code out since it is unused.
 */
/* ======   Debug   ====== */
/* =====   Buffer Pool   ===== */
/* a single Buffer Pool can be invoked from multiple threads in parallel */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct buffer_s {
    pub start: *mut libc::c_void,
    pub capacity: size_t,
}
/* ====   Serial State   ==== */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct range_t {
    pub start: *const libc::c_void,
    pub size: size_t,
}
pub type ZSTDMT_seqPool = ZSTDMT_bufferPool;
pub type ZSTDMT_bufferPool = ZSTDMT_bufferPool_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTDMT_bufferPool_s {
    pub poolMutex: pthread_mutex_t,
    pub bufferSize: size_t,
    pub totalBuffers: libc::c_uint,
    pub nbBuffers: libc::c_uint,
    pub cMem: ZSTD_customMem,
    pub bTable: [buffer_t; 1],
}
/* =====   CCtx Pool   ===== */
/* a single CCtx Pool can be invoked from multiple threads in parallel */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTDMT_CCtxPool {
    pub poolMutex: pthread_mutex_t,
    pub totalCCtx: libc::c_int,
    pub availCCtx: libc::c_int,
    pub cMem: ZSTD_customMem,
    pub cctx: [*mut ZSTD_CCtx; 1],
}
/* **************************************
*  Explicit context
***************************************/
/*= Compression context
 *  When compressing many times,
 *  it is recommended to allocate a context just once, and re-use it for each successive compression operation.
 *  This will make workload friendlier for system's memory.
 *  Use one context per thread for parallel execution in multi-threaded environments. */
pub type ZSTD_CCtx = ZSTD_CCtx_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTDMT_jobDescription {
    pub consumed: size_t,
    pub cSize: size_t,
    pub job_mutex: pthread_mutex_t,
    pub job_cond: pthread_cond_t,
    pub cctxPool: *mut ZSTDMT_CCtxPool,
    pub bufPool: *mut ZSTDMT_bufferPool,
    pub seqPool: *mut ZSTDMT_seqPool,
    pub serial: *mut serialState_t,
    pub dstBuff: buffer_t,
    pub prefix: range_t,
    pub src: range_t,
    pub jobID: libc::c_uint,
    pub firstJob: libc::c_uint,
    pub lastJob: libc::c_uint,
    pub params: ZSTD_CCtx_params,
    pub cdict: *const ZSTD_CDict,
    pub fullFrameSize: libc::c_ulonglong,
    pub dstFlushed: size_t,
    pub frameChecksumNeeded: libc::c_uint,
}
pub type POOL_ctx = POOL_ctx_s;
pub type ZSTD_prefixDict = ZSTD_prefixDict_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_prefixDict_s {
    pub dict: *const libc::c_void,
    pub dictSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
}
pub type ZSTD_dictContentType_e = libc::c_uint;
/* refuses to load a dictionary if it does not respect Zstandard's specification, starting with ZSTD_MAGIC_DICTIONARY */
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
/* ensures dictionary is always loaded as rawContent, even if it starts with ZSTD_MAGIC_DICTIONARY */
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
/* dictionary is "full" when starting with ZSTD_MAGIC_DICTIONARY, otherwise it is "rawContent" */
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;
pub type ZSTD_cStreamStage = libc::c_uint;
pub const zcss_flush: ZSTD_cStreamStage = 2;
pub const zcss_load: ZSTD_cStreamStage = 1;
pub const zcss_init: ZSTD_cStreamStage = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_blockState_t {
    pub prevCBlock: *mut ZSTD_compressedBlockState_t,
    pub nextCBlock: *mut ZSTD_compressedBlockState_t,
    pub matchState: ZSTD_matchState_t,
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
pub struct ZSTD_compressedBlockState_t {
    pub entropy: ZSTD_entropyCTables_t,
    pub rep: [U32; 3],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct rawSeqStore_t {
    pub seq: *mut rawSeq,
    pub pos: size_t,
    pub size: size_t,
    pub capacity: size_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct rawSeq {
    pub offset: U32,
    pub litLength: U32,
    pub matchLength: U32,
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
pub type U16 = uint16_t;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* This header contains definitions
 * that shall **only** be used by modules within lib/compress.
 */
/*-*************************************
*  Dependencies
***************************************/
/*-*************************************
*  Constants
***************************************/
/* For btlazy2 strategy, index 1 now means "unsorted".
                                       It could be confused for a real successor at index "1", if sorted as larger than its predecessor.
                                       It's not a big deal though : candidate will just be sorted again.
                                       Additionnally, candidate position 1 will be lost.
                                       But candidate 1 cannot hide a large tree of candidates, so it's a minimal loss.
                                       The benefit is that ZSTD_DUBT_UNSORTED_MARK cannot be misdhandled after table re-use with a different strategy
                                       Constant required by ZSTD_compressBlock_btlazy2() and ZSTD_reduceTable_internal() */
/*-*************************************
*  Context memory management
***************************************/
pub type ZSTD_compressionStage_e = libc::c_uint;
pub const ZSTDcs_ending: ZSTD_compressionStage_e = 3;
pub const ZSTDcs_ongoing: ZSTD_compressionStage_e = 2;
pub const ZSTDcs_init: ZSTD_compressionStage_e = 1;
pub const ZSTDcs_created: ZSTD_compressionStage_e = 0;
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
pub type ZSTD_EndDirective = libc::c_uint;
/* flush any remaining data _and_ close current frame.
                        * note that frame is only closed after compressed data is fully flushed (return value == 0).
                        * After that point, any additional data starts a new frame.
                        * note : each frame is independent (does not reference any content from previous frame). */
pub const ZSTD_e_end: ZSTD_EndDirective = 2;
/* flush any data provided so far,
                        * it creates (at least) one new block, that can be decoded immediately on reception;
                        * frame will continue: any future data can still reference previously compressed data, improving compression. */
pub const ZSTD_e_flush: ZSTD_EndDirective = 1;
/* collect more data, encoder decides when to output compressed result, for optimal compression ratio */
pub const ZSTD_e_continue: ZSTD_EndDirective = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_parameters {
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
}
pub type ZSTD_dictLoadMethod_e = libc::c_uint;
/* *< Reference dictionary content -- the dictionary buffer must outlive its users. */
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
/* *< Copy dictionary content internally */
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_frameProgression {
    pub ingested: libc::c_ulonglong,
    pub consumed: libc::c_ulonglong,
    pub produced: libc::c_ulonglong,
    pub flushed: libc::c_ulonglong,
    pub currentJobID: libc::c_uint,
    pub nbActiveWorkers: libc::c_uint,
}
/* ! POOL_function :
 *  The function type that can be added to a thread pool.
 */
pub type POOL_function
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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
pub union unnamed_3 {
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
pub type XXH64_hash_t = libc::c_ulonglong;
pub type ZSTD_dictTableLoadMethod_e = libc::c_uint;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct syncPoint_t {
    pub toLoad: size_t,
    pub flush: libc::c_int,
}
/* note : zero means empty */
/* ZSTDMT_parameter :
 * List of parameters that can be set using ZSTDMT_setMTCtxParameter() */
pub type ZSTDMT_parameter = libc::c_uint;
/* Enables rsyncable mode. */
pub const ZSTDMT_p_rsyncable: ZSTDMT_parameter = 2;
/* Each job may reload a part of previous job to enhance compressionr ratio; 0 == no overlap, 6(default) == use 1/8th of window, >=9 == use full window. This is a "sticky" parameter : its value will be re-used on next compression job */
pub const ZSTDMT_p_overlapLog: ZSTDMT_parameter = 1;
/* Each job is compressed in parallel. By default, this value is dynamically determined depending on compression parameters. Can be set explicitly here. */
pub const ZSTDMT_p_jobSize: ZSTDMT_parameter = 0;
/* *< this constant defers to stdlib's functions */
static mut ZSTD_defaultCMem: ZSTD_customMem =
    ZSTD_customMem{customAlloc: None,
                   customFree: None,
                   opaque: 0 as *const libc::c_void as *mut libc::c_void,};
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
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    /* don't use static : performance detrimental  */
    let one: unnamed_3 = unnamed_3{u: 1i32 as U32,};
    return one.c[0usize] as libc::c_uint;
}
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void,
                                 mut value: U32) {
    (*(memPtr as *mut unalign32)).v = value;
}
/* MEM_FORCE_MEMORY_ACCESS */
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
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
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> U32 {
    return (31i32 - val.leading_zeros() as i32) as U32;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_createCCtx(mut nbWorkers: libc::c_uint)
 -> *mut ZSTDMT_CCtx {
    return ZSTDMT_createCCtx_advanced(nbWorkers, ZSTD_defaultCMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_createCCtx_advanced(mut nbWorkers:
                                                        libc::c_uint,
                                                    mut cMem: ZSTD_customMem)
 -> *mut ZSTDMT_CCtx {
    let mut mtctx: *mut ZSTDMT_CCtx = 0 as *mut ZSTDMT_CCtx;
    let mut nbJobs: U32 = nbWorkers.wrapping_add(2i32 as libc::c_uint);
    let mut initError: libc::c_int = 0;
    if nbWorkers < 1i32 as libc::c_uint { return 0 as *mut ZSTDMT_CCtx }
    nbWorkers =
        if nbWorkers < 200i32 as libc::c_uint {
            nbWorkers
        } else { 200i32 as libc::c_uint };
    if 0 !=
           cMem.customAlloc.is_some() as libc::c_int ^
               cMem.customFree.is_some() as libc::c_int {
        return 0 as *mut ZSTDMT_CCtx
    }
    mtctx =
        ZSTD_calloc(::std::mem::size_of::<ZSTDMT_CCtx>() as libc::c_ulong,
                    cMem) as *mut ZSTDMT_CCtx;
    if mtctx.is_null() { return 0 as *mut ZSTDMT_CCtx }
    ZSTDMT_CCtxParam_setNbWorkers(&mut (*mtctx).params, nbWorkers);
    (*mtctx).cMem = cMem;
    (*mtctx).allJobsCompleted = 1i32 as libc::c_uint;
    (*mtctx).factory =
        POOL_create_advanced(nbWorkers as size_t, 0i32 as size_t, cMem);
    (*mtctx).jobs = ZSTDMT_createJobsTable(&mut nbJobs, cMem);
    (*mtctx).jobIDMask = nbJobs.wrapping_sub(1i32 as libc::c_uint);
    (*mtctx).bufPool = ZSTDMT_createBufferPool(nbWorkers, cMem);
    (*mtctx).cctxPool = ZSTDMT_createCCtxPool(nbWorkers as libc::c_int, cMem);
    (*mtctx).seqPool = ZSTDMT_createSeqPool(nbWorkers, cMem);
    initError = ZSTDMT_serialState_init(&mut (*mtctx).serial);
    (*mtctx).roundBuff = kNullRoundBuff;
    if 0 !=
           (*mtctx).factory.is_null() as libc::c_int |
               (*mtctx).jobs.is_null() as libc::c_int |
               (*mtctx).bufPool.is_null() as libc::c_int |
               (*mtctx).cctxPool.is_null() as libc::c_int |
               (*mtctx).seqPool.is_null() as libc::c_int | initError {
        ZSTDMT_freeCCtx(mtctx);
        return 0 as *mut ZSTDMT_CCtx
    }
    return mtctx;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_freeCCtx(mut mtctx: *mut ZSTDMT_CCtx)
 -> size_t {
    if mtctx.is_null() { return 0i32 as size_t }
    POOL_free((*mtctx).factory);
    ZSTDMT_releaseAllJobResources(mtctx);
    ZSTDMT_freeJobsTable((*mtctx).jobs,
                         (*mtctx).jobIDMask.wrapping_add(1i32 as
                                                             libc::c_uint),
                         (*mtctx).cMem);
    ZSTDMT_freeBufferPool((*mtctx).bufPool);
    ZSTDMT_freeCCtxPool((*mtctx).cctxPool);
    ZSTDMT_freeSeqPool((*mtctx).seqPool);
    ZSTDMT_serialState_free(&mut (*mtctx).serial);
    ZSTD_freeCDict((*mtctx).cdictLocal);
    if !(*mtctx).roundBuff.buffer.is_null() {
        ZSTD_free((*mtctx).roundBuff.buffer as *mut libc::c_void,
                  (*mtctx).cMem);
    }
    ZSTD_free(mtctx as *mut libc::c_void, (*mtctx).cMem);
    return 0i32 as size_t;
}
unsafe extern "C" fn ZSTDMT_serialState_free(mut serialState:
                                                 *mut serialState_t) {
    let mut cMem: ZSTD_customMem = (*serialState).params.customMem;
    pthread_mutex_destroy(&mut (*serialState).mutex);
    pthread_cond_destroy(&mut (*serialState).cond);
    pthread_mutex_destroy(&mut (*serialState).ldmWindowMutex);
    pthread_cond_destroy(&mut (*serialState).ldmWindowCond);
    ZSTD_free((*serialState).ldmState.hashTable as *mut libc::c_void, cMem);
    ZSTD_free((*serialState).ldmState.bucketOffsets as *mut libc::c_void,
              cMem);
}
unsafe extern "C" fn ZSTDMT_freeSeqPool(mut seqPool: *mut ZSTDMT_seqPool) {
    ZSTDMT_freeBufferPool(seqPool);
}
unsafe extern "C" fn ZSTDMT_freeBufferPool(mut bufPool:
                                               *mut ZSTDMT_bufferPool) {
    let mut u: libc::c_uint = 0;
    if bufPool.is_null() { return }
    u = 0i32 as libc::c_uint;
    while u < (*bufPool).totalBuffers {
        ZSTD_free((*bufPool).bTable[u as usize].start, (*bufPool).cMem);
        u = u.wrapping_add(1)
    }
    pthread_mutex_destroy(&mut (*bufPool).poolMutex);
    ZSTD_free(bufPool as *mut libc::c_void, (*bufPool).cMem);
}
/* note : all CCtx borrowed from the pool should be released back to the pool _before_ freeing the pool */
unsafe extern "C" fn ZSTDMT_freeCCtxPool(mut pool: *mut ZSTDMT_CCtxPool) {
    let mut cid: libc::c_int = 0;
    cid = 0i32;
    while cid < (*pool).totalCCtx {
        ZSTD_freeCCtx((*pool).cctx[cid as usize]);
        cid += 1
    }
    pthread_mutex_destroy(&mut (*pool).poolMutex);
    ZSTD_free(pool as *mut libc::c_void, (*pool).cMem);
}
unsafe extern "C" fn ZSTDMT_freeJobsTable(mut jobTable:
                                              *mut ZSTDMT_jobDescription,
                                          mut nbJobs: U32,
                                          mut cMem: ZSTD_customMem) {
    let mut jobNb: U32 = 0;
    if jobTable.is_null() { return }
    jobNb = 0i32 as U32;
    while jobNb < nbJobs {
        pthread_mutex_destroy(&mut (*jobTable.offset(jobNb as
                                                         isize)).job_mutex);
        pthread_cond_destroy(&mut (*jobTable.offset(jobNb as
                                                        isize)).job_cond);
        jobNb = jobNb.wrapping_add(1)
    }
    ZSTD_free(jobTable as *mut libc::c_void, cMem);
}
/* ZSTDMT_releaseAllJobResources() :
 * note : ensure all workers are killed first ! */
unsafe extern "C" fn ZSTDMT_releaseAllJobResources(mut mtctx:
                                                       *mut ZSTDMT_CCtx) {
    let mut jobID: libc::c_uint = 0;
    jobID = 0i32 as libc::c_uint;
    while jobID <= (*mtctx).jobIDMask {
        ZSTDMT_releaseBuffer((*mtctx).bufPool,
                             (*(*mtctx).jobs.offset(jobID as isize)).dstBuff);
        (*(*mtctx).jobs.offset(jobID as isize)).dstBuff = g_nullBuffer;
        (*(*mtctx).jobs.offset(jobID as isize)).cSize = 0i32 as size_t;
        jobID = jobID.wrapping_add(1)
    }
    memset((*mtctx).jobs as *mut libc::c_void, 0i32,
           ((*mtctx).jobIDMask.wrapping_add(1i32 as libc::c_uint) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<ZSTDMT_jobDescription>()
                                                as libc::c_ulong));
    (*mtctx).inBuff.buffer = g_nullBuffer;
    (*mtctx).inBuff.filled = 0i32 as size_t;
    (*mtctx).allJobsCompleted = 1i32 as libc::c_uint;
}
static mut g_nullBuffer: buffer_t =
    buffer_s{start: 0 as *const libc::c_void as *mut libc::c_void,
             capacity: 0i32 as size_t,};
/* store buffer for later re-use, up to pool capacity */
unsafe extern "C" fn ZSTDMT_releaseBuffer(mut bufPool: *mut ZSTDMT_bufferPool,
                                          mut buf: buffer_t) {
    if buf.start.is_null() { return }
    pthread_mutex_lock(&mut (*bufPool).poolMutex);
    if (*bufPool).nbBuffers < (*bufPool).totalBuffers {
        let fresh0 = (*bufPool).nbBuffers;
        (*bufPool).nbBuffers = (*bufPool).nbBuffers.wrapping_add(1);
        (*bufPool).bTable[fresh0 as usize] = buf;
        pthread_mutex_unlock(&mut (*bufPool).poolMutex);
        return
    }
    pthread_mutex_unlock(&mut (*bufPool).poolMutex);
    ZSTD_free(buf.start, (*bufPool).cMem);
}
static mut kNullRoundBuff: roundBuff_t =
    roundBuff_t{buffer: 0 as *const BYTE as *mut BYTE,
                capacity: 0i32 as size_t,
                pos: 0i32 as size_t,};
unsafe extern "C" fn ZSTDMT_serialState_init(mut serialState:
                                                 *mut serialState_t)
 -> libc::c_int {
    let mut initError: libc::c_int = 0i32;
    memset(serialState as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<serialState_t>() as libc::c_ulong);
    initError |=
        pthread_mutex_init(&mut (*serialState).mutex,
                           0 as *const pthread_mutexattr_t);
    initError |=
        pthread_cond_init(&mut (*serialState).cond,
                          0 as *const pthread_condattr_t);
    initError |=
        pthread_mutex_init(&mut (*serialState).ldmWindowMutex,
                           0 as *const pthread_mutexattr_t);
    initError |=
        pthread_cond_init(&mut (*serialState).ldmWindowCond,
                          0 as *const pthread_condattr_t);
    return initError;
}
unsafe extern "C" fn ZSTDMT_createSeqPool(mut nbWorkers: libc::c_uint,
                                          mut cMem: ZSTD_customMem)
 -> *mut ZSTDMT_seqPool {
    let seqPool: *mut ZSTDMT_seqPool =
        ZSTDMT_createBufferPool(nbWorkers, cMem);
    if seqPool.is_null() { return 0 as *mut ZSTDMT_seqPool }
    ZSTDMT_setNbSeq(seqPool, 0i32 as size_t);
    return seqPool;
}
unsafe extern "C" fn ZSTDMT_createBufferPool(mut nbWorkers: libc::c_uint,
                                             mut cMem: ZSTD_customMem)
 -> *mut ZSTDMT_bufferPool {
    let maxNbBuffers: libc::c_uint =
        (2i32 as
             libc::c_uint).wrapping_mul(nbWorkers).wrapping_add(3i32 as
                                                                    libc::c_uint);
    let bufPool: *mut ZSTDMT_bufferPool =
        ZSTD_calloc((::std::mem::size_of::<ZSTDMT_bufferPool>() as
                         libc::c_ulong).wrapping_add((maxNbBuffers.wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_uint)
                                                          as
                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<buffer_t>()
                                                                                          as
                                                                                          libc::c_ulong)),
                    cMem) as *mut ZSTDMT_bufferPool;
    if bufPool.is_null() { return 0 as *mut ZSTDMT_bufferPool }
    if 0 !=
           pthread_mutex_init(&mut (*bufPool).poolMutex,
                              0 as *const pthread_mutexattr_t) {
        ZSTD_free(bufPool as *mut libc::c_void, cMem);
        return 0 as *mut ZSTDMT_bufferPool
    }
    (*bufPool).bufferSize = (64i32 * (1i32 << 10i32)) as size_t;
    (*bufPool).totalBuffers = maxNbBuffers;
    (*bufPool).nbBuffers = 0i32 as libc::c_uint;
    (*bufPool).cMem = cMem;
    return bufPool;
}
unsafe extern "C" fn ZSTDMT_setNbSeq(seqPool: *mut ZSTDMT_seqPool,
                                     nbSeq: size_t) {
    ZSTDMT_setBufferSize(seqPool,
                         nbSeq.wrapping_mul(::std::mem::size_of::<rawSeq>() as
                                                libc::c_ulong));
}
/* ZSTDMT_setBufferSize() :
 * all future buffers provided by this buffer pool will have _at least_ this size
 * note : it's better for all buffers to have same size,
 * as they become freely interchangeable, reducing malloc/free usages and memory fragmentation */
unsafe extern "C" fn ZSTDMT_setBufferSize(bufPool: *mut ZSTDMT_bufferPool,
                                          bSize: size_t) {
    pthread_mutex_lock(&mut (*bufPool).poolMutex);
    (*bufPool).bufferSize = bSize;
    pthread_mutex_unlock(&mut (*bufPool).poolMutex);
}
/* ZSTDMT_createCCtxPool() :
 * implies nbWorkers >= 1 , checked by caller ZSTDMT_createCCtx() */
unsafe extern "C" fn ZSTDMT_createCCtxPool(mut nbWorkers: libc::c_int,
                                           mut cMem: ZSTD_customMem)
 -> *mut ZSTDMT_CCtxPool {
    let cctxPool: *mut ZSTDMT_CCtxPool =
        ZSTD_calloc((::std::mem::size_of::<ZSTDMT_CCtxPool>() as
                         libc::c_ulong).wrapping_add(((nbWorkers - 1i32) as
                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut ZSTD_CCtx>()
                                                                                          as
                                                                                          libc::c_ulong)),
                    cMem) as *mut ZSTDMT_CCtxPool;
    if cctxPool.is_null() { return 0 as *mut ZSTDMT_CCtxPool }
    if 0 !=
           pthread_mutex_init(&mut (*cctxPool).poolMutex,
                              0 as *const pthread_mutexattr_t) {
        ZSTD_free(cctxPool as *mut libc::c_void, cMem);
        return 0 as *mut ZSTDMT_CCtxPool
    }
    (*cctxPool).cMem = cMem;
    (*cctxPool).totalCCtx = nbWorkers;
    (*cctxPool).availCCtx = 1i32;
    (*cctxPool).cctx[0usize] = ZSTD_createCCtx_advanced(cMem);
    if (*cctxPool).cctx[0usize].is_null() {
        ZSTDMT_freeCCtxPool(cctxPool);
        return 0 as *mut ZSTDMT_CCtxPool
    }
    return cctxPool;
}
/* ZSTDMT_allocJobsTable()
 * allocate and init a job table.
 * update *nbJobsPtr to next power of 2 value, as size of table */
unsafe extern "C" fn ZSTDMT_createJobsTable(mut nbJobsPtr: *mut U32,
                                            mut cMem: ZSTD_customMem)
 -> *mut ZSTDMT_jobDescription {
    let nbJobsLog2: U32 =
        ZSTD_highbit32(*nbJobsPtr).wrapping_add(1i32 as libc::c_uint);
    let nbJobs: U32 = (1i32 << nbJobsLog2) as U32;
    let mut jobNb: U32 = 0;
    let jobTable: *mut ZSTDMT_jobDescription =
        ZSTD_calloc((nbJobs as
                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<ZSTDMT_jobDescription>()
                                                         as libc::c_ulong),
                    cMem) as *mut ZSTDMT_jobDescription;
    let mut initError: libc::c_int = 0i32;
    if jobTable.is_null() { return 0 as *mut ZSTDMT_jobDescription }
    *nbJobsPtr = nbJobs;
    jobNb = 0i32 as U32;
    while jobNb < nbJobs {
        initError |=
            pthread_mutex_init(&mut (*jobTable.offset(jobNb as
                                                          isize)).job_mutex,
                               0 as *const pthread_mutexattr_t);
        initError |=
            pthread_cond_init(&mut (*jobTable.offset(jobNb as
                                                         isize)).job_cond,
                              0 as *const pthread_condattr_t);
        jobNb = jobNb.wrapping_add(1)
    }
    if initError != 0i32 {
        ZSTDMT_freeJobsTable(jobTable, nbJobs, cMem);
        return 0 as *mut ZSTDMT_jobDescription
    }
    return jobTable;
}
/* ! ZSTDMT_CCtxParam_setNbWorkers()
 *  Set nbWorkers, and clamp it.
 *  Also reset jobSize and overlapLog */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_CCtxParam_setNbWorkers(mut params:
                                                           *mut ZSTD_CCtx_params,
                                                       mut nbWorkers:
                                                           libc::c_uint)
 -> size_t {
    if nbWorkers > 200i32 as libc::c_uint {
        nbWorkers = 200i32 as libc::c_uint
    }
    (*params).nbWorkers = nbWorkers as libc::c_int;
    (*params).overlapLog = 0i32;
    (*params).jobSize = 0i32 as size_t;
    return nbWorkers as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_sizeof_CCtx(mut mtctx: *mut ZSTDMT_CCtx)
 -> size_t {
    if mtctx.is_null() { return 0i32 as size_t }
    return (::std::mem::size_of::<ZSTDMT_CCtx>() as
                libc::c_ulong).wrapping_add(POOL_sizeof((*mtctx).factory)).wrapping_add(ZSTDMT_sizeof_bufferPool((*mtctx).bufPool)).wrapping_add(((*mtctx).jobIDMask.wrapping_add(1i32
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_uint)
                                                                                                                                                      as
                                                                                                                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<ZSTDMT_jobDescription>()
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_ulong)).wrapping_add(ZSTDMT_sizeof_CCtxPool((*mtctx).cctxPool)).wrapping_add(ZSTDMT_sizeof_seqPool((*mtctx).seqPool)).wrapping_add(ZSTD_sizeof_CDict((*mtctx).cdictLocal)).wrapping_add((*mtctx).roundBuff.capacity);
}
unsafe extern "C" fn ZSTDMT_sizeof_seqPool(mut seqPool: *mut ZSTDMT_seqPool)
 -> size_t {
    return ZSTDMT_sizeof_bufferPool(seqPool);
}
/* only works at initialization, not during compression */
unsafe extern "C" fn ZSTDMT_sizeof_bufferPool(mut bufPool:
                                                  *mut ZSTDMT_bufferPool)
 -> size_t {
    let poolSize: size_t =
        (::std::mem::size_of::<ZSTDMT_bufferPool>() as
             libc::c_ulong).wrapping_add(((*bufPool).totalBuffers.wrapping_sub(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                              as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<buffer_t>()
                                                                              as
                                                                              libc::c_ulong));
    let mut u: libc::c_uint = 0;
    let mut totalBufferSize: size_t = 0i32 as size_t;
    pthread_mutex_lock(&mut (*bufPool).poolMutex);
    u = 0i32 as libc::c_uint;
    while u < (*bufPool).totalBuffers {
        totalBufferSize =
            (totalBufferSize as
                 libc::c_ulong).wrapping_add((*bufPool).bTable[u as
                                                                   usize].capacity)
                as size_t as size_t;
        u = u.wrapping_add(1)
    }
    pthread_mutex_unlock(&mut (*bufPool).poolMutex);
    return poolSize.wrapping_add(totalBufferSize);
}
/* only works during initialization phase, not during compression */
unsafe extern "C" fn ZSTDMT_sizeof_CCtxPool(mut cctxPool:
                                                *mut ZSTDMT_CCtxPool)
 -> size_t {
    pthread_mutex_lock(&mut (*cctxPool).poolMutex);
    let nbWorkers: libc::c_uint = (*cctxPool).totalCCtx as libc::c_uint;
    let poolSize: size_t =
        (::std::mem::size_of::<ZSTDMT_CCtxPool>() as
             libc::c_ulong).wrapping_add((nbWorkers.wrapping_sub(1i32 as
                                                                     libc::c_uint)
                                              as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut ZSTD_CCtx>()
                                                                              as
                                                                              libc::c_ulong));
    let mut u: libc::c_uint = 0;
    let mut totalCCtxSize: size_t = 0i32 as size_t;
    u = 0i32 as libc::c_uint;
    while u < nbWorkers {
        totalCCtxSize =
            (totalCCtxSize as
                 libc::c_ulong).wrapping_add(ZSTD_sizeof_CCtx((*cctxPool).cctx[u
                                                                                   as
                                                                                   usize]))
                as size_t as size_t;
        u = u.wrapping_add(1)
    }
    pthread_mutex_unlock(&mut (*cctxPool).poolMutex);
    return poolSize.wrapping_add(totalCCtxSize);
}
/* ===   Simple one-pass compression function   === */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_compressCCtx(mut mtctx: *mut ZSTDMT_CCtx,
                                             mut dst: *mut libc::c_void,
                                             mut dstCapacity: size_t,
                                             mut src: *const libc::c_void,
                                             mut srcSize: size_t,
                                             mut compressionLevel:
                                                 libc::c_int) -> size_t {
    let mut params: ZSTD_parameters =
        ZSTD_getParams(compressionLevel, srcSize as libc::c_ulonglong,
                       0i32 as size_t);
    let overlapLog: libc::c_int =
        ZSTDMT_overlapLog_default(params.cParams.strategy);
    params.fParams.contentSizeFlag = 1i32;
    return ZSTDMT_compress_advanced(mtctx, dst, dstCapacity, src, srcSize,
                                    0 as *const ZSTD_CDict, params,
                                    overlapLog);
}
unsafe extern "C" fn ZSTDMT_overlapLog_default(mut strat: ZSTD_strategy)
 -> libc::c_int {
    match strat as libc::c_uint {
        9 => { return 9i32 }
        8 | 7 => { return 8i32 }
        6 | 5 => { return 7i32 }
        4 | 3 | 2 | 1 | _ => { }
    }
    return 6i32;
}
/* ===   Advanced functions and parameters  === */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_compress_advanced(mut mtctx: *mut ZSTDMT_CCtx,
                                                  mut dst: *mut libc::c_void,
                                                  mut dstCapacity: size_t,
                                                  mut src:
                                                      *const libc::c_void,
                                                  mut srcSize: size_t,
                                                  mut cdict:
                                                      *const ZSTD_CDict,
                                                  mut params: ZSTD_parameters,
                                                  mut overlapLog: libc::c_int)
 -> size_t {
    let mut cctxParams: ZSTD_CCtx_params = (*mtctx).params;
    cctxParams.cParams = params.cParams;
    cctxParams.fParams = params.fParams;
    cctxParams.overlapLog = overlapLog;
    return ZSTDMT_compress_advanced_internal(mtctx, dst, dstCapacity, src,
                                             srcSize, cdict, cctxParams);
}
/* ZSTDMT_compress_advanced_internal() :
 * This is a blocking function : it will only give back control to caller after finishing its compression job.
 */
unsafe extern "C" fn ZSTDMT_compress_advanced_internal(mut mtctx:
                                                           *mut ZSTDMT_CCtx,
                                                       mut dst:
                                                           *mut libc::c_void,
                                                       mut dstCapacity:
                                                           size_t,
                                                       mut src:
                                                           *const libc::c_void,
                                                       mut srcSize: size_t,
                                                       mut cdict:
                                                           *const ZSTD_CDict,
                                                       mut params:
                                                           ZSTD_CCtx_params)
 -> size_t {
    let jobParams: ZSTD_CCtx_params = ZSTDMT_initJobCCtxParams(params);
    let overlapSize: size_t = ZSTDMT_computeOverlapSize(params);
    let nbJobs: libc::c_uint =
        ZSTDMT_computeNbJobs(params, srcSize,
                             params.nbWorkers as libc::c_uint);
    let proposedJobSize: size_t =
        srcSize.wrapping_add(nbJobs.wrapping_sub(1i32 as libc::c_uint) as
                                 libc::c_ulong).wrapping_div(nbJobs as
                                                                 libc::c_ulong);
    /* avoid too small last block */
    let avgJobSize: size_t =
        if (proposedJobSize.wrapping_sub(1i32 as libc::c_ulong) &
                0x1ffffi32 as libc::c_ulong) < 0x7fffi32 as libc::c_ulong {
            proposedJobSize.wrapping_add(0xffffi32 as libc::c_ulong)
        } else { proposedJobSize };
    let srcStart: *const libc::c_char = src as *const libc::c_char;
    let mut remainingSrcSize: size_t = srcSize;
    /* presumes avgJobSize >= 256 KB, which should be the case */
    let compressWithinDst: libc::c_uint =
        if dstCapacity >= ZSTD_compressBound(srcSize) {
            nbJobs
        } else {
            dstCapacity.wrapping_div(ZSTD_compressBound(avgJobSize)) as
                libc::c_uint
        };
    let mut frameStartPos: size_t = 0i32 as size_t;
    let mut dstBufferPos: size_t = 0i32 as size_t;
    params.jobSize = avgJobSize as U32 as size_t;
    if 0 !=
           (nbJobs == 1i32 as libc::c_uint) as libc::c_int |
               (params.nbWorkers <= 1i32) as libc::c_int {
        let cctx: *mut ZSTD_CCtx = (*(*mtctx).cctxPool).cctx[0usize];
        if !cdict.is_null() {
            return ZSTD_compress_usingCDict_advanced(cctx, dst, dstCapacity,
                                                     src, srcSize, cdict,
                                                     jobParams.fParams)
        }
        return ZSTD_compress_advanced_internal(cctx, dst, dstCapacity, src,
                                               srcSize,
                                               0 as *const libc::c_void,
                                               0i32 as size_t, jobParams)
    }
    ZSTDMT_setBufferSize((*mtctx).bufPool, ZSTD_compressBound(avgJobSize));
    if 0 !=
           ZSTDMT_serialState_reset(&mut (*mtctx).serial, (*mtctx).seqPool,
                                    params, avgJobSize) {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    }
    let errcod: size_t = ZSTDMT_expandJobsTable(mtctx, nbJobs);
    if 0 != ERR_isError(errcod) { return errcod }
    let mut u: libc::c_uint = 0;
    u = 0i32 as libc::c_uint;
    while u < nbJobs {
        let jobSize: size_t =
            if remainingSrcSize < avgJobSize {
                remainingSrcSize
            } else { avgJobSize };
        let dstBufferCapacity: size_t = ZSTD_compressBound(jobSize);
        let dstAsBuffer: buffer_t =
            buffer_s{start:
                         (dst as
                              *mut libc::c_char).offset(dstBufferPos as isize)
                             as *mut libc::c_void,
                     capacity: dstBufferCapacity,};
        let dstBuffer: buffer_t =
            if u < compressWithinDst { dstAsBuffer } else { g_nullBuffer };
        let mut dictSize: size_t =
            if 0 != u { overlapSize } else { 0i32 as libc::c_ulong };
        let ref mut fresh1 = (*(*mtctx).jobs.offset(u as isize)).prefix.start;
        *fresh1 =
            srcStart.offset(frameStartPos as
                                isize).offset(-(dictSize as isize)) as
                *const libc::c_void;
        (*(*mtctx).jobs.offset(u as isize)).prefix.size = dictSize;
        let ref mut fresh2 = (*(*mtctx).jobs.offset(u as isize)).src.start;
        *fresh2 =
            srcStart.offset(frameStartPos as isize) as *const libc::c_void;
        (*(*mtctx).jobs.offset(u as isize)).src.size = jobSize;
        (*(*mtctx).jobs.offset(u as isize)).consumed = 0i32 as size_t;
        (*(*mtctx).jobs.offset(u as isize)).cSize = 0i32 as size_t;
        let ref mut fresh3 = (*(*mtctx).jobs.offset(u as isize)).cdict;
        *fresh3 =
            if u == 0i32 as libc::c_uint {
                cdict
            } else { 0 as *const ZSTD_CDict };
        (*(*mtctx).jobs.offset(u as isize)).fullFrameSize =
            srcSize as libc::c_ulonglong;
        (*(*mtctx).jobs.offset(u as isize)).params = jobParams;
        (*(*mtctx).jobs.offset(u as isize)).dstBuff = dstBuffer;
        let ref mut fresh4 = (*(*mtctx).jobs.offset(u as isize)).cctxPool;
        *fresh4 = (*mtctx).cctxPool;
        let ref mut fresh5 = (*(*mtctx).jobs.offset(u as isize)).bufPool;
        *fresh5 = (*mtctx).bufPool;
        let ref mut fresh6 = (*(*mtctx).jobs.offset(u as isize)).seqPool;
        *fresh6 = (*mtctx).seqPool;
        let ref mut fresh7 = (*(*mtctx).jobs.offset(u as isize)).serial;
        *fresh7 = &mut (*mtctx).serial;
        (*(*mtctx).jobs.offset(u as isize)).jobID = u;
        (*(*mtctx).jobs.offset(u as isize)).firstJob =
            (u == 0i32 as libc::c_uint) as libc::c_int as libc::c_uint;
        (*(*mtctx).jobs.offset(u as isize)).lastJob =
            (u == nbJobs.wrapping_sub(1i32 as libc::c_uint)) as libc::c_int as
                libc::c_uint;
        POOL_add((*mtctx).factory, Some(ZSTDMT_compressionJob),
                 &mut *(*mtctx).jobs.offset(u as isize) as
                     *mut ZSTDMT_jobDescription as *mut libc::c_void);
        frameStartPos =
            (frameStartPos as libc::c_ulong).wrapping_add(jobSize) as size_t
                as size_t;
        dstBufferPos =
            (dstBufferPos as libc::c_ulong).wrapping_add(dstBufferCapacity) as
                size_t as size_t;
        remainingSrcSize =
            (remainingSrcSize as libc::c_ulong).wrapping_sub(jobSize) as
                size_t as size_t;
        u = u.wrapping_add(1)
    }
    let mut error: size_t = 0i32 as size_t;
    let mut dstPos: size_t = 0i32 as size_t;
    let mut jobID: libc::c_uint = 0;
    jobID = 0i32 as libc::c_uint;
    while jobID < nbJobs {
        pthread_mutex_lock(&mut (*(*mtctx).jobs.offset(jobID as
                                                           isize)).job_mutex);
        while (*(*mtctx).jobs.offset(jobID as isize)).consumed <
                  (*(*mtctx).jobs.offset(jobID as isize)).src.size {
            pthread_cond_wait(&mut (*(*mtctx).jobs.offset(jobID as
                                                              isize)).job_cond,
                              &mut (*(*mtctx).jobs.offset(jobID as
                                                              isize)).job_mutex);
        }
        pthread_mutex_unlock(&mut (*(*mtctx).jobs.offset(jobID as
                                                             isize)).job_mutex);
        let cSize: size_t = (*(*mtctx).jobs.offset(jobID as isize)).cSize;
        if 0 != ERR_isError(cSize) { error = cSize }
        if 0 == error && dstPos.wrapping_add(cSize) > dstCapacity {
            error = -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
        }
        if 0 != jobID {
            if 0 == error {
                memmove((dst as *mut libc::c_char).offset(dstPos as isize) as
                            *mut libc::c_void,
                        (*(*mtctx).jobs.offset(jobID as isize)).dstBuff.start,
                        cSize);
            }
            if jobID >= compressWithinDst {
                ZSTDMT_releaseBuffer((*mtctx).bufPool,
                                     (*(*mtctx).jobs.offset(jobID as
                                                                isize)).dstBuff);
            }
        }
        (*(*mtctx).jobs.offset(jobID as isize)).dstBuff = g_nullBuffer;
        (*(*mtctx).jobs.offset(jobID as isize)).cSize = 0i32 as size_t;
        dstPos =
            (dstPos as libc::c_ulong).wrapping_add(cSize) as size_t as size_t;
        jobID = jobID.wrapping_add(1)
    }
    if 0 != params.fParams.checksumFlag {
        let checksum: U32 =
            ZSTD_XXH64_digest(&mut (*mtctx).serial.xxhState) as U32;
        if dstPos.wrapping_add(4i32 as libc::c_ulong) > dstCapacity {
            error = -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
        } else {
            MEM_writeLE32((dst as *mut libc::c_char).offset(dstPos as isize)
                              as *mut libc::c_void, checksum);
            dstPos =
                (dstPos as libc::c_ulong).wrapping_add(4i32 as libc::c_ulong)
                    as size_t as size_t
        }
    }
    0 == error;
    return if 0 != error { error } else { dstPos };
}
unsafe extern "C" fn ZSTDMT_computeNbJobs(mut params: ZSTD_CCtx_params,
                                          mut srcSize: size_t,
                                          mut nbWorkers: libc::c_uint)
 -> libc::c_uint {
    let jobSizeTarget: size_t =
        (1i32 as size_t) << ZSTDMT_computeTargetJobLog(params);
    let jobMaxSize: size_t = jobSizeTarget << 2i32;
    let passSizeMax: size_t =
        jobMaxSize.wrapping_mul(nbWorkers as libc::c_ulong);
    let multiplier: libc::c_uint =
        (srcSize.wrapping_div(passSizeMax) as
             libc::c_uint).wrapping_add(1i32 as libc::c_uint);
    let nbJobsLarge: libc::c_uint = multiplier.wrapping_mul(nbWorkers);
    let nbJobsMax: libc::c_uint =
        (srcSize.wrapping_div(jobSizeTarget) as
             libc::c_uint).wrapping_add(1i32 as libc::c_uint);
    let nbJobsSmall: libc::c_uint =
        if nbJobsMax < nbWorkers { nbJobsMax } else { nbWorkers };
    return if multiplier > 1i32 as libc::c_uint {
               nbJobsLarge
           } else { nbJobsSmall };
}
/* ------------------------------------------ */
/* =====   Multi-threaded compression   ===== */
/* ------------------------------------------ */
unsafe extern "C" fn ZSTDMT_computeTargetJobLog(params: ZSTD_CCtx_params)
 -> libc::c_uint {
    if 0 != params.ldmParams.enableLdm {
        return if 21i32 as libc::c_uint >
                      params.cParams.chainLog.wrapping_add(4i32 as
                                                               libc::c_uint) {
                   21i32 as libc::c_uint
               } else {
                   params.cParams.chainLog.wrapping_add(4i32 as libc::c_uint)
               }
    }
    return if 20i32 as libc::c_uint >
                  params.cParams.windowLog.wrapping_add(2i32 as libc::c_uint)
              {
               20i32 as libc::c_uint
           } else {
               params.cParams.windowLog.wrapping_add(2i32 as libc::c_uint)
           };
}
/* ZSTDMT_compressionJob() is a POOL_function type */
unsafe extern "C" fn ZSTDMT_compressionJob(mut jobDescription:
                                               *mut libc::c_void) {
    let mut current_block: u64;
    let job: *mut ZSTDMT_jobDescription =
        jobDescription as *mut ZSTDMT_jobDescription;
    /* do not modify job->params ! copy it, modify the copy */
    let mut jobParams: ZSTD_CCtx_params = (*job).params;
    let cctx: *mut ZSTD_CCtx = ZSTDMT_getCCtx((*job).cctxPool);
    let mut rawSeqStore: rawSeqStore_t = ZSTDMT_getSeq((*job).seqPool);
    let mut dstBuff: buffer_t = (*job).dstBuff;
    let mut lastCBlockSize: size_t = 0i32 as size_t;
    /* ressources */
    if cctx.is_null() {
        pthread_mutex_lock(&mut (*job).job_mutex);
        (*job).cSize =
            -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
        pthread_mutex_unlock(&mut (*job).job_mutex);
    } else {
        if dstBuff.start.is_null() {
            /* streaming job : doesn't provide a dstBuffer */
            dstBuff = ZSTDMT_getBuffer((*job).bufPool);
            if dstBuff.start.is_null() {
                pthread_mutex_lock(&mut (*job).job_mutex);
                (*job).cSize =
                    -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
                pthread_mutex_unlock(&mut (*job).job_mutex);
                current_block = 15798414024618506177;
            } else {
                (*job).dstBuff = dstBuff;
                current_block = 7149356873433890176;
            }
        } else { current_block = 7149356873433890176; }
        match current_block {
            15798414024618506177 => { }
            _ => {
                if 0 != jobParams.ldmParams.enableLdm &&
                       rawSeqStore.seq.is_null() {
                    pthread_mutex_lock(&mut (*job).job_mutex);
                    (*job).cSize =
                        -(ZSTD_error_memory_allocation as libc::c_int) as
                            size_t;
                    pthread_mutex_unlock(&mut (*job).job_mutex);
                } else {
                    if (*job).jobID != 0i32 as libc::c_uint {
                        jobParams.fParams.checksumFlag = 0i32
                    }
                    jobParams.ldmParams.enableLdm = 0i32 as U32;
                    /* init */
                    if !(*job).cdict.is_null() {
                        let initError: size_t =
                            ZSTD_compressBegin_advanced_internal(cctx,
                                                                 0 as
                                                                     *const libc::c_void,
                                                                 0i32 as
                                                                     size_t,
                                                                 ZSTD_dct_auto,
                                                                 ZSTD_dtlm_fast,
                                                                 (*job).cdict,
                                                                 jobParams,
                                                                 (*job).fullFrameSize);
                        if 0 != ERR_isError(initError) {
                            pthread_mutex_lock(&mut (*job).job_mutex);
                            (*job).cSize = initError;
                            pthread_mutex_unlock(&mut (*job).job_mutex);
                            current_block = 15798414024618506177;
                        } else { current_block = 1434579379687443766; }
                    } else {
                        /* srcStart points at reloaded section */
                        let pledgedSrcSize: U64 =
                            (if 0 != (*job).firstJob {
                                 (*job).fullFrameSize
                             } else { (*job).src.size as libc::c_ulonglong })
                                as U64;
                        let forceWindowError: size_t =
                            ZSTD_CCtxParam_setParameter(&mut jobParams,
                                                        ZSTD_c_experimentalParam3,
                                                        (0 == (*job).firstJob)
                                                            as libc::c_int);
                        if 0 != ERR_isError(forceWindowError) {
                            pthread_mutex_lock(&mut (*job).job_mutex);
                            (*job).cSize = forceWindowError;
                            pthread_mutex_unlock(&mut (*job).job_mutex);
                            current_block = 15798414024618506177;
                        } else {
                            let initError_0: size_t =
                                ZSTD_compressBegin_advanced_internal(cctx,
                                                                     (*job).prefix.start,
                                                                     (*job).prefix.size,
                                                                     ZSTD_dct_rawContent,
                                                                     ZSTD_dtlm_fast,
                                                                     0 as
                                                                         *const ZSTD_CDict,
                                                                     jobParams,
                                                                     pledgedSrcSize
                                                                         as
                                                                         libc::c_ulonglong);
                            /* load dictionary in "content-only" mode (no header analysis) */
                            /*cdict*/
                            if 0 != ERR_isError(initError_0) {
                                pthread_mutex_lock(&mut (*job).job_mutex);
                                (*job).cSize = initError_0;
                                pthread_mutex_unlock(&mut (*job).job_mutex);
                                current_block = 15798414024618506177;
                            } else { current_block = 1434579379687443766; }
                        }
                    }
                    match current_block {
                        15798414024618506177 => { }
                        _ => {
                            ZSTDMT_serialState_update((*job).serial, cctx,
                                                      rawSeqStore, (*job).src,
                                                      (*job).jobID);
                            if 0 == (*job).firstJob {
                                /* flush and overwrite frame header when it's not first job */
                                let hSize: size_t =
                                    ZSTD_compressContinue(cctx, dstBuff.start,
                                                          dstBuff.capacity,
                                                          (*job).src.start,
                                                          0i32 as size_t);
                                if 0 != ERR_isError(hSize) {
                                    pthread_mutex_lock(&mut (*job).job_mutex);
                                    (*job).cSize = hSize;
                                    pthread_mutex_unlock(&mut (*job).job_mutex);
                                    current_block = 15798414024618506177;
                                } else {
                                    ZSTD_invalidateRepCodes(cctx);
                                    current_block = 13325891313334703151;
                                }
                            } else { current_block = 13325891313334703151; }
                            match current_block {
                                15798414024618506177 => { }
                                _ => {
                                    /* compress */
                                    let chunkSize: size_t =
                                        (4i32 * (1i32 << 17i32)) as size_t;
                                    let nbChunks: libc::c_int =
                                        (*job).src.size.wrapping_add(chunkSize.wrapping_sub(1i32
                                                                                                as
                                                                                                libc::c_ulong)).wrapping_div(chunkSize)
                                            as libc::c_int;
                                    let mut ip: *const BYTE =
                                        (*job).src.start as *const BYTE;
                                    let ostart: *mut BYTE =
                                        dstBuff.start as *mut BYTE;
                                    let mut op: *mut BYTE = ostart;
                                    let mut oend: *mut BYTE =
                                        op.offset(dstBuff.capacity as isize);
                                    let mut chunkNb: libc::c_int = 0;
                                    ::std::mem::size_of::<size_t>() as
                                        libc::c_ulong >
                                        ::std::mem::size_of::<libc::c_int>()
                                            as libc::c_ulong;
                                    chunkNb = 1i32;
                                    loop  {
                                        if !(chunkNb < nbChunks) {
                                            current_block =
                                                14294131666767243020;
                                            break ;
                                        }
                                        let cSize: size_t =
                                            ZSTD_compressContinue(cctx,
                                                                  op as
                                                                      *mut libc::c_void,
                                                                  oend.wrapping_offset_from(op)
                                                                      as
                                                                      libc::c_long
                                                                      as
                                                                      size_t,
                                                                  ip as
                                                                      *const libc::c_void,
                                                                  chunkSize);
                                        if 0 != ERR_isError(cSize) {
                                            pthread_mutex_lock(&mut (*job).job_mutex);
                                            (*job).cSize = cSize;
                                            pthread_mutex_unlock(&mut (*job).job_mutex);
                                            current_block =
                                                15798414024618506177;
                                            break ;
                                        } else {
                                            ip =
                                                ip.offset(chunkSize as isize);
                                            op = op.offset(cSize as isize);
                                            pthread_mutex_lock(&mut (*job).job_mutex);
                                            (*job).cSize =
                                                ((*job).cSize as
                                                     libc::c_ulong).wrapping_add(cSize)
                                                    as size_t as size_t;
                                            (*job).consumed =
                                                chunkSize.wrapping_mul(chunkNb
                                                                           as
                                                                           libc::c_ulong);
                                            pthread_cond_signal(&mut (*job).job_cond);
                                            pthread_mutex_unlock(&mut (*job).job_mutex);
                                            chunkNb += 1
                                        }
                                    }
                                    match current_block {
                                        15798414024618506177 => { }
                                        _ => {
                                            if 0 !=
                                                   (nbChunks > 0i32) as
                                                       libc::c_int as
                                                       libc::c_uint |
                                                       (*job).lastJob {
                                                /*must output a "last block" flag*/
                                                let lastBlockSize1: size_t =
                                                    (*job).src.size &
                                                        chunkSize.wrapping_sub(1i32
                                                                                   as
                                                                                   libc::c_ulong);
                                                let lastBlockSize: size_t =
                                                    if 0 !=
                                                           (lastBlockSize1 ==
                                                                0i32 as
                                                                    libc::c_ulong)
                                                               as libc::c_int
                                                               &
                                                               ((*job).src.size
                                                                    >=
                                                                    chunkSize)
                                                                   as
                                                                   libc::c_int
                                                       {
                                                        chunkSize
                                                    } else { lastBlockSize1 };
                                                let cSize_0: size_t =
                                                    if 0 != (*job).lastJob {
                                                        ZSTD_compressEnd(cctx,
                                                                         op as
                                                                             *mut libc::c_void,
                                                                         oend.wrapping_offset_from(op)
                                                                             as
                                                                             libc::c_long
                                                                             as
                                                                             size_t,
                                                                         ip as
                                                                             *const libc::c_void,
                                                                         lastBlockSize)
                                                    } else {
                                                        ZSTD_compressContinue(cctx,
                                                                              op
                                                                                  as
                                                                                  *mut libc::c_void,
                                                                              oend.wrapping_offset_from(op)
                                                                                  as
                                                                                  libc::c_long
                                                                                  as
                                                                                  size_t,
                                                                              ip
                                                                                  as
                                                                                  *const libc::c_void,
                                                                              lastBlockSize)
                                                    };
                                                if 0 != ERR_isError(cSize_0) {
                                                    pthread_mutex_lock(&mut (*job).job_mutex);
                                                    (*job).cSize = cSize_0;
                                                    pthread_mutex_unlock(&mut (*job).job_mutex);
                                                } else {
                                                    lastCBlockSize = cSize_0
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
    }
    ZSTDMT_serialState_ensureFinished((*job).serial, (*job).jobID,
                                      (*job).cSize);
    (*job).prefix.size > 0i32 as libc::c_ulong;
    ZSTDMT_releaseSeq((*job).seqPool, rawSeqStore);
    ZSTDMT_releaseCCtx((*job).cctxPool, cctx);
    pthread_mutex_lock(&mut (*job).job_mutex);
    0 != ERR_isError((*job).cSize);
    (*job).cSize =
        ((*job).cSize as libc::c_ulong).wrapping_add(lastCBlockSize) as size_t
            as size_t;
    (*job).consumed = (*job).src.size;
    pthread_cond_signal(&mut (*job).job_cond);
    pthread_mutex_unlock(&mut (*job).job_mutex);
}
unsafe extern "C" fn ZSTDMT_getCCtx(mut cctxPool: *mut ZSTDMT_CCtxPool)
 -> *mut ZSTD_CCtx {
    pthread_mutex_lock(&mut (*cctxPool).poolMutex);
    if 0 != (*cctxPool).availCCtx {
        (*cctxPool).availCCtx -= 1;
        let cctx: *mut ZSTD_CCtx =
            (*cctxPool).cctx[(*cctxPool).availCCtx as usize];
        pthread_mutex_unlock(&mut (*cctxPool).poolMutex);
        return cctx
    }
    pthread_mutex_unlock(&mut (*cctxPool).poolMutex);
    return ZSTD_createCCtx_advanced((*cctxPool).cMem);
}
unsafe extern "C" fn ZSTDMT_releaseCCtx(mut pool: *mut ZSTDMT_CCtxPool,
                                        mut cctx: *mut ZSTD_CCtx) {
    if cctx.is_null() { return }
    pthread_mutex_lock(&mut (*pool).poolMutex);
    if (*pool).availCCtx < (*pool).totalCCtx {
        let fresh8 = (*pool).availCCtx;
        (*pool).availCCtx = (*pool).availCCtx + 1;
        (*pool).cctx[fresh8 as usize] = cctx
    } else { ZSTD_freeCCtx(cctx); }
    pthread_mutex_unlock(&mut (*pool).poolMutex);
}
unsafe extern "C" fn ZSTDMT_getSeq(mut seqPool: *mut ZSTDMT_seqPool)
 -> rawSeqStore_t {
    if (*seqPool).bufferSize == 0i32 as libc::c_ulong {
        return kNullRawSeqStore
    }
    return bufferToSeq(ZSTDMT_getBuffer(seqPool));
}
/* * ZSTDMT_getBuffer() :
 *  assumption : bufPool must be valid
 * @return : a buffer, with start pointer and size
 *  note: allocation may fail, in this case, start==NULL and size==0 */
unsafe extern "C" fn ZSTDMT_getBuffer(mut bufPool: *mut ZSTDMT_bufferPool)
 -> buffer_t {
    let bSize: size_t = (*bufPool).bufferSize;
    pthread_mutex_lock(&mut (*bufPool).poolMutex);
    if 0 != (*bufPool).nbBuffers {
        (*bufPool).nbBuffers = (*bufPool).nbBuffers.wrapping_sub(1);
        let buf: buffer_t = (*bufPool).bTable[(*bufPool).nbBuffers as usize];
        let availBufferSize: size_t = buf.capacity;
        (*bufPool).bTable[(*bufPool).nbBuffers as usize] = g_nullBuffer;
        if 0 !=
               (availBufferSize >= bSize) as libc::c_int &
                   (availBufferSize >> 3i32 <= bSize) as libc::c_int {
            pthread_mutex_unlock(&mut (*bufPool).poolMutex);
            return buf
        }
        ZSTD_free(buf.start, (*bufPool).cMem);
    }
    pthread_mutex_unlock(&mut (*bufPool).poolMutex);
    let mut buffer: buffer_t =
        buffer_s{start: 0 as *mut libc::c_void, capacity: 0,};
    let start: *mut libc::c_void = ZSTD_malloc(bSize, (*bufPool).cMem);
    buffer.start = start;
    buffer.capacity =
        if start.is_null() { 0i32 as libc::c_ulong } else { bSize };
    start.is_null();
    return buffer;
}
unsafe extern "C" fn bufferToSeq(mut buffer: buffer_t) -> rawSeqStore_t {
    let mut seq: rawSeqStore_t =
        rawSeqStore_t{seq: 0 as *mut rawSeq,
                      pos: 0i32 as size_t,
                      size: 0i32 as size_t,
                      capacity: 0i32 as size_t,};
    seq.seq = buffer.start as *mut rawSeq;
    seq.capacity =
        buffer.capacity.wrapping_div(::std::mem::size_of::<rawSeq>() as
                                         libc::c_ulong);
    return seq;
}
/* =====   Seq Pool Wrapper   ====== */
static mut kNullRawSeqStore: rawSeqStore_t =
    rawSeqStore_t{seq: 0 as *const rawSeq as *mut rawSeq,
                  pos: 0i32 as size_t,
                  size: 0i32 as size_t,
                  capacity: 0i32 as size_t,};
unsafe extern "C" fn ZSTDMT_releaseSeq(mut seqPool: *mut ZSTDMT_seqPool,
                                       mut seq: rawSeqStore_t) {
    ZSTDMT_releaseBuffer(seqPool, seqToBuffer(seq));
}
unsafe extern "C" fn seqToBuffer(mut seq: rawSeqStore_t) -> buffer_t {
    let mut buffer: buffer_t =
        buffer_s{start: 0 as *mut libc::c_void, capacity: 0,};
    buffer.start = seq.seq as *mut libc::c_void;
    buffer.capacity =
        seq.capacity.wrapping_mul(::std::mem::size_of::<rawSeq>() as
                                      libc::c_ulong);
    return buffer;
}
unsafe extern "C" fn ZSTDMT_serialState_ensureFinished(mut serialState:
                                                           *mut serialState_t,
                                                       mut jobID:
                                                           libc::c_uint,
                                                       mut cSize: size_t) {
    pthread_mutex_lock(&mut (*serialState).mutex);
    if (*serialState).nextJobID <= jobID {
        (*serialState).nextJobID = jobID.wrapping_add(1i32 as libc::c_uint);
        pthread_cond_broadcast(&mut (*serialState).cond);
        pthread_mutex_lock(&mut (*serialState).ldmWindowMutex);
        ZSTD_window_clear(&mut (*serialState).ldmWindow);
        pthread_cond_signal(&mut (*serialState).ldmWindowCond);
        pthread_mutex_unlock(&mut (*serialState).ldmWindowMutex);
    }
    pthread_mutex_unlock(&mut (*serialState).mutex);
}
/*-*************************************
*  Round buffer management
***************************************/
/* Max current allowed */
/* Maximum chunk size before overflow correction needs to be called again */
/* Maximum ending current index */
/* Maximum beginning lowLimit */
/* *
 * ZSTD_window_clear():
 * Clears the window containing the history by simply setting it to empty.
 */
unsafe extern "C" fn ZSTD_window_clear(mut window: *mut ZSTD_window_t) {
    let endT: size_t =
        (*window).nextSrc.wrapping_offset_from((*window).base) as libc::c_long
            as size_t;
    let end: U32 = endT as U32;
    (*window).lowLimit = end;
    (*window).dictLimit = end;
}
unsafe extern "C" fn ZSTDMT_serialState_update(mut serialState:
                                                   *mut serialState_t,
                                               mut jobCCtx: *mut ZSTD_CCtx,
                                               mut seqStore: rawSeqStore_t,
                                               mut src: range_t,
                                               mut jobID: libc::c_uint) {
    pthread_mutex_lock(&mut (*serialState).mutex);
    while (*serialState).nextJobID < jobID {
        pthread_cond_wait(&mut (*serialState).cond,
                          &mut (*serialState).mutex);
    }
    if (*serialState).nextJobID == jobID {
        if 0 != (*serialState).params.ldmParams.enableLdm {
            let mut error: size_t = 0;
            ZSTD_window_update(&mut (*serialState).ldmState.window, src.start,
                               src.size);
            error =
                ZSTD_ldm_generateSequences(&mut (*serialState).ldmState,
                                           &mut seqStore,
                                           &mut (*serialState).params.ldmParams,
                                           src.start, src.size);
            pthread_mutex_lock(&mut (*serialState).ldmWindowMutex);
            (*serialState).ldmWindow = (*serialState).ldmState.window;
            pthread_cond_signal(&mut (*serialState).ldmWindowCond);
            pthread_mutex_unlock(&mut (*serialState).ldmWindowMutex);
        }
        if 0 != (*serialState).params.fParams.checksumFlag &&
               src.size > 0i32 as libc::c_ulong {
            ZSTD_XXH64_update(&mut (*serialState).xxhState, src.start,
                              src.size);
        }
    }
    (*serialState).nextJobID = (*serialState).nextJobID.wrapping_add(1);
    pthread_cond_broadcast(&mut (*serialState).cond);
    pthread_mutex_unlock(&mut (*serialState).mutex);
    if seqStore.size > 0i32 as libc::c_ulong {
        let err: size_t =
            ZSTD_referenceExternalSequences(jobCCtx, seqStore.seq,
                                            seqStore.size);
    };
}
/* *
 * ZSTD_window_update():
 * Updates the window by appending [src, src + srcSize) to the window.
 * If it is not contiguous, the current prefix becomes the extDict, and we
 * forget about the extDict. Handles overlap of the prefix and extDict.
 * Returns non-zero if the segment is contiguous.
 */
unsafe extern "C" fn ZSTD_window_update(mut window: *mut ZSTD_window_t,
                                        mut src: *const libc::c_void,
                                        mut srcSize: size_t) -> U32 {
    let ip: *const BYTE = src as *const BYTE;
    let mut contiguous: U32 = 1i32 as U32;
    if src != (*window).nextSrc as *const libc::c_void {
        let distanceFromBase: size_t =
            (*window).nextSrc.wrapping_offset_from((*window).base) as
                libc::c_long as size_t;
        (*window).lowLimit = (*window).dictLimit;
        (*window).dictLimit = distanceFromBase as U32;
        (*window).dictBase = (*window).base;
        (*window).base = ip.offset(-(distanceFromBase as isize));
        if (*window).dictLimit.wrapping_sub((*window).lowLimit) <
               8i32 as libc::c_uint {
            (*window).lowLimit = (*window).dictLimit
        }
        contiguous = 0i32 as U32
    }
    (*window).nextSrc = ip.offset(srcSize as isize);
    if 0 !=
           (ip.offset(srcSize as isize) >
                (*window).dictBase.offset((*window).lowLimit as isize)) as
               libc::c_int &
               (ip < (*window).dictBase.offset((*window).dictLimit as isize))
                   as libc::c_int {
        let highInputIdx: ptrdiff_t =
            ip.offset(srcSize as
                          isize).wrapping_offset_from((*window).dictBase) as
                libc::c_long;
        let lowLimitMax: U32 =
            if highInputIdx > (*window).dictLimit as ptrdiff_t {
                (*window).dictLimit
            } else { highInputIdx as U32 };
        (*window).lowLimit = lowLimitMax
    }
    return contiguous;
}
/* Sets parameters relevant to the compression job,
 * initializing others to default values. */
unsafe extern "C" fn ZSTDMT_initJobCCtxParams(params: ZSTD_CCtx_params)
 -> ZSTD_CCtx_params {
    let mut jobParams: ZSTD_CCtx_params =
        ZSTD_CCtx_params_s{format: ZSTD_f_zstd1,
                           cParams:
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
                                                    noDictIDFlag: 0,},
                           compressionLevel: 0,
                           forceWindow: 0,
                           attachDictPref: ZSTD_dictDefaultAttach,
                           nbWorkers: 0,
                           jobSize: 0,
                           overlapLog: 0,
                           rsyncable: 0,
                           ldmParams:
                               ldmParams_t{enableLdm: 0,
                                           hashLog: 0,
                                           bucketSizeLog: 0,
                                           minMatchLength: 0,
                                           hashRateLog: 0,
                                           windowLog: 0,},
                           customMem:
                               ZSTD_customMem{customAlloc: None,
                                              customFree: None,
                                              opaque:
                                                  0 as *mut libc::c_void,},};
    memset(&mut jobParams as *mut ZSTD_CCtx_params as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZSTD_CCtx_params>() as libc::c_ulong);
    jobParams.cParams = params.cParams;
    jobParams.fParams = params.fParams;
    jobParams.compressionLevel = params.compressionLevel;
    return jobParams;
}
unsafe extern "C" fn ZSTDMT_computeOverlapSize(params: ZSTD_CCtx_params)
 -> size_t {
    let overlapRLog: libc::c_int =
        9i32 - ZSTDMT_overlapLog(params.overlapLog, params.cParams.strategy);
    let mut ovLog: libc::c_int =
        (if overlapRLog >= 8i32 {
             0i32 as libc::c_uint
         } else {
             params.cParams.windowLog.wrapping_sub(overlapRLog as
                                                       libc::c_uint)
         }) as libc::c_int;
    if 0 != params.ldmParams.enableLdm {
        ovLog =
            if params.cParams.windowLog <
                   ZSTDMT_computeTargetJobLog(params).wrapping_sub(2i32 as
                                                                       libc::c_uint)
               {
                params.cParams.windowLog
            } else {
                ZSTDMT_computeTargetJobLog(params).wrapping_sub(2i32 as
                                                                    libc::c_uint)
            }.wrapping_sub(overlapRLog as libc::c_uint) as libc::c_int
    }
    return if ovLog == 0i32 {
               0i32 as libc::c_ulong
           } else { (1i32 as size_t) << ovLog };
}
unsafe extern "C" fn ZSTDMT_overlapLog(mut ovlog: libc::c_int,
                                       mut strat: ZSTD_strategy)
 -> libc::c_int {
    if ovlog == 0i32 { return ZSTDMT_overlapLog_default(strat) }
    return ovlog;
}
unsafe extern "C" fn ZSTDMT_expandJobsTable(mut mtctx: *mut ZSTDMT_CCtx,
                                            mut nbWorkers: U32) -> size_t {
    let mut nbJobs: U32 = nbWorkers.wrapping_add(2i32 as libc::c_uint);
    if nbJobs > (*mtctx).jobIDMask.wrapping_add(1i32 as libc::c_uint) {
        ZSTDMT_freeJobsTable((*mtctx).jobs,
                             (*mtctx).jobIDMask.wrapping_add(1i32 as
                                                                 libc::c_uint),
                             (*mtctx).cMem);
        (*mtctx).jobIDMask = 0i32 as libc::c_uint;
        (*mtctx).jobs = ZSTDMT_createJobsTable(&mut nbJobs, (*mtctx).cMem);
        if (*mtctx).jobs.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        }
        (*mtctx).jobIDMask = nbJobs.wrapping_sub(1i32 as libc::c_uint)
    }
    return 0i32 as size_t;
}
unsafe extern "C" fn ZSTDMT_serialState_reset(mut serialState:
                                                  *mut serialState_t,
                                              mut seqPool:
                                                  *mut ZSTDMT_seqPool,
                                              mut params: ZSTD_CCtx_params,
                                              mut jobSize: size_t)
 -> libc::c_int {
    if 0 != params.ldmParams.enableLdm {
        ZSTD_ldm_adjustParameters(&mut params.ldmParams, &mut params.cParams);
        (*serialState).ldmState.hashPower =
            ZSTD_rollingHash_primePower(params.ldmParams.minMatchLength)
    } else {
        memset(&mut params.ldmParams as *mut ldmParams_t as *mut libc::c_void,
               0i32, ::std::mem::size_of::<ldmParams_t>() as libc::c_ulong);
    }
    (*serialState).nextJobID = 0i32 as libc::c_uint;
    if 0 != params.fParams.checksumFlag {
        ZSTD_XXH64_reset(&mut (*serialState).xxhState,
                         0i32 as libc::c_ulonglong);
    }
    if 0 != params.ldmParams.enableLdm {
        let mut cMem: ZSTD_customMem = params.customMem;
        let hashLog: libc::c_uint = params.ldmParams.hashLog;
        let hashSize: size_t =
            ((1i32 as size_t) <<
                 hashLog).wrapping_mul(::std::mem::size_of::<ldmEntry_t>() as
                                           libc::c_ulong);
        let bucketLog: libc::c_uint =
            params.ldmParams.hashLog.wrapping_sub(params.ldmParams.bucketSizeLog);
        let bucketSize: size_t = (1i32 as size_t) << bucketLog;
        let prevBucketLog: libc::c_uint =
            (*serialState).params.ldmParams.hashLog.wrapping_sub((*serialState).params.ldmParams.bucketSizeLog);
        ZSTDMT_setNbSeq(seqPool,
                        ZSTD_ldm_getMaxNbSeq(params.ldmParams, jobSize));
        ZSTD_window_clear(&mut (*serialState).ldmState.window);
        (*serialState).ldmWindow = (*serialState).ldmState.window;
        if (*serialState).ldmState.hashTable.is_null() ||
               (*serialState).params.ldmParams.hashLog < hashLog {
            ZSTD_free((*serialState).ldmState.hashTable as *mut libc::c_void,
                      cMem);
            (*serialState).ldmState.hashTable =
                ZSTD_malloc(hashSize, cMem) as *mut ldmEntry_t
        }
        if (*serialState).ldmState.bucketOffsets.is_null() ||
               prevBucketLog < bucketLog {
            ZSTD_free((*serialState).ldmState.bucketOffsets as
                          *mut libc::c_void, cMem);
            (*serialState).ldmState.bucketOffsets =
                ZSTD_malloc(bucketSize, cMem) as *mut BYTE
        }
        if (*serialState).ldmState.hashTable.is_null() ||
               (*serialState).ldmState.bucketOffsets.is_null() {
            return 1i32
        }
        memset((*serialState).ldmState.hashTable as *mut libc::c_void, 0i32,
               hashSize);
        memset((*serialState).ldmState.bucketOffsets as *mut libc::c_void,
               0i32, bucketSize);
    }
    (*serialState).params = params;
    (*serialState).params.jobSize = jobSize as U32 as size_t;
    return 0i32;
}
/* * ZSTD_rollingHash_primePower() :
 * Compute the primePower to be passed to ZSTD_rollingHash_rotate() for a hash
 * over a window of length bytes.
 */
unsafe extern "C" fn ZSTD_rollingHash_primePower(mut length: U32) -> U64 {
    return ZSTD_ipow(prime8bytes,
                     length.wrapping_sub(1i32 as libc::c_uint) as U64);
}
static mut prime8bytes: U64 = 0xcf1bbcdcb7a56463u64 as U64;
/* * ZSTD_ipow() :
 * Return base^exponent.
 */
unsafe extern "C" fn ZSTD_ipow(mut base: U64, mut exponent: U64) -> U64 {
    let mut power: U64 = 1i32 as U64;
    while 0 != exponent {
        if 0 != exponent & 1i32 as libc::c_ulong {
            power = (power as libc::c_ulong).wrapping_mul(base) as U64 as U64
        }
        exponent >>= 1i32;
        base = (base as libc::c_ulong).wrapping_mul(base) as U64 as U64
    }
    return power;
}
/* ===   Streaming functions   === */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_initCStream(mut mtctx: *mut ZSTDMT_CCtx,
                                            mut compressionLevel: libc::c_int)
 -> size_t {
    let params: ZSTD_parameters =
        ZSTD_getParams(compressionLevel,
                       0u64.wrapping_sub(1i32 as libc::c_ulonglong),
                       0i32 as size_t);
    /* retrieve sticky params */
    let mut cctxParams: ZSTD_CCtx_params = (*mtctx).params;
    cctxParams.cParams = params.cParams;
    cctxParams.fParams = params.fParams;
    return ZSTDMT_initCStream_internal(mtctx, 0 as *const libc::c_void,
                                       0i32 as size_t, ZSTD_dct_auto,
                                       0 as *const ZSTD_CDict, cctxParams,
                                       0u64.wrapping_sub(1i32 as
                                                             libc::c_ulonglong));
}
/* ! ZSTDMT_initCStream_internal() :
 *  Private use only. Init streaming operation.
 *  expects params to be valid.
 *  must receive dict, or cdict, or none, but not both.
 *  @return : 0, or an error code */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_initCStream_internal(mut mtctx:
                                                         *mut ZSTDMT_CCtx,
                                                     mut dict:
                                                         *const libc::c_void,
                                                     mut dictSize: size_t,
                                                     mut dictContentType:
                                                         ZSTD_dictContentType_e,
                                                     mut cdict:
                                                         *const ZSTD_CDict,
                                                     mut params:
                                                         ZSTD_CCtx_params,
                                                     mut pledgedSrcSize:
                                                         libc::c_ulonglong)
 -> size_t {
    if params.nbWorkers != (*mtctx).params.nbWorkers {
        let errcod: size_t =
            ZSTDMT_resize(mtctx, params.nbWorkers as libc::c_uint);
        if 0 != ERR_isError(errcod) { return errcod }
    }
    if params.jobSize != 0i32 as libc::c_ulong &&
           params.jobSize < (1i32 * (1i32 << 20i32)) as libc::c_ulong {
        params.jobSize = (1i32 * (1i32 << 20i32)) as size_t
    }
    if params.jobSize >
           (if 0 != MEM_32bits() {
                512i32 * (1i32 << 20i32)
            } else { 1024i32 * (1i32 << 20i32) }) as size_t {
        params.jobSize =
            (if 0 != MEM_32bits() {
                 512i32 * (1i32 << 20i32)
             } else { 1024i32 * (1i32 << 20i32) }) as size_t
    }
    (*mtctx).singleBlockingThread =
        (pledgedSrcSize <= (1i32 * (1i32 << 20i32)) as libc::c_ulonglong) as
            libc::c_int as libc::c_uint;
    if 0 != (*mtctx).singleBlockingThread {
        let singleThreadParams: ZSTD_CCtx_params =
            ZSTDMT_initJobCCtxParams(params);
        return ZSTD_initCStream_internal((*(*mtctx).cctxPool).cctx[0usize],
                                         dict, dictSize, cdict,
                                         singleThreadParams, pledgedSrcSize)
    }
    if (*mtctx).allJobsCompleted == 0i32 as libc::c_uint {
        ZSTDMT_waitForAllJobsCompleted(mtctx);
        ZSTDMT_releaseAllJobResources(mtctx);
        (*mtctx).allJobsCompleted = 1i32 as libc::c_uint
    }
    (*mtctx).params = params;
    (*mtctx).frameContentSize = pledgedSrcSize;
    if !dict.is_null() {
        ZSTD_freeCDict((*mtctx).cdictLocal);
        (*mtctx).cdictLocal =
            ZSTD_createCDict_advanced(dict, dictSize, ZSTD_dlm_byCopy,
                                      dictContentType, params.cParams,
                                      (*mtctx).cMem);
        (*mtctx).cdict = (*mtctx).cdictLocal;
        if (*mtctx).cdictLocal.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        }
    } else {
        ZSTD_freeCDict((*mtctx).cdictLocal);
        (*mtctx).cdictLocal = 0 as *mut ZSTD_CDict;
        (*mtctx).cdict = cdict
    }
    (*mtctx).targetPrefixSize = ZSTDMT_computeOverlapSize(params);
    (*mtctx).targetSectionSize = params.jobSize;
    if (*mtctx).targetSectionSize == 0i32 as libc::c_ulong {
        (*mtctx).targetSectionSize =
            (1u64 << ZSTDMT_computeTargetJobLog(params)) as size_t
    }
    if 0 != params.rsyncable {
        let jobSizeMB: U32 = ((*mtctx).targetSectionSize >> 20i32) as U32;
        let rsyncBits: U32 =
            ZSTD_highbit32(jobSizeMB).wrapping_add(20i32 as libc::c_uint);
        (*mtctx).rsync.hash = 0i32 as U64;
        (*mtctx).rsync.hitMask =
            (1u64 << rsyncBits).wrapping_sub(1i32 as libc::c_ulonglong) as
                U64;
        (*mtctx).rsync.primePower = ZSTD_rollingHash_primePower(32i32 as U32)
    }
    if (*mtctx).targetSectionSize < (*mtctx).targetPrefixSize {
        (*mtctx).targetSectionSize = (*mtctx).targetPrefixSize
    }
    ZSTDMT_setBufferSize((*mtctx).bufPool,
                         ZSTD_compressBound((*mtctx).targetSectionSize));
    let windowSize: size_t =
        (if 0 != (*mtctx).params.ldmParams.enableLdm {
             1u32 << (*mtctx).params.cParams.windowLog
         } else { 0i32 as libc::c_uint }) as size_t;
    let nbSlackBuffers: size_t =
        (2i32 +
             ((*mtctx).targetPrefixSize > 0i32 as libc::c_ulong) as
                 libc::c_int) as size_t;
    let slackSize: size_t =
        (*mtctx).targetSectionSize.wrapping_mul(nbSlackBuffers);
    let nbWorkers: size_t =
        (if (*mtctx).params.nbWorkers > 1i32 {
             (*mtctx).params.nbWorkers
         } else { 1i32 }) as size_t;
    let sectionsSize: size_t =
        (*mtctx).targetSectionSize.wrapping_mul(nbWorkers);
    let capacity: size_t =
        if windowSize > sectionsSize {
            windowSize
        } else { sectionsSize }.wrapping_add(slackSize);
    if (*mtctx).roundBuff.capacity < capacity {
        if !(*mtctx).roundBuff.buffer.is_null() {
            ZSTD_free((*mtctx).roundBuff.buffer as *mut libc::c_void,
                      (*mtctx).cMem);
        }
        (*mtctx).roundBuff.buffer =
            ZSTD_malloc(capacity, (*mtctx).cMem) as *mut BYTE;
        if (*mtctx).roundBuff.buffer.is_null() {
            (*mtctx).roundBuff.capacity = 0i32 as size_t;
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        }
        (*mtctx).roundBuff.capacity = capacity
    }
    (*mtctx).roundBuff.pos = 0i32 as size_t;
    (*mtctx).inBuff.buffer = g_nullBuffer;
    (*mtctx).inBuff.filled = 0i32 as size_t;
    (*mtctx).inBuff.prefix = kNullRange;
    (*mtctx).doneJobID = 0i32 as libc::c_uint;
    (*mtctx).nextJobID = 0i32 as libc::c_uint;
    (*mtctx).frameEnded = 0i32 as libc::c_uint;
    (*mtctx).allJobsCompleted = 0i32 as libc::c_uint;
    (*mtctx).consumed = 0i32 as libc::c_ulonglong;
    (*mtctx).produced = 0i32 as libc::c_ulonglong;
    if 0 !=
           ZSTDMT_serialState_reset(&mut (*mtctx).serial, (*mtctx).seqPool,
                                    params, (*mtctx).targetSectionSize) {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    }
    return 0i32 as size_t;
}
/* ------------------------------------------ */
/* =====          Worker thread         ===== */
/* ------------------------------------------ */
static mut kNullRange: range_t =
    range_t{start: 0 as *const libc::c_void, size: 0i32 as size_t,};
unsafe extern "C" fn ZSTDMT_waitForAllJobsCompleted(mut mtctx:
                                                        *mut ZSTDMT_CCtx) {
    while (*mtctx).doneJobID < (*mtctx).nextJobID {
        let jobID: libc::c_uint = (*mtctx).doneJobID & (*mtctx).jobIDMask;
        pthread_mutex_lock(&mut (*(*mtctx).jobs.offset(jobID as
                                                           isize)).job_mutex);
        while (*(*mtctx).jobs.offset(jobID as isize)).consumed <
                  (*(*mtctx).jobs.offset(jobID as isize)).src.size {
            pthread_cond_wait(&mut (*(*mtctx).jobs.offset(jobID as
                                                              isize)).job_cond,
                              &mut (*(*mtctx).jobs.offset(jobID as
                                                              isize)).job_mutex);
        }
        pthread_mutex_unlock(&mut (*(*mtctx).jobs.offset(jobID as
                                                             isize)).job_mutex);
        (*mtctx).doneJobID = (*mtctx).doneJobID.wrapping_add(1)
    };
}
/* ZSTDMT_resize() :
 * @return : error code if fails, 0 on success */
unsafe extern "C" fn ZSTDMT_resize(mut mtctx: *mut ZSTDMT_CCtx,
                                   mut nbWorkers: libc::c_uint) -> size_t {
    if 0 != POOL_resize((*mtctx).factory, nbWorkers as size_t) {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    }
    let errcod: size_t = ZSTDMT_expandJobsTable(mtctx, nbWorkers);
    if 0 != ERR_isError(errcod) { return errcod }
    (*mtctx).bufPool = ZSTDMT_expandBufferPool((*mtctx).bufPool, nbWorkers);
    if (*mtctx).bufPool.is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    }
    (*mtctx).cctxPool =
        ZSTDMT_expandCCtxPool((*mtctx).cctxPool, nbWorkers as libc::c_int);
    if (*mtctx).cctxPool.is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    }
    (*mtctx).seqPool = ZSTDMT_expandSeqPool((*mtctx).seqPool, nbWorkers);
    if (*mtctx).seqPool.is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    }
    ZSTDMT_CCtxParam_setNbWorkers(&mut (*mtctx).params, nbWorkers);
    return 0i32 as size_t;
}
unsafe extern "C" fn ZSTDMT_expandSeqPool(mut pool: *mut ZSTDMT_seqPool,
                                          mut nbWorkers: U32)
 -> *mut ZSTDMT_seqPool {
    return ZSTDMT_expandBufferPool(pool, nbWorkers);
}
unsafe extern "C" fn ZSTDMT_expandBufferPool(mut srcBufPool:
                                                 *mut ZSTDMT_bufferPool,
                                             mut nbWorkers: U32)
 -> *mut ZSTDMT_bufferPool {
    let maxNbBuffers: libc::c_uint =
        (2i32 as
             libc::c_uint).wrapping_mul(nbWorkers).wrapping_add(3i32 as
                                                                    libc::c_uint);
    if srcBufPool.is_null() { return 0 as *mut ZSTDMT_bufferPool }
    if (*srcBufPool).totalBuffers >= maxNbBuffers { return srcBufPool }
    let cMem: ZSTD_customMem = (*srcBufPool).cMem;
    let bSize: size_t = (*srcBufPool).bufferSize;
    let mut newBufPool: *mut ZSTDMT_bufferPool = 0 as *mut ZSTDMT_bufferPool;
    ZSTDMT_freeBufferPool(srcBufPool);
    newBufPool = ZSTDMT_createBufferPool(nbWorkers, cMem);
    if newBufPool.is_null() { return newBufPool }
    ZSTDMT_setBufferSize(newBufPool, bSize);
    return newBufPool;
}
unsafe extern "C" fn ZSTDMT_expandCCtxPool(mut srcPool: *mut ZSTDMT_CCtxPool,
                                           mut nbWorkers: libc::c_int)
 -> *mut ZSTDMT_CCtxPool {
    if srcPool.is_null() { return 0 as *mut ZSTDMT_CCtxPool }
    if nbWorkers <= (*srcPool).totalCCtx { return srcPool }
    let cMem: ZSTD_customMem = (*srcPool).cMem;
    ZSTDMT_freeCCtxPool(srcPool);
    return ZSTDMT_createCCtxPool(nbWorkers, cMem);
}
/* *< if srcSize is not known at reset time, use ZSTD_CONTENTSIZE_UNKNOWN. Note: for compatibility with older programs, 0 means the same as ZSTD_CONTENTSIZE_UNKNOWN, but it will change in the future to mean "empty" */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_resetCStream(mut mtctx: *mut ZSTDMT_CCtx,
                                             mut pledgedSrcSize:
                                                 libc::c_ulonglong)
 -> size_t {
    if 0 == pledgedSrcSize {
        pledgedSrcSize = 0u64.wrapping_sub(1i32 as libc::c_ulonglong)
    }
    return ZSTDMT_initCStream_internal(mtctx, 0 as *const libc::c_void,
                                       0i32 as size_t, ZSTD_dct_auto,
                                       0 as *const ZSTD_CDict,
                                       (*mtctx).params, pledgedSrcSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_nextInputSizeHint(mut mtctx:
                                                      *const ZSTDMT_CCtx)
 -> size_t {
    let mut hintInSize: size_t =
        (*mtctx).targetSectionSize.wrapping_sub((*mtctx).inBuff.filled);
    if hintInSize == 0i32 as libc::c_ulong {
        hintInSize = (*mtctx).targetSectionSize
    }
    return hintInSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_compressStream(mut mtctx: *mut ZSTDMT_CCtx,
                                               mut output:
                                                   *mut ZSTD_outBuffer,
                                               mut input: *mut ZSTD_inBuffer)
 -> size_t {
    let errcod: size_t =
        ZSTDMT_compressStream_generic(mtctx, output, input, ZSTD_e_continue);
    if 0 != ERR_isError(errcod) { return errcod }
    return (*mtctx).targetSectionSize.wrapping_sub((*mtctx).inBuff.filled);
}
/* ! ZSTDMT_compressStream_generic() :
 *  Combines ZSTDMT_compressStream() with optional ZSTDMT_flushStream() or ZSTDMT_endStream()
 *  depending on flush directive.
 * @return : minimum amount of data still to be flushed
 *           0 if fully flushed
 *           or an error code
 *  note : needs to be init using any ZSTD_initCStream*() variant */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_compressStream_generic(mut mtctx:
                                                           *mut ZSTDMT_CCtx,
                                                       mut output:
                                                           *mut ZSTD_outBuffer,
                                                       mut input:
                                                           *mut ZSTD_inBuffer,
                                                       mut endOp:
                                                           ZSTD_EndDirective)
 -> size_t {
    let mut forwardInputProgress: libc::c_uint = 0i32 as libc::c_uint;
    if 0 != (*mtctx).singleBlockingThread {
        return ZSTD_compressStream_generic((*(*mtctx).cctxPool).cctx[0usize],
                                           output, input, endOp)
    }
    if 0 != (*mtctx).frameEnded &&
           endOp as libc::c_uint ==
               ZSTD_e_continue as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    }
    if 0 == (*mtctx).params.rsyncable &&
           (*mtctx).nextJobID == 0i32 as libc::c_uint &&
           (*mtctx).inBuff.filled == 0i32 as libc::c_ulong &&
           0 == (*mtctx).jobReady &&
           endOp as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint
           &&
           (*output).size.wrapping_sub((*output).pos) >=
               ZSTD_compressBound((*input).size.wrapping_sub((*input).pos)) {
        let cSize: size_t =
            ZSTDMT_compress_advanced_internal(mtctx,
                                              ((*output).dst as
                                                   *mut libc::c_char).offset((*output).pos
                                                                                 as
                                                                                 isize)
                                                  as *mut libc::c_void,
                                              (*output).size.wrapping_sub((*output).pos),
                                              ((*input).src as
                                                   *const libc::c_char).offset((*input).pos
                                                                                   as
                                                                                   isize)
                                                  as *const libc::c_void,
                                              (*input).size.wrapping_sub((*input).pos),
                                              (*mtctx).cdict,
                                              (*mtctx).params);
        if 0 != ERR_isError(cSize) { return cSize }
        (*input).pos = (*input).size;
        (*output).pos =
            ((*output).pos as libc::c_ulong).wrapping_add(cSize) as size_t as
                size_t;
        (*mtctx).allJobsCompleted = 1i32 as libc::c_uint;
        (*mtctx).frameEnded = 1i32 as libc::c_uint;
        return 0i32 as size_t
    }
    if 0 == (*mtctx).jobReady && (*input).size > (*input).pos {
        if (*mtctx).inBuff.buffer.start.is_null() {
            0 == ZSTDMT_tryGetInputRange(mtctx);
        }
        if !(*mtctx).inBuff.buffer.start.is_null() {
            let syncPoint: syncPoint_t =
                findSynchronizationPoint(mtctx, *input);
            if 0 != syncPoint.flush &&
                   endOp as libc::c_uint ==
                       ZSTD_e_continue as libc::c_int as libc::c_uint {
                endOp = ZSTD_e_flush
            }
            memcpy(((*mtctx).inBuff.buffer.start as
                        *mut libc::c_char).offset((*mtctx).inBuff.filled as
                                                      isize) as
                       *mut libc::c_void,
                   ((*input).src as
                        *const libc::c_char).offset((*input).pos as isize) as
                       *const libc::c_void, syncPoint.toLoad);
            (*input).pos =
                ((*input).pos as libc::c_ulong).wrapping_add(syncPoint.toLoad)
                    as size_t as size_t;
            (*mtctx).inBuff.filled =
                ((*mtctx).inBuff.filled as
                     libc::c_ulong).wrapping_add(syncPoint.toLoad) as size_t
                    as size_t;
            forwardInputProgress =
                (syncPoint.toLoad > 0i32 as libc::c_ulong) as libc::c_int as
                    libc::c_uint
        }
        if (*input).pos < (*input).size &&
               endOp as libc::c_uint ==
                   ZSTD_e_end as libc::c_int as libc::c_uint {
            endOp = ZSTD_e_flush
        }
    }
    if 0 != (*mtctx).jobReady ||
           (*mtctx).inBuff.filled >= (*mtctx).targetSectionSize ||
           endOp as libc::c_uint !=
               ZSTD_e_continue as libc::c_int as libc::c_uint &&
               (*mtctx).inBuff.filled > 0i32 as libc::c_ulong ||
           endOp as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint
               && 0 == (*mtctx).frameEnded {
        let jobSize: size_t = (*mtctx).inBuff.filled;
        let errcod: size_t =
            ZSTDMT_createCompressionJob(mtctx, jobSize, endOp);
        if 0 != ERR_isError(errcod) { return errcod }
    }
    let remainingToFlush: size_t =
        ZSTDMT_flushProduced(mtctx, output,
                             (0 == forwardInputProgress) as libc::c_int as
                                 libc::c_uint, endOp);
    if (*input).pos < (*input).size {
        return if remainingToFlush > 1i32 as libc::c_ulong {
                   remainingToFlush
               } else { 1i32 as libc::c_ulong }
    }
    return remainingToFlush;
}
/* ! ZSTDMT_flushProduced() :
 *  flush whatever data has been produced but not yet flushed in current job.
 *  move to next job if current one is fully flushed.
 * `output` : `pos` will be updated with amount of data flushed .
 * `blockToFlush` : if >0, the function will block and wait if there is no data available to flush .
 * @return : amount of data remaining within internal buffer, 0 if no more, 1 if unknown but > 0, or an error code */
unsafe extern "C" fn ZSTDMT_flushProduced(mut mtctx: *mut ZSTDMT_CCtx,
                                          mut output: *mut ZSTD_outBuffer,
                                          mut blockToFlush: libc::c_uint,
                                          mut end: ZSTD_EndDirective)
 -> size_t {
    let wJobID: libc::c_uint = (*mtctx).doneJobID & (*mtctx).jobIDMask;
    pthread_mutex_lock(&mut (*(*mtctx).jobs.offset(wJobID as
                                                       isize)).job_mutex);
    if 0 != blockToFlush && (*mtctx).doneJobID < (*mtctx).nextJobID {
        while (*(*mtctx).jobs.offset(wJobID as isize)).dstFlushed ==
                  (*(*mtctx).jobs.offset(wJobID as isize)).cSize {
            /* nothing to flush */
            if (*(*mtctx).jobs.offset(wJobID as isize)).consumed ==
                   (*(*mtctx).jobs.offset(wJobID as isize)).src.size {
                break ;
            }
            pthread_cond_wait(&mut (*(*mtctx).jobs.offset(wJobID as
                                                              isize)).job_cond,
                              &mut (*(*mtctx).jobs.offset(wJobID as
                                                              isize)).job_mutex);
        }
    }
    let mut cSize: size_t = (*(*mtctx).jobs.offset(wJobID as isize)).cSize;
    let srcConsumed: size_t =
        (*(*mtctx).jobs.offset(wJobID as isize)).consumed;
    let srcSize: size_t = (*(*mtctx).jobs.offset(wJobID as isize)).src.size;
    pthread_mutex_unlock(&mut (*(*mtctx).jobs.offset(wJobID as
                                                         isize)).job_mutex);
    if 0 != ERR_isError(cSize) {
        ZSTDMT_waitForAllJobsCompleted(mtctx);
        ZSTDMT_releaseAllJobResources(mtctx);
        return cSize
    }
    if srcConsumed == srcSize &&
           0 != (*(*mtctx).jobs.offset(wJobID as isize)).frameChecksumNeeded {
        let checksum: U32 =
            ZSTD_XXH64_digest(&mut (*mtctx).serial.xxhState) as U32;
        MEM_writeLE32(((*(*mtctx).jobs.offset(wJobID as isize)).dstBuff.start
                           as
                           *mut libc::c_char).offset((*(*mtctx).jobs.offset(wJobID
                                                                                as
                                                                                isize)).cSize
                                                         as isize) as
                          *mut libc::c_void, checksum);
        cSize =
            (cSize as libc::c_ulong).wrapping_add(4i32 as libc::c_ulong) as
                size_t as size_t;
        let ref mut fresh9 = (*(*mtctx).jobs.offset(wJobID as isize)).cSize;
        *fresh9 =
            (*fresh9 as libc::c_ulong).wrapping_add(4i32 as libc::c_ulong) as
                size_t as size_t;
        (*(*mtctx).jobs.offset(wJobID as isize)).frameChecksumNeeded =
            0i32 as libc::c_uint
    }
    if cSize > 0i32 as libc::c_ulong {
        let toFlush: size_t =
            if cSize.wrapping_sub((*(*mtctx).jobs.offset(wJobID as
                                                             isize)).dstFlushed)
                   < (*output).size.wrapping_sub((*output).pos) {
                cSize.wrapping_sub((*(*mtctx).jobs.offset(wJobID as
                                                              isize)).dstFlushed)
            } else { (*output).size.wrapping_sub((*output).pos) };
        memcpy(((*output).dst as
                    *mut libc::c_char).offset((*output).pos as isize) as
                   *mut libc::c_void,
               ((*(*mtctx).jobs.offset(wJobID as isize)).dstBuff.start as
                    *const libc::c_char).offset((*(*mtctx).jobs.offset(wJobID
                                                                           as
                                                                           isize)).dstFlushed
                                                    as isize) as
                   *const libc::c_void, toFlush);
        (*output).pos =
            ((*output).pos as libc::c_ulong).wrapping_add(toFlush) as size_t
                as size_t;
        let ref mut fresh10 =
            (*(*mtctx).jobs.offset(wJobID as isize)).dstFlushed;
        *fresh10 =
            (*fresh10 as libc::c_ulong).wrapping_add(toFlush) as size_t as
                size_t;
        if srcConsumed == srcSize &&
               (*(*mtctx).jobs.offset(wJobID as isize)).dstFlushed == cSize {
            ZSTDMT_releaseBuffer((*mtctx).bufPool,
                                 (*(*mtctx).jobs.offset(wJobID as
                                                            isize)).dstBuff);
            (*(*mtctx).jobs.offset(wJobID as isize)).dstBuff = g_nullBuffer;
            (*(*mtctx).jobs.offset(wJobID as isize)).cSize = 0i32 as size_t;
            (*mtctx).consumed =
                (*mtctx).consumed.wrapping_add(srcSize as libc::c_ulonglong);
            (*mtctx).produced =
                (*mtctx).produced.wrapping_add(cSize as libc::c_ulonglong);
            (*mtctx).doneJobID = (*mtctx).doneJobID.wrapping_add(1)
        }
    }
    if cSize > (*(*mtctx).jobs.offset(wJobID as isize)).dstFlushed {
        return cSize.wrapping_sub((*(*mtctx).jobs.offset(wJobID as
                                                             isize)).dstFlushed)
    }
    if srcSize > srcConsumed { return 1i32 as size_t }
    if (*mtctx).doneJobID < (*mtctx).nextJobID { return 1i32 as size_t }
    if 0 != (*mtctx).jobReady { return 1i32 as size_t }
    if (*mtctx).inBuff.filled > 0i32 as libc::c_ulong {
        return 1i32 as size_t
    }
    (*mtctx).allJobsCompleted = (*mtctx).frameEnded;
    if end as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint {
        return (0 == (*mtctx).frameEnded) as libc::c_int as size_t
    }
    return 0i32 as size_t;
}
unsafe extern "C" fn ZSTDMT_createCompressionJob(mut mtctx: *mut ZSTDMT_CCtx,
                                                 mut srcSize: size_t,
                                                 mut endOp: ZSTD_EndDirective)
 -> size_t {
    let jobID: libc::c_uint = (*mtctx).nextJobID & (*mtctx).jobIDMask;
    let endFrame: libc::c_int =
        (endOp as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint)
            as libc::c_int;
    if (*mtctx).nextJobID >
           (*mtctx).doneJobID.wrapping_add((*mtctx).jobIDMask) {
        return 0i32 as size_t
    }
    if 0 == (*mtctx).jobReady {
        let mut src: *const BYTE =
            (*mtctx).inBuff.buffer.start as *const BYTE;
        let ref mut fresh11 =
            (*(*mtctx).jobs.offset(jobID as isize)).src.start;
        *fresh11 = src as *const libc::c_void;
        (*(*mtctx).jobs.offset(jobID as isize)).src.size = srcSize;
        (*(*mtctx).jobs.offset(jobID as isize)).prefix =
            (*mtctx).inBuff.prefix;
        (*(*mtctx).jobs.offset(jobID as isize)).consumed = 0i32 as size_t;
        (*(*mtctx).jobs.offset(jobID as isize)).cSize = 0i32 as size_t;
        (*(*mtctx).jobs.offset(jobID as isize)).params = (*mtctx).params;
        let ref mut fresh12 = (*(*mtctx).jobs.offset(jobID as isize)).cdict;
        *fresh12 =
            if (*mtctx).nextJobID == 0i32 as libc::c_uint {
                (*mtctx).cdict
            } else { 0 as *const ZSTD_CDict };
        (*(*mtctx).jobs.offset(jobID as isize)).fullFrameSize =
            (*mtctx).frameContentSize;
        (*(*mtctx).jobs.offset(jobID as isize)).dstBuff = g_nullBuffer;
        let ref mut fresh13 =
            (*(*mtctx).jobs.offset(jobID as isize)).cctxPool;
        *fresh13 = (*mtctx).cctxPool;
        let ref mut fresh14 = (*(*mtctx).jobs.offset(jobID as isize)).bufPool;
        *fresh14 = (*mtctx).bufPool;
        let ref mut fresh15 = (*(*mtctx).jobs.offset(jobID as isize)).seqPool;
        *fresh15 = (*mtctx).seqPool;
        let ref mut fresh16 = (*(*mtctx).jobs.offset(jobID as isize)).serial;
        *fresh16 = &mut (*mtctx).serial;
        (*(*mtctx).jobs.offset(jobID as isize)).jobID = (*mtctx).nextJobID;
        (*(*mtctx).jobs.offset(jobID as isize)).firstJob =
            ((*mtctx).nextJobID == 0i32 as libc::c_uint) as libc::c_int as
                libc::c_uint;
        (*(*mtctx).jobs.offset(jobID as isize)).lastJob =
            endFrame as libc::c_uint;
        (*(*mtctx).jobs.offset(jobID as isize)).frameChecksumNeeded =
            (0 != (*mtctx).params.fParams.checksumFlag && 0 != endFrame &&
                 (*mtctx).nextJobID > 0i32 as libc::c_uint) as libc::c_int as
                libc::c_uint;
        (*(*mtctx).jobs.offset(jobID as isize)).dstFlushed = 0i32 as size_t;
        (*mtctx).roundBuff.pos =
            ((*mtctx).roundBuff.pos as libc::c_ulong).wrapping_add(srcSize) as
                size_t as size_t;
        (*mtctx).inBuff.buffer = g_nullBuffer;
        (*mtctx).inBuff.filled = 0i32 as size_t;
        if 0 == endFrame {
            let newPrefixSize: size_t =
                if srcSize < (*mtctx).targetPrefixSize {
                    srcSize
                } else { (*mtctx).targetPrefixSize };
            (*mtctx).inBuff.prefix.start =
                src.offset(srcSize as isize).offset(-(newPrefixSize as isize))
                    as *const libc::c_void;
            (*mtctx).inBuff.prefix.size = newPrefixSize
        } else {
            (*mtctx).inBuff.prefix = kNullRange;
            (*mtctx).frameEnded = endFrame as libc::c_uint;
            if (*mtctx).nextJobID == 0i32 as libc::c_uint {
                (*mtctx).params.fParams.checksumFlag = 0i32
            }
        }
        if srcSize == 0i32 as libc::c_ulong &&
               (*mtctx).nextJobID > 0i32 as libc::c_uint {
            ZSTDMT_writeLastEmptyBlock((*mtctx).jobs.offset(jobID as isize));
            (*mtctx).nextJobID = (*mtctx).nextJobID.wrapping_add(1);
            return 0i32 as size_t
        }
    }
    if 0 !=
           POOL_tryAdd((*mtctx).factory, Some(ZSTDMT_compressionJob),
                       &mut *(*mtctx).jobs.offset(jobID as isize) as
                           *mut ZSTDMT_jobDescription as *mut libc::c_void) {
        (*mtctx).nextJobID = (*mtctx).nextJobID.wrapping_add(1);
        (*mtctx).jobReady = 0i32
    } else { (*mtctx).jobReady = 1i32 }
    return 0i32 as size_t;
}
/* ZSTDMT_writeLastEmptyBlock()
 * Write a single empty block with an end-of-frame to finish a frame.
 * Job must be created from streaming variant.
 * This function is always successfull if expected conditions are fulfilled.
 */
unsafe extern "C" fn ZSTDMT_writeLastEmptyBlock(mut job:
                                                    *mut ZSTDMT_jobDescription) {
    (*job).dstBuff = ZSTDMT_getBuffer((*job).bufPool);
    if (*job).dstBuff.start.is_null() {
        (*job).cSize =
            -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
        return
    }
    (*job).src = kNullRange;
    (*job).cSize =
        ZSTD_writeLastEmptyBlock((*job).dstBuff.start,
                                 (*job).dstBuff.capacity);
}
/* *
 * Searches through the input for a synchronization point. If one is found, we
 * will instruct the caller to flush, and return the number of bytes to load.
 * Otherwise, we will load as many bytes as possible and instruct the caller
 * to continue as normal.
 */
unsafe extern "C" fn findSynchronizationPoint(mut mtctx: *const ZSTDMT_CCtx,
                                              input: ZSTD_inBuffer)
 -> syncPoint_t {
    let istart: *const BYTE =
        (input.src as *const BYTE).offset(input.pos as isize);
    let primePower: U64 = (*mtctx).rsync.primePower;
    let hitMask: U64 = (*mtctx).rsync.hitMask;
    let mut syncPoint: syncPoint_t = syncPoint_t{toLoad: 0, flush: 0,};
    let mut hash: U64 = 0;
    let mut prev: *const BYTE = 0 as *const BYTE;
    let mut pos: size_t = 0;
    syncPoint.toLoad =
        if input.size.wrapping_sub(input.pos) <
               (*mtctx).targetSectionSize.wrapping_sub((*mtctx).inBuff.filled)
           {
            input.size.wrapping_sub(input.pos)
        } else {
            (*mtctx).targetSectionSize.wrapping_sub((*mtctx).inBuff.filled)
        };
    syncPoint.flush = 0i32;
    if 0 == (*mtctx).params.rsyncable { return syncPoint }
    if (*mtctx).inBuff.filled.wrapping_add(syncPoint.toLoad) <
           32i32 as libc::c_ulong {
        return syncPoint
    }
    if (*mtctx).inBuff.filled >= 32i32 as libc::c_ulong {
        pos = 0i32 as size_t;
        prev =
            ((*mtctx).inBuff.buffer.start as
                 *const BYTE).offset((*mtctx).inBuff.filled as
                                         isize).offset(-32isize);
        hash =
            ZSTD_rollingHash_compute(prev as *const libc::c_void,
                                     32i32 as size_t)
    } else {
        pos = (32i32 as libc::c_ulong).wrapping_sub((*mtctx).inBuff.filled);
        prev =
            ((*mtctx).inBuff.buffer.start as
                 *const BYTE).offset(-(pos as isize));
        hash =
            ZSTD_rollingHash_compute((*mtctx).inBuff.buffer.start,
                                     (*mtctx).inBuff.filled);
        hash =
            ZSTD_rollingHash_append(hash, istart as *const libc::c_void, pos)
    }
    while pos < syncPoint.toLoad {
        let toRemove: BYTE =
            (if pos < 32i32 as libc::c_ulong {
                 *prev.offset(pos as isize) as libc::c_int
             } else {
                 *istart.offset(pos.wrapping_sub(32i32 as libc::c_ulong) as
                                    isize) as libc::c_int
             }) as BYTE;
        hash =
            ZSTD_rollingHash_rotate(hash, toRemove,
                                    *istart.offset(pos as isize), primePower);
        if hash & hitMask == hitMask {
            syncPoint.toLoad = pos.wrapping_add(1i32 as libc::c_ulong);
            syncPoint.flush = 1i32;
            break ;
        } else { pos = pos.wrapping_add(1) }
    }
    return syncPoint;
}
/* * ZSTD_rollingHash_rotate() :
 * Rotate the rolling hash by one byte.
 */
unsafe extern "C" fn ZSTD_rollingHash_rotate(mut hash: U64,
                                             mut toRemove: BYTE,
                                             mut toAdd: BYTE,
                                             mut primePower: U64) -> U64 {
    hash =
        (hash as
             libc::c_ulong).wrapping_sub(((toRemove as libc::c_int + 10i32) as
                                              libc::c_ulong).wrapping_mul(primePower))
            as U64 as U64;
    hash = (hash as libc::c_ulong).wrapping_mul(prime8bytes) as U64 as U64;
    hash =
        (hash as
             libc::c_ulong).wrapping_add((toAdd as libc::c_int + 10i32) as
                                             libc::c_ulong) as U64 as U64;
    return hash;
}
/* * ZSTD_rollingHash_append() :
 * Add the buffer to the hash value.
 */
unsafe extern "C" fn ZSTD_rollingHash_append(mut hash: U64,
                                             mut buf: *const libc::c_void,
                                             mut size: size_t) -> U64 {
    let mut istart: *const BYTE = buf as *const BYTE;
    let mut pos: size_t = 0;
    pos = 0i32 as size_t;
    while pos < size {
        hash =
            (hash as libc::c_ulong).wrapping_mul(prime8bytes) as U64 as U64;
        hash =
            (hash as
                 libc::c_ulong).wrapping_add((*istart.offset(pos as isize) as
                                                  libc::c_int + 10i32) as
                                                 libc::c_ulong) as U64 as U64;
        pos = pos.wrapping_add(1)
    }
    return hash;
}
/* * ZSTD_rollingHash_compute() :
 * Compute the rolling hash value of the buffer.
 */
unsafe extern "C" fn ZSTD_rollingHash_compute(mut buf: *const libc::c_void,
                                              mut size: size_t) -> U64 {
    return ZSTD_rollingHash_append(0i32 as U64, buf, size);
}
/* *
 * Attempts to set the inBuff to the next section to fill.
 * If any part of the new section is still in use we give up.
 * Returns non-zero if the buffer is filled.
 */
unsafe extern "C" fn ZSTDMT_tryGetInputRange(mut mtctx: *mut ZSTDMT_CCtx)
 -> libc::c_int {
    let inUse: range_t = ZSTDMT_getInputDataInUse(mtctx);
    let spaceLeft: size_t =
        (*mtctx).roundBuff.capacity.wrapping_sub((*mtctx).roundBuff.pos);
    let target: size_t = (*mtctx).targetSectionSize;
    let mut buffer: buffer_t =
        buffer_s{start: 0 as *mut libc::c_void, capacity: 0,};
    if spaceLeft < target {
        let start: *mut BYTE = (*mtctx).roundBuff.buffer;
        let prefixSize: size_t = (*mtctx).inBuff.prefix.size;
        buffer.start = start as *mut libc::c_void;
        buffer.capacity = prefixSize;
        if 0 != ZSTDMT_isOverlapped(buffer, inUse) { return 0i32 }
        ZSTDMT_waitForLdmComplete(mtctx, buffer);
        memmove(start as *mut libc::c_void, (*mtctx).inBuff.prefix.start,
                prefixSize);
        (*mtctx).inBuff.prefix.start = start as *const libc::c_void;
        (*mtctx).roundBuff.pos = prefixSize
    }
    buffer.start =
        (*mtctx).roundBuff.buffer.offset((*mtctx).roundBuff.pos as isize) as
            *mut libc::c_void;
    buffer.capacity = target;
    if 0 != ZSTDMT_isOverlapped(buffer, inUse) { return 0i32 }
    ZSTDMT_waitForLdmComplete(mtctx, buffer);
    (*mtctx).inBuff.buffer = buffer;
    (*mtctx).inBuff.filled = 0i32 as size_t;
    return 1i32;
}
unsafe extern "C" fn ZSTDMT_waitForLdmComplete(mut mtctx: *mut ZSTDMT_CCtx,
                                               mut buffer: buffer_t) {
    if 0 != (*mtctx).params.ldmParams.enableLdm {
        let mut mutex: *mut pthread_mutex_t =
            &mut (*mtctx).serial.ldmWindowMutex;
        pthread_mutex_lock(mutex);
        while 0 != ZSTDMT_doesOverlapWindow(buffer, (*mtctx).serial.ldmWindow)
              {
            pthread_cond_wait(&mut (*mtctx).serial.ldmWindowCond, mutex);
        }
        pthread_mutex_unlock(mutex);
    };
}
unsafe extern "C" fn ZSTDMT_doesOverlapWindow(mut buffer: buffer_t,
                                              mut window: ZSTD_window_t)
 -> libc::c_int {
    let mut extDict: range_t =
        range_t{start: 0 as *const libc::c_void, size: 0,};
    let mut prefix: range_t =
        range_t{start: 0 as *const libc::c_void, size: 0,};
    extDict.start =
        window.dictBase.offset(window.lowLimit as isize) as
            *const libc::c_void;
    extDict.size = window.dictLimit.wrapping_sub(window.lowLimit) as size_t;
    prefix.start =
        window.base.offset(window.dictLimit as isize) as *const libc::c_void;
    prefix.size =
        window.nextSrc.wrapping_offset_from(window.base.offset(window.dictLimit
                                                                   as isize))
            as libc::c_long as size_t;
    return (0 != ZSTDMT_isOverlapped(buffer, extDict) ||
                0 != ZSTDMT_isOverlapped(buffer, prefix)) as libc::c_int;
}
/* *
 * Returns non-zero iff buffer and range overlap.
 */
unsafe extern "C" fn ZSTDMT_isOverlapped(mut buffer: buffer_t,
                                         mut range: range_t) -> libc::c_int {
    let bufferStart: *const BYTE = buffer.start as *const BYTE;
    let bufferEnd: *const BYTE = bufferStart.offset(buffer.capacity as isize);
    let rangeStart: *const BYTE = range.start as *const BYTE;
    let rangeEnd: *const BYTE = rangeStart.offset(range.size as isize);
    if rangeStart.is_null() || bufferStart.is_null() { return 0i32 }
    if bufferStart == bufferEnd || rangeStart == rangeEnd { return 0i32 }
    return (bufferStart < rangeEnd && rangeStart < bufferEnd) as libc::c_int;
}
/* *
 * Returns the range of data used by the earliest job that is not yet complete.
 * If the data of the first job is broken up into two segments, we cover both
 * sections.
 */
unsafe extern "C" fn ZSTDMT_getInputDataInUse(mut mtctx: *mut ZSTDMT_CCtx)
 -> range_t {
    let firstJobID: libc::c_uint = (*mtctx).doneJobID;
    let lastJobID: libc::c_uint = (*mtctx).nextJobID;
    let mut jobID: libc::c_uint = 0;
    jobID = firstJobID;
    while jobID < lastJobID {
        let wJobID: libc::c_uint = jobID & (*mtctx).jobIDMask;
        let mut consumed: size_t = 0;
        pthread_mutex_lock(&mut (*(*mtctx).jobs.offset(wJobID as
                                                           isize)).job_mutex);
        consumed = (*(*mtctx).jobs.offset(wJobID as isize)).consumed;
        pthread_mutex_unlock(&mut (*(*mtctx).jobs.offset(wJobID as
                                                             isize)).job_mutex);
        if consumed < (*(*mtctx).jobs.offset(wJobID as isize)).src.size {
            let mut range: range_t =
                (*(*mtctx).jobs.offset(wJobID as isize)).prefix;
            if range.size == 0i32 as libc::c_ulong {
                range = (*(*mtctx).jobs.offset(wJobID as isize)).src
            }
            return range
        }
        jobID = jobID.wrapping_add(1)
    }
    return kNullRange;
}
/* *< @return : 0 == all flushed; >0 : still some data to be flushed; or an error code (ZSTD_isError()) */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_flushStream(mut mtctx: *mut ZSTDMT_CCtx,
                                            mut output: *mut ZSTD_outBuffer)
 -> size_t {
    if 0 != (*mtctx).singleBlockingThread {
        return ZSTD_flushStream((*(*mtctx).cctxPool).cctx[0usize], output)
    }
    return ZSTDMT_flushStream_internal(mtctx, output, ZSTD_e_flush);
}
unsafe extern "C" fn ZSTDMT_flushStream_internal(mut mtctx: *mut ZSTDMT_CCtx,
                                                 mut output:
                                                     *mut ZSTD_outBuffer,
                                                 mut endFrame:
                                                     ZSTD_EndDirective)
 -> size_t {
    let srcSize: size_t = (*mtctx).inBuff.filled;
    if 0 != (*mtctx).jobReady || srcSize > 0i32 as libc::c_ulong ||
           endFrame as libc::c_uint ==
               ZSTD_e_end as libc::c_int as libc::c_uint &&
               0 == (*mtctx).frameEnded {
        let errcod: size_t =
            ZSTDMT_createCompressionJob(mtctx, srcSize, endFrame);
        if 0 != ERR_isError(errcod) { return errcod }
    }
    return ZSTDMT_flushProduced(mtctx, output, 1i32 as libc::c_uint,
                                endFrame);
}
/* *< @return : 0 == all flushed; >0 : still some data to be flushed; or an error code (ZSTD_isError()) */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_endStream(mut mtctx: *mut ZSTDMT_CCtx,
                                          mut output: *mut ZSTD_outBuffer)
 -> size_t {
    if 0 != (*mtctx).singleBlockingThread {
        return ZSTD_endStream((*(*mtctx).cctxPool).cctx[0usize], output)
    }
    return ZSTDMT_flushStream_internal(mtctx, output, ZSTD_e_end);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_initCStream_advanced(mut mtctx:
                                                         *mut ZSTDMT_CCtx,
                                                     mut dict:
                                                         *const libc::c_void,
                                                     mut dictSize: size_t,
                                                     mut params:
                                                         ZSTD_parameters,
                                                     mut pledgedSrcSize:
                                                         libc::c_ulonglong)
 -> size_t {
    /* retrieve sticky params */
    let mut cctxParams: ZSTD_CCtx_params = (*mtctx).params;
    cctxParams.cParams = params.cParams;
    cctxParams.fParams = params.fParams;
    return ZSTDMT_initCStream_internal(mtctx, dict, dictSize, ZSTD_dct_auto,
                                       0 as *const ZSTD_CDict, cctxParams,
                                       pledgedSrcSize);
}
/* dict can be released after init, a local copy is preserved within zcs */
/* pledgedSrcSize is optional and can be zero == unknown */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_initCStream_usingCDict(mut mtctx:
                                                           *mut ZSTDMT_CCtx,
                                                       mut cdict:
                                                           *const ZSTD_CDict,
                                                       mut fParams:
                                                           ZSTD_frameParameters,
                                                       mut pledgedSrcSize:
                                                           libc::c_ulonglong)
 -> size_t {
    let mut cctxParams: ZSTD_CCtx_params = (*mtctx).params;
    if cdict.is_null() {
        return -(ZSTD_error_dictionary_wrong as libc::c_int) as size_t
    }
    cctxParams.cParams = ZSTD_getCParamsFromCDict(cdict);
    cctxParams.fParams = fParams;
    return ZSTDMT_initCStream_internal(mtctx, 0 as *const libc::c_void,
                                       0i32 as size_t, ZSTD_dct_auto, cdict,
                                       cctxParams, pledgedSrcSize);
}
/* ZSTDMT_setMTCtxParameter() :
 * allow setting individual parameters, one at a time, among a list of enums defined in ZSTDMT_parameter.
 * The function must be called typically after ZSTD_createCCtx() but __before ZSTDMT_init*() !__
 * Parameters not explicitly reset by ZSTDMT_init*() remain the same in consecutive compression sessions.
 * @return : 0, or an error code (which can be tested using ZSTD_isError()) */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_setMTCtxParameter(mut mtctx: *mut ZSTDMT_CCtx,
                                                  mut parameter:
                                                      ZSTDMT_parameter,
                                                  mut value: libc::c_int)
 -> size_t {
    return ZSTDMT_CCtxParam_setMTCtxParameter(&mut (*mtctx).params, parameter,
                                              value);
}
/* ! ZSTDMT_CCtxParam_setMTCtxParameter()
 *  like ZSTDMT_setMTCtxParameter(), but into a ZSTD_CCtx_Params */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_CCtxParam_setMTCtxParameter(mut params:
                                                                *mut ZSTD_CCtx_params,
                                                            mut parameter:
                                                                ZSTDMT_parameter,
                                                            mut value:
                                                                libc::c_int)
 -> size_t {
    match parameter as libc::c_uint {
        0 => {
            if value != 0i32 && value < 1i32 * (1i32 << 20i32) {
                value = 1i32 * (1i32 << 20i32)
            }
            if value >
                   if 0 != MEM_32bits() {
                       512i32 * (1i32 << 20i32)
                   } else { 1024i32 * (1i32 << 20i32) } {
                value =
                    if 0 != MEM_32bits() {
                        512i32 * (1i32 << 20i32)
                    } else { 1024i32 * (1i32 << 20i32) }
            }
            (*params).jobSize = value as size_t;
            return value as size_t
        }
        1 => {
            if value < 0i32 { value = 0i32 }
            if value > 9i32 { value = 9i32 }
            (*params).overlapLog = value;
            return value as size_t
        }
        2 => {
            value = (value != 0i32) as libc::c_int;
            (*params).rsyncable = value;
            return value as size_t
        }
        _ => {
            return -(ZSTD_error_parameter_unsupported as libc::c_int) as
                       size_t
        }
    };
}
/* ZSTDMT_getMTCtxParameter() :
 * Query the ZSTDMT_CCtx for a parameter value.
 * @return : 0, or an error code (which can be tested using ZSTD_isError()) */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_getMTCtxParameter(mut mtctx: *mut ZSTDMT_CCtx,
                                                  mut parameter:
                                                      ZSTDMT_parameter,
                                                  mut value: *mut libc::c_int)
 -> size_t {
    match parameter as libc::c_uint {
        0 => { *value = (*mtctx).params.jobSize as libc::c_int }
        1 => { *value = (*mtctx).params.overlapLog }
        2 => { *value = (*mtctx).params.rsyncable }
        _ => {
            return -(ZSTD_error_parameter_unsupported as libc::c_int) as
                       size_t
        }
    }
    return 0i32 as size_t;
}
/* ========================================================
 * ===  Private interface, for use by ZSTD_compress.c   ===
 * ===  Not exposed in libzstd. Never invoke directly   ===
 * ======================================================== */
/* ! ZSTDMT_toFlushNow()
  *  Tell how many bytes are ready to be flushed immediately.
  *  Probe the oldest active job (not yet entirely flushed) and check its output buffer.
  *  If return 0, it means there is no active job,
  *  or, it means oldest job is still active, but everything produced has been flushed so far,
  *  therefore flushing is limited by speed of oldest job. */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_toFlushNow(mut mtctx: *mut ZSTDMT_CCtx)
 -> size_t {
    let mut toFlush: size_t = 0;
    let jobID: libc::c_uint = (*mtctx).doneJobID;
    if jobID == (*mtctx).nextJobID { return 0i32 as size_t }
    let wJobID: libc::c_uint = jobID & (*mtctx).jobIDMask;
    let jobPtr: *mut ZSTDMT_jobDescription =
        &mut *(*mtctx).jobs.offset(wJobID as isize) as
            *mut ZSTDMT_jobDescription;
    pthread_mutex_lock(&mut (*jobPtr).job_mutex);
    let cResult: size_t = (*jobPtr).cSize;
    let produced: size_t =
        if 0 != ERR_isError(cResult) {
            0i32 as libc::c_ulong
        } else { cResult };
    let flushed: size_t =
        if 0 != ERR_isError(cResult) {
            0i32 as libc::c_ulong
        } else { (*jobPtr).dstFlushed };
    toFlush = produced.wrapping_sub(flushed);
    toFlush == 0i32 as libc::c_ulong &&
        (*jobPtr).consumed >= (*jobPtr).src.size;
    pthread_mutex_unlock(&mut (*(*mtctx).jobs.offset(wJobID as
                                                         isize)).job_mutex);
    return toFlush;
}
/* ! ZSTDMT_updateCParams_whileCompressing() :
 *  Updates only a selected set of compression parameters, to remain compatible with current frame.
 *  New parameters will be applied to next compression job. */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_updateCParams_whileCompressing(mut mtctx:
                                                                   *mut ZSTDMT_CCtx,
                                                               mut cctxParams:
                                                                   *const ZSTD_CCtx_params) {
    /* Do not modify windowLog while compressing */
    let saved_wlog: U32 = (*mtctx).params.cParams.windowLog;
    let compressionLevel: libc::c_int = (*cctxParams).compressionLevel;
    (*mtctx).params.compressionLevel = compressionLevel;
    let mut cParams: ZSTD_compressionParameters =
        ZSTD_getCParamsFromCCtxParams(cctxParams, 0i32 as U64,
                                      0i32 as size_t);
    cParams.windowLog = saved_wlog;
    (*mtctx).params.cParams = cParams;
}
/* ! ZSTDMT_getFrameProgression():
 *  tells how much data has been consumed (input) and produced (output) for current frame.
 *  able to count progression inside worker threads.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_getFrameProgression(mut mtctx:
                                                        *mut ZSTDMT_CCtx)
 -> ZSTD_frameProgression {
    let mut fps: ZSTD_frameProgression =
        ZSTD_frameProgression{ingested: 0,
                              consumed: 0,
                              produced: 0,
                              flushed: 0,
                              currentJobID: 0,
                              nbActiveWorkers: 0,};
    fps.ingested =
        (*mtctx).consumed.wrapping_add((*mtctx).inBuff.filled as
                                           libc::c_ulonglong);
    fps.consumed = (*mtctx).consumed;
    fps.flushed = (*mtctx).produced;
    fps.produced = fps.flushed;
    fps.currentJobID = (*mtctx).nextJobID;
    fps.nbActiveWorkers = 0i32 as libc::c_uint;
    let mut jobNb: libc::c_uint = 0;
    let mut lastJobNb: libc::c_uint =
        (*mtctx).nextJobID.wrapping_add((*mtctx).jobReady as libc::c_uint);
    jobNb = (*mtctx).doneJobID;
    while jobNb < lastJobNb {
        let wJobID: libc::c_uint = jobNb & (*mtctx).jobIDMask;
        let mut jobPtr: *mut ZSTDMT_jobDescription =
            &mut *(*mtctx).jobs.offset(wJobID as isize) as
                *mut ZSTDMT_jobDescription;
        pthread_mutex_lock(&mut (*jobPtr).job_mutex);
        let cResult: size_t = (*jobPtr).cSize;
        let produced: size_t =
            if 0 != ERR_isError(cResult) {
                0i32 as libc::c_ulong
            } else { cResult };
        let flushed: size_t =
            if 0 != ERR_isError(cResult) {
                0i32 as libc::c_ulong
            } else { (*jobPtr).dstFlushed };
        fps.ingested =
            fps.ingested.wrapping_add((*jobPtr).src.size as
                                          libc::c_ulonglong);
        fps.consumed =
            fps.consumed.wrapping_add((*jobPtr).consumed as
                                          libc::c_ulonglong);
        fps.produced =
            fps.produced.wrapping_add(produced as libc::c_ulonglong);
        fps.flushed = fps.flushed.wrapping_add(flushed as libc::c_ulonglong);
        fps.nbActiveWorkers =
            fps.nbActiveWorkers.wrapping_add(((*jobPtr).consumed <
                                                  (*jobPtr).src.size) as
                                                 libc::c_int as libc::c_uint);
        pthread_mutex_unlock(&mut (*(*mtctx).jobs.offset(wJobID as
                                                             isize)).job_mutex);
        jobNb = jobNb.wrapping_add(1)
    }
    return fps;
}