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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
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
    /*-*****************************************
*  FSE detailed API
******************************************/
/*!
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
    fn FSE_optimalTableLog_internal(maxTableLog: libc::c_uint,
                                    srcSize: size_t,
                                    maxSymbolValue: libc::c_uint,
                                    minus: libc::c_uint) -> libc::c_uint;
    /* *< build a fake FSE_CTable, designed to compress always the same symbolValue */
    /* FSE_buildCTable_wksp() :
 * Same as FSE_buildCTable(), but using an externally allocated scratch buffer (`workSpace`).
 * `wkspSize` must be >= `(1<<tableLog)`.
 */
    #[no_mangle]
    fn FSE_buildCTable_wksp(ct: *mut FSE_CTable,
                            normalizedCounter: *const libc::c_short,
                            maxSymbolValue: libc::c_uint,
                            tableLog: libc::c_uint,
                            workSpace: *mut libc::c_void, wkspSize: size_t)
     -> size_t;
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
/* ! Constructor and Destructor of FSE_CTable.
    Note that FSE_CTable size depends on 'tableLog' and 'maxSymbolValue' */
/* don't allocate that. It's only meant to be more restrictive than void* */
pub type FSE_CTable = libc::c_uint;
pub type HUF_repeat = libc::c_uint;
/* *< Can use the previous table and it is assumed to be valid */
pub const HUF_repeat_valid: HUF_repeat = 2;
/* *< Can use the previous table but it must be checked. Note : The previous table must have been constructed by HUF_compress{1, 4}X_repeat */
pub const HUF_repeat_check: HUF_repeat = 1;
/* *< Cannot use the previous table */
pub const HUF_repeat_none: HUF_repeat = 0;
/* incomplete type */
pub type HUF_CElt = HUF_CElt_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct HUF_CElt_s {
    pub val: U16,
    pub nbBits: BYTE,
}
pub type HUF_nbStreams_e = libc::c_uint;
pub const HUF_fourStreams: HUF_nbStreams_e = 1;
pub const HUF_singleStream: HUF_nbStreams_e = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct HUF_compress_tables_t {
    pub count: [libc::c_uint; 256],
    pub CTable: [HUF_CElt; 256],
    pub nodeTable: huffNodeTable,
}
/* * HUF_buildCTable_wksp() :
 *  Same as HUF_buildCTable(), but using externally allocated scratch buffer.
 *  `workSpace` must be aligned on 4-bytes boundaries, and be at least as large as a table of HUF_CTABLE_WORKSPACE_SIZE_U32 unsigned.
 */
pub type huffNodeTable = [nodeElt; 512];
pub type nodeElt = nodeElt_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct nodeElt_s {
    pub count: U32,
    pub parent: U16,
    pub byte: BYTE,
    pub nbBits: BYTE,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct rankPos {
    pub base: U32,
    pub current: U32,
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
unsafe extern "C" fn MEM_write16(mut memPtr: *mut libc::c_void,
                                 mut value: U16) {
    (*(memPtr as *mut unalign16)).v = value;
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
unsafe extern "C" fn MEM_writeLE16(mut memPtr: *mut libc::c_void,
                                   mut val: U16) {
    if 0 != MEM_isLittleEndian() {
        MEM_write16(memPtr, val);
    } else {
        let mut p: *mut BYTE = memPtr as *mut BYTE;
        *p.offset(0isize) = val as BYTE;
        *p.offset(1isize) = (val as libc::c_int >> 8i32) as BYTE
    };
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
/* ******************************************************************
   huff0 huffman codec,
   part of Finite State Entropy library
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
/* *** Dependencies *** */
/* size_t */
/* *** library symbols visibility *** */
/* Note : when linking with -fvisibility=hidden on gcc, or by default on Visual,
 *        HUF symbols remain "private" (internal symbols for library only).
 *        Set macro FSE_DLL_EXPORT to 1 if you want HUF symbols visible on DLL interface */
/* Visual expected */
/* ========================== */
/* ***  simple functions  *** */
/* ========================== */
/* * HUF_compress() :
 *  Compress content from buffer 'src', of size 'srcSize', into buffer 'dst'.
 * 'dst' buffer must be already allocated.
 *  Compression runs faster if `dstCapacity` >= HUF_compressBound(srcSize).
 * `srcSize` must be <= `HUF_BLOCKSIZE_MAX` == 128 KB.
 * @return : size of compressed data (<= `dstCapacity`).
 *  Special values : if return == 0, srcData is not compressible => Nothing is stored within dst !!!
 *                   if HUF_isError(return), compression failed (more details using HUF_getErrorName())
 */
#[no_mangle]
pub unsafe extern "C" fn HUF_compress(mut dst: *mut libc::c_void,
                                      mut maxDstSize: size_t,
                                      mut src: *const libc::c_void,
                                      mut srcSize: size_t) -> size_t {
    return HUF_compress2(dst, maxDstSize, src, srcSize,
                         255i32 as libc::c_uint, 11i32 as libc::c_uint);
}
/* ***   Advanced function   *** */
/* * HUF_compress2() :
 *  Same as HUF_compress(), but offers control over `maxSymbolValue` and `tableLog`.
 * `maxSymbolValue` must be <= HUF_SYMBOLVALUE_MAX .
 * `tableLog` must be `<= HUF_TABLELOG_MAX` . */
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
/* * HUF_compress4X_wksp() :
 *  Same as HUF_compress2(), but uses externally allocated `workSpace`.
 * `workspace` must have minimum alignment of 4, and be at least as large as HUF_WORKSPACE_SIZE */
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
                                 huffLog, HUF_fourStreams, workSpace,
                                 wkspSize, 0 as *mut HUF_CElt,
                                 0 as *mut HUF_repeat, 0i32, 0i32);
}
/* HUF_compress_internal() :
 * `workSpace` must a table of at least HUF_WORKSPACE_SIZE_U32 unsigned */
unsafe extern "C" fn HUF_compress_internal(mut dst: *mut libc::c_void,
                                           mut dstSize: size_t,
                                           mut src: *const libc::c_void,
                                           mut srcSize: size_t,
                                           mut maxSymbolValue: libc::c_uint,
                                           mut huffLog: libc::c_uint,
                                           mut nbStreams: HUF_nbStreams_e,
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
    }
    if wkspSize < (6i32 << 10i32) as libc::c_ulong {
        return -(ZSTD_error_workSpace_tooSmall as libc::c_int) as size_t
    }
    if 0 == srcSize { return 0i32 as size_t }
    if 0 == dstSize { return 0i32 as size_t }
    if srcSize > (128i32 * 1024i32) as libc::c_ulong {
        return -(ZSTD_error_srcSize_wrong as libc::c_int) as size_t
    }
    if huffLog > 12i32 as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    }
    if maxSymbolValue > 255i32 as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as size_t
    }
    if 0 == maxSymbolValue { maxSymbolValue = 255i32 as libc::c_uint }
    if 0 == huffLog { huffLog = 11i32 as libc::c_uint }
    if 0 != preferRepeat && !repeat.is_null() &&
           *repeat as libc::c_uint ==
               HUF_repeat_valid as libc::c_int as libc::c_uint {
        return HUF_compressCTable_internal(ostart, op, oend, src, srcSize,
                                           nbStreams, oldHufTable, bmi2)
    }
    let largest: size_t =
        HIST_count_wksp((*table).count.as_mut_ptr(), &mut maxSymbolValue,
                        src as *const BYTE as *const libc::c_void, srcSize,
                        workSpace, wkspSize);
    if 0 != ERR_isError(largest) { return largest }
    if largest == srcSize {
        *ostart = *(src as *const BYTE).offset(0isize);
        return 1i32 as size_t
    }
    if largest <= (srcSize >> 7i32).wrapping_add(4i32 as libc::c_ulong) {
        return 0i32 as size_t
    }
    if !repeat.is_null() &&
           *repeat as libc::c_uint ==
               HUF_repeat_check as libc::c_int as libc::c_uint &&
           0 ==
               HUF_validateCTable(oldHufTable, (*table).count.as_mut_ptr(),
                                  maxSymbolValue) {
        *repeat = HUF_repeat_none
    }
    if 0 != preferRepeat && !repeat.is_null() &&
           *repeat as libc::c_uint !=
               HUF_repeat_none as libc::c_int as libc::c_uint {
        return HUF_compressCTable_internal(ostart, op, oend, src, srcSize,
                                           nbStreams, oldHufTable, bmi2)
    }
    huffLog = HUF_optimalTableLog(huffLog, srcSize, maxSymbolValue);
    let maxBits: size_t =
        HUF_buildCTable_wksp((*table).CTable.as_mut_ptr(),
                             (*table).count.as_mut_ptr(), maxSymbolValue,
                             huffLog,
                             (*table).nodeTable.as_mut_ptr() as
                                 *mut libc::c_void,
                             ::std::mem::size_of::<huffNodeTable>() as
                                 libc::c_ulong);
    let _var_err__: size_t = maxBits;
    if 0 != ERR_isError(_var_err__) { return _var_err__ }
    huffLog = maxBits as U32;
    memset((*table).CTable.as_mut_ptr().offset(maxSymbolValue.wrapping_add(1i32
                                                                               as
                                                                               libc::c_uint)
                                                   as isize) as
               *mut libc::c_void, 0i32,
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
                        (*table).CTable.as_mut_ptr(), maxSymbolValue,
                        huffLog);
    if 0 != ERR_isError(hSize) { return hSize }
    if !repeat.is_null() &&
           *repeat as libc::c_uint !=
               HUF_repeat_none as libc::c_int as libc::c_uint {
        let oldSize: size_t =
            HUF_estimateCompressedSize(oldHufTable,
                                       (*table).count.as_mut_ptr(),
                                       maxSymbolValue);
        let newSize: size_t =
            HUF_estimateCompressedSize((*table).CTable.as_mut_ptr(),
                                       (*table).count.as_mut_ptr(),
                                       maxSymbolValue);
        if oldSize <= hSize.wrapping_add(newSize) ||
               hSize.wrapping_add(12i32 as libc::c_ulong) >= srcSize {
            return HUF_compressCTable_internal(ostart, op, oend, src, srcSize,
                                               nbStreams, oldHufTable, bmi2)
        }
    }
    if hSize.wrapping_add(12u64) >= srcSize { return 0i32 as size_t }
    op = op.offset(hSize as isize);
    if !repeat.is_null() { *repeat = HUF_repeat_none }
    if !oldHufTable.is_null() {
        memcpy(oldHufTable as *mut libc::c_void,
               (*table).CTable.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<[HUF_CElt; 256]>() as libc::c_ulong);
    }
    return HUF_compressCTable_internal(ostart, op, oend, src, srcSize,
                                       nbStreams,
                                       (*table).CTable.as_mut_ptr(), bmi2);
}
unsafe extern "C" fn HUF_compressCTable_internal(ostart: *mut BYTE,
                                                 mut op: *mut BYTE,
                                                 oend: *mut BYTE,
                                                 mut src: *const libc::c_void,
                                                 mut srcSize: size_t,
                                                 mut nbStreams:
                                                     HUF_nbStreams_e,
                                                 mut CTable: *const HUF_CElt,
                                                 bmi2: libc::c_int)
 -> size_t {
    let cSize: size_t =
        if nbStreams as libc::c_uint ==
               HUF_singleStream as libc::c_int as libc::c_uint {
            HUF_compress1X_usingCTable_internal(op as *mut libc::c_void,
                                                oend.wrapping_offset_from(op)
                                                    as libc::c_long as size_t,
                                                src, srcSize, CTable, bmi2)
        } else {
            HUF_compress4X_usingCTable_internal(op as *mut libc::c_void,
                                                oend.wrapping_offset_from(op)
                                                    as libc::c_long as size_t,
                                                src, srcSize, CTable, bmi2)
        };
    if 0 != ERR_isError(cSize) { return cSize }
    if cSize == 0i32 as libc::c_ulong { return 0i32 as size_t }
    op = op.offset(cSize as isize);
    if op.wrapping_offset_from(ostart) as libc::c_long as size_t >=
           srcSize.wrapping_sub(1i32 as libc::c_ulong) {
        return 0i32 as size_t
    }
    return op.wrapping_offset_from(ostart) as libc::c_long as size_t;
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
    /* first 3 segments */
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
    }
    if srcSize < 12i32 as libc::c_ulong { return 0i32 as size_t }
    op = op.offset(6isize);
    let cSize: size_t =
        HUF_compress1X_usingCTable_internal(op as *mut libc::c_void,
                                            oend.wrapping_offset_from(op) as
                                                libc::c_long as size_t,
                                            ip as *const libc::c_void,
                                            segmentSize, CTable, bmi2);
    if 0 != ERR_isError(cSize) { return cSize }
    if cSize == 0i32 as libc::c_ulong { return 0i32 as size_t }
    MEM_writeLE16(ostart as *mut libc::c_void, cSize as U16);
    op = op.offset(cSize as isize);
    ip = ip.offset(segmentSize as isize);
    let cSize_0: size_t =
        HUF_compress1X_usingCTable_internal(op as *mut libc::c_void,
                                            oend.wrapping_offset_from(op) as
                                                libc::c_long as size_t,
                                            ip as *const libc::c_void,
                                            segmentSize, CTable, bmi2);
    if 0 != ERR_isError(cSize_0) { return cSize_0 }
    if cSize_0 == 0i32 as libc::c_ulong { return 0i32 as size_t }
    MEM_writeLE16(ostart.offset(2isize) as *mut libc::c_void, cSize_0 as U16);
    op = op.offset(cSize_0 as isize);
    ip = ip.offset(segmentSize as isize);
    let cSize_1: size_t =
        HUF_compress1X_usingCTable_internal(op as *mut libc::c_void,
                                            oend.wrapping_offset_from(op) as
                                                libc::c_long as size_t,
                                            ip as *const libc::c_void,
                                            segmentSize, CTable, bmi2);
    if 0 != ERR_isError(cSize_1) { return cSize_1 }
    if cSize_1 == 0i32 as libc::c_ulong { return 0i32 as size_t }
    MEM_writeLE16(ostart.offset(4isize) as *mut libc::c_void, cSize_1 as U16);
    op = op.offset(cSize_1 as isize);
    ip = ip.offset(segmentSize as isize);
    let cSize_2: size_t =
        HUF_compress1X_usingCTable_internal(op as *mut libc::c_void,
                                            oend.wrapping_offset_from(op) as
                                                libc::c_long as size_t,
                                            ip as *const libc::c_void,
                                            iend.wrapping_offset_from(ip) as
                                                libc::c_long as size_t,
                                            CTable, bmi2);
    if 0 != ERR_isError(cSize_2) { return cSize_2 }
    if cSize_2 == 0i32 as libc::c_ulong { return 0i32 as size_t }
    op = op.offset(cSize_2 as isize);
    return op.wrapping_offset_from(ostart) as libc::c_long as size_t;
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
    }
    return HUF_compress1X_usingCTable_internal_default(dst, dstSize, src,
                                                       srcSize, CTable);
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
    if dstSize < 8i32 as libc::c_ulong { return 0i32 as size_t }
    let initErr: size_t =
        BIT_initCStream(&mut bitC, op as *mut libc::c_void,
                        oend.wrapping_offset_from(op) as libc::c_long as
                            size_t);
    if 0 != ERR_isError(initErr) { return 0i32 as size_t }
    n = srcSize & !3i32 as libc::c_ulong;
    let mut current_block_13: u64;
    match srcSize & 3i32 as libc::c_ulong {
        3 => {
            HUF_encodeSymbol(&mut bitC,
                             *ip.offset(n.wrapping_add(2i32 as libc::c_ulong)
                                            as isize) as U32, CTable);
            if (::std::mem::size_of::<size_t>() as
                    libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) <
                   (12i32 * 4i32 + 7i32) as libc::c_ulong {
                BIT_flushBits(&mut bitC);
            }
            /* fall-through */
            current_block_13 = 17462853432275320052;
        }
        2 => { current_block_13 = 17462853432275320052; }
        1 => { current_block_13 = 9520589643232431964; }
        0 | _ => { current_block_13 = 2370887241019905314; }
    }
    match current_block_13 {
        17462853432275320052 => {
            HUF_encodeSymbol(&mut bitC,
                             *ip.offset(n.wrapping_add(1i32 as libc::c_ulong)
                                            as isize) as U32, CTable);
            if (::std::mem::size_of::<size_t>() as
                    libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) <
                   (12i32 * 2i32 + 7i32) as libc::c_ulong {
                BIT_flushBits(&mut bitC);
            }
            /* fall-through */
            current_block_13 = 9520589643232431964;
        }
        _ => { }
    }
    match current_block_13 {
        9520589643232431964 => {
            HUF_encodeSymbol(&mut bitC,
                             *ip.offset(n.wrapping_add(0i32 as libc::c_ulong)
                                            as isize) as U32, CTable);
            BIT_flushBits(&mut bitC);
        }
        _ => { }
    }
    while n > 0i32 as libc::c_ulong {
        HUF_encodeSymbol(&mut bitC,
                         *ip.offset(n.wrapping_sub(1i32 as libc::c_ulong) as
                                        isize) as U32, CTable);
        if (::std::mem::size_of::<size_t>() as
                libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) <
               (12i32 * 2i32 + 7i32) as libc::c_ulong {
            BIT_flushBits(&mut bitC);
        }
        HUF_encodeSymbol(&mut bitC,
                         *ip.offset(n.wrapping_sub(2i32 as libc::c_ulong) as
                                        isize) as U32, CTable);
        if (::std::mem::size_of::<size_t>() as
                libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) <
               (12i32 * 4i32 + 7i32) as libc::c_ulong {
            BIT_flushBits(&mut bitC);
        }
        HUF_encodeSymbol(&mut bitC,
                         *ip.offset(n.wrapping_sub(3i32 as libc::c_ulong) as
                                        isize) as U32, CTable);
        if (::std::mem::size_of::<size_t>() as
                libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) <
               (12i32 * 2i32 + 7i32) as libc::c_ulong {
            BIT_flushBits(&mut bitC);
        }
        HUF_encodeSymbol(&mut bitC,
                         *ip.offset(n.wrapping_sub(4i32 as libc::c_ulong) as
                                        isize) as U32, CTable);
        BIT_flushBits(&mut bitC);
        n =
            (n as libc::c_ulong).wrapping_sub(4i32 as libc::c_ulong) as size_t
                as size_t
    }
    return BIT_closeCStream(&mut bitC);
}
unsafe extern "C" fn HUF_encodeSymbol(mut bitCPtr: *mut BIT_CStream_t,
                                      mut symbol: U32,
                                      mut CTable: *const HUF_CElt) {
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
                                         mut maxSymbolValue: libc::c_uint,
                                         mut huffLog: libc::c_uint)
 -> size_t {
    /* precomputed conversion table */
    let mut bitsToWeight: [BYTE; 13] = [0; 13];
    let mut huffWeight: [BYTE; 255] = [0; 255];
    let mut op: *mut BYTE = dst as *mut BYTE;
    let mut n: U32 = 0;
    if maxSymbolValue > 255i32 as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as size_t
    }
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
                            maxDstSize.wrapping_sub(1i32 as libc::c_ulong),
                            huffWeight.as_mut_ptr() as *const libc::c_void,
                            maxSymbolValue as size_t);
    if 0 != ERR_isError(hSize) { return hSize }
    if 0 !=
           (hSize > 1i32 as libc::c_ulong) as libc::c_int &
               (hSize <
                    maxSymbolValue.wrapping_div(2i32 as libc::c_uint) as
                        libc::c_ulong) as libc::c_int {
        *op.offset(0isize) = hSize as BYTE;
        return hSize.wrapping_add(1i32 as libc::c_ulong)
    }
    if maxSymbolValue > (256i32 - 128i32) as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    if maxSymbolValue.wrapping_add(1i32 as
                                       libc::c_uint).wrapping_div(2i32 as
                                                                      libc::c_uint).wrapping_add(1i32
                                                                                                     as
                                                                                                     libc::c_uint)
           as libc::c_ulong > maxDstSize {
        return -(ZSTD_error_dstSize_tooSmall as libc::c_int) as size_t
    }
    *op.offset(0isize) =
        (128i32 as
             libc::c_uint).wrapping_add(maxSymbolValue.wrapping_sub(1i32 as
                                                                        libc::c_uint))
            as BYTE;
    huffWeight[maxSymbolValue as usize] = 0i32 as BYTE;
    n = 0i32 as U32;
    while n < maxSymbolValue {
        *op.offset(n.wrapping_div(2i32 as
                                      libc::c_uint).wrapping_add(1i32 as
                                                                     libc::c_uint)
                       as isize) =
            (((huffWeight[n as usize] as libc::c_int) << 4i32) +
                 huffWeight[n.wrapping_add(1i32 as libc::c_uint) as usize] as
                     libc::c_int) as BYTE;
        n =
            (n as libc::c_uint).wrapping_add(2i32 as libc::c_uint) as U32 as
                U32
    }
    return maxSymbolValue.wrapping_add(1i32 as
                                           libc::c_uint).wrapping_div(2i32 as
                                                                          libc::c_uint).wrapping_add(1i32
                                                                                                         as
                                                                                                         libc::c_uint)
               as size_t;
}
/* *******************************************************
*  HUF : Huffman block compression
*********************************************************/
/* HUF_compressWeights() :
 * Same as FSE_compress(), but dedicated to huff0's weights compression.
 * The use case needs much less stack memory.
 * Note : all elements within weightTable are supposed to be <= HUF_TABLELOG_MAX.
 */
unsafe extern "C" fn HUF_compressWeights(mut dst: *mut libc::c_void,
                                         mut dstSize: size_t,
                                         mut weightTable: *const libc::c_void,
                                         mut wtSize: size_t) -> size_t {
    let ostart: *mut BYTE = dst as *mut BYTE;
    let mut op: *mut BYTE = ostart;
    let oend: *mut BYTE = ostart.offset(dstSize as isize);
    let mut maxSymbolValue: libc::c_uint = 12i32 as libc::c_uint;
    let mut tableLog: U32 = 6i32 as U32;
    let mut CTable: [FSE_CTable; 59] = [0; 59];
    let mut scratchBuffer: [BYTE; 64] = [0; 64];
    let mut count: [libc::c_uint; 13] = [0; 13];
    let mut norm: [S16; 13] = [0; 13];
    if wtSize <= 1i32 as libc::c_ulong { return 0i32 as size_t }
    let maxCount: libc::c_uint =
        HIST_count_simple(count.as_mut_ptr(), &mut maxSymbolValue,
                          weightTable, wtSize);
    if maxCount as libc::c_ulong == wtSize { return 1i32 as size_t }
    if maxCount == 1i32 as libc::c_uint { return 0i32 as size_t }
    tableLog = FSE_optimalTableLog(tableLog, wtSize, maxSymbolValue);
    let _var_err__: size_t =
        FSE_normalizeCount(norm.as_mut_ptr(), tableLog, count.as_mut_ptr(),
                           wtSize, maxSymbolValue);
    if 0 != ERR_isError(_var_err__) { return _var_err__ }
    let hSize: size_t =
        FSE_writeNCount(op as *mut libc::c_void,
                        oend.wrapping_offset_from(op) as libc::c_long as
                            size_t, norm.as_mut_ptr(), maxSymbolValue,
                        tableLog);
    if 0 != ERR_isError(hSize) { return hSize }
    op = op.offset(hSize as isize);
    let _var_err___0: size_t =
        FSE_buildCTable_wksp(CTable.as_mut_ptr(), norm.as_mut_ptr(),
                             maxSymbolValue, tableLog,
                             scratchBuffer.as_mut_ptr() as *mut libc::c_void,
                             ::std::mem::size_of::<[BYTE; 64]>() as
                                 libc::c_ulong);
    if 0 != ERR_isError(_var_err___0) { return _var_err___0 }
    let cSize: size_t =
        FSE_compress_usingCTable(op as *mut libc::c_void,
                                 oend.wrapping_offset_from(op) as libc::c_long
                                     as size_t, weightTable, wtSize,
                                 CTable.as_mut_ptr());
    if 0 != ERR_isError(cSize) { return cSize }
    if cSize == 0i32 as libc::c_ulong { return 0i32 as size_t }
    op = op.offset(cSize as isize);
    return op.wrapping_offset_from(ostart) as libc::c_long as size_t;
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
/* *< `workSpace` must be aligned on 4-bytes boundaries, `wkspSize` must be >= HUF_WORKSPACE_SIZE */
/* * HUF_buildCTable_wksp() :
 *  Same as HUF_buildCTable(), but using externally allocated scratch buffer.
 * `workSpace` must be aligned on 4-bytes boundaries, and its size must be >= HUF_CTABLE_WORKSPACE_SIZE.
 */
#[no_mangle]
pub unsafe extern "C" fn HUF_buildCTable_wksp(mut tree: *mut HUF_CElt,
                                              mut count: *const libc::c_uint,
                                              mut maxSymbolValue: U32,
                                              mut maxNbBits: U32,
                                              mut workSpace:
                                                  *mut libc::c_void,
                                              mut wkspSize: size_t)
 -> size_t {
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
    }
    if wkspSize < ::std::mem::size_of::<huffNodeTable>() as libc::c_ulong {
        return -(ZSTD_error_workSpace_tooSmall as libc::c_int) as size_t
    }
    if maxNbBits == 0i32 as libc::c_uint { maxNbBits = 11i32 as U32 }
    if maxSymbolValue > 255i32 as libc::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as libc::c_int) as size_t
    }
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
    let ref mut fresh0 = (*huffNode.offset((lowS - 1i32) as isize)).parent;
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
        let mut n1: U32 =
            (if (*huffNode.offset(lowS as isize)).count <
                    (*huffNode.offset(lowN as isize)).count {
                 let fresh1 = lowS;
                 lowS = lowS - 1;
                 fresh1
             } else { let fresh2 = lowN; lowN = lowN + 1; fresh2 }) as U32;
        let mut n2: U32 =
            (if (*huffNode.offset(lowS as isize)).count <
                    (*huffNode.offset(lowN as isize)).count {
                 let fresh3 = lowS;
                 lowS = lowS - 1;
                 fresh3
             } else { let fresh4 = lowN; lowN = lowN + 1; fresh4 }) as U32;
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
            ((*huffNode.offset((*huffNode.offset(n as isize)).parent as
                                   isize)).nbBits as libc::c_int + 1i32) as
                BYTE;
        n = n.wrapping_sub(1)
    }
    n = 0i32 as U32;
    while n <= nonNullRank {
        (*huffNode.offset(n as isize)).nbBits =
            ((*huffNode.offset((*huffNode.offset(n as isize)).parent as
                                   isize)).nbBits as libc::c_int + 1i32) as
                BYTE;
        n = n.wrapping_add(1)
    }
    maxNbBits = HUF_setMaxHeight(huffNode, nonNullRank, maxNbBits);
    let mut nbPerRank: [U16; 13] =
        [0i32 as U16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut valPerRank: [U16; 13] =
        [0i32 as U16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    if maxNbBits > 12i32 as libc::c_uint {
        return -(ZSTD_error_GENERIC as libc::c_int) as size_t
    }
    n = 0i32 as U32;
    while n <= nonNullRank {
        nbPerRank[(*huffNode.offset(n as isize)).nbBits as usize] =
            nbPerRank[(*huffNode.offset(n as isize)).nbBits as
                          usize].wrapping_add(1);
        n = n.wrapping_add(1)
    }
    let mut min: U16 = 0i32 as U16;
    n = maxNbBits;
    while n > 0i32 as libc::c_uint {
        valPerRank[n as usize] = min;
        min =
            (min as libc::c_int + nbPerRank[n as usize] as libc::c_int) as
                U16;
        min = (min as libc::c_int >> 1i32) as U16;
        n = n.wrapping_sub(1)
    }
    n = 0i32 as U32;
    while n <= maxSymbolValue {
        (*tree.offset((*huffNode.offset(n as isize)).byte as isize)).nbBits =
            (*huffNode.offset(n as isize)).nbBits;
        n = n.wrapping_add(1)
    }
    n = 0i32 as U32;
    while n <= maxSymbolValue {
        let fresh6 = valPerRank[(*tree.offset(n as isize)).nbBits as usize];
        valPerRank[(*tree.offset(n as isize)).nbBits as usize] =
            valPerRank[(*tree.offset(n as isize)).nbBits as
                           usize].wrapping_add(1);
        (*tree.offset(n as isize)).val = fresh6;
        n = n.wrapping_add(1)
    }
    return maxNbBits as size_t;
}
unsafe extern "C" fn HUF_setMaxHeight(mut huffNode: *mut nodeElt,
                                      mut lastNonNull: U32,
                                      mut maxNbBits: U32) -> U32 {
    let largestBits: U32 =
        (*huffNode.offset(lastNonNull as isize)).nbBits as U32;
    if largestBits <= maxNbBits { return largestBits }
    let mut totalCost: libc::c_int = 0i32;
    let baseCost: U32 = (1i32 << largestBits.wrapping_sub(maxNbBits)) as U32;
    let mut n: U32 = lastNonNull;
    while (*huffNode.offset(n as isize)).nbBits as libc::c_uint > maxNbBits {
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
    while (*huffNode.offset(n as isize)).nbBits as libc::c_uint == maxNbBits {
        n = n.wrapping_sub(1)
    }
    totalCost >>= largestBits.wrapping_sub(maxNbBits);
    let noSymbol: U32 = 0xf0f0f0f0u32;
    let mut rankLast: [U32; 14] = [0; 14];
    let mut pos: libc::c_int = 0;
    memset(rankLast.as_mut_ptr() as *mut libc::c_void, 0xf0i32,
           ::std::mem::size_of::<[U32; 14]>() as libc::c_ulong);
    let mut currentNbBits: U32 = maxNbBits;
    pos = n as libc::c_int;
    while pos >= 0i32 {
        if !((*huffNode.offset(pos as isize)).nbBits as libc::c_uint >=
                 currentNbBits) {
            currentNbBits = (*huffNode.offset(pos as isize)).nbBits as U32;
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
                rankLast[nBitsToDecrease.wrapping_sub(1i32 as libc::c_uint) as
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
            if (*huffNode.offset(rankLast[nBitsToDecrease as usize] as
                                     isize)).nbBits as libc::c_uint !=
                   maxNbBits.wrapping_sub(nBitsToDecrease) {
                rankLast[nBitsToDecrease as usize] = noSymbol
            }
        }
    }
    while totalCost < 0i32 {
        /* Sometimes, cost correction overshoot */
        if rankLast[1usize] == noSymbol {
            /* special case : no rank 1 symbol (using maxNbBits-1); let's create one from largest rank 0 (using maxNbBits) */
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
    return maxNbBits;
}
unsafe extern "C" fn HUF_sort(mut huffNode: *mut nodeElt,
                              mut count: *const libc::c_uint,
                              mut maxSymbolValue: U32) {
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
/* ****************************************
 *  HUF detailed API
 * ****************************************/
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
/* ***   Tool functions *** */
/* *< maximum input size for a single block compressed with HUF_compress */
/* *< maximum compressed size (worst case) */
#[no_mangle]
pub unsafe extern "C" fn HUF_compressBound(mut size: size_t) -> size_t {
    return (129i32 as
                libc::c_ulong).wrapping_add(size.wrapping_add(size >>
                                                                  8i32).wrapping_add(8i32
                                                                                         as
                                                                                         libc::c_ulong));
}
/* @return : maxNbBits; CTable and count can overlap. In which case, CTable will overwrite count content */
#[no_mangle]
pub unsafe extern "C" fn HUF_buildCTable(mut tree: *mut HUF_CElt,
                                         mut count: *const libc::c_uint,
                                         mut maxSymbolValue: libc::c_uint,
                                         mut maxNbBits: libc::c_uint)
 -> size_t {
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
/* * HUF_compress4X_repeat() :
 *  Same as HUF_compress4X_wksp(), but considers using hufTable if *repeat != HUF_repeat_none.
 *  If it uses hufTable it does not modify hufTable or repeat.
 *  If it doesn't, it sets *repeat = HUF_repeat_none, and it sets hufTable to the table used.
 *  If preferRepeat then the old table will always be used if valid. */
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
                                 huffLog, HUF_fourStreams, workSpace,
                                 wkspSize, hufTable, repeat, preferRepeat,
                                 bmi2);
}
/* * HUF_readCTable() :
 *  Loading a CTable saved with HUF_writeCTable() */
#[no_mangle]
pub unsafe extern "C" fn HUF_readCTable(mut CTable: *mut HUF_CElt,
                                        mut maxSymbolValuePtr:
                                            *mut libc::c_uint,
                                        mut src: *const libc::c_void,
                                        mut srcSize: size_t) -> size_t {
    /* init not required, even though some static analyzer may complain */
    let mut huffWeight: [BYTE; 256] = [0; 256];
    /* large enough for values from 0 to 16 */
    let mut rankVal: [U32; 16] = [0; 16];
    let mut tableLog: U32 = 0i32 as U32;
    let mut nbSymbols: U32 = 0i32 as U32;
    /* get symbol weights */
    let readSize: size_t =
        HUF_readStats(huffWeight.as_mut_ptr(), (255i32 + 1i32) as size_t,
                      rankVal.as_mut_ptr(), &mut nbSymbols, &mut tableLog,
                      src, srcSize);
    if 0 != ERR_isError(readSize) { return readSize }
    if tableLog > 12i32 as libc::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as libc::c_int) as size_t
    }
    if nbSymbols > (*maxSymbolValuePtr).wrapping_add(1i32 as libc::c_uint) {
        return -(ZSTD_error_maxSymbolValue_tooSmall as libc::c_int) as size_t
    }
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
            (min as libc::c_int + nbPerRank[n_2 as usize] as libc::c_int) as
                U16;
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
    return readSize;
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
/* ====================== */
/* single stream variants */
/* ====================== */
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
/* *< `workSpace` must be a table of at least HUF_WORKSPACE_SIZE_U32 unsigned */
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
                                 huffLog, HUF_singleStream, workSpace,
                                 wkspSize, 0 as *mut HUF_CElt,
                                 0 as *mut HUF_repeat, 0i32, 0i32);
}
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
                                 huffLog, HUF_singleStream, workSpace,
                                 wkspSize, hufTable, repeat, preferRepeat,
                                 bmi2);
}