#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( extern_types , libc )]
extern crate libc;
extern "C" {
    pub type ZSTD_DCtx_s;
    #[no_mangle]
    fn ZSTD_createDStream() -> *mut ZSTD_DStream;
    #[no_mangle]
    fn ZSTD_freeDStream(zds: *mut ZSTD_DStream) -> size_t;
    #[no_mangle]
    fn ZSTD_initDStream(zds: *mut ZSTD_DStream) -> size_t;
    #[no_mangle]
    fn ZSTD_decompressStream(zds: *mut ZSTD_DStream,
                             output: *mut ZSTD_outBuffer,
                             input: *mut ZSTD_inBuffer) -> size_t;
    #[no_mangle]
    fn ZSTD_DStreamInSize() -> size_t;
    #[no_mangle]
    fn ZSTD_DStreamOutSize() -> size_t;
    #[no_mangle]
    fn ZSTD_initDStream_usingDict(zds: *mut ZSTD_DStream,
                                  dict: *const libc::c_void, dictSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_createDStream_advanced(customMem: ZSTD_customMem)
     -> *mut ZSTD_DStream;
}
/* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut libc::c_void,
    pub size: size_t,
    pub pos: size_t,
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
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
pub type ZSTD_DCtx = ZSTD_DCtx_s;
pub type size_t = libc::c_ulong;
/* *< position where reading stopped. Will be updated. Necessarily 0 <= pos <= size */
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
/* *< recommended size for output buffer. Guarantee to successfully flush at least one complete compressed block in all circumstances. */
/* *< recommended size for input buffer */
pub type ZSTD_DStream = ZSTD_DCtx;
/* ***************************
*  Streaming
****************************/
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
pub type ZBUFF_DCtx = ZSTD_DStream;
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_createDCtx() -> *mut ZBUFF_DCtx {
    return ZSTD_createDStream();
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_freeDCtx(mut zbd: *mut ZBUFF_DCtx) -> size_t {
    return ZSTD_freeDStream(zbd);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_decompressInit(mut zbd: *mut ZBUFF_DCtx)
 -> size_t {
    return ZSTD_initDStream(zbd);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_decompressInitDictionary(mut zbd:
                                                            *mut ZBUFF_DCtx,
                                                        mut dict:
                                                            *const libc::c_void,
                                                        mut dictSize: size_t)
 -> size_t {
    return ZSTD_initDStream_usingDict(zbd, dict, dictSize);
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_decompressContinue(mut zbd: *mut ZBUFF_DCtx,
                                                  mut dst: *mut libc::c_void,
                                                  mut dstCapacityPtr:
                                                      *mut size_t,
                                                  mut src:
                                                      *const libc::c_void,
                                                  mut srcSizePtr: *mut size_t)
 -> size_t {
    let mut outBuff: ZSTD_outBuffer =
        ZSTD_outBuffer_s{dst: 0 as *mut libc::c_void, size: 0, pos: 0,};
    let mut inBuff: ZSTD_inBuffer =
        ZSTD_inBuffer_s{src: 0 as *const libc::c_void, size: 0, pos: 0,};
    let mut result: size_t = 0;
    outBuff.dst = dst;
    outBuff.pos = 0i32 as size_t;
    outBuff.size = *dstCapacityPtr;
    inBuff.src = src;
    inBuff.pos = 0i32 as size_t;
    inBuff.size = *srcSizePtr;
    result =
        ZSTD_decompressStream(zbd, &mut outBuff as *mut ZSTD_outBuffer,
                              &mut inBuff as *mut ZSTD_inBuffer);
    *dstCapacityPtr = outBuff.pos;
    *srcSizePtr = inBuff.pos;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_recommendedDInSize() -> size_t {
    return ZSTD_DStreamInSize();
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_recommendedDOutSize() -> size_t {
    return ZSTD_DStreamOutSize();
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
pub unsafe extern "C" fn ZBUFF_createDCtx_advanced(mut customMem:
                                                       ZSTD_customMem)
 -> *mut ZBUFF_DCtx {
    return ZSTD_createDStream_advanced(customMem);
}
