#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(libc)]
extern crate libc;
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
pub type ZSTD_ErrorCode = libc::c_uint;
/* never EVER use this value directly, it can change in future versions! Use ZSTD_isError() instead */
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
/* following error codes are __NOT STABLE__, they can be removed or changed in future versions */
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
pub const ZSTD_error_dstBuffer_null: ZSTD_ErrorCode = 74;
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* Note : this module is expected to remain private, do not expose it */
/* ****************************************
*  Dependencies
******************************************/
/* size_t */
/* ****************************************
*  Compiler-specific
******************************************/
/* C99 */
/*-****************************************
*  Customization (error_public.h)
******************************************/
pub type ERR_enum = ZSTD_ErrorCode;
/*-****************************************
*  Error Strings
******************************************/
/* error_private.c */
#[no_mangle]
pub unsafe extern "C" fn ERR_getErrorString(mut code: ERR_enum)
 -> *const libc::c_char {
    static mut notErrorCode: *const libc::c_char =
        b"Unspecified error code\x00" as *const u8 as *const libc::c_char;
    match code as libc::c_uint {
        0 => {
            return b"No error detected\x00" as *const u8 as
                       *const libc::c_char
        }
        1 => {
            return b"Error (generic)\x00" as *const u8 as *const libc::c_char
        }
        10 => {
            return b"Unknown frame descriptor\x00" as *const u8 as
                       *const libc::c_char
        }
        12 => {
            return b"Version not supported\x00" as *const u8 as
                       *const libc::c_char
        }
        14 => {
            return b"Unsupported frame parameter\x00" as *const u8 as
                       *const libc::c_char
        }
        16 => {
            return b"Frame requires too much memory for decoding\x00" as
                       *const u8 as *const libc::c_char
        }
        20 => {
            return b"Corrupted block detected\x00" as *const u8 as
                       *const libc::c_char
        }
        22 => {
            return b"Restored data doesn\'t match checksum\x00" as *const u8
                       as *const libc::c_char
        }
        40 => {
            return b"Unsupported parameter\x00" as *const u8 as
                       *const libc::c_char
        }
        42 => {
            return b"Parameter is out of bound\x00" as *const u8 as
                       *const libc::c_char
        }
        62 => {
            return b"Context should be init first\x00" as *const u8 as
                       *const libc::c_char
        }
        64 => {
            return b"Allocation error : not enough memory\x00" as *const u8 as
                       *const libc::c_char
        }
        66 => {
            return b"workSpace buffer is not large enough\x00" as *const u8 as
                       *const libc::c_char
        }
        60 => {
            return b"Operation not authorized at current processing stage\x00"
                       as *const u8 as *const libc::c_char
        }
        44 => {
            return b"tableLog requires too much memory : unsupported\x00" as
                       *const u8 as *const libc::c_char
        }
        46 => {
            return b"Unsupported max Symbol Value : too large\x00" as
                       *const u8 as *const libc::c_char
        }
        48 => {
            return b"Specified maxSymbolValue is too small\x00" as *const u8
                       as *const libc::c_char
        }
        30 => {
            return b"Dictionary is corrupted\x00" as *const u8 as
                       *const libc::c_char
        }
        32 => {
            return b"Dictionary mismatch\x00" as *const u8 as
                       *const libc::c_char
        }
        34 => {
            return b"Cannot create Dictionary from provided samples\x00" as
                       *const u8 as *const libc::c_char
        }
        70 => {
            return b"Destination buffer is too small\x00" as *const u8 as
                       *const libc::c_char
        }
        72 => {
            return b"Src size is incorrect\x00" as *const u8 as
                       *const libc::c_char
        }
        74 => {
            return b"Operation on NULL destination buffer\x00" as *const u8 as
                       *const libc::c_char
        }
        100 => {
            return b"Frame index is too large\x00" as *const u8 as
                       *const libc::c_char
        }
        102 => {
            return b"An I/O error occurred when reading/seeking\x00" as
                       *const u8 as *const libc::c_char
        }
        120 | _ => { return notErrorCode }
    };
}