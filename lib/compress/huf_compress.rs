#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( extern_types , libc , offset_to )]
extern crate libc;
extern "C" {
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut _IO_2_1_stdin_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stdout_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stderr_: _IO_FILE_plus;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
    /* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
    /* ! Constructor and Destructor of FSE_DTable.
    Note that its size depends on 'tableLog' */
    /* *< Cannot use the previous table */
    /* *< Can use the previous table but it must be checked. Note : The previous table must have been constructed by HUF_compress{1, 4}X_repeat */
    /* * HUF_buildCTable_wksp() :
 *  Same as HUF_buildCTable(), but using externally allocated scratch buffer.
 *  `workSpace` must be aligned on 4-bytes boundaries, and be at least as large as a table of HUF_CTABLE_WORKSPACE_SIZE_U32 unsigned.
 */
    /* * HIST_count_wksp() :
 *  Same as HIST_count(), but using an externally provided scratch buffer.
 *  Benefit is this function will use very little stack space.
 * `workSpace` must be a table of unsigned of size >= HIST_WKSP_SIZE_U32
 */
    #[no_mangle]
    fn HIST_count_wksp(count: *mut libc::c_uint,
                       maxSymbolValuePtr: *mut libc::c_uint,
                       src: *const libc::c_void, srcSize: size_t,
                       workSpace: *mut libc::c_uint) -> size_t;
    /* ! HIST_count_simple() :
 *  Same as HIST_countFast(), this function is unsafe,
 *  and will segfault if any value within `src` is `> *maxSymbolValuePtr`.
 *  It is also a bit slower for large inputs.
 *  However, it does not need any additional memory (not even on stack).
 * @return : count of the most frequent symbol.
 *  Note this function doesn't produce any error (i.e. it must succeed).
 */
    #[no_mangle]
    fn HIST_count_simple(count: *mut libc::c_uint,
                         maxSymbolValuePtr: *mut libc::c_uint,
                         src: *const libc::c_void, srcSize: size_t)
     -> libc::c_uint;
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
    /* ! FSE_compress_usingCTable():
    Compress `src` using `ct` into `dst` which must be already allocated.
    @return : size of compressed data (<= `dstCapacity`),
              or 0 if compressed data could not fit into `dst`,
              or an errorCode, which can be tested using FSE_isError() */
    #[no_mangle]
    fn FSE_compress_usingCTable(dst: *mut libc::c_void, dstCapacity: size_t,
                                src: *const libc::c_void, srcSize: size_t,
                                ct: *const FSE_CTable) -> size_t;
    /* !
Tutorial :
----------
(Note : these functions only decompress FSE-compressed blocks.
 If block is uncompressed, use memcpy() instead
 If block is a single repeated byte, use memset() instead )

The first step is to obtain the normalized frequencies of symbols.
This can be performed by FSE_readNCount() if it was saved using FSE_writeNCount().
'normalizedCounter' must be already allocated, and have at least 'maxSymbolValuePtr[0]+1' cells of signed short.
In practice, that means it's necessary to know 'maxSymbolValue' beforehand,
or size the table to handle worst case situations (typically 256).
FSE_readNCount() will provide 'tableLog' and 'maxSymbolValue'.
The result of FSE_readNCount() is the number of bytes read from 'rBuffer'.
Note that 'rBufferSize' must be at least 4 bytes, even if useful information is less than that.
If there is an error, the function will return an error code, which can be tested using FSE_isError().

The next step is to build the decompression tables 'FSE_DTable' from 'normalizedCounter'.
This is performed by the function FSE_buildDTable().
The space required by 'FSE_DTable' must be already allocated using FSE_createDTable().
If there is an error, the function will return an error code, which can be tested using FSE_isError().

`FSE_DTable` can then be used to decompress `cSrc`, with FSE_decompress_usingDTable().
`cSrcSize` must be strictly correct, otherwise decompression will fail.
FSE_decompress_usingDTable() result will tell how many bytes were regenerated (<=`dstCapacity`).
If there is an error, the function will return an error code, which can be tested using FSE_isError(). (ex: dst buffer too small)
*/
    #[no_mangle]
    fn FSE_optimalTableLog_internal(maxTableLog: libc::c_uint,
                                    srcSize: size_t,
                                    maxSymbolValue: libc::c_uint,
                                    minus: libc::c_uint) -> libc::c_uint;
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
    /* * HUF_compress() :
 *  Compress content from buffer 'src', of size 'srcSize', into buffer 'dst'.
 * 'dst' buffer must be already allocated.
 *  Compression runs faster if `dstCapacity` >= HUF_compressBound(srcSize).
 * `srcSize` must be <= `HUF_BLOCKSIZE_MAX` == 128 KB.
 * @return : size of compressed data (<= `dstCapacity`).
 *  Special values : if return == 0, srcData is not compressible => Nothing is stored within dst !!!
 *                   if HUF_isError(return), compression failed (more details using HUF_getErrorName())
 */
    /* * HUF_compress2() :
 *  Same as HUF_compress(), but offers control over `maxSymbolValue` and `tableLog`.
 * `maxSymbolValue` must be <= HUF_SYMBOLVALUE_MAX .
 * `tableLog` must be `<= HUF_TABLELOG_MAX` . */
    /* *< provides error code string (useful for debugging) */
    /* * HUF_compress4X_wksp() :
 *  Same as HUF_compress2(), but uses externally allocated `workSpace`.
 * `workspace` must have minimum alignment of 4, and be at least as large as HUF_WORKSPACE_SIZE */
    /* * HUF_buildCTable_wksp() :
 *  Same as HUF_buildCTable(), but using externally allocated scratch buffer.
 * `workSpace` must be aligned on 4-bytes boundaries, and its size must be >= HUF_CTABLE_WORKSPACE_SIZE.
 */
    /* *< `workSpace` must be aligned on 4-bytes boundaries, `wkspSize` must be >= HUF_WORKSPACE_SIZE */
    /* ! HUF_compress() does the following:
 *  1. count symbol occurrence from source[] into table count[] using FSE_count() (exposed within "fse.h")
 *  2. (optional) refine tableLog using HUF_optimalTableLog()
 *  3. build Huffman table from count using HUF_buildCTable()
 *  4. save Huffman table to memory buffer using HUF_writeCTable()
 *  5. encode the data stream using HUF_compress4X_usingCTable()
 *
 *  The following API allows targeting specific sub-functions for advanced tasks.
 *  For example, it's possible to compress several blocks using the same 'CTable',
 *  or to save and regenerate 'CTable' using external methods.
 */
    /* *< double-symbols decoder */
    /* *< maximum input size for a single block compressed with HUF_compress */
    /* * HUF_compress4X_repeat() :
 *  Same as HUF_compress4X_wksp(), but considers using hufTable if *repeat != HUF_repeat_none.
 *  If it uses hufTable it does not modify hufTable or repeat.
 *  If it doesn't, it sets *repeat = HUF_repeat_none, and it sets hufTable to the table used.
 *  If preferRepeat then the old table will always be used if valid. */
    /* *< Can use the previous table and it is assumed to be valid */
    /* ! HUF_readStats() :
 *  Read compact Huffman tree, saved by HUF_writeCTable().
 * `huffWeight` is destination buffer.
 * @return : size read from `src` , or an error Code .
 *  Note : Needed by HUF_readCTable() and HUF_readDTableXn() . */
    #[no_mangle]
    fn HUF_readStats(huffWeight: *mut BYTE, hwSize: size_t,
                     rankStats: *mut U32, nbSymbolsPtr: *mut U32,
                     tableLogPtr: *mut U32, src: *const libc::c_void,
                     srcSize: size_t) -> size_t;
}
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct rankPos {
    pub base: U32,
    pub current: U32,
}
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DState_t {
    pub state: size_t,
    pub table: *const libc::c_void,
}
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct nodeElt_s {
    pub count: U32,
    pub parent: U16,
    pub byte: BYTE,
    pub nbBits: BYTE,
}
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
pub type unnamed = libc::c_uint;
pub const MEM_static_assert: unnamed_1 = 1;
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
pub type U32 = uint32_t;
pub type uint32_t = libc::c_uint;
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
pub type size_t = libc::c_ulong;
pub type FSE_CTable = libc::c_uint;
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    u: U32,
    c: [BYTE; 4],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_CStream_t {
    pub bitContainer: size_t,
    pub bitPos: libc::c_uint,
    pub startPtr: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub endPtr: *mut libc::c_char,
}
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct HUF_CElt_s {
    pub val: U16,
    pub nbBits: BYTE,
}
pub type FSE_DTable = libc::c_uint;
pub type HUF_CElt = HUF_CElt_s;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub type uint16_t = libc::c_ushort;
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
pub type ptrdiff_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type ZSTD_ErrorCode = libc::c_uint;
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
pub type ERR_enum = ZSTD_ErrorCode;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct HUF_compress_tables_t {
    pub count: [U32; 256],
    pub CTable: [HUF_CElt; 256],
    pub nodeTable: huffNodeTable,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
pub type U64 = uint64_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: libc::c_uint,
    pub ptr: *const libc::c_char,
    pub start: *const libc::c_char,
    pub limitPtr: *const libc::c_char,
}
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
pub type S16 = int16_t;
pub type U16 = uint16_t;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
pub type BIT_DStream_status = libc::c_uint;
pub type BYTE = uint8_t;
pub type HUF_repeat = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
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
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub type _IO_lock_t = ();
pub type unnamed_1 = libc::c_uint;
pub type huffNodeTable = [nodeElt; 512];
pub type uint8_t = libc::c_uchar;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const libc::c_void,
    pub symbolTT: *const libc::c_void,
    pub stateLog: libc::c_uint,
}
pub type nodeElt = nodeElt_s;
pub const MEM_static_assert_0: unnamed = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub type int16_t = libc::c_short;
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
pub type uint64_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: libc::c_int,
    pub deltaNbBits: U32,
}
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
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
    let one: unnamed_0 = unnamed_0{u: 1i32 as U32,};
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
                    current_block = 4468360398284854458;
                }
                6 => { current_block = 4468360398284854458; }
                5 => { current_block = 17167734969414915964; }
                4 => { current_block = 15133352111665004258; }
                3 => { current_block = 11781329542230641433; }
                2 => { current_block = 2098600215101093122; }
                _ => { current_block = 10886091980245723256; }
            }
            match current_block {
                4468360398284854458 => {
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
                    current_block = 17167734969414915964;
                }
                _ => { }
            }
            match current_block {
                17167734969414915964 => {
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
                    current_block = 15133352111665004258;
                }
                _ => { }
            }
            match current_block {
                15133352111665004258 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(3isize)
                                                              as size_t) <<
                                                             24i32) as size_t
                            as size_t;
                    current_block = 11781329542230641433;
                }
                _ => { }
            }
            match current_block {
                11781329542230641433 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(2isize)
                                                              as size_t) <<
                                                             16i32) as size_t
                            as size_t;
                    current_block = 2098600215101093122;
                }
                _ => { }
            }
            match current_block {
                2098600215101093122 => {
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
pub unsafe extern "C" fn HUF_compress(mut dst: *mut libc::c_void,
                                      mut maxDstSize: size_t,
                                      mut src: *const libc::c_void,
                                      mut srcSize: size_t) -> size_t {
    return HUF_compress2(dst, maxDstSize, src, srcSize,
                         255i32 as libc::c_uint, 11i32 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compress2(mut dst: *mut libc::c_void,
                                       mut dstSize: size_t,
                                       mut src: *const libc::c_void,
                                       mut srcSize: size_t,
                                       mut maxSymbolValue: libc::c_uint,
                                       mut huffLog: libc::c_uint) -> size_t {
    let mut workSpace: [libc::c_uint; 1536] = [0; 1536];
    return HUF_compress4X_wksp(dst, dstSize, src, srcSize, maxSymbolValue,
                               huffLog,
                               workSpace.as_mut_ptr() as *mut libc::c_void,
                               ::std::mem::size_of::<[libc::c_uint; 1536]>()
                                   as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compress4X_wksp(mut dst: *mut libc::c_void,
                                             mut dstSize: size_t,
                                             mut src: *const libc::c_void,
                                             mut srcSize: size_t,
                                             mut maxSymbolValue: libc::c_uint,
                                             mut huffLog: libc::c_uint,
                                             mut workSpace: *mut libc::c_void,
                                             mut wkspSize: size_t) -> size_t {
    return HUF_compress_internal(dst, dstSize, src, srcSize, maxSymbolValue,
                                 huffLog, 0i32 as libc::c_uint, workSpace,
                                 wkspSize, 0 as *mut HUF_CElt,
                                 0 as *mut HUF_repeat, 0i32, 0i32);
}
unsafe extern "C" fn HUF_compress_internal(mut dst: *mut libc::c_void,
                                           mut dstSize: size_t,
                                           mut src: *const libc::c_void,
                                           mut srcSize: size_t,
                                           mut maxSymbolValue: libc::c_uint,
                                           mut huffLog: libc::c_uint,
                                           mut singleStream: libc::c_uint,
                                           mut workSpace: *mut libc::c_void,
                                           mut wkspSize: size_t,
                                           mut oldHufTable: *mut HUF_CElt,
                                           mut repeat: *mut HUF_repeat,
                                           mut preferRepeat: libc::c_int,
                                           bmi2: libc::c_int) -> size_t {
    let table: *mut HUF_compress_tables_t =
        workSpace as *mut HUF_compress_tables_t;
    let ostart: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = ostart.offset(dstSize as isize);
    let mut op: *mut BYTE = ostart;
    if workSpace as size_t & 3i32 as libc::c_ulong != 0i32 as libc::c_ulong {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else if wkspSize <
                  ::std::mem::size_of::<HUF_compress_tables_t>() as
                      libc::c_ulong {
        return -(ZSTD_error_workSpace_tooSmall as libc::c_int) as size_t
    } else if 0 == srcSize {
        return 0i32 as size_t
    } else if 0 == dstSize {
        return 0i32 as size_t
    } else if srcSize > (128i32 * 1024i32) as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else if huffLog > 12i32 as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    } else if maxSymbolValue > 255i32 as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as size_t
    } else {
        if 0 == maxSymbolValue { maxSymbolValue = 255i32 as libc::c_uint }
        if 0 == huffLog { huffLog = 11i32 as libc::c_uint }
        if 0 != preferRepeat && !repeat.is_null() &&
               *repeat as libc::c_uint ==
                   HUF_repeat_valid as libc::c_int as libc::c_uint {
            return HUF_compressCTable_internal(ostart, op, oend, src, srcSize,
                                               singleStream, oldHufTable,
                                               bmi2)
        } else {
            let largest: size_t =
                HIST_count_wksp((*table).count.as_mut_ptr(),
                                &mut maxSymbolValue as *mut libc::c_uint,
                                src as *const BYTE as *const libc::c_void,
                                srcSize, (*table).count.as_mut_ptr());
            if 0 != ERR_isError(largest) {
                return largest
            } else if largest == srcSize {
                *ostart = *(src as *const BYTE).offset(0isize);
                return 1i32 as size_t
            } else if largest <=
                          (srcSize >>
                               7i32).wrapping_add(4i32 as libc::c_ulong) {
                return 0i32 as size_t
            } else {
                if !repeat.is_null() &&
                       *repeat as libc::c_uint ==
                           HUF_repeat_check as libc::c_int as libc::c_uint &&
                       0 ==
                           HUF_validateCTable(oldHufTable,
                                              (*table).count.as_mut_ptr(),
                                              maxSymbolValue) {
                    *repeat = HUF_repeat_none
                }
                if 0 != preferRepeat && !repeat.is_null() &&
                       *repeat as libc::c_uint !=
                           HUF_repeat_none as libc::c_int as libc::c_uint {
                    return HUF_compressCTable_internal(ostart, op, oend, src,
                                                       srcSize, singleStream,
                                                       oldHufTable, bmi2)
                } else {
                    huffLog =
                        HUF_optimalTableLog(huffLog, srcSize, maxSymbolValue);
                    let maxBits: size_t =
                        HUF_buildCTable_wksp((*table).CTable.as_mut_ptr(),
                                             (*table).count.as_mut_ptr(),
                                             maxSymbolValue, huffLog,
                                             (*table).nodeTable.as_mut_ptr()
                                                 as *mut libc::c_void,
                                             ::std::mem::size_of::<huffNodeTable>()
                                                 as libc::c_ulong);
                    if 0 != ERR_isError(maxBits) {
                        return maxBits
                    } else {
                        huffLog = maxBits as U32;
                        memset((*table).CTable.as_mut_ptr().offset(maxSymbolValue.wrapping_add(1i32
                                                                                                   as
                                                                                                   libc::c_uint)
                                                                       as
                                                                       isize)
                                   as *mut libc::c_void, 0i32,
                               (::std::mem::size_of::<[HUF_CElt; 256]>() as
                                    libc::c_ulong).wrapping_sub((maxSymbolValue.wrapping_add(1i32
                                                                                                 as
                                                                                                 libc::c_uint)
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<HUF_CElt>()
                                                                                                     as
                                                                                                     libc::c_ulong)));
                        let hSize: size_t =
                            HUF_writeCTable(op as *mut libc::c_void, dstSize,
                                            (*table).CTable.as_mut_ptr(),
                                            maxSymbolValue, huffLog);
                        if 0 != ERR_isError(hSize) {
                            return hSize
                        } else {
                            if !repeat.is_null() &&
                                   *repeat as libc::c_uint !=
                                       HUF_repeat_none as libc::c_int as
                                           libc::c_uint {
                                let oldSize: size_t =
                                    HUF_estimateCompressedSize(oldHufTable,
                                                               (*table).count.as_mut_ptr(),
                                                               maxSymbolValue);
                                let newSize: size_t =
                                    HUF_estimateCompressedSize((*table).CTable.as_mut_ptr(),
                                                               (*table).count.as_mut_ptr(),
                                                               maxSymbolValue);
                                if oldSize <= hSize.wrapping_add(newSize) ||
                                       hSize.wrapping_add(12i32 as
                                                              libc::c_ulong)
                                           >= srcSize {
                                    return HUF_compressCTable_internal(ostart,
                                                                       op,
                                                                       oend,
                                                                       src,
                                                                       srcSize,
                                                                       singleStream,
                                                                       oldHufTable,
                                                                       bmi2)
                                }
                            }
                            if hSize.wrapping_add(12u64) >= srcSize {
                                return 0i32 as size_t
                            } else {
                                op = op.offset(hSize as isize);
                                if !repeat.is_null() {
                                    *repeat = HUF_repeat_none
                                }
                                if !oldHufTable.is_null() {
                                    memcpy(oldHufTable as *mut libc::c_void,
                                           (*table).CTable.as_mut_ptr() as
                                               *const libc::c_void,
                                           ::std::mem::size_of::<[HUF_CElt; 256]>()
                                               as libc::c_ulong);
                                }
                                return HUF_compressCTable_internal(ostart, op,
                                                                   oend, src,
                                                                   srcSize,
                                                                   singleStream,
                                                                   (*table).CTable.as_mut_ptr(),
                                                                   bmi2)
                            }
                        }
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn HUF_compressCTable_internal(ostart: *mut BYTE,
                                                 mut op: *mut BYTE,
                                                 oend: *mut BYTE,
                                                 mut src: *const libc::c_void,
                                                 mut srcSize: size_t,
                                                 mut singleStream:
                                                     libc::c_uint,
                                                 mut CTable: *const HUF_CElt,
                                                 bmi2: libc::c_int)
 -> size_t {
    let cSize: size_t =
        if 0 != singleStream {
            HUF_compress1X_usingCTable_internal(op as *mut libc::c_void,
                                                op.offset_to(oend).expect("bad offset_to")
                                                    as libc::c_long as size_t,
                                                src, srcSize, CTable, bmi2)
        } else {
            HUF_compress4X_usingCTable_internal(op as *mut libc::c_void,
                                                op.offset_to(oend).expect("bad offset_to")
                                                    as libc::c_long as size_t,
                                                src, srcSize, CTable, bmi2)
        };
    if 0 != ERR_isError(cSize) {
        return cSize
    } else if cSize == 0i32 as libc::c_ulong {
        return 0i32 as size_t
    } else {
        op = op.offset(cSize as isize);
        if ostart.offset_to(op).expect("bad offset_to") as libc::c_long as
               size_t >= srcSize.wrapping_sub(1i32 as libc::c_ulong) {
            return 0i32 as size_t
        } else {
            return ostart.offset_to(op).expect("bad offset_to") as
                       libc::c_long as size_t
        }
    };
}
unsafe extern "C" fn HUF_compress4X_usingCTable_internal(mut dst:
                                                             *mut libc::c_void,
                                                         mut dstSize: size_t,
                                                         mut src:
                                                             *const libc::c_void,
                                                         mut srcSize: size_t,
                                                         mut CTable:
                                                             *const HUF_CElt,
                                                         mut bmi2:
                                                             libc::c_int)
 -> size_t {
    let segmentSize: size_t =
        srcSize.wrapping_add(3i32 as
                                 libc::c_ulong).wrapping_div(4i32 as
                                                                 libc::c_ulong);
    let mut ip: *const BYTE = src as *const BYTE;
    let iend: *const BYTE = ip.offset(srcSize as isize);
    let ostart: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = ostart.offset(dstSize as isize);
    let mut op: *mut BYTE = ostart;
    if dstSize < (6i32 + 1i32 + 1i32 + 1i32 + 8i32) as libc::c_ulong {
        return 0i32 as size_t
    } else if srcSize < 12i32 as libc::c_ulong {
        return 0i32 as size_t
    } else {
        op = op.offset(6isize);
        let cSize: size_t =
            HUF_compress1X_usingCTable_internal(op as *mut libc::c_void,
                                                op.offset_to(oend).expect("bad offset_to")
                                                    as libc::c_long as size_t,
                                                ip as *const libc::c_void,
                                                segmentSize, CTable, bmi2);
        if 0 != ERR_isError(cSize) {
            return cSize
        } else if cSize == 0i32 as libc::c_ulong {
            return 0i32 as size_t
        } else {
            MEM_writeLE16(ostart as *mut libc::c_void, cSize as U16);
            op = op.offset(cSize as isize);
            ip = ip.offset(segmentSize as isize);
            let cSize_0: size_t =
                HUF_compress1X_usingCTable_internal(op as *mut libc::c_void,
                                                    op.offset_to(oend).expect("bad offset_to")
                                                        as libc::c_long as
                                                        size_t,
                                                    ip as *const libc::c_void,
                                                    segmentSize, CTable,
                                                    bmi2);
            if 0 != ERR_isError(cSize_0) {
                return cSize_0
            } else if cSize_0 == 0i32 as libc::c_ulong {
                return 0i32 as size_t
            } else {
                MEM_writeLE16(ostart.offset(2isize) as *mut libc::c_void,
                              cSize_0 as U16);
                op = op.offset(cSize_0 as isize);
                ip = ip.offset(segmentSize as isize);
                let cSize_1: size_t =
                    HUF_compress1X_usingCTable_internal(op as
                                                            *mut libc::c_void,
                                                        op.offset_to(oend).expect("bad offset_to")
                                                            as libc::c_long as
                                                            size_t,
                                                        ip as
                                                            *const libc::c_void,
                                                        segmentSize, CTable,
                                                        bmi2);
                if 0 != ERR_isError(cSize_1) {
                    return cSize_1
                } else if cSize_1 == 0i32 as libc::c_ulong {
                    return 0i32 as size_t
                } else {
                    MEM_writeLE16(ostart.offset(4isize) as *mut libc::c_void,
                                  cSize_1 as U16);
                    op = op.offset(cSize_1 as isize);
                    ip = ip.offset(segmentSize as isize);
                    let cSize_2: size_t =
                        HUF_compress1X_usingCTable_internal(op as
                                                                *mut libc::c_void,
                                                            op.offset_to(oend).expect("bad offset_to")
                                                                as
                                                                libc::c_long
                                                                as size_t,
                                                            ip as
                                                                *const libc::c_void,
                                                            ip.offset_to(iend).expect("bad offset_to")
                                                                as
                                                                libc::c_long
                                                                as size_t,
                                                            CTable, bmi2);
                    if 0 != ERR_isError(cSize_2) {
                        return cSize_2
                    } else if cSize_2 == 0i32 as libc::c_ulong {
                        return 0i32 as size_t
                    } else {
                        op = op.offset(cSize_2 as isize);
                        return ostart.offset_to(op).expect("bad offset_to") as
                                   libc::c_long as size_t
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal(mut dst:
                                                             *mut libc::c_void,
                                                         mut dstSize: size_t,
                                                         mut src:
                                                             *const libc::c_void,
                                                         mut srcSize: size_t,
                                                         mut CTable:
                                                             *const HUF_CElt,
                                                         bmi2: libc::c_int)
 -> size_t {
    if 0 != bmi2 {
        return HUF_compress1X_usingCTable_internal_bmi2(dst, dstSize, src,
                                                        srcSize, CTable)
    } else {
        return HUF_compress1X_usingCTable_internal_default(dst, dstSize, src,
                                                           srcSize, CTable)
    };
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_default(mut dst:
                                                                     *mut libc::c_void,
                                                                 mut dstSize:
                                                                     size_t,
                                                                 mut src:
                                                                     *const libc::c_void,
                                                                 mut srcSize:
                                                                     size_t,
                                                                 mut CTable:
                                                                     *const HUF_CElt)
 -> size_t {
    return HUF_compress1X_usingCTable_internal_body(dst, dstSize, src,
                                                    srcSize, CTable);
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_body(mut dst:
                                                                  *mut libc::c_void,
                                                              mut dstSize:
                                                                  size_t,
                                                              mut src:
                                                                  *const libc::c_void,
                                                              mut srcSize:
                                                                  size_t,
                                                              mut CTable:
                                                                  *const HUF_CElt)
 -> size_t {
    let mut current_block: u64;
    let mut ip: *const BYTE = src as *const BYTE;
    let ostart: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = ostart.offset(dstSize as isize);
    let mut op: *mut BYTE = ostart;
    let mut n: size_t = 0;
    let mut bitC: BIT_CStream_t =
        BIT_CStream_t{bitContainer: 0,
                      bitPos: 0,
                      startPtr: 0 as *mut libc::c_char,
                      ptr: 0 as *mut libc::c_char,
                      endPtr: 0 as *mut libc::c_char,};
    if dstSize < 8i32 as libc::c_ulong {
        return 0i32 as size_t
    } else {
        let initErr: size_t =
            BIT_initCStream(&mut bitC as *mut BIT_CStream_t,
                            op as *mut libc::c_void,
                            op.offset_to(oend).expect("bad offset_to") as
                                libc::c_long as size_t);
        if 0 != ERR_isError(initErr) {
            return 0i32 as size_t
        } else {
            n = srcSize & !3i32 as libc::c_ulong;
            match srcSize & 3i32 as libc::c_ulong {
                3 => {
                    HUF_encodeSymbol(&mut bitC as *mut BIT_CStream_t,
                                     *ip.offset(n.wrapping_add(2i32 as
                                                                   libc::c_ulong)
                                                    as isize) as U32, CTable);
                    if (::std::mem::size_of::<size_t>() as
                            libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong)
                           < (12i32 * 4i32 + 7i32) as libc::c_ulong {
                        BIT_flushBits(&mut bitC as *mut BIT_CStream_t);
                        current_block = 2437549442820723588;
                    } else { current_block = 2437549442820723588; }
                }
                2 => { current_block = 2437549442820723588; }
                1 => { current_block = 6070316113060876105; }
                0 | _ => { current_block = 10879442775620481940; }
            }
            match current_block {
                2437549442820723588 => {
                    HUF_encodeSymbol(&mut bitC as *mut BIT_CStream_t,
                                     *ip.offset(n.wrapping_add(1i32 as
                                                                   libc::c_ulong)
                                                    as isize) as U32, CTable);
                    if (::std::mem::size_of::<size_t>() as
                            libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong)
                           < (12i32 * 2i32 + 7i32) as libc::c_ulong {
                        BIT_flushBits(&mut bitC as *mut BIT_CStream_t);
                        current_block = 6070316113060876105;
                    } else { current_block = 6070316113060876105; }
                }
                _ => { }
            }
            match current_block {
                6070316113060876105 => {
                    HUF_encodeSymbol(&mut bitC as *mut BIT_CStream_t,
                                     *ip.offset(n.wrapping_add(0i32 as
                                                                   libc::c_ulong)
                                                    as isize) as U32, CTable);
                    BIT_flushBits(&mut bitC as *mut BIT_CStream_t);
                }
                _ => { }
            }
            while n > 0i32 as libc::c_ulong {
                HUF_encodeSymbol(&mut bitC as *mut BIT_CStream_t,
                                 *ip.offset(n.wrapping_sub(1i32 as
                                                               libc::c_ulong)
                                                as isize) as U32, CTable);
                if (::std::mem::size_of::<size_t>() as
                        libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) <
                       (12i32 * 2i32 + 7i32) as libc::c_ulong {
                    BIT_flushBits(&mut bitC as *mut BIT_CStream_t);
                }
                HUF_encodeSymbol(&mut bitC as *mut BIT_CStream_t,
                                 *ip.offset(n.wrapping_sub(2i32 as
                                                               libc::c_ulong)
                                                as isize) as U32, CTable);
                if (::std::mem::size_of::<size_t>() as
                        libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) <
                       (12i32 * 4i32 + 7i32) as libc::c_ulong {
                    BIT_flushBits(&mut bitC as *mut BIT_CStream_t);
                }
                HUF_encodeSymbol(&mut bitC as *mut BIT_CStream_t,
                                 *ip.offset(n.wrapping_sub(3i32 as
                                                               libc::c_ulong)
                                                as isize) as U32, CTable);
                if (::std::mem::size_of::<size_t>() as
                        libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) <
                       (12i32 * 2i32 + 7i32) as libc::c_ulong {
                    BIT_flushBits(&mut bitC as *mut BIT_CStream_t);
                }
                HUF_encodeSymbol(&mut bitC as *mut BIT_CStream_t,
                                 *ip.offset(n.wrapping_sub(4i32 as
                                                               libc::c_ulong)
                                                as isize) as U32, CTable);
                BIT_flushBits(&mut bitC as *mut BIT_CStream_t);
                n =
                    (n as libc::c_ulong).wrapping_sub(4i32 as libc::c_ulong)
                        as size_t as size_t
            }
            return BIT_closeCStream(&mut bitC as *mut BIT_CStream_t)
        }
    };
}
unsafe extern "C" fn HUF_encodeSymbol(mut bitCPtr: *mut BIT_CStream_t,
                                      mut symbol: U32,
                                      mut CTable: *const HUF_CElt) -> () {
    BIT_addBitsFast(bitCPtr, (*CTable.offset(symbol as isize)).val as size_t,
                    (*CTable.offset(symbol as isize)).nbBits as libc::c_uint);
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_bmi2(mut dst:
                                                                  *mut libc::c_void,
                                                              mut dstSize:
                                                                  size_t,
                                                              mut src:
                                                                  *const libc::c_void,
                                                              mut srcSize:
                                                                  size_t,
                                                              mut CTable:
                                                                  *const HUF_CElt)
 -> size_t {
    return HUF_compress1X_usingCTable_internal_body(dst, dstSize, src,
                                                    srcSize, CTable);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_writeCTable(mut dst: *mut libc::c_void,
                                         mut maxDstSize: size_t,
                                         mut CTable: *const HUF_CElt,
                                         mut maxSymbolValue: U32,
                                         mut huffLog: U32) -> size_t {
    let mut bitsToWeight: [BYTE; 13] = [0; 13];
    let mut huffWeight: [BYTE; 255] = [0; 255];
    let mut op: *mut BYTE = dst as *mut BYTE;
    let mut n: U32 = 0;
    if maxSymbolValue > 255i32 as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as size_t
    } else {
        bitsToWeight[0usize] = 0i32 as BYTE;
        n = 1i32 as U32;
        while n < huffLog.wrapping_add(1i32 as libc::c_uint) {
            bitsToWeight[n as usize] =
                huffLog.wrapping_add(1i32 as libc::c_uint).wrapping_sub(n) as
                    BYTE;
            n = n.wrapping_add(1)
        }
        n = 0i32 as U32;
        while n < maxSymbolValue {
            huffWeight[n as usize] =
                bitsToWeight[(*CTable.offset(n as isize)).nbBits as usize];
            n = n.wrapping_add(1)
        }
        let hSize: size_t =
            HUF_compressWeights(op.offset(1isize) as *mut libc::c_void,
                                maxDstSize.wrapping_sub(1i32 as
                                                            libc::c_ulong),
                                huffWeight.as_mut_ptr() as
                                    *const libc::c_void,
                                maxSymbolValue as size_t);
        if 0 != ERR_isError(hSize) {
            return hSize
        } else if 0 !=
                      (hSize > 1i32 as libc::c_ulong) as libc::c_int &
                          (hSize <
                               maxSymbolValue.wrapping_div(2i32 as
                                                               libc::c_uint)
                                   as libc::c_ulong) as libc::c_int {
            *op.offset(0isize) = hSize as BYTE;
            return hSize.wrapping_add(1i32 as libc::c_ulong)
        } else if maxSymbolValue > (256i32 - 128i32) as libc::c_uint {
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t
        } else if maxSymbolValue.wrapping_add(1i32 as
                                                  libc::c_uint).wrapping_div(2i32
                                                                                 as
                                                                                 libc::c_uint).wrapping_add(1i32
                                                                                                                as
                                                                                                                libc::c_uint)
                      as libc::c_ulong > maxDstSize {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
        } else {
            *op.offset(0isize) =
                (128i32 as
                     libc::c_uint).wrapping_add(maxSymbolValue.wrapping_sub(1i32
                                                                                as
                                                                                libc::c_uint))
                    as BYTE;
            huffWeight[maxSymbolValue as usize] = 0i32 as BYTE;
            n = 0i32 as U32;
            while n < maxSymbolValue {
                *op.offset(n.wrapping_div(2i32 as
                                              libc::c_uint).wrapping_add(1i32
                                                                             as
                                                                             libc::c_uint)
                               as isize) =
                    (((huffWeight[n as usize] as libc::c_int) << 4i32) +
                         huffWeight[n.wrapping_add(1i32 as libc::c_uint) as
                                        usize] as libc::c_int) as BYTE;
                n =
                    (n as libc::c_uint).wrapping_add(2i32 as libc::c_uint) as
                        U32 as U32
            }
            return maxSymbolValue.wrapping_add(1i32 as
                                                   libc::c_uint).wrapping_div(2i32
                                                                                  as
                                                                                  libc::c_uint).wrapping_add(1i32
                                                                                                                 as
                                                                                                                 libc::c_uint)
                       as size_t
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compressWeights(mut dst: *mut libc::c_void,
                                             mut dstSize: size_t,
                                             mut weightTable:
                                                 *const libc::c_void,
                                             mut wtSize: size_t) -> size_t {
    let ostart: *mut BYTE = dst as *mut BYTE;
    let mut op: *mut BYTE = ostart;
    let oend: *mut BYTE = ostart.offset(dstSize as isize);
    let mut maxSymbolValue: U32 = 12i32 as U32;
    let mut tableLog: U32 = 6i32 as U32;
    let mut CTable: [FSE_CTable; 59] = [0; 59];
    let mut scratchBuffer: [BYTE; 64] = [0; 64];
    let mut count: [U32; 13] = [0; 13];
    let mut norm: [S16; 13] = [0; 13];
    if wtSize <= 1i32 as libc::c_ulong {
        return 0i32 as size_t
    } else {
        let maxCount: libc::c_uint =
            HIST_count_simple(count.as_mut_ptr(),
                              &mut maxSymbolValue as *mut U32, weightTable,
                              wtSize);
        if maxCount as libc::c_ulong == wtSize {
            return 1i32 as size_t
        } else if maxCount == 1i32 as libc::c_uint {
            return 0i32 as size_t
        } else {
            tableLog = FSE_optimalTableLog(tableLog, wtSize, maxSymbolValue);
            let _var_err__: size_t =
                FSE_normalizeCount(norm.as_mut_ptr(), tableLog,
                                   count.as_mut_ptr(), wtSize,
                                   maxSymbolValue);
            if 0 != ERR_isError(_var_err__) {
                return _var_err__
            } else {
                let hSize: size_t =
                    FSE_writeNCount(op as *mut libc::c_void,
                                    op.offset_to(oend).expect("bad offset_to")
                                        as libc::c_long as size_t,
                                    norm.as_mut_ptr(), maxSymbolValue,
                                    tableLog);
                if 0 != ERR_isError(hSize) {
                    return hSize
                } else {
                    op = op.offset(hSize as isize);
                    let _var_err___0: size_t =
                        FSE_buildCTable_wksp(CTable.as_mut_ptr(),
                                             norm.as_mut_ptr(),
                                             maxSymbolValue, tableLog,
                                             scratchBuffer.as_mut_ptr() as
                                                 *mut libc::c_void,
                                             ::std::mem::size_of::<[BYTE; 64]>()
                                                 as libc::c_ulong);
                    if 0 != ERR_isError(_var_err___0) {
                        return _var_err___0
                    } else {
                        let cSize: size_t =
                            FSE_compress_usingCTable(op as *mut libc::c_void,
                                                     op.offset_to(oend).expect("bad offset_to")
                                                         as libc::c_long as
                                                         size_t, weightTable,
                                                     wtSize,
                                                     CTable.as_mut_ptr());
                        if 0 != ERR_isError(cSize) {
                            return cSize
                        } else if cSize == 0i32 as libc::c_ulong {
                            return 0i32 as size_t
                        } else {
                            op = op.offset(cSize as isize);
                            return ostart.offset_to(op).expect("bad offset_to")
                                       as libc::c_long as size_t
                        }
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn HUF_estimateCompressedSize(mut CTable: *mut HUF_CElt,
                                                mut count:
                                                    *const libc::c_uint,
                                                mut maxSymbolValue:
                                                    libc::c_uint) -> size_t {
    let mut nbBits: size_t = 0i32 as size_t;
    let mut s: libc::c_int = 0;
    s = 0i32;
    while s <= maxSymbolValue as libc::c_int {
        nbBits =
            (nbBits as
                 libc::c_ulong).wrapping_add(((*CTable.offset(s as
                                                                  isize)).nbBits
                                                  as
                                                  libc::c_uint).wrapping_mul(*count.offset(s
                                                                                               as
                                                                                               isize))
                                                 as libc::c_ulong) as size_t
                as size_t;
        s += 1
    }
    return nbBits >> 3i32;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_buildCTable_wksp(mut tree: *mut HUF_CElt,
                                              mut count: *const U32,
                                              mut maxSymbolValue: U32,
                                              mut maxNbBits: U32,
                                              mut workSpace:
                                                  *mut libc::c_void,
                                              mut wkspSize: size_t)
 -> size_t {
    let mut n2: U32 = 0;
    let mut n1: U32 = 0;
    let huffNode0: *mut nodeElt = workSpace as *mut nodeElt;
    let huffNode: *mut nodeElt = huffNode0.offset(1isize);
    let mut n: U32 = 0;
    let mut nonNullRank: U32 = 0;
    let mut lowS: libc::c_int = 0;
    let mut lowN: libc::c_int = 0;
    let mut nodeNb: U16 = (255i32 + 1i32) as U16;
    let mut nodeRoot: U32 = 0;
    if workSpace as size_t & 3i32 as libc::c_ulong != 0i32 as libc::c_ulong {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else if wkspSize <
                  ::std::mem::size_of::<huffNodeTable>() as libc::c_ulong {
        return -(ZSTD_error_workSpace_tooSmall as libc::c_int) as size_t
    } else {
        if maxNbBits == 0i32 as libc::c_uint { maxNbBits = 11i32 as U32 }
        if maxSymbolValue > 255i32 as libc::c_uint {
            return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as
                       size_t
        } else {
            memset(huffNode0 as *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<huffNodeTable>() as libc::c_ulong);
            HUF_sort(huffNode, count, maxSymbolValue);
            nonNullRank = maxSymbolValue;
            while (*huffNode.offset(nonNullRank as isize)).count ==
                      0i32 as libc::c_uint {
                nonNullRank = nonNullRank.wrapping_sub(1)
            }
            lowS = nonNullRank as libc::c_int;
            nodeRoot = (nodeNb as libc::c_int + lowS - 1i32) as U32;
            lowN = nodeNb as libc::c_int;
            (*huffNode.offset(nodeNb as isize)).count =
                (*huffNode.offset(lowS as
                                      isize)).count.wrapping_add((*huffNode.offset((lowS
                                                                                        -
                                                                                        1i32)
                                                                                       as
                                                                                       isize)).count);
            let ref mut fresh0 =
                (*huffNode.offset((lowS - 1i32) as isize)).parent;
            *fresh0 = nodeNb;
            (*huffNode.offset(lowS as isize)).parent = *fresh0;
            nodeNb = nodeNb.wrapping_add(1);
            lowS -= 2i32;
            n = nodeNb as U32;
            while n <= nodeRoot {
                (*huffNode.offset(n as isize)).count = 1u32 << 30i32;
                n = n.wrapping_add(1)
            }
            (*huffNode0.offset(0isize)).count = 1u32 << 31i32;
            while nodeNb as libc::c_uint <= nodeRoot {
                n1 =
                    (if (*huffNode.offset(lowS as isize)).count <
                            (*huffNode.offset(lowN as isize)).count {
                         let fresh1 = lowS;
                         lowS = lowS - 1;
                         fresh1
                     } else { let fresh2 = lowN; lowN = lowN + 1; fresh2 }) as
                        U32;
                n2 =
                    (if (*huffNode.offset(lowS as isize)).count <
                            (*huffNode.offset(lowN as isize)).count {
                         let fresh3 = lowS;
                         lowS = lowS - 1;
                         fresh3
                     } else { let fresh4 = lowN; lowN = lowN + 1; fresh4 }) as
                        U32;
                (*huffNode.offset(nodeNb as isize)).count =
                    (*huffNode.offset(n1 as
                                          isize)).count.wrapping_add((*huffNode.offset(n2
                                                                                           as
                                                                                           isize)).count);
                let ref mut fresh5 = (*huffNode.offset(n2 as isize)).parent;
                *fresh5 = nodeNb;
                (*huffNode.offset(n1 as isize)).parent = *fresh5;
                nodeNb = nodeNb.wrapping_add(1)
            }
            (*huffNode.offset(nodeRoot as isize)).nbBits = 0i32 as BYTE;
            n = nodeRoot.wrapping_sub(1i32 as libc::c_uint);
            while n >= (255i32 + 1i32) as libc::c_uint {
                (*huffNode.offset(n as isize)).nbBits =
                    ((*huffNode.offset((*huffNode.offset(n as isize)).parent
                                           as isize)).nbBits as libc::c_int +
                         1i32) as BYTE;
                n = n.wrapping_sub(1)
            }
            n = 0i32 as U32;
            while n <= nonNullRank {
                (*huffNode.offset(n as isize)).nbBits =
                    ((*huffNode.offset((*huffNode.offset(n as isize)).parent
                                           as isize)).nbBits as libc::c_int +
                         1i32) as BYTE;
                n = n.wrapping_add(1)
            }
            maxNbBits = HUF_setMaxHeight(huffNode, nonNullRank, maxNbBits);
            let mut nbPerRank: [U16; 13] =
                [0i32 as U16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            let mut valPerRank: [U16; 13] =
                [0i32 as U16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            if maxNbBits > 12i32 as libc::c_uint {
                return -(ZSTD_error_GENERIC as libc::c_int) as size_t
            } else {
                n = 0i32 as U32;
                while n <= nonNullRank {
                    nbPerRank[(*huffNode.offset(n as isize)).nbBits as usize]
                        =
                        nbPerRank[(*huffNode.offset(n as isize)).nbBits as
                                      usize].wrapping_add(1);
                    n = n.wrapping_add(1)
                }
                let mut min: U16 = 0i32 as U16;
                n = maxNbBits;
                while n > 0i32 as libc::c_uint {
                    valPerRank[n as usize] = min;
                    min =
                        (min as libc::c_int +
                             nbPerRank[n as usize] as libc::c_int) as U16;
                    min = (min as libc::c_int >> 1i32) as U16;
                    n = n.wrapping_sub(1)
                }
                n = 0i32 as U32;
                while n <= maxSymbolValue {
                    (*tree.offset((*huffNode.offset(n as isize)).byte as
                                      isize)).nbBits =
                        (*huffNode.offset(n as isize)).nbBits;
                    n = n.wrapping_add(1)
                }
                n = 0i32 as U32;
                while n <= maxSymbolValue {
                    let fresh6 =
                        valPerRank[(*tree.offset(n as isize)).nbBits as
                                       usize];
                    valPerRank[(*tree.offset(n as isize)).nbBits as usize] =
                        valPerRank[(*tree.offset(n as isize)).nbBits as
                                       usize].wrapping_add(1);
                    (*tree.offset(n as isize)).val = fresh6;
                    n = n.wrapping_add(1)
                }
                return maxNbBits as size_t
            }
        }
    };
}
unsafe extern "C" fn HUF_setMaxHeight(mut huffNode: *mut nodeElt,
                                      mut lastNonNull: U32,
                                      mut maxNbBits: U32) -> U32 {
    let largestBits: U32 =
        (*huffNode.offset(lastNonNull as isize)).nbBits as U32;
    if largestBits <= maxNbBits {
        return largestBits
    } else {
        let mut totalCost: libc::c_int = 0i32;
        let baseCost: U32 =
            (1i32 << largestBits.wrapping_sub(maxNbBits)) as U32;
        let mut n: U32 = lastNonNull;
        while (*huffNode.offset(n as isize)).nbBits as libc::c_uint >
                  maxNbBits {
            totalCost =
                (totalCost as
                     libc::c_uint).wrapping_add(baseCost.wrapping_sub((1i32 <<
                                                                           largestBits.wrapping_sub((*huffNode.offset(n
                                                                                                                          as
                                                                                                                          isize)).nbBits
                                                                                                        as
                                                                                                        libc::c_uint))
                                                                          as
                                                                          libc::c_uint))
                    as libc::c_int as libc::c_int;
            (*huffNode.offset(n as isize)).nbBits = maxNbBits as BYTE;
            n = n.wrapping_sub(1)
        }
        while (*huffNode.offset(n as isize)).nbBits as libc::c_uint ==
                  maxNbBits {
            n = n.wrapping_sub(1)
        }
        totalCost >>= largestBits.wrapping_sub(maxNbBits);
        let noSymbol: U32 = 4042322160u32;
        let mut rankLast: [U32; 14] = [0; 14];
        let mut pos: libc::c_int = 0;
        memset(rankLast.as_mut_ptr() as *mut libc::c_void, 240i32,
               ::std::mem::size_of::<[U32; 14]>() as libc::c_ulong);
        let mut currentNbBits: U32 = maxNbBits;
        pos = n as libc::c_int;
        while pos >= 0i32 {
            if !((*huffNode.offset(pos as isize)).nbBits as libc::c_uint >=
                     currentNbBits) {
                currentNbBits =
                    (*huffNode.offset(pos as isize)).nbBits as U32;
                rankLast[maxNbBits.wrapping_sub(currentNbBits) as usize] =
                    pos as U32
            }
            pos -= 1
        }
        while totalCost > 0i32 {
            let mut nBitsToDecrease: U32 =
                BIT_highbit32(totalCost as
                                  U32).wrapping_add(1i32 as libc::c_uint);
            while nBitsToDecrease > 1i32 as libc::c_uint {
                let mut highPos: U32 = rankLast[nBitsToDecrease as usize];
                let mut lowPos: U32 =
                    rankLast[nBitsToDecrease.wrapping_sub(1i32 as
                                                              libc::c_uint) as
                                 usize];
                if !(highPos == noSymbol) {
                    if lowPos == noSymbol { break ; }
                    let highTotal: U32 =
                        (*huffNode.offset(highPos as isize)).count;
                    let lowTotal: U32 =
                        (2i32 as
                             libc::c_uint).wrapping_mul((*huffNode.offset(lowPos
                                                                              as
                                                                              isize)).count);
                    if highTotal <= lowTotal { break ; }
                }
                nBitsToDecrease = nBitsToDecrease.wrapping_sub(1)
            }
            while nBitsToDecrease <= 12i32 as libc::c_uint &&
                      rankLast[nBitsToDecrease as usize] == noSymbol {
                nBitsToDecrease = nBitsToDecrease.wrapping_add(1)
            }
            totalCost -=
                1i32 << nBitsToDecrease.wrapping_sub(1i32 as libc::c_uint);
            if rankLast[nBitsToDecrease.wrapping_sub(1i32 as libc::c_uint) as
                            usize] == noSymbol {
                rankLast[nBitsToDecrease.wrapping_sub(1i32 as libc::c_uint) as
                             usize] = rankLast[nBitsToDecrease as usize]
            }
            let ref mut fresh7 =
                (*huffNode.offset(rankLast[nBitsToDecrease as usize] as
                                      isize)).nbBits;
            *fresh7 = (*fresh7).wrapping_add(1);
            if rankLast[nBitsToDecrease as usize] == 0i32 as libc::c_uint {
                rankLast[nBitsToDecrease as usize] = noSymbol
            } else {
                rankLast[nBitsToDecrease as usize] =
                    rankLast[nBitsToDecrease as usize].wrapping_sub(1);
                if !((*huffNode.offset(rankLast[nBitsToDecrease as usize] as
                                           isize)).nbBits as libc::c_uint !=
                         maxNbBits.wrapping_sub(nBitsToDecrease)) {
                    continue ;
                }
                rankLast[nBitsToDecrease as usize] = noSymbol
            }
        }
        while totalCost < 0i32 {
            if rankLast[1usize] == noSymbol {
                while (*huffNode.offset(n as isize)).nbBits as libc::c_uint ==
                          maxNbBits {
                    n = n.wrapping_sub(1)
                }
                let ref mut fresh8 =
                    (*huffNode.offset(n.wrapping_add(1i32 as libc::c_uint) as
                                          isize)).nbBits;
                *fresh8 = (*fresh8).wrapping_sub(1);
                rankLast[1usize] = n.wrapping_add(1i32 as libc::c_uint);
                totalCost += 1
            } else {
                let ref mut fresh9 =
                    (*huffNode.offset(rankLast[1usize].wrapping_add(1i32 as
                                                                        libc::c_uint)
                                          as isize)).nbBits;
                *fresh9 = (*fresh9).wrapping_sub(1);
                rankLast[1usize] = rankLast[1usize].wrapping_add(1);
                totalCost += 1
            }
        }
        return maxNbBits
    };
}
unsafe extern "C" fn HUF_sort(mut huffNode: *mut nodeElt,
                              mut count: *const U32, mut maxSymbolValue: U32)
 -> () {
    let mut rank: [rankPos; 32] = [rankPos{base: 0, current: 0,}; 32];
    let mut n: U32 = 0;
    memset(rank.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[rankPos; 32]>() as libc::c_ulong);
    n = 0i32 as U32;
    while n <= maxSymbolValue {
        let mut r: U32 =
            BIT_highbit32((*count.offset(n as
                                             isize)).wrapping_add(1i32 as
                                                                      libc::c_uint));
        rank[r as usize].base = rank[r as usize].base.wrapping_add(1);
        n = n.wrapping_add(1)
    }
    n = 30i32 as U32;
    while n > 0i32 as libc::c_uint {
        rank[n.wrapping_sub(1i32 as libc::c_uint) as usize].base =
            (rank[n.wrapping_sub(1i32 as libc::c_uint) as usize].base as
                 libc::c_uint).wrapping_add(rank[n as usize].base) as U32 as
                U32;
        n = n.wrapping_sub(1)
    }
    n = 0i32 as U32;
    while n < 32i32 as libc::c_uint {
        rank[n as usize].current = rank[n as usize].base;
        n = n.wrapping_add(1)
    }
    n = 0i32 as U32;
    while n <= maxSymbolValue {
        let c: U32 = *count.offset(n as isize);
        let r_0: U32 =
            BIT_highbit32(c.wrapping_add(1i32 as
                                             libc::c_uint)).wrapping_add(1i32
                                                                             as
                                                                             libc::c_uint);
        let fresh10 = rank[r_0 as usize].current;
        rank[r_0 as usize].current =
            rank[r_0 as usize].current.wrapping_add(1);
        let mut pos: U32 = fresh10;
        while pos > rank[r_0 as usize].base &&
                  c >
                      (*huffNode.offset(pos.wrapping_sub(1i32 as libc::c_uint)
                                            as isize)).count {
            *huffNode.offset(pos as isize) =
                *huffNode.offset(pos.wrapping_sub(1i32 as libc::c_uint) as
                                     isize);
            pos = pos.wrapping_sub(1)
        }
        (*huffNode.offset(pos as isize)).count = c;
        (*huffNode.offset(pos as isize)).byte = n as BYTE;
        n = n.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUF_optimalTableLog(mut maxTableLog: libc::c_uint,
                                             mut srcSize: size_t,
                                             mut maxSymbolValue: libc::c_uint)
 -> libc::c_uint {
    return FSE_optimalTableLog_internal(maxTableLog, srcSize, maxSymbolValue,
                                        1i32 as libc::c_uint);
}
unsafe extern "C" fn HUF_validateCTable(mut CTable: *const HUF_CElt,
                                        mut count: *const libc::c_uint,
                                        mut maxSymbolValue: libc::c_uint)
 -> libc::c_int {
    let mut bad: libc::c_int = 0i32;
    let mut s: libc::c_int = 0;
    s = 0i32;
    while s <= maxSymbolValue as libc::c_int {
        bad |=
            (*count.offset(s as isize) != 0i32 as libc::c_uint) as libc::c_int
                &
                ((*CTable.offset(s as isize)).nbBits as libc::c_int == 0i32)
                    as libc::c_int;
        s += 1
    }
    return (0 == bad) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compressBound(mut size: size_t) -> size_t {
    return (129i32 as
                libc::c_ulong).wrapping_add(size.wrapping_add(size >>
                                                                  8i32).wrapping_add(8i32
                                                                                         as
                                                                                         libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn HUF_buildCTable(mut tree: *mut HUF_CElt,
                                         mut count: *const U32,
                                         mut maxSymbolValue: U32,
                                         mut maxNbBits: U32) -> size_t {
    let mut nodeTable: huffNodeTable =
        [nodeElt_s{count: 0, parent: 0, byte: 0, nbBits: 0,}; 512];
    return HUF_buildCTable_wksp(tree, count, maxSymbolValue, maxNbBits,
                                nodeTable.as_mut_ptr() as *mut libc::c_void,
                                ::std::mem::size_of::<huffNodeTable>() as
                                    libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compress4X_usingCTable(mut dst:
                                                        *mut libc::c_void,
                                                    mut dstSize: size_t,
                                                    mut src:
                                                        *const libc::c_void,
                                                    mut srcSize: size_t,
                                                    mut CTable:
                                                        *const HUF_CElt)
 -> size_t {
    return HUF_compress4X_usingCTable_internal(dst, dstSize, src, srcSize,
                                               CTable, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compress4X_repeat(mut dst: *mut libc::c_void,
                                               mut dstSize: size_t,
                                               mut src: *const libc::c_void,
                                               mut srcSize: size_t,
                                               mut maxSymbolValue:
                                                   libc::c_uint,
                                               mut huffLog: libc::c_uint,
                                               mut workSpace:
                                                   *mut libc::c_void,
                                               mut wkspSize: size_t,
                                               mut hufTable: *mut HUF_CElt,
                                               mut repeat: *mut HUF_repeat,
                                               mut preferRepeat: libc::c_int,
                                               mut bmi2: libc::c_int)
 -> size_t {
    return HUF_compress_internal(dst, dstSize, src, srcSize, maxSymbolValue,
                                 huffLog, 0i32 as libc::c_uint, workSpace,
                                 wkspSize, hufTable, repeat, preferRepeat,
                                 bmi2);
}
/* * HUF_readCTable() :
 *  Loading a CTable saved with HUF_writeCTable() */
#[no_mangle]
pub unsafe extern "C" fn HUF_readCTable(mut CTable: *mut HUF_CElt,
                                        mut maxSymbolValuePtr: *mut U32,
                                        mut src: *const libc::c_void,
                                        mut srcSize: size_t) -> size_t {
    let mut huffWeight: [BYTE; 256] = [0; 256];
    let mut rankVal: [U32; 16] = [0; 16];
    let mut tableLog: U32 = 0i32 as U32;
    let mut nbSymbols: U32 = 0i32 as U32;
    let readSize: size_t =
        HUF_readStats(huffWeight.as_mut_ptr(), (255i32 + 1i32) as size_t,
                      rankVal.as_mut_ptr(), &mut nbSymbols as *mut U32,
                      &mut tableLog as *mut U32, src, srcSize);
    if 0 != ERR_isError(readSize) {
        return readSize
    } else if tableLog > 12i32 as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    } else if nbSymbols >
                  (*maxSymbolValuePtr).wrapping_add(1i32 as libc::c_uint) {
        return -(ZSTD_error_maxSymbolValue_tooSmall as libc::c_int) as size_t
    } else {
        let mut n: U32 = 0;
        let mut nextRankStart: U32 = 0i32 as U32;
        n = 1i32 as U32;
        while n <= tableLog {
            let mut current: U32 = nextRankStart;
            nextRankStart =
                (nextRankStart as
                     libc::c_uint).wrapping_add(rankVal[n as usize] <<
                                                    n.wrapping_sub(1i32 as
                                                                       libc::c_uint))
                    as U32 as U32;
            rankVal[n as usize] = current;
            n = n.wrapping_add(1)
        }
        let mut n_0: U32 = 0;
        n_0 = 0i32 as U32;
        while n_0 < nbSymbols {
            let w: U32 = huffWeight[n_0 as usize] as U32;
            (*CTable.offset(n_0 as isize)).nbBits =
                tableLog.wrapping_add(1i32 as libc::c_uint).wrapping_sub(w) as
                    BYTE;
            n_0 = n_0.wrapping_add(1)
        }
        let mut nbPerRank: [U16; 14] =
            [0i32 as U16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut valPerRank: [U16; 14] =
            [0i32 as U16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut n_1: U32 = 0;
        n_1 = 0i32 as U32;
        while n_1 < nbSymbols {
            nbPerRank[(*CTable.offset(n_1 as isize)).nbBits as usize] =
                nbPerRank[(*CTable.offset(n_1 as isize)).nbBits as
                              usize].wrapping_add(1);
            n_1 = n_1.wrapping_add(1)
        }
        valPerRank[tableLog.wrapping_add(1i32 as libc::c_uint) as usize] =
            0i32 as U16;
        let mut min: U16 = 0i32 as U16;
        let mut n_2: U32 = 0;
        n_2 = tableLog;
        while n_2 > 0i32 as libc::c_uint {
            valPerRank[n_2 as usize] = min;
            min =
                (min as libc::c_int + nbPerRank[n_2 as usize] as libc::c_int)
                    as U16;
            min = (min as libc::c_int >> 1i32) as U16;
            n_2 = n_2.wrapping_sub(1)
        }
        let mut n_3: U32 = 0;
        n_3 = 0i32 as U32;
        while n_3 < nbSymbols {
            let fresh11 =
                valPerRank[(*CTable.offset(n_3 as isize)).nbBits as usize];
            valPerRank[(*CTable.offset(n_3 as isize)).nbBits as usize] =
                valPerRank[(*CTable.offset(n_3 as isize)).nbBits as
                               usize].wrapping_add(1);
            (*CTable.offset(n_3 as isize)).val = fresh11;
            n_3 = n_3.wrapping_add(1)
        }
        *maxSymbolValuePtr = nbSymbols.wrapping_sub(1i32 as libc::c_uint);
        return readSize
    };
}
/* * HUF_getNbBits() :
 *  Read nbBits from CTable symbolTable, for symbol `symbolValue` presumed <= HUF_SYMBOLVALUE_MAX
 *  Note 1 : is not inlined, as HUF_CElt definition is private
 *  Note 2 : const void* used, so that it can provide a statically allocated table as argument (which uses type U32) */
#[no_mangle]
pub unsafe extern "C" fn HUF_getNbBits(mut symbolTable: *const libc::c_void,
                                       mut symbolValue: U32) -> U32 {
    let mut table: *const HUF_CElt = symbolTable as *const HUF_CElt;
    return (*table.offset(symbolValue as isize)).nbBits as U32;
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compress1X(mut dst: *mut libc::c_void,
                                        mut dstSize: size_t,
                                        mut src: *const libc::c_void,
                                        mut srcSize: size_t,
                                        mut maxSymbolValue: libc::c_uint,
                                        mut huffLog: libc::c_uint) -> size_t {
    let mut workSpace: [libc::c_uint; 1536] = [0; 1536];
    return HUF_compress1X_wksp(dst, dstSize, src, srcSize, maxSymbolValue,
                               huffLog,
                               workSpace.as_mut_ptr() as *mut libc::c_void,
                               ::std::mem::size_of::<[libc::c_uint; 1536]>()
                                   as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compress1X_wksp(mut dst: *mut libc::c_void,
                                             mut dstSize: size_t,
                                             mut src: *const libc::c_void,
                                             mut srcSize: size_t,
                                             mut maxSymbolValue: libc::c_uint,
                                             mut huffLog: libc::c_uint,
                                             mut workSpace: *mut libc::c_void,
                                             mut wkspSize: size_t) -> size_t {
    return HUF_compress_internal(dst, dstSize, src, srcSize, maxSymbolValue,
                                 huffLog, 1i32 as libc::c_uint, workSpace,
                                 wkspSize, 0 as *mut HUF_CElt,
                                 0 as *mut HUF_repeat, 0i32, 0i32);
}
/* *< `workSpace` must be a table of at least HUF_WORKSPACE_SIZE_U32 unsigned */
#[no_mangle]
pub unsafe extern "C" fn HUF_compress1X_usingCTable(mut dst:
                                                        *mut libc::c_void,
                                                    mut dstSize: size_t,
                                                    mut src:
                                                        *const libc::c_void,
                                                    mut srcSize: size_t,
                                                    mut CTable:
                                                        *const HUF_CElt)
 -> size_t {
    return HUF_compress1X_usingCTable_internal(dst, dstSize, src, srcSize,
                                               CTable, 0i32);
}
/* * HUF_compress1X_repeat() :
 *  Same as HUF_compress1X_wksp(), but considers using hufTable if *repeat != HUF_repeat_none.
 *  If it uses hufTable it does not modify hufTable or repeat.
 *  If it doesn't, it sets *repeat = HUF_repeat_none, and it sets hufTable to the table used.
 *  If preferRepeat then the old table will always be used if valid. */
#[no_mangle]
pub unsafe extern "C" fn HUF_compress1X_repeat(mut dst: *mut libc::c_void,
                                               mut dstSize: size_t,
                                               mut src: *const libc::c_void,
                                               mut srcSize: size_t,
                                               mut maxSymbolValue:
                                                   libc::c_uint,
                                               mut huffLog: libc::c_uint,
                                               mut workSpace:
                                                   *mut libc::c_void,
                                               mut wkspSize: size_t,
                                               mut hufTable: *mut HUF_CElt,
                                               mut repeat: *mut HUF_repeat,
                                               mut preferRepeat: libc::c_int,
                                               mut bmi2: libc::c_int)
 -> size_t {
    return HUF_compress_internal(dst, dstSize, src, srcSize, maxSymbolValue,
                                 huffLog, 1i32 as libc::c_uint, workSpace,
                                 wkspSize, hufTable, repeat, preferRepeat,
                                 bmi2);
}
