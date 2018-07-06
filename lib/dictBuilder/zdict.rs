#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( extern_types , libc , offset_to )]
extern crate libc;
extern "C" {
    pub type _IO_FILE_plus;
    pub type ZSTD_CCtx_s;
    pub type HUF_CElt_s;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
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
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    static mut __tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut __daylight: libc::c_int;
    #[no_mangle]
    static mut __timezone: libc::c_long;
    #[no_mangle]
    fn FSE_isError(code: size_t) -> libc::c_uint;
    /* ! Constructor and Destructor of FSE_DTable.
    Note that its size depends on 'tableLog' */
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
    /* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
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
    /* *< maximum compressed size (worst case) */
    #[no_mangle]
    fn HUF_isError(code: size_t) -> libc::c_uint;
    #[no_mangle]
    fn HUF_buildCTable(CTable: *mut HUF_CElt, count: *const libc::c_uint,
                       maxSymbolValue: libc::c_uint, maxNbBits: libc::c_uint)
     -> size_t;
    #[no_mangle]
    fn HUF_writeCTable(dst: *mut libc::c_void, maxDstSize: size_t,
                       CTable: *const HUF_CElt, maxSymbolValue: libc::c_uint,
                       huffLog: libc::c_uint) -> size_t;
    #[no_mangle]
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
    #[no_mangle]
    fn ZSTD_isError(code: size_t) -> libc::c_uint;
    #[no_mangle]
    fn ZSTD_getErrorName(code: size_t) -> *const libc::c_char;
    #[no_mangle]
    fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
    #[no_mangle]
    fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> size_t;
    #[no_mangle]
    fn ZSTD_getParams(compressionLevel: libc::c_int,
                      estimatedSrcSize: libc::c_ulonglong, dictSize: size_t)
     -> ZSTD_parameters;
    #[no_mangle]
    fn ZSTD_compressBegin_advanced(cctx: *mut ZSTD_CCtx,
                                   dict: *const libc::c_void,
                                   dictSize: size_t, params: ZSTD_parameters,
                                   pledgedSrcSize: libc::c_ulonglong)
     -> size_t;
    #[no_mangle]
    fn ZSTD_copyCCtx(cctx: *mut ZSTD_CCtx, preparedCCtx: *const ZSTD_CCtx,
                     pledgedSrcSize: libc::c_ulonglong) -> size_t;
    #[no_mangle]
    fn ZSTD_compressBlock(cctx: *mut ZSTD_CCtx, dst: *mut libc::c_void,
                          dstCapacity: size_t, src: *const libc::c_void,
                          srcSize: size_t) -> size_t;
    #[no_mangle]
    fn ZSTD_XXH64(input: *const libc::c_void, length: size_t,
                  seed: libc::c_ulonglong) -> XXH64_hash_t;
    #[no_mangle]
    fn ZSTD_getSeqStore(ctx: *const ZSTD_CCtx) -> *const seqStore_t;
    #[no_mangle]
    fn ZSTD_seqToCodes(seqStorePtr: *const seqStore_t) -> ();
    /* ***************************************************************************************
 * START OF ADVANCED AND EXPERIMENTAL FUNCTIONS
 * The definitions in this section are considered experimental.
 * They should never be used with a dynamic library, as prototypes may change in the future.
 * They are provided for advanced scenarios.
 * Use them only in association with static linking.
 * ***************************************************************************************/
    /* !< recommended size for output buffer. Guarantee to successfully flush at least one complete block in all circumstances. */
    /* !< recommended size for input buffer */
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
    /* *
 * Constructs the suffix array of a given string.
 * @param T [0..n-1] The input string.
 * @param SA [0..n-1] The output array of suffixes.
 * @param n The length of the given string.
 * @param openMP enables OpenMP optimization.
 * @return 0 if no error occurred, -1 or -2 otherwise.
 */
    #[no_mangle]
    fn divsufsort(T: *const libc::c_uchar, SA: *mut libc::c_int,
                  n: libc::c_int, openMP: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ZDICT_optimizeTrainFromBuffer_cover(dictBuffer: *mut libc::c_void,
                                           dictBufferCapacity: size_t,
                                           samplesBuffer: *const libc::c_void,
                                           samplesSizes: *const size_t,
                                           nbSamples: libc::c_uint,
                                           parameters:
                                               *mut ZDICT_cover_params_t)
     -> size_t;
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct dictItem {
    pub pos: U32,
    pub length: U32,
    pub savings: U32,
}
pub const ZSTD_error_checksum_wrong: ERR_enum = 22;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
pub const ZSTD_error_frameIndex_tooLarge: ERR_enum = 100;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub const ZSTD_error_maxCode: ERR_enum = 120;
pub const ZSTD_error_workSpace_tooSmall: ERR_enum = 66;
pub const ZSTD_error_init_missing: ERR_enum = 62;
pub const ZSTD_error_stage_wrong: ERR_enum = 60;
pub const ZSTD_error_srcSize_wrong: ERR_enum = 72;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
pub type ERR_enum = libc::c_uint;
pub type clock_t = __clock_t;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
pub type FILE = _IO_FILE;
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
pub type __clock_t = libc::c_long;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub const ZSTD_error_corruption_detected: ERR_enum = 20;
pub type unnamed = libc::c_uint;
pub type FSE_DTable = libc::c_uint;
pub type U32 = uint32_t;
pub const ZSTD_error_seekableIO: ERR_enum = 102;
pub type uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DState_t {
    pub state: size_t,
    pub table: *const libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: libc::c_uint,
    pub ptr: *const libc::c_char,
    pub start: *const libc::c_char,
    pub limitPtr: *const libc::c_char,
}
pub const ZSTD_btopt: ZSTD_strategy = 7;
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
pub const ZSTD_error_no_error: ERR_enum = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZDICT_params_t {
    pub compressionLevel: libc::c_int,
    pub notificationLevel: libc::c_uint,
    pub dictID: libc::c_uint,
}
pub type ZSTD_strategy = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
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
pub const MEM_static_assert: unnamed = 1;
pub type BYTE = uint8_t;
pub type seqDef = seqDef_s;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZDICT_legacy_params_t {
    pub selectivityLevel: libc::c_uint,
    pub zParams: ZDICT_params_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    u: U32,
    c: [BYTE; 4],
}
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_frameParameters {
    pub contentSizeFlag: libc::c_uint,
    pub checksumFlag: libc::c_uint,
    pub noDictIDFlag: libc::c_uint,
}
pub const ZSTD_error_dstSize_tooSmall: ERR_enum = 70;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZDICT_cover_params_t {
    pub k: libc::c_uint,
    pub d: libc::c_uint,
    pub steps: libc::c_uint,
    pub nbThreads: libc::c_uint,
    pub zParams: ZDICT_params_t,
}
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_CStream_t {
    pub bitContainer: size_t,
    pub bitPos: libc::c_uint,
    pub startPtr: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub endPtr: *mut libc::c_char,
}
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct seqDef_s {
    pub offset: U32,
    pub litLength: U16,
    pub matchLength: U16,
}
pub const ZSTD_error_parameter_unsupported: ERR_enum = 40;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
pub type BIT_DStream_status = libc::c_uint;
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type XXH64_hash_t = libc::c_ulonglong;
pub const ZSTD_error_prefix_unknown: ERR_enum = 10;
pub type ptrdiff_t = libc::c_long;
pub const ZSTD_error_GENERIC: ERR_enum = 1;
pub const ZSTD_error_version_unsupported: ERR_enum = 12;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const ZSTD_fast: ZSTD_strategy = 1;
pub const ZSTD_error_frameParameter_windowTooLarge: ERR_enum = 16;
pub type unnamed_1 = libc::c_uint;
pub type S16 = int16_t;
pub const ZSTD_error_memory_allocation: ERR_enum = 64;
pub const MEM_static_assert_0: unnamed_1 = 1;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
pub type uint16_t = libc::c_ushort;
pub const ZSTD_error_parameter_outOfBound: ERR_enum = 42;
pub const ZSTD_error_dictionary_wrong: ERR_enum = 32;
pub type _IO_lock_t = ();
pub const ZSTD_error_maxSymbolValue_tooSmall: ERR_enum = 48;
pub type HUF_CElt = HUF_CElt_s;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const libc::c_void,
    pub symbolTT: *const libc::c_void,
    pub stateLog: libc::c_uint,
}
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub type size_t = libc::c_ulong;
pub const ZSTD_error_frameParameter_unsupported: ERR_enum = 14;
pub type uint8_t = libc::c_uchar;
pub const ZSTD_lazy: ZSTD_strategy = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct offsetCount_t {
    pub offset: U32,
    pub count: U32,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct EStats_ress_t {
    pub ref_0: *mut ZSTD_CCtx,
    pub zc: *mut ZSTD_CCtx,
    pub workPlace: *mut libc::c_void,
}
pub type ZSTD_ErrorCode = ERR_enum;
pub type int16_t = libc::c_short;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const ZSTD_error_dictionary_corrupted: ERR_enum = 30;
pub type FSE_CTable = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_parameters {
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
}
pub type U16 = uint16_t;
pub const ZSTD_error_dictionaryCreation_failed: ERR_enum = 34;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: libc::c_int,
    pub deltaNbBits: U32,
}
pub type U64 = uint64_t;
pub const ZSTD_error_tableLog_tooLarge: ERR_enum = 44;
pub const ZSTD_error_maxSymbolValue_tooLarge: ERR_enum = 46;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub type uint32_t = libc::c_uint;
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
                    current_block = 17864142944814747212;
                }
                6 => { current_block = 17864142944814747212; }
                5 => { current_block = 1829890564992148703; }
                4 => { current_block = 16788051986517518186; }
                3 => { current_block = 272691948937247091; }
                2 => { current_block = 2612814309992382393; }
                _ => { current_block = 10886091980245723256; }
            }
            match current_block {
                17864142944814747212 => {
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
                    current_block = 1829890564992148703;
                }
                _ => { }
            }
            match current_block {
                1829890564992148703 => {
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
                    current_block = 16788051986517518186;
                }
                _ => { }
            }
            match current_block {
                16788051986517518186 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(3isize)
                                                              as size_t) <<
                                                             24i32) as size_t
                            as size_t;
                    current_block = 272691948937247091;
                }
                _ => { }
            }
            match current_block {
                272691948937247091 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(2isize)
                                                              as size_t) <<
                                                             16i32) as size_t
                            as size_t;
                    current_block = 2612814309992382393;
                }
                _ => { }
            }
            match current_block {
                2612814309992382393 => {
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
pub unsafe extern "C" fn ZDICT_trainFromBuffer(mut dictBuffer:
                                                   *mut libc::c_void,
                                               mut dictBufferCapacity: size_t,
                                               mut samplesBuffer:
                                                   *const libc::c_void,
                                               mut samplesSizes:
                                                   *const size_t,
                                               mut nbSamples: libc::c_uint)
 -> size_t {
    let mut params: ZDICT_cover_params_t =
        ZDICT_cover_params_t{k: 0,
                             d: 0,
                             steps: 0,
                             nbThreads: 0,
                             zParams:
                                 ZDICT_params_t{compressionLevel: 0,
                                                notificationLevel: 0,
                                                dictID: 0,},};
    memset(&mut params as *mut ZDICT_cover_params_t as *mut libc::c_void,
           0i32,
           ::std::mem::size_of::<ZDICT_cover_params_t>() as libc::c_ulong);
    params.d = 8i32 as libc::c_uint;
    params.steps = 4i32 as libc::c_uint;
    params.zParams.compressionLevel = 6i32;
    return ZDICT_optimizeTrainFromBuffer_cover(dictBuffer, dictBufferCapacity,
                                               samplesBuffer, samplesSizes,
                                               nbSamples,
                                               &mut params as
                                                   *mut ZDICT_cover_params_t);
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_getDictID(mut dictBuffer: *const libc::c_void,
                                         mut dictSize: size_t)
 -> libc::c_uint {
    if dictSize < 8i32 as libc::c_ulong {
        return 0i32 as libc::c_uint
    } else if MEM_readLE32(dictBuffer) != 3962610743u32 {
        return 0i32 as libc::c_uint
    } else {
        return MEM_readLE32((dictBuffer as *const libc::c_char).offset(4isize)
                                as *const libc::c_void)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_isError(mut errorCode: size_t)
 -> libc::c_uint {
    return ERR_isError(errorCode);
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_getErrorName(mut errorCode: size_t)
 -> *const libc::c_char {
    return ERR_getErrorName(errorCode);
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_finalizeDictionary(mut dictBuffer:
                                                      *mut libc::c_void,
                                                  mut dictBufferCapacity:
                                                      size_t,
                                                  mut customDictContent:
                                                      *const libc::c_void,
                                                  mut dictContentSize: size_t,
                                                  mut samplesBuffer:
                                                      *const libc::c_void,
                                                  mut samplesSizes:
                                                      *const size_t,
                                                  mut nbSamples: libc::c_uint,
                                                  mut params: ZDICT_params_t)
 -> size_t {
    let mut hSize: size_t = 0;
    let mut header: [BYTE; 256] = [0; 256];
    let compressionLevel: libc::c_int =
        if params.compressionLevel == 0i32 {
            g_compressionLevel_default
        } else { params.compressionLevel };
    let notificationLevel: U32 = params.notificationLevel;
    if dictBufferCapacity < dictContentSize {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else if dictContentSize < 128i32 as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else if dictBufferCapacity < 256i32 as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else {
        MEM_writeLE32(header.as_mut_ptr() as *mut libc::c_void,
                      3962610743u32);
        let randomID: U64 =
            ZSTD_XXH64(customDictContent, dictContentSize,
                       0i32 as libc::c_ulonglong) as U64;
        let compliantID: U32 =
            randomID.wrapping_rem((1u32 <<
                                       31i32).wrapping_sub(32768i32 as
                                                               libc::c_uint)
                                      as
                                      libc::c_ulong).wrapping_add(32768i32 as
                                                                      libc::c_ulong)
                as U32;
        let dictID: U32 =
            if 0 != params.dictID { params.dictID } else { compliantID };
        MEM_writeLE32(header.as_mut_ptr().offset(4isize) as *mut libc::c_void,
                      dictID);
        hSize = 8i32 as size_t;
        if notificationLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 7],
                                              &mut [libc::c_char; 7]>(b"\r%70s\r\x00")).as_mut_ptr(),
                    (*::std::mem::transmute::<&[u8; 1],
                                              &mut [libc::c_char; 1]>(b"\x00")).as_mut_ptr());
            fflush(stderr);
        }
        if notificationLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 17],
                                              &mut [libc::c_char; 17]>(b"statistics ... \n\x00")).as_mut_ptr());
            fflush(stderr);
        }
        let eSize: size_t =
            ZDICT_analyzeEntropy(header.as_mut_ptr().offset(hSize as isize) as
                                     *mut libc::c_void,
                                 (256i32 as
                                      libc::c_ulong).wrapping_sub(hSize),
                                 compressionLevel as libc::c_uint,
                                 samplesBuffer, samplesSizes, nbSamples,
                                 customDictContent, dictContentSize,
                                 notificationLevel);
        if 0 != ZDICT_isError(eSize) {
            return eSize
        } else {
            hSize =
                (hSize as libc::c_ulong).wrapping_add(eSize) as size_t as
                    size_t;
            if hSize.wrapping_add(dictContentSize) > dictBufferCapacity {
                dictContentSize = dictBufferCapacity.wrapping_sub(hSize)
            }
            let dictSize: size_t = hSize.wrapping_add(dictContentSize);
            let mut dictEnd: *mut libc::c_char =
                (dictBuffer as *mut libc::c_char).offset(dictSize as isize);
            memmove(dictEnd.offset(-(dictContentSize as isize)) as
                        *mut libc::c_void, customDictContent,
                    dictContentSize);
            memcpy(dictBuffer, header.as_mut_ptr() as *const libc::c_void,
                   hSize);
            return dictSize
        }
    };
}
static mut g_compressionLevel_default: libc::c_int = unsafe { 3i32 };
unsafe extern "C" fn ZDICT_analyzeEntropy(mut dstBuffer: *mut libc::c_void,
                                          mut maxDstSize: size_t,
                                          mut compressionLevel: libc::c_uint,
                                          mut srcBuffer: *const libc::c_void,
                                          mut fileSizes: *const size_t,
                                          mut nbFiles: libc::c_uint,
                                          mut dictBuffer: *const libc::c_void,
                                          mut dictBufferSize: size_t,
                                          mut notificationLevel: libc::c_uint)
 -> size_t {
    let mut countLit: [U32; 256] = [0; 256];
    let mut hufTablehb: [U32; 256] = [0; 256];
    let mut hufTablehv: *mut libc::c_void =
        &mut hufTablehb as *mut [U32; 256] as *mut libc::c_void;
    let mut hufTable: *mut HUF_CElt = hufTablehv as *mut HUF_CElt;
    let mut offcodeCount: [U32; 31] = [0; 31];
    let mut offcodeNCount: [libc::c_short; 31] = [0; 31];
    let mut offcodeMax: U32 =
        ZSTD_highbit32(dictBufferSize.wrapping_add((128i32 * (1i32 << 10i32))
                                                       as libc::c_ulong) as
                           U32);
    let mut matchLengthCount: [U32; 53] = [0; 53];
    let mut matchLengthNCount: [libc::c_short; 53] = [0; 53];
    let mut litLengthCount: [U32; 36] = [0; 36];
    let mut litLengthNCount: [libc::c_short; 36] = [0; 36];
    let mut repOffset: [U32; 1024] = [0; 1024];
    let mut bestRepOffset: [offsetCount_t; 4] =
        [offsetCount_t{offset: 0, count: 0,}; 4];
    let mut esr: EStats_ress_t =
        EStats_ress_t{ref_0: 0 as *mut ZSTD_CCtx,
                      zc: 0 as *mut ZSTD_CCtx,
                      workPlace: 0 as *mut libc::c_void,};
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
    let mut u: U32 = 0;
    let mut huffLog: U32 = 11i32 as U32;
    let mut Offlog: U32 = 8i32 as U32;
    let mut mlLog: U32 = 9i32 as U32;
    let mut llLog: U32 = 9i32 as U32;
    let mut total: U32 = 0;
    let mut pos: size_t = 0i32 as size_t;
    let mut errorCode: size_t = 0;
    let mut eSize: size_t = 0i32 as size_t;
    let totalSrcSize: size_t = ZDICT_totalSampleSize(fileSizes, nbFiles);
    let averageSampleSize: size_t =
        totalSrcSize.wrapping_div(nbFiles.wrapping_add((0 == nbFiles) as
                                                           libc::c_int as
                                                           libc::c_uint) as
                                      libc::c_ulong);
    let mut dstPtr: *mut BYTE = dstBuffer as *mut BYTE;
    esr.ref_0 = ZSTD_createCCtx();
    esr.zc = ZSTD_createCCtx();
    esr.workPlace = malloc((1i32 << 17i32) as libc::c_ulong);
    if esr.ref_0.is_null() || esr.zc.is_null() || esr.workPlace.is_null() {
        eSize = -(ZSTD_error_memory_allocation as libc::c_int) as size_t;
        if notificationLevel >= 1i32 as libc::c_uint {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 20],
                                              &mut [libc::c_char; 20]>(b"Not enough memory \n\x00")).as_mut_ptr());
            fflush(stderr);
        }
    } else if offcodeMax > 30i32 as libc::c_uint {
        eSize =
            -(ZSTD_error_dictionaryCreation_failed as libc::c_int) as size_t
    } else {
        u = 0i32 as U32;
        while u < 256i32 as libc::c_uint {
            countLit[u as usize] = 1i32 as U32;
            u = u.wrapping_add(1)
        }
        u = 0i32 as U32;
        while u <= offcodeMax {
            offcodeCount[u as usize] = 1i32 as U32;
            u = u.wrapping_add(1)
        }
        u = 0i32 as U32;
        while u <= 52i32 as libc::c_uint {
            matchLengthCount[u as usize] = 1i32 as U32;
            u = u.wrapping_add(1)
        }
        u = 0i32 as U32;
        while u <= 35i32 as libc::c_uint {
            litLengthCount[u as usize] = 1i32 as U32;
            u = u.wrapping_add(1)
        }
        memset(repOffset.as_mut_ptr() as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<[U32; 1024]>() as libc::c_ulong);
        repOffset[8usize] = 1i32 as U32;
        repOffset[4usize] = repOffset[8usize];
        repOffset[1usize] = repOffset[4usize];
        memset(bestRepOffset.as_mut_ptr() as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<[offsetCount_t; 4]>() as libc::c_ulong);
        if compressionLevel == 0i32 as libc::c_uint {
            compressionLevel = g_compressionLevel_default as libc::c_uint
        }
        params =
            ZSTD_getParams(compressionLevel as libc::c_int,
                           averageSampleSize as libc::c_ulonglong,
                           dictBufferSize);
        let beginResult: size_t =
            ZSTD_compressBegin_advanced(esr.ref_0, dictBuffer, dictBufferSize,
                                        params, 0i32 as libc::c_ulonglong);
        if 0 != ZSTD_isError(beginResult) {
            if notificationLevel >= 1i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 52],
                                                  &mut [libc::c_char; 52]>(b"error : ZSTD_compressBegin_advanced() failed : %s \n\x00")).as_mut_ptr(),
                        ZSTD_getErrorName(beginResult));
                fflush(stderr);
            }
            eSize = -(ZSTD_error_GENERIC as libc::c_int) as size_t
        } else {
            u = 0i32 as U32;
            while u < nbFiles {
                ZDICT_countEStats(esr, params, countLit.as_mut_ptr(),
                                  offcodeCount.as_mut_ptr(),
                                  matchLengthCount.as_mut_ptr(),
                                  litLengthCount.as_mut_ptr(),
                                  repOffset.as_mut_ptr(),
                                  (srcBuffer as
                                       *const libc::c_char).offset(pos as
                                                                       isize)
                                      as *const libc::c_void,
                                  *fileSizes.offset(u as isize),
                                  notificationLevel);
                pos =
                    (pos as
                         libc::c_ulong).wrapping_add(*fileSizes.offset(u as
                                                                           isize))
                        as size_t as size_t;
                u = u.wrapping_add(1)
            }
            let mut maxNbBits: size_t =
                HUF_buildCTable(hufTable, countLit.as_mut_ptr(),
                                255i32 as libc::c_uint, huffLog);
            if 0 != HUF_isError(maxNbBits) {
                eSize = -(ZSTD_error_GENERIC as libc::c_int) as size_t;
                if notificationLevel >= 1i32 as libc::c_uint {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 25],
                                                      &mut [libc::c_char; 25]>(b" HUF_buildCTable error \n\x00")).as_mut_ptr());
                    fflush(stderr);
                }
            } else {
                if maxNbBits == 8i32 as libc::c_ulong {
                    if notificationLevel >= 2i32 as libc::c_uint {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 100],
                                                          &mut [libc::c_char; 100]>(b"warning : pathological dataset : literals are not compressible : samples are noisy or too regular \n\x00")).as_mut_ptr());
                        fflush(stderr);
                    }
                    ZDICT_flatLit(countLit.as_mut_ptr());
                    maxNbBits =
                        HUF_buildCTable(hufTable, countLit.as_mut_ptr(),
                                        255i32 as libc::c_uint, huffLog)
                }
                huffLog = maxNbBits as U32;
                let mut offset: U32 = 0;
                offset = 1i32 as U32;
                while offset < 1024i32 as libc::c_uint {
                    ZDICT_insertSortCount(bestRepOffset.as_mut_ptr(), offset,
                                          repOffset[offset as usize]);
                    offset = offset.wrapping_add(1)
                }
                total = 0i32 as U32;
                u = 0i32 as U32;
                while u <= offcodeMax {
                    total =
                        (total as
                             libc::c_uint).wrapping_add(offcodeCount[u as
                                                                         usize])
                            as U32 as U32;
                    u = u.wrapping_add(1)
                }
                errorCode =
                    FSE_normalizeCount(offcodeNCount.as_mut_ptr(), Offlog,
                                       offcodeCount.as_mut_ptr(),
                                       total as size_t, offcodeMax);
                if 0 != FSE_isError(errorCode) {
                    eSize = -(ZSTD_error_GENERIC as libc::c_int) as size_t;
                    if notificationLevel >= 1i32 as libc::c_uint {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 45],
                                                          &mut [libc::c_char; 45]>(b"FSE_normalizeCount error with offcodeCount \n\x00")).as_mut_ptr());
                        fflush(stderr);
                    }
                } else {
                    Offlog = errorCode as U32;
                    total = 0i32 as U32;
                    u = 0i32 as U32;
                    while u <= 52i32 as libc::c_uint {
                        total =
                            (total as
                                 libc::c_uint).wrapping_add(matchLengthCount[u
                                                                                 as
                                                                                 usize])
                                as U32 as U32;
                        u = u.wrapping_add(1)
                    }
                    errorCode =
                        FSE_normalizeCount(matchLengthNCount.as_mut_ptr(),
                                           mlLog,
                                           matchLengthCount.as_mut_ptr(),
                                           total as size_t,
                                           52i32 as libc::c_uint);
                    if 0 != FSE_isError(errorCode) {
                        eSize =
                            -(ZSTD_error_GENERIC as libc::c_int) as size_t;
                        if notificationLevel >= 1i32 as libc::c_uint {
                            fprintf(stderr,
                                    (*::std::mem::transmute::<&[u8; 49],
                                                              &mut [libc::c_char; 49]>(b"FSE_normalizeCount error with matchLengthCount \n\x00")).as_mut_ptr());
                            fflush(stderr);
                        }
                    } else {
                        mlLog = errorCode as U32;
                        total = 0i32 as U32;
                        u = 0i32 as U32;
                        while u <= 35i32 as libc::c_uint {
                            total =
                                (total as
                                     libc::c_uint).wrapping_add(litLengthCount[u
                                                                                   as
                                                                                   usize])
                                    as U32 as U32;
                            u = u.wrapping_add(1)
                        }
                        errorCode =
                            FSE_normalizeCount(litLengthNCount.as_mut_ptr(),
                                               llLog,
                                               litLengthCount.as_mut_ptr(),
                                               total as size_t,
                                               35i32 as libc::c_uint);
                        if 0 != FSE_isError(errorCode) {
                            eSize =
                                -(ZSTD_error_GENERIC as libc::c_int) as
                                    size_t;
                            if notificationLevel >= 1i32 as libc::c_uint {
                                fprintf(stderr,
                                        (*::std::mem::transmute::<&[u8; 47],
                                                                  &mut [libc::c_char; 47]>(b"FSE_normalizeCount error with litLengthCount \n\x00")).as_mut_ptr());
                                fflush(stderr);
                            }
                        } else {
                            llLog = errorCode as U32;
                            let hhSize: size_t =
                                HUF_writeCTable(dstPtr as *mut libc::c_void,
                                                maxDstSize, hufTable,
                                                255i32 as libc::c_uint,
                                                huffLog);
                            if 0 != HUF_isError(hhSize) {
                                eSize =
                                    -(ZSTD_error_GENERIC as libc::c_int) as
                                        size_t;
                                if notificationLevel >= 1i32 as libc::c_uint {
                                    fprintf(stderr,
                                            (*::std::mem::transmute::<&[u8; 24],
                                                                      &mut [libc::c_char; 24]>(b"HUF_writeCTable error \n\x00")).as_mut_ptr());
                                    fflush(stderr);
                                }
                            } else {
                                dstPtr = dstPtr.offset(hhSize as isize);
                                maxDstSize =
                                    (maxDstSize as
                                         libc::c_ulong).wrapping_sub(hhSize)
                                        as size_t as size_t;
                                eSize =
                                    (eSize as
                                         libc::c_ulong).wrapping_add(hhSize)
                                        as size_t as size_t;
                                let ohSize: size_t =
                                    FSE_writeNCount(dstPtr as
                                                        *mut libc::c_void,
                                                    maxDstSize,
                                                    offcodeNCount.as_mut_ptr(),
                                                    30i32 as libc::c_uint,
                                                    Offlog);
                                if 0 != FSE_isError(ohSize) {
                                    eSize =
                                        -(ZSTD_error_GENERIC as libc::c_int)
                                            as size_t;
                                    if notificationLevel >=
                                           1i32 as libc::c_uint {
                                        fprintf(stderr,
                                                (*::std::mem::transmute::<&[u8; 43],
                                                                          &mut [libc::c_char; 43]>(b"FSE_writeNCount error with offcodeNCount \n\x00")).as_mut_ptr());
                                        fflush(stderr);
                                    }
                                } else {
                                    dstPtr = dstPtr.offset(ohSize as isize);
                                    maxDstSize =
                                        (maxDstSize as
                                             libc::c_ulong).wrapping_sub(ohSize)
                                            as size_t as size_t;
                                    eSize =
                                        (eSize as
                                             libc::c_ulong).wrapping_add(ohSize)
                                            as size_t as size_t;
                                    let mhSize: size_t =
                                        FSE_writeNCount(dstPtr as
                                                            *mut libc::c_void,
                                                        maxDstSize,
                                                        matchLengthNCount.as_mut_ptr(),
                                                        52i32 as libc::c_uint,
                                                        mlLog);
                                    if 0 != FSE_isError(mhSize) {
                                        eSize =
                                            -(ZSTD_error_GENERIC as
                                                  libc::c_int) as size_t;
                                        if notificationLevel >=
                                               1i32 as libc::c_uint {
                                            fprintf(stderr,
                                                    (*::std::mem::transmute::<&[u8; 47],
                                                                              &mut [libc::c_char; 47]>(b"FSE_writeNCount error with matchLengthNCount \n\x00")).as_mut_ptr());
                                            fflush(stderr);
                                        }
                                    } else {
                                        dstPtr =
                                            dstPtr.offset(mhSize as isize);
                                        maxDstSize =
                                            (maxDstSize as
                                                 libc::c_ulong).wrapping_sub(mhSize)
                                                as size_t as size_t;
                                        eSize =
                                            (eSize as
                                                 libc::c_ulong).wrapping_add(mhSize)
                                                as size_t as size_t;
                                        let lhSize: size_t =
                                            FSE_writeNCount(dstPtr as
                                                                *mut libc::c_void,
                                                            maxDstSize,
                                                            litLengthNCount.as_mut_ptr(),
                                                            35i32 as
                                                                libc::c_uint,
                                                            llLog);
                                        if 0 != FSE_isError(lhSize) {
                                            eSize =
                                                -(ZSTD_error_GENERIC as
                                                      libc::c_int) as size_t;
                                            if notificationLevel >=
                                                   1i32 as libc::c_uint {
                                                fprintf(stderr,
                                                        (*::std::mem::transmute::<&[u8; 45],
                                                                                  &mut [libc::c_char; 45]>(b"FSE_writeNCount error with litlengthNCount \n\x00")).as_mut_ptr());
                                                fflush(stderr);
                                            }
                                        } else {
                                            dstPtr =
                                                dstPtr.offset(lhSize as
                                                                  isize);
                                            maxDstSize =
                                                (maxDstSize as
                                                     libc::c_ulong).wrapping_sub(lhSize)
                                                    as size_t as size_t;
                                            eSize =
                                                (eSize as
                                                     libc::c_ulong).wrapping_add(lhSize)
                                                    as size_t as size_t;
                                            if maxDstSize <
                                                   12i32 as libc::c_ulong {
                                                eSize =
                                                    -(ZSTD_error_GENERIC as
                                                          libc::c_int) as
                                                        size_t;
                                                if notificationLevel >=
                                                       1i32 as libc::c_uint {
                                                    fprintf(stderr,
                                                            (*::std::mem::transmute::<&[u8; 39],
                                                                                      &mut [libc::c_char; 39]>(b"not enough space to write RepOffsets \n\x00")).as_mut_ptr());
                                                    fflush(stderr);
                                                }
                                            } else {
                                                MEM_writeLE32(dstPtr.offset(0isize)
                                                                  as
                                                                  *mut libc::c_void,
                                                              repStartValue[0usize]);
                                                MEM_writeLE32(dstPtr.offset(4isize)
                                                                  as
                                                                  *mut libc::c_void,
                                                              repStartValue[1usize]);
                                                MEM_writeLE32(dstPtr.offset(8isize)
                                                                  as
                                                                  *mut libc::c_void,
                                                              repStartValue[2usize]);
                                                eSize =
                                                    (eSize as
                                                         libc::c_ulong).wrapping_add(12i32
                                                                                         as
                                                                                         libc::c_ulong)
                                                        as size_t as size_t
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
    ZSTD_freeCCtx(esr.ref_0);
    ZSTD_freeCCtx(esr.zc);
    free(esr.workPlace);
    return eSize;
}
unsafe extern "C" fn ZDICT_insertSortCount(mut table: *mut offsetCount_t,
                                           mut val: U32, mut count: U32)
 -> () {
    let mut u: U32 = 0;
    (*table.offset(3isize)).offset = val;
    (*table.offset(3isize)).count = count;
    u = 3i32 as U32;
    while u > 0i32 as libc::c_uint {
        let mut tmp: offsetCount_t = offsetCount_t{offset: 0, count: 0,};
        if (*table.offset(u.wrapping_sub(1i32 as libc::c_uint) as
                              isize)).count >=
               (*table.offset(u as isize)).count {
            break ;
        }
        tmp = *table.offset(u.wrapping_sub(1i32 as libc::c_uint) as isize);
        *table.offset(u.wrapping_sub(1i32 as libc::c_uint) as isize) =
            *table.offset(u as isize);
        *table.offset(u as isize) = tmp;
        u = u.wrapping_sub(1)
    };
}
unsafe extern "C" fn ZDICT_flatLit(mut countLit: *mut U32) -> () {
    let mut u: libc::c_int = 0;
    u = 1i32;
    while u < 256i32 { *countLit.offset(u as isize) = 2i32 as U32; u += 1 }
    *countLit.offset(0isize) = 4i32 as U32;
    *countLit.offset(253isize) = 1i32 as U32;
    *countLit.offset(254isize) = 1i32 as U32;
}
unsafe extern "C" fn ZDICT_countEStats(mut esr: EStats_ress_t,
                                       mut params: ZSTD_parameters,
                                       mut countLit: *mut U32,
                                       mut offsetcodeCount: *mut U32,
                                       mut matchlengthCount: *mut U32,
                                       mut litlengthCount: *mut U32,
                                       mut repOffsets: *mut U32,
                                       mut src: *const libc::c_void,
                                       mut srcSize: size_t,
                                       mut notificationLevel: U32) -> () {
    let blockSizeMax: size_t =
        (if 1i32 << 17i32 < 1i32 << params.cParams.windowLog {
             1i32 << 17i32
         } else { 1i32 << params.cParams.windowLog }) as size_t;
    let mut cSize: size_t = 0;
    if srcSize > blockSizeMax { srcSize = blockSizeMax }
    let errorCode: size_t =
        ZSTD_copyCCtx(esr.zc, esr.ref_0, 0i32 as libc::c_ulonglong);
    if 0 != ZSTD_isError(errorCode) {
        if notificationLevel >= 1i32 as libc::c_uint {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 33],
                                              &mut [libc::c_char; 33]>(b"warning : ZSTD_copyCCtx failed \n\x00")).as_mut_ptr());
            fflush(stderr);
        }
        return
    } else {
        cSize =
            ZSTD_compressBlock(esr.zc, esr.workPlace,
                               (1i32 << 17i32) as size_t, src, srcSize);
        if 0 != ZSTD_isError(cSize) {
            if notificationLevel >= 3i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 46],
                                                  &mut [libc::c_char; 46]>(b"warning : could not compress sample size %u \n\x00")).as_mut_ptr(),
                        srcSize as U32);
                fflush(stderr);
            }
            return
        } else {
            if 0 != cSize {
                let seqStorePtr: *const seqStore_t = ZSTD_getSeqStore(esr.zc);
                let mut bytePtr: *const BYTE = 0 as *const BYTE;
                bytePtr = (*seqStorePtr).litStart;
                while bytePtr < (*seqStorePtr).lit {
                    let ref mut fresh0 = *countLit.offset(*bytePtr as isize);
                    *fresh0 = (*fresh0).wrapping_add(1);
                    bytePtr = bytePtr.offset(1isize)
                }
                let nbSeq: U32 =
                    (*seqStorePtr).sequencesStart.offset_to((*seqStorePtr).sequences).expect("bad offset_to")
                        as libc::c_long as U32;
                ZSTD_seqToCodes(seqStorePtr);
                let mut codePtr: *const BYTE = (*seqStorePtr).ofCode;
                let mut u: U32 = 0;
                u = 0i32 as U32;
                while u < nbSeq {
                    let ref mut fresh1 =
                        *offsetcodeCount.offset(*codePtr.offset(u as isize) as
                                                    isize);
                    *fresh1 = (*fresh1).wrapping_add(1);
                    u = u.wrapping_add(1)
                }
                let mut codePtr_0: *const BYTE = (*seqStorePtr).mlCode;
                let mut u_0: U32 = 0;
                u_0 = 0i32 as U32;
                while u_0 < nbSeq {
                    let ref mut fresh2 =
                        *matchlengthCount.offset(*codePtr_0.offset(u_0 as
                                                                       isize)
                                                     as isize);
                    *fresh2 = (*fresh2).wrapping_add(1);
                    u_0 = u_0.wrapping_add(1)
                }
                let mut codePtr_1: *const BYTE = (*seqStorePtr).llCode;
                let mut u_1: U32 = 0;
                u_1 = 0i32 as U32;
                while u_1 < nbSeq {
                    let ref mut fresh3 =
                        *litlengthCount.offset(*codePtr_1.offset(u_1 as isize)
                                                   as isize);
                    *fresh3 = (*fresh3).wrapping_add(1);
                    u_1 = u_1.wrapping_add(1)
                }
                if nbSeq >= 2i32 as libc::c_uint {
                    let seq: *const seqDef = (*seqStorePtr).sequencesStart;
                    let mut offset1: U32 =
                        (*seq.offset(0isize)).offset.wrapping_sub(3i32 as
                                                                      libc::c_uint);
                    let mut offset2: U32 =
                        (*seq.offset(1isize)).offset.wrapping_sub(3i32 as
                                                                      libc::c_uint);
                    if offset1 >= 1024i32 as libc::c_uint {
                        offset1 = 0i32 as U32
                    }
                    if offset2 >= 1024i32 as libc::c_uint {
                        offset2 = 0i32 as U32
                    }
                    let ref mut fresh4 = *repOffsets.offset(offset1 as isize);
                    *fresh4 =
                        (*fresh4 as
                             libc::c_uint).wrapping_add(3i32 as libc::c_uint)
                            as U32 as U32;
                    let ref mut fresh5 = *repOffsets.offset(offset2 as isize);
                    *fresh5 =
                        (*fresh5 as
                             libc::c_uint).wrapping_add(1i32 as libc::c_uint)
                            as U32 as U32
                }
            }
            return;
        }
    };
}
unsafe extern "C" fn ZDICT_totalSampleSize(mut fileSizes: *const size_t,
                                           mut nbFiles: libc::c_uint)
 -> size_t {
    let mut total: size_t = 0i32 as size_t;
    let mut u: libc::c_uint = 0;
    u = 0i32 as libc::c_uint;
    while u < nbFiles {
        total =
            (total as
                 libc::c_ulong).wrapping_add(*fileSizes.offset(u as isize)) as
                size_t as size_t;
        u = u.wrapping_add(1)
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_legacy(mut dictBuffer:
                                                          *mut libc::c_void,
                                                      mut dictBufferCapacity:
                                                          size_t,
                                                      mut samplesBuffer:
                                                          *const libc::c_void,
                                                      mut samplesSizes:
                                                          *const size_t,
                                                      mut nbSamples:
                                                          libc::c_uint,
                                                      mut params:
                                                          ZDICT_legacy_params_t)
 -> size_t {
    let mut result: size_t = 0;
    let mut newBuff: *mut libc::c_void = 0 as *mut libc::c_void;
    let sBuffSize: size_t = ZDICT_totalSampleSize(samplesSizes, nbSamples);
    if sBuffSize < (128i32 * 4i32) as libc::c_ulong {
        return 0i32 as size_t
    } else {
        newBuff = malloc(sBuffSize.wrapping_add(32i32 as libc::c_ulong));
        if newBuff.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        } else {
            memcpy(newBuff, samplesBuffer, sBuffSize);
            ZDICT_fillNoise((newBuff as
                                 *mut libc::c_char).offset(sBuffSize as isize)
                                as *mut libc::c_void, 32i32 as size_t);
            result =
                ZDICT_trainFromBuffer_unsafe_legacy(dictBuffer,
                                                    dictBufferCapacity,
                                                    newBuff, samplesSizes,
                                                    nbSamples, params);
            free(newBuff);
            return result
        }
    };
}
/* ! ZDICT_trainFromBuffer_unsafe_legacy() :
*   Warning : `samplesBuffer` must be followed by noisy guard band.
*   @return : size of dictionary, or an error code which can be tested with ZDICT_isError()
*/
#[no_mangle]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_unsafe_legacy(mut dictBuffer:
                                                                 *mut libc::c_void,
                                                             mut maxDictSize:
                                                                 size_t,
                                                             mut samplesBuffer:
                                                                 *const libc::c_void,
                                                             mut samplesSizes:
                                                                 *const size_t,
                                                             mut nbSamples:
                                                                 libc::c_uint,
                                                             mut params:
                                                                 ZDICT_legacy_params_t)
 -> size_t {
    let dictListSize: U32 =
        if if 10000i32 as libc::c_uint > nbSamples {
               10000i32 as libc::c_uint
           } else { nbSamples } >
               maxDictSize.wrapping_div(16i32 as libc::c_ulong) as U32 {
            if 10000i32 as libc::c_uint > nbSamples {
                10000i32 as libc::c_uint
            } else { nbSamples }
        } else { maxDictSize.wrapping_div(16i32 as libc::c_ulong) as U32 };
    let dictList: *mut dictItem =
        malloc((dictListSize as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<dictItem>()
                                                    as libc::c_ulong)) as
            *mut dictItem;
    let selectivity: libc::c_uint =
        if params.selectivityLevel == 0i32 as libc::c_uint {
            g_selectivity_default
        } else { params.selectivityLevel };
    let minRep: libc::c_uint =
        if selectivity > 30i32 as libc::c_uint {
            4i32 as libc::c_uint
        } else { nbSamples >> selectivity };
    let targetDictSize: size_t = maxDictSize;
    let samplesBuffSize: size_t =
        ZDICT_totalSampleSize(samplesSizes, nbSamples);
    let mut dictSize: size_t = 0i32 as size_t;
    let notificationLevel: U32 = params.zParams.notificationLevel;
    if dictList.is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    } else if maxDictSize < 256i32 as libc::c_ulong {
        free(dictList as *mut libc::c_void);
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else if samplesBuffSize < (128i32 * 4i32) as libc::c_ulong {
        free(dictList as *mut libc::c_void);
        return -(ZSTD_error_dictionaryCreation_failed as libc::c_int) as
                   size_t
    } else {
        ZDICT_initDictItem(dictList);
        ZDICT_trainBuffer_legacy(dictList, dictListSize, samplesBuffer,
                                 samplesBuffSize, samplesSizes, nbSamples,
                                 minRep, notificationLevel);
        if params.zParams.notificationLevel >= 3i32 as libc::c_uint {
            let nb: U32 =
                if (25i32 as libc::c_uint) < (*dictList.offset(0isize)).pos {
                    25i32 as libc::c_uint
                } else { (*dictList.offset(0isize)).pos };
            let dictContentSize: U32 = ZDICT_dictSize(dictList);
            let mut u: U32 = 0;
            if notificationLevel >= 3i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 40],
                                                  &mut [libc::c_char; 40]>(b"\n %u segments found, of total size %u \n\x00")).as_mut_ptr(),
                        (*dictList.offset(0isize)).pos.wrapping_sub(1i32 as
                                                                        libc::c_uint),
                        dictContentSize);
                fflush(stderr);
            }
            if notificationLevel >= 3i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 24],
                                                  &mut [libc::c_char; 24]>(b"list %u best segments \n\x00")).as_mut_ptr(),
                        nb.wrapping_sub(1i32 as libc::c_uint));
                fflush(stderr);
            }
            u = 1i32 as U32;
            while u < nb {
                let pos: U32 = (*dictList.offset(u as isize)).pos;
                let length: U32 = (*dictList.offset(u as isize)).length;
                let printedLength: U32 =
                    if (40i32 as libc::c_uint) < length {
                        40i32 as libc::c_uint
                    } else { length };
                if pos as libc::c_ulong > samplesBuffSize ||
                       pos.wrapping_add(length) as libc::c_ulong >
                           samplesBuffSize {
                    return -(ZSTD_error_GENERIC as libc::c_int) as size_t
                } else {
                    if notificationLevel >= 3i32 as libc::c_uint {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 46],
                                                          &mut [libc::c_char; 46]>(b"%3u:%3u bytes at pos %8u, savings %7u bytes |\x00")).as_mut_ptr(),
                                u, length, pos,
                                (*dictList.offset(u as isize)).savings);
                        fflush(stderr);
                    }
                    ZDICT_printHex((samplesBuffer as
                                        *const libc::c_char).offset(pos as
                                                                        isize)
                                       as *const libc::c_void,
                                   printedLength as size_t);
                    if notificationLevel >= 3i32 as libc::c_uint {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 4],
                                                          &mut [libc::c_char; 4]>(b"| \n\x00")).as_mut_ptr());
                        fflush(stderr);
                    }
                    u = u.wrapping_add(1)
                }
            }
        }
        let mut dictContentSize_0: U32 = ZDICT_dictSize(dictList);
        if dictContentSize_0 < 128i32 as libc::c_uint {
            free(dictList as *mut libc::c_void);
            return -(ZSTD_error_dictionaryCreation_failed as libc::c_int) as
                       size_t
        } else {
            if (dictContentSize_0 as libc::c_ulong) <
                   targetDictSize.wrapping_div(4i32 as libc::c_ulong) {
                if notificationLevel >= 2i32 as libc::c_uint {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 79],
                                                      &mut [libc::c_char; 79]>(b"!  warning : selected content significantly smaller than requested (%u < %u) \n\x00")).as_mut_ptr(),
                            dictContentSize_0, maxDictSize as U32);
                    fflush(stderr);
                }
                if samplesBuffSize <
                       (10i32 as libc::c_ulong).wrapping_mul(targetDictSize) {
                    if notificationLevel >= 2i32 as libc::c_uint {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 67],
                                                          &mut [libc::c_char; 67]>(b"!  consider increasing the number of samples (total size : %u MB)\n\x00")).as_mut_ptr(),
                                (samplesBuffSize >> 20i32) as U32);
                        fflush(stderr);
                    }
                }
                if minRep > 4i32 as libc::c_uint {
                    if notificationLevel >= 2i32 as libc::c_uint {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 73],
                                                          &mut [libc::c_char; 73]>(b"!  consider increasing selectivity to produce larger dictionary (-s%u) \n\x00")).as_mut_ptr(),
                                selectivity.wrapping_add(1i32 as
                                                             libc::c_uint));
                        fflush(stderr);
                    }
                    if notificationLevel >= 2i32 as libc::c_uint {
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 91],
                                                          &mut [libc::c_char; 91]>(b"!  note : larger dictionaries are not necessarily better, test its efficiency on samples \n\x00")).as_mut_ptr());
                        fflush(stderr);
                    }
                }
            }
            if dictContentSize_0 as libc::c_ulong >
                   targetDictSize.wrapping_mul(3i32 as libc::c_ulong) &&
                   nbSamples > (2i32 * 4i32) as libc::c_uint &&
                   selectivity > 1i32 as libc::c_uint {
                let mut proposedSelectivity: U32 =
                    selectivity.wrapping_sub(1i32 as libc::c_uint);
                while nbSamples >> proposedSelectivity <= 4i32 as libc::c_uint
                      {
                    proposedSelectivity = proposedSelectivity.wrapping_sub(1)
                }
                if notificationLevel >= 2i32 as libc::c_uint {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 80],
                                                      &mut [libc::c_char; 80]>(b"!  note : calculated dictionary significantly larger than requested (%u > %u) \n\x00")).as_mut_ptr(),
                            dictContentSize_0, maxDictSize as U32);
                    fflush(stderr);
                }
                if notificationLevel >= 2i32 as libc::c_uint {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 78],
                                                      &mut [libc::c_char; 78]>(b"!  consider increasing dictionary size, or produce denser dictionary (-s%u) \n\x00")).as_mut_ptr(),
                            proposedSelectivity);
                    fflush(stderr);
                }
                if notificationLevel >= 2i32 as libc::c_uint {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 55],
                                                      &mut [libc::c_char; 55]>(b"!  always test dictionary efficiency on real samples \n\x00")).as_mut_ptr());
                    fflush(stderr);
                }
            }
            let max: U32 = (*dictList).pos;
            let mut currentSize: U32 = 0i32 as U32;
            let mut n: U32 = 0;
            n = 1i32 as U32;
            while n < max {
                currentSize =
                    (currentSize as
                         libc::c_uint).wrapping_add((*dictList.offset(n as
                                                                          isize)).length)
                        as U32 as U32;
                if currentSize as libc::c_ulong > targetDictSize {
                    currentSize =
                        (currentSize as
                             libc::c_uint).wrapping_sub((*dictList.offset(n as
                                                                              isize)).length)
                            as U32 as U32;
                    break ;
                } else { n = n.wrapping_add(1) }
            }
            (*dictList).pos = n;
            dictContentSize_0 = currentSize;
            let mut u_0: U32 = 0;
            let mut ptr: *mut BYTE =
                (dictBuffer as *mut BYTE).offset(maxDictSize as isize);
            u_0 = 1i32 as U32;
            while u_0 < (*dictList).pos {
                let mut l: U32 = (*dictList.offset(u_0 as isize)).length;
                ptr = ptr.offset(-(l as isize));
                if ptr < dictBuffer as *mut BYTE {
                    free(dictList as *mut libc::c_void);
                    return -(ZSTD_error_GENERIC as libc::c_int) as size_t
                } else {
                    memcpy(ptr as *mut libc::c_void,
                           (samplesBuffer as
                                *const libc::c_char).offset((*dictList.offset(u_0
                                                                                  as
                                                                                  isize)).pos
                                                                as isize) as
                               *const libc::c_void, l as libc::c_ulong);
                    u_0 = u_0.wrapping_add(1)
                }
            }
            dictSize =
                ZDICT_addEntropyTablesFromBuffer_advanced(dictBuffer,
                                                          dictContentSize_0 as
                                                              size_t,
                                                          maxDictSize,
                                                          samplesBuffer,
                                                          samplesSizes,
                                                          nbSamples,
                                                          params.zParams);
            free(dictList as *mut libc::c_void);
            return dictSize
        }
    };
}
unsafe extern "C" fn ZDICT_dictSize(mut dictList: *const dictItem) -> U32 {
    let mut u: U32 = 0;
    let mut dictSize: U32 = 0i32 as U32;
    u = 1i32 as U32;
    while u < (*dictList.offset(0isize)).pos {
        dictSize =
            (dictSize as
                 libc::c_uint).wrapping_add((*dictList.offset(u as
                                                                  isize)).length)
                as U32 as U32;
        u = u.wrapping_add(1)
    }
    return dictSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_addEntropyTablesFromBuffer_advanced(mut dictBuffer:
                                                                       *mut libc::c_void,
                                                                   mut dictContentSize:
                                                                       size_t,
                                                                   mut dictBufferCapacity:
                                                                       size_t,
                                                                   mut samplesBuffer:
                                                                       *const libc::c_void,
                                                                   mut samplesSizes:
                                                                       *const size_t,
                                                                   mut nbSamples:
                                                                       libc::c_uint,
                                                                   mut params:
                                                                       ZDICT_params_t)
 -> size_t {
    let compressionLevel: libc::c_int =
        if params.compressionLevel == 0i32 {
            g_compressionLevel_default
        } else { params.compressionLevel };
    let notificationLevel: U32 = params.notificationLevel;
    let mut hSize: size_t = 8i32 as size_t;
    if notificationLevel >= 2i32 as libc::c_uint {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 7],
                                          &mut [libc::c_char; 7]>(b"\r%70s\r\x00")).as_mut_ptr(),
                (*::std::mem::transmute::<&[u8; 1],
                                          &mut [libc::c_char; 1]>(b"\x00")).as_mut_ptr());
        fflush(stderr);
    }
    if notificationLevel >= 2i32 as libc::c_uint {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 17],
                                          &mut [libc::c_char; 17]>(b"statistics ... \n\x00")).as_mut_ptr());
        fflush(stderr);
    }
    let eSize: size_t =
        ZDICT_analyzeEntropy((dictBuffer as
                                  *mut libc::c_char).offset(hSize as isize) as
                                 *mut libc::c_void,
                             dictBufferCapacity.wrapping_sub(hSize),
                             compressionLevel as libc::c_uint, samplesBuffer,
                             samplesSizes, nbSamples,
                             (dictBuffer as
                                  *mut libc::c_char).offset(dictBufferCapacity
                                                                as
                                                                isize).offset(-(dictContentSize
                                                                                    as
                                                                                    isize))
                                 as *const libc::c_void, dictContentSize,
                             notificationLevel);
    if 0 != ZDICT_isError(eSize) {
        return eSize
    } else {
        hSize =
            (hSize as libc::c_ulong).wrapping_add(eSize) as size_t as size_t;
        MEM_writeLE32(dictBuffer, 3962610743u32);
        let randomID: U64 =
            ZSTD_XXH64((dictBuffer as
                            *mut libc::c_char).offset(dictBufferCapacity as
                                                          isize).offset(-(dictContentSize
                                                                              as
                                                                              isize))
                           as *const libc::c_void, dictContentSize,
                       0i32 as libc::c_ulonglong) as U64;
        let compliantID: U32 =
            randomID.wrapping_rem((1u32 <<
                                       31i32).wrapping_sub(32768i32 as
                                                               libc::c_uint)
                                      as
                                      libc::c_ulong).wrapping_add(32768i32 as
                                                                      libc::c_ulong)
                as U32;
        let dictID: U32 =
            if 0 != params.dictID { params.dictID } else { compliantID };
        MEM_writeLE32((dictBuffer as *mut libc::c_char).offset(4isize) as
                          *mut libc::c_void, dictID);
        if hSize.wrapping_add(dictContentSize) < dictBufferCapacity {
            memmove((dictBuffer as *mut libc::c_char).offset(hSize as isize)
                        as *mut libc::c_void,
                    (dictBuffer as
                         *mut libc::c_char).offset(dictBufferCapacity as
                                                       isize).offset(-(dictContentSize
                                                                           as
                                                                           isize))
                        as *const libc::c_void, dictContentSize);
        }
        return if dictBufferCapacity < hSize.wrapping_add(dictContentSize) {
                   dictBufferCapacity
               } else { hSize.wrapping_add(dictContentSize) }
    };
}
static mut g_selectivity_default: U32 = unsafe { 9i32 as U32 };
unsafe extern "C" fn ZDICT_printHex(mut ptr: *const libc::c_void,
                                    mut length: size_t) -> () {
    let b: *const BYTE = ptr as *const BYTE;
    let mut u: size_t = 0;
    u = 0i32 as size_t;
    while u < length {
        let mut c: BYTE = *b.offset(u as isize);
        if (c as libc::c_int) < 32i32 || c as libc::c_int > 126i32 {
            c = '.' as i32 as BYTE
        }
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 3],
                                          &mut [libc::c_char; 3]>(b"%c\x00")).as_mut_ptr(),
                c as libc::c_int);
        fflush(stderr);
        u = u.wrapping_add(1)
    };
}
unsafe extern "C" fn ZDICT_trainBuffer_legacy(mut dictList: *mut dictItem,
                                              mut dictListSize: U32,
                                              buffer: *const libc::c_void,
                                              mut bufferSize: size_t,
                                              mut fileSizes: *const size_t,
                                              mut nbFiles: libc::c_uint,
                                              mut minRatio: U32,
                                              mut notificationLevel: U32)
 -> size_t {
    let suffix0: *mut libc::c_int =
        malloc(bufferSize.wrapping_add(2i32 as
                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                           as
                                                                           libc::c_ulong))
            as *mut libc::c_int;
    let suffix: *mut libc::c_int = suffix0.offset(1isize);
    let mut reverseSuffix: *mut U32 =
        malloc(bufferSize.wrapping_mul(::std::mem::size_of::<U32>() as
                                           libc::c_ulong)) as *mut U32;
    let mut doneMarks: *mut BYTE =
        malloc(bufferSize.wrapping_add(16i32 as
                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<BYTE>()
                                                                           as
                                                                           libc::c_ulong))
            as *mut BYTE;
    let mut filePos: *mut U32 =
        malloc((nbFiles as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<U32>()
                                                    as libc::c_ulong)) as
            *mut U32;
    let mut result: size_t = 0i32 as size_t;
    let mut displayClock: clock_t = 0i32 as clock_t;
    let refreshRate: clock_t =
        1000000i32 as clock_t * 3i32 as libc::c_long / 10i32 as libc::c_long;
    if notificationLevel >= 2i32 as libc::c_uint {
        fprintf(stderr,
                (*::std::mem::transmute::<&[u8; 7],
                                          &mut [libc::c_char; 7]>(b"\r%70s\r\x00")).as_mut_ptr(),
                (*::std::mem::transmute::<&[u8; 1],
                                          &mut [libc::c_char; 1]>(b"\x00")).as_mut_ptr());
        fflush(stderr);
    }
    if suffix0.is_null() || reverseSuffix.is_null() || doneMarks.is_null() ||
           filePos.is_null() {
        result = -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    } else {
        if minRatio < 4i32 as libc::c_uint { minRatio = 4i32 as U32 }
        memset(doneMarks as *mut libc::c_void, 0i32,
               bufferSize.wrapping_add(16i32 as libc::c_ulong));
        if bufferSize > (2000u32 << 20i32) as libc::c_ulong {
            if notificationLevel >= 3i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 45],
                                                  &mut [libc::c_char; 45]>(b"sample set too large : reduced to %u MB ...\n\x00")).as_mut_ptr(),
                        2000u32 << 20i32 >> 20i32);
                fflush(stderr);
            }
        }
        while bufferSize > (2000u32 << 20i32) as libc::c_ulong {
            nbFiles = nbFiles.wrapping_sub(1);
            bufferSize =
                (bufferSize as
                     libc::c_ulong).wrapping_sub(*fileSizes.offset(nbFiles as
                                                                       isize))
                    as size_t as size_t
        }
        if notificationLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    (*::std::mem::transmute::<&[u8; 42],
                                              &mut [libc::c_char; 42]>(b"sorting %u files of total size %u MB ...\n\x00")).as_mut_ptr(),
                    nbFiles, (bufferSize >> 20i32) as U32);
            fflush(stderr);
        }
        let divSuftSortResult: libc::c_int =
            divsufsort(buffer as *const libc::c_uchar, suffix,
                       bufferSize as libc::c_int, 0i32);
        if divSuftSortResult != 0i32 {
            result = -(ZSTD_error_GENERIC as libc::c_int) as size_t
        } else {
            *suffix.offset(bufferSize as isize) = bufferSize as libc::c_int;
            *suffix0.offset(0isize) = bufferSize as libc::c_int;
            let mut pos: size_t = 0;
            pos = 0i32 as size_t;
            while pos < bufferSize {
                *reverseSuffix.offset(*suffix.offset(pos as isize) as isize) =
                    pos as U32;
                pos = pos.wrapping_add(1)
            }
            *filePos.offset(0isize) = 0i32 as U32;
            pos = 1i32 as size_t;
            while pos < nbFiles as libc::c_ulong {
                *filePos.offset(pos as isize) =
                    (*filePos.offset(pos.wrapping_sub(1i32 as libc::c_ulong)
                                         as isize) as
                         libc::c_ulong).wrapping_add(*fileSizes.offset(pos.wrapping_sub(1i32
                                                                                            as
                                                                                            libc::c_ulong)
                                                                           as
                                                                           isize))
                        as U32;
                pos = pos.wrapping_add(1)
            }
            if notificationLevel >= 2i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 23],
                                                  &mut [libc::c_char; 23]>(b"finding patterns ... \n\x00")).as_mut_ptr());
                fflush(stderr);
            }
            if notificationLevel >= 3i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 21],
                                                  &mut [libc::c_char; 21]>(b"minimum ratio : %u \n\x00")).as_mut_ptr(),
                        minRatio);
                fflush(stderr);
            }
            let mut cursor: U32 = 0;
            cursor = 0i32 as U32;
            while (cursor as libc::c_ulong) < bufferSize {
                let mut solution: dictItem =
                    dictItem{pos: 0, length: 0, savings: 0,};
                if 0 != *doneMarks.offset(cursor as isize) {
                    cursor = cursor.wrapping_add(1)
                } else {
                    solution =
                        ZDICT_analyzePos(doneMarks, suffix,
                                         *reverseSuffix.offset(cursor as
                                                                   isize),
                                         buffer, minRatio, notificationLevel);
                    if solution.length == 0i32 as libc::c_uint {
                        cursor = cursor.wrapping_add(1)
                    } else {
                        ZDICT_insertDictItem(dictList, dictListSize, solution,
                                             buffer);
                        cursor =
                            (cursor as
                                 libc::c_uint).wrapping_add(solution.length)
                                as U32 as U32;
                        if !(notificationLevel >= 2i32 as libc::c_uint) {
                            continue ;
                        }
                        if !(ZDICT_clockSpan(displayClock) > refreshRate) {
                            continue ;
                        }
                        displayClock = clock();
                        fprintf(stderr,
                                (*::std::mem::transmute::<&[u8; 12],
                                                          &mut [libc::c_char; 12]>(b"\r%4.2f %% \r\x00")).as_mut_ptr(),
                                cursor as libc::c_double /
                                    bufferSize as libc::c_double *
                                    100i32 as libc::c_double);
                        fflush(stderr);
                        if !(notificationLevel >= 4i32 as libc::c_uint) {
                            continue ;
                        }
                        fflush(stderr);
                    }
                }
            }
        }
    }
    free(suffix0 as *mut libc::c_void);
    free(reverseSuffix as *mut libc::c_void);
    free(doneMarks as *mut libc::c_void);
    free(filePos as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn ZDICT_clockSpan(mut nPrevious: clock_t) -> clock_t {
    return clock() - nPrevious;
}
unsafe extern "C" fn ZDICT_insertDictItem(mut table: *mut dictItem,
                                          mut maxSize: U32, mut elt: dictItem,
                                          mut buffer: *const libc::c_void)
 -> () {
    let mut mergeId: U32 = ZDICT_tryMerge(table, elt, 0i32 as U32, buffer);
    if 0 != mergeId {
        let mut newMerge: U32 = 1i32 as U32;
        while 0 != newMerge {
            newMerge =
                ZDICT_tryMerge(table, *table.offset(mergeId as isize),
                               mergeId, buffer);
            if 0 != newMerge { ZDICT_removeDictItem(table, mergeId); }
            mergeId = newMerge
        }
        return
    } else {
        let mut current: U32 = 0;
        let mut nextElt: U32 = (*table).pos;
        if nextElt >= maxSize {
            nextElt = maxSize.wrapping_sub(1i32 as libc::c_uint)
        }
        current = nextElt.wrapping_sub(1i32 as libc::c_uint);
        while (*table.offset(current as isize)).savings < elt.savings {
            *table.offset(current.wrapping_add(1i32 as libc::c_uint) as isize)
                = *table.offset(current as isize);
            current = current.wrapping_sub(1)
        }
        *table.offset(current.wrapping_add(1i32 as libc::c_uint) as isize) =
            elt;
        (*table).pos = nextElt.wrapping_add(1i32 as libc::c_uint);
        return;
    };
}
/* ! ZDICT_tryMerge() :
    check if dictItem can be merged, do it if possible
    @return : id of destination elt, 0 if not merged
*/
unsafe extern "C" fn ZDICT_tryMerge(mut table: *mut dictItem,
                                    mut elt: dictItem, mut eltNbToSkip: U32,
                                    mut buffer: *const libc::c_void) -> U32 {
    let tableSize: U32 = (*table).pos;
    let eltEnd: U32 = elt.pos.wrapping_add(elt.length);
    let buf: *const libc::c_char = buffer as *const libc::c_char;
    let mut u: U32 = 0;
    u = 1i32 as U32;
    while u < tableSize {
        if !(u == eltNbToSkip) {
            if (*table.offset(u as isize)).pos > elt.pos &&
                   (*table.offset(u as isize)).pos <= eltEnd {
                let addedLength: U32 =
                    (*table.offset(u as isize)).pos.wrapping_sub(elt.pos);
                let ref mut fresh6 = (*table.offset(u as isize)).length;
                *fresh6 =
                    (*fresh6 as libc::c_uint).wrapping_add(addedLength) as U32
                        as U32;
                (*table.offset(u as isize)).pos = elt.pos;
                let ref mut fresh7 = (*table.offset(u as isize)).savings;
                *fresh7 =
                    (*fresh7 as
                         libc::c_uint).wrapping_add(elt.savings.wrapping_mul(addedLength).wrapping_div(elt.length))
                        as U32 as U32;
                let ref mut fresh8 = (*table.offset(u as isize)).savings;
                *fresh8 =
                    (*fresh8 as
                         libc::c_uint).wrapping_add(elt.length.wrapping_div(8i32
                                                                                as
                                                                                libc::c_uint))
                        as U32 as U32;
                elt = *table.offset(u as isize);
                while u > 1i32 as libc::c_uint &&
                          (*table.offset(u.wrapping_sub(1i32 as libc::c_uint)
                                             as isize)).savings < elt.savings
                      {
                    *table.offset(u as isize) =
                        *table.offset(u.wrapping_sub(1i32 as libc::c_uint) as
                                          isize);
                    u = u.wrapping_sub(1)
                }
                *table.offset(u as isize) = elt;
                return u
            }
        }
        u = u.wrapping_add(1)
    }
    u = 1i32 as U32;
    while u < tableSize {
        if !(u == eltNbToSkip) {
            if (*table.offset(u as
                                  isize)).pos.wrapping_add((*table.offset(u as
                                                                              isize)).length)
                   >= elt.pos && (*table.offset(u as isize)).pos < elt.pos {
                let addedLength_0: libc::c_int =
                    (eltEnd as libc::c_int as
                         libc::c_uint).wrapping_sub((*table.offset(u as
                                                                       isize)).pos.wrapping_add((*table.offset(u
                                                                                                                   as
                                                                                                                   isize)).length))
                        as libc::c_int;
                let ref mut fresh9 = (*table.offset(u as isize)).savings;
                *fresh9 =
                    (*fresh9 as
                         libc::c_uint).wrapping_add(elt.length.wrapping_div(8i32
                                                                                as
                                                                                libc::c_uint))
                        as U32 as U32;
                if addedLength_0 > 0i32 {
                    let ref mut fresh10 = (*table.offset(u as isize)).length;
                    *fresh10 =
                        (*fresh10 as
                             libc::c_uint).wrapping_add(addedLength_0 as
                                                            libc::c_uint) as
                            U32 as U32;
                    let ref mut fresh11 = (*table.offset(u as isize)).savings;
                    *fresh11 =
                        (*fresh11 as
                             libc::c_uint).wrapping_add(elt.savings.wrapping_mul(addedLength_0
                                                                                     as
                                                                                     libc::c_uint).wrapping_div(elt.length))
                            as U32 as U32
                }
                elt = *table.offset(u as isize);
                while u > 1i32 as libc::c_uint &&
                          (*table.offset(u.wrapping_sub(1i32 as libc::c_uint)
                                             as isize)).savings < elt.savings
                      {
                    *table.offset(u as isize) =
                        *table.offset(u.wrapping_sub(1i32 as libc::c_uint) as
                                          isize);
                    u = u.wrapping_sub(1)
                }
                *table.offset(u as isize) = elt;
                return u
            } else if MEM_read64(buf.offset((*table.offset(u as isize)).pos as
                                                isize) as *const libc::c_void)
                          ==
                          MEM_read64(buf.offset(elt.pos as
                                                    isize).offset(1isize) as
                                         *const libc::c_void) {
                if 0 !=
                       isIncluded(buf.offset((*table.offset(u as isize)).pos
                                                 as isize) as
                                      *const libc::c_void,
                                  buf.offset(elt.pos as isize).offset(1isize)
                                      as *const libc::c_void,
                                  (*table.offset(u as isize)).length as
                                      size_t) {
                    let addedLength_1: size_t =
                        (if elt.length as libc::c_int -
                                (*table.offset(u as isize)).length as
                                    libc::c_int > 1i32 {
                             elt.length as libc::c_int -
                                 (*table.offset(u as isize)).length as
                                     libc::c_int
                         } else { 1i32 }) as size_t;
                    (*table.offset(u as isize)).pos = elt.pos;
                    let ref mut fresh12 = (*table.offset(u as isize)).savings;
                    *fresh12 =
                        (*fresh12 as
                             libc::c_uint).wrapping_add((elt.savings as
                                                             libc::c_ulong).wrapping_mul(addedLength_1).wrapping_div(elt.length
                                                                                                                         as
                                                                                                                         libc::c_ulong)
                                                            as U32) as U32 as
                            U32;
                    (*table.offset(u as isize)).length =
                        if elt.length <
                               (*table.offset(u as
                                                  isize)).length.wrapping_add(1i32
                                                                                  as
                                                                                  libc::c_uint)
                           {
                            elt.length
                        } else {
                            (*table.offset(u as
                                               isize)).length.wrapping_add(1i32
                                                                               as
                                                                               libc::c_uint)
                        };
                    return u
                }
            }
        }
        u = u.wrapping_add(1)
    }
    return 0i32 as U32;
}
unsafe extern "C" fn isIncluded(mut in_0: *const libc::c_void,
                                mut container: *const libc::c_void,
                                mut length: size_t) -> libc::c_int {
    let ip: *const libc::c_char = in_0 as *const libc::c_char;
    let into: *const libc::c_char = container as *const libc::c_char;
    let mut u: size_t = 0;
    u = 0i32 as size_t;
    while u < length {
        if *ip.offset(u as isize) as libc::c_int !=
               *into.offset(u as isize) as libc::c_int {
            break ;
        }
        u = u.wrapping_add(1)
    }
    return (u == length) as libc::c_int;
}
unsafe extern "C" fn ZDICT_removeDictItem(mut table: *mut dictItem,
                                          mut id: U32) -> () {
    let max: U32 = (*table.offset(0isize)).pos;
    let mut u: U32 = 0;
    if 0 == id {
        return
    } else {
        u = id;
        while u < max.wrapping_sub(1i32 as libc::c_uint) {
            *table.offset(u as isize) =
                *table.offset(u.wrapping_add(1i32 as libc::c_uint) as isize);
            u = u.wrapping_add(1)
        }
        (*table).pos = (*table).pos.wrapping_sub(1);
        return;
    };
}
unsafe extern "C" fn ZDICT_analyzePos(mut doneMarks: *mut BYTE,
                                      mut suffix: *const libc::c_int,
                                      mut start: U32,
                                      mut buffer: *const libc::c_void,
                                      mut minRatio: U32,
                                      mut notificationLevel: U32)
 -> dictItem {
    let mut lengthList: [U32; 64] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut cumulLength: [U32; 64] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut savings: [U32; 64] =
        [0i32 as U32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut b: *const BYTE = buffer as *const BYTE;
    let mut maxLength: size_t = 64i32 as size_t;
    let mut pos: size_t = *suffix.offset(start as isize) as size_t;
    let mut end: U32 = start;
    let mut solution: dictItem = dictItem{pos: 0, length: 0, savings: 0,};
    memset(&mut solution as *mut dictItem as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<dictItem>() as libc::c_ulong);
    *doneMarks.offset(pos as isize) = 1i32 as BYTE;
    if MEM_read16(b.offset(pos as isize).offset(0isize) as
                      *const libc::c_void) as libc::c_int ==
           MEM_read16(b.offset(pos as isize).offset(2isize) as
                          *const libc::c_void) as libc::c_int ||
           MEM_read16(b.offset(pos as isize).offset(1isize) as
                          *const libc::c_void) as libc::c_int ==
               MEM_read16(b.offset(pos as isize).offset(3isize) as
                              *const libc::c_void) as libc::c_int ||
           MEM_read16(b.offset(pos as isize).offset(2isize) as
                          *const libc::c_void) as libc::c_int ==
               MEM_read16(b.offset(pos as isize).offset(4isize) as
                              *const libc::c_void) as libc::c_int {
        let pattern16: U16 =
            MEM_read16(b.offset(pos as isize).offset(4isize) as
                           *const libc::c_void);
        let mut u: U32 = 0;
        let mut patternEnd: U32 = 6i32 as U32;
        while MEM_read16(b.offset(pos as isize).offset(patternEnd as isize) as
                             *const libc::c_void) as libc::c_int ==
                  pattern16 as libc::c_int {
            patternEnd =
                (patternEnd as
                     libc::c_uint).wrapping_add(2i32 as libc::c_uint) as U32
                    as U32
        }
        if *b.offset(pos.wrapping_add(patternEnd as libc::c_ulong) as isize)
               as libc::c_int ==
               *b.offset(pos.wrapping_add(patternEnd as
                                              libc::c_ulong).wrapping_sub(1i32
                                                                              as
                                                                              libc::c_ulong)
                             as isize) as libc::c_int {
            patternEnd = patternEnd.wrapping_add(1)
        }
        u = 1i32 as U32;
        while u < patternEnd {
            *doneMarks.offset(pos.wrapping_add(u as libc::c_ulong) as isize) =
                1i32 as BYTE;
            u = u.wrapping_add(1)
        }
        return solution
    } else {
        let mut length: size_t = 0;
        loop  {
            end = end.wrapping_add(1);
            length =
                ZDICT_count(b.offset(pos as isize) as *const libc::c_void,
                            b.offset(*suffix.offset(end as isize) as isize) as
                                *const libc::c_void);
            if !(length >= 7i32 as libc::c_ulong) { break ; }
        }
        let mut length_0: size_t = 0;
        loop  {
            length_0 =
                ZDICT_count(b.offset(pos as isize) as *const libc::c_void,
                            b.offset(*suffix.offset(start as
                                                        isize).offset(-1isize)
                                         as isize) as *const libc::c_void);
            if length_0 >= 7i32 as libc::c_ulong {
                start = start.wrapping_sub(1)
            }
            if !(length_0 >= 7i32 as libc::c_ulong) { break ; }
        }
        if end.wrapping_sub(start) < minRatio {
            let mut idx: U32 = 0;
            idx = start;
            while idx < end {
                *doneMarks.offset(*suffix.offset(idx as isize) as isize) =
                    1i32 as BYTE;
                idx = idx.wrapping_add(1)
            }
            return solution
        } else {
            let mut i: libc::c_int = 0;
            let mut searchLength: U32 = 0;
            let mut refinedStart: U32 = start;
            let mut refinedEnd: U32 = end;
            if notificationLevel >= 4i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 2],
                                                  &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
                fflush(stderr);
            }
            if notificationLevel >= 4i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 47],
                                                  &mut [libc::c_char; 47]>(b"found %3u matches of length >= %i at pos %7u  \x00")).as_mut_ptr(),
                        end.wrapping_sub(start), 7i32, pos as U32);
                fflush(stderr);
            }
            if notificationLevel >= 4i32 as libc::c_uint {
                fprintf(stderr,
                        (*::std::mem::transmute::<&[u8; 2],
                                                  &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
                fflush(stderr);
            }
            searchLength = 7i32 as U32;
            loop  {
                let mut currentChar: BYTE = 0i32 as BYTE;
                let mut currentCount: U32 = 0i32 as U32;
                let mut currentID: U32 = refinedStart;
                let mut id: U32 = 0;
                let mut selectedCount: U32 = 0i32 as U32;
                let mut selectedID: U32 = currentID;
                id = refinedStart;
                while id < refinedEnd {
                    if *b.offset((*suffix.offset(id as isize) as
                                      libc::c_uint).wrapping_add(searchLength)
                                     as isize) as libc::c_int !=
                           currentChar as libc::c_int {
                        if currentCount > selectedCount {
                            selectedCount = currentCount;
                            selectedID = currentID
                        }
                        currentID = id;
                        currentChar =
                            *b.offset((*suffix.offset(id as isize) as
                                           libc::c_uint).wrapping_add(searchLength)
                                          as isize);
                        currentCount = 0i32 as U32
                    }
                    currentCount = currentCount.wrapping_add(1);
                    id = id.wrapping_add(1)
                }
                if currentCount > selectedCount {
                    selectedCount = currentCount;
                    selectedID = currentID
                }
                if selectedCount < minRatio { break ; }
                refinedStart = selectedID;
                refinedEnd = refinedStart.wrapping_add(selectedCount);
                searchLength = searchLength.wrapping_add(1)
            }
            start = refinedStart;
            pos = *suffix.offset(refinedStart as isize) as size_t;
            end = start;
            memset(lengthList.as_mut_ptr() as *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<[U32; 64]>() as libc::c_ulong);
            let mut length_1: size_t = 0;
            loop  {
                end = end.wrapping_add(1);
                length_1 =
                    ZDICT_count(b.offset(pos as isize) as *const libc::c_void,
                                b.offset(*suffix.offset(end as isize) as
                                             isize) as *const libc::c_void);
                if length_1 >= 64i32 as libc::c_ulong {
                    length_1 = (64i32 - 1i32) as size_t
                }
                lengthList[length_1 as usize] =
                    lengthList[length_1 as usize].wrapping_add(1);
                if !(length_1 >= 7i32 as libc::c_ulong) { break ; }
            }
            let mut length_2: size_t = 7i32 as size_t;
            while 0 !=
                      (length_2 >= 7i32 as libc::c_ulong) as libc::c_int &
                          (start > 0i32 as libc::c_uint) as libc::c_int {
                length_2 =
                    ZDICT_count(b.offset(pos as isize) as *const libc::c_void,
                                b.offset(*suffix.offset(start.wrapping_sub(1i32
                                                                               as
                                                                               libc::c_uint)
                                                            as isize) as
                                             isize) as *const libc::c_void);
                if length_2 >= 64i32 as libc::c_ulong {
                    length_2 = (64i32 - 1i32) as size_t
                }
                lengthList[length_2 as usize] =
                    lengthList[length_2 as usize].wrapping_add(1);
                if !(length_2 >= 7i32 as libc::c_ulong) { continue ; }
                start = start.wrapping_sub(1)
            }
            memset(cumulLength.as_mut_ptr() as *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<[U32; 64]>() as libc::c_ulong);
            cumulLength[maxLength.wrapping_sub(1i32 as libc::c_ulong) as
                            usize] =
                lengthList[maxLength.wrapping_sub(1i32 as libc::c_ulong) as
                               usize];
            i = maxLength.wrapping_sub(2i32 as libc::c_ulong) as libc::c_int;
            while i >= 0i32 {
                cumulLength[i as usize] =
                    cumulLength[(i + 1i32) as
                                    usize].wrapping_add(lengthList[i as
                                                                       usize]);
                i -= 1
            }
            i = 64i32 - 1i32;
            while i >= 7i32 {
                if cumulLength[i as usize] >= minRatio { break ; }
                i -= 1
            }
            maxLength = i as size_t;
            let mut l: U32 = maxLength as U32;
            let c: BYTE =
                *b.offset(pos.wrapping_add(maxLength).wrapping_sub(1i32 as
                                                                       libc::c_ulong)
                              as isize);
            while *b.offset(pos.wrapping_add(l as
                                                 libc::c_ulong).wrapping_sub(2i32
                                                                                 as
                                                                                 libc::c_ulong)
                                as isize) as libc::c_int == c as libc::c_int {
                l = l.wrapping_sub(1)
            }
            maxLength = l as size_t;
            if maxLength < 7i32 as libc::c_ulong {
                return solution
            } else {
                savings[5usize] = 0i32 as U32;
                i = 7i32;
                while i <= maxLength as libc::c_int {
                    savings[i as usize] =
                        savings[(i - 1i32) as
                                    usize].wrapping_add(lengthList[i as
                                                                       usize].wrapping_mul((i
                                                                                                -
                                                                                                3i32)
                                                                                               as
                                                                                               libc::c_uint));
                    i += 1
                }
                if notificationLevel >= 4i32 as libc::c_uint {
                    fprintf(stderr,
                            (*::std::mem::transmute::<&[u8; 70],
                                                      &mut [libc::c_char; 70]>(b"Selected ref at position %u, of length %u : saves %u (ratio: %.2f)  \n\x00")).as_mut_ptr(),
                            pos as U32, maxLength as U32,
                            savings[maxLength as usize],
                            savings[maxLength as usize] as libc::c_double /
                                maxLength as libc::c_double);
                    fflush(stderr);
                }
                solution.pos = pos as U32;
                solution.length = maxLength as U32;
                solution.savings = savings[maxLength as usize];
                let mut id_0: U32 = 0;
                id_0 = start;
                while id_0 < end {
                    let mut p: U32 = 0;
                    let mut pEnd: U32 = 0;
                    let mut length_3: U32 = 0;
                    let testedPos: U32 = *suffix.offset(id_0 as isize) as U32;
                    if testedPos as libc::c_ulong == pos {
                        length_3 = solution.length
                    } else {
                        length_3 =
                            ZDICT_count(b.offset(pos as isize) as
                                            *const libc::c_void,
                                        b.offset(testedPos as isize) as
                                            *const libc::c_void) as U32;
                        if length_3 > solution.length {
                            length_3 = solution.length
                        }
                    }
                    pEnd = testedPos.wrapping_add(length_3);
                    p = testedPos;
                    while p < pEnd {
                        *doneMarks.offset(p as isize) = 1i32 as BYTE;
                        p = p.wrapping_add(1)
                    }
                    id_0 = id_0.wrapping_add(1)
                }
                return solution
            }
        }
    };
}
/* ! ZDICT_count() :
    Count the nb of common bytes between 2 pointers.
    Note : this function presumes end of buffer followed by noisy guard band.
*/
unsafe extern "C" fn ZDICT_count(mut pIn: *const libc::c_void,
                                 mut pMatch: *const libc::c_void) -> size_t {
    let pStart: *const libc::c_char = pIn as *const libc::c_char;
    loop  {
        let diff: size_t = MEM_readST(pMatch) ^ MEM_readST(pIn);
        if 0 == diff {
            pIn =
                (pIn as
                     *const libc::c_char).offset(::std::mem::size_of::<size_t>()
                                                     as libc::c_ulong as
                                                     isize) as
                    *const libc::c_void;
            pMatch =
                (pMatch as
                     *const libc::c_char).offset(::std::mem::size_of::<size_t>()
                                                     as libc::c_ulong as
                                                     isize) as
                    *const libc::c_void
        } else {
            pIn =
                (pIn as
                     *const libc::c_char).offset(ZDICT_NbCommonBytes(diff) as
                                                     isize) as
                    *const libc::c_void;
            return pStart.offset_to(pIn as
                                        *const libc::c_char).expect("bad offset_to")
                       as libc::c_long as size_t
        }
    };
}
unsafe extern "C" fn ZDICT_NbCommonBytes(mut val: size_t) -> libc::c_uint {
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
unsafe extern "C" fn ZDICT_initDictItem(mut d: *mut dictItem) -> () {
    (*d).pos = 1i32 as U32;
    (*d).length = 0i32 as U32;
    (*d).savings = -1i32 as U32;
}
unsafe extern "C" fn ZDICT_fillNoise(mut buffer: *mut libc::c_void,
                                     mut length: size_t) -> () {
    let prime1: libc::c_uint = 2654435761u32;
    let prime2: libc::c_uint = 2246822519u32;
    let mut acc: libc::c_uint = prime1;
    let mut p: size_t = 0i32 as size_t;
    p = 0i32 as size_t;
    while p < length {
        acc = acc.wrapping_mul(prime2);
        *(buffer as *mut libc::c_uchar).offset(p as isize) =
            (acc >> 21i32) as libc::c_uchar;
        p = p.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_addEntropyTablesFromBuffer(mut dictBuffer:
                                                              *mut libc::c_void,
                                                          mut dictContentSize:
                                                              size_t,
                                                          mut dictBufferCapacity:
                                                              size_t,
                                                          mut samplesBuffer:
                                                              *const libc::c_void,
                                                          mut samplesSizes:
                                                              *const size_t,
                                                          mut nbSamples:
                                                              libc::c_uint)
 -> size_t {
    let mut params: ZDICT_params_t =
        ZDICT_params_t{compressionLevel: 0, notificationLevel: 0, dictID: 0,};
    memset(&mut params as *mut ZDICT_params_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZDICT_params_t>() as libc::c_ulong);
    return ZDICT_addEntropyTablesFromBuffer_advanced(dictBuffer,
                                                     dictContentSize,
                                                     dictBufferCapacity,
                                                     samplesBuffer,
                                                     samplesSizes, nbSamples,
                                                     params);
}
