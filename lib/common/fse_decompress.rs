#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(libc, ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
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
}
pub type size_t = libc::c_ulong;
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
/* Start with initCStream, providing the size of buffer to write into.
*  bitStream will never write outside of this buffer.
*  `dstCapacity` must be >= sizeof(bitD->bitContainer), otherwise @return will be an error code.
*
*  bits are first added to a local register.
*  Local register is size_t, hence 64-bits on 64-bits systems, or 32-bits on 32-bits systems.
*  Writing data into memory is an explicit operation, performed by the flushBits function.
*  Hence keep track how many bits are potentially stored into local register to avoid register overflow.
*  After a flushBits, a maximum of 7 bits might still be stored into local register.
*
*  Avoid storing elements of more than 24 bits if you want compatibility with 32-bits bitstream readers.
*
*  Last operation is to close the bitStream.
*  The function returns the final size of CStream in bytes.
*  If data couldn't fit into `dstBuffer`, it will return a 0 ( == not storable)
*/
/*-********************************************
*  bitStream decoding API (read backward)
**********************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BIT_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: libc::c_uint,
    pub ptr: *const libc::c_char,
    pub start: *const libc::c_char,
    pub limitPtr: *const libc::c_char,
}
pub type BIT_DStream_status = libc::c_uint;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
/* ! Constructor and Destructor of FSE_DTable.
    Note that its size depends on 'tableLog' */
/* don't allocate that. It's just a way to be more restrictive than void* */
pub type FSE_DTable = libc::c_uint;
pub type DTable_max_t = [FSE_DTable; 4097];
/* *<
These functions are inner components of FSE_compress_usingCTable().
They allow the creation of custom streams, mixing multiple tables and bit sources.

A key property to keep in mind is that encoding and decoding are done **in reverse direction**.
So the first symbol you will encode is the last you will decode, like a LIFO stack.

You will need a few variables to track your CStream. They are :

FSE_CTable    ct;         // Provided by FSE_buildCTable()
BIT_CStream_t bitStream;  // bitStream tracking structure
FSE_CState_t  state;      // State tracking structure (can have several)


The first thing to do is to init bitStream and state.
    size_t errorCode = BIT_initCStream(&bitStream, dstBuffer, maxDstSize);
    FSE_initCState(&state, ct);

Note that BIT_initCStream() can produce an error code, so its result should be tested, using FSE_isError();
You can then encode your input data, byte after byte.
FSE_encodeSymbol() outputs a maximum of 'tableLog' bits at a time.
Remember decoding will be done in reverse direction.
    FSE_encodeByte(&bitStream, &state, symbol);

At any time, you can also add any bit sequence.
Note : maximum allowed nbBits is 25, for compatibility with 32-bits decoders
    BIT_addBits(&bitStream, bitField, nbBits);

The above methods don't commit data to memory, they just store it into local register, for speed.
Local register size is 64-bits on 64-bits systems, 32-bits on 32-bits systems (size_t).
Writing data to memory is a manual operation, performed by the flushBits function.
    BIT_flushBits(&bitStream);

Your last FSE encoding operation shall be to flush your last state value(s).
    FSE_flushState(&bitStream, &state);

Finally, you must close the bitStream.
The function returns the size of CStream in bytes.
If data couldn't fit into dstBuffer, it will return a 0 ( == not compressible)
If there is an error, it returns an errorCode (which can be tested using FSE_isError()).
    size_t size = BIT_closeCStream(&bitStream);
*/
/* *****************************************
*  FSE symbol decompression API
*******************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FSE_DState_t {
    pub state: size_t,
    pub table: *const libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FSE_decode_t {
    pub newState: libc::c_ushort,
    pub symbol: libc::c_uchar,
    pub nbBits: libc::c_uchar,
}
/* ======    Decompression    ====== */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FSE_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
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
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> U32 {
    return (*(ptr as *const unalign32)).v;
}
unsafe extern "C" fn MEM_read64(mut ptr: *const libc::c_void) -> U64 {
    return (*(ptr as *const unalign64)).v;
}
/* MEM_FORCE_MEMORY_ACCESS */
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
}
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return (in_0 as libc::c_ulonglong).swap_bytes() as U64;
}
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const libc::c_void) -> U32 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read32(memPtr)
    } else { return MEM_swap32(MEM_read32(memPtr)) };
}
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const libc::c_void) -> U64 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read64(memPtr)
    } else { return MEM_swap64(MEM_read64(memPtr)) };
}
unsafe extern "C" fn MEM_readLEST(mut memPtr: *const libc::c_void) -> size_t {
    if 0 != MEM_32bits() {
        return MEM_readLE32(memPtr) as size_t
    } else { return MEM_readLE64(memPtr) };
}
/*-****************************************
*  Error codes handling
******************************************/
/* reported already defined on VS 2015 (Rich Geldreich) */
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as
               libc::c_int as libc::c_uint;
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
/* 1,2,4,8 would be better for bitmap combinations, but slows down performance a bit ... :( */
unsafe extern "C" fn BIT_initDStream(mut bitD: *mut BIT_DStream_t,
                                     mut srcBuffer: *const libc::c_void,
                                     mut srcSize: size_t) -> size_t {
    if srcSize < 1i32 as libc::c_ulong {
        memset(bitD as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<BIT_DStream_t>() as libc::c_ulong);
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    (*bitD).start = srcBuffer as *const libc::c_char;
    (*bitD).limitPtr =
        (*bitD).start.offset(::std::mem::size_of::<size_t>() as libc::c_ulong
                                 as isize);
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
        let mut current_block_20: u64;
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
                /* fall-through */
                current_block_20 = 8186394046759647133;
            }
            6 => { current_block_20 = 8186394046759647133; }
            5 => { current_block_20 = 16366008135885564359; }
            4 => { current_block_20 = 4178014515982532010; }
            3 => { current_block_20 = 5141172686473368009; }
            2 => { current_block_20 = 3174334154968278585; }
            _ => { current_block_20 = 14576567515993809846; }
        }
        match current_block_20 {
            8186394046759647133 => {
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
                /* fall-through */
                current_block_20 = 16366008135885564359;
            }
            _ => { }
        }
        match current_block_20 {
            16366008135885564359 => {
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
                /* fall-through */
                current_block_20 = 4178014515982532010;
            }
            _ => { }
        }
        match current_block_20 {
            4178014515982532010 => {
                (*bitD).bitContainer =
                    ((*bitD).bitContainer as
                         libc::c_ulong).wrapping_add((*(srcBuffer as
                                                            *const BYTE).offset(3isize)
                                                          as size_t) << 24i32)
                        as size_t as size_t;
                /* fall-through */
                current_block_20 = 5141172686473368009;
            }
            _ => { }
        }
        match current_block_20 {
            5141172686473368009 => {
                (*bitD).bitContainer =
                    ((*bitD).bitContainer as
                         libc::c_ulong).wrapping_add((*(srcBuffer as
                                                            *const BYTE).offset(2isize)
                                                          as size_t) << 16i32)
                        as size_t as size_t;
                /* fall-through */
                current_block_20 = 3174334154968278585;
            }
            _ => { }
        }
        match current_block_20 {
            3174334154968278585 => {
                (*bitD).bitContainer =
                    ((*bitD).bitContainer as
                         libc::c_ulong).wrapping_add((*(srcBuffer as
                                                            *const BYTE).offset(1isize)
                                                          as size_t) << 8i32)
                        as size_t as size_t
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
                     libc::c_uint).wrapping_sub(BIT_highbit32(lastByte_0 as
                                                                  U32))
            } else { 0i32 as libc::c_uint };
        if lastByte_0 as libc::c_int == 0i32 {
            return -(ZSTD_error_corruption_detected as libc::c_int) as size_t
        }
        (*bitD).bitsConsumed =
            (*bitD).bitsConsumed.wrapping_add(((::std::mem::size_of::<size_t>()
                                                    as
                                                    libc::c_ulong).wrapping_sub(srcSize)
                                                   as
                                                   U32).wrapping_mul(8i32 as
                                                                         libc::c_uint))
    }
    return srcSize;
}
/* faster, but works only if nbBits >= 1 */
/*-**************************************************************
*  Internal functions
****************************************************************/
unsafe extern "C" fn BIT_highbit32(mut val: U32) -> libc::c_uint {
    return (31i32 - val.leading_zeros() as i32) as libc::c_uint;
}
unsafe extern "C" fn BIT_readBits(mut bitD: *mut BIT_DStream_t,
                                  mut nbBits: libc::c_uint) -> size_t {
    let value: size_t = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
/* ! BIT_lookBits() :
 *  Provides next n bits from local register.
 *  local register is not modified.
 *  On 32-bits, maxNbBits==24.
 *  On 64-bits, maxNbBits==56.
 * @return : value extracted */
unsafe extern "C" fn BIT_lookBits(mut bitD: *const BIT_DStream_t,
                                  mut nbBits: U32) -> size_t {
    return BIT_getMiddleBits((*bitD).bitContainer,
                             (::std::mem::size_of::<size_t>() as
                                  libc::c_ulong).wrapping_mul(8i32 as
                                                                  libc::c_ulong).wrapping_sub((*bitD).bitsConsumed
                                                                                                  as
                                                                                                  libc::c_ulong).wrapping_sub(nbBits
                                                                                                                                  as
                                                                                                                                  libc::c_ulong)
                                 as U32, nbBits);
}
unsafe extern "C" fn BIT_getMiddleBits(mut bitContainer: size_t, start: U32,
                                       nbBits: U32) -> size_t {
    let regMask: U32 =
        (::std::mem::size_of::<size_t>() as
             libc::c_ulong).wrapping_mul(8i32 as
                                             libc::c_ulong).wrapping_sub(1i32
                                                                             as
                                                                             libc::c_ulong)
            as U32;
    return bitContainer >> (start & regMask) &
               BIT_mask[nbBits as usize] as libc::c_ulong;
}
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t,
                                  mut nbBits: U32) {
    (*bitD).bitsConsumed = (*bitD).bitsConsumed.wrapping_add(nbBits);
}
unsafe extern "C" fn BIT_reloadDStream(mut bitD: *mut BIT_DStream_t)
 -> BIT_DStream_status {
    if (*bitD).bitsConsumed as libc::c_ulong >
           (::std::mem::size_of::<size_t>() as
                libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
        return BIT_DStream_overflow
    }
    if (*bitD).ptr >= (*bitD).limitPtr {
        (*bitD).ptr =
            (*bitD).ptr.offset(-(((*bitD).bitsConsumed >> 3i32) as isize));
        (*bitD).bitsConsumed &= 7i32 as libc::c_uint;
        (*bitD).bitContainer =
            MEM_readLEST((*bitD).ptr as *const libc::c_void);
        return BIT_DStream_unfinished
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as libc::c_ulong) <
               (::std::mem::size_of::<size_t>() as
                    libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
            return BIT_DStream_endOfBuffer
        }
        return BIT_DStream_completed
    }
    let mut nbBytes: U32 = (*bitD).bitsConsumed >> 3i32;
    let mut result: BIT_DStream_status = BIT_DStream_unfinished;
    if (*bitD).ptr.offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes =
            (*bitD).ptr.wrapping_offset_from((*bitD).start) as libc::c_long as
                U32;
        result = BIT_DStream_endOfBuffer
    }
    (*bitD).ptr = (*bitD).ptr.offset(-(nbBytes as isize));
    (*bitD).bitsConsumed =
        (*bitD).bitsConsumed.wrapping_sub(nbBytes.wrapping_mul(8i32 as
                                                                   libc::c_uint));
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const libc::c_void);
    return result;
}
/* unsafe version; does not check buffer overflow */
unsafe extern "C" fn BIT_readBitsFast(mut bitD: *mut BIT_DStream_t,
                                      mut nbBits: libc::c_uint) -> size_t {
    let value: size_t = BIT_lookBitsFast(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    return value;
}
/* ! BIT_lookBitsFast() :
 *  unsafe version; only works if nbBits >= 1 */
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
/* ! FSE_decompress():
    Decompress FSE data from buffer 'cSrc', of size 'cSrcSize',
    into already allocated destination buffer 'dst', of size 'dstCapacity'.
    @return : size of regenerated data (<= maxDstSize),
              or an error code, which can be tested using FSE_isError() .

    ** Important ** : FSE_decompress() does not decompress non-compressible nor RLE data !!!
    Why ? : making this distinction requires a header.
    Header management is intentionally delegated to the user layer, which can better manage special cases.
*/
#[no_mangle]
pub unsafe extern "C" fn FSE_decompress(mut dst: *mut libc::c_void,
                                        mut dstCapacity: size_t,
                                        mut cSrc: *const libc::c_void,
                                        mut cSrcSize: size_t) -> size_t {
    /* Static analyzer seems unable to understand this table will be properly initialized later */
    let mut dt: DTable_max_t = [0; 4097];
    return FSE_decompress_wksp(dst, dstCapacity, cSrc, cSrcSize,
                               dt.as_mut_ptr(),
                               (14i32 - 2i32) as libc::c_uint);
}
/* *< build a fake FSE_DTable, designed to always generate the same symbolValue */
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
    /* normal FSE decoding mode */
    let NCountLength: size_t =
        FSE_readNCount(counting.as_mut_ptr(), &mut maxSymbolValue,
                       &mut tableLog, istart as *const libc::c_void,
                       cSrcSize);
    if 0 != ERR_isError(NCountLength) { return NCountLength }
    if tableLog > maxLog {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    }
    ip = ip.offset(NCountLength as isize);
    cSrcSize =
        (cSrcSize as libc::c_ulong).wrapping_sub(NCountLength) as size_t as
            size_t;
    let e: size_t =
        FSE_buildDTable(workSpace, counting.as_mut_ptr(), maxSymbolValue,
                        tableLog);
    if 0 != ERR_isError(e) { return e }
    return FSE_decompress_usingDTable(dst, dstCapacity,
                                      ip as *const libc::c_void, cSrcSize,
                                      workSpace);
}
/* ! FSE_decompress_usingDTable():
    Decompress compressed source `cSrc` of size `cSrcSize` using `dt`
    into `dst` which must be already allocated.
    @return : size of regenerated data (necessarily <= `dstCapacity`),
              or an errorCode, which can be tested using FSE_isError() */
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
    }
    return FSE_decompress_usingDTable_generic(dst, originalSize, cSrc,
                                              cSrcSize, dt,
                                              0i32 as libc::c_uint);
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
    let e: size_t = BIT_initDStream(&mut bitD, cSrc, cSrcSize);
    if 0 != ERR_isError(e) { return e }
    FSE_initDState(&mut state1, &mut bitD, dt);
    FSE_initDState(&mut state2, &mut bitD, dt);
    while 0 !=
              (BIT_reloadDStream(&mut bitD) as libc::c_uint ==
                   BIT_DStream_unfinished as libc::c_int as libc::c_uint) as
                  libc::c_int & (op < olimit) as libc::c_int {
        *op.offset(0isize) =
            (if 0 != fast {
                 FSE_decodeSymbolFast(&mut state1, &mut bitD) as libc::c_int
             } else {
                 FSE_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
             }) as BYTE;
        if ((14i32 - 2i32) * 2i32 + 7i32) as libc::c_ulong >
               (::std::mem::size_of::<size_t>() as
                    libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
            BIT_reloadDStream(&mut bitD);
        }
        *op.offset(1isize) =
            (if 0 != fast {
                 FSE_decodeSymbolFast(&mut state2, &mut bitD) as libc::c_int
             } else {
                 FSE_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
             }) as BYTE;
        /* This test must be static */
        if ((14i32 - 2i32) * 4i32 + 7i32) as libc::c_ulong >
               (::std::mem::size_of::<size_t>() as
                    libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
            if BIT_reloadDStream(&mut bitD) as libc::c_uint >
                   BIT_DStream_unfinished as libc::c_int as libc::c_uint {
                op = op.offset(2isize);
                break ;
            }
        }
        *op.offset(2isize) =
            (if 0 != fast {
                 FSE_decodeSymbolFast(&mut state1, &mut bitD) as libc::c_int
             } else {
                 FSE_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
             }) as BYTE;
        if ((14i32 - 2i32) * 2i32 + 7i32) as libc::c_ulong >
               (::std::mem::size_of::<size_t>() as
                    libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) {
            BIT_reloadDStream(&mut bitD);
        }
        *op.offset(3isize) =
            (if 0 != fast {
                 FSE_decodeSymbolFast(&mut state2, &mut bitD) as libc::c_int
             } else {
                 FSE_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
             }) as BYTE;
        op = op.offset(4isize)
    }
    loop  {
        if op > omax.offset(-2isize) {
            return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
        }
        let fresh0 = op;
        op = op.offset(1);
        *fresh0 =
            (if 0 != fast {
                 FSE_decodeSymbolFast(&mut state1, &mut bitD) as libc::c_int
             } else {
                 FSE_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
             }) as BYTE;
        if BIT_reloadDStream(&mut bitD) as libc::c_uint ==
               BIT_DStream_overflow as libc::c_int as libc::c_uint {
            let fresh1 = op;
            op = op.offset(1);
            *fresh1 =
                (if 0 != fast {
                     FSE_decodeSymbolFast(&mut state2, &mut bitD) as
                         libc::c_int
                 } else {
                     FSE_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
                 }) as BYTE;
            break ;
        } else {
            if op > omax.offset(-2isize) {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
            }
            let fresh2 = op;
            op = op.offset(1);
            *fresh2 =
                (if 0 != fast {
                     FSE_decodeSymbolFast(&mut state2, &mut bitD) as
                         libc::c_int
                 } else {
                     FSE_decodeSymbol(&mut state2, &mut bitD) as libc::c_int
                 }) as BYTE;
            if !(BIT_reloadDStream(&mut bitD) as libc::c_uint ==
                     BIT_DStream_overflow as libc::c_int as libc::c_uint) {
                continue ;
            }
            let fresh3 = op;
            op = op.offset(1);
            *fresh3 =
                (if 0 != fast {
                     FSE_decodeSymbolFast(&mut state1, &mut bitD) as
                         libc::c_int
                 } else {
                     FSE_decodeSymbol(&mut state1, &mut bitD) as libc::c_int
                 }) as BYTE;
            break ;
        }
    }
    return op.wrapping_offset_from(ostart) as libc::c_long as size_t;
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
/* *****************************************
*  FSE unsafe API
*******************************************/
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
                                    mut dt: *const FSE_DTable) {
    let mut ptr: *const libc::c_void = dt as *const libc::c_void;
    let DTableH: *const FSE_DTableHeader = ptr as *const FSE_DTableHeader;
    (*DStatePtr).state =
        BIT_readBits(bitD, (*DTableH).tableLog as libc::c_uint);
    BIT_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1isize) as *const libc::c_void;
}
/* ! FSE_buildDTable():
    Builds 'dt', which must be already allocated, using FSE_createDTable().
    return : 0, or an errorCode, which can be tested using FSE_isError() */
#[no_mangle]
pub unsafe extern "C" fn FSE_buildDTable(mut dt: *mut FSE_DTable,
                                         mut normalizedCounter:
                                             *const libc::c_short,
                                         mut maxSymbolValue: libc::c_uint,
                                         mut tableLog: libc::c_uint)
 -> size_t {
    /* because *dt is unsigned, 32-bits aligned on 32-bits */
    let tdPtr: *mut libc::c_void = dt.offset(1isize) as *mut libc::c_void;
    let tableDecode: *mut FSE_decode_t = tdPtr as *mut FSE_decode_t;
    let mut symbolNext: [U16; 256] = [0; 256];
    let maxSV1: U32 = maxSymbolValue.wrapping_add(1i32 as libc::c_uint);
    let tableSize: U32 = (1i32 << tableLog) as U32;
    let mut highThreshold: U32 = tableSize.wrapping_sub(1i32 as libc::c_uint);
    if maxSymbolValue > 255i32 as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as size_t
    }
    if tableLog > (14i32 - 2i32) as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    }
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
                                    3i32).wrapping_add(3i32 as libc::c_uint);
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
    }
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
    return 0i32 as size_t;
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
pub unsafe extern "C" fn FSE_freeDTable(mut dt: *mut FSE_DTable) {
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
    }
    (*DTableH).tableLog = nbBits as U16;
    (*DTableH).fastMode = 1i32 as U16;
    s = 0i32 as libc::c_uint;
    while s < maxSV1 {
        (*dinfo.offset(s as isize)).newState = 0i32 as libc::c_ushort;
        (*dinfo.offset(s as isize)).symbol = s as BYTE;
        (*dinfo.offset(s as isize)).nbBits = nbBits as BYTE;
        s = s.wrapping_add(1)
    }
    return 0i32 as size_t;
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