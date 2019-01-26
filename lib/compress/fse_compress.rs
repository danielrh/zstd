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
    /* --- advanced histogram functions --- */
    /* * HIST_count_wksp() :
 *  Same as HIST_count(), but using an externally provided scratch buffer.
 *  Benefit is this function will use very little stack space.
 * `workSpace` is a writable buffer which must be 4-bytes aligned,
 * `workSpaceSize` must be >= HIST_WKSP_SIZE
 */
    #[no_mangle]
    fn HIST_count_wksp(count: *mut libc::c_uint,
                       maxSymbolValuePtr: *mut libc::c_uint,
                       src: *const libc::c_void, srcSize: size_t,
                       workSpace: *mut libc::c_void, workSpaceSize: size_t)
     -> size_t;
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
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
/* __pack instructions are safer, but compiler specific, hence potentially problematic for some compilers */
/* currently only defined for gcc and icc */
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct unalign16 {
    pub v: U16,
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
/* ******************************************************************
   bitstream
   Part of FSE library
   Copyright (C) 2013-present, Yann Collet.

   BSD 2-Clause License (http://www.opensource.org/licenses/bsd-license.php)

   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions are
   met:

       * Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.
       * Redistributions in binary form must reproduce the above
   copyright notice, this list of conditions and the following disclaimer
   in the documentation and/or other materials provided with the
   distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
   OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
   SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
   LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
   DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
   THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
   (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
   OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

   You can contact the author at :
   - Source repository : https://github.com/Cyan4973/FiniteStateEntropy
****************************************************************** */
/*
*  This API consists of small unitary functions, which must be inlined for best performance.
*  Since link-time-optimization is not available for all compilers,
*  these functions are defined into a .h to be included.
*/
/*-****************************************
*  Dependencies
******************************************/
/* unaligned access routines */
/* assert(), DEBUGLOG(), RAWLOG() */
/*=========================================
*  Target specific
=========================================*/
/*-******************************************
*  bitStream encoding API (write forward)
********************************************/
/* bitStream can mix input from multiple sources.
 * A critical property of these streams is that they encode and decode in **reverse** direction.
 * So the first bit sequence you add will be the last to be read, like a LIFO stack.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BIT_CStream_t {
    pub bitContainer: size_t,
    pub bitPos: libc::c_uint,
    pub startPtr: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub endPtr: *mut libc::c_char,
}
pub type unnamed_1 = libc::c_uint;
pub const MEM_static_assert: unnamed_1 = 1;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct fseWkspMax_t {
    pub CTable_max: [FSE_CTable; 2561],
    pub scratchBuffer: [BYTE; 4096],
}
/* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
/* don't allocate that. It's only meant to be more restrictive than void* */
pub type FSE_CTable = libc::c_uint;
/* *****************************************
*  FSE symbol compression API
*******************************************/
/*
   This API consists of small unitary functions, which highly benefit from being inlined.
   Hence their body are included in next section.
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const libc::c_void,
    pub symbolTT: *const libc::c_void,
    pub stateLog: libc::c_uint,
}
/* faster, but works only if nbBits is always >= 1 (otherwise, result will be corrupted) */
/* *****************************************
*  Implementation of inlined functions
*******************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: libc::c_int,
    pub deltaNbBits: U32,
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
unsafe extern "C" fn MEM_read16(mut ptr: *const libc::c_void) -> U16 {
    return (*(ptr as *const unalign16)).v;
}
unsafe extern "C" fn MEM_write32(mut memPtr: *mut libc::c_void,
                                 mut value: U32) {
    (*(memPtr as *mut unalign32)).v = value;
}
unsafe extern "C" fn MEM_write64(mut memPtr: *mut libc::c_void,
                                 mut value: U64) {
    (*(memPtr as *mut unalign64)).v = value;
}
/* MEM_FORCE_MEMORY_ACCESS */
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
}
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    return (in_0 as libc::c_ulonglong).swap_bytes() as U64;
}
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut libc::c_void,
                                   mut val32: U32) {
    if 0 != MEM_isLittleEndian() {
        MEM_write32(memPtr, val32);
    } else { MEM_write32(memPtr, MEM_swap32(val32)); };
}
unsafe extern "C" fn MEM_writeLE64(mut memPtr: *mut libc::c_void,
                                   mut val64: U64) {
    if 0 != MEM_isLittleEndian() {
        MEM_write64(memPtr, val64);
    } else { MEM_write64(memPtr, MEM_swap64(val64)); };
}
unsafe extern "C" fn MEM_writeLEST(mut memPtr: *mut libc::c_void,
                                   mut val: size_t) {
    if 0 != MEM_32bits() {
        MEM_writeLE32(memPtr, val as U32);
    } else { MEM_writeLE64(memPtr, val); };
}
/*-****************************************
*  Error codes handling
******************************************/
/* reported already defined on VS 2015 (Rich Geldreich) */
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as
               libc::c_int as libc::c_uint;
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
    }
    return 0i32 as size_t;
}
unsafe extern "C" fn BIT_addBits(mut bitC: *mut BIT_CStream_t,
                                 mut value: size_t,
                                 mut nbBits: libc::c_uint) {
    (*bitC).bitContainer |=
        (value & BIT_mask[nbBits as usize] as libc::c_ulong) <<
            (*bitC).bitPos;
    (*bitC).bitPos = (*bitC).bitPos.wrapping_add(nbBits);
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
unsafe extern "C" fn BIT_flushBits(mut bitC: *mut BIT_CStream_t) {
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
    if (*bitC).ptr >= (*bitC).endPtr { return 0i32 as size_t }
    return ((*bitC).ptr.wrapping_offset_from((*bitC).startPtr) as libc::c_long
                +
                ((*bitC).bitPos > 0i32 as libc::c_uint) as libc::c_int as
                    libc::c_long) as size_t;
}
/* Start by invoking BIT_initDStream().
*  A chunk of the bitStream is then stored into a local register.
*  Local register size is 64-bits on 64-bits systems, 32-bits on 32-bits systems (size_t).
*  You can then retrieve bitFields stored into the local register, **in reverse order**.
*  Local register is explicitly reloaded from memory by the BIT_reloadDStream() method.
*  A reload guarantee a minimum of ((8*sizeof(bitD->bitContainer))-7) bits when its result is BIT_DStream_unfinished.
*  Otherwise, it can be less than that, so proceed accordingly.
*  Checking if DStream has reached its end can be performed with BIT_endOfDStream().
*/
/*-****************************************
*  unsafe API
******************************************/
unsafe extern "C" fn BIT_addBitsFast(mut bitC: *mut BIT_CStream_t,
                                     mut value: size_t,
                                     mut nbBits: libc::c_uint) {
    (*bitC).bitContainer |= value << (*bitC).bitPos;
    (*bitC).bitPos = (*bitC).bitPos.wrapping_add(nbBits);
}
/* faster, but works only if nbBits >= 1 */
/*-**************************************************************
*  Internal functions
****************************************************************/
unsafe extern "C" fn BIT_highbit32(mut val: U32) -> libc::c_uint {
    return (31i32 - val.leading_zeros() as i32) as libc::c_uint;
}
/* faster, but works only if value is "clean", meaning all high bits above nbBits are 0 */
unsafe extern "C" fn BIT_flushBitsFast(mut bitC: *mut BIT_CStream_t) {
    let nbBytes: size_t = ((*bitC).bitPos >> 3i32) as size_t;
    MEM_writeLEST((*bitC).ptr as *mut libc::c_void, (*bitC).bitContainer);
    (*bitC).ptr = (*bitC).ptr.offset(nbBytes as isize);
    (*bitC).bitPos &= 7i32 as libc::c_uint;
    (*bitC).bitContainer >>= nbBytes.wrapping_mul(8i32 as libc::c_ulong);
}
/*-****************************************
*  FSE simple functions
******************************************/
/* FSE_compress() :
    Compress content of buffer 'src', of size 'srcSize', into destination buffer 'dst'.
    'dst' buffer must be already allocated. Compression runs faster is dstCapacity >= FSE_compressBound(srcSize).
    @return : size of compressed data (<= dstCapacity).
    Special values : if return == 0, srcData is not compressible => Nothing is stored within dst !!!
                     if return == 1, srcData is a single byte symbol * srcSize times. Use RLE compression instead.
                     if FSE_isError(return), compression failed (more details using FSE_getErrorName())
*/
#[no_mangle]
pub unsafe extern "C" fn FSE_compress(mut dst: *mut libc::c_void,
                                      mut dstCapacity: size_t,
                                      mut src: *const libc::c_void,
                                      mut srcSize: size_t) -> size_t {
    return FSE_compress2(dst, dstCapacity, src, srcSize,
                         255i32 as libc::c_uint,
                         (13i32 - 2i32) as libc::c_uint);
}
/*-*****************************************
*  FSE advanced functions
******************************************/
/* FSE_compress2() :
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
    }
    return FSE_compress_wksp(dst, dstCapacity, src, srcSize, maxSymbolValue,
                             tableLog,
                             &mut scratchBuffer as *mut fseWkspMax_t as
                                 *mut libc::c_void,
                             ::std::mem::size_of::<fseWkspMax_t>() as
                                 libc::c_ulong);
}
/* *< same as FSE_optimalTableLog(), which used `minus==2` */
/* FSE_compress_wksp() :
 * Same as FSE_compress2(), but using an externally allocated scratch buffer (`workSpace`).
 * FSE_WKSP_SIZE_U32() provides the minimum size required for `workSpace` as a table of FSE_CTable.
 */
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
    let mut count: [libc::c_uint; 256] = [0; 256];
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
    }
    if srcSize <= 1i32 as libc::c_ulong { return 0i32 as size_t }
    if 0 == maxSymbolValue { maxSymbolValue = 255i32 as libc::c_uint }
    if 0 == tableLog { tableLog = (13i32 - 2i32) as libc::c_uint }
    let maxCount: size_t =
        HIST_count_wksp(count.as_mut_ptr(), &mut maxSymbolValue, src, srcSize,
                        scratchBuffer, scratchBufferSize);
    if 0 != ERR_isError(maxCount) { return maxCount }
    if maxCount == srcSize { return 1i32 as size_t }
    if maxCount == 1i32 as libc::c_ulong { return 0i32 as size_t }
    if maxCount < srcSize >> 7i32 { return 0i32 as size_t }
    tableLog = FSE_optimalTableLog(tableLog, srcSize, maxSymbolValue);
    let _var_err__: size_t =
        FSE_normalizeCount(norm.as_mut_ptr(), tableLog, count.as_mut_ptr(),
                           srcSize, maxSymbolValue);
    if 0 != ERR_isError(_var_err__) { return _var_err__ }
    let nc_err: size_t =
        FSE_writeNCount(op as *mut libc::c_void,
                        oend.wrapping_offset_from(op) as libc::c_long as
                            size_t, norm.as_mut_ptr(), maxSymbolValue,
                        tableLog);
    if 0 != ERR_isError(nc_err) { return nc_err }
    op = op.offset(nc_err as isize);
    let _var_err___0: size_t =
        FSE_buildCTable_wksp(CTable, norm.as_mut_ptr(), maxSymbolValue,
                             tableLog, scratchBuffer, scratchBufferSize);
    if 0 != ERR_isError(_var_err___0) { return _var_err___0 }
    let cSize: size_t =
        FSE_compress_usingCTable(op as *mut libc::c_void,
                                 oend.wrapping_offset_from(op) as libc::c_long
                                     as size_t, src, srcSize, CTable);
    if 0 != ERR_isError(cSize) { return cSize }
    if cSize == 0i32 as libc::c_ulong { return 0i32 as size_t }
    op = op.offset(cSize as isize);
    if op.wrapping_offset_from(ostart) as libc::c_long as size_t >=
           srcSize.wrapping_sub(1i32 as libc::c_ulong) {
        return 0i32 as size_t
    }
    return op.wrapping_offset_from(ostart) as libc::c_long as size_t;
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
    if srcSize <= 2i32 as libc::c_ulong { return 0i32 as size_t }
    let initError: size_t = BIT_initCStream(&mut bitC, dst, dstSize);
    if 0 != ERR_isError(initError) { return 0i32 as size_t }
    if 0 != srcSize & 1i32 as libc::c_ulong {
        ip = ip.offset(-1isize);
        FSE_initCState2(&mut CState1, ct, *ip as U32);
        ip = ip.offset(-1isize);
        FSE_initCState2(&mut CState2, ct, *ip as U32);
        ip = ip.offset(-1isize);
        FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as libc::c_uint);
        if 0 != fast {
            BIT_flushBitsFast(&mut bitC);
        } else { BIT_flushBits(&mut bitC); };
    } else {
        ip = ip.offset(-1isize);
        FSE_initCState2(&mut CState2, ct, *ip as U32);
        ip = ip.offset(-1isize);
        FSE_initCState2(&mut CState1, ct, *ip as U32);
    }
    srcSize =
        (srcSize as libc::c_ulong).wrapping_sub(2i32 as libc::c_ulong) as
            size_t as size_t;
    if (::std::mem::size_of::<size_t>() as
            libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) >
           ((14i32 - 2i32) * 4i32 + 7i32) as libc::c_ulong &&
           0 != srcSize & 2i32 as libc::c_ulong {
        ip = ip.offset(-1isize);
        FSE_encodeSymbol(&mut bitC, &mut CState2, *ip as libc::c_uint);
        ip = ip.offset(-1isize);
        FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as libc::c_uint);
        if 0 != fast {
            BIT_flushBitsFast(&mut bitC);
        } else { BIT_flushBits(&mut bitC); };
    }
    while ip > istart {
        ip = ip.offset(-1isize);
        FSE_encodeSymbol(&mut bitC, &mut CState2, *ip as libc::c_uint);
        if (::std::mem::size_of::<size_t>() as
                libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) <
               ((14i32 - 2i32) * 2i32 + 7i32) as libc::c_ulong {
            if 0 != fast {
                BIT_flushBitsFast(&mut bitC);
            } else { BIT_flushBits(&mut bitC); };
        }
        ip = ip.offset(-1isize);
        FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as libc::c_uint);
        if (::std::mem::size_of::<size_t>() as
                libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) >
               ((14i32 - 2i32) * 4i32 + 7i32) as libc::c_ulong {
            ip = ip.offset(-1isize);
            FSE_encodeSymbol(&mut bitC, &mut CState2, *ip as libc::c_uint);
            ip = ip.offset(-1isize);
            FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as libc::c_uint);
        }
        if 0 != fast {
            BIT_flushBitsFast(&mut bitC);
        } else { BIT_flushBits(&mut bitC); };
    }
    FSE_flushCState(&mut bitC, &mut CState2);
    FSE_flushCState(&mut bitC, &mut CState1);
    return BIT_closeCStream(&mut bitC);
}
unsafe extern "C" fn FSE_flushCState(mut bitC: *mut BIT_CStream_t,
                                     mut statePtr: *const FSE_CState_t) {
    BIT_addBits(bitC, (*statePtr).value as size_t, (*statePtr).stateLog);
    BIT_flushBits(bitC);
}
unsafe extern "C" fn FSE_encodeSymbol(mut bitC: *mut BIT_CStream_t,
                                      mut statePtr: *mut FSE_CState_t,
                                      mut symbol: libc::c_uint) {
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
/* ! FSE_initCState2() :
*   Same as FSE_initCState(), but the first symbol to include (which will be the last to be read)
*   uses the smallest state value possible, saving the cost of this symbol */
unsafe extern "C" fn FSE_initCState2(mut statePtr: *mut FSE_CState_t,
                                     mut ct: *const FSE_CTable,
                                     mut symbol: U32) {
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
                                    mut ct: *const FSE_CTable) {
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
/* FSE_buildCTable_wksp() :
 * Same as FSE_buildCTable(), but using an externally allocated scratch buffer (`workSpace`).
 * `wkspSize` must be >= `(1<<tableLog)`.
 */
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
    /* header */
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
    }
    *tableU16.offset(-2i32 as isize) = tableLog as U16;
    *tableU16.offset(-1i32 as isize) = maxSymbolValue as U16;
    let mut u: U32 = 0;
    cumul[0usize] = 0i32 as U32;
    u = 1i32 as U32;
    while u <= maxSymbolValue.wrapping_add(1i32 as libc::c_uint) {
        if *normalizedCounter.offset(u.wrapping_sub(1i32 as libc::c_uint) as
                                         isize) as libc::c_int == -1i32 {
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
        let freq: libc::c_int =
            *normalizedCounter.offset(symbol as isize) as libc::c_int;
        nbOccurences = 0i32;
        while nbOccurences < freq {
            *tableSymbol.offset(position as isize) = symbol as BYTE;
            position = position.wrapping_add(step) & tableMask;
            while position > highThreshold {
                position = position.wrapping_add(step) & tableMask
            }
            nbOccurences += 1
        }
        symbol = symbol.wrapping_add(1)
    }
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
                    total.wrapping_sub(1i32 as libc::c_uint) as libc::c_int;
                total = total.wrapping_add(1)
            }
            _ => {
                let maxBitsOut: U32 =
                    tableLog.wrapping_sub(BIT_highbit32((*normalizedCounter.offset(s_0
                                                                                       as
                                                                                       isize)
                                                             as libc::c_int -
                                                             1i32) as U32));
                let minStatePlus: U32 =
                    ((*normalizedCounter.offset(s_0 as isize) as libc::c_int)
                         << maxBitsOut) as U32;
                (*symbolTT.offset(s_0 as isize)).deltaNbBits =
                    (maxBitsOut << 16i32).wrapping_sub(minStatePlus);
                (*symbolTT.offset(s_0 as isize)).deltaFindState =
                    total.wrapping_sub(*normalizedCounter.offset(s_0 as isize)
                                           as libc::c_uint) as libc::c_int;
                total =
                    total.wrapping_add(*normalizedCounter.offset(s_0 as isize)
                                           as libc::c_uint)
            }
        }
        s_0 = s_0.wrapping_add(1)
    }
    return 0i32 as size_t;
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
    }
    if tableLog < 5i32 as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if bufferSize < FSE_NCountWriteBound(maxSymbolValue, tableLog) {
        return FSE_writeNCount_generic(buffer, bufferSize, normalizedCounter,
                                       maxSymbolValue, tableLog,
                                       0i32 as libc::c_uint)
    }
    return FSE_writeNCount_generic(buffer, bufferSize, normalizedCounter,
                                   maxSymbolValue, tableLog,
                                   1i32 as libc::c_uint);
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
    let mut bitStream: U32 = 0i32 as U32;
    let mut bitCount: libc::c_int = 0i32;
    let mut symbol: libc::c_uint = 0i32 as libc::c_uint;
    let alphabetSize: libc::c_uint =
        maxSymbolValue.wrapping_add(1i32 as libc::c_uint);
    let mut previousIs0: libc::c_int = 0i32;
    bitStream =
        (bitStream as
             libc::c_uint).wrapping_add(tableLog.wrapping_sub(5i32 as
                                                                  libc::c_uint)
                                            << bitCount) as U32 as U32;
    bitCount += 4i32;
    remaining = tableSize + 1i32;
    threshold = tableSize;
    nbBits = tableLog.wrapping_add(1i32 as libc::c_uint) as libc::c_int;
    while symbol < alphabetSize && remaining > 1i32 {
        /* stops at 1 */
        if 0 != previousIs0 {
            let mut start: libc::c_uint = symbol;
            while symbol < alphabetSize &&
                      0 == *normalizedCounter.offset(symbol as isize) {
                symbol = symbol.wrapping_add(1)
            }
            if symbol == alphabetSize {
                /* incorrect distribution */
                break ;
            } else {
                while symbol >= start.wrapping_add(24i32 as libc::c_uint) {
                    start = start.wrapping_add(24i32 as libc::c_uint);
                    bitStream =
                        (bitStream as
                             libc::c_uint).wrapping_add(0xffffu32 << bitCount)
                            as U32 as U32;
                    if 0 == writeIsSafe && out > oend.offset(-2isize) {
                        return -(ZSTD_error_dstSize_tooSmall as libc::c_int)
                                   as size_t
                    }
                    *out.offset(0isize) = bitStream as BYTE;
                    *out.offset(1isize) = (bitStream >> 8i32) as BYTE;
                    out = out.offset(2isize);
                    bitStream >>= 16i32
                }
                while symbol >= start.wrapping_add(3i32 as libc::c_uint) {
                    start = start.wrapping_add(3i32 as libc::c_uint);
                    bitStream =
                        (bitStream as
                             libc::c_uint).wrapping_add((3i32 << bitCount) as
                                                            libc::c_uint) as
                            U32 as U32;
                    bitCount += 2i32
                }
                bitStream =
                    (bitStream as
                         libc::c_uint).wrapping_add(symbol.wrapping_sub(start)
                                                        << bitCount) as U32 as
                        U32;
                bitCount += 2i32;
                if bitCount > 16i32 {
                    if 0 == writeIsSafe && out > oend.offset(-2isize) {
                        return -(ZSTD_error_dstSize_tooSmall as libc::c_int)
                                   as size_t
                    }
                    *out.offset(0isize) = bitStream as BYTE;
                    *out.offset(1isize) = (bitStream >> 8i32) as BYTE;
                    out = out.offset(2isize);
                    bitStream >>= 16i32;
                    bitCount -= 16i32
                }
            }
        }
        let fresh2 = symbol;
        symbol = symbol.wrapping_add(1);
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
        previousIs0 = (count == 1i32) as libc::c_int;
        if remaining < 1i32 {
            return -(ZSTD_error_GENERIC as libc::c_int) as size_t
        }
        while remaining < threshold { nbBits -= 1; threshold >>= 1i32 }
        if bitCount > 16i32 {
            if 0 == writeIsSafe && out > oend.offset(-2isize) {
                return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
            }
            *out.offset(0isize) = bitStream as BYTE;
            *out.offset(1isize) = (bitStream >> 8i32) as BYTE;
            out = out.offset(2isize);
            bitStream >>= 16i32;
            bitCount -= 16i32
        }
    }
    if remaining != 1i32 {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if 0 == writeIsSafe && out > oend.offset(-2isize) {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    *out.offset(0isize) = bitStream as BYTE;
    *out.offset(1isize) = (bitStream >> 8i32) as BYTE;
    out = out.offset(((bitCount + 7i32) / 8i32) as isize);
    return out.wrapping_offset_from(ostart) as libc::c_long as size_t;
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
    if tableLog == 0i32 as libc::c_uint {
        tableLog = (13i32 - 2i32) as libc::c_uint
    }
    if tableLog < 5i32 as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if tableLog > (14i32 - 2i32) as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    }
    if tableLog < FSE_minTableLog(total, maxSymbolValue) {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    static mut rtbTable: [U32; 8] =
        [0i32 as U32, 473195i32 as U32, 504333i32 as U32, 520860i32 as U32,
         550000i32 as U32, 700000i32 as U32, 750000i32 as U32,
         830000i32 as U32];
    let scale: U64 = (62i32 as libc::c_uint).wrapping_sub(tableLog) as U64;
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
        }
        if *count.offset(s as isize) == 0i32 as libc::c_uint {
            *normalizedCounter.offset(s as isize) = 0i32 as libc::c_short
        } else if *count.offset(s as isize) <= lowThreshold {
            *normalizedCounter.offset(s as isize) = -1i32 as libc::c_short;
            stillToDistribute -= 1
        } else {
            let mut proba: libc::c_short =
                ((*count.offset(s as isize) as
                      libc::c_ulong).wrapping_mul(step) >> scale) as
                    libc::c_short;
            if (proba as libc::c_int) < 8i32 {
                let mut restToBeat: U64 =
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
                              > restToBeat) as libc::c_int) as libc::c_short
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
    if -stillToDistribute >=
           *normalizedCounter.offset(largest as isize) as libc::c_int >> 1i32
       {
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
    return tableLog as size_t;
}
/* Secondary normalization method.
   To be used when primary method fails. */
unsafe extern "C" fn FSE_normalizeM2(mut norm: *mut libc::c_short,
                                     mut tableLog: U32,
                                     mut count: *const libc::c_uint,
                                     mut total: size_t,
                                     mut maxSymbolValue: U32) -> size_t {
    let NOT_YET_ASSIGNED: libc::c_short = -2i32 as libc::c_short;
    let mut s: U32 = 0;
    let mut distributed: U32 = 0i32 as U32;
    let mut ToDistribute: U32 = 0;
    /* Init */
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
    if ToDistribute == 0i32 as libc::c_uint { return 0i32 as size_t }
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
    }
    if total == 0i32 as libc::c_ulong {
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
    }
    let vStepLog: U64 = (62i32 as libc::c_uint).wrapping_sub(tableLog) as U64;
    let mid: U64 =
        (1u64 <<
             vStepLog.wrapping_sub(1i32 as
                                       libc::c_ulong)).wrapping_sub(1i32 as
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
            }
            *norm.offset(s as isize) = weight as libc::c_short;
            tmpTotal = end
        }
        s = s.wrapping_add(1)
    }
    return 0i32 as size_t;
}
/* provides the minimum logSize to safely represent a distribution */
unsafe extern "C" fn FSE_minTableLog(mut srcSize: size_t,
                                     mut maxSymbolValue: libc::c_uint)
 -> libc::c_uint {
    let mut minBitsSrc: U32 =
        BIT_highbit32(srcSize as U32).wrapping_add(1i32 as libc::c_uint);
    let mut minBitsSymbols: U32 =
        BIT_highbit32(maxSymbolValue).wrapping_add(2i32 as libc::c_uint);
    let mut minBits: U32 =
        if minBitsSrc < minBitsSymbols { minBitsSrc } else { minBitsSymbols };
    return minBits;
}
/*-*****************************************
*  FSE detailed API
******************************************/
/*
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
/* *** COMPRESSION *** */
/* ! FSE_optimalTableLog():
    dynamically downsize 'tableLog' when conditions are met.
    It saves CPU time, by using smaller tables, while preserving or even improving compression ratio.
    @return : recommended tableLog (necessarily <= 'maxTableLog') */
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
/* FSE_H */
/* *** Dependency *** */
/* *****************************************
*  Static allocation
*******************************************/
/* FSE buffer bounds */
/* Macro version, useful for static allocation */
/* It is possible to statically allocate FSE CTable/DTable as a table of FSE_CTable/FSE_DTable using below macros */
/* or use the size to malloc() space directly. Pay attention to alignment restrictions though */
/* *****************************************
 *  FSE advanced API
 ***************************************** */
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
/*-*****************************************
*  Tool functions
******************************************/
/* maximum compressed size */
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
pub unsafe extern "C" fn FSE_freeCTable(mut ct: *mut FSE_CTable) {
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
    /* memset() is not necessary, even if static analyzer complain about it */
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
    /* header */
    /* assumption : tableLog >= 1 */
    let FSCT: *mut libc::c_void =
        (ptr as *mut U32).offset(1isize).offset((tableSize >> 1i32) as isize)
            as *mut libc::c_void;
    let symbolTT: *mut FSE_symbolCompressionTransform =
        FSCT as *mut FSE_symbolCompressionTransform;
    let mut s: libc::c_uint = 0;
    if nbBits < 1i32 as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
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
    return 0i32 as size_t;
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