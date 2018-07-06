#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( libc , offset_to )]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
    /* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
    /* ! Constructor and Destructor of FSE_DTable.
    Note that its size depends on 'tableLog' */
    /* ! FSE_decompress():
    Decompress FSE data from buffer 'cSrc', of size 'cSrcSize',
    into already allocated destination buffer 'dst', of size 'dstCapacity'.
    @return : size of regenerated data (<= maxDstSize),
              or an error code, which can be tested using FSE_isError() .

    ** Important ** : FSE_decompress() does not decompress non-compressible nor RLE data !!!
    Why ? : making this distinction requires a header.
    Header management is intentionally delegated to the user layer, which can better manage special cases.
*/
    /* *< build a fake FSE_DTable, designed to always generate the same symbolValue */
    /* ! FSE_decompress_usingDTable():
    Decompress compressed source `cSrc` of size `cSrcSize` using `dt`
    into `dst` which must be already allocated.
    @return : size of regenerated data (necessarily <= `dstCapacity`),
              or an errorCode, which can be tested using FSE_isError() */
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
    /* ! FSE_buildDTable():
    Builds 'dt', which must be already allocated, using FSE_createDTable().
    return : 0, or an errorCode, which can be tested using FSE_isError() */
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
}
pub type unnamed = libc::c_uint;
pub const ZSTD_error_frameParameter_unsupported: ERR_enum = 14;
pub const ZSTD_error_parameter_unsupported: ERR_enum = 40;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_CStream_t {
    pub bitContainer: size_t,
    pub bitPos: libc::c_uint,
    pub startPtr: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub endPtr: *mut libc::c_char,
}
pub type uint8_t = libc::c_uchar;
pub const ZSTD_error_checksum_wrong: ERR_enum = 22;
pub const ZSTD_error_prefix_unknown: ERR_enum = 10;
pub const ZSTD_error_workSpace_tooSmall: ERR_enum = 66;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DState_t {
    pub state: size_t,
    pub table: *const libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
pub const ZSTD_error_seekableIO: ERR_enum = 102;
pub const ZSTD_error_frameParameter_windowTooLarge: ERR_enum = 16;
pub const ZSTD_error_GENERIC: ERR_enum = 1;
pub const ZSTD_error_no_error: ERR_enum = 0;
pub const ZSTD_error_memory_allocation: ERR_enum = 64;
pub const ZSTD_error_maxCode: ERR_enum = 120;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: libc::c_uint,
    pub ptr: *const libc::c_char,
    pub start: *const libc::c_char,
    pub limitPtr: *const libc::c_char,
}
pub type DTable_max_t = [FSE_DTable; 4097];
pub type uint64_t = libc::c_ulong;
pub const ZSTD_error_version_unsupported: ERR_enum = 12;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: libc::c_int,
    pub deltaNbBits: U32,
}
pub const ZSTD_error_init_missing: ERR_enum = 62;
pub type FSE_CTable = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
pub type uint16_t = libc::c_ushort;
pub const ZSTD_error_srcSize_wrong: ERR_enum = 72;
pub const ZSTD_error_frameIndex_tooLarge: ERR_enum = 100;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub const ZSTD_error_maxSymbolValue_tooSmall: ERR_enum = 48;
pub const ZSTD_error_dstSize_tooSmall: ERR_enum = 70;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub type S16 = int16_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
pub const ZSTD_error_dictionary_wrong: ERR_enum = 32;
pub type FSE_DTable = libc::c_uint;
pub type uint32_t = libc::c_uint;
pub const ZSTD_error_maxSymbolValue_tooLarge: ERR_enum = 46;
pub const ZSTD_error_dictionaryCreation_failed: ERR_enum = 34;
pub type BIT_DStream_status = libc::c_uint;
pub const MEM_static_assert: unnamed_0 = 1;
pub const MEM_static_assert_0: unnamed = 1;
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type BYTE = uint8_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
pub const ZSTD_error_stage_wrong: ERR_enum = 60;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub const ZSTD_error_parameter_outOfBound: ERR_enum = 42;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub type U32 = uint32_t;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
pub type U16 = uint16_t;
pub type unnamed_0 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    u: U32,
    c: [BYTE; 4],
}
pub const ZSTD_error_dictionary_corrupted: ERR_enum = 30;
pub type ZSTD_ErrorCode = ERR_enum;
pub const ZSTD_error_tableLog_tooLarge: ERR_enum = 44;
pub type int16_t = libc::c_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const libc::c_void,
    pub symbolTT: *const libc::c_void,
    pub stateLog: libc::c_uint,
}
pub type U64 = uint64_t;
pub const ZSTD_error_corruption_detected: ERR_enum = 20;
pub type ERR_enum = libc::c_uint;
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
                    current_block = 4178014515982532010;
                }
                6 => { current_block = 4178014515982532010; }
                5 => { current_block = 5141172686473368009; }
                4 => { current_block = 3174334154968278585; }
                3 => { current_block = 16070532456075173465; }
                2 => { current_block = 6658210076141074664; }
                _ => { current_block = 10886091980245723256; }
            }
            match current_block {
                4178014515982532010 => {
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
                    current_block = 5141172686473368009;
                }
                _ => { }
            }
            match current_block {
                5141172686473368009 => {
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
                    current_block = 3174334154968278585;
                }
                _ => { }
            }
            match current_block {
                3174334154968278585 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(3isize)
                                                              as size_t) <<
                                                             24i32) as size_t
                            as size_t;
                    current_block = 16070532456075173465;
                }
                _ => { }
            }
            match current_block {
                16070532456075173465 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(2isize)
                                                              as size_t) <<
                                                             16i32) as size_t
                            as size_t;
                    current_block = 6658210076141074664;
                }
                _ => { }
            }
            match current_block {
                6658210076141074664 => {
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
#[no_mangle]
pub unsafe extern "C" fn FSE_decompress(mut dst: *mut libc::c_void,
                                        mut dstCapacity: size_t,
                                        mut cSrc: *const libc::c_void,
                                        mut cSrcSize: size_t) -> size_t {
    let mut dt: DTable_max_t = [0; 4097];
    return FSE_decompress_wksp(dst, dstCapacity, cSrc, cSrcSize,
                               dt.as_mut_ptr(),
                               (14i32 - 2i32) as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn FSE_decompress_wksp(mut dst: *mut libc::c_void,
                                             mut dstCapacity: size_t,
                                             mut cSrc: *const libc::c_void,
                                             mut cSrcSize: size_t,
                                             mut workSpace: *mut FSE_DTable,
                                             mut maxLog: libc::c_uint)
 -> size_t {
    let istart: *const BYTE = cSrc as *const BYTE;
    let mut ip: *const BYTE = istart;
    let mut counting: [libc::c_short; 256] = [0; 256];
    let mut tableLog: libc::c_uint = 0;
    let mut maxSymbolValue: libc::c_uint = 255i32 as libc::c_uint;
    let NCountLength: size_t =
        FSE_readNCount(counting.as_mut_ptr(),
                       &mut maxSymbolValue as *mut libc::c_uint,
                       &mut tableLog as *mut libc::c_uint,
                       istart as *const libc::c_void, cSrcSize);
    if 0 != ERR_isError(NCountLength) {
        return NCountLength
    } else if tableLog > maxLog {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    } else {
        ip = ip.offset(NCountLength as isize);
        cSrcSize =
            (cSrcSize as libc::c_ulong).wrapping_sub(NCountLength) as size_t
                as size_t;
        let e: size_t =
            FSE_buildDTable(workSpace, counting.as_mut_ptr(), maxSymbolValue,
                            tableLog);
        if 0 != ERR_isError(e) {
            return e
        } else {
            return FSE_decompress_usingDTable(dst, dstCapacity,
                                              ip as *const libc::c_void,
                                              cSrcSize, workSpace)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn FSE_decompress_usingDTable(mut dst:
                                                        *mut libc::c_void,
                                                    mut originalSize: size_t,
                                                    mut cSrc:
                                                        *const libc::c_void,
                                                    mut cSrcSize: size_t,
                                                    mut dt: *const FSE_DTable)
 -> size_t {
    let mut ptr: *const libc::c_void = dt as *const libc::c_void;
    let mut DTableH: *const FSE_DTableHeader = ptr as *const FSE_DTableHeader;
    let fastMode: U32 = (*DTableH).fastMode as U32;
    if 0 != fastMode {
        return FSE_decompress_usingDTable_generic(dst, originalSize, cSrc,
                                                  cSrcSize, dt,
                                                  1i32 as libc::c_uint)
    } else {
        return FSE_decompress_usingDTable_generic(dst, originalSize, cSrc,
                                                  cSrcSize, dt,
                                                  0i32 as libc::c_uint)
    };
}
unsafe extern "C" fn FSE_decompress_usingDTable_generic(mut dst:
                                                            *mut libc::c_void,
                                                        mut maxDstSize:
                                                            size_t,
                                                        mut cSrc:
                                                            *const libc::c_void,
                                                        mut cSrcSize: size_t,
                                                        mut dt:
                                                            *const FSE_DTable,
                                                        fast: libc::c_uint)
 -> size_t {
    let ostart: *mut BYTE = dst as *mut BYTE;
    let mut op: *mut BYTE = ostart;
    let omax: *mut BYTE = op.offset(maxDstSize as isize);
    let olimit: *mut BYTE = omax.offset(-3isize);
    let mut bitD: BIT_DStream_t =
        BIT_DStream_t{bitContainer: 0,
                      bitsConsumed: 0,
                      ptr: 0 as *const libc::c_char,
                      start: 0 as *const libc::c_char,
                      limitPtr: 0 as *const libc::c_char,};
    let mut state1: FSE_DState_t =
        FSE_DState_t{state: 0, table: 0 as *const libc::c_void,};
    let mut state2: FSE_DState_t =
        FSE_DState_t{state: 0, table: 0 as *const libc::c_void,};
    let e: size_t =
        BIT_initDStream(&mut bitD as *mut BIT_DStream_t, cSrc, cSrcSize);
    if 0 != ERR_isError(e) {
        return e
    } else {
        FSE_initDState(&mut state1 as *mut FSE_DState_t,
                       &mut bitD as *mut BIT_DStream_t, dt);
        FSE_initDState(&mut state2 as *mut FSE_DState_t,
                       &mut bitD as *mut BIT_DStream_t, dt);
        while 0 !=
                  (BIT_reloadDStream(&mut bitD as *mut BIT_DStream_t) as
                       libc::c_uint ==
                       BIT_DStream_unfinished as libc::c_int as libc::c_uint)
                      as libc::c_int & (op < olimit) as libc::c_int {
            *op.offset(0isize) =
                (if 0 != fast {
                     FSE_decodeSymbolFast(&mut state1 as *mut FSE_DState_t,
                                          &mut bitD as *mut BIT_DStream_t) as
                         libc::c_int
                 } else {
                     FSE_decodeSymbol(&mut state1 as *mut FSE_DState_t,
                                      &mut bitD as *mut BIT_DStream_t) as
                         libc::c_int
                 }) as BYTE;
            if ((14i32 - 2i32) * 2i32 + 7i32) as libc::c_ulong >
                   (::std::mem::size_of::<size_t>() as
                        libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
                BIT_reloadDStream(&mut bitD as *mut BIT_DStream_t);
            }
            *op.offset(1isize) =
                (if 0 != fast {
                     FSE_decodeSymbolFast(&mut state2 as *mut FSE_DState_t,
                                          &mut bitD as *mut BIT_DStream_t) as
                         libc::c_int
                 } else {
                     FSE_decodeSymbol(&mut state2 as *mut FSE_DState_t,
                                      &mut bitD as *mut BIT_DStream_t) as
                         libc::c_int
                 }) as BYTE;
            if ((14i32 - 2i32) * 4i32 + 7i32) as libc::c_ulong >
                   (::std::mem::size_of::<size_t>() as
                        libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
                if BIT_reloadDStream(&mut bitD as *mut BIT_DStream_t) as
                       libc::c_uint >
                       BIT_DStream_unfinished as libc::c_int as libc::c_uint {
                    op = op.offset(2isize);
                    break ;
                }
            }
            *op.offset(2isize) =
                (if 0 != fast {
                     FSE_decodeSymbolFast(&mut state1 as *mut FSE_DState_t,
                                          &mut bitD as *mut BIT_DStream_t) as
                         libc::c_int
                 } else {
                     FSE_decodeSymbol(&mut state1 as *mut FSE_DState_t,
                                      &mut bitD as *mut BIT_DStream_t) as
                         libc::c_int
                 }) as BYTE;
            if ((14i32 - 2i32) * 2i32 + 7i32) as libc::c_ulong >
                   (::std::mem::size_of::<size_t>() as
                        libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
                BIT_reloadDStream(&mut bitD as *mut BIT_DStream_t);
            }
            *op.offset(3isize) =
                (if 0 != fast {
                     FSE_decodeSymbolFast(&mut state2 as *mut FSE_DState_t,
                                          &mut bitD as *mut BIT_DStream_t) as
                         libc::c_int
                 } else {
                     FSE_decodeSymbol(&mut state2 as *mut FSE_DState_t,
                                      &mut bitD as *mut BIT_DStream_t) as
                         libc::c_int
                 }) as BYTE;
            op = op.offset(4isize)
        }
        loop  {
            if op > omax.offset(-2isize) {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
            } else {
                let fresh0 = op;
                op = op.offset(1);
                *fresh0 =
                    (if 0 != fast {
                         FSE_decodeSymbolFast(&mut state1 as
                                                  *mut FSE_DState_t,
                                              &mut bitD as *mut BIT_DStream_t)
                             as libc::c_int
                     } else {
                         FSE_decodeSymbol(&mut state1 as *mut FSE_DState_t,
                                          &mut bitD as *mut BIT_DStream_t) as
                             libc::c_int
                     }) as BYTE;
                if BIT_reloadDStream(&mut bitD as *mut BIT_DStream_t) as
                       libc::c_uint ==
                       BIT_DStream_overflow as libc::c_int as libc::c_uint {
                    let fresh1 = op;
                    op = op.offset(1);
                    *fresh1 =
                        (if 0 != fast {
                             FSE_decodeSymbolFast(&mut state2 as
                                                      *mut FSE_DState_t,
                                                  &mut bitD as
                                                      *mut BIT_DStream_t) as
                                 libc::c_int
                         } else {
                             FSE_decodeSymbol(&mut state2 as
                                                  *mut FSE_DState_t,
                                              &mut bitD as *mut BIT_DStream_t)
                                 as libc::c_int
                         }) as BYTE;
                    break ;
                } else if op > omax.offset(-2isize) {
                    return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as
                               size_t
                } else {
                    let fresh2 = op;
                    op = op.offset(1);
                    *fresh2 =
                        (if 0 != fast {
                             FSE_decodeSymbolFast(&mut state2 as
                                                      *mut FSE_DState_t,
                                                  &mut bitD as
                                                      *mut BIT_DStream_t) as
                                 libc::c_int
                         } else {
                             FSE_decodeSymbol(&mut state2 as
                                                  *mut FSE_DState_t,
                                              &mut bitD as *mut BIT_DStream_t)
                                 as libc::c_int
                         }) as BYTE;
                    if !(BIT_reloadDStream(&mut bitD as *mut BIT_DStream_t) as
                             libc::c_uint ==
                             BIT_DStream_overflow as libc::c_int as
                                 libc::c_uint) {
                        continue ;
                    }
                    let fresh3 = op;
                    op = op.offset(1);
                    *fresh3 =
                        (if 0 != fast {
                             FSE_decodeSymbolFast(&mut state1 as
                                                      *mut FSE_DState_t,
                                                  &mut bitD as
                                                      *mut BIT_DStream_t) as
                                 libc::c_int
                         } else {
                             FSE_decodeSymbol(&mut state1 as
                                                  *mut FSE_DState_t,
                                              &mut bitD as *mut BIT_DStream_t)
                                 as libc::c_int
                         }) as BYTE;
                    break ;
                }
            }
        }
        return ostart.offset_to(op).expect("bad offset_to") as libc::c_long as
                   size_t
    };
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
#[no_mangle]
pub unsafe extern "C" fn FSE_buildDTable(mut dt: *mut FSE_DTable,
                                         mut normalizedCounter:
                                             *const libc::c_short,
                                         mut maxSymbolValue: libc::c_uint,
                                         mut tableLog: libc::c_uint)
 -> size_t {
    let tdPtr: *mut libc::c_void = dt.offset(1isize) as *mut libc::c_void;
    let tableDecode: *mut FSE_decode_t = tdPtr as *mut FSE_decode_t;
    let mut symbolNext: [U16; 256] = [0; 256];
    let maxSV1: U32 = maxSymbolValue.wrapping_add(1i32 as libc::c_uint);
    let tableSize: U32 = (1i32 << tableLog) as U32;
    let mut highThreshold: U32 = tableSize.wrapping_sub(1i32 as libc::c_uint);
    if maxSymbolValue > 255i32 as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as size_t
    } else if tableLog > (14i32 - 2i32) as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    } else {
        let mut DTableH: FSE_DTableHeader =
            FSE_DTableHeader{tableLog: 0, fastMode: 0,};
        DTableH.tableLog = tableLog as U16;
        DTableH.fastMode = 1i32 as U16;
        let largeLimit: S16 =
            (1i32 << tableLog.wrapping_sub(1i32 as libc::c_uint)) as S16;
        let mut s: U32 = 0;
        s = 0i32 as U32;
        while s < maxSV1 {
            if *normalizedCounter.offset(s as isize) as libc::c_int == -1i32 {
                let fresh4 = highThreshold;
                highThreshold = highThreshold.wrapping_sub(1);
                (*tableDecode.offset(fresh4 as isize)).symbol = s as BYTE;
                symbolNext[s as usize] = 1i32 as U16
            } else {
                if *normalizedCounter.offset(s as isize) as libc::c_int >=
                       largeLimit as libc::c_int {
                    DTableH.fastMode = 0i32 as U16
                }
                symbolNext[s as usize] =
                    *normalizedCounter.offset(s as isize) as U16
            }
            s = s.wrapping_add(1)
        }
        memcpy(dt as *mut libc::c_void,
               &mut DTableH as *mut FSE_DTableHeader as *const libc::c_void,
               ::std::mem::size_of::<FSE_DTableHeader>() as libc::c_ulong);
        let tableMask: U32 = tableSize.wrapping_sub(1i32 as libc::c_uint);
        let step: U32 =
            (tableSize >>
                 1i32).wrapping_add(tableSize >>
                                        3i32).wrapping_add(3i32 as
                                                               libc::c_uint);
        let mut s_0: U32 = 0;
        let mut position: U32 = 0i32 as U32;
        s_0 = 0i32 as U32;
        while s_0 < maxSV1 {
            let mut i: libc::c_int = 0;
            i = 0i32;
            while i < *normalizedCounter.offset(s_0 as isize) as libc::c_int {
                (*tableDecode.offset(position as isize)).symbol = s_0 as BYTE;
                position = position.wrapping_add(step) & tableMask;
                while position > highThreshold {
                    position = position.wrapping_add(step) & tableMask
                }
                i += 1
            }
            s_0 = s_0.wrapping_add(1)
        }
        if position != 0i32 as libc::c_uint {
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t
        } else {
            let mut u: U32 = 0;
            u = 0i32 as U32;
            while u < tableSize {
                let symbol: BYTE = (*tableDecode.offset(u as isize)).symbol;
                let fresh5 = symbolNext[symbol as usize];
                symbolNext[symbol as usize] =
                    symbolNext[symbol as usize].wrapping_add(1);
                let nextState: U32 = fresh5 as U32;
                (*tableDecode.offset(u as isize)).nbBits =
                    tableLog.wrapping_sub(BIT_highbit32(nextState)) as BYTE;
                (*tableDecode.offset(u as isize)).newState =
                    (nextState <<
                         (*tableDecode.offset(u as isize)).nbBits as
                             libc::c_int).wrapping_sub(tableSize) as U16;
                u = u.wrapping_add(1)
            }
            return 0i32 as size_t
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn FSE_createDTable(mut tableLog: libc::c_uint)
 -> *mut FSE_DTable {
    if tableLog > 15i32 as libc::c_uint { tableLog = 15i32 as libc::c_uint }
    return malloc(((1i32 + (1i32 << tableLog)) as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<U32>()
                                                       as libc::c_ulong)) as
               *mut FSE_DTable;
}
#[no_mangle]
pub unsafe extern "C" fn FSE_freeDTable(mut dt: *mut FSE_DTable) -> () {
    free(dt as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn FSE_buildDTable_raw(mut dt: *mut FSE_DTable,
                                             mut nbBits: libc::c_uint)
 -> size_t {
    let mut ptr: *mut libc::c_void = dt as *mut libc::c_void;
    let DTableH: *mut FSE_DTableHeader = ptr as *mut FSE_DTableHeader;
    let mut dPtr: *mut libc::c_void = dt.offset(1isize) as *mut libc::c_void;
    let dinfo: *mut FSE_decode_t = dPtr as *mut FSE_decode_t;
    let tableSize: libc::c_uint = (1i32 << nbBits) as libc::c_uint;
    let tableMask: libc::c_uint =
        tableSize.wrapping_sub(1i32 as libc::c_uint);
    let maxSV1: libc::c_uint = tableMask.wrapping_add(1i32 as libc::c_uint);
    let mut s: libc::c_uint = 0;
    if nbBits < 1i32 as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        (*DTableH).tableLog = nbBits as U16;
        (*DTableH).fastMode = 1i32 as U16;
        s = 0i32 as libc::c_uint;
        while s < maxSV1 {
            (*dinfo.offset(s as isize)).newState = 0i32 as libc::c_ushort;
            (*dinfo.offset(s as isize)).symbol = s as BYTE;
            (*dinfo.offset(s as isize)).nbBits = nbBits as BYTE;
            s = s.wrapping_add(1)
        }
        return 0i32 as size_t
    };
}
/* *< build a fake FSE_DTable, designed to read a flat distribution where each symbol uses nbBits */
#[no_mangle]
pub unsafe extern "C" fn FSE_buildDTable_rle(mut dt: *mut FSE_DTable,
                                             mut symbolValue: BYTE)
 -> size_t {
    let mut ptr: *mut libc::c_void = dt as *mut libc::c_void;
    let DTableH: *mut FSE_DTableHeader = ptr as *mut FSE_DTableHeader;
    let mut dPtr: *mut libc::c_void = dt.offset(1isize) as *mut libc::c_void;
    let cell: *mut FSE_decode_t = dPtr as *mut FSE_decode_t;
    (*DTableH).tableLog = 0i32 as U16;
    (*DTableH).fastMode = 0i32 as U16;
    (*cell).newState = 0i32 as libc::c_ushort;
    (*cell).symbol = symbolValue;
    (*cell).nbBits = 0i32 as libc::c_uchar;
    return 0i32 as size_t;
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
unsafe extern "C" fn FSE_endOfDState(mut DStatePtr: *const FSE_DState_t)
 -> libc::c_uint {
    return ((*DStatePtr).state == 0i32 as libc::c_ulong) as libc::c_int as
               libc::c_uint;
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
