#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( libc )]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
}
pub const ZSTD_error_maxCode: ERR_enum = 120;
pub const ZSTD_error_parameter_outOfBound: ERR_enum = 42;
pub const ZSTD_error_maxSymbolValue_tooLarge: ERR_enum = 46;
pub const ZSTD_error_seekableIO: ERR_enum = 102;
pub const ZSTD_error_prefix_unknown: ERR_enum = 10;
pub const ZSTD_error_dictionaryCreation_failed: ERR_enum = 34;
pub const ZSTD_error_dictionary_corrupted: ERR_enum = 30;
pub const ZSTD_error_parameter_unsupported: ERR_enum = 40;
pub const ZSTD_error_frameIndex_tooLarge: ERR_enum = 100;
pub const ZSTD_error_corruption_detected: ERR_enum = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: ERR_enum = 16;
pub const ZSTD_error_init_missing: ERR_enum = 62;
pub const ZSTD_error_dstSize_tooSmall: ERR_enum = 70;
pub const ZSTD_error_frameParameter_unsupported: ERR_enum = 14;
pub type ERR_enum = libc::c_uint;
pub const ZSTD_error_version_unsupported: ERR_enum = 12;
pub const ZSTD_error_dictionary_wrong: ERR_enum = 32;
pub const ZSTD_error_GENERIC: ERR_enum = 1;
pub type size_t = libc::c_ulong;
pub type ZSTD_ErrorCode = ERR_enum;
pub const ZSTD_error_maxSymbolValue_tooSmall: ERR_enum = 48;
pub const ZSTD_error_srcSize_wrong: ERR_enum = 72;
pub const ZSTD_error_stage_wrong: ERR_enum = 60;
pub const ZSTD_error_tableLog_tooLarge: ERR_enum = 44;
pub const ZSTD_error_workSpace_tooSmall: ERR_enum = 66;
pub const ZSTD_error_memory_allocation: ERR_enum = 64;
pub const ZSTD_error_no_error: ERR_enum = 0;
pub const ZSTD_error_checksum_wrong: ERR_enum = 22;
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
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_isError(mut errorCode: size_t)
 -> libc::c_uint {
    return ERR_isError(errorCode);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_getErrorName(mut errorCode: size_t)
 -> *const libc::c_char {
    return ERR_getErrorName(errorCode);
}
