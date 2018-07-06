#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( extern_types , libc , offset_to , used )]
extern crate libc;
extern "C" {
    pub type ZSTDMT_CCtx_s;
    pub type HUF_CElt_s;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* *< Cannot use the previous table */
    /* ***************************
*  Streaming
****************************/
    /* *< Copy dictionary content internally */
    /* **************************************
*  Explicit context
***************************************/
    /* !< maximum compression level available */
    /* !< provides readable string from an error code */
    /* *< position where writing stopped. Will be updated. Necessarily 0 <= pos <= size */
    /* *< Cannot use the previous table */
    /* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
    /* *< position where reading stopped. Will be updated. Necessarily 0 <= pos <= size */
    /* ! Constructor and Destructor of FSE_DTable.
    Note that its size depends on 'tableLog' */
    /* *********************************
 *  Bulk processing dictionary API
 *********************************/
    /* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
    /* *< Can use the previous table but it must be checked. Note : The previous table must have been constructed by HUF_compress{1, 4}X_repeat */
    /* *< Can use the previous table but it must be checked */
    /* * HIST_countFast_wksp() :
 *  Same as HIST_countFast(), but using an externally provided scratch buffer.
 * `workSpace` must be a table of unsigned of size >= HIST_WKSP_SIZE_U32
 */
    #[no_mangle]
    fn HIST_countFast_wksp(count: *mut libc::c_uint,
                           maxSymbolValuePtr: *mut libc::c_uint,
                           src: *const libc::c_void, srcSize: size_t,
                           workSpace: *mut libc::c_uint) -> size_t;
    #[no_mangle]
    fn FSE_isError(code: size_t) -> libc::c_uint;
    /* ! FSE_optimalTableLog():
    dynamically downsize 'tableLog' when conditions are met.
    It saves CPU time, by using smaller tables, while preserving or even improving compression ratio.
    @return : recommended tableLog (necessarily <= 'maxTableLog') */
    /* !
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
    /* ! FSE_readNCount():
    Read compactly saved 'normalizedCounter' from 'rBuffer'.
    @return : size read from 'rBuffer',
              or an errorCode, which can be tested using FSE_isError().
              maxSymbolValuePtr[0] and tableLogPtr[0] will also be updated with their respective values */
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
    #[no_mangle]
    fn FSE_readNCount(normalizedCounter: *mut libc::c_short,
                      maxSymbolValuePtr: *mut libc::c_uint,
                      tableLogPtr: *mut libc::c_uint,
                      rBuffer: *const libc::c_void, rBuffSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
    /* *< build a fake FSE_CTable, designed for a flat distribution, where each symbol uses nbBits */
    #[no_mangle]
    fn FSE_buildCTable_rle(ct: *mut FSE_CTable, symbolValue: libc::c_uchar)
     -> size_t;
    /* *< build a fake FSE_CTable, designed to compress always the same symbolValue */
    #[no_mangle]
    fn FSE_buildCTable_wksp(ct: *mut FSE_CTable,
                            normalizedCounter: *const libc::c_short,
                            maxSymbolValue: libc::c_uint,
                            tableLog: libc::c_uint,
                            workSpace: *mut libc::c_void, wkspSize: size_t)
     -> size_t;
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
    /* *< maximum compressed size (worst case) */
    #[no_mangle]
    fn HUF_isError(code: size_t) -> libc::c_uint;
    /* * HUF_compress4X_repeat() :
 *  Same as HUF_compress4X_wksp(), but considers using hufTable if *repeat != HUF_repeat_none.
 *  If it uses hufTable it does not modify hufTable or repeat.
 *  If it doesn't, it sets *repeat = HUF_repeat_none, and it sets hufTable to the table used.
 *  If preferRepeat then the old table will always be used if valid. */
    /* *< Can use the previous table and it is assumed to be valid */
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
    fn ZSTD_free(ptr: *mut libc::c_void, customMem: ZSTD_customMem) -> ();
    #[no_mangle]
    fn ZSTD_isError(code: size_t) -> libc::c_uint;
    #[no_mangle]
    fn ZSTD_compressBlock_btultra_dictMatchState(ms: *mut ZSTD_matchState_t_0,
                                                 seqStore: *mut seqStore_t,
                                                 rep: *mut U32,
                                                 cParams:
                                                     *const ZSTD_compressionParameters,
                                                 src: *const libc::c_void,
                                                 srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btopt_dictMatchState(ms: *mut ZSTD_matchState_t_0,
                                               seqStore: *mut seqStore_t,
                                               rep: *mut U32,
                                               cParams:
                                                   *const ZSTD_compressionParameters,
                                               src: *const libc::c_void,
                                               srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btlazy2_dictMatchState(ms: *mut ZSTD_matchState_t_0,
                                                 seqStore: *mut seqStore_t,
                                                 rep: *mut U32,
                                                 cParams:
                                                     *const ZSTD_compressionParameters,
                                                 src: *const libc::c_void,
                                                 srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_lazy2_dictMatchState(ms: *mut ZSTD_matchState_t_0,
                                               seqStore: *mut seqStore_t,
                                               rep: *mut U32,
                                               cParams:
                                                   *const ZSTD_compressionParameters,
                                               src: *const libc::c_void,
                                               srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_lazy_dictMatchState(ms: *mut ZSTD_matchState_t_0,
                                              seqStore: *mut seqStore_t,
                                              rep: *mut U32,
                                              cParams:
                                                  *const ZSTD_compressionParameters,
                                              src: *const libc::c_void,
                                              srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_greedy_dictMatchState(ms: *mut ZSTD_matchState_t_0,
                                                seqStore: *mut seqStore_t,
                                                rep: *mut U32,
                                                cParams:
                                                    *const ZSTD_compressionParameters,
                                                src: *const libc::c_void,
                                                srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_doubleFast_dictMatchState(ms:
                                                        *mut ZSTD_matchState_t_0,
                                                    seqStore: *mut seqStore_t,
                                                    rep: *mut U32,
                                                    cParams:
                                                        *const ZSTD_compressionParameters,
                                                    src: *const libc::c_void,
                                                    srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_fast_dictMatchState(ms: *mut ZSTD_matchState_t_0,
                                              seqStore: *mut seqStore_t,
                                              rep: *mut U32,
                                              cParams:
                                                  *const ZSTD_compressionParameters,
                                              src: *const libc::c_void,
                                              srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btultra_extDict(ms: *mut ZSTD_matchState_t_0,
                                          seqStore: *mut seqStore_t,
                                          rep: *mut U32,
                                          cParams:
                                              *const ZSTD_compressionParameters,
                                          src: *const libc::c_void,
                                          srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btopt_extDict(ms: *mut ZSTD_matchState_t_0,
                                        seqStore: *mut seqStore_t,
                                        rep: *mut U32,
                                        cParams:
                                            *const ZSTD_compressionParameters,
                                        src: *const libc::c_void,
                                        srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btlazy2_extDict(ms: *mut ZSTD_matchState_t_0,
                                          seqStore: *mut seqStore_t,
                                          rep: *mut U32,
                                          cParams:
                                              *const ZSTD_compressionParameters,
                                          src: *const libc::c_void,
                                          srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_lazy2_extDict(ms: *mut ZSTD_matchState_t_0,
                                        seqStore: *mut seqStore_t,
                                        rep: *mut U32,
                                        cParams:
                                            *const ZSTD_compressionParameters,
                                        src: *const libc::c_void,
                                        srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_lazy_extDict(ms: *mut ZSTD_matchState_t_0,
                                       seqStore: *mut seqStore_t,
                                       rep: *mut U32,
                                       cParams:
                                           *const ZSTD_compressionParameters,
                                       src: *const libc::c_void,
                                       srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_greedy_extDict(ms: *mut ZSTD_matchState_t_0,
                                         seqStore: *mut seqStore_t,
                                         rep: *mut U32,
                                         cParams:
                                             *const ZSTD_compressionParameters,
                                         src: *const libc::c_void,
                                         srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_doubleFast_extDict(ms: *mut ZSTD_matchState_t_0,
                                             seqStore: *mut seqStore_t,
                                             rep: *mut U32,
                                             cParams:
                                                 *const ZSTD_compressionParameters,
                                             src: *const libc::c_void,
                                             srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_fast_extDict(ms: *mut ZSTD_matchState_t_0,
                                       seqStore: *mut seqStore_t,
                                       rep: *mut U32,
                                       cParams:
                                           *const ZSTD_compressionParameters,
                                       src: *const libc::c_void,
                                       srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btultra(ms: *mut ZSTD_matchState_t_0,
                                  seqStore: *mut seqStore_t, rep: *mut U32,
                                  cParams: *const ZSTD_compressionParameters,
                                  src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_btopt(ms: *mut ZSTD_matchState_t_0,
                                seqStore: *mut seqStore_t, rep: *mut U32,
                                cParams: *const ZSTD_compressionParameters,
                                src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    /* * ZSTD_adjustCParams_internal() :
    optimize `cPar` for a given input (`srcSize` and `dictSize`).
    mostly downsizing to reduce memory consumption and initialization latency.
    Both `srcSize` and `dictSize` are optional (use 0 if unknown).
    Note : cPar is assumed validated. Use ZSTD_checkCParams() to ensure this condition. */
    /* * ZSTD_cycleLog() :
 *  condition for correct operation : hashLog > 1 */
    /* *
 * Returns the cost in bits of encoding the distribution described by count
 * using the entropy bound.
 */
    /* *
 * -log2(x / 256) lookup table for x in [0, 256).
 * If x == 0: Return 0
 * Else: Return floor(-log2(x / 256) * 256)
 */
    /* *
 * Returns the cost in bytes of encoding the normalized count header.
 * Returns an error if any of the helper functions return an error.
 */
    /* *
 * Returns the cost in bits of encoding the distribution in count using ctable.
 * Returns an error if ctable cannot represent all the symbols in count.
 */
    /* *
 * Returns the cost in bits of encoding the distribution in count using the
 * table described by norm. The max symbol support by norm is assumed >= max.
 * norm must be valid for every symbol with non-zero probability in count.
 */
    /* ! used in ZSTD_reduceIndex(). pre-emptively increase value of ZSTD_DUBT_UNSORTED_MARK */
    #[no_mangle]
    fn ZSTD_compressBlock_btlazy2(ms: *mut ZSTD_matchState_t_0,
                                  seqStore: *mut seqStore_t, rep: *mut U32,
                                  cParams: *const ZSTD_compressionParameters,
                                  src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_lazy2(ms: *mut ZSTD_matchState_t_0,
                                seqStore: *mut seqStore_t, rep: *mut U32,
                                cParams: *const ZSTD_compressionParameters,
                                src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_lazy(ms: *mut ZSTD_matchState_t_0,
                               seqStore: *mut seqStore_t, rep: *mut U32,
                               cParams: *const ZSTD_compressionParameters,
                               src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_greedy(ms: *mut ZSTD_matchState_t_0,
                                 seqStore: *mut seqStore_t, rep: *mut U32,
                                 cParams: *const ZSTD_compressionParameters,
                                 src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_doubleFast(ms: *mut ZSTD_matchState_t_0,
                                     seqStore: *mut seqStore_t, rep: *mut U32,
                                     cParams:
                                         *const ZSTD_compressionParameters,
                                     src: *const libc::c_void,
                                     srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock_fast(ms: *mut ZSTD_matchState_t_0,
                               seqStore: *mut seqStore_t, rep: *mut U32,
                               cParams: *const ZSTD_compressionParameters,
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
                              ms: *mut ZSTD_matchState_t_0,
                              seqStore: *mut seqStore_t, rep: *mut U32,
                              cParams: *const ZSTD_compressionParameters,
                              src: *const libc::c_void, srcSize: size_t)
     -> size_t;
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
                              srcSize: size_t, minMatch: U32) -> ();
    #[no_mangle]
    fn ZSTD_XXH64_update(statePtr: *mut XXH64_state_t,
                         input: *const libc::c_void, length: size_t)
     -> XXH_errorcode;
    #[no_mangle]
    fn ZSTD_XXH64_digest(statePtr: *const XXH64_state_t) -> XXH64_hash_t;
    #[no_mangle]
    fn ZSTD_updateTree(ms: *mut ZSTD_matchState_t_0,
                       cParams: *const ZSTD_compressionParameters,
                       ip: *const BYTE, iend: *const BYTE) -> ();
    #[no_mangle]
    fn ZSTD_insertAndFindFirstIndex(ms: *mut ZSTD_matchState_t_0,
                                    cParams:
                                        *const ZSTD_compressionParameters,
                                    ip: *const BYTE) -> U32;
    #[no_mangle]
    fn ZSTD_fillDoubleHashTable(ms: *mut ZSTD_matchState_t_0,
                                cParams: *const ZSTD_compressionParameters,
                                end: *const libc::c_void,
                                dtlm: ZSTD_dictTableLoadMethod_e) -> ();
    #[no_mangle]
    fn ZSTD_fillHashTable(ms: *mut ZSTD_matchState_t_0,
                          cParams: *const ZSTD_compressionParameters,
                          end: *const libc::c_void,
                          dtlm: ZSTD_dictTableLoadMethod_e) -> ();
    /* ! ZSTD_compress_frameChunk() :
*   Compress a chunk of data into one or multiple blocks.
*   All blocks will be terminated, all input will be consumed.
*   Function will issue an error if there is not enough `dstCapacity` to hold the compressed content.
*   Frame is supposed already started (header already produced)
*   @return : compressed size, or an error code
*/
    /* ! ZSTD_reduceIndex() :
*   rescale all indexes to avoid future overflow (indexes are U32) */
    /* ! ZSTD_writeEpilogue() :
*   Ends a frame.
*   @return : nb of bytes written into dst (or an error code) */
    /* ! ZSTD_compressBegin_internal() :
 * @return : 0, or an error code */
    /* * ZSTD_compress_insertDictionary() :
*   @return : dictID, or an error code */
    /* ! ZSTD_loadZstdDictionary() :
 * @return : dictID, or an error code
 *  assumptions : magic number supposed already checked
 *                dictSize supposed > 8
 */
    /* ! ZSTD_loadDictionaryContent() :
 *  @return : 0, or an error code
 */
    /* ! ZSTD_resetCCtx_internal() :
    note : `params` are assumed fully validated at this stage */
    /* ! ZSTD_invalidateMatchState()
 * Invalidate all the matches in the match finder tables.
 * Requires nextSrc and base to be set (can be NULL).
 */
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
    #[no_mangle]
    fn ZSTD_malloc(size: size_t, customMem: ZSTD_customMem)
     -> *mut libc::c_void;
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
    fn ZSTDMT_sizeof_CCtx(mtctx: *mut ZSTDMT_CCtx) -> size_t;
    #[no_mangle]
    fn ZSTDMT_getFrameProgression(mtctx: *mut ZSTDMT_CCtx)
     -> ZSTD_frameProgression;
    #[no_mangle]
    fn ZSTDMT_CCtxParam_setMTCtxParameter(params: *mut ZSTD_CCtx_params,
                                          parameter: ZSTDMT_parameter,
                                          value: libc::c_uint) -> size_t;
    #[no_mangle]
    fn ZSTDMT_CCtxParam_setNbWorkers(params: *mut ZSTD_CCtx_params,
                                     nbWorkers: libc::c_uint) -> size_t;
    #[no_mangle]
    fn ZSTDMT_compressStream_generic(mtctx: *mut ZSTDMT_CCtx,
                                     output: *mut ZSTD_outBuffer,
                                     input: *mut ZSTD_inBuffer,
                                     endOp: ZSTD_EndDirective) -> size_t;
    /* ! ZSTD_continueCCtx() :
 *  reuse CCtx without reset (note : requires no dictionary) */
    /* * Equivalence for resetCCtx purposes */
    /* * The parameters are equivalent if ldm is not enabled in both sets or
 *  all the parameters are equivalent. */
    /* ! ZSTD_getCParamsFromCDict() :
 *  as the name implies */
    /* ! ZSTD_initCStream_internal() :
 *  Private use only. Init streaming operation.
 *  expects params to be valid.
 *  must receive dict, or cdict, or none, but not both.
 *  @return : 0, or an error code */
    /* ! ZSTD_compressStream_generic() :
 *  Private use only. To be called from zstdmt_compress.c in single-thread mode. */
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
    /* * ZSTD_clampCParams() :
 *  make CParam values within valid range.
 *  @return : valid CParams */
    /* ! ZSTD_copyCCtx_internal() :
 *  Duplicate an existing context `srcCCtx` into another one `dstCCtx`.
 *  Only works during stage ZSTDcs_init (i.e. after creation, but before first call to ZSTD_compressContinue()).
 *  The "context", in this case, refers to the hash and chain tables,
 *  entropy tables, and dictionary references.
 * `windowLog` value is enforced if != 0, otherwise value is copied from srcCCtx.
 * @return : 0, or an error code */
    /* ! ZSTDMT_updateCParams_whileCompressing() :
 *  Updates only a selected set of compression parameters, to remain compatible with current frame.
 *  New parameters will be applied to next compression job. */
    #[no_mangle]
    fn ZSTDMT_updateCParams_whileCompressing(mtctx: *mut ZSTDMT_CCtx,
                                             cctxParams:
                                                 *const ZSTD_CCtx_params)
     -> ();
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
    fn ZSTD_calloc(size: size_t, customMem: ZSTD_customMem)
     -> *mut libc::c_void;
}
pub const ZSTD_p_minMatch: ZSTD_cParameter = 105;
pub type FSE_repeat = libc::c_uint;
pub const ZSTD_e_continue: ZSTD_EndDirective = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
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
    pub dictMatchState: *const ZSTD_matchState_t_0,
}
pub type unnamed = libc::c_uint;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const ZSTD_p_compressionStrategy: ZSTD_cParameter = 107;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
pub type ZSTD_OptPrice_e = libc::c_uint;
pub type U64 = uint64_t;
pub const zcss_load: ZSTD_cStreamStage = 1;
pub type XXH64_state_t = XXH64_state_s;
pub const ZSTD_error_corruption_detected: ERR_enum = 20;
pub const FSE_repeat_none: FSE_repeat = 0;
pub type ZSTD_EndDirective = libc::c_uint;
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;
pub const XXH_OK: XXH_errorcode = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
pub type ZSTD_dictTableLoadMethod_e = libc::c_uint;
pub type unnamed_0 = libc::c_uint;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type ptrdiff_t = libc::c_long;
pub const ZSTD_error_stage_wrong: ERR_enum = 60;
pub const bt_rle: unnamed_1 = 1;
pub const ZSTD_error_parameter_unsupported: ERR_enum = 40;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct rawSeqStore_t {
    pub seq: *mut rawSeq,
    pub pos: size_t,
    pub size: size_t,
    pub capacity: size_t,
}
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
pub const ZSTD_error_maxSymbolValue_tooLarge: ERR_enum = 46;
pub type ZSTD_cParameter = libc::c_uint;
pub const ZSTD_lazy: ZSTD_strategy = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type HUF_CElt = HUF_CElt_s;
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
pub const zop_predef: ZSTD_OptPrice_e = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_inBuffer_s {
    pub src: *const libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_hufCTables_t {
    pub CTable: [U32; 256],
    pub repeatMode: HUF_repeat,
}
pub type uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_CStream_t {
    pub bitContainer: size_t,
    pub bitPos: libc::c_uint,
    pub startPtr: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub endPtr: *mut libc::c_char,
}
pub const ZSTD_error_workSpace_tooSmall: ERR_enum = 66;
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_match_t {
    pub off: U32,
    pub len: U32,
}
pub const ZSTD_btopt: ZSTD_strategy = 7;
pub type size_t = libc::c_ulong;
pub const bt_reserved: unnamed_1 = 3;
pub const ZSTD_fast: ZSTD_strategy = 1;
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type unnamed_1 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
    u: U32,
    c: [BYTE; 4],
}
pub const ZSTDMT_p_overlapSectionLog: ZSTDMT_parameter = 1;
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
pub type ZSTD_strategy = libc::c_uint;
pub const ZSTD_error_parameter_outOfBound: ERR_enum = 42;
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
pub type ZSTD_CCtx_params = ZSTD_CCtx_params_s;
pub const ZSTD_dfast: ZSTD_strategy = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_frameParameters {
    pub contentSizeFlag: libc::c_uint,
    pub checksumFlag: libc::c_uint,
    pub noDictIDFlag: libc::c_uint,
}
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
pub type ZSTD_CStream = ZSTD_CCtx;
pub type ZSTD_defaultPolicy_e = libc::c_uint;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
pub const ZSTD_p_hashLog: ZSTD_cParameter = 102;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub const ZSTD_error_frameIndex_tooLarge: ERR_enum = 100;
pub const ZSTD_p_contentSizeFlag: ZSTD_cParameter = 200;
pub const ZSTD_error_version_unsupported: ERR_enum = 12;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
pub const FSE_repeat_check: FSE_repeat = 1;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_parameters {
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
}
pub const ZSTD_error_tableLog_tooLarge: ERR_enum = 44;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const MEM_static_assert: unnamed = 1;
pub const ZSTD_error_dictionary_corrupted: ERR_enum = 30;
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
pub const MEM_static_assert_0: unnamed_0 = 1;
pub const ZSTD_error_seekableIO: ERR_enum = 102;
pub const ZSTD_p_checksumFlag: ZSTD_cParameter = 201;
pub const ZSTD_p_compressionLevel: ZSTD_cParameter = 100;
pub type ZSTD_dictLoadMethod_e = libc::c_uint;
pub const ZSTD_error_prefix_unknown: ERR_enum = 10;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: libc::c_uint,
    pub ptr: *const libc::c_char,
    pub start: *const libc::c_char,
    pub limitPtr: *const libc::c_char,
}
pub const ZSTD_p_ldmBucketSizeLog: ZSTD_cParameter = 163;
pub type HUF_repeat = libc::c_uint;
pub const ZSTD_p_forceMaxWindow: ZSTD_cParameter = 1100;
pub type uint32_t = libc::c_uint;
pub type int16_t = libc::c_short;
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
pub type FSE_DTable = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct seqDef_s {
    pub offset: U32,
    pub litLength: U16,
    pub matchLength: U16,
}
pub const ZSTD_error_maxCode: ERR_enum = 120;
pub const set_rle: symbolEncodingType_e = 1;
pub type ZSTD_format_e = libc::c_uint;
pub type ZSTD_CDict = ZSTD_CDict_s;
pub const ZSTDcrp_noMemset: ZSTD_compResetPolicy_e = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct rawSeq {
    pub offset: U32,
    pub litLength: U32,
    pub matchLength: U32,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const libc::c_void,
    pub symbolTT: *const libc::c_void,
    pub stateLog: libc::c_uint,
}
pub type U16 = uint16_t;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
pub const ZSTD_error_frameParameter_unsupported: ERR_enum = 14;
pub const ZSTD_p_searchLog: ZSTD_cParameter = 104;
pub const ZSTD_error_dictionaryCreation_failed: ERR_enum = 34;
pub const set_compressed: symbolEncodingType_e = 2;
pub const zcss_init: ZSTD_cStreamStage = 0;
pub const ZSTDcs_ongoing: ZSTD_compressionStage_e = 2;
pub const ZSTD_e_end: ZSTD_EndDirective = 2;
pub type ZSTD_dictMode_e = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: libc::c_int,
    pub deltaNbBits: U32,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_cpuid_t {
    pub f1c: U32,
    pub f1d: U32,
    pub f7b: U32,
    pub f7c: U32,
}
pub const ZSTD_dictForceCopy: ZSTD_dictAttachPref_e = -1;
pub type uint64_t = libc::c_ulong;
pub const ZSTD_defaultDisallowed: ZSTD_defaultPolicy_e = 0;
pub const ZSTD_p_enableLongDistanceMatching: ZSTD_cParameter = 160;
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
pub type ZSTD_prefixDict = ZSTD_prefixDict_s;
pub type FSE_CTable = libc::c_uint;
pub const ZSTD_p_overlapSizeLog: ZSTD_cParameter = 402;
pub const set_repeat: symbolEncodingType_e = 3;
pub type U32 = uint32_t;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub type XXH_errorcode = libc::c_uint;
pub type ZSTD_compResetPolicy_e = libc::c_uint;
pub type seqDef = seqDef_s;
pub const ZSTD_p_forceAttachDict: ZSTD_cParameter = 1101;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
pub type ZSTD_matchState_t_0 = ZSTD_matchState_t;
pub const ZSTD_error_no_error: ERR_enum = 0;
pub const ZSTD_error_memory_allocation: ERR_enum = 64;
pub const ZSTD_error_init_missing: ERR_enum = 62;
pub const ZSTD_error_srcSize_wrong: ERR_enum = 72;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_blockState_t {
    pub prevCBlock: *mut ZSTD_compressedBlockState_t,
    pub nextCBlock: *mut ZSTD_compressedBlockState_t,
    pub matchState: ZSTD_matchState_t_0,
}
pub type uint16_t = libc::c_ushort;
pub const ZSTD_p_ldmMinMatch: ZSTD_cParameter = 162;
pub const ZSTD_p_windowLog: ZSTD_cParameter = 101;
pub type ZSTD_cStreamStage = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_compressedBlockState_t {
    pub entropy: ZSTD_entropyCTables_t,
    pub rep: [U32; 3],
}
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
pub const ZSTD_error_GENERIC: ERR_enum = 1;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTDMT_p_jobSize: ZSTDMT_parameter = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_prefixDict_s {
    pub dict: *const libc::c_void,
    pub dictSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
}
pub const ZSTDcs_init: ZSTD_compressionStage_e = 1;
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
pub struct ZSTD_window_t {
    pub nextSrc: *const BYTE,
    pub base: *const BYTE,
    pub dictBase: *const BYTE,
    pub dictLimit: U32,
    pub lowLimit: U32,
}
pub const ZSTD_p_format: ZSTD_cParameter = 10;
pub const ZSTD_p_dictIDFlag: ZSTD_cParameter = 202;
pub const ZSTD_error_frameParameter_windowTooLarge: ERR_enum = 16;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_frameProgression {
    pub ingested: libc::c_ulonglong,
    pub consumed: libc::c_ulonglong,
    pub produced: libc::c_ulonglong,
}
pub type ZSTD_compressionStage_e = libc::c_uint;
pub const ZSTD_p_chainLog: ZSTD_cParameter = 103;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub type ZSTDMT_parameter = libc::c_uint;
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
pub const ZSTD_p_jobSize: ZSTD_cParameter = 401;
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
pub const ZSTD_dictDefaultAttach: ZSTD_dictAttachPref_e = 0;
pub const ZSTD_defaultAllowed: ZSTD_defaultPolicy_e = 1;
pub const ZSTD_error_dictionary_wrong: ERR_enum = 32;
pub type ERR_enum = libc::c_uint;
pub type ZSTDMT_CCtx = ZSTDMT_CCtx_s;
pub type ZSTD_dictContentType_e = libc::c_uint;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub const ZSTD_error_maxSymbolValue_tooSmall: ERR_enum = 48;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DState_t {
    pub state: size_t,
    pub table: *const libc::c_void,
}
pub const ZSTD_btultra: ZSTD_strategy = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_entropyCTables_t {
    pub huf: ZSTD_hufCTables_t,
    pub fse: ZSTD_fseCTables_t,
}
pub const set_basic: symbolEncodingType_e = 0;
pub const ZSTD_p_nbWorkers: ZSTD_cParameter = 400;
pub const ZSTDcs_created: ZSTD_compressionStage_e = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ldmState_t {
    pub window: ZSTD_window_t,
    pub hashTable: *mut ldmEntry_t,
    pub bucketOffsets: *mut BYTE,
    pub hashPower: U64,
}
pub const ZSTDb_not_buffered: ZSTD_buffered_policy_e = 0;
pub type BIT_DStream_status = libc::c_uint;
pub const ZSTD_dictForceAttach: ZSTD_dictAttachPref_e = 1;
pub type symbolEncodingType_e = libc::c_uint;
pub type XXH64_hash_t = libc::c_ulonglong;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub const ZSTDcrp_continue: ZSTD_compResetPolicy_e = 0;
pub const bt_compressed: unnamed_1 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_CDict_s {
    pub dictBuffer: *mut libc::c_void,
    pub dictContent: *const libc::c_void,
    pub dictContentSize: size_t,
    pub workspace: *mut libc::c_void,
    pub workspaceSize: size_t,
    pub matchState: ZSTD_matchState_t_0,
    pub cBlockState: ZSTD_compressedBlockState_t,
    pub cParams: ZSTD_compressionParameters,
    pub customMem: ZSTD_customMem,
    pub dictID: U32,
}
pub const ZSTD_p_ldmHashLog: ZSTD_cParameter = 161;
pub const ZSTDcs_ending: ZSTD_compressionStage_e = 3;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub const ZSTD_f_zstd1_magicless: ZSTD_format_e = 1;
pub const ZSTD_error_dstSize_tooSmall: ERR_enum = 70;
pub const ZSTD_p_targetLength: ZSTD_cParameter = 106;
pub type ZSTD_ErrorCode = ERR_enum;
pub const FSE_repeat_valid: FSE_repeat = 2;
pub type S16 = int16_t;
pub const ZSTDb_buffered: ZSTD_buffered_policy_e = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ldmEntry_t {
    pub offset: U32,
    pub checksum: U32,
}
pub type BYTE = uint8_t;
pub const ZSTD_error_checksum_wrong: ERR_enum = 22;
pub type ZSTD_dictAttachPref_e = libc::c_int;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;
pub type ZSTD_blockCompressor =
    Option<unsafe extern "C" fn(_: *mut ZSTD_matchState_t_0,
                                _: *mut seqStore_t, _: *mut U32,
                                _: *const ZSTD_compressionParameters,
                                _: *const libc::c_void, _: size_t) -> size_t>;
pub const ZSTD_e_flush: ZSTD_EndDirective = 1;
pub type ZSTD_buffered_policy_e = libc::c_uint;
pub const bt_raw: unnamed_1 = 0;
pub const ZSTD_p_ldmHashEveryLog: ZSTD_cParameter = 164;
pub const ZSTD_f_zstd1: ZSTD_format_e = 0;
pub const zcss_flush: ZSTD_cStreamStage = 2;
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
    let one: unnamed_2 = unnamed_2{u: 1i32 as U32,};
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
unsafe extern "C" fn ZSTD_cpuid_sse3(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 0i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_pclmuldq(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 1i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_dtes64(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 2i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_monitor(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 3i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_dscpl(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 4i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_vmx(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 5i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_smx(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 6i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_eist(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 7i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_tm2(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 8i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_ssse3(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 9i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_cnxtid(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 10i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_fma(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 12i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_cx16(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 13i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_xtpr(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 14i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_pdcm(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 15i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_pcid(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 17i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_dca(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 18i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_sse41(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 19i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_sse42(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 20i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_x2apic(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 21i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_movbe(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 22i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_popcnt(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 23i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_tscdeadline(cpuid: ZSTD_cpuid_t)
 -> libc::c_int {
    return (cpuid.f1c & 1u32 << 24i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_aes(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 25i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_xsave(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 26i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_osxsave(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 27i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_avx(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 28i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_f16c(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 29i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_rdrand(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1c & 1u32 << 30i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_fpu(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 0i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_vme(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 1i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_de(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 2i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_pse(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 3i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_tsc(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 4i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_msr(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 5i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_pae(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 6i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_mce(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 7i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_cx8(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 8i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_apic(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 9i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_sep(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 11i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_mtrr(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 12i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_pge(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 13i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_mca(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 14i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_cmov(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 15i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_pat(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 16i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_pse36(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 17i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_psn(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 18i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_clfsh(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 19i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_ds(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 21i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_acpi(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 22i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_mmx(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 23i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_fxsr(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 24i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_sse(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 25i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_sse2(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 26i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_ss(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 27i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_htt(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 28i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_tm(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 29i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_pbe(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f1d & 1u32 << 31i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_bmi1(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 3i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_hle(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 4i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_avx2(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 5i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_smep(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 7i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_bmi2(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 8i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_erms(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 9i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_invpcid(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 10i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_rtm(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 11i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_mpx(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 14i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_avx512f(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 16i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_avx512dq(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 17i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_rdseed(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 18i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_adx(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 19i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_smap(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 20i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_avx512ifma(cpuid: ZSTD_cpuid_t)
 -> libc::c_int {
    return (cpuid.f7b & 1u32 << 21i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_pcommit(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 22i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_clflushopt(cpuid: ZSTD_cpuid_t)
 -> libc::c_int {
    return (cpuid.f7b & 1u32 << 23i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_clwb(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 24i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_avx512pf(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 26i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_avx512er(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 27i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_avx512cd(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 28i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_sha(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 29i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_avx512bw(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 30i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_avx512vl(cpuid: ZSTD_cpuid_t) -> libc::c_int {
    return (cpuid.f7b & 1u32 << 31i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_prefetchwt1(cpuid: ZSTD_cpuid_t)
 -> libc::c_int {
    return (cpuid.f7c & 1u32 << 0i32 != 0i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ZSTD_cpuid_avx512vbmi(cpuid: ZSTD_cpuid_t)
 -> libc::c_int {
    return (cpuid.f7c & 1u32 << 1i32 != 0i32 as libc::c_uint) as libc::c_int;
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
                    current_block = 11014916057495648174;
                }
                6 => { current_block = 11014916057495648174; }
                5 => { current_block = 2603929150666947135; }
                4 => { current_block = 5383788646871268118; }
                3 => { current_block = 12040907182523223743; }
                2 => { current_block = 9965995598603309005; }
                _ => { current_block = 10886091980245723256; }
            }
            match current_block {
                11014916057495648174 => {
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
                    current_block = 2603929150666947135;
                }
                _ => { }
            }
            match current_block {
                2603929150666947135 => {
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
                    current_block = 5383788646871268118;
                }
                _ => { }
            }
            match current_block {
                5383788646871268118 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(3isize)
                                                              as size_t) <<
                                                             24i32) as size_t
                            as size_t;
                    current_block = 12040907182523223743;
                }
                _ => { }
            }
            match current_block {
                12040907182523223743 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(2isize)
                                                              as size_t) <<
                                                             16i32) as size_t
                            as size_t;
                    current_block = 9965995598603309005;
                }
                _ => { }
            }
            match current_block {
                9965995598603309005 => {
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
                                                                          searchLength:
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
                                           overlapSizeLog: 0,
                                           ldmParams:
                                               ldmParams_t{enableLdm: 0,
                                                           hashLog: 0,
                                                           bucketSizeLog: 0,
                                                           minMatchLength: 0,
                                                           hashEveryLog: 0,
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
                                                                          searchLength:
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
                                           overlapSizeLog: 0,
                                           ldmParams:
                                               ldmParams_t{enableLdm: 0,
                                                           hashLog: 0,
                                                           bucketSizeLog: 0,
                                                           minMatchLength: 0,
                                                           hashEveryLog: 0,
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
                                                                                       *mut U32,
                                                                               litLengthFreq:
                                                                                   0
                                                                                       as
                                                                                       *mut U32,
                                                                               matchLengthFreq:
                                                                                   0
                                                                                       as
                                                                                       *mut U32,
                                                                               offCodeFreq:
                                                                                   0
                                                                                       as
                                                                                       *mut U32,
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
                                                                        *const ZSTD_matchState_t_0,},},
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
    ZSTD_initCCtx(&mut ctxBody as *mut ZSTD_CCtx, ZSTD_defaultCMem);
    result =
        ZSTD_compressCCtx(&mut ctxBody as *mut ZSTD_CCtx, dst, dstCapacity,
                          src, srcSize, compressionLevel);
    ZSTD_freeCCtxContent(&mut ctxBody as *mut ZSTD_CCtx);
    return result;
}
unsafe extern "C" fn ZSTD_freeCCtxContent(mut cctx: *mut ZSTD_CCtx) -> () {
    ZSTD_free((*cctx).workSpace, (*cctx).customMem);
    (*cctx).workSpace = 0 as *mut libc::c_void;
    ZSTD_freeCDict((*cctx).cdictLocal);
    (*cctx).cdictLocal = 0 as *mut ZSTD_CDict;
    ZSTDMT_freeCCtx((*cctx).mtctx);
    (*cctx).mtctx = 0 as *mut ZSTDMT_CCtx;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeCDict(mut cdict: *mut ZSTD_CDict)
 -> size_t {
    if cdict.is_null() {
        return 0i32 as size_t
    } else {
        let cMem: ZSTD_customMem = (*cdict).customMem;
        ZSTD_free((*cdict).workspace, cMem);
        ZSTD_free((*cdict).dictBuffer, cMem);
        ZSTD_free(cdict as *mut libc::c_void, cMem);
        return 0i32 as size_t
    };
}
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
                                                       searchLength: 0,
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
    params.fParams.contentSizeFlag = 1i32 as libc::c_uint;
    return params;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getCParams(mut compressionLevel: libc::c_int,
                                         mut srcSizeHint: libc::c_ulonglong,
                                         mut dictSize: size_t)
 -> ZSTD_compressionParameters {
    let addedSize: size_t =
        (if 0 != srcSizeHint { 0i32 } else { 500i32 }) as size_t;
    let rSize: U64 =
        (if 0 != srcSizeHint.wrapping_add(dictSize as libc::c_ulonglong) {
             srcSizeHint.wrapping_add(dictSize as
                                          libc::c_ulonglong).wrapping_add(addedSize
                                                                              as
                                                                              libc::c_ulonglong)
         } else { -1i32 as U64 as libc::c_ulonglong }) as U64;
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
    unsafe {
        [[ZSTD_compressionParameters{windowLog: 19i32 as libc::c_uint,
                                     chainLog: 12i32 as libc::c_uint,
                                     hashLog: 13i32 as libc::c_uint,
                                     searchLog: 1i32 as libc::c_uint,
                                     searchLength: 6i32 as libc::c_uint,
                                     targetLength: 1i32 as libc::c_uint,
                                     strategy: ZSTD_fast,},
          ZSTD_compressionParameters{windowLog: 19i32 as libc::c_uint,
                                     chainLog: 13i32 as libc::c_uint,
                                     hashLog: 14i32 as libc::c_uint,
                                     searchLog: 1i32 as libc::c_uint,
                                     searchLength: 7i32 as libc::c_uint,
                                     targetLength: 0i32 as libc::c_uint,
                                     strategy: ZSTD_fast,},
          ZSTD_compressionParameters{windowLog: 19i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 16i32 as libc::c_uint,
                                     searchLog: 1i32 as libc::c_uint,
                                     searchLength: 6i32 as libc::c_uint,
                                     targetLength: 0i32 as libc::c_uint,
                                     strategy: ZSTD_fast,},
          ZSTD_compressionParameters{windowLog: 20i32 as libc::c_uint,
                                     chainLog: 16i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 1i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 1i32 as libc::c_uint,
                                     strategy: ZSTD_dfast,},
          ZSTD_compressionParameters{windowLog: 20i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 18i32 as libc::c_uint,
                                     searchLog: 1i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 1i32 as libc::c_uint,
                                     strategy: ZSTD_dfast,},
          ZSTD_compressionParameters{windowLog: 20i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 18i32 as libc::c_uint,
                                     searchLog: 2i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 2i32 as libc::c_uint,
                                     strategy: ZSTD_greedy,},
          ZSTD_compressionParameters{windowLog: 21i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 2i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 4i32 as libc::c_uint,
                                     strategy: ZSTD_lazy,},
          ZSTD_compressionParameters{windowLog: 21i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 3i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 8i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 21i32 as libc::c_uint,
                                     chainLog: 19i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 3i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 16i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 21i32 as libc::c_uint,
                                     chainLog: 19i32 as libc::c_uint,
                                     hashLog: 20i32 as libc::c_uint,
                                     searchLog: 4i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 16i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 21i32 as libc::c_uint,
                                     chainLog: 20i32 as libc::c_uint,
                                     hashLog: 21i32 as libc::c_uint,
                                     searchLog: 4i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 16i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 21i32 as libc::c_uint,
                                     chainLog: 21i32 as libc::c_uint,
                                     hashLog: 22i32 as libc::c_uint,
                                     searchLog: 4i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 16i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 22i32 as libc::c_uint,
                                     chainLog: 20i32 as libc::c_uint,
                                     hashLog: 22i32 as libc::c_uint,
                                     searchLog: 5i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 16i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 22i32 as libc::c_uint,
                                     chainLog: 21i32 as libc::c_uint,
                                     hashLog: 22i32 as libc::c_uint,
                                     searchLog: 4i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 32i32 as libc::c_uint,
                                     strategy: ZSTD_btlazy2,},
          ZSTD_compressionParameters{windowLog: 22i32 as libc::c_uint,
                                     chainLog: 21i32 as libc::c_uint,
                                     hashLog: 22i32 as libc::c_uint,
                                     searchLog: 5i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 32i32 as libc::c_uint,
                                     strategy: ZSTD_btlazy2,},
          ZSTD_compressionParameters{windowLog: 22i32 as libc::c_uint,
                                     chainLog: 22i32 as libc::c_uint,
                                     hashLog: 22i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 32i32 as libc::c_uint,
                                     strategy: ZSTD_btlazy2,},
          ZSTD_compressionParameters{windowLog: 22i32 as libc::c_uint,
                                     chainLog: 21i32 as libc::c_uint,
                                     hashLog: 22i32 as libc::c_uint,
                                     searchLog: 4i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 48i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 23i32 as libc::c_uint,
                                     chainLog: 22i32 as libc::c_uint,
                                     hashLog: 22i32 as libc::c_uint,
                                     searchLog: 4i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 64i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 23i32 as libc::c_uint,
                                     chainLog: 23i32 as libc::c_uint,
                                     hashLog: 22i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 256i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 23i32 as libc::c_uint,
                                     chainLog: 24i32 as libc::c_uint,
                                     hashLog: 22i32 as libc::c_uint,
                                     searchLog: 7i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 256i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,},
          ZSTD_compressionParameters{windowLog: 25i32 as libc::c_uint,
                                     chainLog: 25i32 as libc::c_uint,
                                     hashLog: 23i32 as libc::c_uint,
                                     searchLog: 7i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 256i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,},
          ZSTD_compressionParameters{windowLog: 26i32 as libc::c_uint,
                                     chainLog: 26i32 as libc::c_uint,
                                     hashLog: 24i32 as libc::c_uint,
                                     searchLog: 7i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 512i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,},
          ZSTD_compressionParameters{windowLog: 27i32 as libc::c_uint,
                                     chainLog: 27i32 as libc::c_uint,
                                     hashLog: 25i32 as libc::c_uint,
                                     searchLog: 9i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 999i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,}],
         [ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 12i32 as libc::c_uint,
                                     hashLog: 13i32 as libc::c_uint,
                                     searchLog: 1i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 1i32 as libc::c_uint,
                                     strategy: ZSTD_fast,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 13i32 as libc::c_uint,
                                     hashLog: 14i32 as libc::c_uint,
                                     searchLog: 1i32 as libc::c_uint,
                                     searchLength: 6i32 as libc::c_uint,
                                     targetLength: 0i32 as libc::c_uint,
                                     strategy: ZSTD_fast,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 14i32 as libc::c_uint,
                                     hashLog: 14i32 as libc::c_uint,
                                     searchLog: 1i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 1i32 as libc::c_uint,
                                     strategy: ZSTD_dfast,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 16i32 as libc::c_uint,
                                     hashLog: 16i32 as libc::c_uint,
                                     searchLog: 1i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 1i32 as libc::c_uint,
                                     strategy: ZSTD_dfast,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 16i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 2i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 2i32 as libc::c_uint,
                                     strategy: ZSTD_greedy,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 18i32 as libc::c_uint,
                                     searchLog: 3i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 2i32 as libc::c_uint,
                                     strategy: ZSTD_greedy,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 3i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 4i32 as libc::c_uint,
                                     strategy: ZSTD_lazy,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 4i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 4i32 as libc::c_uint,
                                     strategy: ZSTD_lazy,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 4i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 8i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 5i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 8i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 8i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 5i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 16i32 as libc::c_uint,
                                     strategy: ZSTD_btlazy2,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 19i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 16i32 as libc::c_uint,
                                     strategy: ZSTD_btlazy2,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 19i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 8i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 16i32 as libc::c_uint,
                                     strategy: ZSTD_btlazy2,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 4i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 24i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 4i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 24i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 19i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 64i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 19i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 8i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 128i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 19i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 10i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 256i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 19i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 10i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 256i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 19i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 11i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 512i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 19i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 12i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 512i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,},
          ZSTD_compressionParameters{windowLog: 18i32 as libc::c_uint,
                                     chainLog: 19i32 as libc::c_uint,
                                     hashLog: 19i32 as libc::c_uint,
                                     searchLog: 13i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 999i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,}],
         [ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 12i32 as libc::c_uint,
                                     hashLog: 12i32 as libc::c_uint,
                                     searchLog: 1i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 1i32 as libc::c_uint,
                                     strategy: ZSTD_fast,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 12i32 as libc::c_uint,
                                     hashLog: 13i32 as libc::c_uint,
                                     searchLog: 1i32 as libc::c_uint,
                                     searchLength: 6i32 as libc::c_uint,
                                     targetLength: 0i32 as libc::c_uint,
                                     strategy: ZSTD_fast,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 13i32 as libc::c_uint,
                                     hashLog: 15i32 as libc::c_uint,
                                     searchLog: 1i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 0i32 as libc::c_uint,
                                     strategy: ZSTD_fast,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 16i32 as libc::c_uint,
                                     searchLog: 2i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 1i32 as libc::c_uint,
                                     strategy: ZSTD_dfast,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 17i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 2i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 1i32 as libc::c_uint,
                                     strategy: ZSTD_dfast,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 16i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 3i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 2i32 as libc::c_uint,
                                     strategy: ZSTD_greedy,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 17i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 3i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 4i32 as libc::c_uint,
                                     strategy: ZSTD_lazy,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 17i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 3i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 8i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 17i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 4i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 8i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 17i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 5i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 8i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 17i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 8i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 17i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 7i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 8i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 16i32 as libc::c_uint,
                                     strategy: ZSTD_btlazy2,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 8i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 16i32 as libc::c_uint,
                                     strategy: ZSTD_btlazy2,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 4i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 32i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 64i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 7i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 128i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 7i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 256i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 8i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 256i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 8i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 256i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 9i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 256i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 10i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 256i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,},
          ZSTD_compressionParameters{windowLog: 17i32 as libc::c_uint,
                                     chainLog: 18i32 as libc::c_uint,
                                     hashLog: 17i32 as libc::c_uint,
                                     searchLog: 11i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 512i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,}],
         [ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 12i32 as libc::c_uint,
                                     hashLog: 13i32 as libc::c_uint,
                                     searchLog: 1i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 1i32 as libc::c_uint,
                                     strategy: ZSTD_fast,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 14i32 as libc::c_uint,
                                     hashLog: 15i32 as libc::c_uint,
                                     searchLog: 1i32 as libc::c_uint,
                                     searchLength: 5i32 as libc::c_uint,
                                     targetLength: 0i32 as libc::c_uint,
                                     strategy: ZSTD_fast,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 14i32 as libc::c_uint,
                                     hashLog: 15i32 as libc::c_uint,
                                     searchLog: 1i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 0i32 as libc::c_uint,
                                     strategy: ZSTD_fast,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 14i32 as libc::c_uint,
                                     hashLog: 14i32 as libc::c_uint,
                                     searchLog: 2i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 1i32 as libc::c_uint,
                                     strategy: ZSTD_dfast,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 14i32 as libc::c_uint,
                                     hashLog: 14i32 as libc::c_uint,
                                     searchLog: 4i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 2i32 as libc::c_uint,
                                     strategy: ZSTD_greedy,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 14i32 as libc::c_uint,
                                     hashLog: 14i32 as libc::c_uint,
                                     searchLog: 3i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 4i32 as libc::c_uint,
                                     strategy: ZSTD_lazy,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 14i32 as libc::c_uint,
                                     hashLog: 14i32 as libc::c_uint,
                                     searchLog: 4i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 8i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 14i32 as libc::c_uint,
                                     hashLog: 14i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 8i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 14i32 as libc::c_uint,
                                     hashLog: 14i32 as libc::c_uint,
                                     searchLog: 8i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 8i32 as libc::c_uint,
                                     strategy: ZSTD_lazy2,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 14i32 as libc::c_uint,
                                     searchLog: 5i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 8i32 as libc::c_uint,
                                     strategy: ZSTD_btlazy2,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 14i32 as libc::c_uint,
                                     searchLog: 9i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 8i32 as libc::c_uint,
                                     strategy: ZSTD_btlazy2,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 14i32 as libc::c_uint,
                                     searchLog: 3i32 as libc::c_uint,
                                     searchLength: 4i32 as libc::c_uint,
                                     targetLength: 12i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 14i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 16i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 14i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 24i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 15i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 48i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 15i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 64i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 15i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 96i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 15i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 128i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 15i32 as libc::c_uint,
                                     searchLog: 8i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 256i32 as libc::c_uint,
                                     strategy: ZSTD_btopt,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 15i32 as libc::c_uint,
                                     searchLog: 6i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 256i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 15i32 as libc::c_uint,
                                     searchLog: 8i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 256i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 15i32 as libc::c_uint,
                                     searchLog: 9i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 256i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,},
          ZSTD_compressionParameters{windowLog: 14i32 as libc::c_uint,
                                     chainLog: 15i32 as libc::c_uint,
                                     hashLog: 15i32 as libc::c_uint,
                                     searchLog: 10i32 as libc::c_uint,
                                     searchLength: 3i32 as libc::c_uint,
                                     targetLength: 512i32 as libc::c_uint,
                                     strategy: ZSTD_btultra,}]]
    };
unsafe extern "C" fn ZSTD_adjustCParams_internal(mut cPar:
                                                     ZSTD_compressionParameters,
                                                 mut srcSize:
                                                     libc::c_ulonglong,
                                                 mut dictSize: size_t)
 -> ZSTD_compressionParameters {
    static mut minSrcSize: U64 = unsafe { 513i32 as U64 };
    static mut maxWindowResize: U64 = 0;
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
        static mut hashSizeMin: U32 = unsafe { (1i32 << 6i32) as U32 };
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
    if 0 != ERR_isError(errcod) {
        return errcod
    } else { return ZSTD_compressEnd(cctx, dst, dstCapacity, src, srcSize) };
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
    if 0 != ZSTD_isError(cSize) {
        return cSize
    } else {
        endResult =
            ZSTD_writeEpilogue(cctx,
                               (dst as
                                    *mut libc::c_char).offset(cSize as isize)
                                   as *mut libc::c_void,
                               dstCapacity.wrapping_sub(cSize));
        if 0 != ZSTD_isError(endResult) {
            return endResult
        } else {
            if (*cctx).pledgedSrcSizePlusOne != 0i32 as libc::c_ulonglong {
                if (*cctx).pledgedSrcSizePlusOne !=
                       (*cctx).consumedSrcSize.wrapping_add(1i32 as
                                                                libc::c_ulonglong)
                   {
                    return -(ZSTD_error_srcSize_wrong as libc::c_int) as
                               size_t
                }
            }
            return cSize.wrapping_add(endResult)
        }
    };
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
    let mut ms: *mut ZSTD_matchState_t_0 =
        &mut (*cctx).blockState.matchState as *mut ZSTD_matchState_t_0;
    let mut fhSize: size_t = 0i32 as size_t;
    if (*cctx).stage as libc::c_uint ==
           ZSTDcs_created as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    } else {
        if 0 != frame &&
               (*cctx).stage as libc::c_uint ==
                   ZSTDcs_init as libc::c_int as libc::c_uint {
            fhSize =
                ZSTD_writeFrameHeader(dst, dstCapacity, (*cctx).appliedParams,
                                      (*cctx).pledgedSrcSizePlusOne.wrapping_sub(1i32
                                                                                     as
                                                                                     libc::c_ulonglong)
                                          as U64, (*cctx).dictID);
            if 0 != ZSTD_isError(fhSize) {
                return fhSize
            } else {
                dstCapacity =
                    (dstCapacity as libc::c_ulong).wrapping_sub(fhSize) as
                        size_t as size_t;
                dst =
                    (dst as *mut libc::c_char).offset(fhSize as isize) as
                        *mut libc::c_void;
                (*cctx).stage = ZSTDcs_ongoing
            }
        }
        if 0 == srcSize {
            return fhSize
        } else {
            if 0 ==
                   ZSTD_window_update(&mut (*ms).window as *mut ZSTD_window_t,
                                      src, srcSize) {
                (*ms).nextToUpdate = (*ms).window.dictLimit
            }
            if 0 != (*cctx).appliedParams.ldmParams.enableLdm {
                ZSTD_window_update(&mut (*cctx).ldmState.window as
                                       *mut ZSTD_window_t, src, srcSize);
            }
            let cSize: size_t =
                if 0 != frame {
                    ZSTD_compress_frameChunk(cctx, dst, dstCapacity, src,
                                             srcSize, lastFrameChunk)
                } else {
                    ZSTD_compressBlock_internal(cctx, dst, dstCapacity, src,
                                                srcSize)
                };
            if 0 != ZSTD_isError(cSize) {
                return cSize
            } else {
                (*cctx).consumedSrcSize =
                    (*cctx).consumedSrcSize.wrapping_add(srcSize as
                                                             libc::c_ulonglong);
                (*cctx).producedCSize =
                    (*cctx).producedCSize.wrapping_add(cSize.wrapping_add(fhSize)
                                                           as
                                                           libc::c_ulonglong);
                if (*cctx).pledgedSrcSizePlusOne != 0i32 as libc::c_ulonglong
                   {
                    if (*cctx).consumedSrcSize.wrapping_add(1i32 as
                                                                libc::c_ulonglong)
                           > (*cctx).pledgedSrcSizePlusOne {
                        return -(ZSTD_error_srcSize_wrong as libc::c_int) as
                                   size_t
                    }
                }
                return cSize.wrapping_add(fhSize)
            }
        }
    };
}
unsafe extern "C" fn ZSTD_compressBlock_internal(mut zc: *mut ZSTD_CCtx,
                                                 mut dst: *mut libc::c_void,
                                                 mut dstCapacity: size_t,
                                                 mut src: *const libc::c_void,
                                                 mut srcSize: size_t)
 -> size_t {
    let mut blockCompressor: ZSTD_blockCompressor = None;
    let ms: *mut ZSTD_matchState_t_0 =
        &mut (*zc).blockState.matchState as *mut ZSTD_matchState_t_0;
    if srcSize <
           ((1i32 + 1i32 + 1i32) as
                libc::c_ulong).wrapping_add(ZSTD_blockHeaderSize).wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_ulong)
       {
        ZSTD_ldm_skipSequences(&mut (*zc).externSeqStore as
                                   *mut rawSeqStore_t, srcSize,
                               (*zc).appliedParams.cParams.searchLength);
        return 0i32 as size_t
    } else {
        ZSTD_resetSeqStore(&mut (*zc).seqStore as *mut seqStore_t);
        (*ms).opt.symbolCosts =
            &mut (*(*zc).blockState.prevCBlock).entropy as
                *mut ZSTD_entropyCTables_t;
        let base: *const BYTE = (*ms).window.base;
        let istart: *const BYTE = src as *const BYTE;
        let current: U32 =
            base.offset_to(istart).expect("bad offset_to") as libc::c_long as
                U32;
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
                ZSTD_ldm_blockCompress(&mut (*zc).externSeqStore as
                                           *mut rawSeqStore_t, ms,
                                       &mut (*zc).seqStore as *mut seqStore_t,
                                       (*(*zc).blockState.nextCBlock).rep.as_mut_ptr(),
                                       &mut (*zc).appliedParams.cParams as
                                           *mut ZSTD_compressionParameters,
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
                ZSTD_ldm_generateSequences(&mut (*zc).ldmState as
                                               *mut ldmState_t,
                                           &mut ldmSeqStore as
                                               *mut rawSeqStore_t,
                                           &mut (*zc).appliedParams.ldmParams
                                               as *mut ldmParams_t, src,
                                           srcSize);
            if 0 != ERR_isError(errcod) {
                return errcod
            } else {
                lastLLSize =
                    ZSTD_ldm_blockCompress(&mut ldmSeqStore as
                                               *mut rawSeqStore_t, ms,
                                           &mut (*zc).seqStore as
                                               *mut seqStore_t,
                                           (*(*zc).blockState.nextCBlock).rep.as_mut_ptr(),
                                           &mut (*zc).appliedParams.cParams as
                                               *mut ZSTD_compressionParameters,
                                           src, srcSize)
            }
        } else {
            blockCompressor =
                ZSTD_selectBlockCompressor((*zc).appliedParams.cParams.strategy,
                                           dictMode);
            lastLLSize =
                blockCompressor.expect("non-null function pointer")(ms,
                                                                    &mut (*zc).seqStore
                                                                        as
                                                                        *mut seqStore_t,
                                                                    (*(*zc).blockState.nextCBlock).rep.as_mut_ptr(),
                                                                    &mut (*zc).appliedParams.cParams
                                                                        as
                                                                        *mut ZSTD_compressionParameters,
                                                                    src,
                                                                    srcSize)
        }
        let lastLiterals: *const BYTE =
            (src as
                 *const BYTE).offset(srcSize as
                                         isize).offset(-(lastLLSize as
                                                             isize));
        ZSTD_storeLastLiterals(&mut (*zc).seqStore as *mut seqStore_t,
                               lastLiterals, lastLLSize);
        let cSize: size_t =
            ZSTD_compressSequences(&mut (*zc).seqStore as *mut seqStore_t,
                                   &mut (*(*zc).blockState.prevCBlock).entropy
                                       as *mut ZSTD_entropyCTables_t,
                                   &mut (*(*zc).blockState.nextCBlock).entropy
                                       as *mut ZSTD_entropyCTables_t,
                                   &mut (*zc).appliedParams as
                                       *mut ZSTD_CCtx_params, dst,
                                   dstCapacity, srcSize,
                                   (*zc).entropyWorkspace, (*zc).bmi2);
        if 0 != ZSTD_isError(cSize) || cSize == 0i32 as libc::c_ulong {
            return cSize
        } else {
            let tmp: *mut ZSTD_compressedBlockState_t =
                (*zc).blockState.prevCBlock;
            (*zc).blockState.prevCBlock = (*zc).blockState.nextCBlock;
            (*zc).blockState.nextCBlock = tmp;
            return cSize
        }
    };
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
                                            mut workspace: *mut U32,
                                            mut bmi2: libc::c_int) -> size_t {
    let cSize: size_t =
        ZSTD_compressSequences_internal(seqStorePtr, prevEntropy, nextEntropy,
                                        cctxParams, dst, dstCapacity,
                                        workspace, bmi2);
    if cSize == 0i32 as libc::c_ulong {
        return 0i32 as size_t
    } else if 0 !=
                  (cSize ==
                       -(ZSTD_error_dstSize_tooSmall as libc::c_int) as
                           size_t) as libc::c_int &
                      (srcSize <= dstCapacity) as libc::c_int {
        return 0i32 as size_t
    } else if 0 != ZSTD_isError(cSize) {
        return cSize
    } else {
        let maxCSize: size_t =
            srcSize.wrapping_sub(ZSTD_minGain(srcSize,
                                              (*cctxParams).cParams.strategy));
        if cSize >= maxCSize {
            return 0i32 as size_t
        } else {
            if (*nextEntropy).fse.offcode_repeatMode as libc::c_uint ==
                   FSE_repeat_valid as libc::c_int as libc::c_uint {
                (*nextEntropy).fse.offcode_repeatMode = FSE_repeat_check
            }
            return cSize
        }
    };
}
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
                                                     mut workspace: *mut U32,
                                                     bmi2: libc::c_int)
 -> size_t {
    let longOffsets: libc::c_int =
        ((*cctxParams).cParams.windowLog >
             (if 0 != MEM_32bits() { 25i32 } else { 57i32 }) as U32) as
            libc::c_int;
    let strategy: ZSTD_strategy = (*cctxParams).cParams.strategy;
    let mut count: [U32; 53] = [0; 53];
    let mut CTable_LitLength: *mut FSE_CTable =
        (*nextEntropy).fse.litlengthCTable.as_mut_ptr();
    let mut CTable_OffsetBits: *mut FSE_CTable =
        (*nextEntropy).fse.offcodeCTable.as_mut_ptr();
    let mut CTable_MatchLength: *mut FSE_CTable =
        (*nextEntropy).fse.matchlengthCTable.as_mut_ptr();
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
        (*seqStorePtr).sequencesStart.offset_to((*seqStorePtr).sequences).expect("bad offset_to")
            as libc::c_long as size_t;
    let mut seqHead: *mut BYTE = 0 as *mut BYTE;
    let mut lastNCount: *mut BYTE = 0 as *mut BYTE;
    let literals: *const BYTE = (*seqStorePtr).litStart;
    let litSize: size_t =
        literals.offset_to((*seqStorePtr).lit).expect("bad offset_to") as
            libc::c_long as size_t;
    let disableLiteralCompression: libc::c_int =
        ((*cctxParams).cParams.strategy as libc::c_uint ==
             ZSTD_fast as libc::c_int as libc::c_uint &&
             (*cctxParams).cParams.targetLength > 0i32 as libc::c_uint) as
            libc::c_int;
    let cSize: size_t =
        ZSTD_compressLiterals(&(*prevEntropy).huf as *const ZSTD_hufCTables_t,
                              &mut (*nextEntropy).huf as
                                  *mut ZSTD_hufCTables_t,
                              (*cctxParams).cParams.strategy,
                              disableLiteralCompression,
                              op as *mut libc::c_void, dstCapacity,
                              literals as *const libc::c_void, litSize,
                              workspace, bmi2);
    if 0 != ZSTD_isError(cSize) {
        return cSize
    } else {
        op = op.offset(cSize as isize);
        if (op.offset_to(oend).expect("bad offset_to") as libc::c_long) <
               (3i32 + 1i32) as libc::c_long {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
        } else {
            if nbSeq < 127i32 as libc::c_ulong {
                let fresh0 = op;
                op = op.offset(1);
                *fresh0 = nbSeq as BYTE
            } else if nbSeq < 32512i32 as libc::c_ulong {
                *op.offset(0isize) =
                    (nbSeq >> 8i32).wrapping_add(128i32 as libc::c_ulong) as
                        BYTE;
                *op.offset(1isize) = nbSeq as BYTE;
                op = op.offset(2isize)
            } else {
                *op.offset(0isize) = 255i32 as BYTE;
                MEM_writeLE16(op.offset(1isize) as *mut libc::c_void,
                              nbSeq.wrapping_sub(32512i32 as libc::c_ulong) as
                                  U16);
                op = op.offset(3isize)
            }
            if nbSeq == 0i32 as libc::c_ulong {
                memcpy(&mut (*nextEntropy).fse as *mut ZSTD_fseCTables_t as
                           *mut libc::c_void,
                       &(*prevEntropy).fse as *const ZSTD_fseCTables_t as
                           *const libc::c_void,
                       ::std::mem::size_of::<ZSTD_fseCTables_t>() as
                           libc::c_ulong);
                return ostart.offset_to(op).expect("bad offset_to") as
                           libc::c_long as size_t
            } else {
                let fresh1 = op;
                op = op.offset(1);
                seqHead = fresh1;
                ZSTD_seqToCodes(seqStorePtr);
                let mut max: U32 = 35i32 as U32;
                let mostFrequent: size_t =
                    HIST_countFast_wksp(count.as_mut_ptr(),
                                        &mut max as *mut U32,
                                        llCodeTable as *const libc::c_void,
                                        nbSeq, workspace);
                (*nextEntropy).fse.litlength_repeatMode =
                    (*prevEntropy).fse.litlength_repeatMode;
                LLtype =
                    ZSTD_selectEncodingType(&mut (*nextEntropy).fse.litlength_repeatMode
                                                as *mut FSE_repeat,
                                            count.as_mut_ptr(), max,
                                            mostFrequent, nbSeq,
                                            9i32 as libc::c_uint,
                                            (*prevEntropy).fse.litlengthCTable.as_ptr(),
                                            LL_defaultNorm.as_ptr(),
                                            LL_defaultNormLog,
                                            ZSTD_defaultAllowed, strategy) as
                        U32;
                let countSize: size_t =
                    ZSTD_buildCTable(op as *mut libc::c_void,
                                     op.offset_to(oend).expect("bad offset_to")
                                         as libc::c_long as size_t,
                                     CTable_LitLength, 9i32 as U32,
                                     LLtype as symbolEncodingType_e,
                                     count.as_mut_ptr(), max, llCodeTable,
                                     nbSeq, LL_defaultNorm.as_ptr(),
                                     LL_defaultNormLog, 35i32 as U32,
                                     (*prevEntropy).fse.litlengthCTable.as_ptr(),
                                     ::std::mem::size_of::<[FSE_CTable; 329]>()
                                         as libc::c_ulong,
                                     workspace as *mut libc::c_void,
                                     (6i32 << 10i32) as size_t);
                if 0 != ZSTD_isError(countSize) {
                    return countSize
                } else {
                    if LLtype == set_compressed as libc::c_int as libc::c_uint
                       {
                        lastNCount = op
                    }
                    op = op.offset(countSize as isize);
                    let mut max_0: U32 = 31i32 as U32;
                    let mostFrequent_0: size_t =
                        HIST_countFast_wksp(count.as_mut_ptr(),
                                            &mut max_0 as *mut U32,
                                            ofCodeTable as
                                                *const libc::c_void, nbSeq,
                                            workspace);
                    let defaultPolicy: ZSTD_defaultPolicy_e =
                        (if max_0 <= 28i32 as libc::c_uint {
                             ZSTD_defaultAllowed as libc::c_int
                         } else { ZSTD_defaultDisallowed as libc::c_int }) as
                            ZSTD_defaultPolicy_e;
                    (*nextEntropy).fse.offcode_repeatMode =
                        (*prevEntropy).fse.offcode_repeatMode;
                    Offtype =
                        ZSTD_selectEncodingType(&mut (*nextEntropy).fse.offcode_repeatMode
                                                    as *mut FSE_repeat,
                                                count.as_mut_ptr(), max_0,
                                                mostFrequent_0, nbSeq,
                                                8i32 as libc::c_uint,
                                                (*prevEntropy).fse.offcodeCTable.as_ptr(),
                                                OF_defaultNorm.as_ptr(),
                                                OF_defaultNormLog,
                                                defaultPolicy, strategy) as
                            U32;
                    let countSize_0: size_t =
                        ZSTD_buildCTable(op as *mut libc::c_void,
                                         op.offset_to(oend).expect("bad offset_to")
                                             as libc::c_long as size_t,
                                         CTable_OffsetBits, 8i32 as U32,
                                         Offtype as symbolEncodingType_e,
                                         count.as_mut_ptr(), max_0,
                                         ofCodeTable, nbSeq,
                                         OF_defaultNorm.as_ptr(),
                                         OF_defaultNormLog, 28i32 as U32,
                                         (*prevEntropy).fse.offcodeCTable.as_ptr(),
                                         ::std::mem::size_of::<[FSE_CTable; 193]>()
                                             as libc::c_ulong,
                                         workspace as *mut libc::c_void,
                                         (6i32 << 10i32) as size_t);
                    if 0 != ZSTD_isError(countSize_0) {
                        return countSize_0
                    } else {
                        if Offtype ==
                               set_compressed as libc::c_int as libc::c_uint {
                            lastNCount = op
                        }
                        op = op.offset(countSize_0 as isize);
                        let mut max_1: U32 = 52i32 as U32;
                        let mostFrequent_1: size_t =
                            HIST_countFast_wksp(count.as_mut_ptr(),
                                                &mut max_1 as *mut U32,
                                                mlCodeTable as
                                                    *const libc::c_void,
                                                nbSeq, workspace);
                        (*nextEntropy).fse.matchlength_repeatMode =
                            (*prevEntropy).fse.matchlength_repeatMode;
                        MLtype =
                            ZSTD_selectEncodingType(&mut (*nextEntropy).fse.matchlength_repeatMode
                                                        as *mut FSE_repeat,
                                                    count.as_mut_ptr(), max_1,
                                                    mostFrequent_1, nbSeq,
                                                    9i32 as libc::c_uint,
                                                    (*prevEntropy).fse.matchlengthCTable.as_ptr(),
                                                    ML_defaultNorm.as_ptr(),
                                                    ML_defaultNormLog,
                                                    ZSTD_defaultAllowed,
                                                    strategy) as U32;
                        let countSize_1: size_t =
                            ZSTD_buildCTable(op as *mut libc::c_void,
                                             op.offset_to(oend).expect("bad offset_to")
                                                 as libc::c_long as size_t,
                                             CTable_MatchLength, 9i32 as U32,
                                             MLtype as symbolEncodingType_e,
                                             count.as_mut_ptr(), max_1,
                                             mlCodeTable, nbSeq,
                                             ML_defaultNorm.as_ptr(),
                                             ML_defaultNormLog, 52i32 as U32,
                                             (*prevEntropy).fse.matchlengthCTable.as_ptr(),
                                             ::std::mem::size_of::<[FSE_CTable; 363]>()
                                                 as libc::c_ulong,
                                             workspace as *mut libc::c_void,
                                             (6i32 << 10i32) as size_t);
                        if 0 != ZSTD_isError(countSize_1) {
                            return countSize_1
                        } else {
                            if MLtype ==
                                   set_compressed as libc::c_int as
                                       libc::c_uint {
                                lastNCount = op
                            }
                            op = op.offset(countSize_1 as isize);
                            *seqHead =
                                (LLtype <<
                                     6i32).wrapping_add(Offtype <<
                                                            4i32).wrapping_add(MLtype
                                                                                   <<
                                                                                   2i32)
                                    as BYTE;
                            let bitstreamSize: size_t =
                                ZSTD_encodeSequences(op as *mut libc::c_void,
                                                     op.offset_to(oend).expect("bad offset_to")
                                                         as libc::c_long as
                                                         size_t,
                                                     CTable_MatchLength,
                                                     mlCodeTable,
                                                     CTable_OffsetBits,
                                                     ofCodeTable,
                                                     CTable_LitLength,
                                                     llCodeTable, sequences,
                                                     nbSeq, longOffsets,
                                                     bmi2);
                            if 0 != ZSTD_isError(bitstreamSize) {
                                return bitstreamSize
                            } else {
                                op = op.offset(bitstreamSize as isize);
                                if !lastNCount.is_null() &&
                                       (lastNCount.offset_to(op).expect("bad offset_to")
                                            as libc::c_long) <
                                           4i32 as libc::c_long {
                                    return 0i32 as size_t
                                } else {
                                    return ostart.offset_to(op).expect("bad offset_to")
                                               as libc::c_long as size_t
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_encodeSequences(mut dst: *mut libc::c_void,
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
                                              mut bmi2: libc::c_int)
 -> size_t {
    if 0 != bmi2 {
        return ZSTD_encodeSequences_bmi2(dst, dstCapacity, CTable_MatchLength,
                                         mlCodeTable, CTable_OffsetBits,
                                         ofCodeTable, CTable_LitLength,
                                         llCodeTable, sequences, nbSeq,
                                         longOffsets)
    } else {
        return ZSTD_encodeSequences_default(dst, dstCapacity,
                                            CTable_MatchLength, mlCodeTable,
                                            CTable_OffsetBits, ofCodeTable,
                                            CTable_LitLength, llCodeTable,
                                            sequences, nbSeq, longOffsets)
    };
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
    let errcod: size_t =
        BIT_initCStream(&mut blockStream as *mut BIT_CStream_t, dst,
                        dstCapacity);
    if 0 != ERR_isError(errcod) {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else {
        FSE_initCState2(&mut stateMatchLength as *mut FSE_CState_t,
                        CTable_MatchLength,
                        *mlCodeTable.offset(nbSeq.wrapping_sub(1i32 as
                                                                   libc::c_ulong)
                                                as isize) as U32);
        FSE_initCState2(&mut stateOffsetBits as *mut FSE_CState_t,
                        CTable_OffsetBits,
                        *ofCodeTable.offset(nbSeq.wrapping_sub(1i32 as
                                                                   libc::c_ulong)
                                                as isize) as U32);
        FSE_initCState2(&mut stateLitLength as *mut FSE_CState_t,
                        CTable_LitLength,
                        *llCodeTable.offset(nbSeq.wrapping_sub(1i32 as
                                                                   libc::c_ulong)
                                                as isize) as U32);
        BIT_addBits(&mut blockStream as *mut BIT_CStream_t,
                    (*sequences.offset(nbSeq.wrapping_sub(1i32 as
                                                              libc::c_ulong)
                                           as isize)).litLength as size_t,
                    LL_bits[*llCodeTable.offset(nbSeq.wrapping_sub(1i32 as
                                                                       libc::c_ulong)
                                                    as isize) as usize]);
        if 0 != MEM_32bits() {
            BIT_flushBits(&mut blockStream as *mut BIT_CStream_t);
        }
        BIT_addBits(&mut blockStream as *mut BIT_CStream_t,
                    (*sequences.offset(nbSeq.wrapping_sub(1i32 as
                                                              libc::c_ulong)
                                           as isize)).matchLength as size_t,
                    ML_bits[*mlCodeTable.offset(nbSeq.wrapping_sub(1i32 as
                                                                       libc::c_ulong)
                                                    as isize) as usize]);
        if 0 != MEM_32bits() {
            BIT_flushBits(&mut blockStream as *mut BIT_CStream_t);
        }
        if 0 != longOffsets {
            let ofBits: U32 =
                *ofCodeTable.offset(nbSeq.wrapping_sub(1i32 as libc::c_ulong)
                                        as isize) as U32;
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
                BIT_addBits(&mut blockStream as *mut BIT_CStream_t,
                            (*sequences.offset(nbSeq.wrapping_sub(1i32 as
                                                                      libc::c_ulong)
                                                   as isize)).offset as
                                size_t, extraBits as libc::c_uint);
                BIT_flushBits(&mut blockStream as *mut BIT_CStream_t);
            }
            BIT_addBits(&mut blockStream as *mut BIT_CStream_t,
                        ((*sequences.offset(nbSeq.wrapping_sub(1i32 as
                                                                   libc::c_ulong)
                                                as isize)).offset >>
                             extraBits) as size_t,
                        ofBits.wrapping_sub(extraBits as libc::c_uint));
        } else {
            BIT_addBits(&mut blockStream as *mut BIT_CStream_t,
                        (*sequences.offset(nbSeq.wrapping_sub(1i32 as
                                                                  libc::c_ulong)
                                               as isize)).offset as size_t,
                        *ofCodeTable.offset(nbSeq.wrapping_sub(1i32 as
                                                                   libc::c_ulong)
                                                as isize) as libc::c_uint);
        }
        BIT_flushBits(&mut blockStream as *mut BIT_CStream_t);
        let mut n: size_t = 0;
        n = nbSeq.wrapping_sub(2i32 as libc::c_ulong);
        while n < nbSeq {
            let llCode: BYTE = *llCodeTable.offset(n as isize);
            let ofCode: BYTE = *ofCodeTable.offset(n as isize);
            let mlCode: BYTE = *mlCodeTable.offset(n as isize);
            let llBits: U32 = LL_bits[llCode as usize];
            let ofBits_0: U32 = ofCode as U32;
            let mlBits: U32 = ML_bits[mlCode as usize];
            FSE_encodeSymbol(&mut blockStream as *mut BIT_CStream_t,
                             &mut stateOffsetBits as *mut FSE_CState_t,
                             ofCode as libc::c_uint);
            FSE_encodeSymbol(&mut blockStream as *mut BIT_CStream_t,
                             &mut stateMatchLength as *mut FSE_CState_t,
                             mlCode as libc::c_uint);
            if 0 != MEM_32bits() {
                BIT_flushBits(&mut blockStream as *mut BIT_CStream_t);
            }
            FSE_encodeSymbol(&mut blockStream as *mut BIT_CStream_t,
                             &mut stateLitLength as *mut FSE_CState_t,
                             llCode as libc::c_uint);
            if 0 != MEM_32bits() ||
                   ofBits_0.wrapping_add(mlBits).wrapping_add(llBits) >=
                       (64i32 - 7i32 - (9i32 + 9i32 + 8i32)) as libc::c_uint {
                BIT_flushBits(&mut blockStream as *mut BIT_CStream_t);
            }
            BIT_addBits(&mut blockStream as *mut BIT_CStream_t,
                        (*sequences.offset(n as isize)).litLength as size_t,
                        llBits);
            if 0 != MEM_32bits() &&
                   llBits.wrapping_add(mlBits) > 24i32 as libc::c_uint {
                BIT_flushBits(&mut blockStream as *mut BIT_CStream_t);
            }
            BIT_addBits(&mut blockStream as *mut BIT_CStream_t,
                        (*sequences.offset(n as isize)).matchLength as size_t,
                        mlBits);
            if 0 != MEM_32bits() ||
                   ofBits_0.wrapping_add(mlBits).wrapping_add(llBits) >
                       56i32 as libc::c_uint {
                BIT_flushBits(&mut blockStream as *mut BIT_CStream_t);
            }
            if 0 != longOffsets {
                let extraBits_0: libc::c_int =
                    ofBits_0.wrapping_sub(if ofBits_0 <
                                                 ((if 0 != MEM_32bits() {
                                                       25i32
                                                   } else { 57i32 }) as
                                                      U32).wrapping_sub(1i32
                                                                            as
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
                    BIT_addBits(&mut blockStream as *mut BIT_CStream_t,
                                (*sequences.offset(n as isize)).offset as
                                    size_t, extraBits_0 as libc::c_uint);
                    BIT_flushBits(&mut blockStream as *mut BIT_CStream_t);
                }
                BIT_addBits(&mut blockStream as *mut BIT_CStream_t,
                            ((*sequences.offset(n as isize)).offset >>
                                 extraBits_0) as size_t,
                            ofBits_0.wrapping_sub(extraBits_0 as
                                                      libc::c_uint));
            } else {
                BIT_addBits(&mut blockStream as *mut BIT_CStream_t,
                            (*sequences.offset(n as isize)).offset as size_t,
                            ofBits_0);
            }
            BIT_flushBits(&mut blockStream as *mut BIT_CStream_t);
            n = n.wrapping_sub(1)
        }
        FSE_flushCState(&mut blockStream as *mut BIT_CStream_t,
                        &mut stateMatchLength as *mut FSE_CState_t);
        FSE_flushCState(&mut blockStream as *mut BIT_CStream_t,
                        &mut stateOffsetBits as *mut FSE_CState_t);
        FSE_flushCState(&mut blockStream as *mut BIT_CStream_t,
                        &mut stateLitLength as *mut FSE_CState_t);
        let streamSize: size_t =
            BIT_closeCStream(&mut blockStream as *mut BIT_CStream_t);
        if streamSize == 0i32 as libc::c_ulong {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
        } else { return streamSize }
    };
}
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
static mut ML_defaultNormLog: U32 = unsafe { 6i32 as U32 };
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
unsafe extern "C" fn ZSTD_buildCTable(mut dst: *mut libc::c_void,
                                      mut dstCapacity: size_t,
                                      mut nextCTable: *mut FSE_CTable,
                                      mut FSELog: U32,
                                      mut type_0: symbolEncodingType_e,
                                      mut count: *mut U32, mut max: U32,
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
            *op = *codeTable.offset(0isize);
            let errcod: size_t = FSE_buildCTable_rle(nextCTable, max as BYTE);
            if 0 != ERR_isError(errcod) {
                return errcod
            } else { return 1i32 as size_t }
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
            if 0 != ERR_isError(errcod_0) {
                return errcod_0
            } else { return 0i32 as size_t }
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
            if 0 != ERR_isError(errcod_1) {
                return errcod_1
            } else {
                let NCountSize: size_t =
                    FSE_writeNCount(op as *mut libc::c_void,
                                    op.offset_to(oend).expect("bad offset_to")
                                        as libc::c_long as size_t,
                                    norm.as_mut_ptr(), max, tableLog);
                if 0 != FSE_isError(NCountSize) {
                    return NCountSize
                } else {
                    let errcod_2: size_t =
                        FSE_buildCTable_wksp(nextCTable, norm.as_mut_ptr(),
                                             max, tableLog, workspace,
                                             workspaceSize);
                    if 0 != ERR_isError(errcod_2) {
                        return errcod_2
                    } else { return NCountSize }
                }
            }
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
        } else { return set_rle }
    } else {
        if (strategy as libc::c_uint) <
               ZSTD_lazy as libc::c_int as libc::c_uint {
            if 0 != isDefaultAllowed as u64 {
                let staticFse_nbSeq_max: size_t = 1000i32 as size_t;
                let mult: size_t =
                    (10i32 as
                         libc::c_uint).wrapping_sub(strategy as libc::c_uint)
                        as size_t;
                let baseLog: size_t = 3i32 as size_t;
                let dynamicFse_nbSeq_min: size_t =
                    ((1i32 as size_t) << defaultNormLog).wrapping_mul(mult) >>
                        baseLog;
                if *repeatMode as libc::c_uint ==
                       FSE_repeat_valid as libc::c_int as libc::c_uint &&
                       nbSeq < staticFse_nbSeq_max {
                    return set_repeat
                } else if nbSeq < dynamicFse_nbSeq_min ||
                              mostFrequent <
                                  nbSeq >>
                                      defaultNormLog.wrapping_sub(1i32 as
                                                                      libc::c_uint)
                 {
                    *repeatMode = FSE_repeat_none;
                    return set_basic
                }
            }
        } else {
            let basicCost: size_t =
                if 0 != isDefaultAllowed as libc::c_uint {
                    ZSTD_crossEntropyCost(defaultNorm, defaultNormLog, count,
                                          max)
                } else { -(ZSTD_error_GENERIC as libc::c_int) as size_t };
            let repeatCost: size_t =
                if *repeatMode as libc::c_uint !=
                       FSE_repeat_none as libc::c_int as libc::c_uint {
                    ZSTD_fseBitCost(prevCTable, count, max)
                } else { -(ZSTD_error_GENERIC as libc::c_int) as size_t };
            let NCountCost: size_t =
                ZSTD_NCountCost(count, max, nbSeq, FSELog);
            let compressedCost: size_t =
                (NCountCost <<
                     3i32).wrapping_add(ZSTD_entropyCost(count, max, nbSeq));
            0 != isDefaultAllowed as u64;
            if basicCost <= repeatCost && basicCost <= compressedCost {
                *repeatMode = FSE_repeat_none;
                return set_basic
            } else if repeatCost <= compressedCost { return set_repeat }
        }
        *repeatMode = FSE_repeat_check;
        return set_compressed
    };
}
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
static mut kInverseProbabiltyLog256: [libc::c_uint; 256] =
    unsafe {
        [0i32 as libc::c_uint, 2048i32 as libc::c_uint,
         1792i32 as libc::c_uint, 1642i32 as libc::c_uint,
         1536i32 as libc::c_uint, 1453i32 as libc::c_uint,
         1386i32 as libc::c_uint, 1329i32 as libc::c_uint,
         1280i32 as libc::c_uint, 1236i32 as libc::c_uint,
         1197i32 as libc::c_uint, 1162i32 as libc::c_uint,
         1130i32 as libc::c_uint, 1100i32 as libc::c_uint,
         1073i32 as libc::c_uint, 1047i32 as libc::c_uint,
         1024i32 as libc::c_uint, 1001i32 as libc::c_uint,
         980i32 as libc::c_uint, 960i32 as libc::c_uint,
         941i32 as libc::c_uint, 923i32 as libc::c_uint,
         906i32 as libc::c_uint, 889i32 as libc::c_uint,
         874i32 as libc::c_uint, 859i32 as libc::c_uint,
         844i32 as libc::c_uint, 830i32 as libc::c_uint,
         817i32 as libc::c_uint, 804i32 as libc::c_uint,
         791i32 as libc::c_uint, 779i32 as libc::c_uint,
         768i32 as libc::c_uint, 756i32 as libc::c_uint,
         745i32 as libc::c_uint, 734i32 as libc::c_uint,
         724i32 as libc::c_uint, 714i32 as libc::c_uint,
         704i32 as libc::c_uint, 694i32 as libc::c_uint,
         685i32 as libc::c_uint, 676i32 as libc::c_uint,
         667i32 as libc::c_uint, 658i32 as libc::c_uint,
         650i32 as libc::c_uint, 642i32 as libc::c_uint,
         633i32 as libc::c_uint, 626i32 as libc::c_uint,
         618i32 as libc::c_uint, 610i32 as libc::c_uint,
         603i32 as libc::c_uint, 595i32 as libc::c_uint,
         588i32 as libc::c_uint, 581i32 as libc::c_uint,
         574i32 as libc::c_uint, 567i32 as libc::c_uint,
         561i32 as libc::c_uint, 554i32 as libc::c_uint,
         548i32 as libc::c_uint, 542i32 as libc::c_uint,
         535i32 as libc::c_uint, 529i32 as libc::c_uint,
         523i32 as libc::c_uint, 517i32 as libc::c_uint,
         512i32 as libc::c_uint, 506i32 as libc::c_uint,
         500i32 as libc::c_uint, 495i32 as libc::c_uint,
         489i32 as libc::c_uint, 484i32 as libc::c_uint,
         478i32 as libc::c_uint, 473i32 as libc::c_uint,
         468i32 as libc::c_uint, 463i32 as libc::c_uint,
         458i32 as libc::c_uint, 453i32 as libc::c_uint,
         448i32 as libc::c_uint, 443i32 as libc::c_uint,
         438i32 as libc::c_uint, 434i32 as libc::c_uint,
         429i32 as libc::c_uint, 424i32 as libc::c_uint,
         420i32 as libc::c_uint, 415i32 as libc::c_uint,
         411i32 as libc::c_uint, 407i32 as libc::c_uint,
         402i32 as libc::c_uint, 398i32 as libc::c_uint,
         394i32 as libc::c_uint, 390i32 as libc::c_uint,
         386i32 as libc::c_uint, 382i32 as libc::c_uint,
         377i32 as libc::c_uint, 373i32 as libc::c_uint,
         370i32 as libc::c_uint, 366i32 as libc::c_uint,
         362i32 as libc::c_uint, 358i32 as libc::c_uint,
         354i32 as libc::c_uint, 350i32 as libc::c_uint,
         347i32 as libc::c_uint, 343i32 as libc::c_uint,
         339i32 as libc::c_uint, 336i32 as libc::c_uint,
         332i32 as libc::c_uint, 329i32 as libc::c_uint,
         325i32 as libc::c_uint, 322i32 as libc::c_uint,
         318i32 as libc::c_uint, 315i32 as libc::c_uint,
         311i32 as libc::c_uint, 308i32 as libc::c_uint,
         305i32 as libc::c_uint, 302i32 as libc::c_uint,
         298i32 as libc::c_uint, 295i32 as libc::c_uint,
         292i32 as libc::c_uint, 289i32 as libc::c_uint,
         286i32 as libc::c_uint, 282i32 as libc::c_uint,
         279i32 as libc::c_uint, 276i32 as libc::c_uint,
         273i32 as libc::c_uint, 270i32 as libc::c_uint,
         267i32 as libc::c_uint, 264i32 as libc::c_uint,
         261i32 as libc::c_uint, 258i32 as libc::c_uint,
         256i32 as libc::c_uint, 253i32 as libc::c_uint,
         250i32 as libc::c_uint, 247i32 as libc::c_uint,
         244i32 as libc::c_uint, 241i32 as libc::c_uint,
         239i32 as libc::c_uint, 236i32 as libc::c_uint,
         233i32 as libc::c_uint, 230i32 as libc::c_uint,
         228i32 as libc::c_uint, 225i32 as libc::c_uint,
         222i32 as libc::c_uint, 220i32 as libc::c_uint,
         217i32 as libc::c_uint, 215i32 as libc::c_uint,
         212i32 as libc::c_uint, 209i32 as libc::c_uint,
         207i32 as libc::c_uint, 204i32 as libc::c_uint,
         202i32 as libc::c_uint, 199i32 as libc::c_uint,
         197i32 as libc::c_uint, 194i32 as libc::c_uint,
         192i32 as libc::c_uint, 190i32 as libc::c_uint,
         187i32 as libc::c_uint, 185i32 as libc::c_uint,
         182i32 as libc::c_uint, 180i32 as libc::c_uint,
         178i32 as libc::c_uint, 175i32 as libc::c_uint,
         173i32 as libc::c_uint, 171i32 as libc::c_uint,
         168i32 as libc::c_uint, 166i32 as libc::c_uint,
         164i32 as libc::c_uint, 162i32 as libc::c_uint,
         159i32 as libc::c_uint, 157i32 as libc::c_uint,
         155i32 as libc::c_uint, 153i32 as libc::c_uint,
         151i32 as libc::c_uint, 149i32 as libc::c_uint,
         146i32 as libc::c_uint, 144i32 as libc::c_uint,
         142i32 as libc::c_uint, 140i32 as libc::c_uint,
         138i32 as libc::c_uint, 136i32 as libc::c_uint,
         134i32 as libc::c_uint, 132i32 as libc::c_uint,
         130i32 as libc::c_uint, 128i32 as libc::c_uint,
         126i32 as libc::c_uint, 123i32 as libc::c_uint,
         121i32 as libc::c_uint, 119i32 as libc::c_uint,
         117i32 as libc::c_uint, 115i32 as libc::c_uint,
         114i32 as libc::c_uint, 112i32 as libc::c_uint,
         110i32 as libc::c_uint, 108i32 as libc::c_uint,
         106i32 as libc::c_uint, 104i32 as libc::c_uint,
         102i32 as libc::c_uint, 100i32 as libc::c_uint,
         98i32 as libc::c_uint, 96i32 as libc::c_uint, 94i32 as libc::c_uint,
         93i32 as libc::c_uint, 91i32 as libc::c_uint, 89i32 as libc::c_uint,
         87i32 as libc::c_uint, 85i32 as libc::c_uint, 83i32 as libc::c_uint,
         82i32 as libc::c_uint, 80i32 as libc::c_uint, 78i32 as libc::c_uint,
         76i32 as libc::c_uint, 74i32 as libc::c_uint, 73i32 as libc::c_uint,
         71i32 as libc::c_uint, 69i32 as libc::c_uint, 67i32 as libc::c_uint,
         66i32 as libc::c_uint, 64i32 as libc::c_uint, 62i32 as libc::c_uint,
         61i32 as libc::c_uint, 59i32 as libc::c_uint, 57i32 as libc::c_uint,
         55i32 as libc::c_uint, 54i32 as libc::c_uint, 52i32 as libc::c_uint,
         50i32 as libc::c_uint, 49i32 as libc::c_uint, 47i32 as libc::c_uint,
         46i32 as libc::c_uint, 44i32 as libc::c_uint, 42i32 as libc::c_uint,
         41i32 as libc::c_uint, 39i32 as libc::c_uint, 37i32 as libc::c_uint,
         36i32 as libc::c_uint, 34i32 as libc::c_uint, 33i32 as libc::c_uint,
         31i32 as libc::c_uint, 30i32 as libc::c_uint, 28i32 as libc::c_uint,
         26i32 as libc::c_uint, 25i32 as libc::c_uint, 23i32 as libc::c_uint,
         22i32 as libc::c_uint, 20i32 as libc::c_uint, 19i32 as libc::c_uint,
         17i32 as libc::c_uint, 16i32 as libc::c_uint, 14i32 as libc::c_uint,
         13i32 as libc::c_uint, 11i32 as libc::c_uint, 10i32 as libc::c_uint,
         8i32 as libc::c_uint, 7i32 as libc::c_uint, 5i32 as libc::c_uint,
         4i32 as libc::c_uint, 2i32 as libc::c_uint, 1i32 as libc::c_uint]
    };
unsafe extern "C" fn ZSTD_NCountCost(mut count: *const libc::c_uint,
                                     max: libc::c_uint, nbSeq: size_t,
                                     FSELog: libc::c_uint) -> size_t {
    let mut wksp: [BYTE; 512] = [0; 512];
    let mut norm: [S16; 53] = [0; 53];
    let tableLog: U32 = FSE_optimalTableLog(FSELog, nbSeq, max);
    let errcod: size_t =
        FSE_normalizeCount(norm.as_mut_ptr(), tableLog, count, nbSeq, max);
    if 0 != ERR_isError(errcod) {
        return errcod
    } else {
        return FSE_writeNCount(wksp.as_mut_ptr() as *mut libc::c_void,
                               ::std::mem::size_of::<[BYTE; 512]>() as
                                   libc::c_ulong, norm.as_mut_ptr(), max,
                               tableLog)
    };
}
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
    FSE_initCState(&mut cstate as *mut FSE_CState_t, ctable);
    if ZSTD_getFSEMaxSymbolValue(ctable) < max {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
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
                } else {
                    cost =
                        (cost as
                             libc::c_ulong).wrapping_add((*count.offset(s as
                                                                            isize)).wrapping_mul(bitCost)
                                                             as libc::c_ulong)
                            as size_t as size_t
                }
            }
            s = s.wrapping_add(1)
        }
        return cost >> kAccuracyLog
    };
}
unsafe extern "C" fn ZSTD_getFSEMaxSymbolValue(mut ctable: *const FSE_CTable)
 -> libc::c_uint {
    let mut ptr: *const libc::c_void = ctable as *const libc::c_void;
    let mut u16ptr: *const U16 = ptr as *const U16;
    let maxSymbolValue: U32 =
        MEM_read16(u16ptr.offset(1isize) as *const libc::c_void) as U32;
    return maxSymbolValue;
}
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
static mut OF_defaultNormLog: U32 = unsafe { 5i32 as U32 };
static mut OF_defaultNorm: [S16; 29] =
    unsafe {
        [1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 2i32 as S16, 2i32 as S16, 2i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16,
         1i32 as S16, 1i32 as S16, 1i32 as S16, 1i32 as S16, -1i32 as S16,
         -1i32 as S16, -1i32 as S16, -1i32 as S16, -1i32 as S16]
    };
static mut LL_defaultNormLog: U32 = unsafe { 6i32 as U32 };
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
#[no_mangle]
pub unsafe extern "C" fn ZSTD_seqToCodes(mut seqStorePtr: *const seqStore_t)
 -> () {
    let sequences: *const seqDef = (*seqStorePtr).sequencesStart;
    let llCodeTable: *mut BYTE = (*seqStorePtr).llCode;
    let ofCodeTable: *mut BYTE = (*seqStorePtr).ofCode;
    let mlCodeTable: *mut BYTE = (*seqStorePtr).mlCode;
    let nbSeq: U32 =
        (*seqStorePtr).sequencesStart.offset_to((*seqStorePtr).sequences).expect("bad offset_to")
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
                                           mut workspace: *mut U32,
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
    } else {
        let minLitSize: size_t =
            (if (*prevHuf).repeatMode as libc::c_uint ==
                    HUF_repeat_valid as libc::c_int as libc::c_uint {
                 6i32
             } else { 63i32 }) as size_t;
        if srcSize <= minLitSize {
            return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize)
        } else if dstCapacity < lhSize.wrapping_add(1i32 as libc::c_ulong) {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
        } else {
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
                                          dstCapacity.wrapping_sub(lhSize),
                                          src, srcSize,
                                          255i32 as libc::c_uint,
                                          11i32 as libc::c_uint,
                                          workspace as *mut libc::c_void,
                                          (6i32 << 10i32) as size_t,
                                          (*nextHuf).CTable.as_mut_ptr() as
                                              *mut HUF_CElt,
                                          &mut repeat as *mut HUF_repeat,
                                          preferRepeat, bmi2)
                } else {
                    HUF_compress4X_repeat(ostart.offset(lhSize as isize) as
                                              *mut libc::c_void,
                                          dstCapacity.wrapping_sub(lhSize),
                                          src, srcSize,
                                          255i32 as libc::c_uint,
                                          11i32 as libc::c_uint,
                                          workspace as *mut libc::c_void,
                                          (6i32 << 10i32) as size_t,
                                          (*nextHuf).CTable.as_mut_ptr() as
                                              *mut HUF_CElt,
                                          &mut repeat as *mut HUF_repeat,
                                          preferRepeat, bmi2)
                };
            if repeat as libc::c_uint !=
                   HUF_repeat_none as libc::c_int as libc::c_uint {
                hType = set_repeat
            }
            if 0 !=
                   ((cLitSize == 0i32 as libc::c_ulong) as libc::c_int |
                        (cLitSize >= srcSize.wrapping_sub(minGain)) as
                            libc::c_int) as libc::c_uint |
                       ERR_isError(cLitSize) {
                memcpy(nextHuf as *mut libc::c_void,
                       prevHuf as *const libc::c_void,
                       ::std::mem::size_of::<ZSTD_hufCTables_t>() as
                           libc::c_ulong);
                return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize)
            } else if cLitSize == 1i32 as libc::c_ulong {
                memcpy(nextHuf as *mut libc::c_void,
                       prevHuf as *const libc::c_void,
                       ::std::mem::size_of::<ZSTD_hufCTables_t>() as
                           libc::c_ulong);
                return ZSTD_compressRleLiteralsBlock(dst, dstCapacity, src,
                                                     srcSize)
            } else {
                if hType as libc::c_uint ==
                       set_compressed as libc::c_int as libc::c_uint {
                    (*nextHuf).repeatMode = HUF_repeat_check
                }
                match lhSize {
                    3 => {
                        let lhc: U32 =
                            (hType as
                                 libc::c_uint).wrapping_add((((0 ==
                                                                   singleStream)
                                                                  as
                                                                  libc::c_int)
                                                                 << 2i32) as
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
                return lhSize.wrapping_add(cLitSize)
            }
        }
    };
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
    } else {
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
                                   libc::c_ulong).wrapping_add(srcSize <<
                                                                   4i32) as
                                  U16);
            }
            3 => {
                MEM_writeLE32(ostart as *mut libc::c_void,
                              ((set_basic as libc::c_int as
                                    U32).wrapping_add((3i32 << 2i32) as
                                                          libc::c_uint) as
                                   libc::c_ulong).wrapping_add(srcSize <<
                                                                   4i32) as
                                  U32);
            }
            _ => { }
        }
        memcpy(ostart.offset(flSize as isize) as *mut libc::c_void, src,
               srcSize);
        return srcSize.wrapping_add(flSize as libc::c_ulong)
    };
}
unsafe extern "C" fn ZSTD_minGain(mut srcSize: size_t,
                                  mut strat: ZSTD_strategy) -> size_t {
    let minlog: U32 =
        (if strat as libc::c_uint ==
                ZSTD_btultra as libc::c_int as libc::c_uint {
             7i32
         } else { 6i32 }) as U32;
    return (srcSize >> minlog).wrapping_add(2i32 as libc::c_ulong);
}
unsafe extern "C" fn ZSTD_storeLastLiterals(mut seqStorePtr: *mut seqStore_t,
                                            mut anchor: *const BYTE,
                                            mut lastLLSize: size_t) -> () {
    memcpy((*seqStorePtr).lit as *mut libc::c_void,
           anchor as *const libc::c_void, lastLLSize);
    (*seqStorePtr).lit = (*seqStorePtr).lit.offset(lastLLSize as isize);
}
unsafe extern "C" fn ZSTD_matchState_dictMode(mut ms:
                                                  *const ZSTD_matchState_t_0)
 -> ZSTD_dictMode_e {
    return (if 0 != ZSTD_window_hasExtDict((*ms).window) {
                ZSTD_extDict as libc::c_int
            } else if !(*ms).dictMatchState.is_null() {
                ZSTD_dictMatchState as libc::c_int
            } else { ZSTD_noDict as libc::c_int }) as ZSTD_dictMode_e;
}
unsafe extern "C" fn ZSTD_window_hasExtDict(window: ZSTD_window_t) -> U32 {
    return (window.lowLimit < window.dictLimit) as libc::c_int as U32;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_selectBlockCompressor(mut strat: ZSTD_strategy,
                                                    mut dictMode:
                                                        ZSTD_dictMode_e)
 -> ZSTD_blockCompressor {
    static mut blockCompressor: [[ZSTD_blockCompressor; 9]; 3] =
        unsafe {
            [[Some(ZSTD_compressBlock_fast), Some(ZSTD_compressBlock_fast),
              Some(ZSTD_compressBlock_doubleFast),
              Some(ZSTD_compressBlock_greedy), Some(ZSTD_compressBlock_lazy),
              Some(ZSTD_compressBlock_lazy2),
              Some(ZSTD_compressBlock_btlazy2),
              Some(ZSTD_compressBlock_btopt),
              Some(ZSTD_compressBlock_btultra)],
             [Some(ZSTD_compressBlock_fast_extDict),
              Some(ZSTD_compressBlock_fast_extDict),
              Some(ZSTD_compressBlock_doubleFast_extDict),
              Some(ZSTD_compressBlock_greedy_extDict),
              Some(ZSTD_compressBlock_lazy_extDict),
              Some(ZSTD_compressBlock_lazy2_extDict),
              Some(ZSTD_compressBlock_btlazy2_extDict),
              Some(ZSTD_compressBlock_btopt_extDict),
              Some(ZSTD_compressBlock_btultra_extDict)],
             [Some(ZSTD_compressBlock_fast_dictMatchState),
              Some(ZSTD_compressBlock_fast_dictMatchState),
              Some(ZSTD_compressBlock_doubleFast_dictMatchState),
              Some(ZSTD_compressBlock_greedy_dictMatchState),
              Some(ZSTD_compressBlock_lazy_dictMatchState),
              Some(ZSTD_compressBlock_lazy2_dictMatchState),
              Some(ZSTD_compressBlock_btlazy2_dictMatchState),
              Some(ZSTD_compressBlock_btopt_dictMatchState),
              Some(ZSTD_compressBlock_btultra_dictMatchState)]]
        };
    let mut selectedCompressor: ZSTD_blockCompressor = None;
    selectedCompressor =
        blockCompressor[dictMode as libc::c_int as
                            usize][strat as U32 as usize];
    return selectedCompressor;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_resetSeqStore(mut ssPtr: *mut seqStore_t)
 -> () {
    (*ssPtr).lit = (*ssPtr).litStart;
    (*ssPtr).sequences = (*ssPtr).sequencesStart;
    (*ssPtr).longLengthID = 0i32 as U32;
}
static mut ZSTD_blockHeaderSize: size_t = unsafe { 3i32 as size_t };
unsafe extern "C" fn ZSTD_compress_frameChunk(mut cctx: *mut ZSTD_CCtx,
                                              mut dst: *mut libc::c_void,
                                              mut dstCapacity: size_t,
                                              mut src: *const libc::c_void,
                                              mut srcSize: size_t,
                                              mut lastFrameChunk: U32)
 -> size_t {
    let mut cBlockHeader24_0: U32 = 0;
    let mut blockSize: size_t = (*cctx).blockSize;
    let mut remaining: size_t = srcSize;
    let mut ip: *const BYTE = src as *const BYTE;
    let ostart: *mut BYTE = dst as *mut BYTE;
    let mut op: *mut BYTE = ostart;
    let maxDist: U32 =
        (1i32 as U32) << (*cctx).appliedParams.cParams.windowLog;
    if 0 != (*cctx).appliedParams.fParams.checksumFlag && 0 != srcSize {
        ZSTD_XXH64_update(&mut (*cctx).xxhState as *mut XXH64_state_t, src,
                          srcSize);
    }
    while 0 != remaining {
        let ms: *mut ZSTD_matchState_t_0 =
            &mut (*cctx).blockState.matchState as *mut ZSTD_matchState_t_0;
        let lastBlock: U32 =
            lastFrameChunk &
                (blockSize >= remaining) as libc::c_int as libc::c_uint;
        if dstCapacity <
               ZSTD_blockHeaderSize.wrapping_add((1i32 + 1i32 + 1i32) as
                                                     libc::c_ulong) {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
        } else {
            if remaining < blockSize { blockSize = remaining }
            if 0 !=
                   ZSTD_window_needOverflowCorrection((*ms).window,
                                                      ip.offset(blockSize as
                                                                    isize) as
                                                          *const libc::c_void)
               {
                let cycleLog: U32 =
                    ZSTD_cycleLog((*cctx).appliedParams.cParams.chainLog,
                                  (*cctx).appliedParams.cParams.strategy);
                let correction: U32 =
                    ZSTD_window_correctOverflow(&mut (*ms).window as
                                                    *mut ZSTD_window_t,
                                                cycleLog, maxDist,
                                                ip as *const libc::c_void);
                ZSTD_reduceIndex(cctx, correction);
                if (*ms).nextToUpdate < correction {
                    (*ms).nextToUpdate = 0i32 as U32
                } else {
                    (*ms).nextToUpdate =
                        ((*ms).nextToUpdate as
                             libc::c_uint).wrapping_sub(correction) as U32 as
                            U32
                }
                (*ms).loadedDictEnd = 0i32 as U32;
                (*ms).dictMatchState = 0 as *const ZSTD_matchState_t_0
            }
            ZSTD_window_enforceMaxDist(&mut (*ms).window as
                                           *mut ZSTD_window_t,
                                       ip.offset(blockSize as isize) as
                                           *const libc::c_void, maxDist,
                                       &mut (*ms).loadedDictEnd as *mut U32,
                                       &mut (*ms).dictMatchState as
                                           *mut *const ZSTD_matchState_t_0);
            if (*ms).nextToUpdate < (*ms).window.lowLimit {
                (*ms).nextToUpdate = (*ms).window.lowLimit
            }
            let mut cSize: size_t =
                ZSTD_compressBlock_internal(cctx,
                                            op.offset(ZSTD_blockHeaderSize as
                                                          isize) as
                                                *mut libc::c_void,
                                            dstCapacity.wrapping_sub(ZSTD_blockHeaderSize),
                                            ip as *const libc::c_void,
                                            blockSize);
            if 0 != ZSTD_isError(cSize) {
                return cSize
            } else {
                if cSize == 0i32 as libc::c_ulong {
                    let cBlockHeader24: U32 =
                        lastBlock.wrapping_add((bt_raw as libc::c_int as U32)
                                                   <<
                                                   1i32).wrapping_add((blockSize
                                                                           <<
                                                                           3i32)
                                                                          as
                                                                          U32);
                    if blockSize.wrapping_add(ZSTD_blockHeaderSize) >
                           dstCapacity {
                        return -(ZSTD_error_dstSize_tooSmall as libc::c_int)
                                   as size_t
                    } else {
                        MEM_writeLE32(op as *mut libc::c_void,
                                      cBlockHeader24);
                        memcpy(op.offset(ZSTD_blockHeaderSize as isize) as
                                   *mut libc::c_void,
                               ip as *const libc::c_void, blockSize);
                        cSize = ZSTD_blockHeaderSize.wrapping_add(blockSize)
                    }
                } else {
                    cBlockHeader24_0 =
                        lastBlock.wrapping_add((bt_compressed as libc::c_int
                                                    as U32) <<
                                                   1i32).wrapping_add((cSize
                                                                           <<
                                                                           3i32)
                                                                          as
                                                                          U32);
                    MEM_writeLE24(op as *mut libc::c_void, cBlockHeader24_0);
                    cSize =
                        (cSize as
                             libc::c_ulong).wrapping_add(ZSTD_blockHeaderSize)
                            as size_t as size_t
                }
                ip = ip.offset(blockSize as isize);
                remaining =
                    (remaining as libc::c_ulong).wrapping_sub(blockSize) as
                        size_t as size_t;
                op = op.offset(cSize as isize);
                dstCapacity =
                    (dstCapacity as libc::c_ulong).wrapping_sub(cSize) as
                        size_t as size_t
            }
        }
    }
    if 0 != lastFrameChunk && op > ostart { (*cctx).stage = ZSTDcs_ending }
    return ostart.offset_to(op).expect("bad offset_to") as libc::c_long as
               size_t;
}
unsafe extern "C" fn ZSTD_window_enforceMaxDist(mut window:
                                                    *mut ZSTD_window_t,
                                                mut srcEnd:
                                                    *const libc::c_void,
                                                mut maxDist: U32,
                                                mut loadedDictEndPtr:
                                                    *mut U32,
                                                mut dictMatchStatePtr:
                                                    *mut *const ZSTD_matchState_t_0)
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
            *dictMatchStatePtr = 0 as *const ZSTD_matchState_t_0
        }
    };
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
unsafe extern "C" fn ZSTD_reduceIndex(mut zc: *mut ZSTD_CCtx,
                                      reducerValue: U32) -> () {
    let ms: *mut ZSTD_matchState_t_0 =
        &mut (*zc).blockState.matchState as *mut ZSTD_matchState_t_0;
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
                                      reducerValue: U32) -> () {
    ZSTD_reduceTable_internal(table, size, reducerValue, 0i32);
}
unsafe extern "C" fn ZSTD_reduceTable_internal(table: *mut U32, size: U32,
                                               reducerValue: U32,
                                               preserveMark: libc::c_int)
 -> () {
    let mut adder: U32 = 0;
    let nbRows: libc::c_int = size as libc::c_int / 16i32;
    let mut cellNb: libc::c_int = 0i32;
    let mut rowNb: libc::c_int = 0;
    rowNb = 0i32;
    while rowNb < nbRows {
        let mut column: libc::c_int = 0;
        column = 0i32;
        while column < 16i32 {
            if 0 != preserveMark {
                adder =
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
                                              reducerValue: U32) -> () {
    ZSTD_reduceTable_internal(table, size, reducerValue, 1i32);
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
unsafe extern "C" fn ZSTD_writeFrameHeader(mut dst: *mut libc::c_void,
                                           mut dstCapacity: size_t,
                                           mut params: ZSTD_CCtx_params,
                                           mut pledgedSrcSize: U64,
                                           mut dictID: U32) -> size_t {
    let op: *mut BYTE = dst as *mut BYTE;
    let dictIDSizeCodeLength: U32 =
        ((dictID > 0i32 as libc::c_uint) as libc::c_int +
             (dictID >= 256i32 as libc::c_uint) as libc::c_int +
             (dictID >= 65536i32 as libc::c_uint) as libc::c_int) as U32;
    let dictIDSizeCode: U32 =
        if 0 != params.fParams.noDictIDFlag {
            0i32 as libc::c_uint
        } else { dictIDSizeCodeLength };
    let checksumFlag: U32 =
        (params.fParams.checksumFlag > 0i32 as libc::c_uint) as libc::c_int as
            U32;
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
                 (pledgedSrcSize >= 4294967295u32 as libc::c_ulong) as
                     libc::c_int
         } else { 0i32 }) as U32;
    let frameHeaderDecriptionByte: BYTE =
        dictIDSizeCode.wrapping_add(checksumFlag <<
                                        2i32).wrapping_add(singleSegment <<
                                                               5i32).wrapping_add(fcsCode
                                                                                      <<
                                                                                      6i32)
            as BYTE;
    let mut pos: size_t = 0i32 as size_t;
    if dstCapacity < ZSTD_frameHeaderSize_max {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else {
        if params.format as libc::c_uint ==
               ZSTD_f_zstd1 as libc::c_int as libc::c_uint {
            MEM_writeLE32(dst, 4247762216u32);
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
                    (pos as libc::c_ulong).wrapping_add(2i32 as libc::c_ulong)
                        as size_t as size_t
            }
            3 => {
                MEM_writeLE32(op.offset(pos as isize) as *mut libc::c_void,
                              dictID);
                pos =
                    (pos as libc::c_ulong).wrapping_add(4i32 as libc::c_ulong)
                        as size_t as size_t
            }
            0 | _ => { }
        }
        match fcsCode {
            1 => {
                MEM_writeLE16(op.offset(pos as isize) as *mut libc::c_void,
                              pledgedSrcSize.wrapping_sub(256i32 as
                                                              libc::c_ulong)
                                  as U16);
                pos =
                    (pos as libc::c_ulong).wrapping_add(2i32 as libc::c_ulong)
                        as size_t as size_t
            }
            2 => {
                MEM_writeLE32(op.offset(pos as isize) as *mut libc::c_void,
                              pledgedSrcSize as U32);
                pos =
                    (pos as libc::c_ulong).wrapping_add(4i32 as libc::c_ulong)
                        as size_t as size_t
            }
            3 => {
                MEM_writeLE64(op.offset(pos as isize) as *mut libc::c_void,
                              pledgedSrcSize);
                pos =
                    (pos as libc::c_ulong).wrapping_add(8i32 as libc::c_ulong)
                        as size_t as size_t
            }
            0 | _ => {
                if 0 != singleSegment {
                    let fresh7 = pos;
                    pos = pos.wrapping_add(1);
                    *op.offset(fresh7 as isize) = pledgedSrcSize as BYTE
                }
            }
        }
        return pos
    };
}
static mut ZSTD_frameHeaderSize_max: size_t = unsafe { 18i32 as size_t };
unsafe extern "C" fn ZSTD_writeEpilogue(mut cctx: *mut ZSTD_CCtx,
                                        mut dst: *mut libc::c_void,
                                        mut dstCapacity: size_t) -> size_t {
    let ostart: *mut BYTE = dst as *mut BYTE;
    let mut op: *mut BYTE = ostart;
    let mut fhSize: size_t = 0i32 as size_t;
    if (*cctx).stage as libc::c_uint ==
           ZSTDcs_created as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    } else {
        if (*cctx).stage as libc::c_uint ==
               ZSTDcs_init as libc::c_int as libc::c_uint {
            fhSize =
                ZSTD_writeFrameHeader(dst, dstCapacity, (*cctx).appliedParams,
                                      0i32 as U64, 0i32 as U32);
            if 0 != ZSTD_isError(fhSize) {
                return fhSize
            } else {
                dstCapacity =
                    (dstCapacity as libc::c_ulong).wrapping_sub(fhSize) as
                        size_t as size_t;
                op = op.offset(fhSize as isize);
                (*cctx).stage = ZSTDcs_ongoing
            }
        }
        if (*cctx).stage as libc::c_uint !=
               ZSTDcs_ending as libc::c_int as libc::c_uint {
            let cBlockHeader24: U32 =
                (1i32 as
                     libc::c_uint).wrapping_add((bt_raw as libc::c_int as U32)
                                                    <<
                                                    1i32).wrapping_add(0i32 as
                                                                           libc::c_uint);
            if dstCapacity < 4i32 as libc::c_ulong {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
            } else {
                MEM_writeLE32(op as *mut libc::c_void, cBlockHeader24);
                op = op.offset(ZSTD_blockHeaderSize as isize);
                dstCapacity =
                    (dstCapacity as
                         libc::c_ulong).wrapping_sub(ZSTD_blockHeaderSize) as
                        size_t as size_t
            }
        }
        if 0 != (*cctx).appliedParams.fParams.checksumFlag {
            let checksum: U32 =
                ZSTD_XXH64_digest(&mut (*cctx).xxhState as *mut XXH64_state_t)
                    as U32;
            if dstCapacity < 4i32 as libc::c_ulong {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
            } else {
                MEM_writeLE32(op as *mut libc::c_void, checksum);
                op = op.offset(4isize)
            }
        }
        (*cctx).stage = ZSTDcs_created;
        return ostart.offset_to(op).expect("bad offset_to") as libc::c_long as
                   size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBegin_internal(mut cctx: *mut ZSTD_CCtx,
                                                     mut dict:
                                                         *const libc::c_void,
                                                     mut dictSize: size_t,
                                                     mut dictContentType:
                                                         ZSTD_dictContentType_e,
                                                     mut dtlm:
                                                         ZSTD_dictTableLoadMethod_e,
                                                     mut cdict:
                                                         *const ZSTD_CDict,
                                                     mut params:
                                                         ZSTD_CCtx_params,
                                                     mut pledgedSrcSize: U64,
                                                     mut zbuff:
                                                         ZSTD_buffered_policy_e)
 -> size_t {
    if !cdict.is_null() && (*cdict).dictContentSize > 0i32 as libc::c_ulong {
        return ZSTD_resetCCtx_usingCDict(cctx, cdict, params, pledgedSrcSize,
                                         zbuff)
    } else {
        let errcod: size_t =
            ZSTD_resetCCtx_internal(cctx, params, pledgedSrcSize,
                                    ZSTDcrp_continue, zbuff);
        if 0 != ERR_isError(errcod) {
            return errcod
        } else {
            let dictID: size_t =
                ZSTD_compress_insertDictionary((*cctx).blockState.prevCBlock,
                                               &mut (*cctx).blockState.matchState
                                                   as
                                                   *mut ZSTD_matchState_t_0,
                                               &mut params as
                                                   *mut ZSTD_CCtx_params,
                                               dict, dictSize,
                                               dictContentType, dtlm,
                                               (*cctx).entropyWorkspace as
                                                   *mut libc::c_void);
            if 0 != ZSTD_isError(dictID) {
                return dictID
            } else { (*cctx).dictID = dictID as U32; return 0i32 as size_t }
        }
    };
}
unsafe extern "C" fn ZSTD_compress_insertDictionary(mut bs:
                                                        *mut ZSTD_compressedBlockState_t,
                                                    mut ms:
                                                        *mut ZSTD_matchState_t_0,
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
    } else {
        ZSTD_reset_compressedBlockState(bs);
        if dictContentType as libc::c_uint ==
               ZSTD_dct_rawContent as libc::c_int as libc::c_uint {
            return ZSTD_loadDictionaryContent(ms, params, dict, dictSize,
                                              dtlm)
        } else {
            if MEM_readLE32(dict) != 3962610743u32 {
                if dictContentType as libc::c_uint ==
                       ZSTD_dct_auto as libc::c_int as libc::c_uint {
                    return ZSTD_loadDictionaryContent(ms, params, dict,
                                                      dictSize, dtlm)
                } else if dictContentType as libc::c_uint ==
                              ZSTD_dct_fullDict as libc::c_int as libc::c_uint
                 {
                    return -(ZSTD_error_dictionary_wrong as libc::c_int) as
                               size_t
                }
            }
            return ZSTD_loadZstdDictionary(bs, ms, params, dict, dictSize,
                                           dtlm, workspace)
        }
    };
}
unsafe extern "C" fn ZSTD_loadZstdDictionary(mut bs:
                                                 *mut ZSTD_compressedBlockState_t,
                                             mut ms: *mut ZSTD_matchState_t_0,
                                             mut params:
                                                 *const ZSTD_CCtx_params,
                                             mut dict: *const libc::c_void,
                                             mut dictSize: size_t,
                                             mut dtlm:
                                                 ZSTD_dictTableLoadMethod_e,
                                             mut workspace: *mut libc::c_void)
 -> size_t {
    let mut maxOffset: U32 = 0;
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
                       &mut maxSymbolValue as *mut libc::c_uint,
                       dictPtr as *const libc::c_void,
                       dictPtr.offset_to(dictEnd).expect("bad offset_to") as
                           libc::c_long as size_t);
    if 0 != HUF_isError(hufHeaderSize) {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    } else if maxSymbolValue < 255i32 as libc::c_uint {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    } else {
        dictPtr = dictPtr.offset(hufHeaderSize as isize);
        let mut offcodeLog: libc::c_uint = 0;
        let offcodeHeaderSize: size_t =
            FSE_readNCount(offcodeNCount.as_mut_ptr(),
                           &mut offcodeMaxValue as *mut libc::c_uint,
                           &mut offcodeLog as *mut libc::c_uint,
                           dictPtr as *const libc::c_void,
                           dictPtr.offset_to(dictEnd).expect("bad offset_to")
                               as libc::c_long as size_t);
        if 0 != FSE_isError(offcodeHeaderSize) {
            return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
        } else if offcodeLog > 8i32 as libc::c_uint {
            return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
        } else {
            let errcod: size_t =
                FSE_buildCTable_wksp((*bs).entropy.fse.offcodeCTable.as_mut_ptr(),
                                     offcodeNCount.as_mut_ptr(),
                                     31i32 as libc::c_uint, offcodeLog,
                                     workspace, (6i32 << 10i32) as size_t);
            if 0 != ERR_isError(errcod) {
                return -(ZSTD_error_dictionary_corrupted as libc::c_int) as
                           size_t
            } else {
                dictPtr = dictPtr.offset(offcodeHeaderSize as isize);
                let mut matchlengthNCount: [libc::c_short; 53] = [0; 53];
                let mut matchlengthMaxValue: libc::c_uint =
                    52i32 as libc::c_uint;
                let mut matchlengthLog: libc::c_uint = 0;
                let matchlengthHeaderSize: size_t =
                    FSE_readNCount(matchlengthNCount.as_mut_ptr(),
                                   &mut matchlengthMaxValue as
                                       *mut libc::c_uint,
                                   &mut matchlengthLog as *mut libc::c_uint,
                                   dictPtr as *const libc::c_void,
                                   dictPtr.offset_to(dictEnd).expect("bad offset_to")
                                       as libc::c_long as size_t);
                if 0 != FSE_isError(matchlengthHeaderSize) {
                    return -(ZSTD_error_dictionary_corrupted as libc::c_int)
                               as size_t
                } else if matchlengthLog > 9i32 as libc::c_uint {
                    return -(ZSTD_error_dictionary_corrupted as libc::c_int)
                               as size_t
                } else {
                    let errcod_0: size_t =
                        ZSTD_checkDictNCount(matchlengthNCount.as_mut_ptr(),
                                             matchlengthMaxValue,
                                             52i32 as libc::c_uint);
                    if 0 != ERR_isError(errcod_0) {
                        return errcod_0
                    } else {
                        let errcod_1: size_t =
                            FSE_buildCTable_wksp((*bs).entropy.fse.matchlengthCTable.as_mut_ptr(),
                                                 matchlengthNCount.as_mut_ptr(),
                                                 matchlengthMaxValue,
                                                 matchlengthLog, workspace,
                                                 (6i32 << 10i32) as size_t);
                        if 0 != ERR_isError(errcod_1) {
                            return -(ZSTD_error_dictionary_corrupted as
                                         libc::c_int) as size_t
                        } else {
                            dictPtr =
                                dictPtr.offset(matchlengthHeaderSize as
                                                   isize);
                            let mut litlengthNCount: [libc::c_short; 36] =
                                [0; 36];
                            let mut litlengthMaxValue: libc::c_uint =
                                35i32 as libc::c_uint;
                            let mut litlengthLog: libc::c_uint = 0;
                            let litlengthHeaderSize: size_t =
                                FSE_readNCount(litlengthNCount.as_mut_ptr(),
                                               &mut litlengthMaxValue as
                                                   *mut libc::c_uint,
                                               &mut litlengthLog as
                                                   *mut libc::c_uint,
                                               dictPtr as *const libc::c_void,
                                               dictPtr.offset_to(dictEnd).expect("bad offset_to")
                                                   as libc::c_long as size_t);
                            if 0 != FSE_isError(litlengthHeaderSize) {
                                return -(ZSTD_error_dictionary_corrupted as
                                             libc::c_int) as size_t
                            } else if litlengthLog > 9i32 as libc::c_uint {
                                return -(ZSTD_error_dictionary_corrupted as
                                             libc::c_int) as size_t
                            } else {
                                let errcod_2: size_t =
                                    ZSTD_checkDictNCount(litlengthNCount.as_mut_ptr(),
                                                         litlengthMaxValue,
                                                         35i32 as
                                                             libc::c_uint);
                                if 0 != ERR_isError(errcod_2) {
                                    return errcod_2
                                } else {
                                    let errcod_3: size_t =
                                        FSE_buildCTable_wksp((*bs).entropy.fse.litlengthCTable.as_mut_ptr(),
                                                             litlengthNCount.as_mut_ptr(),
                                                             litlengthMaxValue,
                                                             litlengthLog,
                                                             workspace,
                                                             (6i32 << 10i32)
                                                                 as size_t);
                                    if 0 != ERR_isError(errcod_3) {
                                        return -(ZSTD_error_dictionary_corrupted
                                                     as libc::c_int) as size_t
                                    } else {
                                        dictPtr =
                                            dictPtr.offset(litlengthHeaderSize
                                                               as isize);
                                        if dictPtr.offset(12isize) > dictEnd {
                                            return -(ZSTD_error_dictionary_corrupted
                                                         as libc::c_int) as
                                                       size_t
                                        } else {
                                            (*bs).rep[0usize] =
                                                MEM_readLE32(dictPtr.offset(0isize)
                                                                 as
                                                                 *const libc::c_void);
                                            (*bs).rep[1usize] =
                                                MEM_readLE32(dictPtr.offset(4isize)
                                                                 as
                                                                 *const libc::c_void);
                                            (*bs).rep[2usize] =
                                                MEM_readLE32(dictPtr.offset(8isize)
                                                                 as
                                                                 *const libc::c_void);
                                            dictPtr = dictPtr.offset(12isize);
                                            let dictContentSize: size_t =
                                                dictPtr.offset_to(dictEnd).expect("bad offset_to")
                                                    as libc::c_long as size_t;
                                            let mut offcodeMax: U32 =
                                                31i32 as U32;
                                            if dictContentSize <=
                                                   (-1i32 as
                                                        U32).wrapping_sub((128i32
                                                                               *
                                                                               (1i32
                                                                                    <<
                                                                                    10i32))
                                                                              as
                                                                              libc::c_uint)
                                                       as libc::c_ulong {
                                                maxOffset =
                                                    (dictContentSize as
                                                         U32).wrapping_add((128i32
                                                                                *
                                                                                (1i32
                                                                                     <<
                                                                                     10i32))
                                                                               as
                                                                               libc::c_uint);
                                                offcodeMax =
                                                    ZSTD_highbit32(maxOffset)
                                            }
                                            let errcod_4: size_t =
                                                ZSTD_checkDictNCount(offcodeNCount.as_mut_ptr(),
                                                                     offcodeMaxValue,
                                                                     if offcodeMax
                                                                            <
                                                                            31i32
                                                                                as
                                                                                libc::c_uint
                                                                        {
                                                                         offcodeMax
                                                                     } else {
                                                                         31i32
                                                                             as
                                                                             libc::c_uint
                                                                     });
                                            if 0 != ERR_isError(errcod_4) {
                                                return errcod_4
                                            } else {
                                                let mut u: U32 = 0;
                                                u = 0i32 as U32;
                                                while u < 3i32 as libc::c_uint
                                                      {
                                                    if (*bs).rep[u as usize]
                                                           ==
                                                           0i32 as
                                                               libc::c_uint {
                                                        return -(ZSTD_error_dictionary_corrupted
                                                                     as
                                                                     libc::c_int)
                                                                   as size_t
                                                    } else if (*bs).rep[u as
                                                                            usize]
                                                                  as
                                                                  libc::c_ulong
                                                                  >
                                                                  dictContentSize
                                                     {
                                                        return -(ZSTD_error_dictionary_corrupted
                                                                     as
                                                                     libc::c_int)
                                                                   as size_t
                                                    } else {
                                                        u = u.wrapping_add(1)
                                                    }
                                                }
                                                (*bs).entropy.huf.repeatMode =
                                                    HUF_repeat_valid;
                                                (*bs).entropy.fse.offcode_repeatMode
                                                    = FSE_repeat_valid;
                                                (*bs).entropy.fse.matchlength_repeatMode
                                                    = FSE_repeat_valid;
                                                (*bs).entropy.fse.litlength_repeatMode
                                                    = FSE_repeat_valid;
                                                let errcod_5: size_t =
                                                    ZSTD_loadDictionaryContent(ms,
                                                                               params,
                                                                               dictPtr
                                                                                   as
                                                                                   *const libc::c_void,
                                                                               dictContentSize,
                                                                               dtlm);
                                                if 0 != ERR_isError(errcod_5)
                                                   {
                                                    return errcod_5
                                                } else { return dictID }
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
    };
}
unsafe extern "C" fn ZSTD_loadDictionaryContent(mut ms:
                                                    *mut ZSTD_matchState_t_0,
                                                mut params:
                                                    *const ZSTD_CCtx_params,
                                                mut src: *const libc::c_void,
                                                mut srcSize: size_t,
                                                mut dtlm:
                                                    ZSTD_dictTableLoadMethod_e)
 -> size_t {
    let ip: *const BYTE = src as *const BYTE;
    let iend: *const BYTE = ip.offset(srcSize as isize);
    let mut cParams: *const ZSTD_compressionParameters =
        &(*params).cParams as *const ZSTD_compressionParameters;
    ZSTD_window_update(&mut (*ms).window as *mut ZSTD_window_t, src, srcSize);
    (*ms).loadedDictEnd =
        if 0 != (*params).forceWindow {
            0i32 as libc::c_uint
        } else {
            (*ms).window.base.offset_to(iend).expect("bad offset_to") as
                libc::c_long as U32
        };
    if srcSize <= 8i32 as libc::c_ulong {
        return 0i32 as size_t
    } else {
        match (*params).cParams.strategy as libc::c_uint {
            1 => {
                ZSTD_fillHashTable(ms, cParams, iend as *const libc::c_void,
                                   dtlm);
            }
            2 => {
                ZSTD_fillDoubleHashTable(ms, cParams,
                                         iend as *const libc::c_void, dtlm);
            }
            3 | 4 | 5 => {
                if srcSize >= 8i32 as libc::c_ulong {
                    ZSTD_insertAndFindFirstIndex(ms, cParams,
                                                 iend.offset(-8isize));
                }
            }
            6 | 7 | 8 => {
                if srcSize >= 8i32 as libc::c_ulong {
                    ZSTD_updateTree(ms, cParams, iend.offset(-8isize), iend);
                }
            }
            _ => { }
        }
        (*ms).nextToUpdate =
            (*ms).window.base.offset_to(iend).expect("bad offset_to") as
                libc::c_long as U32;
        return 0i32 as size_t
    };
}
unsafe extern "C" fn ZSTD_checkDictNCount(mut normalizedCounter:
                                              *mut libc::c_short,
                                          mut dictMaxSymbolValue:
                                              libc::c_uint,
                                          mut maxSymbolValue: libc::c_uint)
 -> size_t {
    let mut s: U32 = 0;
    if dictMaxSymbolValue < maxSymbolValue {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    } else {
        s = 0i32 as U32;
        while s <= maxSymbolValue {
            if *normalizedCounter.offset(s as isize) as libc::c_int == 0i32 {
                return -(ZSTD_error_dictionary_corrupted as libc::c_int) as
                           size_t
            } else { s = s.wrapping_add(1) }
        }
        return 0i32 as size_t
    };
}
unsafe extern "C" fn ZSTD_reset_compressedBlockState(mut bs:
                                                         *mut ZSTD_compressedBlockState_t)
 -> () {
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
static mut repStartValue: [U32; 3] =
    unsafe { [1i32 as U32, 4i32 as U32, 8i32 as U32] };
unsafe extern "C" fn ZSTD_resetCCtx_internal(mut zc: *mut ZSTD_CCtx,
                                             mut params: ZSTD_CCtx_params,
                                             mut pledgedSrcSize: U64,
                                             crp: ZSTD_compResetPolicy_e,
                                             zbuff: ZSTD_buffered_policy_e)
 -> size_t {
    let mut ldmHSize: size_t = 0;
    let mut ldmBucketSize: size_t = 0;
    if crp as libc::c_uint == ZSTDcrp_continue as libc::c_int as libc::c_uint
       {
        if 0 !=
               ZSTD_equivalentParams((*zc).appliedParams, params,
                                     (*zc).inBuffSize, (*zc).blockSize, zbuff,
                                     pledgedSrcSize) {
            (*zc).workSpaceOversizedDuration +=
                ((*zc).workSpaceOversizedDuration > 0i32) as libc::c_int;
            if (*zc).workSpaceOversizedDuration <= 128i32 {
                return ZSTD_continueCCtx(zc, params, pledgedSrcSize)
            }
        }
    }
    if 0 != params.ldmParams.enableLdm {
        ZSTD_ldm_adjustParameters(&mut params.ldmParams as *mut ldmParams_t,
                                  &mut params.cParams as
                                      *mut ZSTD_compressionParameters);
        (*zc).ldmState.hashPower =
            ZSTD_ldm_getHashPower(params.ldmParams.minMatchLength)
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
        (if params.cParams.searchLength == 3i32 as libc::c_uint {
             3i32
         } else { 4i32 }) as U32;
    let maxNbSeq: size_t = blockSize.wrapping_div(divider as libc::c_ulong);
    let tokenSpace: size_t =
        blockSize.wrapping_add((11i32 as
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
        ZSTD_sizeof_matchState(&mut params.cParams as
                                   *mut ZSTD_compressionParameters,
                               1i32 as U32);
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
        } else {
            (*zc).workSpaceSize = 0i32 as size_t;
            ZSTD_free((*zc).workSpace, (*zc).customMem);
            (*zc).workSpace = ZSTD_malloc(neededSpace, (*zc).customMem);
            if (*zc).workSpace.is_null() {
                return -(ZSTD_error_memory_allocation as libc::c_int) as
                           size_t
            } else {
                (*zc).workSpaceSize = neededSpace;
                (*zc).workSpaceOversizedDuration = 0i32;
                ptr = (*zc).workSpace;
                (*zc).blockState.prevCBlock =
                    (*zc).workSpace as *mut ZSTD_compressedBlockState_t;
                (*zc).blockState.nextCBlock =
                    (*zc).blockState.prevCBlock.offset(1isize);
                ptr =
                    (*zc).blockState.nextCBlock.offset(1isize) as
                        *mut libc::c_void;
                (*zc).entropyWorkspace = ptr as *mut U32
            }
        }
    }
    (*zc).appliedParams = params;
    (*zc).pledgedSrcSizePlusOne =
        pledgedSrcSize.wrapping_add(1i32 as libc::c_ulong) as
            libc::c_ulonglong;
    (*zc).consumedSrcSize = 0i32 as libc::c_ulonglong;
    (*zc).producedCSize = 0i32 as libc::c_ulonglong;
    if pledgedSrcSize as libc::c_ulonglong ==
           0u64.wrapping_sub(1i32 as libc::c_ulonglong) {
        (*zc).appliedParams.fParams.contentSizeFlag = 0i32 as libc::c_uint
    }
    (*zc).blockSize = blockSize;
    ZSTD_XXH64_reset(&mut (*zc).xxhState as *mut XXH64_state_t,
                     0i32 as libc::c_ulonglong);
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
        ldmHSize = (1i32 as size_t) << params.ldmParams.hashLog;
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
        ZSTD_reset_matchState(&mut (*zc).blockState.matchState as
                                  *mut ZSTD_matchState_t_0, ptr,
                              &mut params.cParams as
                                  *mut ZSTD_compressionParameters, crp,
                              1i32 as U32);
    (*zc).seqStore.sequencesStart = ptr as *mut seqDef;
    ptr =
        (*zc).seqStore.sequencesStart.offset(maxNbSeq as isize) as
            *mut libc::c_void;
    (*zc).seqStore.llCode = ptr as *mut BYTE;
    (*zc).seqStore.mlCode = (*zc).seqStore.llCode.offset(maxNbSeq as isize);
    (*zc).seqStore.ofCode = (*zc).seqStore.mlCode.offset(maxNbSeq as isize);
    (*zc).seqStore.litStart = (*zc).seqStore.ofCode.offset(maxNbSeq as isize);
    ptr =
        (*zc).seqStore.litStart.offset(blockSize as isize) as
            *mut libc::c_void;
    if 0 != params.ldmParams.enableLdm {
        ldmBucketSize =
            (1i32 as size_t) <<
                params.ldmParams.hashLog.wrapping_sub(params.ldmParams.bucketSizeLog);
        memset(ptr, 0i32, ldmBucketSize);
        (*zc).ldmState.bucketOffsets = ptr as *mut BYTE;
        ptr =
            (*zc).ldmState.bucketOffsets.offset(ldmBucketSize as isize) as
                *mut libc::c_void;
        ZSTD_window_clear(&mut (*zc).ldmState.window as *mut ZSTD_window_t);
    }
    ZSTD_referenceExternalSequences(zc, 0 as *mut rawSeq, 0i32 as size_t);
    (*zc).inBuffSize = buffInSize;
    (*zc).inBuff = ptr as *mut libc::c_char;
    (*zc).outBuffSize = buffOutSize;
    (*zc).outBuff = (*zc).inBuff.offset(buffInSize as isize);
    return 0i32 as size_t;
}
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
#[no_mangle]
pub unsafe extern "C" fn ZSTD_referenceExternalSequences(mut cctx:
                                                             *mut ZSTD_CCtx,
                                                         mut seq: *mut rawSeq,
                                                         mut nbSeq: size_t)
 -> size_t {
    if (*cctx).stage as libc::c_uint !=
           ZSTDcs_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    } else if 0 != (*cctx).appliedParams.ldmParams.enableLdm {
        return -(ZSTD_error_parameter_unsupported as libc::c_int) as size_t
    } else {
        (*cctx).externSeqStore.seq = seq;
        (*cctx).externSeqStore.size = nbSeq;
        (*cctx).externSeqStore.capacity = nbSeq;
        (*cctx).externSeqStore.pos = 0i32 as size_t;
        return 0i32 as size_t
    };
}
unsafe extern "C" fn ZSTD_window_clear(mut window: *mut ZSTD_window_t) -> () {
    let endT: size_t =
        (*window).base.offset_to((*window).nextSrc).expect("bad offset_to") as
            libc::c_long as size_t;
    let end: U32 = endT as U32;
    (*window).lowLimit = end;
    (*window).dictLimit = end;
}
unsafe extern "C" fn ZSTD_reset_matchState(mut ms: *mut ZSTD_matchState_t_0,
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
        if 0 != forCCtx && (*cParams).searchLength == 3i32 as libc::c_uint {
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
    ZSTD_invalidateMatchState(ms);
    if 0 != forCCtx &&
           0 !=
               ((*cParams).strategy as libc::c_uint ==
                    ZSTD_btopt as libc::c_int as libc::c_uint) as libc::c_int
                   |
                   ((*cParams).strategy as libc::c_uint ==
                        ZSTD_btultra as libc::c_int as libc::c_uint) as
                       libc::c_int {
        (*ms).opt.litFreq = ptr as *mut U32;
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
    return ptr;
}
unsafe extern "C" fn ZSTD_invalidateMatchState(mut ms:
                                                   *mut ZSTD_matchState_t_0)
 -> () {
    ZSTD_window_clear(&mut (*ms).window as *mut ZSTD_window_t);
    (*ms).nextToUpdate =
        (*ms).window.dictLimit.wrapping_add(1i32 as libc::c_uint);
    (*ms).nextToUpdate3 =
        (*ms).window.dictLimit.wrapping_add(1i32 as libc::c_uint);
    (*ms).loadedDictEnd = 0i32 as U32;
    (*ms).opt.litLengthSum = 0i32 as U32;
    (*ms).dictMatchState = 0 as *const ZSTD_matchState_t_0;
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
        if 0 != forCCtx && (*cParams).searchLength == 3i32 as libc::c_uint {
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
               ((*cParams).strategy as libc::c_uint ==
                    ZSTD_btopt as libc::c_int as libc::c_uint ||
                    (*cParams).strategy as libc::c_uint ==
                        ZSTD_btultra as libc::c_int as libc::c_uint) {
            optPotentialSpace
        } else { 0i32 as libc::c_ulong };
    return tableSpace.wrapping_add(optSpace);
}
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
    (*cctx).pledgedSrcSizePlusOne =
        pledgedSrcSize.wrapping_add(1i32 as libc::c_ulong) as
            libc::c_ulonglong;
    (*cctx).consumedSrcSize = 0i32 as libc::c_ulonglong;
    (*cctx).producedCSize = 0i32 as libc::c_ulonglong;
    if pledgedSrcSize as libc::c_ulonglong ==
           0u64.wrapping_sub(1i32 as libc::c_ulonglong) {
        (*cctx).appliedParams.fParams.contentSizeFlag = 0i32 as libc::c_uint
    }
    (*cctx).stage = ZSTDcs_init;
    (*cctx).dictID = 0i32 as U32;
    if 0 != params.ldmParams.enableLdm {
        ZSTD_window_clear(&mut (*cctx).ldmState.window as *mut ZSTD_window_t);
    }
    ZSTD_referenceExternalSequences(cctx, 0 as *mut rawSeq, 0i32 as size_t);
    ZSTD_invalidateMatchState(&mut (*cctx).blockState.matchState as
                                  *mut ZSTD_matchState_t_0);
    ZSTD_reset_compressedBlockState((*cctx).blockState.prevCBlock);
    ZSTD_XXH64_reset(&mut (*cctx).xxhState as *mut XXH64_state_t,
                     0i32 as libc::c_ulonglong);
    return 0i32 as size_t;
}
unsafe extern "C" fn ZSTD_equivalentParams(mut params1: ZSTD_CCtx_params,
                                           mut params2: ZSTD_CCtx_params,
                                           mut buffSize1: size_t,
                                           mut blockSize1: size_t,
                                           mut buffPol2:
                                               ZSTD_buffered_policy_e,
                                           mut pledgedSrcSize: U64) -> U32 {
    return (0 != ZSTD_equivalentCParams(params1.cParams, params2.cParams) &&
                0 !=
                    ZSTD_equivalentLdmParams(params1.ldmParams,
                                             params2.ldmParams) &&
                0 !=
                    ZSTD_sufficientBuff(buffSize1, blockSize1, buffPol2,
                                        params2.cParams, pledgedSrcSize)) as
               libc::c_int as U32;
}
unsafe extern "C" fn ZSTD_sufficientBuff(mut bufferSize1: size_t,
                                         mut blockSize1: size_t,
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
    let neededBufferSize2: size_t =
        if buffPol2 as libc::c_uint ==
               ZSTDb_buffered as libc::c_int as libc::c_uint {
            windowSize2.wrapping_add(blockSize2)
        } else { 0i32 as libc::c_ulong };
    return ((blockSize2 <= blockSize1) as libc::c_int &
                (neededBufferSize2 <= bufferSize1) as libc::c_int) as U32;
}
unsafe extern "C" fn ZSTD_equivalentLdmParams(mut ldmParams1: ldmParams_t,
                                              mut ldmParams2: ldmParams_t)
 -> U32 {
    return (0 == ldmParams1.enableLdm && 0 == ldmParams2.enableLdm ||
                ldmParams1.enableLdm == ldmParams2.enableLdm &&
                    ldmParams1.hashLog == ldmParams2.hashLog &&
                    ldmParams1.bucketSizeLog == ldmParams2.bucketSizeLog &&
                    ldmParams1.minMatchLength == ldmParams2.minMatchLength &&
                    ldmParams1.hashEveryLog == ldmParams2.hashEveryLog) as
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
                ((cParams1.searchLength == 3i32 as libc::c_uint) as
                     libc::c_int ==
                     (cParams2.searchLength == 3i32 as libc::c_uint) as
                         libc::c_int) as libc::c_int) as U32;
}
unsafe extern "C" fn ZSTD_resetCCtx_usingCDict(mut cctx: *mut ZSTD_CCtx,
                                               mut cdict: *const ZSTD_CDict,
                                               mut params: ZSTD_CCtx_params,
                                               mut pledgedSrcSize: U64,
                                               mut zbuff:
                                                   ZSTD_buffered_policy_e)
 -> size_t {
    let attachDictSizeCutoffs: [U64; 9] =
        [(8i32 * (1i32 << 10i32)) as U64, (8i32 * (1i32 << 10i32)) as U64,
         (16i32 * (1i32 << 10i32)) as U64, (32i32 * (1i32 << 10i32)) as U64,
         (32i32 * (1i32 << 10i32)) as U64, (32i32 * (1i32 << 10i32)) as U64,
         (32i32 * (1i32 << 10i32)) as U64, (32i32 * (1i32 << 10i32)) as U64,
         (8i32 * (1i32 << 10i32)) as U64];
    let attachDict: libc::c_int =
        ((pledgedSrcSize <=
              attachDictSizeCutoffs[(*cdict).cParams.strategy as usize] ||
              pledgedSrcSize as libc::c_ulonglong ==
                  0u64.wrapping_sub(1i32 as libc::c_ulonglong) ||
              params.attachDictPref as libc::c_int ==
                  ZSTD_dictForceAttach as libc::c_int) &&
             params.attachDictPref as libc::c_int !=
                 ZSTD_dictForceCopy as libc::c_int && 0 == params.forceWindow
             &&
             0 !=
                 ZSTD_equivalentCParams((*cctx).appliedParams.cParams,
                                        (*cdict).cParams)) as libc::c_int;
    let windowLog: libc::c_uint = params.cParams.windowLog;
    params.cParams = (*cdict).cParams;
    params.cParams.windowLog = windowLog;
    ZSTD_resetCCtx_internal(cctx, params, pledgedSrcSize,
                            (if 0 != attachDict {
                                 ZSTDcrp_continue as libc::c_int
                             } else { ZSTDcrp_noMemset as libc::c_int }) as
                                ZSTD_compResetPolicy_e, zbuff);
    if 0 != attachDict {
        let cdictLen: U32 =
            (*cdict).matchState.window.base.offset_to((*cdict).matchState.window.nextSrc).expect("bad offset_to")
                as libc::c_long as U32;
        if !(cdictLen == 0i32 as libc::c_uint) {
            (*cctx).blockState.matchState.dictMatchState =
                &(*cdict).matchState as *const ZSTD_matchState_t_0;
            if (*cctx).blockState.matchState.window.dictLimit < cdictLen {
                (*cctx).blockState.matchState.window.nextSrc =
                    (*cctx).blockState.matchState.window.base.offset(cdictLen
                                                                         as
                                                                         isize);
                ZSTD_window_clear(&mut (*cctx).blockState.matchState.window as
                                      *mut ZSTD_window_t);
            }
            (*cctx).blockState.matchState.loadedDictEnd =
                (*cctx).blockState.matchState.window.dictLimit
        }
    } else {
        let chainSize: size_t =
            if (*cdict).cParams.strategy as libc::c_uint ==
                   ZSTD_fast as libc::c_int as libc::c_uint {
                0i32 as libc::c_ulong
            } else { (1i32 as size_t) << (*cdict).cParams.chainLog };
        let hSize: size_t = (1i32 as size_t) << (*cdict).cParams.hashLog;
        let tableSpace: size_t =
            chainSize.wrapping_add(hSize).wrapping_mul(::std::mem::size_of::<U32>()
                                                           as libc::c_ulong);
        memcpy((*cctx).blockState.matchState.hashTable as *mut libc::c_void,
               (*cdict).matchState.hashTable as *const libc::c_void,
               tableSpace);
        let h3Size: size_t =
            (1i32 as size_t) << (*cctx).blockState.matchState.hashLog3;
        memset((*cctx).blockState.matchState.hashTable3 as *mut libc::c_void,
               0i32,
               h3Size.wrapping_mul(::std::mem::size_of::<U32>() as
                                       libc::c_ulong));
        let mut srcMatchState: *const ZSTD_matchState_t_0 =
            &(*cdict).matchState as *const ZSTD_matchState_t_0;
        let mut dstMatchState: *mut ZSTD_matchState_t_0 =
            &mut (*cctx).blockState.matchState as *mut ZSTD_matchState_t_0;
        (*dstMatchState).window = (*srcMatchState).window;
        (*dstMatchState).nextToUpdate = (*srcMatchState).nextToUpdate;
        (*dstMatchState).nextToUpdate3 = (*srcMatchState).nextToUpdate3;
        (*dstMatchState).loadedDictEnd = (*srcMatchState).loadedDictEnd
    }
    (*cctx).dictID = (*cdict).dictID;
    memcpy((*cctx).blockState.prevCBlock as *mut libc::c_void,
           &(*cdict).cBlockState as *const ZSTD_compressedBlockState_t as
               *const libc::c_void,
           ::std::mem::size_of::<ZSTD_compressedBlockState_t>() as
               libc::c_ulong);
    return 0i32 as size_t;
}
static mut ZSTD_defaultCMem: ZSTD_customMem =
    unsafe {
        ZSTD_customMem{customAlloc: None,
                       customFree: None,
                       opaque: 0 as *const libc::c_void as *mut libc::c_void,}
    };
unsafe extern "C" fn ZSTD_initCCtx(mut cctx: *mut ZSTD_CCtx,
                                   mut memManager: ZSTD_customMem) -> () {
    memset(cctx as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZSTD_CCtx>() as libc::c_ulong);
    (*cctx).customMem = memManager;
    (*cctx).bmi2 = ZSTD_cpuid_bmi2(ZSTD_cpuid());
    let err: size_t = ZSTD_CCtx_resetParameters(cctx);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_resetParameters(mut cctx: *mut ZSTD_CCtx)
 -> size_t {
    if (*cctx).streamStage as libc::c_uint !=
           zcss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    } else {
        (*cctx).cdict = 0 as *const ZSTD_CDict;
        return ZSTD_CCtxParams_reset(&mut (*cctx).requestedParams as
                                         *mut ZSTD_CCtx_params)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtxParams_reset(mut params:
                                                   *mut ZSTD_CCtx_params)
 -> size_t {
    return ZSTD_CCtxParams_init(params, 3i32);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtxParams_init(mut cctxParams:
                                                  *mut ZSTD_CCtx_params,
                                              mut compressionLevel:
                                                  libc::c_int) -> size_t {
    if cctxParams.is_null() {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        memset(cctxParams as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<ZSTD_CCtx_params>() as libc::c_ulong);
        (*cctxParams).compressionLevel = compressionLevel;
        (*cctxParams).fParams.contentSizeFlag = 1i32 as libc::c_uint;
        return 0i32 as size_t
    };
}
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
    } else {
        let cctx: *mut ZSTD_CCtx =
            ZSTD_malloc(::std::mem::size_of::<ZSTD_CCtx>() as libc::c_ulong,
                        customMem) as *mut ZSTD_CCtx;
        if cctx.is_null() {
            return 0 as *mut ZSTD_CCtx
        } else { ZSTD_initCCtx(cctx, customMem); return cctx }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeCCtx(mut cctx: *mut ZSTD_CCtx) -> size_t {
    if cctx.is_null() {
        return 0i32 as size_t
    } else if 0 != (*cctx).staticSize {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    } else {
        ZSTD_freeCCtxContent(cctx);
        ZSTD_free(cctx as *mut libc::c_void, (*cctx).customMem);
        return 0i32 as size_t
    };
}
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
    } else {
        let cdict: *mut ZSTD_CDict =
            ZSTD_malloc(::std::mem::size_of::<ZSTD_CDict>() as libc::c_ulong,
                        customMem) as *mut ZSTD_CDict;
        let workspaceSize: size_t =
            ((6i32 << 10i32) as
                 libc::c_ulong).wrapping_add(ZSTD_sizeof_matchState(&mut cParams
                                                                        as
                                                                        *mut ZSTD_compressionParameters,
                                                                    0i32 as
                                                                        U32));
        let workspace: *mut libc::c_void =
            ZSTD_malloc(workspaceSize, customMem);
        if cdict.is_null() || workspace.is_null() {
            ZSTD_free(cdict as *mut libc::c_void, customMem);
            ZSTD_free(workspace, customMem);
            return 0 as *mut ZSTD_CDict
        } else {
            (*cdict).customMem = customMem;
            (*cdict).workspace = workspace;
            (*cdict).workspaceSize = workspaceSize;
            if 0 !=
                   ZSTD_isError(ZSTD_initCDict_internal(cdict, dictBuffer,
                                                        dictSize,
                                                        dictLoadMethod,
                                                        dictContentType,
                                                        cParams)) {
                ZSTD_freeCDict(cdict);
                return 0 as *mut ZSTD_CDict
            } else { return cdict }
        }
    };
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
    (*cdict).cParams = cParams;
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
        } else { memcpy(internalBuffer, dictBuffer, dictSize); }
    }
    (*cdict).dictContentSize = dictSize;
    ZSTD_reset_compressedBlockState(&mut (*cdict).cBlockState as
                                        *mut ZSTD_compressedBlockState_t);
    let end: *mut libc::c_void =
        ZSTD_reset_matchState(&mut (*cdict).matchState as
                                  *mut ZSTD_matchState_t_0,
                              ((*cdict).workspace as
                                   *mut U32).offset(((6i32 << 10i32) as
                                                         libc::c_ulong).wrapping_div(::std::mem::size_of::<U32>()
                                                                                         as
                                                                                         libc::c_ulong)
                                                        as isize) as
                                  *mut libc::c_void,
                              &mut cParams as *mut ZSTD_compressionParameters,
                              ZSTDcrp_continue, 0i32 as U32);
    let mut params: ZSTD_CCtx_params =
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
    memset(&mut params as *mut ZSTD_CCtx_params as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZSTD_CCtx_params>() as libc::c_ulong);
    params.compressionLevel = 3i32;
    params.fParams.contentSizeFlag = 1i32 as libc::c_uint;
    params.cParams = cParams;
    let dictID: size_t =
        ZSTD_compress_insertDictionary(&mut (*cdict).cBlockState as
                                           *mut ZSTD_compressedBlockState_t,
                                       &mut (*cdict).matchState as
                                           *mut ZSTD_matchState_t_0,
                                       &mut params as *mut ZSTD_CCtx_params,
                                       (*cdict).dictContent,
                                       (*cdict).dictContentSize,
                                       dictContentType, ZSTD_dtlm_full,
                                       (*cdict).workspace);
    if 0 != ZSTD_isError(dictID) {
        return dictID
    } else { (*cdict).dictID = dictID as U32; return 0i32 as size_t };
}
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
    let fParams: ZSTD_frameParameters =
        ZSTD_frameParameters{contentSizeFlag: 1i32 as libc::c_uint,
                             checksumFlag: 0i32 as libc::c_uint,
                             noDictIDFlag: 0i32 as libc::c_uint,};
    return ZSTD_compress_usingCDict_advanced(cctx, dst, dstCapacity, src,
                                             srcSize, cdict, fParams);
}
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
    if 0 != ERR_isError(errcod) {
        return errcod
    } else { return ZSTD_compressEnd(cctx, dst, dstCapacity, src, srcSize) };
}
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
    let mut limitedSrcSize: U32 = 0;
    let mut limitedSrcLog: U32 = 0;
    if cdict.is_null() {
        return -(ZSTD_error_dictionary_wrong as libc::c_int) as size_t
    } else {
        let mut params: ZSTD_CCtx_params = (*cctx).requestedParams;
        params.cParams = ZSTD_getCParamsFromCDict(cdict);
        if pledgedSrcSize != 0u64.wrapping_sub(1i32 as libc::c_ulonglong) {
            limitedSrcSize =
                (if pledgedSrcSize < (1u32 << 19i32) as libc::c_ulonglong {
                     pledgedSrcSize
                 } else { (1u32 << 19i32) as libc::c_ulonglong }) as U32;
            limitedSrcLog =
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
                                           ZSTDb_not_buffered)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getCParamsFromCDict(mut cdict:
                                                      *const ZSTD_CDict)
 -> ZSTD_compressionParameters {
    return (*cdict).cParams;
}
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
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream(mut zcs: *mut ZSTD_CStream,
                                          mut compressionLevel: libc::c_int)
 -> size_t {
    return ZSTD_initCStream_srcSize(zcs, compressionLevel,
                                    0u64.wrapping_sub(1i32 as
                                                          libc::c_ulonglong));
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream_srcSize(mut zcs: *mut ZSTD_CStream,
                                                  mut compressionLevel:
                                                      libc::c_int,
                                                  mut pss: libc::c_ulonglong)
 -> size_t {
    let pledgedSrcSize: U64 =
        (if pss == 0i32 as libc::c_ulonglong {
             0u64.wrapping_sub(1i32 as libc::c_ulonglong)
         } else { pss }) as U64;
    let params: ZSTD_parameters =
        ZSTD_getParams(compressionLevel, pledgedSrcSize as libc::c_ulonglong,
                       0i32 as size_t);
    let cctxParams: ZSTD_CCtx_params =
        ZSTD_assignParamsToCCtxParams((*zcs).requestedParams, params);
    return ZSTD_initCStream_internal(zcs, 0 as *const libc::c_void,
                                     0i32 as size_t, 0 as *const ZSTD_CDict,
                                     cctxParams,
                                     pledgedSrcSize as libc::c_ulonglong);
}
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
    if !dict.is_null() && dictSize >= 8i32 as libc::c_ulong {
        if 0 != (*zcs).staticSize {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        } else {
            ZSTD_freeCDict((*zcs).cdictLocal);
            (*zcs).cdictLocal =
                ZSTD_createCDict_advanced(dict, dictSize, ZSTD_dlm_byCopy,
                                          ZSTD_dct_auto, params.cParams,
                                          (*zcs).customMem);
            (*zcs).cdict = (*zcs).cdictLocal;
            if (*zcs).cdictLocal.is_null() {
                return -(ZSTD_error_memory_allocation as libc::c_int) as
                           size_t
            }
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
                                                params: ZSTD_CCtx_params,
                                                pledgedSrcSize:
                                                    libc::c_ulonglong)
 -> size_t {
    let errcod: size_t =
        ZSTD_compressBegin_internal(cctx, dict, dictSize, dictContentType,
                                    ZSTD_dtlm_fast, cdict, params,
                                    pledgedSrcSize as U64, ZSTDb_buffered);
    if 0 != ERR_isError(errcod) {
        return errcod
    } else {
        (*cctx).inToCompress = 0i32 as size_t;
        (*cctx).inBuffPos = 0i32 as size_t;
        (*cctx).inBuffTarget =
            (*cctx).blockSize.wrapping_add(((*cctx).blockSize as
                                                libc::c_ulonglong ==
                                                pledgedSrcSize) as libc::c_int
                                               as libc::c_ulong);
        (*cctx).outBuffFlushedSize = 0i32 as size_t;
        (*cctx).outBuffContentSize = (*cctx).outBuffFlushedSize;
        (*cctx).streamStage = zcss_load;
        (*cctx).frameEnded = 0i32 as U32;
        return 0i32 as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressStream(mut zcs: *mut ZSTD_CStream,
                                             mut output: *mut ZSTD_outBuffer,
                                             mut input: *mut ZSTD_inBuffer)
 -> size_t {
    if (*output).pos > (*output).size {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else if (*input).pos > (*input).size {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        return ZSTD_compressStream_generic(zcs, output, input,
                                           ZSTD_e_continue)
    };
}
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
        match (*zcs).streamStage as libc::c_uint {
            0 => {
                return -(ZSTD_error_init_missing as libc::c_int) as size_t
            }
            1 => {
                if flushMode as libc::c_uint ==
                       ZSTD_e_end as libc::c_int as libc::c_uint &&
                       op.offset_to(oend).expect("bad offset_to") as
                           libc::c_long as size_t >=
                           ZSTD_compressBound(ip.offset_to(iend).expect("bad offset_to")
                                                  as libc::c_long as size_t)
                       && (*zcs).inBuffPos == 0i32 as libc::c_ulong {
                    let cSize: size_t =
                        ZSTD_compressEnd(zcs, op as *mut libc::c_void,
                                         op.offset_to(oend).expect("bad offset_to")
                                             as libc::c_long as size_t,
                                         ip as *const libc::c_void,
                                         ip.offset_to(iend).expect("bad offset_to")
                                             as libc::c_long as size_t);
                    if 0 != ZSTD_isError(cSize) {
                        return cSize
                    } else {
                        ip = iend;
                        op = op.offset(cSize as isize);
                        (*zcs).frameEnded = 1i32 as U32;
                        ZSTD_CCtx_reset(zcs);
                        someMoreWork = 0i32 as U32;
                        continue ;
                    }
                } else {
                    let toLoad: size_t =
                        (*zcs).inBuffTarget.wrapping_sub((*zcs).inBuffPos);
                    let loaded: size_t =
                        ZSTD_limitCopy((*zcs).inBuff.offset((*zcs).inBuffPos
                                                                as isize) as
                                           *mut libc::c_void, toLoad,
                                       ip as *const libc::c_void,
                                       ip.offset_to(iend).expect("bad offset_to")
                                           as libc::c_long as size_t);
                    (*zcs).inBuffPos =
                        ((*zcs).inBuffPos as
                             libc::c_ulong).wrapping_add(loaded) as size_t as
                            size_t;
                    ip = ip.offset(loaded as isize);
                    if flushMode as libc::c_uint ==
                           ZSTD_e_continue as libc::c_int as libc::c_uint &&
                           (*zcs).inBuffPos < (*zcs).inBuffTarget {
                        someMoreWork = 0i32 as U32;
                        continue ;
                    } else if flushMode as libc::c_uint ==
                                  ZSTD_e_flush as libc::c_int as libc::c_uint
                                  && (*zcs).inBuffPos == (*zcs).inToCompress {
                        someMoreWork = 0i32 as U32;
                        continue ;
                    } else {
                        let mut cDst: *mut libc::c_void =
                            0 as *mut libc::c_void;
                        let mut cSize_0: size_t = 0;
                        let iSize: size_t =
                            (*zcs).inBuffPos.wrapping_sub((*zcs).inToCompress);
                        let mut oSize: size_t =
                            op.offset_to(oend).expect("bad offset_to") as
                                libc::c_long as size_t;
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
                        if 0 != ZSTD_isError(cSize_0) {
                            return cSize_0
                        } else {
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
                                op = op.offset(cSize_0 as isize);
                                if !(0 != (*zcs).frameEnded) { continue ; }
                                someMoreWork = 0i32 as U32;
                                ZSTD_CCtx_reset(zcs);
                                continue ;
                            } else {
                                (*zcs).outBuffContentSize = cSize_0;
                                (*zcs).outBuffFlushedSize = 0i32 as size_t;
                                (*zcs).streamStage = zcss_flush
                            }
                        }
                    }
                }
            }
            2 => { }
            _ => { continue ; }
        }
        let toFlush: size_t =
            (*zcs).outBuffContentSize.wrapping_sub((*zcs).outBuffFlushedSize);
        let flushed: size_t =
            ZSTD_limitCopy(op as *mut libc::c_void,
                           op.offset_to(oend).expect("bad offset_to") as
                               libc::c_long as size_t,
                           (*zcs).outBuff.offset((*zcs).outBuffFlushedSize as
                                                     isize) as
                               *const libc::c_void, toFlush);
        op = op.offset(flushed as isize);
        (*zcs).outBuffFlushedSize =
            ((*zcs).outBuffFlushedSize as libc::c_ulong).wrapping_add(flushed)
                as size_t as size_t;
        if toFlush != flushed {
            someMoreWork = 0i32 as U32
        } else {
            (*zcs).outBuffFlushedSize = 0i32 as size_t;
            (*zcs).outBuffContentSize = (*zcs).outBuffFlushedSize;
            if 0 != (*zcs).frameEnded {
                someMoreWork = 0i32 as U32;
                ZSTD_CCtx_reset(zcs);
            } else { (*zcs).streamStage = zcss_load }
        }
    }
    (*input).pos =
        istart.offset_to(ip).expect("bad offset_to") as libc::c_long as
            size_t;
    (*output).pos =
        ostart.offset_to(op).expect("bad offset_to") as libc::c_long as
            size_t;
    if 0 != (*zcs).frameEnded {
        return 0i32 as size_t
    } else {
        let mut hintInSize: size_t =
            (*zcs).inBuffTarget.wrapping_sub((*zcs).inBuffPos);
        if hintInSize == 0i32 as libc::c_ulong {
            hintInSize = (*zcs).blockSize
        }
        return hintInSize
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_reset(mut cctx: *mut ZSTD_CCtx) -> () {
    (*cctx).streamStage = zcss_init;
    (*cctx).pledgedSrcSizePlusOne = 0i32 as libc::c_ulonglong;
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
    if (*output).pos > (*output).size {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        let errcod: size_t =
            ZSTD_compressStream_generic(zcs, output,
                                        &mut input as *mut ZSTD_inBuffer,
                                        ZSTD_e_flush);
        if 0 != ERR_isError(errcod) {
            return errcod
        } else {
            return (*zcs).outBuffContentSize.wrapping_sub((*zcs).outBuffFlushedSize)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_endStream(mut zcs: *mut ZSTD_CStream,
                                        mut output: *mut ZSTD_outBuffer)
 -> size_t {
    let mut input: ZSTD_inBuffer =
        ZSTD_inBuffer_s{src: 0 as *const libc::c_void,
                        size: 0i32 as size_t,
                        pos: 0i32 as size_t,};
    if (*output).pos > (*output).size {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        let errcod: size_t =
            ZSTD_compressStream_generic(zcs, output,
                                        &mut input as *mut ZSTD_inBuffer,
                                        ZSTD_e_end);
        if 0 != ERR_isError(errcod) {
            return errcod
        } else {
            let lastBlockSize: size_t =
                (if 0 != (*zcs).frameEnded { 0i32 } else { 3i32 }) as size_t;
            let checksumSize: size_t =
                (if 0 != (*zcs).frameEnded {
                     0i32 as libc::c_uint
                 } else {
                     (*zcs).appliedParams.fParams.checksumFlag.wrapping_mul(4i32
                                                                                as
                                                                                libc::c_uint)
                 }) as size_t;
            let toFlush: size_t =
                (*zcs).outBuffContentSize.wrapping_sub((*zcs).outBuffFlushedSize).wrapping_add(lastBlockSize).wrapping_add(checksumSize);
            return toFlush
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CStreamInSize() -> size_t {
    return (1i32 << 17i32) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CStreamOutSize() -> size_t {
    return ZSTD_compressBound((1i32 << 17i32) as
                                  size_t).wrapping_add(ZSTD_blockHeaderSize).wrapping_add(4i32
                                                                                              as
                                                                                              libc::c_ulong);
}
static mut ZSTD_frameHeaderSize_prefix: size_t = unsafe { 5i32 as size_t };
static mut ZSTD_frameHeaderSize_min: size_t = unsafe { 6i32 as size_t };
static mut ZSTD_skippableHeaderSize: size_t = unsafe { 8i32 as size_t };
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_CCtx(mut cctx: *const ZSTD_CCtx)
 -> size_t {
    if cctx.is_null() {
        return 0i32 as size_t
    } else {
        return (::std::mem::size_of::<ZSTD_CCtx>() as
                    libc::c_ulong).wrapping_add((*cctx).workSpaceSize).wrapping_add(ZSTD_sizeof_CDict((*cctx).cdictLocal)).wrapping_add(ZSTD_sizeof_mtctx(cctx))
    };
}
unsafe extern "C" fn ZSTD_sizeof_mtctx(mut cctx: *const ZSTD_CCtx) -> size_t {
    return ZSTDMT_sizeof_CCtx((*cctx).mtctx);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_CDict(mut cdict: *const ZSTD_CDict)
 -> size_t {
    if cdict.is_null() {
        return 0i32 as size_t
    } else {
        return (*cdict).workspaceSize.wrapping_add(if !(*cdict).dictBuffer.is_null()
                                                      {
                                                       (*cdict).dictContentSize
                                                   } else {
                                                       0i32 as libc::c_ulong
                                                   }).wrapping_add(::std::mem::size_of::<ZSTD_CDict>()
                                                                       as
                                                                       libc::c_ulong)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_CStream(mut zcs: *const ZSTD_CStream)
 -> size_t {
    return ZSTD_sizeof_CCtx(zcs);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCCtxSize(mut compressionLevel:
                                                   libc::c_int) -> size_t {
    let mut level: libc::c_int = 0;
    let mut memBudget: size_t = 0i32 as size_t;
    level = 1i32;
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
    return ZSTD_estimateCCtxSize_usingCCtxParams(&params as
                                                     *const ZSTD_CCtx_params);
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
    memset(&mut cctxParams as *mut ZSTD_CCtx_params as *mut libc::c_void,
           0i32, ::std::mem::size_of::<ZSTD_CCtx_params>() as libc::c_ulong);
    cctxParams.cParams = cParams;
    cctxParams.compressionLevel = 3i32;
    cctxParams.fParams.contentSizeFlag = 1i32 as libc::c_uint;
    return cctxParams;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCCtxSize_usingCCtxParams(mut params:
                                                                   *const ZSTD_CCtx_params)
 -> size_t {
    if (*params).nbWorkers > 0i32 as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        let cParams: ZSTD_compressionParameters =
            ZSTD_getCParamsFromCCtxParams(params, 0i32 as U64,
                                          0i32 as size_t);
        let blockSize: size_t =
            if ((1i32 << 17i32) as libc::c_ulong) <
                   (1i32 as size_t) << cParams.windowLog {
                (1i32 << 17i32) as libc::c_ulong
            } else { (1i32 as size_t) << cParams.windowLog };
        let divider: U32 =
            (if cParams.searchLength == 3i32 as libc::c_uint {
                 3i32
             } else { 4i32 }) as U32;
        let maxNbSeq: size_t =
            blockSize.wrapping_div(divider as libc::c_ulong);
        let tokenSpace: size_t =
            blockSize.wrapping_add((11i32 as
                                        libc::c_ulong).wrapping_mul(maxNbSeq));
        let entropySpace: size_t = (6i32 << 10i32) as size_t;
        let blockStateSpace: size_t =
            (2i32 as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<ZSTD_compressedBlockState_t>()
                                                 as libc::c_ulong);
        let matchStateSize: size_t =
            ZSTD_sizeof_matchState(&cParams as
                                       *const ZSTD_compressionParameters,
                                   1i32 as U32);
        let ldmSpace: size_t = ZSTD_ldm_getTableSize((*params).ldmParams);
        let ldmSeqSpace: size_t =
            ZSTD_ldm_getMaxNbSeq((*params).ldmParams,
                                 blockSize).wrapping_mul(::std::mem::size_of::<rawSeq>()
                                                             as
                                                             libc::c_ulong);
        let neededSpace: size_t =
            entropySpace.wrapping_add(blockStateSpace).wrapping_add(tokenSpace).wrapping_add(matchStateSize).wrapping_add(ldmSpace).wrapping_add(ldmSeqSpace);
        return (::std::mem::size_of::<ZSTD_CCtx>() as
                    libc::c_ulong).wrapping_add(neededSpace)
    };
}
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
    if 0 != (*CCtxParams).cParams.searchLength {
        cParams.searchLength = (*CCtxParams).cParams.searchLength
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
pub unsafe extern "C" fn ZSTD_estimateCStreamSize(mut compressionLevel:
                                                      libc::c_int) -> size_t {
    let mut level: libc::c_int = 0;
    let mut memBudget: size_t = 0i32 as size_t;
    level = 1i32;
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
    return ZSTD_estimateCStreamSize_usingCCtxParams(&params as
                                                        *const ZSTD_CCtx_params);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateCStreamSize_usingCCtxParams(mut params:
                                                                      *const ZSTD_CCtx_params)
 -> size_t {
    if (*params).nbWorkers > 0i32 as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
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
        return CCtxSize.wrapping_add(streamingSize)
    };
}
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
                                                libc::c_ulong).wrapping_add(ZSTD_sizeof_matchState(&mut cParams
                                                                                                       as
                                                                                                       *mut ZSTD_compressionParameters,
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
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticCCtx(mut workspace: *mut libc::c_void,
                                             mut workspaceSize: size_t)
 -> *mut ZSTD_CCtx {
    let cctx: *mut ZSTD_CCtx = workspace as *mut ZSTD_CCtx;
    if workspaceSize <= ::std::mem::size_of::<ZSTD_CCtx>() as libc::c_ulong {
        return 0 as *mut ZSTD_CCtx
    } else if 0 != workspace as size_t & 7i32 as libc::c_ulong {
        return 0 as *mut ZSTD_CCtx
    } else {
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
        } else {
            (*cctx).blockState.prevCBlock =
                (*cctx).workSpace as *mut ZSTD_compressedBlockState_t;
            (*cctx).blockState.nextCBlock =
                (*cctx).blockState.prevCBlock.offset(1isize);
            let ptr: *mut libc::c_void =
                (*cctx).blockState.nextCBlock.offset(1isize) as
                    *mut libc::c_void;
            (*cctx).entropyWorkspace = ptr as *mut U32;
            (*cctx).bmi2 = ZSTD_cpuid_bmi2(ZSTD_cpuid());
            return cctx
        }
    };
}
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
    let matchStateSize: size_t =
        ZSTD_sizeof_matchState(&mut cParams as
                                   *mut ZSTD_compressionParameters,
                               0i32 as U32);
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
    } else if workspaceSize < neededSize {
        return 0 as *const ZSTD_CDict
    } else {
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
               ZSTD_isError(ZSTD_initCDict_internal(cdict, dict, dictSize,
                                                    ZSTD_dlm_byRef,
                                                    dictContentType, cParams))
           {
            return 0 as *const ZSTD_CDict
        } else { return cdict }
    };
}
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
#[no_mangle]
pub unsafe extern "C" fn ZSTD_checkCParams(mut cParams:
                                               ZSTD_compressionParameters)
 -> size_t {
    if 0 !=
           (cParams.windowLog < 10i32 as libc::c_uint) as libc::c_int |
               (cParams.windowLog >
                    (if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                            4i32 as libc::c_ulong {
                         30i32
                     } else { 31i32 }) as libc::c_uint) as libc::c_int {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t
    } else if 0 !=
                  (cParams.chainLog < 6i32 as libc::c_uint) as libc::c_int |
                      (cParams.chainLog >
                           (if ::std::mem::size_of::<size_t>() as
                                   libc::c_ulong == 4i32 as libc::c_ulong {
                                29i32
                            } else { 30i32 }) as libc::c_uint) as libc::c_int
     {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t
    } else if 0 !=
                  (cParams.hashLog < 6i32 as libc::c_uint) as libc::c_int |
                      (cParams.hashLog >
                           if ((if ::std::mem::size_of::<size_t>() as
                                       libc::c_ulong == 4i32 as libc::c_ulong
                                   {
                                    30i32
                                } else { 31i32 }) as libc::c_uint) <
                                  30i32 as libc::c_uint {
                               (if ::std::mem::size_of::<size_t>() as
                                       libc::c_ulong == 4i32 as libc::c_ulong
                                   {
                                    30i32
                                } else { 31i32 }) as libc::c_uint
                           } else { 30i32 as libc::c_uint }) as libc::c_int {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t
    } else if 0 !=
                  (cParams.searchLog < 1i32 as libc::c_uint) as libc::c_int |
                      (cParams.searchLog >
                           ((if ::std::mem::size_of::<size_t>() as
                                    libc::c_ulong == 4i32 as libc::c_ulong {
                                 30i32
                             } else { 31i32 }) as
                                libc::c_uint).wrapping_sub(1i32 as
                                                               libc::c_uint))
                          as libc::c_int {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t
    } else if 0 !=
                  (cParams.searchLength < 3i32 as libc::c_uint) as libc::c_int
                      |
                      (cParams.searchLength > 7i32 as libc::c_uint) as
                          libc::c_int {
        return -(ZSTD_error_parameter_outOfBound as libc::c_int) as size_t
    } else if cParams.strategy as U32 > ZSTD_btultra as libc::c_int as U32 {
        return -(ZSTD_error_parameter_unsupported as libc::c_int) as size_t
    } else { return 0i32 as size_t };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_adjustCParams(mut cPar:
                                                ZSTD_compressionParameters,
                                            mut srcSize: libc::c_ulonglong,
                                            mut dictSize: size_t)
 -> ZSTD_compressionParameters {
    cPar = ZSTD_clampCParams(cPar);
    return ZSTD_adjustCParams_internal(cPar, srcSize, dictSize);
}
unsafe extern "C" fn ZSTD_clampCParams(mut cParams:
                                           ZSTD_compressionParameters)
 -> ZSTD_compressionParameters {
    if cParams.windowLog < 10i32 as libc::c_uint {
        cParams.windowLog = 10i32 as libc::c_uint
    } else if cParams.windowLog >
                  (if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                          4i32 as libc::c_ulong {
                       30i32
                   } else { 31i32 }) as libc::c_uint {
        cParams.windowLog =
            (if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                    4i32 as libc::c_ulong {
                 30i32
             } else { 31i32 }) as libc::c_uint
    }
    if cParams.chainLog < 6i32 as libc::c_uint {
        cParams.chainLog = 6i32 as libc::c_uint
    } else if cParams.chainLog >
                  (if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                          4i32 as libc::c_ulong {
                       29i32
                   } else { 30i32 }) as libc::c_uint {
        cParams.chainLog =
            (if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                    4i32 as libc::c_ulong {
                 29i32
             } else { 30i32 }) as libc::c_uint
    }
    if cParams.hashLog < 6i32 as libc::c_uint {
        cParams.hashLog = 6i32 as libc::c_uint
    } else if cParams.hashLog >
                  if ((if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                              4i32 as libc::c_ulong {
                           30i32
                       } else { 31i32 }) as libc::c_uint) <
                         30i32 as libc::c_uint {
                      (if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                              4i32 as libc::c_ulong {
                           30i32
                       } else { 31i32 }) as libc::c_uint
                  } else { 30i32 as libc::c_uint } {
        cParams.hashLog =
            if ((if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                        4i32 as libc::c_ulong {
                     30i32
                 } else { 31i32 }) as libc::c_uint) < 30i32 as libc::c_uint {
                (if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                        4i32 as libc::c_ulong {
                     30i32
                 } else { 31i32 }) as libc::c_uint
            } else { 30i32 as libc::c_uint }
    }
    if cParams.searchLog < 1i32 as libc::c_uint {
        cParams.searchLog = 1i32 as libc::c_uint
    } else if cParams.searchLog >
                  ((if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                           4i32 as libc::c_ulong {
                        30i32
                    } else { 31i32 }) as
                       libc::c_uint).wrapping_sub(1i32 as libc::c_uint) {
        cParams.searchLog =
            ((if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                     4i32 as libc::c_ulong {
                  30i32
              } else { 31i32 }) as
                 libc::c_uint).wrapping_sub(1i32 as libc::c_uint)
    }
    if cParams.searchLength < 3i32 as libc::c_uint {
        cParams.searchLength = 3i32 as libc::c_uint
    } else if cParams.searchLength > 7i32 as libc::c_uint {
        cParams.searchLength = 7i32 as libc::c_uint
    }
    if (cParams.strategy as libc::c_uint) <
           ZSTD_fast as libc::c_int as libc::c_uint {
        cParams.strategy = ZSTD_fast
    } else if cParams.strategy as libc::c_uint >
                  ZSTD_btultra as libc::c_int as libc::c_uint {
        cParams.strategy = ZSTD_btultra
    }
    return cParams;
}
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
    if 0 != ERR_isError(errcod) {
        return errcod
    } else {
        return ZSTD_compress_internal(cctx, dst, dstCapacity, src, srcSize,
                                      dict, dictSize, params)
    };
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
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream_usingDict(mut zcs:
                                                        *mut ZSTD_CStream,
                                                    mut dict:
                                                        *const libc::c_void,
                                                    mut dictSize: size_t,
                                                    mut compressionLevel:
                                                        libc::c_int)
 -> size_t {
    let params: ZSTD_parameters =
        ZSTD_getParams(compressionLevel, 0i32 as libc::c_ulonglong, dictSize);
    let cctxParams: ZSTD_CCtx_params =
        ZSTD_assignParamsToCCtxParams((*zcs).requestedParams, params);
    return ZSTD_initCStream_internal(zcs, dict, dictSize,
                                     0 as *const ZSTD_CDict, cctxParams,
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
    if 0 != ERR_isError(errcod) {
        return errcod
    } else {
        if pledgedSrcSize == 0i32 as libc::c_ulonglong &&
               params.fParams.contentSizeFlag == 0i32 as libc::c_uint {
            pledgedSrcSize = 0u64.wrapping_sub(1i32 as libc::c_ulonglong)
        }
        let cctxParams: ZSTD_CCtx_params =
            ZSTD_assignParamsToCCtxParams((*zcs).requestedParams, params);
        return ZSTD_initCStream_internal(zcs, dict, dictSize,
                                         0 as *const ZSTD_CDict, cctxParams,
                                         pledgedSrcSize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initCStream_usingCDict(mut zcs:
                                                         *mut ZSTD_CStream,
                                                     mut cdict:
                                                         *const ZSTD_CDict)
 -> size_t {
    let fParams: ZSTD_frameParameters =
        ZSTD_frameParameters{contentSizeFlag: 0i32 as libc::c_uint,
                             checksumFlag: 0i32 as libc::c_uint,
                             noDictIDFlag: 0i32 as libc::c_uint,};
    return ZSTD_initCStream_usingCDict_advanced(zcs, cdict, fParams,
                                                0u64.wrapping_sub(1i32 as
                                                                      libc::c_ulonglong));
}
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
    } else {
        let mut params: ZSTD_CCtx_params = (*zcs).requestedParams;
        params.cParams = ZSTD_getCParamsFromCDict(cdict);
        params.fParams = fParams;
        return ZSTD_initCStream_internal(zcs, 0 as *const libc::c_void,
                                         0i32 as size_t, cdict, params,
                                         pledgedSrcSize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_resetCStream(mut zcs: *mut ZSTD_CStream,
                                           mut pledgedSrcSize:
                                               libc::c_ulonglong) -> size_t {
    let mut params: ZSTD_CCtx_params = (*zcs).requestedParams;
    if pledgedSrcSize == 0i32 as libc::c_ulonglong {
        pledgedSrcSize = 0u64.wrapping_sub(1i32 as libc::c_ulonglong)
    }
    params.fParams.contentSizeFlag = 1i32 as libc::c_uint;
    params.cParams =
        ZSTD_getCParamsFromCCtxParams(&mut params as *mut ZSTD_CCtx_params,
                                      pledgedSrcSize as U64, 0i32 as size_t);
    return ZSTD_resetCStream_internal(zcs, 0 as *const libc::c_void,
                                      0i32 as size_t, ZSTD_dct_auto,
                                      (*zcs).cdict, params, pledgedSrcSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getFrameProgression(mut cctx: *const ZSTD_CCtx)
 -> ZSTD_frameProgression {
    if (*cctx).appliedParams.nbWorkers > 0i32 as libc::c_uint {
        return ZSTDMT_getFrameProgression((*cctx).mtctx)
    } else {
        let mut fp: ZSTD_frameProgression =
            ZSTD_frameProgression{ingested: 0, consumed: 0, produced: 0,};
        let buffered: size_t =
            if (*cctx).inBuff.is_null() {
                0i32 as libc::c_ulong
            } else { (*cctx).inBuffPos.wrapping_sub((*cctx).inToCompress) };
        0 != buffered;
        fp.ingested =
            (*cctx).consumedSrcSize.wrapping_add(buffered as
                                                     libc::c_ulonglong);
        fp.consumed = (*cctx).consumedSrcSize;
        fp.produced = (*cctx).producedCSize;
        return fp
    };
}
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
    if 0 != ERR_isError(errcod) {
        return errcod
    } else {
        return ZSTD_compressBegin_internal(cctx, dict, dictSize,
                                           dictContentType, dtlm, cdict,
                                           params, pledgedSrcSize as U64,
                                           ZSTDb_not_buffered)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBegin_usingCDict(mut cctx:
                                                           *mut ZSTD_CCtx,
                                                       mut cdict:
                                                           *const ZSTD_CDict)
 -> size_t {
    let fParams: ZSTD_frameParameters =
        ZSTD_frameParameters{contentSizeFlag: 0i32 as libc::c_uint,
                             checksumFlag: 0i32 as libc::c_uint,
                             noDictIDFlag: 0i32 as libc::c_uint,};
    return ZSTD_compressBegin_usingCDict_advanced(cctx, cdict, fParams,
                                                  0u64.wrapping_sub(1i32 as
                                                                        libc::c_ulonglong));
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_copyCCtx(mut dstCCtx: *mut ZSTD_CCtx,
                                       mut srcCCtx: *const ZSTD_CCtx,
                                       mut pledgedSrcSize: libc::c_ulonglong)
 -> size_t {
    let mut fParams: ZSTD_frameParameters =
        ZSTD_frameParameters{contentSizeFlag: 1i32 as libc::c_uint,
                             checksumFlag: 0i32 as libc::c_uint,
                             noDictIDFlag: 0i32 as libc::c_uint,};
    let zbuff: ZSTD_buffered_policy_e =
        ((*srcCCtx).inBuffSize > 0i32 as libc::c_ulong) as libc::c_int as
            ZSTD_buffered_policy_e;
    if pledgedSrcSize == 0i32 as libc::c_ulonglong {
        pledgedSrcSize = 0u64.wrapping_sub(1i32 as libc::c_ulonglong)
    }
    fParams.contentSizeFlag =
        (pledgedSrcSize != 0u64.wrapping_sub(1i32 as libc::c_ulonglong)) as
            libc::c_int as libc::c_uint;
    return ZSTD_copyCCtx_internal(dstCCtx, srcCCtx, fParams,
                                  pledgedSrcSize as U64, zbuff);
}
unsafe extern "C" fn ZSTD_copyCCtx_internal(mut dstCCtx: *mut ZSTD_CCtx,
                                            mut srcCCtx: *const ZSTD_CCtx,
                                            mut fParams: ZSTD_frameParameters,
                                            mut pledgedSrcSize: U64,
                                            mut zbuff: ZSTD_buffered_policy_e)
 -> size_t {
    if (*srcCCtx).stage as libc::c_uint !=
           ZSTDcs_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    } else {
        memcpy(&mut (*dstCCtx).customMem as *mut ZSTD_customMem as
                   *mut libc::c_void,
               &(*srcCCtx).customMem as *const ZSTD_customMem as
                   *const libc::c_void,
               ::std::mem::size_of::<ZSTD_customMem>() as libc::c_ulong);
        let mut params: ZSTD_CCtx_params = (*dstCCtx).requestedParams;
        params.cParams = (*srcCCtx).appliedParams.cParams;
        params.fParams = fParams;
        ZSTD_resetCCtx_internal(dstCCtx, params, pledgedSrcSize,
                                ZSTDcrp_noMemset, zbuff);
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
        memcpy((*dstCCtx).blockState.matchState.hashTable as
                   *mut libc::c_void,
               (*srcCCtx).blockState.matchState.hashTable as
                   *const libc::c_void, tableSpace);
        let mut srcMatchState: *const ZSTD_matchState_t_0 =
            &(*srcCCtx).blockState.matchState as *const ZSTD_matchState_t_0;
        let mut dstMatchState: *mut ZSTD_matchState_t_0 =
            &mut (*dstCCtx).blockState.matchState as *mut ZSTD_matchState_t_0;
        (*dstMatchState).window = (*srcMatchState).window;
        (*dstMatchState).nextToUpdate = (*srcMatchState).nextToUpdate;
        (*dstMatchState).nextToUpdate3 = (*srcMatchState).nextToUpdate3;
        (*dstMatchState).loadedDictEnd = (*srcMatchState).loadedDictEnd;
        (*dstCCtx).dictID = (*srcCCtx).dictID;
        memcpy((*dstCCtx).blockState.prevCBlock as *mut libc::c_void,
               (*srcCCtx).blockState.prevCBlock as *const libc::c_void,
               ::std::mem::size_of::<ZSTD_compressedBlockState_t>() as
                   libc::c_ulong);
        return 0i32 as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_setParameter(mut cctx: *mut ZSTD_CCtx,
                                                mut param: ZSTD_cParameter,
                                                mut value: libc::c_uint)
 -> size_t {
    if (*cctx).streamStage as libc::c_uint !=
           zcss_init as libc::c_int as libc::c_uint {
        if 0 != ZSTD_isUpdateAuthorized(param) {
            (*cctx).cParamsChanged = 1i32
        } else { return -(ZSTD_error_stage_wrong as libc::c_int) as size_t }
    }
    match param as libc::c_uint {
        10 => {
            return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams as
                                                   *mut ZSTD_CCtx_params,
                                               param, value)
        }
        100 => {
            if !(*cctx).cdict.is_null() {
                return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
            } else {
                return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams
                                                       as
                                                       *mut ZSTD_CCtx_params,
                                                   param, value)
            }
        }
        101 | 102 | 103 | 104 | 105 | 106 | 107 => {
            if !(*cctx).cdict.is_null() {
                return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
            } else {
                return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams
                                                       as
                                                       *mut ZSTD_CCtx_params,
                                                   param, value)
            }
        }
        200 | 201 | 202 => {
            return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams as
                                                   *mut ZSTD_CCtx_params,
                                               param, value)
        }
        1100 => {
            return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams as
                                                   *mut ZSTD_CCtx_params,
                                               param, value)
        }
        1101 => {
            return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams as
                                                   *mut ZSTD_CCtx_params,
                                               param, value)
        }
        400 => {
            if value > 0i32 as libc::c_uint && 0 != (*cctx).staticSize {
                return -(ZSTD_error_parameter_unsupported as libc::c_int) as
                           size_t
            } else {
                return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams
                                                       as
                                                       *mut ZSTD_CCtx_params,
                                                   param, value)
            }
        }
        401 | 402 => {
            return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams as
                                                   *mut ZSTD_CCtx_params,
                                               param, value)
        }
        160 | 161 | 162 | 163 | 164 => {
            if !(*cctx).cdict.is_null() {
                return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
            } else {
                return ZSTD_CCtxParam_setParameter(&mut (*cctx).requestedParams
                                                       as
                                                       *mut ZSTD_CCtx_params,
                                                   param, value)
            }
        }
        _ => {
            return -(ZSTD_error_parameter_unsupported as libc::c_int) as
                       size_t
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtxParam_setParameter(mut CCtxParams:
                                                         *mut ZSTD_CCtx_params,
                                                     mut param:
                                                         ZSTD_cParameter,
                                                     mut value: libc::c_uint)
 -> size_t {
    match param as libc::c_uint {
        10 => {
            if value > ZSTD_f_zstd1_magicless as libc::c_int as libc::c_uint {
                return -(ZSTD_error_parameter_unsupported as libc::c_int) as
                           size_t
            } else {
                (*CCtxParams).format = value as ZSTD_format_e;
                return (*CCtxParams).format as size_t
            }
        }
        100 => {
            let mut cLevel: libc::c_int = value as libc::c_int;
            if cLevel > ZSTD_maxCLevel() { cLevel = ZSTD_maxCLevel() }
            if 0 != cLevel { (*CCtxParams).compressionLevel = cLevel }
            if (*CCtxParams).compressionLevel >= 0i32 {
                return (*CCtxParams).compressionLevel as size_t
            } else { return 0i32 as size_t }
        }
        101 => {
            if value > 0i32 as libc::c_uint {
                if 0 !=
                       (value < 10i32 as libc::c_uint) as libc::c_int |
                           (value >
                                (if ::std::mem::size_of::<size_t>() as
                                        libc::c_ulong == 4i32 as libc::c_ulong
                                    {
                                     30i32
                                 } else { 31i32 }) as libc::c_uint) as
                               libc::c_int {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).cParams.windowLog = value;
            return (*CCtxParams).cParams.windowLog as size_t
        }
        102 => {
            if value > 0i32 as libc::c_uint {
                if 0 !=
                       (value < 6i32 as libc::c_uint) as libc::c_int |
                           (value >
                                if ((if ::std::mem::size_of::<size_t>() as
                                            libc::c_ulong ==
                                            4i32 as libc::c_ulong {
                                         30i32
                                     } else { 31i32 }) as libc::c_uint) <
                                       30i32 as libc::c_uint {
                                    (if ::std::mem::size_of::<size_t>() as
                                            libc::c_ulong ==
                                            4i32 as libc::c_ulong {
                                         30i32
                                     } else { 31i32 }) as libc::c_uint
                                } else { 30i32 as libc::c_uint }) as
                               libc::c_int {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).cParams.hashLog = value;
            return (*CCtxParams).cParams.hashLog as size_t
        }
        103 => {
            if value > 0i32 as libc::c_uint {
                if 0 !=
                       (value < 6i32 as libc::c_uint) as libc::c_int |
                           (value >
                                (if ::std::mem::size_of::<size_t>() as
                                        libc::c_ulong == 4i32 as libc::c_ulong
                                    {
                                     29i32
                                 } else { 30i32 }) as libc::c_uint) as
                               libc::c_int {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).cParams.chainLog = value;
            return (*CCtxParams).cParams.chainLog as size_t
        }
        104 => {
            if value > 0i32 as libc::c_uint {
                if 0 !=
                       (value < 1i32 as libc::c_uint) as libc::c_int |
                           (value >
                                ((if ::std::mem::size_of::<size_t>() as
                                         libc::c_ulong ==
                                         4i32 as libc::c_ulong {
                                      30i32
                                  } else { 31i32 }) as
                                     libc::c_uint).wrapping_sub(1i32 as
                                                                    libc::c_uint))
                               as libc::c_int {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).cParams.searchLog = value;
            return value as size_t
        }
        105 => {
            if value > 0i32 as libc::c_uint {
                if 0 !=
                       (value < 3i32 as libc::c_uint) as libc::c_int |
                           (value > 7i32 as libc::c_uint) as libc::c_int {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).cParams.searchLength = value;
            return (*CCtxParams).cParams.searchLength as size_t
        }
        106 => {
            (*CCtxParams).cParams.targetLength = value;
            return (*CCtxParams).cParams.targetLength as size_t
        }
        107 => {
            if value > 0i32 as libc::c_uint {
                if 0 !=
                       (value < ZSTD_fast as libc::c_int as libc::c_uint) as
                           libc::c_int |
                           (value >
                                ZSTD_btultra as libc::c_int as libc::c_uint)
                               as libc::c_int {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).cParams.strategy = value as ZSTD_strategy;
            return (*CCtxParams).cParams.strategy as size_t
        }
        200 => {
            (*CCtxParams).fParams.contentSizeFlag =
                (value > 0i32 as libc::c_uint) as libc::c_int as libc::c_uint;
            return (*CCtxParams).fParams.contentSizeFlag as size_t
        }
        201 => {
            (*CCtxParams).fParams.checksumFlag =
                (value > 0i32 as libc::c_uint) as libc::c_int as libc::c_uint;
            return (*CCtxParams).fParams.checksumFlag as size_t
        }
        202 => {
            (*CCtxParams).fParams.noDictIDFlag =
                (0 == value) as libc::c_int as libc::c_uint;
            return (0 == (*CCtxParams).fParams.noDictIDFlag) as libc::c_int as
                       size_t
        }
        1100 => {
            (*CCtxParams).forceWindow =
                (value > 0i32 as libc::c_uint) as libc::c_int;
            return (*CCtxParams).forceWindow as size_t
        }
        1101 => {
            (*CCtxParams).attachDictPref =
                (if 0 != value {
                     if value > 0i32 as libc::c_uint {
                         ZSTD_dictForceAttach as libc::c_int
                     } else { ZSTD_dictForceCopy as libc::c_int }
                 } else { ZSTD_dictDefaultAttach as libc::c_int }) as
                    ZSTD_dictAttachPref_e;
            return (*CCtxParams).attachDictPref as size_t
        }
        400 => { return ZSTDMT_CCtxParam_setNbWorkers(CCtxParams, value) }
        401 => {
            return ZSTDMT_CCtxParam_setMTCtxParameter(CCtxParams,
                                                      ZSTDMT_p_jobSize, value)
        }
        402 => {
            return ZSTDMT_CCtxParam_setMTCtxParameter(CCtxParams,
                                                      ZSTDMT_p_overlapSectionLog,
                                                      value)
        }
        160 => {
            (*CCtxParams).ldmParams.enableLdm =
                (value > 0i32 as libc::c_uint) as libc::c_int as U32;
            return (*CCtxParams).ldmParams.enableLdm as size_t
        }
        161 => {
            if value > 0i32 as libc::c_uint {
                if 0 !=
                       (value < 6i32 as libc::c_uint) as libc::c_int |
                           (value >
                                if ((if ::std::mem::size_of::<size_t>() as
                                            libc::c_ulong ==
                                            4i32 as libc::c_ulong {
                                         30i32
                                     } else { 31i32 }) as libc::c_uint) <
                                       30i32 as libc::c_uint {
                                    (if ::std::mem::size_of::<size_t>() as
                                            libc::c_ulong ==
                                            4i32 as libc::c_ulong {
                                         30i32
                                     } else { 31i32 }) as libc::c_uint
                                } else { 30i32 as libc::c_uint }) as
                               libc::c_int {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).ldmParams.hashLog = value;
            return (*CCtxParams).ldmParams.hashLog as size_t
        }
        162 => {
            if value > 0i32 as libc::c_uint {
                if 0 !=
                       (value < 4i32 as libc::c_uint) as libc::c_int |
                           (value > 4096i32 as libc::c_uint) as libc::c_int {
                    return -(ZSTD_error_parameter_outOfBound as libc::c_int)
                               as size_t
                }
            }
            (*CCtxParams).ldmParams.minMatchLength = value;
            return (*CCtxParams).ldmParams.minMatchLength as size_t
        }
        163 => {
            if value > 8i32 as libc::c_uint {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as
                           size_t
            } else {
                (*CCtxParams).ldmParams.bucketSizeLog = value;
                return (*CCtxParams).ldmParams.bucketSizeLog as size_t
            }
        }
        164 => {
            if value >
                   ((if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                            4i32 as libc::c_ulong {
                         30i32
                     } else { 31i32 }) as
                        libc::c_uint).wrapping_sub(6i32 as libc::c_uint) {
                return -(ZSTD_error_parameter_outOfBound as libc::c_int) as
                           size_t
            } else {
                (*CCtxParams).ldmParams.hashEveryLog = value;
                return (*CCtxParams).ldmParams.hashEveryLog as size_t
            }
        }
        _ => {
            return -(ZSTD_error_parameter_unsupported as libc::c_int) as
                       size_t
        }
    };
}
unsafe extern "C" fn ZSTD_isUpdateAuthorized(mut param: ZSTD_cParameter)
 -> libc::c_int {
    match param as libc::c_uint {
        100 | 102 | 103 | 104 | 105 | 106 | 107 => { return 1i32 }
        10 | 101 | 200 | 201 | 202 | 1100 | 400 | 401 | 402 | 160 | 161 | 162
        | 163 | 164 | 1101 | _ => {
            return 0i32
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_getParameter(mut cctx: *mut ZSTD_CCtx,
                                                mut param: ZSTD_cParameter,
                                                mut value: *mut libc::c_uint)
 -> size_t {
    return ZSTD_CCtxParam_getParameter(&mut (*cctx).requestedParams as
                                           *mut ZSTD_CCtx_params, param,
                                       value);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtxParam_getParameter(mut CCtxParams:
                                                         *mut ZSTD_CCtx_params,
                                                     mut param:
                                                         ZSTD_cParameter,
                                                     mut value:
                                                         *mut libc::c_uint)
 -> size_t {
    match param as libc::c_uint {
        10 => { *value = (*CCtxParams).format as libc::c_uint }
        100 => { *value = (*CCtxParams).compressionLevel as libc::c_uint }
        101 => { *value = (*CCtxParams).cParams.windowLog }
        102 => { *value = (*CCtxParams).cParams.hashLog }
        103 => { *value = (*CCtxParams).cParams.chainLog }
        104 => { *value = (*CCtxParams).cParams.searchLog }
        105 => { *value = (*CCtxParams).cParams.searchLength }
        106 => { *value = (*CCtxParams).cParams.targetLength }
        107 => { *value = (*CCtxParams).cParams.strategy as libc::c_uint }
        200 => { *value = (*CCtxParams).fParams.contentSizeFlag }
        201 => { *value = (*CCtxParams).fParams.checksumFlag }
        202 => {
            *value =
                (0 == (*CCtxParams).fParams.noDictIDFlag) as libc::c_int as
                    libc::c_uint
        }
        1100 => { *value = (*CCtxParams).forceWindow as libc::c_uint }
        1101 => { *value = (*CCtxParams).attachDictPref as libc::c_uint }
        400 => { *value = (*CCtxParams).nbWorkers }
        401 => { *value = (*CCtxParams).jobSize }
        402 => { *value = (*CCtxParams).overlapSizeLog }
        160 => { *value = (*CCtxParams).ldmParams.enableLdm }
        161 => { *value = (*CCtxParams).ldmParams.hashLog }
        162 => { *value = (*CCtxParams).ldmParams.minMatchLength }
        163 => { *value = (*CCtxParams).ldmParams.bucketSizeLog }
        164 => { *value = (*CCtxParams).ldmParams.hashEveryLog }
        _ => {
            return -(ZSTD_error_parameter_unsupported as libc::c_int) as
                       size_t
        }
    }
    return 0i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_setPledgedSrcSize(mut cctx: *mut ZSTD_CCtx,
                                                     mut pledgedSrcSize:
                                                         libc::c_ulonglong)
 -> size_t {
    if (*cctx).streamStage as libc::c_uint !=
           zcss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    } else {
        (*cctx).pledgedSrcSizePlusOne =
            pledgedSrcSize.wrapping_add(1i32 as libc::c_ulonglong);
        return 0i32 as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_loadDictionary(mut cctx: *mut ZSTD_CCtx,
                                                  mut dict:
                                                      *const libc::c_void,
                                                  mut dictSize: size_t)
 -> size_t {
    return ZSTD_CCtx_loadDictionary_advanced(cctx, dict, dictSize,
                                             ZSTD_dlm_byCopy, ZSTD_dct_auto);
}
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
    } else if 0 != (*cctx).staticSize {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    } else {
        ZSTD_freeCDict((*cctx).cdictLocal);
        if dict == 0 as *mut libc::c_void || dictSize == 0i32 as libc::c_ulong
           {
            (*cctx).cdictLocal = 0 as *mut ZSTD_CDict;
            (*cctx).cdict = 0 as *const ZSTD_CDict
        } else {
            let cParams: ZSTD_compressionParameters =
                ZSTD_getCParamsFromCCtxParams(&mut (*cctx).requestedParams as
                                                  *mut ZSTD_CCtx_params,
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
                return -(ZSTD_error_memory_allocation as libc::c_int) as
                           size_t
            }
        }
        return 0i32 as size_t
    };
}
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
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_refCDict(mut cctx: *mut ZSTD_CCtx,
                                            mut cdict: *const ZSTD_CDict)
 -> size_t {
    if (*cctx).streamStage as libc::c_uint !=
           zcss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    } else {
        (*cctx).cdict = cdict;
        memset(&mut (*cctx).prefixDict as *mut ZSTD_prefixDict as
                   *mut libc::c_void, 0i32,
               ::std::mem::size_of::<ZSTD_prefixDict>() as libc::c_ulong);
        return 0i32 as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_refPrefix(mut cctx: *mut ZSTD_CCtx,
                                             mut prefix: *const libc::c_void,
                                             mut prefixSize: size_t)
 -> size_t {
    return ZSTD_CCtx_refPrefix_advanced(cctx, prefix, prefixSize,
                                        ZSTD_dct_rawContent);
}
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
    } else {
        (*cctx).cdict = 0 as *const ZSTD_CDict;
        (*cctx).prefixDict.dict = prefix;
        (*cctx).prefixDict.dictSize = prefixSize;
        (*cctx).prefixDict.dictContentType = dictContentType;
        return 0i32 as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compress_generic(mut cctx: *mut ZSTD_CCtx,
                                               mut output:
                                                   *mut ZSTD_outBuffer,
                                               mut input: *mut ZSTD_inBuffer,
                                               mut endOp: ZSTD_EndDirective)
 -> size_t {
    if (*output).pos > (*output).size {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else if (*input).pos > (*input).size {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        if (*cctx).streamStage as libc::c_uint ==
               zcss_init as libc::c_int as libc::c_uint {
            let mut params: ZSTD_CCtx_params = (*cctx).requestedParams;
            let prefixDict: ZSTD_prefixDict = (*cctx).prefixDict;
            memset(&mut (*cctx).prefixDict as *mut ZSTD_prefixDict as
                       *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<ZSTD_prefixDict>() as libc::c_ulong);
            if endOp as libc::c_uint ==
                   ZSTD_e_end as libc::c_int as libc::c_uint {
                (*cctx).pledgedSrcSizePlusOne =
                    (*input).size.wrapping_add(1i32 as libc::c_ulong) as
                        libc::c_ulonglong
            }
            params.cParams =
                ZSTD_getCParamsFromCCtxParams(&mut (*cctx).requestedParams as
                                                  *mut ZSTD_CCtx_params,
                                              (*cctx).pledgedSrcSizePlusOne.wrapping_sub(1i32
                                                                                             as
                                                                                             libc::c_ulonglong)
                                                  as U64, 0i32 as size_t);
            if (*cctx).pledgedSrcSizePlusOne.wrapping_sub(1i32 as
                                                              libc::c_ulonglong)
                   <= (1u32 << 20i32) as libc::c_ulonglong {
                params.nbWorkers = 0i32 as libc::c_uint
            }
            if params.nbWorkers > 0i32 as libc::c_uint {
                if (*cctx).mtctx.is_null() {
                    (*cctx).mtctx =
                        ZSTDMT_createCCtx_advanced(params.nbWorkers,
                                                   (*cctx).customMem);
                    if (*cctx).mtctx.is_null() {
                        return -(ZSTD_error_memory_allocation as libc::c_int)
                                   as size_t
                    }
                }
                let errcod: size_t =
                    ZSTDMT_initCStream_internal((*cctx).mtctx,
                                                prefixDict.dict,
                                                prefixDict.dictSize,
                                                ZSTD_dct_rawContent,
                                                (*cctx).cdict, params,
                                                (*cctx).pledgedSrcSizePlusOne.wrapping_sub(1i32
                                                                                               as
                                                                                               libc::c_ulonglong));
                if 0 != ERR_isError(errcod) {
                    return errcod
                } else {
                    (*cctx).streamStage = zcss_load;
                    (*cctx).appliedParams.nbWorkers = params.nbWorkers
                }
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
        if (*cctx).appliedParams.nbWorkers > 0i32 as libc::c_uint {
            if 0 != (*cctx).cParamsChanged {
                ZSTDMT_updateCParams_whileCompressing((*cctx).mtctx,
                                                      &mut (*cctx).requestedParams
                                                          as
                                                          *mut ZSTD_CCtx_params);
                (*cctx).cParamsChanged = 0i32
            }
            let flushMin: size_t =
                ZSTDMT_compressStream_generic((*cctx).mtctx, output, input,
                                              endOp);
            if 0 != ZSTD_isError(flushMin) ||
                   endOp as libc::c_uint ==
                       ZSTD_e_end as libc::c_int as libc::c_uint &&
                       flushMin == 0i32 as libc::c_ulong {
                ZSTD_CCtx_reset(cctx);
            }
            return flushMin
        } else {
            let errcod_1: size_t =
                ZSTD_compressStream_generic(cctx, output, input, endOp);
            if 0 != ERR_isError(errcod_1) {
                return errcod_1
            } else {
                return (*cctx).outBuffContentSize.wrapping_sub((*cctx).outBuffFlushedSize)
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compress_generic_simpleArgs(mut cctx:
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
    let cErr: size_t =
        ZSTD_compress_generic(cctx, &mut output as *mut ZSTD_outBuffer,
                              &mut input as *mut ZSTD_inBuffer, endOp);
    *dstPos = output.pos;
    *srcPos = input.pos;
    return cErr;
}
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
    } else {
        params =
            ZSTD_calloc(::std::mem::size_of::<ZSTD_CCtx_params>() as
                            libc::c_ulong, customMem) as
                *mut ZSTD_CCtx_params;
        if params.is_null() {
            return 0 as *mut ZSTD_CCtx_params
        } else {
            (*params).customMem = customMem;
            (*params).compressionLevel = 3i32;
            (*params).fParams.contentSizeFlag = 1i32 as libc::c_uint;
            return params
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeCCtxParams(mut params:
                                                 *mut ZSTD_CCtx_params)
 -> size_t {
    if params.is_null() {
        return 0i32 as size_t
    } else {
        ZSTD_free(params as *mut libc::c_void, (*params).customMem);
        return 0i32 as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtxParams_init_advanced(mut cctxParams:
                                                           *mut ZSTD_CCtx_params,
                                                       mut params:
                                                           ZSTD_parameters)
 -> size_t {
    if cctxParams.is_null() {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        let errcod: size_t = ZSTD_checkCParams(params.cParams);
        if 0 != ERR_isError(errcod) {
            return errcod
        } else {
            memset(cctxParams as *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<ZSTD_CCtx_params>() as
                       libc::c_ulong);
            (*cctxParams).cParams = params.cParams;
            (*cctxParams).fParams = params.fParams;
            (*cctxParams).compressionLevel = 3i32;
            return 0i32 as size_t
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_CCtx_setParametersUsingCCtxParams(mut cctx:
                                                                    *mut ZSTD_CCtx,
                                                                mut params:
                                                                    *const ZSTD_CCtx_params)
 -> size_t {
    if (*cctx).streamStage as libc::c_uint !=
           zcss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    } else if !(*cctx).cdict.is_null() {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    } else { (*cctx).requestedParams = *params; return 0i32 as size_t };
}
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
    } else {
        return ZSTD_compressContinue_internal(cctx, dst, dstCapacity, src,
                                              srcSize, 0i32 as U32,
                                              0i32 as U32)
    };
}
static mut ZSTD_fcs_fieldSize: [size_t; 4] =
    unsafe {
        [0i32 as size_t, 2i32 as size_t, 4i32 as size_t, 8i32 as size_t]
    };
static mut ZSTD_did_fieldSize: [size_t; 4] =
    unsafe {
        [0i32 as size_t, 1i32 as size_t, 2i32 as size_t, 4i32 as size_t]
    };
static mut ZSTD_frameIdSize: size_t = unsafe { 4i32 as size_t };
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
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getSeqStore(mut ctx: *const ZSTD_CCtx)
 -> *const seqStore_t {
    return &(*ctx).seqStore as *const seqStore_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_invalidateRepCodes(mut cctx: *mut ZSTD_CCtx)
 -> () {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 3i32 {
        (*(*cctx).blockState.prevCBlock).rep[i as usize] = 0i32 as U32;
        i += 1
    };
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
#[no_mangle]
pub unsafe extern "C" fn ZSTD_writeLastEmptyBlock(mut dst: *mut libc::c_void,
                                                  mut dstCapacity: size_t)
 -> size_t {
    if dstCapacity < ZSTD_blockHeaderSize {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else {
        let cBlockHeader24: U32 =
            (1i32 as
                 libc::c_uint).wrapping_add((bt_raw as libc::c_int as U32) <<
                                                1i32);
        MEM_writeLE24(dst, cBlockHeader24);
        return ZSTD_blockHeaderSize
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_noCompressBlock(mut dst: *mut libc::c_void,
                                              mut dstCapacity: size_t,
                                              mut src: *const libc::c_void,
                                              mut srcSize: size_t) -> size_t {
    if srcSize.wrapping_add(ZSTD_blockHeaderSize) > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else {
        memcpy((dst as *mut BYTE).offset(ZSTD_blockHeaderSize as isize) as
                   *mut libc::c_void, src, srcSize);
        MEM_writeLE24(dst,
                      ((srcSize << 2i32) as
                           U32).wrapping_add(bt_raw as libc::c_int as U32));
        return ZSTD_blockHeaderSize.wrapping_add(srcSize)
    };
}
unsafe extern "C" fn run_static_initializers() -> () {
    maxWindowResize =
        (1u64 <<
             ((if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                      4i32 as libc::c_ulong {
                   30i32
               } else { 31i32 }) as
                  libc::c_uint).wrapping_sub(1i32 as libc::c_uint)) as U64
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn() -> (); 1] =
    [run_static_initializers];
