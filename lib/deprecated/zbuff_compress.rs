#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( extern_types , libc )]
extern crate libc;
extern "C" {
    pub type ZSTD_CCtx_s;
    #[no_mangle]
    fn ZSTD_createCStream() -> *mut ZSTD_CStream;
    #[no_mangle]
    fn ZSTD_freeCStream(zcs: *mut ZSTD_CStream) -> size_t;
    #[no_mangle]
    fn ZSTD_initCStream(zcs: *mut ZSTD_CStream, compressionLevel: libc::c_int)
     -> size_t;
    #[no_mangle]
    fn ZSTD_compressStream(zcs: *mut ZSTD_CStream,
                           output: *mut ZSTD_outBuffer,
                           input: *mut ZSTD_inBuffer) -> size_t;
    #[no_mangle]
    fn ZSTD_flushStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer)
     -> size_t;
    #[no_mangle]
    fn ZSTD_endStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer)
     -> size_t;
    #[no_mangle]
    fn ZSTD_CStreamInSize() -> size_t;
    #[no_mangle]
    fn ZSTD_CStreamOutSize() -> size_t;
    #[no_mangle]
    fn ZSTD_initCStream_usingDict(zcs: *mut ZSTD_CStream,
                                  dict: *const libc::c_void, dictSize: size_t,
                                  compressionLevel: libc::c_int) -> size_t;
    #[no_mangle]
    fn ZSTD_createCStream_advanced(customMem: ZSTD_customMem)
     -> *mut ZSTD_CStream;
    #[no_mangle]
    fn ZSTD_initCStream_advanced(zcs: *mut ZSTD_CStream,
                                 dict: *const libc::c_void, dictSize: size_t,
                                 params: ZSTD_parameters,
                                 pledgedSrcSize: libc::c_ulonglong) -> size_t;
}
pub const ZSTD_fast: ZSTD_strategy = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_compressionParameters {
    pub windowLog: libc::c_uint,
    pub chainLog: libc::c_uint,
    pub hashLog: libc::c_uint,
    pub searchLog: libc::c_uint,
    pub searchLength: libc::c_uint,
    pub targetLength: libc::c_uint,
    pub strategy: ZSTD_strategy,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_inBuffer_s {
    pub src: *const libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
/* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
/* ***************************
*  Streaming
****************************/
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
/* *< position where reading stopped. Will be updated. Necessarily 0 <= pos <= size */
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
pub const ZSTD_dfast: ZSTD_strategy = 2;
/* *< position where writing stopped. Will be updated. Necessarily 0 <= pos <= size */
pub type ZSTD_CStream = ZSTD_CCtx;
pub type ZBUFF_CCtx = ZSTD_CStream;
/* **************************************
*  Explicit context
***************************************/
/* !< maximum compression level available */
/* !< provides readable string from an error code */
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type size_t = libc::c_ulong;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const ZSTD_btopt: ZSTD_strategy = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub type ZSTD_strategy = libc::c_uint;
pub const ZSTD_lazy: ZSTD_strategy = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_parameters {
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
}
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_frameParameters {
    pub contentSizeFlag: libc::c_uint,
    pub checksumFlag: libc::c_uint,
    pub noDictIDFlag: libc::c_uint,
}
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_createCCtx() -> *mut ZBUFF_CCtx {
    return ZSTD_createCStream();
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_freeCCtx(mut zbc: *mut ZBUFF_CCtx) -> size_t {
    return ZSTD_freeCStream(zbc);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_compressInit(mut zbc: *mut ZBUFF_CCtx,
                                            mut compressionLevel: libc::c_int)
 -> size_t {
    return ZSTD_initCStream(zbc, compressionLevel);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_compressInitDictionary(mut zbc:
                                                          *mut ZBUFF_CCtx,
                                                      mut dict:
                                                          *const libc::c_void,
                                                      mut dictSize: size_t,
                                                      mut compressionLevel:
                                                          libc::c_int)
 -> size_t {
    return ZSTD_initCStream_usingDict(zbc, dict, dictSize, compressionLevel);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_compressContinue(mut zbc: *mut ZBUFF_CCtx,
                                                mut dst: *mut libc::c_void,
                                                mut dstCapacityPtr:
                                                    *mut size_t,
                                                mut src: *const libc::c_void,
                                                mut srcSizePtr: *mut size_t)
 -> size_t {
    let mut result: size_t = 0;
    let mut outBuff: ZSTD_outBuffer =
        ZSTD_outBuffer_s{dst: 0 as *mut libc::c_void, size: 0, pos: 0,};
    let mut inBuff: ZSTD_inBuffer =
        ZSTD_inBuffer_s{src: 0 as *const libc::c_void, size: 0, pos: 0,};
    outBuff.dst = dst;
    outBuff.pos = 0i32 as size_t;
    outBuff.size = *dstCapacityPtr;
    inBuff.src = src;
    inBuff.pos = 0i32 as size_t;
    inBuff.size = *srcSizePtr;
    result =
        ZSTD_compressStream(zbc, &mut outBuff as *mut ZSTD_outBuffer,
                            &mut inBuff as *mut ZSTD_inBuffer);
    *dstCapacityPtr = outBuff.pos;
    *srcSizePtr = inBuff.pos;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_compressFlush(mut zbc: *mut ZBUFF_CCtx,
                                             mut dst: *mut libc::c_void,
                                             mut dstCapacityPtr: *mut size_t)
 -> size_t {
    let mut result: size_t = 0;
    let mut outBuff: ZSTD_outBuffer =
        ZSTD_outBuffer_s{dst: 0 as *mut libc::c_void, size: 0, pos: 0,};
    outBuff.dst = dst;
    outBuff.pos = 0i32 as size_t;
    outBuff.size = *dstCapacityPtr;
    result = ZSTD_flushStream(zbc, &mut outBuff as *mut ZSTD_outBuffer);
    *dstCapacityPtr = outBuff.pos;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_compressEnd(mut zbc: *mut ZBUFF_CCtx,
                                           mut dst: *mut libc::c_void,
                                           mut dstCapacityPtr: *mut size_t)
 -> size_t {
    let mut result: size_t = 0;
    let mut outBuff: ZSTD_outBuffer =
        ZSTD_outBuffer_s{dst: 0 as *mut libc::c_void, size: 0, pos: 0,};
    outBuff.dst = dst;
    outBuff.pos = 0i32 as size_t;
    outBuff.size = *dstCapacityPtr;
    result = ZSTD_endStream(zbc, &mut outBuff as *mut ZSTD_outBuffer);
    *dstCapacityPtr = outBuff.pos;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_recommendedCInSize() -> size_t {
    return ZSTD_CStreamInSize();
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_recommendedCOutSize() -> size_t {
    return ZSTD_CStreamOutSize();
}
/* ***************************************************************************************
 * START OF ADVANCED AND EXPERIMENTAL FUNCTIONS
 * The definitions in this section are considered experimental.
 * They should never be used with a dynamic library, as prototypes may change in the future.
 * They are provided for advanced scenarios.
 * Use them only in association with static linking.
 * ***************************************************************************************/
/* ***************************************************************************************
 * START OF ADVANCED AND EXPERIMENTAL FUNCTIONS
 * The definitions in this section are considered experimental.
 * They should never be used with a dynamic library, as prototypes may change in the future.
 * They are provided for advanced scenarios.
 * Use them only in association with static linking.
 * ***************************************************************************************/
/* !< recommended size for output buffer. Guarantee to successfully flush at least one complete block in all circumstances. */
/* !< recommended size for input buffer */
static mut ZSTD_frameHeaderSize_prefix: size_t = unsafe { 5i32 as size_t };
static mut ZSTD_frameHeaderSize_min: size_t = unsafe { 6i32 as size_t };
static mut ZSTD_frameHeaderSize_max: size_t = unsafe { 18i32 as size_t };
static mut ZSTD_skippableHeaderSize: size_t = unsafe { 8i32 as size_t };
static mut ZSTD_defaultCMem: ZSTD_customMem =
    unsafe {
        ZSTD_customMem{customAlloc: None,
                       customFree: None,
                       opaque: 0 as *const libc::c_void as *mut libc::c_void,}
    };
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_createCCtx_advanced(mut customMem:
                                                       ZSTD_customMem)
 -> *mut ZBUFF_CCtx {
    return ZSTD_createCStream_advanced(customMem);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_compressInit_advanced(mut zbc: *mut ZBUFF_CCtx,
                                                     mut dict:
                                                         *const libc::c_void,
                                                     mut dictSize: size_t,
                                                     mut params:
                                                         ZSTD_parameters,
                                                     mut pledgedSrcSize:
                                                         libc::c_ulonglong)
 -> size_t {
    if pledgedSrcSize == 0i32 as libc::c_ulonglong {
        pledgedSrcSize = 0u64.wrapping_sub(1i32 as libc::c_ulonglong)
    }
    return ZSTD_initCStream_advanced(zbc, dict, dictSize, params,
                                     pledgedSrcSize);
}
