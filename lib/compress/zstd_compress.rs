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
    pub type ZSTDMT_CCtx_s;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* * HIST_countFast_wksp() :
 *  Same as HIST_countFast(), but using an externally provided scratch buffer.
 * `workSpace` is a writable buffer which must be 4-bytes aligned,
 * `workSpaceSize` must be >= HIST_WKSP_SIZE
 */
    #[no_mangle]
    fn HIST_countFast_wksp(count: *mut libc::c_uint,
                           maxSymbolValuePtr: *mut libc::c_uint,
                           src: *const libc::c_void, srcSize: size_t,
                           workSpace: *mut libc::c_void,
                           workSpaceSize: size_t) -> size_t;
    /*-*****************************************
*  FSE detailed API
******************************************/
/*!
FSE_compress() does the following:
1. count symbol occurrence from source[] into table count[] (see hist.h)
2. normalize counters so that sum(count[]) == Power_of_2 (2^tableLog)
3. save normalized counters to memory buffer using writeNCount()
4. build encoding table 'CTable' from normalized counters
5. encode the data stream using encoding table 'CTable'

FSE_decompress() does the following:
1. read normalized counters with readNCount()
2. build decoding table 'DTable' from normalized counters
3. decode the data stream using decoding table 'DTable'

The following API allows targeting specific sub-functions for advanced tasks.
For example, it's possible to compress several blocks using the same 'CTable',
or to save and provide normalized distribution using external method.
*/
    /* *** COMPRESSION *** */
    /* ! FSE_optimalTableLog():
    dynamically downsize 'tableLog' when conditions are met.
    It saves CPU time, by using smaller tables, while preserving or even improving compression ratio.
    @return : recommended tableLog (necessarily <= 'maxTableLog') */
    #[no_mangle]
    fn FSE_optimalTableLog(maxTableLog: libc::c_uint, srcSize: size_t,
                           maxSymbolValue: libc::c_uint) -> libc::c_uint;
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
    /* *< build a fake FSE_CTable, designed for a flat distribution, where each symbol uses nbBits */
    #[no_mangle]
    fn FSE_buildCTable_rle(ct: *mut FSE_CTable, symbolValue: libc::c_uchar)
     -> size_t;
    /* *< build a fake FSE_CTable, designed to compress always the same symbolValue */
    /* FSE_buildCTable_wksp() :
 * Same as FSE_buildCTable(), but using an externally allocated scratch buffer (`workSpace`).
 * `wkspSize` must be >= `(1<<tableLog)`.
 */
    #[no_mangle]
    fn FSE_buildCTable_wksp(ct: *mut FSE_CTable,
                            normalizedCounter: *const libc::c_short,
                            maxSymbolValue: libc::c_uint,
                            tableLog: libc::c_uint,
                            workSpace: *mut libc::c_void, wkspSize: size_t)
     -> size_t;
    /* * HUF_compress4X_repeat() :
 *  Same as HUF_compress4X_wksp(), but considers using hufTable if *repeat != HUF_repeat_none.
 *  If it uses hufTable it does not modify hufTable or repeat.
 *  If it doesn't, it sets *repeat = HUF_repeat_none, and it sets hufTable to the table used.
 *  If preferRepeat then the old table will always be used if valid. */
    #[no_mangle]
    fn HUF_compress4X_repeat(dst: *mut libc::c_void, dstSize: size_t,
                             src: *const libc::c_void, srcSize: size_t,
                             maxSymbolValue: libc::c_uint,
                             tableLog: libc::c_uint,
                             workSpace: *mut libc::c_void, wkspSize: size_t,
                             hufTable: *mut HUF_CElt, repeat: *mut HUF_repeat,
                             preferRepeat: libc::c_int, bmi2: libc::c_int)
     -> size_t;
    /* * HUF_readCTable() :
 *  Loading a CTable saved with HUF_writeCTable() */
    #[no_mangle]
    fn HUF_readCTable(CTable: *mut HUF_CElt,
                      maxSymbolValuePtr: *mut libc::c_uint,
                      src: *const libc::c_void, srcSize: size_t) -> size_t;
    /* * HUF_compress1X_repeat() :
 *  Same as HUF_compress1X_wksp(), but considers using hufTable if *repeat != HUF_repeat_none.
 *  If it uses hufTable it does not modify hufTable or repeat.
 *  If it doesn't, it sets *repeat = HUF_repeat_none, and it sets hufTable to the table used.
 *  If preferRepeat then the old table will always be used if valid. */
    #[no_mangle]
    fn HUF_compress1X_repeat(dst: *mut libc::c_void, dstSize: size_t,
                             src: *const libc::c_void, srcSize: size_t,
                             maxSymbolValue: libc::c_uint,
                             tableLog: libc::c_uint,
                             workSpace: *mut libc::c_void, wkspSize: size_t,
                             hufTable: *mut HUF_CElt, repeat: *mut HUF_repeat,
                             preferRepeat: libc::c_int, bmi2: libc::c_int)
     -> size_t;
    #[no_mangle]
    fn ZSTDMT_freeCCtx(mtctx: *mut ZSTDMT_CCtx) -> size_t;
    #[no_mangle]
    fn ZSTD_free(ptr: *mut libc::c_void, customMem: ZSTD_customMem);
    #[no_mangle]
    fn ZSTD_compressBlock_btultra_dictMatchState(ms: *mut ZSTD_matchState_t,
                                                 seqStore: *mut seqStore_t,
                                                 rep: *mut U32,
                                                 src: *const libc::c_void,
                                                 srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btopt_dictMatchState(ms: *mut ZSTD_matchState_t,
                                               seqStore: *mut seqStore_t,
                                               rep: *mut U32,
                                               src: *const libc::c_void,
                                               srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btlazy2_dictMatchState(ms: *mut ZSTD_matchState_t,
                                                 seqStore: *mut seqStore_t,
                                                 rep: *mut U32,
                                                 src: *const libc::c_void,
                                                 srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_lazy2_dictMatchState(ms: *mut ZSTD_matchState_t,
                                               seqStore: *mut seqStore_t,
                                               rep: *mut U32,
                                               src: *const libc::c_void,
                                               srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_lazy_dictMatchState(ms: *mut ZSTD_matchState_t,
                                              seqStore: *mut seqStore_t,
                                              rep: *mut U32,
                                              src: *const libc::c_void,
                                              srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_greedy_dictMatchState(ms: *mut ZSTD_matchState_t,
                                                seqStore: *mut seqStore_t,
                                                rep: *mut U32,
                                                src: *const libc::c_void,
                                                srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_doubleFast_dictMatchState(ms:
                                                        *mut ZSTD_matchState_t,
                                                    seqStore: *mut seqStore_t,
                                                    rep: *mut U32,
                                                    src: *const libc::c_void,
                                                    srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_fast_dictMatchState(ms: *mut ZSTD_matchState_t,
                                              seqStore: *mut seqStore_t,
                                              rep: *mut U32,
                                              src: *const libc::c_void,
                                              srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btultra_extDict(ms: *mut ZSTD_matchState_t,
                                          seqStore: *mut seqStore_t,
                                          rep: *mut U32,
                                          src: *const libc::c_void,
                                          srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btopt_extDict(ms: *mut ZSTD_matchState_t,
                                        seqStore: *mut seqStore_t,
                                        rep: *mut U32,
                                        src: *const libc::c_void,
                                        srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btlazy2_extDict(ms: *mut ZSTD_matchState_t,
                                          seqStore: *mut seqStore_t,
                                          rep: *mut U32,
                                          src: *const libc::c_void,
                                          srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_lazy2_extDict(ms: *mut ZSTD_matchState_t,
                                        seqStore: *mut seqStore_t,
                                        rep: *mut U32,
                                        src: *const libc::c_void,
                                        srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_lazy_extDict(ms: *mut ZSTD_matchState_t,
                                       seqStore: *mut seqStore_t,
                                       rep: *mut U32,
                                       src: *const libc::c_void,
                                       srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_greedy_extDict(ms: *mut ZSTD_matchState_t,
                                         seqStore: *mut seqStore_t,
                                         rep: *mut U32,
                                         src: *const libc::c_void,
                                         srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_doubleFast_extDict(ms: *mut ZSTD_matchState_t,
                                             seqStore: *mut seqStore_t,
                                             rep: *mut U32,
                                             src: *const libc::c_void,
                                             srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_fast_extDict(ms: *mut ZSTD_matchState_t,
                                       seqStore: *mut seqStore_t,
                                       rep: *mut U32,
                                       src: *const libc::c_void,
                                       srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btultra2(ms: *mut ZSTD_matchState_t,
                                   seqStore: *mut seqStore_t, rep: *mut U32,
                                   src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btultra(ms: *mut ZSTD_matchState_t,
                                  seqStore: *mut seqStore_t, rep: *mut U32,
                                  src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btopt(ms: *mut ZSTD_matchState_t,
                                seqStore: *mut seqStore_t, rep: *mut U32,
                                src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btlazy2(ms: *mut ZSTD_matchState_t,
                                  seqStore: *mut seqStore_t, rep: *mut U32,
                                  src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_lazy2(ms: *mut ZSTD_matchState_t,
                                seqStore: *mut seqStore_t, rep: *mut U32,
                                src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_lazy(ms: *mut ZSTD_matchState_t,
                               seqStore: *mut seqStore_t, rep: *mut U32,
                               src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_greedy(ms: *mut ZSTD_matchState_t,
                                 seqStore: *mut seqStore_t, rep: *mut U32,
                                 src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_doubleFast(ms: *mut ZSTD_matchState_t,
                                     seqStore: *mut seqStore_t, rep: *mut U32,
                                     src: *const libc::c_void,
                                     srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_fast(ms: *mut ZSTD_matchState_t,
                               seqStore: *mut seqStore_t, rep: *mut U32,
                               src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    /* *
 * ZSTD_ldm_blockCompress():
 *
 * Compresses a block using the predefined sequences, along with a secondary
 * block compressor. The literals section of every sequence is passed to the
 * secondary block compressor, and those sequences are interspersed with the
 * predefined sequences. Returns the length of the last literals.
 * Updates `rawSeqStore.pos` to indicate how many sequences have been consumed.
 * `rawSeqStore.seq` may also be updated to split the last sequence between two
 * blocks.
 * @return The length of the last literals.
 *
 * NOTE: The source must be at most the maximum block size, but the predefined
 * sequences can be any size, and may be longer than the block. In the case that
 * they are longer than the block, the last sequences may need to be split into
 * two. We handle that case correctly, and update `rawSeqStore` appropriately.
 * NOTE: This function does not return any errors.
 */
    #[no_mangle]
    fn ZSTD_ldm_blockCompress(rawSeqStore: *mut rawSeqStore_t,
                              ms: *mut ZSTD_matchState_t,
                              seqStore: *mut seqStore_t, rep: *mut U32,
                              src: *const libc::c_void, srcSize: size_t)
     -> size_t;
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
    /* *
 * ZSTD_ldm_skipSequences():
 *
 * Skip past `srcSize` bytes worth of sequences in `rawSeqStore`.
 * Avoids emitting matches less than `minMatch` bytes.
 * Must be called for data with is not passed to ZSTD_ldm_blockCompress().
 */
    #[no_mangle]
    fn ZSTD_ldm_skipSequences(rawSeqStore: *mut rawSeqStore_t,
                              srcSize: size_t, minMatch: U32);
    #[no_mangle]
    fn ZSTD_XXH64_update(statePtr: *mut XXH64_state_t,
                         input: *const libc::c_void, length: size_t)
     -> XXH_errorcode;
    #[no_mangle]
    fn ZSTD_XXH64_digest(statePtr: *const XXH64_state_t) -> XXH64_hash_t;
    /*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
    /* used in ZSTD_loadDictionaryContent() */
    #[no_mangle]
    fn ZSTD_updateTree(ms: *mut ZSTD_matchState_t, ip: *const BYTE,
                       iend: *const BYTE);
    /*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
    #[no_mangle]
    fn ZSTD_insertAndFindFirstIndex(ms: *mut ZSTD_matchState_t,
                                    ip: *const BYTE) -> U32;
    /*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
    /* U32 */
    /* ZSTD_CCtx, size_t */
    #[no_mangle]
    fn ZSTD_fillDoubleHashTable(ms: *mut ZSTD_matchState_t,
                                end: *const libc::c_void,
                                dtlm: ZSTD_dictTableLoadMethod_e);
    /*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
    /* U32 */
    #[no_mangle]
    fn ZSTD_fillHashTable(ms: *mut ZSTD_matchState_t,
                          end: *const libc::c_void,
                          dtlm: ZSTD_dictTableLoadMethod_e);
    /* * ZSTD_ldm_getSeqSpace() :
 *  Return an upper bound on the number of sequences that can be produced by
 *  the long distance matcher, or 0 if LDM is disabled.
 */
    #[no_mangle]
    fn ZSTD_ldm_getMaxNbSeq(params: ldmParams_t, maxChunkSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_XXH64_reset(statePtr: *mut XXH64_state_t, seed: libc::c_ulonglong)
     -> XXH_errorcode;
    /* * ZSTD_ldm_getTableSize() :
 *  Estimate the space needed for long distance matching tables or 0 if LDM is
 *  disabled.
 */
    #[no_mangle]
    fn ZSTD_ldm_getTableSize(params: ldmParams_t) -> size_t;
    /* custom memory allocation functions */
    #[no_mangle]
    fn ZSTD_malloc(size: size_t, customMem: ZSTD_customMem)
     -> *mut libc::c_void;
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
    #[no_mangle]
    fn ZSTDMT_nextInputSizeHint(mtctx: *const ZSTDMT_CCtx) -> size_t;
    /* ! ZSTDMT_compressStream_generic() :
 *  Combines ZSTDMT_compressStream() with optional ZSTDMT_flushStream() or ZSTDMT_endStream()
 *  depending on flush directive.
 * @return : minimum amount of data still to be flushed
 *           0 if fully flushed
 *           or an error code
 *  note : needs to be init using any ZSTD_initCStream*() variant */
    #[no_mangle]
    fn ZSTDMT_compressStream_generic(mtctx: *mut ZSTDMT_CCtx,
                                     output: *mut ZSTD_outBuffer,
                                     input: *mut ZSTD_inBuffer,
                                     endOp: ZSTD_EndDirective) -> size_t;
    /* ! ZSTDMT_updateCParams_whileCompressing() :
 *  Updates only a selected set of compression parameters, to remain compatible with current frame.
 *  New parameters will be applied to next compression job. */
    #[no_mangle]
    fn ZSTDMT_updateCParams_whileCompressing(mtctx: *mut ZSTDMT_CCtx,
                                             cctxParams:
                                                 *const ZSTD_CCtx_params);
    /* ! ZSTDMT_initCStream_internal() :
 *  Private use only. Init streaming operation.
 *  expects params to be valid.
 *  must receive dict, or cdict, or none, but not both.
 *  @return : 0, or an error code */
    #[no_mangle]
    fn ZSTDMT_initCStream_internal(zcs: *mut ZSTDMT_CCtx,
                                   dict: *const libc::c_void,
                                   dictSize: size_t,
                                   dictContentType: ZSTD_dictContentType_e,
                                   cdict: *const ZSTD_CDict,
                                   params: ZSTD_CCtx_params,
                                   pledgedSrcSize: libc::c_ulonglong)
     -> size_t;
    #[no_mangle]
    fn ZSTDMT_createCCtx_advanced(nbWorkers: libc::c_uint,
                                  cMem: ZSTD_customMem) -> *mut ZSTDMT_CCtx;
    #[no_mangle]
    fn ZSTDMT_sizeof_CCtx(mtctx: *mut ZSTDMT_CCtx) -> size_t;
    /* ! ZSTDMT_CCtxParam_setMTCtxParameter()
 *  like ZSTDMT_setMTCtxParameter(), but into a ZSTD_CCtx_Params */
    #[no_mangle]
    fn ZSTDMT_CCtxParam_setMTCtxParameter(params: *mut ZSTD_CCtx_params,
                                          parameter: ZSTDMT_parameter,
                                          value: libc::c_int) -> size_t;
    /* ! ZSTDMT_CCtxParam_setNbWorkers()
 *  Set nbWorkers, and clamp it.
 *  Also reset jobSize and overlapLog */
    #[no_mangle]
    fn ZSTDMT_CCtxParam_setNbWorkers(params: *mut ZSTD_CCtx_params,
                                     nbWorkers: libc::c_uint) -> size_t;
    #[no_mangle]
    fn ZSTD_calloc(size: size_t, customMem: ZSTD_customMem)
     -> *mut libc::c_void;
    /* ! ZSTDMT_getFrameProgression():
 *  tells how much data has been consumed (input) and produced (output) for current frame.
 *  able to count progression inside worker threads.
 */
    #[no_mangle]
    fn ZSTDMT_getFrameProgression(mtctx: *mut ZSTDMT_CCtx)
     -> ZSTD_frameProgression;
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
    fn ZSTDMT_toFlushNow(mtctx: *mut ZSTDMT_CCtx) -> size_t;
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
/* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
/* don't allocate that. It's only meant to be more restrictive than void* */
pub type FSE_CTable = libc::c_uint;
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
/* ******************************************************************
   bitstream
   Part of FSE library
   Copyright (C) 2013-present, Yann Collet.

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
   - Source repository : https://github.com/Cyan4973/FiniteStateEntropy
****************************************************************** */
/*
*  This API consists of small unitary functions, which must be inlined for best performance.
*  Since link-time-optimization is not available for all compilers,
*  these functions are defined into a .h to be included.
*/
/*-****************************************
*  Dependencies
******************************************/
/* unaligned access routines */
/*=========================================
*  Target specific
=========================================*/
/*-******************************************
*  bitStream encoding API (write forward)
********************************************/
/* bitStream can mix input from multiple sources.
 * A critical property of these streams is that they encode and decode in **reverse** direction.
 * So the first bit sequence you add will be the last to be read, like a LIFO stack.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BIT_CStream_t {
    pub bitContainer: size_t,
    pub bitPos: libc::c_uint,
    pub startPtr: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub endPtr: *mut libc::c_char,
}
pub type unnamed_1 = libc::c_uint;
pub const MEM_static_assert: unnamed_1 = 1;
/* *< same as FSE_decompress(), using an externally allocated `workSpace` produced with `FSE_DTABLE_SIZE_U32(maxLog)` */
pub type FSE_repeat = libc::c_uint;
/* *< Can use the previous table and it is asumed to be valid */
pub const FSE_repeat_valid: FSE_repeat = 2;
/* *< Can use the previous table but it must be checked */
pub const FSE_repeat_check: FSE_repeat = 1;
/* *< Cannot use the previous table */
pub const FSE_repeat_none: FSE_repeat = 0;
/* *****************************************
*  FSE symbol compression API
*******************************************/
/*!
   This API consists of small unitary functions, which highly benefit from being inlined.
   Hence their body are included in next section.
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const libc::c_void,
    pub symbolTT: *const libc::c_void,
    pub stateLog: libc::c_uint,
}
/* faster, but works only if nbBits is always >= 1 (otherwise, result will be corrupted) */
/* *****************************************
*  Implementation of inlined functions
*******************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: libc::c_int,
    pub deltaNbBits: U32,
}
/* incomplete type */
pub type HUF_CElt = HUF_CElt_s;
pub type HUF_repeat = libc::c_uint;
/* *< Can use the previous table and it is assumed to be valid */
pub const HUF_repeat_valid: HUF_repeat = 2;
/* *< Can use the previous table but it must be checked. Note : The previous table must have been constructed by HUF_compress{1, 4}X_repeat */
pub const HUF_repeat_check: HUF_repeat = 1;
/* *< Cannot use the previous table */
pub const HUF_repeat_none: HUF_repeat = 0;
/* **************************************
*  Explicit context
***************************************/
/*= Compression context
 *  When compressing many times,
 *  it is recommended to allocate a context just once, and re-use it for each successive compression operation.
 *  This will make workload friendlier for system's memory.
 *  Use one context per thread for parallel execution in multi-threaded environments. */
pub type ZSTD_CCtx = ZSTD_CCtx_s;
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
pub type ZSTDMT_CCtx = ZSTDMT_CCtx_s;
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
/* **********************************
 *  Bulk processing dictionary API
 **********************************/
pub type ZSTD_CDict = ZSTD_CDict_s;
/*-*************************************
*  Context memory management
***************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_CDict_s {
    pub dictBuffer: *mut libc::c_void,
    pub dictContent: *const libc::c_void,
    pub dictContentSize: size_t,
    pub workspace: *mut libc::c_void,
    pub workspaceSize: size_t,
    pub matchState: ZSTD_matchState_t,
    pub cBlockState: ZSTD_compressedBlockState_t,
    pub customMem: ZSTD_customMem,
    pub dictID: U32,
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
pub struct ZSTD_compressedBlockState_t {
    pub entropy: ZSTD_entropyCTables_t,
    pub rep: [U32; 3],
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [U32; 256],
    pub repeatMode: HUF_repeat,
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
pub struct ZSTD_window_t {
    pub nextSrc: *const BYTE,
    pub base: *const BYTE,
    pub dictBase: *const BYTE,
    pub dictLimit: U32,
    pub lowLimit: U32,
}
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_parameters {
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
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
pub type ZSTD_defaultPolicy_e = libc::c_uint;
pub const ZSTD_defaultAllowed: ZSTD_defaultPolicy_e = 1;
pub const ZSTD_defaultDisallowed: ZSTD_defaultPolicy_e = 0;
pub type ZSTD_blockCompressor
    =
    Option<unsafe extern "C" fn(_: *mut ZSTD_matchState_t, _: *mut seqStore_t,
                                _: *mut U32, _: *const libc::c_void,
                                _: size_t) -> size_t>;
pub type ZSTD_dictMode_e = libc::c_uint;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
pub const bt_compressed: unnamed_2 = 2;
pub const bt_raw: unnamed_2 = 0;
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
pub type ZSTD_buffered_policy_e = libc::c_uint;
pub const ZSTDb_buffered: ZSTD_buffered_policy_e = 1;
pub const ZSTDb_not_buffered: ZSTD_buffered_policy_e = 0;
pub type ZSTD_dictTableLoadMethod_e = libc::c_uint;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
pub type ZSTD_compResetPolicy_e = libc::c_uint;
pub const ZSTDcrp_noMemset: ZSTD_compResetPolicy_e = 1;
pub const ZSTDcrp_continue: ZSTD_compResetPolicy_e = 0;
pub type ZSTD_ResetDirective = libc::c_uint;
pub const ZSTD_reset_session_and_parameters: ZSTD_ResetDirective = 3;
pub const ZSTD_reset_parameters: ZSTD_ResetDirective = 2;
pub const ZSTD_reset_session_only: ZSTD_ResetDirective = 1;
pub type ZSTD_dictLoadMethod_e = libc::c_uint;
/* *< Reference dictionary content -- the dictionary buffer must outlive its users. */
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
/* *< Copy dictionary content internally */
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_bounds {
    pub error: size_t,
    pub lowerBound: libc::c_int,
    pub upperBound: libc::c_int,
}
/* ZSTDMT_parameter :
 * List of parameters that can be set using ZSTDMT_setMTCtxParameter() */
pub type ZSTDMT_parameter = libc::c_uint;
/* Enables rsyncable mode. */
pub const ZSTDMT_p_rsyncable: ZSTDMT_parameter = 2;
/* Each job may reload a part of previous job to enhance compressionr ratio; 0 == no overlap, 6(default) == use 1/8th of window, >=9 == use full window. This is a "sticky" parameter : its value will be re-used on next compression job */
pub const ZSTDMT_p_overlapLog: ZSTDMT_parameter = 1;
/* Each job is compressed in parallel. By default, this value is dynamically determined depending on compression parameters. Can be set explicitly here. */
pub const ZSTDMT_p_jobSize: ZSTDMT_parameter = 0;
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
pub type unnamed_2 = libc::c_uint;
pub const bt_reserved: unnamed_2 = 3;
pub const bt_rle: unnamed_2 = 1;
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
    let one: unnamed = unnamed{u: 1i32 as U32,};
    return one.c[0usize] as libc::c_uint;
}
unsafe extern "C" fn MEM_read16(mut ptr: *const libc::c_void) -> U16 {
    return (*(ptr as *const unalign16)).v;
}
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> U32 {
    return (*(ptr as *const unalign32)).v;
}
unsafe extern "C" fn MEM_write16(mut memPtr: *mut libc::c_void,
                                 mut value: U16) {
    (*(memPtr as *mut unalign16)).v = value;
}
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void,
                                 mut value: U32) {
    (*(memPtr as *mut unalign32)).v = value;
}
unsafe extern "C" fn MEM_write64(mut memPtr: *mut libc::c_void,
                                 mut value: U64) {
    (*(memPtr as *mut unalign64)).v = value;
}
/* MEM_FORCE_MEMORY_ACCESS */
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
}
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return (in_0 as libc::c_ulonglong).swap_bytes() as U64;
}
unsafe extern "C" fn MEM_writeLE16(mut memPtr: *mut libc::c_void,
                                   mut val: U16) {
    if 0 != MEM_isLittleEndian() {
        MEM_write16(memPtr, val);
    } else {
        let mut p: *mut BYTE = memPtr as *mut BYTE;
        *p.offset(0isize) = val as BYTE;
        *p.offset(1isize) = (val as libc::c_int >> 8i32) as BYTE
    };
}
unsafe extern "C" fn MEM_writeLE24(mut memPtr: *mut libc::c_void,
                                   mut val: U32) {
    MEM_writeLE16(memPtr, val as U16);
    *(memPtr as *mut BYTE).offset(2isize) = (val >> 16i32) as BYTE;
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
unsafe extern "C" fn MEM_writeLE64(mut memPtr: *mut libc::c_void,
                                   mut val64: U64) {
    if 0 != MEM_isLittleEndian() {
        MEM_write64(memPtr, val64);
    } else { MEM_write64(memPtr, MEM_swap64(val64)); };
}
unsafe extern "C" fn MEM_writeLEST(mut memPtr: *mut libc::c_void,
                                   mut val: size_t) {
    if 0 != MEM_32bits() {
        MEM_writeLE32(memPtr, val as U32);
    } else { MEM_writeLE64(memPtr, val); };
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
    }
    return 0i32 as size_t;
}
unsafe extern "C" fn BIT_addBits(mut bitC: *mut BIT_CStream_t,
                                 mut value: size_t,
                                 mut nbBits: libc::c_uint) {
    (*bitC).bitContainer |=
        (value & BIT_mask[nbBits as usize] as libc::c_ulong) <<
            (*bitC).bitPos;
    (*bitC).bitPos = (*bitC).bitPos.wrapping_add(nbBits);
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
unsafe extern "C" fn BIT_flushBits(mut bitC: *mut BIT_CStream_t) {
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
    if (*bitC).ptr >= (*bitC).endPtr { return 0i32 as size_t }
    return ((*bitC).ptr.wrapping_offset_from((*bitC).startPtr) as libc::c_long
                +
                ((*bitC).bitPos > 0i32 as libc::c_uint) as libc::c_int as
                    libc::c_long) as size_t;
}
/* Start by invoking BIT_initDStream().
*  A chunk of the bitStream is then stored into a local register.
*  Local register size is 64-bits on 64-bits systems, 32-bits on 32-bits systems (size_t).
*  You can then retrieve bitFields stored into the local register, **in reverse order**.
*  Local register is explicitly reloaded from memory by the BIT_reloadDStream() method.
*  A reload guarantee a minimum of ((8*sizeof(bitD->bitContainer))-7) bits when its result is BIT_DStream_unfinished.
*  Otherwise, it can be less than that, so proceed accordingly.
*  Checking if DStream has reached its end can be performed with BIT_endOfDStream().
*/
/*-****************************************
*  unsafe API
******************************************/
unsafe extern "C" fn BIT_addBitsFast(mut bitC: *mut BIT_CStream_t,
                                     mut value: size_t,
                                     mut nbBits: libc::c_uint) {
    (*bitC).bitContainer |= value << (*bitC).bitPos;
    (*bitC).bitPos = (*bitC).bitPos.wrapping_add(nbBits);
}
unsafe extern "C" fn FSE_initCState(mut statePtr: *mut FSE_CState_t,
                                    mut ct: *const FSE_CTable) {
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
                                      mut symbol: libc::c_uint) {
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
                                     mut statePtr: *const FSE_CState_t) {
    BIT_addBits(bitC, (*statePtr).value as size_t, (*statePtr).stateLog);
    BIT_flushBits(bitC);
}
/* ! FSE_initCState2() :
*   Same as FSE_initCState(), but the first symbol to include (which will be the last to be read)
*   uses the smallest state value possible, saving the cost of this symbol */
unsafe extern "C" fn FSE_initCState2(mut statePtr: *mut FSE_CState_t,
                                     mut ct: *const FSE_CTable,
                                     mut symbol: U32) {
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
/* FSE_bitCost() :
 * Approximate symbol cost, as fractional value, using fixed-point format (accuracyLog fractional bits)
 * note 1 : assume symbolValue is valid (<= maxSymbolValue)
 * note 2 : if freq[symbolValue]==0, @return a fake cost of tableLog+1 bits */
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
/* **************************************
*  Default constant
***************************************/
/* **************************************
*  Simple API
***************************************/
/*! ZSTD_compress() :
 *  Compresses `src` content as a single zstd compressed frame into already allocated `dst`.
 *  Hint : compression runs faster if `dstCapacity` >=  `ZSTD_compressBound(srcSize)`.
 *  @return : compressed size written into `dst` (<= `dstCapacity),
 *            or an error code if it fails (which can be tested using ZSTD_isError()). */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compress(mut dst: *mut libc::c_void,
                                       mut dstCapacity: size_t,
                                       mut src: *const libc::c_void,
                                       mut srcSize: size_t,
                                       mut compressionLevel: libc::c_int)
 -> size_t {
    let mut result: size_t = 0;
    let mut ctxBody: ZSTD_CCtx =
        ZSTD_CCtx_s{stage: ZSTDcs_created,
                    cParamsChanged: 0,
                    bmi2: 0,
                    requestedParams:
                        ZSTD_CCtx_params_s{format: ZSTD_f_zstd1,
                                           cParams:
                                               ZSTD_compressionParameters{windowLog:
                                                                              0,
                                                                          chainLog:
                                                                              0,
                                                                          hashLog:
                                                                              0,
                                                                          searchLog:
                                                                              0,
                                                                          minMatch:
                                                                              0,
                                                                          targetLength:
                                                                              0,
                                                                          strategy:
                                                                              0
                                                                                  as
                                                                                  ZSTD_strategy,},
                                           fParams:
                                               ZSTD_frameParameters{contentSizeFlag:
                                                                        0,
                                                                    checksumFlag:
                                                                        0,
                                                                    noDictIDFlag:
                                                                        0,},
                                           compressionLevel: 0,
                                           forceWindow: 0,
                                           attachDictPref:
                                               ZSTD_dictDefaultAttach,
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
                                               ZSTD_customMem{customAlloc:
                                                                  None,
                                                              customFree:
                                                                  None,
                                                              opaque:
                                                                  0 as
                                                                      *mut libc::c_void,},},
                    appliedParams:
                        ZSTD_CCtx_params_s{format: ZSTD_f_zstd1,
                                           cParams:
                                               ZSTD_compressionParameters{windowLog:
                                                                              0,
                                                                          chainLog:
                                                                              0,
                                                                          hashLog:
                                                                              0,
                                                                          searchLog:
                                                                              0,
                                                                          minMatch:
                                                                              0,
                                                                          targetLength:
                                                                              0,
                                                                          strategy:
                                                                              0
                                                                                  as
                                                                                  ZSTD_strategy,},
                                           fParams:
                                               ZSTD_frameParameters{contentSizeFlag:
                                                                        0,
                                                                    checksumFlag:
                                                                        0,
                                                                    noDictIDFlag:
                                                                        0,},
                                           compressionLevel: 0,
                                           forceWindow: 0,
                                           attachDictPref:
                                               ZSTD_dictDefaultAttach,
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
                                               ZSTD_customMem{customAlloc:
                                                                  None,
                                                              customFree:
                                                                  None,
                                                              opaque:
                                                                  0 as
                                                                      *mut libc::c_void,},},
                    dictID: 0,
                    workSpaceOversizedDuration: 0,
                    workSpace: 0 as *mut libc::c_void,
                    workSpaceSize: 0,
                    blockSize: 0,
                    pledgedSrcSizePlusOne: 0,
                    consumedSrcSize: 0,
                    producedCSize: 0,
                    xxhState:
                        XXH64_state_s{total_len: 0,
                                      v1: 0,
                                      v2: 0,
                                      v3: 0,
                                      v4: 0,
                                      mem64: [0; 4],
                                      memsize: 0,
                                      reserved: [0; 2],},
                    customMem:
                        ZSTD_customMem{customAlloc: None,
                                       customFree: None,
                                       opaque: 0 as *mut libc::c_void,},
                    staticSize: 0,
                    seqStore:
                        seqStore_t{sequencesStart: 0 as *mut seqDef,
                                   sequences: 0 as *mut seqDef,
                                   litStart: 0 as *mut BYTE,
                                   lit: 0 as *mut BYTE,
                                   llCode: 0 as *mut BYTE,
                                   mlCode: 0 as *mut BYTE,
                                   ofCode: 0 as *mut BYTE,
                                   maxNbSeq: 0,
                                   maxNbLit: 0,
                                   longLengthID: 0,
                                   longLengthPos: 0,},
                    ldmState:
                        ldmState_t{window:
                                       ZSTD_window_t{nextSrc:
                                                         0 as *const BYTE,
                                                     base: 0 as *const BYTE,
                                                     dictBase:
                                                         0 as *const BYTE,
                                                     dictLimit: 0,
                                                     lowLimit: 0,},
                                   hashTable: 0 as *mut ldmEntry_t,
                                   bucketOffsets: 0 as *mut BYTE,
                                   hashPower: 0,},
                    ldmSequences: 0 as *mut rawSeq,
                    maxNbLdmSequences: 0,
                    externSeqStore:
                        rawSeqStore_t{seq: 0 as *mut rawSeq,
                                      pos: 0,
                                      size: 0,
                                      capacity: 0,},
                    blockState:
                        ZSTD_blockState_t{prevCBlock:
                                              0 as
                                                  *mut ZSTD_compressedBlockState_t,
                                          nextCBlock:
                                              0 as
                                                  *mut ZSTD_compressedBlockState_t,
                                          matchState:
                                              ZSTD_matchState_t{window:
                                                                    ZSTD_window_t{nextSrc:
                                                                                      0
                                                                                          as
                                                                                          *const BYTE,
                                                                                  base:
                                                                                      0
                                                                                          as
                                                                                          *const BYTE,
                                                                                  dictBase:
                                                                                      0
                                                                                          as
                                                                                          *const BYTE,
                                                                                  dictLimit:
                                                                                      0,
                                                                                  lowLimit:
                                                                                      0,},
                                                                loadedDictEnd:
                                                                    0,
                                                                nextToUpdate:
                                                                    0,
                                                                nextToUpdate3:
                                                                    0,
                                                                hashLog3: 0,
                                                                hashTable:
                                                                    0 as
                                                                        *mut U32,
                                                                hashTable3:
                                                                    0 as
                                                                        *mut U32,
                                                                chainTable:
                                                                    0 as
                                                                        *mut U32,
                                                                opt:
                                                                    optState_t{litFreq:
                                                                                   0
                                                                                       as
                                                                                       *mut libc::c_uint,
                                                                               litLengthFreq:
                                                                                   0
                                                                                       as
                                                                                       *mut libc::c_uint,
                                                                               matchLengthFreq:
                                                                                   0
                                                                                       as
                                                                                       *mut libc::c_uint,
                                                                               offCodeFreq:
                                                                                   0
                                                                                       as
                                                                                       *mut libc::c_uint,
                                                                               matchTable:
                                                                                   0
                                                                                       as
                                                                                       *mut ZSTD_match_t,
                                                                               priceTable:
                                                                                   0
                                                                                       as
                                                                                       *mut ZSTD_optimal_t,
                                                                               litSum:
                                                                                   0,
                                                                               litLengthSum:
                                                                                   0,
                                                                               matchLengthSum:
                                                                                   0,
                                                                               offCodeSum:
                                                                                   0,
                                                                               litSumBasePrice:
                                                                                   0,
                                                                               litLengthSumBasePrice:
                                                                                   0,
                                                                               matchLengthSumBasePrice:
                                                                                   0,
                                                                               offCodeSumBasePrice:
                                                                                   0,
                                                                               priceType:
                                                                                   zop_dynamic,
                                                                               symbolCosts:
                                                                                   0
                                                                                       as
                                                                                       *const ZSTD_entropyCTables_t,},
                                                                dictMatchState:
                                                                    0 as
                                                                        *const ZSTD_matchState_t,
                                                                cParams:
                                                                    ZSTD_compressionParameters{windowLog:
                                                                                                   0,
                                                                                               chainLog:
                                                                                                   0,
                                                                                               hashLog:
                                                                                                   0,
                                                                                               searchLog:
                                                                                                   0,
                                                                                               minMatch:
                                                                                                   0,
                                                                                               targetLength:
                                                                                                   0,
                                                                                               strategy:
                                                                                                   0
                                                                                                       as
                                                                                                       ZSTD_strategy,},},},
                    entropyWorkspace: 0 as *mut U32,
                    inBuff: 0 as *mut libc::c_char,
                    inBuffSize: 0,
                    inToCompress: 0,
                    inBuffPos: 0,
                    inBuffTarget: 0,
                    outBuff: 0 as *mut libc::c_char,
                    outBuffSize: 0,
                    outBuffContentSize: 0,
                    outBuffFlushedSize: 0,
                    streamStage: zcss_init,
                    frameEnded: 0,
                    cdictLocal: 0 as *mut ZSTD_CDict,
                    cdict: 0 as *const ZSTD_CDict,
                    prefixDict:
                        ZSTD_prefixDict_s{dict: 0 as *const libc::c_void,
                                          dictSize: 0,
                                          dictContentType: ZSTD_dct_auto,},
                    mtctx: 0 as *mut ZSTDMT_CCtx,};
    ZSTD_initCCtx(&mut ctxBody, ZSTD_defaultCMem);
    result =
        ZSTD_compressCCtx(&mut ctxBody, dst, dstCapacity, src, srcSize,
                          compressionLevel);
    ZSTD_freeCCtxContent(&mut ctxBody);
    return result;
}
unsafe extern "C" fn ZSTD_freeCCtxContent(mut cctx: *mut ZSTD_CCtx) {
    ZSTD_free((*cctx).workSpace, (*cctx).customMem);
    (*cctx).workSpace = 0 as *mut libc::c_void;
    ZSTD_freeCDict((*cctx).cdictLocal);
    (*cctx).cdictLocal = 0 as *mut ZSTD_CDict;
    ZSTDMT_freeCCtx((*cctx).mtctx);
    (*cctx).mtctx = 0 as *mut ZSTDMT_CCtx;
}
/* ! ZSTD_freeCDict() :
 *  Function frees memory allocated by ZSTD_createCDict(). */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeCDict(mut cdict: *mut ZSTD_CDict)
 -> size_t {
    if cdict.is_null() { return 0i32 as size_t }
    let cMem: ZSTD_customMem = (*cdict).customMem;
    ZSTD_free((*cdict).workspace, cMem);
    ZSTD_free((*cdict).dictBuffer, cMem);
    ZSTD_free(cdict as *mut libc::c_void, cMem);
    return 0i32 as size_t;
}
/* ! ZSTD_compressCCtx() :
 *  Same as ZSTD_compress(), using an explicit ZSTD_CCtx
 *  The function will compress at requested compression level,
 *  ignoring any other parameter */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressCCtx(mut cctx: *mut ZSTD_CCtx,
                                           mut dst: *mut libc::c_void,
                                           mut dstCapacity: size_t,
                                           mut src: *const libc::c_void,
                                           mut srcSize: size_t,
                                           mut compressionLevel: libc::c_int)
 -> size_t {
    return ZSTD_compress_usingDict(cctx, dst, dstCapacity, src, srcSize,
                                   0 as *const libc::c_void, 0i32 as size_t,
                                   compressionLevel);
}
/* *************************
*  Simple dictionary API
***************************/
/*! ZSTD_compress_usingDict() :
 *  Compression at an explicit compression level using a Dictionary.
 *  A dictionary can be any arbitrary data segment (also called a prefix),
 *  or a buffer with specified information (see dictBuilder/zdict.h).
 *  Note : This function loads the dictionary, resulting in significant startup delay.
 *         It's intended for a dictionary used only once.
 *  Note 2 : When `dict == NULL || dictSize < 8` no dictionary is used. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compress_usingDict(mut cctx: *mut ZSTD_CCtx,
                                                 mut dst: *mut libc::c_void,
                                                 mut dstCapacity: size_t,
                                                 mut src: *const libc::c_void,
                                                 mut srcSize: size_t,
                                                 mut dict:
                                                     *const libc::c_void,
                                                 mut dictSize: size_t,
                                                 mut compressionLevel:
                                                     libc::c_int) -> size_t {
    let params: ZSTD_parameters =
        ZSTD_getParams(compressionLevel,
                       srcSize.wrapping_add((0 == srcSize) as libc::c_int as
                                                libc::c_ulong) as
                           libc::c_ulonglong,
                       if !dict.is_null() {
                           dictSize
                       } else { 0i32 as libc::c_ulong });
    let mut cctxParams: ZSTD_CCtx_params =
        ZSTD_assignParamsToCCtxParams((*cctx).requestedParams, params);
    return ZSTD_compress_advanced_internal(cctx, dst, dstCapacity, src,
                                           srcSize, dict, dictSize,
                                           cctxParams);
}
/* ! ZSTD_getParams() :
 *  same as ZSTD_getCParams(), but @return a full `ZSTD_parameters` object instead of sub-component `ZSTD_compressionParameters`.
 *  All fields of `ZSTD_frameParameters` are set to default : contentSize=1, checksum=0, noDictID=0 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getParams(mut compressionLevel: libc::c_int,
                                        mut srcSizeHint: libc::c_ulonglong,
                                        mut dictSize: size_t)
 -> ZSTD_parameters {
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
    let cParams: ZSTD_compressionParameters =
        ZSTD_getCParams(compressionLevel, srcSizeHint, dictSize);
    memset(&mut params as *mut ZSTD_parameters as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZSTD_parameters>() as libc::c_ulong);
    params.cParams = cParams;
    params.fParams.contentSizeFlag = 1i32;
    return params;
}
/* ! ZSTD_getCParams() :
 * @return ZSTD_compressionParameters structure for a selected compression level and estimated srcSize.
 * `estimatedSrcSize` value is optional, select 0 if not known */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getCParams(mut compressionLevel: libc::c_int,
                                         mut srcSizeHint: libc::c_ulonglong,
                                         mut dictSize: size_t)
 -> ZSTD_compressionParameters {
    let addedSize: size_t =
        (if 0 != srcSizeHint { 0i32 } else { 500i32 }) as size_t;
    /* intentional overflow for srcSizeHint == ZSTD_CONTENTSIZE_UNKNOWN */
    let rSize: U64 =
        (if 0 != srcSizeHint.wrapping_add(dictSize as libc::c_ulonglong) {
             srcSizeHint.wrapping_add(dictSize as
                                          libc::c_ulonglong).wrapping_add(addedSize
                                                                              as
                                                                              libc::c_ulonglong)
         } else { 0u64.wrapping_sub(1i32 as libc::c_ulonglong) }) as U64;
    let tableID: U32 =
        ((rSize <= (256i32 * (1i32 << 10i32)) as libc::c_ulong) as libc::c_int
             +
             (rSize <= (128i32 * (1i32 << 10i32)) as libc::c_ulong) as
                 libc::c_int +
             (rSize <= (16i32 * (1i32 << 10i32)) as libc::c_ulong) as
                 libc::c_int) as U32;
    let mut row: libc::c_int = compressionLevel;
    if compressionLevel == 0i32 { row = 3i32 }
    if compressionLevel < 0i32 { row = 0i32 }
    if compressionLevel > 22i32 { row = 22i32 }
    let mut cp: ZSTD_compressionParameters =
        ZSTD_defaultCParameters[tableID as usize][row as usize];
    if compressionLevel < 0i32 {
        cp.targetLength = -compressionLevel as libc::c_uint
    }
    return ZSTD_adjustCParams_internal(cp, srcSizeHint, dictSize);
}
static mut ZSTD_defaultCParameters: [[ZSTD_compressionParameters; 23]; 4] =
    [[ZSTD_compressionParameters{windowLog: 19i32 as libc::c_uint,
                                 chainLog: 12i32 as libc::c_uint,
                                 hashLog: 13i32 as libc::c_uint,
                                 searchLog: 1i32 as libc::c_uint,
                                 minMatch: 6i32 as libc::c_uint,
                                 targetLength: 1i32 as libc::c_uint,
                                 strategy: ZSTD_fast,},
      ZSTD_compressionParameters{windowLog: 19i32 as libc::c_uint,
                                 chainLog: 13i32 as libc::c_uint,
                                 hashLog: 14i32 as libc::c_uint,
                                 searchLog: 1i32 as libc::c_uint,
                                 minMatch: 7i32 as libc::c_uint,
                                 targetLength: 0i32 as libc::c_uint,
                                 strategy: ZSTD_fast,},
      ZSTD_compressionParameters{windowLog: 20i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 16i32 as libc::c_uint,
                                 searchLog: 1i32 as libc::c_uint,
                                 minMatch: 6i32 as libc::c_uint,
                                 targetLength: 0i32 as libc::c_uint,
                                 strategy: ZSTD_fast,},
      ZSTD_compressionParameters{windowLog: 21i32 as libc::c_uint,
                                 chainLog: 16i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 1i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 1i32 as libc::c_uint,
                                 strategy: ZSTD_dfast,},
      ZSTD_compressionParameters{windowLog: 21i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 18i32 as libc::c_uint,
                                 searchLog: 1i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 1i32 as libc::c_uint,
                                 strategy: ZSTD_dfast,},
      ZSTD_compressionParameters{windowLog: 21i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 2i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 2i32 as libc::c_uint,
                                 strategy: ZSTD_greedy,},
      ZSTD_compressionParameters{windowLog: 21i32 as libc::c_uint,
                                 chainLog: 19i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 3i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 4i32 as libc::c_uint,
                                 strategy: ZSTD_greedy,},
      ZSTD_compressionParameters{windowLog: 21i32 as libc::c_uint,
                                 chainLog: 19i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 3i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 8i32 as libc::c_uint,
                                 strategy: ZSTD_lazy,},
      ZSTD_compressionParameters{windowLog: 21i32 as libc::c_uint,
                                 chainLog: 19i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 3i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 16i32 as libc::c_uint,
                                 strategy: ZSTD_lazy2,},
      ZSTD_compressionParameters{windowLog: 21i32 as libc::c_uint,
                                 chainLog: 19i32 as libc::c_uint,
                                 hashLog: 20i32 as libc::c_uint,
                                 searchLog: 4i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 16i32 as libc::c_uint,
                                 strategy: ZSTD_lazy2,},
      ZSTD_compressionParameters{windowLog: 22i32 as libc::c_uint,
                                 chainLog: 20i32 as libc::c_uint,
                                 hashLog: 21i32 as libc::c_uint,
                                 searchLog: 4i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 16i32 as libc::c_uint,
                                 strategy: ZSTD_lazy2,},
      ZSTD_compressionParameters{windowLog: 22i32 as libc::c_uint,
                                 chainLog: 21i32 as libc::c_uint,
                                 hashLog: 22i32 as libc::c_uint,
                                 searchLog: 4i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 16i32 as libc::c_uint,
                                 strategy: ZSTD_lazy2,},
      ZSTD_compressionParameters{windowLog: 22i32 as libc::c_uint,
                                 chainLog: 21i32 as libc::c_uint,
                                 hashLog: 22i32 as libc::c_uint,
                                 searchLog: 5i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 16i32 as libc::c_uint,
                                 strategy: ZSTD_lazy2,},
      ZSTD_compressionParameters{windowLog: 22i32 as libc::c_uint,
                                 chainLog: 21i32 as libc::c_uint,
                                 hashLog: 22i32 as libc::c_uint,
                                 searchLog: 5i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 32i32 as libc::c_uint,
                                 strategy: ZSTD_btlazy2,},
      ZSTD_compressionParameters{windowLog: 22i32 as libc::c_uint,
                                 chainLog: 22i32 as libc::c_uint,
                                 hashLog: 23i32 as libc::c_uint,
                                 searchLog: 5i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 32i32 as libc::c_uint,
                                 strategy: ZSTD_btlazy2,},
      ZSTD_compressionParameters{windowLog: 22i32 as libc::c_uint,
                                 chainLog: 23i32 as libc::c_uint,
                                 hashLog: 23i32 as libc::c_uint,
                                 searchLog: 6i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 32i32 as libc::c_uint,
                                 strategy: ZSTD_btlazy2,},
      ZSTD_compressionParameters{windowLog: 22i32 as libc::c_uint,
                                 chainLog: 22i32 as libc::c_uint,
                                 hashLog: 22i32 as libc::c_uint,
                                 searchLog: 5i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 48i32 as libc::c_uint,
                                 strategy: ZSTD_btopt,},
      ZSTD_compressionParameters{windowLog: 23i32 as libc::c_uint,
                                 chainLog: 23i32 as libc::c_uint,
                                 hashLog: 22i32 as libc::c_uint,
                                 searchLog: 5i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 64i32 as libc::c_uint,
                                 strategy: ZSTD_btopt,},
      ZSTD_compressionParameters{windowLog: 23i32 as libc::c_uint,
                                 chainLog: 23i32 as libc::c_uint,
                                 hashLog: 22i32 as libc::c_uint,
                                 searchLog: 6i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 64i32 as libc::c_uint,
                                 strategy: ZSTD_btultra,},
      ZSTD_compressionParameters{windowLog: 23i32 as libc::c_uint,
                                 chainLog: 24i32 as libc::c_uint,
                                 hashLog: 22i32 as libc::c_uint,
                                 searchLog: 7i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 256i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 25i32 as libc::c_uint,
                                 chainLog: 25i32 as libc::c_uint,
                                 hashLog: 23i32 as libc::c_uint,
                                 searchLog: 7i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 256i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 26i32 as libc::c_uint,
                                 chainLog: 26i32 as libc::c_uint,
                                 hashLog: 24i32 as libc::c_uint,
                                 searchLog: 7i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 512i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 27i32 as libc::c_uint,
                                 chainLog: 27i32 as libc::c_uint,
                                 hashLog: 25i32 as libc::c_uint,
                                 searchLog: 9i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 999i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,}],
     [ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 12i32 as libc::c_uint,
                                 hashLog: 13i32 as libc::c_uint,
                                 searchLog: 1i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 1i32 as libc::c_uint,
                                 strategy: ZSTD_fast,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 13i32 as libc::c_uint,
                                 hashLog: 14i32 as libc::c_uint,
                                 searchLog: 1i32 as libc::c_uint,
                                 minMatch: 6i32 as libc::c_uint,
                                 targetLength: 0i32 as libc::c_uint,
                                 strategy: ZSTD_fast,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 14i32 as libc::c_uint,
                                 hashLog: 14i32 as libc::c_uint,
                                 searchLog: 1i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 1i32 as libc::c_uint,
                                 strategy: ZSTD_dfast,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 16i32 as libc::c_uint,
                                 hashLog: 16i32 as libc::c_uint,
                                 searchLog: 1i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 1i32 as libc::c_uint,
                                 strategy: ZSTD_dfast,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 16i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 2i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 2i32 as libc::c_uint,
                                 strategy: ZSTD_greedy,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 18i32 as libc::c_uint,
                                 searchLog: 3i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 2i32 as libc::c_uint,
                                 strategy: ZSTD_greedy,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 3i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 4i32 as libc::c_uint,
                                 strategy: ZSTD_lazy,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 4i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 4i32 as libc::c_uint,
                                 strategy: ZSTD_lazy,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 4i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 8i32 as libc::c_uint,
                                 strategy: ZSTD_lazy2,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 5i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 8i32 as libc::c_uint,
                                 strategy: ZSTD_lazy2,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 6i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 8i32 as libc::c_uint,
                                 strategy: ZSTD_lazy2,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 5i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 12i32 as libc::c_uint,
                                 strategy: ZSTD_btlazy2,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 19i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 7i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 12i32 as libc::c_uint,
                                 strategy: ZSTD_btlazy2,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 4i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 16i32 as libc::c_uint,
                                 strategy: ZSTD_btopt,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 4i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 32i32 as libc::c_uint,
                                 strategy: ZSTD_btopt,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 6i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 128i32 as libc::c_uint,
                                 strategy: ZSTD_btopt,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 19i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 6i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 128i32 as libc::c_uint,
                                 strategy: ZSTD_btultra,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 19i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 8i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 256i32 as libc::c_uint,
                                 strategy: ZSTD_btultra,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 19i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 6i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 128i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 19i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 8i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 256i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 19i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 10i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 512i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 19i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 12i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 512i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                 chainLog: 19i32 as libc::c_uint,
                                 hashLog: 19i32 as libc::c_uint,
                                 searchLog: 13i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 999i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,}],
     [ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 12i32 as libc::c_uint,
                                 hashLog: 12i32 as libc::c_uint,
                                 searchLog: 1i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 1i32 as libc::c_uint,
                                 strategy: ZSTD_fast,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 12i32 as libc::c_uint,
                                 hashLog: 13i32 as libc::c_uint,
                                 searchLog: 1i32 as libc::c_uint,
                                 minMatch: 6i32 as libc::c_uint,
                                 targetLength: 0i32 as libc::c_uint,
                                 strategy: ZSTD_fast,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 13i32 as libc::c_uint,
                                 hashLog: 15i32 as libc::c_uint,
                                 searchLog: 1i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 0i32 as libc::c_uint,
                                 strategy: ZSTD_fast,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 16i32 as libc::c_uint,
                                 searchLog: 2i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 1i32 as libc::c_uint,
                                 strategy: ZSTD_dfast,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 17i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 2i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 1i32 as libc::c_uint,
                                 strategy: ZSTD_dfast,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 16i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 3i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 2i32 as libc::c_uint,
                                 strategy: ZSTD_greedy,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 17i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 3i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 4i32 as libc::c_uint,
                                 strategy: ZSTD_lazy,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 17i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 3i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 8i32 as libc::c_uint,
                                 strategy: ZSTD_lazy2,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 17i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 4i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 8i32 as libc::c_uint,
                                 strategy: ZSTD_lazy2,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 17i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 5i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 8i32 as libc::c_uint,
                                 strategy: ZSTD_lazy2,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 17i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 6i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 8i32 as libc::c_uint,
                                 strategy: ZSTD_lazy2,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 17i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 5i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 8i32 as libc::c_uint,
                                 strategy: ZSTD_btlazy2,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 7i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 12i32 as libc::c_uint,
                                 strategy: ZSTD_btlazy2,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 3i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 12i32 as libc::c_uint,
                                 strategy: ZSTD_btopt,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 4i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 32i32 as libc::c_uint,
                                 strategy: ZSTD_btopt,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 6i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 256i32 as libc::c_uint,
                                 strategy: ZSTD_btopt,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 6i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 128i32 as libc::c_uint,
                                 strategy: ZSTD_btultra,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 8i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 256i32 as libc::c_uint,
                                 strategy: ZSTD_btultra,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 10i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 512i32 as libc::c_uint,
                                 strategy: ZSTD_btultra,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 5i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 256i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 7i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 512i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 9i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 512i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                 chainLog: 18i32 as libc::c_uint,
                                 hashLog: 17i32 as libc::c_uint,
                                 searchLog: 11i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 999i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,}],
     [ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 12i32 as libc::c_uint,
                                 hashLog: 13i32 as libc::c_uint,
                                 searchLog: 1i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 1i32 as libc::c_uint,
                                 strategy: ZSTD_fast,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 14i32 as libc::c_uint,
                                 hashLog: 15i32 as libc::c_uint,
                                 searchLog: 1i32 as libc::c_uint,
                                 minMatch: 5i32 as libc::c_uint,
                                 targetLength: 0i32 as libc::c_uint,
                                 strategy: ZSTD_fast,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 14i32 as libc::c_uint,
                                 hashLog: 15i32 as libc::c_uint,
                                 searchLog: 1i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 0i32 as libc::c_uint,
                                 strategy: ZSTD_fast,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 14i32 as libc::c_uint,
                                 hashLog: 15i32 as libc::c_uint,
                                 searchLog: 2i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 1i32 as libc::c_uint,
                                 strategy: ZSTD_dfast,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 14i32 as libc::c_uint,
                                 hashLog: 14i32 as libc::c_uint,
                                 searchLog: 4i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 2i32 as libc::c_uint,
                                 strategy: ZSTD_greedy,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 14i32 as libc::c_uint,
                                 hashLog: 14i32 as libc::c_uint,
                                 searchLog: 3i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 4i32 as libc::c_uint,
                                 strategy: ZSTD_lazy,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 14i32 as libc::c_uint,
                                 hashLog: 14i32 as libc::c_uint,
                                 searchLog: 4i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 8i32 as libc::c_uint,
                                 strategy: ZSTD_lazy2,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 14i32 as libc::c_uint,
                                 hashLog: 14i32 as libc::c_uint,
                                 searchLog: 6i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 8i32 as libc::c_uint,
                                 strategy: ZSTD_lazy2,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 14i32 as libc::c_uint,
                                 hashLog: 14i32 as libc::c_uint,
                                 searchLog: 8i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 8i32 as libc::c_uint,
                                 strategy: ZSTD_lazy2,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 14i32 as libc::c_uint,
                                 searchLog: 5i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 8i32 as libc::c_uint,
                                 strategy: ZSTD_btlazy2,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 14i32 as libc::c_uint,
                                 searchLog: 9i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 8i32 as libc::c_uint,
                                 strategy: ZSTD_btlazy2,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 14i32 as libc::c_uint,
                                 searchLog: 3i32 as libc::c_uint,
                                 minMatch: 4i32 as libc::c_uint,
                                 targetLength: 12i32 as libc::c_uint,
                                 strategy: ZSTD_btopt,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 14i32 as libc::c_uint,
                                 searchLog: 4i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 24i32 as libc::c_uint,
                                 strategy: ZSTD_btopt,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 14i32 as libc::c_uint,
                                 searchLog: 5i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 32i32 as libc::c_uint,
                                 strategy: ZSTD_btultra,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 15i32 as libc::c_uint,
                                 searchLog: 6i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 64i32 as libc::c_uint,
                                 strategy: ZSTD_btultra,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 15i32 as libc::c_uint,
                                 searchLog: 7i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 256i32 as libc::c_uint,
                                 strategy: ZSTD_btultra,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 15i32 as libc::c_uint,
                                 searchLog: 5i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 48i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 15i32 as libc::c_uint,
                                 searchLog: 6i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 128i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 15i32 as libc::c_uint,
                                 searchLog: 7i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 256i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 15i32 as libc::c_uint,
                                 searchLog: 8i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 256i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 15i32 as libc::c_uint,
                                 searchLog: 8i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 512i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 15i32 as libc::c_uint,
                                 searchLog: 9i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 512i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,},
      ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                 chainLog: 15i32 as libc::c_uint,
                                 hashLog: 15i32 as libc::c_uint,
                                 searchLog: 10i32 as libc::c_uint,
                                 minMatch: 3i32 as libc::c_uint,
                                 targetLength: 999i32 as libc::c_uint,
                                 strategy: ZSTD_btultra2,}]];
// Initialized in run_static_initializers
static mut maxWindowResize: U64 = 0;
/* * ZSTD_adjustCParams_internal() :
 *  optimize `cPar` for a specified input (`srcSize` and `dictSize`).
 *  mostly downsize to reduce memory consumption and initialization latency.
 * `srcSize` can be ZSTD_CONTENTSIZE_UNKNOWN when not known.
 *  note : for the time being, `srcSize==0` means "unknown" too, for compatibility with older convention.
 *  condition : cPar is presumed validated (can be checked using ZSTD_checkCParams()). */
unsafe extern "C" fn ZSTD_adjustCParams_internal(mut cPar:
                                                     ZSTD_compressionParameters,
                                                 mut srcSize:
                                                     libc::c_ulonglong,
                                                 mut dictSize: size_t)
 -> ZSTD_compressionParameters {
    /* (1<<9) + 1 */
    static mut minSrcSize: U64 = 513i32 as U64;
    if 0 != dictSize &&
           srcSize.wrapping_add(1i32 as libc::c_ulonglong) <
               2i32 as libc::c_ulonglong {
        srcSize = minSrcSize as libc::c_ulonglong
    } else if srcSize == 0i32 as libc::c_ulonglong {
        srcSize = 0u64.wrapping_sub(1i32 as libc::c_ulonglong)
    }
    if srcSize < maxWindowResize as libc::c_ulonglong &&
           dictSize < maxWindowResize {
        let tSize: U32 =
            srcSize.wrapping_add(dictSize as libc::c_ulonglong) as U32;
        static mut hashSizeMin: U32 = (1i32 << 6i32) as U32;
        let srcLog: U32 =
            if tSize < hashSizeMin {
                6i32 as libc::c_uint
            } else {
                ZSTD_highbit32(tSize.wrapping_sub(1i32 as
                                                      libc::c_uint)).wrapping_add(1i32
                                                                                      as
                                                                                      libc::c_uint)
            };
        if cPar.windowLog > srcLog { cPar.windowLog = srcLog }
    }
    if cPar.hashLog > cPar.windowLog.wrapping_add(1i32 as libc::c_uint) {
        cPar.hashLog = cPar.windowLog.wrapping_add(1i32 as libc::c_uint)
    }
    let cycleLog: U32 = ZSTD_cycleLog(cPar.chainLog, cPar.strategy);
    if cycleLog > cPar.windowLog {
        cPar.chainLog =
            cPar.chainLog.wrapping_sub(cycleLog.wrapping_sub(cPar.windowLog))
    }
    if cPar.windowLog < 10i32 as libc::c_uint {
        cPar.windowLog = 10i32 as libc::c_uint
    }
    return cPar;
}
/* * ZSTD_cycleLog() :
 *  condition for correct operation : hashLog > 1 */
unsafe extern "C" fn ZSTD_cycleLog(mut hashLog: U32, mut strat: ZSTD_strategy)
 -> U32 {
    let btScale: U32 =
        (strat as U32 >= ZSTD_btlazy2 as libc::c_int as U32) as libc::c_int as
            U32;
    return hashLog.wrapping_sub(btScale);
}
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> U32 {
    return (31i32 - val.leading_zeros() as i32) as U32;
}
/* ZSTD_assignParamsToCCtxParams() :
 * params is presumed valid at this stage */
unsafe extern "C" fn ZSTD_assignParamsToCCtxParams(mut cctxParams:
                                                       ZSTD_CCtx_params,
                                                   mut params:
                                                       ZSTD_parameters)
 -> ZSTD_CCtx_params {
    let mut ret: ZSTD_CCtx_params = cctxParams;
    ret.cParams = params.cParams;
    ret.fParams = params.fParams;
    ret.compressionLevel = 3i32;
    return ret;
}
/* ZSTD_compress_advanced_internal() :
 * Private use only. To be called from zstdmt_compress.c. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compress_advanced_internal(mut cctx:
                                                             *mut ZSTD_CCtx,
                                                         mut dst:
                                                             *mut libc::c_void,
                                                         mut dstCapacity:
                                                             size_t,
                                                         mut src:
                                                             *const libc::c_void,
                                                         mut srcSize: size_t,
                                                         mut dict:
                                                             *const libc::c_void,
                                                         mut dictSize: size_t,
                                                         mut params:
                                                             ZSTD_CCtx_params)
 -> size_t {
    let errcod: size_t =
        ZSTD_compressBegin_internal(cctx, dict, dictSize, ZSTD_dct_auto,
                                    ZSTD_dtlm_fast, 0 as *const ZSTD_CDict,
                                    params, srcSize, ZSTDb_not_buffered);
    if 0 != ERR_isError(errcod) { return errcod }
    return ZSTD_compressEnd(cctx, dst, dstCapacity, src, srcSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressEnd(mut cctx: *mut ZSTD_CCtx,
                                          mut dst: *mut libc::c_void,
                                          mut dstCapacity: size_t,
                                          mut src: *const libc::c_void,
                                          mut srcSize: size_t) -> size_t {
    let mut endResult: size_t = 0;
    let cSize: size_t =
        ZSTD_compressContinue_internal(cctx, dst, dstCapacity, src, srcSize,
                                       1i32 as U32, 1i32 as U32);
    if 0 != ERR_isError(cSize) { return cSize }
    endResult =
        ZSTD_writeEpilogue(cctx,
                           (dst as *mut libc::c_char).offset(cSize as isize)
                               as *mut libc::c_void,
                           dstCapacity.wrapping_sub(cSize));
    if 0 != ERR_isError(endResult) { return endResult }
    if (*cctx).pledgedSrcSizePlusOne != 0i32 as libc::c_ulonglong {
        if (*cctx).pledgedSrcSizePlusOne !=
               (*cctx).consumedSrcSize.wrapping_add(1i32 as libc::c_ulonglong)
           {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
        }
    }
    return cSize.wrapping_add(endResult);
}
unsafe extern "C" fn ZSTD_compressContinue_internal(mut cctx: *mut ZSTD_CCtx,
                                                    mut dst:
                                                        *mut libc::c_void,
                                                    mut dstCapacity: size_t,
                                                    mut src:
                                                        *const libc::c_void,
                                                    mut srcSize: size_t,
                                                    mut frame: U32,
                                                    mut lastFrameChunk: U32)
 -> size_t {
    let ms: *mut ZSTD_matchState_t = &mut (*cctx).blockState.matchState;
    let mut fhSize: size_t = 0i32 as size_t;
    if (*cctx).stage as libc::c_uint ==
           ZSTDcs_created as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    }
    if 0 != frame &&
           (*cctx).stage as libc::c_uint ==
               ZSTDcs_init as libc::c_int as libc::c_uint {
        fhSize =
            ZSTD_writeFrameHeader(dst, dstCapacity, (*cctx).appliedParams,
                                  (*cctx).pledgedSrcSizePlusOne.wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_ulonglong)
                                      as U64, (*cctx).dictID);
        if 0 != ERR_isError(fhSize) { return fhSize }
        dstCapacity =
            (dstCapacity as libc::c_ulong).wrapping_sub(fhSize) as size_t as
                size_t;
        dst =
            (dst as *mut libc::c_char).offset(fhSize as isize) as
                *mut libc::c_void;
        (*cctx).stage = ZSTDcs_ongoing
    }
    if 0 == srcSize { return fhSize }
    if 0 == ZSTD_window_update(&mut (*ms).window, src, srcSize) {
        (*ms).nextToUpdate = (*ms).window.dictLimit
    }
    if 0 != (*cctx).appliedParams.ldmParams.enableLdm {
        ZSTD_window_update(&mut (*cctx).ldmState.window, src, srcSize);
    }
    if 0 == frame {
        if 0 !=
               ZSTD_window_needOverflowCorrection((*ms).window,
                                                  (src as
                                                       *const libc::c_char).offset(srcSize
                                                                                       as
                                                                                       isize)
                                                      as *const libc::c_void)
           {
            let cycleLog: U32 =
                ZSTD_cycleLog((*cctx).appliedParams.cParams.chainLog,
                              (*cctx).appliedParams.cParams.strategy);
            let correction: U32 =
                ZSTD_window_correctOverflow(&mut (*ms).window, cycleLog,
                                            (1i32 <<
                                                 (*cctx).appliedParams.cParams.windowLog)
                                                as U32, src);
            ZSTD_reduceIndex(cctx, correction);
            if (*ms).nextToUpdate < correction {
                (*ms).nextToUpdate = 0i32 as U32
            } else {
                (*ms).nextToUpdate =
                    ((*ms).nextToUpdate as
                         libc::c_uint).wrapping_sub(correction) as U32 as U32
            }
            (*ms).loadedDictEnd = 0i32 as U32;
            (*ms).dictMatchState = 0 as *const ZSTD_matchState_t
        }
    }
    let cSize: size_t =
        if 0 != frame {
            ZSTD_compress_frameChunk(cctx, dst, dstCapacity, src, srcSize,
                                     lastFrameChunk)
        } else {
            ZSTD_compressBlock_internal(cctx, dst, dstCapacity, src, srcSize)
        };
    if 0 != ERR_isError(cSize) { return cSize }
    (*cctx).consumedSrcSize =
        (*cctx).consumedSrcSize.wrapping_add(srcSize as libc::c_ulonglong);
    (*cctx).producedCSize =
        (*cctx).producedCSize.wrapping_add(cSize.wrapping_add(fhSize) as
                                               libc::c_ulonglong);
    if (*cctx).pledgedSrcSizePlusOne != 0i32 as libc::c_ulonglong {
        if (*cctx).consumedSrcSize.wrapping_add(1i32 as libc::c_ulonglong) >
               (*cctx).pledgedSrcSizePlusOne {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
        }
    }
    return cSize.wrapping_add(fhSize);
}
unsafe extern "C" fn ZSTD_compressBlock_internal(mut zc: *mut ZSTD_CCtx,
                                                 mut dst: *mut libc::c_void,
                                                 mut dstCapacity: size_t,
                                                 mut src: *const libc::c_void,
                                                 mut srcSize: size_t)
 -> size_t {
    let ms: *mut ZSTD_matchState_t = &mut (*zc).blockState.matchState;
    let mut cSize: size_t = 0;
    ZSTD_assertEqualCParams((*zc).appliedParams.cParams, (*ms).cParams);
    if srcSize <
           ((1i32 + 1i32 + 1i32) as
                libc::c_ulong).wrapping_add(ZSTD_blockHeaderSize).wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_ulong)
       {
        ZSTD_ldm_skipSequences(&mut (*zc).externSeqStore, srcSize,
                               (*zc).appliedParams.cParams.minMatch);
        cSize = 0i32 as size_t
    } else {
        /* don't even attempt compression below a certain srcSize */
        ZSTD_resetSeqStore(&mut (*zc).seqStore);
        (*ms).opt.symbolCosts = &mut (*(*zc).blockState.prevCBlock).entropy;
        let base: *const BYTE = (*ms).window.base;
        let istart: *const BYTE = src as *const BYTE;
        let current: U32 =
            istart.wrapping_offset_from(base) as libc::c_long as U32;
        ::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong ==
            8i32 as libc::c_ulong;
        if current > (*ms).nextToUpdate.wrapping_add(384i32 as libc::c_uint) {
            (*ms).nextToUpdate =
                current.wrapping_sub(if (192i32 as libc::c_uint) <
                                            current.wrapping_sub((*ms).nextToUpdate).wrapping_sub(384i32
                                                                                                      as
                                                                                                      libc::c_uint)
                                        {
                                         192i32 as libc::c_uint
                                     } else {
                                         current.wrapping_sub((*ms).nextToUpdate).wrapping_sub(384i32
                                                                                                   as
                                                                                                   libc::c_uint)
                                     })
        }
        let dictMode: ZSTD_dictMode_e = ZSTD_matchState_dictMode(ms);
        let mut lastLLSize: size_t = 0;
        let mut i: libc::c_int = 0;
        i = 0i32;
        while i < 3i32 {
            (*(*zc).blockState.nextCBlock).rep[i as usize] =
                (*(*zc).blockState.prevCBlock).rep[i as usize];
            i += 1
        }
        if (*zc).externSeqStore.pos < (*zc).externSeqStore.size {
            lastLLSize =
                ZSTD_ldm_blockCompress(&mut (*zc).externSeqStore, ms,
                                       &mut (*zc).seqStore,
                                       (*(*zc).blockState.nextCBlock).rep.as_mut_ptr(),
                                       src, srcSize)
        } else if 0 != (*zc).appliedParams.ldmParams.enableLdm {
            let mut ldmSeqStore: rawSeqStore_t =
                rawSeqStore_t{seq: 0 as *mut rawSeq,
                              pos: 0i32 as size_t,
                              size: 0i32 as size_t,
                              capacity: 0i32 as size_t,};
            ldmSeqStore.seq = (*zc).ldmSequences;
            ldmSeqStore.capacity = (*zc).maxNbLdmSequences;
            let errcod: size_t =
                ZSTD_ldm_generateSequences(&mut (*zc).ldmState,
                                           &mut ldmSeqStore,
                                           &mut (*zc).appliedParams.ldmParams,
                                           src, srcSize);
            if 0 != ERR_isError(errcod) { return errcod }
            lastLLSize =
                ZSTD_ldm_blockCompress(&mut ldmSeqStore, ms,
                                       &mut (*zc).seqStore,
                                       (*(*zc).blockState.nextCBlock).rep.as_mut_ptr(),
                                       src, srcSize)
        } else {
            let blockCompressor: ZSTD_blockCompressor =
                ZSTD_selectBlockCompressor((*zc).appliedParams.cParams.strategy,
                                           dictMode);
            lastLLSize =
                blockCompressor.expect("non-null function pointer")(ms,
                                                                    &mut (*zc).seqStore,
                                                                    (*(*zc).blockState.nextCBlock).rep.as_mut_ptr(),
                                                                    src,
                                                                    srcSize)
        }
        let lastLiterals: *const BYTE =
            (src as
                 *const BYTE).offset(srcSize as
                                         isize).offset(-(lastLLSize as
                                                             isize));
        ZSTD_storeLastLiterals(&mut (*zc).seqStore, lastLiterals, lastLLSize);
        cSize =
            ZSTD_compressSequences(&mut (*zc).seqStore,
                                   &mut (*(*zc).blockState.prevCBlock).entropy,
                                   &mut (*(*zc).blockState.nextCBlock).entropy,
                                   &mut (*zc).appliedParams, dst, dstCapacity,
                                   srcSize,
                                   (*zc).entropyWorkspace as
                                       *mut libc::c_void,
                                   (6i32 << 10i32) as size_t, (*zc).bmi2)
    }
    /* statically allocated in resetCCtx */
    if 0 == ERR_isError(cSize) && cSize != 0i32 as libc::c_ulong {
        let tmp: *mut ZSTD_compressedBlockState_t =
            (*zc).blockState.prevCBlock;
        (*zc).blockState.prevCBlock = (*zc).blockState.nextCBlock;
        (*zc).blockState.nextCBlock = tmp
    }
    if (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode as
           libc::c_uint == FSE_repeat_valid as libc::c_int as libc::c_uint {
        (*(*zc).blockState.prevCBlock).entropy.fse.offcode_repeatMode =
            FSE_repeat_check
    }
    return cSize;
}
unsafe extern "C" fn ZSTD_compressSequences(mut seqStorePtr: *mut seqStore_t,
                                            mut prevEntropy:
                                                *const ZSTD_entropyCTables_t,
                                            mut nextEntropy:
                                                *mut ZSTD_entropyCTables_t,
                                            mut cctxParams:
                                                *const ZSTD_CCtx_params,
                                            mut dst: *mut libc::c_void,
                                            mut dstCapacity: size_t,
                                            mut srcSize: size_t,
                                            mut workspace: *mut libc::c_void,
                                            mut wkspSize: size_t,
                                            mut bmi2: libc::c_int) -> size_t {
    let cSize: size_t =
        ZSTD_compressSequences_internal(seqStorePtr, prevEntropy, nextEntropy,
                                        cctxParams, dst, dstCapacity,
                                        workspace, wkspSize, bmi2);
    if cSize == 0i32 as libc::c_ulong { return 0i32 as size_t }
    if 0 !=
           (cSize == -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t)
               as libc::c_int & (srcSize <= dstCapacity) as libc::c_int {
        return 0i32 as size_t
    }
    if 0 != ERR_isError(cSize) { return cSize }
    let maxCSize: size_t =
        srcSize.wrapping_sub(ZSTD_minGain(srcSize,
                                          (*cctxParams).cParams.strategy));
    if cSize >= maxCSize { return 0i32 as size_t }
    return cSize;
}
/* ZSTD_compressSequences_internal():
 * actually compresses both literals and sequences */
unsafe extern "C" fn ZSTD_compressSequences_internal(mut seqStorePtr:
                                                         *mut seqStore_t,
                                                     mut prevEntropy:
                                                         *const ZSTD_entropyCTables_t,
                                                     mut nextEntropy:
                                                         *mut ZSTD_entropyCTables_t,
                                                     mut cctxParams:
                                                         *const ZSTD_CCtx_params,
                                                     mut dst:
                                                         *mut libc::c_void,
                                                     mut dstCapacity: size_t,
                                                     mut workspace:
                                                         *mut libc::c_void,
                                                     mut wkspSize: size_t,
                                                     bmi2: libc::c_int)
 -> size_t {
    let longOffsets: libc::c_int =
        ((*cctxParams).cParams.windowLog >
             (if 0 != MEM_32bits() { 25i32 } else { 57i32 }) as U32) as
            libc::c_int;
    let strategy: ZSTD_strategy = (*cctxParams).cParams.strategy;
    let mut count: [libc::c_uint; 53] = [0; 53];
    let mut CTable_LitLength: *mut FSE_CTable =
        (*nextEntropy).fse.litlengthCTable.as_mut_ptr();
    let mut CTable_OffsetBits: *mut FSE_CTable =
        (*nextEntropy).fse.offcodeCTable.as_mut_ptr();
    let mut CTable_MatchLength: *mut FSE_CTable =
        (*nextEntropy).fse.matchlengthCTable.as_mut_ptr();
    /* compressed, raw or rle */
    let mut LLtype: U32 = 0;
    let mut Offtype: U32 = 0;
    let mut MLtype: U32 = 0;
    let sequences: *const seqDef = (*seqStorePtr).sequencesStart;
    let ofCodeTable: *const BYTE = (*seqStorePtr).ofCode;
    let llCodeTable: *const BYTE = (*seqStorePtr).llCode;
    let mlCodeTable: *const BYTE = (*seqStorePtr).mlCode;
    let ostart: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = ostart.offset(dstCapacity as isize);
    let mut op: *mut BYTE = ostart;
    let nbSeq: size_t =
        (*seqStorePtr).sequences.wrapping_offset_from((*seqStorePtr).sequencesStart)
            as libc::c_long as size_t;
    let mut seqHead: *mut BYTE = 0 as *mut BYTE;
    let mut lastNCount: *mut BYTE = 0 as *mut BYTE;
    let literals: *const BYTE = (*seqStorePtr).litStart;
    let litSize: size_t =
        (*seqStorePtr).lit.wrapping_offset_from(literals) as libc::c_long as
            size_t;
    let disableLiteralCompression: libc::c_int =
        ((*cctxParams).cParams.strategy as libc::c_uint ==
             ZSTD_fast as libc::c_int as libc::c_uint &&
             (*cctxParams).cParams.targetLength > 0i32 as libc::c_uint) as
            libc::c_int;
    let cSize: size_t =
        ZSTD_compressLiterals(&(*prevEntropy).huf, &mut (*nextEntropy).huf,
                              (*cctxParams).cParams.strategy,
                              disableLiteralCompression,
                              op as *mut libc::c_void, dstCapacity,
                              literals as *const libc::c_void, litSize,
                              workspace, wkspSize, bmi2);
    if 0 != ERR_isError(cSize) { return cSize }
    op = op.offset(cSize as isize);
    if (oend.wrapping_offset_from(op) as libc::c_long) <
           (3i32 + 1i32) as libc::c_long {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    if nbSeq < 0x7fi32 as libc::c_ulong {
        let fresh0 = op;
        op = op.offset(1);
        *fresh0 = nbSeq as BYTE
    } else if nbSeq < 0x7f00i32 as libc::c_ulong {
        *op.offset(0isize) =
            (nbSeq >> 8i32).wrapping_add(0x80i32 as libc::c_ulong) as BYTE;
        *op.offset(1isize) = nbSeq as BYTE;
        op = op.offset(2isize)
    } else {
        *op.offset(0isize) = 0xffi32 as BYTE;
        MEM_writeLE16(op.offset(1isize) as *mut libc::c_void,
                      nbSeq.wrapping_sub(0x7f00i32 as libc::c_ulong) as U16);
        op = op.offset(3isize)
    }
    if nbSeq == 0i32 as libc::c_ulong {
        memcpy(&mut (*nextEntropy).fse as *mut ZSTD_fseCTables_t as
                   *mut libc::c_void,
               &(*prevEntropy).fse as *const ZSTD_fseCTables_t as
                   *const libc::c_void,
               ::std::mem::size_of::<ZSTD_fseCTables_t>() as libc::c_ulong);
        return op.wrapping_offset_from(ostart) as libc::c_long as size_t
    }
    let fresh1 = op;
    op = op.offset(1);
    seqHead = fresh1;
    ZSTD_seqToCodes(seqStorePtr);
    let mut max: libc::c_uint = 35i32 as libc::c_uint;
    let mostFrequent: size_t =
        HIST_countFast_wksp(count.as_mut_ptr(), &mut max,
                            llCodeTable as *const libc::c_void, nbSeq,
                            workspace, wkspSize);
    (*nextEntropy).fse.litlength_repeatMode =
        (*prevEntropy).fse.litlength_repeatMode;
    LLtype =
        ZSTD_selectEncodingType(&mut (*nextEntropy).fse.litlength_repeatMode,
                                count.as_mut_ptr(), max, mostFrequent, nbSeq,
                                9i32 as libc::c_uint,
                                (*prevEntropy).fse.litlengthCTable.as_ptr(),
                                LL_defaultNorm.as_ptr(), LL_defaultNormLog,
                                ZSTD_defaultAllowed, strategy) as U32;
    let countSize: size_t =
        ZSTD_buildCTable(op as *mut libc::c_void,
                         oend.wrapping_offset_from(op) as libc::c_long as
                             size_t, CTable_LitLength, 9i32 as U32,
                         LLtype as symbolEncodingType_e, count.as_mut_ptr(),
                         max, llCodeTable, nbSeq, LL_defaultNorm.as_ptr(),
                         LL_defaultNormLog, 35i32 as U32,
                         (*prevEntropy).fse.litlengthCTable.as_ptr(),
                         ::std::mem::size_of::<[FSE_CTable; 329]>() as
                             libc::c_ulong, workspace, wkspSize);
    if 0 != ERR_isError(countSize) { return countSize }
    if LLtype == set_compressed as libc::c_int as libc::c_uint {
        lastNCount = op
    }
    op = op.offset(countSize as isize);
    let mut max_0: libc::c_uint = 31i32 as libc::c_uint;
    let mostFrequent_0: size_t =
        HIST_countFast_wksp(count.as_mut_ptr(), &mut max_0,
                            ofCodeTable as *const libc::c_void, nbSeq,
                            workspace, wkspSize);
    let defaultPolicy: ZSTD_defaultPolicy_e =
        (if max_0 <= 28i32 as libc::c_uint {
             ZSTD_defaultAllowed as libc::c_int
         } else { ZSTD_defaultDisallowed as libc::c_int }) as
            ZSTD_defaultPolicy_e;
    (*nextEntropy).fse.offcode_repeatMode =
        (*prevEntropy).fse.offcode_repeatMode;
    Offtype =
        ZSTD_selectEncodingType(&mut (*nextEntropy).fse.offcode_repeatMode,
                                count.as_mut_ptr(), max_0, mostFrequent_0,
                                nbSeq, 8i32 as libc::c_uint,
                                (*prevEntropy).fse.offcodeCTable.as_ptr(),
                                OF_defaultNorm.as_ptr(), OF_defaultNormLog,
                                defaultPolicy, strategy) as U32;
    let countSize_0: size_t =
        ZSTD_buildCTable(op as *mut libc::c_void,
                         oend.wrapping_offset_from(op) as libc::c_long as
                             size_t, CTable_OffsetBits, 8i32 as U32,
                         Offtype as symbolEncodingType_e, count.as_mut_ptr(),
                         max_0, ofCodeTable, nbSeq, OF_defaultNorm.as_ptr(),
                         OF_defaultNormLog, 28i32 as U32,
                         (*prevEntropy).fse.offcodeCTable.as_ptr(),
                         ::std::mem::size_of::<[FSE_CTable; 193]>() as
                             libc::c_ulong, workspace, wkspSize);
    if 0 != ERR_isError(countSize_0) { return countSize_0 }
    if Offtype == set_compressed as libc::c_int as libc::c_uint {
        lastNCount = op
    }
    op = op.offset(countSize_0 as isize);
    let mut max_1: libc::c_uint = 52i32 as libc::c_uint;
    let mostFrequent_1: size_t =
        HIST_countFast_wksp(count.as_mut_ptr(), &mut max_1,
                            mlCodeTable as *const libc::c_void, nbSeq,
                            workspace, wkspSize);
    (*nextEntropy).fse.matchlength_repeatMode =
        (*prevEntropy).fse.matchlength_repeatMode;
    MLtype =
        ZSTD_selectEncodingType(&mut (*nextEntropy).fse.matchlength_repeatMode,
                                count.as_mut_ptr(), max_1, mostFrequent_1,
                                nbSeq, 9i32 as libc::c_uint,
                                (*prevEntropy).fse.matchlengthCTable.as_ptr(),
                                ML_defaultNorm.as_ptr(), ML_defaultNormLog,
                                ZSTD_defaultAllowed, strategy) as U32;
    let countSize_1: size_t =
        ZSTD_buildCTable(op as *mut libc::c_void,
                         oend.wrapping_offset_from(op) as libc::c_long as
                             size_t, CTable_MatchLength, 9i32 as U32,
                         MLtype as symbolEncodingType_e, count.as_mut_ptr(),
                         max_1, mlCodeTable, nbSeq, ML_defaultNorm.as_ptr(),
                         ML_defaultNormLog, 52i32 as U32,
                         (*prevEntropy).fse.matchlengthCTable.as_ptr(),
                         ::std::mem::size_of::<[FSE_CTable; 363]>() as
                             libc::c_ulong, workspace, wkspSize);
    if 0 != ERR_isError(countSize_1) { return countSize_1 }
    if MLtype == set_compressed as libc::c_int as libc::c_uint {
        lastNCount = op
    }
    op = op.offset(countSize_1 as isize);
    *seqHead =
        (LLtype <<
             6i32).wrapping_add(Offtype << 4i32).wrapping_add(MLtype << 2i32)
            as BYTE;
    let bitstreamSize: size_t =
        ZSTD_encodeSequences(op as *mut libc::c_void,
                             oend.wrapping_offset_from(op) as libc::c_long as
                                 size_t, CTable_MatchLength, mlCodeTable,
                             CTable_OffsetBits, ofCodeTable, CTable_LitLength,
                             llCodeTable, sequences, nbSeq, longOffsets,
                             bmi2);
    if 0 != ERR_isError(bitstreamSize) { return bitstreamSize }
    op = op.offset(bitstreamSize as isize);
    if !lastNCount.is_null() &&
           (op.wrapping_offset_from(lastNCount) as libc::c_long) <
               4i32 as libc::c_long {
        return 0i32 as size_t
    }
    return op.wrapping_offset_from(ostart) as libc::c_long as size_t;
}
unsafe extern "C" fn ZSTD_encodeSequences(mut dst: *mut libc::c_void,
                                          mut dstCapacity: size_t,
                                          mut CTable_MatchLength:
                                              *const FSE_CTable,
                                          mut mlCodeTable: *const BYTE,
                                          mut CTable_OffsetBits:
                                              *const FSE_CTable,
                                          mut ofCodeTable: *const BYTE,
                                          mut CTable_LitLength:
                                              *const FSE_CTable,
                                          mut llCodeTable: *const BYTE,
                                          mut sequences: *const seqDef,
                                          mut nbSeq: size_t,
                                          mut longOffsets: libc::c_int,
                                          mut bmi2: libc::c_int) -> size_t {
    if 0 != bmi2 {
        return ZSTD_encodeSequences_bmi2(dst, dstCapacity, CTable_MatchLength,
                                         mlCodeTable, CTable_OffsetBits,
                                         ofCodeTable, CTable_LitLength,
                                         llCodeTable, sequences, nbSeq,
                                         longOffsets)
    }
    return ZSTD_encodeSequences_default(dst, dstCapacity, CTable_MatchLength,
                                        mlCodeTable, CTable_OffsetBits,
                                        ofCodeTable, CTable_LitLength,
                                        llCodeTable, sequences, nbSeq,
                                        longOffsets);
}
unsafe extern "C" fn ZSTD_encodeSequences_default(mut dst: *mut libc::c_void,
                                                  mut dstCapacity: size_t,
                                                  mut CTable_MatchLength:
                                                      *const FSE_CTable,
                                                  mut mlCodeTable:
                                                      *const BYTE,
                                                  mut CTable_OffsetBits:
                                                      *const FSE_CTable,
                                                  mut ofCodeTable:
                                                      *const BYTE,
                                                  mut CTable_LitLength:
                                                      *const FSE_CTable,
                                                  mut llCodeTable:
                                                      *const BYTE,
                                                  mut sequences:
                                                      *const seqDef,
                                                  mut nbSeq: size_t,
                                                  mut longOffsets:
                                                      libc::c_int) -> size_t {
    return ZSTD_encodeSequences_body(dst, dstCapacity, CTable_MatchLength,
                                     mlCodeTable, CTable_OffsetBits,
                                     ofCodeTable, CTable_LitLength,
                                     llCodeTable, sequences, nbSeq,
                                     longOffsets);
}
unsafe extern "C" fn ZSTD_encodeSequences_body(mut dst: *mut libc::c_void,
                                               mut dstCapacity: size_t,
                                               mut CTable_MatchLength:
                                                   *const FSE_CTable,
                                               mut mlCodeTable: *const BYTE,
                                               mut CTable_OffsetBits:
                                                   *const FSE_CTable,
                                               mut ofCodeTable: *const BYTE,
                                               mut CTable_LitLength:
                                                   *const FSE_CTable,
                                               mut llCodeTable: *const BYTE,
                                               mut sequences: *const seqDef,
                                               mut nbSeq: size_t,
                                               mut longOffsets: libc::c_int)
 -> size_t {
    let mut blockStream: BIT_CStream_t =
        BIT_CStream_t{bitContainer: 0,
                      bitPos: 0,
                      startPtr: 0 as *mut libc::c_char,
                      ptr: 0 as *mut libc::c_char,
                      endPtr: 0 as *mut libc::c_char,};
    let mut stateMatchLength: FSE_CState_t =
        FSE_CState_t{value: 0,
                     stateTable: 0 as *const libc::c_void,
                     symbolTT: 0 as *const libc::c_void,
                     stateLog: 0,};
    let mut stateOffsetBits: FSE_CState_t =
        FSE_CState_t{value: 0,
                     stateTable: 0 as *const libc::c_void,
                     symbolTT: 0 as *const libc::c_void,
                     stateLog: 0,};
    let mut stateLitLength: FSE_CState_t =
        FSE_CState_t{value: 0,
                     stateTable: 0 as *const libc::c_void,
                     symbolTT: 0 as *const libc::c_void,
                     stateLog: 0,};
    let errcod: size_t = BIT_initCStream(&mut blockStream, dst, dstCapacity);
    if 0 != ERR_isError(errcod) {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    FSE_initCState2(&mut stateMatchLength, CTable_MatchLength,
                    *mlCodeTable.offset(nbSeq.wrapping_sub(1i32 as
                                                               libc::c_ulong)
                                            as isize) as U32);
    FSE_initCState2(&mut stateOffsetBits, CTable_OffsetBits,
                    *ofCodeTable.offset(nbSeq.wrapping_sub(1i32 as
                                                               libc::c_ulong)
                                            as isize) as U32);
    FSE_initCState2(&mut stateLitLength, CTable_LitLength,
                    *llCodeTable.offset(nbSeq.wrapping_sub(1i32 as
                                                               libc::c_ulong)
                                            as isize) as U32);
    BIT_addBits(&mut blockStream,
                (*sequences.offset(nbSeq.wrapping_sub(1i32 as libc::c_ulong)
                                       as isize)).litLength as size_t,
                LL_bits[*llCodeTable.offset(nbSeq.wrapping_sub(1i32 as
                                                                   libc::c_ulong)
                                                as isize) as usize]);
    if 0 != MEM_32bits() { BIT_flushBits(&mut blockStream); }
    BIT_addBits(&mut blockStream,
                (*sequences.offset(nbSeq.wrapping_sub(1i32 as libc::c_ulong)
                                       as isize)).matchLength as size_t,
                ML_bits[*mlCodeTable.offset(nbSeq.wrapping_sub(1i32 as
                                                                   libc::c_ulong)
                                                as isize) as usize]);
    if 0 != MEM_32bits() { BIT_flushBits(&mut blockStream); }
    if 0 != longOffsets {
        let ofBits: U32 =
            *ofCodeTable.offset(nbSeq.wrapping_sub(1i32 as libc::c_ulong) as
                                    isize) as U32;
        let extraBits: libc::c_int =
            ofBits.wrapping_sub(if ofBits <
                                       ((if 0 != MEM_32bits() {
                                             25i32
                                         } else { 57i32 }) as
                                            U32).wrapping_sub(1i32 as
                                                                  libc::c_uint)
                                   {
                                    ofBits
                                } else {
                                    ((if 0 != MEM_32bits() {
                                          25i32
                                      } else { 57i32 }) as
                                         U32).wrapping_sub(1i32 as
                                                               libc::c_uint)
                                }) as libc::c_int;
        if 0 != extraBits {
            BIT_addBits(&mut blockStream,
                        (*sequences.offset(nbSeq.wrapping_sub(1i32 as
                                                                  libc::c_ulong)
                                               as isize)).offset as size_t,
                        extraBits as libc::c_uint);
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(&mut blockStream,
                    ((*sequences.offset(nbSeq.wrapping_sub(1i32 as
                                                               libc::c_ulong)
                                            as isize)).offset >> extraBits) as
                        size_t,
                    ofBits.wrapping_sub(extraBits as libc::c_uint));
    } else {
        BIT_addBits(&mut blockStream,
                    (*sequences.offset(nbSeq.wrapping_sub(1i32 as
                                                              libc::c_ulong)
                                           as isize)).offset as size_t,
                    *ofCodeTable.offset(nbSeq.wrapping_sub(1i32 as
                                                               libc::c_ulong)
                                            as isize) as libc::c_uint);
    }
    BIT_flushBits(&mut blockStream);
    let mut n: size_t = 0;
    n = nbSeq.wrapping_sub(2i32 as libc::c_ulong);
    while n < nbSeq {
        let llCode: BYTE = *llCodeTable.offset(n as isize);
        let ofCode: BYTE = *ofCodeTable.offset(n as isize);
        let mlCode: BYTE = *mlCodeTable.offset(n as isize);
        let llBits: U32 = LL_bits[llCode as usize];
        let ofBits_0: U32 = ofCode as U32;
        let mlBits: U32 = ML_bits[mlCode as usize];
        FSE_encodeSymbol(&mut blockStream, &mut stateOffsetBits,
                         ofCode as libc::c_uint);
        FSE_encodeSymbol(&mut blockStream, &mut stateMatchLength,
                         mlCode as libc::c_uint);
        if 0 != MEM_32bits() { BIT_flushBits(&mut blockStream); }
        FSE_encodeSymbol(&mut blockStream, &mut stateLitLength,
                         llCode as libc::c_uint);
        if 0 != MEM_32bits() ||
               ofBits_0.wrapping_add(mlBits).wrapping_add(llBits) >=
                   (64i32 - 7i32 - (9i32 + 9i32 + 8i32)) as libc::c_uint {
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(&mut blockStream,
                    (*sequences.offset(n as isize)).litLength as size_t,
                    llBits);
        if 0 != MEM_32bits() &&
               llBits.wrapping_add(mlBits) > 24i32 as libc::c_uint {
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(&mut blockStream,
                    (*sequences.offset(n as isize)).matchLength as size_t,
                    mlBits);
        if 0 != MEM_32bits() ||
               ofBits_0.wrapping_add(mlBits).wrapping_add(llBits) >
                   56i32 as libc::c_uint {
            BIT_flushBits(&mut blockStream);
        }
        if 0 != longOffsets {
            let extraBits_0: libc::c_int =
                ofBits_0.wrapping_sub(if ofBits_0 <
                                             ((if 0 != MEM_32bits() {
                                                   25i32
                                               } else { 57i32 }) as
                                                  U32).wrapping_sub(1i32 as
                                                                        libc::c_uint)
                                         {
                                          ofBits_0
                                      } else {
                                          ((if 0 != MEM_32bits() {
                                                25i32
                                            } else { 57i32 }) as
                                               U32).wrapping_sub(1i32 as
                                                                     libc::c_uint)
                                      }) as libc::c_int;
            if 0 != extraBits_0 {
                BIT_addBits(&mut blockStream,
                            (*sequences.offset(n as isize)).offset as size_t,
                            extraBits_0 as libc::c_uint);
                BIT_flushBits(&mut blockStream);
            }
            BIT_addBits(&mut blockStream,
                        ((*sequences.offset(n as isize)).offset >>
                             extraBits_0) as size_t,
                        ofBits_0.wrapping_sub(extraBits_0 as libc::c_uint));
        } else {
            BIT_addBits(&mut blockStream,
                        (*sequences.offset(n as isize)).offset as size_t,
                        ofBits_0);
        }
        BIT_flushBits(&mut blockStream);
        n = n.wrapping_sub(1)
    }
    FSE_flushCState(&mut blockStream, &mut stateMatchLength);
    FSE_flushCState(&mut blockStream, &mut stateOffsetBits);
    FSE_flushCState(&mut blockStream, &mut stateLitLength);
    let streamSize: size_t = BIT_closeCStream(&mut blockStream);
    if streamSize == 0i32 as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    return streamSize;
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
unsafe extern "C" fn ZSTD_encodeSequences_bmi2(mut dst: *mut libc::c_void,
                                               mut dstCapacity: size_t,
                                               mut CTable_MatchLength:
                                                   *const FSE_CTable,
                                               mut mlCodeTable: *const BYTE,
                                               mut CTable_OffsetBits:
                                                   *const FSE_CTable,
                                               mut ofCodeTable: *const BYTE,
                                               mut CTable_LitLength:
                                                   *const FSE_CTable,
                                               mut llCodeTable: *const BYTE,
                                               mut sequences: *const seqDef,
                                               mut nbSeq: size_t,
                                               mut longOffsets: libc::c_int)
 -> size_t {
    return ZSTD_encodeSequences_body(dst, dstCapacity, CTable_MatchLength,
                                     mlCodeTable, CTable_OffsetBits,
                                     ofCodeTable, CTable_LitLength,
                                     llCodeTable, sequences, nbSeq,
                                     longOffsets);
}
/* for static allocation */
static mut ML_defaultNormLog: U32 = 6i32 as U32;
static mut ML_defaultNorm: [S16; 53] =
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
     -1i32 as S16, -1i32 as S16, -1i32 as S16];
unsafe extern "C" fn ZSTD_buildCTable(mut dst: *mut libc::c_void,
                                      mut dstCapacity: size_t,
                                      mut nextCTable: *mut FSE_CTable,
                                      mut FSELog: U32,
                                      mut type_0: symbolEncodingType_e,
                                      mut count: *mut libc::c_uint,
                                      mut max: U32,
                                      mut codeTable: *const BYTE,
                                      mut nbSeq: size_t,
                                      mut defaultNorm: *const S16,
                                      mut defaultNormLog: U32,
                                      mut defaultMax: U32,
                                      mut prevCTable: *const FSE_CTable,
                                      mut prevCTableSize: size_t,
                                      mut workspace: *mut libc::c_void,
                                      mut workspaceSize: size_t) -> size_t {
    let mut op: *mut BYTE = dst as *mut BYTE;
    let oend: *const BYTE = op.offset(dstCapacity as isize);
    match type_0 as libc::c_uint {
        1 => {
            let errcod: size_t = FSE_buildCTable_rle(nextCTable, max as BYTE);
            if 0 != ERR_isError(errcod) { return errcod }
            if dstCapacity == 0i32 as libc::c_ulong {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
            }
            *op = *codeTable.offset(0isize);
            return 1i32 as size_t
        }
        3 => {
            memcpy(nextCTable as *mut libc::c_void,
                   prevCTable as *const libc::c_void, prevCTableSize);
            return 0i32 as size_t
        }
        0 => {
            let errcod_0: size_t =
                FSE_buildCTable_wksp(nextCTable, defaultNorm, defaultMax,
                                     defaultNormLog, workspace,
                                     workspaceSize);
            if 0 != ERR_isError(errcod_0) { return errcod_0 }
            return 0i32 as size_t
        }
        2 => {
            let mut norm: [S16; 53] = [0; 53];
            let mut nbSeq_1: size_t = nbSeq;
            let tableLog: U32 = FSE_optimalTableLog(FSELog, nbSeq, max);
            if *count.offset(*codeTable.offset(nbSeq.wrapping_sub(1i32 as
                                                                      libc::c_ulong)
                                                   as isize) as isize) >
                   1i32 as libc::c_uint {
                let ref mut fresh2 =
                    *count.offset(*codeTable.offset(nbSeq.wrapping_sub(1i32 as
                                                                           libc::c_ulong)
                                                        as isize) as isize);
                *fresh2 = (*fresh2).wrapping_sub(1);
                nbSeq_1 = nbSeq_1.wrapping_sub(1)
            }
            let errcod_1: size_t =
                FSE_normalizeCount(norm.as_mut_ptr(), tableLog, count,
                                   nbSeq_1, max);
            if 0 != ERR_isError(errcod_1) { return errcod_1 }
            let NCountSize: size_t =
                FSE_writeNCount(op as *mut libc::c_void,
                                oend.wrapping_offset_from(op) as libc::c_long
                                    as size_t, norm.as_mut_ptr(), max,
                                tableLog);
            if 0 != ERR_isError(NCountSize) { return NCountSize }
            let errcod_2: size_t =
                FSE_buildCTable_wksp(nextCTable, norm.as_mut_ptr(), max,
                                     tableLog, workspace, workspaceSize);
            if 0 != ERR_isError(errcod_2) { return errcod_2 }
            return NCountSize
        }
        _ => { return -(ZSTD_error_GENERIC as libc::c_int) as size_t }
    };
}
unsafe extern "C" fn ZSTD_selectEncodingType(mut repeatMode: *mut FSE_repeat,
                                             mut count: *const libc::c_uint,
                                             max: libc::c_uint,
                                             mostFrequent: size_t,
                                             mut nbSeq: size_t,
                                             FSELog: libc::c_uint,
                                             mut prevCTable:
                                                 *const FSE_CTable,
                                             mut defaultNorm:
                                                 *const libc::c_short,
                                             mut defaultNormLog: U32,
                                             isDefaultAllowed:
                                                 ZSTD_defaultPolicy_e,
                                             strategy: ZSTD_strategy)
 -> symbolEncodingType_e {
    if mostFrequent == nbSeq {
        *repeatMode = FSE_repeat_none;
        if 0 != isDefaultAllowed as libc::c_uint &&
               nbSeq <= 2i32 as libc::c_ulong {
            return set_basic
        }
        return set_rle
    }
    if (strategy as libc::c_uint) < ZSTD_lazy as libc::c_int as libc::c_uint {
        if 0 != isDefaultAllowed as u64 {
            let staticFse_nbSeq_max: size_t = 1000i32 as size_t;
            let mult: size_t =
                (10i32 as libc::c_uint).wrapping_sub(strategy as libc::c_uint)
                    as size_t;
            let baseLog: size_t = 3i32 as size_t;
            let dynamicFse_nbSeq_min: size_t =
                ((1i32 as size_t) << defaultNormLog).wrapping_mul(mult) >>
                    baseLog;
            if *repeatMode as libc::c_uint ==
                   FSE_repeat_valid as libc::c_int as libc::c_uint &&
                   nbSeq < staticFse_nbSeq_max {
                return set_repeat
            }
            if nbSeq < dynamicFse_nbSeq_min ||
                   mostFrequent <
                       nbSeq >>
                           defaultNormLog.wrapping_sub(1i32 as libc::c_uint) {
                *repeatMode = FSE_repeat_none;
                return set_basic
            }
        }
    } else {
        let basicCost: size_t =
            if 0 != isDefaultAllowed as libc::c_uint {
                ZSTD_crossEntropyCost(defaultNorm, defaultNormLog, count, max)
            } else { -(ZSTD_error_GENERIC as libc::c_int) as size_t };
        let repeatCost: size_t =
            if *repeatMode as libc::c_uint !=
                   FSE_repeat_none as libc::c_int as libc::c_uint {
                ZSTD_fseBitCost(prevCTable, count, max)
            } else { -(ZSTD_error_GENERIC as libc::c_int) as size_t };
        let NCountCost: size_t = ZSTD_NCountCost(count, max, nbSeq, FSELog);
        let compressedCost: size_t =
            (NCountCost <<
                 3i32).wrapping_add(ZSTD_entropyCost(count, max, nbSeq));
        0 != isDefaultAllowed as u64;
        if basicCost <= repeatCost && basicCost <= compressedCost {
            *repeatMode = FSE_repeat_none;
            return set_basic
        }
        if repeatCost <= compressedCost { return set_repeat }
    }
    *repeatMode = FSE_repeat_check;
    return set_compressed;
}
/* *
 * Returns the cost in bits of encoding the distribution described by count
 * using the entropy bound.
 */
unsafe extern "C" fn ZSTD_entropyCost(mut count: *const libc::c_uint,
                                      max: libc::c_uint, total: size_t)
 -> size_t {
    let mut cost: libc::c_uint = 0i32 as libc::c_uint;
    let mut s: libc::c_uint = 0;
    s = 0i32 as libc::c_uint;
    while s <= max {
        let mut norm: libc::c_uint =
            ((256i32 as libc::c_uint).wrapping_mul(*count.offset(s as isize))
                 as libc::c_ulong).wrapping_div(total) as libc::c_uint;
        if *count.offset(s as isize) != 0i32 as libc::c_uint &&
               norm == 0i32 as libc::c_uint {
            norm = 1i32 as libc::c_uint
        }
        cost =
            cost.wrapping_add((*count.offset(s as
                                                 isize)).wrapping_mul(kInverseProbabiltyLog256[norm
                                                                                                   as
                                                                                                   usize]));
        s = s.wrapping_add(1)
    }
    return (cost >> 8i32) as size_t;
}
/* *
 * -log2(x / 256) lookup table for x in [0, 256).
 * If x == 0: Return 0
 * Else: Return floor(-log2(x / 256) * 256)
 */
static mut kInverseProbabiltyLog256: [libc::c_uint; 256] =
    [0i32 as libc::c_uint, 2048i32 as libc::c_uint, 1792i32 as libc::c_uint,
     1642i32 as libc::c_uint, 1536i32 as libc::c_uint,
     1453i32 as libc::c_uint, 1386i32 as libc::c_uint,
     1329i32 as libc::c_uint, 1280i32 as libc::c_uint,
     1236i32 as libc::c_uint, 1197i32 as libc::c_uint,
     1162i32 as libc::c_uint, 1130i32 as libc::c_uint,
     1100i32 as libc::c_uint, 1073i32 as libc::c_uint,
     1047i32 as libc::c_uint, 1024i32 as libc::c_uint,
     1001i32 as libc::c_uint, 980i32 as libc::c_uint, 960i32 as libc::c_uint,
     941i32 as libc::c_uint, 923i32 as libc::c_uint, 906i32 as libc::c_uint,
     889i32 as libc::c_uint, 874i32 as libc::c_uint, 859i32 as libc::c_uint,
     844i32 as libc::c_uint, 830i32 as libc::c_uint, 817i32 as libc::c_uint,
     804i32 as libc::c_uint, 791i32 as libc::c_uint, 779i32 as libc::c_uint,
     768i32 as libc::c_uint, 756i32 as libc::c_uint, 745i32 as libc::c_uint,
     734i32 as libc::c_uint, 724i32 as libc::c_uint, 714i32 as libc::c_uint,
     704i32 as libc::c_uint, 694i32 as libc::c_uint, 685i32 as libc::c_uint,
     676i32 as libc::c_uint, 667i32 as libc::c_uint, 658i32 as libc::c_uint,
     650i32 as libc::c_uint, 642i32 as libc::c_uint, 633i32 as libc::c_uint,
     626i32 as libc::c_uint, 618i32 as libc::c_uint, 610i32 as libc::c_uint,
     603i32 as libc::c_uint, 595i32 as libc::c_uint, 588i32 as libc::c_uint,
     581i32 as libc::c_uint, 574i32 as libc::c_uint, 567i32 as libc::c_uint,
     561i32 as libc::c_uint, 554i32 as libc::c_uint, 548i32 as libc::c_uint,
     542i32 as libc::c_uint, 535i32 as libc::c_uint, 529i32 as libc::c_uint,
     523i32 as libc::c_uint, 517i32 as libc::c_uint, 512i32 as libc::c_uint,
     506i32 as libc::c_uint, 500i32 as libc::c_uint, 495i32 as libc::c_uint,
     489i32 as libc::c_uint, 484i32 as libc::c_uint, 478i32 as libc::c_uint,
     473i32 as libc::c_uint, 468i32 as libc::c_uint, 463i32 as libc::c_uint,
     458i32 as libc::c_uint, 453i32 as libc::c_uint, 448i32 as libc::c_uint,
     443i32 as libc::c_uint, 438i32 as libc::c_uint, 434i32 as libc::c_uint,
     429i32 as libc::c_uint, 424i32 as libc::c_uint, 420i32 as libc::c_uint,
     415i32 as libc::c_uint, 411i32 as libc::c_uint, 407i32 as libc::c_uint,
     402i32 as libc::c_uint, 398i32 as libc::c_uint, 394i32 as libc::c_uint,
     390i32 as libc::c_uint, 386i32 as libc::c_uint, 382i32 as libc::c_uint,
     377i32 as libc::c_uint, 373i32 as libc::c_uint, 370i32 as libc::c_uint,
     366i32 as libc::c_uint, 362i32 as libc::c_uint, 358i32 as libc::c_uint,
     354i32 as libc::c_uint, 350i32 as libc::c_uint, 347i32 as libc::c_uint,
     343i32 as libc::c_uint, 339i32 as libc::c_uint, 336i32 as libc::c_uint,
     332i32 as libc::c_uint, 329i32 as libc::c_uint, 325i32 as libc::c_uint,
     322i32 as libc::c_uint, 318i32 as libc::c_uint, 315i32 as libc::c_uint,
     311i32 as libc::c_uint, 308i32 as libc::c_uint, 305i32 as libc::c_uint,
     302i32 as libc::c_uint, 298i32 as libc::c_uint, 295i32 as libc::c_uint,
     292i32 as libc::c_uint, 289i32 as libc::c_uint, 286i32 as libc::c_uint,
     282i32 as libc::c_uint, 279i32 as libc::c_uint, 276i32 as libc::c_uint,
     273i32 as libc::c_uint, 270i32 as libc::c_uint, 267i32 as libc::c_uint,
     264i32 as libc::c_uint, 261i32 as libc::c_uint, 258i32 as libc::c_uint,
     256i32 as libc::c_uint, 253i32 as libc::c_uint, 250i32 as libc::c_uint,
     247i32 as libc::c_uint, 244i32 as libc::c_uint, 241i32 as libc::c_uint,
     239i32 as libc::c_uint, 236i32 as libc::c_uint, 233i32 as libc::c_uint,
     230i32 as libc::c_uint, 228i32 as libc::c_uint, 225i32 as libc::c_uint,
     222i32 as libc::c_uint, 220i32 as libc::c_uint, 217i32 as libc::c_uint,
     215i32 as libc::c_uint, 212i32 as libc::c_uint, 209i32 as libc::c_uint,
     207i32 as libc::c_uint, 204i32 as libc::c_uint, 202i32 as libc::c_uint,
     199i32 as libc::c_uint, 197i32 as libc::c_uint, 194i32 as libc::c_uint,
     192i32 as libc::c_uint, 190i32 as libc::c_uint, 187i32 as libc::c_uint,
     185i32 as libc::c_uint, 182i32 as libc::c_uint, 180i32 as libc::c_uint,
     178i32 as libc::c_uint, 175i32 as libc::c_uint, 173i32 as libc::c_uint,
     171i32 as libc::c_uint, 168i32 as libc::c_uint, 166i32 as libc::c_uint,
     164i32 as libc::c_uint, 162i32 as libc::c_uint, 159i32 as libc::c_uint,
     157i32 as libc::c_uint, 155i32 as libc::c_uint, 153i32 as libc::c_uint,
     151i32 as libc::c_uint, 149i32 as libc::c_uint, 146i32 as libc::c_uint,
     144i32 as libc::c_uint, 142i32 as libc::c_uint, 140i32 as libc::c_uint,
     138i32 as libc::c_uint, 136i32 as libc::c_uint, 134i32 as libc::c_uint,
     132i32 as libc::c_uint, 130i32 as libc::c_uint, 128i32 as libc::c_uint,
     126i32 as libc::c_uint, 123i32 as libc::c_uint, 121i32 as libc::c_uint,
     119i32 as libc::c_uint, 117i32 as libc::c_uint, 115i32 as libc::c_uint,
     114i32 as libc::c_uint, 112i32 as libc::c_uint, 110i32 as libc::c_uint,
     108i32 as libc::c_uint, 106i32 as libc::c_uint, 104i32 as libc::c_uint,
     102i32 as libc::c_uint, 100i32 as libc::c_uint, 98i32 as libc::c_uint,
     96i32 as libc::c_uint, 94i32 as libc::c_uint, 93i32 as libc::c_uint,
     91i32 as libc::c_uint, 89i32 as libc::c_uint, 87i32 as libc::c_uint,
     85i32 as libc::c_uint, 83i32 as libc::c_uint, 82i32 as libc::c_uint,
     80i32 as libc::c_uint, 78i32 as libc::c_uint, 76i32 as libc::c_uint,
     74i32 as libc::c_uint, 73i32 as libc::c_uint, 71i32 as libc::c_uint,
     69i32 as libc::c_uint, 67i32 as libc::c_uint, 66i32 as libc::c_uint,
     64i32 as libc::c_uint, 62i32 as libc::c_uint, 61i32 as libc::c_uint,
     59i32 as libc::c_uint, 57i32 as libc::c_uint, 55i32 as libc::c_uint,
     54i32 as libc::c_uint, 52i32 as libc::c_uint, 50i32 as libc::c_uint,
     49i32 as libc::c_uint, 47i32 as libc::c_uint, 46i32 as libc::c_uint,
     44i32 as libc::c_uint, 42i32 as libc::c_uint, 41i32 as libc::c_uint,
     39i32 as libc::c_uint, 37i32 as libc::c_uint, 36i32 as libc::c_uint,
     34i32 as libc::c_uint, 33i32 as libc::c_uint, 31i32 as libc::c_uint,
     30i32 as libc::c_uint, 28i32 as libc::c_uint, 26i32 as libc::c_uint,
     25i32 as libc::c_uint, 23i32 as libc::c_uint, 22i32 as libc::c_uint,
     20i32 as libc::c_uint, 19i32 as libc::c_uint, 17i32 as libc::c_uint,
     16i32 as libc::c_uint, 14i32 as libc::c_uint, 13i32 as libc::c_uint,
     11i32 as libc::c_uint, 10i32 as libc::c_uint, 8i32 as libc::c_uint,
     7i32 as libc::c_uint, 5i32 as libc::c_uint, 4i32 as libc::c_uint,
     2i32 as libc::c_uint, 1i32 as libc::c_uint];
/* *
 * Returns the cost in bytes of encoding the normalized count header.
 * Returns an error if any of the helper functions return an error.
 */
unsafe extern "C" fn ZSTD_NCountCost(mut count: *const libc::c_uint,
                                     max: libc::c_uint, nbSeq: size_t,
                                     FSELog: libc::c_uint) -> size_t {
    let mut wksp: [BYTE; 512] = [0; 512];
    let mut norm: [S16; 53] = [0; 53];
    let tableLog: U32 = FSE_optimalTableLog(FSELog, nbSeq, max);
    let errcod: size_t =
        FSE_normalizeCount(norm.as_mut_ptr(), tableLog, count, nbSeq, max);
    if 0 != ERR_isError(errcod) { return errcod }
    return FSE_writeNCount(wksp.as_mut_ptr() as *mut libc::c_void,
                           ::std::mem::size_of::<[BYTE; 512]>() as
                               libc::c_ulong, norm.as_mut_ptr(), max,
                           tableLog);
}
/* *
 * Returns the cost in bits of encoding the distribution in count using ctable.
 * Returns an error if ctable cannot represent all the symbols in count.
 */
unsafe extern "C" fn ZSTD_fseBitCost(mut ctable: *const FSE_CTable,
                                     mut count: *const libc::c_uint,
                                     max: libc::c_uint) -> size_t {
    let kAccuracyLog: libc::c_uint = 8i32 as libc::c_uint;
    let mut cost: size_t = 0i32 as size_t;
    let mut s: libc::c_uint = 0;
    let mut cstate: FSE_CState_t =
        FSE_CState_t{value: 0,
                     stateTable: 0 as *const libc::c_void,
                     symbolTT: 0 as *const libc::c_void,
                     stateLog: 0,};
    FSE_initCState(&mut cstate, ctable);
    if ZSTD_getFSEMaxSymbolValue(ctable) < max {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    s = 0i32 as libc::c_uint;
    while s <= max {
        let tableLog: libc::c_uint = cstate.stateLog;
        let badCost: libc::c_uint =
            tableLog.wrapping_add(1i32 as libc::c_uint) << kAccuracyLog;
        let bitCost: libc::c_uint =
            FSE_bitCost(cstate.symbolTT, tableLog, s, kAccuracyLog);
        if !(*count.offset(s as isize) == 0i32 as libc::c_uint) {
            if bitCost >= badCost {
                return -(ZSTD_error_GENERIC as libc::c_int) as size_t
            }
            cost =
                (cost as
                     libc::c_ulong).wrapping_add((*count.offset(s as
                                                                    isize)).wrapping_mul(bitCost)
                                                     as libc::c_ulong) as
                    size_t as size_t
        }
        s = s.wrapping_add(1)
    }
    return cost >> kAccuracyLog;
}
unsafe extern "C" fn ZSTD_getFSEMaxSymbolValue(mut ctable: *const FSE_CTable)
 -> libc::c_uint {
    let mut ptr: *const libc::c_void = ctable as *const libc::c_void;
    let mut u16ptr: *const U16 = ptr as *const U16;
    let maxSymbolValue: U32 =
        MEM_read16(u16ptr.offset(1isize) as *const libc::c_void) as U32;
    return maxSymbolValue;
}
/* *
 * Returns the cost in bits of encoding the distribution in count using the
 * table described by norm. The max symbol support by norm is assumed >= max.
 * norm must be valid for every symbol with non-zero probability in count.
 */
unsafe extern "C" fn ZSTD_crossEntropyCost(mut norm: *const libc::c_short,
                                           mut accuracyLog: libc::c_uint,
                                           mut count: *const libc::c_uint,
                                           max: libc::c_uint) -> size_t {
    let shift: libc::c_uint =
        (8i32 as libc::c_uint).wrapping_sub(accuracyLog);
    let mut cost: size_t = 0i32 as size_t;
    let mut s: libc::c_uint = 0;
    s = 0i32 as libc::c_uint;
    while s <= max {
        let normAcc: libc::c_uint =
            (if *norm.offset(s as isize) as libc::c_int != -1i32 {
                 *norm.offset(s as isize) as libc::c_int
             } else { 1i32 }) as libc::c_uint;
        let norm256: libc::c_uint = normAcc << shift;
        cost =
            (cost as
                 libc::c_ulong).wrapping_add((*count.offset(s as
                                                                isize)).wrapping_mul(kInverseProbabiltyLog256[norm256
                                                                                                                  as
                                                                                                                  usize])
                                                 as libc::c_ulong) as size_t
                as size_t;
        s = s.wrapping_add(1)
    }
    return cost >> 8i32;
}
/* for static allocation */
static mut OF_defaultNormLog: U32 = 5i32 as U32;
static mut OF_defaultNorm: [S16; 29] =
    [1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
     1i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16, 1i32 as S16,
     1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
     1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
     1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, -1i32 as S16,
     -1i32 as S16, -1i32 as S16, -1i32 as S16, -1i32 as S16];
/* for static allocation */
static mut LL_defaultNormLog: U32 = 6i32 as U32;
static mut LL_defaultNorm: [S16; 36] =
    [4i32 as S16, 3i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16,
     2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16,
     2i32 as S16, 2i32 as S16, 2i32 as S16, 1i32 as S16, 1i32 as S16,
     1i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16,
     2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16,
     3i32 as S16, 2i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
     1i32 as S16, 1i32 as S16, -1i32 as S16, -1i32 as S16, -1i32 as S16,
     -1i32 as S16];
/* compress, dictBuilder, decodeCorpus (shouldn't get its definition from here) */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_seqToCodes(mut seqStorePtr: *const seqStore_t) {
    let sequences: *const seqDef = (*seqStorePtr).sequencesStart;
    let llCodeTable: *mut BYTE = (*seqStorePtr).llCode;
    let ofCodeTable: *mut BYTE = (*seqStorePtr).ofCode;
    let mlCodeTable: *mut BYTE = (*seqStorePtr).mlCode;
    let nbSeq: U32 =
        (*seqStorePtr).sequences.wrapping_offset_from((*seqStorePtr).sequencesStart)
            as libc::c_long as U32;
    let mut u: U32 = 0;
    u = 0i32 as U32;
    while u < nbSeq {
        let llv: U32 = (*sequences.offset(u as isize)).litLength as U32;
        let mlv: U32 = (*sequences.offset(u as isize)).matchLength as U32;
        *llCodeTable.offset(u as isize) = ZSTD_LLcode(llv) as BYTE;
        *ofCodeTable.offset(u as isize) =
            ZSTD_highbit32((*sequences.offset(u as isize)).offset) as BYTE;
        *mlCodeTable.offset(u as isize) = ZSTD_MLcode(mlv) as BYTE;
        u = u.wrapping_add(1)
    }
    if (*seqStorePtr).longLengthID == 1i32 as libc::c_uint {
        *llCodeTable.offset((*seqStorePtr).longLengthPos as isize) =
            35i32 as BYTE
    }
    if (*seqStorePtr).longLengthID == 2i32 as libc::c_uint {
        *mlCodeTable.offset((*seqStorePtr).longLengthPos as isize) =
            52i32 as BYTE
    };
}
/* ZSTD_MLcode() :
 * note : mlBase = matchLength - MINMATCH;
 *        because it's the format it's stored in seqStore->sequences */
unsafe extern "C" fn ZSTD_MLcode(mut mlBase: U32) -> U32 {
    static mut ML_Code: [BYTE; 128] =
        [0i32 as BYTE, 1i32 as BYTE, 2i32 as BYTE, 3i32 as BYTE, 4i32 as BYTE,
         5i32 as BYTE, 6i32 as BYTE, 7i32 as BYTE, 8i32 as BYTE, 9i32 as BYTE,
         10i32 as BYTE, 11i32 as BYTE, 12i32 as BYTE, 13i32 as BYTE,
         14i32 as BYTE, 15i32 as BYTE, 16i32 as BYTE, 17i32 as BYTE,
         18i32 as BYTE, 19i32 as BYTE, 20i32 as BYTE, 21i32 as BYTE,
         22i32 as BYTE, 23i32 as BYTE, 24i32 as BYTE, 25i32 as BYTE,
         26i32 as BYTE, 27i32 as BYTE, 28i32 as BYTE, 29i32 as BYTE,
         30i32 as BYTE, 31i32 as BYTE, 32i32 as BYTE, 32i32 as BYTE,
         33i32 as BYTE, 33i32 as BYTE, 34i32 as BYTE, 34i32 as BYTE,
         35i32 as BYTE, 35i32 as BYTE, 36i32 as BYTE, 36i32 as BYTE,
         36i32 as BYTE, 36i32 as BYTE, 37i32 as BYTE, 37i32 as BYTE,
         37i32 as BYTE, 37i32 as BYTE, 38i32 as BYTE, 38i32 as BYTE,
         38i32 as BYTE, 38i32 as BYTE, 38i32 as BYTE, 38i32 as BYTE,
         38i32 as BYTE, 38i32 as BYTE, 39i32 as BYTE, 39i32 as BYTE,
         39i32 as BYTE, 39i32 as BYTE, 39i32 as BYTE, 39i32 as BYTE,
         39i32 as BYTE, 39i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
         40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
         40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
         40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE, 40i32 as BYTE,
         40i32 as BYTE, 40i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
         41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
         41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
         41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE, 41i32 as BYTE,
         41i32 as BYTE, 41i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE, 42i32 as BYTE,
         42i32 as BYTE, 42i32 as BYTE];
    static mut ML_deltaCode: U32 = 36i32 as U32;
    return if mlBase > 127i32 as libc::c_uint {
               ZSTD_highbit32(mlBase).wrapping_add(ML_deltaCode)
           } else { ML_Code[mlBase as usize] as libc::c_uint };
}
unsafe extern "C" fn ZSTD_LLcode(mut litLength: U32) -> U32 {
    static mut LL_Code: [BYTE; 64] =
        [0i32 as BYTE, 1i32 as BYTE, 2i32 as BYTE, 3i32 as BYTE, 4i32 as BYTE,
         5i32 as BYTE, 6i32 as BYTE, 7i32 as BYTE, 8i32 as BYTE, 9i32 as BYTE,
         10i32 as BYTE, 11i32 as BYTE, 12i32 as BYTE, 13i32 as BYTE,
         14i32 as BYTE, 15i32 as BYTE, 16i32 as BYTE, 16i32 as BYTE,
         17i32 as BYTE, 17i32 as BYTE, 18i32 as BYTE, 18i32 as BYTE,
         19i32 as BYTE, 19i32 as BYTE, 20i32 as BYTE, 20i32 as BYTE,
         20i32 as BYTE, 20i32 as BYTE, 21i32 as BYTE, 21i32 as BYTE,
         21i32 as BYTE, 21i32 as BYTE, 22i32 as BYTE, 22i32 as BYTE,
         22i32 as BYTE, 22i32 as BYTE, 22i32 as BYTE, 22i32 as BYTE,
         22i32 as BYTE, 22i32 as BYTE, 23i32 as BYTE, 23i32 as BYTE,
         23i32 as BYTE, 23i32 as BYTE, 23i32 as BYTE, 23i32 as BYTE,
         23i32 as BYTE, 23i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE,
         24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE,
         24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE,
         24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE, 24i32 as BYTE,
         24i32 as BYTE, 24i32 as BYTE];
    static mut LL_deltaCode: U32 = 19i32 as U32;
    return if litLength > 63i32 as libc::c_uint {
               ZSTD_highbit32(litLength).wrapping_add(LL_deltaCode)
           } else { LL_Code[litLength as usize] as libc::c_uint };
}
unsafe extern "C" fn ZSTD_compressLiterals(mut prevHuf:
                                               *const ZSTD_hufCTables_t,
                                           mut nextHuf:
                                               *mut ZSTD_hufCTables_t,
                                           mut strategy: ZSTD_strategy,
                                           mut disableLiteralCompression:
                                               libc::c_int,
                                           mut dst: *mut libc::c_void,
                                           mut dstCapacity: size_t,
                                           mut src: *const libc::c_void,
                                           mut srcSize: size_t,
                                           mut workspace: *mut libc::c_void,
                                           mut wkspSize: size_t,
                                           bmi2: libc::c_int) -> size_t {
    let minGain: size_t = ZSTD_minGain(srcSize, strategy);
    let lhSize: size_t =
        (3i32 +
             (srcSize >= (1i32 * (1i32 << 10i32)) as libc::c_ulong) as
                 libc::c_int +
             (srcSize >= (16i32 * (1i32 << 10i32)) as libc::c_ulong) as
                 libc::c_int) as size_t;
    let ostart: *mut BYTE = dst as *mut BYTE;
    let mut singleStream: U32 =
        (srcSize < 256i32 as libc::c_ulong) as libc::c_int as U32;
    let mut hType: symbolEncodingType_e = set_compressed;
    let mut cLitSize: size_t = 0;
    memcpy(nextHuf as *mut libc::c_void, prevHuf as *const libc::c_void,
           ::std::mem::size_of::<ZSTD_hufCTables_t>() as libc::c_ulong);
    if 0 != disableLiteralCompression {
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize)
    }
    let minLitSize: size_t =
        (if (*prevHuf).repeatMode as libc::c_uint ==
                HUF_repeat_valid as libc::c_int as libc::c_uint {
             6i32
         } else { 63i32 }) as size_t;
    if srcSize <= minLitSize {
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize)
    }
    if dstCapacity < lhSize.wrapping_add(1i32 as libc::c_ulong) {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    let mut repeat: HUF_repeat = (*prevHuf).repeatMode;
    let preferRepeat: libc::c_int =
        if (strategy as libc::c_uint) <
               ZSTD_lazy as libc::c_int as libc::c_uint {
            (srcSize <= 1024i32 as libc::c_ulong) as libc::c_int
        } else { 0i32 };
    if repeat as libc::c_uint ==
           HUF_repeat_valid as libc::c_int as libc::c_uint &&
           lhSize == 3i32 as libc::c_ulong {
        singleStream = 1i32 as U32
    }
    cLitSize =
        if 0 != singleStream {
            HUF_compress1X_repeat(ostart.offset(lhSize as isize) as
                                      *mut libc::c_void,
                                  dstCapacity.wrapping_sub(lhSize), src,
                                  srcSize, 255i32 as libc::c_uint,
                                  11i32 as libc::c_uint, workspace, wkspSize,
                                  (*nextHuf).CTable.as_mut_ptr() as
                                      *mut HUF_CElt, &mut repeat,
                                  preferRepeat, bmi2)
        } else {
            HUF_compress4X_repeat(ostart.offset(lhSize as isize) as
                                      *mut libc::c_void,
                                  dstCapacity.wrapping_sub(lhSize), src,
                                  srcSize, 255i32 as libc::c_uint,
                                  11i32 as libc::c_uint, workspace, wkspSize,
                                  (*nextHuf).CTable.as_mut_ptr() as
                                      *mut HUF_CElt, &mut repeat,
                                  preferRepeat, bmi2)
        };
    if repeat as libc::c_uint !=
           HUF_repeat_none as libc::c_int as libc::c_uint {
        hType = set_repeat
    }
    if 0 !=
           ((cLitSize == 0i32 as libc::c_ulong) as libc::c_int |
                (cLitSize >= srcSize.wrapping_sub(minGain)) as libc::c_int) as
               libc::c_uint | ERR_isError(cLitSize) {
        memcpy(nextHuf as *mut libc::c_void, prevHuf as *const libc::c_void,
               ::std::mem::size_of::<ZSTD_hufCTables_t>() as libc::c_ulong);
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize)
    }
    if cLitSize == 1i32 as libc::c_ulong {
        memcpy(nextHuf as *mut libc::c_void, prevHuf as *const libc::c_void,
               ::std::mem::size_of::<ZSTD_hufCTables_t>() as libc::c_ulong);
        return ZSTD_compressRleLiteralsBlock(dst, dstCapacity, src, srcSize)
    }
    if hType as libc::c_uint == set_compressed as libc::c_int as libc::c_uint
       {
        (*nextHuf).repeatMode = HUF_repeat_check
    }
    match lhSize {
        3 => {
            let lhc: U32 =
                (hType as
                     libc::c_uint).wrapping_add((((0 == singleStream) as
                                                      libc::c_int) << 2i32) as
                                                    libc::c_uint).wrapping_add((srcSize
                                                                                    as
                                                                                    U32)
                                                                                   <<
                                                                                   4i32).wrapping_add((cLitSize
                                                                                                           as
                                                                                                           U32)
                                                                                                          <<
                                                                                                          14i32);
            MEM_writeLE24(ostart as *mut libc::c_void, lhc);
        }
        4 => {
            let lhc_0: U32 =
                (hType as
                     libc::c_uint).wrapping_add((2i32 << 2i32) as
                                                    libc::c_uint).wrapping_add((srcSize
                                                                                    as
                                                                                    U32)
                                                                                   <<
                                                                                   4i32).wrapping_add((cLitSize
                                                                                                           as
                                                                                                           U32)
                                                                                                          <<
                                                                                                          18i32);
            MEM_writeLE32(ostart as *mut libc::c_void, lhc_0);
        }
        5 => {
            let lhc_1: U32 =
                (hType as
                     libc::c_uint).wrapping_add((3i32 << 2i32) as
                                                    libc::c_uint).wrapping_add((srcSize
                                                                                    as
                                                                                    U32)
                                                                                   <<
                                                                                   4i32).wrapping_add((cLitSize
                                                                                                           as
                                                                                                           U32)
                                                                                                          <<
                                                                                                          22i32);
            MEM_writeLE32(ostart as *mut libc::c_void, lhc_1);
            *ostart.offset(4isize) = (cLitSize >> 10i32) as BYTE
        }
        _ => { }
    }
    return lhSize.wrapping_add(cLitSize);
}
unsafe extern "C" fn ZSTD_compressRleLiteralsBlock(mut dst: *mut libc::c_void,
                                                   mut dstCapacity: size_t,
                                                   mut src:
                                                       *const libc::c_void,
                                                   mut srcSize: size_t)
 -> size_t {
    let ostart: *mut BYTE = dst as *mut BYTE;
    let flSize: U32 =
        (1i32 + (srcSize > 31i32 as libc::c_ulong) as libc::c_int +
             (srcSize > 4095i32 as libc::c_ulong) as libc::c_int) as U32;
    match flSize {
        1 => {
            *ostart.offset(0isize) =
                (set_rle as libc::c_int as U32 as
                     libc::c_ulong).wrapping_add(srcSize << 3i32) as BYTE
        }
        2 => {
            MEM_writeLE16(ostart as *mut libc::c_void,
                          ((set_rle as libc::c_int as
                                U32).wrapping_add((1i32 << 2i32) as
                                                      libc::c_uint) as
                               libc::c_ulong).wrapping_add(srcSize << 4i32) as
                              U16);
        }
        3 => {
            MEM_writeLE32(ostart as *mut libc::c_void,
                          ((set_rle as libc::c_int as
                                U32).wrapping_add((3i32 << 2i32) as
                                                      libc::c_uint) as
                               libc::c_ulong).wrapping_add(srcSize << 4i32) as
                              U32);
        }
        _ => { }
    }
    *ostart.offset(flSize as isize) = *(src as *const BYTE);
    return flSize.wrapping_add(1i32 as libc::c_uint) as size_t;
}
unsafe extern "C" fn ZSTD_noCompressLiterals(mut dst: *mut libc::c_void,
                                             mut dstCapacity: size_t,
                                             mut src: *const libc::c_void,
                                             mut srcSize: size_t) -> size_t {
    let ostart: *mut BYTE = dst as *mut BYTE;
    let flSize: U32 =
        (1i32 + (srcSize > 31i32 as libc::c_ulong) as libc::c_int +
             (srcSize > 4095i32 as libc::c_ulong) as libc::c_int) as U32;
    if srcSize.wrapping_add(flSize as libc::c_ulong) > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    match flSize {
        1 => {
            *ostart.offset(0isize) =
                (set_basic as libc::c_int as U32 as
                     libc::c_ulong).wrapping_add(srcSize << 3i32) as BYTE
        }
        2 => {
            MEM_writeLE16(ostart as *mut libc::c_void,
                          ((set_basic as libc::c_int as
                                U32).wrapping_add((1i32 << 2i32) as
                                                      libc::c_uint) as
                               libc::c_ulong).wrapping_add(srcSize << 4i32) as
                              U16);
        }
        3 => {
            MEM_writeLE32(ostart as *mut libc::c_void,
                          ((set_basic as libc::c_int as
                                U32).wrapping_add((3i32 << 2i32) as
                                                      libc::c_uint) as
                               libc::c_ulong).wrapping_add(srcSize << 4i32) as
                              U32);
        }
        _ => { }
    }
    memcpy(ostart.offset(flSize as isize) as *mut libc::c_void, src, srcSize);
    return srcSize.wrapping_add(flSize as libc::c_ulong);
}
/* ZSTD_minGain() :
 * minimum compression required
 * to generate a compress block or a compressed literals section.
 * note : use same formula for both situations */
unsafe extern "C" fn ZSTD_minGain(mut srcSize: size_t,
                                  mut strat: ZSTD_strategy) -> size_t {
    let minlog: U32 =
        if strat as libc::c_uint >=
               ZSTD_btultra as libc::c_int as libc::c_uint {
            (strat as U32).wrapping_sub(1i32 as libc::c_uint)
        } else { 6i32 as libc::c_uint };
    return (srcSize >> minlog).wrapping_add(2i32 as libc::c_ulong);
}
unsafe extern "C" fn ZSTD_storeLastLiterals(mut seqStorePtr: *mut seqStore_t,
                                            mut anchor: *const BYTE,
                                            mut lastLLSize: size_t) {
    memcpy((*seqStorePtr).lit as *mut libc::c_void,
           anchor as *const libc::c_void, lastLLSize);
    (*seqStorePtr).lit = (*seqStorePtr).lit.offset(lastLLSize as isize);
}
/* *
 * ZSTD_matchState_dictMode():
 * Inspects the provided matchState and figures out what dictMode should be
 * passed to the compressor.
 */
unsafe extern "C" fn ZSTD_matchState_dictMode(mut ms:
                                                  *const ZSTD_matchState_t)
 -> ZSTD_dictMode_e {
    return (if 0 != ZSTD_window_hasExtDict((*ms).window) {
                ZSTD_extDict as libc::c_int
            } else if !(*ms).dictMatchState.is_null() {
                ZSTD_dictMatchState as libc::c_int
            } else { ZSTD_noDict as libc::c_int }) as ZSTD_dictMode_e;
}
/* *
 * ZSTD_window_hasExtDict():
 * Returns non-zero if the window has a non-empty extDict.
 */
unsafe extern "C" fn ZSTD_window_hasExtDict(window: ZSTD_window_t) -> U32 {
    return (window.lowLimit < window.dictLimit) as libc::c_int as U32;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_selectBlockCompressor(mut strat: ZSTD_strategy,
                                                    mut dictMode:
                                                        ZSTD_dictMode_e)
 -> ZSTD_blockCompressor {
    static mut blockCompressor: [[ZSTD_blockCompressor; 10]; 3] =
        [[Some(ZSTD_compressBlock_fast), Some(ZSTD_compressBlock_fast),
          Some(ZSTD_compressBlock_doubleFast),
          Some(ZSTD_compressBlock_greedy), Some(ZSTD_compressBlock_lazy),
          Some(ZSTD_compressBlock_lazy2), Some(ZSTD_compressBlock_btlazy2),
          Some(ZSTD_compressBlock_btopt), Some(ZSTD_compressBlock_btultra),
          Some(ZSTD_compressBlock_btultra2)],
         [Some(ZSTD_compressBlock_fast_extDict),
          Some(ZSTD_compressBlock_fast_extDict),
          Some(ZSTD_compressBlock_doubleFast_extDict),
          Some(ZSTD_compressBlock_greedy_extDict),
          Some(ZSTD_compressBlock_lazy_extDict),
          Some(ZSTD_compressBlock_lazy2_extDict),
          Some(ZSTD_compressBlock_btlazy2_extDict),
          Some(ZSTD_compressBlock_btopt_extDict),
          Some(ZSTD_compressBlock_btultra_extDict),
          Some(ZSTD_compressBlock_btultra_extDict)],
         [Some(ZSTD_compressBlock_fast_dictMatchState),
          Some(ZSTD_compressBlock_fast_dictMatchState),
          Some(ZSTD_compressBlock_doubleFast_dictMatchState),
          Some(ZSTD_compressBlock_greedy_dictMatchState),
          Some(ZSTD_compressBlock_lazy_dictMatchState),
          Some(ZSTD_compressBlock_lazy2_dictMatchState),
          Some(ZSTD_compressBlock_btlazy2_dictMatchState),
          Some(ZSTD_compressBlock_btopt_dictMatchState),
          Some(ZSTD_compressBlock_btultra_dictMatchState),
          Some(ZSTD_compressBlock_btultra_dictMatchState)]];
    /* default for 0 */
    /* default for 0 */
    /* default for 0 */
    let mut selectedCompressor: ZSTD_blockCompressor = None;
    selectedCompressor =
        blockCompressor[dictMode as libc::c_int as
                            usize][strat as libc::c_int as usize];
    return selectedCompressor;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_resetSeqStore(mut ssPtr: *mut seqStore_t) {
    (*ssPtr).lit = (*ssPtr).litStart;
    (*ssPtr).sequences = (*ssPtr).sequencesStart;
    (*ssPtr).longLengthID = 0i32 as U32;
}
/* magic number size */
/* C standard doesn't allow `static const` variable to be init using another `static const` variable */
static mut ZSTD_blockHeaderSize: size_t = 3i32 as size_t;
/* opt parser space */
/* hashlog3 space */
unsafe extern "C" fn ZSTD_assertEqualCParams(mut cParams1:
                                                 ZSTD_compressionParameters,
                                             mut cParams2:
                                                 ZSTD_compressionParameters) {
}
/* ! ZSTD_compress_frameChunk() :
*   Compress a chunk of data into one or multiple blocks.
*   All blocks will be terminated, all input will be consumed.
*   Function will issue an error if there is not enough `dstCapacity` to hold the compressed content.
*   Frame is supposed already started (header already produced)
*   @return : compressed size, or an error code
*/
unsafe extern "C" fn ZSTD_compress_frameChunk(mut cctx: *mut ZSTD_CCtx,
                                              mut dst: *mut libc::c_void,
                                              mut dstCapacity: size_t,
                                              mut src: *const libc::c_void,
                                              mut srcSize: size_t,
                                              mut lastFrameChunk: U32)
 -> size_t {
    let mut blockSize: size_t = (*cctx).blockSize;
    let mut remaining: size_t = srcSize;
    let mut ip: *const BYTE = src as *const BYTE;
    let ostart: *mut BYTE = dst as *mut BYTE;
    let mut op: *mut BYTE = ostart;
    let maxDist: U32 =
        (1i32 as U32) << (*cctx).appliedParams.cParams.windowLog;
    if 0 != (*cctx).appliedParams.fParams.checksumFlag && 0 != srcSize {
        ZSTD_XXH64_update(&mut (*cctx).xxhState, src, srcSize);
    }
    while 0 != remaining {
        let ms: *mut ZSTD_matchState_t = &mut (*cctx).blockState.matchState;
        let lastBlock: U32 =
            lastFrameChunk &
                (blockSize >= remaining) as libc::c_int as libc::c_uint;
        if dstCapacity <
               ZSTD_blockHeaderSize.wrapping_add((1i32 + 1i32 + 1i32) as
                                                     libc::c_ulong) {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
        }
        if remaining < blockSize { blockSize = remaining }
        if 0 !=
               ZSTD_window_needOverflowCorrection((*ms).window,
                                                  ip.offset(blockSize as
                                                                isize) as
                                                      *const libc::c_void) {
            let cycleLog: U32 =
                ZSTD_cycleLog((*cctx).appliedParams.cParams.chainLog,
                              (*cctx).appliedParams.cParams.strategy);
            let correction: U32 =
                ZSTD_window_correctOverflow(&mut (*ms).window, cycleLog,
                                            maxDist,
                                            ip as *const libc::c_void);
            ZSTD_reduceIndex(cctx, correction);
            if (*ms).nextToUpdate < correction {
                (*ms).nextToUpdate = 0i32 as U32
            } else {
                (*ms).nextToUpdate =
                    ((*ms).nextToUpdate as
                         libc::c_uint).wrapping_sub(correction) as U32 as U32
            }
            (*ms).loadedDictEnd = 0i32 as U32;
            (*ms).dictMatchState = 0 as *const ZSTD_matchState_t
        }
        ZSTD_window_enforceMaxDist(&mut (*ms).window,
                                   ip.offset(blockSize as isize) as
                                       *const libc::c_void, maxDist,
                                   &mut (*ms).loadedDictEnd,
                                   &mut (*ms).dictMatchState);
        if (*ms).nextToUpdate < (*ms).window.lowLimit {
            (*ms).nextToUpdate = (*ms).window.lowLimit
        }
        let mut cSize: size_t =
            ZSTD_compressBlock_internal(cctx,
                                        op.offset(ZSTD_blockHeaderSize as
                                                      isize) as
                                            *mut libc::c_void,
                                        dstCapacity.wrapping_sub(ZSTD_blockHeaderSize),
                                        ip as *const libc::c_void, blockSize);
        if 0 != ERR_isError(cSize) { return cSize }
        if cSize == 0i32 as libc::c_ulong {
            cSize =
                ZSTD_noCompressBlock(op as *mut libc::c_void, dstCapacity,
                                     ip as *const libc::c_void, blockSize,
                                     lastBlock);
            if 0 != ERR_isError(cSize) { return cSize }
        } else {
            let cBlockHeader24: U32 =
                lastBlock.wrapping_add((bt_compressed as libc::c_int as U32)
                                           <<
                                           1i32).wrapping_add((cSize << 3i32)
                                                                  as U32);
            MEM_writeLE24(op as *mut libc::c_void, cBlockHeader24);
            cSize =
                (cSize as libc::c_ulong).wrapping_add(ZSTD_blockHeaderSize) as
                    size_t as size_t
        }
        ip = ip.offset(blockSize as isize);
        remaining =
            (remaining as libc::c_ulong).wrapping_sub(blockSize) as size_t as
                size_t;
        op = op.offset(cSize as isize);
        dstCapacity =
            (dstCapacity as libc::c_ulong).wrapping_sub(cSize) as size_t as
                size_t
    }
    if 0 != lastFrameChunk && op > ostart { (*cctx).stage = ZSTDcs_ending }
    return op.wrapping_offset_from(ostart) as libc::c_long as size_t;
}
/*-*******************************************************
*  Block entropic compression
*********************************************************/
/* See doc/zstd_compression_format.md for detailed format description */
unsafe extern "C" fn ZSTD_noCompressBlock(mut dst: *mut libc::c_void,
                                          mut dstCapacity: size_t,
                                          mut src: *const libc::c_void,
                                          mut srcSize: size_t,
                                          mut lastBlock: U32) -> size_t {
    let cBlockHeader24: U32 =
        lastBlock.wrapping_add((bt_raw as libc::c_int as U32) <<
                                   1i32).wrapping_add((srcSize << 3i32) as
                                                          U32);
    if srcSize.wrapping_add(ZSTD_blockHeaderSize) > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    MEM_writeLE24(dst, cBlockHeader24);
    memcpy((dst as *mut BYTE).offset(ZSTD_blockHeaderSize as isize) as
               *mut libc::c_void, src, srcSize);
    return ZSTD_blockHeaderSize.wrapping_add(srcSize);
}
/* *
 * ZSTD_window_enforceMaxDist():
 * Updates lowLimit so that:
 *    (srcEnd - base) - lowLimit == maxDist + loadedDictEnd
 *
 * This allows a simple check that index >= lowLimit to see if index is valid.
 * This must be called before a block compression call, with srcEnd as the block
 * source end.
 *
 * If loadedDictEndPtr is not NULL, we set it to zero once we update lowLimit.
 * This is because dictionaries are allowed to be referenced as long as the last
 * byte of the dictionary is in the window, but once they are out of range,
 * they cannot be referenced. If loadedDictEndPtr is NULL, we use
 * loadedDictEnd == 0.
 *
 * In normal dict mode, the dict is between lowLimit and dictLimit. In
 * dictMatchState mode, lowLimit and dictLimit are the same, and the dictionary
 * is below them. forceWindow and dictMatchState are therefore incompatible.
 */
unsafe extern "C" fn ZSTD_window_enforceMaxDist(mut window:
                                                    *mut ZSTD_window_t,
                                                mut srcEnd:
                                                    *const libc::c_void,
                                                mut maxDist: U32,
                                                mut loadedDictEndPtr:
                                                    *mut U32,
                                                mut dictMatchStatePtr:
                                                    *mut *const ZSTD_matchState_t) {
    let blockEndIdx: U32 =
        (srcEnd as *const BYTE).wrapping_offset_from((*window).base) as
            libc::c_long as U32;
    let mut loadedDictEnd: U32 =
        if !loadedDictEndPtr.is_null() {
            *loadedDictEndPtr
        } else { 0i32 as libc::c_uint };
    if blockEndIdx > maxDist.wrapping_add(loadedDictEnd) {
        let newLowLimit: U32 = blockEndIdx.wrapping_sub(maxDist);
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
/* *
 * ZSTD_window_correctOverflow():
 * Reduces the indices to protect from index overflow.
 * Returns the correction made to the indices, which must be applied to every
 * stored index.
 *
 * The least significant cycleLog bits of the indices must remain the same,
 * which may be 0. Every index up to maxDist in the past must be valid.
 * NOTE: (maxDist & cycleMask) must be zero.
 */
unsafe extern "C" fn ZSTD_window_correctOverflow(mut window:
                                                     *mut ZSTD_window_t,
                                                 mut cycleLog: U32,
                                                 mut maxDist: U32,
                                                 mut src: *const libc::c_void)
 -> U32 {
    /* preemptive overflow correction:
     * 1. correction is large enough:
     *    lowLimit > (3<<29) ==> current > 3<<29 + 1<<windowLog
     *    1<<windowLog <= newCurrent < 1<<chainLog + 1<<windowLog
     *
     *    current - newCurrent
     *    > (3<<29 + 1<<windowLog) - (1<<windowLog + 1<<chainLog)
     *    > (3<<29) - (1<<chainLog)
     *    > (3<<29) - (1<<30)             (NOTE: chainLog <= 30)
     *    > 1<<29
     *
     * 2. (ip+ZSTD_CHUNKSIZE_MAX - cctx->base) doesn't overflow:
     *    After correction, current is less than (1<<chainLog + 1<<windowLog).
     *    In 64-bit mode we are safe, because we have 64-bit ptrdiff_t.
     *    In 32-bit mode we are safe, because (chainLog <= 29), so
     *    ip+ZSTD_CHUNKSIZE_MAX - cctx->base < 1<<32.
     * 3. (cctx->lowLimit + 1<<windowLog) < 1<<32:
     *    windowLog <= 31 ==> 3<<29 + 1<<windowLog < 7<<29 < 1<<32.
     */
    let cycleMask: U32 =
        (1u32 << cycleLog).wrapping_sub(1i32 as libc::c_uint);
    let current: U32 =
        (src as *const BYTE).wrapping_offset_from((*window).base) as
            libc::c_long as U32;
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
/* ! ZSTD_reduceIndex() :
*   rescale all indexes to avoid future overflow (indexes are U32) */
unsafe extern "C" fn ZSTD_reduceIndex(mut zc: *mut ZSTD_CCtx,
                                      reducerValue: U32) {
    let ms: *mut ZSTD_matchState_t = &mut (*zc).blockState.matchState;
    let hSize: U32 = (1i32 as U32) << (*zc).appliedParams.cParams.hashLog;
    ZSTD_reduceTable((*ms).hashTable, hSize, reducerValue);
    if (*zc).appliedParams.cParams.strategy as libc::c_uint !=
           ZSTD_fast as libc::c_int as libc::c_uint {
        let chainSize: U32 =
            (1i32 as U32) << (*zc).appliedParams.cParams.chainLog;
        if (*zc).appliedParams.cParams.strategy as libc::c_uint ==
               ZSTD_btlazy2 as libc::c_int as libc::c_uint {
            ZSTD_reduceTable_btlazy2((*ms).chainTable, chainSize,
                                     reducerValue);
        } else {
            ZSTD_reduceTable((*ms).chainTable, chainSize, reducerValue);
        }
    }
    if 0 != (*ms).hashLog3 {
        let h3Size: U32 = (1i32 as U32) << (*ms).hashLog3;
        ZSTD_reduceTable((*ms).hashTable3, h3Size, reducerValue);
    };
}
unsafe extern "C" fn ZSTD_reduceTable(table: *mut U32, size: U32,
                                      reducerValue: U32) {
    ZSTD_reduceTable_internal(table, size, reducerValue, 0i32);
}
/* ! ZSTD_reduceTable() :
 *  reduce table indexes by `reducerValue`, or squash to zero.
 *  PreserveMark preserves "unsorted mark" for btlazy2 strategy.
 *  It must be set to a clear 0/1 value, to remove branch during inlining.
 *  Presume table size is a multiple of ZSTD_ROWSIZE
 *  to help auto-vectorization */
unsafe extern "C" fn ZSTD_reduceTable_internal(table: *mut U32, size: U32,
                                               reducerValue: U32,
                                               preserveMark: libc::c_int) {
    let nbRows: libc::c_int = size as libc::c_int / 16i32;
    let mut cellNb: libc::c_int = 0i32;
    let mut rowNb: libc::c_int = 0;
    rowNb = 0i32;
    while rowNb < nbRows {
        let mut column: libc::c_int = 0;
        column = 0i32;
        while column < 16i32 {
            if 0 != preserveMark {
                let adder: U32 =
                    if *table.offset(cellNb as isize) == 1i32 as libc::c_uint
                       {
                        reducerValue
                    } else { 0i32 as libc::c_uint };
                let ref mut fresh3 = *table.offset(cellNb as isize);
                *fresh3 =
                    (*fresh3 as libc::c_uint).wrapping_add(adder) as U32 as
                        U32
            }
            if *table.offset(cellNb as isize) < reducerValue {
                *table.offset(cellNb as isize) = 0i32 as U32
            } else {
                let ref mut fresh4 = *table.offset(cellNb as isize);
                *fresh4 =
                    (*fresh4 as libc::c_uint).wrapping_sub(reducerValue) as
                        U32 as U32
            }
            cellNb += 1;
            column += 1
        }
        rowNb += 1
    };
}
unsafe extern "C" fn ZSTD_reduceTable_btlazy2(table: *mut U32, size: U32,
                                              reducerValue: U32) {
    ZSTD_reduceTable_internal(table, size, reducerValue, 1i32);
}
/* *
 * ZSTD_window_needOverflowCorrection():
 * Returns non-zero if the indices are getting too large and need overflow
 * protection.
 */
unsafe extern "C" fn ZSTD_window_needOverflowCorrection(window: ZSTD_window_t,
                                                        mut srcEnd:
                                                            *const libc::c_void)
 -> U32 {
    let current: U32 =
        (srcEnd as *const BYTE).wrapping_offset_from(window.base) as
            libc::c_long as U32;
    return (current >
                (3u32 <<
                     29i32).wrapping_add(1u32 <<
                                             if ::std::mem::size_of::<size_t>()
                                                    as libc::c_ulong ==
                                                    4i32 as libc::c_ulong {
                                                 30i32
                                             } else { 31i32 })) as libc::c_int
               as U32;
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
unsafe extern "C" fn ZSTD_writeFrameHeader(mut dst: *mut libc::c_void,
                                           mut dstCapacity: size_t,
                                           mut params: ZSTD_CCtx_params,
                                           mut pledgedSrcSize: U64,
                                           mut dictID: U32) -> size_t {
    let op: *mut BYTE = dst as *mut BYTE;
    /* 0-3 */
    let dictIDSizeCodeLength: U32 =
        ((dictID > 0i32 as libc::c_uint) as libc::c_int +
             (dictID >= 256i32 as libc::c_uint) as libc::c_int +
             (dictID >= 65536i32 as libc::c_uint) as libc::c_int) as U32;
    /* 0-3 */
    let dictIDSizeCode: U32 =
        if 0 != params.fParams.noDictIDFlag {
            0i32 as libc::c_uint
        } else { dictIDSizeCodeLength };
    let checksumFlag: U32 =
        (params.fParams.checksumFlag > 0i32) as libc::c_int as U32;
    let windowSize: U32 = (1i32 as U32) << params.cParams.windowLog;
    let singleSegment: U32 =
        (0 != params.fParams.contentSizeFlag &&
             windowSize as libc::c_ulong >= pledgedSrcSize) as libc::c_int as
            U32;
    let windowLogByte: BYTE =
        (params.cParams.windowLog.wrapping_sub(10i32 as libc::c_uint) << 3i32)
            as BYTE;
    let fcsCode: U32 =
        (if 0 != params.fParams.contentSizeFlag {
             (pledgedSrcSize >= 256i32 as libc::c_ulong) as libc::c_int +
                 (pledgedSrcSize >= (65536i32 + 256i32) as libc::c_ulong) as
                     libc::c_int +
                 (pledgedSrcSize >= 0xffffffffu32 as libc::c_ulong) as
                     libc::c_int
         } else { 0i32 }) as U32;
    /* 0-3 */
    let frameHeaderDecriptionByte: BYTE =
        dictIDSizeCode.wrapping_add(checksumFlag <<
                                        2i32).wrapping_add(singleSegment <<
                                                               5i32).wrapping_add(fcsCode
                                                                                      <<
                                                                                      6i32)
            as BYTE;
    let mut pos: size_t = 0i32 as size_t;
    if dstCapacity < 18i32 as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    if params.format as libc::c_uint ==
           ZSTD_f_zstd1 as libc::c_int as libc::c_uint {
        MEM_writeLE32(dst, 0xfd2fb528u32);
        pos = 4i32 as size_t
    }
    let fresh5 = pos;
    pos = pos.wrapping_add(1);
    *op.offset(fresh5 as isize) = frameHeaderDecriptionByte;
    if 0 == singleSegment {
        let fresh6 = pos;
        pos = pos.wrapping_add(1);
        *op.offset(fresh6 as isize) = windowLogByte
    }
    match dictIDSizeCode {
        1 => {
            *op.offset(pos as isize) = dictID as BYTE;
            pos = pos.wrapping_add(1)
        }
        2 => {
            MEM_writeLE16(op.offset(pos as isize) as *mut libc::c_void,
                          dictID as U16);
            pos =
                (pos as libc::c_ulong).wrapping_add(2i32 as libc::c_ulong) as
                    size_t as size_t
        }
        3 => {
            MEM_writeLE32(op.offset(pos as isize) as *mut libc::c_void,
                          dictID);
            pos =
                (pos as libc::c_ulong).wrapping_add(4i32 as libc::c_ulong) as
                    size_t as size_t
        }
        0 | _ => { }
    }
    match fcsCode {
        1 => {
            MEM_writeLE16(op.offset(pos as isize) as *mut libc::c_void,
                          pledgedSrcSize.wrapping_sub(256i32 as libc::c_ulong)
                              as U16);
            pos =
                (pos as libc::c_ulong).wrapping_add(2i32 as libc::c_ulong) as
                    size_t as size_t
        }
        2 => {
            MEM_writeLE32(op.offset(pos as isize) as *mut libc::c_void,
                          pledgedSrcSize as U32);
            pos =
                (pos as libc::c_ulong).wrapping_add(4i32 as libc::c_ulong) as
                    size_t as size_t
        }
        3 => {
            MEM_writeLE64(op.offset(pos as isize) as *mut libc::c_void,
                          pledgedSrcSize);
            pos =
                (pos as libc::c_ulong).wrapping_add(8i32 as libc::c_ulong) as
                    size_t as size_t
        }
        0 | _ => {
            if 0 != singleSegment {
                let fresh7 = pos;
                pos = pos.wrapping_add(1);
                *op.offset(fresh7 as isize) = pledgedSrcSize as BYTE
            }
        }
    }
    return pos;
}
/* ! ZSTD_writeEpilogue() :
*   Ends a frame.
*   @return : nb of bytes written into dst (or an error code) */
unsafe extern "C" fn ZSTD_writeEpilogue(mut cctx: *mut ZSTD_CCtx,
                                        mut dst: *mut libc::c_void,
                                        mut dstCapacity: size_t) -> size_t {
    let ostart: *mut BYTE = dst as *mut BYTE;
    let mut op: *mut BYTE = ostart;
    let mut fhSize: size_t = 0i32 as size_t;
    if (*cctx).stage as libc::c_uint ==
           ZSTDcs_created as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    }
    if (*cctx).stage as libc::c_uint ==
           ZSTDcs_init as libc::c_int as libc::c_uint {
        fhSize =
            ZSTD_writeFrameHeader(dst, dstCapacity, (*cctx).appliedParams,
                                  0i32 as U64, 0i32 as U32);
        if 0 != ERR_isError(fhSize) { return fhSize }
        dstCapacity =
            (dstCapacity as libc::c_ulong).wrapping_sub(fhSize) as size_t as
                size_t;
        op = op.offset(fhSize as isize);
        (*cctx).stage = ZSTDcs_ongoing
    }
    if (*cctx).stage as libc::c_uint !=
           ZSTDcs_ending as libc::c_int as libc::c_uint {
        let cBlockHeader24: U32 =
            (1i32 as
                 libc::c_uint).wrapping_add((bt_raw as libc::c_int as U32) <<
                                                1i32).wrapping_add(0i32 as
                                                                       libc::c_uint);
        if dstCapacity < 4i32 as libc::c_ulong {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
        }
        MEM_writeLE32(op as *mut libc::c_void, cBlockHeader24);
        op = op.offset(ZSTD_blockHeaderSize as isize);
        dstCapacity =
            (dstCapacity as libc::c_ulong).wrapping_sub(ZSTD_blockHeaderSize)
                as size_t as size_t
    }
    if 0 != (*cctx).appliedParams.fParams.checksumFlag {
        let checksum: U32 = ZSTD_XXH64_digest(&mut (*cctx).xxhState) as U32;
        if dstCapacity < 4i32 as libc::c_ulong {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
        }
        MEM_writeLE32(op as *mut libc::c_void, checksum);
        op = op.offset(4isize)
    }
    (*cctx).stage = ZSTDcs_created;
    return op.wrapping_offset_from(ostart) as libc::c_long as size_t;
}
/* ! ZSTD_compressBegin_internal() :
 * @return : 0, or an error code */
unsafe extern "C" fn ZSTD_compressBegin_internal(mut cctx: *mut ZSTD_CCtx,
                                                 mut dict:
                                                     *const libc::c_void,
                                                 mut dictSize: size_t,
                                                 mut dictContentType:
                                                     ZSTD_dictContentType_e,
                                                 mut dtlm:
                                                     ZSTD_dictTableLoadMethod_e,
                                                 mut cdict: *const ZSTD_CDict,
                                                 mut params: ZSTD_CCtx_params,
                                                 mut pledgedSrcSize: U64,
                                                 mut zbuff:
                                                     ZSTD_buffered_policy_e)
 -> size_t {
    if !cdict.is_null() && (*cdict).dictContentSize > 0i32 as libc::c_ulong {
        return ZSTD_resetCCtx_usingCDict(cctx, cdict, params, pledgedSrcSize,
                                         zbuff)
    }
    let errcod: size_t =
        ZSTD_resetCCtx_internal(cctx, params, pledgedSrcSize,
                                ZSTDcrp_continue, zbuff);
    if 0 != ERR_isError(errcod) { return errcod }
    let dictID: size_t =
        ZSTD_compress_insertDictionary((*cctx).blockState.prevCBlock,
                                       &mut (*cctx).blockState.matchState,
                                       &mut params, dict, dictSize,
                                       dictContentType, dtlm,
                                       (*cctx).entropyWorkspace as
                                           *mut libc::c_void);
    if 0 != ERR_isError(dictID) { return dictID }
    (*cctx).dictID = dictID as U32;
    return 0i32 as size_t;
}
/* * ZSTD_compress_insertDictionary() :
*   @return : dictID, or an error code */
unsafe extern "C" fn ZSTD_compress_insertDictionary(mut bs:
                                                        *mut ZSTD_compressedBlockState_t,
                                                    mut ms:
                                                        *mut ZSTD_matchState_t,
                                                    mut params:
                                                        *const ZSTD_CCtx_params,
                                                    mut dict:
                                                        *const libc::c_void,
                                                    mut dictSize: size_t,
                                                    mut dictContentType:
                                                        ZSTD_dictContentType_e,
                                                    mut dtlm:
                                                        ZSTD_dictTableLoadMethod_e,
                                                    mut workspace:
                                                        *mut libc::c_void)
 -> size_t {
    if dict == 0 as *mut libc::c_void || dictSize <= 8i32 as libc::c_ulong {
        return 0i32 as size_t
    }
    ZSTD_reset_compressedBlockState(bs);
    if dictContentType as libc::c_uint ==
           ZSTD_dct_rawContent as libc::c_int as libc::c_uint {
        return ZSTD_loadDictionaryContent(ms, params, dict, dictSize, dtlm)
    }
    if MEM_readLE32(dict) != 0xec30a437u32 {
        if dictContentType as libc::c_uint ==
               ZSTD_dct_auto as libc::c_int as libc::c_uint {
            return ZSTD_loadDictionaryContent(ms, params, dict, dictSize,
                                              dtlm)
        }
        if dictContentType as libc::c_uint ==
               ZSTD_dct_fullDict as libc::c_int as libc::c_uint {
            return -(ZSTD_error_dictionary_wrong as libc::c_int) as size_t
        }
    }
    return ZSTD_loadZstdDictionary(bs, ms, params, dict, dictSize, dtlm,
                                   workspace);
}
/* Dictionary format :
 * See :
 * https://github.com/facebook/zstd/blob/master/doc/zstd_compression_format.md#dictionary-format
 */
/*! ZSTD_loadZstdDictionary() :
 * @return : dictID, or an error code
 *  assumptions : magic number supposed already checked
 *                dictSize supposed > 8
 */
unsafe extern "C" fn ZSTD_loadZstdDictionary(mut bs:
                                                 *mut ZSTD_compressedBlockState_t,
                                             mut ms: *mut ZSTD_matchState_t,
                                             mut params:
                                                 *const ZSTD_CCtx_params,
                                             mut dict: *const libc::c_void,
                                             mut dictSize: size_t,
                                             mut dtlm:
                                                 ZSTD_dictTableLoadMethod_e,
                                             mut workspace: *mut libc::c_void)
 -> size_t {
    let mut dictPtr: *const BYTE = dict as *const BYTE;
    let dictEnd: *const BYTE = dictPtr.offset(dictSize as isize);
    let mut offcodeNCount: [libc::c_short; 32] = [0; 32];
    let mut offcodeMaxValue: libc::c_uint = 31i32 as libc::c_uint;
    let mut dictID: size_t = 0;
    dictPtr = dictPtr.offset(4isize);
    dictID =
        (if 0 != (*params).fParams.noDictIDFlag {
             0i32 as libc::c_uint
         } else { MEM_readLE32(dictPtr as *const libc::c_void) }) as size_t;
    dictPtr = dictPtr.offset(4isize);
    let mut maxSymbolValue: libc::c_uint = 255i32 as libc::c_uint;
    let hufHeaderSize: size_t =
        HUF_readCTable((*bs).entropy.huf.CTable.as_mut_ptr() as *mut HUF_CElt,
                       &mut maxSymbolValue, dictPtr as *const libc::c_void,
                       dictEnd.wrapping_offset_from(dictPtr) as libc::c_long
                           as size_t);
    if 0 != ERR_isError(hufHeaderSize) {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    if maxSymbolValue < 255i32 as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    dictPtr = dictPtr.offset(hufHeaderSize as isize);
    let mut offcodeLog: libc::c_uint = 0;
    let offcodeHeaderSize: size_t =
        FSE_readNCount(offcodeNCount.as_mut_ptr(), &mut offcodeMaxValue,
                       &mut offcodeLog, dictPtr as *const libc::c_void,
                       dictEnd.wrapping_offset_from(dictPtr) as libc::c_long
                           as size_t);
    if 0 != ERR_isError(offcodeHeaderSize) {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    if offcodeLog > 8i32 as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    let errcod: size_t =
        FSE_buildCTable_wksp((*bs).entropy.fse.offcodeCTable.as_mut_ptr(),
                             offcodeNCount.as_mut_ptr(),
                             31i32 as libc::c_uint, offcodeLog, workspace,
                             (6i32 << 10i32) as size_t);
    if 0 != ERR_isError(errcod) {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
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
    if matchlengthLog > 9i32 as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    let errcod_0: size_t =
        ZSTD_checkDictNCount(matchlengthNCount.as_mut_ptr(),
                             matchlengthMaxValue, 52i32 as libc::c_uint);
    if 0 != ERR_isError(errcod_0) { return errcod_0 }
    let errcod_1: size_t =
        FSE_buildCTable_wksp((*bs).entropy.fse.matchlengthCTable.as_mut_ptr(),
                             matchlengthNCount.as_mut_ptr(),
                             matchlengthMaxValue, matchlengthLog, workspace,
                             (6i32 << 10i32) as size_t);
    if 0 != ERR_isError(errcod_1) {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
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
    if litlengthLog > 9i32 as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    let errcod_2: size_t =
        ZSTD_checkDictNCount(litlengthNCount.as_mut_ptr(), litlengthMaxValue,
                             35i32 as libc::c_uint);
    if 0 != ERR_isError(errcod_2) { return errcod_2 }
    let errcod_3: size_t =
        FSE_buildCTable_wksp((*bs).entropy.fse.litlengthCTable.as_mut_ptr(),
                             litlengthNCount.as_mut_ptr(), litlengthMaxValue,
                             litlengthLog, workspace,
                             (6i32 << 10i32) as size_t);
    if 0 != ERR_isError(errcod_3) {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    dictPtr = dictPtr.offset(litlengthHeaderSize as isize);
    if dictPtr.offset(12isize) > dictEnd {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    (*bs).rep[0usize] =
        MEM_readLE32(dictPtr.offset(0isize) as *const libc::c_void);
    (*bs).rep[1usize] =
        MEM_readLE32(dictPtr.offset(4isize) as *const libc::c_void);
    (*bs).rep[2usize] =
        MEM_readLE32(dictPtr.offset(8isize) as *const libc::c_void);
    dictPtr = dictPtr.offset(12isize);
    let dictContentSize: size_t =
        dictEnd.wrapping_offset_from(dictPtr) as libc::c_long as size_t;
    let mut offcodeMax: U32 = 31i32 as U32;
    if dictContentSize <=
           (-1i32 as
                U32).wrapping_sub((128i32 * (1i32 << 10i32)) as libc::c_uint)
               as libc::c_ulong {
        let maxOffset: U32 =
            (dictContentSize as
                 U32).wrapping_add((128i32 * (1i32 << 10i32)) as
                                       libc::c_uint);
        offcodeMax = ZSTD_highbit32(maxOffset)
    }
    let errcod_4: size_t =
        ZSTD_checkDictNCount(offcodeNCount.as_mut_ptr(), offcodeMaxValue,
                             if offcodeMax < 31i32 as libc::c_uint {
                                 offcodeMax
                             } else { 31i32 as libc::c_uint });
    if 0 != ERR_isError(errcod_4) { return errcod_4 }
    let mut u: U32 = 0;
    u = 0i32 as U32;
    while u < 3i32 as libc::c_uint {
        if (*bs).rep[u as usize] == 0i32 as libc::c_uint {
            return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
        }
        if (*bs).rep[u as usize] as libc::c_ulong > dictContentSize {
            return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
        }
        u = u.wrapping_add(1)
    }
    (*bs).entropy.huf.repeatMode = HUF_repeat_valid;
    (*bs).entropy.fse.offcode_repeatMode = FSE_repeat_valid;
    (*bs).entropy.fse.matchlength_repeatMode = FSE_repeat_valid;
    (*bs).entropy.fse.litlength_repeatMode = FSE_repeat_valid;
    let errcod_5: size_t =
        ZSTD_loadDictionaryContent(ms, params, dictPtr as *const libc::c_void,
                                   dictContentSize, dtlm);
    if 0 != ERR_isError(errcod_5) { return errcod_5 }
    return dictID;
}
/* ! ZSTD_loadDictionaryContent() :
 *  @return : 0, or an error code
 */
unsafe extern "C" fn ZSTD_loadDictionaryContent(mut ms:
                                                    *mut ZSTD_matchState_t,
                                                mut params:
                                                    *const ZSTD_CCtx_params,
                                                mut src: *const libc::c_void,
                                                mut srcSize: size_t,
                                                mut dtlm:
                                                    ZSTD_dictTableLoadMethod_e)
 -> size_t {
    let ip: *const BYTE = src as *const BYTE;
    let iend: *const BYTE = ip.offset(srcSize as isize);
    ZSTD_window_update(&mut (*ms).window, src, srcSize);
    (*ms).loadedDictEnd =
        if 0 != (*params).forceWindow {
            0i32 as libc::c_uint
        } else {
            iend.wrapping_offset_from((*ms).window.base) as libc::c_long as
                U32
        };
    ZSTD_assertEqualCParams((*params).cParams, (*ms).cParams);
    if srcSize <= 8i32 as libc::c_ulong { return 0i32 as size_t }
    match (*params).cParams.strategy as libc::c_uint {
        1 => { ZSTD_fillHashTable(ms, iend as *const libc::c_void, dtlm); }
        2 => {
            ZSTD_fillDoubleHashTable(ms, iend as *const libc::c_void, dtlm);
        }
        3 | 4 | 5 => {
            if srcSize >= 8i32 as libc::c_ulong {
                ZSTD_insertAndFindFirstIndex(ms, iend.offset(-8isize));
            }
        }
        6 | 7 | 8 | 9 => {
            if srcSize >= 8i32 as libc::c_ulong {
                ZSTD_updateTree(ms, iend.offset(-8isize), iend);
            }
        }
        _ => { }
    }
    (*ms).nextToUpdate =
        iend.wrapping_offset_from((*ms).window.base) as libc::c_long as U32;
    return 0i32 as size_t;
}
/* Dictionaries that assign zero probability to symbols that show up causes problems
   when FSE encoding.  Refuse dictionaries that assign zero probability to symbols
   that we may encounter during compression.
   NOTE: This behavior is not standard and could be improved in the future. */
unsafe extern "C" fn ZSTD_checkDictNCount(mut normalizedCounter:
                                              *mut libc::c_short,
                                          mut dictMaxSymbolValue:
                                              libc::c_uint,
                                          mut maxSymbolValue: libc::c_uint)
 -> size_t {
    let mut s: U32 = 0;
    if dictMaxSymbolValue < maxSymbolValue {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    }
    s = 0i32 as U32;
    while s <= maxSymbolValue {
        if *normalizedCounter.offset(s as isize) as libc::c_int == 0i32 {
            return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
        }
        s = s.wrapping_add(1)
    }
    return 0i32 as size_t;
}
unsafe extern "C" fn ZSTD_reset_compressedBlockState(mut bs:
                                                         *mut ZSTD_compressedBlockState_t) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 3i32 {
        (*bs).rep[i as usize] = repStartValue[i as usize];
        i += 1
    }
    (*bs).entropy.huf.repeatMode = HUF_repeat_none;
    (*bs).entropy.fse.offcode_repeatMode = FSE_repeat_none;
    (*bs).entropy.fse.matchlength_repeatMode = FSE_repeat_none;
    (*bs).entropy.fse.litlength_repeatMode = FSE_repeat_none;
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
/* define "workspace is too large" as this number of times larger than needed */
/* when workspace is continuously too large
                                         * during at least this number of times,
                                         * context's memory usage is considered wasteful,
                                         * because it's sized to handle a worst case scenario which rarely happens.
                                         * In which case, resize it down to free some memory */
/* ! ZSTD_resetCCtx_internal() :
    note : `params` are assumed fully validated at this stage */
unsafe extern "C" fn ZSTD_resetCCtx_internal(mut zc: *mut ZSTD_CCtx,
                                             mut params: ZSTD_CCtx_params,
                                             mut pledgedSrcSize: U64,
                                             crp: ZSTD_compResetPolicy_e,
                                             zbuff: ZSTD_buffered_policy_e)
 -> size_t {
    if crp as libc::c_uint == ZSTDcrp_continue as libc::c_int as libc::c_uint
       {
        if 0 !=
               ZSTD_equivalentParams((*zc).appliedParams, params,
                                     (*zc).inBuffSize,
                                     (*zc).seqStore.maxNbSeq,
                                     (*zc).seqStore.maxNbLit, zbuff,
                                     pledgedSrcSize) {
            (*zc).workSpaceOversizedDuration +=
                ((*zc).workSpaceOversizedDuration > 0i32) as libc::c_int;
            if (*zc).workSpaceOversizedDuration <= 128i32 {
                return ZSTD_continueCCtx(zc, params, pledgedSrcSize)
            }
        }
    }
    if 0 != params.ldmParams.enableLdm {
        ZSTD_ldm_adjustParameters(&mut params.ldmParams, &mut params.cParams);
        (*zc).ldmState.hashPower =
            ZSTD_rollingHash_primePower(params.ldmParams.minMatchLength)
    }
    let windowSize: size_t =
        if 1i32 as libc::c_ulong >
               if (1i32 as U64) << params.cParams.windowLog < pledgedSrcSize {
                   (1i32 as U64) << params.cParams.windowLog
               } else { pledgedSrcSize } {
            1i32 as libc::c_ulong
        } else if (1i32 as U64) << params.cParams.windowLog < pledgedSrcSize {
            (1i32 as U64) << params.cParams.windowLog
        } else { pledgedSrcSize };
    let blockSize: size_t =
        if ((1i32 << 17i32) as libc::c_ulong) < windowSize {
            (1i32 << 17i32) as libc::c_ulong
        } else { windowSize };
    let divider: U32 =
        (if params.cParams.minMatch == 3i32 as libc::c_uint {
             3i32
         } else { 4i32 }) as U32;
    let maxNbSeq: size_t = blockSize.wrapping_div(divider as libc::c_ulong);
    let tokenSpace: size_t =
        (8i32 as
             libc::c_ulong).wrapping_add(blockSize).wrapping_add((11i32 as
                                                                      libc::c_ulong).wrapping_mul(maxNbSeq));
    let buffOutSize: size_t =
        if zbuff as libc::c_uint ==
               ZSTDb_buffered as libc::c_int as libc::c_uint {
            ZSTD_compressBound(blockSize).wrapping_add(1i32 as libc::c_ulong)
        } else { 0i32 as libc::c_ulong };
    let buffInSize: size_t =
        if zbuff as libc::c_uint ==
               ZSTDb_buffered as libc::c_int as libc::c_uint {
            windowSize.wrapping_add(blockSize)
        } else { 0i32 as libc::c_ulong };
    let matchStateSize: size_t =
        ZSTD_sizeof_matchState(&mut params.cParams, 1i32 as U32);
    let maxNbLdmSeq: size_t =
        ZSTD_ldm_getMaxNbSeq(params.ldmParams, blockSize);
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let entropySpace: size_t = (6i32 << 10i32) as size_t;
    let blockStateSpace: size_t =
        (2i32 as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<ZSTD_compressedBlockState_t>()
                                             as libc::c_ulong);
    let bufferSpace: size_t = buffInSize.wrapping_add(buffOutSize);
    let ldmSpace: size_t = ZSTD_ldm_getTableSize(params.ldmParams);
    let ldmSeqSpace: size_t =
        maxNbLdmSeq.wrapping_mul(::std::mem::size_of::<rawSeq>() as
                                     libc::c_ulong);
    let neededSpace: size_t =
        entropySpace.wrapping_add(blockStateSpace).wrapping_add(ldmSpace).wrapping_add(ldmSeqSpace).wrapping_add(matchStateSize).wrapping_add(tokenSpace).wrapping_add(bufferSpace);
    let workSpaceTooSmall: libc::c_int =
        ((*zc).workSpaceSize < neededSpace) as libc::c_int;
    let workSpaceTooLarge: libc::c_int =
        ((*zc).workSpaceSize >
             (3i32 as libc::c_ulong).wrapping_mul(neededSpace)) as
            libc::c_int;
    let workSpaceWasteful: libc::c_int =
        (0 != workSpaceTooLarge && (*zc).workSpaceOversizedDuration > 128i32)
            as libc::c_int;
    (*zc).workSpaceOversizedDuration =
        if 0 != workSpaceTooLarge {
            (*zc).workSpaceOversizedDuration + 1i32
        } else { 0i32 };
    if 0 != workSpaceTooSmall || 0 != workSpaceWasteful {
        if 0 != (*zc).staticSize {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        }
        (*zc).workSpaceSize = 0i32 as size_t;
        ZSTD_free((*zc).workSpace, (*zc).customMem);
        (*zc).workSpace = ZSTD_malloc(neededSpace, (*zc).customMem);
        if (*zc).workSpace.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        }
        (*zc).workSpaceSize = neededSpace;
        (*zc).workSpaceOversizedDuration = 0i32;
        (*zc).blockState.prevCBlock =
            (*zc).workSpace as *mut ZSTD_compressedBlockState_t;
        (*zc).blockState.nextCBlock =
            (*zc).blockState.prevCBlock.offset(1isize);
        ptr = (*zc).blockState.nextCBlock.offset(1isize) as *mut libc::c_void;
        (*zc).entropyWorkspace = ptr as *mut U32
    }
    (*zc).appliedParams = params;
    (*zc).blockState.matchState.cParams = params.cParams;
    (*zc).pledgedSrcSizePlusOne =
        pledgedSrcSize.wrapping_add(1i32 as libc::c_ulong) as
            libc::c_ulonglong;
    (*zc).consumedSrcSize = 0i32 as libc::c_ulonglong;
    (*zc).producedCSize = 0i32 as libc::c_ulonglong;
    if pledgedSrcSize as libc::c_ulonglong ==
           0u64.wrapping_sub(1i32 as libc::c_ulonglong) {
        (*zc).appliedParams.fParams.contentSizeFlag = 0i32
    }
    (*zc).blockSize = blockSize;
    ZSTD_XXH64_reset(&mut (*zc).xxhState, 0i32 as libc::c_ulonglong);
    (*zc).stage = ZSTDcs_init;
    (*zc).dictID = 0i32 as U32;
    ZSTD_reset_compressedBlockState((*zc).blockState.prevCBlock);
    ptr =
        (*zc).entropyWorkspace.offset(((6i32 << 10i32) as
                                           libc::c_ulong).wrapping_div(::std::mem::size_of::<U32>()
                                                                           as
                                                                           libc::c_ulong)
                                          as isize) as *mut libc::c_void;
    if 0 != params.ldmParams.enableLdm {
        let ldmHSize: size_t = (1i32 as size_t) << params.ldmParams.hashLog;
        memset(ptr, 0i32,
               ldmHSize.wrapping_mul(::std::mem::size_of::<ldmEntry_t>() as
                                         libc::c_ulong));
        (*zc).ldmState.hashTable = ptr as *mut ldmEntry_t;
        ptr =
            (*zc).ldmState.hashTable.offset(ldmHSize as isize) as
                *mut libc::c_void;
        (*zc).ldmSequences = ptr as *mut rawSeq;
        ptr =
            (*zc).ldmSequences.offset(maxNbLdmSeq as isize) as
                *mut libc::c_void;
        (*zc).maxNbLdmSequences = maxNbLdmSeq;
        memset(&mut (*zc).ldmState.window as *mut ZSTD_window_t as
                   *mut libc::c_void, 0i32,
               ::std::mem::size_of::<ZSTD_window_t>() as libc::c_ulong);
    }
    ptr =
        ZSTD_reset_matchState(&mut (*zc).blockState.matchState, ptr,
                              &mut params.cParams, crp, 1i32 as U32);
    (*zc).seqStore.maxNbSeq = maxNbSeq;
    (*zc).seqStore.sequencesStart = ptr as *mut seqDef;
    ptr =
        (*zc).seqStore.sequencesStart.offset(maxNbSeq as isize) as
            *mut libc::c_void;
    (*zc).seqStore.llCode = ptr as *mut BYTE;
    (*zc).seqStore.mlCode = (*zc).seqStore.llCode.offset(maxNbSeq as isize);
    (*zc).seqStore.ofCode = (*zc).seqStore.mlCode.offset(maxNbSeq as isize);
    (*zc).seqStore.litStart = (*zc).seqStore.ofCode.offset(maxNbSeq as isize);
    (*zc).seqStore.maxNbLit = blockSize;
    ptr =
        (*zc).seqStore.litStart.offset(blockSize as isize).offset(8isize) as
            *mut libc::c_void;
    if 0 != params.ldmParams.enableLdm {
        let ldmBucketSize: size_t =
            (1i32 as size_t) <<
                params.ldmParams.hashLog.wrapping_sub(params.ldmParams.bucketSizeLog);
        memset(ptr, 0i32, ldmBucketSize);
        (*zc).ldmState.bucketOffsets = ptr as *mut BYTE;
        ptr =
            (*zc).ldmState.bucketOffsets.offset(ldmBucketSize as isize) as
                *mut libc::c_void;
        ZSTD_window_clear(&mut (*zc).ldmState.window);
    }
    ZSTD_referenceExternalSequences(zc, 0 as *mut rawSeq, 0i32 as size_t);
    (*zc).inBuffSize = buffInSize;
    (*zc).inBuff = ptr as *mut libc::c_char;
    (*zc).outBuffSize = buffOutSize;
    (*zc).outBuff = (*zc).inBuff.offset(buffInSize as isize);
    return 0i32 as size_t;
}
/*======  Helper functions  ======*/
/* margin, from 64 to 0 */
/* this formula ensures that bound(A) + bound(B) <= bound(A+B) as long as A and B >= 128 KB */
/* !< maximum compressed size in worst case single-pass scenario */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBound(mut srcSize: size_t) -> size_t {
    return srcSize.wrapping_add(srcSize >>
                                    8i32).wrapping_add(if srcSize <
                                                              (128i32 <<
                                                                   10i32) as
                                                                  libc::c_ulong
                                                          {
                                                           ((128i32 << 10i32)
                                                                as
                                                                libc::c_ulong).wrapping_sub(srcSize)
                                                               >> 11i32
                                                       } else {
                                                           0i32 as
                                                               libc::c_ulong
                                                       });
}
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
pub unsafe extern "C" fn ZSTD_referenceExternalSequences(mut cctx:
                                                             *mut ZSTD_CCtx,
                                                         mut seq: *mut rawSeq,
                                                         mut nbSeq: size_t)
 -> size_t {
    if (*cctx).stage as libc::c_uint !=
           ZSTDcs_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    }
    if 0 != (*cctx).appliedParams.ldmParams.enableLdm {
        return -(ZSTD_error_parameter_unsupported as libc::c_int) as size_t
    }
    (*cctx).externSeqStore.seq = seq;
    (*cctx).externSeqStore.size = nbSeq;
    (*cctx).externSeqStore.capacity = nbSeq;
    (*cctx).externSeqStore.pos = 0i32 as size_t;
    return 0i32 as size_t;
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
unsafe extern "C" fn ZSTD_reset_matchState(mut ms: *mut ZSTD_matchState_t,
                                           mut ptr: *mut libc::c_void,
                                           mut cParams:
                                               *const ZSTD_compressionParameters,
                                           crp: ZSTD_compResetPolicy_e,
                                           forCCtx: U32)
 -> *mut libc::c_void {
    let chainSize: size_t =
        if (*cParams).strategy as libc::c_uint ==
               ZSTD_fast as libc::c_int as libc::c_uint {
            0i32 as libc::c_ulong
        } else { (1i32 as size_t) << (*cParams).chainLog };
    let hSize: size_t = (1i32 as size_t) << (*cParams).hashLog;
    let hashLog3: U32 =
        if 0 != forCCtx && (*cParams).minMatch == 3i32 as libc::c_uint {
            if (17i32 as libc::c_uint) < (*cParams).windowLog {
                17i32 as libc::c_uint
            } else { (*cParams).windowLog }
        } else { 0i32 as libc::c_uint };
    let h3Size: size_t = (1i32 as size_t) << hashLog3;
    let tableSpace: size_t =
        chainSize.wrapping_add(hSize).wrapping_add(h3Size).wrapping_mul(::std::mem::size_of::<U32>()
                                                                            as
                                                                            libc::c_ulong);
    (*ms).hashLog3 = hashLog3;
    memset(&mut (*ms).window as *mut ZSTD_window_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZSTD_window_t>() as libc::c_ulong);
    (*ms).window.dictLimit = 1i32 as U32;
    (*ms).window.lowLimit = 1i32 as U32;
    (*ms).window.nextSrc = (*ms).window.base.offset(1isize);
    ZSTD_invalidateMatchState(ms);
    if 0 != forCCtx &&
           (*cParams).strategy as libc::c_uint >=
               ZSTD_btopt as libc::c_int as libc::c_uint {
        (*ms).opt.litFreq = ptr as *mut libc::c_uint;
        (*ms).opt.litLengthFreq =
            (*ms).opt.litFreq.offset((1i32 << 8i32) as isize);
        (*ms).opt.matchLengthFreq =
            (*ms).opt.litLengthFreq.offset((35i32 + 1i32) as isize);
        (*ms).opt.offCodeFreq =
            (*ms).opt.matchLengthFreq.offset((52i32 + 1i32) as isize);
        ptr =
            (*ms).opt.offCodeFreq.offset((31i32 + 1i32) as isize) as
                *mut libc::c_void;
        (*ms).opt.matchTable = ptr as *mut ZSTD_match_t;
        ptr =
            (*ms).opt.matchTable.offset((1i32 << 12i32) as
                                            isize).offset(1isize) as
                *mut libc::c_void;
        (*ms).opt.priceTable = ptr as *mut ZSTD_optimal_t;
        ptr =
            (*ms).opt.priceTable.offset((1i32 << 12i32) as
                                            isize).offset(1isize) as
                *mut libc::c_void
    }
    if crp as libc::c_uint != ZSTDcrp_noMemset as libc::c_int as libc::c_uint
       {
        memset(ptr, 0i32, tableSpace);
    }
    (*ms).hashTable = ptr as *mut U32;
    (*ms).chainTable = (*ms).hashTable.offset(hSize as isize);
    (*ms).hashTable3 = (*ms).chainTable.offset(chainSize as isize);
    ptr = (*ms).hashTable3.offset(h3Size as isize) as *mut libc::c_void;
    (*ms).cParams = *cParams;
    return ptr;
}
/* ! ZSTD_invalidateMatchState()
 * Invalidate all the matches in the match finder tables.
 * Requires nextSrc and base to be set (can be NULL).
 */
unsafe extern "C" fn ZSTD_invalidateMatchState(mut ms:
                                                   *mut ZSTD_matchState_t) {
    ZSTD_window_clear(&mut (*ms).window);
    (*ms).nextToUpdate = (*ms).window.dictLimit;
    (*ms).nextToUpdate3 = (*ms).window.dictLimit;
    (*ms).loadedDictEnd = 0i32 as U32;
    (*ms).opt.litLengthSum = 0i32 as U32;
    (*ms).dictMatchState = 0 as *const ZSTD_matchState_t;
}
unsafe extern "C" fn ZSTD_sizeof_matchState(cParams:
                                                *const ZSTD_compressionParameters,
                                            forCCtx: U32) -> size_t {
    let chainSize: size_t =
        if (*cParams).strategy as libc::c_uint ==
               ZSTD_fast as libc::c_int as libc::c_uint {
            0i32 as libc::c_ulong
        } else { (1i32 as size_t) << (*cParams).chainLog };
    let hSize: size_t = (1i32 as size_t) << (*cParams).hashLog;
    let hashLog3: U32 =
        if 0 != forCCtx && (*cParams).minMatch == 3i32 as libc::c_uint {
            if (17i32 as libc::c_uint) < (*cParams).windowLog {
                17i32 as libc::c_uint
            } else { (*cParams).windowLog }
        } else { 0i32 as libc::c_uint };
    let h3Size: size_t = (1i32 as size_t) << hashLog3;
    let tableSpace: size_t =
        chainSize.wrapping_add(hSize).wrapping_add(h3Size).wrapping_mul(::std::mem::size_of::<U32>()
                                                                            as
                                                                            libc::c_ulong);
    let optPotentialSpace: size_t =
        ((52i32 + 1i32 + (35i32 + 1i32) + (31i32 + 1i32) + (1i32 << 8i32)) as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<U32>() as
                                             libc::c_ulong).wrapping_add((((1i32
                                                                                <<
                                                                                12i32)
                                                                               +
                                                                               1i32)
                                                                              as
                                                                              libc::c_ulong).wrapping_mul((::std::mem::size_of::<ZSTD_match_t>()
                                                                                                               as
                                                                                                               libc::c_ulong).wrapping_add(::std::mem::size_of::<ZSTD_optimal_t>()
                                                                                                                                               as
                                                                                                                                               libc::c_ulong)));
    let optSpace: size_t =
        if 0 != forCCtx &&
               (*cParams).strategy as libc::c_uint >=
                   ZSTD_btopt as libc::c_int as libc::c_uint {
            optPotentialSpace
        } else { 0i32 as libc::c_ulong };
    return tableSpace.wrapping_add(optSpace);
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
/* ! ZSTD_continueCCtx() :
 *  reuse CCtx without reset (note : requires no dictionary) */
unsafe extern "C" fn ZSTD_continueCCtx(mut cctx: *mut ZSTD_CCtx,
                                       mut params: ZSTD_CCtx_params,
                                       mut pledgedSrcSize: U64) -> size_t {
    let windowSize: size_t =
        if 1i32 as libc::c_ulong >
               if (1i32 as U64) << params.cParams.windowLog < pledgedSrcSize {
                   (1i32 as U64) << params.cParams.windowLog
               } else { pledgedSrcSize } {
            1i32 as libc::c_ulong
        } else if (1i32 as U64) << params.cParams.windowLog < pledgedSrcSize {
            (1i32 as U64) << params.cParams.windowLog
        } else { pledgedSrcSize };
    let blockSize: size_t =
        if ((1i32 << 17i32) as libc::c_ulong) < windowSize {
            (1i32 << 17i32) as libc::c_ulong
        } else { windowSize };
    (*cctx).blockSize = blockSize;
    (*cctx).appliedParams = params;
    (*cctx).blockState.matchState.cParams = params.cParams;
    (*cctx).pledgedSrcSizePlusOne =
        pledgedSrcSize.wrapping_add(1i32 as libc::c_ulong) as
            libc::c_ulonglong;
    (*cctx).consumedSrcSize = 0i32 as libc::c_ulonglong;
    (*cctx).producedCSize = 0i32 as libc::c_ulonglong;
    if pledgedSrcSize as libc::c_ulonglong ==
           0u64.wrapping_sub(1i32 as libc::c_ulonglong) {
        (*cctx).appliedParams.fParams.contentSizeFlag = 0i32
    }
    (*cctx).stage = ZSTDcs_init;
    (*cctx).dictID = 0i32 as U32;
    if 0 != params.ldmParams.enableLdm {
        ZSTD_window_clear(&mut (*cctx).ldmState.window);
    }
    ZSTD_referenceExternalSequences(cctx, 0 as *mut rawSeq, 0i32 as size_t);
    ZSTD_invalidateMatchState(&mut (*cctx).blockState.matchState);
    ZSTD_reset_compressedBlockState((*cctx).blockState.prevCBlock);
    ZSTD_XXH64_reset(&mut (*cctx).xxhState, 0i32 as libc::c_ulonglong);
    return 0i32 as size_t;
}
/* * Equivalence for resetCCtx purposes */
unsafe extern "C" fn ZSTD_equivalentParams(mut params1: ZSTD_CCtx_params,
                                           mut params2: ZSTD_CCtx_params,
                                           mut buffSize1: size_t,
                                           mut maxNbSeq1: size_t,
                                           mut maxNbLit1: size_t,
                                           mut buffPol2:
                                               ZSTD_buffered_policy_e,
                                           mut pledgedSrcSize: U64) -> U32 {
    if 0 == ZSTD_equivalentCParams(params1.cParams, params2.cParams) {
        return 0i32 as U32
    }
    if 0 == ZSTD_equivalentLdmParams(params1.ldmParams, params2.ldmParams) {
        return 0i32 as U32
    }
    if 0 ==
           ZSTD_sufficientBuff(buffSize1, maxNbSeq1, maxNbLit1, buffPol2,
                               params2.cParams, pledgedSrcSize) {
        return 0i32 as U32
    }
    return 1i32 as U32;
}
/* ZSTD_sufficientBuff() :
 * check internal buffers exist for streaming if buffPol == ZSTDb_buffered .
 * Note : they are assumed to be correctly sized if ZSTD_equivalentCParams()==1 */
unsafe extern "C" fn ZSTD_sufficientBuff(mut bufferSize1: size_t,
                                         mut maxNbSeq1: size_t,
                                         mut maxNbLit1: size_t,
                                         mut buffPol2: ZSTD_buffered_policy_e,
                                         mut cParams2:
                                             ZSTD_compressionParameters,
                                         mut pledgedSrcSize: U64) -> U32 {
    let windowSize2: size_t =
        if 1i32 as libc::c_ulong >
               if (1i32 as U64) << cParams2.windowLog < pledgedSrcSize {
                   (1i32 as U64) << cParams2.windowLog
               } else { pledgedSrcSize } {
            1i32 as libc::c_ulong
        } else if (1i32 as U64) << cParams2.windowLog < pledgedSrcSize {
            (1i32 as U64) << cParams2.windowLog
        } else { pledgedSrcSize };
    let blockSize2: size_t =
        if ((1i32 << 17i32) as libc::c_ulong) < windowSize2 {
            (1i32 << 17i32) as libc::c_ulong
        } else { windowSize2 };
    let maxNbSeq2: size_t =
        blockSize2.wrapping_div((if cParams2.minMatch == 3i32 as libc::c_uint
                                    {
                                     3i32
                                 } else { 4i32 }) as libc::c_ulong);
    let maxNbLit2: size_t = blockSize2;
    let neededBufferSize2: size_t =
        if buffPol2 as libc::c_uint ==
               ZSTDb_buffered as libc::c_int as libc::c_uint {
            windowSize2.wrapping_add(blockSize2)
        } else { 0i32 as libc::c_ulong };
    return ((maxNbLit2 <= maxNbLit1) as libc::c_int &
                (maxNbSeq2 <= maxNbSeq1) as libc::c_int &
                (neededBufferSize2 <= bufferSize1) as libc::c_int) as U32;
}
/* * The parameters are equivalent if ldm is not enabled in both sets or
 *  all the parameters are equivalent. */
unsafe extern "C" fn ZSTD_equivalentLdmParams(mut ldmParams1: ldmParams_t,
                                              mut ldmParams2: ldmParams_t)
 -> U32 {
    return (0 == ldmParams1.enableLdm && 0 == ldmParams2.enableLdm ||
                ldmParams1.enableLdm == ldmParams2.enableLdm &&
                    ldmParams1.hashLog == ldmParams2.hashLog &&
                    ldmParams1.bucketSizeLog == ldmParams2.bucketSizeLog &&
                    ldmParams1.minMatchLength == ldmParams2.minMatchLength &&
                    ldmParams1.hashRateLog == ldmParams2.hashRateLog) as
               libc::c_int as U32;
}
unsafe extern "C" fn ZSTD_equivalentCParams(mut cParams1:
                                                ZSTD_compressionParameters,
                                            mut cParams2:
                                                ZSTD_compressionParameters)
 -> U32 {
    return ((cParams1.hashLog == cParams2.hashLog) as libc::c_int &
                (cParams1.chainLog == cParams2.chainLog) as libc::c_int &
                (cParams1.strategy as libc::c_uint ==
                     cParams2.strategy as libc::c_uint) as libc::c_int &
                ((cParams1.minMatch == 3i32 as libc::c_uint) as libc::c_int ==
                     (cParams2.minMatch == 3i32 as libc::c_uint) as
                         libc::c_int) as libc::c_int) as U32;
}
/* We have a choice between copying the dictionary context into the working
 * context, or referencing the dictionary context from the working context
 * in-place. We decide here which strategy to use. */
unsafe extern "C" fn ZSTD_resetCCtx_usingCDict(mut cctx: *mut ZSTD_CCtx,
                                               mut cdict: *const ZSTD_CDict,
                                               mut params: ZSTD_CCtx_params,
                                               mut pledgedSrcSize: U64,
                                               mut zbuff:
                                                   ZSTD_buffered_policy_e)
 -> size_t {
    if 0 != ZSTD_shouldAttachDict(cdict, params, pledgedSrcSize) {
        return ZSTD_resetCCtx_byAttachingCDict(cctx, cdict, params,
                                               pledgedSrcSize, zbuff)
    } else {
        return ZSTD_resetCCtx_byCopyingCDict(cctx, cdict, params,
                                             pledgedSrcSize, zbuff)
    };
}
unsafe extern "C" fn ZSTD_resetCCtx_byCopyingCDict(mut cctx: *mut ZSTD_CCtx,
                                                   mut cdict:
                                                       *const ZSTD_CDict,
                                                   mut params:
                                                       ZSTD_CCtx_params,
                                                   mut pledgedSrcSize: U64,
                                                   mut zbuff:
                                                       ZSTD_buffered_policy_e)
 -> size_t {
    let mut cdict_cParams: *const ZSTD_compressionParameters =
        &(*cdict).matchState.cParams;
    let windowLog: libc::c_uint = params.cParams.windowLog;
    params.cParams = *cdict_cParams;
    params.cParams.windowLog = windowLog;
    ZSTD_resetCCtx_internal(cctx, params, pledgedSrcSize, ZSTDcrp_noMemset,
                            zbuff);
    let chainSize: size_t =
        if (*cdict_cParams).strategy as libc::c_uint ==
               ZSTD_fast as libc::c_int as libc::c_uint {
            0i32 as libc::c_ulong
        } else { (1i32 as size_t) << (*cdict_cParams).chainLog };
    let hSize: size_t = (1i32 as size_t) << (*cdict_cParams).hashLog;
    let tableSpace: size_t =
        chainSize.wrapping_add(hSize).wrapping_mul(::std::mem::size_of::<U32>()
                                                       as libc::c_ulong);
    memcpy((*cctx).blockState.matchState.hashTable as *mut libc::c_void,
           (*cdict).matchState.hashTable as *const libc::c_void, tableSpace);
    let h3Size: size_t =
        (1i32 as size_t) << (*cctx).blockState.matchState.hashLog3;
    memset((*cctx).blockState.matchState.hashTable3 as *mut libc::c_void,
           0i32,
           h3Size.wrapping_mul(::std::mem::size_of::<U32>() as
                                   libc::c_ulong));
    let mut srcMatchState: *const ZSTD_matchState_t = &(*cdict).matchState;
    let mut dstMatchState: *mut ZSTD_matchState_t =
        &mut (*cctx).blockState.matchState;
    (*dstMatchState).window = (*srcMatchState).window;
    (*dstMatchState).nextToUpdate = (*srcMatchState).nextToUpdate;
    (*dstMatchState).nextToUpdate3 = (*srcMatchState).nextToUpdate3;
    (*dstMatchState).loadedDictEnd = (*srcMatchState).loadedDictEnd;
    (*cctx).dictID = (*cdict).dictID;
    memcpy((*cctx).blockState.prevCBlock as *mut libc::c_void,
           &(*cdict).cBlockState as *const ZSTD_compressedBlockState_t as
               *const libc::c_void,
           ::std::mem::size_of::<ZSTD_compressedBlockState_t>() as
               libc::c_ulong);
    return 0i32 as size_t;
}
/* dictMatchState isn't correctly
                                 * handled in _enforceMaxDist */
unsafe extern "C" fn ZSTD_resetCCtx_byAttachingCDict(mut cctx: *mut ZSTD_CCtx,
                                                     mut cdict:
                                                         *const ZSTD_CDict,
                                                     mut params:
                                                         ZSTD_CCtx_params,
                                                     mut pledgedSrcSize: U64,
                                                     mut zbuff:
                                                         ZSTD_buffered_policy_e)
 -> size_t {
    let mut cdict_cParams: *const ZSTD_compressionParameters =
        &(*cdict).matchState.cParams;
    let windowLog: libc::c_uint = params.cParams.windowLog;
    params.cParams =
        ZSTD_adjustCParams_internal(*cdict_cParams,
                                    pledgedSrcSize as libc::c_ulonglong,
                                    0i32 as size_t);
    params.cParams.windowLog = windowLog;
    ZSTD_resetCCtx_internal(cctx, params, pledgedSrcSize, ZSTDcrp_continue,
                            zbuff);
    let cdictEnd: U32 =
        (*cdict).matchState.window.nextSrc.wrapping_offset_from((*cdict).matchState.window.base)
            as libc::c_long as U32;
    let cdictLen: U32 =
        cdictEnd.wrapping_sub((*cdict).matchState.window.dictLimit);
    if !(cdictLen == 0i32 as libc::c_uint) {
        (*cctx).blockState.matchState.dictMatchState = &(*cdict).matchState;
        if (*cctx).blockState.matchState.window.dictLimit < cdictEnd {
            (*cctx).blockState.matchState.window.nextSrc =
                (*cctx).blockState.matchState.window.base.offset(cdictEnd as
                                                                     isize);
            ZSTD_window_clear(&mut (*cctx).blockState.matchState.window);
        }
        (*cctx).blockState.matchState.loadedDictEnd =
            (*cctx).blockState.matchState.window.dictLimit
    }
    (*cctx).dictID = (*cdict).dictID;
    memcpy((*cctx).blockState.prevCBlock as *mut libc::c_void,
           &(*cdict).cBlockState as *const ZSTD_compressedBlockState_t as
               *const libc::c_void,
           ::std::mem::size_of::<ZSTD_compressedBlockState_t>() as
               libc::c_ulong);
    return 0i32 as size_t;
}
/* unused */
/* ZSTD_fast */
/* ZSTD_dfast */
/* ZSTD_greedy */
/* ZSTD_lazy */
/* ZSTD_lazy2 */
/* ZSTD_btlazy2 */
/* ZSTD_btopt */
/* ZSTD_btultra */
/* ZSTD_btultra2 */
unsafe extern "C" fn ZSTD_shouldAttachDict(mut cdict: *const ZSTD_CDict,
                                           mut params: ZSTD_CCtx_params,
                                           mut pledgedSrcSize: U64)
 -> libc::c_int {
    let mut cutoff: size_t =
        attachDictSizeCutoffs[(*cdict).matchState.cParams.strategy as usize];
    return ((pledgedSrcSize <= cutoff ||
                 pledgedSrcSize as libc::c_ulonglong ==
                     0u64.wrapping_sub(1i32 as libc::c_ulonglong) ||
                 params.attachDictPref as libc::c_uint ==
                     ZSTD_dictForceAttach as libc::c_int as libc::c_uint) &&
                params.attachDictPref as libc::c_uint !=
                    ZSTD_dictForceCopy as libc::c_int as libc::c_uint &&
                0 == params.forceWindow) as libc::c_int;
}
/* These are the approximate sizes for each strategy past which copying the
 * dictionary tables into the working context is faster than using them
 * in-place.
 */
static mut attachDictSizeCutoffs: [size_t; 10] =
    [(8i32 * (1i32 << 10i32)) as size_t, (8i32 * (1i32 << 10i32)) as size_t,
     (16i32 * (1i32 << 10i32)) as size_t, (32i32 * (1i32 << 10i32)) as size_t,
     (32i32 * (1i32 << 10i32)) as size_t, (32i32 * (1i32 << 10i32)) as size_t,
     (32i32 * (1i32 << 10i32)) as size_t, (32i32 * (1i32 << 10i32)) as size_t,
     (8i32 * (1i32 << 10i32)) as size_t, (8i32 * (1i32 << 10i32)) as size_t];
/* *< this constant defers to stdlib's functions */
static mut ZSTD_defaultCMem: ZSTD_customMem =
    ZSTD_customMem{customAlloc: None,
                   customFree: None,
                   opaque: 0 as *const libc::c_void as *mut libc::c_void,};
unsafe extern "C" fn ZSTD_initCCtx(mut cctx: *mut ZSTD_CCtx,
                                   mut memManager: ZSTD_customMem) {
    memset(cctx as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZSTD_CCtx>() as libc::c_ulong);
    (*cctx).customMem = memManager;
    (*cctx).bmi2 = ZSTD_cpuid_bmi2(ZSTD_cpuid());
    let err: size_t = ZSTD_CCtx_reset(cctx, ZSTD_reset_parameters);
}
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
pub unsafe extern "C" fn ZSTD_CCtx_reset(mut cctx: *mut ZSTD_CCtx,
                                         mut reset: ZSTD_ResetDirective)
 -> size_t {
    if reset as libc::c_uint ==
           ZSTD_reset_session_only as libc::c_int as libc::c_uint ||
           reset as libc::c_uint ==
               ZSTD_reset_session_and_parameters as libc::c_int as
                   libc::c_uint {
        (*cctx).streamStage = zcss_init;
        (*cctx).pledgedSrcSizePlusOne = 0i32 as libc::c_ulonglong
    }
    if reset as libc::c_uint ==
           ZSTD_reset_parameters as libc::c_int as libc::c_uint ||
           reset as libc::c_uint ==
               ZSTD_reset_session_and_parameters as libc::c_int as
                   libc::c_uint {
        if (*cctx).streamStage as libc::c_uint !=
               zcss_init as libc::c_int as libc::c_uint {
            return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
        }
        (*cctx).cdict = 0 as *const ZSTD_CDict;
        return ZSTD_CCtxParams_reset(&mut (*cctx).requestedParams)
    }
    return 0i32 as size_t;
}
/* ! ZSTD_CCtxParams_reset() :
 *  Reset params to default values.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtxParams_reset(mut params:
                                                   *mut ZSTD_CCtx_params)
 -> size_t {
    return ZSTD_CCtxParams_init(params, 3i32);
}
/* ! ZSTD_CCtxParams_init() :
 *  Initializes the compression parameters of cctxParams according to
 *  compression level. All other parameters are reset to their default values.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtxParams_init(mut cctxParams:
                                                  *mut ZSTD_CCtx_params,
                                              mut compressionLevel:
                                                  libc::c_int) -> size_t {
    if cctxParams.is_null() {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    memset(cctxParams as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZSTD_CCtx_params>() as libc::c_ulong);
    (*cctxParams).compressionLevel = compressionLevel;
    (*cctxParams).fParams.contentSizeFlag = 1i32;
    return 0i32 as size_t;
}
/* !< maximum compression level available */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_maxCLevel() -> libc::c_int { return 22i32; }
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCCtx() -> *mut ZSTD_CCtx {
    return ZSTD_createCCtx_advanced(ZSTD_defaultCMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCCtx_advanced(mut customMem:
                                                      ZSTD_customMem)
 -> *mut ZSTD_CCtx {
    if 0 !=
           customMem.customAlloc.is_none() as libc::c_int ^
               customMem.customFree.is_none() as libc::c_int {
        return 0 as *mut ZSTD_CCtx
    }
    let cctx: *mut ZSTD_CCtx =
        ZSTD_malloc(::std::mem::size_of::<ZSTD_CCtx>() as libc::c_ulong,
                    customMem) as *mut ZSTD_CCtx;
    if cctx.is_null() { return 0 as *mut ZSTD_CCtx }
    ZSTD_initCCtx(cctx, customMem);
    return cctx;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeCCtx(mut cctx: *mut ZSTD_CCtx) -> size_t {
    if cctx.is_null() { return 0i32 as size_t }
    if 0 != (*cctx).staticSize {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    }
    ZSTD_freeCCtxContent(cctx);
    ZSTD_free(cctx as *mut libc::c_void, (*cctx).customMem);
    return 0i32 as size_t;
}
/* ! ZSTD_createCDict() :
 *  When compressing multiple messages / blocks using the same dictionary, it's recommended to load it only once.
 *  ZSTD_createCDict() will create a digested dictionary, ready to start future compression operations without startup cost.
 *  ZSTD_CDict can be created once and shared by multiple threads concurrently, since its usage is read-only.
 * `dictBuffer` can be released after ZSTD_CDict creation, because its content is copied within CDict.
 *  Consider experimental function `ZSTD_createCDict_byReference()` if you prefer to not duplicate `dictBuffer` content.
 *  Note : A ZSTD_CDict can be created from an empty dictBuffer, but it is inefficient when used to compress small data. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCDict(mut dict: *const libc::c_void,
                                          mut dictSize: size_t,
                                          mut compressionLevel: libc::c_int)
 -> *mut ZSTD_CDict {
    let mut cParams: ZSTD_compressionParameters =
        ZSTD_getCParams(compressionLevel, 0i32 as libc::c_ulonglong,
                        dictSize);
    return ZSTD_createCDict_advanced(dict, dictSize, ZSTD_dlm_byCopy,
                                     ZSTD_dct_auto, cParams,
                                     ZSTD_defaultCMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCDict_advanced(mut dictBuffer:
                                                       *const libc::c_void,
                                                   mut dictSize: size_t,
                                                   mut dictLoadMethod:
                                                       ZSTD_dictLoadMethod_e,
                                                   mut dictContentType:
                                                       ZSTD_dictContentType_e,
                                                   mut cParams:
                                                       ZSTD_compressionParameters,
                                                   mut customMem:
                                                       ZSTD_customMem)
 -> *mut ZSTD_CDict {
    if 0 !=
           customMem.customAlloc.is_none() as libc::c_int ^
               customMem.customFree.is_none() as libc::c_int {
        return 0 as *mut ZSTD_CDict
    }
    let cdict: *mut ZSTD_CDict =
        ZSTD_malloc(::std::mem::size_of::<ZSTD_CDict>() as libc::c_ulong,
                    customMem) as *mut ZSTD_CDict;
    let workspaceSize: size_t =
        ((6i32 << 10i32) as
             libc::c_ulong).wrapping_add(ZSTD_sizeof_matchState(&mut cParams,
                                                                0i32 as U32));
    let workspace: *mut libc::c_void = ZSTD_malloc(workspaceSize, customMem);
    if cdict.is_null() || workspace.is_null() {
        ZSTD_free(cdict as *mut libc::c_void, customMem);
        ZSTD_free(workspace, customMem);
        return 0 as *mut ZSTD_CDict
    }
    (*cdict).customMem = customMem;
    (*cdict).workspace = workspace;
    (*cdict).workspaceSize = workspaceSize;
    if 0 !=
           ERR_isError(ZSTD_initCDict_internal(cdict, dictBuffer, dictSize,
                                               dictLoadMethod,
                                               dictContentType, cParams)) {
        ZSTD_freeCDict(cdict);
        return 0 as *mut ZSTD_CDict
    }
    return cdict;
}
unsafe extern "C" fn ZSTD_initCDict_internal(mut cdict: *mut ZSTD_CDict,
                                             mut dictBuffer:
                                                 *const libc::c_void,
                                             mut dictSize: size_t,
                                             mut dictLoadMethod:
                                                 ZSTD_dictLoadMethod_e,
                                             mut dictContentType:
                                                 ZSTD_dictContentType_e,
                                             mut cParams:
                                                 ZSTD_compressionParameters)
 -> size_t {
    (*cdict).matchState.cParams = cParams;
    if dictLoadMethod as libc::c_uint ==
           ZSTD_dlm_byRef as libc::c_int as libc::c_uint ||
           dictBuffer.is_null() || 0 == dictSize {
        (*cdict).dictBuffer = 0 as *mut libc::c_void;
        (*cdict).dictContent = dictBuffer
    } else {
        let internalBuffer: *mut libc::c_void =
            ZSTD_malloc(dictSize, (*cdict).customMem);
        (*cdict).dictBuffer = internalBuffer;
        (*cdict).dictContent = internalBuffer;
        if internalBuffer.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        }
        memcpy(internalBuffer, dictBuffer, dictSize);
    }
    (*cdict).dictContentSize = dictSize;
    ZSTD_reset_compressedBlockState(&mut (*cdict).cBlockState);
    let end: *mut libc::c_void =
        ZSTD_reset_matchState(&mut (*cdict).matchState,
                              ((*cdict).workspace as
                                   *mut U32).offset(((6i32 << 10i32) as
                                                         libc::c_ulong).wrapping_div(::std::mem::size_of::<U32>()
                                                                                         as
                                                                                         libc::c_ulong)
                                                        as isize) as
                                  *mut libc::c_void, &mut cParams,
                              ZSTDcrp_continue, 0i32 as U32);
    let mut params: ZSTD_CCtx_params =
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
    memset(&mut params as *mut ZSTD_CCtx_params as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZSTD_CCtx_params>() as libc::c_ulong);
    params.compressionLevel = 3i32;
    params.fParams.contentSizeFlag = 1i32;
    params.cParams = cParams;
    let dictID: size_t =
        ZSTD_compress_insertDictionary(&mut (*cdict).cBlockState,
                                       &mut (*cdict).matchState, &mut params,
                                       (*cdict).dictContent,
                                       (*cdict).dictContentSize,
                                       dictContentType, ZSTD_dtlm_full,
                                       (*cdict).workspace);
    if 0 != ERR_isError(dictID) { return dictID }
    (*cdict).dictID = dictID as U32;
    return 0i32 as size_t;
}
/* ! ZSTD_compress_usingCDict() :
 *  Compression using a digested Dictionary.
 *  Recommended when same dictionary is used multiple times.
 *  Note : compression level is _decided at dictionary creation time_,
 *     and frame parameters are hardcoded (dictID=yes, contentSize=yes, checksum=no) */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compress_usingCDict(mut cctx: *mut ZSTD_CCtx,
                                                  mut dst: *mut libc::c_void,
                                                  mut dstCapacity: size_t,
                                                  mut src:
                                                      *const libc::c_void,
                                                  mut srcSize: size_t,
                                                  mut cdict:
                                                      *const ZSTD_CDict)
 -> size_t {
    /*content*/
    /*checksum*/
    /*noDictID*/
    let fParams: ZSTD_frameParameters =
        ZSTD_frameParameters{contentSizeFlag: 1i32,
                             checksumFlag: 0i32,
                             noDictIDFlag: 0i32,};
    return ZSTD_compress_usingCDict_advanced(cctx, dst, dstCapacity, src,
                                             srcSize, cdict, fParams);
}
/* ! ZSTD_compress_usingCDict_advanced() :
 *  Same as ZSTD_compress_usingCDict(), with fine-tune control over frame parameters */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compress_usingCDict_advanced(mut cctx:
                                                               *mut ZSTD_CCtx,
                                                           mut dst:
                                                               *mut libc::c_void,
                                                           mut dstCapacity:
                                                               size_t,
                                                           mut src:
                                                               *const libc::c_void,
                                                           mut srcSize:
                                                               size_t,
                                                           mut cdict:
                                                               *const ZSTD_CDict,
                                                           mut fParams:
                                                               ZSTD_frameParameters)
 -> size_t {
    let errcod: size_t =
        ZSTD_compressBegin_usingCDict_advanced(cctx, cdict, fParams,
                                               srcSize as libc::c_ulonglong);
    if 0 != ERR_isError(errcod) { return errcod }
    return ZSTD_compressEnd(cctx, dst, dstCapacity, src, srcSize);
}
/* compression parameters are already set within cdict. pledgedSrcSize must be correct. If srcSize is not known, use macro ZSTD_CONTENTSIZE_UNKNOWN */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBegin_usingCDict_advanced(cctx:
                                                                    *mut ZSTD_CCtx,
                                                                cdict:
                                                                    *const ZSTD_CDict,
                                                                fParams:
                                                                    ZSTD_frameParameters,
                                                                pledgedSrcSize:
                                                                    libc::c_ulonglong)
 -> size_t {
    if cdict.is_null() {
        return -(ZSTD_error_dictionary_wrong as libc::c_int) as size_t
    }
    let mut params: ZSTD_CCtx_params = (*cctx).requestedParams;
    params.cParams = ZSTD_getCParamsFromCDict(cdict);
    if pledgedSrcSize != 0u64.wrapping_sub(1i32 as libc::c_ulonglong) {
        let limitedSrcSize: U32 =
            (if pledgedSrcSize < (1u32 << 19i32) as libc::c_ulonglong {
                 pledgedSrcSize
             } else { (1u32 << 19i32) as libc::c_ulonglong }) as U32;
        let limitedSrcLog: U32 =
            if limitedSrcSize > 1i32 as libc::c_uint {
                ZSTD_highbit32(limitedSrcSize.wrapping_sub(1i32 as
                                                               libc::c_uint)).wrapping_add(1i32
                                                                                               as
                                                                                               libc::c_uint)
            } else { 1i32 as libc::c_uint };
        params.cParams.windowLog =
            if params.cParams.windowLog > limitedSrcLog {
                params.cParams.windowLog
            } else { limitedSrcLog }
    }
    params.fParams = fParams;
    return ZSTD_compressBegin_internal(cctx, 0 as *const libc::c_void,
                                       0i32 as size_t, ZSTD_dct_auto,
                                       ZSTD_dtlm_fast, cdict, params,
                                       pledgedSrcSize as U64,
                                       ZSTDb_not_buffered);
}
/* ! ZSTD_getCParamsFromCDict() :
 *  as the name implies */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getCParamsFromCDict(mut cdict:
                                                      *const ZSTD_CDict)
 -> ZSTD_compressionParameters {
    return (*cdict).matchState.cParams;
}
/*===== ZSTD_CStream management functions =====*/
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCStream() -> *mut ZSTD_CStream {
    return ZSTD_createCStream_advanced(ZSTD_defaultCMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCStream_advanced(mut customMem:
                                                         ZSTD_customMem)
 -> *mut ZSTD_CStream {
    return ZSTD_createCCtx_advanced(customMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeCStream(mut zcs: *mut ZSTD_CStream)
 -> size_t {
    return ZSTD_freeCCtx(zcs);
}
/*===== Streaming compression functions =====*/
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream(mut zcs: *mut ZSTD_CStream,
                                          mut compressionLevel: libc::c_int)
 -> size_t {
    return ZSTD_initCStream_srcSize(zcs, compressionLevel,
                                    0u64.wrapping_sub(1i32 as
                                                          libc::c_ulonglong));
}
/* *******************************************************************
*  Advanced streaming functions
*  Warning : most of these functions are now redundant with the Advanced API.
*  Once Advanced API reaches "stable" status,
*  redundant functions will be deprecated, and then at some point removed.
********************************************************************/
/*=====   Advanced Streaming compression functions  =====*/
/* *< pledgedSrcSize must be correct. If it is not known at init time, use ZSTD_CONTENTSIZE_UNKNOWN. Note that, for compatibility with older programs, "0" also disables frame content size field. It may be enabled in the future. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream_srcSize(mut zcs: *mut ZSTD_CStream,
                                                  mut compressionLevel:
                                                      libc::c_int,
                                                  mut pss: libc::c_ulonglong)
 -> size_t {
    /* temporary : 0 interpreted as "unknown" during transition period. Users willing to specify "unknown" **must** use ZSTD_CONTENTSIZE_UNKNOWN. `0` will be interpreted as "empty" in the future */
    let pledgedSrcSize: U64 =
        (if pss == 0i32 as libc::c_ulonglong {
             0u64.wrapping_sub(1i32 as libc::c_ulonglong)
         } else { pss }) as U64;
    ZSTD_CCtxParams_init(&mut (*zcs).requestedParams, compressionLevel);
    return ZSTD_initCStream_internal(zcs, 0 as *const libc::c_void,
                                     0i32 as size_t, 0 as *const ZSTD_CDict,
                                     (*zcs).requestedParams,
                                     pledgedSrcSize as libc::c_ulonglong);
}
/* ! ZSTD_initCStream_internal() :
 *  Private use only. Init streaming operation.
 *  expects params to be valid.
 *  must receive dict, or cdict, or none, but not both.
 *  @return : 0, or an error code */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream_internal(mut zcs: *mut ZSTD_CStream,
                                                   mut dict:
                                                       *const libc::c_void,
                                                   mut dictSize: size_t,
                                                   mut cdict:
                                                       *const ZSTD_CDict,
                                                   mut params:
                                                       ZSTD_CCtx_params,
                                                   mut pledgedSrcSize:
                                                       libc::c_ulonglong)
 -> size_t {
    params.cParams =
        ZSTD_getCParamsFromCCtxParams(&mut params, pledgedSrcSize as U64,
                                      dictSize);
    if !dict.is_null() && dictSize >= 8i32 as libc::c_ulong {
        if 0 != (*zcs).staticSize {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        }
        ZSTD_freeCDict((*zcs).cdictLocal);
        (*zcs).cdictLocal =
            ZSTD_createCDict_advanced(dict, dictSize, ZSTD_dlm_byCopy,
                                      ZSTD_dct_auto, params.cParams,
                                      (*zcs).customMem);
        (*zcs).cdict = (*zcs).cdictLocal;
        if (*zcs).cdictLocal.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        }
    } else {
        if !cdict.is_null() {
            params.cParams = ZSTD_getCParamsFromCDict(cdict)
        }
        ZSTD_freeCDict((*zcs).cdictLocal);
        (*zcs).cdictLocal = 0 as *mut ZSTD_CDict;
        (*zcs).cdict = cdict
    }
    return ZSTD_resetCStream_internal(zcs, 0 as *const libc::c_void,
                                      0i32 as size_t, ZSTD_dct_auto,
                                      (*zcs).cdict, params, pledgedSrcSize);
}
unsafe extern "C" fn ZSTD_resetCStream_internal(mut cctx: *mut ZSTD_CStream,
                                                dict: *const libc::c_void,
                                                dictSize: size_t,
                                                dictContentType:
                                                    ZSTD_dictContentType_e,
                                                cdict: *const ZSTD_CDict,
                                                mut params: ZSTD_CCtx_params,
                                                pledgedSrcSize:
                                                    libc::c_ulonglong)
 -> size_t {
    params.cParams =
        ZSTD_getCParamsFromCCtxParams(&mut params, pledgedSrcSize as U64,
                                      dictSize);
    let errcod: size_t =
        ZSTD_compressBegin_internal(cctx, dict, dictSize, dictContentType,
                                    ZSTD_dtlm_fast, cdict, params,
                                    pledgedSrcSize as U64, ZSTDb_buffered);
    if 0 != ERR_isError(errcod) { return errcod }
    (*cctx).inToCompress = 0i32 as size_t;
    (*cctx).inBuffPos = 0i32 as size_t;
    (*cctx).inBuffTarget =
        (*cctx).blockSize.wrapping_add(((*cctx).blockSize as libc::c_ulonglong
                                            == pledgedSrcSize) as libc::c_int
                                           as libc::c_ulong);
    (*cctx).outBuffFlushedSize = 0i32 as size_t;
    (*cctx).outBuffContentSize = (*cctx).outBuffFlushedSize;
    (*cctx).streamStage = zcss_load;
    (*cctx).frameEnded = 0i32 as U32;
    return 0i32 as size_t;
}
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
pub unsafe extern "C" fn ZSTD_getCParamsFromCCtxParams(mut CCtxParams:
                                                           *const ZSTD_CCtx_params,
                                                       mut srcSizeHint: U64,
                                                       mut dictSize: size_t)
 -> ZSTD_compressionParameters {
    let mut cParams: ZSTD_compressionParameters =
        ZSTD_getCParams((*CCtxParams).compressionLevel,
                        srcSizeHint as libc::c_ulonglong, dictSize);
    if 0 != (*CCtxParams).ldmParams.enableLdm {
        cParams.windowLog = 27i32 as libc::c_uint
    }
    if 0 != (*CCtxParams).cParams.windowLog {
        cParams.windowLog = (*CCtxParams).cParams.windowLog
    }
    if 0 != (*CCtxParams).cParams.hashLog {
        cParams.hashLog = (*CCtxParams).cParams.hashLog
    }
    if 0 != (*CCtxParams).cParams.chainLog {
        cParams.chainLog = (*CCtxParams).cParams.chainLog
    }
    if 0 != (*CCtxParams).cParams.searchLog {
        cParams.searchLog = (*CCtxParams).cParams.searchLog
    }
    if 0 != (*CCtxParams).cParams.minMatch {
        cParams.minMatch = (*CCtxParams).cParams.minMatch
    }
    if 0 != (*CCtxParams).cParams.targetLength {
        cParams.targetLength = (*CCtxParams).cParams.targetLength
    }
    if 0 != (*CCtxParams).cParams.strategy as u64 {
        cParams.strategy = (*CCtxParams).cParams.strategy
    }
    return ZSTD_adjustCParams_internal(cParams,
                                       srcSizeHint as libc::c_ulonglong,
                                       dictSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressStream(mut zcs: *mut ZSTD_CStream,
                                             mut output: *mut ZSTD_outBuffer,
                                             mut input: *mut ZSTD_inBuffer)
 -> size_t {
    let errcod: size_t =
        ZSTD_compressStream2(zcs, output, input, ZSTD_e_continue);
    if 0 != ERR_isError(errcod) { return errcod }
    return ZSTD_nextInputSizeHint_MTorST(zcs);
}
unsafe extern "C" fn ZSTD_nextInputSizeHint_MTorST(mut cctx: *const ZSTD_CCtx)
 -> size_t {
    if (*cctx).appliedParams.nbWorkers >= 1i32 {
        return ZSTDMT_nextInputSizeHint((*cctx).mtctx)
    }
    return ZSTD_nextInputSizeHint(cctx);
}
/*======   Compression   ======*/
unsafe extern "C" fn ZSTD_nextInputSizeHint(mut cctx: *const ZSTD_CCtx)
 -> size_t {
    let mut hintInSize: size_t =
        (*cctx).inBuffTarget.wrapping_sub((*cctx).inBuffPos);
    if hintInSize == 0i32 as libc::c_ulong { hintInSize = (*cctx).blockSize }
    return hintInSize;
}
/* ! ZSTD_compressStream2() :
 *  Behaves about the same as ZSTD_compressStream, with additional control on end directive.
 *  - Compression parameters are pushed into CCtx before starting compression, using ZSTD_CCtx_set*()
 *  - Compression parameters cannot be changed once compression is started (save a list of exceptions in multi-threading mode)
 *  - outpot->pos must be <= dstCapacity, input->pos must be <= srcSize
 *  - outpot->pos and input->pos will be updated. They are guaranteed to remain below their respective limit.
 *  - When nbWorkers==0 (default), function is blocking : it completes its job before returning to caller.
 *  - When nbWorkers>=1, function is non-blocking : it just acquires a copy of input, and distributes jobs to internal worker threads, flush whatever is available,
 *                                                  and then immediately returns, just indicating that there is some data remaining to be flushed.
 *                                                  The function nonetheless guarantees forward progress : it will return only after it reads or write at least 1+ byte.
 *  - Exception : if the first call requests a ZSTD_e_end directive and provides enough dstCapacity, the function delegates to ZSTD_compress2() which is always blocking.
 *  - @return provides a minimum amount of data remaining to be flushed from internal buffers
 *            or an error code, which can be tested using ZSTD_isError().
 *            if @return != 0, flush is not fully completed, there is still some data left within internal buffers.
 *            This is useful for ZSTD_e_flush, since in this case more flushes are necessary to empty all buffers.
 *            For ZSTD_e_end, @return == 0 when internal buffers are fully flushed and frame is completed.
 *  - after a ZSTD_e_end directive, if internal buffer is not fully flushed (@return != 0),
 *            only ZSTD_e_end or ZSTD_e_flush operations are allowed.
 *            Before starting a new compression job, or changing compression parameters,
 *            it is required to fully flush internal buffers.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressStream2(mut cctx: *mut ZSTD_CCtx,
                                              mut output: *mut ZSTD_outBuffer,
                                              mut input: *mut ZSTD_inBuffer,
                                              mut endOp: ZSTD_EndDirective)
 -> size_t {
    if (*output).pos > (*output).size {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if (*input).pos > (*input).size {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if (*cctx).streamStage as libc::c_uint ==
           zcss_init as libc::c_int as libc::c_uint {
        let mut params: ZSTD_CCtx_params = (*cctx).requestedParams;
        let prefixDict: ZSTD_prefixDict = (*cctx).prefixDict;
        memset(&mut (*cctx).prefixDict as *mut ZSTD_prefixDict as
                   *mut libc::c_void, 0i32,
               ::std::mem::size_of::<ZSTD_prefixDict>() as libc::c_ulong);
        if endOp as libc::c_uint == ZSTD_e_end as libc::c_int as libc::c_uint
           {
            (*cctx).pledgedSrcSizePlusOne =
                (*input).size.wrapping_add(1i32 as libc::c_ulong) as
                    libc::c_ulonglong
        }
        params.cParams =
            ZSTD_getCParamsFromCCtxParams(&mut (*cctx).requestedParams,
                                          (*cctx).pledgedSrcSizePlusOne.wrapping_sub(1i32
                                                                                         as
                                                                                         libc::c_ulonglong)
                                              as U64, 0i32 as size_t);
        if (*cctx).pledgedSrcSizePlusOne.wrapping_sub(1i32 as
                                                          libc::c_ulonglong)
               <= (1i32 * (1i32 << 20i32)) as libc::c_ulonglong {
            params.nbWorkers = 0i32
        }
        if params.nbWorkers > 0i32 {
            if (*cctx).mtctx.is_null() {
                (*cctx).mtctx =
                    ZSTDMT_createCCtx_advanced(params.nbWorkers as
                                                   libc::c_uint,
                                               (*cctx).customMem);
                if (*cctx).mtctx.is_null() {
                    return -(ZSTD_error_memory_allocation as libc::c_int) as
                               size_t
                }
            }
            let errcod: size_t =
                ZSTDMT_initCStream_internal((*cctx).mtctx, prefixDict.dict,
                                            prefixDict.dictSize,
                                            ZSTD_dct_rawContent,
                                            (*cctx).cdict, params,
                                            (*cctx).pledgedSrcSizePlusOne.wrapping_sub(1i32
                                                                                           as
                                                                                           libc::c_ulonglong));
            if 0 != ERR_isError(errcod) { return errcod }
            (*cctx).streamStage = zcss_load;
            (*cctx).appliedParams.nbWorkers = params.nbWorkers
        } else {
            let errcod_0: size_t =
                ZSTD_resetCStream_internal(cctx, prefixDict.dict,
                                           prefixDict.dictSize,
                                           prefixDict.dictContentType,
                                           (*cctx).cdict, params,
                                           (*cctx).pledgedSrcSizePlusOne.wrapping_sub(1i32
                                                                                          as
                                                                                          libc::c_ulonglong));
            if 0 != ERR_isError(errcod_0) { return errcod_0 }
        }
    }
    if (*cctx).appliedParams.nbWorkers > 0i32 {
        if 0 != (*cctx).cParamsChanged {
            ZSTDMT_updateCParams_whileCompressing((*cctx).mtctx,
                                                  &mut (*cctx).requestedParams);
            (*cctx).cParamsChanged = 0i32
        }
        let flushMin: size_t =
            ZSTDMT_compressStream_generic((*cctx).mtctx, output, input,
                                          endOp);
        if 0 != ERR_isError(flushMin) ||
               endOp as libc::c_uint ==
                   ZSTD_e_end as libc::c_int as libc::c_uint &&
                   flushMin == 0i32 as libc::c_ulong {
            ZSTD_CCtx_reset(cctx, ZSTD_reset_session_only);
        }
        return flushMin
    }
    let errcod_1: size_t =
        ZSTD_compressStream_generic(cctx, output, input, endOp);
    if 0 != ERR_isError(errcod_1) { return errcod_1 }
    return (*cctx).outBuffContentSize.wrapping_sub((*cctx).outBuffFlushedSize);
}
/* ! ZSTD_compressStream_generic() :
 *  Private use only. To be called from zstdmt_compress.c in single-thread mode. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressStream_generic(mut zcs:
                                                         *mut ZSTD_CStream,
                                                     mut output:
                                                         *mut ZSTD_outBuffer,
                                                     mut input:
                                                         *mut ZSTD_inBuffer,
                                                     flushMode:
                                                         ZSTD_EndDirective)
 -> size_t {
    let istart: *const libc::c_char = (*input).src as *const libc::c_char;
    let iend: *const libc::c_char = istart.offset((*input).size as isize);
    let mut ip: *const libc::c_char = istart.offset((*input).pos as isize);
    let ostart: *mut libc::c_char = (*output).dst as *mut libc::c_char;
    let oend: *mut libc::c_char = ostart.offset((*output).size as isize);
    let mut op: *mut libc::c_char = ostart.offset((*output).pos as isize);
    let mut someMoreWork: U32 = 1i32 as U32;
    while 0 != someMoreWork {
        let mut current_block_63: u64;
        match (*zcs).streamStage as libc::c_uint {
            0 => {
                return -(ZSTD_error_init_missing as libc::c_int) as size_t
            }
            1 => {
                if flushMode as libc::c_uint ==
                       ZSTD_e_end as libc::c_int as libc::c_uint &&
                       oend.wrapping_offset_from(op) as libc::c_long as size_t
                           >=
                           ZSTD_compressBound(iend.wrapping_offset_from(ip) as
                                                  libc::c_long as size_t) &&
                       (*zcs).inBuffPos == 0i32 as libc::c_ulong {
                    /* enough dstCapacity */
                    /* shortcut to compression pass directly into output buffer */
                    let cSize: size_t =
                        ZSTD_compressEnd(zcs, op as *mut libc::c_void,
                                         oend.wrapping_offset_from(op) as
                                             libc::c_long as size_t,
                                         ip as *const libc::c_void,
                                         iend.wrapping_offset_from(ip) as
                                             libc::c_long as size_t);
                    if 0 != ERR_isError(cSize) { return cSize }
                    ip = iend;
                    op = op.offset(cSize as isize);
                    (*zcs).frameEnded = 1i32 as U32;
                    ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
                    someMoreWork = 0i32 as U32;
                    current_block_63 = 2754258178208450300;
                } else {
                    /* complete loading into inBuffer */
                    let toLoad: size_t =
                        (*zcs).inBuffTarget.wrapping_sub((*zcs).inBuffPos);
                    let loaded: size_t =
                        ZSTD_limitCopy((*zcs).inBuff.offset((*zcs).inBuffPos
                                                                as isize) as
                                           *mut libc::c_void, toLoad,
                                       ip as *const libc::c_void,
                                       iend.wrapping_offset_from(ip) as
                                           libc::c_long as size_t);
                    (*zcs).inBuffPos =
                        ((*zcs).inBuffPos as
                             libc::c_ulong).wrapping_add(loaded) as size_t as
                            size_t;
                    ip = ip.offset(loaded as isize);
                    if flushMode as libc::c_uint ==
                           ZSTD_e_continue as libc::c_int as libc::c_uint &&
                           (*zcs).inBuffPos < (*zcs).inBuffTarget {
                        someMoreWork = 0i32 as U32;
                        current_block_63 = 2754258178208450300;
                    } else if flushMode as libc::c_uint ==
                                  ZSTD_e_flush as libc::c_int as libc::c_uint
                                  && (*zcs).inBuffPos == (*zcs).inToCompress {
                        someMoreWork = 0i32 as U32;
                        current_block_63 = 2754258178208450300;
                    } else {
                        let mut cDst: *mut libc::c_void =
                            0 as *mut libc::c_void;
                        let mut cSize_0: size_t = 0;
                        let iSize: size_t =
                            (*zcs).inBuffPos.wrapping_sub((*zcs).inToCompress);
                        let mut oSize: size_t =
                            oend.wrapping_offset_from(op) as libc::c_long as
                                size_t;
                        let lastBlock: libc::c_uint =
                            (flushMode as libc::c_uint ==
                                 ZSTD_e_end as libc::c_int as libc::c_uint &&
                                 ip == iend) as libc::c_int as libc::c_uint;
                        if oSize >= ZSTD_compressBound(iSize) {
                            cDst = op as *mut libc::c_void
                        } else {
                            cDst = (*zcs).outBuff as *mut libc::c_void;
                            oSize = (*zcs).outBuffSize
                        }
                        cSize_0 =
                            if 0 != lastBlock {
                                ZSTD_compressEnd(zcs, cDst, oSize,
                                                 (*zcs).inBuff.offset((*zcs).inToCompress
                                                                          as
                                                                          isize)
                                                     as *const libc::c_void,
                                                 iSize)
                            } else {
                                ZSTD_compressContinue(zcs, cDst, oSize,
                                                      (*zcs).inBuff.offset((*zcs).inToCompress
                                                                               as
                                                                               isize)
                                                          as
                                                          *const libc::c_void,
                                                      iSize)
                            };
                        if 0 != ERR_isError(cSize_0) { return cSize_0 }
                        (*zcs).frameEnded = lastBlock;
                        (*zcs).inBuffTarget =
                            (*zcs).inBuffPos.wrapping_add((*zcs).blockSize);
                        if (*zcs).inBuffTarget > (*zcs).inBuffSize {
                            (*zcs).inBuffPos = 0i32 as size_t;
                            (*zcs).inBuffTarget = (*zcs).blockSize
                        }
                        0 == lastBlock;
                        (*zcs).inToCompress = (*zcs).inBuffPos;
                        if cDst == op as *mut libc::c_void {
                            /* no need to flush */
                            op = op.offset(cSize_0 as isize);
                            if 0 != (*zcs).frameEnded {
                                someMoreWork = 0i32 as U32;
                                ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
                            }
                            current_block_63 = 2754258178208450300;
                        } else {
                            (*zcs).outBuffContentSize = cSize_0;
                            (*zcs).outBuffFlushedSize = 0i32 as size_t;
                            (*zcs).streamStage = zcss_flush;
                            /* fall-through */
                            current_block_63 = 9437013279121998969;
                        }
                    }
                }
            }
            2 => { current_block_63 = 9437013279121998969; }
            _ => { current_block_63 = 2754258178208450300; }
        }
        match current_block_63 {
            9437013279121998969 => {
                let toFlush: size_t =
                    (*zcs).outBuffContentSize.wrapping_sub((*zcs).outBuffFlushedSize);
                let flushed: size_t =
                    ZSTD_limitCopy(op as *mut libc::c_void,
                                   oend.wrapping_offset_from(op) as
                                       libc::c_long as size_t,
                                   (*zcs).outBuff.offset((*zcs).outBuffFlushedSize
                                                             as isize) as
                                       *const libc::c_void, toFlush);
                op = op.offset(flushed as isize);
                (*zcs).outBuffFlushedSize =
                    ((*zcs).outBuffFlushedSize as
                         libc::c_ulong).wrapping_add(flushed) as size_t as
                        size_t;
                if toFlush != flushed {
                    someMoreWork = 0i32 as U32
                } else {
                    (*zcs).outBuffFlushedSize = 0i32 as size_t;
                    (*zcs).outBuffContentSize = (*zcs).outBuffFlushedSize;
                    if 0 != (*zcs).frameEnded {
                        someMoreWork = 0i32 as U32;
                        ZSTD_CCtx_reset(zcs, ZSTD_reset_session_only);
                    } else { (*zcs).streamStage = zcss_load }
                }
            }
            _ => { }
        }
    }
    (*input).pos = ip.wrapping_offset_from(istart) as libc::c_long as size_t;
    (*output).pos = op.wrapping_offset_from(ostart) as libc::c_long as size_t;
    if 0 != (*zcs).frameEnded { return 0i32 as size_t }
    return ZSTD_nextInputSizeHint(zcs);
}
unsafe extern "C" fn ZSTD_limitCopy(mut dst: *mut libc::c_void,
                                    mut dstCapacity: size_t,
                                    mut src: *const libc::c_void,
                                    mut srcSize: size_t) -> size_t {
    let length: size_t =
        if dstCapacity < srcSize { dstCapacity } else { srcSize };
    if 0 != length { memcpy(dst, src, length); }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressContinue(mut cctx: *mut ZSTD_CCtx,
                                               mut dst: *mut libc::c_void,
                                               mut dstCapacity: size_t,
                                               mut src: *const libc::c_void,
                                               mut srcSize: size_t)
 -> size_t {
    return ZSTD_compressContinue_internal(cctx, dst, dstCapacity, src,
                                          srcSize, 1i32 as U32, 0i32 as U32);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_flushStream(mut zcs: *mut ZSTD_CStream,
                                          mut output: *mut ZSTD_outBuffer)
 -> size_t {
    let mut input: ZSTD_inBuffer =
        ZSTD_inBuffer_s{src: 0 as *const libc::c_void,
                        size: 0i32 as size_t,
                        pos: 0i32 as size_t,};
    return ZSTD_compressStream2(zcs, output, &mut input, ZSTD_e_flush);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_endStream(mut zcs: *mut ZSTD_CStream,
                                        mut output: *mut ZSTD_outBuffer)
 -> size_t {
    let mut input: ZSTD_inBuffer =
        ZSTD_inBuffer_s{src: 0 as *const libc::c_void,
                        size: 0i32 as size_t,
                        pos: 0i32 as size_t,};
    let remainingToFlush: size_t =
        ZSTD_compressStream2(zcs, output, &mut input, ZSTD_e_end);
    let errcod: size_t = remainingToFlush;
    if 0 != ERR_isError(errcod) { return errcod }
    if (*zcs).appliedParams.nbWorkers > 0i32 { return remainingToFlush }
    let lastBlockSize: size_t =
        (if 0 != (*zcs).frameEnded { 0i32 } else { 3i32 }) as size_t;
    let checksumSize: size_t =
        (if 0 != (*zcs).frameEnded {
             0i32
         } else { (*zcs).appliedParams.fParams.checksumFlag * 4i32 }) as
            size_t;
    let toFlush: size_t =
        remainingToFlush.wrapping_add(lastBlockSize).wrapping_add(checksumSize);
    return toFlush;
}
/* *< recommended size for input buffer */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CStreamInSize() -> size_t {
    return (1i32 << 17i32) as size_t;
}
/* *< recommended size for output buffer. Guarantee to successfully flush at least one complete compressed block in all circumstances. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CStreamOutSize() -> size_t {
    return ZSTD_compressBound((1i32 << 17i32) as
                                  size_t).wrapping_add(ZSTD_blockHeaderSize).wrapping_add(4i32
                                                                                              as
                                                                                              libc::c_ulong);
}
/* ZSTD_H_235446 */
/* ***************************************************************************************
 *   ADVANCED AND EXPERIMENTAL FUNCTIONS
 ****************************************************************************************
 * The definitions in the following section are considered experimental.
 * They are provided for advanced scenarios.
 * They should never be used with a dynamic library, as prototypes may change in the future.
 * Use them only in association with static linking.
 * ***************************************************************************************/
/* ***************************************************************************************
 *   Candidate API for promotion to stable status
 ****************************************************************************************
 * The following symbols and constants form the "staging area" :
 * they are considered to join "stable API" by v1.4.0.
 * The proposal is written so that it can be made stable "as is",
 * though it's still possible to suggest improvements.
 * Staging is in fact last chance for changes,
 * the API is locked once reaching "stable" status.
 * ***************************************************************************************/
/* ===  Constants   === */
/* all magic numbers are supposed read/written to/from files/memory using little-endian convention */
/* valid since v0.8.0 */
/* valid since v0.7.0 */
/* all 16 values, from 0x184D2A50 to 0x184D2A5F, signal the beginning of a skippable frame */
/* ===   query limits   === */
/* !< minimum negative compression level allowed */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_minCLevel() -> libc::c_int {
    return -(1i32 << 17i32);
}
/* ===   Memory management   === */
/* ! ZSTD_sizeof_*() :
 *  These functions give the _current_ memory usage of selected object.
 *  Note that object memory usage can evolve (increase or decrease) over time. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_CCtx(mut cctx: *const ZSTD_CCtx)
 -> size_t {
    if cctx.is_null() { return 0i32 as size_t }
    return (::std::mem::size_of::<ZSTD_CCtx>() as
                libc::c_ulong).wrapping_add((*cctx).workSpaceSize).wrapping_add(ZSTD_sizeof_CDict((*cctx).cdictLocal)).wrapping_add(ZSTD_sizeof_mtctx(cctx));
}
unsafe extern "C" fn ZSTD_sizeof_mtctx(mut cctx: *const ZSTD_CCtx) -> size_t {
    return ZSTDMT_sizeof_CCtx((*cctx).mtctx);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_CDict(mut cdict: *const ZSTD_CDict)
 -> size_t {
    if cdict.is_null() { return 0i32 as size_t }
    return (*cdict).workspaceSize.wrapping_add(if !(*cdict).dictBuffer.is_null()
                                                  {
                                                   (*cdict).dictContentSize
                                               } else {
                                                   0i32 as libc::c_ulong
                                               }).wrapping_add(::std::mem::size_of::<ZSTD_CDict>()
                                                                   as
                                                                   libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_CStream(mut zcs: *const ZSTD_CStream)
 -> size_t {
    return ZSTD_sizeof_CCtx(zcs);
}
/* ! ZSTD_cParam_getBounds() :
 *  All parameters must belong to an interval with lower and upper bounds,
 *  otherwise they will either trigger an error or be automatically clamped.
 * @return : a structure, ZSTD_bounds, which contains
 *         - an error status field, which must be tested using ZSTD_isError()
 *         - lower and upper bounds, both inclusive
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_cParam_getBounds(mut param: ZSTD_cParameter)
 -> ZSTD_bounds {
    let mut bounds: ZSTD_bounds =
        ZSTD_bounds{error: 0i32 as size_t,
                    lowerBound: 0i32,
                    upperBound: 0i32,};
    match param as libc::c_uint {
        100 => {
            bounds.lowerBound = ZSTD_minCLevel();
            bounds.upperBound = ZSTD_maxCLevel();
            return bounds
        }
        101 => {
            bounds.lowerBound = 10i32;
            bounds.upperBound =
                if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                       4i32 as libc::c_ulong {
                    30i32
                } else { 31i32 };
            return bounds
        }
        102 => {
            bounds.lowerBound = 6i32;
            bounds.upperBound =
                if if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                          4i32 as libc::c_ulong {
                       30i32
                   } else { 31i32 } < 30i32 {
                    if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                           4i32 as libc::c_ulong {
                        30i32
                    } else { 31i32 }
                } else { 30i32 };
            return bounds
        }
        103 => {
            bounds.lowerBound = 6i32;
            bounds.upperBound =
                if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                       4i32 as libc::c_ulong {
                    29i32
                } else { 30i32 };
            return bounds
        }
        104 => {
            bounds.lowerBound = 1i32;
            bounds.upperBound =
                if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                       4i32 as libc::c_ulong {
                    30i32
                } else { 31i32 } - 1i32;
            return bounds
        }
        105 => {
            bounds.lowerBound = 3i32;
            bounds.upperBound = 7i32;
            return bounds
        }
        106 => {
            bounds.lowerBound = 0i32;
            bounds.upperBound = 1i32 << 17i32;
            return bounds
        }
        107 => {
            bounds.lowerBound = ZSTD_fast as libc::c_int;
            bounds.upperBound = ZSTD_btultra2 as libc::c_int;
            return bounds
        }
        200 => {
            bounds.lowerBound = 0i32;
            bounds.upperBound = 1i32;
            return bounds
        }
        201 => {
            bounds.lowerBound = 0i32;
            bounds.upperBound = 1i32;
            return bounds
        }
        202 => {
            bounds.lowerBound = 0i32;
            bounds.upperBound = 1i32;
            return bounds
        }
        400 => {
            bounds.lowerBound = 0i32;
            bounds.upperBound = 200i32;
            return bounds
        }
        401 => {
            bounds.lowerBound = 0i32;
            bounds.upperBound =
                if 0 != MEM_32bits() {
                    512i32 * (1i32 << 20i32)
                } else { 1024i32 * (1i32 << 20i32) };
            return bounds
        }
        402 => {
            bounds.lowerBound = 0i32;
            bounds.upperBound = 9i32;
            return bounds
        }
        160 => {
            bounds.lowerBound = 0i32;
            bounds.upperBound = 1i32;
            return bounds
        }
        161 => {
            bounds.lowerBound = 6i32;
            bounds.upperBound =
                if if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                          4i32 as libc::c_ulong {
                       30i32
                   } else { 31i32 } < 30i32 {
                    if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                           4i32 as libc::c_ulong {
                        30i32
                    } else { 31i32 }
                } else { 30i32 };
            return bounds
        }
        162 => {
            bounds.lowerBound = 4i32;
            bounds.upperBound = 4096i32;
            return bounds
        }
        163 => {
            bounds.lowerBound = 1i32;
            bounds.upperBound = 8i32;
            return bounds
        }
        164 => {
            bounds.lowerBound = 0i32;
            bounds.upperBound =
                if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                       4i32 as libc::c_ulong {
                    30i32
                } else { 31i32 } - 6i32;
            return bounds
        }
        500 => {
            bounds.lowerBound = 0i32;
            bounds.upperBound = 1i32;
            return bounds
        }
        1000 => {
            bounds.lowerBound = 0i32;
            bounds.upperBound = 1i32;
            return bounds
        }
        10 => {
            bounds.lowerBound = ZSTD_f_zstd1 as libc::c_int;
            bounds.upperBound = ZSTD_f_zstd1_magicless as libc::c_int;
            return bounds
        }
        1001 => {
            bounds.lowerBound = ZSTD_dictDefaultAttach as libc::c_int;
            bounds.upperBound = ZSTD_dictForceCopy as libc::c_int;
            return bounds
        }
        _ => {
            let boundError: ZSTD_bounds =
                ZSTD_bounds{error:
                                -(ZSTD_error_parameter_unsupported as
                                      libc::c_int) as size_t,
                            lowerBound: 0i32,
                            upperBound: 0i32,};
            return boundError
        }
    };
}
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
pub unsafe extern "C" fn ZSTD_CCtx_setParameter(mut cctx: *mut ZSTD_CCtx,
                                                mut param: ZSTD_cParameter,
                                                mut value: libc::c_int)
 -> size_t {
    if (*cctx).streamStage as libc::c_uint !=
           zcss_init as libc::c_int as libc::c_uint {
        if 0 != ZSTD_isUpdateAuthorized(param) {
            (*cctx).cParamsChanged = 1i32
        } else { return -(ZSTD_error_stage_wrong as libc::c_int) as size_t }
    }
    match param as libc::c_uint {
        10 => {
            return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams,
                                               param, value)
        }
        100 => {
            if !(*cctx).cdict.is_null() {
                return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
            }
            return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams,
                                               param, value)
        }
        101 | 102 | 103 | 104 | 105 | 106 | 107 => {
            if !(*cctx).cdict.is_null() {
                return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
            }
            return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams,
                                               param, value)
        }
        200 | 201 | 202 => {
            return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams,
                                               param, value)
        }
        1000 => {
            return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams,
                                               param, value)
        }
        1001 => {
            return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams,
                                               param, value)
        }
        400 => {
            if value != 0i32 && 0 != (*cctx).staticSize {
                return -(ZSTD_error_parameter_unsupported as libc::c_int) as
                           size_t
            }
            return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams,
                                               param, value)
        }
        401 | 402 | 500 => {
            return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams,
                                               param, value)
        }
        160 | 161 | 162 | 163 | 164 => {
            if !(*cctx).cdict.is_null() {
                return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
            }
            return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams,
                                               param, value)
        }
        _ => {
            return -(ZSTD_error_parameter_unsupported as libc::c_int) as
                       size_t
        }
    };
}
/* ! ZSTD_CCtxParam_setParameter() :
 *  Similar to ZSTD_CCtx_setParameter.
 *  Set one compression parameter, selected by enum ZSTD_cParameter.
 *  Parameters must be applied to a ZSTD_CCtx using ZSTD_CCtx_setParametersUsingCCtxParams().
 * @result : 0, or an error code (which can be tested with ZSTD_isError()).
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtxParam_setParameter(mut CCtxParams:
                                                         *mut ZSTD_CCtx_params,
                                                     mut param:
                                                         ZSTD_cParameter,
                                                     mut value: libc::c_int)
 -> size_t {
    match param as libc::c_uint {
        10 => {
            if 0 == ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam2, value)
               {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as
                           size_t
            }
            (*CCtxParams).format = value as ZSTD_format_e;
            return (*CCtxParams).format as size_t
        }
        100 => {
            let mut cLevel: libc::c_int = value;
            if cLevel > ZSTD_maxCLevel() { cLevel = ZSTD_maxCLevel() }
            if cLevel < ZSTD_minCLevel() { cLevel = ZSTD_minCLevel() }
            if 0 != cLevel { (*CCtxParams).compressionLevel = cLevel }
            if (*CCtxParams).compressionLevel >= 0i32 {
                return (*CCtxParams).compressionLevel as size_t
            }
            return 0i32 as size_t
        }
        101 => {
            if value != 0i32 {
                if 0 == ZSTD_cParam_withinBounds(ZSTD_c_windowLog, value) {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).cParams.windowLog = value as libc::c_uint;
            return (*CCtxParams).cParams.windowLog as size_t
        }
        102 => {
            if value != 0i32 {
                if 0 == ZSTD_cParam_withinBounds(ZSTD_c_hashLog, value) {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).cParams.hashLog = value as libc::c_uint;
            return (*CCtxParams).cParams.hashLog as size_t
        }
        103 => {
            if value != 0i32 {
                if 0 == ZSTD_cParam_withinBounds(ZSTD_c_chainLog, value) {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).cParams.chainLog = value as libc::c_uint;
            return (*CCtxParams).cParams.chainLog as size_t
        }
        104 => {
            if value != 0i32 {
                if 0 == ZSTD_cParam_withinBounds(ZSTD_c_searchLog, value) {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).cParams.searchLog = value as libc::c_uint;
            return value as size_t
        }
        105 => {
            if value != 0i32 {
                if 0 == ZSTD_cParam_withinBounds(ZSTD_c_minMatch, value) {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).cParams.minMatch = value as libc::c_uint;
            return (*CCtxParams).cParams.minMatch as size_t
        }
        106 => {
            if 0 == ZSTD_cParam_withinBounds(ZSTD_c_targetLength, value) {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as
                           size_t
            }
            (*CCtxParams).cParams.targetLength = value as libc::c_uint;
            return (*CCtxParams).cParams.targetLength as size_t
        }
        107 => {
            if value != 0i32 {
                if 0 == ZSTD_cParam_withinBounds(ZSTD_c_strategy, value) {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).cParams.strategy = value as ZSTD_strategy;
            return (*CCtxParams).cParams.strategy as size_t
        }
        200 => {
            (*CCtxParams).fParams.contentSizeFlag =
                (value != 0i32) as libc::c_int;
            return (*CCtxParams).fParams.contentSizeFlag as size_t
        }
        201 => {
            (*CCtxParams).fParams.checksumFlag =
                (value != 0i32) as libc::c_int;
            return (*CCtxParams).fParams.checksumFlag as size_t
        }
        202 => {
            (*CCtxParams).fParams.noDictIDFlag = (0 == value) as libc::c_int;
            return (0 == (*CCtxParams).fParams.noDictIDFlag) as libc::c_int as
                       size_t
        }
        1000 => {
            (*CCtxParams).forceWindow = (value != 0i32) as libc::c_int;
            return (*CCtxParams).forceWindow as size_t
        }
        1001 => {
            let pref: ZSTD_dictAttachPref_e = value as ZSTD_dictAttachPref_e;
            if 0 ==
                   ZSTD_cParam_withinBounds(ZSTD_c_experimentalParam4,
                                            pref as libc::c_int) {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as
                           size_t
            }
            (*CCtxParams).attachDictPref = pref;
            return (*CCtxParams).attachDictPref as size_t
        }
        400 => {
            return ZSTDMT_CCtxParam_setNbWorkers(CCtxParams,
                                                 value as libc::c_uint)
        }
        401 => {
            return ZSTDMT_CCtxParam_setMTCtxParameter(CCtxParams,
                                                      ZSTDMT_p_jobSize, value)
        }
        402 => {
            return ZSTDMT_CCtxParam_setMTCtxParameter(CCtxParams,
                                                      ZSTDMT_p_overlapLog,
                                                      value)
        }
        500 => {
            return ZSTDMT_CCtxParam_setMTCtxParameter(CCtxParams,
                                                      ZSTDMT_p_rsyncable,
                                                      value)
        }
        160 => {
            (*CCtxParams).ldmParams.enableLdm =
                (value != 0i32) as libc::c_int as U32;
            return (*CCtxParams).ldmParams.enableLdm as size_t
        }
        161 => {
            if value != 0i32 {
                if 0 == ZSTD_cParam_withinBounds(ZSTD_c_ldmHashLog, value) {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).ldmParams.hashLog = value as U32;
            return (*CCtxParams).ldmParams.hashLog as size_t
        }
        162 => {
            if value != 0i32 {
                if 0 == ZSTD_cParam_withinBounds(ZSTD_c_ldmMinMatch, value) {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).ldmParams.minMatchLength = value as U32;
            return (*CCtxParams).ldmParams.minMatchLength as size_t
        }
        163 => {
            if value != 0i32 {
                if 0 ==
                       ZSTD_cParam_withinBounds(ZSTD_c_ldmBucketSizeLog,
                                                value) {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).ldmParams.bucketSizeLog = value as U32;
            return (*CCtxParams).ldmParams.bucketSizeLog as size_t
        }
        164 => {
            if value >
                   if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                          4i32 as libc::c_ulong {
                       30i32
                   } else { 31i32 } - 6i32 {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as
                           size_t
            }
            (*CCtxParams).ldmParams.hashRateLog = value as U32;
            return (*CCtxParams).ldmParams.hashRateLog as size_t
        }
        _ => {
            return -(ZSTD_error_parameter_unsupported as libc::c_int) as
                       size_t
        }
    };
}
/* ZSTD_cParam_withinBounds:
 * @return 1 if value is within cParam bounds,
 * 0 otherwise */
unsafe extern "C" fn ZSTD_cParam_withinBounds(mut cParam: ZSTD_cParameter,
                                              mut value: libc::c_int)
 -> libc::c_int {
    let bounds: ZSTD_bounds = ZSTD_cParam_getBounds(cParam);
    if 0 != ERR_isError(bounds.error) { return 0i32 }
    if value < bounds.lowerBound { return 0i32 }
    if value > bounds.upperBound { return 0i32 }
    return 1i32;
}
unsafe extern "C" fn ZSTD_isUpdateAuthorized(mut param: ZSTD_cParameter)
 -> libc::c_int {
    match param as libc::c_uint {
        100 | 102 | 103 | 104 | 105 | 106 | 107 => { return 1i32 }
        10 | 101 | 200 | 201 | 202 | 1000 | 400 | 401 | 402 | 500 | 160 | 161
        | 162 | 163 | 164 | 1001 | _ => {
            return 0i32
        }
    };
}
/* ! ZSTD_CCtx_setPledgedSrcSize() :
 *  Total input data size to be compressed as a single frame.
 *  Value will be written in frame header, unless if explicitly forbidden using ZSTD_c_contentSizeFlag.
 *  This value will also be controlled at end of frame, and trigger an error if not respected.
 * @result : 0, or an error code (which can be tested with ZSTD_isError()).
 *  Note 1 : pledgedSrcSize==0 actually means zero, aka an empty frame.
 *           In order to mean "unknown content size", pass constant ZSTD_CONTENTSIZE_UNKNOWN.
 *           ZSTD_CONTENTSIZE_UNKNOWN is default value for any new frame.
 *  Note 2 : pledgedSrcSize is only valid once, for the next frame.
 *           It's discarded at the end of the frame, and replaced by ZSTD_CONTENTSIZE_UNKNOWN.
 *  Note 3 : Whenever all input data is provided and consumed in a single round,
 *           for example with ZSTD_compress2(),
 *           or invoking immediately ZSTD_compressStream2(,,,ZSTD_e_end),
 *           this value is automatically overriden by srcSize instead.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_setPledgedSrcSize(mut cctx: *mut ZSTD_CCtx,
                                                     mut pledgedSrcSize:
                                                         libc::c_ulonglong)
 -> size_t {
    if (*cctx).streamStage as libc::c_uint !=
           zcss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    }
    (*cctx).pledgedSrcSizePlusOne =
        pledgedSrcSize.wrapping_add(1i32 as libc::c_ulonglong);
    return 0i32 as size_t;
}
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
pub unsafe extern "C" fn ZSTD_CCtx_loadDictionary(mut cctx: *mut ZSTD_CCtx,
                                                  mut dict:
                                                      *const libc::c_void,
                                                  mut dictSize: size_t)
 -> size_t {
    return ZSTD_CCtx_loadDictionary_advanced(cctx, dict, dictSize,
                                             ZSTD_dlm_byCopy, ZSTD_dct_auto);
}
/* ! ZSTD_CCtx_loadDictionary_advanced() :
 *  Same as ZSTD_CCtx_loadDictionary(), but gives finer control over
 *  how to load the dictionary (by copy ? by reference ?)
 *  and how to interpret it (automatic ? force raw mode ? full mode only ?) */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_loadDictionary_advanced(mut cctx:
                                                               *mut ZSTD_CCtx,
                                                           mut dict:
                                                               *const libc::c_void,
                                                           mut dictSize:
                                                               size_t,
                                                           mut dictLoadMethod:
                                                               ZSTD_dictLoadMethod_e,
                                                           mut dictContentType:
                                                               ZSTD_dictContentType_e)
 -> size_t {
    if (*cctx).streamStage as libc::c_uint !=
           zcss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    }
    if 0 != (*cctx).staticSize {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    }
    ZSTD_freeCDict((*cctx).cdictLocal);
    if dict == 0 as *mut libc::c_void || dictSize == 0i32 as libc::c_ulong {
        (*cctx).cdictLocal = 0 as *mut ZSTD_CDict;
        (*cctx).cdict = 0 as *const ZSTD_CDict
    } else {
        let cParams: ZSTD_compressionParameters =
            ZSTD_getCParamsFromCCtxParams(&mut (*cctx).requestedParams,
                                          (*cctx).pledgedSrcSizePlusOne.wrapping_sub(1i32
                                                                                         as
                                                                                         libc::c_ulonglong)
                                              as U64, dictSize);
        (*cctx).cdictLocal =
            ZSTD_createCDict_advanced(dict, dictSize, dictLoadMethod,
                                      dictContentType, cParams,
                                      (*cctx).customMem);
        (*cctx).cdict = (*cctx).cdictLocal;
        if (*cctx).cdictLocal.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        }
    }
    return 0i32 as size_t;
}
/* ! ZSTD_CCtx_refCDict() :
 *  Reference a prepared dictionary, to be used for all next compressed frames.
 *  Note that compression parameters are enforced from within CDict,
 *  and supercede any compression parameter previously set within CCtx.
 *  The dictionary will remain valid for future compressed frames using same CCtx.
 * @result : 0, or an error code (which can be tested with ZSTD_isError()).
 *  Special : Referencing a NULL CDict means "return to no-dictionary mode".
 *  Note 1 : Currently, only one dictionary can be managed.
 *           Referencing a new dictionary effectively "discards" any previous one.
 *  Note 2 : CDict is just referenced, its lifetime must outlive its usage within CCtx. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_refCDict(mut cctx: *mut ZSTD_CCtx,
                                            mut cdict: *const ZSTD_CDict)
 -> size_t {
    if (*cctx).streamStage as libc::c_uint !=
           zcss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    }
    (*cctx).cdict = cdict;
    memset(&mut (*cctx).prefixDict as *mut ZSTD_prefixDict as
               *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZSTD_prefixDict>() as libc::c_ulong);
    return 0i32 as size_t;
}
/* ! ZSTD_CCtx_refPrefix() :
 *  Reference a prefix (single-usage dictionary) for next compressed frame.
 *  A prefix is **only used once**. Tables are discarded at end of frame (ZSTD_e_end).
 *  Decompression will need same prefix to properly regenerate data.
 *  Compressing with a prefix is similar in outcome as performing a diff and compressing it,
 *  but performs much faster, especially during decompression (compression speed is tunable with compression level).
 * @result : 0, or an error code (which can be tested with ZSTD_isError()).
 *  Special: Adding any prefix (including NULL) invalidates any previous prefix or dictionary
 *  Note 1 : Prefix buffer is referenced. It **must** outlive compression.
 *           Its content must remain unmodified during compression.
 *  Note 2 : If the intention is to diff some large src data blob with some prior version of itself,
 *           ensure that the window size is large enough to contain the entire source.
 *           See ZSTD_c_windowLog.
 *  Note 3 : Referencing a prefix involves building tables, which are dependent on compression parameters.
 *           It's a CPU consuming operation, with non-negligible impact on latency.
 *           If there is a need to use the same prefix multiple times, consider loadDictionary instead.
 *  Note 4 : By default, the prefix is interpreted as raw content (ZSTD_dm_rawContent).
 *           Use experimental ZSTD_CCtx_refPrefix_advanced() to alter dictionary interpretation. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_refPrefix(mut cctx: *mut ZSTD_CCtx,
                                             mut prefix: *const libc::c_void,
                                             mut prefixSize: size_t)
 -> size_t {
    return ZSTD_CCtx_refPrefix_advanced(cctx, prefix, prefixSize,
                                        ZSTD_dct_rawContent);
}
/* ! ZSTD_CCtx_refPrefix_advanced() :
 *  Same as ZSTD_CCtx_refPrefix(), but gives finer control over
 *  how to interpret prefix content (automatic ? force raw mode (default) ? full mode only ?) */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_refPrefix_advanced(mut cctx:
                                                          *mut ZSTD_CCtx,
                                                      mut prefix:
                                                          *const libc::c_void,
                                                      mut prefixSize: size_t,
                                                      mut dictContentType:
                                                          ZSTD_dictContentType_e)
 -> size_t {
    if (*cctx).streamStage as libc::c_uint !=
           zcss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    }
    (*cctx).cdict = 0 as *const ZSTD_CDict;
    (*cctx).prefixDict.dict = prefix;
    (*cctx).prefixDict.dictSize = prefixSize;
    (*cctx).prefixDict.dictContentType = dictContentType;
    return 0i32 as size_t;
}
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
pub unsafe extern "C" fn ZSTD_compress2(mut cctx: *mut ZSTD_CCtx,
                                        mut dst: *mut libc::c_void,
                                        mut dstCapacity: size_t,
                                        mut src: *const libc::c_void,
                                        mut srcSize: size_t) -> size_t {
    ZSTD_CCtx_reset(cctx, ZSTD_reset_session_only);
    let mut oPos: size_t = 0i32 as size_t;
    let mut iPos: size_t = 0i32 as size_t;
    let result: size_t =
        ZSTD_compressStream2_simpleArgs(cctx, dst, dstCapacity, &mut oPos,
                                        src, srcSize, &mut iPos, ZSTD_e_end);
    if 0 != ERR_isError(result) { return result }
    if result != 0i32 as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    return oPos;
}
/* ! ZSTD_compressStream2_simpleArgs() :
 *  Same as ZSTD_compressStream2(),
 *  but using only integral types as arguments.
 *  This variant might be helpful for binders from dynamic languages
 *  which have troubles handling structures containing memory pointers.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressStream2_simpleArgs(mut cctx:
                                                             *mut ZSTD_CCtx,
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
                                                             *mut size_t,
                                                         mut endOp:
                                                             ZSTD_EndDirective)
 -> size_t {
    let mut output: ZSTD_outBuffer =
        ZSTD_outBuffer_s{dst: dst, size: dstCapacity, pos: *dstPos,};
    let mut input: ZSTD_inBuffer =
        ZSTD_inBuffer_s{src: src, size: srcSize, pos: *srcPos,};
    /* ZSTD_compressStream2() will check validity of dstPos and srcPos */
    let cErr: size_t =
        ZSTD_compressStream2(cctx, &mut output, &mut input, endOp);
    *dstPos = output.pos;
    *srcPos = input.pos;
    return cErr;
}
/* **************************************
*  Memory management
***************************************/
/* ! ZSTD_estimate*() :
 *  These functions make it possible to estimate memory usage
 *  of a future {D,C}Ctx, before its creation.
 *  ZSTD_estimateCCtxSize() will provide a budget large enough for any compression level up to selected one.
 *  It will also consider src size to be arbitrarily "large", which is worst case.
 *  If srcSize is known to always be small, ZSTD_estimateCCtxSize_usingCParams() can provide a tighter estimation.
 *  ZSTD_estimateCCtxSize_usingCParams() can be used in tandem with ZSTD_getCParams() to create cParams from compressionLevel.
 *  ZSTD_estimateCCtxSize_usingCCtxParams() can be used in tandem with ZSTD_CCtxParam_setParameter(). Only single-threaded compression is supported. This function will return an error code if ZSTD_c_nbWorkers is >= 1.
 *  Note : CCtx size estimation is only correct for single-threaded compression. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCCtxSize(mut compressionLevel:
                                                   libc::c_int) -> size_t {
    let mut level: libc::c_int = 0;
    let mut memBudget: size_t = 0i32 as size_t;
    level = if compressionLevel < 1i32 { compressionLevel } else { 1i32 };
    while level <= compressionLevel {
        let newMB: size_t = ZSTD_estimateCCtxSize_internal(level);
        if newMB > memBudget { memBudget = newMB }
        level += 1
    }
    return memBudget;
}
unsafe extern "C" fn ZSTD_estimateCCtxSize_internal(mut compressionLevel:
                                                        libc::c_int)
 -> size_t {
    let cParams: ZSTD_compressionParameters =
        ZSTD_getCParams(compressionLevel, 0i32 as libc::c_ulonglong,
                        0i32 as size_t);
    return ZSTD_estimateCCtxSize_usingCParams(cParams);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCCtxSize_usingCParams(mut cParams:
                                                                ZSTD_compressionParameters)
 -> size_t {
    let params: ZSTD_CCtx_params = ZSTD_makeCCtxParamsFromCParams(cParams);
    return ZSTD_estimateCCtxSize_usingCCtxParams(&params);
}
unsafe extern "C" fn ZSTD_makeCCtxParamsFromCParams(mut cParams:
                                                        ZSTD_compressionParameters)
 -> ZSTD_CCtx_params {
    let mut cctxParams: ZSTD_CCtx_params =
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
    memset(&mut cctxParams as *mut ZSTD_CCtx_params as *mut libc::c_void,
           0i32, ::std::mem::size_of::<ZSTD_CCtx_params>() as libc::c_ulong);
    cctxParams.cParams = cParams;
    cctxParams.compressionLevel = 3i32;
    cctxParams.fParams.contentSizeFlag = 1i32;
    return cctxParams;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCCtxSize_usingCCtxParams(mut params:
                                                                   *const ZSTD_CCtx_params)
 -> size_t {
    if (*params).nbWorkers > 0i32 {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    let cParams: ZSTD_compressionParameters =
        ZSTD_getCParamsFromCCtxParams(params, 0i32 as U64, 0i32 as size_t);
    let blockSize: size_t =
        if ((1i32 << 17i32) as libc::c_ulong) <
               (1i32 as size_t) << cParams.windowLog {
            (1i32 << 17i32) as libc::c_ulong
        } else { (1i32 as size_t) << cParams.windowLog };
    let divider: U32 =
        (if cParams.minMatch == 3i32 as libc::c_uint { 3i32 } else { 4i32 })
            as U32;
    let maxNbSeq: size_t = blockSize.wrapping_div(divider as libc::c_ulong);
    let tokenSpace: size_t =
        (8i32 as
             libc::c_ulong).wrapping_add(blockSize).wrapping_add((11i32 as
                                                                      libc::c_ulong).wrapping_mul(maxNbSeq));
    let entropySpace: size_t = (6i32 << 10i32) as size_t;
    let blockStateSpace: size_t =
        (2i32 as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<ZSTD_compressedBlockState_t>()
                                             as libc::c_ulong);
    let matchStateSize: size_t =
        ZSTD_sizeof_matchState(&cParams, 1i32 as U32);
    let ldmSpace: size_t = ZSTD_ldm_getTableSize((*params).ldmParams);
    let ldmSeqSpace: size_t =
        ZSTD_ldm_getMaxNbSeq((*params).ldmParams,
                             blockSize).wrapping_mul(::std::mem::size_of::<rawSeq>()
                                                         as libc::c_ulong);
    let neededSpace: size_t =
        entropySpace.wrapping_add(blockStateSpace).wrapping_add(tokenSpace).wrapping_add(matchStateSize).wrapping_add(ldmSpace).wrapping_add(ldmSeqSpace);
    return (::std::mem::size_of::<ZSTD_CCtx>() as
                libc::c_ulong).wrapping_add(neededSpace);
}
/* ! ZSTD_estimateCStreamSize() :
 *  ZSTD_estimateCStreamSize() will provide a budget large enough for any compression level up to selected one.
 *  It will also consider src size to be arbitrarily "large", which is worst case.
 *  If srcSize is known to always be small, ZSTD_estimateCStreamSize_usingCParams() can provide a tighter estimation.
 *  ZSTD_estimateCStreamSize_usingCParams() can be used in tandem with ZSTD_getCParams() to create cParams from compressionLevel.
 *  ZSTD_estimateCStreamSize_usingCCtxParams() can be used in tandem with ZSTD_CCtxParam_setParameter(). Only single-threaded compression is supported. This function will return an error code if ZSTD_c_nbWorkers is >= 1.
 *  Note : CStream size estimation is only correct for single-threaded compression.
 *  ZSTD_DStream memory budget depends on window Size.
 *  This information can be passed manually, using ZSTD_estimateDStreamSize,
 *  or deducted from a valid frame Header, using ZSTD_estimateDStreamSize_fromFrame();
 *  Note : if streaming is init with function ZSTD_init?Stream_usingDict(),
 *         an internal ?Dict will be created, which additional size is not estimated here.
 *         In this case, get total size by adding ZSTD_estimate?DictSize */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCStreamSize(mut compressionLevel:
                                                      libc::c_int) -> size_t {
    let mut level: libc::c_int = 0;
    let mut memBudget: size_t = 0i32 as size_t;
    level = if compressionLevel < 1i32 { compressionLevel } else { 1i32 };
    while level <= compressionLevel {
        let newMB: size_t = ZSTD_estimateCStreamSize_internal(level);
        if newMB > memBudget { memBudget = newMB }
        level += 1
    }
    return memBudget;
}
unsafe extern "C" fn ZSTD_estimateCStreamSize_internal(mut compressionLevel:
                                                           libc::c_int)
 -> size_t {
    let cParams: ZSTD_compressionParameters =
        ZSTD_getCParams(compressionLevel, 0i32 as libc::c_ulonglong,
                        0i32 as size_t);
    return ZSTD_estimateCStreamSize_usingCParams(cParams);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCStreamSize_usingCParams(mut cParams:
                                                                   ZSTD_compressionParameters)
 -> size_t {
    let params: ZSTD_CCtx_params = ZSTD_makeCCtxParamsFromCParams(cParams);
    return ZSTD_estimateCStreamSize_usingCCtxParams(&params);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCStreamSize_usingCCtxParams(mut params:
                                                                      *const ZSTD_CCtx_params)
 -> size_t {
    if (*params).nbWorkers > 0i32 {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    let CCtxSize: size_t = ZSTD_estimateCCtxSize_usingCCtxParams(params);
    let blockSize: size_t =
        if ((1i32 << 17i32) as libc::c_ulong) <
               (1i32 as size_t) << (*params).cParams.windowLog {
            (1i32 << 17i32) as libc::c_ulong
        } else { (1i32 as size_t) << (*params).cParams.windowLog };
    let inBuffSize: size_t =
        ((1i32 as size_t) <<
             (*params).cParams.windowLog).wrapping_add(blockSize);
    let outBuffSize: size_t =
        ZSTD_compressBound(blockSize).wrapping_add(1i32 as libc::c_ulong);
    let streamingSize: size_t = inBuffSize.wrapping_add(outBuffSize);
    return CCtxSize.wrapping_add(streamingSize);
}
/* ! ZSTD_estimate?DictSize() :
 *  ZSTD_estimateCDictSize() will bet that src size is relatively "small", and content is copied, like ZSTD_createCDict().
 *  ZSTD_estimateCDictSize_advanced() makes it possible to control compression parameters precisely, like ZSTD_createCDict_advanced().
 *  Note : dictionaries created by reference (`ZSTD_dlm_byRef`) are logically smaller.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCDictSize(mut dictSize: size_t,
                                                mut compressionLevel:
                                                    libc::c_int) -> size_t {
    let cParams: ZSTD_compressionParameters =
        ZSTD_getCParams(compressionLevel, 0i32 as libc::c_ulonglong,
                        dictSize);
    return ZSTD_estimateCDictSize_advanced(dictSize, cParams,
                                           ZSTD_dlm_byCopy);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCDictSize_advanced(mut dictSize: size_t,
                                                         mut cParams:
                                                             ZSTD_compressionParameters,
                                                         mut dictLoadMethod:
                                                             ZSTD_dictLoadMethod_e)
 -> size_t {
    return (::std::mem::size_of::<ZSTD_CDict>() as
                libc::c_ulong).wrapping_add((6i32 << 10i32) as
                                                libc::c_ulong).wrapping_add(ZSTD_sizeof_matchState(&mut cParams,
                                                                                                   0i32
                                                                                                       as
                                                                                                       U32)).wrapping_add(if dictLoadMethod
                                                                                                                                 as
                                                                                                                                 libc::c_uint
                                                                                                                                 ==
                                                                                                                                 ZSTD_dlm_byRef
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     libc::c_uint
                                                                                                                             {
                                                                                                                              0i32
                                                                                                                                  as
                                                                                                                                  libc::c_ulong
                                                                                                                          } else {
                                                                                                                              dictSize
                                                                                                                          });
}
/* ! ZSTD_initStatic*() :
 *  Initialize an object using a pre-allocated fixed-size buffer.
 *  workspace: The memory area to emplace the object into.
 *             Provided pointer *must be 8-bytes aligned*.
 *             Buffer must outlive object.
 *  workspaceSize: Use ZSTD_estimate*Size() to determine
 *                 how large workspace must be to support target scenario.
 * @return : pointer to object (same address as workspace, just different type),
 *           or NULL if error (size too small, incorrect alignment, etc.)
 *  Note : zstd will never resize nor malloc() when using a static buffer.
 *         If the object requires more memory than available,
 *         zstd will just error out (typically ZSTD_error_memory_allocation).
 *  Note 2 : there is no corresponding "free" function.
 *           Since workspace is allocated externally, it must be freed externally too.
 *  Note 3 : cParams : use ZSTD_getCParams() to convert a compression level
 *           into its associated cParams.
 *  Limitation 1 : currently not compatible with internal dictionary creation, triggered by
 *                 ZSTD_CCtx_loadDictionary(), ZSTD_initCStream_usingDict() or ZSTD_initDStream_usingDict().
 *  Limitation 2 : static cctx currently not compatible with multi-threading.
 *  Limitation 3 : static dctx is incompatible with legacy support.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticCCtx(mut workspace: *mut libc::c_void,
                                             mut workspaceSize: size_t)
 -> *mut ZSTD_CCtx {
    let cctx: *mut ZSTD_CCtx = workspace as *mut ZSTD_CCtx;
    if workspaceSize <= ::std::mem::size_of::<ZSTD_CCtx>() as libc::c_ulong {
        return 0 as *mut ZSTD_CCtx
    }
    if 0 != workspace as size_t & 7i32 as libc::c_ulong {
        return 0 as *mut ZSTD_CCtx
    }
    memset(workspace, 0i32, workspaceSize);
    (*cctx).staticSize = workspaceSize;
    (*cctx).workSpace = cctx.offset(1isize) as *mut libc::c_void;
    (*cctx).workSpaceSize =
        workspaceSize.wrapping_sub(::std::mem::size_of::<ZSTD_CCtx>() as
                                       libc::c_ulong);
    if (*cctx).workSpaceSize <
           ((6i32 << 10i32) as
                libc::c_ulong).wrapping_add((2i32 as
                                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<ZSTD_compressedBlockState_t>()
                                                                                 as
                                                                                 libc::c_ulong))
       {
        return 0 as *mut ZSTD_CCtx
    }
    (*cctx).blockState.prevCBlock =
        (*cctx).workSpace as *mut ZSTD_compressedBlockState_t;
    (*cctx).blockState.nextCBlock =
        (*cctx).blockState.prevCBlock.offset(1isize);
    let ptr: *mut libc::c_void =
        (*cctx).blockState.nextCBlock.offset(1isize) as *mut libc::c_void;
    (*cctx).entropyWorkspace = ptr as *mut U32;
    (*cctx).bmi2 = ZSTD_cpuid_bmi2(ZSTD_cpuid());
    return cctx;
}
/* *< same as ZSTD_initStaticCCtx() */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticCStream(mut workspace:
                                                    *mut libc::c_void,
                                                mut workspaceSize: size_t)
 -> *mut ZSTD_CStream {
    return ZSTD_initStaticCCtx(workspace, workspaceSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticCDict(mut workspace:
                                                  *mut libc::c_void,
                                              mut workspaceSize: size_t,
                                              mut dict: *const libc::c_void,
                                              mut dictSize: size_t,
                                              mut dictLoadMethod:
                                                  ZSTD_dictLoadMethod_e,
                                              mut dictContentType:
                                                  ZSTD_dictContentType_e,
                                              mut cParams:
                                                  ZSTD_compressionParameters)
 -> *const ZSTD_CDict {
    /* forCCtx */
    let matchStateSize: size_t =
        ZSTD_sizeof_matchState(&mut cParams, 0i32 as U32);
    let neededSize: size_t =
        (::std::mem::size_of::<ZSTD_CDict>() as
             libc::c_ulong).wrapping_add(if dictLoadMethod as libc::c_uint ==
                                                ZSTD_dlm_byRef as libc::c_int
                                                    as libc::c_uint {
                                             0i32 as libc::c_ulong
                                         } else {
                                             dictSize
                                         }).wrapping_add((6i32 << 10i32) as
                                                             libc::c_ulong).wrapping_add(matchStateSize);
    let cdict: *mut ZSTD_CDict = workspace as *mut ZSTD_CDict;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if 0 != workspace as size_t & 7i32 as libc::c_ulong {
        return 0 as *const ZSTD_CDict
    }
    if workspaceSize < neededSize { return 0 as *const ZSTD_CDict }
    if dictLoadMethod as libc::c_uint ==
           ZSTD_dlm_byCopy as libc::c_int as libc::c_uint {
        memcpy(cdict.offset(1isize) as *mut libc::c_void, dict, dictSize);
        dict = cdict.offset(1isize) as *const libc::c_void;
        ptr =
            (workspace as
                 *mut libc::c_char).offset(::std::mem::size_of::<ZSTD_CDict>()
                                               as libc::c_ulong as
                                               isize).offset(dictSize as
                                                                 isize) as
                *mut libc::c_void
    } else { ptr = cdict.offset(1isize) as *mut libc::c_void }
    (*cdict).workspace = ptr;
    (*cdict).workspaceSize =
        ((6i32 << 10i32) as libc::c_ulong).wrapping_add(matchStateSize);
    if 0 !=
           ERR_isError(ZSTD_initCDict_internal(cdict, dict, dictSize,
                                               ZSTD_dlm_byRef,
                                               dictContentType, cParams)) {
        return 0 as *const ZSTD_CDict
    }
    return cdict;
}
/* **************************************
*  Advanced compression functions
***************************************/
/* ! ZSTD_createCDict_byReference() :
 *  Create a digested dictionary for compression
 *  Dictionary content is just referenced, not duplicated.
 *  As a consequence, `dictBuffer` **must** outlive CDict,
 *  and its content must remain unmodified throughout the lifetime of CDict. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCDict_byReference(mut dict:
                                                          *const libc::c_void,
                                                      mut dictSize: size_t,
                                                      mut compressionLevel:
                                                          libc::c_int)
 -> *mut ZSTD_CDict {
    let mut cParams: ZSTD_compressionParameters =
        ZSTD_getCParams(compressionLevel, 0i32 as libc::c_ulonglong,
                        dictSize);
    return ZSTD_createCDict_advanced(dict, dictSize, ZSTD_dlm_byRef,
                                     ZSTD_dct_auto, cParams,
                                     ZSTD_defaultCMem);
}
/* ! ZSTD_checkCParams() :
 *  Ensure param values remain within authorized range.
 * @return 0 on success, or an error code (can be checked with ZSTD_isError()) */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_checkCParams(mut cParams:
                                               ZSTD_compressionParameters)
 -> size_t {
    if 0 ==
           ZSTD_cParam_withinBounds(ZSTD_c_windowLog,
                                    cParams.windowLog as libc::c_int) {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t
    }
    if 0 ==
           ZSTD_cParam_withinBounds(ZSTD_c_chainLog,
                                    cParams.chainLog as libc::c_int) {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t
    }
    if 0 ==
           ZSTD_cParam_withinBounds(ZSTD_c_hashLog,
                                    cParams.hashLog as libc::c_int) {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t
    }
    if 0 ==
           ZSTD_cParam_withinBounds(ZSTD_c_searchLog,
                                    cParams.searchLog as libc::c_int) {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t
    }
    if 0 ==
           ZSTD_cParam_withinBounds(ZSTD_c_minMatch,
                                    cParams.minMatch as libc::c_int) {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t
    }
    if 0 ==
           ZSTD_cParam_withinBounds(ZSTD_c_targetLength,
                                    cParams.targetLength as libc::c_int) {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t
    }
    if 0 ==
           ZSTD_cParam_withinBounds(ZSTD_c_strategy,
                                    cParams.strategy as libc::c_int) {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t
    }
    return 0i32 as size_t;
}
/* ! ZSTD_adjustCParams() :
 *  optimize params for a given `srcSize` and `dictSize`.
 * `srcSize` can be unknown, in which case use ZSTD_CONTENTSIZE_UNKNOWN.
 * `dictSize` must be `0` when there is no dictionary.
 *  cPar can be invalid : all parameters will be clamped within valid range in the @return struct.
 *  This function never fails (wide contract) */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_adjustCParams(mut cPar:
                                                ZSTD_compressionParameters,
                                            mut srcSize: libc::c_ulonglong,
                                            mut dictSize: size_t)
 -> ZSTD_compressionParameters {
    cPar = ZSTD_clampCParams(cPar);
    return ZSTD_adjustCParams_internal(cPar, srcSize, dictSize);
}
/* * ZSTD_clampCParams() :
 *  make CParam values within valid range.
 *  @return : valid CParams */
unsafe extern "C" fn ZSTD_clampCParams(mut cParams:
                                           ZSTD_compressionParameters)
 -> ZSTD_compressionParameters {
    let bounds: ZSTD_bounds = ZSTD_cParam_getBounds(ZSTD_c_windowLog);
    if (cParams.windowLog as libc::c_int) < bounds.lowerBound {
        cParams.windowLog = bounds.lowerBound as libc::c_uint
    } else if cParams.windowLog as libc::c_int > bounds.upperBound {
        cParams.windowLog = bounds.upperBound as libc::c_uint
    }
    let bounds_0: ZSTD_bounds = ZSTD_cParam_getBounds(ZSTD_c_chainLog);
    if (cParams.chainLog as libc::c_int) < bounds_0.lowerBound {
        cParams.chainLog = bounds_0.lowerBound as libc::c_uint
    } else if cParams.chainLog as libc::c_int > bounds_0.upperBound {
        cParams.chainLog = bounds_0.upperBound as libc::c_uint
    }
    let bounds_1: ZSTD_bounds = ZSTD_cParam_getBounds(ZSTD_c_hashLog);
    if (cParams.hashLog as libc::c_int) < bounds_1.lowerBound {
        cParams.hashLog = bounds_1.lowerBound as libc::c_uint
    } else if cParams.hashLog as libc::c_int > bounds_1.upperBound {
        cParams.hashLog = bounds_1.upperBound as libc::c_uint
    }
    let bounds_2: ZSTD_bounds = ZSTD_cParam_getBounds(ZSTD_c_searchLog);
    if (cParams.searchLog as libc::c_int) < bounds_2.lowerBound {
        cParams.searchLog = bounds_2.lowerBound as libc::c_uint
    } else if cParams.searchLog as libc::c_int > bounds_2.upperBound {
        cParams.searchLog = bounds_2.upperBound as libc::c_uint
    }
    let bounds_3: ZSTD_bounds = ZSTD_cParam_getBounds(ZSTD_c_minMatch);
    if (cParams.minMatch as libc::c_int) < bounds_3.lowerBound {
        cParams.minMatch = bounds_3.lowerBound as libc::c_uint
    } else if cParams.minMatch as libc::c_int > bounds_3.upperBound {
        cParams.minMatch = bounds_3.upperBound as libc::c_uint
    }
    let bounds_4: ZSTD_bounds = ZSTD_cParam_getBounds(ZSTD_c_targetLength);
    if (cParams.targetLength as libc::c_int) < bounds_4.lowerBound {
        cParams.targetLength = bounds_4.lowerBound as libc::c_uint
    } else if cParams.targetLength as libc::c_int > bounds_4.upperBound {
        cParams.targetLength = bounds_4.upperBound as libc::c_uint
    }
    let bounds_5: ZSTD_bounds = ZSTD_cParam_getBounds(ZSTD_c_strategy);
    if (cParams.strategy as libc::c_int) < bounds_5.lowerBound {
        cParams.strategy = bounds_5.lowerBound as ZSTD_strategy
    } else if cParams.strategy as libc::c_int > bounds_5.upperBound {
        cParams.strategy = bounds_5.upperBound as ZSTD_strategy
    }
    return cParams;
}
/* ! ZSTD_compress_advanced() :
 *  Same as ZSTD_compress_usingDict(), with fine-tune control over compression parameters (by structure) */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compress_advanced(mut cctx: *mut ZSTD_CCtx,
                                                mut dst: *mut libc::c_void,
                                                mut dstCapacity: size_t,
                                                mut src: *const libc::c_void,
                                                mut srcSize: size_t,
                                                mut dict: *const libc::c_void,
                                                mut dictSize: size_t,
                                                mut params: ZSTD_parameters)
 -> size_t {
    let errcod: size_t = ZSTD_checkCParams(params.cParams);
    if 0 != ERR_isError(errcod) { return errcod }
    return ZSTD_compress_internal(cctx, dst, dstCapacity, src, srcSize, dict,
                                  dictSize, params);
}
unsafe extern "C" fn ZSTD_compress_internal(mut cctx: *mut ZSTD_CCtx,
                                            mut dst: *mut libc::c_void,
                                            mut dstCapacity: size_t,
                                            mut src: *const libc::c_void,
                                            mut srcSize: size_t,
                                            mut dict: *const libc::c_void,
                                            mut dictSize: size_t,
                                            mut params: ZSTD_parameters)
 -> size_t {
    let cctxParams: ZSTD_CCtx_params =
        ZSTD_assignParamsToCCtxParams((*cctx).requestedParams, params);
    return ZSTD_compress_advanced_internal(cctx, dst, dstCapacity, src,
                                           srcSize, dict, dictSize,
                                           cctxParams);
}
/* ! ZSTD_CCtx_loadDictionary_byReference() :
 *  Same as ZSTD_CCtx_loadDictionary(), but dictionary content is referenced, instead of being copied into CCtx.
 *  It saves some memory, but also requires that `dict` outlives its usage within `cctx` */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_loadDictionary_byReference(mut cctx:
                                                                  *mut ZSTD_CCtx,
                                                              mut dict:
                                                                  *const libc::c_void,
                                                              mut dictSize:
                                                                  size_t)
 -> size_t {
    return ZSTD_CCtx_loadDictionary_advanced(cctx, dict, dictSize,
                                             ZSTD_dlm_byRef, ZSTD_dct_auto);
}
/* ===   experimental parameters   === */
/* these parameters can be used with ZSTD_setParameter()
 * they are not guaranteed to remain supported in the future */
/* Enables rsyncable mode,
  * which makes compressed files more rsync friendly
  * by adding periodic synchronization points to the compressed data.
  * The target average block size is ZSTD_c_jobSize / 2.
  * It's possible to modify the job size to increase or decrease
  * the granularity of the synchronization point.
  * Once the jobSize is smaller than the window size,
  * it will result in compression ratio degradation.
  * NOTE 1: rsyncable mode only works when multithreading is enabled.
  * NOTE 2: rsyncable performs poorly in combination with long range mode,
  * since it will decrease the effectiveness of synchronization points,
  * though mileage may vary.
  * NOTE 3: Rsyncable mode limits maximum compression speed to ~400 MB/s.
  * If the selected compression level is already running significantly slower,
  * the overall speed won't be significantly impacted.
  */
/* Select a compression format.
 * The value must be of type ZSTD_format_e.
 * See ZSTD_format_e enum definition for details */
/* Force back-reference distances to remain < windowSize,
 * even when referencing into Dictionary content (default:0) */
/* Controls whether the contents of a CDict
 * are used in place, or copied into the working context.
 * Accepts values from the ZSTD_dictAttachPref_e enum.
 * See the comments on that enum for an explanation of the feature. */
/* ! ZSTD_CCtx_getParameter() :
 *  Get the requested compression parameter value, selected by enum ZSTD_cParameter,
 *  and store it into int* value.
 * @return : 0, or an error code (which can be tested with ZSTD_isError()).
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_getParameter(mut cctx: *mut ZSTD_CCtx,
                                                mut param: ZSTD_cParameter,
                                                mut value: *mut libc::c_int)
 -> size_t {
    return ZSTD_CCtxParam_getParameter(&mut (*cctx).requestedParams, param,
                                       value);
}
/* ! ZSTD_CCtxParam_getParameter() :
 * Similar to ZSTD_CCtx_getParameter.
 * Get the requested value of one compression parameter, selected by enum ZSTD_cParameter.
 * @result : 0, or an error code (which can be tested with ZSTD_isError()).
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtxParam_getParameter(mut CCtxParams:
                                                         *mut ZSTD_CCtx_params,
                                                     mut param:
                                                         ZSTD_cParameter,
                                                     mut value:
                                                         *mut libc::c_int)
 -> size_t {
    match param as libc::c_uint {
        10 => { *value = (*CCtxParams).format as libc::c_int }
        100 => { *value = (*CCtxParams).compressionLevel }
        101 => { *value = (*CCtxParams).cParams.windowLog as libc::c_int }
        102 => { *value = (*CCtxParams).cParams.hashLog as libc::c_int }
        103 => { *value = (*CCtxParams).cParams.chainLog as libc::c_int }
        104 => { *value = (*CCtxParams).cParams.searchLog as libc::c_int }
        105 => { *value = (*CCtxParams).cParams.minMatch as libc::c_int }
        106 => { *value = (*CCtxParams).cParams.targetLength as libc::c_int }
        107 => {
            *value =
                (*CCtxParams).cParams.strategy as libc::c_uint as libc::c_int
        }
        200 => { *value = (*CCtxParams).fParams.contentSizeFlag }
        201 => { *value = (*CCtxParams).fParams.checksumFlag }
        202 => {
            *value = (0 == (*CCtxParams).fParams.noDictIDFlag) as libc::c_int
        }
        1000 => { *value = (*CCtxParams).forceWindow }
        1001 => { *value = (*CCtxParams).attachDictPref as libc::c_int }
        400 => { *value = (*CCtxParams).nbWorkers }
        401 => { *value = (*CCtxParams).jobSize as libc::c_int }
        402 => { *value = (*CCtxParams).overlapLog }
        500 => { *value = (*CCtxParams).rsyncable }
        160 => { *value = (*CCtxParams).ldmParams.enableLdm as libc::c_int }
        161 => { *value = (*CCtxParams).ldmParams.hashLog as libc::c_int }
        162 => {
            *value = (*CCtxParams).ldmParams.minMatchLength as libc::c_int
        }
        163 => {
            *value = (*CCtxParams).ldmParams.bucketSizeLog as libc::c_int
        }
        164 => { *value = (*CCtxParams).ldmParams.hashRateLog as libc::c_int }
        _ => {
            return -(ZSTD_error_parameter_unsupported as libc::c_int) as
                       size_t
        }
    }
    return 0i32 as size_t;
}
/* ! ZSTD_CCtx_params :
 *  Quick howto :
 *  - ZSTD_createCCtxParams() : Create a ZSTD_CCtx_params structure
 *  - ZSTD_CCtxParam_setParameter() : Push parameters one by one into
 *                                    an existing ZSTD_CCtx_params structure.
 *                                    This is similar to
 *                                    ZSTD_CCtx_setParameter().
 *  - ZSTD_CCtx_setParametersUsingCCtxParams() : Apply parameters to
 *                                    an existing CCtx.
 *                                    These parameters will be applied to
 *                                    all subsequent frames.
 *  - ZSTD_compressStream2() : Do compression using the CCtx.
 *  - ZSTD_freeCCtxParams() : Free the memory.
 *
 *  This can be used with ZSTD_estimateCCtxSize_advanced_usingCCtxParams()
 *  for static allocation of CCtx for single-threaded compression.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createCCtxParams() -> *mut ZSTD_CCtx_params {
    return ZSTD_createCCtxParams_advanced(ZSTD_defaultCMem);
}
unsafe extern "C" fn ZSTD_createCCtxParams_advanced(mut customMem:
                                                        ZSTD_customMem)
 -> *mut ZSTD_CCtx_params {
    let mut params: *mut ZSTD_CCtx_params = 0 as *mut ZSTD_CCtx_params;
    if 0 !=
           customMem.customAlloc.is_none() as libc::c_int ^
               customMem.customFree.is_none() as libc::c_int {
        return 0 as *mut ZSTD_CCtx_params
    }
    params =
        ZSTD_calloc(::std::mem::size_of::<ZSTD_CCtx_params>() as
                        libc::c_ulong, customMem) as *mut ZSTD_CCtx_params;
    if params.is_null() { return 0 as *mut ZSTD_CCtx_params }
    (*params).customMem = customMem;
    (*params).compressionLevel = 3i32;
    (*params).fParams.contentSizeFlag = 1i32;
    return params;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeCCtxParams(mut params:
                                                 *mut ZSTD_CCtx_params)
 -> size_t {
    if params.is_null() { return 0i32 as size_t }
    ZSTD_free(params as *mut libc::c_void, (*params).customMem);
    return 0i32 as size_t;
}
/* ! ZSTD_CCtxParams_init_advanced() :
 *  Initializes the compression and frame parameters of cctxParams according to
 *  params. All other parameters are reset to their default values.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtxParams_init_advanced(mut cctxParams:
                                                           *mut ZSTD_CCtx_params,
                                                       mut params:
                                                           ZSTD_parameters)
 -> size_t {
    if cctxParams.is_null() {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    let errcod: size_t = ZSTD_checkCParams(params.cParams);
    if 0 != ERR_isError(errcod) { return errcod }
    memset(cctxParams as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZSTD_CCtx_params>() as libc::c_ulong);
    (*cctxParams).cParams = params.cParams;
    (*cctxParams).fParams = params.fParams;
    (*cctxParams).compressionLevel = 3i32;
    return 0i32 as size_t;
}
/* ! ZSTD_CCtx_setParametersUsingCCtxParams() :
 *  Apply a set of ZSTD_CCtx_params to the compression context.
 *  This can be done even after compression is started,
 *    if nbWorkers==0, this will have no impact until a new compression is started.
 *    if nbWorkers>=1, new parameters will be picked up at next job,
 *       with a few restrictions (windowLog, pledgedSrcSize, nbWorkers, jobSize, and overlapLog are not updated).
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_setParametersUsingCCtxParams(mut cctx:
                                                                    *mut ZSTD_CCtx,
                                                                mut params:
                                                                    *const ZSTD_CCtx_params)
 -> size_t {
    if (*cctx).streamStage as libc::c_uint !=
           zcss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    }
    if !(*cctx).cdict.is_null() {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    }
    (*cctx).requestedParams = *params;
    return 0i32 as size_t;
}
/* *< creates of an internal CDict (incompatible with static CCtx), except if dict == NULL or dictSize < 8, in which case no dict is used. Note: dict is loaded with ZSTD_dm_auto (treated as a full zstd dictionary if it begins with ZSTD_MAGIC_DICTIONARY, else as raw content) and ZSTD_dlm_byCopy.*/
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream_usingDict(mut zcs:
                                                        *mut ZSTD_CStream,
                                                    mut dict:
                                                        *const libc::c_void,
                                                    mut dictSize: size_t,
                                                    mut compressionLevel:
                                                        libc::c_int)
 -> size_t {
    ZSTD_CCtxParams_init(&mut (*zcs).requestedParams, compressionLevel);
    return ZSTD_initCStream_internal(zcs, dict, dictSize,
                                     0 as *const ZSTD_CDict,
                                     (*zcs).requestedParams,
                                     0u64.wrapping_sub(1i32 as
                                                           libc::c_ulonglong));
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream_advanced(mut zcs: *mut ZSTD_CStream,
                                                   mut dict:
                                                       *const libc::c_void,
                                                   mut dictSize: size_t,
                                                   mut params:
                                                       ZSTD_parameters,
                                                   mut pledgedSrcSize:
                                                       libc::c_ulonglong)
 -> size_t {
    let errcod: size_t = ZSTD_checkCParams(params.cParams);
    if 0 != ERR_isError(errcod) { return errcod }
    if pledgedSrcSize == 0i32 as libc::c_ulonglong &&
           params.fParams.contentSizeFlag == 0i32 {
        pledgedSrcSize = 0u64.wrapping_sub(1i32 as libc::c_ulonglong)
    }
    (*zcs).requestedParams =
        ZSTD_assignParamsToCCtxParams((*zcs).requestedParams, params);
    return ZSTD_initCStream_internal(zcs, dict, dictSize,
                                     0 as *const ZSTD_CDict,
                                     (*zcs).requestedParams, pledgedSrcSize);
}
/* *< pledgedSrcSize must be correct. If srcSize is not known at init time, use value ZSTD_CONTENTSIZE_UNKNOWN. dict is loaded with ZSTD_dm_auto and ZSTD_dlm_byCopy. */
/* *< note : cdict will just be referenced, and must outlive compression session */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream_usingCDict(mut zcs:
                                                         *mut ZSTD_CStream,
                                                     mut cdict:
                                                         *const ZSTD_CDict)
 -> size_t {
    /* contentSizeFlag */
    /* checksum */
    /* hideDictID */
    let fParams: ZSTD_frameParameters =
        ZSTD_frameParameters{contentSizeFlag: 0i32,
                             checksumFlag: 0i32,
                             noDictIDFlag: 0i32,};
    return ZSTD_initCStream_usingCDict_advanced(zcs, cdict, fParams,
                                                0u64.wrapping_sub(1i32 as
                                                                      libc::c_ulonglong));
}
/* *< same as ZSTD_initCStream_usingCDict(), with control over frame parameters. pledgedSrcSize must be correct. If srcSize is not known at init time, use value ZSTD_CONTENTSIZE_UNKNOWN. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream_usingCDict_advanced(mut zcs:
                                                                  *mut ZSTD_CStream,
                                                              mut cdict:
                                                                  *const ZSTD_CDict,
                                                              mut fParams:
                                                                  ZSTD_frameParameters,
                                                              mut pledgedSrcSize:
                                                                  libc::c_ulonglong)
 -> size_t {
    if cdict.is_null() {
        return -(ZSTD_error_dictionary_wrong as libc::c_int) as size_t
    }
    let mut params: ZSTD_CCtx_params = (*zcs).requestedParams;
    params.cParams = ZSTD_getCParamsFromCDict(cdict);
    params.fParams = fParams;
    return ZSTD_initCStream_internal(zcs, 0 as *const libc::c_void,
                                     0i32 as size_t, cdict, params,
                                     pledgedSrcSize);
}
/* ! ZSTD_resetCStream() :
 *  start a new frame, using same parameters from previous frame.
 *  This is typically useful to skip dictionary loading stage, since it will re-use it in-place.
 *  Note that zcs must be init at least once before using ZSTD_resetCStream().
 *  If pledgedSrcSize is not known at reset time, use macro ZSTD_CONTENTSIZE_UNKNOWN.
 *  If pledgedSrcSize > 0, its value must be correct, as it will be written in header, and controlled at the end.
 *  For the time being, pledgedSrcSize==0 is interpreted as "srcSize unknown" for compatibility with older programs,
 *  but it will change to mean "empty" in future version, so use macro ZSTD_CONTENTSIZE_UNKNOWN instead.
 * @return : 0, or an error code (which can be tested using ZSTD_isError())
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_resetCStream(mut zcs: *mut ZSTD_CStream,
                                           mut pledgedSrcSize:
                                               libc::c_ulonglong) -> size_t {
    let mut params: ZSTD_CCtx_params = (*zcs).requestedParams;
    if pledgedSrcSize == 0i32 as libc::c_ulonglong {
        pledgedSrcSize = 0u64.wrapping_sub(1i32 as libc::c_ulonglong)
    }
    params.fParams.contentSizeFlag = 1i32;
    return ZSTD_resetCStream_internal(zcs, 0 as *const libc::c_void,
                                      0i32 as size_t, ZSTD_dct_auto,
                                      (*zcs).cdict, params, pledgedSrcSize);
}
/* ZSTD_getFrameProgression() :
 * tells how much data has been ingested (read from input)
 * consumed (input actually compressed) and produced (output) for current frame.
 * Note : (ingested - consumed) is amount of input data buffered internally, not yet compressed.
 * Aggregates progression inside active worker threads.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getFrameProgression(mut cctx: *const ZSTD_CCtx)
 -> ZSTD_frameProgression {
    if (*cctx).appliedParams.nbWorkers > 0i32 {
        return ZSTDMT_getFrameProgression((*cctx).mtctx)
    }
    let mut fp: ZSTD_frameProgression =
        ZSTD_frameProgression{ingested: 0,
                              consumed: 0,
                              produced: 0,
                              flushed: 0,
                              currentJobID: 0,
                              nbActiveWorkers: 0,};
    let buffered: size_t =
        if (*cctx).inBuff.is_null() {
            0i32 as libc::c_ulong
        } else { (*cctx).inBuffPos.wrapping_sub((*cctx).inToCompress) };
    0 != buffered;
    fp.ingested =
        (*cctx).consumedSrcSize.wrapping_add(buffered as libc::c_ulonglong);
    fp.consumed = (*cctx).consumedSrcSize;
    fp.produced = (*cctx).producedCSize;
    fp.flushed = (*cctx).producedCSize;
    fp.currentJobID = 0i32 as libc::c_uint;
    fp.nbActiveWorkers = 0i32 as libc::c_uint;
    return fp;
}
/* ! ZSTD_toFlushNow() :
 *  Tell how many bytes are ready to be flushed immediately.
 *  Useful for multithreading scenarios (nbWorkers >= 1).
 *  Probe the oldest active job, defined as oldest job not yet entirely flushed,
 *  and check its output buffer.
 * @return : amount of data stored in oldest job and ready to be flushed immediately.
 *  if @return == 0, it means either :
 *  + there is no active job (could be checked with ZSTD_frameProgression()), or
 *  + oldest job is still actively compressing data,
 *    but everything it has produced has also been flushed so far,
 *    therefore flush speed is limited by production speed of oldest job
 *    irrespective of the speed of concurrent (and newer) jobs.
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_toFlushNow(mut cctx: *mut ZSTD_CCtx) -> size_t {
    if (*cctx).appliedParams.nbWorkers > 0i32 {
        return ZSTDMT_toFlushNow((*cctx).mtctx)
    }
    return 0i32 as size_t;
}
/* ********************************************************************
*  Buffer-less and synchronous inner streaming functions
*
*  This is an advanced API, giving full control over buffer management, for users which need direct control over memory.
*  But it's also a complex one, with several restrictions, documented below.
*  Prefer normal streaming API for an easier experience.
********************************************************************* */
/* *
  Buffer-less streaming compression (synchronous mode)

  A ZSTD_CCtx object is required to track streaming operations.
  Use ZSTD_createCCtx() / ZSTD_freeCCtx() to manage resource.
  ZSTD_CCtx object can be re-used multiple times within successive compression operations.

  Start by initializing a context.
  Use ZSTD_compressBegin(), or ZSTD_compressBegin_usingDict() for dictionary compression,
  or ZSTD_compressBegin_advanced(), for finer parameter control.
  It's also possible to duplicate a reference context which has already been initialized, using ZSTD_copyCCtx()

  Then, consume your input using ZSTD_compressContinue().
  There are some important considerations to keep in mind when using this advanced function :
  - ZSTD_compressContinue() has no internal buffer. It uses externally provided buffers only.
  - Interface is synchronous : input is consumed entirely and produces 1+ compressed blocks.
  - Caller must ensure there is enough space in `dst` to store compressed data under worst case scenario.
    Worst case evaluation is provided by ZSTD_compressBound().
    ZSTD_compressContinue() doesn't guarantee recover after a failed compression.
  - ZSTD_compressContinue() presumes prior input ***is still accessible and unmodified*** (up to maximum distance size, see WindowLog).
    It remembers all previous contiguous blocks, plus one separated memory segment (which can itself consists of multiple contiguous blocks)
  - ZSTD_compressContinue() detects that prior input has been overwritten when `src` buffer overlaps.
    In which case, it will "discard" the relevant memory section from its history.

  Finish a frame with ZSTD_compressEnd(), which will write the last block(s) and optional checksum.
  It's possible to use srcSize==0, in which case, it will write a final empty block to end the frame.
  Without last block mark, frames are considered unfinished (hence corrupted) by compliant decoders.

  `ZSTD_CCtx` object can be re-used (ZSTD_compressBegin()) to compress again.
*/
/*=====   Buffer-less streaming compression functions  =====*/
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBegin(mut cctx: *mut ZSTD_CCtx,
                                            mut compressionLevel: libc::c_int)
 -> size_t {
    return ZSTD_compressBegin_usingDict(cctx, 0 as *const libc::c_void,
                                        0i32 as size_t, compressionLevel);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBegin_usingDict(mut cctx:
                                                          *mut ZSTD_CCtx,
                                                      mut dict:
                                                          *const libc::c_void,
                                                      mut dictSize: size_t,
                                                      mut compressionLevel:
                                                          libc::c_int)
 -> size_t {
    let params: ZSTD_parameters =
        ZSTD_getParams(compressionLevel,
                       0u64.wrapping_sub(1i32 as libc::c_ulonglong),
                       dictSize);
    let cctxParams: ZSTD_CCtx_params =
        ZSTD_assignParamsToCCtxParams((*cctx).requestedParams, params);
    return ZSTD_compressBegin_internal(cctx, dict, dictSize, ZSTD_dct_auto,
                                       ZSTD_dtlm_fast, 0 as *const ZSTD_CDict,
                                       cctxParams,
                                       0u64.wrapping_sub(1i32 as
                                                             libc::c_ulonglong)
                                           as U64, ZSTDb_not_buffered);
}
/* *< pledgedSrcSize : If srcSize is not known at init time, use ZSTD_CONTENTSIZE_UNKNOWN */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBegin_advanced(mut cctx: *mut ZSTD_CCtx,
                                                     mut dict:
                                                         *const libc::c_void,
                                                     mut dictSize: size_t,
                                                     mut params:
                                                         ZSTD_parameters,
                                                     mut pledgedSrcSize:
                                                         libc::c_ulonglong)
 -> size_t {
    let cctxParams: ZSTD_CCtx_params =
        ZSTD_assignParamsToCCtxParams((*cctx).requestedParams, params);
    return ZSTD_compressBegin_advanced_internal(cctx, dict, dictSize,
                                                ZSTD_dct_auto, ZSTD_dtlm_fast,
                                                0 as *const ZSTD_CDict,
                                                cctxParams, pledgedSrcSize);
}
/* ZSTD_compressBegin_advanced_internal() :
 * Private use only. To be called from zstdmt_compress.c. */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBegin_advanced_internal(mut cctx:
                                                                  *mut ZSTD_CCtx,
                                                              mut dict:
                                                                  *const libc::c_void,
                                                              mut dictSize:
                                                                  size_t,
                                                              mut dictContentType:
                                                                  ZSTD_dictContentType_e,
                                                              mut dtlm:
                                                                  ZSTD_dictTableLoadMethod_e,
                                                              mut cdict:
                                                                  *const ZSTD_CDict,
                                                              mut params:
                                                                  ZSTD_CCtx_params,
                                                              mut pledgedSrcSize:
                                                                  libc::c_ulonglong)
 -> size_t {
    let errcod: size_t = ZSTD_checkCParams(params.cParams);
    if 0 != ERR_isError(errcod) { return errcod }
    return ZSTD_compressBegin_internal(cctx, dict, dictSize, dictContentType,
                                       dtlm, cdict, params,
                                       pledgedSrcSize as U64,
                                       ZSTDb_not_buffered);
}
/* *< note: fails if cdict==NULL */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBegin_usingCDict(mut cctx:
                                                           *mut ZSTD_CCtx,
                                                       mut cdict:
                                                           *const ZSTD_CDict)
 -> size_t {
    /*content*/
    /*checksum*/
    /*noDictID*/
    let fParams: ZSTD_frameParameters =
        ZSTD_frameParameters{contentSizeFlag: 0i32,
                             checksumFlag: 0i32,
                             noDictIDFlag: 0i32,};
    return ZSTD_compressBegin_usingCDict_advanced(cctx, cdict, fParams,
                                                  0u64.wrapping_sub(1i32 as
                                                                        libc::c_ulonglong));
}
/* *<  note: if pledgedSrcSize is not known, use ZSTD_CONTENTSIZE_UNKNOWN */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_copyCCtx(mut dstCCtx: *mut ZSTD_CCtx,
                                       mut srcCCtx: *const ZSTD_CCtx,
                                       mut pledgedSrcSize: libc::c_ulonglong)
 -> size_t {
    /*content*/
    /*checksum*/
    /*noDictID*/
    let mut fParams: ZSTD_frameParameters =
        ZSTD_frameParameters{contentSizeFlag: 1i32,
                             checksumFlag: 0i32,
                             noDictIDFlag: 0i32,};
    let zbuff: ZSTD_buffered_policy_e =
        ((*srcCCtx).inBuffSize > 0i32 as libc::c_ulong) as libc::c_int as
            ZSTD_buffered_policy_e;
    if pledgedSrcSize == 0i32 as libc::c_ulonglong {
        pledgedSrcSize = 0u64.wrapping_sub(1i32 as libc::c_ulonglong)
    }
    fParams.contentSizeFlag =
        (pledgedSrcSize != 0u64.wrapping_sub(1i32 as libc::c_ulonglong)) as
            libc::c_int;
    return ZSTD_copyCCtx_internal(dstCCtx, srcCCtx, fParams,
                                  pledgedSrcSize as U64, zbuff);
}
/* ! ZSTD_copyCCtx_internal() :
 *  Duplicate an existing context `srcCCtx` into another one `dstCCtx`.
 *  Only works during stage ZSTDcs_init (i.e. after creation, but before first call to ZSTD_compressContinue()).
 *  The "context", in this case, refers to the hash and chain tables,
 *  entropy tables, and dictionary references.
 * `windowLog` value is enforced if != 0, otherwise value is copied from srcCCtx.
 * @return : 0, or an error code */
unsafe extern "C" fn ZSTD_copyCCtx_internal(mut dstCCtx: *mut ZSTD_CCtx,
                                            mut srcCCtx: *const ZSTD_CCtx,
                                            mut fParams: ZSTD_frameParameters,
                                            mut pledgedSrcSize: U64,
                                            mut zbuff: ZSTD_buffered_policy_e)
 -> size_t {
    if (*srcCCtx).stage as libc::c_uint !=
           ZSTDcs_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    }
    memcpy(&mut (*dstCCtx).customMem as *mut ZSTD_customMem as
               *mut libc::c_void,
           &(*srcCCtx).customMem as *const ZSTD_customMem as
               *const libc::c_void,
           ::std::mem::size_of::<ZSTD_customMem>() as libc::c_ulong);
    let mut params: ZSTD_CCtx_params = (*dstCCtx).requestedParams;
    params.cParams = (*srcCCtx).appliedParams.cParams;
    params.fParams = fParams;
    ZSTD_resetCCtx_internal(dstCCtx, params, pledgedSrcSize, ZSTDcrp_noMemset,
                            zbuff);
    let chainSize: size_t =
        if (*srcCCtx).appliedParams.cParams.strategy as libc::c_uint ==
               ZSTD_fast as libc::c_int as libc::c_uint {
            0i32 as libc::c_ulong
        } else {
            (1i32 as size_t) << (*srcCCtx).appliedParams.cParams.chainLog
        };
    let hSize: size_t =
        (1i32 as size_t) << (*srcCCtx).appliedParams.cParams.hashLog;
    let h3Size: size_t =
        (1i32 as size_t) << (*srcCCtx).blockState.matchState.hashLog3;
    let tableSpace: size_t =
        chainSize.wrapping_add(hSize).wrapping_add(h3Size).wrapping_mul(::std::mem::size_of::<U32>()
                                                                            as
                                                                            libc::c_ulong);
    memcpy((*dstCCtx).blockState.matchState.hashTable as *mut libc::c_void,
           (*srcCCtx).blockState.matchState.hashTable as *const libc::c_void,
           tableSpace);
    let mut srcMatchState: *const ZSTD_matchState_t =
        &(*srcCCtx).blockState.matchState;
    let mut dstMatchState: *mut ZSTD_matchState_t =
        &mut (*dstCCtx).blockState.matchState;
    (*dstMatchState).window = (*srcMatchState).window;
    (*dstMatchState).nextToUpdate = (*srcMatchState).nextToUpdate;
    (*dstMatchState).nextToUpdate3 = (*srcMatchState).nextToUpdate3;
    (*dstMatchState).loadedDictEnd = (*srcMatchState).loadedDictEnd;
    (*dstCCtx).dictID = (*srcCCtx).dictID;
    memcpy((*dstCCtx).blockState.prevCBlock as *mut libc::c_void,
           (*srcCCtx).blockState.prevCBlock as *const libc::c_void,
           ::std::mem::size_of::<ZSTD_compressedBlockState_t>() as
               libc::c_ulong);
    return 0i32 as size_t;
}
/* ============================ */
/**       Block level API       */
/* ============================ */
/* !
    Block functions produce and decode raw zstd blocks, without frame metadata.
    Frame metadata cost is typically ~18 bytes, which can be non-negligible for very small blocks (< 100 bytes).
    User will have to take in charge required information to regenerate data, such as compressed and content sizes.

    A few rules to respect :
    - Compressing and decompressing require a context structure
      + Use ZSTD_createCCtx() and ZSTD_createDCtx()
    - It is necessary to init context before starting
      + compression : any ZSTD_compressBegin*() variant, including with dictionary
      + decompression : any ZSTD_decompressBegin*() variant, including with dictionary
      + copyCCtx() and copyDCtx() can be used too
    - Block size is limited, it must be <= ZSTD_getBlockSize() <= ZSTD_BLOCKSIZE_MAX == 128 KB
      + If input is larger than a block size, it's necessary to split input data into multiple blocks
      + For inputs larger than a single block, really consider using regular ZSTD_compress() instead.
        Frame metadata is not that costly, and quickly becomes negligible as source size grows larger.
    - When a block is considered not compressible enough, ZSTD_compressBlock() result will be zero.
      In which case, nothing is produced into `dst` !
      + User must test for such outcome and deal directly with uncompressed data
      + ZSTD_decompressBlock() doesn't accept uncompressed data as input !!!
      + In case of multiple successive blocks, should some of them be uncompressed,
        decoder must be informed of their existence in order to follow proper history.
        Use ZSTD_insertBlock() for such a case.
*/
/*=====   Raw zstd block functions  =====*/
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getBlockSize(mut cctx: *const ZSTD_CCtx)
 -> size_t {
    let cParams: ZSTD_compressionParameters = (*cctx).appliedParams.cParams;
    return (if ((1i32 << 17i32) as libc::c_uint) <
                   (1i32 as U32) << cParams.windowLog {
                (1i32 << 17i32) as libc::c_uint
            } else { (1i32 as U32) << cParams.windowLog }) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock(mut cctx: *mut ZSTD_CCtx,
                                            mut dst: *mut libc::c_void,
                                            mut dstCapacity: size_t,
                                            mut src: *const libc::c_void,
                                            mut srcSize: size_t) -> size_t {
    let blockSizeMax: size_t = ZSTD_getBlockSize(cctx);
    if srcSize > blockSizeMax {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    return ZSTD_compressContinue_internal(cctx, dst, dstCapacity, src,
                                          srcSize, 0i32 as U32, 0i32 as U32);
}
/* compress & dictBuilder */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getSeqStore(mut ctx: *const ZSTD_CCtx)
 -> *const seqStore_t {
    return &(*ctx).seqStore;
}
/* Software version */
/* ZSTD_invalidateRepCodes() :
 * ensures next compression will not use repcodes from previous block.
 * Note : only works with regular variant;
 *        do not use with extDict variant ! */
/* zstdmt, adaptive_compression (shouldn't get this definition from here) */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_invalidateRepCodes(mut cctx: *mut ZSTD_CCtx) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 3i32 {
        (*(*cctx).blockState.prevCBlock).rep[i as usize] = 0i32 as U32;
        i += 1
    };
}
/* ZSTD_writeLastEmptyBlock() :
 * output an empty Block with end-of-frame mark to complete a frame
 * @return : size of data written into `dst` (== ZSTD_blockHeaderSize (defined in zstd_internal.h))
 *           or an error code if `dstCapcity` is too small (<ZSTD_blockHeaderSize)
 */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_writeLastEmptyBlock(mut dst: *mut libc::c_void,
                                                  mut dstCapacity: size_t)
 -> size_t {
    if dstCapacity < ZSTD_blockHeaderSize {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    let cBlockHeader24: U32 =
        (1i32 as
             libc::c_uint).wrapping_add((bt_raw as libc::c_int as U32) <<
                                            1i32);
    MEM_writeLE24(dst, cBlockHeader24);
    return ZSTD_blockHeaderSize;
}
unsafe extern "C" fn run_static_initializers() {
    maxWindowResize =
        (1u64 <<
             if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                    4i32 as libc::c_ulong {
                 30i32
             } else { 31i32 } - 1i32) as U64
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];