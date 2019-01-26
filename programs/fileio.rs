#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc, ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    /* **************************************
*  Explicit context
***************************************/
/*= Compression context
 *  When compressing many times,
 *  it is recommended to allocate a context just once, and re-use it for each successive compression operation.
 *  This will make workload friendlier for system's memory.
 *  Use one context per thread for parallel execution in multi-threaded environments. */
    pub type ZSTD_CCtx_s;
    /*= Decompression context
 *  When decompressing many times,
 *  it is recommended to allocate a context only once,
 *  and re-use it for each successive compression operation.
 *  This will make workload friendlier for system's memory.
 *  Use one context per thread for parallel execution. */
    pub type ZSTD_DCtx_s;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn getchar() -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t,
              __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    /* relies on standard C (note : clock_t measurements can be wrong when using multi-threading) */
    #[no_mangle]
    fn UTIL_getTime() -> UTIL_time_t;
    /* returns time span in microseconds */
    #[no_mangle]
    fn UTIL_clockSpanMicro(clockStart: UTIL_time_t) -> U64;
    #[no_mangle]
    fn UTIL_fileExist(filename: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn UTIL_isRegularFile(infilename: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn UTIL_setFileStat(filename: *const libc::c_char, statbuf: *mut stat_t)
     -> libc::c_int;
    #[no_mangle]
    fn UTIL_isDirectory(infilename: *const libc::c_char) -> U32;
    #[no_mangle]
    fn UTIL_getFileStat(infilename: *const libc::c_char, statbuf: *mut stat_t)
     -> libc::c_int;
    #[no_mangle]
    fn UTIL_getFileSize(infilename: *const libc::c_char) -> U64;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn signal(__sig: libc::c_int, __handler: __sighandler_t)
     -> __sighandler_t;
    #[no_mangle]
    fn raise(__sig: libc::c_int) -> libc::c_int;
    /* ! ZSTD_getFrameContentSize() : requires v1.3.0+
 *  `src` should point to the start of a ZSTD encoded frame.
 *  `srcSize` must be at least as large as the frame header.
 *            hint : any size >= `ZSTD_frameHeaderSize_max` is large enough.
 *  @return : - decompressed size of `src` frame content, if known
 *            - ZSTD_CONTENTSIZE_UNKNOWN if the size cannot be determined
 *            - ZSTD_CONTENTSIZE_ERROR if an error occurred (e.g. invalid magic number, srcSize too small)
 *   note 1 : a 0 return value means the frame is valid but "empty".
 *   note 2 : decompressed size is an optional field, it may not be present, typically in streaming mode.
 *            When `return==ZSTD_CONTENTSIZE_UNKNOWN`, data to decompress could be any size.
 *            In which case, it's necessary to use streaming mode to decompress data.
 *            Optionally, application can rely on some implicit limit,
 *            as ZSTD_decompress() only needs an upper bound of decompressed size.
 *            (For example, data could be necessarily cut into blocks <= 16 KB).
 *   note 3 : decompressed size is always present when compression is completed using single-pass functions,
 *            such as ZSTD_compress(), ZSTD_compressCCtx() ZSTD_compress_usingDict() or ZSTD_compress_usingCDict().
 *   note 4 : decompressed size can be very large (64-bits value),
 *            potentially larger than what local system can handle as a single memory segment.
 *            In which case, it's necessary to use streaming mode to decompress data.
 *   note 5 : If source is untrusted, decompressed size could be wrong or intentionally modified.
 *            Always ensure return value fits within application's authorized limits.
 *            Each application can set its own limits.
 *   note 6 : This function replaces ZSTD_getDecompressedSize() */
    #[no_mangle]
    fn ZSTD_getFrameContentSize(src: *const libc::c_void, srcSize: size_t)
     -> libc::c_ulonglong;
    #[no_mangle]
    fn ZSTD_isError(code: size_t) -> libc::c_uint;
    #[no_mangle]
    fn ZSTD_getErrorName(code: size_t) -> *const libc::c_char;
    /* !< maximum compression level available */
    #[no_mangle]
    fn ZSTD_maxCLevel() -> libc::c_int;
    #[no_mangle]
    fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
    #[no_mangle]
    fn ZSTD_freeCStream(zcs: *mut ZSTD_CStream) -> size_t;
    /* *< recommended size for input buffer */
    #[no_mangle]
    fn ZSTD_CStreamInSize() -> size_t;
    /* *< recommended size for output buffer. Guarantee to successfully flush at least one complete compressed block in all circumstances. */
    #[no_mangle]
    fn ZSTD_CStreamOutSize() -> size_t;
    /*===== ZSTD_DStream management functions =====*/
    #[no_mangle]
    fn ZSTD_createDStream() -> *mut ZSTD_DStream;
    #[no_mangle]
    fn ZSTD_freeDStream(zds: *mut ZSTD_DStream) -> size_t;
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
    /* ZSTD_H_235446 */
    /* ***************************************************************************************
 *   ADVANCED AND EXPERIMENTAL FUNCTIONS
 ****************************************************************************************
 * The definitions in the following section are considered experimental.
 * They are provided for advanced scenarios.
 * They should never be used with a dynamic library, as prototypes may change in the future.
 * Use them only in association with static linking.
 * ***************************************************************************************/
    /* ***************************************************************************************
 *   Candidate API for promotion to stable status
 ****************************************************************************************
 * The following symbols and constants form the "staging area" :
 * they are considered to join "stable API" by v1.4.0.
 * The proposal is written so that it can be made stable "as is",
 * though it's still possible to suggest improvements.
 * Staging is in fact last chance for changes,
 * the API is locked once reaching "stable" status.
 * ***************************************************************************************/
    /* ===  Constants   === */
    /* all magic numbers are supposed read/written to/from files/memory using little-endian convention */
    /* valid since v0.8.0 */
    /* valid since v0.7.0 */
    /* all 16 values, from 0x184D2A50 to 0x184D2A5F, signal the beginning of a skippable frame */
    /* ===   query limits   === */
    /* !< minimum negative compression level allowed */
    #[no_mangle]
    fn ZSTD_minCLevel() -> libc::c_int;
    /* ! ZSTD_CCtx_setParameter() :
 *  Set one compression parameter, selected by enum ZSTD_cParameter.
 *  All parameters have valid bounds. Bounds can be queried using ZSTD_cParam_getBounds().
 *  Providing a value beyond bound will either clamp it, or trigger an error (depending on parameter).
 *  Setting a parameter is generally only possible during frame initialization (before starting compression).
 *  Exception : when using multi-threading mode (nbWorkers >= 1),
 *              the following parameters can be updated _during_ compression (within same frame):
 *              => compressionLevel, hashLog, chainLog, searchLog, minMatch, targetLength and strategy.
 *              new parameters will be active for next job only (after a flush()).
 * @return : an error code (which can be tested using ZSTD_isError()).
 */
    #[no_mangle]
    fn ZSTD_CCtx_setParameter(cctx: *mut ZSTD_CCtx, param: ZSTD_cParameter,
                              value: libc::c_int) -> size_t;
    /* ! ZSTD_CCtx_setPledgedSrcSize() :
 *  Total input data size to be compressed as a single frame.
 *  Value will be written in frame header, unless if explicitly forbidden using ZSTD_c_contentSizeFlag.
 *  This value will also be controlled at end of frame, and trigger an error if not respected.
 * @result : 0, or an error code (which can be tested with ZSTD_isError()).
 *  Note 1 : pledgedSrcSize==0 actually means zero, aka an empty frame.
 *           In order to mean "unknown content size", pass constant ZSTD_CONTENTSIZE_UNKNOWN.
 *           ZSTD_CONTENTSIZE_UNKNOWN is default value for any new frame.
 *  Note 2 : pledgedSrcSize is only valid once, for the next frame.
 *           It's discarded at the end of the frame, and replaced by ZSTD_CONTENTSIZE_UNKNOWN.
 *  Note 3 : Whenever all input data is provided and consumed in a single round,
 *           for example with ZSTD_compress2(),
 *           or invoking immediately ZSTD_compressStream2(,,,ZSTD_e_end),
 *           this value is automatically overriden by srcSize instead.
 */
    #[no_mangle]
    fn ZSTD_CCtx_setPledgedSrcSize(cctx: *mut ZSTD_CCtx,
                                   pledgedSrcSize: libc::c_ulonglong)
     -> size_t;
    /* ! ZSTD_CCtx_loadDictionary() :
 *  Create an internal CDict from `dict` buffer.
 *  Decompression will have to use same dictionary.
 * @result : 0, or an error code (which can be tested with ZSTD_isError()).
 *  Special: Loading a NULL (or 0-size) dictionary invalidates previous dictionary,
 *           meaning "return to no-dictionary mode".
 *  Note 1 : Dictionary is sticky, it will be used for all future compressed frames.
 *           To return to "no-dictionary" situation, load a NULL dictionary (or reset parameters).
 *  Note 2 : Loading a dictionary involves building tables.
 *           It's also a CPU consuming operation, with non-negligible impact on latency.
 *           Tables are dependent on compression parameters, and for this reason,
 *           compression parameters can no longer be changed after loading a dictionary.
 *  Note 3 :`dict` content will be copied internally.
 *           Use experimental ZSTD_CCtx_loadDictionary_byReference() to reference content instead.
 *           In such a case, dictionary buffer must outlive its users.
 *  Note 4 : Use ZSTD_CCtx_loadDictionary_advanced()
 *           to precisely select how dictionary content must be interpreted. */
    #[no_mangle]
    fn ZSTD_CCtx_loadDictionary(cctx: *mut ZSTD_CCtx,
                                dict: *const libc::c_void, dictSize: size_t)
     -> size_t;
    /* ! ZSTD_compressStream2() :
 *  Behaves about the same as ZSTD_compressStream, with additional control on end directive.
 *  - Compression parameters are pushed into CCtx before starting compression, using ZSTD_CCtx_set*()
 *  - Compression parameters cannot be changed once compression is started (save a list of exceptions in multi-threading mode)
 *  - outpot->pos must be <= dstCapacity, input->pos must be <= srcSize
 *  - outpot->pos and input->pos will be updated. They are guaranteed to remain below their respective limit.
 *  - When nbWorkers==0 (default), function is blocking : it completes its job before returning to caller.
 *  - When nbWorkers>=1, function is non-blocking : it just acquires a copy of input, and distributes jobs to internal worker threads, flush whatever is available,
 *                                                  and then immediately returns, just indicating that there is some data remaining to be flushed.
 *                                                  The function nonetheless guarantees forward progress : it will return only after it reads or write at least 1+ byte.
 *  - Exception : if the first call requests a ZSTD_e_end directive and provides enough dstCapacity, the function delegates to ZSTD_compress2() which is always blocking.
 *  - @return provides a minimum amount of data remaining to be flushed from internal buffers
 *            or an error code, which can be tested using ZSTD_isError().
 *            if @return != 0, flush is not fully completed, there is still some data left within internal buffers.
 *            This is useful for ZSTD_e_flush, since in this case more flushes are necessary to empty all buffers.
 *            For ZSTD_e_end, @return == 0 when internal buffers are fully flushed and frame is completed.
 *  - after a ZSTD_e_end directive, if internal buffer is not fully flushed (@return != 0),
 *            only ZSTD_e_end or ZSTD_e_flush operations are allowed.
 *            Before starting a new compression job, or changing compression parameters,
 *            it is required to fully flush internal buffers.
 */
    #[no_mangle]
    fn ZSTD_compressStream2(cctx: *mut ZSTD_CCtx, output: *mut ZSTD_outBuffer,
                            input: *mut ZSTD_inBuffer,
                            endOp: ZSTD_EndDirective) -> size_t;
    /* ! ZSTD_frameHeaderSize() :
 *  srcSize must be >= ZSTD_FRAMEHEADERSIZE_PREFIX.
 * @return : size of the Frame Header,
 *           or an error code (if srcSize is too small) */
    #[no_mangle]
    fn ZSTD_frameHeaderSize(src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    /* **************************************
*  Advanced decompression functions
***************************************/
    /* ! ZSTD_isFrame() :
 *  Tells if the content of `buffer` starts with a valid Frame Identifier.
 *  Note : Frame Identifier is 4 bytes. If `size < 4`, @return will always be 0.
 *  Note 2 : Legacy Frame Identifiers are considered valid only if Legacy Support is enabled.
 *  Note 3 : Skippable Frame Identifiers are considered valid. */
    #[no_mangle]
    fn ZSTD_isFrame(buffer: *const libc::c_void, size: size_t)
     -> libc::c_uint;
    /* ! ZSTD_DCtx_setMaxWindowSize() :
 *  Refuses allocating internal buffers for frames requiring a window size larger than provided limit.
 *  This protects a decoder context from reserving too much memory for itself (potential attack scenario).
 *  This parameter is only useful in streaming mode, since no internal buffer is allocated in single-pass mode.
 *  By default, a decompression context accepts all window sizes <= (1 << ZSTD_WINDOWLOG_LIMIT_DEFAULT)
 * @return : 0, or an error code (which can be tested using ZSTD_isError()).
 */
    #[no_mangle]
    fn ZSTD_DCtx_setMaxWindowSize(dctx: *mut ZSTD_DCtx, maxWindowSize: size_t)
     -> size_t;
    /* ZSTD_getFrameProgression() :
 * tells how much data has been ingested (read from input)
 * consumed (input actually compressed) and produced (output) for current frame.
 * Note : (ingested - consumed) is amount of input data buffered internally, not yet compressed.
 * Aggregates progression inside active worker threads.
 */
    #[no_mangle]
    fn ZSTD_getFrameProgression(cctx: *const ZSTD_CCtx)
     -> ZSTD_frameProgression;
    /* ! ZSTD_toFlushNow() :
 *  Tell how many bytes are ready to be flushed immediately.
 *  Useful for multithreading scenarios (nbWorkers >= 1).
 *  Probe the oldest active job, defined as oldest job not yet entirely flushed,
 *  and check its output buffer.
 * @return : amount of data stored in oldest job and ready to be flushed immediately.
 *  if @return == 0, it means either :
 *  + there is no active job (could be checked with ZSTD_frameProgression()), or
 *  + oldest job is still actively compressing data,
 *    but everything it has produced has also been flushed so far,
 *    therefore flush speed is limited by production speed of oldest job
 *    irrespective of the speed of concurrent (and newer) jobs.
 */
    #[no_mangle]
    fn ZSTD_toFlushNow(cctx: *mut ZSTD_CCtx) -> size_t;
    /*=====   Advanced Streaming decompression functions  =====*/
    #[no_mangle]
    fn ZSTD_initDStream_usingDict(zds: *mut ZSTD_DStream,
                                  dict: *const libc::c_void, dictSize: size_t)
     -> size_t;
    #[no_mangle]
    fn ZSTD_resetDStream(zds: *mut ZSTD_DStream) -> size_t;
    /* * ZSTD_getFrameHeader() :
 *  decode Frame Header, or requires larger `srcSize`.
 * @return : 0, `zfhPtr` is correctly filled,
 *          >0, `srcSize` is too small, value is wanted `srcSize` amount,
 *           or an error code, which can be tested using ZSTD_isError() */
    #[no_mangle]
    fn ZSTD_getFrameHeader(zfhPtr: *mut ZSTD_frameHeader,
                           src: *const libc::c_void, srcSize: size_t)
     -> size_t;
    /* ! ZSTD_getErrorCode() :
    convert a `size_t` function result into a `ZSTD_ErrorCode` enum type,
    which can be used to compare with enum list published above */
    #[no_mangle]
    fn ZSTD_getErrorCode(functionResult: size_t) -> ZSTD_ErrorCode;
    #[no_mangle]
    fn backtrace_symbols(__array: *const *mut libc::c_void,
                         __size: libc::c_int) -> *mut *mut libc::c_char;
    #[no_mangle]
    fn backtrace(__array: *mut *mut libc::c_void, __size: libc::c_int)
     -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __syscall_ulong_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atime: __time_t,
    pub st_atimensec: __syscall_ulong_t,
    pub st_mtime: __time_t,
    pub st_mtimensec: __syscall_ulong_t,
    pub st_ctime: __time_t,
    pub st_ctimensec: __syscall_ulong_t,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type clock_t = __clock_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type UTIL_time_t = timespec;
/*-****************************************
*  File functions
******************************************/
pub type stat_t = stat;
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub type ZSTD_CCtx = ZSTD_CCtx_s;
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
/*-***********************************************************************
*  Streaming compression - HowTo
*
*  A ZSTD_CStream object is required to track streaming operation.
*  Use ZSTD_createCStream() and ZSTD_freeCStream() to create/release resources.
*  ZSTD_CStream objects can be reused multiple times on consecutive compression operations.
*  It is recommended to re-use ZSTD_CStream since it will play nicer with system's memory, by re-using already allocated memory.
*
*  For parallel execution, use one separate ZSTD_CStream per thread.
*
*  note : since v1.3.0, ZSTD_CStream and ZSTD_CCtx are the same thing.
*
*  Parameters are sticky : when starting a new compression on the same context,
*  it will re-use the same sticky parameters as previous compression session.
*  When in doubt, it's recommended to fully initialize the context before usage.
*  Use ZSTD_initCStream() to set the parameter to a selected compression level.
*  Use advanced API (ZSTD_CCtx_setParameter(), etc.) to set more specific parameters.
*
*  Use ZSTD_compressStream() as many times as necessary to consume input stream.
*  The function will automatically update both `pos` fields within `input` and `output`.
*  Note that the function may not consume the entire input,
*  for example, because the output buffer is already full,
*  in which case `input.pos < input.size`.
*  The caller must check if input has been entirely consumed.
*  If not, the caller must make some room to receive more compressed data,
*  and then present again remaining input data.
* @return : a size hint, preferred nb of bytes to use as input for next function call
*           or an error code, which can be tested using ZSTD_isError().
*           Note 1 : it's just a hint, to help latency a little, any value will work fine.
*           Note 2 : size hint is guaranteed to be <= ZSTD_CStreamInSize()
*
*  At any moment, it's possible to flush whatever data might remain stuck within internal buffer,
*  using ZSTD_flushStream(). `output->pos` will be updated.
*  Note that, if `output->size` is too small, a single invocation of ZSTD_flushStream() might not be enough (return code > 0).
*  In which case, make some room to receive more compressed data, and call again ZSTD_flushStream().
*  @return : 0 if internal buffers are entirely flushed,
*            >0 if some data still present within internal buffer (the value is minimal estimation of remaining size),
*            or an error code, which can be tested using ZSTD_isError().
*
*  ZSTD_endStream() instructs to finish a frame.
*  It will perform a flush and write frame epilogue.
*  The epilogue is required for decoders to consider a frame completed.
*  flush() operation is the same, and follows same rules as ZSTD_flushStream().
*  @return : 0 if frame fully completed and fully flushed,
*            >0 if some data still present within internal buffer (the value is minimal estimation of remaining size),
*            or an error code, which can be tested using ZSTD_isError().
*
* *******************************************************************/
/* *< CCtx and CStream are now effectively same object (>= v1.3.0) */
                                 /* Continue to distinguish them for compatibility with older versions <= v1.2.0 */
pub type ZSTD_CStream = ZSTD_CCtx;
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
/* **************************************
*  Advanced compression API
***************************************/
/* API design :
 *   Parameters are pushed one by one into an existing context,
 *   using ZSTD_CCtx_set*() functions.
 *   Pushed parameters are sticky : they are valid for next compressed frame, and any subsequent frame.
 *   "sticky" parameters are applicable to `ZSTD_compress2()` and `ZSTD_compressStream*()` !
 *   They do not apply to "simple" one-shot variants such as ZSTD_compressCCtx()
 *
 *   It's possible to reset all parameters to "default" using ZSTD_CCtx_reset().
 *
 *   This API supercedes all other "advanced" API entry points in the experimental section.
 *   In the future, we expect to remove from experimental API entry points which are redundant with this API.
 */
/* Compression strategies, listed from fastest to strongest */
pub type ZSTD_strategy = libc::c_uint;
pub const ZSTD_btultra2: ZSTD_strategy = 9;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const ZSTD_btopt: ZSTD_strategy = 7;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const ZSTD_lazy: ZSTD_strategy = 4;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub const ZSTD_fast: ZSTD_strategy = 1;
pub type ZSTD_cParameter = libc::c_uint;
pub const ZSTD_c_experimentalParam4: ZSTD_cParameter = 1001;
pub const ZSTD_c_experimentalParam3: ZSTD_cParameter = 1000;
pub const ZSTD_c_experimentalParam2: ZSTD_cParameter = 10;
/* note : additional experimental parameters are also available
     * within the experimental section of the API.
     * At the time of this writing, they include :
     * ZSTD_c_rsyncable
     * ZSTD_c_format
     * ZSTD_c_forceMaxWindow
     * ZSTD_c_forceAttachDict
     * Because they are not stable, it's necessary to define ZSTD_STATIC_LINKING_ONLY to access them.
     * note : never ever use experimentalParam? names directly;
     *        also, the enums values themselves are unstable and can still change.
     */
pub const ZSTD_c_experimentalParam1: ZSTD_cParameter = 500;
/* Control the overlap size, as a fraction of window size.
                              * The overlap size is an amount of data reloaded from previous job at the beginning of a new job.
                              * It helps preserve compression ratio, while each job is compressed in parallel.
                              * This value is enforced only when nbWorkers >= 1.
                              * Larger values increase compression ratio, but decrease speed.
                              * Possible values range from 0 to 9 :
                              * - 0 means "default" : value will be determined by the library, depending on strategy
                              * - 1 means "no overlap"
                              * - 9 means "full overlap", using a full window size.
                              * Each intermediate rank increases/decreases load size by a factor 2 :
                              * 9: full window;  8: w/2;  7: w/4;  6: w/8;  5:w/16;  4: w/32;  3:w/64;  2:w/128;  1:no overlap;  0:default
                              * default value varies between 6 and 9, depending on strategy */
pub const ZSTD_c_overlapLog: ZSTD_cParameter = 402;
/* Size of a compression job. This value is enforced only when nbWorkers >= 1.
                              * Each compression job is completed in parallel, so this value can indirectly impact the nb of active threads.
                              * 0 means default, which is dynamically determined based on compression parameters.
                              * Job size must be a minimum of overlap size, or 1 MB, whichever is largest.
                              * The minimum size is automatically and transparently enforced */
pub const ZSTD_c_jobSize: ZSTD_cParameter = 401;
/* multi-threading parameters */
    /* These parameters are only useful if multi-threading is enabled (compiled with build macro ZSTD_MULTITHREAD).
     * They return an error otherwise. */
/* Select how many threads will be spawned to compress in parallel.
                              * When nbWorkers >= 1, triggers asynchronous mode when used with ZSTD_compressStream*() :
                              * ZSTD_compressStream*() consumes input and flush output if possible, but immediately gives back control to caller,
                              * while compression work is performed in parallel, within worker threads.
                              * (note : a strong exception to this rule is when first invocation of ZSTD_compressStream2() sets ZSTD_e_end :
                              *  in which case, ZSTD_compressStream2() delegates to ZSTD_compress2(), which is always a blocking call).
                              * More workers improve speed, but also increase memory usage.
                              * Default value is `0`, aka "single-threaded mode" : no worker is spawned, compression is performed inside Caller's thread, all invocations are blocking */
pub const ZSTD_c_nbWorkers: ZSTD_cParameter = 400;
/* When applicable, dictionary's ID is written into frame header (default:1) */
pub const ZSTD_c_dictIDFlag: ZSTD_cParameter = 202;
/* A 32-bits checksum of content is written at end of frame (default:0) */
pub const ZSTD_c_checksumFlag: ZSTD_cParameter = 201;
/* frame parameters */
/* Content size will be written into frame header _whenever known_ (default:1)
                              * Content size must be known at the beginning of compression.
                              * This is automatically the case when using ZSTD_compress2(),
                              * For streaming variants, content size must be provided with ZSTD_CCtx_setPledgedSrcSize() */
pub const ZSTD_c_contentSizeFlag: ZSTD_cParameter = 200;
/* Frequency of inserting/looking up entries into the LDM hash table.
                              * Must be clamped between 0 and (ZSTD_WINDOWLOG_MAX - ZSTD_HASHLOG_MIN).
                              * Default is MAX(0, (windowLog - ldmHashLog)), optimizing hash table usage.
                              * Larger values improve compression speed.
                              * Deviating far from default value will likely result in a compression ratio decrease.
                              * Special: value 0 means "automatically determine hashRateLog". */
pub const ZSTD_c_ldmHashRateLog: ZSTD_cParameter = 164;
/* Log size of each bucket in the LDM hash table for collision resolution.
                              * Larger values improve collision resolution but decrease compression speed.
                              * The maximum value is ZSTD_LDM_BUCKETSIZELOG_MAX.
                              * Special: value 0 means "use default value" (default: 3). */
pub const ZSTD_c_ldmBucketSizeLog: ZSTD_cParameter = 163;
/* Minimum match size for long distance matcher.
                              * Larger/too small values usually decrease compression ratio.
                              * Must be clamped between ZSTD_LDM_MINMATCH_MIN and ZSTD_LDM_MINMATCH_MAX.
                              * Special: value 0 means "use default value" (default: 64). */
pub const ZSTD_c_ldmMinMatch: ZSTD_cParameter = 162;
/* Size of the table for long distance matching, as a power of 2.
                              * Larger values increase memory usage and compression ratio,
                              * but decrease compression speed.
                              * Must be clamped between ZSTD_HASHLOG_MIN and ZSTD_HASHLOG_MAX
                              * default: windowlog - 7.
                              * Special: value 0 means "automatically determine hashlog". */
pub const ZSTD_c_ldmHashLog: ZSTD_cParameter = 161;
/* LDM mode parameters */
/* Enable long distance matching.
                                     * This parameter is designed to improve compression ratio
                                     * for large inputs, by finding large matches at long distance.
                                     * It increases memory usage and window size.
                                     * Note: enabling this parameter increases default ZSTD_c_windowLog to 128 MB
                                     * except when expressly set to a different value. */
pub const ZSTD_c_enableLongDistanceMatching: ZSTD_cParameter = 160;
/* See ZSTD_strategy enum definition.
                              * The higher the value of selected strategy, the more complex it is,
                              * resulting in stronger and slower compression.
                              * Special: value 0 means "use default strategy". */
pub const ZSTD_c_strategy: ZSTD_cParameter = 107;
/* Impact of this field depends on strategy.
                              * For strategies btopt, btultra & btultra2:
                              *     Length of Match considered "good enough" to stop search.
                              *     Larger values make compression stronger, and slower.
                              * For strategy fast:
                              *     Distance between match sampling.
                              *     Larger values make compression faster, and weaker.
                              * Special: value 0 means "use default targetLength". */
pub const ZSTD_c_targetLength: ZSTD_cParameter = 106;
/* Minimum size of searched matches.
                              * Note that Zstandard can still find matches of smaller size,
                              * it just tweaks its search algorithm to look for this size and larger.
                              * Larger values increase compression and decompression speed, but decrease ratio.
                              * Must be clamped between ZSTD_MINMATCH_MIN and ZSTD_MINMATCH_MAX.
                              * Note that currently, for all strategies < btopt, effective minimum is 4.
                              *                    , for all strategies > fast, effective maximum is 6.
                              * Special: value 0 means "use default minMatchLength". */
pub const ZSTD_c_minMatch: ZSTD_cParameter = 105;
/* Number of search attempts, as a power of 2.
                              * More attempts result in better and slower compression.
                              * This parameter is useless when using "fast" and "dFast" strategies.
                              * Special: value 0 means "use default searchLog". */
pub const ZSTD_c_searchLog: ZSTD_cParameter = 104;
/* Size of the multi-probe search table, as a power of 2.
                              * Resulting memory usage is (1 << (chainLog+2)).
                              * Must be clamped between ZSTD_CHAINLOG_MIN and ZSTD_CHAINLOG_MAX.
                              * Larger tables result in better and slower compression.
                              * This parameter is useless when using "fast" strategy.
                              * It's still useful when using "dfast" strategy,
                              * in which case it defines a secondary probe table.
                              * Special: value 0 means "use default chainLog". */
pub const ZSTD_c_chainLog: ZSTD_cParameter = 103;
/* Size of the initial probe table, as a power of 2.
                              * Resulting memory usage is (1 << (hashLog+2)).
                              * Must be clamped between ZSTD_HASHLOG_MIN and ZSTD_HASHLOG_MAX.
                              * Larger tables improve compression ratio of strategies <= dFast,
                              * and improve speed of strategies > dFast.
                              * Special: value 0 means "use default hashLog". */
pub const ZSTD_c_hashLog: ZSTD_cParameter = 102;
/* Maximum allowed back-reference distance, expressed as power of 2.
                              * Must be clamped between ZSTD_WINDOWLOG_MIN and ZSTD_WINDOWLOG_MAX.
                              * Special: value 0 means "use default windowLog".
                              * Note: Using a windowLog greater than ZSTD_WINDOWLOG_LIMIT_DEFAULT
                              *       requires explicitly allowing such window size at decompression stage if using streaming. */
pub const ZSTD_c_windowLog: ZSTD_cParameter = 101;
/* compression parameters */
/* Update all compression parameters according to pre-defined cLevel table
                              * Default level is ZSTD_CLEVEL_DEFAULT==3.
                              * Special: value 0 means default, which is controlled by ZSTD_CLEVEL_DEFAULT.
                              * Note 1 : it's possible to pass a negative compression level.
                              * Note 2 : setting a level sets all default values of other compression parameters */
pub const ZSTD_c_compressionLevel: ZSTD_cParameter = 100;
pub type ZSTD_EndDirective = libc::c_uint;
/* flush any remaining data _and_ close current frame.
                        * note that frame is only closed after compressed data is fully flushed (return value == 0).
                        * After that point, any additional data starts a new frame.
                        * note : each frame is independent (does not reference any content from previous frame). */
pub const ZSTD_e_end: ZSTD_EndDirective = 2;
/* flush any data provided so far,
                        * it creates (at least) one new block, that can be decoded immediately on reception;
                        * frame will continue: any future data can still reference previously compressed data, improving compression. */
pub const ZSTD_e_flush: ZSTD_EndDirective = 1;
/* collect more data, encoder decides when to output compressed result, for optimal compression ratio */
pub const ZSTD_e_continue: ZSTD_EndDirective = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_compressionParameters {
    pub windowLog: libc::c_uint,
    pub chainLog: libc::c_uint,
    pub hashLog: libc::c_uint,
    pub searchLog: libc::c_uint,
    pub minMatch: libc::c_uint,
    pub targetLength: libc::c_uint,
    pub strategy: ZSTD_strategy,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_frameProgression {
    pub ingested: libc::c_ulonglong,
    pub consumed: libc::c_ulonglong,
    pub produced: libc::c_ulonglong,
    pub flushed: libc::c_ulonglong,
    pub currentJobID: libc::c_uint,
    pub nbActiveWorkers: libc::c_uint,
}
/*-
  Buffer-less streaming decompression (synchronous mode)

  A ZSTD_DCtx object is required to track streaming operations.
  Use ZSTD_createDCtx() / ZSTD_freeDCtx() to manage it.
  A ZSTD_DCtx object can be re-used multiple times.

  First typical operation is to retrieve frame parameters, using ZSTD_getFrameHeader().
  Frame header is extracted from the beginning of compressed frame, so providing only the frame's beginning is enough.
  Data fragment must be large enough to ensure successful decoding.
 `ZSTD_frameHeaderSize_max` bytes is guaranteed to always be large enough.
  @result : 0 : successful decoding, the `ZSTD_frameHeader` structure is correctly filled.
           >0 : `srcSize` is too small, please provide at least @result bytes on next attempt.
           errorCode, which can be tested using ZSTD_isError().

  It fills a ZSTD_frameHeader structure with important information to correctly decode the frame,
  such as the dictionary ID, content size, or maximum back-reference distance (`windowSize`).
  Note that these values could be wrong, either because of data corruption, or because a 3rd party deliberately spoofs false information.
  As a consequence, check that values remain within valid application range.
  For example, do not allocate memory blindly, check that `windowSize` is within expectation.
  Each application can set its own limits, depending on local restrictions.
  For extended interoperability, it is recommended to support `windowSize` of at least 8 MB.

  ZSTD_decompressContinue() needs previous data blocks during decompression, up to `windowSize` bytes.
  ZSTD_decompressContinue() is very sensitive to contiguity,
  if 2 blocks don't follow each other, make sure that either the compressor breaks contiguity at the same place,
  or that previous contiguous segment is large enough to properly handle maximum back-reference distance.
  There are multiple ways to guarantee this condition.

  The most memory efficient way is to use a round buffer of sufficient size.
  Sufficient size is determined by invoking ZSTD_decodingBufferSize_min(),
  which can @return an error code if required value is too large for current system (in 32-bits mode).
  In a round buffer methodology, ZSTD_decompressContinue() decompresses each block next to previous one,
  up to the moment there is not enough room left in the buffer to guarantee decoding another full block,
  which maximum size is provided in `ZSTD_frameHeader` structure, field `blockSizeMax`.
  At which point, decoding can resume from the beginning of the buffer.
  Note that already decoded data stored in the buffer should be flushed before being overwritten.

  There are alternatives possible, for example using two or more buffers of size `windowSize` each, though they consume more memory.

  Finally, if you control the compression process, you can also ignore all buffer size rules,
  as long as the encoder and decoder progress in "lock-step",
  aka use exactly the same buffer sizes, break contiguity at the same place, etc.

  Once buffers are setup, start decompression, with ZSTD_decompressBegin().
  If decompression requires a dictionary, use ZSTD_decompressBegin_usingDict() or ZSTD_decompressBegin_usingDDict().

  Then use ZSTD_nextSrcSizeToDecompress() and ZSTD_decompressContinue() alternatively.
  ZSTD_nextSrcSizeToDecompress() tells how many bytes to provide as 'srcSize' to ZSTD_decompressContinue().
  ZSTD_decompressContinue() requires this _exact_ amount of bytes, or it will fail.

 @result of ZSTD_decompressContinue() is the number of bytes regenerated within 'dst' (necessarily <= dstCapacity).
  It can be zero : it just means ZSTD_decompressContinue() has decoded some metadata item.
  It can also be an error code, which can be tested with ZSTD_isError().

  A frame is fully decoded when ZSTD_nextSrcSizeToDecompress() returns zero.
  Context can then be reset to start a new decompression.

  Note : it's possible to know if next input to present is a header or a block, using ZSTD_nextInputType().
  This information is not required to properly decode a frame.

  == Special case : skippable frames ==

  Skippable frames allow integration of user-defined data into a flow of concatenated frames.
  Skippable frames will be ignored (skipped) by decompressor.
  The format of skippable frames is as follows :
  a) Skippable frame ID - 4 Bytes, Little endian format, any value from 0x184D2A50 to 0x184D2A5F
  b) Frame Size - 4 Bytes, Little endian format, unsigned 32-bits
  c) Frame Content - any content (User Data) of length equal to Frame Size
  For skippable frames ZSTD_getFrameHeader() returns zfhPtr->frameType==ZSTD_skippableFrame.
  For skippable frames ZSTD_decompressContinue() always returns 0 : it only skips the content.
*/
/*=====   Buffer-less streaming decompression functions  =====*/
pub type ZSTD_frameType_e = libc::c_uint;
pub const ZSTD_skippableFrame: ZSTD_frameType_e = 1;
pub const ZSTD_frame: ZSTD_frameType_e = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZSTD_frameHeader {
    pub frameContentSize: libc::c_ulonglong,
    pub windowSize: libc::c_ulonglong,
    pub blockSizeMax: libc::c_uint,
    pub frameType: ZSTD_frameType_e,
    pub headerSize: libc::c_uint,
    pub dictID: libc::c_uint,
    pub checksumFlag: libc::c_uint,
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
/* ZSTD_compressionParameters */
/* *************************************
*  Special i/o constants
**************************************/
/*-*************************************
*  Types
***************************************/
pub type FIO_compressionType_t = libc::c_uint;
pub const FIO_lz4Compression: FIO_compressionType_t = 4;
pub const FIO_lzmaCompression: FIO_compressionType_t = 3;
pub const FIO_xzCompression: FIO_compressionType_t = 2;
pub const FIO_gzipCompression: FIO_compressionType_t = 1;
pub const FIO_zstdCompression: FIO_compressionType_t = 0;
/*-************************************************************
* Avoid fseek()'s 2GiB barrier with MSVC, macOS, *BSD, MinGW
***************************************************************/
/* No point defining Large file for 64 bit */
/*-*************************************
*  Parameters: Typedefs
***************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FIO_prefs_s {
    pub compressionType: FIO_compressionType_t,
    pub sparseFileSupport: U32,
    pub dictIDFlag: U32,
    pub checksumFlag: U32,
    pub blockSize: U32,
    pub overlapLog: libc::c_uint,
    pub adaptiveMode: U32,
    pub rsyncable: U32,
    pub minAdaptLevel: libc::c_int,
    pub maxAdaptLevel: libc::c_int,
    pub ldmFlag: U32,
    pub ldmHashLog: U32,
    pub ldmMinMatch: U32,
    pub ldmBucketSizeLog: U32,
    pub ldmHashRateLog: U32,
    pub removeSrcFile: U32,
    pub overwrite: U32,
    pub memLimit: libc::c_uint,
    pub nbWorkers: libc::c_uint,
}
pub type FIO_prefs_t = FIO_prefs_s;
pub type FIO_display_prefs_t = FIO_display_prefs_s;
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* *************************************
*  Compiler Options
***************************************/
/* Visual */
/*-*************************************
*  Includes
***************************************/
/* fprintf, fopen, fread, _fileno, stdin, stdout */
/* malloc, free */
/* strcmp, strlen */
/* errno */
/* U32, U64 */
/* ZSTD_magicNumber, ZSTD_frameHeaderSize_max */
/*-*************************************
*  Constants
***************************************/
/* 8 MB */
/* protection against large input (attack scenario) */
/*-*************************************
*  Macros
***************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FIO_display_prefs_s {
    pub displayLevel: libc::c_int,
    pub noProgress: U32,
}
/* **********************************************************************
 *  Compression
 ************************************************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cRess_t {
    pub srcFile: *mut FILE,
    pub dstFile: *mut FILE,
    pub srcBuffer: *mut libc::c_void,
    pub srcBufferSize: size_t,
    pub dstBuffer: *mut libc::c_void,
    pub dstBufferSize: size_t,
    pub cctx: *mut ZSTD_CStream,
}
pub type speedChange_e = libc::c_uint;
pub const faster: speedChange_e = 2;
pub const slower: speedChange_e = 1;
pub const noChange: speedChange_e = 0;
/* #ifndef ZSTD_NOCOMPRESS */
/* **************************************************************************
 *  Decompression
 ***************************************************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct dRess_t {
    pub srcBuffer: *mut libc::c_void,
    pub srcBufferSize: size_t,
    pub srcBufferLoaded: size_t,
    pub dstBuffer: *mut libc::c_void,
    pub dstBufferSize: size_t,
    pub dctx: *mut ZSTD_DStream,
    pub dstFile: *mut FILE,
}
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
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
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
/* **************************************************************************
 *  .zst file info (--list command)
 ***************************************************************************/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct fileInfo_t {
    pub decompressedSize: U64,
    pub compressedSize: U64,
    pub windowSize: U64,
    pub numActualFrames: libc::c_int,
    pub numSkippableFrames: libc::c_int,
    pub decompUnavailable: libc::c_int,
    pub usesCheck: libc::c_int,
    pub nbFiles: U32,
}
pub type InfoError = libc::c_uint;
pub const info_truncated_input: InfoError = 4;
pub const info_file_error: InfoError = 3;
pub const info_not_zstd: InfoError = 2;
pub const info_frame_error: InfoError = 1;
pub const info_success: InfoError = 0;
unsafe extern "C" fn MEM_isLittleEndian() -> libc::c_uint {
    /* don't use static : performance detrimental  */
    let one: unnamed = unnamed{u: 1i32 as U32,};
    return one.c[0usize] as libc::c_uint;
}
unsafe extern "C" fn MEM_read16(mut ptr: *const libc::c_void) -> U16 {
    return (*(ptr as *const unalign16)).v;
}
unsafe extern "C" fn MEM_read32(mut ptr: *const libc::c_void) -> U32 {
    return (*(ptr as *const unalign32)).v;
}
/* MEM_FORCE_MEMORY_ACCESS */
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
}
/*=== Little endian r/w ===*/
unsafe extern "C" fn MEM_readLE16(mut memPtr: *const libc::c_void) -> U16 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read16(memPtr)
    } else {
        let mut p: *const BYTE = memPtr as *const BYTE;
        return (*p.offset(0isize) as libc::c_int +
                    ((*p.offset(1isize) as libc::c_int) << 8i32)) as U16
    };
}
unsafe extern "C" fn MEM_readLE24(mut memPtr: *const libc::c_void) -> U32 {
    return (MEM_readLE16(memPtr) as libc::c_int +
                ((*(memPtr as *const BYTE).offset(2isize) as libc::c_int) <<
                     16i32)) as U32;
}
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const libc::c_void) -> U32 {
    if 0 != MEM_isLittleEndian() {
        return MEM_read32(memPtr)
    } else { return MEM_swap32(MEM_read32(memPtr)) };
}
#[no_mangle]
pub unsafe extern "C" fn FIO_createPreferences() -> *mut FIO_prefs_t {
    let ret: *mut FIO_prefs_t =
        malloc(::std::mem::size_of::<FIO_prefs_t>() as libc::c_ulong) as
            *mut FIO_prefs_t;
    if ret.is_null() {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 321i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    21i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Allocation error : not enough memory\x00" as *const u8
                        as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(21i32);
    }
    (*ret).compressionType = FIO_zstdCompression;
    (*ret).overwrite = 0i32 as U32;
    (*ret).sparseFileSupport = 1i32 as U32;
    (*ret).dictIDFlag = 1i32 as U32;
    (*ret).checksumFlag = 1i32 as U32;
    (*ret).removeSrcFile = 0i32 as U32;
    (*ret).memLimit = 0i32 as libc::c_uint;
    (*ret).nbWorkers = 1i32 as libc::c_uint;
    (*ret).blockSize = 0i32 as U32;
    (*ret).overlapLog = 9999i32 as libc::c_uint;
    (*ret).adaptiveMode = 0i32 as U32;
    (*ret).rsyncable = 0i32 as U32;
    (*ret).minAdaptLevel = -50i32;
    (*ret).maxAdaptLevel = 22i32;
    (*ret).ldmFlag = 0i32 as U32;
    (*ret).ldmHashLog = 0i32 as U32;
    (*ret).ldmMinMatch = 0i32 as U32;
    (*ret).ldmBucketSizeLog = 9999i32 as U32;
    (*ret).ldmHashRateLog = 9999i32 as U32;
    return ret;
}
static mut g_display_prefs: FIO_display_prefs_t =
    FIO_display_prefs_s{displayLevel: 2i32, noProgress: 0i32 as U32,};
#[no_mangle]
pub unsafe extern "C" fn FIO_freePreferences(prefs: *mut FIO_prefs_t) {
    free(prefs as *mut libc::c_void);
}
/*-*************************************
*  Parameters
***************************************/
#[no_mangle]
pub unsafe extern "C" fn FIO_setCompressionType(prefs: *mut FIO_prefs_t,
                                                mut compressionType:
                                                    FIO_compressionType_t) {
    (*prefs).compressionType = compressionType;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_overwriteMode(prefs: *mut FIO_prefs_t) {
    (*prefs).overwrite = 1i32 as U32;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setAdaptiveMode(prefs: *mut FIO_prefs_t,
                                             mut adapt: libc::c_uint) {
    if adapt > 0i32 as libc::c_uint &&
           (*prefs).nbWorkers == 0i32 as libc::c_uint {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 399i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    1i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Adaptive mode is not compatible with single thread mode \n\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(1i32);
    }
    (*prefs).adaptiveMode = adapt;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setAdaptMin(prefs: *mut FIO_prefs_t,
                                         mut minCLevel: libc::c_int) {
    if minCLevel >= ZSTD_minCLevel() {
    } else {
        __assert_fail(b"minCLevel >= ZSTD_minCLevel()\x00" as *const u8 as
                          *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                          as *const u8 as *const libc::c_char,
                      412i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 46],
                                                &[libc::c_char; 46]>(b"void FIO_setAdaptMin(FIO_prefs_t *const, int)\x00")).as_ptr());
    };
    (*prefs).minAdaptLevel = minCLevel;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setAdaptMax(prefs: *mut FIO_prefs_t,
                                         mut maxCLevel: libc::c_int) {
    (*prefs).maxAdaptLevel = maxCLevel;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setBlockSize(prefs: *mut FIO_prefs_t,
                                          mut blockSize: libc::c_uint) {
    if 0 != blockSize && (*prefs).nbWorkers == 0i32 as libc::c_uint {
        if g_display_prefs.displayLevel >= 2i32 {
            fprintf(stderr,
                    b"Setting block size is useless in single-thread mode \n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    (*prefs).blockSize = blockSize;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setChecksumFlag(prefs: *mut FIO_prefs_t,
                                             mut checksumFlag: libc::c_uint) {
    (*prefs).checksumFlag = checksumFlag;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setDictIDFlag(prefs: *mut FIO_prefs_t,
                                           mut dictIDFlag: libc::c_uint) {
    (*prefs).dictIDFlag = dictIDFlag;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmBucketSizeLog(prefs: *mut FIO_prefs_t,
                                                 mut ldmBucketSizeLog:
                                                     libc::c_uint) {
    (*prefs).ldmBucketSizeLog = ldmBucketSizeLog;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmFlag(prefs: *mut FIO_prefs_t,
                                        mut ldmFlag: libc::c_uint) {
    (*prefs).ldmFlag = (ldmFlag > 0i32 as libc::c_uint) as libc::c_int as U32;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmHashRateLog(prefs: *mut FIO_prefs_t,
                                               mut ldmHashRateLog:
                                                   libc::c_uint) {
    (*prefs).ldmHashRateLog = ldmHashRateLog;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmHashLog(prefs: *mut FIO_prefs_t,
                                           mut ldmHashLog: libc::c_uint) {
    (*prefs).ldmHashLog = ldmHashLog;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmMinMatch(prefs: *mut FIO_prefs_t,
                                            mut ldmMinMatch: libc::c_uint) {
    (*prefs).ldmMinMatch = ldmMinMatch;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setMemLimit(prefs: *mut FIO_prefs_t,
                                         mut memLimit: libc::c_uint) {
    (*prefs).memLimit = memLimit;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setNbWorkers(prefs: *mut FIO_prefs_t,
                                          mut nbWorkers: libc::c_uint) {
    (*prefs).nbWorkers = nbWorkers;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setOverlapLog(prefs: *mut FIO_prefs_t,
                                           mut overlapLog: libc::c_uint) {
    if 0 != overlapLog && (*prefs).nbWorkers == 0i32 as libc::c_uint {
        if g_display_prefs.displayLevel >= 2i32 {
            fprintf(stderr,
                    b"Setting overlapLog is useless in single-thread mode \n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    (*prefs).overlapLog = overlapLog;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setRemoveSrcFile(prefs: *mut FIO_prefs_t,
                                              mut flag: libc::c_uint) {
    (*prefs).removeSrcFile =
        (flag > 0i32 as libc::c_uint) as libc::c_int as U32;
}
/* *< 0: no sparse; 1: disable on stdout; 2: always enabled */
#[no_mangle]
pub unsafe extern "C" fn FIO_setSparseWrite(prefs: *mut FIO_prefs_t,
                                            mut sparse: libc::c_uint) {
    (*prefs).sparseFileSupport = sparse;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setRsyncable(prefs: *mut FIO_prefs_t,
                                          mut rsyncable: libc::c_uint) {
    if rsyncable > 0i32 as libc::c_uint &&
           (*prefs).nbWorkers == 0i32 as libc::c_uint {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 405i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    1i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Rsyncable mode is not compatible with single thread mode \n\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(1i32);
    }
    (*prefs).rsyncable = rsyncable;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setNoProgress(mut noProgress: libc::c_uint) {
    g_display_prefs.noProgress = noProgress;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setNotificationLevel(mut level: libc::c_uint) {
    g_display_prefs.displayLevel = level as libc::c_int;
}
/*-*************************************
*  Single File functions
***************************************/
/** FIO_compressFilename() :
    @return : 0 == ok;  1 == pb with src file. */
#[no_mangle]
pub unsafe extern "C" fn FIO_compressFilename(prefs: *mut FIO_prefs_t,
                                              mut dstFileName:
                                                  *const libc::c_char,
                                              mut srcFileName:
                                                  *const libc::c_char,
                                              mut dictFileName:
                                                  *const libc::c_char,
                                              mut compressionLevel:
                                                  libc::c_int,
                                              mut comprParams:
                                                  ZSTD_compressionParameters)
 -> libc::c_int {
    let start: clock_t = clock();
    let fileSize: U64 = UTIL_getFileSize(srcFileName);
    let srcSize: U64 =
        (if fileSize == -1i32 as U64 {
             0u64.wrapping_sub(1i32 as libc::c_ulonglong)
         } else { fileSize as libc::c_ulonglong }) as U64;
    let ress: cRess_t =
        FIO_createCResources(prefs, dictFileName, compressionLevel, srcSize,
                             comprParams);
    let result: libc::c_int =
        FIO_compressFilename_srcFile(prefs, ress, dstFileName, srcFileName,
                                     compressionLevel);
    let seconds: libc::c_double =
        (clock() - start) as libc::c_double /
            1000000i32 as __clock_t as libc::c_double;
    if g_display_prefs.displayLevel >= 4i32 {
        fprintf(stderr,
                b"Completed in %.2f sec \n\x00" as *const u8 as
                    *const libc::c_char, seconds);
    }
    FIO_freeCResources(ress);
    return result;
}
unsafe extern "C" fn FIO_createCResources(prefs: *mut FIO_prefs_t,
                                          mut dictFileName:
                                              *const libc::c_char,
                                          mut cLevel: libc::c_int,
                                          mut srcSize: U64,
                                          mut comprParams:
                                              ZSTD_compressionParameters)
 -> cRess_t {
    let mut ress: cRess_t =
        cRess_t{srcFile: 0 as *mut FILE,
                dstFile: 0 as *mut FILE,
                srcBuffer: 0 as *mut libc::c_void,
                srcBufferSize: 0,
                dstBuffer: 0 as *mut libc::c_void,
                dstBufferSize: 0,
                cctx: 0 as *mut ZSTD_CStream,};
    memset(&mut ress as *mut cRess_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<cRess_t>() as libc::c_ulong);
    if g_display_prefs.displayLevel >= 6i32 {
        fprintf(stderr,
                b"FIO_createCResources \n\x00" as *const u8 as
                    *const libc::c_char);
    }
    ress.cctx = ZSTD_createCCtx();
    if ress.cctx.is_null() {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 637i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    30i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"allocation error (%s): can\'t create ZSTD_CCtx\x00" as
                        *const u8 as *const libc::c_char,
                    strerror(*__errno_location()));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(30i32);
    }
    ress.srcBufferSize = ZSTD_CStreamInSize();
    ress.srcBuffer = malloc(ress.srcBufferSize);
    ress.dstBufferSize = ZSTD_CStreamOutSize();
    ress.dstBuffer = malloc(ress.dstBufferSize);
    if ress.srcBuffer.is_null() || ress.dstBuffer.is_null() {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 643i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    31i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"allocation error : not enough memory\x00" as *const u8
                        as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(31i32);
    }
    let mut dictBuffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let dictBuffSize: size_t =
        FIO_createDictBuffer(&mut dictBuffer, dictFileName);
    if !dictFileName.is_null() && dictBuffer.is_null() {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 649i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    32i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"allocation error : can\'t create dictBuffer\x00" as
                        *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(32i32);
    }
    if 0 != (*prefs).adaptiveMode && 0 == (*prefs).ldmFlag &&
           0 == comprParams.windowLog {
        comprParams.windowLog = 23i32 as libc::c_uint
    }
    let mut err: size_t = 0;
    err = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_contentSizeFlag, 1i32);
    if 0 != ZSTD_isError(err) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_contentSizeFlag, 1)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 654i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_0: size_t = 0;
    err_0 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_dictIDFlag,
                               (*prefs).dictIDFlag as libc::c_int);
    if 0 != ZSTD_isError(err_0) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_dictIDFlag, prefs->dictIDFlag)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 655i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_0));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_1: size_t = 0;
    err_1 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_checksumFlag,
                               (*prefs).checksumFlag as libc::c_int);
    if 0 != ZSTD_isError(err_1) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_checksumFlag, prefs->checksumFlag)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 656i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_1));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_2: size_t = 0;
    err_2 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_compressionLevel, cLevel);
    if 0 != ZSTD_isError(err_2) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_compressionLevel, cLevel)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 658i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_2));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_3: size_t = 0;
    err_3 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_enableLongDistanceMatching,
                               (*prefs).ldmFlag as libc::c_int);
    if 0 != ZSTD_isError(err_3) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_enableLongDistanceMatching, prefs->ldmFlag)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 660i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_3));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_4: size_t = 0;
    err_4 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmHashLog,
                               (*prefs).ldmHashLog as libc::c_int);
    if 0 != ZSTD_isError(err_4) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmHashLog, prefs->ldmHashLog)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 661i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_4));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_5: size_t = 0;
    err_5 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmMinMatch,
                               (*prefs).ldmMinMatch as libc::c_int);
    if 0 != ZSTD_isError(err_5) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmMinMatch, prefs->ldmMinMatch)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 662i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_5));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    if (*prefs).ldmBucketSizeLog != 9999i32 as libc::c_uint {
        let mut err_6: size_t = 0;
        err_6 =
            ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmBucketSizeLog,
                                   (*prefs).ldmBucketSizeLog as libc::c_int);
        if 0 != ZSTD_isError(err_6) {
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"%s \n\x00" as *const u8 as *const libc::c_char,
                        b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmBucketSizeLog, prefs->ldmBucketSizeLog)\x00"
                            as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 664i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 11i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                        ZSTD_getErrorName(err_6));
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(11i32);
        }
    }
    if (*prefs).ldmHashRateLog != 9999i32 as libc::c_uint {
        let mut err_7: size_t = 0;
        err_7 =
            ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmHashRateLog,
                                   (*prefs).ldmHashRateLog as libc::c_int);
        if 0 != ZSTD_isError(err_7) {
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"%s \n\x00" as *const u8 as *const libc::c_char,
                        b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmHashRateLog, prefs->ldmHashRateLog)\x00"
                            as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 667i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 11i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                        ZSTD_getErrorName(err_7));
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(11i32);
        }
    }
    let mut err_8: size_t = 0;
    err_8 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_windowLog,
                               comprParams.windowLog as libc::c_int);
    if 0 != ZSTD_isError(err_8) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_windowLog, comprParams.windowLog)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 670i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_8));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_9: size_t = 0;
    err_9 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_chainLog,
                               comprParams.chainLog as libc::c_int);
    if 0 != ZSTD_isError(err_9) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_chainLog, comprParams.chainLog)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 671i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_9));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_10: size_t = 0;
    err_10 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_hashLog,
                               comprParams.hashLog as libc::c_int);
    if 0 != ZSTD_isError(err_10) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_hashLog, comprParams.hashLog)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 672i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_10));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_11: size_t = 0;
    err_11 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_searchLog,
                               comprParams.searchLog as libc::c_int);
    if 0 != ZSTD_isError(err_11) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_searchLog, comprParams.searchLog)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 673i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_11));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_12: size_t = 0;
    err_12 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_minMatch,
                               comprParams.minMatch as libc::c_int);
    if 0 != ZSTD_isError(err_12) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_minMatch, comprParams.minMatch)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 674i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_12));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_13: size_t = 0;
    err_13 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_targetLength,
                               comprParams.targetLength as libc::c_int);
    if 0 != ZSTD_isError(err_13) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_targetLength, comprParams.targetLength)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 675i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_13));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_14: size_t = 0;
    err_14 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_strategy,
                               comprParams.strategy as libc::c_int);
    if 0 != ZSTD_isError(err_14) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_strategy, comprParams.strategy)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 676i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_14));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    if g_display_prefs.displayLevel >= 5i32 {
        fprintf(stderr,
                b"set nb workers = %u \n\x00" as *const u8 as
                    *const libc::c_char, (*prefs).nbWorkers);
    }
    let mut err_15: size_t = 0;
    err_15 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_nbWorkers,
                               (*prefs).nbWorkers as libc::c_int);
    if 0 != ZSTD_isError(err_15) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_nbWorkers, prefs->nbWorkers)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 680i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_15));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_16: size_t = 0;
    err_16 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_jobSize,
                               (*prefs).blockSize as libc::c_int);
    if 0 != ZSTD_isError(err_16) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_jobSize, prefs->blockSize)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 681i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_16));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    if (*prefs).overlapLog != 9999i32 as libc::c_uint {
        if g_display_prefs.displayLevel >= 3i32 {
            fprintf(stderr,
                    b"set overlapLog = %u \n\x00" as *const u8 as
                        *const libc::c_char, (*prefs).overlapLog);
        }
        let mut err_17: size_t = 0;
        err_17 =
            ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_overlapLog,
                                   (*prefs).overlapLog as libc::c_int);
        if 0 != ZSTD_isError(err_17) {
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"%s \n\x00" as *const u8 as *const libc::c_char,
                        b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_overlapLog, prefs->overlapLog)\x00"
                            as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 684i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 11i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                        ZSTD_getErrorName(err_17));
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(11i32);
        }
    }
    let mut err_18: size_t = 0;
    err_18 =
        ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_experimentalParam1,
                               (*prefs).rsyncable as libc::c_int);
    if 0 != ZSTD_isError(err_18) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_experimentalParam1, prefs->rsyncable)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 686i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_18));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_19: size_t = 0;
    err_19 =
        ZSTD_CCtx_setPledgedSrcSize(ress.cctx, srcSize as libc::c_ulonglong);
    if 0 != ZSTD_isError(err_19) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setPledgedSrcSize(ress.cctx, srcSize)\x00" as
                        *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 689i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_19));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_20: size_t = 0;
    err_20 = ZSTD_CCtx_loadDictionary(ress.cctx, dictBuffer, dictBuffSize);
    if 0 != ZSTD_isError(err_20) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_loadDictionary(ress.cctx, dictBuffer, dictBuffSize)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 690i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_20));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    let mut err_21: size_t = 0;
    err_21 =
        ZSTD_CCtx_setPledgedSrcSize(ress.cctx,
                                    0u64.wrapping_sub(1i32 as
                                                          libc::c_ulonglong));
    if 0 != ZSTD_isError(err_21) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_CCtx_setPledgedSrcSize(ress.cctx, (0ULL - 1))\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 691i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_21));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    free(dictBuffer);
    return ress;
}
/* ! FIO_createDictBuffer() :
 *  creates a buffer, pointed by `*bufferPtr`,
 *  loads `filename` content into it, up to DICTSIZE_MAX bytes.
 * @return : loaded size
 *  if fileName==NULL, returns 0 and a NULL pointer
 */
unsafe extern "C" fn FIO_createDictBuffer(mut bufferPtr:
                                              *mut *mut libc::c_void,
                                          mut fileName: *const libc::c_char)
 -> size_t {
    let mut fileHandle: *mut FILE = 0 as *mut FILE;
    let mut fileSize: U64 = 0;
    if !bufferPtr.is_null() {
    } else {
        __assert_fail(b"bufferPtr != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                          as *const u8 as *const libc::c_char,
                      587i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"size_t FIO_createDictBuffer(void **, const char *)\x00")).as_ptr());
    };
    *bufferPtr = 0 as *mut libc::c_void;
    if fileName.is_null() { return 0i32 as size_t }
    if g_display_prefs.displayLevel >= 4i32 {
        fprintf(stderr,
                b"Loading %s as dictionary \n\x00" as *const u8 as
                    *const libc::c_char, fileName);
    }
    fileHandle =
        fopen(fileName, b"rb\x00" as *const u8 as *const libc::c_char);
    if fileHandle.is_null() {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 593i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    31i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s: %s\x00" as *const u8 as *const libc::c_char,
                    fileName, strerror(*__errno_location()));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(31i32);
    }
    fileSize = UTIL_getFileSize(fileName);
    if fileSize > (32i32 * (1i32 << 20i32)) as libc::c_ulong {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 598i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    32i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Dictionary file %s is too large (> %u MB)\x00" as
                        *const u8 as *const libc::c_char, fileName,
                    32i32 * (1i32 << 20i32) >> 20i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(32i32);
    }
    *bufferPtr = malloc(fileSize);
    if (*bufferPtr).is_null() {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 601i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    34i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(34i32);
    }
    let readSize: size_t =
        fread(*bufferPtr, 1i32 as size_t, fileSize, fileHandle);
    if readSize != fileSize {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 605i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    35i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Error reading dictionary file %s : %s\x00" as *const u8
                        as *const libc::c_char, fileName,
                    strerror(*__errno_location()));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(35i32);
    }
    fclose(fileHandle);
    return fileSize;
}
/* ! FIO_compressFilename_srcFile() :
 *  @return : 0 : compression completed correctly,
 *            1 : missing or pb opening srcFileName
 */
unsafe extern "C" fn FIO_compressFilename_srcFile(prefs: *mut FIO_prefs_t,
                                                  mut ress: cRess_t,
                                                  mut dstFileName:
                                                      *const libc::c_char,
                                                  mut srcFileName:
                                                      *const libc::c_char,
                                                  mut compressionLevel:
                                                      libc::c_int)
 -> libc::c_int {
    let mut result: libc::c_int = 0;
    if 0 != UTIL_isDirectory(srcFileName) {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: %s is a directory -- ignored \n\x00" as *const u8
                        as *const libc::c_char, srcFileName);
        }
        return 1i32
    }
    ress.srcFile = FIO_openSrcFile(srcFileName);
    if ress.srcFile.is_null() { return 1i32 }
    result =
        FIO_compressFilename_dstFile(prefs, ress, dstFileName, srcFileName,
                                     compressionLevel);
    fclose(ress.srcFile);
    ress.srcFile = 0 as *mut FILE;
    if 0 != (*prefs).removeSrcFile && result == 0i32 &&
           0 !=
               strcmp(srcFileName,
                      b"/*stdin*\\\x00" as *const u8 as *const libc::c_char) {
        clearHandler();
        if 0 != FIO_remove(srcFileName) {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 1320i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 1i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: %s: %s\x00" as *const u8 as
                            *const libc::c_char, srcFileName,
                        strerror(*__errno_location()));
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(1i32);
        }
    }
    return result;
}
/*-*************************************
*  Functions
***************************************/
/** FIO_remove() :
 * @result : Unlink `fileName`, even if it's read-only */
unsafe extern "C" fn FIO_remove(mut path: *const libc::c_char)
 -> libc::c_int {
    if 0 == UTIL_isRegularFile(path) {
        if g_display_prefs.displayLevel >= 2i32 {
            fprintf(stderr,
                    b"zstd: Refusing to remove non-regular file %s \n\x00" as
                        *const u8 as *const libc::c_char, path);
        }
        return 0i32
    }
    return remove(path);
}
/* Idempotent */
unsafe extern "C" fn clearHandler() {
    if !g_artefact.is_null() { signal(2i32, None); }
    g_artefact = 0 as *const libc::c_char;
}
/* in case it would be already defined */
/*-************************************
*  Signal (Ctrl-C trapping)
**************************************/
static mut g_artefact: *const libc::c_char = 0 as *const libc::c_char;
/* ! FIO_compressFilename_dstFile() :
 *  open dstFileName, or pass-through if ress.dstFile != NULL,
 *  then start compression with FIO_compressFilename_internal().
 *  Manages source removal (--rm) and file permissions transfer.
 *  note : ress.srcFile must be != NULL,
 *  so reach this function through FIO_compressFilename_srcFile().
 *  @return : 0 : compression completed correctly,
 *            1 : pb
 */
unsafe extern "C" fn FIO_compressFilename_dstFile(prefs: *mut FIO_prefs_t,
                                                  mut ress: cRess_t,
                                                  mut dstFileName:
                                                      *const libc::c_char,
                                                  mut srcFileName:
                                                      *const libc::c_char,
                                                  mut compressionLevel:
                                                      libc::c_int)
 -> libc::c_int {
    let mut closeDstFile: libc::c_int = 0i32;
    let mut result: libc::c_int = 0;
    let mut statbuf: stat_t =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atime: 0,
             st_atimensec: 0,
             st_mtime: 0,
             st_mtimensec: 0,
             st_ctime: 0,
             st_ctimensec: 0,
             __glibc_reserved: [0; 3],};
    let mut transfer_permissions: libc::c_int = 0i32;
    if !ress.srcFile.is_null() {
    } else {
        __assert_fail(b"ress.srcFile != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                          as *const u8 as *const libc::c_char,
                      1239i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 95],
                                                &[libc::c_char; 95]>(b"int FIO_compressFilename_dstFile(FIO_prefs_t *const, cRess_t, const char *, const char *, int)\x00")).as_ptr());
    };
    if ress.dstFile.is_null() {
        closeDstFile = 1i32;
        if g_display_prefs.displayLevel >= 6i32 {
            fprintf(stderr,
                    b"FIO_compressFilename_dstFile: opening dst: %s\x00" as
                        *const u8 as *const libc::c_char, dstFileName);
        }
        ress.dstFile = FIO_openDstFile(prefs, srcFileName, dstFileName);
        if ress.dstFile.is_null() { return 1i32 }
        addHandler(dstFileName);
        if 0 !=
               strcmp(srcFileName,
                      b"/*stdin*\\\x00" as *const u8 as *const libc::c_char)
               && 0 != UTIL_getFileStat(srcFileName, &mut statbuf) {
            transfer_permissions = 1i32
        }
    }
    result =
        FIO_compressFilename_internal(prefs, ress, dstFileName, srcFileName,
                                      compressionLevel);
    if 0 != closeDstFile {
        let dstFile: *mut FILE = ress.dstFile;
        ress.dstFile = 0 as *mut FILE;
        clearHandler();
        if 0 != fclose(dstFile) {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: %s: %s \n\x00" as *const u8 as
                            *const libc::c_char, dstFileName,
                        strerror(*__errno_location()));
            }
            result = 1i32
        }
        if result != 0i32 &&
               0 !=
                   strcmp(dstFileName,
                          b"/dev/null\x00" as *const u8 as
                              *const libc::c_char) &&
               0 !=
                   strcmp(dstFileName,
                          b"/*stdout*\\\x00" as *const u8 as
                              *const libc::c_char) {
            FIO_remove(dstFileName);
        } else if 0 !=
                      strcmp(dstFileName,
                             b"/*stdout*\\\x00" as *const u8 as
                                 *const libc::c_char) &&
                      0 !=
                          strcmp(dstFileName,
                                 b"/dev/null\x00" as *const u8 as
                                     *const libc::c_char) &&
                      0 != transfer_permissions {
            UTIL_setFileStat(dstFileName, &mut statbuf);
        }
    }
    return result;
}
/* ! FIO_compressFilename_internal() :
 *  same as FIO_compressFilename_extRess(), with `ress.desFile` already opened.
 *  @return : 0 : compression completed correctly,
 *            1 : missing or pb opening srcFileName
 */
unsafe extern "C" fn FIO_compressFilename_internal(prefs: *mut FIO_prefs_t,
                                                   mut ress: cRess_t,
                                                   mut dstFileName:
                                                       *const libc::c_char,
                                                   mut srcFileName:
                                                       *const libc::c_char,
                                                   mut compressionLevel:
                                                       libc::c_int)
 -> libc::c_int {
    let mut readsize: U64 = 0i32 as U64;
    let mut compressedfilesize: U64 = 0i32 as U64;
    let fileSize: U64 = UTIL_getFileSize(srcFileName);
    if g_display_prefs.displayLevel >= 5i32 {
        fprintf(stderr,
                b"%s: %u bytes \n\x00" as *const u8 as *const libc::c_char,
                srcFileName, fileSize as libc::c_uint);
    }
    match (*prefs).compressionType as libc::c_uint {
        1 => {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 1181i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 20i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: %s: file cannot be compressed as gzip (zstd compiled without ZSTD_GZCOMPRESS) -- ignored \n\x00"
                            as *const u8 as *const libc::c_char, srcFileName);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(20i32);
        }
        2 | 3 => {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 1192i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 20i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: %s: file cannot be compressed as xz/lzma (zstd compiled without ZSTD_LZMACOMPRESS) -- ignored \n\x00"
                            as *const u8 as *const libc::c_char, srcFileName);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(20i32);
        }
        4 => {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 1202i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 20i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: %s: file cannot be compressed as lz4 (zstd compiled without ZSTD_LZ4COMPRESS) -- ignored \n\x00"
                            as *const u8 as *const libc::c_char, srcFileName);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(20i32);
        }
        0 | _ => {
            compressedfilesize =
                FIO_compressZstdFrame(prefs, &mut ress, srcFileName, fileSize,
                                      compressionLevel, &mut readsize) as U64
        }
    }
    if g_display_prefs.displayLevel >= 2i32 {
        fprintf(stderr, b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char);
    }
    if g_display_prefs.displayLevel >= 2i32 {
        fprintf(stderr,
                b"%-20s :%6.2f%%   (%6llu => %6llu bytes, %s) \n\x00" as
                    *const u8 as *const libc::c_char, srcFileName,
                compressedfilesize as libc::c_double /
                    readsize.wrapping_add((0 == readsize) as libc::c_int as
                                              libc::c_ulong) as libc::c_double
                    * 100i32 as libc::c_double, readsize as libc::c_ulonglong,
                compressedfilesize as libc::c_ulonglong, dstFileName);
    }
    return 0i32;
}
unsafe extern "C" fn FIO_compressZstdFrame(prefs: *mut FIO_prefs_t,
                                           mut ressPtr: *const cRess_t,
                                           mut srcFileName:
                                               *const libc::c_char,
                                           mut fileSize: U64,
                                           mut compressionLevel: libc::c_int,
                                           mut readsize: *mut U64)
 -> libc::c_ulonglong {
    let ress: cRess_t = *ressPtr;
    let srcFile: *mut FILE = ress.srcFile;
    let dstFile: *mut FILE = ress.dstFile;
    let mut compressedfilesize: U64 = 0i32 as U64;
    let mut directive: ZSTD_EndDirective = ZSTD_e_continue;
    /* stats */
    let mut previous_zfp_update: ZSTD_frameProgression =
        ZSTD_frameProgression{ingested: 0i32 as libc::c_ulonglong,
                              consumed: 0i32 as libc::c_ulonglong,
                              produced: 0i32 as libc::c_ulonglong,
                              flushed: 0i32 as libc::c_ulonglong,
                              currentJobID: 0i32 as libc::c_uint,
                              nbActiveWorkers: 0i32 as libc::c_uint,};
    let mut previous_zfp_correction: ZSTD_frameProgression =
        ZSTD_frameProgression{ingested: 0i32 as libc::c_ulonglong,
                              consumed: 0i32 as libc::c_ulonglong,
                              produced: 0i32 as libc::c_ulonglong,
                              flushed: 0i32 as libc::c_ulonglong,
                              currentJobID: 0i32 as libc::c_uint,
                              nbActiveWorkers: 0i32 as libc::c_uint,};
    let mut speedChange: speedChange_e = noChange;
    let mut flushWaiting: libc::c_uint = 0i32 as libc::c_uint;
    let mut inputPresented: libc::c_uint = 0i32 as libc::c_uint;
    let mut inputBlocked: libc::c_uint = 0i32 as libc::c_uint;
    let mut lastJobID: libc::c_uint = 0i32 as libc::c_uint;
    if g_display_prefs.displayLevel >= 6i32 {
        fprintf(stderr,
                b"compression using zstd format \n\x00" as *const u8 as
                    *const libc::c_char);
    }
    if fileSize != -1i32 as U64 {
        let mut err: size_t = 0;
        err =
            ZSTD_CCtx_setPledgedSrcSize(ress.cctx,
                                        fileSize as libc::c_ulonglong);
        if 0 != ZSTD_isError(err) {
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"%s \n\x00" as *const u8 as *const libc::c_char,
                        b"ZSTD_CCtx_setPledgedSrcSize(ress.cctx, fileSize)\x00"
                            as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 989i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 11i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                        ZSTD_getErrorName(err));
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(11i32);
        }
    }
    loop  {
        let mut stillToFlush: size_t = 0;
        let inSize: size_t =
            fread(ress.srcBuffer, 1i32 as size_t, ress.srcBufferSize,
                  srcFile);
        let mut inBuff: ZSTD_inBuffer =
            ZSTD_inBuffer_s{src: ress.srcBuffer,
                            size: inSize,
                            pos: 0i32 as size_t,};
        if g_display_prefs.displayLevel >= 6i32 {
            fprintf(stderr,
                    b"fread %u bytes from source \n\x00" as *const u8 as
                        *const libc::c_char, inSize as libc::c_uint);
        }
        *readsize =
            (*readsize as libc::c_ulong).wrapping_add(inSize) as U64 as U64;
        if inSize == 0i32 as libc::c_ulong || *readsize == fileSize {
            directive = ZSTD_e_end
        }
        stillToFlush = 1i32 as size_t;
        while inBuff.pos != inBuff.size ||
                  directive as libc::c_uint ==
                      ZSTD_e_end as libc::c_int as libc::c_uint &&
                      stillToFlush != 0i32 as libc::c_ulong {
            let oldIPos: size_t = inBuff.pos;
            let mut outBuff: ZSTD_outBuffer =
                ZSTD_outBuffer_s{dst: ress.dstBuffer,
                                 size: ress.dstBufferSize,
                                 pos: 0i32 as size_t,};
            let toFlushNow: size_t = ZSTD_toFlushNow(ress.cctx);
            stillToFlush =
                ZSTD_compressStream2(ress.cctx, &mut outBuff, &mut inBuff,
                                     directive);
            if 0 != ZSTD_isError(stillToFlush) {
                if g_display_prefs.displayLevel >= 5i32 {
                    fprintf(stderr,
                            b"%s \n\x00" as *const u8 as *const libc::c_char,
                            b"ZSTD_compressStream2(ress.cctx, &outBuff, &inBuff, directive)\x00"
                                as *const u8 as *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"zstd: \x00" as *const u8 as
                                *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 5i32 {
                    fprintf(stderr,
                            b"Error defined at %s, line %i : \n\x00" as
                                *const u8 as *const libc::c_char,
                            b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                                as *const u8 as *const libc::c_char, 1012i32);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"error %i : \x00" as *const u8 as
                                *const libc::c_char, 11i32);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            ZSTD_getErrorName(stillToFlush));
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(11i32);
            }
            inputPresented = inputPresented.wrapping_add(1);
            if oldIPos == inBuff.pos {
                inputBlocked = inputBlocked.wrapping_add(1)
            }
            if 0 == toFlushNow { flushWaiting = 1i32 as libc::c_uint }
            if g_display_prefs.displayLevel >= 6i32 {
                fprintf(stderr,
                        b"ZSTD_compress_generic(end:%u) => input pos(%u)<=(%u)size ; output generated %u bytes \n\x00"
                            as *const u8 as *const libc::c_char,
                        directive as libc::c_uint, inBuff.pos as libc::c_uint,
                        inBuff.size as libc::c_uint,
                        outBuff.pos as libc::c_uint);
            }
            if 0 != outBuff.pos {
                let sizeCheck: size_t =
                    fwrite(ress.dstBuffer, 1i32 as size_t, outBuff.pos,
                           dstFile);
                if sizeCheck != outBuff.pos {
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b"zstd: \x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    if g_display_prefs.displayLevel >= 5i32 {
                        fprintf(stderr,
                                b"Error defined at %s, line %i : \n\x00" as
                                    *const u8 as *const libc::c_char,
                                b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                                    as *const u8 as *const libc::c_char,
                                1026i32);
                    }
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b"error %i : \x00" as *const u8 as
                                    *const libc::c_char, 25i32);
                    }
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b"Write error : %s (cannot write compressed block)\x00"
                                    as *const u8 as *const libc::c_char,
                                strerror(*__errno_location()));
                    }
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b" \n\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    exit(25i32);
                }
                compressedfilesize =
                    (compressedfilesize as
                         libc::c_ulong).wrapping_add(outBuff.pos) as U64 as
                        U64
            }
            if 0 == g_display_prefs.noProgress &&
                   UTIL_clockSpanMicro(g_displayClock) > g_refreshRate {
                let zfp: ZSTD_frameProgression =
                    ZSTD_getFrameProgression(ress.cctx);
                let cShare: libc::c_double =
                    zfp.produced as libc::c_double /
                        zfp.consumed.wrapping_add((0 == zfp.consumed) as
                                                      libc::c_int as
                                                      libc::c_ulonglong) as
                            libc::c_double * 100i32 as libc::c_double;
                if g_display_prefs.displayLevel >= 3i32 {
                    if g_display_prefs.displayLevel >= 3i32 &&
                           0 == g_display_prefs.noProgress {
                        if 0 == g_display_prefs.noProgress &&
                               UTIL_clockSpanMicro(g_displayClock) >
                                   g_refreshRate ||
                               g_display_prefs.displayLevel >= 4i32 {
                            g_displayClock = UTIL_getTime();
                            fprintf(stderr,
                                    b"\r(L%i) Buffered :%4u MB - Consumed :%4u MB - Compressed :%4u MB => %.2f%% \x00"
                                        as *const u8 as *const libc::c_char,
                                    compressionLevel,
                                    (zfp.ingested.wrapping_sub(zfp.consumed)
                                         >> 20i32) as libc::c_uint,
                                    (zfp.consumed >> 20i32) as libc::c_uint,
                                    (zfp.produced >> 20i32) as libc::c_uint,
                                    cShare);
                            if g_display_prefs.displayLevel >= 4i32 {
                                fflush(stderr);
                            }
                        }
                    }
                } else {
                    if g_display_prefs.displayLevel >= 2i32 {
                        fprintf(stderr,
                                b"\rRead : %u \x00" as *const u8 as
                                    *const libc::c_char,
                                (zfp.consumed >> 20i32) as libc::c_uint);
                    }
                    if fileSize != -1i32 as U64 {
                        if g_display_prefs.displayLevel >= 2i32 {
                            fprintf(stderr,
                                    b"/ %u \x00" as *const u8 as
                                        *const libc::c_char,
                                    (fileSize >> 20i32) as libc::c_uint);
                        }
                    }
                    if g_display_prefs.displayLevel >= 2i32 {
                        fprintf(stderr,
                                b"MB ==> %2.f%% \x00" as *const u8 as
                                    *const libc::c_char, cShare);
                    }
                    g_displayClock = UTIL_getTime()
                }
                if 0 != (*prefs).adaptiveMode {
                    if zfp.currentJobID > 1i32 as libc::c_uint {
                        let mut newlyProduced: libc::c_ulonglong =
                            zfp.produced.wrapping_sub(previous_zfp_update.produced);
                        let mut newlyFlushed: libc::c_ulonglong =
                            zfp.flushed.wrapping_sub(previous_zfp_update.flushed);
                        if zfp.produced >= previous_zfp_update.produced {
                        } else {
                            __assert_fail(b"zfp.produced >= previous_zfp_update.produced\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          1059i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 109],
                                                                    &[libc::c_char; 109]>(b"unsigned long long FIO_compressZstdFrame(FIO_prefs_t *const, const cRess_t *, const char *, U64, int, U64 *)\x00")).as_ptr());
                        };
                        if (*prefs).nbWorkers >= 1i32 as libc::c_uint {
                        } else {
                            __assert_fail(b"prefs->nbWorkers >= 1\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          1060i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 109],
                                                                    &[libc::c_char; 109]>(b"unsigned long long FIO_compressZstdFrame(FIO_prefs_t *const, const cRess_t *, const char *, U64, int, U64 *)\x00")).as_ptr());
                        };
                        if zfp.consumed == previous_zfp_update.consumed &&
                               zfp.nbActiveWorkers == 0i32 as libc::c_uint {
                            if g_display_prefs.displayLevel >= 6i32 {
                                fprintf(stderr,
                                        b"all buffers full : compression stopped => slow down \n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                            speedChange = slower
                        }
                        previous_zfp_update = zfp;
                        if newlyProduced >
                               newlyFlushed.wrapping_mul(9i32 as
                                                             libc::c_ulonglong).wrapping_div(8i32
                                                                                                 as
                                                                                                 libc::c_ulonglong)
                               && flushWaiting == 0i32 as libc::c_uint {
                            if g_display_prefs.displayLevel >= 6i32 {
                                fprintf(stderr,
                                        b"compression faster than flush (%llu > %llu), and flushed was never slowed down by lack of production => slow down \n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        newlyProduced, newlyFlushed);
                            }
                            speedChange = slower
                        }
                        flushWaiting = 0i32 as libc::c_uint
                    }
                    if zfp.currentJobID > lastJobID {
                        if g_display_prefs.displayLevel >= 6i32 {
                            fprintf(stderr,
                                    b"compression level adaptation check \n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                        if zfp.currentJobID >
                               (*prefs).nbWorkers.wrapping_add(1i32 as
                                                                   libc::c_uint)
                           {
                            if inputBlocked <= 0i32 as libc::c_uint {
                                if g_display_prefs.displayLevel >= 6i32 {
                                    fprintf(stderr,
                                            b"input is never blocked => input is slower than ingestion \n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                                speedChange = slower
                            } else if speedChange as libc::c_uint ==
                                          noChange as libc::c_int as
                                              libc::c_uint {
                                let mut newlyIngested: libc::c_ulonglong =
                                    zfp.ingested.wrapping_sub(previous_zfp_correction.ingested);
                                let mut newlyConsumed: libc::c_ulonglong =
                                    zfp.consumed.wrapping_sub(previous_zfp_correction.consumed);
                                let mut newlyProduced_0: libc::c_ulonglong =
                                    zfp.produced.wrapping_sub(previous_zfp_correction.produced);
                                let mut newlyFlushed_0: libc::c_ulonglong =
                                    zfp.flushed.wrapping_sub(previous_zfp_correction.flushed);
                                previous_zfp_correction = zfp;
                                if inputPresented > 0i32 as libc::c_uint {
                                } else {
                                    __assert_fail(b"inputPresented > 0\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  1099i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 109],
                                                                            &[libc::c_char; 109]>(b"unsigned long long FIO_compressZstdFrame(FIO_prefs_t *const, const cRess_t *, const char *, U64, int, U64 *)\x00")).as_ptr());
                                };
                                if g_display_prefs.displayLevel >= 6i32 {
                                    fprintf(stderr,
                                            b"input blocked %u/%u(%.2f) - ingested:%u vs %u:consumed - flushed:%u vs %u:produced \n\x00"
                                                as *const u8 as
                                                *const libc::c_char,
                                            inputBlocked, inputPresented,
                                            inputBlocked as libc::c_double /
                                                inputPresented as
                                                    libc::c_double *
                                                100i32 as libc::c_double,
                                            newlyIngested as libc::c_uint,
                                            newlyConsumed as libc::c_uint,
                                            newlyFlushed_0 as libc::c_uint,
                                            newlyProduced_0 as libc::c_uint);
                                }
                                if inputBlocked >
                                       inputPresented.wrapping_div(8i32 as
                                                                       libc::c_uint)
                                       &&
                                       newlyFlushed_0.wrapping_mul(33i32 as
                                                                       libc::c_ulonglong).wrapping_div(32i32
                                                                                                           as
                                                                                                           libc::c_ulonglong)
                                           > newlyProduced_0 &&
                                       newlyIngested.wrapping_mul(33i32 as
                                                                      libc::c_ulonglong).wrapping_div(32i32
                                                                                                          as
                                                                                                          libc::c_ulonglong)
                                           > newlyConsumed {
                                    if g_display_prefs.displayLevel >= 6i32 {
                                        fprintf(stderr,
                                                b"recommend faster as in(%llu) >= (%llu)comp(%llu) <= out(%llu) \n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                newlyIngested, newlyConsumed,
                                                newlyProduced_0,
                                                newlyFlushed_0);
                                    }
                                    speedChange = faster
                                }
                            }
                            inputBlocked = 0i32 as libc::c_uint;
                            inputPresented = 0i32 as libc::c_uint
                        }
                        if speedChange as libc::c_uint ==
                               slower as libc::c_int as libc::c_uint {
                            if g_display_prefs.displayLevel >= 6i32 {
                                fprintf(stderr,
                                        b"slower speed , higher compression \n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                            compressionLevel += 1;
                            if compressionLevel > ZSTD_maxCLevel() {
                                compressionLevel = ZSTD_maxCLevel()
                            }
                            if compressionLevel > (*prefs).maxAdaptLevel {
                                compressionLevel = (*prefs).maxAdaptLevel
                            }
                            compressionLevel +=
                                (compressionLevel == 0i32) as libc::c_int;
                            ZSTD_CCtx_setParameter(ress.cctx,
                                                   ZSTD_c_compressionLevel,
                                                   compressionLevel);
                        }
                        if speedChange as libc::c_uint ==
                               faster as libc::c_int as libc::c_uint {
                            if g_display_prefs.displayLevel >= 6i32 {
                                fprintf(stderr,
                                        b"faster speed , lighter compression \n\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                            }
                            compressionLevel -= 1;
                            if compressionLevel < (*prefs).minAdaptLevel {
                                compressionLevel = (*prefs).minAdaptLevel
                            }
                            compressionLevel -=
                                (compressionLevel == 0i32) as libc::c_int;
                            ZSTD_CCtx_setParameter(ress.cctx,
                                                   ZSTD_c_compressionLevel,
                                                   compressionLevel);
                        }
                        speedChange = noChange;
                        lastJobID = zfp.currentJobID
                    }
                }
            }
        }
        if !(directive as libc::c_uint !=
                 ZSTD_e_end as libc::c_int as libc::c_uint) {
            break ;
        }
    }
    if 0 != ferror(srcFile) {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 1142i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    26i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Read error : I/O error\x00" as *const u8 as
                        *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(26i32);
    }
    if fileSize != -1i32 as U64 && *readsize != fileSize {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 1146i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    27i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Read error : Incomplete read : %llu / %llu B\x00" as
                        *const u8 as *const libc::c_char,
                    *readsize as libc::c_ulonglong,
                    fileSize as libc::c_ulonglong);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(27i32);
    }
    return compressedfilesize as libc::c_ulonglong;
}
static mut g_displayClock: UTIL_time_t =
    timespec{tv_sec: 0i32 as __time_t, tv_nsec: 0i32 as __syscall_slong_t,};
static mut g_refreshRate: U64 = (1000000i32 / 6i32) as U64;
unsafe extern "C" fn addHandler(mut dstFileName: *const libc::c_char) {
    if 0 != UTIL_isRegularFile(dstFileName) {
        g_artefact = dstFileName;
        signal(2i32, Some(INThandler));
    } else { g_artefact = 0 as *const libc::c_char };
}
unsafe extern "C" fn INThandler(mut sig: libc::c_int) {
    if sig == 2i32 {
    } else {
        __assert_fail(b"sig==SIGINT\x00" as *const u8 as *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                          as *const u8 as *const libc::c_char,
                      136i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 21],
                                                &[libc::c_char; 21]>(b"void INThandler(int)\x00")).as_ptr());
    };
    signal(sig,
           ::std::mem::transmute::<libc::intptr_t,
                                   __sighandler_t>(1i32 as libc::intptr_t));
    if !g_artefact.is_null() {
        if 0 != UTIL_isRegularFile(g_artefact) {
        } else {
            __assert_fail(b"UTIL_isRegularFile(g_artefact)\x00" as *const u8
                              as *const libc::c_char,
                          b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                              as *const u8 as *const libc::c_char,
                          141i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 21],
                                                    &[libc::c_char; 21]>(b"void INThandler(int)\x00")).as_ptr());
        };
        remove(g_artefact);
    }
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    exit(2i32);
}
/* * FIO_openDstFile() :
 *  condition : `dstFileName` must be non-NULL.
 * @result : FILE* to `dstFileName`, or NULL if it fails */
unsafe extern "C" fn FIO_openDstFile(prefs: *mut FIO_prefs_t,
                                     mut srcFileName: *const libc::c_char,
                                     mut dstFileName: *const libc::c_char)
 -> *mut FILE {
    if !dstFileName.is_null() {
    } else {
        __assert_fail(b"dstFileName != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                          as *const u8 as *const libc::c_char,
                      499i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 70],
                                                &[libc::c_char; 70]>(b"FILE *FIO_openDstFile(FIO_prefs_t *const, const char *, const char *)\x00")).as_ptr());
    };
    if 0 ==
           strcmp(dstFileName,
                  b"/*stdout*\\\x00" as *const u8 as *const libc::c_char) {
        if g_display_prefs.displayLevel >= 4i32 {
            fprintf(stderr,
                    b"Using stdout for output \n\x00" as *const u8 as
                        *const libc::c_char);
        }
        if (*prefs).sparseFileSupport == 1i32 as libc::c_uint {
            (*prefs).sparseFileSupport = 0i32 as U32;
            if g_display_prefs.displayLevel >= 4i32 {
                fprintf(stderr,
                        b"Sparse File Support is automatically disabled on stdout ; try --sparse \n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        return stdout
    }
    if !srcFileName.is_null() {
        let mut srcStat: stat_t =
            stat{st_dev: 0,
                 st_ino: 0,
                 st_nlink: 0,
                 st_mode: 0,
                 st_uid: 0,
                 st_gid: 0,
                 __pad0: 0,
                 st_rdev: 0,
                 st_size: 0,
                 st_blksize: 0,
                 st_blocks: 0,
                 st_atime: 0,
                 st_atimensec: 0,
                 st_mtime: 0,
                 st_mtimensec: 0,
                 st_ctime: 0,
                 st_ctimensec: 0,
                 __glibc_reserved: [0; 3],};
        let mut dstStat: stat_t =
            stat{st_dev: 0,
                 st_ino: 0,
                 st_nlink: 0,
                 st_mode: 0,
                 st_uid: 0,
                 st_gid: 0,
                 __pad0: 0,
                 st_rdev: 0,
                 st_size: 0,
                 st_blksize: 0,
                 st_blocks: 0,
                 st_atime: 0,
                 st_atimensec: 0,
                 st_mtime: 0,
                 st_mtimensec: 0,
                 st_ctime: 0,
                 st_ctimensec: 0,
                 __glibc_reserved: [0; 3],};
        if 0 != UTIL_getFileStat(srcFileName, &mut srcStat) &&
               0 != UTIL_getFileStat(dstFileName, &mut dstStat) {
            if srcStat.st_dev == dstStat.st_dev &&
                   srcStat.st_ino == dstStat.st_ino {
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"zstd: Refusing to open a output file which will overwrite the input file \n\x00"
                                as *const u8 as *const libc::c_char);
                }
                return 0 as *mut FILE
            }
        }
    }
    if (*prefs).sparseFileSupport == 1i32 as libc::c_uint {
        (*prefs).sparseFileSupport = 1i32 as U32
    }
    if 0 != UTIL_isRegularFile(dstFileName) {
        let fCheck: *mut FILE =
            fopen(dstFileName, b"rb\x00" as *const u8 as *const libc::c_char);
        if 0 ==
               strcmp(dstFileName,
                      b"/dev/null\x00" as *const u8 as *const libc::c_char) {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 543i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 40i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"%s is unexpectedly categorized as a regular file\x00"
                            as *const u8 as *const libc::c_char, dstFileName);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(40i32);
        }
        if !fCheck.is_null() {
            fclose(fCheck);
            if 0 == (*prefs).overwrite {
                if g_display_prefs.displayLevel <= 1i32 {
                    fprintf(stderr,
                            b"zstd: %s already exists; not overwritten  \n\x00"
                                as *const u8 as *const libc::c_char,
                            dstFileName);
                    return 0 as *mut FILE
                }
                fprintf(stderr,
                        b"zstd: %s already exists; overwrite (y/N) ? \x00" as
                            *const u8 as *const libc::c_char, dstFileName);
                let mut ch: libc::c_int = getchar();
                if ch != 'Y' as i32 && ch != 'y' as i32 {
                    fprintf(stderr,
                            b"    not overwritten  \n\x00" as *const u8 as
                                *const libc::c_char);
                    return 0 as *mut FILE
                }
                while ch != -1i32 && ch != '\n' as i32 { ch = getchar() }
            }
            FIO_remove(dstFileName);
        }
    }
    let f: *mut FILE =
        fopen(dstFileName, b"wb\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: %s: %s\n\x00" as *const u8 as *const libc::c_char,
                    dstFileName, strerror(*__errno_location()));
        }
    }
    return f;
}
/* * FIO_openSrcFile() :
 *  condition : `srcFileName` must be non-NULL.
 * @result : FILE* to `srcFileName`, or NULL if it fails */
unsafe extern "C" fn FIO_openSrcFile(mut srcFileName: *const libc::c_char)
 -> *mut FILE {
    if !srcFileName.is_null() {
    } else {
        __assert_fail(b"srcFileName != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                          as *const u8 as *const libc::c_char,
                      468i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"FILE *FIO_openSrcFile(const char *)\x00")).as_ptr());
    };
    if 0 ==
           strcmp(srcFileName,
                  b"/*stdin*\\\x00" as *const u8 as *const libc::c_char) {
        if g_display_prefs.displayLevel >= 4i32 {
            fprintf(stderr,
                    b"Using stdin for input \n\x00" as *const u8 as
                        *const libc::c_char);
        }
        return stdin
    }
    if 0 == UTIL_fileExist(srcFileName) {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: can\'t stat %s : %s -- ignored \n\x00" as
                        *const u8 as *const libc::c_char, srcFileName,
                    strerror(*__errno_location()));
        }
        return 0 as *mut FILE
    }
    if 0 == UTIL_isRegularFile(srcFileName) {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: %s is not a regular file -- ignored \n\x00" as
                        *const u8 as *const libc::c_char, srcFileName);
        }
        return 0 as *mut FILE
    }
    let f: *mut FILE =
        fopen(srcFileName, b"rb\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: %s: %s \n\x00" as *const u8 as
                        *const libc::c_char, srcFileName,
                    strerror(*__errno_location()));
        }
    }
    return f;
}
unsafe extern "C" fn FIO_freeCResources(mut ress: cRess_t) {
    free(ress.srcBuffer);
    free(ress.dstBuffer);
    ZSTD_freeCStream(ress.cctx);
}
/* * FIO_decompressFilename() :
    @return : 0 == ok;  1 == pb with src file. */
#[no_mangle]
pub unsafe extern "C" fn FIO_decompressFilename(prefs: *mut FIO_prefs_t,
                                                mut dstFileName:
                                                    *const libc::c_char,
                                                mut srcFileName:
                                                    *const libc::c_char,
                                                mut dictFileName:
                                                    *const libc::c_char)
 -> libc::c_int {
    let ress: dRess_t = FIO_createDResources(prefs, dictFileName);
    let decodingError: libc::c_int =
        FIO_decompressSrcFile(prefs, ress, dstFileName, srcFileName);
    FIO_freeDResources(ress);
    return decodingError;
}
unsafe extern "C" fn FIO_createDResources(prefs: *mut FIO_prefs_t,
                                          mut dictFileName:
                                              *const libc::c_char)
 -> dRess_t {
    let mut ress: dRess_t =
        dRess_t{srcBuffer: 0 as *mut libc::c_void,
                srcBufferSize: 0,
                srcBufferLoaded: 0,
                dstBuffer: 0 as *mut libc::c_void,
                dstBufferSize: 0,
                dctx: 0 as *mut ZSTD_DStream,
                dstFile: 0 as *mut FILE,};
    memset(&mut ress as *mut dRess_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<dRess_t>() as libc::c_ulong);
    ress.dctx = ZSTD_createDStream();
    if ress.dctx.is_null() {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 1448i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    60i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Error: %s : can\'t create ZSTD_DStream\x00" as *const u8
                        as *const libc::c_char,
                    strerror(*__errno_location()));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(60i32);
    }
    let mut err: size_t = 0;
    err = ZSTD_DCtx_setMaxWindowSize(ress.dctx, (*prefs).memLimit as size_t);
    if 0 != ZSTD_isError(err) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_DCtx_setMaxWindowSize(ress.dctx, prefs->memLimit)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 1449i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    ress.srcBufferSize = ZSTD_DStreamInSize();
    ress.srcBuffer = malloc(ress.srcBufferSize);
    ress.dstBufferSize = ZSTD_DStreamOutSize();
    ress.dstBuffer = malloc(ress.dstBufferSize);
    if ress.srcBuffer.is_null() || ress.dstBuffer.is_null() {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 1455i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    61i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Allocation error : not enough memory\x00" as *const u8
                        as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(61i32);
    }
    let mut dictBuffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let dictBufferSize: size_t =
        FIO_createDictBuffer(&mut dictBuffer, dictFileName);
    let mut err_0: size_t = 0;
    err_0 = ZSTD_initDStream_usingDict(ress.dctx, dictBuffer, dictBufferSize);
    if 0 != ZSTD_isError(err_0) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_initDStream_usingDict(ress.dctx, dictBuffer, dictBufferSize)\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 1460i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err_0));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    free(dictBuffer);
    return ress;
}
/* * FIO_decompressSrcFile() :
    Open `srcFileName`, transfer control to decompressDstFile()
    @return : 0 : OK
              1 : error
*/
unsafe extern "C" fn FIO_decompressSrcFile(prefs: *mut FIO_prefs_t,
                                           mut ress: dRess_t,
                                           mut dstFileName:
                                               *const libc::c_char,
                                           mut srcFileName:
                                               *const libc::c_char)
 -> libc::c_int {
    let mut srcFile: *mut FILE = 0 as *mut FILE;
    let mut result: libc::c_int = 0;
    if 0 != UTIL_isDirectory(srcFileName) {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: %s is a directory -- ignored \n\x00" as *const u8
                        as *const libc::c_char, srcFileName);
        }
        return 1i32
    }
    srcFile = FIO_openSrcFile(srcFileName);
    if srcFile.is_null() { return 1i32 }
    ress.srcBufferLoaded = 0i32 as size_t;
    result =
        FIO_decompressDstFile(prefs, ress, srcFile, dstFileName, srcFileName);
    if 0 != fclose(srcFile) {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: %s: %s \n\x00" as *const u8 as
                        *const libc::c_char, srcFileName,
                    strerror(*__errno_location()));
        }
        return 1i32
    }
    if 0 != (*prefs).removeSrcFile && result == 0i32 &&
           0 !=
               strcmp(srcFileName,
                      b"/*stdin*\\\x00" as *const u8 as *const libc::c_char) {
        clearHandler();
        if 0 != FIO_remove(srcFileName) {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: %s: %s \n\x00" as *const u8 as
                            *const libc::c_char, srcFileName,
                        strerror(*__errno_location()));
            }
            return 1i32
        }
    }
    return result;
}
/* * FIO_decompressDstFile() :
    open `dstFileName`,
    or path-through if ress.dstFile is already != 0,
    then start decompression process (FIO_decompressFrames()).
    @return : 0 : OK
              1 : operation aborted
*/
unsafe extern "C" fn FIO_decompressDstFile(prefs: *mut FIO_prefs_t,
                                           mut ress: dRess_t,
                                           mut srcFile: *mut FILE,
                                           mut dstFileName:
                                               *const libc::c_char,
                                           mut srcFileName:
                                               *const libc::c_char)
 -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut statbuf: stat_t =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atime: 0,
             st_atimensec: 0,
             st_mtime: 0,
             st_mtimensec: 0,
             st_ctime: 0,
             st_ctimensec: 0,
             __glibc_reserved: [0; 3],};
    let mut transfer_permissions: libc::c_int = 0i32;
    let mut releaseDstFile: libc::c_int = 0i32;
    if ress.dstFile.is_null() {
        releaseDstFile = 1i32;
        ress.dstFile = FIO_openDstFile(prefs, srcFileName, dstFileName);
        if ress.dstFile.is_null() { return 1i32 }
        addHandler(dstFileName);
        if 0 !=
               strcmp(srcFileName,
                      b"/*stdin*\\\x00" as *const u8 as *const libc::c_char)
               && 0 != UTIL_getFileStat(srcFileName, &mut statbuf) {
            transfer_permissions = 1i32
        }
    }
    result =
        FIO_decompressFrames(prefs, ress, srcFile, dstFileName, srcFileName);
    if 0 != releaseDstFile {
        let dstFile: *mut FILE = ress.dstFile;
        clearHandler();
        ress.dstFile = 0 as *mut FILE;
        if 0 != fclose(dstFile) {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: %s: %s \n\x00" as *const u8 as
                            *const libc::c_char, dstFileName,
                        strerror(*__errno_location()));
            }
            result = 1i32
        }
        if result != 0i32 &&
               0 !=
                   strcmp(dstFileName,
                          b"/dev/null\x00" as *const u8 as
                              *const libc::c_char) &&
               0 !=
                   strcmp(dstFileName,
                          b"/*stdout*\\\x00" as *const u8 as
                              *const libc::c_char) {
            FIO_remove(dstFileName);
        } else if 0 !=
                      strcmp(dstFileName,
                             b"/*stdout*\\\x00" as *const u8 as
                                 *const libc::c_char) &&
                      0 !=
                          strcmp(dstFileName,
                                 b"/dev/null\x00" as *const u8 as
                                     *const libc::c_char) &&
                      0 != transfer_permissions {
            UTIL_setFileStat(dstFileName, &mut statbuf);
        }
    }
    return result;
}
/* * FIO_decompressFrames() :
 *  Find and decode frames inside srcFile
 *  srcFile presumed opened and valid
 * @return : 0 : OK
 *           1 : error
 */
unsafe extern "C" fn FIO_decompressFrames(prefs: *mut FIO_prefs_t,
                                          mut ress: dRess_t,
                                          mut srcFile: *mut FILE,
                                          mut dstFileName:
                                              *const libc::c_char,
                                          mut srcFileName:
                                              *const libc::c_char)
 -> libc::c_int {
    let mut readSomething: libc::c_uint = 0i32 as libc::c_uint;
    let mut filesize: libc::c_ulonglong = 0i32 as libc::c_ulonglong;
    if !srcFile.is_null() {
    } else {
        __assert_fail(b"srcFile != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                          as *const u8 as *const libc::c_char,
                      1935i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 90],
                                                &[libc::c_char; 90]>(b"int FIO_decompressFrames(FIO_prefs_t *const, dRess_t, FILE *, const char *, const char *)\x00")).as_ptr());
    };
    loop  {
        /* check magic number -> version */
        let toRead: size_t = 4i32 as size_t;
        let buf: *const BYTE = ress.srcBuffer as *const BYTE;
        if ress.srcBufferLoaded < toRead {
            ress.srcBufferLoaded =
                (ress.srcBufferLoaded as
                     libc::c_ulong).wrapping_add(fread((ress.srcBuffer as
                                                            *mut libc::c_char).offset(ress.srcBufferLoaded
                                                                                          as
                                                                                          isize)
                                                           as
                                                           *mut libc::c_void,
                                                       1i32 as size_t,
                                                       toRead.wrapping_sub(ress.srcBufferLoaded),
                                                       srcFile)) as size_t as
                    size_t
        }
        if ress.srcBufferLoaded == 0i32 as libc::c_ulong {
            if readSomething == 0i32 as libc::c_uint {
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"zstd: %s: unexpected end of file \n\x00" as
                                *const u8 as *const libc::c_char,
                            srcFileName);
                }
                return 1i32
            }
            /* else, just reached frame boundary */
            /* no more input */
            break ;
        } else {
            readSomething = 1i32 as libc::c_uint;
            if ress.srcBufferLoaded < toRead {
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"zstd: %s: unknown header \n\x00" as *const u8 as
                                *const libc::c_char, srcFileName);
                }
                return 1i32
            }
            if 0 !=
                   ZSTD_isFrame(buf as *const libc::c_void,
                                ress.srcBufferLoaded) {
                let frameSize: libc::c_ulonglong =
                    FIO_decompressZstdFrame(prefs, &mut ress, srcFile,
                                            srcFileName, filesize as U64);
                if frameSize == -2i32 as libc::c_ulonglong { return 1i32 }
                filesize = filesize.wrapping_add(frameSize)
            } else if *buf.offset(0isize) as libc::c_int == 31i32 &&
                          *buf.offset(1isize) as libc::c_int == 139i32 {
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"zstd: %s: gzip file cannot be uncompressed (zstd compiled without HAVE_ZLIB) -- ignored \n\x00"
                                as *const u8 as *const libc::c_char,
                            srcFileName);
                }
                return 1i32
            } else if *buf.offset(0isize) as libc::c_int == 0xfdi32 &&
                          *buf.offset(1isize) as libc::c_int == 0x37i32 ||
                          *buf.offset(0isize) as libc::c_int == 0x5di32 &&
                              *buf.offset(1isize) as libc::c_int == 0i32 {
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"zstd: %s: xz/lzma file cannot be uncompressed (zstd compiled without HAVE_LZMA) -- ignored \n\x00"
                                as *const u8 as *const libc::c_char,
                            srcFileName);
                }
                return 1i32
            } else if MEM_readLE32(buf as *const libc::c_void) ==
                          0x184d2204i32 as libc::c_uint {
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"zstd: %s: lz4 file cannot be uncompressed (zstd compiled without HAVE_LZ4) -- ignored \n\x00"
                                as *const u8 as *const libc::c_char,
                            srcFileName);
                }
                return 1i32
            } else if 0 != (*prefs).overwrite &&
                          0 ==
                              strcmp(dstFileName,
                                     b"/*stdout*\\\x00" as *const u8 as
                                         *const libc::c_char) {
                return FIO_passThrough(prefs, ress.dstFile, srcFile,
                                       ress.srcBuffer, ress.srcBufferSize,
                                       ress.srcBufferLoaded) as libc::c_int
            } else {
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"zstd: %s: unsupported format \n\x00" as
                                *const u8 as *const libc::c_char,
                            srcFileName);
                }
                return 1i32
            }
        }
    }
    if g_display_prefs.displayLevel >= 2i32 {
        fprintf(stderr, b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char);
    }
    if g_display_prefs.displayLevel >= 2i32 {
        fprintf(stderr,
                b"%-20s: %llu bytes \n\x00" as *const u8 as
                    *const libc::c_char, srcFileName, filesize);
    }
    return 0i32;
}
/* * FIO_passThrough() : just copy input into output, for compatibility with gzip -df mode
    @return : 0 (no error) */
unsafe extern "C" fn FIO_passThrough(prefs: *mut FIO_prefs_t,
                                     mut foutput: *mut FILE,
                                     mut finput: *mut FILE,
                                     mut buffer: *mut libc::c_void,
                                     mut bufferSize: size_t,
                                     mut alreadyLoaded: size_t)
 -> libc::c_uint {
    let blockSize: size_t =
        if ((64i32 * (1i32 << 10i32)) as libc::c_ulong) < bufferSize {
            (64i32 * (1i32 << 10i32)) as libc::c_ulong
        } else { bufferSize };
    let mut readFromInput: size_t = 1i32 as size_t;
    let mut storedSkips: libc::c_uint = 0i32 as libc::c_uint;
    let sizeCheck: size_t =
        fwrite(buffer, 1i32 as size_t, alreadyLoaded, foutput);
    if sizeCheck != alreadyLoaded {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Pass-through write error \n\x00" as *const u8 as
                        *const libc::c_char);
        }
        return 1i32 as libc::c_uint
    }
    while 0 != readFromInput {
        readFromInput = fread(buffer, 1i32 as size_t, blockSize, finput);
        storedSkips =
            FIO_fwriteSparse(prefs, foutput, buffer, readFromInput,
                             storedSkips)
    }
    FIO_fwriteSparseEnd(prefs, foutput, storedSkips);
    return 0i32 as libc::c_uint;
}
unsafe extern "C" fn FIO_fwriteSparseEnd(prefs: *mut FIO_prefs_t,
                                         mut file: *mut FILE,
                                         mut storedSkips: libc::c_uint) {
    if storedSkips > 0i32 as libc::c_uint {
        if (*prefs).sparseFileSupport > 0i32 as libc::c_uint {
        } else {
            __assert_fail(b"prefs->sparseFileSupport > 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                              as *const u8 as *const libc::c_char,
                          1549i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 67],
                                                    &[libc::c_char; 67]>(b"void FIO_fwriteSparseEnd(FIO_prefs_t *const, FILE *, unsigned int)\x00")).as_ptr());
        };
        if fseek(file,
                 storedSkips.wrapping_sub(1i32 as libc::c_uint) as
                     libc::c_long, 1i32) != 0i32 {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 1551i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 69i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"Final skip error (sparse file support)\x00" as
                            *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(69i32);
        }
        let lastZeroByte: [libc::c_char; 1] = [0i32 as libc::c_char];
        if fwrite(lastZeroByte.as_ptr() as *const libc::c_void,
                  1i32 as size_t, 1i32 as size_t, file) !=
               1i32 as libc::c_ulong {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 1556i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 69i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"Write error : cannot write last zero\x00" as
                            *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(69i32);
        }
    };
}
// Initialized in run_static_initializers
static mut segmentSizeT: size_t = 0;
// Initialized in run_static_initializers
static mut maskT: size_t = 0;
/* * FIO_fwriteSparse() :
*   @return : storedSkips, to be provided to next call to FIO_fwriteSparse() of LZ4IO_fwriteSparseEnd() */
unsafe extern "C" fn FIO_fwriteSparse(prefs: *mut FIO_prefs_t,
                                      mut file: *mut FILE,
                                      mut buffer: *const libc::c_void,
                                      mut bufferSize: size_t,
                                      mut storedSkips: libc::c_uint)
 -> libc::c_uint {
    /* Buffer is supposed malloc'ed, hence aligned on size_t */
    let bufferT: *const size_t = buffer as *const size_t;
    let mut bufferSizeT: size_t =
        bufferSize.wrapping_div(::std::mem::size_of::<size_t>() as
                                    libc::c_ulong);
    let bufferTEnd: *const size_t = bufferT.offset(bufferSizeT as isize);
    let mut ptrT: *const size_t = bufferT;
    /* 0-test re-attempted every 32 KB */
    if 0 == (*prefs).sparseFileSupport {
        let sizeCheck: size_t =
            fwrite(buffer, 1i32 as size_t, bufferSize, file);
        if sizeCheck != bufferSize {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 1489i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 70i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"Write error : %s (cannot write decoded block)\x00"
                            as *const u8 as *const libc::c_char,
                        strerror(*__errno_location()));
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(70i32);
        }
        return 0i32 as libc::c_uint
    }
    if storedSkips > (1i32 as libc::c_uint).wrapping_mul(1u32 << 30i32) {
        let seekResult: libc::c_int =
            fseek(file,
                  (1i32 as libc::c_uint).wrapping_mul(1u32 << 30i32) as
                      libc::c_long, 1i32);
        if seekResult != 0i32 {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 1497i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 71i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"1 GB skip error (sparse file support)\x00" as
                            *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(71i32);
        }
        storedSkips =
            storedSkips.wrapping_sub((1i32 as
                                          libc::c_uint).wrapping_mul(1u32 <<
                                                                         30i32))
    }
    while ptrT < bufferTEnd {
        let mut seg0SizeT: size_t = segmentSizeT;
        let mut nb0T: size_t = 0;
        if seg0SizeT > bufferSizeT { seg0SizeT = bufferSizeT }
        bufferSizeT =
            (bufferSizeT as libc::c_ulong).wrapping_sub(seg0SizeT) as size_t
                as size_t;
        nb0T = 0i32 as size_t;
        while nb0T < seg0SizeT &&
                  *ptrT.offset(nb0T as isize) == 0i32 as libc::c_ulong {
            nb0T = nb0T.wrapping_add(1)
        }
        storedSkips =
            storedSkips.wrapping_add(nb0T.wrapping_mul(::std::mem::size_of::<size_t>()
                                                           as libc::c_ulong)
                                         as libc::c_uint);
        if nb0T != seg0SizeT {
            let seekResult_0: libc::c_int =
                fseek(file, storedSkips as libc::c_long, 1i32);
            if 0 != seekResult_0 {
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"zstd: \x00" as *const u8 as
                                *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 5i32 {
                    fprintf(stderr,
                            b"Error defined at %s, line %i : \n\x00" as
                                *const u8 as *const libc::c_char,
                            b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                                as *const u8 as *const libc::c_char, 1513i32);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"error %i : \x00" as *const u8 as
                                *const libc::c_char, 72i32);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"Sparse skip error ; try --no-sparse\x00" as
                                *const u8 as *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(72i32);
            }
            storedSkips = 0i32 as libc::c_uint;
            seg0SizeT =
                (seg0SizeT as libc::c_ulong).wrapping_sub(nb0T) as size_t as
                    size_t;
            ptrT = ptrT.offset(nb0T as isize);
            let sizeCheck_0: size_t =
                fwrite(ptrT as *const libc::c_void,
                       ::std::mem::size_of::<size_t>() as libc::c_ulong,
                       seg0SizeT, file);
            if sizeCheck_0 != seg0SizeT {
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"zstd: \x00" as *const u8 as
                                *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 5i32 {
                    fprintf(stderr,
                            b"Error defined at %s, line %i : \n\x00" as
                                *const u8 as *const libc::c_char,
                            b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                                as *const u8 as *const libc::c_char, 1519i32);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"error %i : \x00" as *const u8 as
                                *const libc::c_char, 73i32);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"Write error : cannot write decoded block\x00" as
                                *const u8 as *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(73i32);
            }
        }
        ptrT = ptrT.offset(seg0SizeT as isize)
    }
    if 0 != bufferSize & maskT {
        let restStart: *const libc::c_char =
            bufferTEnd as *const libc::c_char;
        let mut restPtr: *const libc::c_char = restStart;
        let mut restSize: size_t = bufferSize & maskT;
        let restEnd: *const libc::c_char =
            restStart.offset(restSize as isize);
        while restPtr < restEnd && *restPtr as libc::c_int == 0i32 {
            restPtr = restPtr.offset(1isize)
        }
        storedSkips =
            storedSkips.wrapping_add(restPtr.wrapping_offset_from(restStart)
                                         as libc::c_long as libc::c_uint);
        if restPtr != restEnd {
            let mut seekResult_1: libc::c_int =
                fseek(file, storedSkips as libc::c_long, 1i32);
            if 0 != seekResult_1 {
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"zstd: \x00" as *const u8 as
                                *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 5i32 {
                    fprintf(stderr,
                            b"Error defined at %s, line %i : \n\x00" as
                                *const u8 as *const libc::c_char,
                            b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                                as *const u8 as *const libc::c_char, 1536i32);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"error %i : \x00" as *const u8 as
                                *const libc::c_char, 74i32);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"Sparse skip error ; try --no-sparse\x00" as
                                *const u8 as *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(74i32);
            }
            storedSkips = 0i32 as libc::c_uint;
            let sizeCheck_1: size_t =
                fwrite(restPtr as *const libc::c_void, 1i32 as size_t,
                       restEnd.wrapping_offset_from(restPtr) as libc::c_long
                           as size_t, file);
            if sizeCheck_1 !=
                   restEnd.wrapping_offset_from(restPtr) as libc::c_long as
                       size_t {
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"zstd: \x00" as *const u8 as
                                *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 5i32 {
                    fprintf(stderr,
                            b"Error defined at %s, line %i : \n\x00" as
                                *const u8 as *const libc::c_char,
                            b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                                as *const u8 as *const libc::c_char, 1540i32);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"error %i : \x00" as *const u8 as
                                *const libc::c_char, 75i32);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"Write error : cannot write decoded end of block\x00"
                                as *const u8 as *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(75i32);
            }
        }
    }
    return storedSkips;
}
/* * FIO_decompressFrame() :
 *  @return : size of decoded zstd frame, or an error code
*/
unsafe extern "C" fn FIO_decompressZstdFrame(prefs: *mut FIO_prefs_t,
                                             mut ress: *mut dRess_t,
                                             mut finput: *mut FILE,
                                             mut srcFileName:
                                                 *const libc::c_char,
                                             mut alreadyDecoded: U64)
 -> libc::c_ulonglong {
    let mut frameSize: U64 = 0i32 as U64;
    let mut storedSkips: U32 = 0i32 as U32;
    let srcFileLength: size_t = strlen(srcFileName);
    if srcFileLength > 20i32 as libc::c_ulong {
        srcFileName =
            srcFileName.offset(srcFileLength.wrapping_sub(20i32 as
                                                              libc::c_ulong)
                                   as isize)
    }
    ZSTD_resetDStream((*ress).dctx);
    let toDecode: size_t = 18i32 as size_t;
    if (*ress).srcBufferLoaded < toDecode {
        let toRead: size_t = toDecode.wrapping_sub((*ress).srcBufferLoaded);
        let startPosition: *mut libc::c_void =
            ((*ress).srcBuffer as
                 *mut libc::c_char).offset((*ress).srcBufferLoaded as isize)
                as *mut libc::c_void;
        (*ress).srcBufferLoaded =
            ((*ress).srcBufferLoaded as
                 libc::c_ulong).wrapping_add(fread(startPosition,
                                                   1i32 as size_t, toRead,
                                                   finput)) as size_t as
                size_t
    }
    loop  {
        let mut inBuff: ZSTD_inBuffer =
            ZSTD_inBuffer_s{src: (*ress).srcBuffer,
                            size: (*ress).srcBufferLoaded,
                            pos: 0i32 as size_t,};
        let mut outBuff: ZSTD_outBuffer =
            ZSTD_outBuffer_s{dst: (*ress).dstBuffer,
                             size: (*ress).dstBufferSize,
                             pos: 0i32 as size_t,};
        let readSizeHint: size_t =
            ZSTD_decompressStream((*ress).dctx, &mut outBuff, &mut inBuff);
        if 0 != ZSTD_isError(readSizeHint) {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"%s : Decoding error (36) : %s \n\x00" as *const u8
                            as *const libc::c_char, srcFileName,
                        ZSTD_getErrorName(readSizeHint));
            }
            FIO_zstdErrorHelp(prefs, ress, readSizeHint, srcFileName);
            return -2i32 as libc::c_ulonglong
        }
        storedSkips =
            FIO_fwriteSparse(prefs, (*ress).dstFile, (*ress).dstBuffer,
                             outBuff.pos, storedSkips);
        frameSize =
            (frameSize as libc::c_ulong).wrapping_add(outBuff.pos) as U64 as
                U64;
        if g_display_prefs.displayLevel >= 2i32 &&
               0 == g_display_prefs.noProgress {
            if 0 == g_display_prefs.noProgress &&
                   UTIL_clockSpanMicro(g_displayClock) > g_refreshRate ||
                   g_display_prefs.displayLevel >= 4i32 {
                g_displayClock = UTIL_getTime();
                fprintf(stderr,
                        b"\r%-20.20s : %u MB...     \x00" as *const u8 as
                            *const libc::c_char, srcFileName,
                        (alreadyDecoded.wrapping_add(frameSize) >> 20i32) as
                            libc::c_uint);
                if g_display_prefs.displayLevel >= 4i32 { fflush(stderr); }
            }
        }
        if inBuff.pos > 0i32 as libc::c_ulong {
            memmove((*ress).srcBuffer,
                    ((*ress).srcBuffer as
                         *mut libc::c_char).offset(inBuff.pos as isize) as
                        *const libc::c_void,
                    inBuff.size.wrapping_sub(inBuff.pos));
            (*ress).srcBufferLoaded =
                ((*ress).srcBufferLoaded as
                     libc::c_ulong).wrapping_sub(inBuff.pos) as size_t as
                    size_t
        }
        if readSizeHint == 0i32 as libc::c_ulong {
            /* end of frame */
            break ;
        } else {
            if inBuff.size != inBuff.pos {
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"%s : Decoding error (37) : should consume entire input \n\x00"
                                as *const u8 as *const libc::c_char,
                            srcFileName);
                }
                return -2i32 as libc::c_ulonglong
            }
            let toDecode_0: size_t =
                if readSizeHint < (*ress).srcBufferSize {
                    readSizeHint
                } else { (*ress).srcBufferSize };
            if (*ress).srcBufferLoaded < toDecode_0 {
                let toRead_0: size_t =
                    toDecode_0.wrapping_sub((*ress).srcBufferLoaded);
                let startPosition_0: *mut libc::c_void =
                    ((*ress).srcBuffer as
                         *mut libc::c_char).offset((*ress).srcBufferLoaded as
                                                       isize) as
                        *mut libc::c_void;
                let readSize: size_t =
                    fread(startPosition_0, 1i32 as size_t, toRead_0, finput);
                if readSize == 0i32 as libc::c_ulong {
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b"%s : Read error (39) : premature end \n\x00"
                                    as *const u8 as *const libc::c_char,
                                srcFileName);
                    }
                    return -2i32 as libc::c_ulonglong
                }
                (*ress).srcBufferLoaded =
                    ((*ress).srcBufferLoaded as
                         libc::c_ulong).wrapping_add(readSize) as size_t as
                        size_t
            }
        }
    }
    FIO_fwriteSparseEnd(prefs, (*ress).dstFile, storedSkips);
    return frameSize as libc::c_ulonglong;
}
/* FIO_zstdErrorHelp() :
 * detailed error message when requested window size is too large */
unsafe extern "C" fn FIO_zstdErrorHelp(prefs: *mut FIO_prefs_t,
                                       mut ress: *mut dRess_t,
                                       mut err: size_t,
                                       mut srcFileName: *const libc::c_char) {
    let mut header: ZSTD_frameHeader =
        ZSTD_frameHeader{frameContentSize: 0,
                         windowSize: 0,
                         blockSizeMax: 0,
                         frameType: ZSTD_frame,
                         headerSize: 0,
                         dictID: 0,
                         checksumFlag: 0,};
    if ZSTD_getErrorCode(err) as libc::c_uint !=
           ZSTD_error_frameParameter_windowTooLarge as libc::c_int as
               libc::c_uint {
        return
    }
    err =
        ZSTD_getFrameHeader(&mut header, (*ress).srcBuffer,
                            (*ress).srcBufferLoaded);
    if err == 0i32 as libc::c_ulong {
        let windowSize: libc::c_ulonglong = header.windowSize;
        let windowLog: libc::c_uint =
            FIO_highbit64(windowSize).wrapping_add((windowSize &
                                                        windowSize.wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_ulonglong)
                                                        !=
                                                        0i32 as
                                                            libc::c_ulonglong)
                                                       as libc::c_int as
                                                       libc::c_uint);
        if (*prefs).memLimit > 0i32 as libc::c_uint {
        } else {
            __assert_fail(b"prefs->memLimit > 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                              as *const u8 as *const libc::c_char,
                          1613i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 76],
                                                    &[libc::c_char; 76]>(b"void FIO_zstdErrorHelp(FIO_prefs_t *const, dRess_t *, size_t, const char *)\x00")).as_ptr());
        };
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"%s : Window size larger than maximum : %llu > %u\n\x00"
                        as *const u8 as *const libc::c_char, srcFileName,
                    windowSize, (*prefs).memLimit);
        }
        if windowLog <=
               (if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                       4i32 as libc::c_ulong {
                    30i32
                } else { 31i32 }) as libc::c_uint {
            let windowMB: libc::c_uint =
                (windowSize >>
                     20i32).wrapping_add((windowSize &
                                              (1i32 * (1i32 << 20i32) - 1i32)
                                                  as libc::c_ulonglong !=
                                              0i32 as libc::c_ulonglong) as
                                             libc::c_int as libc::c_ulonglong)
                    as libc::c_uint;
            if windowSize < (1u64 << 52i32) as U64 as libc::c_ulonglong {
            } else {
                __assert_fail(b"windowSize < (U64)(1ULL << 52)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                                  as *const u8 as *const libc::c_char,
                              1618i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 76],
                                                        &[libc::c_char; 76]>(b"void FIO_zstdErrorHelp(FIO_prefs_t *const, dRess_t *, size_t, const char *)\x00")).as_ptr());
            };
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"%s : Use --long=%u or --memory=%uMB\n\x00" as
                            *const u8 as *const libc::c_char, srcFileName,
                        windowLog, windowMB);
            }
            return
        }
    }
    if g_display_prefs.displayLevel >= 1i32 {
        fprintf(stderr,
                b"%s : Window log larger than ZSTD_WINDOWLOG_MAX=%u; not supported\n\x00"
                    as *const u8 as *const libc::c_char, srcFileName,
                if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
                       4i32 as libc::c_ulong {
                    30i32
                } else { 31i32 });
    };
}
/* FIO_highbit64() :
 * gives position of highest bit.
 * note : only works for v > 0 !
 */
unsafe extern "C" fn FIO_highbit64(mut v: libc::c_ulonglong) -> libc::c_uint {
    let mut count: libc::c_uint = 0i32 as libc::c_uint;
    if v != 0i32 as libc::c_ulonglong {
    } else {
        __assert_fail(b"v != 0\x00" as *const u8 as *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                          as *const u8 as *const libc::c_char,
                      1592i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"unsigned int FIO_highbit64(unsigned long long)\x00")).as_ptr());
    };
    v >>= 1i32;
    while 0 != v { v >>= 1i32; count = count.wrapping_add(1) }
    return count;
}
unsafe extern "C" fn FIO_freeDResources(mut ress: dRess_t) {
    let mut err: size_t = 0;
    err = ZSTD_freeDStream(ress.dctx);
    if 0 != ZSTD_isError(err) {
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr, b"%s \n\x00" as *const u8 as *const libc::c_char,
                    b"ZSTD_freeDStream(ress.dctx)\x00" as *const u8 as
                        *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: \x00" as *const u8 as *const libc::c_char);
        }
        if g_display_prefs.displayLevel >= 5i32 {
            fprintf(stderr,
                    b"Error defined at %s, line %i : \n\x00" as *const u8 as
                        *const libc::c_char,
                    b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00" as
                        *const u8 as *const libc::c_char, 1469i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"error %i : \x00" as *const u8 as *const libc::c_char,
                    11i32);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    ZSTD_getErrorName(err));
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(11i32);
    }
    free(ress.srcBuffer);
    free(ress.dstBuffer);
}
#[no_mangle]
pub unsafe extern "C" fn FIO_listMultipleFiles(mut numFiles: libc::c_uint,
                                               mut filenameTable:
                                                   *mut *const libc::c_char,
                                               mut displayLevel: libc::c_int)
 -> libc::c_int {
    let mut u: libc::c_uint = 0;
    u = 0i32 as libc::c_uint;
    while u < numFiles {
        if 0 ==
               strcmp(*filenameTable.offset(u as isize),
                      b"/*stdin*\\\x00" as *const u8 as *const libc::c_char) {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: --list does not support reading from standard input\x00"
                            as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            return 1i32
        }
        u = u.wrapping_add(1)
    }
    if numFiles == 0i32 as libc::c_uint {
        if 0 == isatty(fileno(stdin)) {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: --list does not support reading from standard input \n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"No files given \n\x00" as *const u8 as
                        *const libc::c_char);
        }
        return 1i32
    }
    if displayLevel <= 2i32 {
        fprintf(stdout,
                b"Frames  Skips  Compressed  Uncompressed  Ratio  Check  Filename\n\x00"
                    as *const u8 as *const libc::c_char);
    }
    let mut error: libc::c_int = 0i32;
    let mut total: fileInfo_t =
        fileInfo_t{decompressedSize: 0,
                   compressedSize: 0,
                   windowSize: 0,
                   numActualFrames: 0,
                   numSkippableFrames: 0,
                   decompUnavailable: 0,
                   usesCheck: 0,
                   nbFiles: 0,};
    memset(&mut total as *mut fileInfo_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<fileInfo_t>() as libc::c_ulong);
    total.usesCheck = 1i32;
    let mut u_0: libc::c_uint = 0;
    u_0 = 0i32 as libc::c_uint;
    while u_0 < numFiles {
        error |=
            FIO_listFile(&mut total, *filenameTable.offset(u_0 as isize),
                         displayLevel);
        u_0 = u_0.wrapping_add(1)
    }
    if numFiles > 1i32 as libc::c_uint && displayLevel <= 2i32 {
        let unit: libc::c_uint =
            (if total.compressedSize <
                    (1i32 * (1i32 << 20i32)) as libc::c_ulong {
                 1i32 * (1i32 << 10i32)
             } else { 1i32 * (1i32 << 20i32) }) as libc::c_uint;
        let unitStr: *const libc::c_char =
            if total.compressedSize <
                   (1i32 * (1i32 << 20i32)) as libc::c_ulong {
                b"KB\x00" as *const u8 as *const libc::c_char
            } else { b"MB\x00" as *const u8 as *const libc::c_char };
        let compressedSizeUnit: libc::c_double =
            total.compressedSize as libc::c_double / unit as libc::c_double;
        let decompressedSizeUnit: libc::c_double =
            total.decompressedSize as libc::c_double / unit as libc::c_double;
        let ratio: libc::c_double =
            if total.compressedSize == 0i32 as libc::c_ulong {
                0i32 as libc::c_double
            } else {
                total.decompressedSize as libc::c_double /
                    total.compressedSize as libc::c_double
            };
        let checkString: *const libc::c_char =
            if 0 != total.usesCheck {
                b"XXH64\x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char };
        fprintf(stdout,
                b"----------------------------------------------------------------- \n\x00"
                    as *const u8 as *const libc::c_char);
        if 0 != total.decompUnavailable {
            fprintf(stdout,
                    b"%6d  %5d  %7.2f %2s                       %5s  %u files\n\x00"
                        as *const u8 as *const libc::c_char,
                    total.numSkippableFrames + total.numActualFrames,
                    total.numSkippableFrames, compressedSizeUnit, unitStr,
                    checkString, total.nbFiles);
        } else {
            fprintf(stdout,
                    b"%6d  %5d  %7.2f %2s  %9.2f %2s  %5.3f  %5s  %u files\n\x00"
                        as *const u8 as *const libc::c_char,
                    total.numSkippableFrames + total.numActualFrames,
                    total.numSkippableFrames, compressedSizeUnit, unitStr,
                    decompressedSizeUnit, unitStr, ratio, checkString,
                    total.nbFiles);
        }
    }
    return error;
}
unsafe extern "C" fn FIO_listFile(mut total: *mut fileInfo_t,
                                  mut inFileName: *const libc::c_char,
                                  mut displayLevel: libc::c_int)
 -> libc::c_int {
    let mut info: fileInfo_t =
        fileInfo_t{decompressedSize: 0,
                   compressedSize: 0,
                   windowSize: 0,
                   numActualFrames: 0,
                   numSkippableFrames: 0,
                   decompUnavailable: 0,
                   usesCheck: 0,
                   nbFiles: 0,};
    memset(&mut info as *mut fileInfo_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<fileInfo_t>() as libc::c_ulong);
    let error: InfoError = getFileInfo(&mut info, inFileName);
    match error as libc::c_uint {
        1 => {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"Error while parsing \"%s\" \n\x00" as *const u8 as
                            *const libc::c_char, inFileName);
            }
        }
        2 => {
            fprintf(stdout,
                    b"File \"%s\" not compressed by zstd \n\x00" as *const u8
                        as *const libc::c_char, inFileName);
            if displayLevel > 2i32 {
                fprintf(stdout,
                        b"\n\x00" as *const u8 as *const libc::c_char);
            }
            return 1i32
        }
        3 => {
            if displayLevel > 2i32 {
                fprintf(stdout,
                        b"\n\x00" as *const u8 as *const libc::c_char);
            }
            return 1i32
        }
        4 => {
            fprintf(stdout,
                    b"File \"%s\" is truncated \n\x00" as *const u8 as
                        *const libc::c_char, inFileName);
            if displayLevel > 2i32 {
                fprintf(stdout,
                        b"\n\x00" as *const u8 as *const libc::c_char);
            }
            return 1i32
        }
        0 | _ => { }
    }
    displayInfo(inFileName, &mut info, displayLevel);
    *total = FIO_addFInfo(*total, info);
    if error as libc::c_uint == info_success as libc::c_int as libc::c_uint ||
           error as libc::c_uint ==
               info_frame_error as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"error == info_success || error == info_frame_error\x00"
                          as *const u8 as *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                          as *const u8 as *const libc::c_char,
                      2467i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 50],
                                                &[libc::c_char; 50]>(b"int FIO_listFile(fileInfo_t *, const char *, int)\x00")).as_ptr());
    };
    return error as libc::c_int;
}
/* * getFileInfo() :
 *  Reads information from file, stores in *info
 * @return : InfoError status
 */
unsafe extern "C" fn getFileInfo(mut info: *mut fileInfo_t,
                                 mut srcFileName: *const libc::c_char)
 -> InfoError {
    if 0 == UTIL_isRegularFile(srcFileName) {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Error : %s is not a file\x00" as *const u8 as
                        *const libc::c_char, srcFileName);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        return info_file_error
    }
    return getFileInfo_fileConfirmed(info, srcFileName);
}
unsafe extern "C" fn getFileInfo_fileConfirmed(mut info: *mut fileInfo_t,
                                               mut inFileName:
                                                   *const libc::c_char)
 -> InfoError {
    let mut status: InfoError = info_success;
    let srcFile: *mut FILE = FIO_openSrcFile(inFileName);
    if srcFile.is_null() {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"Error: could not open source file %s\x00" as *const u8
                        as *const libc::c_char, inFileName);
        }
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        return info_file_error
    }
    (*info).compressedSize = UTIL_getFileSize(inFileName);
    status = FIO_analyzeFrames(info, srcFile);
    fclose(srcFile);
    (*info).nbFiles = 1i32 as U32;
    return status;
}
unsafe extern "C" fn FIO_analyzeFrames(mut info: *mut fileInfo_t,
                                       srcFile: *mut FILE) -> InfoError {
    loop  {
        let mut headerBuffer: [BYTE; 18] = [0; 18];
        let numBytesRead: size_t =
            fread(headerBuffer.as_mut_ptr() as *mut libc::c_void,
                  1i32 as size_t,
                  ::std::mem::size_of::<[BYTE; 18]>() as libc::c_ulong,
                  srcFile);
        if numBytesRead < 6i32 as libc::c_ulong {
            if 0 != feof(srcFile) && numBytesRead == 0i32 as libc::c_ulong &&
                   (*info).compressedSize > 0i32 as libc::c_ulong &&
                   (*info).compressedSize != -1i32 as U64 {
                let mut file_position: libc::c_ulonglong =
                    ftell(srcFile) as libc::c_ulonglong;
                let mut file_size: libc::c_ulonglong =
                    (*info).compressedSize as libc::c_ulonglong;
                if file_position != file_size {
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b"Error: seeked to position %llu, which is beyond file size of %llu\n\x00"
                                    as *const u8 as *const libc::c_char,
                                file_position, file_size);
                    }
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b" \n\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    return info_truncated_input
                }
                /* correct end of file => success */
                break ;
            } else {
                if 0 != feof(srcFile) {
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b"Error: reached end of file with incomplete frame\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b" \n\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    return info_not_zstd
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"Error: did not reach end of file but ran out of frames\x00"
                                as *const u8 as *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                return info_frame_error
            }
        } else {
            let magicNumber: U32 =
                MEM_readLE32(headerBuffer.as_mut_ptr() as
                                 *const libc::c_void);
            if magicNumber == 0xfd2fb528u32 {
                let mut header: ZSTD_frameHeader =
                    ZSTD_frameHeader{frameContentSize: 0,
                                     windowSize: 0,
                                     blockSizeMax: 0,
                                     frameType: ZSTD_frame,
                                     headerSize: 0,
                                     dictID: 0,
                                     checksumFlag: 0,};
                let frameContentSize: U64 =
                    ZSTD_getFrameContentSize(headerBuffer.as_mut_ptr() as
                                                 *const libc::c_void,
                                             numBytesRead) as U64;
                if frameContentSize as libc::c_ulonglong ==
                       0u64.wrapping_sub(2i32 as libc::c_ulonglong) ||
                       frameContentSize as libc::c_ulonglong ==
                           0u64.wrapping_sub(1i32 as libc::c_ulonglong) {
                    (*info).decompUnavailable = 1i32
                } else {
                    (*info).decompressedSize =
                        ((*info).decompressedSize as
                             libc::c_ulong).wrapping_add(frameContentSize) as
                            U64 as U64
                }
                if ZSTD_getFrameHeader(&mut header,
                                       headerBuffer.as_mut_ptr() as
                                           *const libc::c_void, numBytesRead)
                       != 0i32 as libc::c_ulong {
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b"Error: could not decode frame header\x00" as
                                    *const u8 as *const libc::c_char);
                    }
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b" \n\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    return info_frame_error
                }
                (*info).windowSize = header.windowSize as U64;
                let headerSize: size_t =
                    ZSTD_frameHeaderSize(headerBuffer.as_mut_ptr() as
                                             *const libc::c_void,
                                         numBytesRead);
                if 0 != ZSTD_isError(headerSize) {
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b"Error: could not determine frame header size\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b" \n\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    return info_frame_error
                }
                if fseek(srcFile,
                         headerSize as libc::c_long -
                             numBytesRead as libc::c_long, 1i32) != 0i32 {
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b"Error: could not move to end of frame header\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b" \n\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    return info_frame_error
                }
                let mut lastBlock: libc::c_int = 0i32;
                loop  {
                    let mut blockHeaderBuffer: [BYTE; 3] = [0; 3];
                    if fread(blockHeaderBuffer.as_mut_ptr() as
                                 *mut libc::c_void, 1i32 as size_t,
                             3i32 as size_t, srcFile) != 3i32 as libc::c_ulong
                       {
                        if g_display_prefs.displayLevel >= 1i32 {
                            fprintf(stderr,
                                    b"Error while reading block header\x00" as
                                        *const u8 as *const libc::c_char);
                        }
                        if g_display_prefs.displayLevel >= 1i32 {
                            fprintf(stderr,
                                    b" \n\x00" as *const u8 as
                                        *const libc::c_char);
                        }
                        return info_frame_error
                    }
                    let blockHeader: U32 =
                        MEM_readLE24(blockHeaderBuffer.as_mut_ptr() as
                                         *const libc::c_void);
                    let blockTypeID: U32 =
                        blockHeader >> 1i32 & 3i32 as libc::c_uint;
                    let isRLE: U32 =
                        (blockTypeID == 1i32 as libc::c_uint) as libc::c_int
                            as U32;
                    let isWrongBlock: U32 =
                        (blockTypeID == 3i32 as libc::c_uint) as libc::c_int
                            as U32;
                    let blockSize: libc::c_long =
                        if 0 != isRLE {
                            1i32 as libc::c_long
                        } else { (blockHeader >> 3i32) as libc::c_long };
                    if 0 != isWrongBlock {
                        if g_display_prefs.displayLevel >= 1i32 {
                            fprintf(stderr,
                                    b"Error: unsupported block type\x00" as
                                        *const u8 as *const libc::c_char);
                        }
                        if g_display_prefs.displayLevel >= 1i32 {
                            fprintf(stderr,
                                    b" \n\x00" as *const u8 as
                                        *const libc::c_char);
                        }
                        return info_frame_error
                    }
                    lastBlock =
                        (blockHeader & 1i32 as libc::c_uint) as libc::c_int;
                    if fseek(srcFile, blockSize, 1i32) != 0i32 {
                        if g_display_prefs.displayLevel >= 1i32 {
                            fprintf(stderr,
                                    b"Error: could not skip to end of block\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                        if g_display_prefs.displayLevel >= 1i32 {
                            fprintf(stderr,
                                    b" \n\x00" as *const u8 as
                                        *const libc::c_char);
                        }
                        return info_frame_error
                    }
                    if !(lastBlock != 1i32) { break ; }
                }
                let frameHeaderDescriptor: BYTE = headerBuffer[4usize];
                let contentChecksumFlag: libc::c_int =
                    (frameHeaderDescriptor as libc::c_int & 1i32 << 2i32) >>
                        2i32;
                if 0 != contentChecksumFlag {
                    (*info).usesCheck = 1i32;
                    if fseek(srcFile, 4i32 as libc::c_long, 1i32) != 0i32 {
                        if g_display_prefs.displayLevel >= 1i32 {
                            fprintf(stderr,
                                    b"Error: could not skip past checksum\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                        if g_display_prefs.displayLevel >= 1i32 {
                            fprintf(stderr,
                                    b" \n\x00" as *const u8 as
                                        *const libc::c_char);
                        }
                        return info_frame_error
                    }
                }
                (*info).numActualFrames += 1
            } else if magicNumber & 0xfffffff0u32 ==
                          0x184d2a50i32 as libc::c_uint {
                let frameSize: U32 =
                    MEM_readLE32(headerBuffer.as_mut_ptr().offset(4isize) as
                                     *const libc::c_void);
                let seek: libc::c_long =
                    ((8i32 as libc::c_uint).wrapping_add(frameSize) as
                         libc::c_ulong).wrapping_sub(numBytesRead) as
                        libc::c_long;
                if fseek(srcFile, seek, 1i32) != 0i32 {
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b"Error: could not find end of skippable frame\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    if g_display_prefs.displayLevel >= 1i32 {
                        fprintf(stderr,
                                b" \n\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    return info_frame_error
                }
                (*info).numSkippableFrames += 1
            } else { return info_not_zstd }
        }
    }
    return info_success;
}
unsafe extern "C" fn FIO_addFInfo(mut fi1: fileInfo_t, mut fi2: fileInfo_t)
 -> fileInfo_t {
    let mut total: fileInfo_t =
        fileInfo_t{decompressedSize: 0,
                   compressedSize: 0,
                   windowSize: 0,
                   numActualFrames: 0,
                   numSkippableFrames: 0,
                   decompUnavailable: 0,
                   usesCheck: 0,
                   nbFiles: 0,};
    memset(&mut total as *mut fileInfo_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<fileInfo_t>() as libc::c_ulong);
    total.numActualFrames = fi1.numActualFrames + fi2.numActualFrames;
    total.numSkippableFrames =
        fi1.numSkippableFrames + fi2.numSkippableFrames;
    total.compressedSize =
        fi1.compressedSize.wrapping_add(fi2.compressedSize);
    total.decompressedSize =
        fi1.decompressedSize.wrapping_add(fi2.decompressedSize);
    total.decompUnavailable = fi1.decompUnavailable | fi2.decompUnavailable;
    total.usesCheck = fi1.usesCheck & fi2.usesCheck;
    total.nbFiles = fi1.nbFiles.wrapping_add(fi2.nbFiles);
    return total;
}
unsafe extern "C" fn displayInfo(mut inFileName: *const libc::c_char,
                                 mut info: *const fileInfo_t,
                                 mut displayLevel: libc::c_int) {
    let unit: libc::c_uint =
        (if (*info).compressedSize < (1i32 * (1i32 << 20i32)) as libc::c_ulong
            {
             1i32 * (1i32 << 10i32)
         } else { 1i32 * (1i32 << 20i32) }) as libc::c_uint;
    let unitStr: *const libc::c_char =
        if (*info).compressedSize < (1i32 * (1i32 << 20i32)) as libc::c_ulong
           {
            b"KB\x00" as *const u8 as *const libc::c_char
        } else { b"MB\x00" as *const u8 as *const libc::c_char };
    let windowSizeUnit: libc::c_double =
        (*info).windowSize as libc::c_double / unit as libc::c_double;
    let compressedSizeUnit: libc::c_double =
        (*info).compressedSize as libc::c_double / unit as libc::c_double;
    let decompressedSizeUnit: libc::c_double =
        (*info).decompressedSize as libc::c_double / unit as libc::c_double;
    let ratio: libc::c_double =
        if (*info).compressedSize == 0i32 as libc::c_ulong {
            0i32 as libc::c_double
        } else {
            (*info).decompressedSize as libc::c_double /
                (*info).compressedSize as libc::c_double
        };
    let checkString: *const libc::c_char =
        if 0 != (*info).usesCheck {
            b"XXH64\x00" as *const u8 as *const libc::c_char
        } else { b"None\x00" as *const u8 as *const libc::c_char };
    if displayLevel <= 2i32 {
        if 0 == (*info).decompUnavailable {
            fprintf(stdout,
                    b"%6d  %5d  %7.2f %2s  %9.2f %2s  %5.3f  %5s  %s\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*info).numSkippableFrames + (*info).numActualFrames,
                    (*info).numSkippableFrames, compressedSizeUnit, unitStr,
                    decompressedSizeUnit, unitStr, ratio, checkString,
                    inFileName);
        } else {
            fprintf(stdout,
                    b"%6d  %5d  %7.2f %2s                       %5s  %s\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*info).numSkippableFrames + (*info).numActualFrames,
                    (*info).numSkippableFrames, compressedSizeUnit, unitStr,
                    checkString, inFileName);
        }
    } else {
        fprintf(stdout, b"%s \n\x00" as *const u8 as *const libc::c_char,
                inFileName);
        fprintf(stdout,
                b"# Zstandard Frames: %d\n\x00" as *const u8 as
                    *const libc::c_char, (*info).numActualFrames);
        if 0 != (*info).numSkippableFrames {
            fprintf(stdout,
                    b"# Skippable Frames: %d\n\x00" as *const u8 as
                        *const libc::c_char, (*info).numSkippableFrames);
        }
        fprintf(stdout,
                b"Window Size: %.2f %2s (%llu B)\n\x00" as *const u8 as
                    *const libc::c_char, windowSizeUnit, unitStr,
                (*info).windowSize as libc::c_ulonglong);
        fprintf(stdout,
                b"Compressed Size: %.2f %2s (%llu B)\n\x00" as *const u8 as
                    *const libc::c_char, compressedSizeUnit, unitStr,
                (*info).compressedSize as libc::c_ulonglong);
        if 0 == (*info).decompUnavailable {
            fprintf(stdout,
                    b"Decompressed Size: %.2f %2s (%llu B)\n\x00" as *const u8
                        as *const libc::c_char, decompressedSizeUnit, unitStr,
                    (*info).decompressedSize as libc::c_ulonglong);
            fprintf(stdout,
                    b"Ratio: %.4f\n\x00" as *const u8 as *const libc::c_char,
                    ratio);
        }
        fprintf(stdout,
                b"Check: %s\n\x00" as *const u8 as *const libc::c_char,
                checkString);
        fprintf(stdout, b"\n\x00" as *const u8 as *const libc::c_char);
    };
}
/*-*************************************
*  Multiple File functions
***************************************/
/** FIO_compressMultipleFilenames() :
    @return : nb of missing files */
#[no_mangle]
pub unsafe extern "C" fn FIO_compressMultipleFilenames(prefs:
                                                           *mut FIO_prefs_t,
                                                       mut inFileNamesTable:
                                                           *mut *const libc::c_char,
                                                       mut nbFiles:
                                                           libc::c_uint,
                                                       mut outFileName:
                                                           *const libc::c_char,
                                                       mut suffix:
                                                           *const libc::c_char,
                                                       mut dictFileName:
                                                           *const libc::c_char,
                                                       mut compressionLevel:
                                                           libc::c_int,
                                                       mut comprParams:
                                                           ZSTD_compressionParameters)
 -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let firstFileSize: U64 =
        UTIL_getFileSize(*inFileNamesTable.offset(0isize));
    let firstSrcSize: U64 =
        (if firstFileSize == -1i32 as U64 {
             0u64.wrapping_sub(1i32 as libc::c_ulonglong)
         } else { firstFileSize as libc::c_ulonglong }) as U64;
    let srcSize: U64 =
        (if nbFiles != 1i32 as libc::c_uint {
             0u64.wrapping_sub(1i32 as libc::c_ulonglong)
         } else { firstSrcSize as libc::c_ulonglong }) as U64;
    let mut ress: cRess_t =
        FIO_createCResources(prefs, dictFileName, compressionLevel, srcSize,
                             comprParams);
    if !outFileName.is_null() || !suffix.is_null() {
    } else {
        __assert_fail(b"outFileName != NULL || suffix != NULL\x00" as
                          *const u8 as *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                          as *const u8 as *const libc::c_char,
                      1394i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 158],
                                                &[libc::c_char; 158]>(b"int FIO_compressMultipleFilenames(FIO_prefs_t *const, const char **, unsigned int, const char *, const char *, const char *, int, ZSTD_compressionParameters)\x00")).as_ptr());
    };
    if !outFileName.is_null() {
        ress.dstFile =
            FIO_openDstFile(prefs, 0 as *const libc::c_char, outFileName);
        if ress.dstFile.is_null() {
            error = 1i32
        } else {
            let mut u: libc::c_uint = 0;
            u = 0i32 as libc::c_uint;
            while u < nbFiles {
                error |=
                    FIO_compressFilename_srcFile(prefs, ress, outFileName,
                                                 *inFileNamesTable.offset(u as
                                                                              isize),
                                                 compressionLevel);
                u = u.wrapping_add(1)
            }
            if 0 != fclose(ress.dstFile) {
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"zstd: \x00" as *const u8 as
                                *const libc::c_char);
                }
                if g_display_prefs.displayLevel >= 5i32 {
                    fprintf(stderr,
                            b"Error defined at %s, line %i : \n\x00" as
                                *const u8 as *const libc::c_char,
                            b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                                as *const u8 as *const libc::c_char, 1406i32);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"error %i : \x00" as *const u8 as
                                *const libc::c_char, 29i32);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b"Write error (%s) : cannot properly close %s\x00"
                                as *const u8 as *const libc::c_char,
                            strerror(*__errno_location()), outFileName);
                }
                if g_display_prefs.displayLevel >= 1i32 {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(29i32);
            }
            ress.dstFile = 0 as *mut FILE
        }
    } else {
        let mut u_0: libc::c_uint = 0;
        u_0 = 0i32 as libc::c_uint;
        while u_0 < nbFiles {
            let srcFileName: *const libc::c_char =
                *inFileNamesTable.offset(u_0 as isize);
            let dstFileName: *const libc::c_char =
                FIO_determineCompressedName(srcFileName, suffix);
            error |=
                FIO_compressFilename_srcFile(prefs, ress, dstFileName,
                                             srcFileName, compressionLevel);
            u_0 = u_0.wrapping_add(1)
        }
    }
    FIO_freeCResources(ress);
    return error;
}
/* FIO_determineCompressedName() :
 * create a destination filename for compressed srcFileName.
 * @return a pointer to it.
 * This function never returns an error (it may abort() in case of pb)
 */
unsafe extern "C" fn FIO_determineCompressedName(mut srcFileName:
                                                     *const libc::c_char,
                                                 mut suffix:
                                                     *const libc::c_char)
 -> *const libc::c_char {
    static mut dfnbCapacity: size_t = 0i32 as size_t;
    /* using static allocation : this function cannot be multi-threaded */
    static mut dstFileNameBuffer: *mut libc::c_char =
        0 as *const libc::c_char as *mut libc::c_char;
    let sfnSize: size_t = strlen(srcFileName);
    let suffixSize: size_t = strlen(suffix);
    if dfnbCapacity <=
           sfnSize.wrapping_add(suffixSize).wrapping_add(1i32 as
                                                             libc::c_ulong) {
        free(dstFileNameBuffer as *mut libc::c_void);
        dfnbCapacity =
            sfnSize.wrapping_add(suffixSize).wrapping_add(30i32 as
                                                              libc::c_ulong);
        dstFileNameBuffer = malloc(dfnbCapacity) as *mut libc::c_char;
        if dstFileNameBuffer.is_null() {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 1366i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 30i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: %s\x00" as *const u8 as *const libc::c_char,
                        strerror(*__errno_location()));
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(30i32);
        }
    }
    if !dstFileNameBuffer.is_null() {
    } else {
        __assert_fail(b"dstFileNameBuffer != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                          as *const u8 as *const libc::c_char,
                      1368i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 68],
                                                &[libc::c_char; 68]>(b"const char *FIO_determineCompressedName(const char *, const char *)\x00")).as_ptr());
    };
    memcpy(dstFileNameBuffer as *mut libc::c_void,
           srcFileName as *const libc::c_void, sfnSize);
    memcpy(dstFileNameBuffer.offset(sfnSize as isize) as *mut libc::c_void,
           suffix as *const libc::c_void,
           suffixSize.wrapping_add(1i32 as libc::c_ulong));
    return dstFileNameBuffer;
}
/* * FIO_decompressMultipleFilenames() :
    @return : nb of missing or skipped files */
#[no_mangle]
pub unsafe extern "C" fn FIO_decompressMultipleFilenames(prefs:
                                                             *mut FIO_prefs_t,
                                                         mut srcNamesTable:
                                                             *mut *const libc::c_char,
                                                         mut nbFiles:
                                                             libc::c_uint,
                                                         mut outFileName:
                                                             *const libc::c_char,
                                                         mut dictFileName:
                                                             *const libc::c_char)
 -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut ress: dRess_t = FIO_createDResources(prefs, dictFileName);
    if !outFileName.is_null() {
        let mut u: libc::c_uint = 0;
        ress.dstFile =
            FIO_openDstFile(prefs, 0 as *const libc::c_char, outFileName);
        if ress.dstFile.is_null() {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 2203i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 71i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"cannot open %s\x00" as *const u8 as
                            *const libc::c_char, outFileName);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(71i32);
        }
        u = 0i32 as libc::c_uint;
        while u < nbFiles {
            error |=
                FIO_decompressSrcFile(prefs, ress, outFileName,
                                      *srcNamesTable.offset(u as isize));
            u = u.wrapping_add(1)
        }
        if 0 != fclose(ress.dstFile) {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 2208i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 72i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"Write error : %s : cannot properly close output file\x00"
                            as *const u8 as *const libc::c_char,
                        strerror(*__errno_location()));
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(72i32);
        }
    } else {
        let mut u_0: libc::c_uint = 0;
        u_0 = 0i32 as libc::c_uint;
        while u_0 < nbFiles {
            /* create dstFileName */
            let srcFileName: *const libc::c_char =
                *srcNamesTable.offset(u_0 as isize);
            let dstFileName: *const libc::c_char =
                FIO_determineDstName(srcFileName);
            if dstFileName.is_null() {
                error = 1i32
            } else {
                error |=
                    FIO_decompressSrcFile(prefs, ress, dstFileName,
                                          srcFileName)
            }
            u_0 = u_0.wrapping_add(1)
        }
    }
    FIO_freeDResources(ress);
    return error;
}
/* FIO_determineDstName() :
 * create a destination filename from a srcFileName.
 * @return a pointer to it.
 * @return == NULL if there is an error */
unsafe extern "C" fn FIO_determineDstName(mut srcFileName:
                                              *const libc::c_char)
 -> *const libc::c_char {
    static mut dfnbCapacity: size_t = 0i32 as size_t;
    /* using static allocation : this function cannot be multi-threaded */
    static mut dstFileNameBuffer: *mut libc::c_char =
        0 as *const libc::c_char as *mut libc::c_char;
    let sfnSize: size_t = strlen(srcFileName);
    let mut suffixSize: size_t = 0;
    let suffixPtr: *const libc::c_char = strrchr(srcFileName, '.' as i32);
    if suffixPtr.is_null() {
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: %s: unknown suffix -- ignored \n\x00" as *const u8
                        as *const libc::c_char, srcFileName);
        }
        return 0 as *const libc::c_char
    }
    suffixSize = strlen(suffixPtr);
    if sfnSize <= suffixSize ||
           0 !=
               strcmp(suffixPtr,
                      b".zst\x00" as *const u8 as *const libc::c_char) {
        let mut suffixlist: *const libc::c_char =
            b".zst\x00" as *const u8 as *const libc::c_char;
        if g_display_prefs.displayLevel >= 1i32 {
            fprintf(stderr,
                    b"zstd: %s: unknown suffix (%s expected) -- ignored \n\x00"
                        as *const u8 as *const libc::c_char, srcFileName,
                    suffixlist);
        }
        return 0 as *const libc::c_char
    }
    if dfnbCapacity.wrapping_add(suffixSize) <=
           sfnSize.wrapping_add(1i32 as libc::c_ulong) {
        free(dstFileNameBuffer as *mut libc::c_void);
        dfnbCapacity = sfnSize.wrapping_add(20i32 as libc::c_ulong);
        dstFileNameBuffer = malloc(dfnbCapacity) as *mut libc::c_char;
        if dstFileNameBuffer.is_null() {
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"zstd: \x00" as *const u8 as *const libc::c_char);
            }
            if g_display_prefs.displayLevel >= 5i32 {
                fprintf(stderr,
                        b"Error defined at %s, line %i : \n\x00" as *const u8
                            as *const libc::c_char,
                        b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                            as *const u8 as *const libc::c_char, 2178i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"error %i : \x00" as *const u8 as
                            *const libc::c_char, 74i32);
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b"%s : not enough memory for dstFileName\x00" as
                            *const u8 as *const libc::c_char,
                        strerror(*__errno_location()));
            }
            if g_display_prefs.displayLevel >= 1i32 {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(74i32);
        }
    }
    if !dstFileNameBuffer.is_null() {
    } else {
        __assert_fail(b"dstFileNameBuffer != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/fileio.c\x00"
                          as *const u8 as *const libc::c_char,
                      2182i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"const char *FIO_determineDstName(const char *)\x00")).as_ptr());
    };
    memcpy(dstFileNameBuffer as *mut libc::c_void,
           srcFileName as *const libc::c_void,
           sfnSize.wrapping_sub(suffixSize));
    *dstFileNameBuffer.offset(sfnSize.wrapping_sub(suffixSize) as isize) =
        '\u{0}' as i32 as libc::c_char;
    return dstFileNameBuffer;
}
/*-*************************************
*  Advanced stuff (should actually be hosted elsewhere)
***************************************/
/* custom crash signal handler */
#[no_mangle]
pub unsafe extern "C" fn FIO_addAbortHandler() {
    signal(6i32, Some(ABRThandler));
    signal(8i32, Some(ABRThandler));
    signal(4i32, Some(ABRThandler));
    signal(11i32, Some(ABRThandler));
    signal(7i32, Some(ABRThandler));
}
/*-*********************************************************
*  Termination signal trapping (Print debug stack trace)
***********************************************************/
/* Clang compiler */
/* __has_feature(address_sanitizer) */
/* GCC compiler */
/* automatic detector : backtrace enabled by default on linux+glibc and osx */
/* note : after this point, BACKTRACE_ENABLE is necessarily defined */
/* backtrace, backtrace_symbols */
unsafe extern "C" fn ABRThandler(mut sig: libc::c_int) {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut addrlist: [*mut libc::c_void; 50] = [0 as *mut libc::c_void; 50];
    let mut symbollist: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut addrlen: U32 = 0;
    let mut i: U32 = 0;
    match sig {
        6 => { name = b"SIGABRT\x00" as *const u8 as *const libc::c_char }
        8 => { name = b"SIGFPE\x00" as *const u8 as *const libc::c_char }
        4 => { name = b"SIGILL\x00" as *const u8 as *const libc::c_char }
        2 => { name = b"SIGINT\x00" as *const u8 as *const libc::c_char }
        11 => { name = b"SIGSEGV\x00" as *const u8 as *const libc::c_char }
        _ => { name = b"UNKNOWN\x00" as *const u8 as *const libc::c_char }
    }
    fprintf(stderr,
            b"Caught %s signal, printing stack:\n\x00" as *const u8 as
                *const libc::c_char, name);
    addrlen = backtrace(addrlist.as_mut_ptr(), 50i32) as U32;
    if addrlen == 0i32 as libc::c_uint {
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        return
    }
    symbollist =
        backtrace_symbols(addrlist.as_mut_ptr(), addrlen as libc::c_int);
    i = 2i32 as U32;
    while i < addrlen {
        fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char,
                *symbollist.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free(symbollist as *mut libc::c_void);
    signal(sig, None);
    raise(sig);
}
unsafe extern "C" fn run_static_initializers() {
    segmentSizeT =
        ((32i32 * (1i32 << 10i32)) as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<size_t>() as
                                             libc::c_ulong);
    maskT =
        (::std::mem::size_of::<size_t>() as
             libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong)
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];