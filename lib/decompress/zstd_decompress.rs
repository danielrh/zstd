#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( libc , offset_to )]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* ***************************
*  Streaming
****************************/
    /* ! ZSTD_decompress_usingDDict() :
 *  Decompression using a digested Dictionary.
 *  Faster startup than ZSTD_decompress_usingDict(), recommended when same dictionary is used multiple times. */
    /* ! ZSTD_freeDDict() :
 *  Function frees memory allocated with ZSTD_createDDict() */
    /* ! ZSTD_createDDict() :
 *  Create a digested dictionary, ready to start decompression operation without startup delay.
 *  dictBuffer can be released after DDict creation, as its content is copied inside DDict */
    /* ! Constructor and Destructor of FSE_DTable.
    Note that its size depends on 'tableLog' */
    /* *< recommended size for output buffer. Guarantee to successfully flush at least one complete compressed block in all circumstances. */
    /* *< recommended size for input buffer */
    /* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
    /* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
    /* *< position where reading stopped. Will be updated. Necessarily 0 <= pos <= size */
    /* *< Copy dictionary content internally */
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
    fn ZSTD_getErrorCode(functionResult: size_t) -> ZSTD_ErrorCode;
    #[no_mangle]
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
    #[no_mangle]
    fn HUF_readDTableX2_wksp(DTable: *mut HUF_DTable,
                             src: *const libc::c_void, srcSize: size_t,
                             workSpace: *mut libc::c_void, wkspSize: size_t)
     -> size_t;
    #[no_mangle]
    fn HUF_decompress1X_usingDTable_bmi2(dst: *mut libc::c_void,
                                         maxDstSize: size_t,
                                         cSrc: *const libc::c_void,
                                         cSrcSize: size_t,
                                         DTable: *const HUF_DTable,
                                         bmi2: libc::c_int) -> size_t;
    #[no_mangle]
    fn HUF_decompress1X1_DCtx_wksp_bmi2(dctx: *mut HUF_DTable,
                                        dst: *mut libc::c_void,
                                        dstSize: size_t,
                                        cSrc: *const libc::c_void,
                                        cSrcSize: size_t,
                                        workSpace: *mut libc::c_void,
                                        wkspSize: size_t, bmi2: libc::c_int)
     -> size_t;
    #[no_mangle]
    fn HUF_decompress4X_usingDTable_bmi2(dst: *mut libc::c_void,
                                         maxDstSize: size_t,
                                         cSrc: *const libc::c_void,
                                         cSrcSize: size_t,
                                         DTable: *const HUF_DTable,
                                         bmi2: libc::c_int) -> size_t;
    #[no_mangle]
    fn HUF_decompress4X_hufOnly_wksp_bmi2(dctx: *mut HUF_DTable,
                                          dst: *mut libc::c_void,
                                          dstSize: size_t,
                                          cSrc: *const libc::c_void,
                                          cSrcSize: size_t,
                                          workSpace: *mut libc::c_void,
                                          wkspSize: size_t, bmi2: libc::c_int)
     -> size_t;
    #[no_mangle]
    fn ZSTD_malloc(size: size_t, customMem: ZSTD_customMem)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ZSTD_free(ptr: *mut libc::c_void, customMem: ZSTD_customMem) -> ();
    #[no_mangle]
    fn ZSTD_XXH64_digest(statePtr: *const XXH64_state_t) -> XXH64_hash_t;
    #[no_mangle]
    fn ZSTD_XXH64_update(statePtr: *mut XXH64_state_t,
                         input: *const libc::c_void, length: size_t)
     -> XXH_errorcode;
    #[no_mangle]
    fn ZSTD_XXH64_reset(statePtr: *mut XXH64_state_t, seed: libc::c_ulonglong)
     -> XXH_errorcode;
}
pub const ZSTD_error_dictionaryCreation_failed: ERR_enum = 34;
pub type U64 = uint64_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_CStream_t {
    pub bitContainer: size_t,
    pub bitPos: libc::c_uint,
    pub startPtr: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub endPtr: *mut libc::c_char,
}
pub const bt_reserved: blockType_e = 3;
pub const set_compressed: symbolEncodingType_e = 2;
pub type HUF_DTable = U32;
pub const ZSTD_error_dictionary_wrong: ERR_enum = 32;
pub const ZSTD_error_tableLog_tooLarge: ERR_enum = 44;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_frameHeader {
    pub frameContentSize: libc::c_ulonglong,
    pub windowSize: libc::c_ulonglong,
    pub blockSizeMax: libc::c_uint,
    pub frameType: ZSTD_frameType_e,
    pub headerSize: libc::c_uint,
    pub dictID: libc::c_uint,
    pub checksumFlag: libc::c_uint,
}
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_entropyDTables_t {
    pub LLTable: [ZSTD_seqSymbol; 513],
    pub OFTable: [ZSTD_seqSymbol; 257],
    pub MLTable: [ZSTD_seqSymbol; 513],
    pub hufTable: [HUF_DTable; 4097],
    pub workspace: [U32; 512],
    pub rep: [U32; 3],
}
pub const ZSTDnit_block: ZSTD_nextInputType_e = 2;
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct blockProperties_t {
    pub blockType: blockType_e,
    pub lastBlock: U32,
    pub origSize: U32,
}
pub const zdss_load: ZSTD_dStreamStage = 3;
pub const ZSTD_error_init_missing: ERR_enum = 62;
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
pub const ZSTD_error_parameter_outOfBound: ERR_enum = 42;
pub type U16 = uint16_t;
pub type uint64_t = libc::c_ulong;
pub type blockType_e = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_seqSymbol_header {
    pub fastMode: U32,
    pub tableLog: U32,
}
pub const zdss_read: ZSTD_dStreamStage = 2;
pub type ZSTD_DDict = ZSTD_DDict_s;
pub const set_basic: symbolEncodingType_e = 0;
pub const bt_rle: blockType_e = 1;
pub type XXH64_state_t = XXH64_state_s;
pub const ZSTD_lo_isRegularOffset: ZSTD_longOffset_e = 0;
pub const ZSTDnit_checksum: ZSTD_nextInputType_e = 4;
pub const ZSTD_error_dstSize_tooSmall: ERR_enum = 70;
pub const ZSTD_error_parameter_unsupported: ERR_enum = 40;
pub const bt_raw: blockType_e = 0;
pub const ZSTD_error_corruption_detected: ERR_enum = 20;
pub type FSE_DTable = libc::c_uint;
pub type ZSTD_dictContentType_e = libc::c_uint;
pub const ZSTDds_decompressBlock: ZSTD_dStage = 3;
pub const set_repeat: symbolEncodingType_e = 3;
pub const ZSTD_error_GENERIC: ERR_enum = 1;
pub type ERR_enum = libc::c_uint;
pub const ZSTD_f_zstd1_magicless: ZSTD_format_e = 1;
pub type BIT_DStream_status = libc::c_uint;
pub type size_t = libc::c_ulong;
pub const ZSTD_lo_isLongOffset: ZSTD_longOffset_e = 1;
pub const ZSTD_error_checksum_wrong: ERR_enum = 22;
pub type ZSTD_DStream = ZSTD_DCtx;
pub const DStream_p_maxWindowSize: ZSTD_DStreamParameter_e = 0;
pub const MEM_static_assert: unnamed_1 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
pub const ZSTD_error_no_error: ERR_enum = 0;
pub const ZSTDds_checkChecksum: ZSTD_dStage = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct seqState_t {
    pub DStream: BIT_DStream_t,
    pub stateLL: ZSTD_fseState,
    pub stateOffb: ZSTD_fseState,
    pub stateML: ZSTD_fseState,
    pub prevOffset: [size_t; 3],
    pub prefixStart: *const BYTE,
    pub dictEnd: *const BYTE,
    pub pos: size_t,
}
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub const ZSTD_error_version_unsupported: ERR_enum = 12;
pub type U32 = uint32_t;
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
pub const zdss_init: ZSTD_dStreamStage = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_frameType_e = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
pub type FSE_CTable = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
pub const ZSTD_error_workSpace_tooSmall: ERR_enum = 66;
pub type ZSTD_DCtx = ZSTD_DCtx_s;
pub const ZSTDnit_lastBlock: ZSTD_nextInputType_e = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_inBuffer_s {
    pub src: *const libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type int16_t = libc::c_short;
pub const zdss_flush: ZSTD_dStreamStage = 4;
pub const ZSTD_error_frameIndex_tooLarge: ERR_enum = 100;
pub const set_rle: symbolEncodingType_e = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    u: U32,
    c: [BYTE; 4],
}
pub const XXH_OK: XXH_errorcode = 0;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
pub type XXH64_hash_t = libc::c_ulonglong;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const ZSTD_error_maxSymbolValue_tooLarge: ERR_enum = 46;
pub const ZSTD_frame: ZSTD_frameType_e = 0;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub type ZSTD_dStreamStage = libc::c_uint;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub type ZSTD_ErrorCode = ERR_enum;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub const ZSTDds_decompressLastBlock: ZSTD_dStage = 4;
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
pub const ZSTD_error_frameParameter_unsupported: ERR_enum = 14;
pub type ZSTD_nextInputType_e = libc::c_uint;
pub const MEM_static_assert_0: unnamed_0 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_cpuid_t {
    pub f1c: U32,
    pub f1d: U32,
    pub f7b: U32,
    pub f7c: U32,
}
pub type ZSTD_longOffset_e = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_seqSymbol {
    pub nextState: U16,
    pub nbAdditionalBits: BYTE,
    pub nbBits: BYTE,
    pub baseValue: U32,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_fseState {
    pub state: size_t,
    pub table: *const ZSTD_seqSymbol,
}
pub type XXH_errorcode = libc::c_uint;
pub type ZSTD_format_e = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
pub const ZSTDds_getFrameHeaderSize: ZSTD_dStage = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_DDict_s {
    pub dictBuffer: *mut libc::c_void,
    pub dictContent: *const libc::c_void,
    pub dictSize: size_t,
    pub entropy: ZSTD_entropyDTables_t,
    pub dictID: U32,
    pub entropyPresent: U32,
    pub cMem: ZSTD_customMem,
}
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;
pub const ZSTD_error_stage_wrong: ERR_enum = 60;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: libc::c_int,
    pub deltaNbBits: U32,
}
pub const bt_compressed: blockType_e = 2;
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
pub const ZSTD_error_maxCode: ERR_enum = 120;
pub const ZSTD_error_maxSymbolValue_tooSmall: ERR_enum = 48;
pub const ZSTD_f_zstd1: ZSTD_format_e = 0;
pub const ZSTD_skippableFrame: ZSTD_frameType_e = 1;
pub type BYTE = uint8_t;
pub type S16 = int16_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DState_t {
    pub state: size_t,
    pub table: *const libc::c_void,
}
pub type ZSTD_dictLoadMethod_e = libc::c_uint;
pub const ZSTDds_decodeBlockHeader: ZSTD_dStage = 2;
pub const ZSTDds_decodeFrameHeader: ZSTD_dStage = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct seq_t {
    pub litLength: size_t,
    pub matchLength: size_t,
    pub offset: size_t,
    pub match_0: *const BYTE,
}
pub const ZSTD_error_memory_allocation: ERR_enum = 64;
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_DCtx_s {
    pub LLTptr: *const ZSTD_seqSymbol,
    pub MLTptr: *const ZSTD_seqSymbol,
    pub OFTptr: *const ZSTD_seqSymbol,
    pub HUFptr: *const HUF_DTable,
    pub entropy: ZSTD_entropyDTables_t,
    pub previousDstEnd: *const libc::c_void,
    pub prefixStart: *const libc::c_void,
    pub virtualStart: *const libc::c_void,
    pub dictEnd: *const libc::c_void,
    pub expected: size_t,
    pub fParams: ZSTD_frameHeader,
    pub decodedSize: U64,
    pub bType: blockType_e,
    pub stage: ZSTD_dStage,
    pub litEntropy: U32,
    pub fseEntropy: U32,
    pub xxhState: XXH64_state_t,
    pub headerSize: size_t,
    pub dictID: U32,
    pub format: ZSTD_format_e,
    pub litPtr: *const BYTE,
    pub customMem: ZSTD_customMem,
    pub litSize: size_t,
    pub rleSize: size_t,
    pub staticSize: size_t,
    pub bmi2: libc::c_int,
    pub ddictLocal: *mut ZSTD_DDict,
    pub ddict: *const ZSTD_DDict,
    pub streamStage: ZSTD_dStreamStage,
    pub inBuff: *mut libc::c_char,
    pub inBuffSize: size_t,
    pub inPos: size_t,
    pub maxWindowSize: size_t,
    pub outBuff: *mut libc::c_char,
    pub outBuffSize: size_t,
    pub outStart: size_t,
    pub outEnd: size_t,
    pub lhSize: size_t,
    pub legacyContext: *mut libc::c_void,
    pub previousLegacyVersion: U32,
    pub legacyVersion: U32,
    pub hostageByte: U32,
    pub noForwardProgress: libc::c_int,
    pub litBuffer: [BYTE; 131080],
    pub headerBuffer: [BYTE; 18],
}
pub const ZSTDnit_skippableFrame: ZSTD_nextInputType_e = 5;
pub type uint8_t = libc::c_uchar;
pub type ZSTD_DStreamParameter_e = libc::c_uint;
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
pub type unnamed_0 = libc::c_uint;
pub type unnamed_1 = libc::c_uint;
pub type uint32_t = libc::c_uint;
pub type symbolEncodingType_e = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const libc::c_void,
    pub symbolTT: *const libc::c_void,
    pub stateLog: libc::c_uint,
}
pub const ZSTDds_skipFrame: ZSTD_dStage = 7;
pub const ZSTD_error_srcSize_wrong: ERR_enum = 72;
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
pub const ZSTDnit_frameHeader: ZSTD_nextInputType_e = 0;
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
pub type ZSTD_dStage = libc::c_uint;
pub const ZSTDds_decodeSkippableHeader: ZSTD_dStage = 6;
pub const zdss_loadHeader: ZSTD_dStreamStage = 1;
pub const ZSTDnit_blockHeader: ZSTD_nextInputType_e = 1;
pub const ZSTD_error_prefix_unknown: ERR_enum = 10;
pub type ptrdiff_t = libc::c_long;
pub const ZSTD_error_seekableIO: ERR_enum = 102;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: libc::c_uint,
    pub ptr: *const libc::c_char,
    pub start: *const libc::c_char,
    pub limitPtr: *const libc::c_char,
}
pub const ZSTD_error_dictionary_corrupted: ERR_enum = 30;
pub const ZSTD_error_frameParameter_windowTooLarge: ERR_enum = 16;
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
    let one: unnamed = unnamed{u: 1i32 as U32,};
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
                    current_block = 16829082034480816705;
                }
                6 => { current_block = 16829082034480816705; }
                5 => { current_block = 15471647155328524506; }
                4 => { current_block = 13278447473922156413; }
                3 => { current_block = 9754527956383385389; }
                2 => { current_block = 11713782215396347339; }
                _ => { current_block = 10886091980245723256; }
            }
            match current_block {
                16829082034480816705 => {
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
                    current_block = 15471647155328524506;
                }
                _ => { }
            }
            match current_block {
                15471647155328524506 => {
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
                    current_block = 13278447473922156413;
                }
                _ => { }
            }
            match current_block {
                13278447473922156413 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(3isize)
                                                              as size_t) <<
                                                             24i32) as size_t
                            as size_t;
                    current_block = 9754527956383385389;
                }
                _ => { }
            }
            match current_block {
                9754527956383385389 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(2isize)
                                                              as size_t) <<
                                                             16i32) as size_t
                            as size_t;
                    current_block = 11713782215396347339;
                }
                _ => { }
            }
            match current_block {
                11713782215396347339 => {
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
pub unsafe extern "C" fn ZSTD_decompress(mut dst: *mut libc::c_void,
                                         mut dstCapacity: size_t,
                                         mut src: *const libc::c_void,
                                         mut srcSize: size_t) -> size_t {
    let mut regenSize: size_t = 0;
    let dctx: *mut ZSTD_DCtx = ZSTD_createDCtx();
    if dctx.is_null() {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    } else {
        regenSize = ZSTD_decompressDCtx(dctx, dst, dstCapacity, src, srcSize);
        ZSTD_freeDCtx(dctx);
        return regenSize
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDCtx() -> *mut ZSTD_DCtx {
    return ZSTD_createDCtx_advanced(ZSTD_defaultCMem);
}
static mut ZSTD_defaultCMem: ZSTD_customMem =
    unsafe {
        ZSTD_customMem{customAlloc: None,
                       customFree: None,
                       opaque: 0 as *const libc::c_void as *mut libc::c_void,}
    };
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDCtx_advanced(mut customMem:
                                                      ZSTD_customMem)
 -> *mut ZSTD_DCtx {
    if 0 !=
           customMem.customAlloc.is_none() as libc::c_int ^
               customMem.customFree.is_none() as libc::c_int {
        return 0 as *mut ZSTD_DCtx
    } else {
        let dctx: *mut ZSTD_DCtx =
            ZSTD_malloc(::std::mem::size_of::<ZSTD_DCtx>() as libc::c_ulong,
                        customMem) as *mut ZSTD_DCtx;
        if dctx.is_null() {
            return 0 as *mut ZSTD_DCtx
        } else {
            (*dctx).customMem = customMem;
            ZSTD_initDCtx_internal(dctx);
            return dctx
        }
    };
}
unsafe extern "C" fn ZSTD_initDCtx_internal(mut dctx: *mut ZSTD_DCtx) -> () {
    (*dctx).format = ZSTD_f_zstd1;
    (*dctx).staticSize = 0i32 as size_t;
    (*dctx).maxWindowSize =
        ((1i32 as U32) << 27i32).wrapping_add(1i32 as libc::c_uint) as size_t;
    (*dctx).ddict = 0 as *const ZSTD_DDict;
    (*dctx).ddictLocal = 0 as *mut ZSTD_DDict;
    (*dctx).inBuff = 0 as *mut libc::c_char;
    (*dctx).inBuffSize = 0i32 as size_t;
    (*dctx).outBuffSize = 0i32 as size_t;
    (*dctx).streamStage = zdss_init;
    (*dctx).legacyContext = 0 as *mut libc::c_void;
    (*dctx).previousLegacyVersion = 0i32 as U32;
    (*dctx).noForwardProgress = 0i32;
    (*dctx).bmi2 = ZSTD_cpuid_bmi2(ZSTD_cpuid());
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeDCtx(mut dctx: *mut ZSTD_DCtx) -> size_t {
    if dctx.is_null() {
        return 0i32 as size_t
    } else if 0 != (*dctx).staticSize {
        return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
    } else {
        let cMem: ZSTD_customMem = (*dctx).customMem;
        ZSTD_freeDDict((*dctx).ddictLocal);
        (*dctx).ddictLocal = 0 as *mut ZSTD_DDict;
        ZSTD_free((*dctx).inBuff as *mut libc::c_void, cMem);
        (*dctx).inBuff = 0 as *mut libc::c_char;
        ZSTD_free(dctx as *mut libc::c_void, cMem);
        return 0i32 as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeDDict(mut ddict: *mut ZSTD_DDict)
 -> size_t {
    if ddict.is_null() {
        return 0i32 as size_t
    } else {
        let cMem: ZSTD_customMem = (*ddict).cMem;
        ZSTD_free((*ddict).dictBuffer, cMem);
        ZSTD_free(ddict as *mut libc::c_void, cMem);
        return 0i32 as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressDCtx(mut dctx: *mut ZSTD_DCtx,
                                             mut dst: *mut libc::c_void,
                                             mut dstCapacity: size_t,
                                             mut src: *const libc::c_void,
                                             mut srcSize: size_t) -> size_t {
    return ZSTD_decompress_usingDict(dctx, dst, dstCapacity, src, srcSize,
                                     0 as *const libc::c_void,
                                     0i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompress_usingDict(mut dctx: *mut ZSTD_DCtx,
                                                   mut dst: *mut libc::c_void,
                                                   mut dstCapacity: size_t,
                                                   mut src:
                                                       *const libc::c_void,
                                                   mut srcSize: size_t,
                                                   mut dict:
                                                       *const libc::c_void,
                                                   mut dictSize: size_t)
 -> size_t {
    return ZSTD_decompressMultiFrame(dctx, dst, dstCapacity, src, srcSize,
                                     dict, dictSize, 0 as *const ZSTD_DDict);
}
unsafe extern "C" fn ZSTD_decompressMultiFrame(mut dctx: *mut ZSTD_DCtx,
                                               mut dst: *mut libc::c_void,
                                               mut dstCapacity: size_t,
                                               mut src: *const libc::c_void,
                                               mut srcSize: size_t,
                                               mut dict: *const libc::c_void,
                                               mut dictSize: size_t,
                                               mut ddict: *const ZSTD_DDict)
 -> size_t {
    let dststart: *mut libc::c_void = dst;
    let mut moreThan1Frame: libc::c_int = 0i32;
    if !ddict.is_null() {
        dict = ZSTD_DDictDictContent(ddict);
        dictSize = ZSTD_DDictDictSize(ddict)
    }
    while srcSize >= ZSTD_frameHeaderSize_prefix {
        let magicNumber: U32 = MEM_readLE32(src);
        if magicNumber & 4294967280u32 == 407710288u32 {
            let mut skippableSize: size_t = 0;
            if srcSize < ZSTD_skippableHeaderSize {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
            } else {
                skippableSize =
                    (MEM_readLE32((src as
                                       *const BYTE).offset(ZSTD_frameIdSize as
                                                               isize) as
                                      *const libc::c_void) as
                         libc::c_ulong).wrapping_add(ZSTD_skippableHeaderSize);
                if srcSize < skippableSize {
                    return -(ZSTD_error_srcSize_wrong as libc::c_int) as
                               size_t
                } else {
                    src =
                        (src as *const BYTE).offset(skippableSize as isize) as
                            *const libc::c_void;
                    srcSize =
                        (srcSize as libc::c_ulong).wrapping_sub(skippableSize)
                            as size_t as size_t
                }
            }
        } else {
            if !ddict.is_null() {
                let errcod: size_t =
                    ZSTD_decompressBegin_usingDDict(dctx, ddict);
                if 0 != ERR_isError(errcod) { return errcod }
            } else {
                let errcod_0: size_t =
                    ZSTD_decompressBegin_usingDict(dctx, dict, dictSize);
                if 0 != ERR_isError(errcod_0) { return errcod_0 }
            }
            ZSTD_checkContinuity(dctx, dst);
            let res: size_t =
                ZSTD_decompressFrame(dctx, dst, dstCapacity,
                                     &mut src as *mut *const libc::c_void,
                                     &mut srcSize as *mut size_t);
            if ZSTD_getErrorCode(res) as libc::c_uint ==
                   ZSTD_error_prefix_unknown as libc::c_int as libc::c_uint &&
                   moreThan1Frame == 1i32 {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
            } else if 0 != ERR_isError(res) {
                return res
            } else {
                dst =
                    (dst as *mut BYTE).offset(res as isize) as
                        *mut libc::c_void;
                dstCapacity =
                    (dstCapacity as libc::c_ulong).wrapping_sub(res) as size_t
                        as size_t;
                moreThan1Frame = 1i32
            }
        }
    }
    if 0 != srcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else {
        return (dststart as
                    *mut BYTE).offset_to(dst as
                                             *mut BYTE).expect("bad offset_to")
                   as libc::c_long as size_t
    };
}
/* ! ZSTD_decompressFrame() :
*   @dctx must be properly initialized */
unsafe extern "C" fn ZSTD_decompressFrame(mut dctx: *mut ZSTD_DCtx,
                                          mut dst: *mut libc::c_void,
                                          mut dstCapacity: size_t,
                                          mut srcPtr:
                                              *mut *const libc::c_void,
                                          mut srcSizePtr: *mut size_t)
 -> size_t {
    let mut ip: *const BYTE = *srcPtr as *const BYTE;
    let ostart: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = ostart.offset(dstCapacity as isize);
    let mut op: *mut BYTE = ostart;
    let mut remainingSize: size_t = *srcSizePtr;
    if remainingSize <
           ZSTD_frameHeaderSize_min.wrapping_add(ZSTD_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else {
        let frameHeaderSize: size_t =
            ZSTD_frameHeaderSize(ip as *const libc::c_void,
                                 ZSTD_frameHeaderSize_prefix);
        if 0 != ERR_isError(frameHeaderSize) {
            return frameHeaderSize
        } else if remainingSize <
                      frameHeaderSize.wrapping_add(ZSTD_blockHeaderSize) {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
        } else {
            let errcod: size_t =
                ZSTD_decodeFrameHeader(dctx, ip as *const libc::c_void,
                                       frameHeaderSize);
            if 0 != ERR_isError(errcod) {
                return errcod
            } else {
                ip = ip.offset(frameHeaderSize as isize);
                remainingSize =
                    (remainingSize as
                         libc::c_ulong).wrapping_sub(frameHeaderSize) as
                        size_t as size_t;
                loop  {
                    let mut decodedSize: size_t = 0;
                    let mut blockProperties: blockProperties_t =
                        blockProperties_t{blockType: bt_raw,
                                          lastBlock: 0,
                                          origSize: 0,};
                    let cBlockSize: size_t =
                        ZSTD_getcBlockSize(ip as *const libc::c_void,
                                           remainingSize,
                                           &mut blockProperties as
                                               *mut blockProperties_t);
                    if 0 != ERR_isError(cBlockSize) {
                        return cBlockSize
                    } else {
                        ip = ip.offset(ZSTD_blockHeaderSize as isize);
                        remainingSize =
                            (remainingSize as
                                 libc::c_ulong).wrapping_sub(ZSTD_blockHeaderSize)
                                as size_t as size_t;
                        if cBlockSize > remainingSize {
                            return -(ZSTD_error_srcSize_wrong as libc::c_int)
                                       as size_t
                        } else {
                            match blockProperties.blockType as libc::c_uint {
                                2 => {
                                    decodedSize =
                                        ZSTD_decompressBlock_internal(dctx,
                                                                      op as
                                                                          *mut libc::c_void,
                                                                      op.offset_to(oend).expect("bad offset_to")
                                                                          as
                                                                          libc::c_long
                                                                          as
                                                                          size_t,
                                                                      ip as
                                                                          *const libc::c_void,
                                                                      cBlockSize,
                                                                      1i32)
                                }
                                0 => {
                                    decodedSize =
                                        ZSTD_copyRawBlock(op as
                                                              *mut libc::c_void,
                                                          op.offset_to(oend).expect("bad offset_to")
                                                              as libc::c_long
                                                              as size_t,
                                                          ip as
                                                              *const libc::c_void,
                                                          cBlockSize)
                                }
                                1 => {
                                    decodedSize =
                                        ZSTD_generateNxBytes(op as
                                                                 *mut libc::c_void,
                                                             op.offset_to(oend).expect("bad offset_to")
                                                                 as
                                                                 libc::c_long
                                                                 as size_t,
                                                             *ip,
                                                             blockProperties.origSize
                                                                 as size_t)
                                }
                                3 | _ => {
                                    return -(ZSTD_error_corruption_detected as
                                                 libc::c_int) as size_t
                                }
                            }
                            if 0 != ERR_isError(decodedSize) {
                                return decodedSize
                            } else {
                                if 0 != (*dctx).fParams.checksumFlag {
                                    ZSTD_XXH64_update(&mut (*dctx).xxhState as
                                                          *mut XXH64_state_t,
                                                      op as
                                                          *const libc::c_void,
                                                      decodedSize);
                                }
                                op = op.offset(decodedSize as isize);
                                ip = ip.offset(cBlockSize as isize);
                                remainingSize =
                                    (remainingSize as
                                         libc::c_ulong).wrapping_sub(cBlockSize)
                                        as size_t as size_t;
                                if 0 != blockProperties.lastBlock { break ; }
                            }
                        }
                    }
                }
                if (*dctx).fParams.frameContentSize !=
                       0u64.wrapping_sub(1i32 as libc::c_ulonglong) {
                    if ostart.offset_to(op).expect("bad offset_to") as
                           libc::c_long as U64 as libc::c_ulonglong !=
                           (*dctx).fParams.frameContentSize {
                        return -(ZSTD_error_corruption_detected as
                                     libc::c_int) as size_t
                    }
                }
                if 0 != (*dctx).fParams.checksumFlag {
                    let checkCalc: U32 =
                        ZSTD_XXH64_digest(&mut (*dctx).xxhState as
                                              *mut XXH64_state_t) as U32;
                    let mut checkRead: U32 = 0;
                    if remainingSize < 4i32 as libc::c_ulong {
                        return -(ZSTD_error_checksum_wrong as libc::c_int) as
                                   size_t
                    } else {
                        checkRead = MEM_readLE32(ip as *const libc::c_void);
                        if checkRead != checkCalc {
                            return -(ZSTD_error_checksum_wrong as libc::c_int)
                                       as size_t
                        } else {
                            ip = ip.offset(4isize);
                            remainingSize =
                                (remainingSize as
                                     libc::c_ulong).wrapping_sub(4i32 as
                                                                     libc::c_ulong)
                                    as size_t as size_t
                        }
                    }
                }
                *srcPtr = ip as *const libc::c_void;
                *srcSizePtr = remainingSize;
                return ostart.offset_to(op).expect("bad offset_to") as
                           libc::c_long as size_t
            }
        }
    };
}
/* ! ZSTD_getcBlockSize() :
 *  Provides the size of compressed block from block header `src` */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getcBlockSize(mut src: *const libc::c_void,
                                            mut srcSize: size_t,
                                            mut bpPtr: *mut blockProperties_t)
 -> size_t {
    if srcSize < ZSTD_blockHeaderSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else {
        let cBlockHeader: U32 = MEM_readLE24(src);
        let cSize: U32 = cBlockHeader >> 3i32;
        (*bpPtr).lastBlock = cBlockHeader & 1i32 as libc::c_uint;
        (*bpPtr).blockType =
            (cBlockHeader >> 1i32 & 3i32 as libc::c_uint) as blockType_e;
        (*bpPtr).origSize = cSize;
        if (*bpPtr).blockType as libc::c_uint ==
               bt_rle as libc::c_int as libc::c_uint {
            return 1i32 as size_t
        } else if (*bpPtr).blockType as libc::c_uint ==
                      bt_reserved as libc::c_int as libc::c_uint {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        } else { return cSize as size_t }
    };
}
static mut ZSTD_blockHeaderSize: size_t = unsafe { 3i32 as size_t };
unsafe extern "C" fn ZSTD_generateNxBytes(mut dst: *mut libc::c_void,
                                          mut dstCapacity: size_t,
                                          mut byte: BYTE, mut length: size_t)
 -> size_t {
    if length > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else { memset(dst, byte as libc::c_int, length); return length };
}
unsafe extern "C" fn ZSTD_copyRawBlock(mut dst: *mut libc::c_void,
                                       mut dstCapacity: size_t,
                                       mut src: *const libc::c_void,
                                       mut srcSize: size_t) -> size_t {
    if srcSize > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else { memcpy(dst, src, srcSize); return srcSize };
}
unsafe extern "C" fn ZSTD_decompressBlock_internal(mut dctx: *mut ZSTD_DCtx,
                                                   mut dst: *mut libc::c_void,
                                                   mut dstCapacity: size_t,
                                                   mut src:
                                                       *const libc::c_void,
                                                   mut srcSize: size_t,
                                                   frame: libc::c_int)
 -> size_t {
    let mut ip: *const BYTE = src as *const BYTE;
    let isLongOffset: ZSTD_longOffset_e =
        (0 != MEM_32bits() &&
             (0 == frame ||
                  (*dctx).fParams.windowSize >
                      1u64 <<
                          (if 0 != MEM_32bits() { 25i32 } else { 57i32 }) as
                              U32)) as libc::c_int as ZSTD_longOffset_e;
    if srcSize >= (1i32 << 17i32) as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else {
        let litCSize: size_t = ZSTD_decodeLiteralsBlock(dctx, src, srcSize);
        if 0 != ERR_isError(litCSize) {
            return litCSize
        } else {
            ip = ip.offset(litCSize as isize);
            srcSize =
                (srcSize as libc::c_ulong).wrapping_sub(litCSize) as size_t as
                    size_t;
            let mut nbSeq: libc::c_int = 0;
            let seqHSize: size_t =
                ZSTD_decodeSeqHeaders(dctx, &mut nbSeq as *mut libc::c_int,
                                      ip as *const libc::c_void, srcSize);
            if 0 != ERR_isError(seqHSize) {
                return seqHSize
            } else {
                ip = ip.offset(seqHSize as isize);
                srcSize =
                    (srcSize as libc::c_ulong).wrapping_sub(seqHSize) as
                        size_t as size_t;
                if (0 == frame ||
                        (*dctx).fParams.windowSize >
                            (1i32 << 24i32) as libc::c_ulonglong) &&
                       nbSeq > 0i32 {
                    let shareLongOffsets: U32 =
                        ZSTD_getLongOffsetsShare((*dctx).OFTptr);
                    let minShare: U32 =
                        (if 0 != MEM_64bits() { 7i32 } else { 20i32 }) as U32;
                    if shareLongOffsets >= minShare {
                        return ZSTD_decompressSequencesLong(dctx, dst,
                                                            dstCapacity,
                                                            ip as
                                                                *const libc::c_void,
                                                            srcSize, nbSeq,
                                                            isLongOffset)
                    }
                }
                return ZSTD_decompressSequences(dctx, dst, dstCapacity,
                                                ip as *const libc::c_void,
                                                srcSize, nbSeq, isLongOffset)
            }
        }
    };
}
unsafe extern "C" fn ZSTD_decompressSequences(mut dctx: *mut ZSTD_DCtx,
                                              mut dst: *mut libc::c_void,
                                              mut maxDstSize: size_t,
                                              mut seqStart:
                                                  *const libc::c_void,
                                              mut seqSize: size_t,
                                              mut nbSeq: libc::c_int,
                                              isLongOffset: ZSTD_longOffset_e)
 -> size_t {
    if 0 != (*dctx).bmi2 {
        return ZSTD_decompressSequences_bmi2(dctx, dst, maxDstSize, seqStart,
                                             seqSize, nbSeq, isLongOffset)
    } else {
        return ZSTD_decompressSequences_default(dctx, dst, maxDstSize,
                                                seqStart, seqSize, nbSeq,
                                                isLongOffset)
    };
}
unsafe extern "C" fn ZSTD_decompressSequences_default(mut dctx:
                                                          *mut ZSTD_DCtx,
                                                      mut dst:
                                                          *mut libc::c_void,
                                                      mut maxDstSize: size_t,
                                                      mut seqStart:
                                                          *const libc::c_void,
                                                      mut seqSize: size_t,
                                                      mut nbSeq: libc::c_int,
                                                      isLongOffset:
                                                          ZSTD_longOffset_e)
 -> size_t {
    return ZSTD_decompressSequences_body(dctx, dst, maxDstSize, seqStart,
                                         seqSize, nbSeq, isLongOffset);
}
unsafe extern "C" fn ZSTD_decompressSequences_body(mut dctx: *mut ZSTD_DCtx,
                                                   mut dst: *mut libc::c_void,
                                                   mut maxDstSize: size_t,
                                                   mut seqStart:
                                                       *const libc::c_void,
                                                   mut seqSize: size_t,
                                                   mut nbSeq: libc::c_int,
                                                   isLongOffset:
                                                       ZSTD_longOffset_e)
 -> size_t {
    let mut ip: *const BYTE = seqStart as *const BYTE;
    let iend: *const BYTE = ip.offset(seqSize as isize);
    let ostart: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = ostart.offset(maxDstSize as isize);
    let mut op: *mut BYTE = ostart;
    let mut litPtr: *const BYTE = (*dctx).litPtr;
    let litEnd: *const BYTE = litPtr.offset((*dctx).litSize as isize);
    let prefixStart: *const BYTE = (*dctx).prefixStart as *const BYTE;
    let vBase: *const BYTE = (*dctx).virtualStart as *const BYTE;
    let dictEnd: *const BYTE = (*dctx).dictEnd as *const BYTE;
    if 0 != nbSeq {
        let mut seqState: seqState_t =
            seqState_t{DStream:
                           BIT_DStream_t{bitContainer: 0,
                                         bitsConsumed: 0,
                                         ptr: 0 as *const libc::c_char,
                                         start: 0 as *const libc::c_char,
                                         limitPtr: 0 as *const libc::c_char,},
                       stateLL:
                           ZSTD_fseState{state: 0,
                                         table: 0 as *const ZSTD_seqSymbol,},
                       stateOffb:
                           ZSTD_fseState{state: 0,
                                         table: 0 as *const ZSTD_seqSymbol,},
                       stateML:
                           ZSTD_fseState{state: 0,
                                         table: 0 as *const ZSTD_seqSymbol,},
                       prevOffset: [0; 3],
                       prefixStart: 0 as *const BYTE,
                       dictEnd: 0 as *const BYTE,
                       pos: 0,};
        (*dctx).fseEntropy = 1i32 as U32;
        let mut i: U32 = 0;
        i = 0i32 as U32;
        while i < 3i32 as libc::c_uint {
            seqState.prevOffset[i as usize] =
                (*dctx).entropy.rep[i as usize] as size_t;
            i = i.wrapping_add(1)
        }
        let errcod: size_t =
            BIT_initDStream(&mut seqState.DStream as *mut BIT_DStream_t,
                            ip as *const libc::c_void,
                            ip.offset_to(iend).expect("bad offset_to") as
                                libc::c_long as size_t);
        if 0 != ERR_isError(errcod) {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        } else {
            ZSTD_initFseState(&mut seqState.stateLL as *mut ZSTD_fseState,
                              &mut seqState.DStream as *mut BIT_DStream_t,
                              (*dctx).LLTptr);
            ZSTD_initFseState(&mut seqState.stateOffb as *mut ZSTD_fseState,
                              &mut seqState.DStream as *mut BIT_DStream_t,
                              (*dctx).OFTptr);
            ZSTD_initFseState(&mut seqState.stateML as *mut ZSTD_fseState,
                              &mut seqState.DStream as *mut BIT_DStream_t,
                              (*dctx).MLTptr);
            while BIT_reloadDStream(&mut seqState.DStream as
                                        *mut BIT_DStream_t) as libc::c_uint <=
                      BIT_DStream_completed as libc::c_int as libc::c_uint &&
                      0 != nbSeq {
                nbSeq -= 1;
                let sequence: seq_t =
                    ZSTD_decodeSequence(&mut seqState as *mut seqState_t,
                                        isLongOffset);
                let oneSeqSize: size_t =
                    ZSTD_execSequence(op, oend, sequence,
                                      &mut litPtr as *mut *const BYTE, litEnd,
                                      prefixStart, vBase, dictEnd);
                if 0 != ERR_isError(oneSeqSize) {
                    return oneSeqSize
                } else { op = op.offset(oneSeqSize as isize) }
            }
            if 0 != nbSeq {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            } else {
                let mut i_0: U32 = 0;
                i_0 = 0i32 as U32;
                while i_0 < 3i32 as libc::c_uint {
                    (*dctx).entropy.rep[i_0 as usize] =
                        seqState.prevOffset[i_0 as usize] as U32;
                    i_0 = i_0.wrapping_add(1)
                }
            }
        }
    }
    let lastLLSize: size_t =
        litPtr.offset_to(litEnd).expect("bad offset_to") as libc::c_long as
            size_t;
    if lastLLSize >
           op.offset_to(oend).expect("bad offset_to") as libc::c_long as
               size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else {
        memcpy(op as *mut libc::c_void, litPtr as *const libc::c_void,
               lastLLSize);
        op = op.offset(lastLLSize as isize);
        return ostart.offset_to(op).expect("bad offset_to") as libc::c_long as
                   size_t
    };
}
unsafe extern "C" fn ZSTD_decodeSequence(mut seqState: *mut seqState_t,
                                         longOffsets: ZSTD_longOffset_e)
 -> seq_t {
    let mut seq: seq_t =
        seq_t{litLength: 0,
              matchLength: 0,
              offset: 0,
              match_0: 0 as *const BYTE,};
    let llBits: U32 =
        (*(*seqState).stateLL.table.offset((*seqState).stateLL.state as
                                               isize)).nbAdditionalBits as
            U32;
    let mlBits: U32 =
        (*(*seqState).stateML.table.offset((*seqState).stateML.state as
                                               isize)).nbAdditionalBits as
            U32;
    let ofBits: U32 =
        (*(*seqState).stateOffb.table.offset((*seqState).stateOffb.state as
                                                 isize)).nbAdditionalBits as
            U32;
    let totalBits: U32 = llBits.wrapping_add(mlBits).wrapping_add(ofBits);
    let llBase: U32 =
        (*(*seqState).stateLL.table.offset((*seqState).stateLL.state as
                                               isize)).baseValue;
    let mlBase: U32 =
        (*(*seqState).stateML.table.offset((*seqState).stateML.state as
                                               isize)).baseValue;
    let ofBase: U32 =
        (*(*seqState).stateOffb.table.offset((*seqState).stateOffb.state as
                                                 isize)).baseValue;
    let mut offset: size_t = 0;
    if 0 == ofBits {
        offset = 0i32 as size_t
    } else if 0 != MEM_32bits() && 0 != longOffsets as libc::c_uint &&
                  ofBits >= 25i32 as libc::c_uint {
        let extraBits: U32 =
            ofBits.wrapping_sub(if ofBits <
                                       (32i32 as
                                            libc::c_uint).wrapping_sub((*seqState).DStream.bitsConsumed)
                                   {
                                    ofBits
                                } else {
                                    (32i32 as
                                         libc::c_uint).wrapping_sub((*seqState).DStream.bitsConsumed)
                                });
        offset =
            (ofBase as
                 libc::c_ulong).wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream
                                                                  as
                                                                  *mut BIT_DStream_t,
                                                              ofBits.wrapping_sub(extraBits))
                                                 << extraBits);
        BIT_reloadDStream(&mut (*seqState).DStream as *mut BIT_DStream_t);
        if 0 != extraBits {
            offset =
                (offset as
                     libc::c_ulong).wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream
                                                                      as
                                                                      *mut BIT_DStream_t,
                                                                  extraBits))
                    as size_t as size_t
        }
    } else {
        offset =
            (ofBase as
                 libc::c_ulong).wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream
                                                                  as
                                                                  *mut BIT_DStream_t,
                                                              ofBits));
        if 0 != MEM_32bits() {
            BIT_reloadDStream(&mut (*seqState).DStream as *mut BIT_DStream_t);
        }
    }
    if ofBits <= 1i32 as libc::c_uint {
        offset =
            (offset as
                 libc::c_ulong).wrapping_add((llBase == 0i32 as libc::c_uint)
                                                 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t;
        if 0 != offset {
            let mut temp: size_t =
                if offset == 3i32 as libc::c_ulong {
                    (*seqState).prevOffset[0usize].wrapping_sub(1i32 as
                                                                    libc::c_ulong)
                } else { (*seqState).prevOffset[offset as usize] };
            temp =
                (temp as
                     libc::c_ulong).wrapping_add((0 == temp) as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t;
            if offset != 1i32 as libc::c_ulong {
                (*seqState).prevOffset[2usize] =
                    (*seqState).prevOffset[1usize]
            }
            (*seqState).prevOffset[1usize] = (*seqState).prevOffset[0usize];
            offset = temp;
            (*seqState).prevOffset[0usize] = offset
        } else { offset = (*seqState).prevOffset[0usize] }
    } else {
        (*seqState).prevOffset[2usize] = (*seqState).prevOffset[1usize];
        (*seqState).prevOffset[1usize] = (*seqState).prevOffset[0usize];
        (*seqState).prevOffset[0usize] = offset
    }
    seq.offset = offset;
    seq.matchLength =
        (mlBase as
             libc::c_ulong).wrapping_add(if mlBits > 0i32 as libc::c_uint {
                                             BIT_readBitsFast(&mut (*seqState).DStream
                                                                  as
                                                                  *mut BIT_DStream_t,
                                                              mlBits)
                                         } else { 0i32 as libc::c_ulong });
    if 0 != MEM_32bits() &&
           mlBits.wrapping_add(llBits) >=
               (25i32 - if 30i32 > 25i32 { 30i32 - 25i32 } else { 0i32 }) as
                   libc::c_uint {
        BIT_reloadDStream(&mut (*seqState).DStream as *mut BIT_DStream_t);
    }
    if 0 != MEM_64bits() &&
           totalBits >= (57i32 - (9i32 + 9i32 + 8i32)) as libc::c_uint {
        BIT_reloadDStream(&mut (*seqState).DStream as *mut BIT_DStream_t);
    }
    seq.litLength =
        (llBase as
             libc::c_ulong).wrapping_add(if llBits > 0i32 as libc::c_uint {
                                             BIT_readBitsFast(&mut (*seqState).DStream
                                                                  as
                                                                  *mut BIT_DStream_t,
                                                              llBits)
                                         } else { 0i32 as libc::c_ulong });
    if 0 != MEM_32bits() {
        BIT_reloadDStream(&mut (*seqState).DStream as *mut BIT_DStream_t);
    }
    ZSTD_updateFseState(&mut (*seqState).stateLL as *mut ZSTD_fseState,
                        &mut (*seqState).DStream as *mut BIT_DStream_t);
    ZSTD_updateFseState(&mut (*seqState).stateML as *mut ZSTD_fseState,
                        &mut (*seqState).DStream as *mut BIT_DStream_t);
    if 0 != MEM_32bits() {
        BIT_reloadDStream(&mut (*seqState).DStream as *mut BIT_DStream_t);
    }
    ZSTD_updateFseState(&mut (*seqState).stateOffb as *mut ZSTD_fseState,
                        &mut (*seqState).DStream as *mut BIT_DStream_t);
    return seq;
}
unsafe extern "C" fn ZSTD_updateFseState(mut DStatePtr: *mut ZSTD_fseState,
                                         mut bitD: *mut BIT_DStream_t) -> () {
    let DInfo: ZSTD_seqSymbol =
        *(*DStatePtr).table.offset((*DStatePtr).state as isize);
    let nbBits: U32 = DInfo.nbBits as U32;
    let lowBits: size_t = BIT_readBits(bitD, nbBits);
    (*DStatePtr).state =
        (DInfo.nextState as libc::c_ulong).wrapping_add(lowBits);
}
unsafe extern "C" fn ZSTD_execSequence(mut op: *mut BYTE, oend: *mut BYTE,
                                       mut sequence: seq_t,
                                       mut litPtr: *mut *const BYTE,
                                       litLimit: *const BYTE,
                                       prefixStart: *const BYTE,
                                       virtualStart: *const BYTE,
                                       dictEnd: *const BYTE) -> size_t {
    let mut sub2: libc::c_int = 0;
    static mut dec64table: [libc::c_int; 8] =
        unsafe { [8i32, 8i32, 8i32, 7i32, 8i32, 9i32, 10i32, 11i32] };
    static mut dec32table: [U32; 8] =
        unsafe {
            [0i32 as U32, 1i32 as U32, 2i32 as U32, 1i32 as U32, 4i32 as U32,
             4i32 as U32, 4i32 as U32, 4i32 as U32]
        };
    let oLitEnd: *mut BYTE = op.offset(sequence.litLength as isize);
    let sequenceLength: size_t =
        sequence.litLength.wrapping_add(sequence.matchLength);
    let oMatchEnd: *mut BYTE = op.offset(sequenceLength as isize);
    let oend_w: *mut BYTE = oend.offset(-8isize);
    let iLitEnd: *const BYTE = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const BYTE =
        oLitEnd.offset(-(sequence.offset as isize));
    if oMatchEnd > oend {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else if iLitEnd > litLimit {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    } else if oLitEnd > oend_w {
        return ZSTD_execSequenceLast7(op, oend, sequence, litPtr, litLimit,
                                      prefixStart, virtualStart, dictEnd)
    } else {
        ZSTD_copy8(op as *mut libc::c_void, *litPtr as *const libc::c_void);
        if sequence.litLength > 8i32 as libc::c_ulong {
            ZSTD_wildcopy(op.offset(8isize) as *mut libc::c_void,
                          (*litPtr).offset(8isize) as *const libc::c_void,
                          sequence.litLength.wrapping_sub(8i32 as
                                                              libc::c_ulong)
                              as ptrdiff_t);
        }
        op = oLitEnd;
        *litPtr = iLitEnd;
        if sequence.offset >
               prefixStart.offset_to(oLitEnd).expect("bad offset_to") as
                   libc::c_long as size_t {
            if sequence.offset >
                   virtualStart.offset_to(oLitEnd).expect("bad offset_to") as
                       libc::c_long as size_t {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            } else {
                match_0 =
                    dictEnd.offset(prefixStart.offset_to(match_0).expect("bad offset_to")
                                       as libc::c_long as isize);
                if match_0.offset(sequence.matchLength as isize) <= dictEnd {
                    memmove(oLitEnd as *mut libc::c_void,
                            match_0 as *const libc::c_void,
                            sequence.matchLength);
                    return sequenceLength
                } else {
                    let length1: size_t =
                        match_0.offset_to(dictEnd).expect("bad offset_to") as
                            libc::c_long as size_t;
                    memmove(oLitEnd as *mut libc::c_void,
                            match_0 as *const libc::c_void, length1);
                    op = oLitEnd.offset(length1 as isize);
                    sequence.matchLength =
                        (sequence.matchLength as
                             libc::c_ulong).wrapping_sub(length1) as size_t as
                            size_t;
                    match_0 = prefixStart;
                    if op > oend_w ||
                           sequence.matchLength < 3i32 as libc::c_ulong {
                        let mut i: U32 = 0;
                        i = 0i32 as U32;
                        while (i as libc::c_ulong) < sequence.matchLength {
                            *op.offset(i as isize) =
                                *match_0.offset(i as isize);
                            i = i.wrapping_add(1)
                        }
                        return sequenceLength
                    }
                }
            }
        }
        if sequence.offset < 8i32 as libc::c_ulong {
            sub2 = dec64table[sequence.offset as usize];
            *op.offset(0isize) = *match_0.offset(0isize);
            *op.offset(1isize) = *match_0.offset(1isize);
            *op.offset(2isize) = *match_0.offset(2isize);
            *op.offset(3isize) = *match_0.offset(3isize);
            match_0 =
                match_0.offset(dec32table[sequence.offset as usize] as isize);
            ZSTD_copy4(op.offset(4isize) as *mut libc::c_void,
                       match_0 as *const libc::c_void);
            match_0 = match_0.offset(-(sub2 as isize))
        } else {
            ZSTD_copy8(op as *mut libc::c_void,
                       match_0 as *const libc::c_void);
        }
        op = op.offset(8isize);
        match_0 = match_0.offset(8isize);
        if oMatchEnd > oend.offset(-((16i32 - 3i32) as isize)) {
            if op < oend_w {
                ZSTD_wildcopy(op as *mut libc::c_void,
                              match_0 as *const libc::c_void,
                              op.offset_to(oend_w).expect("bad offset_to") as
                                  libc::c_long);
                match_0 =
                    match_0.offset(op.offset_to(oend_w).expect("bad offset_to")
                                       as libc::c_long as isize);
                op = oend_w
            }
            while op < oMatchEnd {
                let fresh1 = op;
                op = op.offset(1);
                let fresh0 = match_0;
                match_0 = match_0.offset(1);
                *fresh1 = *fresh0
            }
        } else {
            ZSTD_wildcopy(op as *mut libc::c_void,
                          match_0 as *const libc::c_void,
                          sequence.matchLength as ptrdiff_t -
                              8i32 as libc::c_long);
        }
        return sequenceLength
    };
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
unsafe extern "C" fn ZSTD_copy8(mut dst: *mut libc::c_void,
                                mut src: *const libc::c_void) -> () {
    memcpy(dst, src, 8i32 as libc::c_ulong);
}
/* !
 *  NO_FORWARD_PROGRESS_MAX :
 *  maximum allowed nb of calls to ZSTD_decompressStream() and ZSTD_decompress_generic()
 *  without any forward progress
 *  (defined as: no byte read from input, and no byte flushed to output)
 *  before triggering an error.
 */
/* !
 *  MAXWINDOWSIZE_DEFAULT :
 *  maximum window size accepted by DStream __by default__.
 *  Frames requiring more memory will be rejected.
 *  It's possible to set a different limit using ZSTD_DCtx_setMaxWindowSize().
 */
/* !
*  LEGACY_SUPPORT :
*  if set to 1+, ZSTD_decompress() can decode older formats (v0.1+)
*/
/* !
 * HEAPMODE :
 * Select how default decompression function ZSTD_decompress() allocates its context,
 * on stack (0), or into heap (1, default; requires malloc()).
 * Note that functions with explicit context such as ZSTD_decompressDCtx() are unaffected.
 */
unsafe extern "C" fn ZSTD_copy4(mut dst: *mut libc::c_void,
                                mut src: *const libc::c_void) -> () {
    memcpy(dst, src, 4i32 as libc::c_ulong);
}
unsafe extern "C" fn ZSTD_execSequenceLast7(mut op: *mut BYTE,
                                            oend: *mut BYTE,
                                            mut sequence: seq_t,
                                            mut litPtr: *mut *const BYTE,
                                            litLimit: *const BYTE,
                                            base: *const BYTE,
                                            vBase: *const BYTE,
                                            dictEnd: *const BYTE) -> size_t {
    let oLitEnd: *mut BYTE = op.offset(sequence.litLength as isize);
    let sequenceLength: size_t =
        sequence.litLength.wrapping_add(sequence.matchLength);
    let oMatchEnd: *mut BYTE = op.offset(sequenceLength as isize);
    let oend_w: *mut BYTE = oend.offset(-8isize);
    let iLitEnd: *const BYTE = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const BYTE =
        oLitEnd.offset(-(sequence.offset as isize));
    if oMatchEnd > oend {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else if iLitEnd > litLimit {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    } else if oLitEnd <= oend_w {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        if op < oend_w {
            ZSTD_wildcopy(op as *mut libc::c_void,
                          *litPtr as *const libc::c_void,
                          op.offset_to(oend_w).expect("bad offset_to") as
                              libc::c_long);
            *litPtr =
                (*litPtr).offset(op.offset_to(oend_w).expect("bad offset_to")
                                     as libc::c_long as isize);
            op = oend_w
        }
        while op < oLitEnd {
            let fresh3 = op;
            op = op.offset(1);
            let fresh2 = *litPtr;
            *litPtr = (*litPtr).offset(1);
            *fresh3 = *fresh2
        }
        if sequence.offset >
               base.offset_to(oLitEnd).expect("bad offset_to") as libc::c_long
                   as size_t {
            if sequence.offset >
                   vBase.offset_to(oLitEnd).expect("bad offset_to") as
                       libc::c_long as size_t {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            } else {
                match_0 =
                    dictEnd.offset(-(match_0.offset_to(base).expect("bad offset_to")
                                         as libc::c_long as isize));
                if match_0.offset(sequence.matchLength as isize) <= dictEnd {
                    memmove(oLitEnd as *mut libc::c_void,
                            match_0 as *const libc::c_void,
                            sequence.matchLength);
                    return sequenceLength
                } else {
                    let length1: size_t =
                        match_0.offset_to(dictEnd).expect("bad offset_to") as
                            libc::c_long as size_t;
                    memmove(oLitEnd as *mut libc::c_void,
                            match_0 as *const libc::c_void, length1);
                    op = oLitEnd.offset(length1 as isize);
                    sequence.matchLength =
                        (sequence.matchLength as
                             libc::c_ulong).wrapping_sub(length1) as size_t as
                            size_t;
                    match_0 = base
                }
            }
        }
        while op < oMatchEnd {
            let fresh5 = op;
            op = op.offset(1);
            let fresh4 = match_0;
            match_0 = match_0.offset(1);
            *fresh5 = *fresh4
        }
        return sequenceLength
    };
}
unsafe extern "C" fn ZSTD_initFseState(mut DStatePtr: *mut ZSTD_fseState,
                                       mut bitD: *mut BIT_DStream_t,
                                       mut dt: *const ZSTD_seqSymbol) -> () {
    let mut ptr: *const libc::c_void = dt as *const libc::c_void;
    let DTableH: *const ZSTD_seqSymbol_header =
        ptr as *const ZSTD_seqSymbol_header;
    (*DStatePtr).state = BIT_readBits(bitD, (*DTableH).tableLog);
    BIT_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1isize);
}
unsafe extern "C" fn ZSTD_decompressSequences_bmi2(mut dctx: *mut ZSTD_DCtx,
                                                   mut dst: *mut libc::c_void,
                                                   mut maxDstSize: size_t,
                                                   mut seqStart:
                                                       *const libc::c_void,
                                                   mut seqSize: size_t,
                                                   mut nbSeq: libc::c_int,
                                                   isLongOffset:
                                                       ZSTD_longOffset_e)
 -> size_t {
    return ZSTD_decompressSequences_body(dctx, dst, maxDstSize, seqStart,
                                         seqSize, nbSeq, isLongOffset);
}
unsafe extern "C" fn ZSTD_decompressSequencesLong(mut dctx: *mut ZSTD_DCtx,
                                                  mut dst: *mut libc::c_void,
                                                  mut maxDstSize: size_t,
                                                  mut seqStart:
                                                      *const libc::c_void,
                                                  mut seqSize: size_t,
                                                  mut nbSeq: libc::c_int,
                                                  isLongOffset:
                                                      ZSTD_longOffset_e)
 -> size_t {
    if 0 != (*dctx).bmi2 {
        return ZSTD_decompressSequencesLong_bmi2(dctx, dst, maxDstSize,
                                                 seqStart, seqSize, nbSeq,
                                                 isLongOffset)
    } else {
        return ZSTD_decompressSequencesLong_default(dctx, dst, maxDstSize,
                                                    seqStart, seqSize, nbSeq,
                                                    isLongOffset)
    };
}
unsafe extern "C" fn ZSTD_decompressSequencesLong_default(mut dctx:
                                                              *mut ZSTD_DCtx,
                                                          mut dst:
                                                              *mut libc::c_void,
                                                          mut maxDstSize:
                                                              size_t,
                                                          mut seqStart:
                                                              *const libc::c_void,
                                                          mut seqSize: size_t,
                                                          mut nbSeq:
                                                              libc::c_int,
                                                          isLongOffset:
                                                              ZSTD_longOffset_e)
 -> size_t {
    return ZSTD_decompressSequencesLong_body(dctx, dst, maxDstSize, seqStart,
                                             seqSize, nbSeq, isLongOffset);
}
unsafe extern "C" fn ZSTD_decompressSequencesLong_body(mut dctx:
                                                           *mut ZSTD_DCtx,
                                                       mut dst:
                                                           *mut libc::c_void,
                                                       mut maxDstSize: size_t,
                                                       mut seqStart:
                                                           *const libc::c_void,
                                                       mut seqSize: size_t,
                                                       mut nbSeq: libc::c_int,
                                                       isLongOffset:
                                                           ZSTD_longOffset_e)
 -> size_t {
    let mut ip: *const BYTE = seqStart as *const BYTE;
    let iend: *const BYTE = ip.offset(seqSize as isize);
    let ostart: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = ostart.offset(maxDstSize as isize);
    let mut op: *mut BYTE = ostart;
    let mut litPtr: *const BYTE = (*dctx).litPtr;
    let litEnd: *const BYTE = litPtr.offset((*dctx).litSize as isize);
    let prefixStart: *const BYTE = (*dctx).prefixStart as *const BYTE;
    let dictStart: *const BYTE = (*dctx).virtualStart as *const BYTE;
    let dictEnd: *const BYTE = (*dctx).dictEnd as *const BYTE;
    if 0 != nbSeq {
        let mut sequences: [seq_t; 4] =
            [seq_t{litLength: 0,
                   matchLength: 0,
                   offset: 0,
                   match_0: 0 as *const BYTE,}; 4];
        let seqAdvance: libc::c_int = if nbSeq < 4i32 { nbSeq } else { 4i32 };
        let mut seqState: seqState_t =
            seqState_t{DStream:
                           BIT_DStream_t{bitContainer: 0,
                                         bitsConsumed: 0,
                                         ptr: 0 as *const libc::c_char,
                                         start: 0 as *const libc::c_char,
                                         limitPtr: 0 as *const libc::c_char,},
                       stateLL:
                           ZSTD_fseState{state: 0,
                                         table: 0 as *const ZSTD_seqSymbol,},
                       stateOffb:
                           ZSTD_fseState{state: 0,
                                         table: 0 as *const ZSTD_seqSymbol,},
                       stateML:
                           ZSTD_fseState{state: 0,
                                         table: 0 as *const ZSTD_seqSymbol,},
                       prevOffset: [0; 3],
                       prefixStart: 0 as *const BYTE,
                       dictEnd: 0 as *const BYTE,
                       pos: 0,};
        let mut seqNb: libc::c_int = 0;
        (*dctx).fseEntropy = 1i32 as U32;
        let mut i: U32 = 0;
        i = 0i32 as U32;
        while i < 3i32 as libc::c_uint {
            seqState.prevOffset[i as usize] =
                (*dctx).entropy.rep[i as usize] as size_t;
            i = i.wrapping_add(1)
        }
        seqState.prefixStart = prefixStart;
        seqState.pos =
            prefixStart.offset_to(op).expect("bad offset_to") as libc::c_long
                as size_t;
        seqState.dictEnd = dictEnd;
        let errcod: size_t =
            BIT_initDStream(&mut seqState.DStream as *mut BIT_DStream_t,
                            ip as *const libc::c_void,
                            ip.offset_to(iend).expect("bad offset_to") as
                                libc::c_long as size_t);
        if 0 != ERR_isError(errcod) {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        } else {
            ZSTD_initFseState(&mut seqState.stateLL as *mut ZSTD_fseState,
                              &mut seqState.DStream as *mut BIT_DStream_t,
                              (*dctx).LLTptr);
            ZSTD_initFseState(&mut seqState.stateOffb as *mut ZSTD_fseState,
                              &mut seqState.DStream as *mut BIT_DStream_t,
                              (*dctx).OFTptr);
            ZSTD_initFseState(&mut seqState.stateML as *mut ZSTD_fseState,
                              &mut seqState.DStream as *mut BIT_DStream_t,
                              (*dctx).MLTptr);
            seqNb = 0i32;
            while BIT_reloadDStream(&mut seqState.DStream as
                                        *mut BIT_DStream_t) as libc::c_uint <=
                      BIT_DStream_completed as libc::c_int as libc::c_uint &&
                      seqNb < seqAdvance {
                sequences[seqNb as usize] =
                    ZSTD_decodeSequenceLong(&mut seqState as *mut seqState_t,
                                            isLongOffset);
                seqNb += 1
            }
            if seqNb < seqAdvance {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            } else {
                while BIT_reloadDStream(&mut seqState.DStream as
                                            *mut BIT_DStream_t) as
                          libc::c_uint <=
                          BIT_DStream_completed as libc::c_int as libc::c_uint
                          && seqNb < nbSeq {
                    let sequence: seq_t =
                        ZSTD_decodeSequenceLong(&mut seqState as
                                                    *mut seqState_t,
                                                isLongOffset);
                    let oneSeqSize: size_t =
                        ZSTD_execSequenceLong(op, oend,
                                              sequences[(seqNb - 4i32 &
                                                             4i32 - 1i32) as
                                                            usize],
                                              &mut litPtr as *mut *const BYTE,
                                              litEnd, prefixStart, dictStart,
                                              dictEnd);
                    if 0 != ERR_isError(oneSeqSize) {
                        return oneSeqSize
                    } else {
                        sequences[(seqNb & 4i32 - 1i32) as usize] = sequence;
                        op = op.offset(oneSeqSize as isize);
                        seqNb += 1
                    }
                }
                if seqNb < nbSeq {
                    return -(ZSTD_error_corruption_detected as libc::c_int) as
                               size_t
                } else {
                    seqNb -= seqAdvance;
                    while seqNb < nbSeq {
                        let oneSeqSize_0: size_t =
                            ZSTD_execSequenceLong(op, oend,
                                                  sequences[(seqNb &
                                                                 4i32 - 1i32)
                                                                as usize],
                                                  &mut litPtr as
                                                      *mut *const BYTE,
                                                  litEnd, prefixStart,
                                                  dictStart, dictEnd);
                        if 0 != ERR_isError(oneSeqSize_0) {
                            return oneSeqSize_0
                        } else {
                            op = op.offset(oneSeqSize_0 as isize);
                            seqNb += 1
                        }
                    }
                    let mut i_0: U32 = 0;
                    i_0 = 0i32 as U32;
                    while i_0 < 3i32 as libc::c_uint {
                        (*dctx).entropy.rep[i_0 as usize] =
                            seqState.prevOffset[i_0 as usize] as U32;
                        i_0 = i_0.wrapping_add(1)
                    }
                }
            }
        }
    }
    let lastLLSize: size_t =
        litPtr.offset_to(litEnd).expect("bad offset_to") as libc::c_long as
            size_t;
    if lastLLSize >
           op.offset_to(oend).expect("bad offset_to") as libc::c_long as
               size_t {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else {
        memcpy(op as *mut libc::c_void, litPtr as *const libc::c_void,
               lastLLSize);
        op = op.offset(lastLLSize as isize);
        return ostart.offset_to(op).expect("bad offset_to") as libc::c_long as
                   size_t
    };
}
unsafe extern "C" fn ZSTD_execSequenceLong(mut op: *mut BYTE, oend: *mut BYTE,
                                           mut sequence: seq_t,
                                           mut litPtr: *mut *const BYTE,
                                           litLimit: *const BYTE,
                                           prefixStart: *const BYTE,
                                           dictStart: *const BYTE,
                                           dictEnd: *const BYTE) -> size_t {
    static mut dec64table: [libc::c_int; 8] =
        unsafe { [8i32, 8i32, 8i32, 7i32, 8i32, 9i32, 10i32, 11i32] };
    let mut sub2: libc::c_int = 0;
    static mut dec32table: [U32; 8] =
        unsafe {
            [0i32 as U32, 1i32 as U32, 2i32 as U32, 1i32 as U32, 4i32 as U32,
             4i32 as U32, 4i32 as U32, 4i32 as U32]
        };
    let oLitEnd: *mut BYTE = op.offset(sequence.litLength as isize);
    let sequenceLength: size_t =
        sequence.litLength.wrapping_add(sequence.matchLength);
    let oMatchEnd: *mut BYTE = op.offset(sequenceLength as isize);
    let oend_w: *mut BYTE = oend.offset(-8isize);
    let iLitEnd: *const BYTE = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const BYTE = sequence.match_0;
    if oMatchEnd > oend {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else if iLitEnd > litLimit {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    } else if oLitEnd > oend_w {
        return ZSTD_execSequenceLast7(op, oend, sequence, litPtr, litLimit,
                                      prefixStart, dictStart, dictEnd)
    } else {
        ZSTD_copy8(op as *mut libc::c_void, *litPtr as *const libc::c_void);
        if sequence.litLength > 8i32 as libc::c_ulong {
            ZSTD_wildcopy(op.offset(8isize) as *mut libc::c_void,
                          (*litPtr).offset(8isize) as *const libc::c_void,
                          sequence.litLength.wrapping_sub(8i32 as
                                                              libc::c_ulong)
                              as ptrdiff_t);
        }
        op = oLitEnd;
        *litPtr = iLitEnd;
        if sequence.offset >
               prefixStart.offset_to(oLitEnd).expect("bad offset_to") as
                   libc::c_long as size_t {
            if sequence.offset >
                   dictStart.offset_to(oLitEnd).expect("bad offset_to") as
                       libc::c_long as size_t {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            } else if match_0.offset(sequence.matchLength as isize) <= dictEnd
             {
                memmove(oLitEnd as *mut libc::c_void,
                        match_0 as *const libc::c_void, sequence.matchLength);
                return sequenceLength
            } else {
                let length1: size_t =
                    match_0.offset_to(dictEnd).expect("bad offset_to") as
                        libc::c_long as size_t;
                memmove(oLitEnd as *mut libc::c_void,
                        match_0 as *const libc::c_void, length1);
                op = oLitEnd.offset(length1 as isize);
                sequence.matchLength =
                    (sequence.matchLength as
                         libc::c_ulong).wrapping_sub(length1) as size_t as
                        size_t;
                match_0 = prefixStart;
                if op > oend_w || sequence.matchLength < 3i32 as libc::c_ulong
                   {
                    let mut i: U32 = 0;
                    i = 0i32 as U32;
                    while (i as libc::c_ulong) < sequence.matchLength {
                        *op.offset(i as isize) = *match_0.offset(i as isize);
                        i = i.wrapping_add(1)
                    }
                    return sequenceLength
                }
            }
        }
        if sequence.offset < 8i32 as libc::c_ulong {
            sub2 = dec64table[sequence.offset as usize];
            *op.offset(0isize) = *match_0.offset(0isize);
            *op.offset(1isize) = *match_0.offset(1isize);
            *op.offset(2isize) = *match_0.offset(2isize);
            *op.offset(3isize) = *match_0.offset(3isize);
            match_0 =
                match_0.offset(dec32table[sequence.offset as usize] as isize);
            ZSTD_copy4(op.offset(4isize) as *mut libc::c_void,
                       match_0 as *const libc::c_void);
            match_0 = match_0.offset(-(sub2 as isize))
        } else {
            ZSTD_copy8(op as *mut libc::c_void,
                       match_0 as *const libc::c_void);
        }
        op = op.offset(8isize);
        match_0 = match_0.offset(8isize);
        if oMatchEnd > oend.offset(-((16i32 - 3i32) as isize)) {
            if op < oend_w {
                ZSTD_wildcopy(op as *mut libc::c_void,
                              match_0 as *const libc::c_void,
                              op.offset_to(oend_w).expect("bad offset_to") as
                                  libc::c_long);
                match_0 =
                    match_0.offset(op.offset_to(oend_w).expect("bad offset_to")
                                       as libc::c_long as isize);
                op = oend_w
            }
            while op < oMatchEnd {
                let fresh7 = op;
                op = op.offset(1);
                let fresh6 = match_0;
                match_0 = match_0.offset(1);
                *fresh7 = *fresh6
            }
        } else {
            ZSTD_wildcopy(op as *mut libc::c_void,
                          match_0 as *const libc::c_void,
                          sequence.matchLength as ptrdiff_t -
                              8i32 as libc::c_long);
        }
        return sequenceLength
    };
}
unsafe extern "C" fn ZSTD_decodeSequenceLong(mut seqState: *mut seqState_t,
                                             longOffsets: ZSTD_longOffset_e)
 -> seq_t {
    let mut seq: seq_t =
        seq_t{litLength: 0,
              matchLength: 0,
              offset: 0,
              match_0: 0 as *const BYTE,};
    let llBits: U32 =
        (*(*seqState).stateLL.table.offset((*seqState).stateLL.state as
                                               isize)).nbAdditionalBits as
            U32;
    let mlBits: U32 =
        (*(*seqState).stateML.table.offset((*seqState).stateML.state as
                                               isize)).nbAdditionalBits as
            U32;
    let ofBits: U32 =
        (*(*seqState).stateOffb.table.offset((*seqState).stateOffb.state as
                                                 isize)).nbAdditionalBits as
            U32;
    let totalBits: U32 = llBits.wrapping_add(mlBits).wrapping_add(ofBits);
    let llBase: U32 =
        (*(*seqState).stateLL.table.offset((*seqState).stateLL.state as
                                               isize)).baseValue;
    let mlBase: U32 =
        (*(*seqState).stateML.table.offset((*seqState).stateML.state as
                                               isize)).baseValue;
    let ofBase: U32 =
        (*(*seqState).stateOffb.table.offset((*seqState).stateOffb.state as
                                                 isize)).baseValue;
    let mut offset: size_t = 0;
    if 0 == ofBits {
        offset = 0i32 as size_t
    } else if 0 != MEM_32bits() && 0 != longOffsets as libc::c_uint {
        let extraBits: U32 =
            ofBits.wrapping_sub(if ofBits < (25i32 - 1i32) as libc::c_uint {
                                    ofBits
                                } else { (25i32 - 1i32) as libc::c_uint });
        offset =
            (ofBase as
                 libc::c_ulong).wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream
                                                                  as
                                                                  *mut BIT_DStream_t,
                                                              ofBits.wrapping_sub(extraBits))
                                                 << extraBits);
        if 0 != MEM_32bits() || 0 != extraBits {
            BIT_reloadDStream(&mut (*seqState).DStream as *mut BIT_DStream_t);
        }
        if 0 != extraBits {
            offset =
                (offset as
                     libc::c_ulong).wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream
                                                                      as
                                                                      *mut BIT_DStream_t,
                                                                  extraBits))
                    as size_t as size_t
        }
    } else {
        offset =
            (ofBase as
                 libc::c_ulong).wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream
                                                                  as
                                                                  *mut BIT_DStream_t,
                                                              ofBits));
        if 0 != MEM_32bits() {
            BIT_reloadDStream(&mut (*seqState).DStream as *mut BIT_DStream_t);
        }
    }
    if ofBits <= 1i32 as libc::c_uint {
        offset =
            (offset as
                 libc::c_ulong).wrapping_add((llBase == 0i32 as libc::c_uint)
                                                 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t;
        if 0 != offset {
            let mut temp: size_t =
                if offset == 3i32 as libc::c_ulong {
                    (*seqState).prevOffset[0usize].wrapping_sub(1i32 as
                                                                    libc::c_ulong)
                } else { (*seqState).prevOffset[offset as usize] };
            temp =
                (temp as
                     libc::c_ulong).wrapping_add((0 == temp) as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t;
            if offset != 1i32 as libc::c_ulong {
                (*seqState).prevOffset[2usize] =
                    (*seqState).prevOffset[1usize]
            }
            (*seqState).prevOffset[1usize] = (*seqState).prevOffset[0usize];
            offset = temp;
            (*seqState).prevOffset[0usize] = offset
        } else { offset = (*seqState).prevOffset[0usize] }
    } else {
        (*seqState).prevOffset[2usize] = (*seqState).prevOffset[1usize];
        (*seqState).prevOffset[1usize] = (*seqState).prevOffset[0usize];
        (*seqState).prevOffset[0usize] = offset
    }
    seq.offset = offset;
    seq.matchLength =
        (mlBase as
             libc::c_ulong).wrapping_add(if mlBits > 0i32 as libc::c_uint {
                                             BIT_readBitsFast(&mut (*seqState).DStream
                                                                  as
                                                                  *mut BIT_DStream_t,
                                                              mlBits)
                                         } else { 0i32 as libc::c_ulong });
    if 0 != MEM_32bits() &&
           mlBits.wrapping_add(llBits) >=
               (25i32 - if 30i32 > 25i32 { 30i32 - 25i32 } else { 0i32 }) as
                   libc::c_uint {
        BIT_reloadDStream(&mut (*seqState).DStream as *mut BIT_DStream_t);
    }
    if 0 != MEM_64bits() &&
           totalBits >= (57i32 - (9i32 + 9i32 + 8i32)) as libc::c_uint {
        BIT_reloadDStream(&mut (*seqState).DStream as *mut BIT_DStream_t);
    }
    seq.litLength =
        (llBase as
             libc::c_ulong).wrapping_add(if llBits > 0i32 as libc::c_uint {
                                             BIT_readBitsFast(&mut (*seqState).DStream
                                                                  as
                                                                  *mut BIT_DStream_t,
                                                              llBits)
                                         } else { 0i32 as libc::c_ulong });
    if 0 != MEM_32bits() {
        BIT_reloadDStream(&mut (*seqState).DStream as *mut BIT_DStream_t);
    }
    let pos: size_t = (*seqState).pos.wrapping_add(seq.litLength);
    let matchBase: *const BYTE =
        if seq.offset > pos {
            (*seqState).dictEnd
        } else { (*seqState).prefixStart };
    seq.match_0 =
        matchBase.offset(pos as isize).offset(-(seq.offset as isize));
    (*seqState).pos = pos.wrapping_add(seq.matchLength);
    ZSTD_updateFseState(&mut (*seqState).stateLL as *mut ZSTD_fseState,
                        &mut (*seqState).DStream as *mut BIT_DStream_t);
    ZSTD_updateFseState(&mut (*seqState).stateML as *mut ZSTD_fseState,
                        &mut (*seqState).DStream as *mut BIT_DStream_t);
    if 0 != MEM_32bits() {
        BIT_reloadDStream(&mut (*seqState).DStream as *mut BIT_DStream_t);
    }
    ZSTD_updateFseState(&mut (*seqState).stateOffb as *mut ZSTD_fseState,
                        &mut (*seqState).DStream as *mut BIT_DStream_t);
    return seq;
}
unsafe extern "C" fn ZSTD_decompressSequencesLong_bmi2(mut dctx:
                                                           *mut ZSTD_DCtx,
                                                       mut dst:
                                                           *mut libc::c_void,
                                                       mut maxDstSize: size_t,
                                                       mut seqStart:
                                                           *const libc::c_void,
                                                       mut seqSize: size_t,
                                                       mut nbSeq: libc::c_int,
                                                       isLongOffset:
                                                           ZSTD_longOffset_e)
 -> size_t {
    return ZSTD_decompressSequencesLong_body(dctx, dst, maxDstSize, seqStart,
                                             seqSize, nbSeq, isLongOffset);
}
unsafe extern "C" fn ZSTD_getLongOffsetsShare(mut offTable:
                                                  *const ZSTD_seqSymbol)
 -> libc::c_uint {
    let mut ptr: *const libc::c_void = offTable as *const libc::c_void;
    let tableLog: U32 =
        (*(ptr as *const ZSTD_seqSymbol_header).offset(0isize)).tableLog;
    let mut table: *const ZSTD_seqSymbol = offTable.offset(1isize);
    let max: U32 = (1i32 << tableLog) as U32;
    let mut u: U32 = 0;
    let mut total: U32 = 0i32 as U32;
    u = 0i32 as U32;
    while u < max {
        if (*table.offset(u as isize)).nbAdditionalBits as libc::c_int > 22i32
           {
            total =
                (total as libc::c_uint).wrapping_add(1i32 as libc::c_uint) as
                    U32 as U32
        }
        u = u.wrapping_add(1)
    }
    total <<= (8i32 as libc::c_uint).wrapping_sub(tableLog);
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decodeSeqHeaders(mut dctx: *mut ZSTD_DCtx,
                                               mut nbSeqPtr: *mut libc::c_int,
                                               mut src: *const libc::c_void,
                                               mut srcSize: size_t)
 -> size_t {
    let istart: *const BYTE = src as *const BYTE;
    let iend: *const BYTE = istart.offset(srcSize as isize);
    let mut ip: *const BYTE = istart;
    if srcSize < 1i32 as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else {
        let fresh8 = ip;
        ip = ip.offset(1);
        let mut nbSeq: libc::c_int = *fresh8 as libc::c_int;
        if 0 == nbSeq {
            *nbSeqPtr = 0i32;
            return 1i32 as size_t
        } else {
            if nbSeq > 127i32 {
                if nbSeq == 255i32 {
                    if ip.offset(2isize) > iend {
                        return -(ZSTD_error_srcSize_wrong as libc::c_int) as
                                   size_t
                    } else {
                        nbSeq =
                            MEM_readLE16(ip as *const libc::c_void) as
                                libc::c_int + 32512i32;
                        ip = ip.offset(2isize)
                    }
                } else if ip >= iend {
                    return -(ZSTD_error_srcSize_wrong as libc::c_int) as
                               size_t
                } else {
                    let fresh9 = ip;
                    ip = ip.offset(1);
                    nbSeq = (nbSeq - 128i32 << 8i32) + *fresh9 as libc::c_int
                }
            }
            *nbSeqPtr = nbSeq;
            if ip.offset(4isize) > iend {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
            } else {
                let LLtype: symbolEncodingType_e =
                    (*ip as libc::c_int >> 6i32) as symbolEncodingType_e;
                let OFtype: symbolEncodingType_e =
                    (*ip as libc::c_int >> 4i32 & 3i32) as
                        symbolEncodingType_e;
                let MLtype: symbolEncodingType_e =
                    (*ip as libc::c_int >> 2i32 & 3i32) as
                        symbolEncodingType_e;
                ip = ip.offset(1isize);
                let llhSize: size_t =
                    ZSTD_buildSeqTable((*dctx).entropy.LLTable.as_mut_ptr(),
                                       &mut (*dctx).LLTptr as
                                           *mut *const ZSTD_seqSymbol, LLtype,
                                       35i32 as U32, 9i32 as U32,
                                       ip as *const libc::c_void,
                                       ip.offset_to(iend).expect("bad offset_to")
                                           as libc::c_long as size_t,
                                       LL_base.as_ptr(), LL_bits.as_ptr(),
                                       LL_defaultDTable.as_ptr(),
                                       (*dctx).fseEntropy);
                if 0 != ERR_isError(llhSize) {
                    return -(ZSTD_error_corruption_detected as libc::c_int) as
                               size_t
                } else {
                    ip = ip.offset(llhSize as isize);
                    let ofhSize: size_t =
                        ZSTD_buildSeqTable((*dctx).entropy.OFTable.as_mut_ptr(),
                                           &mut (*dctx).OFTptr as
                                               *mut *const ZSTD_seqSymbol,
                                           OFtype, 31i32 as U32, 8i32 as U32,
                                           ip as *const libc::c_void,
                                           ip.offset_to(iend).expect("bad offset_to")
                                               as libc::c_long as size_t,
                                           OF_base.as_ptr(), OF_bits.as_ptr(),
                                           OF_defaultDTable.as_ptr(),
                                           (*dctx).fseEntropy);
                    if 0 != ERR_isError(ofhSize) {
                        return -(ZSTD_error_corruption_detected as
                                     libc::c_int) as size_t
                    } else {
                        ip = ip.offset(ofhSize as isize);
                        let mlhSize: size_t =
                            ZSTD_buildSeqTable((*dctx).entropy.MLTable.as_mut_ptr(),
                                               &mut (*dctx).MLTptr as
                                                   *mut *const ZSTD_seqSymbol,
                                               MLtype, 52i32 as U32,
                                               9i32 as U32,
                                               ip as *const libc::c_void,
                                               ip.offset_to(iend).expect("bad offset_to")
                                                   as libc::c_long as size_t,
                                               ML_base.as_ptr(),
                                               ML_bits.as_ptr(),
                                               ML_defaultDTable.as_ptr(),
                                               (*dctx).fseEntropy);
                        if 0 != ERR_isError(mlhSize) {
                            return -(ZSTD_error_corruption_detected as
                                         libc::c_int) as size_t
                        } else {
                            ip = ip.offset(mlhSize as isize);
                            return istart.offset_to(ip).expect("bad offset_to")
                                       as libc::c_long as size_t
                        }
                    }
                }
            }
        }
    };
}
static mut ML_defaultDTable: [ZSTD_seqSymbol; 65] =
    unsafe {
        [ZSTD_seqSymbol{nextState: 1i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 1i32 as BYTE,
                        baseValue: 6i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 3i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 4i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 5i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 6i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 8i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 9i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 11i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 13i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 16i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 19i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 22i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 25i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 28i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 31i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 34i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 37i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 41i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 2i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 47i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 3i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 59i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 4i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 83i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 7i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 131i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 9i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 515i32 as U32,},
         ZSTD_seqSymbol{nextState: 16i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 4i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 5i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 6i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 7i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 9i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 10i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 12i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 15i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 18i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 21i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 24i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 27i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 30i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 33i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 35i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 39i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 2i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 43i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 3i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 51i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 4i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 67i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 5i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 99i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 8i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 259i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 4i32 as U32,},
         ZSTD_seqSymbol{nextState: 48i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 4i32 as U32,},
         ZSTD_seqSymbol{nextState: 16i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 5i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 7i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 8i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 10i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 11i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 14i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 17i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 20i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 23i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 26i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 29i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 32i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 16i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 65539i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 15i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 32771i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 14i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 16387i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 13i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 8195i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 12i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 4099i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 11i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 2051i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 10i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 1027i32 as U32,}]
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
static mut ML_base: [U32; 53] =
    unsafe {
        [3i32 as U32, 4i32 as U32, 5i32 as U32, 6i32 as U32, 7i32 as U32,
         8i32 as U32, 9i32 as U32, 10i32 as U32, 11i32 as U32, 12i32 as U32,
         13i32 as U32, 14i32 as U32, 15i32 as U32, 16i32 as U32, 17i32 as U32,
         18i32 as U32, 19i32 as U32, 20i32 as U32, 21i32 as U32, 22i32 as U32,
         23i32 as U32, 24i32 as U32, 25i32 as U32, 26i32 as U32, 27i32 as U32,
         28i32 as U32, 29i32 as U32, 30i32 as U32, 31i32 as U32, 32i32 as U32,
         33i32 as U32, 34i32 as U32, 35i32 as U32, 37i32 as U32, 39i32 as U32,
         41i32 as U32, 43i32 as U32, 47i32 as U32, 51i32 as U32, 59i32 as U32,
         67i32 as U32, 83i32 as U32, 99i32 as U32, 131i32 as U32,
         259i32 as U32, 515i32 as U32, 1027i32 as U32, 2051i32 as U32,
         4099i32 as U32, 8195i32 as U32, 16387i32 as U32, 32771i32 as U32,
         65539i32 as U32]
    };
/* ! ZSTD_buildSeqTable() :
 * @return : nb bytes read from src,
 *           or an error code if it fails */
unsafe extern "C" fn ZSTD_buildSeqTable(mut DTableSpace: *mut ZSTD_seqSymbol,
                                        mut DTablePtr:
                                            *mut *const ZSTD_seqSymbol,
                                        mut type_0: symbolEncodingType_e,
                                        mut max: U32, mut maxLog: U32,
                                        mut src: *const libc::c_void,
                                        mut srcSize: size_t,
                                        mut baseValue: *const U32,
                                        mut nbAdditionalBits: *const U32,
                                        mut defaultTable:
                                            *const ZSTD_seqSymbol,
                                        mut flagRepeatTable: U32) -> size_t {
    let mut nbBits: U32 = 0;
    let mut baseline: U32 = 0;
    let mut symbol: U32 = 0;
    match type_0 as libc::c_uint {
        1 => {
            if 0 == srcSize {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
            } else if *(src as *const BYTE) as libc::c_uint > max {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            } else {
                symbol = *(src as *const BYTE) as U32;
                baseline = *baseValue.offset(symbol as isize);
                nbBits = *nbAdditionalBits.offset(symbol as isize);
                ZSTD_buildSeqTable_rle(DTableSpace, baseline, nbBits);
                *DTablePtr = DTableSpace;
                return 1i32 as size_t
            }
        }
        0 => { *DTablePtr = defaultTable; return 0i32 as size_t }
        3 => {
            if 0 == flagRepeatTable {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            } else { return 0i32 as size_t }
        }
        2 => {
            let mut tableLog: U32 = 0;
            let mut norm: [S16; 53] = [0; 53];
            let headerSize: size_t =
                FSE_readNCount(norm.as_mut_ptr(), &mut max as *mut U32,
                               &mut tableLog as *mut U32, src, srcSize);
            if 0 != ERR_isError(headerSize) {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            } else if tableLog > maxLog {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            } else {
                ZSTD_buildFSETable(DTableSpace, norm.as_mut_ptr(), max,
                                   baseValue, nbAdditionalBits, tableLog);
                *DTablePtr = DTableSpace;
                return headerSize
            }
        }
        _ => { return -(ZSTD_error_GENERIC as libc::c_int) as size_t }
    };
}
unsafe extern "C" fn ZSTD_buildFSETable(mut dt: *mut ZSTD_seqSymbol,
                                        mut normalizedCounter:
                                            *const libc::c_short,
                                        mut maxSymbolValue: libc::c_uint,
                                        mut baseValue: *const U32,
                                        mut nbAdditionalBits: *const U32,
                                        mut tableLog: libc::c_uint) -> () {
    let tableDecode: *mut ZSTD_seqSymbol = dt.offset(1isize);
    let mut symbolNext: [U16; 53] = [0; 53];
    let maxSV1: U32 = maxSymbolValue.wrapping_add(1i32 as libc::c_uint);
    let tableSize: U32 = (1i32 << tableLog) as U32;
    let mut highThreshold: U32 = tableSize.wrapping_sub(1i32 as libc::c_uint);
    let mut DTableH: ZSTD_seqSymbol_header =
        ZSTD_seqSymbol_header{fastMode: 0, tableLog: 0,};
    DTableH.tableLog = tableLog;
    DTableH.fastMode = 1i32 as U32;
    let largeLimit: S16 =
        (1i32 << tableLog.wrapping_sub(1i32 as libc::c_uint)) as S16;
    let mut s: U32 = 0;
    s = 0i32 as U32;
    while s < maxSV1 {
        if *normalizedCounter.offset(s as isize) as libc::c_int == -1i32 {
            let fresh10 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh10 as isize)).baseValue = s;
            symbolNext[s as usize] = 1i32 as U16
        } else {
            if *normalizedCounter.offset(s as isize) as libc::c_int >=
                   largeLimit as libc::c_int {
                DTableH.fastMode = 0i32 as U32
            }
            symbolNext[s as usize] =
                *normalizedCounter.offset(s as isize) as U16
        }
        s = s.wrapping_add(1)
    }
    memcpy(dt as *mut libc::c_void,
           &mut DTableH as *mut ZSTD_seqSymbol_header as *const libc::c_void,
           ::std::mem::size_of::<ZSTD_seqSymbol_header>() as libc::c_ulong);
    let tableMask: U32 = tableSize.wrapping_sub(1i32 as libc::c_uint);
    let step: U32 =
        (tableSize >>
             1i32).wrapping_add(tableSize >>
                                    3i32).wrapping_add(3i32 as libc::c_uint);
    let mut s_0: U32 = 0;
    let mut position: U32 = 0i32 as U32;
    s_0 = 0i32 as U32;
    while s_0 < maxSV1 {
        let mut i: libc::c_int = 0;
        i = 0i32;
        while i < *normalizedCounter.offset(s_0 as isize) as libc::c_int {
            (*tableDecode.offset(position as isize)).baseValue = s_0;
            position = position.wrapping_add(step) & tableMask;
            while position > highThreshold {
                position = position.wrapping_add(step) & tableMask
            }
            i += 1
        }
        s_0 = s_0.wrapping_add(1)
    }
    let mut u: U32 = 0;
    u = 0i32 as U32;
    while u < tableSize {
        let symbol: U32 = (*tableDecode.offset(u as isize)).baseValue;
        let fresh11 = symbolNext[symbol as usize];
        symbolNext[symbol as usize] =
            symbolNext[symbol as usize].wrapping_add(1);
        let nextState: U32 = fresh11 as U32;
        (*tableDecode.offset(u as isize)).nbBits =
            tableLog.wrapping_sub(BIT_highbit32(nextState)) as BYTE;
        (*tableDecode.offset(u as isize)).nextState =
            (nextState <<
                 (*tableDecode.offset(u as isize)).nbBits as
                     libc::c_int).wrapping_sub(tableSize) as U16;
        (*tableDecode.offset(u as isize)).nbAdditionalBits =
            *nbAdditionalBits.offset(symbol as isize) as BYTE;
        (*tableDecode.offset(u as isize)).baseValue =
            *baseValue.offset(symbol as isize);
        u = u.wrapping_add(1)
    };
}
unsafe extern "C" fn ZSTD_buildSeqTable_rle(mut dt: *mut ZSTD_seqSymbol,
                                            mut baseValue: U32,
                                            mut nbAddBits: U32) -> () {
    let mut ptr: *mut libc::c_void = dt as *mut libc::c_void;
    let DTableH: *mut ZSTD_seqSymbol_header =
        ptr as *mut ZSTD_seqSymbol_header;
    let cell: *mut ZSTD_seqSymbol = dt.offset(1isize);
    (*DTableH).tableLog = 0i32 as U32;
    (*DTableH).fastMode = 0i32 as U32;
    (*cell).nbBits = 0i32 as BYTE;
    (*cell).nextState = 0i32 as U16;
    (*cell).nbAdditionalBits = nbAddBits as BYTE;
    (*cell).baseValue = baseValue;
}
static mut OF_defaultDTable: [ZSTD_seqSymbol; 33] =
    unsafe {
        [ZSTD_seqSymbol{nextState: 1i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 1i32 as BYTE,
                        baseValue: 5i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 0i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 6i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 61i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 9i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 509i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 15i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 32765i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 21i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 2097149i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 3i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 5i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 7i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 125i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 12i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 4093i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 18i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 262141i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 23i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 8388605i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 5i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 29i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 8i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 253i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 14i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 16381i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 20i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 1048573i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 2i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 1i32 as U32,},
         ZSTD_seqSymbol{nextState: 16i32 as U16,
                        nbAdditionalBits: 7i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 125i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 11i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 2045i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 17i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 131069i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 22i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 4194301i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 4i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 13i32 as U32,},
         ZSTD_seqSymbol{nextState: 16i32 as U16,
                        nbAdditionalBits: 8i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 253i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 13i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 8189i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 19i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 524285i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 1i32 as U32,},
         ZSTD_seqSymbol{nextState: 16i32 as U16,
                        nbAdditionalBits: 6i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 61i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 10i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 1021i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 16i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 65533i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 28i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 268435453i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 27i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 134217725i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 26i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 67108861i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 25i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 33554429i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 24i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 16777213i32 as U32,}]
    };
static mut OF_bits: [U32; 32] =
    unsafe {
        [0i32 as U32, 1i32 as U32, 2i32 as U32, 3i32 as U32, 4i32 as U32,
         5i32 as U32, 6i32 as U32, 7i32 as U32, 8i32 as U32, 9i32 as U32,
         10i32 as U32, 11i32 as U32, 12i32 as U32, 13i32 as U32, 14i32 as U32,
         15i32 as U32, 16i32 as U32, 17i32 as U32, 18i32 as U32, 19i32 as U32,
         20i32 as U32, 21i32 as U32, 22i32 as U32, 23i32 as U32, 24i32 as U32,
         25i32 as U32, 26i32 as U32, 27i32 as U32, 28i32 as U32, 29i32 as U32,
         30i32 as U32, 31i32 as U32]
    };
static mut OF_base: [U32; 32] =
    unsafe {
        [0i32 as U32, 1i32 as U32, 1i32 as U32, 5i32 as U32, 13i32 as U32,
         29i32 as U32, 61i32 as U32, 125i32 as U32, 253i32 as U32,
         509i32 as U32, 1021i32 as U32, 2045i32 as U32, 4093i32 as U32,
         8189i32 as U32, 16381i32 as U32, 32765i32 as U32, 65533i32 as U32,
         131069i32 as U32, 262141i32 as U32, 524285i32 as U32,
         1048573i32 as U32, 2097149i32 as U32, 4194301i32 as U32,
         8388605i32 as U32, 16777213i32 as U32, 33554429i32 as U32,
         67108861i32 as U32, 134217725i32 as U32, 268435453i32 as U32,
         536870909i32 as U32, 1073741821i32 as U32, 2147483645i32 as U32]
    };
static mut LL_defaultDTable: [ZSTD_seqSymbol; 65] =
    unsafe {
        [ZSTD_seqSymbol{nextState: 1i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 1i32 as BYTE,
                        baseValue: 6i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 0i32 as U32,},
         ZSTD_seqSymbol{nextState: 16i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 0i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 1i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 3i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 4i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 6i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 7i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 9i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 10i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 12i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 14i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 16i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 20i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 22i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 2i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 28i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 3i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 32i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 4i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 48i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 6i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 64i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 7i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 128i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 8i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 256i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 10i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 1024i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 12i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 4096i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 0i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 1i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 2i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 4i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 5i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 7i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 8i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 10i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 11i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 13i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 16i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 18i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 22i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 2i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 24i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 3i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 32i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 3i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 40i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 6i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 64i32 as U32,},
         ZSTD_seqSymbol{nextState: 16i32 as U16,
                        nbAdditionalBits: 6i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 64i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 7i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 128i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 9i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 512i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 11i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 2048i32 as U32,},
         ZSTD_seqSymbol{nextState: 48i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 0i32 as U32,},
         ZSTD_seqSymbol{nextState: 16i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 4i32 as BYTE,
                        baseValue: 1i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 2i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 3i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 5i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 6i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 8i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 9i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 11i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 12i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 0i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 15i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 18i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 1i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 20i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 2i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 24i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 2i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 28i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 3i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 40i32 as U32,},
         ZSTD_seqSymbol{nextState: 32i32 as U16,
                        nbAdditionalBits: 4i32 as BYTE,
                        nbBits: 5i32 as BYTE,
                        baseValue: 48i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 16i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 65536i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 15i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 32768i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 14i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 16384i32 as U32,},
         ZSTD_seqSymbol{nextState: 0i32 as U16,
                        nbAdditionalBits: 13i32 as BYTE,
                        nbBits: 6i32 as BYTE,
                        baseValue: 8192i32 as U32,}]
    };
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
static mut LL_base: [U32; 36] =
    unsafe {
        [0i32 as U32, 1i32 as U32, 2i32 as U32, 3i32 as U32, 4i32 as U32,
         5i32 as U32, 6i32 as U32, 7i32 as U32, 8i32 as U32, 9i32 as U32,
         10i32 as U32, 11i32 as U32, 12i32 as U32, 13i32 as U32, 14i32 as U32,
         15i32 as U32, 16i32 as U32, 18i32 as U32, 20i32 as U32, 22i32 as U32,
         24i32 as U32, 28i32 as U32, 32i32 as U32, 40i32 as U32, 48i32 as U32,
         64i32 as U32, 128i32 as U32, 256i32 as U32, 512i32 as U32,
         1024i32 as U32, 2048i32 as U32, 4096i32 as U32, 8192i32 as U32,
         16384i32 as U32, 32768i32 as U32, 65536i32 as U32]
    };
/* ! ZSTD_decodeLiteralsBlock() :
 * @return : nb of bytes read from src (< srcSize )
 *  note : symbol not declared but exposed for fullbench */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decodeLiteralsBlock(mut dctx: *mut ZSTD_DCtx,
                                                  mut src:
                                                      *const libc::c_void,
                                                  mut srcSize: size_t)
 -> size_t {
    if srcSize < (1i32 + 1i32 + 1i32) as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    } else {
        let istart: *const BYTE = src as *const BYTE;
        let litEncType: symbolEncodingType_e =
            (*istart.offset(0isize) as libc::c_int & 3i32) as
                symbolEncodingType_e;
        match litEncType as libc::c_uint {
            3 => {
                if (*dctx).litEntropy == 0i32 as libc::c_uint {
                    return -(ZSTD_error_dictionary_corrupted as libc::c_int)
                               as size_t
                }
            }
            2 => { }
            0 => {
                let mut litSize_0: size_t = 0;
                let mut lhSize_0: size_t = 0;
                let lhlCode_0: U32 =
                    (*istart.offset(0isize) as libc::c_int >> 2i32 & 3i32) as
                        U32;
                match lhlCode_0 {
                    1 => {
                        lhSize_0 = 2i32 as size_t;
                        litSize_0 =
                            (MEM_readLE16(istart as *const libc::c_void) as
                                 libc::c_int >> 4i32) as size_t
                    }
                    3 => {
                        lhSize_0 = 3i32 as size_t;
                        litSize_0 =
                            (MEM_readLE24(istart as *const libc::c_void) >>
                                 4i32) as size_t
                    }
                    0 | 2 | _ => {
                        lhSize_0 = 1i32 as size_t;
                        litSize_0 =
                            (*istart.offset(0isize) as libc::c_int >> 3i32) as
                                size_t
                    }
                }
                if lhSize_0.wrapping_add(litSize_0).wrapping_add(8i32 as
                                                                     libc::c_ulong)
                       > srcSize {
                    if litSize_0.wrapping_add(lhSize_0) > srcSize {
                        return -(ZSTD_error_corruption_detected as
                                     libc::c_int) as size_t
                    } else {
                        memcpy((*dctx).litBuffer.as_mut_ptr() as
                                   *mut libc::c_void,
                               istart.offset(lhSize_0 as isize) as
                                   *const libc::c_void, litSize_0);
                        (*dctx).litPtr = (*dctx).litBuffer.as_mut_ptr();
                        (*dctx).litSize = litSize_0;
                        memset((*dctx).litBuffer.as_mut_ptr().offset((*dctx).litSize
                                                                         as
                                                                         isize)
                                   as *mut libc::c_void, 0i32,
                               8i32 as libc::c_ulong);
                        return lhSize_0.wrapping_add(litSize_0)
                    }
                } else {
                    (*dctx).litPtr = istart.offset(lhSize_0 as isize);
                    (*dctx).litSize = litSize_0;
                    return lhSize_0.wrapping_add(litSize_0)
                }
            }
            1 => {
                let lhlCode_1: U32 =
                    (*istart.offset(0isize) as libc::c_int >> 2i32 & 3i32) as
                        U32;
                let mut litSize_1: size_t = 0;
                let mut lhSize_1: size_t = 0;
                match lhlCode_1 {
                    1 => {
                        lhSize_1 = 2i32 as size_t;
                        litSize_1 =
                            (MEM_readLE16(istart as *const libc::c_void) as
                                 libc::c_int >> 4i32) as size_t
                    }
                    3 => {
                        lhSize_1 = 3i32 as size_t;
                        litSize_1 =
                            (MEM_readLE24(istart as *const libc::c_void) >>
                                 4i32) as size_t;
                        if srcSize < 4i32 as libc::c_ulong {
                            return -(ZSTD_error_corruption_detected as
                                         libc::c_int) as size_t
                        }
                    }
                    0 | 2 | _ => {
                        lhSize_1 = 1i32 as size_t;
                        litSize_1 =
                            (*istart.offset(0isize) as libc::c_int >> 3i32) as
                                size_t
                    }
                }
                if litSize_1 > (1i32 << 17i32) as libc::c_ulong {
                    return -(ZSTD_error_corruption_detected as libc::c_int) as
                               size_t
                } else {
                    memset((*dctx).litBuffer.as_mut_ptr() as
                               *mut libc::c_void,
                           *istart.offset(lhSize_1 as isize) as libc::c_int,
                           litSize_1.wrapping_add(8i32 as libc::c_ulong));
                    (*dctx).litPtr = (*dctx).litBuffer.as_mut_ptr();
                    (*dctx).litSize = litSize_1;
                    return lhSize_1.wrapping_add(1i32 as libc::c_ulong)
                }
            }
            _ => {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            }
        }
        if srcSize < 5i32 as libc::c_ulong {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        } else {
            let mut lhSize: size_t = 0;
            let mut litSize: size_t = 0;
            let mut litCSize: size_t = 0;
            let mut singleStream: U32 = 0i32 as U32;
            let lhlCode: U32 =
                (*istart.offset(0isize) as libc::c_int >> 2i32 & 3i32) as U32;
            let lhc: U32 = MEM_readLE32(istart as *const libc::c_void);
            match lhlCode {
                2 => {
                    lhSize = 4i32 as size_t;
                    litSize =
                        (lhc >> 4i32 & 16383i32 as libc::c_uint) as size_t;
                    litCSize = (lhc >> 18i32) as size_t
                }
                3 => {
                    lhSize = 5i32 as size_t;
                    litSize =
                        (lhc >> 4i32 & 262143i32 as libc::c_uint) as size_t;
                    litCSize =
                        (lhc >>
                             22i32).wrapping_add(((*istart.offset(4isize) as
                                                       libc::c_int) << 10i32)
                                                     as libc::c_uint) as
                            size_t
                }
                0 | 1 | _ => {
                    singleStream = (0 == lhlCode) as libc::c_int as U32;
                    lhSize = 3i32 as size_t;
                    litSize =
                        (lhc >> 4i32 & 1023i32 as libc::c_uint) as size_t;
                    litCSize =
                        (lhc >> 14i32 & 1023i32 as libc::c_uint) as size_t
                }
            }
            if litSize > (1i32 << 17i32) as libc::c_ulong {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            } else if litCSize.wrapping_add(lhSize) > srcSize {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            } else if 0 !=
                          ERR_isError(if litEncType as libc::c_uint ==
                                             set_repeat as libc::c_int as
                                                 libc::c_uint {
                                          if 0 != singleStream {
                                              HUF_decompress1X_usingDTable_bmi2((*dctx).litBuffer.as_mut_ptr()
                                                                                    as
                                                                                    *mut libc::c_void,
                                                                                litSize,
                                                                                istart.offset(lhSize
                                                                                                  as
                                                                                                  isize)
                                                                                    as
                                                                                    *const libc::c_void,
                                                                                litCSize,
                                                                                (*dctx).HUFptr,
                                                                                (*dctx).bmi2)
                                          } else {
                                              HUF_decompress4X_usingDTable_bmi2((*dctx).litBuffer.as_mut_ptr()
                                                                                    as
                                                                                    *mut libc::c_void,
                                                                                litSize,
                                                                                istart.offset(lhSize
                                                                                                  as
                                                                                                  isize)
                                                                                    as
                                                                                    *const libc::c_void,
                                                                                litCSize,
                                                                                (*dctx).HUFptr,
                                                                                (*dctx).bmi2)
                                          }
                                      } else if 0 != singleStream {
                                          HUF_decompress1X1_DCtx_wksp_bmi2((*dctx).entropy.hufTable.as_mut_ptr(),
                                                                           (*dctx).litBuffer.as_mut_ptr()
                                                                               as
                                                                               *mut libc::c_void,
                                                                           litSize,
                                                                           istart.offset(lhSize
                                                                                             as
                                                                                             isize)
                                                                               as
                                                                               *const libc::c_void,
                                                                           litCSize,
                                                                           (*dctx).entropy.workspace.as_mut_ptr()
                                                                               as
                                                                               *mut libc::c_void,
                                                                           ::std::mem::size_of::<[U32; 512]>()
                                                                               as
                                                                               libc::c_ulong,
                                                                           (*dctx).bmi2)
                                      } else {
                                          HUF_decompress4X_hufOnly_wksp_bmi2((*dctx).entropy.hufTable.as_mut_ptr(),
                                                                             (*dctx).litBuffer.as_mut_ptr()
                                                                                 as
                                                                                 *mut libc::c_void,
                                                                             litSize,
                                                                             istart.offset(lhSize
                                                                                               as
                                                                                               isize)
                                                                                 as
                                                                                 *const libc::c_void,
                                                                             litCSize,
                                                                             (*dctx).entropy.workspace.as_mut_ptr()
                                                                                 as
                                                                                 *mut libc::c_void,
                                                                             ::std::mem::size_of::<[U32; 512]>()
                                                                                 as
                                                                                 libc::c_ulong,
                                                                             (*dctx).bmi2)
                                      }) {
                return -(ZSTD_error_corruption_detected as libc::c_int) as
                           size_t
            } else {
                (*dctx).litPtr = (*dctx).litBuffer.as_mut_ptr();
                (*dctx).litSize = litSize;
                (*dctx).litEntropy = 1i32 as U32;
                if litEncType as libc::c_uint ==
                       set_compressed as libc::c_int as libc::c_uint {
                    (*dctx).HUFptr = (*dctx).entropy.hufTable.as_mut_ptr()
                }
                memset((*dctx).litBuffer.as_mut_ptr().offset((*dctx).litSize
                                                                 as isize) as
                           *mut libc::c_void, 0i32, 8i32 as libc::c_ulong);
                return litCSize.wrapping_add(lhSize)
            }
        }
    };
}
/* ***************************************************************************************
 * START OF ADVANCED AND EXPERIMENTAL FUNCTIONS
 * The definitions in this section are considered experimental.
 * They should never be used with a dynamic library, as prototypes may change in the future.
 * They are provided for advanced scenarios.
 * Use them only in association with static linking.
 * ***************************************************************************************/
/* !< recommended size for output buffer. Guarantee to successfully flush at least one complete block in all circumstances. */
/* !< recommended size for input buffer */
/* *< DCtx and DStream are now effectively same object (>= v1.3.0) */
static mut ZSTD_frameHeaderSize_prefix: size_t = unsafe { 5i32 as size_t };
#[no_mangle]
pub unsafe extern "C" fn ZSTD_frameHeaderSize(mut src: *const libc::c_void,
                                              mut srcSize: size_t) -> size_t {
    return ZSTD_frameHeaderSize_internal(src, srcSize, ZSTD_f_zstd1);
}
/* * ZSTD_frameHeaderSize_internal() :
 *  srcSize must be large enough to reach header size fields.
 *  note : only works for formats ZSTD_f_zstd1 and ZSTD_f_zstd1_magicless.
 * @return : size of the Frame Header
 *           or an error code, which can be tested with ZSTD_isError() */
unsafe extern "C" fn ZSTD_frameHeaderSize_internal(mut src:
                                                       *const libc::c_void,
                                                   mut srcSize: size_t,
                                                   mut format: ZSTD_format_e)
 -> size_t {
    let minInputSize: size_t = ZSTD_startingInputLength(format);
    if srcSize < minInputSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else {
        let fhd: BYTE =
            *(src as
                  *const BYTE).offset(minInputSize.wrapping_sub(1i32 as
                                                                    libc::c_ulong)
                                          as isize);
        let dictID: U32 = (fhd as libc::c_int & 3i32) as U32;
        let singleSegment: U32 = (fhd as libc::c_int >> 5i32 & 1i32) as U32;
        let fcsId: U32 = (fhd as libc::c_int >> 6i32) as U32;
        return minInputSize.wrapping_add((0 == singleSegment) as libc::c_int
                                             as
                                             libc::c_ulong).wrapping_add(ZSTD_did_fieldSize[dictID
                                                                                                as
                                                                                                usize]).wrapping_add(ZSTD_fcs_fieldSize[fcsId
                                                                                                                                            as
                                                                                                                                            usize]).wrapping_add((0
                                                                                                                                                                      !=
                                                                                                                                                                      singleSegment
                                                                                                                                                                      &&
                                                                                                                                                                      0
                                                                                                                                                                          ==
                                                                                                                                                                          fcsId)
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_int
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_ulong)
    };
}
unsafe extern "C" fn ZSTD_startingInputLength(mut format: ZSTD_format_e)
 -> size_t {
    let startingInputLength: size_t =
        if format as libc::c_uint ==
               ZSTD_f_zstd1_magicless as libc::c_int as libc::c_uint {
            ZSTD_frameHeaderSize_prefix.wrapping_sub(ZSTD_frameIdSize)
        } else { ZSTD_frameHeaderSize_prefix };
    return startingInputLength;
}
static mut ZSTD_frameIdSize: size_t = unsafe { 4i32 as size_t };
static mut ZSTD_fcs_fieldSize: [size_t; 4] =
    unsafe {
        [0i32 as size_t, 2i32 as size_t, 4i32 as size_t, 8i32 as size_t]
    };
static mut ZSTD_did_fieldSize: [size_t; 4] =
    unsafe {
        [0i32 as size_t, 1i32 as size_t, 2i32 as size_t, 4i32 as size_t]
    };
/* * ZSTD_decodeFrameHeader() :
*   `headerSize` must be the size provided by ZSTD_frameHeaderSize().
*   @return : 0 if success, or an error code, which can be tested using ZSTD_isError() */
unsafe extern "C" fn ZSTD_decodeFrameHeader(mut dctx: *mut ZSTD_DCtx,
                                            mut src: *const libc::c_void,
                                            mut headerSize: size_t)
 -> size_t {
    let result: size_t =
        ZSTD_getFrameHeader_advanced(&mut (*dctx).fParams as
                                         *mut ZSTD_frameHeader, src,
                                     headerSize, (*dctx).format);
    if 0 != ERR_isError(result) {
        return result
    } else if result > 0i32 as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else if 0 != (*dctx).fParams.dictID &&
                  (*dctx).dictID != (*dctx).fParams.dictID {
        return -(ZSTD_error_dictionary_wrong as libc::c_int) as size_t
    } else {
        if 0 != (*dctx).fParams.checksumFlag {
            ZSTD_XXH64_reset(&mut (*dctx).xxhState as *mut XXH64_state_t,
                             0i32 as libc::c_ulonglong);
        }
        return 0i32 as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getFrameHeader_advanced(mut zfhPtr:
                                                          *mut ZSTD_frameHeader,
                                                      mut src:
                                                          *const libc::c_void,
                                                      mut srcSize: size_t,
                                                      mut format:
                                                          ZSTD_format_e)
 -> size_t {
    let mut ip: *const BYTE = src as *const BYTE;
    let minInputSize: size_t = ZSTD_startingInputLength(format);
    if srcSize < minInputSize {
        return minInputSize
    } else if format as libc::c_uint !=
                  ZSTD_f_zstd1_magicless as libc::c_int as libc::c_uint &&
                  MEM_readLE32(src) != 4247762216u32 {
        if MEM_readLE32(src) & 4294967280u32 == 407710288u32 {
            if srcSize < ZSTD_skippableHeaderSize {
                return ZSTD_skippableHeaderSize
            } else {
                memset(zfhPtr as *mut libc::c_void, 0i32,
                       ::std::mem::size_of::<ZSTD_frameHeader>() as
                           libc::c_ulong);
                (*zfhPtr).frameContentSize =
                    MEM_readLE32((src as
                                      *const libc::c_char).offset(ZSTD_frameIdSize
                                                                      as
                                                                      isize)
                                     as *const libc::c_void) as
                        libc::c_ulonglong;
                (*zfhPtr).frameType = ZSTD_skippableFrame;
                return 0i32 as size_t
            }
        } else {
            return -(ZSTD_error_prefix_unknown as libc::c_int) as size_t
        }
    } else {
        let fhsize: size_t =
            ZSTD_frameHeaderSize_internal(src, srcSize, format);
        if srcSize < fhsize {
            return fhsize
        } else {
            (*zfhPtr).headerSize = fhsize as U32;
            let fhdByte: BYTE =
                *ip.offset(minInputSize.wrapping_sub(1i32 as libc::c_ulong) as
                               isize);
            let mut pos: size_t = minInputSize;
            let dictIDSizeCode: U32 = (fhdByte as libc::c_int & 3i32) as U32;
            let checksumFlag: U32 =
                (fhdByte as libc::c_int >> 2i32 & 1i32) as U32;
            let singleSegment: U32 =
                (fhdByte as libc::c_int >> 5i32 & 1i32) as U32;
            let fcsID: U32 = (fhdByte as libc::c_int >> 6i32) as U32;
            let mut windowSize: U64 = 0i32 as U64;
            let mut dictID: U32 = 0i32 as U32;
            let mut frameContentSize: U64 =
                0u64.wrapping_sub(1i32 as libc::c_ulonglong) as U64;
            if fhdByte as libc::c_int & 8i32 != 0i32 {
                return -(ZSTD_error_frameParameter_unsupported as libc::c_int)
                           as size_t
            } else {
                if 0 == singleSegment {
                    let fresh12 = pos;
                    pos = pos.wrapping_add(1);
                    let wlByte: BYTE = *ip.offset(fresh12 as isize);
                    let windowLog: U32 =
                        ((wlByte as libc::c_int >> 3i32) + 10i32) as U32;
                    if windowLog >
                           (if ::std::mem::size_of::<size_t>() as
                                   libc::c_ulong == 4i32 as libc::c_ulong {
                                30i32
                            } else { 31i32 }) as libc::c_uint {
                        return -(ZSTD_error_frameParameter_windowTooLarge as
                                     libc::c_int) as size_t
                    } else {
                        windowSize = (1u64 << windowLog) as U64;
                        windowSize =
                            (windowSize as
                                 libc::c_ulong).wrapping_add((windowSize >>
                                                                  3i32).wrapping_mul((wlByte
                                                                                          as
                                                                                          libc::c_int
                                                                                          &
                                                                                          7i32)
                                                                                         as
                                                                                         libc::c_ulong))
                                as U64 as U64
                    }
                }
                match dictIDSizeCode {
                    1 => {
                        dictID = *ip.offset(pos as isize) as U32;
                        pos = pos.wrapping_add(1)
                    }
                    2 => {
                        dictID =
                            MEM_readLE16(ip.offset(pos as isize) as
                                             *const libc::c_void) as U32;
                        pos =
                            (pos as
                                 libc::c_ulong).wrapping_add(2i32 as
                                                                 libc::c_ulong)
                                as size_t as size_t
                    }
                    3 => {
                        dictID =
                            MEM_readLE32(ip.offset(pos as isize) as
                                             *const libc::c_void);
                        pos =
                            (pos as
                                 libc::c_ulong).wrapping_add(4i32 as
                                                                 libc::c_ulong)
                                as size_t as size_t
                    }
                    0 | _ => { }
                }
                match fcsID {
                    1 => {
                        frameContentSize =
                            (MEM_readLE16(ip.offset(pos as isize) as
                                              *const libc::c_void) as
                                 libc::c_int + 256i32) as U64
                    }
                    2 => {
                        frameContentSize =
                            MEM_readLE32(ip.offset(pos as isize) as
                                             *const libc::c_void) as U64
                    }
                    3 => {
                        frameContentSize =
                            MEM_readLE64(ip.offset(pos as isize) as
                                             *const libc::c_void)
                    }
                    0 | _ => {
                        if 0 != singleSegment {
                            frameContentSize = *ip.offset(pos as isize) as U64
                        }
                    }
                }
                if 0 != singleSegment { windowSize = frameContentSize }
                (*zfhPtr).frameType = ZSTD_frame;
                (*zfhPtr).frameContentSize =
                    frameContentSize as libc::c_ulonglong;
                (*zfhPtr).windowSize = windowSize as libc::c_ulonglong;
                (*zfhPtr).blockSizeMax =
                    (if windowSize < (1i32 << 17i32) as libc::c_ulong {
                         windowSize
                     } else { (1i32 << 17i32) as libc::c_ulong }) as
                        libc::c_uint;
                (*zfhPtr).dictID = dictID;
                (*zfhPtr).checksumFlag = checksumFlag;
                return 0i32 as size_t
            }
        }
    };
}
static mut ZSTD_skippableHeaderSize: size_t = unsafe { 8i32 as size_t };
static mut ZSTD_frameHeaderSize_min: size_t = unsafe { 6i32 as size_t };
unsafe extern "C" fn ZSTD_checkContinuity(mut dctx: *mut ZSTD_DCtx,
                                          mut dst: *const libc::c_void)
 -> () {
    if dst != (*dctx).previousDstEnd {
        (*dctx).dictEnd = (*dctx).previousDstEnd;
        (*dctx).virtualStart =
            (dst as
                 *const libc::c_char).offset(-(((*dctx).prefixStart as
                                                    *const libc::c_char).offset_to((*dctx).previousDstEnd
                                                                                       as
                                                                                       *const libc::c_char).expect("bad offset_to")
                                                   as libc::c_long as isize))
                as *const libc::c_void;
        (*dctx).prefixStart = dst;
        (*dctx).previousDstEnd = dst
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBegin_usingDict(mut dctx:
                                                            *mut ZSTD_DCtx,
                                                        mut dict:
                                                            *const libc::c_void,
                                                        mut dictSize: size_t)
 -> size_t {
    let errcod: size_t = ZSTD_decompressBegin(dctx);
    if 0 != ERR_isError(errcod) {
        return errcod
    } else {
        if !dict.is_null() && 0 != dictSize {
            let errcod_0: size_t =
                ZSTD_decompress_insertDictionary(dctx, dict, dictSize);
            if 0 != ERR_isError(errcod_0) {
                return -(ZSTD_error_dictionary_corrupted as libc::c_int) as
                           size_t
            }
        }
        return 0i32 as size_t
    };
}
unsafe extern "C" fn ZSTD_decompress_insertDictionary(mut dctx:
                                                          *mut ZSTD_DCtx,
                                                      mut dict:
                                                          *const libc::c_void,
                                                      mut dictSize: size_t)
 -> size_t {
    if dictSize < 8i32 as libc::c_ulong {
        return ZSTD_refDictContent(dctx, dict, dictSize)
    } else {
        let magic: U32 = MEM_readLE32(dict);
        if magic != 3962610743u32 {
            return ZSTD_refDictContent(dctx, dict, dictSize)
        } else {
            (*dctx).dictID =
                MEM_readLE32((dict as
                                  *const libc::c_char).offset(ZSTD_frameIdSize
                                                                  as isize) as
                                 *const libc::c_void);
            let eSize: size_t =
                ZSTD_loadEntropy(&mut (*dctx).entropy as
                                     *mut ZSTD_entropyDTables_t, dict,
                                 dictSize);
            if 0 != ERR_isError(eSize) {
                return -(ZSTD_error_dictionary_corrupted as libc::c_int) as
                           size_t
            } else {
                dict =
                    (dict as *const libc::c_char).offset(eSize as isize) as
                        *const libc::c_void;
                dictSize =
                    (dictSize as libc::c_ulong).wrapping_sub(eSize) as size_t
                        as size_t;
                (*dctx).fseEntropy = 1i32 as U32;
                (*dctx).litEntropy = (*dctx).fseEntropy;
                return ZSTD_refDictContent(dctx, dict, dictSize)
            }
        }
    };
}
unsafe extern "C" fn ZSTD_refDictContent(mut dctx: *mut ZSTD_DCtx,
                                         mut dict: *const libc::c_void,
                                         mut dictSize: size_t) -> size_t {
    (*dctx).dictEnd = (*dctx).previousDstEnd;
    (*dctx).virtualStart =
        (dict as
             *const libc::c_char).offset(-(((*dctx).prefixStart as
                                                *const libc::c_char).offset_to((*dctx).previousDstEnd
                                                                                   as
                                                                                   *const libc::c_char).expect("bad offset_to")
                                               as libc::c_long as isize)) as
            *const libc::c_void;
    (*dctx).prefixStart = dict;
    (*dctx).previousDstEnd =
        (dict as *const libc::c_char).offset(dictSize as isize) as
            *const libc::c_void;
    return 0i32 as size_t;
}
unsafe extern "C" fn ZSTD_loadEntropy(mut entropy: *mut ZSTD_entropyDTables_t,
                                      dict: *const libc::c_void,
                                      dictSize: size_t) -> size_t {
    let mut dictPtr: *const BYTE = dict as *const BYTE;
    let dictEnd: *const BYTE = dictPtr.offset(dictSize as isize);
    if dictSize <= 8i32 as libc::c_ulong {
        return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
    } else {
        dictPtr = dictPtr.offset(8isize);
        let hSize: size_t =
            HUF_readDTableX2_wksp((*entropy).hufTable.as_mut_ptr(),
                                  dictPtr as *const libc::c_void,
                                  dictPtr.offset_to(dictEnd).expect("bad offset_to")
                                      as libc::c_long as size_t,
                                  (*entropy).workspace.as_mut_ptr() as
                                      *mut libc::c_void,
                                  ::std::mem::size_of::<[U32; 512]>() as
                                      libc::c_ulong);
        if 0 != ERR_isError(hSize) {
            return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
        } else {
            dictPtr = dictPtr.offset(hSize as isize);
            let mut offcodeNCount: [libc::c_short; 32] = [0; 32];
            let mut offcodeMaxValue: U32 = 31i32 as U32;
            let mut offcodeLog: U32 = 0;
            let offcodeHeaderSize: size_t =
                FSE_readNCount(offcodeNCount.as_mut_ptr(),
                               &mut offcodeMaxValue as *mut U32,
                               &mut offcodeLog as *mut U32,
                               dictPtr as *const libc::c_void,
                               dictPtr.offset_to(dictEnd).expect("bad offset_to")
                                   as libc::c_long as size_t);
            if 0 != ERR_isError(offcodeHeaderSize) {
                return -(ZSTD_error_dictionary_corrupted as libc::c_int) as
                           size_t
            } else if offcodeMaxValue > 31i32 as libc::c_uint {
                return -(ZSTD_error_dictionary_corrupted as libc::c_int) as
                           size_t
            } else if offcodeLog > 8i32 as libc::c_uint {
                return -(ZSTD_error_dictionary_corrupted as libc::c_int) as
                           size_t
            } else {
                ZSTD_buildFSETable((*entropy).OFTable.as_mut_ptr(),
                                   offcodeNCount.as_mut_ptr(),
                                   offcodeMaxValue, OF_base.as_ptr(),
                                   OF_bits.as_ptr(), offcodeLog);
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
                if 0 != ERR_isError(matchlengthHeaderSize) {
                    return -(ZSTD_error_dictionary_corrupted as libc::c_int)
                               as size_t
                } else if matchlengthMaxValue > 52i32 as libc::c_uint {
                    return -(ZSTD_error_dictionary_corrupted as libc::c_int)
                               as size_t
                } else if matchlengthLog > 9i32 as libc::c_uint {
                    return -(ZSTD_error_dictionary_corrupted as libc::c_int)
                               as size_t
                } else {
                    ZSTD_buildFSETable((*entropy).MLTable.as_mut_ptr(),
                                       matchlengthNCount.as_mut_ptr(),
                                       matchlengthMaxValue, ML_base.as_ptr(),
                                       ML_bits.as_ptr(), matchlengthLog);
                    dictPtr = dictPtr.offset(matchlengthHeaderSize as isize);
                    let mut litlengthNCount: [libc::c_short; 36] = [0; 36];
                    let mut litlengthMaxValue: libc::c_uint =
                        35i32 as libc::c_uint;
                    let mut litlengthLog: libc::c_uint = 0;
                    let litlengthHeaderSize: size_t =
                        FSE_readNCount(litlengthNCount.as_mut_ptr(),
                                       &mut litlengthMaxValue as
                                           *mut libc::c_uint,
                                       &mut litlengthLog as *mut libc::c_uint,
                                       dictPtr as *const libc::c_void,
                                       dictPtr.offset_to(dictEnd).expect("bad offset_to")
                                           as libc::c_long as size_t);
                    if 0 != ERR_isError(litlengthHeaderSize) {
                        return -(ZSTD_error_dictionary_corrupted as
                                     libc::c_int) as size_t
                    } else if litlengthMaxValue > 35i32 as libc::c_uint {
                        return -(ZSTD_error_dictionary_corrupted as
                                     libc::c_int) as size_t
                    } else if litlengthLog > 9i32 as libc::c_uint {
                        return -(ZSTD_error_dictionary_corrupted as
                                     libc::c_int) as size_t
                    } else {
                        ZSTD_buildFSETable((*entropy).LLTable.as_mut_ptr(),
                                           litlengthNCount.as_mut_ptr(),
                                           litlengthMaxValue,
                                           LL_base.as_ptr(), LL_bits.as_ptr(),
                                           litlengthLog);
                        dictPtr =
                            dictPtr.offset(litlengthHeaderSize as isize);
                        if dictPtr.offset(12isize) > dictEnd {
                            return -(ZSTD_error_dictionary_corrupted as
                                         libc::c_int) as size_t
                        } else {
                            let mut i: libc::c_int = 0;
                            let dictContentSize: size_t =
                                dictPtr.offset(12isize).offset_to(dictEnd).expect("bad offset_to")
                                    as libc::c_long as size_t;
                            i = 0i32;
                            while i < 3i32 {
                                let rep: U32 =
                                    MEM_readLE32(dictPtr as
                                                     *const libc::c_void);
                                dictPtr = dictPtr.offset(4isize);
                                if rep == 0i32 as libc::c_uint ||
                                       rep as libc::c_ulong >= dictContentSize
                                   {
                                    return -(ZSTD_error_dictionary_corrupted
                                                 as libc::c_int) as size_t
                                } else {
                                    (*entropy).rep[i as usize] = rep;
                                    i += 1
                                }
                            }
                            return (dict as
                                        *const BYTE).offset_to(dictPtr).expect("bad offset_to")
                                       as libc::c_long as size_t
                        }
                    }
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBegin(mut dctx: *mut ZSTD_DCtx)
 -> size_t {
    (*dctx).expected = ZSTD_startingInputLength((*dctx).format);
    (*dctx).stage = ZSTDds_getFrameHeaderSize;
    (*dctx).decodedSize = 0i32 as U64;
    (*dctx).previousDstEnd = 0 as *const libc::c_void;
    (*dctx).prefixStart = 0 as *const libc::c_void;
    (*dctx).virtualStart = 0 as *const libc::c_void;
    (*dctx).dictEnd = 0 as *const libc::c_void;
    (*dctx).entropy.hufTable[0usize] = (12i32 * 16777217i32) as HUF_DTable;
    (*dctx).fseEntropy = 0i32 as U32;
    (*dctx).litEntropy = (*dctx).fseEntropy;
    (*dctx).dictID = 0i32 as U32;
    memcpy((*dctx).entropy.rep.as_mut_ptr() as *mut libc::c_void,
           repStartValue.as_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[U32; 3]>() as libc::c_ulong);
    (*dctx).LLTptr = (*dctx).entropy.LLTable.as_mut_ptr();
    (*dctx).MLTptr = (*dctx).entropy.MLTable.as_mut_ptr();
    (*dctx).OFTptr = (*dctx).entropy.OFTable.as_mut_ptr();
    (*dctx).HUFptr = (*dctx).entropy.hufTable.as_mut_ptr();
    return 0i32 as size_t;
}
static mut repStartValue: [U32; 3] =
    unsafe { [1i32 as U32, 4i32 as U32, 8i32 as U32] };
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBegin_usingDDict(mut dstDCtx:
                                                             *mut ZSTD_DCtx,
                                                         mut ddict:
                                                             *const ZSTD_DDict)
 -> size_t {
    let errcod: size_t = ZSTD_decompressBegin(dstDCtx);
    if 0 != ERR_isError(errcod) {
        return errcod
    } else {
        if !ddict.is_null() {
            (*dstDCtx).dictID = (*ddict).dictID;
            (*dstDCtx).prefixStart = (*ddict).dictContent;
            (*dstDCtx).virtualStart = (*ddict).dictContent;
            (*dstDCtx).dictEnd =
                ((*ddict).dictContent as
                     *const BYTE).offset((*ddict).dictSize as isize) as
                    *const libc::c_void;
            (*dstDCtx).previousDstEnd = (*dstDCtx).dictEnd;
            if 0 != (*ddict).entropyPresent {
                (*dstDCtx).litEntropy = 1i32 as U32;
                (*dstDCtx).fseEntropy = 1i32 as U32;
                (*dstDCtx).LLTptr = (*ddict).entropy.LLTable.as_ptr();
                (*dstDCtx).MLTptr = (*ddict).entropy.MLTable.as_ptr();
                (*dstDCtx).OFTptr = (*ddict).entropy.OFTable.as_ptr();
                (*dstDCtx).HUFptr = (*ddict).entropy.hufTable.as_ptr();
                (*dstDCtx).entropy.rep[0usize] = (*ddict).entropy.rep[0usize];
                (*dstDCtx).entropy.rep[1usize] = (*ddict).entropy.rep[1usize];
                (*dstDCtx).entropy.rep[2usize] = (*ddict).entropy.rep[2usize]
            } else {
                (*dstDCtx).litEntropy = 0i32 as U32;
                (*dstDCtx).fseEntropy = 0i32 as U32
            }
        }
        return 0i32 as size_t
    };
}
unsafe extern "C" fn ZSTD_DDictDictSize(mut ddict: *const ZSTD_DDict)
 -> size_t {
    return (*ddict).dictSize;
}
unsafe extern "C" fn ZSTD_DDictDictContent(mut ddict: *const ZSTD_DDict)
 -> *const libc::c_void {
    return (*ddict).dictContent;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getFrameContentSize(mut src:
                                                      *const libc::c_void,
                                                  mut srcSize: size_t)
 -> libc::c_ulonglong {
    let mut zfh: ZSTD_frameHeader =
        ZSTD_frameHeader{frameContentSize: 0,
                         windowSize: 0,
                         blockSizeMax: 0,
                         frameType: ZSTD_frame,
                         headerSize: 0,
                         dictID: 0,
                         checksumFlag: 0,};
    if ZSTD_getFrameHeader(&mut zfh as *mut ZSTD_frameHeader, src, srcSize) !=
           0i32 as libc::c_ulong {
        return 0u64.wrapping_sub(2i32 as libc::c_ulonglong)
    } else if zfh.frameType as libc::c_uint ==
                  ZSTD_skippableFrame as libc::c_int as libc::c_uint {
        return 0i32 as libc::c_ulonglong
    } else { return zfh.frameContentSize };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getFrameHeader(mut zfhPtr:
                                                 *mut ZSTD_frameHeader,
                                             mut src: *const libc::c_void,
                                             mut srcSize: size_t) -> size_t {
    return ZSTD_getFrameHeader_advanced(zfhPtr, src, srcSize, ZSTD_f_zstd1);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDecompressedSize(mut src:
                                                      *const libc::c_void,
                                                  mut srcSize: size_t)
 -> libc::c_ulonglong {
    let ret: libc::c_ulonglong = ZSTD_getFrameContentSize(src, srcSize);
    return if ret >= 0u64.wrapping_sub(2i32 as libc::c_ulonglong) {
               0i32 as libc::c_ulonglong
           } else { ret };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDDict(mut dict: *const libc::c_void,
                                          mut dictSize: size_t)
 -> *mut ZSTD_DDict {
    let allocator: ZSTD_customMem =
        ZSTD_customMem{customAlloc: None,
                       customFree: None,
                       opaque: 0 as *mut libc::c_void,};
    return ZSTD_createDDict_advanced(dict, dictSize, ZSTD_dlm_byCopy,
                                     ZSTD_dct_auto, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDDict_advanced(mut dict:
                                                       *const libc::c_void,
                                                   mut dictSize: size_t,
                                                   mut dictLoadMethod:
                                                       ZSTD_dictLoadMethod_e,
                                                   mut dictContentType:
                                                       ZSTD_dictContentType_e,
                                                   mut customMem:
                                                       ZSTD_customMem)
 -> *mut ZSTD_DDict {
    if 0 !=
           customMem.customAlloc.is_none() as libc::c_int ^
               customMem.customFree.is_none() as libc::c_int {
        return 0 as *mut ZSTD_DDict
    } else {
        let ddict: *mut ZSTD_DDict =
            ZSTD_malloc(::std::mem::size_of::<ZSTD_DDict>() as libc::c_ulong,
                        customMem) as *mut ZSTD_DDict;
        if ddict.is_null() {
            return 0 as *mut ZSTD_DDict
        } else {
            (*ddict).cMem = customMem;
            if 0 !=
                   ERR_isError(ZSTD_initDDict_internal(ddict, dict, dictSize,
                                                       dictLoadMethod,
                                                       dictContentType)) {
                ZSTD_freeDDict(ddict);
                return 0 as *mut ZSTD_DDict
            } else { return ddict }
        }
    };
}
unsafe extern "C" fn ZSTD_initDDict_internal(mut ddict: *mut ZSTD_DDict,
                                             mut dict: *const libc::c_void,
                                             mut dictSize: size_t,
                                             mut dictLoadMethod:
                                                 ZSTD_dictLoadMethod_e,
                                             mut dictContentType:
                                                 ZSTD_dictContentType_e)
 -> size_t {
    if dictLoadMethod as libc::c_uint ==
           ZSTD_dlm_byRef as libc::c_int as libc::c_uint || dict.is_null() ||
           0 == dictSize {
        (*ddict).dictBuffer = 0 as *mut libc::c_void;
        (*ddict).dictContent = dict
    } else {
        let internalBuffer: *mut libc::c_void =
            ZSTD_malloc(dictSize, (*ddict).cMem);
        (*ddict).dictBuffer = internalBuffer;
        (*ddict).dictContent = internalBuffer;
        if internalBuffer.is_null() {
            return -(ZSTD_error_memory_allocation as libc::c_int) as size_t
        } else { memcpy(internalBuffer, dict, dictSize); }
    }
    (*ddict).dictSize = dictSize;
    (*ddict).entropy.hufTable[0usize] = (12i32 * 16777217i32) as HUF_DTable;
    let errcod: size_t = ZSTD_loadEntropy_inDDict(ddict, dictContentType);
    if 0 != ERR_isError(errcod) {
        return errcod
    } else { return 0i32 as size_t };
}
unsafe extern "C" fn ZSTD_loadEntropy_inDDict(mut ddict: *mut ZSTD_DDict,
                                              mut dictContentType:
                                                  ZSTD_dictContentType_e)
 -> size_t {
    (*ddict).dictID = 0i32 as U32;
    (*ddict).entropyPresent = 0i32 as U32;
    if dictContentType as libc::c_uint ==
           ZSTD_dct_rawContent as libc::c_int as libc::c_uint {
        return 0i32 as size_t
    } else if (*ddict).dictSize < 8i32 as libc::c_ulong {
        if dictContentType as libc::c_uint ==
               ZSTD_dct_fullDict as libc::c_int as libc::c_uint {
            return -(ZSTD_error_dictionary_corrupted as libc::c_int) as size_t
        } else { return 0i32 as size_t }
    } else {
        let magic: U32 = MEM_readLE32((*ddict).dictContent);
        if magic != 3962610743u32 {
            if dictContentType as libc::c_uint ==
                   ZSTD_dct_fullDict as libc::c_int as libc::c_uint {
                return -(ZSTD_error_dictionary_corrupted as libc::c_int) as
                           size_t
            } else { return 0i32 as size_t }
        } else {
            (*ddict).dictID =
                MEM_readLE32(((*ddict).dictContent as
                                  *const libc::c_char).offset(ZSTD_frameIdSize
                                                                  as isize) as
                                 *const libc::c_void);
            let errcod: size_t =
                ZSTD_loadEntropy(&mut (*ddict).entropy as
                                     *mut ZSTD_entropyDTables_t,
                                 (*ddict).dictContent, (*ddict).dictSize);
            if 0 != ERR_isError(errcod) {
                return -(ZSTD_error_dictionary_corrupted as libc::c_int) as
                           size_t
            } else {
                (*ddict).entropyPresent = 1i32 as U32;
                return 0i32 as size_t
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompress_usingDDict(mut dctx: *mut ZSTD_DCtx,
                                                    mut dst:
                                                        *mut libc::c_void,
                                                    mut dstCapacity: size_t,
                                                    mut src:
                                                        *const libc::c_void,
                                                    mut srcSize: size_t,
                                                    mut ddict:
                                                        *const ZSTD_DDict)
 -> size_t {
    return ZSTD_decompressMultiFrame(dctx, dst, dstCapacity, src, srcSize,
                                     0 as *const libc::c_void, 0i32 as size_t,
                                     ddict);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDStream() -> *mut ZSTD_DStream {
    return ZSTD_createDStream_advanced(ZSTD_defaultCMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDStream_advanced(mut customMem:
                                                         ZSTD_customMem)
 -> *mut ZSTD_DStream {
    return ZSTD_createDCtx_advanced(customMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeDStream(mut zds: *mut ZSTD_DStream)
 -> size_t {
    return ZSTD_freeDCtx(zds);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initDStream(mut zds: *mut ZSTD_DStream)
 -> size_t {
    return ZSTD_initDStream_usingDict(zds, 0 as *const libc::c_void,
                                      0i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initDStream_usingDict(mut zds:
                                                        *mut ZSTD_DStream,
                                                    mut dict:
                                                        *const libc::c_void,
                                                    mut dictSize: size_t)
 -> size_t {
    (*zds).streamStage = zdss_init;
    (*zds).noForwardProgress = 0i32;
    let errcod: size_t = ZSTD_DCtx_loadDictionary(zds, dict, dictSize);
    if 0 != ERR_isError(errcod) {
        return errcod
    } else { return ZSTD_frameHeaderSize_prefix };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_loadDictionary(mut dctx: *mut ZSTD_DCtx,
                                                  mut dict:
                                                      *const libc::c_void,
                                                  mut dictSize: size_t)
 -> size_t {
    return ZSTD_DCtx_loadDictionary_advanced(dctx, dict, dictSize,
                                             ZSTD_dlm_byCopy, ZSTD_dct_auto);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_loadDictionary_advanced(mut dctx:
                                                               *mut ZSTD_DCtx,
                                                           mut dict:
                                                               *const libc::c_void,
                                                           mut dictSize:
                                                               size_t,
                                                           mut dictLoadMethod:
                                                               ZSTD_dictLoadMethod_e,
                                                           mut dictContentType:
                                                               ZSTD_dictContentType_e)
 -> size_t {
    if (*dctx).streamStage as libc::c_uint !=
           zdss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    } else {
        ZSTD_freeDDict((*dctx).ddictLocal);
        if !dict.is_null() && dictSize >= 8i32 as libc::c_ulong {
            (*dctx).ddictLocal =
                ZSTD_createDDict_advanced(dict, dictSize, dictLoadMethod,
                                          dictContentType, (*dctx).customMem);
            if (*dctx).ddictLocal.is_null() {
                return -(ZSTD_error_memory_allocation as libc::c_int) as
                           size_t
            }
        } else { (*dctx).ddictLocal = 0 as *mut ZSTD_DDict }
        (*dctx).ddict = (*dctx).ddictLocal;
        return 0i32 as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressStream(mut zds: *mut ZSTD_DStream,
                                               mut output:
                                                   *mut ZSTD_outBuffer,
                                               mut input: *mut ZSTD_inBuffer)
 -> size_t {
    let mut current_block: u64;
    let istart: *const libc::c_char =
        ((*input).src as *const libc::c_char).offset((*input).pos as isize);
    let iend: *const libc::c_char =
        ((*input).src as *const libc::c_char).offset((*input).size as isize);
    let mut ip: *const libc::c_char = istart;
    let ostart: *mut libc::c_char =
        ((*output).dst as *mut libc::c_char).offset((*output).pos as isize);
    let oend: *mut libc::c_char =
        ((*output).dst as *mut libc::c_char).offset((*output).size as isize);
    let mut op: *mut libc::c_char = ostart;
    let mut someMoreWork: U32 = 1i32 as U32;
    if (*input).pos > (*input).size {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else if (*output).pos > (*output).size {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else {
        while 0 != someMoreWork {
            match (*zds).streamStage as libc::c_uint {
                0 => {
                    ZSTD_resetDStream(zds);
                    current_block = 2868539653012386629;
                }
                1 => { current_block = 2868539653012386629; }
                2 => { current_block = 7427571413727699167; }
                3 => { current_block = 2989495919056355252; }
                4 => { current_block = 6072622540298447352; }
                _ => { return -(ZSTD_error_GENERIC as libc::c_int) as size_t }
            }
            match current_block {
                2868539653012386629 => {
                    let hSize: size_t =
                        ZSTD_getFrameHeader_advanced(&mut (*zds).fParams as
                                                         *mut ZSTD_frameHeader,
                                                     (*zds).headerBuffer.as_mut_ptr()
                                                         as
                                                         *const libc::c_void,
                                                     (*zds).lhSize,
                                                     (*zds).format);
                    if 0 != ERR_isError(hSize) {
                        return hSize
                    } else if hSize != 0i32 as libc::c_ulong {
                        let toLoad: size_t =
                            hSize.wrapping_sub((*zds).lhSize);
                        let remainingInput: size_t =
                            ip.offset_to(iend).expect("bad offset_to") as
                                libc::c_long as size_t;
                        if toLoad > remainingInput {
                            if remainingInput > 0i32 as libc::c_ulong {
                                memcpy((*zds).headerBuffer.as_mut_ptr().offset((*zds).lhSize
                                                                                   as
                                                                                   isize)
                                           as *mut libc::c_void,
                                       ip as *const libc::c_void,
                                       remainingInput);
                                (*zds).lhSize =
                                    ((*zds).lhSize as
                                         libc::c_ulong).wrapping_add(remainingInput)
                                        as size_t as size_t
                            }
                            (*input).pos = (*input).size;
                            return if ZSTD_frameHeaderSize_min > hSize {
                                       ZSTD_frameHeaderSize_min
                                   } else {
                                       hSize
                                   }.wrapping_sub((*zds).lhSize).wrapping_add(ZSTD_blockHeaderSize)
                        } else {
                            memcpy((*zds).headerBuffer.as_mut_ptr().offset((*zds).lhSize
                                                                               as
                                                                               isize)
                                       as *mut libc::c_void,
                                   ip as *const libc::c_void, toLoad);
                            (*zds).lhSize = hSize;
                            ip = ip.offset(toLoad as isize);
                            continue ;
                        }
                    } else {
                        if 0 != (*zds).fParams.frameContentSize &&
                               0 != (*zds).fParams.windowSize &&
                               op.offset_to(oend).expect("bad offset_to") as
                                   libc::c_long as size_t as libc::c_ulonglong
                                   >= (*zds).fParams.frameContentSize {
                            let cSize: size_t =
                                ZSTD_findFrameCompressedSize(istart as
                                                                 *const libc::c_void,
                                                             istart.offset_to(iend).expect("bad offset_to")
                                                                 as
                                                                 libc::c_long
                                                                 as size_t);
                            if cSize <=
                                   istart.offset_to(iend).expect("bad offset_to")
                                       as libc::c_long as size_t {
                                let decompressedSize: size_t =
                                    ZSTD_decompress_usingDDict(zds,
                                                               op as
                                                                   *mut libc::c_void,
                                                               op.offset_to(oend).expect("bad offset_to")
                                                                   as
                                                                   libc::c_long
                                                                   as size_t,
                                                               istart as
                                                                   *const libc::c_void,
                                                               cSize,
                                                               (*zds).ddict);
                                if 0 != ERR_isError(decompressedSize) {
                                    return decompressedSize
                                } else {
                                    ip = istart.offset(cSize as isize);
                                    op = op.offset(decompressedSize as isize);
                                    (*zds).expected = 0i32 as size_t;
                                    (*zds).streamStage = zdss_init;
                                    someMoreWork = 0i32 as U32;
                                    continue ;
                                }
                            }
                        }
                        let errcod: size_t =
                            ZSTD_decompressBegin_usingDDict(zds,
                                                            (*zds).ddict);
                        if 0 != ERR_isError(errcod) {
                            return errcod
                        } else {
                            if MEM_readLE32((*zds).headerBuffer.as_mut_ptr()
                                                as *const libc::c_void) &
                                   4294967280u32 == 407710288u32 {
                                (*zds).expected =
                                    MEM_readLE32((*zds).headerBuffer.as_mut_ptr().offset(ZSTD_frameIdSize
                                                                                             as
                                                                                             isize)
                                                     as *const libc::c_void)
                                        as size_t;
                                (*zds).stage = ZSTDds_skipFrame
                            } else {
                                let errcod_0: size_t =
                                    ZSTD_decodeFrameHeader(zds,
                                                           (*zds).headerBuffer.as_mut_ptr()
                                                               as
                                                               *const libc::c_void,
                                                           (*zds).lhSize);
                                if 0 != ERR_isError(errcod_0) {
                                    return errcod_0
                                } else {
                                    (*zds).expected = ZSTD_blockHeaderSize;
                                    (*zds).stage = ZSTDds_decodeBlockHeader
                                }
                            }
                            (*zds).fParams.windowSize =
                                if (*zds).fParams.windowSize >
                                       (1u32 << 10i32) as libc::c_ulonglong {
                                    (*zds).fParams.windowSize
                                } else {
                                    (1u32 << 10i32) as libc::c_ulonglong
                                };
                            if (*zds).fParams.windowSize >
                                   (*zds).maxWindowSize as libc::c_ulonglong {
                                return -(ZSTD_error_frameParameter_windowTooLarge
                                             as libc::c_int) as size_t
                            } else {
                                let neededInBuffSize: size_t =
                                    (if (*zds).fParams.blockSizeMax >
                                            4i32 as libc::c_uint {
                                         (*zds).fParams.blockSizeMax
                                     } else { 4i32 as libc::c_uint }) as
                                        size_t;
                                let neededOutBuffSize: size_t =
                                    ZSTD_decodingBufferSize_min((*zds).fParams.windowSize,
                                                                (*zds).fParams.frameContentSize);
                                if (*zds).inBuffSize < neededInBuffSize ||
                                       (*zds).outBuffSize < neededOutBuffSize
                                   {
                                    let bufferSize: size_t =
                                        neededInBuffSize.wrapping_add(neededOutBuffSize);
                                    if 0 != (*zds).staticSize {
                                        if bufferSize >
                                               (*zds).staticSize.wrapping_sub(::std::mem::size_of::<ZSTD_DCtx>()
                                                                                  as
                                                                                  libc::c_ulong)
                                           {
                                            return -(ZSTD_error_memory_allocation
                                                         as libc::c_int) as
                                                       size_t
                                        }
                                    } else {
                                        ZSTD_free((*zds).inBuff as
                                                      *mut libc::c_void,
                                                  (*zds).customMem);
                                        (*zds).inBuffSize = 0i32 as size_t;
                                        (*zds).outBuffSize = 0i32 as size_t;
                                        (*zds).inBuff =
                                            ZSTD_malloc(bufferSize,
                                                        (*zds).customMem) as
                                                *mut libc::c_char;
                                        if (*zds).inBuff.is_null() {
                                            return -(ZSTD_error_memory_allocation
                                                         as libc::c_int) as
                                                       size_t
                                        }
                                    }
                                    (*zds).inBuffSize = neededInBuffSize;
                                    (*zds).outBuff =
                                        (*zds).inBuff.offset((*zds).inBuffSize
                                                                 as isize);
                                    (*zds).outBuffSize = neededOutBuffSize
                                }
                                (*zds).streamStage = zdss_read
                            }
                        }
                    }
                    current_block = 7427571413727699167;
                }
                _ => { }
            }
            match current_block {
                7427571413727699167 => {
                    let neededInSize: size_t =
                        ZSTD_nextSrcSizeToDecompress(zds);
                    if neededInSize == 0i32 as libc::c_ulong {
                        (*zds).streamStage = zdss_init;
                        someMoreWork = 0i32 as U32;
                        continue ;
                    } else if ip.offset_to(iend).expect("bad offset_to") as
                                  libc::c_long as size_t >= neededInSize {
                        let isSkipFrame: libc::c_int = ZSTD_isSkipFrame(zds);
                        let decodedSize: size_t =
                            ZSTD_decompressContinue(zds,
                                                    (*zds).outBuff.offset((*zds).outStart
                                                                              as
                                                                              isize)
                                                        as *mut libc::c_void,
                                                    if 0 != isSkipFrame {
                                                        0i32 as libc::c_ulong
                                                    } else {
                                                        (*zds).outBuffSize.wrapping_sub((*zds).outStart)
                                                    },
                                                    ip as *const libc::c_void,
                                                    neededInSize);
                        if 0 != ERR_isError(decodedSize) {
                            return decodedSize
                        } else {
                            ip = ip.offset(neededInSize as isize);
                            if 0 == decodedSize && 0 == isSkipFrame {
                                continue ;
                            }
                            (*zds).outEnd =
                                (*zds).outStart.wrapping_add(decodedSize);
                            (*zds).streamStage = zdss_flush;
                            continue ;
                        }
                    } else if ip == iend {
                        someMoreWork = 0i32 as U32;
                        continue ;
                    } else { (*zds).streamStage = zdss_load }
                    current_block = 2989495919056355252;
                }
                _ => { }
            }
            match current_block {
                2989495919056355252 => {
                    let neededInSize_0: size_t =
                        ZSTD_nextSrcSizeToDecompress(zds);
                    let toLoad_0: size_t =
                        neededInSize_0.wrapping_sub((*zds).inPos);
                    let isSkipFrame_0: libc::c_int = ZSTD_isSkipFrame(zds);
                    let mut loadedSize: size_t = 0;
                    if 0 != isSkipFrame_0 {
                        loadedSize =
                            if toLoad_0 <
                                   ip.offset_to(iend).expect("bad offset_to")
                                       as libc::c_long as size_t {
                                toLoad_0
                            } else {
                                ip.offset_to(iend).expect("bad offset_to") as
                                    libc::c_long as size_t
                            }
                    } else if toLoad_0 >
                                  (*zds).inBuffSize.wrapping_sub((*zds).inPos)
                     {
                        return -(ZSTD_error_corruption_detected as
                                     libc::c_int) as size_t
                    } else {
                        loadedSize =
                            ZSTD_limitCopy((*zds).inBuff.offset((*zds).inPos
                                                                    as isize)
                                               as *mut libc::c_void, toLoad_0,
                                           ip as *const libc::c_void,
                                           ip.offset_to(iend).expect("bad offset_to")
                                               as libc::c_long as size_t)
                    }
                    ip = ip.offset(loadedSize as isize);
                    (*zds).inPos =
                        ((*zds).inPos as
                             libc::c_ulong).wrapping_add(loadedSize) as size_t
                            as size_t;
                    if loadedSize < toLoad_0 {
                        someMoreWork = 0i32 as U32;
                        continue ;
                    } else {
                        let decodedSize_0: size_t =
                            ZSTD_decompressContinue(zds,
                                                    (*zds).outBuff.offset((*zds).outStart
                                                                              as
                                                                              isize)
                                                        as *mut libc::c_void,
                                                    (*zds).outBuffSize.wrapping_sub((*zds).outStart),
                                                    (*zds).inBuff as
                                                        *const libc::c_void,
                                                    neededInSize_0);
                        if 0 != ERR_isError(decodedSize_0) {
                            return decodedSize_0
                        } else {
                            (*zds).inPos = 0i32 as size_t;
                            if 0 == decodedSize_0 && 0 == isSkipFrame_0 {
                                (*zds).streamStage = zdss_read;
                                continue ;
                            } else {
                                (*zds).outEnd =
                                    (*zds).outStart.wrapping_add(decodedSize_0);
                                (*zds).streamStage = zdss_flush
                            }
                        }
                    }
                }
                _ => { }
            }
            let toFlushSize: size_t =
                (*zds).outEnd.wrapping_sub((*zds).outStart);
            let flushedSize: size_t =
                ZSTD_limitCopy(op as *mut libc::c_void,
                               op.offset_to(oend).expect("bad offset_to") as
                                   libc::c_long as size_t,
                               (*zds).outBuff.offset((*zds).outStart as isize)
                                   as *const libc::c_void, toFlushSize);
            op = op.offset(flushedSize as isize);
            (*zds).outStart =
                ((*zds).outStart as libc::c_ulong).wrapping_add(flushedSize)
                    as size_t as size_t;
            if flushedSize == toFlushSize {
                (*zds).streamStage = zdss_read;
                if !(((*zds).outBuffSize as libc::c_ulonglong) <
                         (*zds).fParams.frameContentSize &&
                         (*zds).outStart.wrapping_add((*zds).fParams.blockSizeMax
                                                          as libc::c_ulong) >
                             (*zds).outBuffSize) {
                    continue ;
                }
                (*zds).outEnd = 0i32 as size_t;
                (*zds).outStart = (*zds).outEnd
            } else { someMoreWork = 0i32 as U32 }
        }
        (*input).pos =
            ((*input).src as
                 *const libc::c_char).offset_to(ip).expect("bad offset_to") as
                libc::c_long as size_t;
        (*output).pos =
            ((*output).dst as
                 *mut libc::c_char).offset_to(op).expect("bad offset_to") as
                libc::c_long as size_t;
        if ip == istart && op == ostart {
            (*zds).noForwardProgress += 1;
            if (*zds).noForwardProgress >= 16i32 {
                if op == oend {
                    return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as
                               size_t
                } else if ip == iend {
                    return -(ZSTD_error_srcSize_wrong as libc::c_int) as
                               size_t
                }
            }
        } else { (*zds).noForwardProgress = 0i32 }
        let mut nextSrcSizeHint: size_t = ZSTD_nextSrcSizeToDecompress(zds);
        if 0 == nextSrcSizeHint {
            if (*zds).outEnd == (*zds).outStart {
                if 0 != (*zds).hostageByte {
                    if (*input).pos >= (*input).size {
                        (*zds).streamStage = zdss_read;
                        return 1i32 as size_t
                    } else { (*input).pos = (*input).pos.wrapping_add(1) }
                }
                return 0i32 as size_t
            } else {
                if 0 == (*zds).hostageByte {
                    (*input).pos = (*input).pos.wrapping_sub(1);
                    (*zds).hostageByte = 1i32 as U32
                }
                return 1i32 as size_t
            }
        } else {
            nextSrcSizeHint =
                (nextSrcSizeHint as
                     libc::c_ulong).wrapping_add(ZSTD_blockHeaderSize.wrapping_mul((ZSTD_nextInputType(zds)
                                                                                        as
                                                                                        libc::c_uint
                                                                                        ==
                                                                                        ZSTDnit_block
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint)
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong))
                    as size_t as size_t;
            nextSrcSizeHint =
                (nextSrcSizeHint as libc::c_ulong).wrapping_sub((*zds).inPos)
                    as size_t as size_t;
            return nextSrcSizeHint
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_nextSrcSizeToDecompress(mut dctx:
                                                          *mut ZSTD_DCtx)
 -> size_t {
    return (*dctx).expected;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_nextInputType(mut dctx: *mut ZSTD_DCtx)
 -> ZSTD_nextInputType_e {
    match (*dctx).stage as libc::c_uint {
        2 => { return ZSTDnit_blockHeader }
        3 => { return ZSTDnit_block }
        4 => { return ZSTDnit_lastBlock }
        5 => { return ZSTDnit_checksum }
        6 | 7 => { return ZSTDnit_skippableFrame }
        0 | 1 | _ => { return ZSTDnit_frameHeader }
    };
}
unsafe extern "C" fn ZSTD_limitCopy(mut dst: *mut libc::c_void,
                                    mut dstCapacity: size_t,
                                    mut src: *const libc::c_void,
                                    mut srcSize: size_t) -> size_t {
    let length: size_t =
        if dstCapacity < srcSize { dstCapacity } else { srcSize };
    memcpy(dst, src, length);
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressContinue(mut dctx: *mut ZSTD_DCtx,
                                                 mut dst: *mut libc::c_void,
                                                 mut dstCapacity: size_t,
                                                 mut src: *const libc::c_void,
                                                 mut srcSize: size_t)
 -> size_t {
    if srcSize != (*dctx).expected {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else {
        if 0 != dstCapacity { ZSTD_checkContinuity(dctx, dst); }
        match (*dctx).stage as libc::c_uint {
            0 => {
                if (*dctx).format as libc::c_uint ==
                       ZSTD_f_zstd1 as libc::c_int as libc::c_uint {
                    if MEM_readLE32(src) & 4294967280u32 == 407710288u32 {
                        memcpy((*dctx).headerBuffer.as_mut_ptr() as
                                   *mut libc::c_void, src, srcSize);
                        (*dctx).expected =
                            ZSTD_skippableHeaderSize.wrapping_sub(srcSize);
                        (*dctx).stage = ZSTDds_decodeSkippableHeader;
                        return 0i32 as size_t
                    }
                }
                (*dctx).headerSize =
                    ZSTD_frameHeaderSize_internal(src, srcSize,
                                                  (*dctx).format);
                if 0 != ERR_isError((*dctx).headerSize) {
                    return (*dctx).headerSize
                } else {
                    memcpy((*dctx).headerBuffer.as_mut_ptr() as
                               *mut libc::c_void, src, srcSize);
                    (*dctx).expected =
                        (*dctx).headerSize.wrapping_sub(srcSize);
                    (*dctx).stage = ZSTDds_decodeFrameHeader;
                    return 0i32 as size_t
                }
            }
            1 => {
                memcpy((*dctx).headerBuffer.as_mut_ptr().offset((*dctx).headerSize.wrapping_sub(srcSize)
                                                                    as isize)
                           as *mut libc::c_void, src, srcSize);
                let errcod: size_t =
                    ZSTD_decodeFrameHeader(dctx,
                                           (*dctx).headerBuffer.as_mut_ptr()
                                               as *const libc::c_void,
                                           (*dctx).headerSize);
                if 0 != ERR_isError(errcod) {
                    return errcod
                } else {
                    (*dctx).expected = ZSTD_blockHeaderSize;
                    (*dctx).stage = ZSTDds_decodeBlockHeader;
                    return 0i32 as size_t
                }
            }
            2 => {
                let mut bp: blockProperties_t =
                    blockProperties_t{blockType: bt_raw,
                                      lastBlock: 0,
                                      origSize: 0,};
                let cBlockSize: size_t =
                    ZSTD_getcBlockSize(src, ZSTD_blockHeaderSize,
                                       &mut bp as *mut blockProperties_t);
                if 0 != ERR_isError(cBlockSize) {
                    return cBlockSize
                } else {
                    (*dctx).expected = cBlockSize;
                    (*dctx).bType = bp.blockType;
                    (*dctx).rleSize = bp.origSize as size_t;
                    if 0 != cBlockSize {
                        (*dctx).stage =
                            (if 0 != bp.lastBlock {
                                 ZSTDds_decompressLastBlock as libc::c_int
                             } else { ZSTDds_decompressBlock as libc::c_int })
                                as ZSTD_dStage;
                        return 0i32 as size_t
                    } else {
                        if 0 != bp.lastBlock {
                            if 0 != (*dctx).fParams.checksumFlag {
                                (*dctx).expected = 4i32 as size_t;
                                (*dctx).stage = ZSTDds_checkChecksum
                            } else {
                                (*dctx).expected = 0i32 as size_t;
                                (*dctx).stage = ZSTDds_getFrameHeaderSize
                            }
                        } else {
                            (*dctx).expected = ZSTD_blockHeaderSize;
                            (*dctx).stage = ZSTDds_decodeBlockHeader
                        }
                        return 0i32 as size_t
                    }
                }
            }
            4 | 3 => {
                let mut rSize: size_t = 0;
                match (*dctx).bType as libc::c_uint {
                    2 => {
                        rSize =
                            ZSTD_decompressBlock_internal(dctx, dst,
                                                          dstCapacity, src,
                                                          srcSize, 1i32)
                    }
                    0 => {
                        rSize =
                            ZSTD_copyRawBlock(dst, dstCapacity, src, srcSize)
                    }
                    1 => {
                        rSize =
                            ZSTD_setRleBlock(dst, dstCapacity, src, srcSize,
                                             (*dctx).rleSize)
                    }
                    3 | _ => {
                        return -(ZSTD_error_corruption_detected as
                                     libc::c_int) as size_t
                    }
                }
                if 0 != ERR_isError(rSize) {
                    return rSize
                } else {
                    (*dctx).decodedSize =
                        ((*dctx).decodedSize as
                             libc::c_ulong).wrapping_add(rSize) as U64 as U64;
                    if 0 != (*dctx).fParams.checksumFlag {
                        ZSTD_XXH64_update(&mut (*dctx).xxhState as
                                              *mut XXH64_state_t, dst, rSize);
                    }
                    if (*dctx).stage as libc::c_uint ==
                           ZSTDds_decompressLastBlock as libc::c_int as
                               libc::c_uint {
                        if (*dctx).fParams.frameContentSize !=
                               0u64.wrapping_sub(1i32 as libc::c_ulonglong) {
                            if (*dctx).decodedSize as libc::c_ulonglong !=
                                   (*dctx).fParams.frameContentSize {
                                return -(ZSTD_error_corruption_detected as
                                             libc::c_int) as size_t
                            }
                        }
                        if 0 != (*dctx).fParams.checksumFlag {
                            (*dctx).expected = 4i32 as size_t;
                            (*dctx).stage = ZSTDds_checkChecksum
                        } else {
                            (*dctx).expected = 0i32 as size_t;
                            (*dctx).stage = ZSTDds_getFrameHeaderSize
                        }
                    } else {
                        (*dctx).stage = ZSTDds_decodeBlockHeader;
                        (*dctx).expected = ZSTD_blockHeaderSize;
                        (*dctx).previousDstEnd =
                            (dst as *mut libc::c_char).offset(rSize as isize)
                                as *const libc::c_void
                    }
                    return rSize
                }
            }
            5 => {
                let h32: U32 =
                    ZSTD_XXH64_digest(&mut (*dctx).xxhState as
                                          *mut XXH64_state_t) as U32;
                let check32: U32 = MEM_readLE32(src);
                if check32 != h32 {
                    return -(ZSTD_error_checksum_wrong as libc::c_int) as
                               size_t
                } else {
                    (*dctx).expected = 0i32 as size_t;
                    (*dctx).stage = ZSTDds_getFrameHeaderSize;
                    return 0i32 as size_t
                }
            }
            6 => {
                memcpy((*dctx).headerBuffer.as_mut_ptr().offset(ZSTD_skippableHeaderSize.wrapping_sub(srcSize)
                                                                    as isize)
                           as *mut libc::c_void, src, srcSize);
                (*dctx).expected =
                    MEM_readLE32((*dctx).headerBuffer.as_mut_ptr().offset(ZSTD_frameIdSize
                                                                              as
                                                                              isize)
                                     as *const libc::c_void) as size_t;
                (*dctx).stage = ZSTDds_skipFrame;
                return 0i32 as size_t
            }
            7 => {
                (*dctx).expected = 0i32 as size_t;
                (*dctx).stage = ZSTDds_getFrameHeaderSize;
                return 0i32 as size_t
            }
            _ => { return -(ZSTD_error_GENERIC as libc::c_int) as size_t }
        }
    };
}
unsafe extern "C" fn ZSTD_setRleBlock(mut dst: *mut libc::c_void,
                                      mut dstCapacity: size_t,
                                      mut src: *const libc::c_void,
                                      mut srcSize: size_t,
                                      mut regenSize: size_t) -> size_t {
    if srcSize != 1i32 as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else if regenSize > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else {
        memset(dst, *(src as *const BYTE) as libc::c_int, regenSize);
        return regenSize
    };
}
unsafe extern "C" fn ZSTD_isSkipFrame(mut dctx: *mut ZSTD_DCtx)
 -> libc::c_int {
    return ((*dctx).stage as libc::c_uint ==
                ZSTDds_skipFrame as libc::c_int as libc::c_uint) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decodingBufferSize_min(mut windowSize:
                                                         libc::c_ulonglong,
                                                     mut frameContentSize:
                                                         libc::c_ulonglong)
 -> size_t {
    let blockSize: size_t =
        (if windowSize < (1i32 << 17i32) as libc::c_ulonglong {
             windowSize
         } else { (1i32 << 17i32) as libc::c_ulonglong }) as size_t;
    let neededRBSize: libc::c_ulonglong =
        windowSize.wrapping_add(blockSize as
                                    libc::c_ulonglong).wrapping_add((8i32 *
                                                                         2i32)
                                                                        as
                                                                        libc::c_ulonglong);
    let neededSize: libc::c_ulonglong =
        if frameContentSize < neededRBSize {
            frameContentSize
        } else { neededRBSize };
    let minRBSize: size_t = neededSize as size_t;
    if minRBSize as libc::c_ulonglong != neededSize {
        return -(ZSTD_error_frameParameter_windowTooLarge as libc::c_int) as
                   size_t
    } else { return minRBSize };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_findFrameCompressedSize(mut src:
                                                          *const libc::c_void,
                                                      mut srcSize: size_t)
 -> size_t {
    if srcSize >= ZSTD_skippableHeaderSize &&
           MEM_readLE32(src) & 4294967280u32 == 407710288u32 {
        return ZSTD_skippableHeaderSize.wrapping_add(MEM_readLE32((src as
                                                                       *const BYTE).offset(ZSTD_frameIdSize
                                                                                               as
                                                                                               isize)
                                                                      as
                                                                      *const libc::c_void)
                                                         as libc::c_ulong)
    } else {
        let mut ip: *const BYTE = src as *const BYTE;
        let ipstart: *const BYTE = ip;
        let mut remainingSize: size_t = srcSize;
        let mut zfh: ZSTD_frameHeader =
            ZSTD_frameHeader{frameContentSize: 0,
                             windowSize: 0,
                             blockSizeMax: 0,
                             frameType: ZSTD_frame,
                             headerSize: 0,
                             dictID: 0,
                             checksumFlag: 0,};
        let ret: size_t =
            ZSTD_getFrameHeader(&mut zfh as *mut ZSTD_frameHeader, src,
                                srcSize);
        if 0 != ERR_isError(ret) {
            return ret
        } else if ret > 0i32 as libc::c_ulong {
            return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
        } else {
            ip = ip.offset(zfh.headerSize as isize);
            remainingSize =
                (remainingSize as
                     libc::c_ulong).wrapping_sub(zfh.headerSize as
                                                     libc::c_ulong) as size_t
                    as size_t;
            loop  {
                let mut blockProperties: blockProperties_t =
                    blockProperties_t{blockType: bt_raw,
                                      lastBlock: 0,
                                      origSize: 0,};
                let cBlockSize: size_t =
                    ZSTD_getcBlockSize(ip as *const libc::c_void,
                                       remainingSize,
                                       &mut blockProperties as
                                           *mut blockProperties_t);
                if 0 != ERR_isError(cBlockSize) {
                    return cBlockSize
                } else if ZSTD_blockHeaderSize.wrapping_add(cBlockSize) >
                              remainingSize {
                    return -(ZSTD_error_srcSize_wrong as libc::c_int) as
                               size_t
                } else {
                    ip =
                        ip.offset(ZSTD_blockHeaderSize.wrapping_add(cBlockSize)
                                      as isize);
                    remainingSize =
                        (remainingSize as
                             libc::c_ulong).wrapping_sub(ZSTD_blockHeaderSize.wrapping_add(cBlockSize))
                            as size_t as size_t;
                    if 0 != blockProperties.lastBlock { break ; }
                }
            }
            if 0 != zfh.checksumFlag {
                if remainingSize < 4i32 as libc::c_ulong {
                    return -(ZSTD_error_srcSize_wrong as libc::c_int) as
                               size_t
                } else {
                    ip = ip.offset(4isize);
                    remainingSize =
                        (remainingSize as
                             libc::c_ulong).wrapping_sub(4i32 as
                                                             libc::c_ulong) as
                            size_t as size_t
                }
            }
            return ipstart.offset_to(ip).expect("bad offset_to") as
                       libc::c_long as size_t
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_resetDStream(mut dctx: *mut ZSTD_DStream)
 -> size_t {
    (*dctx).streamStage = zdss_loadHeader;
    (*dctx).outEnd = 0i32 as size_t;
    (*dctx).outStart = (*dctx).outEnd;
    (*dctx).inPos = (*dctx).outStart;
    (*dctx).lhSize = (*dctx).inPos;
    (*dctx).legacyVersion = 0i32 as U32;
    (*dctx).hostageByte = 0i32 as U32;
    return ZSTD_frameHeaderSize_prefix;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DStreamInSize() -> size_t {
    return ((1i32 << 17i32) as
                libc::c_ulong).wrapping_add(ZSTD_blockHeaderSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DStreamOutSize() -> size_t {
    return (1i32 << 17i32) as size_t;
}
static mut ZSTD_frameHeaderSize_max: size_t = unsafe { 18i32 as size_t };
#[no_mangle]
pub unsafe extern "C" fn ZSTD_findDecompressedSize(mut src:
                                                       *const libc::c_void,
                                                   mut srcSize: size_t)
 -> libc::c_ulonglong {
    let mut totalDstSize: libc::c_ulonglong = 0i32 as libc::c_ulonglong;
    while srcSize >= ZSTD_frameHeaderSize_prefix {
        let magicNumber: U32 = MEM_readLE32(src);
        if magicNumber & 4294967280u32 == 407710288u32 {
            let mut skippableSize: size_t = 0;
            if srcSize < ZSTD_skippableHeaderSize {
                return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t as
                           libc::c_ulonglong
            } else {
                skippableSize =
                    (MEM_readLE32((src as
                                       *const BYTE).offset(ZSTD_frameIdSize as
                                                               isize) as
                                      *const libc::c_void) as
                         libc::c_ulong).wrapping_add(ZSTD_skippableHeaderSize);
                if srcSize < skippableSize {
                    return 0u64.wrapping_sub(2i32 as libc::c_ulonglong)
                } else {
                    src =
                        (src as *const BYTE).offset(skippableSize as isize) as
                            *const libc::c_void;
                    srcSize =
                        (srcSize as libc::c_ulong).wrapping_sub(skippableSize)
                            as size_t as size_t
                }
            }
        } else {
            let ret: libc::c_ulonglong =
                ZSTD_getFrameContentSize(src, srcSize);
            if ret >= 0u64.wrapping_sub(2i32 as libc::c_ulonglong) {
                return ret
            } else if totalDstSize.wrapping_add(ret) < totalDstSize {
                return 0u64.wrapping_sub(2i32 as libc::c_ulonglong)
            } else {
                totalDstSize = totalDstSize.wrapping_add(ret);
                let frameSrcSize: size_t =
                    ZSTD_findFrameCompressedSize(src, srcSize);
                if 0 != ERR_isError(frameSrcSize) {
                    return 0u64.wrapping_sub(2i32 as libc::c_ulonglong)
                } else {
                    src =
                        (src as *const BYTE).offset(frameSrcSize as isize) as
                            *const libc::c_void;
                    srcSize =
                        (srcSize as libc::c_ulong).wrapping_sub(frameSrcSize)
                            as size_t as size_t
                }
            }
        }
    }
    if 0 != srcSize {
        return 0u64.wrapping_sub(2i32 as libc::c_ulonglong)
    } else { return totalDstSize };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_DCtx(mut dctx: *const ZSTD_DCtx)
 -> size_t {
    if dctx.is_null() {
        return 0i32 as size_t
    } else {
        return (::std::mem::size_of::<ZSTD_DCtx>() as
                    libc::c_ulong).wrapping_add(ZSTD_sizeof_DDict((*dctx).ddictLocal)).wrapping_add((*dctx).inBuffSize).wrapping_add((*dctx).outBuffSize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_DDict(mut ddict: *const ZSTD_DDict)
 -> size_t {
    if ddict.is_null() {
        return 0i32 as size_t
    } else {
        return (::std::mem::size_of::<ZSTD_DDict>() as
                    libc::c_ulong).wrapping_add(if !(*ddict).dictBuffer.is_null()
                                                   {
                                                    (*ddict).dictSize
                                                } else {
                                                    0i32 as libc::c_ulong
                                                })
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_DStream(mut dctx: *const ZSTD_DStream)
 -> size_t {
    return ZSTD_sizeof_DCtx(dctx);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateDCtxSize() -> size_t {
    return ::std::mem::size_of::<ZSTD_DCtx>() as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateDStreamSize(mut windowSize: size_t)
 -> size_t {
    let blockSize: size_t =
        if windowSize < (1i32 << 17i32) as libc::c_ulong {
            windowSize
        } else { (1i32 << 17i32) as libc::c_ulong };
    let inBuffSize: size_t = blockSize;
    let outBuffSize: size_t =
        ZSTD_decodingBufferSize_min(windowSize as libc::c_ulonglong,
                                    0u64.wrapping_sub(1i32 as
                                                          libc::c_ulonglong));
    return ZSTD_estimateDCtxSize().wrapping_add(inBuffSize).wrapping_add(outBuffSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateDStreamSize_fromFrame(mut src:
                                                                *const libc::c_void,
                                                            mut srcSize:
                                                                size_t)
 -> size_t {
    let windowSizeMax: U32 =
        1u32 <<
            (if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                    4i32 as libc::c_ulong {
                 30i32
             } else { 31i32 }) as libc::c_uint;
    let mut zfh: ZSTD_frameHeader =
        ZSTD_frameHeader{frameContentSize: 0,
                         windowSize: 0,
                         blockSizeMax: 0,
                         frameType: ZSTD_frame,
                         headerSize: 0,
                         dictID: 0,
                         checksumFlag: 0,};
    let err: size_t =
        ZSTD_getFrameHeader(&mut zfh as *mut ZSTD_frameHeader, src, srcSize);
    if 0 != ERR_isError(err) {
        return err
    } else if err > 0i32 as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else if zfh.windowSize > windowSizeMax as libc::c_ulonglong {
        return -(ZSTD_error_frameParameter_windowTooLarge as libc::c_int) as
                   size_t
    } else { return ZSTD_estimateDStreamSize(zfh.windowSize as size_t) };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateDDictSize(mut dictSize: size_t,
                                                mut dictLoadMethod:
                                                    ZSTD_dictLoadMethod_e)
 -> size_t {
    return (::std::mem::size_of::<ZSTD_DDict>() as
                libc::c_ulong).wrapping_add(if dictLoadMethod as libc::c_uint
                                                   ==
                                                   ZSTD_dlm_byRef as
                                                       libc::c_int as
                                                       libc::c_uint {
                                                0i32 as libc::c_ulong
                                            } else { dictSize });
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticDCtx(mut workspace: *mut libc::c_void,
                                             mut workspaceSize: size_t)
 -> *mut ZSTD_DCtx {
    let dctx: *mut ZSTD_DCtx = workspace as *mut ZSTD_DCtx;
    if 0 != workspace as size_t & 7i32 as libc::c_ulong {
        return 0 as *mut ZSTD_DCtx
    } else if workspaceSize <
                  ::std::mem::size_of::<ZSTD_DCtx>() as libc::c_ulong {
        return 0 as *mut ZSTD_DCtx
    } else {
        ZSTD_initDCtx_internal(dctx);
        (*dctx).staticSize = workspaceSize;
        (*dctx).inBuff = dctx.offset(1isize) as *mut libc::c_char;
        return dctx
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticDStream(mut workspace:
                                                    *mut libc::c_void,
                                                mut workspaceSize: size_t)
 -> *mut ZSTD_DStream {
    return ZSTD_initStaticDCtx(workspace, workspaceSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticDDict(mut workspace:
                                                  *mut libc::c_void,
                                              mut workspaceSize: size_t,
                                              mut dict: *const libc::c_void,
                                              mut dictSize: size_t,
                                              mut dictLoadMethod:
                                                  ZSTD_dictLoadMethod_e,
                                              mut dictContentType:
                                                  ZSTD_dictContentType_e)
 -> *const ZSTD_DDict {
    let neededSpace: size_t =
        (::std::mem::size_of::<ZSTD_DDict>() as
             libc::c_ulong).wrapping_add(if dictLoadMethod as libc::c_uint ==
                                                ZSTD_dlm_byRef as libc::c_int
                                                    as libc::c_uint {
                                             0i32 as libc::c_ulong
                                         } else { dictSize });
    let ddict: *mut ZSTD_DDict = workspace as *mut ZSTD_DDict;
    if 0 != workspace as size_t & 7i32 as libc::c_ulong {
        return 0 as *const ZSTD_DDict
    } else if workspaceSize < neededSpace {
        return 0 as *const ZSTD_DDict
    } else {
        if dictLoadMethod as libc::c_uint ==
               ZSTD_dlm_byCopy as libc::c_int as libc::c_uint {
            memcpy(ddict.offset(1isize) as *mut libc::c_void, dict, dictSize);
            dict = ddict.offset(1isize) as *const libc::c_void
        }
        if 0 !=
               ERR_isError(ZSTD_initDDict_internal(ddict, dict, dictSize,
                                                   ZSTD_dlm_byRef,
                                                   dictContentType)) {
            return 0 as *const ZSTD_DDict
        } else { return ddict }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_isFrame(mut buffer: *const libc::c_void,
                                      mut size: size_t) -> libc::c_uint {
    if size < ZSTD_frameIdSize {
        return 0i32 as libc::c_uint
    } else {
        let magic: U32 = MEM_readLE32(buffer);
        if magic == 4247762216u32 {
            return 1i32 as libc::c_uint
        } else if magic & 4294967280u32 == 407710288u32 {
            return 1i32 as libc::c_uint
        } else { return 0i32 as libc::c_uint }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDDict_byReference(mut dictBuffer:
                                                          *const libc::c_void,
                                                      mut dictSize: size_t)
 -> *mut ZSTD_DDict {
    let allocator: ZSTD_customMem =
        ZSTD_customMem{customAlloc: None,
                       customFree: None,
                       opaque: 0 as *mut libc::c_void,};
    return ZSTD_createDDict_advanced(dictBuffer, dictSize, ZSTD_dlm_byRef,
                                     ZSTD_dct_auto, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDictID_fromDict(mut dict:
                                                     *const libc::c_void,
                                                 mut dictSize: size_t)
 -> libc::c_uint {
    if dictSize < 8i32 as libc::c_ulong {
        return 0i32 as libc::c_uint
    } else if MEM_readLE32(dict) != 3962610743u32 {
        return 0i32 as libc::c_uint
    } else {
        return MEM_readLE32((dict as
                                 *const libc::c_char).offset(ZSTD_frameIdSize
                                                                 as isize) as
                                *const libc::c_void)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDictID_fromDDict(mut ddict:
                                                      *const ZSTD_DDict)
 -> libc::c_uint {
    if ddict.is_null() {
        return 0i32 as libc::c_uint
    } else {
        return ZSTD_getDictID_fromDict((*ddict).dictContent,
                                       (*ddict).dictSize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDictID_fromFrame(mut src:
                                                      *const libc::c_void,
                                                  mut srcSize: size_t)
 -> libc::c_uint {
    let mut zfp: ZSTD_frameHeader =
        ZSTD_frameHeader{frameContentSize: 0i32 as libc::c_ulonglong,
                         windowSize: 0i32 as libc::c_ulonglong,
                         blockSizeMax: 0i32 as libc::c_uint,
                         frameType: ZSTD_frame,
                         headerSize: 0i32 as libc::c_uint,
                         dictID: 0i32 as libc::c_uint,
                         checksumFlag: 0i32 as libc::c_uint,};
    let hError: size_t =
        ZSTD_getFrameHeader(&mut zfp as *mut ZSTD_frameHeader, src, srcSize);
    if 0 != ERR_isError(hError) {
        return 0i32 as libc::c_uint
    } else { return zfp.dictID };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_setDStreamParameter(mut dctx: *mut ZSTD_DStream,
                                                  mut paramType:
                                                      ZSTD_DStreamParameter_e,
                                                  mut paramValue:
                                                      libc::c_uint)
 -> size_t {
    if (*dctx).streamStage as libc::c_uint !=
           zdss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    } else {
        match paramType as libc::c_uint {
            0 => {
                (*dctx).maxWindowSize =
                    (if 0 != paramValue { paramValue } else { -1i32 as U32 })
                        as size_t;
                return 0i32 as size_t
            }
            _ => {
                return -(ZSTD_error_parameter_unsupported as libc::c_int) as
                           size_t
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initDStream_usingDDict(mut dctx:
                                                         *mut ZSTD_DStream,
                                                     mut ddict:
                                                         *const ZSTD_DDict)
 -> size_t {
    let initResult: size_t = ZSTD_initDStream(dctx);
    (*dctx).ddict = ddict;
    return initResult;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_copyDCtx(mut dstDCtx: *mut ZSTD_DCtx,
                                       mut srcDCtx: *const ZSTD_DCtx) -> () {
    let toCopy: size_t =
        (dstDCtx as
             *mut libc::c_char).offset_to(&mut (*dstDCtx).inBuff as
                                              *mut *mut libc::c_char as
                                              *mut libc::c_char).expect("bad offset_to")
            as libc::c_long as size_t;
    memcpy(dstDCtx as *mut libc::c_void, srcDCtx as *const libc::c_void,
           toCopy);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_loadDictionary_byReference(mut dctx:
                                                                  *mut ZSTD_DCtx,
                                                              mut dict:
                                                                  *const libc::c_void,
                                                              mut dictSize:
                                                                  size_t)
 -> size_t {
    return ZSTD_DCtx_loadDictionary_advanced(dctx, dict, dictSize,
                                             ZSTD_dlm_byRef, ZSTD_dct_auto);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_refDDict(mut dctx: *mut ZSTD_DCtx,
                                            mut ddict: *const ZSTD_DDict)
 -> size_t {
    if (*dctx).streamStage as libc::c_uint !=
           zdss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    } else { (*dctx).ddict = ddict; return 0i32 as size_t };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_refPrefix(mut dctx: *mut ZSTD_DCtx,
                                             mut prefix: *const libc::c_void,
                                             mut prefixSize: size_t)
 -> size_t {
    return ZSTD_DCtx_refPrefix_advanced(dctx, prefix, prefixSize,
                                        ZSTD_dct_rawContent);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_refPrefix_advanced(mut dctx:
                                                          *mut ZSTD_DCtx,
                                                      mut prefix:
                                                          *const libc::c_void,
                                                      mut prefixSize: size_t,
                                                      mut dictContentType:
                                                          ZSTD_dictContentType_e)
 -> size_t {
    return ZSTD_DCtx_loadDictionary_advanced(dctx, prefix, prefixSize,
                                             ZSTD_dlm_byRef, dictContentType);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_setMaxWindowSize(mut dctx: *mut ZSTD_DCtx,
                                                    mut maxWindowSize: size_t)
 -> size_t {
    if (*dctx).streamStage as libc::c_uint !=
           zdss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    } else { (*dctx).maxWindowSize = maxWindowSize; return 0i32 as size_t };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_setFormat(mut dctx: *mut ZSTD_DCtx,
                                             mut format: ZSTD_format_e)
 -> size_t {
    if (*dctx).streamStage as libc::c_uint !=
           zdss_init as libc::c_int as libc::c_uint {
        return -(ZSTD_error_stage_wrong as libc::c_int) as size_t
    } else { (*dctx).format = format; return 0i32 as size_t };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompress_generic(mut dctx: *mut ZSTD_DCtx,
                                                 mut output:
                                                     *mut ZSTD_outBuffer,
                                                 mut input:
                                                     *mut ZSTD_inBuffer)
 -> size_t {
    return ZSTD_decompressStream(dctx, output, input);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompress_generic_simpleArgs(mut dctx:
                                                                *mut ZSTD_DCtx,
                                                            mut dst:
                                                                *mut libc::c_void,
                                                            mut dstCapacity:
                                                                size_t,
                                                            mut dstPos:
                                                                *mut size_t,
                                                            mut src:
                                                                *const libc::c_void,
                                                            mut srcSize:
                                                                size_t,
                                                            mut srcPos:
                                                                *mut size_t)
 -> size_t {
    let mut output: ZSTD_outBuffer =
        ZSTD_outBuffer_s{dst: dst, size: dstCapacity, pos: *dstPos,};
    let mut input: ZSTD_inBuffer =
        ZSTD_inBuffer_s{src: src, size: srcSize, pos: *srcPos,};
    let cErr: size_t =
        ZSTD_decompress_generic(dctx, &mut output as *mut ZSTD_outBuffer,
                                &mut input as *mut ZSTD_inBuffer);
    *dstPos = output.pos;
    *srcPos = input.pos;
    return cErr;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_reset(mut dctx: *mut ZSTD_DCtx) -> () {
    ZSTD_initDStream(dctx);
    (*dctx).format = ZSTD_f_zstd1;
    (*dctx).maxWindowSize =
        ((1i32 as U32) << 27i32).wrapping_add(1i32 as libc::c_uint) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBlock(mut dctx: *mut ZSTD_DCtx,
                                              mut dst: *mut libc::c_void,
                                              mut dstCapacity: size_t,
                                              mut src: *const libc::c_void,
                                              mut srcSize: size_t) -> size_t {
    let mut dSize: size_t = 0;
    ZSTD_checkContinuity(dctx, dst);
    dSize =
        ZSTD_decompressBlock_internal(dctx, dst, dstCapacity, src, srcSize,
                                      0i32);
    (*dctx).previousDstEnd =
        (dst as *mut libc::c_char).offset(dSize as isize) as
            *const libc::c_void;
    return dSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_insertBlock(mut dctx: *mut ZSTD_DCtx,
                                          mut blockStart: *const libc::c_void,
                                          mut blockSize: size_t) -> size_t {
    ZSTD_checkContinuity(dctx, blockStart);
    (*dctx).previousDstEnd =
        (blockStart as *const libc::c_char).offset(blockSize as isize) as
            *const libc::c_void;
    return blockSize;
}
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
