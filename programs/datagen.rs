#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( extern_types , libc )]
extern crate libc;
extern "C" {
    pub type _IO_FILE_plus;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut opterr: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
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
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t,
              __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn perror(__s: *const libc::c_char) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
}
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign16 {
    pub v: U16,
}
pub type U64 = uint64_t;
pub type BYTE = uint8_t;
pub type _IO_lock_t = ();
pub type uint32_t = libc::c_uint;
pub type U32 = uint32_t;
pub type FILE = _IO_FILE;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    u: U32,
    c: [BYTE; 4],
}
pub const MEM_static_assert: unnamed_0 = 1;
pub type uint16_t = libc::c_ushort;
pub type __off64_t = libc::c_long;
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
pub struct unalign32 {
    pub v: U32,
}
pub type uint8_t = libc::c_uchar;
pub type uint64_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type unnamed_0 = libc::c_uint;
pub type U16 = uint16_t;
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalignArch {
    pub v: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C , packed )]
pub struct unalign64 {
    pub v: U64,
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
unsafe extern "C" fn RDG_rand(mut src: *mut U32) -> U32 {
    static mut prime1: U32 = unsafe { 2654435761u32 };
    static mut prime2: U32 = unsafe { 2246822519u32 };
    let mut rand32: U32 = *src;
    rand32 = (rand32 as libc::c_uint).wrapping_mul(prime1) as U32 as U32;
    rand32 ^= prime2;
    rand32 = rand32 << 13i32 | rand32 >> 32i32 - 13i32;
    *src = rand32;
    return rand32 >> 5i32;
}
unsafe extern "C" fn RDG_fillLiteralDistrib(mut ldt: *mut BYTE,
                                            mut ld: libc::c_double) -> () {
    let firstChar: BYTE =
        (if ld <= 0.0f64 { 0i32 } else { '(' as i32 }) as BYTE;
    let lastChar: BYTE =
        (if ld <= 0.0f64 { 255i32 } else { '}' as i32 }) as BYTE;
    let mut character: BYTE =
        (if ld <= 0.0f64 { 0i32 } else { '0' as i32 }) as BYTE;
    let mut u: U32 = 0;
    if ld <= 0.0f64 { ld = 0.0f64 }
    u = 0i32 as U32;
    while u < (1i32 << 13i32) as libc::c_uint {
        let weight: U32 =
            ((((1i32 << 13i32) as libc::c_uint).wrapping_sub(u) as
                  libc::c_double * ld) as
                 U32).wrapping_add(1i32 as libc::c_uint);
        let end: U32 =
            if u.wrapping_add(weight) < (1i32 << 13i32) as libc::c_uint {
                u.wrapping_add(weight)
            } else { (1i32 << 13i32) as libc::c_uint };
        while u < end {
            let fresh0 = u;
            u = u.wrapping_add(1);
            *ldt.offset(fresh0 as isize) = character
        }
        character = character.wrapping_add(1);
        if !(character as libc::c_int > lastChar as libc::c_int) {
            continue ;
        }
        character = firstChar
    };
}
unsafe extern "C" fn RDG_genChar(mut seed: *mut U32, mut ldt: *const BYTE)
 -> BYTE {
    let id: U32 = RDG_rand(seed) & ((1i32 << 13i32) - 1i32) as libc::c_uint;
    return *ldt.offset(id as isize);
}
unsafe extern "C" fn RDG_rand15Bits(mut seedPtr: *mut libc::c_uint) -> U32 {
    return RDG_rand(seedPtr) & 32767i32 as libc::c_uint;
}
unsafe extern "C" fn RDG_randLength(mut seedPtr: *mut libc::c_uint) -> U32 {
    if 0 != RDG_rand(seedPtr) & 7i32 as libc::c_uint {
        return RDG_rand(seedPtr) & 15i32 as libc::c_uint
    } else {
        return (RDG_rand(seedPtr) &
                    511i32 as
                        libc::c_uint).wrapping_add(15i32 as libc::c_uint)
    };
}
#[no_mangle]
pub unsafe extern "C" fn RDG_genBlock(mut buffer: *mut libc::c_void,
                                      mut buffSize: size_t,
                                      mut prefixSize: size_t,
                                      mut matchProba: libc::c_double,
                                      mut ldt: *const BYTE,
                                      mut seedPtr: *mut libc::c_uint) -> () {
    let buffPtr: *mut BYTE = buffer as *mut BYTE;
    let matchProba32: U32 = (32768i32 as libc::c_double * matchProba) as U32;
    let mut pos: size_t = prefixSize;
    let mut prevOffset: U32 = 1i32 as U32;
    while matchProba >= 1.0f64 {
        let mut size0: size_t =
            (RDG_rand(seedPtr) & 3i32 as libc::c_uint) as size_t;
        size0 =
            (1i32 as size_t) <<
                (16i32 as
                     libc::c_ulong).wrapping_add(size0.wrapping_mul(2i32 as
                                                                        libc::c_ulong));
        size0 =
            (size0 as
                 libc::c_ulong).wrapping_add(RDG_rand(seedPtr) as
                                                 libc::c_ulong &
                                                 size0.wrapping_sub(1i32 as
                                                                        libc::c_ulong))
                as size_t as size_t;
        if buffSize < pos.wrapping_add(size0) {
            memset(buffPtr.offset(pos as isize) as *mut libc::c_void, 0i32,
                   buffSize.wrapping_sub(pos));
            return
        } else {
            memset(buffPtr.offset(pos as isize) as *mut libc::c_void, 0i32,
                   size0);
            pos =
                (pos as libc::c_ulong).wrapping_add(size0) as size_t as
                    size_t;
            *buffPtr.offset(pos.wrapping_sub(1i32 as libc::c_ulong) as isize)
                = RDG_genChar(seedPtr, ldt)
        }
    }
    if pos == 0i32 as libc::c_ulong {
        *buffPtr.offset(0isize) = RDG_genChar(seedPtr, ldt);
        pos = 1i32 as size_t
    }
    while pos < buffSize {
        if RDG_rand15Bits(seedPtr) < matchProba32 {
            let length: U32 =
                RDG_randLength(seedPtr).wrapping_add(4i32 as libc::c_uint);
            let d: U32 =
                (if pos.wrapping_add(length as libc::c_ulong) < buffSize {
                     pos.wrapping_add(length as libc::c_ulong)
                 } else { buffSize }) as U32;
            let repeatOffset: U32 =
                (RDG_rand(seedPtr) & 15i32 as libc::c_uint ==
                     2i32 as libc::c_uint) as libc::c_int as U32;
            let randOffset: U32 =
                RDG_rand15Bits(seedPtr).wrapping_add(1i32 as libc::c_uint);
            let offset: U32 =
                if 0 != repeatOffset {
                    prevOffset
                } else {
                    (if (randOffset as libc::c_ulong) < pos {
                         randOffset as libc::c_ulong
                     } else { pos }) as U32
                };
            let mut match_0: size_t =
                pos.wrapping_sub(offset as libc::c_ulong);
            while pos < d as libc::c_ulong {
                let fresh2 = pos;
                pos = pos.wrapping_add(1);
                let fresh1 = match_0;
                match_0 = match_0.wrapping_add(1);
                *buffPtr.offset(fresh2 as isize) =
                    *buffPtr.offset(fresh1 as isize)
            }
            prevOffset = offset
        } else {
            let length_0: U32 = RDG_randLength(seedPtr);
            let d_0: U32 =
                (if pos.wrapping_add(length_0 as libc::c_ulong) < buffSize {
                     pos.wrapping_add(length_0 as libc::c_ulong)
                 } else { buffSize }) as U32;
            while pos < d_0 as libc::c_ulong {
                let fresh3 = pos;
                pos = pos.wrapping_add(1);
                *buffPtr.offset(fresh3 as isize) = RDG_genChar(seedPtr, ldt)
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn RDG_genBuffer(mut buffer: *mut libc::c_void,
                                       mut size: size_t,
                                       mut matchProba: libc::c_double,
                                       mut litProba: libc::c_double,
                                       mut seed: libc::c_uint) -> () {
    let mut ldt: [BYTE; 8192] = [0; 8192];
    memset(ldt.as_mut_ptr() as *mut libc::c_void, '0' as i32,
           ::std::mem::size_of::<[BYTE; 8192]>() as libc::c_ulong);
    if litProba <= 0.0f64 { litProba = matchProba / 4.5f64 }
    RDG_fillLiteralDistrib(ldt.as_mut_ptr(), litProba);
    RDG_genBlock(buffer, size, 0i32 as size_t, matchProba, ldt.as_mut_ptr(),
                 &mut seed as *mut libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn RDG_genStdout(mut size: libc::c_ulonglong,
                                       mut matchProba: libc::c_double,
                                       mut litProba: libc::c_double,
                                       mut seed: libc::c_uint) -> () {
    let stdBlockSize: size_t = (128i32 * (1i32 << 10i32)) as size_t;
    let stdDictSize: size_t = (32i32 * (1i32 << 10i32)) as size_t;
    let buff: *mut BYTE =
        malloc(stdDictSize.wrapping_add(stdBlockSize)) as *mut BYTE;
    let mut total: U64 = 0i32 as U64;
    let mut ldt: [BYTE; 8192] = [0; 8192];
    if buff.is_null() {
        perror((*::std::mem::transmute::<&[u8; 8],
                                         &mut [libc::c_char; 8]>(b"datagen\x00")).as_mut_ptr());
        exit(1i32);
    } else {
        if litProba <= 0.0f64 { litProba = matchProba / 4.5f64 }
        memset(ldt.as_mut_ptr() as *mut libc::c_void, '0' as i32,
               ::std::mem::size_of::<[BYTE; 8192]>() as libc::c_ulong);
        RDG_fillLiteralDistrib(ldt.as_mut_ptr(), litProba);
        RDG_genBlock(buff as *mut libc::c_void, stdDictSize, 0i32 as size_t,
                     matchProba, ldt.as_mut_ptr(),
                     &mut seed as *mut libc::c_uint);
        while (total as libc::c_ulonglong) < size {
            let genBlockSize: size_t =
                (if (stdBlockSize as libc::c_ulonglong) <
                        size.wrapping_sub(total as libc::c_ulonglong) {
                     stdBlockSize as libc::c_ulonglong
                 } else { size.wrapping_sub(total as libc::c_ulonglong) }) as
                    size_t;
            RDG_genBlock(buff as *mut libc::c_void,
                         stdDictSize.wrapping_add(stdBlockSize), stdDictSize,
                         matchProba, ldt.as_mut_ptr(),
                         &mut seed as *mut libc::c_uint);
            total =
                (total as libc::c_ulong).wrapping_add(genBlockSize) as U64 as
                    U64;
            let unused: size_t =
                fwrite(buff as *const libc::c_void, 1i32 as size_t,
                       genBlockSize, stdout);
            memcpy(buff as *mut libc::c_void,
                   buff.offset(stdBlockSize as isize) as *const libc::c_void,
                   stdDictSize);
        }
        free(buff as *mut libc::c_void);
        return;
    };
}
