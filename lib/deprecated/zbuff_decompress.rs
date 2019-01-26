#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc)]
extern crate libc;
extern "C" {
    /*= Decompression context
 *  When decompressing many times,
 *  it is recommended to allocate a context only once,
 *  and re-use it for each successive compression operation.
 *  This will make workload friendlier for system's memory.
 *  Use one context per thread for parallel execution. */
    pub type ZSTD_DCtx_s;
    /*===== ZSTD_DStream management functions =====*/
    #[no_mangle]
    fn ZSTD_createDStream() -> *mut ZSTD_DStream;
    #[no_mangle]
    fn ZSTD_freeDStream(zds: *mut ZSTD_DStream) -> size_t;
    /*===== Streaming decompression functions =====*/
    #[no_mangle]
    fn ZSTD_initDStream(zds: *mut ZSTD_DStream) -> size_t;
    #[no_mangle]
    fn ZSTD_decompressStream(zds: *mut ZSTD_DStream,
                             output: *mut ZSTD_outBuffer,
                             input: *mut ZSTD_inBuffer) -> size_t;
    /* !< recommended size for input buffer */
    #[no_mangle]
    fn ZSTD_DStreamInSize() -> size_t;
    /* !< recommended size for output buffer. Guarantee to successfully flush at least one complete block in all circumstances. */
    #[no_mangle]
    fn ZSTD_DStreamOutSize() -> size_t;
    /*=====   Advanced Streaming decompression functions  =====*/
    #[no_mangle]
    fn ZSTD_initDStream_usingDict(zds: *mut ZSTD_DStream,
                                  dict: *const libc::c_void, dictSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_createDStream_advanced(customMem: ZSTD_customMem)
     -> *mut ZSTD_DStream;
}
pub type size_t = libc::c_ulong;
pub type ZSTD_DCtx = ZSTD_DCtx_s;
/* ***************************
*  Streaming
****************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_inBuffer_s {
    pub src: *const libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
/*-***************************************************************************
*  Streaming decompression - HowTo
*
*  A ZSTD_DStream object is required to track streaming operations.
*  Use ZSTD_createDStream() and ZSTD_freeDStream() to create/release resources.
*  ZSTD_DStream objects can be re-used multiple times.
*
*  Use ZSTD_initDStream() to start a new decompression operation.
* @return : recommended first input size
*  Alternatively, use advanced API to set specific properties.
*
*  Use ZSTD_decompressStream() repetitively to consume your input.
*  The function will update both `pos` fields.
*  If `input.pos < input.size`, some input has not been consumed.
*  It's up to the caller to present again remaining data.
*  The function tries to flush all data decoded immediately, respecting output buffer size.
*  If `output.pos < output.size`, decoder has flushed everything it could.
*  But if `output.pos == output.size`, there might be some data left within internal buffers.,
*  In which case, call ZSTD_decompressStream() again to flush whatever remains in the buffer.
*  Note : with no additional input provided, amount of data flushed is necessarily <= ZSTD_BLOCKSIZE_MAX.
* @return : 0 when a frame is completely decoded and fully flushed,
*        or an error code, which can be tested using ZSTD_isError(),
*        or any other value > 0, which means there is still some decoding or flushing to do to complete current frame :
*                                the return value is a suggested next input size (just a hint for better latency)
*                                that will never request more than the remaining frame size.
* *******************************************************************************/
/* *< DCtx and DStream are now effectively same object (>= v1.3.0) */
                                 /* For compatibility with versions <= v1.2.0, prefer differentiating them. */
pub type ZSTD_DStream = ZSTD_DCtx;
/*-*************************************************
*  Streaming compression - howto
*
*  A ZBUFF_CCtx object is required to track streaming operation.
*  Use ZBUFF_createCCtx() and ZBUFF_freeCCtx() to create/release resources.
*  ZBUFF_CCtx objects can be reused multiple times.
*
*  Start by initializing ZBUF_CCtx.
*  Use ZBUFF_compressInit() to start a new compression operation.
*  Use ZBUFF_compressInitDictionary() for a compression which requires a dictionary.
*
*  Use ZBUFF_compressContinue() repetitively to consume input stream.
*  *srcSizePtr and *dstCapacityPtr can be any size.
*  The function will report how many bytes were read or written within *srcSizePtr and *dstCapacityPtr.
*  Note that it may not consume the entire input, in which case it's up to the caller to present again remaining data.
*  The content of `dst` will be overwritten (up to *dstCapacityPtr) at each call, so save its content if it matters or change @dst .
*  @return : a hint to preferred nb of bytes to use as input for next function call (it's just a hint, to improve latency)
*            or an error code, which can be tested using ZBUFF_isError().
*
*  At any moment, it's possible to flush whatever data remains within buffer, using ZBUFF_compressFlush().
*  The nb of bytes written into `dst` will be reported into *dstCapacityPtr.
*  Note that the function cannot output more than *dstCapacityPtr,
*  therefore, some content might still be left into internal buffer if *dstCapacityPtr is too small.
*  @return : nb of bytes still present into internal buffer (0 if it's empty)
*            or an error code, which can be tested using ZBUFF_isError().
*
*  ZBUFF_compressEnd() instructs to finish a frame.
*  It will perform a flush and write frame epilogue.
*  The epilogue is required for decoders to consider a frame completed.
*  Similar to ZBUFF_compressFlush(), it may not be able to output the entire internal buffer content if *dstCapacityPtr is too small.
*  In which case, call again ZBUFF_compressFlush() to complete the flush.
*  @return : nb of bytes still present into internal buffer (0 if it's empty)
*            or an error code, which can be tested using ZBUFF_isError().
*
*  Hint : _recommended buffer_ sizes (not compulsory) : ZBUFF_recommendedCInSize() / ZBUFF_recommendedCOutSize()
*  input : ZBUFF_recommendedCInSize==128 KB block size is the internal unit, use this value to reduce intermediate stages (better latency)
*  output : ZBUFF_recommendedCOutSize==ZSTD_compressBound(128 KB) + 3 + 3 : ensures it's always possible to write/flush/end a full block. Skip some buffering.
*  By using both, it ensures that input will be entirely consumed, and output will always contain the result, reducing intermediate buffering.
* **************************************************/
pub type ZBUFF_DCtx = ZSTD_DStream;
/* ! Custom memory allocation :
 *  These prototypes make it possible to pass your own allocation/free functions.
 *  ZSTD_customMem is provided at creation time, using ZSTD_create*_advanced() variants listed below.
 *  All allocation/free operations will be completed using these custom variants instead of regular <stdlib.h> ones.
 */
pub type ZSTD_allocFunction
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
               -> *mut libc::c_void>;
pub type ZSTD_freeFunction
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut libc::c_void,
}
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
    result = ZSTD_decompressStream(zbd, &mut outBuff, &mut inBuff);
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
/* ! ZBUFF_createDCtx_advanced() :
 *  Create a ZBUFF decompression context using external alloc and free functions */
#[no_mangle]
pub unsafe extern "C" fn ZBUFF_createDCtx_advanced(mut customMem:
                                                       ZSTD_customMem)
 -> *mut ZBUFF_DCtx {
    return ZSTD_createDStream_advanced(customMem);
}