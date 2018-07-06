#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( extern_types , libc , offset_to )]
extern crate libc;
extern "C" {
    pub type ZSTD_CDict_s;
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
    #[no_mangle]
    fn ZSTD_compressBound(srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_isError(code: size_t) -> libc::c_uint;
    #[no_mangle]
    fn ZSTD_maxCLevel() -> libc::c_int;
    #[no_mangle]
    fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> size_t;
    #[no_mangle]
    fn ZSTD_freeCDict(CDict: *mut ZSTD_CDict) -> size_t;
    #[no_mangle]
    fn ZSTD_flushStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer)
     -> size_t;
    #[no_mangle]
    fn ZSTD_endStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer)
     -> size_t;
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
    #[no_mangle]
    fn ZSTD_getParams(compressionLevel: libc::c_int,
                      estimatedSrcSize: libc::c_ulonglong, dictSize: size_t)
     -> ZSTD_parameters;
    #[no_mangle]
    fn ZSTD_compress_usingCDict_advanced(cctx: *mut ZSTD_CCtx,
                                         dst: *mut libc::c_void,
                                         dstCapacity: size_t,
                                         src: *const libc::c_void,
                                         srcSize: size_t,
                                         cdict: *const ZSTD_CDict,
                                         fParams: ZSTD_frameParameters)
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
    fn ZSTD_CCtxParam_setParameter(params: *mut ZSTD_CCtx_params,
                                   param: ZSTD_cParameter,
                                   value: libc::c_uint) -> size_t;
    #[no_mangle]
    fn POOL_create_advanced(numThreads: size_t, queueSize: size_t,
                            customMem: ZSTD_customMem) -> *mut POOL_ctx;
    /* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
    /* *< position where writing stopped. Will be updated. Necessarily 0 <= pos <= size */
    /* *< Cannot use the previous table */
    /* *< Cannot use the previous table */
    /* ! POOL_function :
 *  The function type that can be added to a thread pool.
 */
    /* ***************************
*  Streaming
****************************/
    /* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
    /* **************************************
*  Explicit context
***************************************/
    /* !< maximum compression level available */
    /* !< provides readable string from an error code */
    /* *< Can use the previous table but it must be checked. Note : The previous table must have been constructed by HUF_compress{1, 4}X_repeat */
    /* *********************************
 *  Bulk processing dictionary API
 *********************************/
    /* ! Constructor and Destructor of FSE_DTable.
    Note that its size depends on 'tableLog' */
    /* *< Copy dictionary content internally */
    /* *< Can use the previous table but it must be checked */
    /* *< position where reading stopped. Will be updated. Necessarily 0 <= pos <= size */
    /* ***************************************************************************************
 * START OF ADVANCED AND EXPERIMENTAL FUNCTIONS
 * The definitions in this section are considered experimental.
 * They should never be used with a dynamic library, as prototypes may change in the future.
 * They are provided for advanced scenarios.
 * Use them only in association with static linking.
 * ***************************************************************************************/
    /* ***************************************************************************************
 * START OF ADVANCED AND EXPERIMENTAL FUNCTIONS
 * The definitions in this section are considered experimental.
 * They should never be used with a dynamic library, as prototypes may change in the future.
 * They are provided for advanced scenarios.
 * Use them only in association with static linking.
 * ***************************************************************************************/
    /* ***************************************************************************************
 * START OF ADVANCED AND EXPERIMENTAL FUNCTIONS
 * The definitions in this section are considered experimental.
 * They should never be used with a dynamic library, as prototypes may change in the future.
 * They are provided for advanced scenarios.
 * Use them only in association with static linking.
 * ***************************************************************************************/
    /* ***************************************************************************************
 * START OF ADVANCED AND EXPERIMENTAL FUNCTIONS
 * The definitions in this section are considered experimental.
 * They should never be used with a dynamic library, as prototypes may change in the future.
 * They are provided for advanced scenarios.
 * Use them only in association with static linking.
 * ***************************************************************************************/
    /* !< recommended size for output buffer. Guarantee to successfully flush at least one complete block in all circumstances. */
    /* !< recommended size for input buffer */
    /* ! POOL_free() :
 *  Free a thread pool returned by POOL_create().
 */
    #[no_mangle]
    fn POOL_free(ctx: *mut POOL_ctx) -> ();
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
                opaque: *mut libc::c_void) -> ();
    /* ! POOL_tryAdd() :
 *  Add the job `function(opaque)` to thread pool _if_ a worker is available.
 *  Returns immediately even if not (does not block).
 * @return : 1 if successful, 0 if not.
 */
    #[no_mangle]
    fn POOL_tryAdd(ctx: *mut POOL_ctx, function: POOL_function,
                   opaque: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    static mut __tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut __daylight: libc::c_int;
    #[no_mangle]
    static mut __timezone: libc::c_long;
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
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
    #[no_mangle]
    fn ZSTD_XXH64_reset(statePtr: *mut XXH64_state_t, seed: libc::c_ulonglong)
     -> XXH_errorcode;
    #[no_mangle]
    fn ZSTD_XXH64_update(statePtr: *mut XXH64_state_t,
                         input: *const libc::c_void, length: size_t)
     -> XXH_errorcode;
    #[no_mangle]
    fn ZSTD_XXH64_digest(statePtr: *const XXH64_state_t) -> XXH64_hash_t;
    #[no_mangle]
    fn ZSTD_malloc(size: size_t, customMem: ZSTD_customMem)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ZSTD_calloc(size: size_t, customMem: ZSTD_customMem)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ZSTD_free(ptr: *mut libc::c_void, customMem: ZSTD_customMem) -> ();
    #[no_mangle]
    fn ZSTD_invalidateRepCodes(cctx: *mut ZSTD_CCtx) -> ();
    #[no_mangle]
    fn ZSTD_referenceExternalSequences(cctx: *mut ZSTD_CCtx, seq: *mut rawSeq,
                                       nbSeq: size_t) -> size_t;
    /* *<
Let's now decompose FSE_decompress_usingDTable() into its unitary components.
You will decode FSE-encoded symbols from the bitStream,
and also any other bitFields you put in, **in reverse order**.

You will need a few variables to track your bitStream. They are :

BIT_DStream_t DStream;    // Stream context
FSE_DState_t  DState;     // State context. Multiple ones are possible
FSE_DTable*   DTablePtr;  // Decoding table, provided by FSE_buildDTable()

The first thing to do is to init the bitStream.
    errorCode = BIT_initDStream(&DStream, srcBuffer, srcSize);

You should then retrieve your initial state(s)
(in reverse flushing order if you have several ones) :
    errorCode = FSE_initDState(&DState, &DStream, DTablePtr);

You can then decode your data, symbol after symbol.
For information the maximum number of bits read by FSE_decodeSymbol() is 'tableLog'.
Keep in mind that symbols are decoded in reverse order, like a LIFO stack (last in, first out).
    unsigned char symbol = FSE_decodeSymbol(&DState, &DStream);

You can retrieve any bitfield you eventually stored into the bitStream (in reverse order)
Note : maximum allowed nbBits is 25, for 32-bits compatibility
    size_t bitField = BIT_readBits(&DStream, nbBits);

All above operations only read from local register (which size depends on size_t).
Refueling the register from memory is manually performed by the reload method.
    endSignal = FSE_reloadDStream(&DStream);

BIT_reloadDStream() result tells if there is still some more data to read from DStream.
BIT_DStream_unfinished : there is still some data left into the DStream.
BIT_DStream_endOfBuffer : Dstream reached end of buffer. Its container may no longer be completely filled.
BIT_DStream_completed : Dstream reached its exact end, corresponding in general to decompression completed.
BIT_DStream_tooFar : Dstream went too far. Decompression result is corrupted.

When reaching end of buffer (BIT_DStream_endOfBuffer), progress slowly, notably if you decode multiple symbols per loop,
to properly detect the exact end of stream.
After each decoded symbol, check if DStream is fully consumed using this simple test :
    BIT_reloadDStream(&DStream) >= BIT_DStream_completed

When it's done, verify decompression is fully completed, by checking both DStream and the relevant states.
Checking if DStream has reached its end is performed by :
    BIT_endOfDStream(&DStream);
Check also the states. There might be some symbols left there, if some high probability ones (>50%) are possible.
    FSE_endOfDState(&DState);
*/
    /* * ZSTDMT_getBuffer() :
 *  assumption : bufPool must be valid
 * @return : a buffer, with start pointer and size
 *  note: allocation may fail, in this case, start==NULL and size==0 */
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
    /* * ZSTD_ldm_getTableSize() :
 *  Return prime8bytes^(minMatchLength-1) */
    #[no_mangle]
    fn ZSTD_ldm_getHashPower(minMatchLength: U32) -> U64;
    /* * ZSTD_ldm_adjustParameters() :
 *  If the params->hashEveryLog is not set, set it to its default value based on
 *  windowLog and params->hashLog.
 *
 *  Ensures that params->bucketSizeLog is <= params->hashLog (setting it to
 *  params->hashLog if it is not).
 *
 *  Ensures that the minMatchLength >= targetLength during optimal parsing.
 */
    #[no_mangle]
    fn ZSTD_ldm_adjustParameters(params: *mut ldmParams_t,
                                 cParams: *const ZSTD_compressionParameters)
     -> ();
    #[no_mangle]
    fn ZSTD_compress_advanced_internal(cctx: *mut ZSTD_CCtx,
                                       dst: *mut libc::c_void,
                                       dstCapacity: size_t,
                                       src: *const libc::c_void,
                                       srcSize: size_t,
                                       dict: *const libc::c_void,
                                       dictSize: size_t,
                                       params: ZSTD_CCtx_params) -> size_t;
    /* ! ZSTDMT_initCStream_internal() :
 *  Private use only. Init streaming operation.
 *  expects params to be valid.
 *  must receive dict, or cdict, or none, but not both.
 *  @return : 0, or an error code */
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
    #[no_mangle]
    fn ZSTD_writeLastEmptyBlock(dst: *mut libc::c_void, dstCapacity: size_t)
     -> size_t;
    /* ! ZSTDMT_flushProduced() :
 * `output` : `pos` will be updated with amount of data flushed .
 * `blockToFlush` : if >0, the function will block and wait if there is no data available to flush .
 * @return : amount of data remaining within internal buffer, 0 if no more, 1 if unknown but > 0, or an error code */
    /* *
 * Attempts to set the inBuff to the next section to fill.
 * If any part of the new section is still in use we give up.
 * Returns non-zero if the buffer is filled.
 */
    /* *
 * Returns non-zero iff buffer and range overlap.
 */
    /* *
 * Returns the range of data used by the earliest job that is not yet complete.
 * If the data of the first job is broken up into two segments, we cover both
 * sections.
 */
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
    #[no_mangle]
    fn ZSTD_getCParamsFromCCtxParams(CCtxParams: *const ZSTD_CCtx_params,
                                     srcSizeHint: U64, dictSize: size_t)
     -> ZSTD_compressionParameters;
}
pub type __pthread_list_t = __pthread_internal_list;
pub type ZSTD_ErrorCode = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_inBuffer_s {
    pub src: *const libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type BYTE = uint8_t;
pub type ZSTD_compressionStage_e = libc::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub type ZSTDMT_CCtx = ZSTDMT_CCtx_s;
pub const ZSTD_p_forceAttachDict: ZSTD_cParameter = 1101;
pub type FSE_CTable = libc::c_uint;
pub type ZSTD_CStream = ZSTD_CCtx;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const libc::c_void,
    pub symbolTT: *const libc::c_void,
    pub stateLog: libc::c_uint,
}
pub const ZSTD_p_checksumFlag: ZSTD_cParameter = 201;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const ZSTD_p_overlapSizeLog: ZSTD_cParameter = 402;
pub type uint32_t = libc::c_uint;
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
pub const FSE_repeat_check: FSE_repeat = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct rawSeqStore_t {
    pub seq: *mut rawSeq,
    pub pos: size_t,
    pub size: size_t,
    pub capacity: size_t,
}
pub type uint64_t = libc::c_ulong;
pub const ZSTD_dictDefaultAttach: ZSTD_dictAttachPref_e = 0;
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct roundBuff_t {
    pub buffer: *mut BYTE,
    pub capacity: size_t,
    pub pos: size_t,
}
pub type POOL_ctx = POOL_ctx_s;
pub type HUF_repeat = libc::c_uint;
pub type int16_t = libc::c_short;
pub const HUF_repeat_check: HUF_repeat = 1;
pub type XXH64_hash_t = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
pub type ZSTDMT_bufferPool = ZSTDMT_bufferPool_s;
pub type U32 = uint32_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct buffer_s {
    pub start: *mut libc::c_void,
    pub capacity: size_t,
}
pub type seqDef = seqDef_s;
pub type POOL_function =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct inBuff_t {
    pub prefix: range_t,
    pub buffer: buffer_t,
    pub filled: size_t,
}
pub const ZSTD_f_zstd1: ZSTD_format_e = 0;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub type ZSTD_dictAttachPref_e = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_hufCTables_t {
    pub CTable: [U32; 256],
    pub repeatMode: HUF_repeat,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct range_t {
    pub start: *const libc::c_void,
    pub size: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
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
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
pub type ZSTD_cParameter = libc::c_uint;
pub type ZSTD_dictMode_e = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub __lock: libc::c_int,
    pub __futex: libc::c_uint,
    pub __total_seq: libc::c_ulonglong,
    pub __wakeup_seq: libc::c_ulonglong,
    pub __woken_seq: libc::c_ulonglong,
    pub __mutex: *mut libc::c_void,
    pub __nwaiters: libc::c_uint,
    pub __broadcast_seq: libc::c_uint,
}
pub type ZSTDMT_parameter = libc::c_uint;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub const XXH_OK: XXH_errorcode = 0;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
pub const ZSTD_p_searchLog: ZSTD_cParameter = 104;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
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
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
pub type ZSTD_matchState_t = ZSTD_matchState_t_0;
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
pub const ZSTD_p_contentSizeFlag: ZSTD_cParameter = 200;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ldmParams_t {
    pub enableLdm: U32,
    pub hashLog: U32,
    pub bucketSizeLog: U32,
    pub minMatchLength: U32,
    pub hashEveryLog: U32,
    pub windowLog: U32,
}
pub type unnamed_0 = libc::c_uint;
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub type ZSTD_dictContentType_e = libc::c_uint;
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
pub const ZSTD_e_end: ZSTD_EndDirective = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: libc::c_int,
    pub deltaNbBits: U32,
}
pub const ZSTD_p_ldmBucketSizeLog: ZSTD_cParameter = 163;
pub type U16 = uint16_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
pub type ZSTD_dictTableLoadMethod_e = libc::c_uint;
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_compressedBlockState_t {
    pub entropy: ZSTD_entropyCTables_t,
    pub rep: [U32; 3],
}
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub const zcss_load: ZSTD_cStreamStage = 1;
pub const ZSTD_p_compressionStrategy: ZSTD_cParameter = 107;
pub const ZSTD_lazy: ZSTD_strategy = 4;
pub const ZSTD_btopt: ZSTD_strategy = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_compressionParameters {
    pub windowLog: libc::c_uint,
    pub chainLog: libc::c_uint,
    pub hashLog: libc::c_uint,
    pub searchLog: libc::c_uint,
    pub searchLength: libc::c_uint,
    pub targetLength: libc::c_uint,
    pub strategy: ZSTD_strategy,
}
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub const zcss_flush: ZSTD_cStreamStage = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_prefixDict_s {
    pub dict: *const libc::c_void,
    pub dictSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ldmState_t {
    pub window: ZSTD_window_t,
    pub hashTable: *mut ldmEntry_t,
    pub bucketOffsets: *mut BYTE,
    pub hashPower: U64,
}
pub type ZSTD_dictLoadMethod_e = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_condattr_t {
    __size: [libc::c_char; 4],
    __align: libc::c_int,
}
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;
pub type ZSTD_cStreamStage = libc::c_uint;
pub const ZSTD_p_ldmHashEveryLog: ZSTD_cParameter = 164;
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub type ZSTDMT_seqPool = ZSTDMT_bufferPool;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_match_t {
    pub off: U32,
    pub len: U32,
}
pub type U64 = uint64_t;
pub const ZSTD_f_zstd1_magicless: ZSTD_format_e = 1;
pub const HUF_repeat_valid: HUF_repeat = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: libc::c_uint,
    pub ptr: *const libc::c_char,
    pub start: *const libc::c_char,
    pub limitPtr: *const libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTDMT_CCtxPool {
    pub poolMutex: pthread_mutex_t,
    pub totalCCtx: libc::c_uint,
    pub availCCtx: libc::c_uint,
    pub cMem: ZSTD_customMem,
    pub cctx: [*mut ZSTD_CCtx; 1],
}
pub type buffer_t = buffer_s;
pub type ptrdiff_t = libc::c_long;
pub type ZSTD_CDict = ZSTD_CDict_s;
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_fseCTables_t {
    pub offcodeCTable: [FSE_CTable; 193],
    pub matchlengthCTable: [FSE_CTable; 363],
    pub litlengthCTable: [FSE_CTable; 329],
    pub offcode_repeatMode: FSE_repeat,
    pub matchlength_repeatMode: FSE_repeat,
    pub litlength_repeatMode: FSE_repeat,
}
pub const ZSTD_p_format: ZSTD_cParameter = 10;
#[derive ( Copy , Clone )]
#[repr ( C )]
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
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
#[derive ( Copy , Clone )]
#[repr ( C )]
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DState_t {
    pub state: size_t,
    pub table: *const libc::c_void,
}
pub const MEM_static_assert: unnamed_0 = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct optState_t {
    pub litFreq: *mut U32,
    pub litLengthFreq: *mut U32,
    pub matchLengthFreq: *mut U32,
    pub offCodeFreq: *mut U32,
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
pub type ZSTD_OptPrice_e = libc::c_uint;
pub const ZSTDMT_p_jobSize: ZSTDMT_parameter = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_CCtx_params_s {
    pub format: ZSTD_format_e,
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
    pub compressionLevel: libc::c_int,
    pub forceWindow: libc::c_int,
    pub attachDictPref: ZSTD_dictAttachPref_e,
    pub nbWorkers: libc::c_uint,
    pub jobSize: libc::c_uint,
    pub overlapSizeLog: libc::c_uint,
    pub ldmParams: ldmParams_t,
    pub customMem: ZSTD_customMem,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_matchState_t_0 {
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
}
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub const ZSTD_p_ldmMinMatch: ZSTD_cParameter = 162;
pub const ZSTD_fast: ZSTD_strategy = 1;
pub const ZSTD_p_chainLog: ZSTD_cParameter = 103;
pub const ZSTDMT_p_overlapSectionLog: ZSTDMT_parameter = 1;
pub const ZSTD_p_compressionLevel: ZSTD_cParameter = 100;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    u: U32,
    c: [BYTE; 4],
}
pub type FSE_DTable = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_optimal_t {
    pub price: libc::c_int,
    pub off: U32,
    pub mlen: U32,
    pub litlen: U32,
    pub rep: [U32; 3],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ldmEntry_t {
    pub offset: U32,
    pub checksum: U32,
}
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub const ZSTD_p_dictIDFlag: ZSTD_cParameter = 202;
pub type ZSTD_prefixDict = ZSTD_prefixDict_s;
pub type ZSTD_CCtx_params = ZSTD_CCtx_params_s;
pub type XXH64_state_t = XXH64_state_s;
pub type ZSTD_strategy = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_blockState_t {
    pub prevCBlock: *mut ZSTD_compressedBlockState_t,
    pub nextCBlock: *mut ZSTD_compressedBlockState_t,
    pub matchState: ZSTD_matchState_t,
}
pub const ZSTD_e_continue: ZSTD_EndDirective = 0;
pub const ZSTD_p_hashLog: ZSTD_cParameter = 102;
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_mutexattr_t {
    __size: [libc::c_char; 4],
    __align: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_frameProgression {
    pub ingested: libc::c_ulonglong,
    pub consumed: libc::c_ulonglong,
    pub produced: libc::c_ulonglong,
}
pub const ZSTD_p_forceMaxWindow: ZSTD_cParameter = 1100;
pub const ZSTD_p_windowLog: ZSTD_cParameter = 101;
pub const ZSTD_dictForceCopy: ZSTD_dictAttachPref_e = -1;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
pub type S16 = int16_t;
pub const ZSTD_p_jobSize: ZSTD_cParameter = 401;
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
pub const ZSTDcs_ending: ZSTD_compressionStage_e = 3;
pub const ZSTD_greedy: ZSTD_strategy = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct seqStore_t {
    pub sequencesStart: *mut seqDef,
    pub sequences: *mut seqDef,
    pub litStart: *mut BYTE,
    pub lit: *mut BYTE,
    pub llCode: *mut BYTE,
    pub mlCode: *mut BYTE,
    pub ofCode: *mut BYTE,
    pub longLengthID: U32,
    pub longLengthPos: U32,
}
pub const ZSTDcs_created: ZSTD_compressionStage_e = 0;
pub const ZSTD_p_minMatch: ZSTD_cParameter = 105;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct rawSeq {
    pub offset: U32,
    pub litLength: U32,
    pub matchLength: U32,
}
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub const ZSTD_p_ldmHashLog: ZSTD_cParameter = 161;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_parameters {
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
pub const ZSTD_dictForceAttach: ZSTD_dictAttachPref_e = 1;
pub type ZSTD_EndDirective = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_window_t {
    pub nextSrc: *const BYTE,
    pub base: *const BYTE,
    pub dictBase: *const BYTE,
    pub dictLimit: U32,
    pub lowLimit: U32,
}
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_entropyCTables_t {
    pub huf: ZSTD_hufCTables_t,
    pub fse: ZSTD_fseCTables_t,
}
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
pub type ZSTD_format_e = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_mutex_t {
    __data: __pthread_mutex_s,
    __size: [libc::c_char; 40],
    __align: libc::c_long,
}
pub const MEM_static_assert_0: unnamed_2 = 1;
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_cond_t {
    __data: unnamed,
    __size: [libc::c_char; 48],
    __align: libc::c_longlong,
}
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
pub const ZSTD_e_flush: ZSTD_EndDirective = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct seqDef_s {
    pub offset: U32,
    pub litLength: U16,
    pub matchLength: U16,
}
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
pub type FSE_repeat = libc::c_uint;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_p_targetLength: ZSTD_cParameter = 106;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTDMT_bufferPool_s {
    pub poolMutex: pthread_mutex_t,
    pub bufferSize: size_t,
    pub totalBuffers: libc::c_uint,
    pub nbBuffers: libc::c_uint,
    pub cMem: ZSTD_customMem,
    pub bTable: [buffer_t; 1],
}
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
pub type XXH_errorcode = libc::c_uint;
pub const ZSTDcs_ongoing: ZSTD_compressionStage_e = 2;
pub type BIT_DStream_status = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
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
pub type uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_frameParameters {
    pub contentSizeFlag: libc::c_uint,
    pub checksumFlag: libc::c_uint,
    pub noDictIDFlag: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
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
pub type unnamed_2 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
pub const FSE_repeat_valid: FSE_repeat = 2;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
pub const ZSTD_p_nbWorkers: ZSTD_cParameter = 400;
pub const FSE_repeat_none: FSE_repeat = 0;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub const ZSTDcs_init: ZSTD_compressionStage_e = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_CStream_t {
    pub bitContainer: size_t,
    pub bitPos: libc::c_uint,
    pub startPtr: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub endPtr: *mut libc::c_char,
}
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
pub type ERR_enum = ZSTD_ErrorCode;
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
pub const ZSTD_p_enableLongDistanceMatching: ZSTD_cParameter = 160;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
pub const zcss_init: ZSTD_cStreamStage = 0;
static mut ZSTD_frameHeaderSize_prefix: size_t = unsafe { 5i32 as size_t };
static mut ZSTD_frameHeaderSize_min: size_t = unsafe { 6i32 as size_t };
static mut ZSTD_frameHeaderSize_max: size_t = unsafe { 18i32 as size_t };
static mut ZSTD_skippableHeaderSize: size_t = unsafe { 8i32 as size_t };
static mut ZSTD_defaultCMem: ZSTD_customMem =
    unsafe {
        ZSTD_customMem{customAlloc: None,
                       customFree: None,
                       opaque: 0 as *const libc::c_void as *mut libc::c_void,}
    };
unsafe extern "C" fn MEM_check() -> () { }
unsafe extern "C" fn MEM_32bits() -> libc::c_uint {
    return (::std::mem::size_of::<size_t>() as libc::c_ulong ==
                4i32 as libc::c_ulong) as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn MEM_64bits() -> libc::c_uint {
    return (::std::mem::size_of::<size_t>() as libc::c_ulong ==
                8i32 as libc::c_ulong) as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    let one: unnamed_1 = unnamed_1{u: 1i32 as U32,};
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
unsafe extern "C" fn MEM_write16(mut memPtr: *mut libc::c_void,
                                 mut value: U16) -> () {
    (*(memPtr as *mut unalign16)).v = value;
}
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void,
                                 mut value: U32) -> () {
    (*(memPtr as *mut unalign32)).v = value;
}
unsafe extern "C" fn MEM_write64(mut memPtr: *mut libc::c_void,
                                 mut value: U64) -> () {
    (*(memPtr as *mut unalign64)).v = value;
}
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0 << 24i32 & 4278190080u32 |
               in_0 << 8i32 & 16711680i32 as libc::c_uint |
               in_0 >> 8i32 & 65280i32 as libc::c_uint |
               in_0 >> 24i32 & 255i32 as libc::c_uint;
}
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return ((in_0 << 56i32) as libc::c_ulonglong & 18374686479671623680u64 |
                (in_0 << 40i32) as libc::c_ulonglong & 71776119061217280u64 |
                (in_0 << 24i32) as libc::c_ulonglong & 280375465082880u64 |
                (in_0 << 8i32) as libc::c_ulonglong & 1095216660480u64 |
                (in_0 >> 8i32) as libc::c_ulonglong & 4278190080u64 |
                (in_0 >> 24i32) as libc::c_ulonglong & 16711680u64 |
                (in_0 >> 40i32) as libc::c_ulonglong & 65280u64 |
                (in_0 >> 56i32) as libc::c_ulonglong & 255u64) as U64;
}
unsafe extern "C" fn MEM_swapST(mut in_0: size_t) -> size_t {
    if 0 != MEM_32bits() {
        return MEM_swap32(in_0 as U32) as size_t
    } else { return MEM_swap64(in_0) };
}
unsafe extern "C" fn MEM_readLE16(mut memPtr: *const libc::c_void) -> U16 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read16(memPtr)
    } else {
        let mut p: *const BYTE = memPtr as *const BYTE;
        return (*p.offset(0isize) as libc::c_int +
                    ((*p.offset(1isize) as libc::c_int) << 8i32)) as U16
    };
}
unsafe extern "C" fn MEM_writeLE16(mut memPtr: *mut libc::c_void,
                                   mut val: U16) -> () {
    if 0 != MEM_isLittleEndian() {
        MEM_write16(memPtr, val);
    } else {
        let mut p: *mut BYTE = memPtr as *mut BYTE;
        *p.offset(0isize) = val as BYTE;
        *p.offset(1isize) = (val as libc::c_int >> 8i32) as BYTE
    };
}
unsafe extern "C" fn MEM_readLE24(mut memPtr: *const libc::c_void) -> U32 {
    return (MEM_readLE16(memPtr) as libc::c_int +
                ((*(memPtr as *const BYTE).offset(2isize) as libc::c_int) <<
                     16i32)) as U32;
}
unsafe extern "C" fn MEM_writeLE24(mut memPtr: *mut libc::c_void,
                                   mut val: U32) -> () {
    MEM_writeLE16(memPtr, val as U16);
    *(memPtr as *mut BYTE).offset(2isize) = (val >> 16i32) as BYTE;
}
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const libc::c_void) -> U32 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read32(memPtr)
    } else { return MEM_swap32(MEM_read32(memPtr)) };
}
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut libc::c_void,
                                   mut val32: U32) -> () {
    if 0 != MEM_isLittleEndian() {
        MEM_write32(memPtr, val32);
    } else { MEM_write32(memPtr, MEM_swap32(val32)); };
}
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const libc::c_void) -> U64 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read64(memPtr)
    } else { return MEM_swap64(MEM_read64(memPtr)) };
}
unsafe extern "C" fn MEM_writeLE64(mut memPtr: *mut libc::c_void,
                                   mut val64: U64) -> () {
    if 0 != MEM_isLittleEndian() {
        MEM_write64(memPtr, val64);
    } else { MEM_write64(memPtr, MEM_swap64(val64)); };
}
unsafe extern "C" fn MEM_readLEST(mut memPtr: *const libc::c_void) -> size_t {
    if 0 != MEM_32bits() {
        return MEM_readLE32(memPtr) as size_t
    } else { return MEM_readLE64(memPtr) };
}
unsafe extern "C" fn MEM_writeLEST(mut memPtr: *mut libc::c_void,
                                   mut val: size_t) -> () {
    if 0 != MEM_32bits() {
        MEM_writeLE32(memPtr, val as U32);
    } else { MEM_writeLE64(memPtr, val); };
}
unsafe extern "C" fn MEM_readBE32(mut memPtr: *const libc::c_void) -> U32 {
    if 0 != MEM_isLittleEndian() {
        return MEM_swap32(MEM_read32(memPtr))
    } else { return MEM_read32(memPtr) };
}
unsafe extern "C" fn MEM_writeBE32(mut memPtr: *mut libc::c_void,
                                   mut val32: U32) -> () {
    if 0 != MEM_isLittleEndian() {
        MEM_write32(memPtr, MEM_swap32(val32));
    } else { MEM_write32(memPtr, val32); };
}
unsafe extern "C" fn MEM_readBE64(mut memPtr: *const libc::c_void) -> U64 {
    if 0 != MEM_isLittleEndian() {
        return MEM_swap64(MEM_read64(memPtr))
    } else { return MEM_read64(memPtr) };
}
unsafe extern "C" fn MEM_writeBE64(mut memPtr: *mut libc::c_void,
                                   mut val64: U64) -> () {
    if 0 != MEM_isLittleEndian() {
        MEM_write64(memPtr, MEM_swap64(val64));
    } else { MEM_write64(memPtr, val64); };
}
unsafe extern "C" fn MEM_readBEST(mut memPtr: *const libc::c_void) -> size_t {
    if 0 != MEM_32bits() {
        return MEM_readBE32(memPtr) as size_t
    } else { return MEM_readBE64(memPtr) };
}
unsafe extern "C" fn MEM_writeBEST(mut memPtr: *mut libc::c_void,
                                   mut val: size_t) -> () {
    if 0 != MEM_32bits() {
        MEM_writeBE32(memPtr, val as U32);
    } else { MEM_writeBE64(memPtr, val); };
}
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as
               libc::c_int as libc::c_uint;
}
unsafe extern "C" fn ERR_getErrorCode(mut code: size_t) -> ERR_enum {
    if 0 == ERR_isError(code) {
        return ZSTD_error_no_error
    } else { return (0i32 as libc::c_ulong).wrapping_sub(code) as ERR_enum };
}
unsafe extern "C" fn ERR_getErrorName(mut code: size_t)
 -> *const libc::c_char {
    return ERR_getErrorString(ERR_getErrorCode(code));
}
unsafe extern "C" fn BIT_initCStream(mut bitC: *mut BIT_CStream_t,
                                     mut startPtr: *mut libc::c_void,
                                     mut dstCapacity: size_t) -> size_t {
    (*bitC).bitContainer = 0i32 as size_t;
    (*bitC).bitPos = 0i32 as libc::c_uint;
    (*bitC).startPtr = startPtr as *mut libc::c_char;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC).endPtr =
        (*bitC).startPtr.offset(dstCapacity as
                                    isize).offset(-(::std::mem::size_of::<size_t>()
                                                        as libc::c_ulong as
                                                        isize));
    if dstCapacity <= ::std::mem::size_of::<size_t>() as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else { return 0i32 as size_t };
}
unsafe extern "C" fn BIT_addBits(mut bitC: *mut BIT_CStream_t,
                                 mut value: size_t, mut nbBits: libc::c_uint)
 -> () {
    (*bitC).bitContainer |=
        (value & BIT_mask[nbBits as usize] as libc::c_ulong) <<
            (*bitC).bitPos;
    (*bitC).bitPos = (*bitC).bitPos.wrapping_add(nbBits);
}
static mut BIT_mask: [libc::c_uint; 32] =
    unsafe {
        [0i32 as libc::c_uint, 1i32 as libc::c_uint, 3i32 as libc::c_uint,
         7i32 as libc::c_uint, 15i32 as libc::c_uint, 31i32 as libc::c_uint,
         63i32 as libc::c_uint, 127i32 as libc::c_uint,
         255i32 as libc::c_uint, 511i32 as libc::c_uint,
         1023i32 as libc::c_uint, 2047i32 as libc::c_uint,
         4095i32 as libc::c_uint, 8191i32 as libc::c_uint,
         16383i32 as libc::c_uint, 32767i32 as libc::c_uint,
         65535i32 as libc::c_uint, 131071i32 as libc::c_uint,
         262143i32 as libc::c_uint, 524287i32 as libc::c_uint,
         1048575i32 as libc::c_uint, 2097151i32 as libc::c_uint,
         4194303i32 as libc::c_uint, 8388607i32 as libc::c_uint,
         16777215i32 as libc::c_uint, 33554431i32 as libc::c_uint,
         67108863i32 as libc::c_uint, 134217727i32 as libc::c_uint,
         268435455i32 as libc::c_uint, 536870911i32 as libc::c_uint,
         1073741823i32 as libc::c_uint, 2147483647i32 as libc::c_uint]
    };
unsafe extern "C" fn BIT_flushBits(mut bitC: *mut BIT_CStream_t) -> () {
    let nbBytes: size_t = ((*bitC).bitPos >> 3i32) as size_t;
    MEM_writeLEST((*bitC).ptr as *mut libc::c_void, (*bitC).bitContainer);
    (*bitC).ptr = (*bitC).ptr.offset(nbBytes as isize);
    if (*bitC).ptr > (*bitC).endPtr { (*bitC).ptr = (*bitC).endPtr }
    (*bitC).bitPos &= 7i32 as libc::c_uint;
    (*bitC).bitContainer >>= nbBytes.wrapping_mul(8i32 as libc::c_ulong);
}
unsafe extern "C" fn BIT_closeCStream(mut bitC: *mut BIT_CStream_t)
 -> size_t {
    BIT_addBitsFast(bitC, 1i32 as size_t, 1i32 as libc::c_uint);
    BIT_flushBits(bitC);
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0i32 as size_t
    } else {
        return ((*bitC).startPtr.offset_to((*bitC).ptr).expect("bad offset_to")
                    as libc::c_long +
                    ((*bitC).bitPos > 0i32 as libc::c_uint) as libc::c_int as
                        libc::c_long) as size_t
    };
}
unsafe extern "C" fn BIT_addBitsFast(mut bitC: *mut BIT_CStream_t,
                                     mut value: size_t,
                                     mut nbBits: libc::c_uint) -> () {
    (*bitC).bitContainer |= value << (*bitC).bitPos;
    (*bitC).bitPos = (*bitC).bitPos.wrapping_add(nbBits);
}
unsafe extern "C" fn BIT_initDStream(mut bitD: *mut BIT_DStream_t,
                                     mut srcBuffer: *const libc::c_void,
                                     mut srcSize: size_t) -> size_t {
    let mut current_block: u64;
    if srcSize < 1i32 as libc::c_ulong {
        memset(bitD as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<BIT_DStream_t>() as libc::c_ulong);
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else {
        (*bitD).start = srcBuffer as *const libc::c_char;
        (*bitD).limitPtr =
            (*bitD).start.offset(::std::mem::size_of::<size_t>() as
                                     libc::c_ulong as isize);
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
                    current_block = 2758364018772413702;
                }
                6 => { current_block = 2758364018772413702; }
                5 => { current_block = 12231332282017165356; }
                4 => { current_block = 16758983331069857237; }
                3 => { current_block = 9576270869749310253; }
                2 => { current_block = 2679705107483395490; }
                _ => { current_block = 10886091980245723256; }
            }
            match current_block {
                2758364018772413702 => {
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
                    current_block = 12231332282017165356;
                }
                _ => { }
            }
            match current_block {
                12231332282017165356 => {
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
                    current_block = 16758983331069857237;
                }
                _ => { }
            }
            match current_block {
                16758983331069857237 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(3isize)
                                                              as size_t) <<
                                                             24i32) as size_t
                            as size_t;
                    current_block = 9576270869749310253;
                }
                _ => { }
            }
            match current_block {
                9576270869749310253 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(2isize)
                                                              as size_t) <<
                                                             16i32) as size_t
                            as size_t;
                    current_block = 2679705107483395490;
                }
                _ => { }
            }
            match current_block {
                2679705107483395490 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(1isize)
                                                              as size_t) <<
                                                             8i32) as size_t
                            as size_t
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
                         libc::c_uint).wrapping_sub(BIT_highbit32(lastByte_0
                                                                      as U32))
                } else { 0i32 as libc::c_uint };
            if lastByte_0 as libc::c_int == 0i32 {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            } else {
                (*bitD).bitsConsumed =
                    (*bitD).bitsConsumed.wrapping_add(((::std::mem::size_of::<size_t>()
                                                            as
                                                            libc::c_ulong).wrapping_sub(srcSize)
                                                           as
                                                           U32).wrapping_mul(8i32
                                                                                 as
                                                                                 libc::c_uint))
            }
        }
        return srcSize
    };
}
unsafe extern "C" fn BIT_highbit32(mut val: U32) -> libc::c_uint {
    return (31i32 - val.leading_zeros() as i32) as libc::c_uint;
}
unsafe extern "C" fn BIT_readBits(mut bitD: *mut BIT_DStream_t,
                                  mut nbBits: U32) -> size_t {
    let value: size_t = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
unsafe extern "C" fn BIT_lookBits(mut bitD: *const BIT_DStream_t,
                                  mut nbBits: U32) -> size_t {
    let regMask: U32 =
        (::std::mem::size_of::<size_t>() as
             libc::c_ulong).wrapping_mul(8i32 as
                                             libc::c_ulong).wrapping_sub(1i32
                                                                             as
                                                                             libc::c_ulong)
            as U32;
    return (*bitD).bitContainer << ((*bitD).bitsConsumed & regMask) >> 1i32 >>
               (regMask.wrapping_sub(nbBits) & regMask);
}
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t,
                                  mut nbBits: U32) -> () {
    (*bitD).bitsConsumed = (*bitD).bitsConsumed.wrapping_add(nbBits);
}
unsafe extern "C" fn BIT_reloadDStream(mut bitD: *mut BIT_DStream_t)
 -> BIT_DStream_status {
    if (*bitD).bitsConsumed as libc::c_ulong >
           (::std::mem::size_of::<size_t>() as
                libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
        return BIT_DStream_overflow
    } else if (*bitD).ptr >= (*bitD).limitPtr {
        (*bitD).ptr =
            (*bitD).ptr.offset(-(((*bitD).bitsConsumed >> 3i32) as isize));
        (*bitD).bitsConsumed &= 7i32 as libc::c_uint;
        (*bitD).bitContainer =
            MEM_readLEST((*bitD).ptr as *const libc::c_void);
        return BIT_DStream_unfinished
    } else if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as libc::c_ulong) <
               (::std::mem::size_of::<size_t>() as
                    libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
            return BIT_DStream_endOfBuffer
        } else { return BIT_DStream_completed }
    } else {
        let mut nbBytes: U32 = (*bitD).bitsConsumed >> 3i32;
        let mut result: BIT_DStream_status = BIT_DStream_unfinished;
        if (*bitD).ptr.offset(-(nbBytes as isize)) < (*bitD).start {
            nbBytes =
                (*bitD).start.offset_to((*bitD).ptr).expect("bad offset_to")
                    as libc::c_long as U32;
            result = BIT_DStream_endOfBuffer
        }
        (*bitD).ptr = (*bitD).ptr.offset(-(nbBytes as isize));
        (*bitD).bitsConsumed =
            (*bitD).bitsConsumed.wrapping_sub(nbBytes.wrapping_mul(8i32 as
                                                                       libc::c_uint));
        (*bitD).bitContainer =
            MEM_readLEST((*bitD).ptr as *const libc::c_void);
        return result
    };
}
unsafe extern "C" fn BIT_endOfDStream(mut DStream: *const BIT_DStream_t)
 -> libc::c_uint {
    return ((*DStream).ptr == (*DStream).start &&
                (*DStream).bitsConsumed as libc::c_ulong ==
                    (::std::mem::size_of::<size_t>() as
                         libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong))
               as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn BIT_flushBitsFast(mut bitC: *mut BIT_CStream_t) -> () {
    let nbBytes: size_t = ((*bitC).bitPos >> 3i32) as size_t;
    MEM_writeLEST((*bitC).ptr as *mut libc::c_void, (*bitC).bitContainer);
    (*bitC).ptr = (*bitC).ptr.offset(nbBytes as isize);
    (*bitC).bitPos &= 7i32 as libc::c_uint;
    (*bitC).bitContainer >>= nbBytes.wrapping_mul(8i32 as libc::c_ulong);
}
unsafe extern "C" fn BIT_readBitsFast(mut bitD: *mut BIT_DStream_t,
                                      mut nbBits: U32) -> size_t {
    let value: size_t = BIT_lookBitsFast(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
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
unsafe extern "C" fn BIT_getUpperBits(mut bitContainer: size_t, start: U32)
 -> size_t {
    return bitContainer >> start;
}
unsafe extern "C" fn BIT_getMiddleBits(mut bitContainer: size_t, start: U32,
                                       nbBits: U32) -> size_t {
    return bitContainer >> start & BIT_mask[nbBits as usize] as libc::c_ulong;
}
unsafe extern "C" fn BIT_getLowerBits(mut bitContainer: size_t, nbBits: U32)
 -> size_t {
    return bitContainer & BIT_mask[nbBits as usize] as libc::c_ulong;
}
unsafe extern "C" fn FSE_initCState(mut statePtr: *mut FSE_CState_t,
                                    mut ct: *const FSE_CTable) -> () {
    let mut ptr: *const libc::c_void = ct as *const libc::c_void;
    let mut u16ptr: *const U16 = ptr as *const U16;
    let tableLog: U32 = MEM_read16(ptr) as U32;
    (*statePtr).value = (1i32 as ptrdiff_t) << tableLog;
    (*statePtr).stateTable = u16ptr.offset(2isize) as *const libc::c_void;
    (*statePtr).symbolTT =
        ct.offset(1isize).offset((if 0 != tableLog {
                                      1i32 <<
                                          tableLog.wrapping_sub(1i32 as
                                                                    libc::c_uint)
                                  } else { 1i32 }) as isize) as
            *const libc::c_void;
    (*statePtr).stateLog = tableLog;
}
unsafe extern "C" fn FSE_encodeSymbol(mut bitC: *mut BIT_CStream_t,
                                      mut statePtr: *mut FSE_CState_t,
                                      mut symbol: U32) -> () {
    let symbolTT: FSE_symbolCompressionTransform =
        *((*statePtr).symbolTT as
              *const FSE_symbolCompressionTransform).offset(symbol as isize);
    let stateTable: *const U16 = (*statePtr).stateTable as *const U16;
    let nbBitsOut: U32 =
        ((*statePtr).value + symbolTT.deltaNbBits as libc::c_long >> 16i32) as
            U32;
    BIT_addBits(bitC, (*statePtr).value as size_t, nbBitsOut);
    (*statePtr).value =
        *stateTable.offset((((*statePtr).value >> nbBitsOut) +
                                symbolTT.deltaFindState as libc::c_long) as
                               isize) as ptrdiff_t;
}
unsafe extern "C" fn FSE_flushCState(mut bitC: *mut BIT_CStream_t,
                                     mut statePtr: *const FSE_CState_t)
 -> () {
    BIT_addBits(bitC, (*statePtr).value as size_t, (*statePtr).stateLog);
    BIT_flushBits(bitC);
}
unsafe extern "C" fn FSE_initDState(mut DStatePtr: *mut FSE_DState_t,
                                    mut bitD: *mut BIT_DStream_t,
                                    mut dt: *const FSE_DTable) -> () {
    let mut ptr: *const libc::c_void = dt as *const libc::c_void;
    let DTableH: *const FSE_DTableHeader = ptr as *const FSE_DTableHeader;
    (*DStatePtr).state =
        BIT_readBits(bitD, (*DTableH).tableLog as libc::c_uint);
    BIT_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1isize) as *const libc::c_void;
}
unsafe extern "C" fn FSE_decodeSymbol(mut DStatePtr: *mut FSE_DState_t,
                                      mut bitD: *mut BIT_DStream_t)
 -> libc::c_uchar {
    let DInfo: FSE_decode_t =
        *((*DStatePtr).table as
              *const FSE_decode_t).offset((*DStatePtr).state as isize);
    let nbBits: U32 = DInfo.nbBits as U32;
    let symbol: BYTE = DInfo.symbol;
    let lowBits: size_t = BIT_readBits(bitD, nbBits);
    (*DStatePtr).state =
        (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
    return symbol;
}
unsafe extern "C" fn FSE_endOfDState(mut DStatePtr: *const FSE_DState_t)
 -> libc::c_uint {
    return ((*DStatePtr).state == 0i32 as libc::c_ulong) as libc::c_int as
               libc::c_uint;
}
unsafe extern "C" fn FSE_decodeSymbolFast(mut DStatePtr: *mut FSE_DState_t,
                                          mut bitD: *mut BIT_DStream_t)
 -> libc::c_uchar {
    let DInfo: FSE_decode_t =
        *((*DStatePtr).table as
              *const FSE_decode_t).offset((*DStatePtr).state as isize);
    let nbBits: U32 = DInfo.nbBits as U32;
    let symbol: BYTE = DInfo.symbol;
    let lowBits: size_t = BIT_readBitsFast(bitD, nbBits);
    (*DStatePtr).state =
        (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
    return symbol;
}
unsafe extern "C" fn FSE_initCState2(mut statePtr: *mut FSE_CState_t,
                                     mut ct: *const FSE_CTable,
                                     mut symbol: U32) -> () {
    FSE_initCState(statePtr, ct);
    let symbolTT: FSE_symbolCompressionTransform =
        *((*statePtr).symbolTT as
              *const FSE_symbolCompressionTransform).offset(symbol as isize);
    let mut stateTable: *const U16 = (*statePtr).stateTable as *const U16;
    let mut nbBitsOut: U32 =
        symbolTT.deltaNbBits.wrapping_add((1i32 << 15i32) as libc::c_uint) >>
            16i32;
    (*statePtr).value =
        (nbBitsOut << 16i32).wrapping_sub(symbolTT.deltaNbBits) as ptrdiff_t;
    (*statePtr).value =
        *stateTable.offset((((*statePtr).value >> nbBitsOut) +
                                symbolTT.deltaFindState as libc::c_long) as
                               isize) as ptrdiff_t;
}
unsafe extern "C" fn FSE_getMaxNbBits(mut symbolTTPtr: *const libc::c_void,
                                      mut symbolValue: U32) -> U32 {
    let mut symbolTT: *const FSE_symbolCompressionTransform =
        symbolTTPtr as *const FSE_symbolCompressionTransform;
    return (*symbolTT.offset(symbolValue as
                                 isize)).deltaNbBits.wrapping_add(((1i32 <<
                                                                        16i32)
                                                                       - 1i32)
                                                                      as
                                                                      libc::c_uint)
               >> 16i32;
}
unsafe extern "C" fn FSE_bitCost(mut symbolTTPtr: *const libc::c_void,
                                 mut tableLog: U32, mut symbolValue: U32,
                                 mut accuracyLog: U32) -> U32 {
    let mut symbolTT: *const FSE_symbolCompressionTransform =
        symbolTTPtr as *const FSE_symbolCompressionTransform;
    let minNbBits: U32 =
        (*symbolTT.offset(symbolValue as isize)).deltaNbBits >> 16i32;
    let threshold: U32 =
        minNbBits.wrapping_add(1i32 as libc::c_uint) << 16i32;
    let tableSize: U32 = (1i32 << tableLog) as U32;
    let deltaFromThreshold: U32 =
        threshold.wrapping_sub((*symbolTT.offset(symbolValue as
                                                     isize)).deltaNbBits.wrapping_add(tableSize));
    let normalizedDeltaFromThreshold: U32 =
        deltaFromThreshold << accuracyLog >> tableLog;
    let bitMultiplier: U32 = (1i32 << accuracyLog) as U32;
    return minNbBits.wrapping_add(1i32 as
                                      libc::c_uint).wrapping_mul(bitMultiplier).wrapping_sub(normalizedDeltaFromThreshold);
}
unsafe extern "C" fn FSE_peekSymbol(mut DStatePtr: *const FSE_DState_t)
 -> BYTE {
    let DInfo: FSE_decode_t =
        *((*DStatePtr).table as
              *const FSE_decode_t).offset((*DStatePtr).state as isize);
    return DInfo.symbol;
}
unsafe extern "C" fn FSE_updateState(mut DStatePtr: *mut FSE_DState_t,
                                     mut bitD: *mut BIT_DStream_t) -> () {
    let DInfo: FSE_decode_t =
        *((*DStatePtr).table as
              *const FSE_decode_t).offset((*DStatePtr).state as isize);
    let nbBits: U32 = DInfo.nbBits as U32;
    let lowBits: size_t = BIT_readBits(bitD, nbBits);
    (*DStatePtr).state =
        (DInfo.newState as libc::c_ulong).wrapping_add(lowBits);
}
static mut repStartValue: [U32; 3] =
    unsafe { [1i32 as U32, 4i32 as U32, 8i32 as U32] };
static mut ZSTD_fcs_fieldSize: [size_t; 4] =
    unsafe {
        [0i32 as size_t, 2i32 as size_t, 4i32 as size_t, 8i32 as size_t]
    };
static mut ZSTD_did_fieldSize: [size_t; 4] =
    unsafe {
        [0i32 as size_t, 1i32 as size_t, 2i32 as size_t, 4i32 as size_t]
    };
static mut ZSTD_frameIdSize: size_t = unsafe { 4i32 as size_t };
static mut ZSTD_blockHeaderSize: size_t = unsafe { 3i32 as size_t };
static mut LL_bits: [U32; 36] =
    unsafe {
        [0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32,
         0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32,
         0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32, 0i32 as U32,
         0i32 as U32, 1i32 as U32, 1i32 as U32, 1i32 as U32, 1i32 as U32,
         2i32 as U32, 2i32 as U32, 3i32 as U32, 3i32 as U32, 4i32 as U32,
         6i32 as U32, 7i32 as U32, 8i32 as U32, 9i32 as U32, 10i32 as U32,
         11i32 as U32, 12i32 as U32, 13i32 as U32, 14i32 as U32, 15i32 as U32,
         16i32 as U32]
    };
static mut LL_defaultNorm: [S16; 36] =
    unsafe {
        [4i32 as S16, 3i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16,
         2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16,
         2i32 as S16, 2i32 as S16, 2i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16,
         2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16,
         3i32 as S16, 2i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, -1i32 as S16, -1i32 as S16, -1i32 as S16,
         -1i32 as S16]
    };
static mut LL_defaultNormLog: U32 = unsafe { 6i32 as U32 };
static mut ML_bits: [U32; 53] =
    unsafe {
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
         14i32 as U32, 15i32 as U32, 16i32 as U32]
    };
static mut ML_defaultNorm: [S16; 53] =
    unsafe {
        [1i32 as S16, 4i32 as S16, 3i32 as S16, 2i32 as S16, 2i32 as S16,
         2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, -1i32 as S16, -1i32 as S16, -1i32 as S16, -1i32 as S16,
         -1i32 as S16, -1i32 as S16, -1i32 as S16]
    };
static mut ML_defaultNormLog: U32 = unsafe { 6i32 as U32 };
static mut OF_defaultNorm: [S16; 29] =
    unsafe {
        [1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, -1i32 as S16,
         -1i32 as S16, -1i32 as S16, -1i32 as S16, -1i32 as S16]
    };
static mut OF_defaultNormLog: U32 = unsafe { 5i32 as U32 };
unsafe extern "C" fn ZSTD_copy8(mut dst: *mut libc::c_void,
                                mut src: *const libc::c_void) -> () {
    memcpy(dst, src, 8i32 as libc::c_ulong);
}
unsafe extern "C" fn ZSTD_wildcopy(mut dst: *mut libc::c_void,
                                   mut src: *const libc::c_void,
                                   mut length: ptrdiff_t) -> () {
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
unsafe extern "C" fn ZSTD_wildcopy_e(mut dst: *mut libc::c_void,
                                     mut src: *const libc::c_void,
                                     mut dstEnd: *mut libc::c_void) -> () {
    let mut ip: *const BYTE = src as *const BYTE;
    let mut op: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = dstEnd as *mut BYTE;
    loop  {
        ZSTD_copy8(op as *mut libc::c_void, ip as *const libc::c_void);
        op = op.offset(8isize);
        ip = ip.offset(8isize);
        if !(op < oend) { break ; }
    };
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
    if nbWorkers < 1i32 as libc::c_uint {
        return 0 as *mut ZSTDMT_CCtx
    } else {
        nbWorkers =
            if nbWorkers < 200i32 as libc::c_uint {
                nbWorkers
            } else { 200i32 as libc::c_uint };
        if 0 !=
               (cMem.customAlloc !=
                    ::std::mem::transmute::<*mut libc::c_void,
                                            ZSTD_allocFunction>(0 as
                                                                    *mut libc::c_void))
                   as libc::c_int ^
                   (cMem.customFree !=
                        ::std::mem::transmute::<*mut libc::c_void,
                                                ZSTD_freeFunction>(0 as
                                                                       *mut libc::c_void))
                       as libc::c_int {
            return 0 as *mut ZSTDMT_CCtx
        } else {
            mtctx =
                ZSTD_calloc(::std::mem::size_of::<ZSTDMT_CCtx>() as
                                libc::c_ulong, cMem) as *mut ZSTDMT_CCtx;
            if mtctx.is_null() {
                return 0 as *mut ZSTDMT_CCtx
            } else {
                ZSTDMT_CCtxParam_setNbWorkers(&mut (*mtctx).params as
                                                  *mut ZSTD_CCtx_params,
                                              nbWorkers);
                (*mtctx).cMem = cMem;
                (*mtctx).allJobsCompleted = 1i32 as libc::c_uint;
                (*mtctx).factory =
                    POOL_create_advanced(nbWorkers as size_t, 0i32 as size_t,
                                         cMem);
                (*mtctx).jobs =
                    ZSTDMT_createJobsTable(&mut nbJobs as *mut U32, cMem);
                (*mtctx).jobIDMask =
                    nbJobs.wrapping_sub(1i32 as libc::c_uint);
                (*mtctx).bufPool = ZSTDMT_createBufferPool(nbWorkers, cMem);
                (*mtctx).cctxPool = ZSTDMT_createCCtxPool(nbWorkers, cMem);
                (*mtctx).seqPool = ZSTDMT_createSeqPool(nbWorkers, cMem);
                initError =
                    ZSTDMT_serialState_init(&mut (*mtctx).serial as
                                                *mut serialState_t);
                (*mtctx).roundBuff = kNullRoundBuff;
                if 0 !=
                       (*mtctx).factory.is_null() as libc::c_int |
                           (*mtctx).jobs.is_null() as libc::c_int |
                           (*mtctx).bufPool.is_null() as libc::c_int |
                           (*mtctx).cctxPool.is_null() as libc::c_int |
                           (*mtctx).seqPool.is_null() as libc::c_int |
                           initError {
                    ZSTDMT_freeCCtx(mtctx);
                    return 0 as *mut ZSTDMT_CCtx
                } else { return mtctx }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_freeCCtx(mut mtctx: *mut ZSTDMT_CCtx)
 -> size_t {
    if mtctx.is_null() {
        return 0i32 as size_t
    } else {
        POOL_free((*mtctx).factory);
        ZSTDMT_releaseAllJobResources(mtctx);
        ZSTDMT_freeJobsTable((*mtctx).jobs,
                             (*mtctx).jobIDMask.wrapping_add(1i32 as
                                                                 libc::c_uint),
                             (*mtctx).cMem);
        ZSTDMT_freeBufferPool((*mtctx).bufPool);
        ZSTDMT_freeCCtxPool((*mtctx).cctxPool);
        ZSTDMT_freeSeqPool((*mtctx).seqPool);
        ZSTDMT_serialState_free(&mut (*mtctx).serial as *mut serialState_t);
        ZSTD_freeCDict((*mtctx).cdictLocal);
        if !(*mtctx).roundBuff.buffer.is_null() {
            ZSTD_free((*mtctx).roundBuff.buffer as *mut libc::c_void,
                      (*mtctx).cMem);
        }
        ZSTD_free(mtctx as *mut libc::c_void, (*mtctx).cMem);
        return 0i32 as size_t
    };
}
unsafe extern "C" fn ZSTDMT_serialState_free(mut serialState:
                                                 *mut serialState_t) -> () {
    let mut cMem: ZSTD_customMem = (*serialState).params.customMem;
    pthread_mutex_destroy(&mut (*serialState).mutex as *mut pthread_mutex_t);
    pthread_cond_destroy(&mut (*serialState).cond as *mut pthread_cond_t);
    pthread_mutex_destroy(&mut (*serialState).ldmWindowMutex as
                              *mut pthread_mutex_t);
    pthread_cond_destroy(&mut (*serialState).ldmWindowCond as
                             *mut pthread_cond_t);
    ZSTD_free((*serialState).ldmState.hashTable as *mut libc::c_void, cMem);
    ZSTD_free((*serialState).ldmState.bucketOffsets as *mut libc::c_void,
              cMem);
}
unsafe extern "C" fn ZSTDMT_freeSeqPool(mut seqPool: *mut ZSTDMT_seqPool)
 -> () {
    ZSTDMT_freeBufferPool(seqPool);
}
unsafe extern "C" fn ZSTDMT_freeBufferPool(mut bufPool:
                                               *mut ZSTDMT_bufferPool) -> () {
    let mut u: libc::c_uint = 0;
    if bufPool.is_null() {
        return
    } else {
        u = 0i32 as libc::c_uint;
        while u < (*bufPool).totalBuffers {
            ZSTD_free((*bufPool).bTable[u as usize].start, (*bufPool).cMem);
            u = u.wrapping_add(1)
        }
        pthread_mutex_destroy(&mut (*bufPool).poolMutex as
                                  *mut pthread_mutex_t);
        ZSTD_free(bufPool as *mut libc::c_void, (*bufPool).cMem);
        return;
    };
}
unsafe extern "C" fn ZSTDMT_freeCCtxPool(mut pool: *mut ZSTDMT_CCtxPool)
 -> () {
    let mut u: libc::c_uint = 0;
    u = 0i32 as libc::c_uint;
    while u < (*pool).totalCCtx {
        ZSTD_freeCCtx((*pool).cctx[u as usize]);
        u = u.wrapping_add(1)
    }
    pthread_mutex_destroy(&mut (*pool).poolMutex as *mut pthread_mutex_t);
    ZSTD_free(pool as *mut libc::c_void, (*pool).cMem);
}
unsafe extern "C" fn ZSTDMT_freeJobsTable(mut jobTable:
                                              *mut ZSTDMT_jobDescription,
                                          mut nbJobs: U32,
                                          mut cMem: ZSTD_customMem) -> () {
    let mut jobNb: U32 = 0;
    if jobTable.is_null() {
        return
    } else {
        jobNb = 0i32 as U32;
        while jobNb < nbJobs {
            pthread_mutex_destroy(&mut (*jobTable.offset(jobNb as
                                                             isize)).job_mutex
                                      as *mut pthread_mutex_t);
            pthread_cond_destroy(&mut (*jobTable.offset(jobNb as
                                                            isize)).job_cond
                                     as *mut pthread_cond_t);
            jobNb = jobNb.wrapping_add(1)
        }
        ZSTD_free(jobTable as *mut libc::c_void, cMem);
        return;
    };
}
unsafe extern "C" fn ZSTDMT_releaseAllJobResources(mut mtctx:
                                                       *mut ZSTDMT_CCtx)
 -> () {
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
    unsafe {
        buffer_s{start: 0 as *const libc::c_void as *mut libc::c_void,
                 capacity: 0i32 as size_t,}
    };
unsafe extern "C" fn ZSTDMT_releaseBuffer(mut bufPool: *mut ZSTDMT_bufferPool,
                                          mut buf: buffer_t) -> () {
    if buf.start.is_null() {
        return
    } else {
        pthread_mutex_lock(&mut (*bufPool).poolMutex as *mut pthread_mutex_t);
        if (*bufPool).nbBuffers < (*bufPool).totalBuffers {
            let fresh0 = (*bufPool).nbBuffers;
            (*bufPool).nbBuffers = (*bufPool).nbBuffers.wrapping_add(1);
            (*bufPool).bTable[fresh0 as usize] = buf;
            pthread_mutex_unlock(&mut (*bufPool).poolMutex as
                                     *mut pthread_mutex_t);
            return
        } else {
            pthread_mutex_unlock(&mut (*bufPool).poolMutex as
                                     *mut pthread_mutex_t);
            ZSTD_free(buf.start, (*bufPool).cMem);
            return;
        }
    };
}
static mut kNullRoundBuff: roundBuff_t =
    unsafe {
        roundBuff_t{buffer: 0 as *const BYTE as *mut BYTE,
                    capacity: 0i32 as size_t,
                    pos: 0i32 as size_t,}
    };
unsafe extern "C" fn ZSTDMT_serialState_init(mut serialState:
                                                 *mut serialState_t)
 -> libc::c_int {
    let mut initError: libc::c_int = 0i32;
    memset(serialState as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<serialState_t>() as libc::c_ulong);
    initError |=
        pthread_mutex_init(&mut (*serialState).mutex as *mut pthread_mutex_t,
                           0 as *const pthread_mutexattr_t);
    initError |=
        pthread_cond_init(&mut (*serialState).cond as *mut pthread_cond_t,
                          0 as *const pthread_condattr_t);
    initError |=
        pthread_mutex_init(&mut (*serialState).ldmWindowMutex as
                               *mut pthread_mutex_t,
                           0 as *const pthread_mutexattr_t);
    initError |=
        pthread_cond_init(&mut (*serialState).ldmWindowCond as
                              *mut pthread_cond_t,
                          0 as *const pthread_condattr_t);
    return initError;
}
unsafe extern "C" fn ZSTDMT_createSeqPool(mut nbWorkers: libc::c_uint,
                                          mut cMem: ZSTD_customMem)
 -> *mut ZSTDMT_seqPool {
    let mut seqPool: *mut ZSTDMT_seqPool =
        ZSTDMT_createBufferPool(nbWorkers, cMem);
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
    if bufPool.is_null() {
        return 0 as *mut ZSTDMT_bufferPool
    } else if 0 !=
                  pthread_mutex_init(&mut (*bufPool).poolMutex as
                                         *mut pthread_mutex_t,
                                     0 as *const pthread_mutexattr_t) {
        ZSTD_free(bufPool as *mut libc::c_void, cMem);
        return 0 as *mut ZSTDMT_bufferPool
    } else {
        (*bufPool).bufferSize = (64i32 * (1i32 << 10i32)) as size_t;
        (*bufPool).totalBuffers = maxNbBuffers;
        (*bufPool).nbBuffers = 0i32 as libc::c_uint;
        (*bufPool).cMem = cMem;
        return bufPool
    };
}
unsafe extern "C" fn ZSTDMT_setNbSeq(seqPool: *mut ZSTDMT_seqPool,
                                     nbSeq: size_t) -> () {
    ZSTDMT_setBufferSize(seqPool,
                         nbSeq.wrapping_mul(::std::mem::size_of::<rawSeq>() as
                                                libc::c_ulong));
}
unsafe extern "C" fn ZSTDMT_setBufferSize(bufPool: *mut ZSTDMT_bufferPool,
                                          bSize: size_t) -> () {
    pthread_mutex_lock(&mut (*bufPool).poolMutex as *mut pthread_mutex_t);
    (*bufPool).bufferSize = bSize;
    pthread_mutex_unlock(&mut (*bufPool).poolMutex as *mut pthread_mutex_t);
}
unsafe extern "C" fn ZSTDMT_createCCtxPool(mut nbWorkers: libc::c_uint,
                                           mut cMem: ZSTD_customMem)
 -> *mut ZSTDMT_CCtxPool {
    let cctxPool: *mut ZSTDMT_CCtxPool =
        ZSTD_calloc((::std::mem::size_of::<ZSTDMT_CCtxPool>() as
                         libc::c_ulong).wrapping_add((nbWorkers.wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_uint)
                                                          as
                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut ZSTD_CCtx>()
                                                                                          as
                                                                                          libc::c_ulong)),
                    cMem) as *mut ZSTDMT_CCtxPool;
    if cctxPool.is_null() {
        return 0 as *mut ZSTDMT_CCtxPool
    } else if 0 !=
                  pthread_mutex_init(&mut (*cctxPool).poolMutex as
                                         *mut pthread_mutex_t,
                                     0 as *const pthread_mutexattr_t) {
        ZSTD_free(cctxPool as *mut libc::c_void, cMem);
        return 0 as *mut ZSTDMT_CCtxPool
    } else {
        (*cctxPool).cMem = cMem;
        (*cctxPool).totalCCtx = nbWorkers;
        (*cctxPool).availCCtx = 1i32 as libc::c_uint;
        (*cctxPool).cctx[0usize] = ZSTD_createCCtx_advanced(cMem);
        if (*cctxPool).cctx[0usize].is_null() {
            ZSTDMT_freeCCtxPool(cctxPool);
            return 0 as *mut ZSTDMT_CCtxPool
        } else { return cctxPool }
    };
}
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
    if jobTable.is_null() {
        return 0 as *mut ZSTDMT_jobDescription
    } else {
        *nbJobsPtr = nbJobs;
        jobNb = 0i32 as U32;
        while jobNb < nbJobs {
            initError |=
                pthread_mutex_init(&mut (*jobTable.offset(jobNb as
                                                              isize)).job_mutex
                                       as *mut pthread_mutex_t,
                                   0 as *const pthread_mutexattr_t);
            initError |=
                pthread_cond_init(&mut (*jobTable.offset(jobNb as
                                                             isize)).job_cond
                                      as *mut pthread_cond_t,
                                  0 as *const pthread_condattr_t);
            jobNb = jobNb.wrapping_add(1)
        }
        if initError != 0i32 {
            ZSTDMT_freeJobsTable(jobTable, nbJobs, cMem);
            return 0 as *mut ZSTDMT_jobDescription
        } else { return jobTable }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_CCtxParam_setNbWorkers(mut params:
                                                           *mut ZSTD_CCtx_params,
                                                       mut nbWorkers:
                                                           libc::c_uint)
 -> size_t {
    if nbWorkers > 200i32 as libc::c_uint {
        nbWorkers = 200i32 as libc::c_uint
    }
    (*params).nbWorkers = nbWorkers;
    (*params).overlapSizeLog = 6i32 as libc::c_uint;
    (*params).jobSize = 0i32 as libc::c_uint;
    return nbWorkers as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_sizeof_CCtx(mut mtctx: *mut ZSTDMT_CCtx)
 -> size_t {
    if mtctx.is_null() {
        return 0i32 as size_t
    } else {
        return (::std::mem::size_of::<ZSTDMT_CCtx>() as
                    libc::c_ulong).wrapping_add(POOL_sizeof((*mtctx).factory)).wrapping_add(ZSTDMT_sizeof_bufferPool((*mtctx).bufPool)).wrapping_add(((*mtctx).jobIDMask.wrapping_add(1i32
                                                                                                                                                                                          as
                                                                                                                                                                                          libc::c_uint)
                                                                                                                                                          as
                                                                                                                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<ZSTDMT_jobDescription>()
                                                                                                                                                                                          as
                                                                                                                                                                                          libc::c_ulong)).wrapping_add(ZSTDMT_sizeof_CCtxPool((*mtctx).cctxPool)).wrapping_add(ZSTDMT_sizeof_seqPool((*mtctx).seqPool)).wrapping_add(ZSTD_sizeof_CDict((*mtctx).cdictLocal)).wrapping_add((*mtctx).roundBuff.capacity)
    };
}
unsafe extern "C" fn ZSTDMT_sizeof_seqPool(mut seqPool: *mut ZSTDMT_seqPool)
 -> size_t {
    return ZSTDMT_sizeof_bufferPool(seqPool);
}
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
    pthread_mutex_lock(&mut (*bufPool).poolMutex as *mut pthread_mutex_t);
    u = 0i32 as libc::c_uint;
    while u < (*bufPool).totalBuffers {
        totalBufferSize =
            (totalBufferSize as
                 libc::c_ulong).wrapping_add((*bufPool).bTable[u as
                                                                   usize].capacity)
                as size_t as size_t;
        u = u.wrapping_add(1)
    }
    pthread_mutex_unlock(&mut (*bufPool).poolMutex as *mut pthread_mutex_t);
    return poolSize.wrapping_add(totalBufferSize);
}
unsafe extern "C" fn ZSTDMT_sizeof_CCtxPool(mut cctxPool:
                                                *mut ZSTDMT_CCtxPool)
 -> size_t {
    pthread_mutex_lock(&mut (*cctxPool).poolMutex as *mut pthread_mutex_t);
    let nbWorkers: libc::c_uint = (*cctxPool).totalCCtx;
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
    pthread_mutex_unlock(&mut (*cctxPool).poolMutex as *mut pthread_mutex_t);
    return poolSize.wrapping_add(totalCCtxSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_compressCCtx(mut mtctx: *mut ZSTDMT_CCtx,
                                             mut dst: *mut libc::c_void,
                                             mut dstCapacity: size_t,
                                             mut src: *const libc::c_void,
                                             mut srcSize: size_t,
                                             mut compressionLevel:
                                                 libc::c_int) -> size_t {
    let overlapLog: U32 =
        (if compressionLevel >= ZSTD_maxCLevel() { 9i32 } else { 6i32 }) as
            U32;
    let mut params: ZSTD_parameters =
        ZSTD_getParams(compressionLevel, srcSize as libc::c_ulonglong,
                       0i32 as size_t);
    params.fParams.contentSizeFlag = 1i32 as libc::c_uint;
    return ZSTDMT_compress_advanced(mtctx, dst, dstCapacity, src, srcSize,
                                    0 as *const ZSTD_CDict, params,
                                    overlapLog);
}
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
                                                  mut overlapLog:
                                                      libc::c_uint)
 -> size_t {
    let mut cctxParams: ZSTD_CCtx_params = (*mtctx).params;
    cctxParams.cParams = params.cParams;
    cctxParams.fParams = params.fParams;
    cctxParams.overlapSizeLog = overlapLog;
    return ZSTDMT_compress_advanced_internal(mtctx, dst, dstCapacity, src,
                                             srcSize, cdict, cctxParams);
}
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
    let overlapSize: size_t =
        (1i32 as size_t) << ZSTDMT_computeOverlapLog(params);
    let nbJobs: libc::c_uint =
        ZSTDMT_computeNbJobs(params, srcSize, params.nbWorkers);
    let proposedJobSize: size_t =
        srcSize.wrapping_add(nbJobs.wrapping_sub(1i32 as libc::c_uint) as
                                 libc::c_ulong).wrapping_div(nbJobs as
                                                                 libc::c_ulong);
    let avgJobSize: size_t =
        if (proposedJobSize.wrapping_sub(1i32 as libc::c_ulong) &
                131071i32 as libc::c_ulong) < 32767i32 as libc::c_ulong {
            proposedJobSize.wrapping_add(65535i32 as libc::c_ulong)
        } else { proposedJobSize };
    let srcStart: *const libc::c_char = src as *const libc::c_char;
    let mut remainingSrcSize: size_t = srcSize;
    let compressWithinDst: libc::c_uint =
        if dstCapacity >= ZSTD_compressBound(srcSize) {
            nbJobs
        } else {
            dstCapacity.wrapping_div(ZSTD_compressBound(avgJobSize)) as
                libc::c_uint
        };
    let mut frameStartPos: size_t = 0i32 as size_t;
    let mut dstBufferPos: size_t = 0i32 as size_t;
    params.jobSize = avgJobSize as U32;
    if 0 !=
           (nbJobs == 1i32 as libc::c_uint) as libc::c_int |
               (params.nbWorkers <= 1i32 as libc::c_uint) as libc::c_int {
        let cctx: *mut ZSTD_CCtx = (*(*mtctx).cctxPool).cctx[0usize];
        if !cdict.is_null() {
            return ZSTD_compress_usingCDict_advanced(cctx, dst, dstCapacity,
                                                     src, srcSize, cdict,
                                                     jobParams.fParams)
        } else {
            return ZSTD_compress_advanced_internal(cctx, dst, dstCapacity,
                                                   src, srcSize,
                                                   0 as *const libc::c_void,
                                                   0i32 as size_t, jobParams)
        }
    } else {
        ZSTDMT_setBufferSize((*mtctx).bufPool,
                             ZSTD_compressBound(avgJobSize));
        if 0 !=
               ZSTDMT_serialState_reset(&mut (*mtctx).serial as
                                            *mut serialState_t,
                                        (*mtctx).seqPool, params, avgJobSize)
           {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        } else {
            let errcod: size_t = ZSTDMT_expandJobsTable(mtctx, nbJobs);
            if 0 != ERR_isError(errcod) {
                return errcod
            } else {
                let mut u: libc::c_uint = 0;
                u = 0i32 as libc::c_uint;
                while u < nbJobs {
                    let jobSize: size_t =
                        if remainingSrcSize < avgJobSize {
                            remainingSrcSize
                        } else { avgJobSize };
                    let dstBufferCapacity: size_t =
                        ZSTD_compressBound(jobSize);
                    let dstAsBuffer: buffer_t =
                        buffer_s{start:
                                     (dst as
                                          *mut libc::c_char).offset(dstBufferPos
                                                                        as
                                                                        isize)
                                         as *mut libc::c_void,
                                 capacity: dstBufferCapacity,};
                    let dstBuffer: buffer_t =
                        if u < compressWithinDst {
                            dstAsBuffer
                        } else { g_nullBuffer };
                    let mut dictSize: size_t =
                        if 0 != u {
                            overlapSize
                        } else { 0i32 as libc::c_ulong };
                    let ref mut fresh1 =
                        (*(*mtctx).jobs.offset(u as isize)).prefix.start;
                    *fresh1 =
                        srcStart.offset(frameStartPos as
                                            isize).offset(-(dictSize as
                                                                isize)) as
                            *const libc::c_void;
                    (*(*mtctx).jobs.offset(u as isize)).prefix.size =
                        dictSize;
                    let ref mut fresh2 =
                        (*(*mtctx).jobs.offset(u as isize)).src.start;
                    *fresh2 =
                        srcStart.offset(frameStartPos as isize) as
                            *const libc::c_void;
                    (*(*mtctx).jobs.offset(u as isize)).src.size = jobSize;
                    (*(*mtctx).jobs.offset(u as isize)).consumed =
                        0i32 as size_t;
                    (*(*mtctx).jobs.offset(u as isize)).cSize =
                        0i32 as size_t;
                    let ref mut fresh3 =
                        (*(*mtctx).jobs.offset(u as isize)).cdict;
                    *fresh3 =
                        if u == 0i32 as libc::c_uint {
                            cdict
                        } else { 0 as *const ZSTD_CDict };
                    (*(*mtctx).jobs.offset(u as isize)).fullFrameSize =
                        srcSize as libc::c_ulonglong;
                    (*(*mtctx).jobs.offset(u as isize)).params = jobParams;
                    (*(*mtctx).jobs.offset(u as isize)).dstBuff = dstBuffer;
                    let ref mut fresh4 =
                        (*(*mtctx).jobs.offset(u as isize)).cctxPool;
                    *fresh4 = (*mtctx).cctxPool;
                    let ref mut fresh5 =
                        (*(*mtctx).jobs.offset(u as isize)).bufPool;
                    *fresh5 = (*mtctx).bufPool;
                    let ref mut fresh6 =
                        (*(*mtctx).jobs.offset(u as isize)).seqPool;
                    *fresh6 = (*mtctx).seqPool;
                    let ref mut fresh7 =
                        (*(*mtctx).jobs.offset(u as isize)).serial;
                    *fresh7 = &mut (*mtctx).serial as *mut serialState_t;
                    (*(*mtctx).jobs.offset(u as isize)).jobID = u;
                    (*(*mtctx).jobs.offset(u as isize)).firstJob =
                        (u == 0i32 as libc::c_uint) as libc::c_int as
                            libc::c_uint;
                    (*(*mtctx).jobs.offset(u as isize)).lastJob =
                        (u == nbJobs.wrapping_sub(1i32 as libc::c_uint)) as
                            libc::c_int as libc::c_uint;
                    POOL_add((*mtctx).factory, Some(ZSTDMT_compressionJob),
                             &mut *(*mtctx).jobs.offset(u as isize) as
                                 *mut ZSTDMT_jobDescription as
                                 *mut libc::c_void);
                    frameStartPos =
                        (frameStartPos as libc::c_ulong).wrapping_add(jobSize)
                            as size_t as size_t;
                    dstBufferPos =
                        (dstBufferPos as
                             libc::c_ulong).wrapping_add(dstBufferCapacity) as
                            size_t as size_t;
                    remainingSrcSize =
                        (remainingSrcSize as
                             libc::c_ulong).wrapping_sub(jobSize) as size_t as
                            size_t;
                    u = u.wrapping_add(1)
                }
                let mut error: size_t = 0i32 as size_t;
                let mut dstPos: size_t = 0i32 as size_t;
                let mut jobID: libc::c_uint = 0;
                jobID = 0i32 as libc::c_uint;
                while jobID < nbJobs {
                    pthread_mutex_lock(&mut (*(*mtctx).jobs.offset(jobID as
                                                                       isize)).job_mutex
                                           as *mut pthread_mutex_t);
                    while (*(*mtctx).jobs.offset(jobID as isize)).consumed <
                              (*(*mtctx).jobs.offset(jobID as isize)).src.size
                          {
                        pthread_cond_wait(&mut (*(*mtctx).jobs.offset(jobID as
                                                                          isize)).job_cond
                                              as *mut pthread_cond_t,
                                          &mut (*(*mtctx).jobs.offset(jobID as
                                                                          isize)).job_mutex
                                              as *mut pthread_mutex_t);
                    }
                    pthread_mutex_unlock(&mut (*(*mtctx).jobs.offset(jobID as
                                                                         isize)).job_mutex
                                             as *mut pthread_mutex_t);
                    let cSize: size_t =
                        (*(*mtctx).jobs.offset(jobID as isize)).cSize;
                    if 0 != ZSTD_isError(cSize) { error = cSize }
                    if 0 == error && dstPos.wrapping_add(cSize) > dstCapacity
                       {
                        error =
                            -(ZSTD_error_dstSize_tooSmall as libc::c_int) as
                                size_t
                    }
                    if 0 != jobID {
                        if 0 == error {
                            memmove((dst as
                                         *mut libc::c_char).offset(dstPos as
                                                                       isize)
                                        as *mut libc::c_void,
                                    (*(*mtctx).jobs.offset(jobID as
                                                               isize)).dstBuff.start,
                                    cSize);
                        }
                        if jobID >= compressWithinDst {
                            ZSTDMT_releaseBuffer((*mtctx).bufPool,
                                                 (*(*mtctx).jobs.offset(jobID
                                                                            as
                                                                            isize)).dstBuff);
                        }
                    }
                    (*(*mtctx).jobs.offset(jobID as isize)).dstBuff =
                        g_nullBuffer;
                    (*(*mtctx).jobs.offset(jobID as isize)).cSize =
                        0i32 as size_t;
                    dstPos =
                        (dstPos as libc::c_ulong).wrapping_add(cSize) as
                            size_t as size_t;
                    jobID = jobID.wrapping_add(1)
                }
                if 0 != params.fParams.checksumFlag {
                    let checksum: U32 =
                        ZSTD_XXH64_digest(&mut (*mtctx).serial.xxhState as
                                              *mut XXH64_state_t) as U32;
                    if dstPos.wrapping_add(4i32 as libc::c_ulong) >
                           dstCapacity {
                        error =
                            -(ZSTD_error_dstSize_tooSmall as libc::c_int) as
                                size_t
                    } else {
                        MEM_writeLE32((dst as
                                           *mut libc::c_char).offset(dstPos as
                                                                         isize)
                                          as *mut libc::c_void, checksum);
                        dstPos =
                            (dstPos as
                                 libc::c_ulong).wrapping_add(4i32 as
                                                                 libc::c_ulong)
                                as size_t as size_t
                    }
                }
                0 == error;
                return if 0 != error { error } else { dstPos }
            }
        }
    };
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
unsafe extern "C" fn ZSTDMT_computeTargetJobLog(params: ZSTD_CCtx_params)
 -> size_t {
    if 0 != params.ldmParams.enableLdm {
        return (if 21i32 as libc::c_uint >
                       params.cParams.chainLog.wrapping_add(4i32 as
                                                                libc::c_uint)
                   {
                    21i32 as libc::c_uint
                } else {
                    params.cParams.chainLog.wrapping_add(4i32 as libc::c_uint)
                }) as size_t
    } else {
        return (if 20i32 as libc::c_uint >
                       params.cParams.windowLog.wrapping_add(2i32 as
                                                                 libc::c_uint)
                   {
                    20i32 as libc::c_uint
                } else {
                    params.cParams.windowLog.wrapping_add(2i32 as
                                                              libc::c_uint)
                }) as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_compressionJob(mut jobDescription:
                                                   *mut libc::c_void) -> () {
    let mut current_block: u64;
    let job: *mut ZSTDMT_jobDescription =
        jobDescription as *mut ZSTDMT_jobDescription;
    let mut jobParams: ZSTD_CCtx_params = (*job).params;
    let cctx: *mut ZSTD_CCtx = ZSTDMT_getCCtx((*job).cctxPool);
    let mut rawSeqStore: rawSeqStore_t = ZSTDMT_getSeq((*job).seqPool);
    let mut dstBuff: buffer_t = (*job).dstBuff;
    if cctx.is_null() {
        (*job).cSize =
            -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    } else {
        if dstBuff.start.is_null() {
            dstBuff = ZSTDMT_getBuffer((*job).bufPool);
            if dstBuff.start.is_null() {
                (*job).cSize =
                    -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
                current_block = 15381884091895413528;
            } else {
                (*job).dstBuff = dstBuff;
                current_block = 14916268686031723178;
            }
        } else { current_block = 14916268686031723178; }
        match current_block {
            15381884091895413528 => { }
            _ => {
                if 0 != jobParams.ldmParams.enableLdm &&
                       rawSeqStore.seq.is_null() {
                    (*job).cSize =
                        -(ZSTD_error_memory_allocation as libc::c_int) as
                            size_t
                } else {
                    if (*job).jobID != 0i32 as libc::c_uint {
                        jobParams.fParams.checksumFlag = 0i32 as libc::c_uint
                    }
                    jobParams.ldmParams.enableLdm = 0i32 as U32;
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
                        if 0 != ZSTD_isError(initError) {
                            (*job).cSize = initError;
                            current_block = 15381884091895413528;
                        } else { current_block = 11006700562992250127; }
                    } else {
                        let pledgedSrcSize: U64 =
                            (if 0 != (*job).firstJob {
                                 (*job).fullFrameSize
                             } else { (*job).src.size as libc::c_ulonglong })
                                as U64;
                        let forceWindowError: size_t =
                            ZSTD_CCtxParam_setParameter(&mut jobParams as
                                                            *mut ZSTD_CCtx_params,
                                                        ZSTD_p_forceMaxWindow,
                                                        (0 == (*job).firstJob)
                                                            as libc::c_int as
                                                            libc::c_uint);
                        if 0 != ZSTD_isError(forceWindowError) {
                            (*job).cSize = forceWindowError;
                            current_block = 15381884091895413528;
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
                            if 0 != ZSTD_isError(initError_0) {
                                (*job).cSize = initError_0;
                                current_block = 15381884091895413528;
                            } else { current_block = 11006700562992250127; }
                        }
                    }
                    match current_block {
                        15381884091895413528 => { }
                        _ => {
                            ZSTDMT_serialState_update((*job).serial, cctx,
                                                      rawSeqStore, (*job).src,
                                                      (*job).jobID);
                            if 0 == (*job).firstJob {
                                let hSize: size_t =
                                    ZSTD_compressContinue(cctx, dstBuff.start,
                                                          dstBuff.capacity,
                                                          (*job).src.start,
                                                          0i32 as size_t);
                                if 0 != ZSTD_isError(hSize) {
                                    (*job).cSize = hSize;
                                    current_block = 15381884091895413528;
                                } else {
                                    ZSTD_invalidateRepCodes(cctx);
                                    current_block = 5689001924483802034;
                                }
                            } else { current_block = 5689001924483802034; }
                            match current_block {
                                15381884091895413528 => { }
                                _ => {
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
                                                18317007320854588510;
                                            break ;
                                        }
                                        let cSize: size_t =
                                            ZSTD_compressContinue(cctx,
                                                                  op as
                                                                      *mut libc::c_void,
                                                                  op.offset_to(oend).expect("bad offset_to")
                                                                      as
                                                                      libc::c_long
                                                                      as
                                                                      size_t,
                                                                  ip as
                                                                      *const libc::c_void,
                                                                  chunkSize);
                                        if 0 != ZSTD_isError(cSize) {
                                            (*job).cSize = cSize;
                                            current_block =
                                                15381884091895413528;
                                            break ;
                                        } else {
                                            ip =
                                                ip.offset(chunkSize as isize);
                                            op = op.offset(cSize as isize);
                                            pthread_mutex_lock(&mut (*job).job_mutex
                                                                   as
                                                                   *mut pthread_mutex_t);
                                            (*job).cSize =
                                                ((*job).cSize as
                                                     libc::c_ulong).wrapping_add(cSize)
                                                    as size_t as size_t;
                                            (*job).consumed =
                                                chunkSize.wrapping_mul(chunkNb
                                                                           as
                                                                           libc::c_ulong);
                                            pthread_cond_signal(&mut (*job).job_cond
                                                                    as
                                                                    *mut pthread_cond_t);
                                            pthread_mutex_unlock(&mut (*job).job_mutex
                                                                     as
                                                                     *mut pthread_mutex_t);
                                            chunkNb += 1
                                        }
                                    }
                                    match current_block {
                                        15381884091895413528 => { }
                                        _ => {
                                            if 0 !=
                                                   (nbChunks > 0i32) as
                                                       libc::c_int as
                                                       libc::c_uint |
                                                       (*job).lastJob {
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
                                                                         op.offset_to(oend).expect("bad offset_to")
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
                                                                              op.offset_to(oend).expect("bad offset_to")
                                                                                  as
                                                                                  libc::c_long
                                                                                  as
                                                                                  size_t,
                                                                              ip
                                                                                  as
                                                                                  *const libc::c_void,
                                                                              lastBlockSize)
                                                    };
                                                if 0 != ZSTD_isError(cSize_0)
                                                   {
                                                    (*job).cSize = cSize_0
                                                } else {
                                                    pthread_mutex_lock(&mut (*job).job_mutex
                                                                           as
                                                                           *mut pthread_mutex_t);
                                                    (*job).cSize =
                                                        ((*job).cSize as
                                                             libc::c_ulong).wrapping_add(cSize_0)
                                                            as size_t as
                                                            size_t;
                                                    pthread_mutex_unlock(&mut (*job).job_mutex
                                                                             as
                                                                             *mut pthread_mutex_t);
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
    pthread_mutex_lock(&mut (*job).job_mutex as *mut pthread_mutex_t);
    (*job).consumed = (*job).src.size;
    pthread_cond_signal(&mut (*job).job_cond as *mut pthread_cond_t);
    pthread_mutex_unlock(&mut (*job).job_mutex as *mut pthread_mutex_t);
}
unsafe extern "C" fn ZSTDMT_getCCtx(mut cctxPool: *mut ZSTDMT_CCtxPool)
 -> *mut ZSTD_CCtx {
    pthread_mutex_lock(&mut (*cctxPool).poolMutex as *mut pthread_mutex_t);
    if 0 != (*cctxPool).availCCtx {
        (*cctxPool).availCCtx = (*cctxPool).availCCtx.wrapping_sub(1);
        let cctx: *mut ZSTD_CCtx =
            (*cctxPool).cctx[(*cctxPool).availCCtx as usize];
        pthread_mutex_unlock(&mut (*cctxPool).poolMutex as
                                 *mut pthread_mutex_t);
        return cctx
    } else {
        pthread_mutex_unlock(&mut (*cctxPool).poolMutex as
                                 *mut pthread_mutex_t);
        return ZSTD_createCCtx_advanced((*cctxPool).cMem)
    };
}
unsafe extern "C" fn ZSTDMT_releaseCCtx(mut pool: *mut ZSTDMT_CCtxPool,
                                        mut cctx: *mut ZSTD_CCtx) -> () {
    if cctx.is_null() {
        return
    } else {
        pthread_mutex_lock(&mut (*pool).poolMutex as *mut pthread_mutex_t);
        if (*pool).availCCtx < (*pool).totalCCtx {
            let fresh8 = (*pool).availCCtx;
            (*pool).availCCtx = (*pool).availCCtx.wrapping_add(1);
            (*pool).cctx[fresh8 as usize] = cctx
        } else { ZSTD_freeCCtx(cctx); }
        pthread_mutex_unlock(&mut (*pool).poolMutex as *mut pthread_mutex_t);
        return;
    };
}
unsafe extern "C" fn ZSTDMT_getSeq(mut seqPool: *mut ZSTDMT_seqPool)
 -> rawSeqStore_t {
    if (*seqPool).bufferSize == 0i32 as libc::c_ulong {
        return kNullRawSeqStore
    } else { return bufferToSeq(ZSTDMT_getBuffer(seqPool)) };
}
unsafe extern "C" fn ZSTDMT_getBuffer(mut bufPool: *mut ZSTDMT_bufferPool)
 -> buffer_t {
    let bSize: size_t = (*bufPool).bufferSize;
    pthread_mutex_lock(&mut (*bufPool).poolMutex as *mut pthread_mutex_t);
    if 0 != (*bufPool).nbBuffers {
        (*bufPool).nbBuffers = (*bufPool).nbBuffers.wrapping_sub(1);
        let buf: buffer_t = (*bufPool).bTable[(*bufPool).nbBuffers as usize];
        let availBufferSize: size_t = buf.capacity;
        (*bufPool).bTable[(*bufPool).nbBuffers as usize] = g_nullBuffer;
        if 0 !=
               (availBufferSize >= bSize) as libc::c_int &
                   (availBufferSize >> 3i32 <= bSize) as libc::c_int {
            pthread_mutex_unlock(&mut (*bufPool).poolMutex as
                                     *mut pthread_mutex_t);
            return buf
        } else { ZSTD_free(buf.start, (*bufPool).cMem); }
    }
    pthread_mutex_unlock(&mut (*bufPool).poolMutex as *mut pthread_mutex_t);
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
static mut kNullRawSeqStore: rawSeqStore_t =
    unsafe {
        rawSeqStore_t{seq: 0 as *const rawSeq as *mut rawSeq,
                      pos: 0i32 as size_t,
                      size: 0i32 as size_t,
                      capacity: 0i32 as size_t,}
    };
unsafe extern "C" fn ZSTDMT_releaseSeq(mut seqPool: *mut ZSTDMT_seqPool,
                                       mut seq: rawSeqStore_t) -> () {
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
                                                       mut cSize: size_t)
 -> () {
    pthread_mutex_lock(&mut (*serialState).mutex as *mut pthread_mutex_t);
    if (*serialState).nextJobID <= jobID {
        (*serialState).nextJobID = jobID.wrapping_add(1i32 as libc::c_uint);
        pthread_cond_broadcast(&mut (*serialState).cond as
                                   *mut pthread_cond_t);
        pthread_mutex_lock(&mut (*serialState).ldmWindowMutex as
                               *mut pthread_mutex_t);
        ZSTD_window_clear(&mut (*serialState).ldmWindow as
                              *mut ZSTD_window_t);
        pthread_cond_signal(&mut (*serialState).ldmWindowCond as
                                *mut pthread_cond_t);
        pthread_mutex_unlock(&mut (*serialState).ldmWindowMutex as
                                 *mut pthread_mutex_t);
    }
    pthread_mutex_unlock(&mut (*serialState).mutex as *mut pthread_mutex_t);
}
unsafe extern "C" fn ZSTD_window_clear(mut window: *mut ZSTD_window_t) -> () {
    let endT: size_t =
        (*window).base.offset_to((*window).nextSrc).expect("bad offset_to") as
            libc::c_long as size_t;
    let end: U32 = endT as U32;
    (*window).lowLimit = end;
    (*window).dictLimit = end;
}
unsafe extern "C" fn ZSTDMT_serialState_update(mut serialState:
                                                   *mut serialState_t,
                                               mut jobCCtx: *mut ZSTD_CCtx,
                                               mut seqStore: rawSeqStore_t,
                                               mut src: range_t,
                                               mut jobID: libc::c_uint)
 -> () {
    let mut error: size_t = 0;
    pthread_mutex_lock(&mut (*serialState).mutex as *mut pthread_mutex_t);
    while (*serialState).nextJobID < jobID {
        pthread_cond_wait(&mut (*serialState).cond as *mut pthread_cond_t,
                          &mut (*serialState).mutex as *mut pthread_mutex_t);
    }
    if (*serialState).nextJobID == jobID {
        if 0 != (*serialState).params.ldmParams.enableLdm {
            error = 0;
            ZSTD_window_update(&mut (*serialState).ldmState.window as
                                   *mut ZSTD_window_t, src.start, src.size);
            error =
                ZSTD_ldm_generateSequences(&mut (*serialState).ldmState as
                                               *mut ldmState_t,
                                           &mut seqStore as
                                               *mut rawSeqStore_t,
                                           &mut (*serialState).params.ldmParams
                                               as *mut ldmParams_t, src.start,
                                           src.size);
            pthread_mutex_lock(&mut (*serialState).ldmWindowMutex as
                                   *mut pthread_mutex_t);
            (*serialState).ldmWindow = (*serialState).ldmState.window;
            pthread_cond_signal(&mut (*serialState).ldmWindowCond as
                                    *mut pthread_cond_t);
            pthread_mutex_unlock(&mut (*serialState).ldmWindowMutex as
                                     *mut pthread_mutex_t);
        }
        if 0 != (*serialState).params.fParams.checksumFlag &&
               src.size > 0i32 as libc::c_ulong {
            ZSTD_XXH64_update(&mut (*serialState).xxhState as
                                  *mut XXH64_state_t, src.start, src.size);
        }
    }
    (*serialState).nextJobID = (*serialState).nextJobID.wrapping_add(1);
    pthread_cond_broadcast(&mut (*serialState).cond as *mut pthread_cond_t);
    pthread_mutex_unlock(&mut (*serialState).mutex as *mut pthread_mutex_t);
    if seqStore.size > 0i32 as libc::c_ulong {
        let err: size_t =
            ZSTD_referenceExternalSequences(jobCCtx, seqStore.seq,
                                            seqStore.size);
    };
}
unsafe extern "C" fn ZSTD_window_update(mut window: *mut ZSTD_window_t,
                                        mut src: *const libc::c_void,
                                        mut srcSize: size_t) -> U32 {
    let ip: *const BYTE = src as *const BYTE;
    let mut contiguous: U32 = 1i32 as U32;
    if src != (*window).nextSrc as *const libc::c_void {
        let distanceFromBase: size_t =
            (*window).base.offset_to((*window).nextSrc).expect("bad offset_to")
                as libc::c_long as size_t;
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
            (*window).dictBase.offset_to(ip.offset(srcSize as
                                                       isize)).expect("bad offset_to")
                as libc::c_long;
        let lowLimitMax: U32 =
            if highInputIdx > (*window).dictLimit as ptrdiff_t {
                (*window).dictLimit
            } else { highInputIdx as U32 };
        (*window).lowLimit = lowLimitMax
    }
    return contiguous;
}
unsafe extern "C" fn ZSTDMT_initJobCCtxParams(params: ZSTD_CCtx_params)
 -> ZSTD_CCtx_params {
    let mut jobParams: ZSTD_CCtx_params =
        ZSTD_CCtx_params_s{format: ZSTD_f_zstd1,
                           cParams:
                               ZSTD_compressionParameters{windowLog: 0,
                                                          chainLog: 0,
                                                          hashLog: 0,
                                                          searchLog: 0,
                                                          searchLength: 0,
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
                           overlapSizeLog: 0,
                           ldmParams:
                               ldmParams_t{enableLdm: 0,
                                           hashLog: 0,
                                           bucketSizeLog: 0,
                                           minMatchLength: 0,
                                           hashEveryLog: 0,
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
unsafe extern "C" fn ZSTDMT_computeOverlapLog(params: ZSTD_CCtx_params)
 -> size_t {
    let overlapRLog: libc::c_uint =
        if params.overlapSizeLog > 9i32 as libc::c_uint {
            0i32 as libc::c_uint
        } else { (9i32 as libc::c_uint).wrapping_sub(params.overlapSizeLog) };
    if 0 != params.ldmParams.enableLdm {
        return if (params.cParams.windowLog as libc::c_ulong) <
                      ZSTDMT_computeTargetJobLog(params).wrapping_sub(2i32 as
                                                                          libc::c_ulong)
                  {
                   params.cParams.windowLog as libc::c_ulong
               } else {
                   ZSTDMT_computeTargetJobLog(params).wrapping_sub(2i32 as
                                                                       libc::c_ulong)
               }.wrapping_sub(overlapRLog as libc::c_ulong)
    } else {
        return (if overlapRLog >= 9i32 as libc::c_uint {
                    0i32 as libc::c_uint
                } else { params.cParams.windowLog.wrapping_sub(overlapRLog) })
                   as size_t
    };
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
        (*mtctx).jobs =
            ZSTDMT_createJobsTable(&mut nbJobs as *mut U32, (*mtctx).cMem);
        if (*mtctx).jobs.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        } else {
            (*mtctx).jobIDMask = nbJobs.wrapping_sub(1i32 as libc::c_uint)
        }
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
        ZSTD_ldm_adjustParameters(&mut params.ldmParams as *mut ldmParams_t,
                                  &mut params.cParams as
                                      *mut ZSTD_compressionParameters);
        (*serialState).ldmState.hashPower =
            ZSTD_ldm_getHashPower(params.ldmParams.minMatchLength)
    } else {
        memset(&mut params.ldmParams as *mut ldmParams_t as *mut libc::c_void,
               0i32, ::std::mem::size_of::<ldmParams_t>() as libc::c_ulong);
    }
    (*serialState).nextJobID = 0i32 as libc::c_uint;
    if 0 != params.fParams.checksumFlag {
        ZSTD_XXH64_reset(&mut (*serialState).xxhState as *mut XXH64_state_t,
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
        ZSTD_window_clear(&mut (*serialState).ldmState.window as
                              *mut ZSTD_window_t);
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
        } else {
            memset((*serialState).ldmState.hashTable as *mut libc::c_void,
                   0i32, hashSize);
            memset((*serialState).ldmState.bucketOffsets as *mut libc::c_void,
                   0i32, bucketSize);
        }
    }
    (*serialState).params = params;
    (*serialState).params.jobSize = jobSize as U32;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_initCStream(mut mtctx: *mut ZSTDMT_CCtx,
                                            mut compressionLevel: libc::c_int)
 -> size_t {
    let params: ZSTD_parameters =
        ZSTD_getParams(compressionLevel,
                       0u64.wrapping_sub(1i32 as libc::c_ulonglong),
                       0i32 as size_t);
    let mut cctxParams: ZSTD_CCtx_params = (*mtctx).params;
    cctxParams.cParams = params.cParams;
    cctxParams.fParams = params.fParams;
    return ZSTDMT_initCStream_internal(mtctx, 0 as *const libc::c_void,
                                       0i32 as size_t, ZSTD_dct_auto,
                                       0 as *const ZSTD_CDict, cctxParams,
                                       0u64.wrapping_sub(1i32 as
                                                             libc::c_ulonglong));
}
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
        let errcod: size_t = ZSTDMT_resize(mtctx, params.nbWorkers);
        if 0 != ERR_isError(errcod) { return errcod }
    }
    if params.jobSize > 0i32 as libc::c_uint && params.jobSize < 1u32 << 20i32
       {
        params.jobSize = 1u32 << 20i32
    }
    if params.jobSize >
           if 0 != MEM_32bits() {
               (512i32 * (1i32 << 20i32)) as libc::c_uint
           } else { (2i32 as libc::c_uint).wrapping_mul(1u32 << 30i32) } {
        params.jobSize =
            if 0 != MEM_32bits() {
                (512i32 * (1i32 << 20i32)) as libc::c_uint
            } else { (2i32 as libc::c_uint).wrapping_mul(1u32 << 30i32) }
    }
    (*mtctx).singleBlockingThread =
        (pledgedSrcSize <= (1u32 << 20i32) as libc::c_ulonglong) as
            libc::c_int as libc::c_uint;
    if 0 != (*mtctx).singleBlockingThread {
        let singleThreadParams: ZSTD_CCtx_params =
            ZSTDMT_initJobCCtxParams(params);
        return ZSTD_initCStream_internal((*(*mtctx).cctxPool).cctx[0usize],
                                         dict, dictSize, cdict,
                                         singleThreadParams, pledgedSrcSize)
    } else {
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
                return -(ZSTD_error_memory_allocation as libc::c_int) as
                           size_t
            }
        } else {
            ZSTD_freeCDict((*mtctx).cdictLocal);
            (*mtctx).cdictLocal = 0 as *mut ZSTD_CDict;
            (*mtctx).cdict = cdict
        }
        (*mtctx).targetPrefixSize =
            (1i32 as size_t) << ZSTDMT_computeOverlapLog(params);
        (*mtctx).targetSectionSize = params.jobSize as size_t;
        if (*mtctx).targetSectionSize == 0i32 as libc::c_ulong {
            (*mtctx).targetSectionSize =
                (1u64 << ZSTDMT_computeTargetJobLog(params)) as size_t
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
            (if (*mtctx).params.nbWorkers > 1i32 as libc::c_uint {
                 (*mtctx).params.nbWorkers
             } else { 1i32 as libc::c_uint }) as size_t;
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
                return -(ZSTD_error_memory_allocation as libc::c_int) as
                           size_t
            } else { (*mtctx).roundBuff.capacity = capacity }
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
               ZSTDMT_serialState_reset(&mut (*mtctx).serial as
                                            *mut serialState_t,
                                        (*mtctx).seqPool, params,
                                        (*mtctx).targetSectionSize) {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        } else { return 0i32 as size_t }
    };
}
static mut kNullRange: range_t =
    unsafe {
        range_t{start: 0 as *const libc::c_void, size: 0i32 as size_t,}
    };
unsafe extern "C" fn ZSTDMT_waitForAllJobsCompleted(mut mtctx:
                                                        *mut ZSTDMT_CCtx)
 -> () {
    while (*mtctx).doneJobID < (*mtctx).nextJobID {
        let jobID: libc::c_uint = (*mtctx).doneJobID & (*mtctx).jobIDMask;
        pthread_mutex_lock(&mut (*(*mtctx).jobs.offset(jobID as
                                                           isize)).job_mutex
                               as *mut pthread_mutex_t);
        while (*(*mtctx).jobs.offset(jobID as isize)).consumed <
                  (*(*mtctx).jobs.offset(jobID as isize)).src.size {
            pthread_cond_wait(&mut (*(*mtctx).jobs.offset(jobID as
                                                              isize)).job_cond
                                  as *mut pthread_cond_t,
                              &mut (*(*mtctx).jobs.offset(jobID as
                                                              isize)).job_mutex
                                  as *mut pthread_mutex_t);
        }
        pthread_mutex_unlock(&mut (*(*mtctx).jobs.offset(jobID as
                                                             isize)).job_mutex
                                 as *mut pthread_mutex_t);
        (*mtctx).doneJobID = (*mtctx).doneJobID.wrapping_add(1)
    };
}
unsafe extern "C" fn ZSTDMT_resize(mut mtctx: *mut ZSTDMT_CCtx,
                                   mut nbWorkers: libc::c_uint) -> size_t {
    if 0 != POOL_resize((*mtctx).factory, nbWorkers as size_t) {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    } else {
        let errcod: size_t = ZSTDMT_expandJobsTable(mtctx, nbWorkers);
        if 0 != ERR_isError(errcod) {
            return errcod
        } else {
            (*mtctx).bufPool =
                ZSTDMT_expandBufferPool((*mtctx).bufPool, nbWorkers);
            if (*mtctx).bufPool.is_null() {
                return -(ZSTD_error_memory_allocation as libc::c_int) as
                           size_t
            } else {
                (*mtctx).cctxPool =
                    ZSTDMT_expandCCtxPool((*mtctx).cctxPool, nbWorkers);
                if (*mtctx).cctxPool.is_null() {
                    return -(ZSTD_error_memory_allocation as libc::c_int) as
                               size_t
                } else {
                    (*mtctx).seqPool =
                        ZSTDMT_expandSeqPool((*mtctx).seqPool, nbWorkers);
                    if (*mtctx).seqPool.is_null() {
                        return -(ZSTD_error_memory_allocation as libc::c_int)
                                   as size_t
                    } else {
                        ZSTDMT_CCtxParam_setNbWorkers(&mut (*mtctx).params as
                                                          *mut ZSTD_CCtx_params,
                                                      nbWorkers);
                        return 0i32 as size_t
                    }
                }
            }
        }
    };
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
    if srcBufPool.is_null() {
        return 0 as *mut ZSTDMT_bufferPool
    } else if (*srcBufPool).totalBuffers >= maxNbBuffers {
        return srcBufPool
    } else {
        let cMem: ZSTD_customMem = (*srcBufPool).cMem;
        let bSize: size_t = (*srcBufPool).bufferSize;
        let mut newBufPool: *mut ZSTDMT_bufferPool =
            0 as *mut ZSTDMT_bufferPool;
        ZSTDMT_freeBufferPool(srcBufPool);
        newBufPool = ZSTDMT_createBufferPool(nbWorkers, cMem);
        if newBufPool.is_null() {
            return newBufPool
        } else { ZSTDMT_setBufferSize(newBufPool, bSize); return newBufPool }
    };
}
unsafe extern "C" fn ZSTDMT_expandCCtxPool(mut srcPool: *mut ZSTDMT_CCtxPool,
                                           mut nbWorkers: libc::c_uint)
 -> *mut ZSTDMT_CCtxPool {
    if srcPool.is_null() {
        return 0 as *mut ZSTDMT_CCtxPool
    } else if nbWorkers <= (*srcPool).totalCCtx {
        return srcPool
    } else {
        let cMem: ZSTD_customMem = (*srcPool).cMem;
        ZSTDMT_freeCCtxPool(srcPool);
        return ZSTDMT_createCCtxPool(nbWorkers, cMem)
    };
}
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
pub unsafe extern "C" fn ZSTDMT_compressStream(mut mtctx: *mut ZSTDMT_CCtx,
                                               mut output:
                                                   *mut ZSTD_outBuffer,
                                               mut input: *mut ZSTD_inBuffer)
 -> size_t {
    let errcod: size_t =
        ZSTDMT_compressStream_generic(mtctx, output, input, ZSTD_e_continue);
    if 0 != ERR_isError(errcod) {
        return errcod
    } else {
        return (*mtctx).targetSectionSize.wrapping_sub((*mtctx).inBuff.filled)
    };
}
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
    } else if 0 != (*mtctx).frameEnded &&
                  endOp as libc::c_uint ==
                      ZSTD_e_continue as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    } else if (*mtctx).nextJobID == 0i32 as libc::c_uint &&
                  (*mtctx).inBuff.filled == 0i32 as libc::c_ulong &&
                  0 == (*mtctx).jobReady &&
                  endOp as libc::c_uint ==
                      ZSTD_e_end as libc::c_int as libc::c_uint &&
                  (*output).size.wrapping_sub((*output).pos) >=
                      ZSTD_compressBound((*input).size.wrapping_sub((*input).pos))
     {
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
        if 0 != ZSTD_isError(cSize) {
            return cSize
        } else {
            (*input).pos = (*input).size;
            (*output).pos =
                ((*output).pos as libc::c_ulong).wrapping_add(cSize) as size_t
                    as size_t;
            (*mtctx).allJobsCompleted = 1i32 as libc::c_uint;
            (*mtctx).frameEnded = 1i32 as libc::c_uint;
            return 0i32 as size_t
        }
    } else {
        if 0 == (*mtctx).jobReady && (*input).size > (*input).pos {
            if (*mtctx).inBuff.buffer.start.is_null() {
                0 == ZSTDMT_tryGetInputRange(mtctx);
            }
            if !(*mtctx).inBuff.buffer.start.is_null() {
                let toLoad: size_t =
                    if (*input).size.wrapping_sub((*input).pos) <
                           (*mtctx).targetSectionSize.wrapping_sub((*mtctx).inBuff.filled)
                       {
                        (*input).size.wrapping_sub((*input).pos)
                    } else {
                        (*mtctx).targetSectionSize.wrapping_sub((*mtctx).inBuff.filled)
                    };
                memcpy(((*mtctx).inBuff.buffer.start as
                            *mut libc::c_char).offset((*mtctx).inBuff.filled
                                                          as isize) as
                           *mut libc::c_void,
                       ((*input).src as
                            *const libc::c_char).offset((*input).pos as isize)
                           as *const libc::c_void, toLoad);
                (*input).pos =
                    ((*input).pos as libc::c_ulong).wrapping_add(toLoad) as
                        size_t as size_t;
                (*mtctx).inBuff.filled =
                    ((*mtctx).inBuff.filled as
                         libc::c_ulong).wrapping_add(toLoad) as size_t as
                        size_t;
                forwardInputProgress =
                    (toLoad > 0i32 as libc::c_ulong) as libc::c_int as
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
               endOp as libc::c_uint ==
                   ZSTD_e_end as libc::c_int as libc::c_uint &&
                   0 == (*mtctx).frameEnded {
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
        } else { return remainingToFlush }
    };
}
unsafe extern "C" fn ZSTDMT_flushProduced(mut mtctx: *mut ZSTDMT_CCtx,
                                          mut output: *mut ZSTD_outBuffer,
                                          mut blockToFlush: libc::c_uint,
                                          mut end: ZSTD_EndDirective)
 -> size_t {
    let wJobID: libc::c_uint = (*mtctx).doneJobID & (*mtctx).jobIDMask;
    pthread_mutex_lock(&mut (*(*mtctx).jobs.offset(wJobID as isize)).job_mutex
                           as *mut pthread_mutex_t);
    if 0 != blockToFlush && (*mtctx).doneJobID < (*mtctx).nextJobID {
        while (*(*mtctx).jobs.offset(wJobID as isize)).dstFlushed ==
                  (*(*mtctx).jobs.offset(wJobID as isize)).cSize {
            if (*(*mtctx).jobs.offset(wJobID as isize)).consumed ==
                   (*(*mtctx).jobs.offset(wJobID as isize)).src.size {
                break ;
            }
            pthread_cond_wait(&mut (*(*mtctx).jobs.offset(wJobID as
                                                              isize)).job_cond
                                  as *mut pthread_cond_t,
                              &mut (*(*mtctx).jobs.offset(wJobID as
                                                              isize)).job_mutex
                                  as *mut pthread_mutex_t);
        }
    }
    let mut cSize: size_t = (*(*mtctx).jobs.offset(wJobID as isize)).cSize;
    let srcConsumed: size_t =
        (*(*mtctx).jobs.offset(wJobID as isize)).consumed;
    let srcSize: size_t = (*(*mtctx).jobs.offset(wJobID as isize)).src.size;
    pthread_mutex_unlock(&mut (*(*mtctx).jobs.offset(wJobID as
                                                         isize)).job_mutex as
                             *mut pthread_mutex_t);
    if 0 != ZSTD_isError(cSize) {
        ZSTDMT_waitForAllJobsCompleted(mtctx);
        ZSTDMT_releaseAllJobResources(mtctx);
        return cSize
    } else {
        if srcConsumed == srcSize &&
               0 !=
                   (*(*mtctx).jobs.offset(wJobID as
                                              isize)).frameChecksumNeeded {
            let checksum: U32 =
                ZSTD_XXH64_digest(&mut (*mtctx).serial.xxhState as
                                      *mut XXH64_state_t) as U32;
            MEM_writeLE32(((*(*mtctx).jobs.offset(wJobID as
                                                      isize)).dstBuff.start as
                               *mut libc::c_char).offset((*(*mtctx).jobs.offset(wJobID
                                                                                    as
                                                                                    isize)).cSize
                                                             as isize) as
                              *mut libc::c_void, checksum);
            cSize =
                (cSize as libc::c_ulong).wrapping_add(4i32 as libc::c_ulong)
                    as size_t as size_t;
            let ref mut fresh9 =
                (*(*mtctx).jobs.offset(wJobID as isize)).cSize;
            *fresh9 =
                (*fresh9 as libc::c_ulong).wrapping_add(4i32 as libc::c_ulong)
                    as size_t as size_t;
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
                ((*output).pos as libc::c_ulong).wrapping_add(toFlush) as
                    size_t as size_t;
            let ref mut fresh10 =
                (*(*mtctx).jobs.offset(wJobID as isize)).dstFlushed;
            *fresh10 =
                (*fresh10 as libc::c_ulong).wrapping_add(toFlush) as size_t as
                    size_t;
            if srcConsumed == srcSize &&
                   (*(*mtctx).jobs.offset(wJobID as isize)).dstFlushed ==
                       cSize {
                ZSTDMT_releaseBuffer((*mtctx).bufPool,
                                     (*(*mtctx).jobs.offset(wJobID as
                                                                isize)).dstBuff);
                (*(*mtctx).jobs.offset(wJobID as isize)).dstBuff =
                    g_nullBuffer;
                (*(*mtctx).jobs.offset(wJobID as isize)).cSize =
                    0i32 as size_t;
                (*mtctx).consumed =
                    (*mtctx).consumed.wrapping_add(srcSize as
                                                       libc::c_ulonglong);
                (*mtctx).produced =
                    (*mtctx).produced.wrapping_add(cSize as
                                                       libc::c_ulonglong);
                (*mtctx).doneJobID = (*mtctx).doneJobID.wrapping_add(1)
            }
        }
        if cSize > (*(*mtctx).jobs.offset(wJobID as isize)).dstFlushed {
            return cSize.wrapping_sub((*(*mtctx).jobs.offset(wJobID as
                                                                 isize)).dstFlushed)
        } else if srcSize > srcConsumed {
            return 1i32 as size_t
        } else if (*mtctx).doneJobID < (*mtctx).nextJobID {
            return 1i32 as size_t
        } else if 0 != (*mtctx).jobReady {
            return 1i32 as size_t
        } else if (*mtctx).inBuff.filled > 0i32 as libc::c_ulong {
            return 1i32 as size_t
        } else {
            (*mtctx).allJobsCompleted = (*mtctx).frameEnded;
            if end as libc::c_uint ==
                   ZSTD_e_end as libc::c_int as libc::c_uint {
                return (0 == (*mtctx).frameEnded) as libc::c_int as size_t
            } else { return 0i32 as size_t }
        }
    };
}
unsafe extern "C" fn ZSTDMT_createCompressionJob(mut mtctx: *mut ZSTDMT_CCtx,
                                                 mut srcSize: size_t,
                                                 mut endOp: ZSTD_EndDirective)
 -> size_t {
    let mut newPrefixSize: size_t = 0;
    let jobID: libc::c_uint = (*mtctx).nextJobID & (*mtctx).jobIDMask;
    let endFrame: libc::c_int =
        (endOp as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint)
            as libc::c_int;
    if (*mtctx).nextJobID >
           (*mtctx).doneJobID.wrapping_add((*mtctx).jobIDMask) {
        return 0i32 as size_t
    } else {
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
            let ref mut fresh12 =
                (*(*mtctx).jobs.offset(jobID as isize)).cdict;
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
            let ref mut fresh14 =
                (*(*mtctx).jobs.offset(jobID as isize)).bufPool;
            *fresh14 = (*mtctx).bufPool;
            let ref mut fresh15 =
                (*(*mtctx).jobs.offset(jobID as isize)).seqPool;
            *fresh15 = (*mtctx).seqPool;
            let ref mut fresh16 =
                (*(*mtctx).jobs.offset(jobID as isize)).serial;
            *fresh16 = &mut (*mtctx).serial as *mut serialState_t;
            (*(*mtctx).jobs.offset(jobID as isize)).jobID =
                (*mtctx).nextJobID;
            (*(*mtctx).jobs.offset(jobID as isize)).firstJob =
                ((*mtctx).nextJobID == 0i32 as libc::c_uint) as libc::c_int as
                    libc::c_uint;
            (*(*mtctx).jobs.offset(jobID as isize)).lastJob =
                endFrame as libc::c_uint;
            (*(*mtctx).jobs.offset(jobID as isize)).frameChecksumNeeded =
                (0 != endFrame && (*mtctx).nextJobID > 0i32 as libc::c_uint &&
                     0 != (*mtctx).params.fParams.checksumFlag) as libc::c_int
                    as libc::c_uint;
            (*(*mtctx).jobs.offset(jobID as isize)).dstFlushed =
                0i32 as size_t;
            (*mtctx).roundBuff.pos =
                ((*mtctx).roundBuff.pos as
                     libc::c_ulong).wrapping_add(srcSize) as size_t as size_t;
            (*mtctx).inBuff.buffer = g_nullBuffer;
            (*mtctx).inBuff.filled = 0i32 as size_t;
            if 0 == endFrame {
                newPrefixSize =
                    if srcSize < (*mtctx).targetPrefixSize {
                        srcSize
                    } else { (*mtctx).targetPrefixSize };
                (*mtctx).inBuff.prefix.start =
                    src.offset(srcSize as
                                   isize).offset(-(newPrefixSize as isize)) as
                        *const libc::c_void;
                (*mtctx).inBuff.prefix.size = newPrefixSize
            } else {
                (*mtctx).inBuff.prefix = kNullRange;
                (*mtctx).frameEnded = endFrame as libc::c_uint;
                if (*mtctx).nextJobID == 0i32 as libc::c_uint {
                    (*mtctx).params.fParams.checksumFlag =
                        0i32 as libc::c_uint
                }
            }
            if srcSize == 0i32 as libc::c_ulong &&
                   (*mtctx).nextJobID > 0i32 as libc::c_uint {
                ZSTDMT_writeLastEmptyBlock((*mtctx).jobs.offset(jobID as
                                                                    isize));
                (*mtctx).nextJobID = (*mtctx).nextJobID.wrapping_add(1);
                return 0i32 as size_t
            }
        }
        if 0 !=
               POOL_tryAdd((*mtctx).factory, Some(ZSTDMT_compressionJob),
                           &mut *(*mtctx).jobs.offset(jobID as isize) as
                               *mut ZSTDMT_jobDescription as
                               *mut libc::c_void) {
            (*mtctx).nextJobID = (*mtctx).nextJobID.wrapping_add(1);
            (*mtctx).jobReady = 0i32
        } else { (*mtctx).jobReady = 1i32 }
        return 0i32 as size_t
    };
}
unsafe extern "C" fn ZSTDMT_writeLastEmptyBlock(mut job:
                                                    *mut ZSTDMT_jobDescription)
 -> () {
    (*job).dstBuff = ZSTDMT_getBuffer((*job).bufPool);
    if (*job).dstBuff.start.is_null() {
        (*job).cSize =
            -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
        return
    } else {
        (*job).src = kNullRange;
        (*job).cSize =
            ZSTD_writeLastEmptyBlock((*job).dstBuff.start,
                                     (*job).dstBuff.capacity);
        return;
    };
}
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
        if 0 != ZSTDMT_isOverlapped(buffer, inUse) {
            return 0i32
        } else {
            ZSTDMT_waitForLdmComplete(mtctx, buffer);
            memmove(start as *mut libc::c_void, (*mtctx).inBuff.prefix.start,
                    prefixSize);
            (*mtctx).inBuff.prefix.start = start as *const libc::c_void;
            (*mtctx).roundBuff.pos = prefixSize
        }
    }
    buffer.start =
        (*mtctx).roundBuff.buffer.offset((*mtctx).roundBuff.pos as isize) as
            *mut libc::c_void;
    buffer.capacity = target;
    if 0 != ZSTDMT_isOverlapped(buffer, inUse) {
        return 0i32
    } else {
        ZSTDMT_waitForLdmComplete(mtctx, buffer);
        (*mtctx).inBuff.buffer = buffer;
        (*mtctx).inBuff.filled = 0i32 as size_t;
        return 1i32
    };
}
unsafe extern "C" fn ZSTDMT_waitForLdmComplete(mut mtctx: *mut ZSTDMT_CCtx,
                                               mut buffer: buffer_t) -> () {
    if 0 != (*mtctx).params.ldmParams.enableLdm {
        let mut mutex: *mut pthread_mutex_t =
            &mut (*mtctx).serial.ldmWindowMutex as *mut pthread_mutex_t;
        pthread_mutex_lock(mutex);
        while 0 != ZSTDMT_doesOverlapWindow(buffer, (*mtctx).serial.ldmWindow)
              {
            pthread_cond_wait(&mut (*mtctx).serial.ldmWindowCond as
                                  *mut pthread_cond_t, mutex);
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
        window.base.offset(window.dictLimit as
                               isize).offset_to(window.nextSrc).expect("bad offset_to")
            as libc::c_long as size_t;
    return (0 != ZSTDMT_isOverlapped(buffer, extDict) ||
                0 != ZSTDMT_isOverlapped(buffer, prefix)) as libc::c_int;
}
unsafe extern "C" fn ZSTDMT_isOverlapped(mut buffer: buffer_t,
                                         mut range: range_t) -> libc::c_int {
    let bufferStart: *const BYTE = buffer.start as *const BYTE;
    let bufferEnd: *const BYTE = bufferStart.offset(buffer.capacity as isize);
    let rangeStart: *const BYTE = range.start as *const BYTE;
    let rangeEnd: *const BYTE = rangeStart.offset(range.size as isize);
    if rangeStart.is_null() || bufferStart.is_null() {
        return 0i32
    } else if bufferStart == bufferEnd || rangeStart == rangeEnd {
        return 0i32
    } else {
        return (bufferStart < rangeEnd && rangeStart < bufferEnd) as
                   libc::c_int
    };
}
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
                                                           isize)).job_mutex
                               as *mut pthread_mutex_t);
        consumed = (*(*mtctx).jobs.offset(wJobID as isize)).consumed;
        pthread_mutex_unlock(&mut (*(*mtctx).jobs.offset(wJobID as
                                                             isize)).job_mutex
                                 as *mut pthread_mutex_t);
        if consumed < (*(*mtctx).jobs.offset(wJobID as isize)).src.size {
            let mut range: range_t =
                (*(*mtctx).jobs.offset(wJobID as isize)).prefix;
            if range.size == 0i32 as libc::c_ulong {
                range = (*(*mtctx).jobs.offset(wJobID as isize)).src
            }
            return range
        } else { jobID = jobID.wrapping_add(1) }
    }
    return kNullRange;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_flushStream(mut mtctx: *mut ZSTDMT_CCtx,
                                            mut output: *mut ZSTD_outBuffer)
 -> size_t {
    if 0 != (*mtctx).singleBlockingThread {
        return ZSTD_flushStream((*(*mtctx).cctxPool).cctx[0usize], output)
    } else {
        return ZSTDMT_flushStream_internal(mtctx, output, ZSTD_e_flush)
    };
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
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_endStream(mut mtctx: *mut ZSTDMT_CCtx,
                                          mut output: *mut ZSTD_outBuffer)
 -> size_t {
    if 0 != (*mtctx).singleBlockingThread {
        return ZSTD_endStream((*(*mtctx).cctxPool).cctx[0usize], output)
    } else { return ZSTDMT_flushStream_internal(mtctx, output, ZSTD_e_end) };
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
    let mut cctxParams: ZSTD_CCtx_params = (*mtctx).params;
    cctxParams.cParams = params.cParams;
    cctxParams.fParams = params.fParams;
    return ZSTDMT_initCStream_internal(mtctx, dict, dictSize, ZSTD_dct_auto,
                                       0 as *const ZSTD_CDict, cctxParams,
                                       pledgedSrcSize);
}
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
    } else {
        cctxParams.cParams = ZSTD_getCParamsFromCDict(cdict);
        cctxParams.fParams = fParams;
        return ZSTDMT_initCStream_internal(mtctx, 0 as *const libc::c_void,
                                           0i32 as size_t, ZSTD_dct_auto,
                                           cdict, cctxParams, pledgedSrcSize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_setMTCtxParameter(mut mtctx: *mut ZSTDMT_CCtx,
                                                  mut parameter:
                                                      ZSTDMT_parameter,
                                                  mut value: libc::c_uint)
 -> size_t {
    match parameter as libc::c_uint {
        0 => {
            return ZSTDMT_CCtxParam_setMTCtxParameter(&mut (*mtctx).params as
                                                          *mut ZSTD_CCtx_params,
                                                      parameter, value)
        }
        1 => {
            return ZSTDMT_CCtxParam_setMTCtxParameter(&mut (*mtctx).params as
                                                          *mut ZSTD_CCtx_params,
                                                      parameter, value)
        }
        _ => {
            return -(ZSTD_error_parameter_unsupported as libc::c_int) as
                       size_t
        }
    };
}
/* ! ZSTDMT_compressStream_generic() :
 *  Combines ZSTDMT_compressStream() with optional ZSTDMT_flushStream() or ZSTDMT_endStream()
 *  depending on flush directive.
 * @return : minimum amount of data still to be flushed
 *           0 if fully flushed
 *           or an error code
 *  note : needs to be init using any ZSTD_initCStream*() variant */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_CCtxParam_setMTCtxParameter(mut params:
                                                                *mut ZSTD_CCtx_params,
                                                            mut parameter:
                                                                ZSTDMT_parameter,
                                                            mut value:
                                                                libc::c_uint)
 -> size_t {
    match parameter as libc::c_uint {
        0 => {
            if 0 !=
                   (value > 0i32 as libc::c_uint) as libc::c_int &
                       (value < 1u32 << 20i32) as libc::c_int {
                value = 1u32 << 20i32
            }
            if value >
                   if 0 != MEM_32bits() {
                       (512i32 * (1i32 << 20i32)) as libc::c_uint
                   } else {
                       (2i32 as libc::c_uint).wrapping_mul(1u32 << 30i32)
                   } {
                value =
                    if 0 != MEM_32bits() {
                        (512i32 * (1i32 << 20i32)) as libc::c_uint
                    } else {
                        (2i32 as libc::c_uint).wrapping_mul(1u32 << 30i32)
                    }
            }
            (*params).jobSize = value;
            return value as size_t
        }
        1 => {
            if value > 9i32 as libc::c_uint { value = 9i32 as libc::c_uint }
            (*params).overlapSizeLog =
                if value >= 9i32 as libc::c_uint {
                    9i32 as libc::c_uint
                } else { value };
            return value as size_t
        }
        _ => {
            return -(ZSTD_error_parameter_unsupported as libc::c_int) as
                       size_t
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_getMTCtxParameter(mut mtctx: *mut ZSTDMT_CCtx,
                                                  mut parameter:
                                                      ZSTDMT_parameter,
                                                  mut value:
                                                      *mut libc::c_uint)
 -> size_t {
    match parameter as libc::c_uint {
        0 => { *value = (*mtctx).params.jobSize }
        1 => { *value = (*mtctx).params.overlapSizeLog }
        _ => {
            return -(ZSTD_error_parameter_unsupported as libc::c_int) as
                       size_t
        }
    }
    return 0i32 as size_t;
}
/* ! ZSTDMT_updateCParams_whileCompressing() :
 *  Updates only a selected set of compression parameters, to remain compatible with current frame.
 *  New parameters will be applied to next compression job. */
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_updateCParams_whileCompressing(mut mtctx:
                                                                   *mut ZSTDMT_CCtx,
                                                               mut cctxParams:
                                                                   *const ZSTD_CCtx_params)
 -> () {
    let saved_wlog: U32 = (*mtctx).params.cParams.windowLog;
    let compressionLevel: libc::c_int = (*cctxParams).compressionLevel;
    (*mtctx).params.compressionLevel = compressionLevel;
    let mut cParams: ZSTD_compressionParameters =
        ZSTD_getCParamsFromCCtxParams(cctxParams, 0i32 as U64,
                                      0i32 as size_t);
    cParams.windowLog = saved_wlog;
    (*mtctx).params.cParams = cParams;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDMT_getFrameProgression(mut mtctx:
                                                        *mut ZSTDMT_CCtx)
 -> ZSTD_frameProgression {
    let mut fps: ZSTD_frameProgression =
        ZSTD_frameProgression{ingested: 0, consumed: 0, produced: 0,};
    fps.consumed = (*mtctx).consumed;
    fps.produced = (*mtctx).produced;
    fps.ingested =
        (*mtctx).consumed.wrapping_add((*mtctx).inBuff.filled as
                                           libc::c_ulonglong);
    let mut jobNb: libc::c_uint = 0;
    let mut lastJobNb: libc::c_uint =
        (*mtctx).nextJobID.wrapping_add((*mtctx).jobReady as libc::c_uint);
    jobNb = (*mtctx).doneJobID;
    while jobNb < lastJobNb {
        let wJobID: libc::c_uint = jobNb & (*mtctx).jobIDMask;
        pthread_mutex_lock(&mut (*(*mtctx).jobs.offset(wJobID as
                                                           isize)).job_mutex
                               as *mut pthread_mutex_t);
        let cResult: size_t = (*(*mtctx).jobs.offset(wJobID as isize)).cSize;
        let produced: size_t =
            if 0 != ZSTD_isError(cResult) {
                0i32 as libc::c_ulong
            } else { cResult };
        fps.consumed =
            fps.consumed.wrapping_add((*(*mtctx).jobs.offset(wJobID as
                                                                 isize)).consumed
                                          as libc::c_ulonglong);
        fps.ingested =
            fps.ingested.wrapping_add((*(*mtctx).jobs.offset(wJobID as
                                                                 isize)).src.size
                                          as libc::c_ulonglong);
        fps.produced =
            fps.produced.wrapping_add(produced as libc::c_ulonglong);
        pthread_mutex_unlock(&mut (*(*mtctx).jobs.offset(wJobID as
                                                             isize)).job_mutex
                                 as *mut pthread_mutex_t);
        jobNb = jobNb.wrapping_add(1)
    }
    return fps;
}
unsafe extern "C" fn ZSTD_LLcode(mut litLength: U32) -> U32 {
    static mut LL_Code: [BYTE; 64] =
        unsafe {
            [0i32 as BYTE, 1i32 as BYTE, 2i32 as BYTE, 3i32 as BYTE,
             4i32 as BYTE, 5i32 as BYTE, 6i32 as BYTE, 7i32 as BYTE,
             8i32 as BYTE, 9i32 as BYTE, 10i32 as BYTE, 11i32 as BYTE,
             12i32 as BYTE, 13i32 as BYTE, 14i32 as BYTE, 15i32 as BYTE,
             16i32 as BYTE, 16i32 as BYTE, 17i32 as BYTE, 17i32 as BYTE,
             18i32 as BYTE, 18i32 as BYTE, 19i32 as BYTE, 19i32 as BYTE,
             20i32 as BYTE, 20i32 as BYTE, 20i32 as BYTE, 20i32 as BYTE,
             21i32 as BYTE, 21i32 as BYTE, 21i32 as BYTE, 21i32 as BYTE,
             22i32 as BYTE, 22i32 as BYTE, 22i32 as BYTE, 22i32 as BYTE,
             22i32 as BYTE, 22i32 as BYTE, 22i32 as BYTE, 22i32 as BYTE,
             23i32 as BYTE, 23i32 as BYTE, 23i32 as BYTE, 23i32 as BYTE,
             23i32 as BYTE, 23i32 as BYTE, 23i32 as BYTE, 23i32 as BYTE,
             24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE,
             24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE,
             24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE,
             24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE]
        };
    static mut LL_deltaCode: U32 = unsafe { 19i32 as U32 };
    return if litLength > 63i32 as libc::c_uint {
               ZSTD_highbit32(litLength).wrapping_add(LL_deltaCode)
           } else { LL_Code[litLength as usize] as libc::c_uint };
}
unsafe extern "C" fn ZSTD_MLcode(mut mlBase: U32) -> U32 {
    static mut ML_Code: [BYTE; 128] =
        unsafe {
            [0i32 as BYTE, 1i32 as BYTE, 2i32 as BYTE, 3i32 as BYTE,
             4i32 as BYTE, 5i32 as BYTE, 6i32 as BYTE, 7i32 as BYTE,
             8i32 as BYTE, 9i32 as BYTE, 10i32 as BYTE, 11i32 as BYTE,
             12i32 as BYTE, 13i32 as BYTE, 14i32 as BYTE, 15i32 as BYTE,
             16i32 as BYTE, 17i32 as BYTE, 18i32 as BYTE, 19i32 as BYTE,
             20i32 as BYTE, 21i32 as BYTE, 22i32 as BYTE, 23i32 as BYTE,
             24i32 as BYTE, 25i32 as BYTE, 26i32 as BYTE, 27i32 as BYTE,
             28i32 as BYTE, 29i32 as BYTE, 30i32 as BYTE, 31i32 as BYTE,
             32i32 as BYTE, 32i32 as BYTE, 33i32 as BYTE, 33i32 as BYTE,
             34i32 as BYTE, 34i32 as BYTE, 35i32 as BYTE, 35i32 as BYTE,
             36i32 as BYTE, 36i32 as BYTE, 36i32 as BYTE, 36i32 as BYTE,
             37i32 as BYTE, 37i32 as BYTE, 37i32 as BYTE, 37i32 as BYTE,
             38i32 as BYTE, 38i32 as BYTE, 38i32 as BYTE, 38i32 as BYTE,
             38i32 as BYTE, 38i32 as BYTE, 38i32 as BYTE, 38i32 as BYTE,
             39i32 as BYTE, 39i32 as BYTE, 39i32 as BYTE, 39i32 as BYTE,
             39i32 as BYTE, 39i32 as BYTE, 39i32 as BYTE, 39i32 as BYTE,
             40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
             40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
             40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
             40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
             41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
             41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
             41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
             41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
             42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE]
        };
    static mut ML_deltaCode: U32 = unsafe { 36i32 as U32 };
    return if mlBase > 127i32 as libc::c_uint {
               ZSTD_highbit32(mlBase).wrapping_add(ML_deltaCode)
           } else { ML_Code[mlBase as usize] as libc::c_uint };
}
unsafe extern "C" fn ZSTD_storeSeq(mut seqStorePtr: *mut seqStore_t,
                                   mut litLength: size_t,
                                   mut literals: *const libc::c_void,
                                   mut offsetCode: U32, mut mlBase: size_t)
 -> () {
    ZSTD_wildcopy((*seqStorePtr).lit as *mut libc::c_void, literals,
                  litLength as ptrdiff_t);
    (*seqStorePtr).lit = (*seqStorePtr).lit.offset(litLength as isize);
    if litLength > 65535i32 as libc::c_ulong {
        (*seqStorePtr).longLengthID = 1i32 as U32;
        (*seqStorePtr).longLengthPos =
            (*seqStorePtr).sequencesStart.offset_to((*seqStorePtr).sequences).expect("bad offset_to")
                as libc::c_long as U32
    }
    (*(*seqStorePtr).sequences.offset(0isize)).litLength = litLength as U16;
    (*(*seqStorePtr).sequences.offset(0isize)).offset =
        offsetCode.wrapping_add(1i32 as libc::c_uint);
    if mlBase > 65535i32 as libc::c_ulong {
        (*seqStorePtr).longLengthID = 2i32 as U32;
        (*seqStorePtr).longLengthPos =
            (*seqStorePtr).sequencesStart.offset_to((*seqStorePtr).sequences).expect("bad offset_to")
                as libc::c_long as U32
    }
    (*(*seqStorePtr).sequences.offset(0isize)).matchLength = mlBase as U16;
    (*seqStorePtr).sequences = (*seqStorePtr).sequences.offset(1isize);
}
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
        if 0 != diff {
            return ZSTD_NbCommonBytes(diff) as size_t
        } else {
            pIn =
                pIn.offset(::std::mem::size_of::<size_t>() as libc::c_ulong as
                               isize);
            pMatch =
                pMatch.offset(::std::mem::size_of::<size_t>() as libc::c_ulong
                                  as isize);
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
                    return pStart.offset_to(pIn).expect("bad offset_to") as
                               libc::c_long as size_t
                }
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
    return pStart.offset_to(pIn).expect("bad offset_to") as libc::c_long as
               size_t;
}
unsafe extern "C" fn ZSTD_count_2segments(mut ip: *const BYTE,
                                          mut match_0: *const BYTE,
                                          mut iEnd: *const BYTE,
                                          mut mEnd: *const BYTE,
                                          mut iStart: *const BYTE) -> size_t {
    let vEnd: *const BYTE =
        if ip.offset(match_0.offset_to(mEnd).expect("bad offset_to") as
                         libc::c_long as isize) < iEnd {
            ip.offset(match_0.offset_to(mEnd).expect("bad offset_to") as
                          libc::c_long as isize)
        } else { iEnd };
    let matchLength: size_t = ZSTD_count(ip, match_0, vEnd);
    if match_0.offset(matchLength as isize) != mEnd {
        return matchLength
    } else {
        return matchLength.wrapping_add(ZSTD_count(ip.offset(matchLength as
                                                                 isize),
                                                   iStart, iEnd))
    };
}
static mut prime3bytes: U32 = unsafe { 506832829u32 };
unsafe extern "C" fn ZSTD_hash3(mut u: U32, mut h: U32) -> U32 {
    return (u << 32i32 - 24i32).wrapping_mul(prime3bytes) >>
               (32i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash3Ptr(mut ptr: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash3(MEM_readLE32(ptr), h) as size_t;
}
static mut prime4bytes: U32 = unsafe { 2654435761u32 };
unsafe extern "C" fn ZSTD_hash4(mut u: U32, mut h: U32) -> U32 {
    return u.wrapping_mul(prime4bytes) >>
               (32i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash4Ptr(mut ptr: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash4(MEM_read32(ptr), h) as size_t;
}
static mut prime5bytes: U64 = unsafe { 889523592379u64 as U64 };
unsafe extern "C" fn ZSTD_hash5(mut u: U64, mut h: U32) -> size_t {
    return (u << 64i32 - 40i32).wrapping_mul(prime5bytes) >>
               (64i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash5Ptr(mut p: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash5(MEM_readLE64(p), h);
}
static mut prime6bytes: U64 = unsafe { 227718039650203u64 as U64 };
unsafe extern "C" fn ZSTD_hash6(mut u: U64, mut h: U32) -> size_t {
    return (u << 64i32 - 48i32).wrapping_mul(prime6bytes) >>
               (64i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash6Ptr(mut p: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash6(MEM_readLE64(p), h);
}
static mut prime7bytes: U64 = unsafe { 58295818150454627u64 as U64 };
unsafe extern "C" fn ZSTD_hash7(mut u: U64, mut h: U32) -> size_t {
    return (u << 64i32 - 56i32).wrapping_mul(prime7bytes) >>
               (64i32 as libc::c_uint).wrapping_sub(h);
}
unsafe extern "C" fn ZSTD_hash7Ptr(mut p: *const libc::c_void, mut h: U32)
 -> size_t {
    return ZSTD_hash7(MEM_readLE64(p), h);
}
static mut prime8bytes: U64 = unsafe { 14923729446382167139u64 as U64 };
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
unsafe extern "C" fn ZSTD_window_hasExtDict(window: ZSTD_window_t) -> U32 {
    return (window.lowLimit < window.dictLimit) as libc::c_int as U32;
}
unsafe extern "C" fn ZSTD_matchState_dictMode(mut ms:
                                                  *const ZSTD_matchState_t)
 -> ZSTD_dictMode_e {
    return (if 0 != ZSTD_window_hasExtDict((*ms).window) {
                ZSTD_extDict as libc::c_int
            } else if !(*ms).dictMatchState.is_null() {
                ZSTD_dictMatchState as libc::c_int
            } else { ZSTD_noDict as libc::c_int }) as ZSTD_dictMode_e;
}
unsafe extern "C" fn ZSTD_window_needOverflowCorrection(window: ZSTD_window_t,
                                                        mut srcEnd:
                                                            *const libc::c_void)
 -> U32 {
    let current: U32 =
        window.base.offset_to(srcEnd as *const BYTE).expect("bad offset_to")
            as libc::c_long as U32;
    return (current >
                (3u32 <<
                     29i32).wrapping_add(1u32 <<
                                             (if ::std::mem::size_of::<size_t>()
                                                     as libc::c_ulong ==
                                                     4i32 as libc::c_ulong {
                                                  30i32
                                              } else { 31i32 }) as
                                                 libc::c_uint)) as libc::c_int
               as U32;
}
unsafe extern "C" fn ZSTD_window_correctOverflow(mut window:
                                                     *mut ZSTD_window_t,
                                                 mut cycleLog: U32,
                                                 mut maxDist: U32,
                                                 mut src: *const libc::c_void)
 -> U32 {
    let cycleMask: U32 =
        (1u32 << cycleLog).wrapping_sub(1i32 as libc::c_uint);
    let current: U32 =
        (*window).base.offset_to(src as *const BYTE).expect("bad offset_to")
            as libc::c_long as U32;
    let newCurrent: U32 = (current & cycleMask).wrapping_add(maxDist);
    let correction: U32 = current.wrapping_sub(newCurrent);
    (*window).base = (*window).base.offset(correction as isize);
    (*window).dictBase = (*window).dictBase.offset(correction as isize);
    (*window).lowLimit =
        ((*window).lowLimit as libc::c_uint).wrapping_sub(correction) as U32
            as U32;
    (*window).dictLimit =
        ((*window).dictLimit as libc::c_uint).wrapping_sub(correction) as U32
            as U32;
    return correction;
}
unsafe extern "C" fn ZSTD_window_enforceMaxDist(mut window:
                                                    *mut ZSTD_window_t,
                                                mut srcEnd:
                                                    *const libc::c_void,
                                                mut maxDist: U32,
                                                mut loadedDictEndPtr:
                                                    *mut U32,
                                                mut dictMatchStatePtr:
                                                    *mut *const ZSTD_matchState_t)
 -> () {
    let current: U32 =
        (*window).base.offset_to(srcEnd as
                                     *const BYTE).expect("bad offset_to") as
            libc::c_long as U32;
    let mut loadedDictEnd: U32 =
        if !loadedDictEndPtr.is_null() {
            *loadedDictEndPtr
        } else { 0i32 as libc::c_uint };
    if current > maxDist.wrapping_add(loadedDictEnd) {
        let newLowLimit: U32 = current.wrapping_sub(maxDist);
        if (*window).lowLimit < newLowLimit {
            (*window).lowLimit = newLowLimit
        }
        if (*window).dictLimit < (*window).lowLimit {
            (*window).dictLimit = (*window).lowLimit
        }
        if !loadedDictEndPtr.is_null() { *loadedDictEndPtr = 0i32 as U32 }
        if !dictMatchStatePtr.is_null() {
            *dictMatchStatePtr = 0 as *const ZSTD_matchState_t
        }
    };
}
unsafe extern "C" fn ZSTD_fWeight(mut rawStat: U32) -> libc::c_double {
    let fp_accuracy: U32 = 8i32 as U32;
    let fp_multiplier: U32 = (1i32 << fp_accuracy) as U32;
    let stat: U32 = rawStat.wrapping_add(1i32 as libc::c_uint);
    let hb: U32 = ZSTD_highbit32(stat);
    let BWeight: U32 = hb.wrapping_mul(fp_multiplier);
    let FWeight: U32 = stat << fp_accuracy >> hb;
    let weight: U32 = BWeight.wrapping_add(FWeight);
    return weight as libc::c_double / fp_multiplier as libc::c_double;
}
unsafe extern "C" fn ZSTD_debugTable(mut table: *const U32, mut max: U32)
 -> () {
    let mut u: libc::c_uint = 0;
    let mut sum: libc::c_uint = 0;
    u = 0i32 as libc::c_uint;
    sum = 0i32 as libc::c_uint;
    while u <= max {
        sum = sum.wrapping_add(*table.offset(u as isize));
        u = u.wrapping_add(1)
    }
    u = 0i32 as libc::c_uint;
    while u <= max { u = u.wrapping_add(1) };
}
