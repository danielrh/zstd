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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
    /* ! Constructor and Destructor of FSE_DTable.
    Note that its size depends on 'tableLog' */
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
    #[no_mangle]
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct fseWkspMax_t {
    pub CTable_max: [FSE_CTable; 2561],
    pub scratchBuffer: [BYTE; 4096],
}
pub type FSE_CTable = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
pub type uint32_t = libc::c_uint;
pub type unnamed = libc::c_uint;
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_DState_t {
    pub state: size_t,
    pub table: *const libc::c_void,
}
pub type unnamed_0 = libc::c_uint;
pub const MEM_static_assert: unnamed = 1;
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub type S16 = int16_t;
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
pub type U64 = uint64_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    u: U32,
    c: [BYTE; 4],
}
pub type BYTE = uint8_t;
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
pub type int16_t = libc::c_short;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
pub type FSE_DTable = libc::c_uint;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub const MEM_static_assert_0: unnamed_0 = 1;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: libc::c_uint,
    pub ptr: *const libc::c_char,
    pub start: *const libc::c_char,
    pub limitPtr: *const libc::c_char,
}
pub type uint64_t = libc::c_ulong;
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub type ptrdiff_t = libc::c_long;
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub type ZSTD_ErrorCode = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: libc::c_int,
    pub deltaNbBits: U32,
}
pub type uint8_t = libc::c_uchar;
pub type BIT_DStream_status = libc::c_uint;
pub type uint16_t = libc::c_ushort;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct BIT_CStream_t {
    pub bitContainer: size_t,
    pub bitPos: libc::c_uint,
    pub startPtr: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub endPtr: *mut libc::c_char,
}
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub type ERR_enum = ZSTD_ErrorCode;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
pub type U16 = uint16_t;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
pub type U32 = uint32_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const libc::c_void,
    pub symbolTT: *const libc::c_void,
    pub stateLog: libc::c_uint,
}
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
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
                    current_block = 15226767471340840255;
                }
                6 => { current_block = 15226767471340840255; }
                5 => { current_block = 15369305621768284676; }
                4 => { current_block = 334414897321631846; }
                3 => { current_block = 11443089290281171369; }
                2 => { current_block = 5416333213904981546; }
                _ => { current_block = 10886091980245723256; }
            }
            match current_block {
                15226767471340840255 => {
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
                    current_block = 15369305621768284676;
                }
                _ => { }
            }
            match current_block {
                15369305621768284676 => {
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
                    current_block = 334414897321631846;
                }
                _ => { }
            }
            match current_block {
                334414897321631846 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(3isize)
                                                              as size_t) <<
                                                             24i32) as size_t
                            as size_t;
                    current_block = 11443089290281171369;
                }
                _ => { }
            }
            match current_block {
                11443089290281171369 => {
                    (*bitD).bitContainer =
                        ((*bitD).bitContainer as
                             libc::c_ulong).wrapping_add((*(srcBuffer as
                                                                *const BYTE).offset(2isize)
                                                              as size_t) <<
                                                             16i32) as size_t
                            as size_t;
                    current_block = 5416333213904981546;
                }
                _ => { }
            }
            match current_block {
                5416333213904981546 => {
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
/* ! FSE_compress() :
    Compress content of buffer 'src', of size 'srcSize', into destination buffer 'dst'.
    'dst' buffer must be already allocated. Compression runs faster is dstCapacity >= FSE_compressBound(srcSize).
    @return : size of compressed data (<= dstCapacity).
    Special values : if return == 0, srcData is not compressible => Nothing is stored within dst !!!
                     if return == 1, srcData is a single byte symbol * srcSize times. Use RLE compression instead.
                     if FSE_isError(return), compression failed (more details using FSE_getErrorName())
*/
/* *< library version number; to be used when checking dll version */
#[no_mangle]
pub unsafe extern "C" fn FSE_compress(mut dst: *mut libc::c_void,
                                      mut dstCapacity: size_t,
                                      mut src: *const libc::c_void,
                                      mut srcSize: size_t) -> size_t {
    return FSE_compress2(dst, dstCapacity, src, srcSize,
                         255i32 as libc::c_uint,
                         (13i32 - 2i32) as libc::c_uint);
}
/* ! FSE_compress2() :
    Same as FSE_compress(), but allows the selection of 'maxSymbolValue' and 'tableLog'
    Both parameters can be defined as '0' to mean : use default value
    @return : size of compressed data
    Special values : if return == 0, srcData is not compressible => Nothing is stored within cSrc !!!
                     if return == 1, srcData is a single byte symbol * srcSize times. Use RLE compression.
                     if FSE_isError(return), it's an error code.
*/
#[no_mangle]
pub unsafe extern "C" fn FSE_compress2(mut dst: *mut libc::c_void,
                                       mut dstCapacity: size_t,
                                       mut src: *const libc::c_void,
                                       mut srcSize: size_t,
                                       mut maxSymbolValue: libc::c_uint,
                                       mut tableLog: libc::c_uint) -> size_t {
    let mut scratchBuffer: fseWkspMax_t =
        fseWkspMax_t{CTable_max: [0; 2561], scratchBuffer: [0; 4096],};
    if tableLog > (14i32 - 2i32) as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    } else {
        return FSE_compress_wksp(dst, dstCapacity, src, srcSize,
                                 maxSymbolValue, tableLog,
                                 &mut scratchBuffer as *mut fseWkspMax_t as
                                     *mut libc::c_void,
                                 ::std::mem::size_of::<fseWkspMax_t>() as
                                     libc::c_ulong)
    };
}
/* *< same as FSE_optimalTableLog(), which used `minus==2` */
#[no_mangle]
pub unsafe extern "C" fn FSE_compress_wksp(mut dst: *mut libc::c_void,
                                           mut dstSize: size_t,
                                           mut src: *const libc::c_void,
                                           mut srcSize: size_t,
                                           mut maxSymbolValue: libc::c_uint,
                                           mut tableLog: libc::c_uint,
                                           mut workSpace: *mut libc::c_void,
                                           mut wkspSize: size_t) -> size_t {
    let ostart: *mut BYTE = dst as *mut BYTE;
    let mut op: *mut BYTE = ostart;
    let oend: *mut BYTE = ostart.offset(dstSize as isize);
    let mut count: [U32; 256] = [0; 256];
    let mut norm: [S16; 256] = [0; 256];
    let mut CTable: *mut FSE_CTable = workSpace as *mut FSE_CTable;
    let CTableSize: size_t =
        ((1i32 + (1i32 << tableLog.wrapping_sub(1i32 as libc::c_uint))) as
             libc::c_uint).wrapping_add(maxSymbolValue.wrapping_add(1i32 as
                                                                        libc::c_uint).wrapping_mul(2i32
                                                                                                       as
                                                                                                       libc::c_uint))
            as size_t;
    let mut scratchBuffer: *mut libc::c_void =
        CTable.offset(CTableSize as isize) as *mut libc::c_void;
    let scratchBufferSize: size_t =
        wkspSize.wrapping_sub(CTableSize.wrapping_mul(::std::mem::size_of::<FSE_CTable>()
                                                          as libc::c_ulong));
    if wkspSize <
           ((1i32 + (1i32 << tableLog.wrapping_sub(1i32 as libc::c_uint))) as
                libc::c_uint).wrapping_add(maxSymbolValue.wrapping_add(1i32 as
                                                                           libc::c_uint).wrapping_mul(2i32
                                                                                                          as
                                                                                                          libc::c_uint)).wrapping_add((if tableLog
                                                                                                                                              >
                                                                                                                                              12i32
                                                                                                                                                  as
                                                                                                                                                  libc::c_uint
                                                                                                                                          {
                                                                                                                                           1i32
                                                                                                                                               <<
                                                                                                                                               tableLog.wrapping_sub(2i32
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_uint)
                                                                                                                                       } else {
                                                                                                                                           1024i32
                                                                                                                                       })
                                                                                                                                          as
                                                                                                                                          libc::c_uint)
               as libc::c_ulong {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    } else if srcSize <= 1i32 as libc::c_ulong {
        return 0i32 as size_t
    } else {
        if 0 == maxSymbolValue { maxSymbolValue = 255i32 as libc::c_uint }
        if 0 == tableLog { tableLog = (13i32 - 2i32) as libc::c_uint }
        let maxCount: size_t =
            HIST_count_wksp(count.as_mut_ptr(),
                            &mut maxSymbolValue as *mut libc::c_uint, src,
                            srcSize, scratchBuffer as *mut libc::c_uint);
        if 0 != ERR_isError(maxCount) {
            return maxCount
        } else if maxCount == srcSize {
            return 1i32 as size_t
        } else if maxCount == 1i32 as libc::c_ulong {
            return 0i32 as size_t
        } else if maxCount < srcSize >> 7i32 {
            return 0i32 as size_t
        } else {
            tableLog = FSE_optimalTableLog(tableLog, srcSize, maxSymbolValue);
            let _var_err__: size_t =
                FSE_normalizeCount(norm.as_mut_ptr(), tableLog,
                                   count.as_mut_ptr(), srcSize,
                                   maxSymbolValue);
            if 0 != ERR_isError(_var_err__) {
                return _var_err__
            } else {
                let nc_err: size_t =
                    FSE_writeNCount(op as *mut libc::c_void,
                                    op.offset_to(oend).expect("bad offset_to")
                                        as libc::c_long as size_t,
                                    norm.as_mut_ptr(), maxSymbolValue,
                                    tableLog);
                if 0 != ERR_isError(nc_err) {
                    return nc_err
                } else {
                    op = op.offset(nc_err as isize);
                    let _var_err___0: size_t =
                        FSE_buildCTable_wksp(CTable, norm.as_mut_ptr(),
                                             maxSymbolValue, tableLog,
                                             scratchBuffer,
                                             scratchBufferSize);
                    if 0 != ERR_isError(_var_err___0) {
                        return _var_err___0
                    } else {
                        let cSize: size_t =
                            FSE_compress_usingCTable(op as *mut libc::c_void,
                                                     op.offset_to(oend).expect("bad offset_to")
                                                         as libc::c_long as
                                                         size_t, src, srcSize,
                                                     CTable);
                        if 0 != ERR_isError(cSize) {
                            return cSize
                        } else if cSize == 0i32 as libc::c_ulong {
                            return 0i32 as size_t
                        } else {
                            op = op.offset(cSize as isize);
                            if ostart.offset_to(op).expect("bad offset_to") as
                                   libc::c_long as size_t >=
                                   srcSize.wrapping_sub(1i32 as libc::c_ulong)
                               {
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
    };
}
/* ! FSE_compress_usingCTable():
    Compress `src` using `ct` into `dst` which must be already allocated.
    @return : size of compressed data (<= `dstCapacity`),
              or 0 if compressed data could not fit into `dst`,
              or an errorCode, which can be tested using FSE_isError() */
#[no_mangle]
pub unsafe extern "C" fn FSE_compress_usingCTable(mut dst: *mut libc::c_void,
                                                  mut dstSize: size_t,
                                                  mut src:
                                                      *const libc::c_void,
                                                  mut srcSize: size_t,
                                                  mut ct: *const FSE_CTable)
 -> size_t {
    let fast: libc::c_uint =
        (dstSize >= srcSize.wrapping_add(srcSize >> 7i32)) as libc::c_int as
            libc::c_uint;
    if 0 != fast {
        return FSE_compress_usingCTable_generic(dst, dstSize, src, srcSize,
                                                ct, 1i32 as libc::c_uint)
    } else {
        return FSE_compress_usingCTable_generic(dst, dstSize, src, srcSize,
                                                ct, 0i32 as libc::c_uint)
    };
}
unsafe extern "C" fn FSE_compress_usingCTable_generic(mut dst:
                                                          *mut libc::c_void,
                                                      mut dstSize: size_t,
                                                      mut src:
                                                          *const libc::c_void,
                                                      mut srcSize: size_t,
                                                      mut ct:
                                                          *const FSE_CTable,
                                                      fast: libc::c_uint)
 -> size_t {
    let istart: *const BYTE = src as *const BYTE;
    let iend: *const BYTE = istart.offset(srcSize as isize);
    let mut ip: *const BYTE = iend;
    let mut bitC: BIT_CStream_t =
        BIT_CStream_t{bitContainer: 0,
                      bitPos: 0,
                      startPtr: 0 as *mut libc::c_char,
                      ptr: 0 as *mut libc::c_char,
                      endPtr: 0 as *mut libc::c_char,};
    let mut CState1: FSE_CState_t =
        FSE_CState_t{value: 0,
                     stateTable: 0 as *const libc::c_void,
                     symbolTT: 0 as *const libc::c_void,
                     stateLog: 0,};
    let mut CState2: FSE_CState_t =
        FSE_CState_t{value: 0,
                     stateTable: 0 as *const libc::c_void,
                     symbolTT: 0 as *const libc::c_void,
                     stateLog: 0,};
    if srcSize <= 2i32 as libc::c_ulong {
        return 0i32 as size_t
    } else {
        let initError: size_t =
            BIT_initCStream(&mut bitC as *mut BIT_CStream_t, dst, dstSize);
        if 0 != ERR_isError(initError) {
            return 0i32 as size_t
        } else {
            if 0 != srcSize & 1i32 as libc::c_ulong {
                ip = ip.offset(-1isize);
                FSE_initCState2(&mut CState1 as *mut FSE_CState_t, ct,
                                *ip as U32);
                ip = ip.offset(-1isize);
                FSE_initCState2(&mut CState2 as *mut FSE_CState_t, ct,
                                *ip as U32);
                ip = ip.offset(-1isize);
                FSE_encodeSymbol(&mut bitC as *mut BIT_CStream_t,
                                 &mut CState1 as *mut FSE_CState_t,
                                 *ip as libc::c_uint);
                if 0 != fast {
                    BIT_flushBitsFast(&mut bitC as *mut BIT_CStream_t);
                } else { BIT_flushBits(&mut bitC as *mut BIT_CStream_t); };
            } else {
                ip = ip.offset(-1isize);
                FSE_initCState2(&mut CState2 as *mut FSE_CState_t, ct,
                                *ip as U32);
                ip = ip.offset(-1isize);
                FSE_initCState2(&mut CState1 as *mut FSE_CState_t, ct,
                                *ip as U32);
            }
            srcSize =
                (srcSize as libc::c_ulong).wrapping_sub(2i32 as libc::c_ulong)
                    as size_t as size_t;
            if (::std::mem::size_of::<size_t>() as
                    libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) >
                   ((14i32 - 2i32) * 4i32 + 7i32) as libc::c_ulong &&
                   0 != srcSize & 2i32 as libc::c_ulong {
                ip = ip.offset(-1isize);
                FSE_encodeSymbol(&mut bitC as *mut BIT_CStream_t,
                                 &mut CState2 as *mut FSE_CState_t,
                                 *ip as libc::c_uint);
                ip = ip.offset(-1isize);
                FSE_encodeSymbol(&mut bitC as *mut BIT_CStream_t,
                                 &mut CState1 as *mut FSE_CState_t,
                                 *ip as libc::c_uint);
                if 0 != fast {
                    BIT_flushBitsFast(&mut bitC as *mut BIT_CStream_t);
                } else { BIT_flushBits(&mut bitC as *mut BIT_CStream_t); };
            }
            while ip > istart {
                ip = ip.offset(-1isize);
                FSE_encodeSymbol(&mut bitC as *mut BIT_CStream_t,
                                 &mut CState2 as *mut FSE_CState_t,
                                 *ip as libc::c_uint);
                if (::std::mem::size_of::<size_t>() as
                        libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) <
                       ((14i32 - 2i32) * 2i32 + 7i32) as libc::c_ulong {
                    if 0 != fast {
                        BIT_flushBitsFast(&mut bitC as *mut BIT_CStream_t);
                    } else {
                        BIT_flushBits(&mut bitC as *mut BIT_CStream_t);
                    };
                }
                ip = ip.offset(-1isize);
                FSE_encodeSymbol(&mut bitC as *mut BIT_CStream_t,
                                 &mut CState1 as *mut FSE_CState_t,
                                 *ip as libc::c_uint);
                if (::std::mem::size_of::<size_t>() as
                        libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) >
                       ((14i32 - 2i32) * 4i32 + 7i32) as libc::c_ulong {
                    ip = ip.offset(-1isize);
                    FSE_encodeSymbol(&mut bitC as *mut BIT_CStream_t,
                                     &mut CState2 as *mut FSE_CState_t,
                                     *ip as libc::c_uint);
                    ip = ip.offset(-1isize);
                    FSE_encodeSymbol(&mut bitC as *mut BIT_CStream_t,
                                     &mut CState1 as *mut FSE_CState_t,
                                     *ip as libc::c_uint);
                }
                if 0 != fast {
                    BIT_flushBitsFast(&mut bitC as *mut BIT_CStream_t);
                } else { BIT_flushBits(&mut bitC as *mut BIT_CStream_t); };
            }
            FSE_flushCState(&mut bitC as *mut BIT_CStream_t,
                            &mut CState2 as *mut FSE_CState_t);
            FSE_flushCState(&mut bitC as *mut BIT_CStream_t,
                            &mut CState1 as *mut FSE_CState_t);
            return BIT_closeCStream(&mut bitC as *mut BIT_CStream_t)
        }
    };
}
unsafe extern "C" fn FSE_flushCState(mut bitC: *mut BIT_CStream_t,
                                     mut statePtr: *const FSE_CState_t)
 -> () {
    BIT_addBits(bitC, (*statePtr).value as size_t, (*statePtr).stateLog);
    BIT_flushBits(bitC);
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
/* *< build a fake FSE_CTable, designed to compress always the same symbolValue */
#[no_mangle]
pub unsafe extern "C" fn FSE_buildCTable_wksp(mut ct: *mut FSE_CTable,
                                              mut normalizedCounter:
                                                  *const libc::c_short,
                                              mut maxSymbolValue:
                                                  libc::c_uint,
                                              mut tableLog: libc::c_uint,
                                              mut workSpace:
                                                  *mut libc::c_void,
                                              mut wkspSize: size_t)
 -> size_t {
    let tableSize: U32 = (1i32 << tableLog) as U32;
    let tableMask: U32 = tableSize.wrapping_sub(1i32 as libc::c_uint);
    let ptr: *mut libc::c_void = ct as *mut libc::c_void;
    let tableU16: *mut U16 = (ptr as *mut U16).offset(2isize);
    let FSCT: *mut libc::c_void =
        (ptr as
             *mut U32).offset(1isize).offset((if 0 != tableLog {
                                                  tableSize >> 1i32
                                              } else { 1i32 as libc::c_uint })
                                                 as isize) as
            *mut libc::c_void;
    let symbolTT: *mut FSE_symbolCompressionTransform =
        FSCT as *mut FSE_symbolCompressionTransform;
    let step: U32 =
        (tableSize >>
             1i32).wrapping_add(tableSize >>
                                    3i32).wrapping_add(3i32 as libc::c_uint);
    let mut cumul: [U32; 257] = [0; 257];
    let tableSymbol: *mut BYTE = workSpace as *mut BYTE;
    let mut highThreshold: U32 = tableSize.wrapping_sub(1i32 as libc::c_uint);
    if ((1i32 as size_t) <<
            tableLog).wrapping_mul(::std::mem::size_of::<BYTE>() as
                                       libc::c_ulong) > wkspSize {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    } else {
        *tableU16.offset(-2i32 as isize) = tableLog as U16;
        *tableU16.offset(-1i32 as isize) = maxSymbolValue as U16;
        let mut u: U32 = 0;
        cumul[0usize] = 0i32 as U32;
        u = 1i32 as U32;
        while u <= maxSymbolValue.wrapping_add(1i32 as libc::c_uint) {
            if *normalizedCounter.offset(u.wrapping_sub(1i32 as libc::c_uint)
                                             as isize) as libc::c_int == -1i32
               {
                cumul[u as usize] =
                    cumul[u.wrapping_sub(1i32 as libc::c_uint) as
                              usize].wrapping_add(1i32 as libc::c_uint);
                let fresh0 = highThreshold;
                highThreshold = highThreshold.wrapping_sub(1);
                *tableSymbol.offset(fresh0 as isize) =
                    u.wrapping_sub(1i32 as libc::c_uint) as BYTE
            } else {
                cumul[u as usize] =
                    cumul[u.wrapping_sub(1i32 as libc::c_uint) as
                              usize].wrapping_add(*normalizedCounter.offset(u.wrapping_sub(1i32
                                                                                               as
                                                                                               libc::c_uint)
                                                                                as
                                                                                isize)
                                                      as libc::c_uint)
            }
            u = u.wrapping_add(1)
        }
        cumul[maxSymbolValue.wrapping_add(1i32 as libc::c_uint) as usize] =
            tableSize.wrapping_add(1i32 as libc::c_uint);
        let mut position: U32 = 0i32 as U32;
        let mut symbol: U32 = 0;
        symbol = 0i32 as U32;
        while symbol <= maxSymbolValue {
            let mut nbOccurences: libc::c_int = 0;
            nbOccurences = 0i32;
            while nbOccurences <
                      *normalizedCounter.offset(symbol as isize) as
                          libc::c_int {
                *tableSymbol.offset(position as isize) = symbol as BYTE;
                position = position.wrapping_add(step) & tableMask;
                while position > highThreshold {
                    position = position.wrapping_add(step) & tableMask
                }
                nbOccurences += 1
            }
            symbol = symbol.wrapping_add(1)
        }
        if position != 0i32 as libc::c_uint {
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t
        } else {
            let mut u_0: U32 = 0;
            u_0 = 0i32 as U32;
            while u_0 < tableSize {
                let mut s: BYTE = *tableSymbol.offset(u_0 as isize);
                let fresh1 = cumul[s as usize];
                cumul[s as usize] = cumul[s as usize].wrapping_add(1);
                *tableU16.offset(fresh1 as isize) =
                    tableSize.wrapping_add(u_0) as U16;
                u_0 = u_0.wrapping_add(1)
            }
            let mut total: libc::c_uint = 0i32 as libc::c_uint;
            let mut s_0: libc::c_uint = 0;
            s_0 = 0i32 as libc::c_uint;
            while s_0 <= maxSymbolValue {
                match *normalizedCounter.offset(s_0 as isize) as libc::c_int {
                    0 => {
                        (*symbolTT.offset(s_0 as isize)).deltaNbBits =
                            (tableLog.wrapping_add(1i32 as libc::c_uint) <<
                                 16i32).wrapping_sub((1i32 << tableLog) as
                                                         libc::c_uint)
                    }
                    -1 | 1 => {
                        (*symbolTT.offset(s_0 as isize)).deltaNbBits =
                            (tableLog <<
                                 16i32).wrapping_sub((1i32 << tableLog) as
                                                         libc::c_uint);
                        (*symbolTT.offset(s_0 as isize)).deltaFindState =
                            total.wrapping_sub(1i32 as libc::c_uint) as
                                libc::c_int;
                        total = total.wrapping_add(1)
                    }
                    _ => {
                        let maxBitsOut: U32 =
                            tableLog.wrapping_sub(BIT_highbit32((*normalizedCounter.offset(s_0
                                                                                               as
                                                                                               isize)
                                                                     as
                                                                     libc::c_int
                                                                     - 1i32)
                                                                    as U32));
                        let minStatePlus: U32 =
                            ((*normalizedCounter.offset(s_0 as isize) as
                                  libc::c_int) << maxBitsOut) as U32;
                        (*symbolTT.offset(s_0 as isize)).deltaNbBits =
                            (maxBitsOut << 16i32).wrapping_sub(minStatePlus);
                        (*symbolTT.offset(s_0 as isize)).deltaFindState =
                            total.wrapping_sub(*normalizedCounter.offset(s_0
                                                                             as
                                                                             isize)
                                                   as libc::c_uint) as
                                libc::c_int;
                        total =
                            total.wrapping_add(*normalizedCounter.offset(s_0
                                                                             as
                                                                             isize)
                                                   as libc::c_uint)
                    }
                }
                s_0 = s_0.wrapping_add(1)
            }
            return 0i32 as size_t
        }
    };
}
/* ! FSE_writeNCount():
    Compactly save 'normalizedCounter' into 'buffer'.
    @return : size of the compressed table,
              or an errorCode, which can be tested using FSE_isError(). */
#[no_mangle]
pub unsafe extern "C" fn FSE_writeNCount(mut buffer: *mut libc::c_void,
                                         mut bufferSize: size_t,
                                         mut normalizedCounter:
                                             *const libc::c_short,
                                         mut maxSymbolValue: libc::c_uint,
                                         mut tableLog: libc::c_uint)
 -> size_t {
    if tableLog > (14i32 - 2i32) as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    } else if tableLog < 5i32 as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else if bufferSize < FSE_NCountWriteBound(maxSymbolValue, tableLog) {
        return FSE_writeNCount_generic(buffer, bufferSize, normalizedCounter,
                                       maxSymbolValue, tableLog,
                                       0i32 as libc::c_uint)
    } else {
        return FSE_writeNCount_generic(buffer, bufferSize, normalizedCounter,
                                       maxSymbolValue, tableLog,
                                       1i32 as libc::c_uint)
    };
}
unsafe extern "C" fn FSE_writeNCount_generic(mut header: *mut libc::c_void,
                                             mut headerBufferSize: size_t,
                                             mut normalizedCounter:
                                                 *const libc::c_short,
                                             mut maxSymbolValue: libc::c_uint,
                                             mut tableLog: libc::c_uint,
                                             mut writeIsSafe: libc::c_uint)
 -> size_t {
    let ostart: *mut BYTE = header as *mut BYTE;
    let mut out: *mut BYTE = ostart;
    let oend: *mut BYTE = ostart.offset(headerBufferSize as isize);
    let mut nbBits: libc::c_int = 0;
    let tableSize: libc::c_int = 1i32 << tableLog;
    let mut remaining: libc::c_int = 0;
    let mut threshold: libc::c_int = 0;
    let mut bitStream: U32 = 0;
    let mut bitCount: libc::c_int = 0;
    let mut charnum: libc::c_uint = 0i32 as libc::c_uint;
    let mut previous0: libc::c_int = 0i32;
    bitStream = 0i32 as U32;
    bitCount = 0i32;
    bitStream =
        (bitStream as
             libc::c_uint).wrapping_add(tableLog.wrapping_sub(5i32 as
                                                                  libc::c_uint)
                                            << bitCount) as U32 as U32;
    bitCount += 4i32;
    remaining = tableSize + 1i32;
    threshold = tableSize;
    nbBits = tableLog.wrapping_add(1i32 as libc::c_uint) as libc::c_int;
    while remaining > 1i32 {
        if 0 != previous0 {
            let mut start: libc::c_uint = charnum;
            while 0 == *normalizedCounter.offset(charnum as isize) {
                charnum = charnum.wrapping_add(1)
            }
            while charnum >= start.wrapping_add(24i32 as libc::c_uint) {
                start = start.wrapping_add(24i32 as libc::c_uint);
                bitStream =
                    (bitStream as
                         libc::c_uint).wrapping_add(65535u32 << bitCount) as
                        U32 as U32;
                if 0 == writeIsSafe && out > oend.offset(-2isize) {
                    return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as
                               size_t
                } else {
                    *out.offset(0isize) = bitStream as BYTE;
                    *out.offset(1isize) = (bitStream >> 8i32) as BYTE;
                    out = out.offset(2isize);
                    bitStream >>= 16i32
                }
            }
            while charnum >= start.wrapping_add(3i32 as libc::c_uint) {
                start = start.wrapping_add(3i32 as libc::c_uint);
                bitStream =
                    (bitStream as
                         libc::c_uint).wrapping_add((3i32 << bitCount) as
                                                        libc::c_uint) as U32
                        as U32;
                bitCount += 2i32
            }
            bitStream =
                (bitStream as
                     libc::c_uint).wrapping_add(charnum.wrapping_sub(start) <<
                                                    bitCount) as U32 as U32;
            bitCount += 2i32;
            if bitCount > 16i32 {
                if 0 == writeIsSafe && out > oend.offset(-2isize) {
                    return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as
                               size_t
                } else {
                    *out.offset(0isize) = bitStream as BYTE;
                    *out.offset(1isize) = (bitStream >> 8i32) as BYTE;
                    out = out.offset(2isize);
                    bitStream >>= 16i32;
                    bitCount -= 16i32
                }
            }
        }
        let fresh2 = charnum;
        charnum = charnum.wrapping_add(1);
        let mut count: libc::c_int =
            *normalizedCounter.offset(fresh2 as isize) as libc::c_int;
        let max: libc::c_int = 2i32 * threshold - 1i32 - remaining;
        remaining -= if count < 0i32 { -count } else { count };
        count += 1;
        if count >= threshold { count += max }
        bitStream =
            (bitStream as
                 libc::c_uint).wrapping_add((count << bitCount) as
                                                libc::c_uint) as U32 as U32;
        bitCount += nbBits;
        bitCount -= (count < max) as libc::c_int;
        previous0 = (count == 1i32) as libc::c_int;
        if remaining < 1i32 {
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t
        } else {
            while remaining < threshold { nbBits -= 1; threshold >>= 1i32 }
            if !(bitCount > 16i32) { continue ; }
            if 0 == writeIsSafe && out > oend.offset(-2isize) {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
            } else {
                *out.offset(0isize) = bitStream as BYTE;
                *out.offset(1isize) = (bitStream >> 8i32) as BYTE;
                out = out.offset(2isize);
                bitStream >>= 16i32;
                bitCount -= 16i32
            }
        }
    }
    if 0 == writeIsSafe && out > oend.offset(-2isize) {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    } else {
        *out.offset(0isize) = bitStream as BYTE;
        *out.offset(1isize) = (bitStream >> 8i32) as BYTE;
        out = out.offset(((bitCount + 7i32) / 8i32) as isize);
        if charnum > maxSymbolValue.wrapping_add(1i32 as libc::c_uint) {
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t
        } else {
            return ostart.offset_to(out).expect("bad offset_to") as
                       libc::c_long as size_t
        }
    };
}
/* ! FSE_NCountWriteBound():
    Provides the maximum possible size of an FSE normalized table, given 'maxSymbolValue' and 'tableLog'.
    Typically useful for allocation purpose. */
#[no_mangle]
pub unsafe extern "C" fn FSE_NCountWriteBound(mut maxSymbolValue:
                                                  libc::c_uint,
                                              mut tableLog: libc::c_uint)
 -> size_t {
    let maxHeaderSize: size_t =
        (maxSymbolValue.wrapping_add(1i32 as
                                         libc::c_uint).wrapping_mul(tableLog)
             >> 3i32).wrapping_add(3i32 as libc::c_uint) as size_t;
    return if 0 != maxSymbolValue {
               maxHeaderSize
           } else { 512i32 as libc::c_ulong };
}
/* ! FSE_normalizeCount():
    normalize counts so that sum(count[]) == Power_of_2 (2^tableLog)
    'normalizedCounter' is a table of short, of minimum size (maxSymbolValue+1).
    @return : tableLog,
              or an errorCode, which can be tested using FSE_isError() */
#[no_mangle]
pub unsafe extern "C" fn FSE_normalizeCount(mut normalizedCounter:
                                                *mut libc::c_short,
                                            mut tableLog: libc::c_uint,
                                            mut count: *const libc::c_uint,
                                            mut total: size_t,
                                            mut maxSymbolValue: libc::c_uint)
 -> size_t {
    let mut restToBeat: U64 = 0;
    if tableLog == 0i32 as libc::c_uint {
        tableLog = (13i32 - 2i32) as libc::c_uint
    }
    if tableLog < 5i32 as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else if tableLog > (14i32 - 2i32) as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    } else if tableLog < FSE_minTableLog(total, maxSymbolValue) {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        static mut rtbTable: [U32; 8] =
            unsafe {
                [0i32 as U32, 473195i32 as U32, 504333i32 as U32,
                 520860i32 as U32, 550000i32 as U32, 700000i32 as U32,
                 750000i32 as U32, 830000i32 as U32]
            };
        let scale: U64 =
            (62i32 as libc::c_uint).wrapping_sub(tableLog) as U64;
        let step: U64 = ((1i32 as U64) << 62i32).wrapping_div(total);
        let vStep: U64 =
            (1u64 << scale.wrapping_sub(20i32 as libc::c_ulong)) as U64;
        let mut stillToDistribute: libc::c_int = 1i32 << tableLog;
        let mut s: libc::c_uint = 0;
        let mut largest: libc::c_uint = 0i32 as libc::c_uint;
        let mut largestP: libc::c_short = 0i32 as libc::c_short;
        let mut lowThreshold: U32 = (total >> tableLog) as U32;
        s = 0i32 as libc::c_uint;
        while s <= maxSymbolValue {
            if *count.offset(s as isize) as libc::c_ulong == total {
                return 0i32 as size_t
            } else {
                if *count.offset(s as isize) == 0i32 as libc::c_uint {
                    *normalizedCounter.offset(s as isize) =
                        0i32 as libc::c_short
                } else if *count.offset(s as isize) <= lowThreshold {
                    *normalizedCounter.offset(s as isize) =
                        -1i32 as libc::c_short;
                    stillToDistribute -= 1
                } else {
                    let mut proba: libc::c_short =
                        ((*count.offset(s as isize) as
                              libc::c_ulong).wrapping_mul(step) >> scale) as
                            libc::c_short;
                    if (proba as libc::c_int) < 8i32 {
                        restToBeat =
                            vStep.wrapping_mul(rtbTable[proba as usize] as
                                                   libc::c_ulong);
                        proba =
                            (proba as libc::c_int +
                                 ((*count.offset(s as isize) as
                                       libc::c_ulong).wrapping_mul(step).wrapping_sub((proba
                                                                                           as
                                                                                           U64)
                                                                                          <<
                                                                                          scale)
                                      > restToBeat) as libc::c_int) as
                                libc::c_short
                    }
                    if proba as libc::c_int > largestP as libc::c_int {
                        largestP = proba;
                        largest = s
                    }
                    *normalizedCounter.offset(s as isize) = proba;
                    stillToDistribute -= proba as libc::c_int
                }
                s = s.wrapping_add(1)
            }
        }
        if -stillToDistribute >=
               *normalizedCounter.offset(largest as isize) as libc::c_int >>
                   1i32 {
            let errorCode: size_t =
                FSE_normalizeM2(normalizedCounter, tableLog, count, total,
                                maxSymbolValue);
            if 0 != ERR_isError(errorCode) { return errorCode }
        } else {
            let ref mut fresh3 = *normalizedCounter.offset(largest as isize);
            *fresh3 =
                (*fresh3 as libc::c_int +
                     stillToDistribute as libc::c_short as libc::c_int) as
                    libc::c_short
        }
        return tableLog as size_t
    };
}
unsafe extern "C" fn FSE_normalizeM2(mut norm: *mut libc::c_short,
                                     mut tableLog: U32,
                                     mut count: *const libc::c_uint,
                                     mut total: size_t,
                                     mut maxSymbolValue: U32) -> size_t {
    let NOT_YET_ASSIGNED: libc::c_short = -2i32 as libc::c_short;
    let mut s: U32 = 0;
    let mut distributed: U32 = 0i32 as U32;
    let mut ToDistribute: U32 = 0;
    let lowThreshold: U32 = (total >> tableLog) as U32;
    let mut lowOne: U32 =
        (total.wrapping_mul(3i32 as libc::c_ulong) >>
             tableLog.wrapping_add(1i32 as libc::c_uint)) as U32;
    s = 0i32 as U32;
    while s <= maxSymbolValue {
        if *count.offset(s as isize) == 0i32 as libc::c_uint {
            *norm.offset(s as isize) = 0i32 as libc::c_short
        } else if *count.offset(s as isize) <= lowThreshold {
            *norm.offset(s as isize) = -1i32 as libc::c_short;
            distributed = distributed.wrapping_add(1);
            total =
                (total as
                     libc::c_ulong).wrapping_sub(*count.offset(s as isize) as
                                                     libc::c_ulong) as size_t
                    as size_t
        } else if *count.offset(s as isize) <= lowOne {
            *norm.offset(s as isize) = 1i32 as libc::c_short;
            distributed = distributed.wrapping_add(1);
            total =
                (total as
                     libc::c_ulong).wrapping_sub(*count.offset(s as isize) as
                                                     libc::c_ulong) as size_t
                    as size_t
        } else { *norm.offset(s as isize) = NOT_YET_ASSIGNED }
        s = s.wrapping_add(1)
    }
    ToDistribute =
        ((1i32 << tableLog) as libc::c_uint).wrapping_sub(distributed);
    if total.wrapping_div(ToDistribute as libc::c_ulong) >
           lowOne as libc::c_ulong {
        lowOne =
            total.wrapping_mul(3i32 as
                                   libc::c_ulong).wrapping_div(ToDistribute.wrapping_mul(2i32
                                                                                             as
                                                                                             libc::c_uint)
                                                                   as
                                                                   libc::c_ulong)
                as U32;
        s = 0i32 as U32;
        while s <= maxSymbolValue {
            if *norm.offset(s as isize) as libc::c_int ==
                   NOT_YET_ASSIGNED as libc::c_int &&
                   *count.offset(s as isize) <= lowOne {
                *norm.offset(s as isize) = 1i32 as libc::c_short;
                distributed = distributed.wrapping_add(1);
                total =
                    (total as
                         libc::c_ulong).wrapping_sub(*count.offset(s as isize)
                                                         as libc::c_ulong) as
                        size_t as size_t
            }
            s = s.wrapping_add(1)
        }
        ToDistribute =
            ((1i32 << tableLog) as libc::c_uint).wrapping_sub(distributed)
    }
    if distributed == maxSymbolValue.wrapping_add(1i32 as libc::c_uint) {
        let mut maxV: U32 = 0i32 as U32;
        let mut maxC: U32 = 0i32 as U32;
        s = 0i32 as U32;
        while s <= maxSymbolValue {
            if *count.offset(s as isize) > maxC {
                maxV = s;
                maxC = *count.offset(s as isize)
            }
            s = s.wrapping_add(1)
        }
        let ref mut fresh4 = *norm.offset(maxV as isize);
        *fresh4 =
            (*fresh4 as libc::c_int +
                 ToDistribute as libc::c_short as libc::c_int) as
                libc::c_short;
        return 0i32 as size_t
    } else if total == 0i32 as libc::c_ulong {
        s = 0i32 as U32;
        while ToDistribute > 0i32 as libc::c_uint {
            if *norm.offset(s as isize) as libc::c_int > 0i32 {
                ToDistribute = ToDistribute.wrapping_sub(1);
                let ref mut fresh5 = *norm.offset(s as isize);
                *fresh5 += 1
            }
            s =
                s.wrapping_add(1i32 as
                                   libc::c_uint).wrapping_rem(maxSymbolValue.wrapping_add(1i32
                                                                                              as
                                                                                              libc::c_uint))
        }
        return 0i32 as size_t
    } else {
        let vStepLog: U64 =
            (62i32 as libc::c_uint).wrapping_sub(tableLog) as U64;
        let mid: U64 =
            (1u64 <<
                 vStepLog.wrapping_sub(1i32 as
                                           libc::c_ulong)).wrapping_sub(1i32
                                                                            as
                                                                            libc::c_ulonglong)
                as U64;
        let rStep: U64 =
            ((1i32 as U64) <<
                 vStepLog).wrapping_mul(ToDistribute as
                                            libc::c_ulong).wrapping_add(mid).wrapping_div(total);
        let mut tmpTotal: U64 = mid;
        s = 0i32 as U32;
        while s <= maxSymbolValue {
            if *norm.offset(s as isize) as libc::c_int ==
                   NOT_YET_ASSIGNED as libc::c_int {
                let end: U64 =
                    tmpTotal.wrapping_add((*count.offset(s as isize) as
                                               libc::c_ulong).wrapping_mul(rStep));
                let sStart: U32 = (tmpTotal >> vStepLog) as U32;
                let sEnd: U32 = (end >> vStepLog) as U32;
                let weight: U32 = sEnd.wrapping_sub(sStart);
                if weight < 1i32 as libc::c_uint {
                    return -(ZSTD_error_GENERIC as libc::c_int) as size_t
                } else {
                    *norm.offset(s as isize) = weight as libc::c_short;
                    tmpTotal = end
                }
            }
            s = s.wrapping_add(1)
        }
        return 0i32 as size_t
    };
}
unsafe extern "C" fn FSE_minTableLog(mut srcSize: size_t,
                                     mut maxSymbolValue: libc::c_uint)
 -> libc::c_uint {
    let mut minBitsSrc: U32 =
        BIT_highbit32(srcSize.wrapping_sub(1i32 as libc::c_ulong) as
                          U32).wrapping_add(1i32 as libc::c_uint);
    let mut minBitsSymbols: U32 =
        BIT_highbit32(maxSymbolValue).wrapping_add(2i32 as libc::c_uint);
    let mut minBits: U32 =
        if minBitsSrc < minBitsSymbols { minBitsSrc } else { minBitsSymbols };
    return minBits;
}
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
pub unsafe extern "C" fn FSE_optimalTableLog(mut maxTableLog: libc::c_uint,
                                             mut srcSize: size_t,
                                             mut maxSymbolValue: libc::c_uint)
 -> libc::c_uint {
    return FSE_optimalTableLog_internal(maxTableLog, srcSize, maxSymbolValue,
                                        2i32 as libc::c_uint);
}
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
pub unsafe extern "C" fn FSE_optimalTableLog_internal(mut maxTableLog:
                                                          libc::c_uint,
                                                      mut srcSize: size_t,
                                                      mut maxSymbolValue:
                                                          libc::c_uint,
                                                      mut minus: libc::c_uint)
 -> libc::c_uint {
    let mut maxBitsSrc: U32 =
        BIT_highbit32(srcSize.wrapping_sub(1i32 as libc::c_ulong) as
                          U32).wrapping_sub(minus);
    let mut tableLog: U32 = maxTableLog;
    let mut minBits: U32 = FSE_minTableLog(srcSize, maxSymbolValue);
    if tableLog == 0i32 as libc::c_uint { tableLog = (13i32 - 2i32) as U32 }
    if maxBitsSrc < tableLog { tableLog = maxBitsSrc }
    if minBits > tableLog { tableLog = minBits }
    if tableLog < 5i32 as libc::c_uint { tableLog = 5i32 as U32 }
    if tableLog > (14i32 - 2i32) as libc::c_uint {
        tableLog = (14i32 - 2i32) as U32
    }
    return tableLog;
}
#[no_mangle]
pub unsafe extern "C" fn FSE_compressBound(mut size: size_t) -> size_t {
    return (512i32 as
                libc::c_ulong).wrapping_add(size.wrapping_add(size >> 7i32));
}
#[no_mangle]
pub unsafe extern "C" fn FSE_createCTable(mut maxSymbolValue: libc::c_uint,
                                          mut tableLog: libc::c_uint)
 -> *mut FSE_CTable {
    let mut size: size_t = 0;
    if tableLog > 15i32 as libc::c_uint { tableLog = 15i32 as libc::c_uint }
    size =
        (((1i32 + (1i32 << tableLog.wrapping_sub(1i32 as libc::c_uint))) as
              libc::c_uint).wrapping_add(maxSymbolValue.wrapping_add(1i32 as
                                                                         libc::c_uint).wrapping_mul(2i32
                                                                                                        as
                                                                                                        libc::c_uint))
             as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<U32>() as
                                             libc::c_ulong);
    return malloc(size) as *mut FSE_CTable;
}
#[no_mangle]
pub unsafe extern "C" fn FSE_freeCTable(mut ct: *mut FSE_CTable) -> () {
    free(ct as *mut libc::c_void);
}
/* ! FSE_buildCTable():
    Builds `ct`, which must be already allocated, using FSE_createCTable().
    @return : 0, or an errorCode, which can be tested using FSE_isError() */
#[no_mangle]
pub unsafe extern "C" fn FSE_buildCTable(mut ct: *mut FSE_CTable,
                                         mut normalizedCounter:
                                             *const libc::c_short,
                                         mut maxSymbolValue: libc::c_uint,
                                         mut tableLog: libc::c_uint)
 -> size_t {
    let mut tableSymbol: [BYTE; 4096] = [0; 4096];
    return FSE_buildCTable_wksp(ct, normalizedCounter, maxSymbolValue,
                                tableLog,
                                tableSymbol.as_mut_ptr() as *mut libc::c_void,
                                ::std::mem::size_of::<[BYTE; 4096]>() as
                                    libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn FSE_buildCTable_raw(mut ct: *mut FSE_CTable,
                                             mut nbBits: libc::c_uint)
 -> size_t {
    let tableSize: libc::c_uint = (1i32 << nbBits) as libc::c_uint;
    let tableMask: libc::c_uint =
        tableSize.wrapping_sub(1i32 as libc::c_uint);
    let maxSymbolValue: libc::c_uint = tableMask;
    let ptr: *mut libc::c_void = ct as *mut libc::c_void;
    let tableU16: *mut U16 = (ptr as *mut U16).offset(2isize);
    let FSCT: *mut libc::c_void =
        (ptr as *mut U32).offset(1isize).offset((tableSize >> 1i32) as isize)
            as *mut libc::c_void;
    let symbolTT: *mut FSE_symbolCompressionTransform =
        FSCT as *mut FSE_symbolCompressionTransform;
    let mut s: libc::c_uint = 0;
    if nbBits < 1i32 as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    } else {
        *tableU16.offset(-2i32 as isize) = nbBits as U16;
        *tableU16.offset(-1i32 as isize) = maxSymbolValue as U16;
        s = 0i32 as libc::c_uint;
        while s < tableSize {
            *tableU16.offset(s as isize) = tableSize.wrapping_add(s) as U16;
            s = s.wrapping_add(1)
        }
        let deltaNbBits: U32 =
            (nbBits << 16i32).wrapping_sub((1i32 << nbBits) as libc::c_uint);
        s = 0i32 as libc::c_uint;
        while s <= maxSymbolValue {
            (*symbolTT.offset(s as isize)).deltaNbBits = deltaNbBits;
            (*symbolTT.offset(s as isize)).deltaFindState =
                s.wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
            s = s.wrapping_add(1)
        }
        return 0i32 as size_t
    };
}
/* *< build a fake FSE_CTable, designed for a flat distribution, where each symbol uses nbBits */
#[no_mangle]
pub unsafe extern "C" fn FSE_buildCTable_rle(mut ct: *mut FSE_CTable,
                                             mut symbolValue: BYTE)
 -> size_t {
    let mut ptr: *mut libc::c_void = ct as *mut libc::c_void;
    let mut tableU16: *mut U16 = (ptr as *mut U16).offset(2isize);
    let mut FSCTptr: *mut libc::c_void =
        (ptr as *mut U32).offset(2isize) as *mut libc::c_void;
    let mut symbolTT: *mut FSE_symbolCompressionTransform =
        FSCTptr as *mut FSE_symbolCompressionTransform;
    *tableU16.offset(-2i32 as isize) = 0i32 as U16;
    *tableU16.offset(-1i32 as isize) = symbolValue as U16;
    *tableU16.offset(0isize) = 0i32 as U16;
    *tableU16.offset(1isize) = 0i32 as U16;
    (*symbolTT.offset(symbolValue as isize)).deltaNbBits = 0i32 as U32;
    (*symbolTT.offset(symbolValue as isize)).deltaFindState = 0i32;
    return 0i32 as size_t;
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
/* ! FSE_sizeof_CTable() :
    FSE_CTable is a variable size structure which contains :
    `U16 tableLog;`
    `U16 maxSymbolValue;`
    `U16 nextStateNumber[1 << tableLog];`                         // This size is variable
    `FSE_symbolCompressionTransform symbolTT[maxSymbolValue+1];`  // This size is variable
Allocation is manual (C standard does not support variable-size structures).
*/
#[no_mangle]
pub unsafe extern "C" fn FSE_sizeof_CTable(mut maxSymbolValue: libc::c_uint,
                                           mut tableLog: libc::c_uint)
 -> size_t {
    if tableLog > (14i32 - 2i32) as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    } else {
        return (((1i32 +
                      (1i32 << tableLog.wrapping_sub(1i32 as libc::c_uint)))
                     as
                     libc::c_uint).wrapping_add(maxSymbolValue.wrapping_add(1i32
                                                                                as
                                                                                libc::c_uint).wrapping_mul(2i32
                                                                                                               as
                                                                                                               libc::c_uint))
                    as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<U32>()
                                                    as libc::c_ulong)
    };
}
