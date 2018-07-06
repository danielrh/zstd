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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
    /* * HUF_decompress() :
 *  Decompress HUF data from buffer 'cSrc', of size 'cSrcSize',
 *  into already allocated buffer 'dst', of minimum size 'dstSize'.
 * `originalSize` : **must** be the ***exact*** size of original (uncompressed) data.
 *  Note : in contrast with FSE, HUF_decompress can regenerate
 *         RLE (cSrcSize==1) and uncompressed (cSrcSize==dstSize) data,
 *         because it knows size to regenerate (originalSize).
 * @return : size of regenerated data (== originalSize),
 *           or an error code, which can be tested using HUF_isError()
 */
    /* * HUF_selectDecoder() :
 *  Tells which decoder is likely to decode faster,
 *  based on a set of pre-computed metrics.
 * @return : 0==HUF_decompress4X1, 1==HUF_decompress4X2 .
 *  Assumption : 0 < dstSize <= 128 KB */
    /* *< single-symbol decoder */
    /* *< single-symbol decoder */
    /* *< double-symbols decoder */
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
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
pub type HUF_DTable = U32;
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct HUF_DEltX1 {
    pub byte: BYTE,
    pub nbBits: BYTE,
}
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
pub const MEM_static_assert: unnamed_0 = 1;
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
pub type uint32_t = libc::c_uint;
pub type unnamed = libc::c_uint;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub type BYTE = uint8_t;
pub type U16 = uint16_t;
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_CStream_t {
    pub bitContainer: size_t,
    pub bitPos: libc::c_uint,
    pub startPtr: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub endPtr: *mut libc::c_char,
}
pub type BIT_DStream_status = libc::c_uint;
pub type U64 = uint64_t;
pub type uint64_t = libc::c_ulong;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub type decompressionAlgo =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t,
                                _: *const libc::c_void, _: size_t) -> size_t>;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
pub type rankValCol_t = [U32; 13];
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
pub type uint16_t = libc::c_ushort;
pub type ZSTD_ErrorCode = libc::c_uint;
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct DTableDesc {
    pub maxTableLog: BYTE,
    pub tableType: BYTE,
    pub tableLog: BYTE,
    pub reserved: BYTE,
}
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
pub type unnamed_0 = libc::c_uint;
pub type size_t = libc::c_ulong;
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct HUF_DEltX2 {
    pub sequence: U16,
    pub nbBits: BYTE,
    pub length: BYTE,
}
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
pub type U32 = uint32_t;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub type ERR_enum = ZSTD_ErrorCode;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    u: U32,
    c: [BYTE; 4],
}
pub const MEM_static_assert_0: unnamed = 1;
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct algo_time_t {
    pub tableTime: U32,
    pub decode256Time: U32,
}
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub type uint8_t = libc::c_uchar;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
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
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sortedSymbol_t {
    pub symbol: BYTE,
    pub weight: BYTE,
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
                    current_block = 8269516112928124952;
                }
                6 => { current_block = 8269516112928124952; }
                5 => { current_block = 7660420706351058135; }
                4 => { current_block = 16948057605998975775; }
                3 => { current_block = 11957493614642545515; }
                2 => { current_block = 4571405419322263919; }
                _ => { current_block = 10886091980245723256; }
            }
            match current_block {
                8269516112928124952 => {
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
                    current_block = 7660420706351058135;
                }
                _ => { }
            }
            match current_block {
                7660420706351058135 => {
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
                    current_block = 16948057605998975775;
                }
                _ => { }
            }
            match current_block {
                16948057605998975775 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(3isize)
                                                              as size_t) <<
                                                             24i32) as size_t
                            as size_t;
                    current_block = 11957493614642545515;
                }
                _ => { }
            }
            match current_block {
                11957493614642545515 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(2isize)
                                                              as size_t) <<
                                                             16i32) as size_t
                            as size_t;
                    current_block = 4571405419322263919;
                }
                _ => { }
            }
            match current_block {
                4571405419322263919 => {
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
pub unsafe extern "C" fn HUF_decompress(mut dst: *mut libc::c_void,
                                        mut dstSize: size_t,
                                        mut cSrc: *const libc::c_void,
                                        mut cSrcSize: size_t) -> size_t {
    static mut decompress: [decompressionAlgo; 2] =
        unsafe { [Some(HUF_decompress4X1), Some(HUF_decompress4X2)] };
    if dstSize == 0i32 as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else if cSrcSize > dstSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    } else if cSrcSize == dstSize {
        memcpy(dst, cSrc, dstSize);
        return dstSize
    } else if cSrcSize == 1i32 as libc::c_ulong {
        memset(dst, *(cSrc as *const BYTE) as libc::c_int, dstSize);
        return dstSize
    } else {
        let algoNb: U32 = HUF_selectDecoder(dstSize, cSrcSize);
        return decompress[algoNb as
                              usize].expect("non-null function pointer")(dst,
                                                                         dstSize,
                                                                         cSrc,
                                                                         cSrcSize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUF_selectDecoder(mut dstSize: size_t,
                                           mut cSrcSize: size_t) -> U32 {
    let Q: U32 =
        if cSrcSize >= dstSize {
            15i32 as libc::c_uint
        } else {
            cSrcSize.wrapping_mul(16i32 as
                                      libc::c_ulong).wrapping_div(dstSize) as
                U32
        };
    let D256: U32 = (dstSize >> 8i32) as U32;
    let DTime0: U32 =
        algoTime[Q as
                     usize][0usize].tableTime.wrapping_add(algoTime[Q as
                                                                        usize][0usize].decode256Time.wrapping_mul(D256));
    let mut DTime1: U32 =
        algoTime[Q as
                     usize][1usize].tableTime.wrapping_add(algoTime[Q as
                                                                        usize][1usize].decode256Time.wrapping_mul(D256));
    DTime1 =
        (DTime1 as libc::c_uint).wrapping_add(DTime1 >> 3i32) as U32 as U32;
    return (DTime1 < DTime0) as libc::c_int as U32;
}
static mut algoTime: [[algo_time_t; 3]; 16] =
    unsafe {
        [[algo_time_t{tableTime: 0i32 as U32, decode256Time: 0i32 as U32,},
          algo_time_t{tableTime: 1i32 as U32, decode256Time: 1i32 as U32,},
          algo_time_t{tableTime: 2i32 as U32, decode256Time: 2i32 as U32,}],
         [algo_time_t{tableTime: 0i32 as U32, decode256Time: 0i32 as U32,},
          algo_time_t{tableTime: 1i32 as U32, decode256Time: 1i32 as U32,},
          algo_time_t{tableTime: 2i32 as U32, decode256Time: 2i32 as U32,}],
         [algo_time_t{tableTime: 38i32 as U32, decode256Time: 130i32 as U32,},
          algo_time_t{tableTime: 1313i32 as U32,
                      decode256Time: 74i32 as U32,},
          algo_time_t{tableTime: 2151i32 as U32,
                      decode256Time: 38i32 as U32,}],
         [algo_time_t{tableTime: 448i32 as U32,
                      decode256Time: 128i32 as U32,},
          algo_time_t{tableTime: 1353i32 as U32,
                      decode256Time: 74i32 as U32,},
          algo_time_t{tableTime: 2238i32 as U32,
                      decode256Time: 41i32 as U32,}],
         [algo_time_t{tableTime: 556i32 as U32,
                      decode256Time: 128i32 as U32,},
          algo_time_t{tableTime: 1353i32 as U32,
                      decode256Time: 74i32 as U32,},
          algo_time_t{tableTime: 2238i32 as U32,
                      decode256Time: 47i32 as U32,}],
         [algo_time_t{tableTime: 714i32 as U32,
                      decode256Time: 128i32 as U32,},
          algo_time_t{tableTime: 1418i32 as U32,
                      decode256Time: 74i32 as U32,},
          algo_time_t{tableTime: 2436i32 as U32,
                      decode256Time: 53i32 as U32,}],
         [algo_time_t{tableTime: 883i32 as U32,
                      decode256Time: 128i32 as U32,},
          algo_time_t{tableTime: 1437i32 as U32,
                      decode256Time: 74i32 as U32,},
          algo_time_t{tableTime: 2464i32 as U32,
                      decode256Time: 61i32 as U32,}],
         [algo_time_t{tableTime: 897i32 as U32,
                      decode256Time: 128i32 as U32,},
          algo_time_t{tableTime: 1515i32 as U32,
                      decode256Time: 75i32 as U32,},
          algo_time_t{tableTime: 2622i32 as U32,
                      decode256Time: 68i32 as U32,}],
         [algo_time_t{tableTime: 926i32 as U32,
                      decode256Time: 128i32 as U32,},
          algo_time_t{tableTime: 1613i32 as U32,
                      decode256Time: 75i32 as U32,},
          algo_time_t{tableTime: 2730i32 as U32,
                      decode256Time: 75i32 as U32,}],
         [algo_time_t{tableTime: 947i32 as U32,
                      decode256Time: 128i32 as U32,},
          algo_time_t{tableTime: 1729i32 as U32,
                      decode256Time: 77i32 as U32,},
          algo_time_t{tableTime: 3359i32 as U32,
                      decode256Time: 77i32 as U32,}],
         [algo_time_t{tableTime: 1107i32 as U32,
                      decode256Time: 128i32 as U32,},
          algo_time_t{tableTime: 2083i32 as U32,
                      decode256Time: 81i32 as U32,},
          algo_time_t{tableTime: 4006i32 as U32,
                      decode256Time: 84i32 as U32,}],
         [algo_time_t{tableTime: 1177i32 as U32,
                      decode256Time: 128i32 as U32,},
          algo_time_t{tableTime: 2379i32 as U32,
                      decode256Time: 87i32 as U32,},
          algo_time_t{tableTime: 4785i32 as U32,
                      decode256Time: 88i32 as U32,}],
         [algo_time_t{tableTime: 1242i32 as U32,
                      decode256Time: 128i32 as U32,},
          algo_time_t{tableTime: 2415i32 as U32,
                      decode256Time: 93i32 as U32,},
          algo_time_t{tableTime: 5155i32 as U32,
                      decode256Time: 84i32 as U32,}],
         [algo_time_t{tableTime: 1349i32 as U32,
                      decode256Time: 128i32 as U32,},
          algo_time_t{tableTime: 2644i32 as U32,
                      decode256Time: 106i32 as U32,},
          algo_time_t{tableTime: 5260i32 as U32,
                      decode256Time: 106i32 as U32,}],
         [algo_time_t{tableTime: 1455i32 as U32,
                      decode256Time: 128i32 as U32,},
          algo_time_t{tableTime: 2422i32 as U32,
                      decode256Time: 124i32 as U32,},
          algo_time_t{tableTime: 4174i32 as U32,
                      decode256Time: 124i32 as U32,}],
         [algo_time_t{tableTime: 722i32 as U32,
                      decode256Time: 128i32 as U32,},
          algo_time_t{tableTime: 1891i32 as U32,
                      decode256Time: 145i32 as U32,},
          algo_time_t{tableTime: 1936i32 as U32,
                      decode256Time: 146i32 as U32,}]]
    };
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X2(mut dst: *mut libc::c_void,
                                           mut dstSize: size_t,
                                           mut cSrc: *const libc::c_void,
                                           mut cSrcSize: size_t) -> size_t {
    let mut DTable: [HUF_DTable; 4097] =
        [(12i32 as U32).wrapping_mul(16777217i32 as libc::c_uint), 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    return HUF_decompress4X2_DCtx(DTable.as_mut_ptr(), dst, dstSize, cSrc,
                                  cSrcSize);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X2_DCtx(mut dctx: *mut HUF_DTable,
                                                mut dst: *mut libc::c_void,
                                                mut dstSize: size_t,
                                                mut cSrc: *const libc::c_void,
                                                mut cSrcSize: size_t)
 -> size_t {
    let mut workSpace: [U32; 512] = [0; 512];
    return HUF_decompress4X2_DCtx_wksp(dctx, dst, dstSize, cSrc, cSrcSize,
                                       workSpace.as_mut_ptr() as
                                           *mut libc::c_void,
                                       ::std::mem::size_of::<[U32; 512]>() as
                                           libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X2_DCtx_wksp(mut dctx:
                                                         *mut HUF_DTable,
                                                     mut dst:
                                                         *mut libc::c_void,
                                                     mut dstSize: size_t,
                                                     mut cSrc:
                                                         *const libc::c_void,
                                                     mut cSrcSize: size_t,
                                                     mut workSpace:
                                                         *mut libc::c_void,
                                                     mut wkspSize: size_t)
 -> size_t {
    return HUF_decompress4X2_DCtx_wksp_bmi2(dctx, dst, dstSize, cSrc,
                                            cSrcSize, workSpace, wkspSize,
                                            0i32);
}
unsafe extern "C" fn HUF_decompress4X2_DCtx_wksp_bmi2(mut dctx:
                                                          *mut HUF_DTable,
                                                      mut dst:
                                                          *mut libc::c_void,
                                                      mut dstSize: size_t,
                                                      mut cSrc:
                                                          *const libc::c_void,
                                                      mut cSrcSize: size_t,
                                                      mut workSpace:
                                                          *mut libc::c_void,
                                                      mut wkspSize: size_t,
                                                      mut bmi2: libc::c_int)
 -> size_t {
    let mut ip: *const BYTE = cSrc as *const BYTE;
    let mut hSize: size_t =
        HUF_readDTableX2_wksp(dctx, cSrc, cSrcSize, workSpace, wkspSize);
    if 0 != ERR_isError(hSize) {
        return hSize
    } else if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else {
        ip = ip.offset(hSize as isize);
        cSrcSize =
            (cSrcSize as libc::c_ulong).wrapping_sub(hSize) as size_t as
                size_t;
        return HUF_decompress4X2_usingDTable_internal(dst, dstSize,
                                                      ip as
                                                          *const libc::c_void,
                                                      cSrcSize, dctx, bmi2)
    };
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal(mut dst:
                                                                *mut libc::c_void,
                                                            mut dstSize:
                                                                size_t,
                                                            mut cSrc:
                                                                *const libc::c_void,
                                                            mut cSrcSize:
                                                                size_t,
                                                            mut DTable:
                                                                *const HUF_DTable,
                                                            mut bmi2:
                                                                libc::c_int)
 -> size_t {
    if 0 != bmi2 {
        return HUF_decompress4X2_usingDTable_internal_bmi2(dst, dstSize, cSrc,
                                                           cSrcSize, DTable)
    } else {
        return HUF_decompress4X2_usingDTable_internal_default(dst, dstSize,
                                                              cSrc, cSrcSize,
                                                              DTable)
    };
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_default(mut dst:
                                                                        *mut libc::c_void,
                                                                    mut dstSize:
                                                                        size_t,
                                                                    mut cSrc:
                                                                        *const libc::c_void,
                                                                    mut cSrcSize:
                                                                        size_t,
                                                                    mut DTable:
                                                                        *const HUF_DTable)
 -> size_t {
    return HUF_decompress4X2_usingDTable_internal_body(dst, dstSize, cSrc,
                                                       cSrcSize, DTable);
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_body(mut dst:
                                                                     *mut libc::c_void,
                                                                 mut dstSize:
                                                                     size_t,
                                                                 mut cSrc:
                                                                     *const libc::c_void,
                                                                 mut cSrcSize:
                                                                     size_t,
                                                                 mut DTable:
                                                                     *const HUF_DTable)
 -> size_t {
    if cSrcSize < 10i32 as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    } else {
        let istart: *const BYTE = cSrc as *const BYTE;
        let ostart: *mut BYTE = dst as *mut BYTE;
        let oend: *mut BYTE = ostart.offset(dstSize as isize);
        let dtPtr: *const libc::c_void =
            DTable.offset(1isize) as *const libc::c_void;
        let dt: *const HUF_DEltX2 = dtPtr as *const HUF_DEltX2;
        let mut bitD1: BIT_DStream_t =
            BIT_DStream_t{bitContainer: 0,
                          bitsConsumed: 0,
                          ptr: 0 as *const libc::c_char,
                          start: 0 as *const libc::c_char,
                          limitPtr: 0 as *const libc::c_char,};
        let mut bitD2: BIT_DStream_t =
            BIT_DStream_t{bitContainer: 0,
                          bitsConsumed: 0,
                          ptr: 0 as *const libc::c_char,
                          start: 0 as *const libc::c_char,
                          limitPtr: 0 as *const libc::c_char,};
        let mut bitD3: BIT_DStream_t =
            BIT_DStream_t{bitContainer: 0,
                          bitsConsumed: 0,
                          ptr: 0 as *const libc::c_char,
                          start: 0 as *const libc::c_char,
                          limitPtr: 0 as *const libc::c_char,};
        let mut bitD4: BIT_DStream_t =
            BIT_DStream_t{bitContainer: 0,
                          bitsConsumed: 0,
                          ptr: 0 as *const libc::c_char,
                          start: 0 as *const libc::c_char,
                          limitPtr: 0 as *const libc::c_char,};
        let length1: size_t =
            MEM_readLE16(istart as *const libc::c_void) as size_t;
        let length2: size_t =
            MEM_readLE16(istart.offset(2isize) as *const libc::c_void) as
                size_t;
        let length3: size_t =
            MEM_readLE16(istart.offset(4isize) as *const libc::c_void) as
                size_t;
        let length4: size_t =
            cSrcSize.wrapping_sub(length1.wrapping_add(length2).wrapping_add(length3).wrapping_add(6i32
                                                                                                       as
                                                                                                       libc::c_ulong));
        let istart1: *const BYTE = istart.offset(6isize);
        let istart2: *const BYTE = istart1.offset(length1 as isize);
        let istart3: *const BYTE = istart2.offset(length2 as isize);
        let istart4: *const BYTE = istart3.offset(length3 as isize);
        let segmentSize: size_t =
            dstSize.wrapping_add(3i32 as
                                     libc::c_ulong).wrapping_div(4i32 as
                                                                     libc::c_ulong);
        let opStart2: *mut BYTE = ostart.offset(segmentSize as isize);
        let opStart3: *mut BYTE = opStart2.offset(segmentSize as isize);
        let opStart4: *mut BYTE = opStart3.offset(segmentSize as isize);
        let mut op1: *mut BYTE = ostart;
        let mut op2: *mut BYTE = opStart2;
        let mut op3: *mut BYTE = opStart3;
        let mut op4: *mut BYTE = opStart4;
        let mut endSignal: U32 = 0;
        let dtd: DTableDesc = HUF_getDTableDesc(DTable);
        let dtLog: U32 = dtd.tableLog as U32;
        if length4 > cSrcSize {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        } else {
            let err_: size_t =
                BIT_initDStream(&mut bitD1 as *mut BIT_DStream_t,
                                istart1 as *const libc::c_void, length1);
            if 0 != ERR_isError(err_) {
                return err_
            } else {
                let err__0: size_t =
                    BIT_initDStream(&mut bitD2 as *mut BIT_DStream_t,
                                    istart2 as *const libc::c_void, length2);
                if 0 != ERR_isError(err__0) {
                    return err__0
                } else {
                    let err__1: size_t =
                        BIT_initDStream(&mut bitD3 as *mut BIT_DStream_t,
                                        istart3 as *const libc::c_void,
                                        length3);
                    if 0 != ERR_isError(err__1) {
                        return err__1
                    } else {
                        let err__2: size_t =
                            BIT_initDStream(&mut bitD4 as *mut BIT_DStream_t,
                                            istart4 as *const libc::c_void,
                                            length4);
                        if 0 != ERR_isError(err__2) {
                            return err__2
                        } else {
                            endSignal =
                                BIT_reloadDStream(&mut bitD1 as
                                                      *mut BIT_DStream_t) as
                                    libc::c_uint |
                                    BIT_reloadDStream(&mut bitD2 as
                                                          *mut BIT_DStream_t)
                                        as libc::c_uint |
                                    BIT_reloadDStream(&mut bitD3 as
                                                          *mut BIT_DStream_t)
                                        as libc::c_uint |
                                    BIT_reloadDStream(&mut bitD4 as
                                                          *mut BIT_DStream_t)
                                        as libc::c_uint;
                            while 0 !=
                                      (endSignal ==
                                           BIT_DStream_unfinished as
                                               libc::c_int as libc::c_uint) as
                                          libc::c_int &
                                          (op4 <
                                               oend.offset(-((::std::mem::size_of::<size_t>()
                                                                  as
                                                                  libc::c_ulong).wrapping_sub(1i32
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                                 as isize)))
                                              as libc::c_int {
                                if 0 != MEM_64bits() {
                                    op1 =
                                        op1.offset(HUF_decodeSymbolX2(op1 as
                                                                          *mut libc::c_void,
                                                                      &mut bitD1
                                                                          as
                                                                          *mut BIT_DStream_t,
                                                                      dt,
                                                                      dtLog)
                                                       as isize)
                                }
                                if 0 != MEM_64bits() {
                                    op2 =
                                        op2.offset(HUF_decodeSymbolX2(op2 as
                                                                          *mut libc::c_void,
                                                                      &mut bitD2
                                                                          as
                                                                          *mut BIT_DStream_t,
                                                                      dt,
                                                                      dtLog)
                                                       as isize)
                                }
                                if 0 != MEM_64bits() {
                                    op3 =
                                        op3.offset(HUF_decodeSymbolX2(op3 as
                                                                          *mut libc::c_void,
                                                                      &mut bitD3
                                                                          as
                                                                          *mut BIT_DStream_t,
                                                                      dt,
                                                                      dtLog)
                                                       as isize)
                                }
                                if 0 != MEM_64bits() {
                                    op4 =
                                        op4.offset(HUF_decodeSymbolX2(op4 as
                                                                          *mut libc::c_void,
                                                                      &mut bitD4
                                                                          as
                                                                          *mut BIT_DStream_t,
                                                                      dt,
                                                                      dtLog)
                                                       as isize)
                                }
                                if 0 != MEM_64bits() || 12i32 <= 12i32 {
                                    op1 =
                                        op1.offset(HUF_decodeSymbolX2(op1 as
                                                                          *mut libc::c_void,
                                                                      &mut bitD1
                                                                          as
                                                                          *mut BIT_DStream_t,
                                                                      dt,
                                                                      dtLog)
                                                       as isize)
                                }
                                if 0 != MEM_64bits() || 12i32 <= 12i32 {
                                    op2 =
                                        op2.offset(HUF_decodeSymbolX2(op2 as
                                                                          *mut libc::c_void,
                                                                      &mut bitD2
                                                                          as
                                                                          *mut BIT_DStream_t,
                                                                      dt,
                                                                      dtLog)
                                                       as isize)
                                }
                                if 0 != MEM_64bits() || 12i32 <= 12i32 {
                                    op3 =
                                        op3.offset(HUF_decodeSymbolX2(op3 as
                                                                          *mut libc::c_void,
                                                                      &mut bitD3
                                                                          as
                                                                          *mut BIT_DStream_t,
                                                                      dt,
                                                                      dtLog)
                                                       as isize)
                                }
                                if 0 != MEM_64bits() || 12i32 <= 12i32 {
                                    op4 =
                                        op4.offset(HUF_decodeSymbolX2(op4 as
                                                                          *mut libc::c_void,
                                                                      &mut bitD4
                                                                          as
                                                                          *mut BIT_DStream_t,
                                                                      dt,
                                                                      dtLog)
                                                       as isize)
                                }
                                if 0 != MEM_64bits() {
                                    op1 =
                                        op1.offset(HUF_decodeSymbolX2(op1 as
                                                                          *mut libc::c_void,
                                                                      &mut bitD1
                                                                          as
                                                                          *mut BIT_DStream_t,
                                                                      dt,
                                                                      dtLog)
                                                       as isize)
                                }
                                if 0 != MEM_64bits() {
                                    op2 =
                                        op2.offset(HUF_decodeSymbolX2(op2 as
                                                                          *mut libc::c_void,
                                                                      &mut bitD2
                                                                          as
                                                                          *mut BIT_DStream_t,
                                                                      dt,
                                                                      dtLog)
                                                       as isize)
                                }
                                if 0 != MEM_64bits() {
                                    op3 =
                                        op3.offset(HUF_decodeSymbolX2(op3 as
                                                                          *mut libc::c_void,
                                                                      &mut bitD3
                                                                          as
                                                                          *mut BIT_DStream_t,
                                                                      dt,
                                                                      dtLog)
                                                       as isize)
                                }
                                if 0 != MEM_64bits() {
                                    op4 =
                                        op4.offset(HUF_decodeSymbolX2(op4 as
                                                                          *mut libc::c_void,
                                                                      &mut bitD4
                                                                          as
                                                                          *mut BIT_DStream_t,
                                                                      dt,
                                                                      dtLog)
                                                       as isize)
                                }
                                op1 =
                                    op1.offset(HUF_decodeSymbolX2(op1 as
                                                                      *mut libc::c_void,
                                                                  &mut bitD1
                                                                      as
                                                                      *mut BIT_DStream_t,
                                                                  dt, dtLog)
                                                   as isize);
                                op2 =
                                    op2.offset(HUF_decodeSymbolX2(op2 as
                                                                      *mut libc::c_void,
                                                                  &mut bitD2
                                                                      as
                                                                      *mut BIT_DStream_t,
                                                                  dt, dtLog)
                                                   as isize);
                                op3 =
                                    op3.offset(HUF_decodeSymbolX2(op3 as
                                                                      *mut libc::c_void,
                                                                  &mut bitD3
                                                                      as
                                                                      *mut BIT_DStream_t,
                                                                  dt, dtLog)
                                                   as isize);
                                op4 =
                                    op4.offset(HUF_decodeSymbolX2(op4 as
                                                                      *mut libc::c_void,
                                                                  &mut bitD4
                                                                      as
                                                                      *mut BIT_DStream_t,
                                                                  dt, dtLog)
                                                   as isize);
                                endSignal =
                                    BIT_reloadDStream(&mut bitD1 as
                                                          *mut BIT_DStream_t)
                                        as libc::c_uint |
                                        BIT_reloadDStream(&mut bitD2 as
                                                              *mut BIT_DStream_t)
                                            as libc::c_uint |
                                        BIT_reloadDStream(&mut bitD3 as
                                                              *mut BIT_DStream_t)
                                            as libc::c_uint |
                                        BIT_reloadDStream(&mut bitD4 as
                                                              *mut BIT_DStream_t)
                                            as libc::c_uint
                            }
                            if op1 > opStart2 {
                                return -(ZSTD_error_corruption_detected as
                                             libc::c_int) as size_t
                            } else if op2 > opStart3 {
                                return -(ZSTD_error_corruption_detected as
                                             libc::c_int) as size_t
                            } else if op3 > opStart4 {
                                return -(ZSTD_error_corruption_detected as
                                             libc::c_int) as size_t
                            } else {
                                HUF_decodeStreamX2(op1,
                                                   &mut bitD1 as
                                                       *mut BIT_DStream_t,
                                                   opStart2, dt, dtLog);
                                HUF_decodeStreamX2(op2,
                                                   &mut bitD2 as
                                                       *mut BIT_DStream_t,
                                                   opStart3, dt, dtLog);
                                HUF_decodeStreamX2(op3,
                                                   &mut bitD3 as
                                                       *mut BIT_DStream_t,
                                                   opStart4, dt, dtLog);
                                HUF_decodeStreamX2(op4,
                                                   &mut bitD4 as
                                                       *mut BIT_DStream_t,
                                                   oend, dt, dtLog);
                                let endCheck: U32 =
                                    BIT_endOfDStream(&mut bitD1 as
                                                         *mut BIT_DStream_t) &
                                        BIT_endOfDStream(&mut bitD2 as
                                                             *mut BIT_DStream_t)
                                        &
                                        BIT_endOfDStream(&mut bitD3 as
                                                             *mut BIT_DStream_t)
                                        &
                                        BIT_endOfDStream(&mut bitD4 as
                                                             *mut BIT_DStream_t);
                                if 0 == endCheck {
                                    return -(ZSTD_error_corruption_detected as
                                                 libc::c_int) as size_t
                                } else { return dstSize }
                            }
                        }
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn HUF_getDTableDesc(mut table: *const HUF_DTable)
 -> DTableDesc {
    let mut dtd: DTableDesc =
        DTableDesc{maxTableLog: 0, tableType: 0, tableLog: 0, reserved: 0,};
    memcpy(&mut dtd as *mut DTableDesc as *mut libc::c_void,
           table as *const libc::c_void,
           ::std::mem::size_of::<DTableDesc>() as libc::c_ulong);
    return dtd;
}
unsafe extern "C" fn HUF_decodeStreamX2(mut p: *mut BYTE,
                                        mut bitDPtr: *mut BIT_DStream_t,
                                        pEnd: *mut BYTE,
                                        dt: *const HUF_DEltX2, dtLog: U32)
 -> size_t {
    let pStart: *mut BYTE = p;
    while 0 !=
              (BIT_reloadDStream(bitDPtr) as libc::c_uint ==
                   BIT_DStream_unfinished as libc::c_int as libc::c_uint) as
                  libc::c_int &
                  (p <
                       pEnd.offset(-((::std::mem::size_of::<size_t>() as
                                          libc::c_ulong).wrapping_sub(1i32 as
                                                                          libc::c_ulong)
                                         as isize))) as libc::c_int {
        if 0 != MEM_64bits() {
            p =
                p.offset(HUF_decodeSymbolX2(p as *mut libc::c_void, bitDPtr,
                                            dt, dtLog) as isize)
        }
        if 0 != MEM_64bits() || 12i32 <= 12i32 {
            p =
                p.offset(HUF_decodeSymbolX2(p as *mut libc::c_void, bitDPtr,
                                            dt, dtLog) as isize)
        }
        if 0 != MEM_64bits() {
            p =
                p.offset(HUF_decodeSymbolX2(p as *mut libc::c_void, bitDPtr,
                                            dt, dtLog) as isize)
        }
        p =
            p.offset(HUF_decodeSymbolX2(p as *mut libc::c_void, bitDPtr, dt,
                                        dtLog) as isize)
    }
    while 0 !=
              (BIT_reloadDStream(bitDPtr) as libc::c_uint ==
                   BIT_DStream_unfinished as libc::c_int as libc::c_uint) as
                  libc::c_int & (p <= pEnd.offset(-2isize)) as libc::c_int {
        p =
            p.offset(HUF_decodeSymbolX2(p as *mut libc::c_void, bitDPtr, dt,
                                        dtLog) as isize)
    }
    while p <= pEnd.offset(-2isize) {
        p =
            p.offset(HUF_decodeSymbolX2(p as *mut libc::c_void, bitDPtr, dt,
                                        dtLog) as isize)
    }
    if p < pEnd {
        p =
            p.offset(HUF_decodeLastSymbolX2(p as *mut libc::c_void, bitDPtr,
                                            dt, dtLog) as isize)
    }
    return pStart.offset_to(p).expect("bad offset_to") as libc::c_long as
               size_t;
}
unsafe extern "C" fn HUF_decodeLastSymbolX2(mut op: *mut libc::c_void,
                                            mut DStream: *mut BIT_DStream_t,
                                            mut dt: *const HUF_DEltX2,
                                            dtLog: U32) -> U32 {
    let val: size_t = BIT_lookBitsFast(DStream, dtLog);
    memcpy(op, dt.offset(val as isize) as *const libc::c_void,
           1i32 as libc::c_ulong);
    if (*dt.offset(val as isize)).length as libc::c_int == 1i32 {
        BIT_skipBits(DStream, (*dt.offset(val as isize)).nbBits as U32);
    } else if ((*DStream).bitsConsumed as libc::c_ulong) <
                  (::std::mem::size_of::<size_t>() as
                       libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
        BIT_skipBits(DStream, (*dt.offset(val as isize)).nbBits as U32);
        if (*DStream).bitsConsumed as libc::c_ulong >
               (::std::mem::size_of::<size_t>() as
                    libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
            (*DStream).bitsConsumed =
                (::std::mem::size_of::<size_t>() as
                     libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) as
                    libc::c_uint
        }
    }
    return 1i32 as U32;
}
unsafe extern "C" fn HUF_decodeSymbolX2(mut op: *mut libc::c_void,
                                        mut DStream: *mut BIT_DStream_t,
                                        mut dt: *const HUF_DEltX2, dtLog: U32)
 -> U32 {
    let val: size_t = BIT_lookBitsFast(DStream, dtLog);
    memcpy(op, dt.offset(val as isize) as *const libc::c_void,
           2i32 as libc::c_ulong);
    BIT_skipBits(DStream, (*dt.offset(val as isize)).nbBits as U32);
    return (*dt.offset(val as isize)).length as U32;
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_bmi2(mut dst:
                                                                     *mut libc::c_void,
                                                                 mut dstSize:
                                                                     size_t,
                                                                 mut cSrc:
                                                                     *const libc::c_void,
                                                                 mut cSrcSize:
                                                                     size_t,
                                                                 mut DTable:
                                                                     *const HUF_DTable)
 -> size_t {
    return HUF_decompress4X2_usingDTable_internal_body(dst, dstSize, cSrc,
                                                       cSrcSize, DTable);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readDTableX2_wksp(mut DTable: *mut HUF_DTable,
                                               mut src: *const libc::c_void,
                                               mut srcSize: size_t,
                                               mut workSpace:
                                                   *mut libc::c_void,
                                               mut wkspSize: size_t)
 -> size_t {
    let mut tableLog: U32 = 0;
    let mut maxW: U32 = 0;
    let mut sizeOfSort: U32 = 0;
    let mut nbSymbols: U32 = 0;
    let mut dtd: DTableDesc = HUF_getDTableDesc(DTable);
    let maxTableLog: U32 = dtd.maxTableLog as U32;
    let mut iSize: size_t = 0;
    let mut dtPtr: *mut libc::c_void =
        DTable.offset(1isize) as *mut libc::c_void;
    let dt: *mut HUF_DEltX2 = dtPtr as *mut HUF_DEltX2;
    let mut rankStart: *mut U32 = 0 as *mut U32;
    let mut rankVal: *mut rankValCol_t = 0 as *mut rankValCol_t;
    let mut rankStats: *mut U32 = 0 as *mut U32;
    let mut rankStart0: *mut U32 = 0 as *mut U32;
    let mut sortedSymbol: *mut sortedSymbol_t = 0 as *mut sortedSymbol_t;
    let mut weightList: *mut BYTE = 0 as *mut BYTE;
    let mut spaceUsed32: size_t = 0i32 as size_t;
    rankVal =
        (workSpace as *mut U32).offset(spaceUsed32 as isize) as
            *mut rankValCol_t;
    spaceUsed32 =
        (spaceUsed32 as
             libc::c_ulong).wrapping_add((::std::mem::size_of::<rankValCol_t>()
                                              as
                                              libc::c_ulong).wrapping_mul(12i32
                                                                              as
                                                                              libc::c_ulong)
                                             >> 2i32) as size_t as size_t;
    rankStats = (workSpace as *mut U32).offset(spaceUsed32 as isize);
    spaceUsed32 =
        (spaceUsed32 as
             libc::c_ulong).wrapping_add((12i32 + 1i32) as libc::c_ulong) as
            size_t as size_t;
    rankStart0 = (workSpace as *mut U32).offset(spaceUsed32 as isize);
    spaceUsed32 =
        (spaceUsed32 as
             libc::c_ulong).wrapping_add((12i32 + 2i32) as libc::c_ulong) as
            size_t as size_t;
    sortedSymbol =
        (workSpace as
             *mut sortedSymbol_t).offset(spaceUsed32.wrapping_mul(::std::mem::size_of::<U32>()
                                                                      as
                                                                      libc::c_ulong).wrapping_div(::std::mem::size_of::<sortedSymbol_t>()
                                                                                                      as
                                                                                                      libc::c_ulong)
                                             as isize);
    spaceUsed32 =
        (spaceUsed32 as
             libc::c_ulong).wrapping_add(((::std::mem::size_of::<sortedSymbol_t>()
                                               as
                                               libc::c_ulong).wrapping_mul((255i32
                                                                                +
                                                                                1i32)
                                                                               as
                                                                               libc::c_ulong).wrapping_add((::std::mem::size_of::<U32>()
                                                                                                                as
                                                                                                                libc::c_ulong).wrapping_sub(1i32
                                                                                                                                                as
                                                                                                                                                libc::c_ulong))
                                              &
                                              !(::std::mem::size_of::<U32>()
                                                    as
                                                    libc::c_ulong).wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_ulong))
                                             >> 2i32) as size_t as size_t;
    weightList =
        (workSpace as *mut U32).offset(spaceUsed32 as isize) as *mut BYTE;
    spaceUsed32 =
        (spaceUsed32 as
             libc::c_ulong).wrapping_add((((255i32 + 1i32) as
                                               libc::c_ulong).wrapping_add((::std::mem::size_of::<U32>()
                                                                                as
                                                                                libc::c_ulong).wrapping_sub(1i32
                                                                                                                as
                                                                                                                libc::c_ulong))
                                              &
                                              !(::std::mem::size_of::<U32>()
                                                    as
                                                    libc::c_ulong).wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_ulong))
                                             >> 2i32) as size_t as size_t;
    if spaceUsed32 << 2i32 > wkspSize {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    } else {
        rankStart = rankStart0.offset(1isize);
        memset(rankStats as *mut libc::c_void, 0i32,
               (::std::mem::size_of::<U32>() as
                    libc::c_ulong).wrapping_mul((2i32 * 12i32 + 2i32 + 1i32)
                                                    as libc::c_ulong));
        if maxTableLog > 12i32 as libc::c_uint {
            return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
        } else {
            iSize =
                HUF_readStats(weightList, (255i32 + 1i32) as size_t,
                              rankStats, &mut nbSymbols as *mut U32,
                              &mut tableLog as *mut U32, src, srcSize);
            if 0 != ERR_isError(iSize) {
                return iSize
            } else if tableLog > maxTableLog {
                return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as
                           size_t
            } else {
                maxW = tableLog;
                while *rankStats.offset(maxW as isize) == 0i32 as libc::c_uint
                      {
                    maxW = maxW.wrapping_sub(1)
                }
                let mut w: U32 = 0;
                let mut nextRankStart: U32 = 0i32 as U32;
                w = 1i32 as U32;
                while w < maxW.wrapping_add(1i32 as libc::c_uint) {
                    let mut current: U32 = nextRankStart;
                    nextRankStart =
                        (nextRankStart as
                             libc::c_uint).wrapping_add(*rankStats.offset(w as
                                                                              isize))
                            as U32 as U32;
                    *rankStart.offset(w as isize) = current;
                    w = w.wrapping_add(1)
                }
                *rankStart.offset(0isize) = nextRankStart;
                sizeOfSort = nextRankStart;
                let mut s: U32 = 0;
                s = 0i32 as U32;
                while s < nbSymbols {
                    let w_0: U32 = *weightList.offset(s as isize) as U32;
                    let ref mut fresh0 = *rankStart.offset(w_0 as isize);
                    let fresh1 = *fresh0;
                    *fresh0 = (*fresh0).wrapping_add(1);
                    let r: U32 = fresh1;
                    (*sortedSymbol.offset(r as isize)).symbol = s as BYTE;
                    (*sortedSymbol.offset(r as isize)).weight = w_0 as BYTE;
                    s = s.wrapping_add(1)
                }
                *rankStart.offset(0isize) = 0i32 as U32;
                let rankVal0: *mut U32 =
                    (*rankVal.offset(0isize)).as_mut_ptr();
                let rescale: libc::c_int =
                    maxTableLog.wrapping_sub(tableLog).wrapping_sub(1i32 as
                                                                        libc::c_uint)
                        as libc::c_int;
                let mut nextRankVal: U32 = 0i32 as U32;
                let mut w_1: U32 = 0;
                w_1 = 1i32 as U32;
                while w_1 < maxW.wrapping_add(1i32 as libc::c_uint) {
                    let mut current_0: U32 = nextRankVal;
                    nextRankVal =
                        (nextRankVal as
                             libc::c_uint).wrapping_add(*rankStats.offset(w_1
                                                                              as
                                                                              isize)
                                                            <<
                                                            w_1.wrapping_add(rescale
                                                                                 as
                                                                                 libc::c_uint))
                            as U32 as U32;
                    *rankVal0.offset(w_1 as isize) = current_0;
                    w_1 = w_1.wrapping_add(1)
                }
                let minBits: U32 =
                    tableLog.wrapping_add(1i32 as
                                              libc::c_uint).wrapping_sub(maxW);
                let mut consumed: U32 = 0;
                consumed = minBits;
                while consumed <
                          maxTableLog.wrapping_sub(minBits).wrapping_add(1i32
                                                                             as
                                                                             libc::c_uint)
                      {
                    let rankValPtr: *mut U32 =
                        (*rankVal.offset(consumed as isize)).as_mut_ptr();
                    let mut w_2: U32 = 0;
                    w_2 = 1i32 as U32;
                    while w_2 < maxW.wrapping_add(1i32 as libc::c_uint) {
                        *rankValPtr.offset(w_2 as isize) =
                            *rankVal0.offset(w_2 as isize) >> consumed;
                        w_2 = w_2.wrapping_add(1)
                    }
                    consumed = consumed.wrapping_add(1)
                }
                HUF_fillDTableX2(dt, maxTableLog, sortedSymbol, sizeOfSort,
                                 rankStart0, rankVal, maxW,
                                 tableLog.wrapping_add(1i32 as libc::c_uint));
                dtd.tableLog = maxTableLog as BYTE;
                dtd.tableType = 1i32 as BYTE;
                memcpy(DTable as *mut libc::c_void,
                       &mut dtd as *mut DTableDesc as *const libc::c_void,
                       ::std::mem::size_of::<DTableDesc>() as libc::c_ulong);
                return iSize
            }
        }
    };
}
unsafe extern "C" fn HUF_fillDTableX2(mut DTable: *mut HUF_DEltX2,
                                      targetLog: U32,
                                      mut sortedList: *const sortedSymbol_t,
                                      sortedListSize: U32,
                                      mut rankStart: *const U32,
                                      mut rankValOrigin: *mut rankValCol_t,
                                      maxWeight: U32, nbBitsBaseline: U32)
 -> () {
    let mut rankVal: [U32; 13] = [0; 13];
    let scaleLog: libc::c_int =
        nbBitsBaseline.wrapping_sub(targetLog) as libc::c_int;
    let minBits: U32 = nbBitsBaseline.wrapping_sub(maxWeight);
    let mut s: U32 = 0;
    memcpy(rankVal.as_mut_ptr() as *mut libc::c_void,
           rankValOrigin as *const libc::c_void,
           ::std::mem::size_of::<[U32; 13]>() as libc::c_ulong);
    s = 0i32 as U32;
    while s < sortedListSize {
        let symbol: U16 = (*sortedList.offset(s as isize)).symbol as U16;
        let weight: U32 = (*sortedList.offset(s as isize)).weight as U32;
        let nbBits: U32 = nbBitsBaseline.wrapping_sub(weight);
        let start: U32 = rankVal[weight as usize];
        let length: U32 = (1i32 << targetLog.wrapping_sub(nbBits)) as U32;
        if targetLog.wrapping_sub(nbBits) >= minBits {
            let mut sortedRank: U32 = 0;
            let mut minWeight: libc::c_int =
                nbBits.wrapping_add(scaleLog as libc::c_uint) as libc::c_int;
            if minWeight < 1i32 { minWeight = 1i32 }
            sortedRank = *rankStart.offset(minWeight as isize);
            HUF_fillDTableX2Level2(DTable.offset(start as isize),
                                   targetLog.wrapping_sub(nbBits), nbBits,
                                   (*rankValOrigin.offset(nbBits as
                                                              isize)).as_mut_ptr(),
                                   minWeight,
                                   sortedList.offset(sortedRank as isize),
                                   sortedListSize.wrapping_sub(sortedRank),
                                   nbBitsBaseline, symbol);
        } else {
            let mut DElt: HUF_DEltX2 =
                HUF_DEltX2{sequence: 0, nbBits: 0, length: 0,};
            MEM_writeLE16(&mut DElt.sequence as *mut U16 as *mut libc::c_void,
                          symbol);
            DElt.nbBits = nbBits as BYTE;
            DElt.length = 1i32 as BYTE;
            let end: U32 = start.wrapping_add(length);
            let mut u: U32 = 0;
            u = start;
            while u < end {
                *DTable.offset(u as isize) = DElt;
                u = u.wrapping_add(1)
            }
        }
        rankVal[weight as usize] =
            (rankVal[weight as usize] as libc::c_uint).wrapping_add(length) as
                U32 as U32;
        s = s.wrapping_add(1)
    };
}
unsafe extern "C" fn HUF_fillDTableX2Level2(mut DTable: *mut HUF_DEltX2,
                                            mut sizeLog: U32, consumed: U32,
                                            mut rankValOrigin: *const U32,
                                            minWeight: libc::c_int,
                                            mut sortedSymbols:
                                                *const sortedSymbol_t,
                                            sortedListSize: U32,
                                            mut nbBitsBaseline: U32,
                                            mut baseSeq: U16) -> () {
    let mut DElt: HUF_DEltX2 = HUF_DEltX2{sequence: 0, nbBits: 0, length: 0,};
    let mut rankVal: [U32; 13] = [0; 13];
    memcpy(rankVal.as_mut_ptr() as *mut libc::c_void,
           rankValOrigin as *const libc::c_void,
           ::std::mem::size_of::<[U32; 13]>() as libc::c_ulong);
    if minWeight > 1i32 {
        let mut i: U32 = 0;
        let mut skipSize: U32 = rankVal[minWeight as usize];
        MEM_writeLE16(&mut DElt.sequence as *mut U16 as *mut libc::c_void,
                      baseSeq);
        DElt.nbBits = consumed as BYTE;
        DElt.length = 1i32 as BYTE;
        i = 0i32 as U32;
        while i < skipSize {
            *DTable.offset(i as isize) = DElt;
            i = i.wrapping_add(1)
        }
    }
    let mut s: U32 = 0;
    s = 0i32 as U32;
    while s < sortedListSize {
        let symbol: U32 = (*sortedSymbols.offset(s as isize)).symbol as U32;
        let weight: U32 = (*sortedSymbols.offset(s as isize)).weight as U32;
        let nbBits: U32 = nbBitsBaseline.wrapping_sub(weight);
        let length: U32 = (1i32 << sizeLog.wrapping_sub(nbBits)) as U32;
        let start: U32 = rankVal[weight as usize];
        let mut i_0: U32 = start;
        let end: U32 = start.wrapping_add(length);
        MEM_writeLE16(&mut DElt.sequence as *mut U16 as *mut libc::c_void,
                      (baseSeq as libc::c_uint).wrapping_add(symbol << 8i32)
                          as U16);
        DElt.nbBits = nbBits.wrapping_add(consumed) as BYTE;
        DElt.length = 2i32 as BYTE;
        loop  {
            let fresh2 = i_0;
            i_0 = i_0.wrapping_add(1);
            *DTable.offset(fresh2 as isize) = DElt;
            if !(i_0 < end) { break ; }
        }
        rankVal[weight as usize] =
            (rankVal[weight as usize] as libc::c_uint).wrapping_add(length) as
                U32 as U32;
        s = s.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X1(mut dst: *mut libc::c_void,
                                           mut dstSize: size_t,
                                           mut cSrc: *const libc::c_void,
                                           mut cSrcSize: size_t) -> size_t {
    let mut DTable: [HUF_DTable; 2049] =
        [((12i32 - 1i32) as U32).wrapping_mul(16777217i32 as libc::c_uint), 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    return HUF_decompress4X1_DCtx(DTable.as_mut_ptr(), dst, dstSize, cSrc,
                                  cSrcSize);
}
/* *< considers RLE and uncompressed as errors */
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X1_DCtx(mut dctx: *mut HUF_DTable,
                                                mut dst: *mut libc::c_void,
                                                mut dstSize: size_t,
                                                mut cSrc: *const libc::c_void,
                                                mut cSrcSize: size_t)
 -> size_t {
    let mut workSpace: [U32; 512] = [0; 512];
    return HUF_decompress4X1_DCtx_wksp(dctx, dst, dstSize, cSrc, cSrcSize,
                                       workSpace.as_mut_ptr() as
                                           *mut libc::c_void,
                                       ::std::mem::size_of::<[U32; 512]>() as
                                           libc::c_ulong);
}
/* *< single-symbol decoder */
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X1_DCtx_wksp(mut dctx:
                                                         *mut HUF_DTable,
                                                     mut dst:
                                                         *mut libc::c_void,
                                                     mut dstSize: size_t,
                                                     mut cSrc:
                                                         *const libc::c_void,
                                                     mut cSrcSize: size_t,
                                                     mut workSpace:
                                                         *mut libc::c_void,
                                                     mut wkspSize: size_t)
 -> size_t {
    return HUF_decompress4X1_DCtx_wksp_bmi2(dctx, dst, dstSize, cSrc,
                                            cSrcSize, workSpace, wkspSize,
                                            0i32);
}
unsafe extern "C" fn HUF_decompress4X1_DCtx_wksp_bmi2(mut dctx:
                                                          *mut HUF_DTable,
                                                      mut dst:
                                                          *mut libc::c_void,
                                                      mut dstSize: size_t,
                                                      mut cSrc:
                                                          *const libc::c_void,
                                                      mut cSrcSize: size_t,
                                                      mut workSpace:
                                                          *mut libc::c_void,
                                                      mut wkspSize: size_t,
                                                      mut bmi2: libc::c_int)
 -> size_t {
    let mut ip: *const BYTE = cSrc as *const BYTE;
    let hSize: size_t =
        HUF_readDTableX1_wksp(dctx, cSrc, cSrcSize, workSpace, wkspSize);
    if 0 != ERR_isError(hSize) {
        return hSize
    } else if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else {
        ip = ip.offset(hSize as isize);
        cSrcSize =
            (cSrcSize as libc::c_ulong).wrapping_sub(hSize) as size_t as
                size_t;
        return HUF_decompress4X1_usingDTable_internal(dst, dstSize,
                                                      ip as
                                                          *const libc::c_void,
                                                      cSrcSize, dctx, bmi2)
    };
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal(mut dst:
                                                                *mut libc::c_void,
                                                            mut dstSize:
                                                                size_t,
                                                            mut cSrc:
                                                                *const libc::c_void,
                                                            mut cSrcSize:
                                                                size_t,
                                                            mut DTable:
                                                                *const HUF_DTable,
                                                            mut bmi2:
                                                                libc::c_int)
 -> size_t {
    if 0 != bmi2 {
        return HUF_decompress4X1_usingDTable_internal_bmi2(dst, dstSize, cSrc,
                                                           cSrcSize, DTable)
    } else {
        return HUF_decompress4X1_usingDTable_internal_default(dst, dstSize,
                                                              cSrc, cSrcSize,
                                                              DTable)
    };
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_default(mut dst:
                                                                        *mut libc::c_void,
                                                                    mut dstSize:
                                                                        size_t,
                                                                    mut cSrc:
                                                                        *const libc::c_void,
                                                                    mut cSrcSize:
                                                                        size_t,
                                                                    mut DTable:
                                                                        *const HUF_DTable)
 -> size_t {
    return HUF_decompress4X1_usingDTable_internal_body(dst, dstSize, cSrc,
                                                       cSrcSize, DTable);
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_body(mut dst:
                                                                     *mut libc::c_void,
                                                                 mut dstSize:
                                                                     size_t,
                                                                 mut cSrc:
                                                                     *const libc::c_void,
                                                                 mut cSrcSize:
                                                                     size_t,
                                                                 mut DTable:
                                                                     *const HUF_DTable)
 -> size_t {
    if cSrcSize < 10i32 as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    } else {
        let istart: *const BYTE = cSrc as *const BYTE;
        let ostart: *mut BYTE = dst as *mut BYTE;
        let oend: *mut BYTE = ostart.offset(dstSize as isize);
        let dtPtr: *const libc::c_void =
            DTable.offset(1isize) as *const libc::c_void;
        let dt: *const HUF_DEltX1 = dtPtr as *const HUF_DEltX1;
        let mut bitD1: BIT_DStream_t =
            BIT_DStream_t{bitContainer: 0,
                          bitsConsumed: 0,
                          ptr: 0 as *const libc::c_char,
                          start: 0 as *const libc::c_char,
                          limitPtr: 0 as *const libc::c_char,};
        let mut bitD2: BIT_DStream_t =
            BIT_DStream_t{bitContainer: 0,
                          bitsConsumed: 0,
                          ptr: 0 as *const libc::c_char,
                          start: 0 as *const libc::c_char,
                          limitPtr: 0 as *const libc::c_char,};
        let mut bitD3: BIT_DStream_t =
            BIT_DStream_t{bitContainer: 0,
                          bitsConsumed: 0,
                          ptr: 0 as *const libc::c_char,
                          start: 0 as *const libc::c_char,
                          limitPtr: 0 as *const libc::c_char,};
        let mut bitD4: BIT_DStream_t =
            BIT_DStream_t{bitContainer: 0,
                          bitsConsumed: 0,
                          ptr: 0 as *const libc::c_char,
                          start: 0 as *const libc::c_char,
                          limitPtr: 0 as *const libc::c_char,};
        let length1: size_t =
            MEM_readLE16(istart as *const libc::c_void) as size_t;
        let length2: size_t =
            MEM_readLE16(istart.offset(2isize) as *const libc::c_void) as
                size_t;
        let length3: size_t =
            MEM_readLE16(istart.offset(4isize) as *const libc::c_void) as
                size_t;
        let length4: size_t =
            cSrcSize.wrapping_sub(length1.wrapping_add(length2).wrapping_add(length3).wrapping_add(6i32
                                                                                                       as
                                                                                                       libc::c_ulong));
        let istart1: *const BYTE = istart.offset(6isize);
        let istart2: *const BYTE = istart1.offset(length1 as isize);
        let istart3: *const BYTE = istart2.offset(length2 as isize);
        let istart4: *const BYTE = istart3.offset(length3 as isize);
        let segmentSize: size_t =
            dstSize.wrapping_add(3i32 as
                                     libc::c_ulong).wrapping_div(4i32 as
                                                                     libc::c_ulong);
        let opStart2: *mut BYTE = ostart.offset(segmentSize as isize);
        let opStart3: *mut BYTE = opStart2.offset(segmentSize as isize);
        let opStart4: *mut BYTE = opStart3.offset(segmentSize as isize);
        let mut op1: *mut BYTE = ostart;
        let mut op2: *mut BYTE = opStart2;
        let mut op3: *mut BYTE = opStart3;
        let mut op4: *mut BYTE = opStart4;
        let mut endSignal: U32 = BIT_DStream_unfinished as libc::c_int as U32;
        let dtd: DTableDesc = HUF_getDTableDesc(DTable);
        let dtLog: U32 = dtd.tableLog as U32;
        if length4 > cSrcSize {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        } else {
            let err_: size_t =
                BIT_initDStream(&mut bitD1 as *mut BIT_DStream_t,
                                istart1 as *const libc::c_void, length1);
            if 0 != ERR_isError(err_) {
                return err_
            } else {
                let err__0: size_t =
                    BIT_initDStream(&mut bitD2 as *mut BIT_DStream_t,
                                    istart2 as *const libc::c_void, length2);
                if 0 != ERR_isError(err__0) {
                    return err__0
                } else {
                    let err__1: size_t =
                        BIT_initDStream(&mut bitD3 as *mut BIT_DStream_t,
                                        istart3 as *const libc::c_void,
                                        length3);
                    if 0 != ERR_isError(err__1) {
                        return err__1
                    } else {
                        let err__2: size_t =
                            BIT_initDStream(&mut bitD4 as *mut BIT_DStream_t,
                                            istart4 as *const libc::c_void,
                                            length4);
                        if 0 != ERR_isError(err__2) {
                            return err__2
                        } else {
                            endSignal =
                                BIT_reloadDStream(&mut bitD1 as
                                                      *mut BIT_DStream_t) as
                                    libc::c_uint |
                                    BIT_reloadDStream(&mut bitD2 as
                                                          *mut BIT_DStream_t)
                                        as libc::c_uint |
                                    BIT_reloadDStream(&mut bitD3 as
                                                          *mut BIT_DStream_t)
                                        as libc::c_uint |
                                    BIT_reloadDStream(&mut bitD4 as
                                                          *mut BIT_DStream_t)
                                        as libc::c_uint;
                            while endSignal ==
                                      BIT_DStream_unfinished as libc::c_int as
                                          libc::c_uint &&
                                      op4 < oend.offset(-3isize) {
                                if 0 != MEM_64bits() {
                                    let fresh3 = op1;
                                    op1 = op1.offset(1);
                                    *fresh3 =
                                        HUF_decodeSymbolX1(&mut bitD1 as
                                                               *mut BIT_DStream_t,
                                                           dt, dtLog)
                                }
                                if 0 != MEM_64bits() {
                                    let fresh4 = op2;
                                    op2 = op2.offset(1);
                                    *fresh4 =
                                        HUF_decodeSymbolX1(&mut bitD2 as
                                                               *mut BIT_DStream_t,
                                                           dt, dtLog)
                                }
                                if 0 != MEM_64bits() {
                                    let fresh5 = op3;
                                    op3 = op3.offset(1);
                                    *fresh5 =
                                        HUF_decodeSymbolX1(&mut bitD3 as
                                                               *mut BIT_DStream_t,
                                                           dt, dtLog)
                                }
                                if 0 != MEM_64bits() {
                                    let fresh6 = op4;
                                    op4 = op4.offset(1);
                                    *fresh6 =
                                        HUF_decodeSymbolX1(&mut bitD4 as
                                                               *mut BIT_DStream_t,
                                                           dt, dtLog)
                                }
                                if 0 != MEM_64bits() || 12i32 <= 12i32 {
                                    let fresh7 = op1;
                                    op1 = op1.offset(1);
                                    *fresh7 =
                                        HUF_decodeSymbolX1(&mut bitD1 as
                                                               *mut BIT_DStream_t,
                                                           dt, dtLog)
                                }
                                if 0 != MEM_64bits() || 12i32 <= 12i32 {
                                    let fresh8 = op2;
                                    op2 = op2.offset(1);
                                    *fresh8 =
                                        HUF_decodeSymbolX1(&mut bitD2 as
                                                               *mut BIT_DStream_t,
                                                           dt, dtLog)
                                }
                                if 0 != MEM_64bits() || 12i32 <= 12i32 {
                                    let fresh9 = op3;
                                    op3 = op3.offset(1);
                                    *fresh9 =
                                        HUF_decodeSymbolX1(&mut bitD3 as
                                                               *mut BIT_DStream_t,
                                                           dt, dtLog)
                                }
                                if 0 != MEM_64bits() || 12i32 <= 12i32 {
                                    let fresh10 = op4;
                                    op4 = op4.offset(1);
                                    *fresh10 =
                                        HUF_decodeSymbolX1(&mut bitD4 as
                                                               *mut BIT_DStream_t,
                                                           dt, dtLog)
                                }
                                if 0 != MEM_64bits() {
                                    let fresh11 = op1;
                                    op1 = op1.offset(1);
                                    *fresh11 =
                                        HUF_decodeSymbolX1(&mut bitD1 as
                                                               *mut BIT_DStream_t,
                                                           dt, dtLog)
                                }
                                if 0 != MEM_64bits() {
                                    let fresh12 = op2;
                                    op2 = op2.offset(1);
                                    *fresh12 =
                                        HUF_decodeSymbolX1(&mut bitD2 as
                                                               *mut BIT_DStream_t,
                                                           dt, dtLog)
                                }
                                if 0 != MEM_64bits() {
                                    let fresh13 = op3;
                                    op3 = op3.offset(1);
                                    *fresh13 =
                                        HUF_decodeSymbolX1(&mut bitD3 as
                                                               *mut BIT_DStream_t,
                                                           dt, dtLog)
                                }
                                if 0 != MEM_64bits() {
                                    let fresh14 = op4;
                                    op4 = op4.offset(1);
                                    *fresh14 =
                                        HUF_decodeSymbolX1(&mut bitD4 as
                                                               *mut BIT_DStream_t,
                                                           dt, dtLog)
                                }
                                let fresh15 = op1;
                                op1 = op1.offset(1);
                                *fresh15 =
                                    HUF_decodeSymbolX1(&mut bitD1 as
                                                           *mut BIT_DStream_t,
                                                       dt, dtLog);
                                let fresh16 = op2;
                                op2 = op2.offset(1);
                                *fresh16 =
                                    HUF_decodeSymbolX1(&mut bitD2 as
                                                           *mut BIT_DStream_t,
                                                       dt, dtLog);
                                let fresh17 = op3;
                                op3 = op3.offset(1);
                                *fresh17 =
                                    HUF_decodeSymbolX1(&mut bitD3 as
                                                           *mut BIT_DStream_t,
                                                       dt, dtLog);
                                let fresh18 = op4;
                                op4 = op4.offset(1);
                                *fresh18 =
                                    HUF_decodeSymbolX1(&mut bitD4 as
                                                           *mut BIT_DStream_t,
                                                       dt, dtLog);
                                BIT_reloadDStream(&mut bitD1 as
                                                      *mut BIT_DStream_t);
                                BIT_reloadDStream(&mut bitD2 as
                                                      *mut BIT_DStream_t);
                                BIT_reloadDStream(&mut bitD3 as
                                                      *mut BIT_DStream_t);
                                BIT_reloadDStream(&mut bitD4 as
                                                      *mut BIT_DStream_t);
                            }
                            if op1 > opStart2 {
                                return -(ZSTD_error_corruption_detected as
                                             libc::c_int) as size_t
                            } else if op2 > opStart3 {
                                return -(ZSTD_error_corruption_detected as
                                             libc::c_int) as size_t
                            } else if op3 > opStart4 {
                                return -(ZSTD_error_corruption_detected as
                                             libc::c_int) as size_t
                            } else {
                                HUF_decodeStreamX1(op1,
                                                   &mut bitD1 as
                                                       *mut BIT_DStream_t,
                                                   opStart2, dt, dtLog);
                                HUF_decodeStreamX1(op2,
                                                   &mut bitD2 as
                                                       *mut BIT_DStream_t,
                                                   opStart3, dt, dtLog);
                                HUF_decodeStreamX1(op3,
                                                   &mut bitD3 as
                                                       *mut BIT_DStream_t,
                                                   opStart4, dt, dtLog);
                                HUF_decodeStreamX1(op4,
                                                   &mut bitD4 as
                                                       *mut BIT_DStream_t,
                                                   oend, dt, dtLog);
                                let endCheck: U32 =
                                    BIT_endOfDStream(&mut bitD1 as
                                                         *mut BIT_DStream_t) &
                                        BIT_endOfDStream(&mut bitD2 as
                                                             *mut BIT_DStream_t)
                                        &
                                        BIT_endOfDStream(&mut bitD3 as
                                                             *mut BIT_DStream_t)
                                        &
                                        BIT_endOfDStream(&mut bitD4 as
                                                             *mut BIT_DStream_t);
                                if 0 == endCheck {
                                    return -(ZSTD_error_corruption_detected as
                                                 libc::c_int) as size_t
                                } else { return dstSize }
                            }
                        }
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn HUF_decodeStreamX1(mut p: *mut BYTE,
                                        bitDPtr: *mut BIT_DStream_t,
                                        pEnd: *mut BYTE,
                                        dt: *const HUF_DEltX1, dtLog: U32)
 -> size_t {
    let pStart: *mut BYTE = p;
    while 0 !=
              (BIT_reloadDStream(bitDPtr) as libc::c_uint ==
                   BIT_DStream_unfinished as libc::c_int as libc::c_uint) as
                  libc::c_int & (p < pEnd.offset(-3isize)) as libc::c_int {
        if 0 != MEM_64bits() {
            let fresh19 = p;
            p = p.offset(1);
            *fresh19 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog)
        }
        if 0 != MEM_64bits() || 12i32 <= 12i32 {
            let fresh20 = p;
            p = p.offset(1);
            *fresh20 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog)
        }
        if 0 != MEM_64bits() {
            let fresh21 = p;
            p = p.offset(1);
            *fresh21 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog)
        }
        let fresh22 = p;
        p = p.offset(1);
        *fresh22 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog)
    }
    if 0 != MEM_32bits() {
        while 0 !=
                  (BIT_reloadDStream(bitDPtr) as libc::c_uint ==
                       BIT_DStream_unfinished as libc::c_int as libc::c_uint)
                      as libc::c_int & (p < pEnd) as libc::c_int {
            let fresh23 = p;
            p = p.offset(1);
            *fresh23 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog)
        }
    }
    while p < pEnd {
        let fresh24 = p;
        p = p.offset(1);
        *fresh24 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog)
    }
    return pStart.offset_to(pEnd).expect("bad offset_to") as libc::c_long as
               size_t;
}
unsafe extern "C" fn HUF_decodeSymbolX1(mut Dstream: *mut BIT_DStream_t,
                                        mut dt: *const HUF_DEltX1, dtLog: U32)
 -> BYTE {
    let val: size_t = BIT_lookBitsFast(Dstream, dtLog);
    let c: BYTE = (*dt.offset(val as isize)).byte;
    BIT_skipBits(Dstream, (*dt.offset(val as isize)).nbBits as U32);
    return c;
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_bmi2(mut dst:
                                                                     *mut libc::c_void,
                                                                 mut dstSize:
                                                                     size_t,
                                                                 mut cSrc:
                                                                     *const libc::c_void,
                                                                 mut cSrcSize:
                                                                     size_t,
                                                                 mut DTable:
                                                                     *const HUF_DTable)
 -> size_t {
    return HUF_decompress4X1_usingDTable_internal_body(dst, dstSize, cSrc,
                                                       cSrcSize, DTable);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readDTableX1_wksp(mut DTable: *mut HUF_DTable,
                                               mut src: *const libc::c_void,
                                               mut srcSize: size_t,
                                               mut workSpace:
                                                   *mut libc::c_void,
                                               mut wkspSize: size_t)
 -> size_t {
    let mut tableLog: U32 = 0i32 as U32;
    let mut nbSymbols: U32 = 0i32 as U32;
    let mut iSize: size_t = 0;
    let dtPtr: *mut libc::c_void = DTable.offset(1isize) as *mut libc::c_void;
    let dt: *mut HUF_DEltX1 = dtPtr as *mut HUF_DEltX1;
    let mut rankVal: *mut U32 = 0 as *mut U32;
    let mut huffWeight: *mut BYTE = 0 as *mut BYTE;
    let mut spaceUsed32: size_t = 0i32 as size_t;
    rankVal = (workSpace as *mut U32).offset(spaceUsed32 as isize);
    spaceUsed32 =
        (spaceUsed32 as
             libc::c_ulong).wrapping_add((15i32 + 1i32) as libc::c_ulong) as
            size_t as size_t;
    huffWeight =
        (workSpace as *mut U32).offset(spaceUsed32 as isize) as *mut BYTE;
    spaceUsed32 =
        (spaceUsed32 as
             libc::c_ulong).wrapping_add((((255i32 + 1i32) as
                                               libc::c_ulong).wrapping_add((::std::mem::size_of::<U32>()
                                                                                as
                                                                                libc::c_ulong).wrapping_sub(1i32
                                                                                                                as
                                                                                                                libc::c_ulong))
                                              &
                                              !(::std::mem::size_of::<U32>()
                                                    as
                                                    libc::c_ulong).wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_ulong))
                                             >> 2i32) as size_t as size_t;
    if spaceUsed32 << 2i32 > wkspSize {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    } else {
        iSize =
            HUF_readStats(huffWeight, (255i32 + 1i32) as size_t, rankVal,
                          &mut nbSymbols as *mut U32,
                          &mut tableLog as *mut U32, src, srcSize);
        if 0 != ERR_isError(iSize) {
            return iSize
        } else {
            let mut dtd: DTableDesc = HUF_getDTableDesc(DTable);
            if tableLog > (dtd.maxTableLog as libc::c_int + 1i32) as U32 {
                return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as
                           size_t
            } else {
                dtd.tableType = 0i32 as BYTE;
                dtd.tableLog = tableLog as BYTE;
                memcpy(DTable as *mut libc::c_void,
                       &mut dtd as *mut DTableDesc as *const libc::c_void,
                       ::std::mem::size_of::<DTableDesc>() as libc::c_ulong);
                let mut n: U32 = 0;
                let mut nextRankStart: U32 = 0i32 as U32;
                n = 1i32 as U32;
                while n < tableLog.wrapping_add(1i32 as libc::c_uint) {
                    let current: U32 = nextRankStart;
                    nextRankStart =
                        (nextRankStart as
                             libc::c_uint).wrapping_add(*rankVal.offset(n as
                                                                            isize)
                                                            <<
                                                            n.wrapping_sub(1i32
                                                                               as
                                                                               libc::c_uint))
                            as U32 as U32;
                    *rankVal.offset(n as isize) = current;
                    n = n.wrapping_add(1)
                }
                let mut n_0: U32 = 0;
                n_0 = 0i32 as U32;
                while n_0 < nbSymbols {
                    let w: U32 = *huffWeight.offset(n_0 as isize) as U32;
                    let length: U32 = (1i32 << w >> 1i32) as U32;
                    let mut u: U32 = 0;
                    let mut D: HUF_DEltX1 = HUF_DEltX1{byte: 0, nbBits: 0,};
                    D.byte = n_0 as BYTE;
                    D.nbBits =
                        tableLog.wrapping_add(1i32 as
                                                  libc::c_uint).wrapping_sub(w)
                            as BYTE;
                    u = *rankVal.offset(w as isize);
                    while u <
                              (*rankVal.offset(w as
                                                   isize)).wrapping_add(length)
                          {
                        *dt.offset(u as isize) = D;
                        u = u.wrapping_add(1)
                    }
                    let ref mut fresh25 = *rankVal.offset(w as isize);
                    *fresh25 =
                        (*fresh25 as libc::c_uint).wrapping_add(length) as U32
                            as U32;
                    n_0 = n_0.wrapping_add(1)
                }
                return iSize
            }
        }
    };
}
/* *< double-symbols decoder */
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X_DCtx(mut dctx: *mut HUF_DTable,
                                               mut dst: *mut libc::c_void,
                                               mut dstSize: size_t,
                                               mut cSrc: *const libc::c_void,
                                               mut cSrcSize: size_t)
 -> size_t {
    if dstSize == 0i32 as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else if cSrcSize > dstSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    } else if cSrcSize == dstSize {
        memcpy(dst, cSrc, dstSize);
        return dstSize
    } else if cSrcSize == 1i32 as libc::c_ulong {
        memset(dst, *(cSrc as *const BYTE) as libc::c_int, dstSize);
        return dstSize
    } else {
        let algoNb: U32 = HUF_selectDecoder(dstSize, cSrcSize);
        return if 0 != algoNb {
                   HUF_decompress4X2_DCtx(dctx, dst, dstSize, cSrc, cSrcSize)
               } else {
                   HUF_decompress4X1_DCtx(dctx, dst, dstSize, cSrc, cSrcSize)
               }
    };
}
/* *< decodes RLE and uncompressed */
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X_hufOnly(mut dctx: *mut HUF_DTable,
                                                  mut dst: *mut libc::c_void,
                                                  mut dstSize: size_t,
                                                  mut cSrc:
                                                      *const libc::c_void,
                                                  mut cSrcSize: size_t)
 -> size_t {
    let mut workSpace: [U32; 512] = [0; 512];
    return HUF_decompress4X_hufOnly_wksp(dctx, dst, dstSize, cSrc, cSrcSize,
                                         workSpace.as_mut_ptr() as
                                             *mut libc::c_void,
                                         ::std::mem::size_of::<[U32; 512]>()
                                             as libc::c_ulong);
}
/* *< considers RLE and uncompressed as errors */
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X_hufOnly_wksp(mut dctx:
                                                           *mut HUF_DTable,
                                                       mut dst:
                                                           *mut libc::c_void,
                                                       mut dstSize: size_t,
                                                       mut cSrc:
                                                           *const libc::c_void,
                                                       mut cSrcSize: size_t,
                                                       mut workSpace:
                                                           *mut libc::c_void,
                                                       mut wkspSize: size_t)
 -> size_t {
    if dstSize == 0i32 as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else if cSrcSize == 0i32 as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    } else {
        let algoNb: U32 = HUF_selectDecoder(dstSize, cSrcSize);
        return if 0 != algoNb {
                   HUF_decompress4X2_DCtx_wksp(dctx, dst, dstSize, cSrc,
                                               cSrcSize, workSpace, wkspSize)
               } else {
                   HUF_decompress4X1_DCtx_wksp(dctx, dst, dstSize, cSrc,
                                               cSrcSize, workSpace, wkspSize)
               }
    };
}
/* *
 *  The minimum workspace size for the `workSpace` used in
 *  HUF_readDTableX1_wksp() and HUF_readDTableX2_wksp().
 *
 *  The space used depends on HUF_TABLELOG_MAX, ranging from ~1500 bytes when
 *  HUF_TABLE_LOG_MAX=12 to ~1850 bytes when HUF_TABLE_LOG_MAX=15.
 *  Buffer overflow errors may potentially occur if code modifications result in
 *  a required workspace size greater than that specified in the following
 *  macro.
 */
#[no_mangle]
pub unsafe extern "C" fn HUF_readDTableX1(mut DTable: *mut HUF_DTable,
                                          mut src: *const libc::c_void,
                                          mut srcSize: size_t) -> size_t {
    let mut workSpace: [U32; 512] = [0; 512];
    return HUF_readDTableX1_wksp(DTable, src, srcSize,
                                 workSpace.as_mut_ptr() as *mut libc::c_void,
                                 ::std::mem::size_of::<[U32; 512]>() as
                                     libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readDTableX2(mut DTable: *mut HUF_DTable,
                                          mut src: *const libc::c_void,
                                          mut srcSize: size_t) -> size_t {
    let mut workSpace: [U32; 512] = [0; 512];
    return HUF_readDTableX2_wksp(DTable, src, srcSize,
                                 workSpace.as_mut_ptr() as *mut libc::c_void,
                                 ::std::mem::size_of::<[U32; 512]>() as
                                     libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X_usingDTable(mut dst:
                                                          *mut libc::c_void,
                                                      mut maxDstSize: size_t,
                                                      mut cSrc:
                                                          *const libc::c_void,
                                                      mut cSrcSize: size_t,
                                                      mut DTable:
                                                          *const HUF_DTable)
 -> size_t {
    let dtd: DTableDesc = HUF_getDTableDesc(DTable);
    return if 0 != dtd.tableType as libc::c_int {
               HUF_decompress4X2_usingDTable_internal(dst, maxDstSize, cSrc,
                                                      cSrcSize, DTable, 0i32)
           } else {
               HUF_decompress4X1_usingDTable_internal(dst, maxDstSize, cSrc,
                                                      cSrcSize, DTable, 0i32)
           };
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X1_usingDTable(mut dst:
                                                           *mut libc::c_void,
                                                       mut dstSize: size_t,
                                                       mut cSrc:
                                                           *const libc::c_void,
                                                       mut cSrcSize: size_t,
                                                       mut DTable:
                                                           *const HUF_DTable)
 -> size_t {
    let mut dtd: DTableDesc = HUF_getDTableDesc(DTable);
    if dtd.tableType as libc::c_int != 0i32 {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        return HUF_decompress4X1_usingDTable_internal(dst, dstSize, cSrc,
                                                      cSrcSize, DTable, 0i32)
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X2_usingDTable(mut dst:
                                                           *mut libc::c_void,
                                                       mut dstSize: size_t,
                                                       mut cSrc:
                                                           *const libc::c_void,
                                                       mut cSrcSize: size_t,
                                                       mut DTable:
                                                           *const HUF_DTable)
 -> size_t {
    let mut dtd: DTableDesc = HUF_getDTableDesc(DTable);
    if dtd.tableType as libc::c_int != 1i32 {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        return HUF_decompress4X2_usingDTable_internal(dst, dstSize, cSrc,
                                                      cSrcSize, DTable, 0i32)
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X1(mut dst: *mut libc::c_void,
                                           mut dstSize: size_t,
                                           mut cSrc: *const libc::c_void,
                                           mut cSrcSize: size_t) -> size_t {
    let mut DTable: [HUF_DTable; 2049] =
        [((12i32 - 1i32) as U32).wrapping_mul(16777217i32 as libc::c_uint), 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    return HUF_decompress1X1_DCtx(DTable.as_mut_ptr(), dst, dstSize, cSrc,
                                  cSrcSize);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X1_DCtx(mut DCtx: *mut HUF_DTable,
                                                mut dst: *mut libc::c_void,
                                                mut dstSize: size_t,
                                                mut cSrc: *const libc::c_void,
                                                mut cSrcSize: size_t)
 -> size_t {
    let mut workSpace: [U32; 512] = [0; 512];
    return HUF_decompress1X1_DCtx_wksp(DCtx, dst, dstSize, cSrc, cSrcSize,
                                       workSpace.as_mut_ptr() as
                                           *mut libc::c_void,
                                       ::std::mem::size_of::<[U32; 512]>() as
                                           libc::c_ulong);
}
/* *< single-symbol decoder */
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X1_DCtx_wksp(mut DCtx:
                                                         *mut HUF_DTable,
                                                     mut dst:
                                                         *mut libc::c_void,
                                                     mut dstSize: size_t,
                                                     mut cSrc:
                                                         *const libc::c_void,
                                                     mut cSrcSize: size_t,
                                                     mut workSpace:
                                                         *mut libc::c_void,
                                                     mut wkspSize: size_t)
 -> size_t {
    let mut ip: *const BYTE = cSrc as *const BYTE;
    let hSize: size_t =
        HUF_readDTableX1_wksp(DCtx, cSrc, cSrcSize, workSpace, wkspSize);
    if 0 != ERR_isError(hSize) {
        return hSize
    } else if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else {
        ip = ip.offset(hSize as isize);
        cSrcSize =
            (cSrcSize as libc::c_ulong).wrapping_sub(hSize) as size_t as
                size_t;
        return HUF_decompress1X1_usingDTable_internal(dst, dstSize,
                                                      ip as
                                                          *const libc::c_void,
                                                      cSrcSize, DCtx, 0i32)
    };
}
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal(mut dst:
                                                                *mut libc::c_void,
                                                            mut dstSize:
                                                                size_t,
                                                            mut cSrc:
                                                                *const libc::c_void,
                                                            mut cSrcSize:
                                                                size_t,
                                                            mut DTable:
                                                                *const HUF_DTable,
                                                            mut bmi2:
                                                                libc::c_int)
 -> size_t {
    if 0 != bmi2 {
        return HUF_decompress1X1_usingDTable_internal_bmi2(dst, dstSize, cSrc,
                                                           cSrcSize, DTable)
    } else {
        return HUF_decompress1X1_usingDTable_internal_default(dst, dstSize,
                                                              cSrc, cSrcSize,
                                                              DTable)
    };
}
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal_default(mut dst:
                                                                        *mut libc::c_void,
                                                                    mut dstSize:
                                                                        size_t,
                                                                    mut cSrc:
                                                                        *const libc::c_void,
                                                                    mut cSrcSize:
                                                                        size_t,
                                                                    mut DTable:
                                                                        *const HUF_DTable)
 -> size_t {
    return HUF_decompress1X1_usingDTable_internal_body(dst, dstSize, cSrc,
                                                       cSrcSize, DTable);
}
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal_body(mut dst:
                                                                     *mut libc::c_void,
                                                                 mut dstSize:
                                                                     size_t,
                                                                 mut cSrc:
                                                                     *const libc::c_void,
                                                                 mut cSrcSize:
                                                                     size_t,
                                                                 mut DTable:
                                                                     *const HUF_DTable)
 -> size_t {
    let mut op: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = op.offset(dstSize as isize);
    let mut dtPtr: *const libc::c_void =
        DTable.offset(1isize) as *const libc::c_void;
    let dt: *const HUF_DEltX1 = dtPtr as *const HUF_DEltX1;
    let mut bitD: BIT_DStream_t =
        BIT_DStream_t{bitContainer: 0,
                      bitsConsumed: 0,
                      ptr: 0 as *const libc::c_char,
                      start: 0 as *const libc::c_char,
                      limitPtr: 0 as *const libc::c_char,};
    let dtd: DTableDesc = HUF_getDTableDesc(DTable);
    let dtLog: U32 = dtd.tableLog as U32;
    let err_: size_t =
        BIT_initDStream(&mut bitD as *mut BIT_DStream_t, cSrc, cSrcSize);
    if 0 != ERR_isError(err_) {
        return err_
    } else {
        HUF_decodeStreamX1(op, &mut bitD as *mut BIT_DStream_t, oend, dt,
                           dtLog);
        if 0 == BIT_endOfDStream(&mut bitD as *mut BIT_DStream_t) {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        } else { return dstSize }
    };
}
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal_bmi2(mut dst:
                                                                     *mut libc::c_void,
                                                                 mut dstSize:
                                                                     size_t,
                                                                 mut cSrc:
                                                                     *const libc::c_void,
                                                                 mut cSrcSize:
                                                                     size_t,
                                                                 mut DTable:
                                                                     *const HUF_DTable)
 -> size_t {
    return HUF_decompress1X1_usingDTable_internal_body(dst, dstSize, cSrc,
                                                       cSrcSize, DTable);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X2(mut dst: *mut libc::c_void,
                                           mut dstSize: size_t,
                                           mut cSrc: *const libc::c_void,
                                           mut cSrcSize: size_t) -> size_t {
    let mut DTable: [HUF_DTable; 4097] =
        [(12i32 as U32).wrapping_mul(16777217i32 as libc::c_uint), 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    return HUF_decompress1X2_DCtx(DTable.as_mut_ptr(), dst, dstSize, cSrc,
                                  cSrcSize);
}
/* *< single-symbol decoder */
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X2_DCtx(mut DCtx: *mut HUF_DTable,
                                                mut dst: *mut libc::c_void,
                                                mut dstSize: size_t,
                                                mut cSrc: *const libc::c_void,
                                                mut cSrcSize: size_t)
 -> size_t {
    let mut workSpace: [U32; 512] = [0; 512];
    return HUF_decompress1X2_DCtx_wksp(DCtx, dst, dstSize, cSrc, cSrcSize,
                                       workSpace.as_mut_ptr() as
                                           *mut libc::c_void,
                                       ::std::mem::size_of::<[U32; 512]>() as
                                           libc::c_ulong);
}
/* *< double-symbols decoder */
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X2_DCtx_wksp(mut DCtx:
                                                         *mut HUF_DTable,
                                                     mut dst:
                                                         *mut libc::c_void,
                                                     mut dstSize: size_t,
                                                     mut cSrc:
                                                         *const libc::c_void,
                                                     mut cSrcSize: size_t,
                                                     mut workSpace:
                                                         *mut libc::c_void,
                                                     mut wkspSize: size_t)
 -> size_t {
    let mut ip: *const BYTE = cSrc as *const BYTE;
    let hSize: size_t =
        HUF_readDTableX2_wksp(DCtx, cSrc, cSrcSize, workSpace, wkspSize);
    if 0 != ERR_isError(hSize) {
        return hSize
    } else if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else {
        ip = ip.offset(hSize as isize);
        cSrcSize =
            (cSrcSize as libc::c_ulong).wrapping_sub(hSize) as size_t as
                size_t;
        return HUF_decompress1X2_usingDTable_internal(dst, dstSize,
                                                      ip as
                                                          *const libc::c_void,
                                                      cSrcSize, DCtx, 0i32)
    };
}
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal(mut dst:
                                                                *mut libc::c_void,
                                                            mut dstSize:
                                                                size_t,
                                                            mut cSrc:
                                                                *const libc::c_void,
                                                            mut cSrcSize:
                                                                size_t,
                                                            mut DTable:
                                                                *const HUF_DTable,
                                                            mut bmi2:
                                                                libc::c_int)
 -> size_t {
    if 0 != bmi2 {
        return HUF_decompress1X2_usingDTable_internal_bmi2(dst, dstSize, cSrc,
                                                           cSrcSize, DTable)
    } else {
        return HUF_decompress1X2_usingDTable_internal_default(dst, dstSize,
                                                              cSrc, cSrcSize,
                                                              DTable)
    };
}
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal_default(mut dst:
                                                                        *mut libc::c_void,
                                                                    mut dstSize:
                                                                        size_t,
                                                                    mut cSrc:
                                                                        *const libc::c_void,
                                                                    mut cSrcSize:
                                                                        size_t,
                                                                    mut DTable:
                                                                        *const HUF_DTable)
 -> size_t {
    return HUF_decompress1X2_usingDTable_internal_body(dst, dstSize, cSrc,
                                                       cSrcSize, DTable);
}
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal_body(mut dst:
                                                                     *mut libc::c_void,
                                                                 mut dstSize:
                                                                     size_t,
                                                                 mut cSrc:
                                                                     *const libc::c_void,
                                                                 mut cSrcSize:
                                                                     size_t,
                                                                 mut DTable:
                                                                     *const HUF_DTable)
 -> size_t {
    let mut bitD: BIT_DStream_t =
        BIT_DStream_t{bitContainer: 0,
                      bitsConsumed: 0,
                      ptr: 0 as *const libc::c_char,
                      start: 0 as *const libc::c_char,
                      limitPtr: 0 as *const libc::c_char,};
    let err_: size_t =
        BIT_initDStream(&mut bitD as *mut BIT_DStream_t, cSrc, cSrcSize);
    if 0 != ERR_isError(err_) {
        return err_
    } else {
        let ostart: *mut BYTE = dst as *mut BYTE;
        let oend: *mut BYTE = ostart.offset(dstSize as isize);
        let dtPtr: *const libc::c_void =
            DTable.offset(1isize) as *const libc::c_void;
        let dt: *const HUF_DEltX2 = dtPtr as *const HUF_DEltX2;
        let dtd: DTableDesc = HUF_getDTableDesc(DTable);
        HUF_decodeStreamX2(ostart, &mut bitD as *mut BIT_DStream_t, oend, dt,
                           dtd.tableLog as U32);
        if 0 == BIT_endOfDStream(&mut bitD as *mut BIT_DStream_t) {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        } else { return dstSize }
    };
}
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal_bmi2(mut dst:
                                                                     *mut libc::c_void,
                                                                 mut dstSize:
                                                                     size_t,
                                                                 mut cSrc:
                                                                     *const libc::c_void,
                                                                 mut cSrcSize:
                                                                     size_t,
                                                                 mut DTable:
                                                                     *const HUF_DTable)
 -> size_t {
    return HUF_decompress1X2_usingDTable_internal_body(dst, dstSize, cSrc,
                                                       cSrcSize, DTable);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X_DCtx(mut dctx: *mut HUF_DTable,
                                               mut dst: *mut libc::c_void,
                                               mut dstSize: size_t,
                                               mut cSrc: *const libc::c_void,
                                               mut cSrcSize: size_t)
 -> size_t {
    let mut workSpace: [U32; 512] = [0; 512];
    return HUF_decompress1X_DCtx_wksp(dctx, dst, dstSize, cSrc, cSrcSize,
                                      workSpace.as_mut_ptr() as
                                          *mut libc::c_void,
                                      ::std::mem::size_of::<[U32; 512]>() as
                                          libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X_DCtx_wksp(mut dctx: *mut HUF_DTable,
                                                    mut dst:
                                                        *mut libc::c_void,
                                                    mut dstSize: size_t,
                                                    mut cSrc:
                                                        *const libc::c_void,
                                                    mut cSrcSize: size_t,
                                                    mut workSpace:
                                                        *mut libc::c_void,
                                                    mut wkspSize: size_t)
 -> size_t {
    if dstSize == 0i32 as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else if cSrcSize > dstSize {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    } else if cSrcSize == dstSize {
        memcpy(dst, cSrc, dstSize);
        return dstSize
    } else if cSrcSize == 1i32 as libc::c_ulong {
        memset(dst, *(cSrc as *const BYTE) as libc::c_int, dstSize);
        return dstSize
    } else {
        let algoNb: U32 = HUF_selectDecoder(dstSize, cSrcSize);
        return if 0 != algoNb {
                   HUF_decompress1X2_DCtx_wksp(dctx, dst, dstSize, cSrc,
                                               cSrcSize, workSpace, wkspSize)
               } else {
                   HUF_decompress1X1_DCtx_wksp(dctx, dst, dstSize, cSrc,
                                               cSrcSize, workSpace, wkspSize)
               }
    };
}
/* *< double-symbols decoder */
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X_usingDTable(mut dst:
                                                          *mut libc::c_void,
                                                      mut maxDstSize: size_t,
                                                      mut cSrc:
                                                          *const libc::c_void,
                                                      mut cSrcSize: size_t,
                                                      mut DTable:
                                                          *const HUF_DTable)
 -> size_t {
    let dtd: DTableDesc = HUF_getDTableDesc(DTable);
    return if 0 != dtd.tableType as libc::c_int {
               HUF_decompress1X2_usingDTable_internal(dst, maxDstSize, cSrc,
                                                      cSrcSize, DTable, 0i32)
           } else {
               HUF_decompress1X1_usingDTable_internal(dst, maxDstSize, cSrc,
                                                      cSrcSize, DTable, 0i32)
           };
}
/* *< automatic selection of sing or double symbol decoder, based on DTable */
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X1_usingDTable(mut dst:
                                                           *mut libc::c_void,
                                                       mut dstSize: size_t,
                                                       mut cSrc:
                                                           *const libc::c_void,
                                                       mut cSrcSize: size_t,
                                                       mut DTable:
                                                           *const HUF_DTable)
 -> size_t {
    let mut dtd: DTableDesc = HUF_getDTableDesc(DTable);
    if dtd.tableType as libc::c_int != 0i32 {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        return HUF_decompress1X1_usingDTable_internal(dst, dstSize, cSrc,
                                                      cSrcSize, DTable, 0i32)
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X2_usingDTable(mut dst:
                                                           *mut libc::c_void,
                                                       mut dstSize: size_t,
                                                       mut cSrc:
                                                           *const libc::c_void,
                                                       mut cSrcSize: size_t,
                                                       mut DTable:
                                                           *const HUF_DTable)
 -> size_t {
    let mut dtd: DTableDesc = HUF_getDTableDesc(DTable);
    if dtd.tableType as libc::c_int != 1i32 {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        return HUF_decompress1X2_usingDTable_internal(dst, dstSize, cSrc,
                                                      cSrcSize, DTable, 0i32)
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X_usingDTable_bmi2(mut dst:
                                                               *mut libc::c_void,
                                                           mut maxDstSize:
                                                               size_t,
                                                           mut cSrc:
                                                               *const libc::c_void,
                                                           mut cSrcSize:
                                                               size_t,
                                                           mut DTable:
                                                               *const HUF_DTable,
                                                           mut bmi2:
                                                               libc::c_int)
 -> size_t {
    let dtd: DTableDesc = HUF_getDTableDesc(DTable);
    return if 0 != dtd.tableType as libc::c_int {
               HUF_decompress1X2_usingDTable_internal(dst, maxDstSize, cSrc,
                                                      cSrcSize, DTable, bmi2)
           } else {
               HUF_decompress1X1_usingDTable_internal(dst, maxDstSize, cSrc,
                                                      cSrcSize, DTable, bmi2)
           };
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress1X1_DCtx_wksp_bmi2(mut dctx:
                                                              *mut HUF_DTable,
                                                          mut dst:
                                                              *mut libc::c_void,
                                                          mut dstSize: size_t,
                                                          mut cSrc:
                                                              *const libc::c_void,
                                                          mut cSrcSize:
                                                              size_t,
                                                          mut workSpace:
                                                              *mut libc::c_void,
                                                          mut wkspSize:
                                                              size_t,
                                                          mut bmi2:
                                                              libc::c_int)
 -> size_t {
    let mut ip: *const BYTE = cSrc as *const BYTE;
    let hSize: size_t =
        HUF_readDTableX1_wksp(dctx, cSrc, cSrcSize, workSpace, wkspSize);
    if 0 != ERR_isError(hSize) {
        return hSize
    } else if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    } else {
        ip = ip.offset(hSize as isize);
        cSrcSize =
            (cSrcSize as libc::c_ulong).wrapping_sub(hSize) as size_t as
                size_t;
        return HUF_decompress1X1_usingDTable_internal(dst, dstSize,
                                                      ip as
                                                          *const libc::c_void,
                                                      cSrcSize, dctx, bmi2)
    };
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X_usingDTable_bmi2(mut dst:
                                                               *mut libc::c_void,
                                                           mut maxDstSize:
                                                               size_t,
                                                           mut cSrc:
                                                               *const libc::c_void,
                                                           mut cSrcSize:
                                                               size_t,
                                                           mut DTable:
                                                               *const HUF_DTable,
                                                           mut bmi2:
                                                               libc::c_int)
 -> size_t {
    let dtd: DTableDesc = HUF_getDTableDesc(DTable);
    return if 0 != dtd.tableType as libc::c_int {
               HUF_decompress4X2_usingDTable_internal(dst, maxDstSize, cSrc,
                                                      cSrcSize, DTable, bmi2)
           } else {
               HUF_decompress4X1_usingDTable_internal(dst, maxDstSize, cSrc,
                                                      cSrcSize, DTable, bmi2)
           };
}
#[no_mangle]
pub unsafe extern "C" fn HUF_decompress4X_hufOnly_wksp_bmi2(mut dctx:
                                                                *mut HUF_DTable,
                                                            mut dst:
                                                                *mut libc::c_void,
                                                            mut dstSize:
                                                                size_t,
                                                            mut cSrc:
                                                                *const libc::c_void,
                                                            mut cSrcSize:
                                                                size_t,
                                                            mut workSpace:
                                                                *mut libc::c_void,
                                                            mut wkspSize:
                                                                size_t,
                                                            mut bmi2:
                                                                libc::c_int)
 -> size_t {
    if dstSize == 0i32 as libc::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else if cSrcSize == 0i32 as libc::c_ulong {
        return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
    } else {
        let algoNb: U32 = HUF_selectDecoder(dstSize, cSrcSize);
        return if 0 != algoNb {
                   HUF_decompress4X2_DCtx_wksp_bmi2(dctx, dst, dstSize, cSrc,
                                                    cSrcSize, workSpace,
                                                    wkspSize, bmi2)
               } else {
                   HUF_decompress4X1_DCtx_wksp_bmi2(dctx, dst, dstSize, cSrc,
                                                    cSrcSize, workSpace,
                                                    wkspSize, bmi2)
               }
    };
}
