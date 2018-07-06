#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( libc )]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
}
pub const ZSTD_error_checksum_wrong: ERR_enum = 22;
pub type uint64_t = libc::c_ulong;
pub type U64 = uint64_t;
pub const MEM_static_assert: unnamed = 1;
pub const ZSTD_error_memory_allocation: ERR_enum = 64;
pub const ZSTD_error_dstSize_tooSmall: ERR_enum = 70;
pub const ZSTD_error_workSpace_tooSmall: ERR_enum = 66;
pub const ZSTD_error_tableLog_tooLarge: ERR_enum = 44;
pub type U32 = uint32_t;
pub const ZSTD_error_parameter_unsupported: ERR_enum = 40;
pub const ZSTD_error_version_unsupported: ERR_enum = 12;
pub const ZSTD_error_seekableIO: ERR_enum = 102;
pub const ZSTD_error_dictionaryCreation_failed: ERR_enum = 34;
pub const ZSTD_error_dictionary_corrupted: ERR_enum = 30;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub type uint8_t = libc::c_uchar;
pub type BYTE = uint8_t;
pub type ZSTD_ErrorCode = ERR_enum;
pub const ZSTD_error_frameParameter_windowTooLarge: ERR_enum = 16;
pub const ZSTD_error_stage_wrong: ERR_enum = 60;
pub type U16 = uint16_t;
pub const ZSTD_error_no_error: ERR_enum = 0;
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
pub const ZSTD_error_maxSymbolValue_tooSmall: ERR_enum = 48;
pub const ZSTD_error_maxCode: ERR_enum = 120;
pub const ZSTD_error_prefix_unknown: ERR_enum = 10;
pub const ZSTD_error_parameter_outOfBound: ERR_enum = 42;
pub const ZSTD_error_maxSymbolValue_tooLarge: ERR_enum = 46;
pub const ZSTD_error_frameIndex_tooLarge: ERR_enum = 100;
pub const ZSTD_error_GENERIC: ERR_enum = 1;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign32 {
    pub v: U32,
}
pub const ZSTD_error_srcSize_wrong: ERR_enum = 72;
pub const ZSTD_error_corruption_detected: ERR_enum = 20;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
}
pub const ZSTD_error_frameParameter_unsupported: ERR_enum = 14;
pub type size_t = libc::c_ulong;
pub const ZSTD_error_dictionary_wrong: ERR_enum = 32;
pub type uint16_t = libc::c_ushort;
pub type ERR_enum = libc::c_uint;
pub type unnamed = libc::c_uint;
pub const ZSTD_error_init_missing: ERR_enum = 62;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    u: U32,
    c: [BYTE; 4],
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
/* ! HIST_count():
 *  Provides the precise count of each byte within a table 'count'.
 *  'count' is a table of unsigned int, of minimum size (*maxSymbolValuePtr+1).
 *  Updates *maxSymbolValuePtr with actual largest symbol value detected.
 *  @return : count of the most frequent symbol (which isn't identified).
 *            or an error code, which can be tested using HIST_isError().
 *            note : if return == srcSize, there is only one symbol.
 */
#[no_mangle]
pub unsafe extern "C" fn HIST_count(mut count: *mut libc::c_uint,
                                    mut maxSymbolValuePtr: *mut libc::c_uint,
                                    mut src: *const libc::c_void,
                                    mut srcSize: size_t) -> size_t {
    let mut tmpCounters: [libc::c_uint; 1024] = [0; 1024];
    return HIST_count_wksp(count, maxSymbolValuePtr, src, srcSize,
                           tmpCounters.as_mut_ptr());
}
/* * HIST_count_wksp() :
 *  Same as HIST_count(), but using an externally provided scratch buffer.
 *  Benefit is this function will use very little stack space.
 * `workSpace` must be a table of unsigned of size >= HIST_WKSP_SIZE_U32
 */
#[no_mangle]
pub unsafe extern "C" fn HIST_count_wksp(mut count: *mut libc::c_uint,
                                         mut maxSymbolValuePtr:
                                             *mut libc::c_uint,
                                         mut source: *const libc::c_void,
                                         mut sourceSize: size_t,
                                         mut workSpace: *mut libc::c_uint)
 -> size_t {
    if *maxSymbolValuePtr < 255i32 as libc::c_uint {
        return HIST_count_parallel_wksp(count, maxSymbolValuePtr, source,
                                        sourceSize, 1i32 as libc::c_uint,
                                        workSpace)
    } else {
        *maxSymbolValuePtr = 255i32 as libc::c_uint;
        return HIST_countFast_wksp(count, maxSymbolValuePtr, source,
                                   sourceSize, workSpace)
    };
}
/* * HIST_countFast_wksp() :
 *  Same as HIST_countFast(), but using an externally provided scratch buffer.
 * `workSpace` must be a table of unsigned of size >= HIST_WKSP_SIZE_U32
 */
#[no_mangle]
pub unsafe extern "C" fn HIST_countFast_wksp(mut count: *mut libc::c_uint,
                                             mut maxSymbolValuePtr:
                                                 *mut libc::c_uint,
                                             mut source: *const libc::c_void,
                                             mut sourceSize: size_t,
                                             mut workSpace: *mut libc::c_uint)
 -> size_t {
    if sourceSize < 1500i32 as libc::c_ulong {
        return HIST_count_simple(count, maxSymbolValuePtr, source, sourceSize)
                   as size_t
    } else {
        return HIST_count_parallel_wksp(count, maxSymbolValuePtr, source,
                                        sourceSize, 0i32 as libc::c_uint,
                                        workSpace)
    };
}
unsafe extern "C" fn HIST_count_parallel_wksp(mut count: *mut libc::c_uint,
                                              mut maxSymbolValuePtr:
                                                  *mut libc::c_uint,
                                              mut source: *const libc::c_void,
                                              mut sourceSize: size_t,
                                              mut checkMax: libc::c_uint,
                                              workSpace: *mut libc::c_uint)
 -> size_t {
    let mut c: U32 = 0;
    let mut ip: *const BYTE = source as *const BYTE;
    let iend: *const BYTE = ip.offset(sourceSize as isize);
    let mut maxSymbolValue: libc::c_uint = *maxSymbolValuePtr;
    let mut max: libc::c_uint = 0i32 as libc::c_uint;
    let Counting1: *mut U32 = workSpace;
    let Counting2: *mut U32 = Counting1.offset(256isize);
    let Counting3: *mut U32 = Counting2.offset(256isize);
    let Counting4: *mut U32 = Counting3.offset(256isize);
    memset(workSpace as *mut libc::c_void, 0i32,
           ((4i32 * 256i32) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                as libc::c_ulong));
    if 0 == sourceSize {
        memset(count as *mut libc::c_void, 0i32,
               maxSymbolValue.wrapping_add(1i32 as libc::c_uint) as
                   libc::c_ulong);
        *maxSymbolValuePtr = 0i32 as libc::c_uint;
        return 0i32 as size_t
    } else {
        if 0 == maxSymbolValue { maxSymbolValue = 255i32 as libc::c_uint }
        let mut cached: U32 = MEM_read32(ip as *const libc::c_void);
        ip = ip.offset(4isize);
        while ip < iend.offset(-15isize) {
            c = cached;
            cached = MEM_read32(ip as *const libc::c_void);
            ip = ip.offset(4isize);
            let ref mut fresh0 = *Counting1.offset(c as BYTE as isize);
            *fresh0 = (*fresh0).wrapping_add(1);
            let ref mut fresh1 =
                *Counting2.offset((c >> 8i32) as BYTE as isize);
            *fresh1 = (*fresh1).wrapping_add(1);
            let ref mut fresh2 =
                *Counting3.offset((c >> 16i32) as BYTE as isize);
            *fresh2 = (*fresh2).wrapping_add(1);
            let ref mut fresh3 = *Counting4.offset((c >> 24i32) as isize);
            *fresh3 = (*fresh3).wrapping_add(1);
            c = cached;
            cached = MEM_read32(ip as *const libc::c_void);
            ip = ip.offset(4isize);
            let ref mut fresh4 = *Counting1.offset(c as BYTE as isize);
            *fresh4 = (*fresh4).wrapping_add(1);
            let ref mut fresh5 =
                *Counting2.offset((c >> 8i32) as BYTE as isize);
            *fresh5 = (*fresh5).wrapping_add(1);
            let ref mut fresh6 =
                *Counting3.offset((c >> 16i32) as BYTE as isize);
            *fresh6 = (*fresh6).wrapping_add(1);
            let ref mut fresh7 = *Counting4.offset((c >> 24i32) as isize);
            *fresh7 = (*fresh7).wrapping_add(1);
            c = cached;
            cached = MEM_read32(ip as *const libc::c_void);
            ip = ip.offset(4isize);
            let ref mut fresh8 = *Counting1.offset(c as BYTE as isize);
            *fresh8 = (*fresh8).wrapping_add(1);
            let ref mut fresh9 =
                *Counting2.offset((c >> 8i32) as BYTE as isize);
            *fresh9 = (*fresh9).wrapping_add(1);
            let ref mut fresh10 =
                *Counting3.offset((c >> 16i32) as BYTE as isize);
            *fresh10 = (*fresh10).wrapping_add(1);
            let ref mut fresh11 = *Counting4.offset((c >> 24i32) as isize);
            *fresh11 = (*fresh11).wrapping_add(1);
            c = cached;
            cached = MEM_read32(ip as *const libc::c_void);
            ip = ip.offset(4isize);
            let ref mut fresh12 = *Counting1.offset(c as BYTE as isize);
            *fresh12 = (*fresh12).wrapping_add(1);
            let ref mut fresh13 =
                *Counting2.offset((c >> 8i32) as BYTE as isize);
            *fresh13 = (*fresh13).wrapping_add(1);
            let ref mut fresh14 =
                *Counting3.offset((c >> 16i32) as BYTE as isize);
            *fresh14 = (*fresh14).wrapping_add(1);
            let ref mut fresh15 = *Counting4.offset((c >> 24i32) as isize);
            *fresh15 = (*fresh15).wrapping_add(1)
        }
        ip = ip.offset(-4isize);
        while ip < iend {
            let fresh16 = ip;
            ip = ip.offset(1);
            let ref mut fresh17 = *Counting1.offset(*fresh16 as isize);
            *fresh17 = (*fresh17).wrapping_add(1)
        }
        if 0 != checkMax {
            let mut s: U32 = 0;
            s = 255i32 as U32;
            while s > maxSymbolValue {
                let ref mut fresh18 = *Counting1.offset(s as isize);
                *fresh18 =
                    (*fresh18 as
                         libc::c_uint).wrapping_add((*Counting2.offset(s as
                                                                           isize)).wrapping_add(*Counting3.offset(s
                                                                                                                      as
                                                                                                                      isize)).wrapping_add(*Counting4.offset(s
                                                                                                                                                                 as
                                                                                                                                                                 isize)))
                        as U32 as U32;
                if 0 != *Counting1.offset(s as isize) {
                    return -(ZSTD_error_maxSymbolValue_tooSmall as
                                 libc::c_int) as size_t
                } else { s = s.wrapping_sub(1) }
            }
        }
        let mut s_0: U32 = 0;
        if maxSymbolValue > 255i32 as libc::c_uint {
            maxSymbolValue = 255i32 as libc::c_uint
        }
        s_0 = 0i32 as U32;
        while s_0 <= maxSymbolValue {
            *count.offset(s_0 as isize) =
                (*Counting1.offset(s_0 as
                                       isize)).wrapping_add(*Counting2.offset(s_0
                                                                                  as
                                                                                  isize)).wrapping_add(*Counting3.offset(s_0
                                                                                                                             as
                                                                                                                             isize)).wrapping_add(*Counting4.offset(s_0
                                                                                                                                                                        as
                                                                                                                                                                        isize));
            if *count.offset(s_0 as isize) > max {
                max = *count.offset(s_0 as isize)
            }
            s_0 = s_0.wrapping_add(1)
        }
        while 0 == *count.offset(maxSymbolValue as isize) {
            maxSymbolValue = maxSymbolValue.wrapping_sub(1)
        }
        *maxSymbolValuePtr = maxSymbolValue;
        return max as size_t
    };
}
/* ! HIST_count_simple() :
 *  Same as HIST_countFast(), this function is unsafe,
 *  and will segfault if any value within `src` is `> *maxSymbolValuePtr`.
 *  It is also a bit slower for large inputs.
 *  However, it does not need any additional memory (not even on stack).
 * @return : count of the most frequent symbol.
 *  Note this function doesn't produce any error (i.e. it must succeed).
 */
#[no_mangle]
pub unsafe extern "C" fn HIST_count_simple(mut count: *mut libc::c_uint,
                                           mut maxSymbolValuePtr:
                                               *mut libc::c_uint,
                                           mut src: *const libc::c_void,
                                           mut srcSize: size_t)
 -> libc::c_uint {
    let mut ip: *const BYTE = src as *const BYTE;
    let end: *const BYTE = ip.offset(srcSize as isize);
    let mut maxSymbolValue: libc::c_uint = *maxSymbolValuePtr;
    let mut largestCount: libc::c_uint = 0i32 as libc::c_uint;
    memset(count as *mut libc::c_void, 0i32,
           (maxSymbolValue.wrapping_add(1i32 as libc::c_uint) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                as libc::c_ulong));
    if srcSize == 0i32 as libc::c_ulong {
        *maxSymbolValuePtr = 0i32 as libc::c_uint;
        return 0i32 as libc::c_uint
    } else {
        while ip < end {
            let fresh19 = ip;
            ip = ip.offset(1);
            let ref mut fresh20 = *count.offset(*fresh19 as isize);
            *fresh20 = (*fresh20).wrapping_add(1)
        }
        while 0 == *count.offset(maxSymbolValue as isize) {
            maxSymbolValue = maxSymbolValue.wrapping_sub(1)
        }
        *maxSymbolValuePtr = maxSymbolValue;
        let mut s: U32 = 0;
        s = 0i32 as U32;
        while s <= maxSymbolValue {
            if *count.offset(s as isize) > largestCount {
                largestCount = *count.offset(s as isize)
            }
            s = s.wrapping_add(1)
        }
        return largestCount
    };
}
#[no_mangle]
pub unsafe extern "C" fn HIST_isError(mut code: size_t) -> libc::c_uint {
    return ERR_isError(code);
}
/* * HIST_countFast() :
 *  same as HIST_count(), but blindly trusts that all byte values within src are <= *maxSymbolValuePtr.
 *  This function is unsafe, and will segfault if any value within `src` is `> *maxSymbolValuePtr`
 */
#[no_mangle]
pub unsafe extern "C" fn HIST_countFast(mut count: *mut libc::c_uint,
                                        mut maxSymbolValuePtr:
                                            *mut libc::c_uint,
                                        mut source: *const libc::c_void,
                                        mut sourceSize: size_t) -> size_t {
    let mut tmpCounters: [libc::c_uint; 1024] = [0; 1024];
    return HIST_countFast_wksp(count, maxSymbolValuePtr, source, sourceSize,
                               tmpCounters.as_mut_ptr());
}
