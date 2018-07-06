#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( const_slice_as_ptr , libc )]
extern crate libc;
pub const ZSTD_error_workSpace_tooSmall: ERR_enum = 66;
pub type ERR_enum = libc::c_uint;
pub const ZSTD_error_frameIndex_tooLarge: ERR_enum = 100;
pub type size_t = libc::c_ulong;
pub const ZSTD_error_checksum_wrong: ERR_enum = 22;
pub const ZSTD_error_maxSymbolValue_tooSmall: ERR_enum = 48;
pub const ZSTD_error_parameter_unsupported: ERR_enum = 40;
pub const ZSTD_error_maxCode: ERR_enum = 120;
pub const ZSTD_error_dictionary_wrong: ERR_enum = 32;
pub const ZSTD_error_init_missing: ERR_enum = 62;
pub const ZSTD_error_memory_allocation: ERR_enum = 64;
pub const ZSTD_error_parameter_outOfBound: ERR_enum = 42;
pub const ZSTD_error_GENERIC: ERR_enum = 1;
pub const ZSTD_error_tableLog_tooLarge: ERR_enum = 44;
pub const ZSTD_error_dictionary_corrupted: ERR_enum = 30;
pub const ZSTD_error_maxSymbolValue_tooLarge: ERR_enum = 46;
pub const ZSTD_error_frameParameter_windowTooLarge: ERR_enum = 16;
pub const ZSTD_error_version_unsupported: ERR_enum = 12;
pub const ZSTD_error_seekableIO: ERR_enum = 102;
pub const ZSTD_error_prefix_unknown: ERR_enum = 10;
pub const ZSTD_error_no_error: ERR_enum = 0;
pub const ZSTD_error_stage_wrong: ERR_enum = 60;
pub type ZSTD_ErrorCode = ERR_enum;
pub const ZSTD_error_corruption_detected: ERR_enum = 20;
pub const ZSTD_error_dictionaryCreation_failed: ERR_enum = 34;
pub const ZSTD_error_frameParameter_unsupported: ERR_enum = 14;
pub const ZSTD_error_dstSize_tooSmall: ERR_enum = 70;
pub const ZSTD_error_srcSize_wrong: ERR_enum = 72;
unsafe extern "C" fn ERR_isError(mut code: size_t) -> libc::c_uint {
    return (code > -(ZSTD_error_maxCode as libc::c_int) as size_t) as
               libc::c_int as libc::c_uint;
}
unsafe extern "C" fn ERR_getErrorCode(mut code: size_t) -> ERR_enum {
    if 0 == ERR_isError(code) {
        return ZSTD_error_no_error
    } else { return (0i32 as libc::c_ulong).wrapping_sub(code) as ERR_enum };
}
#[no_mangle]
pub unsafe extern "C" fn ERR_getErrorString(mut code: ERR_enum)
 -> *const libc::c_char {
    static mut notErrorCode: *const libc::c_char =
        unsafe {
            [85, 110, 115, 112, 101, 99, 105, 102, 105, 101, 100, 32, 101,
             114, 114, 111, 114, 32, 99, 111, 100, 101, 0].as_ptr() as
                *const _
        };
    match code as libc::c_uint {
        0 => {
            return (*::std::mem::transmute::<&[u8; 18],
                                             &mut [libc::c_char; 18]>(b"No error detected\x00")).as_mut_ptr()
        }
        1 => {
            return (*::std::mem::transmute::<&[u8; 16],
                                             &mut [libc::c_char; 16]>(b"Error (generic)\x00")).as_mut_ptr()
        }
        10 => {
            return (*::std::mem::transmute::<&[u8; 25],
                                             &mut [libc::c_char; 25]>(b"Unknown frame descriptor\x00")).as_mut_ptr()
        }
        12 => {
            return (*::std::mem::transmute::<&[u8; 22],
                                             &mut [libc::c_char; 22]>(b"Version not supported\x00")).as_mut_ptr()
        }
        14 => {
            return (*::std::mem::transmute::<&[u8; 28],
                                             &mut [libc::c_char; 28]>(b"Unsupported frame parameter\x00")).as_mut_ptr()
        }
        16 => {
            return (*::std::mem::transmute::<&[u8; 44],
                                             &mut [libc::c_char; 44]>(b"Frame requires too much memory for decoding\x00")).as_mut_ptr()
        }
        20 => {
            return (*::std::mem::transmute::<&[u8; 25],
                                             &mut [libc::c_char; 25]>(b"Corrupted block detected\x00")).as_mut_ptr()
        }
        22 => {
            return (*::std::mem::transmute::<&[u8; 37],
                                             &mut [libc::c_char; 37]>(b"Restored data doesn\'t match checksum\x00")).as_mut_ptr()
        }
        40 => {
            return (*::std::mem::transmute::<&[u8; 22],
                                             &mut [libc::c_char; 22]>(b"Unsupported parameter\x00")).as_mut_ptr()
        }
        42 => {
            return (*::std::mem::transmute::<&[u8; 26],
                                             &mut [libc::c_char; 26]>(b"Parameter is out of bound\x00")).as_mut_ptr()
        }
        62 => {
            return (*::std::mem::transmute::<&[u8; 29],
                                             &mut [libc::c_char; 29]>(b"Context should be init first\x00")).as_mut_ptr()
        }
        64 => {
            return (*::std::mem::transmute::<&[u8; 37],
                                             &mut [libc::c_char; 37]>(b"Allocation error : not enough memory\x00")).as_mut_ptr()
        }
        66 => {
            return (*::std::mem::transmute::<&[u8; 37],
                                             &mut [libc::c_char; 37]>(b"workSpace buffer is not large enough\x00")).as_mut_ptr()
        }
        60 => {
            return (*::std::mem::transmute::<&[u8; 53],
                                             &mut [libc::c_char; 53]>(b"Operation not authorized at current processing stage\x00")).as_mut_ptr()
        }
        44 => {
            return (*::std::mem::transmute::<&[u8; 48],
                                             &mut [libc::c_char; 48]>(b"tableLog requires too much memory : unsupported\x00")).as_mut_ptr()
        }
        46 => {
            return (*::std::mem::transmute::<&[u8; 41],
                                             &mut [libc::c_char; 41]>(b"Unsupported max Symbol Value : too large\x00")).as_mut_ptr()
        }
        48 => {
            return (*::std::mem::transmute::<&[u8; 38],
                                             &mut [libc::c_char; 38]>(b"Specified maxSymbolValue is too small\x00")).as_mut_ptr()
        }
        30 => {
            return (*::std::mem::transmute::<&[u8; 24],
                                             &mut [libc::c_char; 24]>(b"Dictionary is corrupted\x00")).as_mut_ptr()
        }
        32 => {
            return (*::std::mem::transmute::<&[u8; 20],
                                             &mut [libc::c_char; 20]>(b"Dictionary mismatch\x00")).as_mut_ptr()
        }
        34 => {
            return (*::std::mem::transmute::<&[u8; 47],
                                             &mut [libc::c_char; 47]>(b"Cannot create Dictionary from provided samples\x00")).as_mut_ptr()
        }
        70 => {
            return (*::std::mem::transmute::<&[u8; 32],
                                             &mut [libc::c_char; 32]>(b"Destination buffer is too small\x00")).as_mut_ptr()
        }
        72 => {
            return (*::std::mem::transmute::<&[u8; 22],
                                             &mut [libc::c_char; 22]>(b"Src size is incorrect\x00")).as_mut_ptr()
        }
        100 => {
            return (*::std::mem::transmute::<&[u8; 25],
                                             &mut [libc::c_char; 25]>(b"Frame index is too large\x00")).as_mut_ptr()
        }
        102 => {
            return (*::std::mem::transmute::<&[u8; 43],
                                             &mut [libc::c_char; 43]>(b"An I/O error occurred when reading/seeking\x00")).as_mut_ptr()
        }
        120 | _ => { return notErrorCode }
    };
}
unsafe extern "C" fn ERR_getErrorName(mut code: size_t)
 -> *const libc::c_char {
    return ERR_getErrorString(ERR_getErrorCode(code));
}
