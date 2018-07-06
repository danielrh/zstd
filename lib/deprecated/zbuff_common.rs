#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( libc )]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn ERR_getErrorString(code: ERR_enum) -> *const libc::c_char;
}
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub type size_t = libc::c_ulong;
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
pub type ERR_enum = ZSTD_ErrorCode;
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub type ZSTD_ErrorCode = libc::c_uint;
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
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
