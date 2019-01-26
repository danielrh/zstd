#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc)]
#![feature(ptr_wrapping_offset_from)]
mod fileio;
mod datagen;
mod dibio;
mod util;
mod benchzstd;
mod benchfn;
extern crate libc;
extern crate zstd;
extern "C" {
    pub type FIO_prefs_s;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn getchar() -> libc::c_int;
    #[no_mangle]
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    /*
 * Copyright (c) 2016-present, Przemyslaw Skibinski, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
    /*-****************************************
*  Dependencies
******************************************/
    /* PLATFORM_POSIX_VERSION, ZSTD_NANOSLEEP_SUPPORT, ZSTD_SETPRIORITY_SUPPORT */
    /* malloc, realloc, free */
    /* size_t, ptrdiff_t */
    /* fprintf */
    /* stat, utime */
    /* stat, chmod */
    /* chown, stat */
    /* utime */
    /* clock_t, clock, CLOCKS_PER_SEC, nanosleep */
    /*-************************************************************
* Avoid fseek()'s 2GiB barrier with MSVC, macOS, *BSD, MinGW
***************************************************************/
    /* No point defining Large file for 64 bit */
    /*-*************************************************
*  Sleep & priority functions: Windows - Posix - others
***************************************************/
    /* Unix-like operating system */
    /* sleep */
    /* necessarily defined in platform.h */
    /* setpriority */
    /* unknown non-unix operating systen */
    /*-*************************************
*  Constants
***************************************/
    /*-****************************************
*  Compiler specifics
******************************************/
    /* C99 */
    /*-****************************************
*  Console log
******************************************/
    #[no_mangle]
    static mut g_utilDisplayLevel: libc::c_int;
    #[no_mangle]
    fn UTIL_isLink(infilename: *const libc::c_char) -> U32;
    /* opendir, readdir require POSIX.1-2001 */
    /* opendir, readdir */
    /* strerror, memcpy */
    /* #ifdef _WIN32 */
    /*
 * UTIL_createFileList - takes a list of files and directories (params: inputNames, inputNamesNb), scans directories,
 *                       and returns a new list of files (params: return value, allocatedBuffer, allocatedNamesNb).
 * After finishing usage of the list the structures should be freed with UTIL_freeFileList(params: return value, allocatedBuffer)
 * In case of error UTIL_createFileList returns NULL and UTIL_freeFileList should not be called.
 */
    #[no_mangle]
    fn UTIL_createFileList(inputNames: *mut *const libc::c_char,
                           inputNamesNb: libc::c_uint,
                           allocatedBuffer: *mut *mut libc::c_char,
                           allocatedNamesNb: *mut libc::c_uint,
                           followLinks: libc::c_int)
     -> *mut *const libc::c_char;
    #[no_mangle]
    fn UTIL_countPhysicalCores() -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    /* !< maximum compression level available */
    #[no_mangle]
    fn ZSTD_maxCLevel() -> libc::c_int;
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
    #[no_mangle]
    fn FIO_createPreferences() -> *mut FIO_prefs_t;
    #[no_mangle]
    fn FIO_freePreferences(prefs: *mut FIO_prefs_t);
    /*-*************************************
*  Parameters
***************************************/
    #[no_mangle]
    fn FIO_setCompressionType(prefs: *mut FIO_prefs_t,
                              compressionType: FIO_compressionType_t);
    #[no_mangle]
    fn FIO_overwriteMode(prefs: *mut FIO_prefs_t);
    #[no_mangle]
    fn FIO_setAdaptiveMode(prefs: *mut FIO_prefs_t, adapt: libc::c_uint);
    #[no_mangle]
    fn FIO_setAdaptMin(prefs: *mut FIO_prefs_t, minCLevel: libc::c_int);
    #[no_mangle]
    fn FIO_setAdaptMax(prefs: *mut FIO_prefs_t, maxCLevel: libc::c_int);
    #[no_mangle]
    fn FIO_setBlockSize(prefs: *mut FIO_prefs_t, blockSize: libc::c_uint);
    #[no_mangle]
    fn FIO_setChecksumFlag(prefs: *mut FIO_prefs_t,
                           checksumFlag: libc::c_uint);
    #[no_mangle]
    fn FIO_setDictIDFlag(prefs: *mut FIO_prefs_t, dictIDFlag: libc::c_uint);
    #[no_mangle]
    fn FIO_setLdmBucketSizeLog(prefs: *mut FIO_prefs_t,
                               ldmBucketSizeLog: libc::c_uint);
    #[no_mangle]
    fn FIO_setLdmFlag(prefs: *mut FIO_prefs_t, ldmFlag: libc::c_uint);
    #[no_mangle]
    fn FIO_setLdmHashRateLog(prefs: *mut FIO_prefs_t,
                             ldmHashRateLog: libc::c_uint);
    #[no_mangle]
    fn FIO_setLdmHashLog(prefs: *mut FIO_prefs_t, ldmHashLog: libc::c_uint);
    #[no_mangle]
    fn FIO_setLdmMinMatch(prefs: *mut FIO_prefs_t, ldmMinMatch: libc::c_uint);
    #[no_mangle]
    fn FIO_setMemLimit(prefs: *mut FIO_prefs_t, memLimit: libc::c_uint);
    #[no_mangle]
    fn FIO_setNbWorkers(prefs: *mut FIO_prefs_t, nbWorkers: libc::c_uint);
    #[no_mangle]
    fn FIO_setOverlapLog(prefs: *mut FIO_prefs_t, overlapLog: libc::c_uint);
    #[no_mangle]
    fn FIO_setRemoveSrcFile(prefs: *mut FIO_prefs_t, flag: libc::c_uint);
    #[no_mangle]
    fn FIO_setSparseWrite(prefs: *mut FIO_prefs_t, sparse: libc::c_uint);
    #[no_mangle]
    fn FIO_setRsyncable(prefs: *mut FIO_prefs_t, rsyncable: libc::c_uint);
    #[no_mangle]
    fn FIO_setNoProgress(noProgress: libc::c_uint);
    #[no_mangle]
    fn FIO_setNotificationLevel(level: libc::c_uint);
    /*-*************************************
*  Single File functions
***************************************/
/** FIO_compressFilename() :
    @return : 0 == ok;  1 == pb with src file. */
    #[no_mangle]
    fn FIO_compressFilename(prefs: *mut FIO_prefs_t,
                            outfilename: *const libc::c_char,
                            infilename: *const libc::c_char,
                            dictFileName: *const libc::c_char,
                            compressionLevel: libc::c_int,
                            comprParams: ZSTD_compressionParameters)
     -> libc::c_int;
    /* * FIO_decompressFilename() :
    @return : 0 == ok;  1 == pb with src file. */
    #[no_mangle]
    fn FIO_decompressFilename(prefs: *mut FIO_prefs_t,
                              outfilename: *const libc::c_char,
                              infilename: *const libc::c_char,
                              dictFileName: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn FIO_listMultipleFiles(numFiles: libc::c_uint,
                             filenameTable: *mut *const libc::c_char,
                             displayLevel: libc::c_int) -> libc::c_int;
    /*-*************************************
*  Multiple File functions
***************************************/
/** FIO_compressMultipleFilenames() :
    @return : nb of missing files */
    #[no_mangle]
    fn FIO_compressMultipleFilenames(prefs: *mut FIO_prefs_t,
                                     srcNamesTable: *mut *const libc::c_char,
                                     nbFiles: libc::c_uint,
                                     outFileName: *const libc::c_char,
                                     suffix: *const libc::c_char,
                                     dictFileName: *const libc::c_char,
                                     compressionLevel: libc::c_int,
                                     comprParams: ZSTD_compressionParameters)
     -> libc::c_int;
    /* * FIO_decompressMultipleFilenames() :
    @return : nb of missing or skipped files */
    #[no_mangle]
    fn FIO_decompressMultipleFilenames(prefs: *mut FIO_prefs_t,
                                       srcNamesTable:
                                           *mut *const libc::c_char,
                                       nbFiles: libc::c_uint,
                                       outFileName: *const libc::c_char,
                                       dictFileName: *const libc::c_char)
     -> libc::c_int;
    /*-*************************************
*  Advanced stuff (should actually be hosted elsewhere)
***************************************/
    /* custom crash signal handler */
    #[no_mangle]
    fn FIO_addAbortHandler();
    /* returns default parameters used by nonAdvanced functions */
    #[no_mangle]
    fn BMK_initAdvancedParams() -> BMK_advancedParams_t;
    /* ! BMK_benchFilesAdvanced():
 *  Same as BMK_benchFiles(),
 *  with more controls, provided through advancedParams_t structure */
    #[no_mangle]
    fn BMK_benchFilesAdvanced(fileNamesTable: *const *const libc::c_char,
                              nbFiles: libc::c_uint,
                              dictFileName: *const libc::c_char,
                              cLevel: libc::c_int,
                              compressionParams:
                                  *const ZSTD_compressionParameters,
                              displayLevel: libc::c_int,
                              adv: *const BMK_advancedParams_t)
     -> BMK_benchOutcome_t;
    /* ! BMK_syntheticTest() -- called from zstdcli */
/*  Generates a sample with datagen, using compressibility argument */
/*  cLevel - compression level to benchmark, errors if invalid
 *  compressibility - determines compressibility of sample
 *  compressionParams - basic compression Parameters
 *  displayLevel - see benchFiles
 *  adv - see advanced_Params_t
 * @return:
 *      a variant, which expresses either an error, or a valid result.
 *      Use BMK_isSuccessful_benchOutcome() to check if function was successful.
 *      If yes, extract the valid result with BMK_extract_benchResult(),
 *      it will contain :
 *          .cSpeed: compression speed in bytes per second,
 *          .dSpeed: decompression speed in bytes per second,
 *          .cSize : compressed size, in bytes
 *          .cMem  : memory budget required for the compression context
 */
    #[no_mangle]
    fn BMK_syntheticTest(cLevel: libc::c_int, compressibility: libc::c_double,
                         compressionParams: *const ZSTD_compressionParameters,
                         displayLevel: libc::c_int,
                         adv: *const BMK_advancedParams_t)
     -> BMK_benchOutcome_t;
    /*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
    /* This library is designed for a single-threaded console application.
*  It exit() and printf() into stderr when it encounters an error condition. */
    /*-*************************************
*  Dependencies
***************************************/
    /*-*************************************
*  Public functions
***************************************/
/* DiB_trainFromFiles() :
    Train a dictionary from a set of files provided by `fileNamesTable`.
    Resulting dictionary is written into file `dictFileName`.
    `parameters` is optional and can be provided with values set to 0, meaning "default".
    @return : 0 == ok. Any other : error.
*/
    #[no_mangle]
    fn DiB_trainFromFiles(dictFileName: *const libc::c_char,
                          maxDictSize: libc::c_uint,
                          fileNamesTable: *mut *const libc::c_char,
                          nbFiles: libc::c_uint, chunkSize: size_t,
                          params: *mut ZDICT_legacy_params_t,
                          coverParams: *mut ZDICT_cover_params_t,
                          fastCoverParams: *mut ZDICT_fastCover_params_t,
                          optimize: libc::c_int) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uint32_t = __uint32_t;
pub type U32 = uint32_t;
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
pub type FIO_prefs_t = FIO_prefs_s;
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* benchzstd :
  * benchmark Zstandard compression / decompression
  * over a set of files or buffers
  * and display progress result and final summary
  */
/* ===  Dependencies  === */
/* size_t */
/* ZSTD_compressionParameters */
/* ===  Constants  === */
/* ===  Benchmark functions  === */
/* Creates a variant `typeName`, able to express "error or valid result".
 * Functions with return type `typeName`
 * must first check if result is valid, using BMK_isSuccessful_*(),
 * and only then can extract `baseType`.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BMK_benchResult_t {
    pub cSize: size_t,
    pub cSpeed: libc::c_ulonglong,
    pub dSpeed: libc::c_ulonglong,
    pub cMem: size_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BMK_benchOutcome_t {
    pub internal_never_use_directly: BMK_benchResult_t,
    pub tag: libc::c_int,
}
pub type BMK_mode_t = libc::c_uint;
pub const BMK_compressOnly: BMK_mode_t = 2;
pub const BMK_decodeOnly: BMK_mode_t = 1;
pub const BMK_both: BMK_mode_t = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BMK_advancedParams_t {
    pub mode: BMK_mode_t,
    pub nbSeconds: libc::c_uint,
    pub blockSize: size_t,
    pub nbWorkers: libc::c_uint,
    pub realTime: libc::c_uint,
    pub additionalParam: libc::c_int,
    pub ldmFlag: libc::c_uint,
    pub ldmMinMatch: libc::c_uint,
    pub ldmHashLog: libc::c_uint,
    pub ldmBucketSizeLog: libc::c_uint,
    pub ldmHashRateLog: libc::c_uint,
}
/* ====================================================================================
 * The definitions in this section are considered experimental.
 * They should never be used with a dynamic library, as they may change in the future.
 * They are provided for advanced usages.
 * Use them only in association with static linking.
 * ==================================================================================== */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZDICT_params_t {
    pub compressionLevel: libc::c_int,
    pub notificationLevel: libc::c_uint,
    pub dictID: libc::c_uint,
}
/* ! ZDICT_cover_params_t:
 *  k and d are the only required parameters.
 *  For others, value 0 means default.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZDICT_cover_params_t {
    pub k: libc::c_uint,
    pub d: libc::c_uint,
    pub steps: libc::c_uint,
    pub nbThreads: libc::c_uint,
    pub splitPoint: libc::c_double,
    pub zParams: ZDICT_params_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZDICT_fastCover_params_t {
    pub k: libc::c_uint,
    pub d: libc::c_uint,
    pub f: libc::c_uint,
    pub steps: libc::c_uint,
    pub nbThreads: libc::c_uint,
    pub splitPoint: libc::c_double,
    pub accel: libc::c_uint,
    pub zParams: ZDICT_params_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ZDICT_legacy_params_t {
    pub selectivityLevel: libc::c_uint,
    pub zParams: ZDICT_params_t,
}
pub type dictType = libc::c_uint;
pub const legacy: dictType = 2;
pub const fastCover: dictType = 1;
pub const cover: dictType = 0;
pub type zstd_operation_mode = libc::c_uint;
pub const zom_list: zstd_operation_mode = 5;
pub const zom_train: zstd_operation_mode = 4;
pub const zom_bench: zstd_operation_mode = 3;
pub const zom_test: zstd_operation_mode = 2;
pub const zom_decompress: zstd_operation_mode = 1;
pub const zom_compress: zstd_operation_mode = 0;
unsafe extern "C" fn UTIL_freeFileList(mut filenameTable:
                                           *mut *const libc::c_char,
                                       mut allocatedBuffer:
                                           *mut libc::c_char) {
    if !allocatedBuffer.is_null() {
        free(allocatedBuffer as *mut libc::c_void);
    }
    if !filenameTable.is_null() { free(filenameTable as *mut libc::c_void); };
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
/*-************************************
*  Tuning parameters
**************************************/
/* without using --ultra */
/*-************************************
*  Dependencies
**************************************/
/* fprintf(), stdin, stdout, stderr */
/* getenv */
/* strcmp, strlen */
/* errno */
/* ZSTD_minCLevel */
/*-************************************
*  Constants
**************************************/
static mut g_defaultDictName: *const libc::c_char =
    b"dictionary\x00" as *const u8 as *const libc::c_char;
static mut g_defaultMaxDictSize: libc::c_uint =
    (110i32 * (1i32 << 10i32)) as libc::c_uint;
static mut g_defaultDictCLevel: libc::c_int = 3i32;
static mut g_defaultSelectivityLevel: libc::c_uint = 9i32 as libc::c_uint;
static mut g_defaultMaxWindowLog: libc::c_uint = 27i32 as libc::c_uint;
/* Default for parameters where 0 is valid */
static mut g_overlapLog: U32 = 9999i32 as U32;
static mut g_ldmHashLog: U32 = 0i32 as U32;
static mut g_ldmMinMatch: U32 = 0i32 as U32;
static mut g_ldmHashRateLog: U32 = 9999i32 as U32;
static mut g_ldmBucketSizeLog: U32 = 9999i32 as U32;
/*-************************************
*  Display Macros
**************************************/
/* 0 : no display,  1: errors,  2 : + result + interaction + warnings,  3 : + progression,  4 : + information */
static mut g_displayLevel: libc::c_int = 2i32;
static mut g_displayOut: *mut FILE = 0 as *const FILE as *mut FILE;
/*-************************************
*  Command Line
**************************************/
unsafe extern "C" fn usage(mut programName: *const libc::c_char)
 -> libc::c_int {
    fprintf(g_displayOut,
            b"Usage : \n\x00" as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"      %s [args] [FILE(s)] [-o file] \n\x00" as *const u8 as
                *const libc::c_char, programName);
    fprintf(g_displayOut, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"FILE    : a filename \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(g_displayOut,
            b"          with no FILE, or when FILE is - , read standard input\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"Arguments : \n\x00" as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b" -#     : # compression level (1-%d, default: %d) \n\x00" as
                *const u8 as *const libc::c_char, 19i32, 3i32);
    fprintf(g_displayOut,
            b" -d     : decompression \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(g_displayOut,
            b" -D file: use `file` as Dictionary \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(g_displayOut,
            b" -o file: result stored into `file` (only if 1 input file) \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b" -f     : overwrite output without prompting and (de)compress links \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"--rm    : remove source file(s) after successful de/compression \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b" -k     : preserve source file(s) (default) \n\x00" as *const u8
                as *const libc::c_char);
    fprintf(g_displayOut,
            b" -h/-H  : display help/long help and exit \n\x00" as *const u8
                as *const libc::c_char);
    return 0i32;
}
unsafe extern "C" fn usage_advanced(mut programName: *const libc::c_char)
 -> libc::c_int {
    fprintf(g_displayOut,
            b"*** %s %i-bits %s, by %s ***\n\x00" as *const u8 as
                *const libc::c_char,
            b"zstd command line interface\x00" as *const u8 as
                *const libc::c_char,
            (::std::mem::size_of::<size_t>() as
                 libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) as
                libc::c_int,
            b"v1.3.8\x00" as *const u8 as *const libc::c_char,
            b"Yann Collet\x00" as *const u8 as *const libc::c_char);
    usage(programName);
    fprintf(g_displayOut, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"Advanced arguments : \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(g_displayOut,
            b" -V     : display Version number and exit \n\x00" as *const u8
                as *const libc::c_char);
    fprintf(g_displayOut,
            b" -v     : verbose mode; specify multiple times to increase verbosity\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b" -q     : suppress warnings; specify twice to suppress errors too\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b" -c     : force write to standard output, even if it is the console\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b" -l     : print information about zstd compressed files \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"--ultra : enable levels beyond %i, up to %i (requires more memory)\n\x00"
                as *const u8 as *const libc::c_char, 19i32, ZSTD_maxCLevel());
    fprintf(g_displayOut,
            b"--long[=#]: enable long distance matching with given window log (default: %u)\n\x00"
                as *const u8 as *const libc::c_char, g_defaultMaxWindowLog);
    fprintf(g_displayOut,
            b"--fast[=#]: switch to ultra fast compression level (default: %u)\n\x00"
                as *const u8 as *const libc::c_char, 1i32);
    fprintf(g_displayOut,
            b"--adapt : dynamically adapt compression level to I/O conditions \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b" -T#    : spawns # compression threads (default: 1, 0==# cores) \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b" -B#    : select size of each job (default: 0==automatic) \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b" --rsyncable : compress using a rsync-friendly method (-B sets block size) \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"--no-dictID : don\'t write dictID into header (dictionary compression)\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"--[no-]check : integrity check (default: enabled) \n\x00" as
                *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b" -r     : operate recursively on directories \n\x00" as
                *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"--format=zstd : compress files to the .zst format (default) \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"--test  : test compressed file integrity \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(g_displayOut,
            b"--[no-]sparse : sparse mode (default: enabled on file, disabled on stdout)\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b" -M#    : Set a memory usage limit for decompression \n\x00" as
                *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"--no-progress : do not display the progress bar \n\x00" as
                *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"--      : All arguments after \"--\" are treated as files \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"Dictionary builder : \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(g_displayOut,
            b"--train ## : create a dictionary from a training set of files \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"--train-cover[=k=#,d=#,steps=#,split=#] : use the cover algorithm with optional args\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"--train-fastcover[=k=#,d=#,f=#,steps=#,split=#,accel=#] : use the fast cover algorithm with optional args\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"--train-legacy[=s=#] : use the legacy algorithm with selectivity (default: %u)\n\x00"
                as *const u8 as *const libc::c_char,
            g_defaultSelectivityLevel);
    fprintf(g_displayOut,
            b" -o file : `file` is dictionary name (default: %s) \n\x00" as
                *const u8 as *const libc::c_char, g_defaultDictName);
    fprintf(g_displayOut,
            b"--maxdict=# : limit dictionary to specified size (default: %u) \n\x00"
                as *const u8 as *const libc::c_char, g_defaultMaxDictSize);
    fprintf(g_displayOut,
            b"--dictID=# : force dictionary ID to specified value (default: random)\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"Benchmark arguments : \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(g_displayOut,
            b" -b#    : benchmark file(s), using # compression level (default: %d) \n\x00"
                as *const u8 as *const libc::c_char, 3i32);
    fprintf(g_displayOut,
            b" -e#    : test all compression levels from -bX to # (default: 1)\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b" -i#    : minimum evaluation time in seconds (default: 3s) \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b" -B#    : cut file into independent blocks of size # (default: no block)\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(g_displayOut,
            b"--priority=rt : set process priority to real-time \n\x00" as
                *const u8 as *const libc::c_char);
    return 0i32;
}
unsafe extern "C" fn badusage(mut programName: *const libc::c_char)
 -> libc::c_int {
    if g_displayLevel >= 1i32 {
        fprintf(g_displayOut,
                b"Incorrect parameters\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    if g_displayLevel >= 2i32 { usage(programName); }
    return 1i32;
}
unsafe extern "C" fn waitEnter() {
    let mut unused: libc::c_int = 0;
    fprintf(g_displayOut,
            b"Press enter to continue...\n\x00" as *const u8 as
                *const libc::c_char);
    unused = getchar();
}
unsafe extern "C" fn lastNameFromPath(mut path: *const libc::c_char)
 -> *const libc::c_char {
    let mut name: *const libc::c_char = path;
    if !strrchr(name, '/' as i32).is_null() {
        name = strrchr(name, '/' as i32).offset(1isize)
    }
    if !strrchr(name, '\\' as i32).is_null() {
        name = strrchr(name, '\\' as i32).offset(1isize)
    }
    return name;
}
/* ! exeNameMatch() :
    @return : a non-zero value if exeName matches test, excluding the extension
   */
unsafe extern "C" fn exeNameMatch(mut exeName: *const libc::c_char,
                                  mut test: *const libc::c_char)
 -> libc::c_int {
    return (0 == strncmp(exeName, test, strlen(test)) &&
                (*exeName.offset(strlen(test) as isize) as libc::c_int ==
                     '\u{0}' as i32 ||
                     *exeName.offset(strlen(test) as isize) as libc::c_int ==
                         '.' as i32)) as libc::c_int;
}
unsafe extern "C" fn errorOut(mut msg: *const libc::c_char) {
    fprintf(g_displayOut, b"%s \n\x00" as *const u8 as *const libc::c_char,
            msg);
    exit(1i32);
}
// Initialized in run_static_initializers
static mut max: libc::c_uint = 0;
/* ! readU32FromCharChecked() :
 * @return 0 if success, and store the result in *value.
 *  allows and interprets K, KB, KiB, M, MB and MiB suffix.
 *  Will also modify `*stringPtr`, advancing it to position where it stopped reading.
 * @return 1 if an overflow error occurs */
unsafe extern "C" fn readU32FromCharChecked(mut stringPtr:
                                                *mut *const libc::c_char,
                                            mut value: *mut libc::c_uint)
 -> libc::c_int {
    let mut result: libc::c_uint = 0i32 as libc::c_uint;
    while **stringPtr as libc::c_int >= '0' as i32 &&
              **stringPtr as libc::c_int <= '9' as i32 {
        if result > max { return 1i32 }
        result = result.wrapping_mul(10i32 as libc::c_uint);
        result =
            result.wrapping_add((**stringPtr as libc::c_int - '0' as i32) as
                                    libc::c_uint);
        *stringPtr = (*stringPtr).offset(1isize)
    }
    if **stringPtr as libc::c_int == 'K' as i32 ||
           **stringPtr as libc::c_int == 'M' as i32 {
        let maxK: libc::c_uint = -1i32 as libc::c_uint >> 10i32;
        if result > maxK { return 1i32 }
        result <<= 10i32;
        if **stringPtr as libc::c_int == 'M' as i32 {
            if result > maxK { return 1i32 }
            result <<= 10i32
        }
        *stringPtr = (*stringPtr).offset(1isize);
        if **stringPtr as libc::c_int == 'i' as i32 {
            *stringPtr = (*stringPtr).offset(1isize)
        }
        if **stringPtr as libc::c_int == 'B' as i32 {
            *stringPtr = (*stringPtr).offset(1isize)
        }
    }
    *value = result;
    return 0i32;
}
/* ! readU32FromChar() :
 * @return : unsigned integer value read from input in `char` format.
 *  allows and interprets K, KB, KiB, M, MB and MiB suffix.
 *  Will also modify `*stringPtr`, advancing it to position where it stopped reading.
 *  Note : function will exit() program if digit sequence overflows */
unsafe extern "C" fn readU32FromChar(mut stringPtr: *mut *const libc::c_char)
 -> libc::c_uint {
    static mut errorMsg: [libc::c_char; 31] =
        [101, 114, 114, 111, 114, 58, 32, 110, 117, 109, 101, 114, 105, 99,
         32, 118, 97, 108, 117, 101, 32, 116, 111, 111, 32, 108, 97, 114, 103,
         101, 0];
    let mut result: libc::c_uint = 0;
    if 0 != readU32FromCharChecked(stringPtr, &mut result) {
        errorOut(errorMsg.as_ptr());
    }
    return result;
}
/* * longCommandWArg() :
 *  check if *stringPtr is the same as longCommand.
 *  If yes, @return 1 and advances *stringPtr to the position which immediately follows longCommand.
 * @return 0 and doesn't modify *stringPtr otherwise.
 */
unsafe extern "C" fn longCommandWArg(mut stringPtr: *mut *const libc::c_char,
                                     mut longCommand: *const libc::c_char)
 -> libc::c_uint {
    let comSize: size_t = strlen(longCommand);
    let result: libc::c_int =
        (0 == strncmp(*stringPtr, longCommand, comSize)) as libc::c_int;
    if 0 != result { *stringPtr = (*stringPtr).offset(comSize as isize) }
    return result as libc::c_uint;
}
/* *
 * parseCoverParameters() :
 * reads cover parameters from *stringPtr (e.g. "--train-cover=k=48,d=8,steps=32") into *params
 * @return 1 means that cover parameters were correct
 * @return 0 in case of malformed parameters
 */
unsafe extern "C" fn parseCoverParameters(mut stringPtr: *const libc::c_char,
                                          mut params:
                                              *mut ZDICT_cover_params_t)
 -> libc::c_uint {
    memset(params as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZDICT_cover_params_t>() as libc::c_ulong);
    loop  {
        if 0 !=
               longCommandWArg(&mut stringPtr,
                               b"k=\x00" as *const u8 as *const libc::c_char)
           {
            (*params).k = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"d=\x00" as *const u8 as
                                          *const libc::c_char) {
            (*params).d = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"steps=\x00" as *const u8 as
                                          *const libc::c_char) {
            (*params).steps = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"split=\x00" as *const u8 as
                                          *const libc::c_char) {
            let mut splitPercentage: libc::c_uint =
                readU32FromChar(&mut stringPtr);
            (*params).splitPoint =
                splitPercentage as libc::c_double / 100.0f64;
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else { return 0i32 as libc::c_uint }
    }
    if *stringPtr.offset(0isize) as libc::c_int != 0i32 {
        return 0i32 as libc::c_uint
    }
    if g_displayLevel >= 4i32 {
        fprintf(g_displayOut,
                b"cover: k=%u\nd=%u\nsteps=%u\nsplit=%u\n\x00" as *const u8 as
                    *const libc::c_char, (*params).k, (*params).d,
                (*params).steps,
                ((*params).splitPoint * 100i32 as libc::c_double) as
                    libc::c_uint);
    }
    return 1i32 as libc::c_uint;
}
/* *
 * parseFastCoverParameters() :
 * reads fastcover parameters from *stringPtr (e.g. "--train-fastcover=k=48,d=8,f=20,steps=32,accel=2") into *params
 * @return 1 means that fastcover parameters were correct
 * @return 0 in case of malformed parameters
 */
unsafe extern "C" fn parseFastCoverParameters(mut stringPtr:
                                                  *const libc::c_char,
                                              mut params:
                                                  *mut ZDICT_fastCover_params_t)
 -> libc::c_uint {
    memset(params as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZDICT_fastCover_params_t>() as
               libc::c_ulong);
    loop  {
        if 0 !=
               longCommandWArg(&mut stringPtr,
                               b"k=\x00" as *const u8 as *const libc::c_char)
           {
            (*params).k = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"d=\x00" as *const u8 as
                                          *const libc::c_char) {
            (*params).d = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"f=\x00" as *const u8 as
                                          *const libc::c_char) {
            (*params).f = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"steps=\x00" as *const u8 as
                                          *const libc::c_char) {
            (*params).steps = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"accel=\x00" as *const u8 as
                                          *const libc::c_char) {
            (*params).accel = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"split=\x00" as *const u8 as
                                          *const libc::c_char) {
            let mut splitPercentage: libc::c_uint =
                readU32FromChar(&mut stringPtr);
            (*params).splitPoint =
                splitPercentage as libc::c_double / 100.0f64;
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else { return 0i32 as libc::c_uint }
    }
    if *stringPtr.offset(0isize) as libc::c_int != 0i32 {
        return 0i32 as libc::c_uint
    }
    if g_displayLevel >= 4i32 {
        fprintf(g_displayOut,
                b"cover: k=%u\nd=%u\nf=%u\nsteps=%u\nsplit=%u\naccel=%u\n\x00"
                    as *const u8 as *const libc::c_char, (*params).k,
                (*params).d, (*params).f, (*params).steps,
                ((*params).splitPoint * 100i32 as libc::c_double) as
                    libc::c_uint, (*params).accel);
    }
    return 1i32 as libc::c_uint;
}
/* *
 * parseLegacyParameters() :
 * reads legacy dictioanry builter parameters from *stringPtr (e.g. "--train-legacy=selectivity=8") into *selectivity
 * @return 1 means that legacy dictionary builder parameters were correct
 * @return 0 in case of malformed parameters
 */
unsafe extern "C" fn parseLegacyParameters(mut stringPtr: *const libc::c_char,
                                           mut selectivity: *mut libc::c_uint)
 -> libc::c_uint {
    if 0 ==
           longCommandWArg(&mut stringPtr,
                           b"s=\x00" as *const u8 as *const libc::c_char) &&
           0 ==
               longCommandWArg(&mut stringPtr,
                               b"selectivity=\x00" as *const u8 as
                                   *const libc::c_char) {
        return 0i32 as libc::c_uint
    }
    *selectivity = readU32FromChar(&mut stringPtr);
    if *stringPtr.offset(0isize) as libc::c_int != 0i32 {
        return 0i32 as libc::c_uint
    }
    if g_displayLevel >= 4i32 {
        fprintf(g_displayOut,
                b"legacy: selectivity=%u\n\x00" as *const u8 as
                    *const libc::c_char, *selectivity);
    }
    return 1i32 as libc::c_uint;
}
unsafe extern "C" fn defaultCoverParams() -> ZDICT_cover_params_t {
    let mut params: ZDICT_cover_params_t =
        ZDICT_cover_params_t{k: 0,
                             d: 0,
                             steps: 0,
                             nbThreads: 0,
                             splitPoint: 0.,
                             zParams:
                                 ZDICT_params_t{compressionLevel: 0,
                                                notificationLevel: 0,
                                                dictID: 0,},};
    memset(&mut params as *mut ZDICT_cover_params_t as *mut libc::c_void,
           0i32,
           ::std::mem::size_of::<ZDICT_cover_params_t>() as libc::c_ulong);
    params.d = 8i32 as libc::c_uint;
    params.steps = 4i32 as libc::c_uint;
    params.splitPoint = 1.0f64;
    return params;
}
unsafe extern "C" fn defaultFastCoverParams() -> ZDICT_fastCover_params_t {
    let mut params: ZDICT_fastCover_params_t =
        ZDICT_fastCover_params_t{k: 0,
                                 d: 0,
                                 f: 0,
                                 steps: 0,
                                 nbThreads: 0,
                                 splitPoint: 0.,
                                 accel: 0,
                                 zParams:
                                     ZDICT_params_t{compressionLevel: 0,
                                                    notificationLevel: 0,
                                                    dictID: 0,},};
    memset(&mut params as *mut ZDICT_fastCover_params_t as *mut libc::c_void,
           0i32,
           ::std::mem::size_of::<ZDICT_fastCover_params_t>() as
               libc::c_ulong);
    params.d = 8i32 as libc::c_uint;
    params.f = 20i32 as libc::c_uint;
    params.steps = 4i32 as libc::c_uint;
    params.splitPoint = 0.75f64;
    params.accel = 1i32 as libc::c_uint;
    return params;
}
/* * parseAdaptParameters() :
 *  reads adapt parameters from *stringPtr (e.g. "--zstd=min=1,max=19) and store them into adaptMinPtr and adaptMaxPtr.
 *  Both adaptMinPtr and adaptMaxPtr must be already allocated and correctly initialized.
 *  There is no guarantee that any of these values will be updated.
 *  @return 1 means that parsing was successful,
 *  @return 0 in case of malformed parameters
 */
unsafe extern "C" fn parseAdaptParameters(mut stringPtr: *const libc::c_char,
                                          mut adaptMinPtr: *mut libc::c_int,
                                          mut adaptMaxPtr: *mut libc::c_int)
 -> libc::c_uint {
    loop  {
        if 0 !=
               longCommandWArg(&mut stringPtr,
                               b"min=\x00" as *const u8 as
                                   *const libc::c_char) {
            *adaptMinPtr = readU32FromChar(&mut stringPtr) as libc::c_int;
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"max=\x00" as *const u8 as
                                          *const libc::c_char) {
            *adaptMaxPtr = readU32FromChar(&mut stringPtr) as libc::c_int;
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else {
            if g_displayLevel >= 4i32 {
                fprintf(g_displayOut,
                        b"invalid compression parameter \n\x00" as *const u8
                            as *const libc::c_char);
            }
            return 0i32 as libc::c_uint
        }
    }
    if *stringPtr.offset(0isize) as libc::c_int != 0i32 {
        return 0i32 as libc::c_uint
    }
    if *adaptMinPtr > *adaptMaxPtr {
        if g_displayLevel >= 4i32 {
            fprintf(g_displayOut,
                    b"incoherent adaptation limits \n\x00" as *const u8 as
                        *const libc::c_char);
        }
        return 0i32 as libc::c_uint
    }
    return 1i32 as libc::c_uint;
}
/* * parseCompressionParameters() :
 *  reads compression parameters from *stringPtr (e.g. "--zstd=wlog=23,clog=23,hlog=22,slog=6,mml=3,tlen=48,strat=6") into *params
 *  @return 1 means that compression parameters were correct
 *  @return 0 in case of malformed parameters
 */
unsafe extern "C" fn parseCompressionParameters(mut stringPtr:
                                                    *const libc::c_char,
                                                mut params:
                                                    *mut ZSTD_compressionParameters)
 -> libc::c_uint {
    loop  {
        if 0 !=
               longCommandWArg(&mut stringPtr,
                               b"windowLog=\x00" as *const u8 as
                                   *const libc::c_char) ||
               0 !=
                   longCommandWArg(&mut stringPtr,
                                   b"wlog=\x00" as *const u8 as
                                       *const libc::c_char) {
            (*params).windowLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"chainLog=\x00" as *const u8 as
                                          *const libc::c_char) ||
                      0 !=
                          longCommandWArg(&mut stringPtr,
                                          b"clog=\x00" as *const u8 as
                                              *const libc::c_char) {
            (*params).chainLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"hashLog=\x00" as *const u8 as
                                          *const libc::c_char) ||
                      0 !=
                          longCommandWArg(&mut stringPtr,
                                          b"hlog=\x00" as *const u8 as
                                              *const libc::c_char) {
            (*params).hashLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"searchLog=\x00" as *const u8 as
                                          *const libc::c_char) ||
                      0 !=
                          longCommandWArg(&mut stringPtr,
                                          b"slog=\x00" as *const u8 as
                                              *const libc::c_char) {
            (*params).searchLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"minMatch=\x00" as *const u8 as
                                          *const libc::c_char) ||
                      0 !=
                          longCommandWArg(&mut stringPtr,
                                          b"mml=\x00" as *const u8 as
                                              *const libc::c_char) {
            (*params).minMatch = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"targetLength=\x00" as *const u8 as
                                          *const libc::c_char) ||
                      0 !=
                          longCommandWArg(&mut stringPtr,
                                          b"tlen=\x00" as *const u8 as
                                              *const libc::c_char) {
            (*params).targetLength = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"strategy=\x00" as *const u8 as
                                          *const libc::c_char) ||
                      0 !=
                          longCommandWArg(&mut stringPtr,
                                          b"strat=\x00" as *const u8 as
                                              *const libc::c_char) {
            (*params).strategy =
                readU32FromChar(&mut stringPtr) as ZSTD_strategy;
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"overlapLog=\x00" as *const u8 as
                                          *const libc::c_char) ||
                      0 !=
                          longCommandWArg(&mut stringPtr,
                                          b"ovlog=\x00" as *const u8 as
                                              *const libc::c_char) {
            g_overlapLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"ldmHashLog=\x00" as *const u8 as
                                          *const libc::c_char) ||
                      0 !=
                          longCommandWArg(&mut stringPtr,
                                          b"lhlog=\x00" as *const u8 as
                                              *const libc::c_char) {
            g_ldmHashLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"ldmMinMatch=\x00" as *const u8 as
                                          *const libc::c_char) ||
                      0 !=
                          longCommandWArg(&mut stringPtr,
                                          b"lmml=\x00" as *const u8 as
                                              *const libc::c_char) {
            g_ldmMinMatch = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"ldmBucketSizeLog=\x00" as *const u8 as
                                          *const libc::c_char) ||
                      0 !=
                          longCommandWArg(&mut stringPtr,
                                          b"lblog=\x00" as *const u8 as
                                              *const libc::c_char) {
            g_ldmBucketSizeLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else if 0 !=
                      longCommandWArg(&mut stringPtr,
                                      b"ldmHashRateLog=\x00" as *const u8 as
                                          *const libc::c_char) ||
                      0 !=
                          longCommandWArg(&mut stringPtr,
                                          b"lhrlog=\x00" as *const u8 as
                                              *const libc::c_char) {
            g_ldmHashRateLog = readU32FromChar(&mut stringPtr);
            if !(*stringPtr.offset(0isize) as libc::c_int == ',' as i32) {
                break ;
            }
            stringPtr = stringPtr.offset(1isize)
        } else {
            if g_displayLevel >= 4i32 {
                fprintf(g_displayOut,
                        b"invalid compression parameter \n\x00" as *const u8
                            as *const libc::c_char);
            }
            return 0i32 as libc::c_uint
        }
    }
    if g_displayLevel >= 4i32 {
        fprintf(g_displayOut,
                b"windowLog=%d, chainLog=%d, hashLog=%d, searchLog=%d \n\x00"
                    as *const u8 as *const libc::c_char, (*params).windowLog,
                (*params).chainLog, (*params).hashLog, (*params).searchLog);
    }
    if g_displayLevel >= 4i32 {
        fprintf(g_displayOut,
                b"minMatch=%d, targetLength=%d, strategy=%d \n\x00" as
                    *const u8 as *const libc::c_char, (*params).minMatch,
                (*params).targetLength, (*params).strategy as libc::c_uint);
    }
    if *stringPtr.offset(0isize) as libc::c_int != 0i32 {
        return 0i32 as libc::c_uint
    }
    return 1i32 as libc::c_uint;
}
unsafe extern "C" fn printVersion() {
    fprintf(g_displayOut,
            b"*** %s %i-bits %s, by %s ***\n\x00" as *const u8 as
                *const libc::c_char,
            b"zstd command line interface\x00" as *const u8 as
                *const libc::c_char,
            (::std::mem::size_of::<size_t>() as
                 libc::c_ulong).wrapping_mul(8i32 as libc::c_ulong) as
                libc::c_int,
            b"v1.3.8\x00" as *const u8 as *const libc::c_char,
            b"Yann Collet\x00" as *const u8 as *const libc::c_char);
    if g_displayLevel >= 3i32 {
        fprintf(g_displayOut,
                b"*** supports: zstd\x00" as *const u8 as
                    *const libc::c_char);
    }
    if g_displayLevel >= 3i32 {
        fprintf(g_displayOut, b"\n\x00" as *const u8 as *const libc::c_char);
    }
    if g_displayLevel >= 4i32 {
        fprintf(g_displayOut,
                b"_POSIX_C_SOURCE defined: %ldL\n\x00" as *const u8 as
                    *const libc::c_char, 200112i64);
    }
    if g_displayLevel >= 4i32 {
        fprintf(g_displayOut,
                b"_POSIX_VERSION defined: %ldL \n\x00" as *const u8 as
                    *const libc::c_char, 200112i64);
    }
    if g_displayLevel >= 4i32 {
        fprintf(g_displayOut,
                b"PLATFORM_POSIX_VERSION defined: %ldL\n\x00" as *const u8 as
                    *const libc::c_char, 200112i64);
    };
}
/* Environment variables for parameter setting */
/* functions that pick up environment variables */
unsafe extern "C" fn init_cLevel() -> libc::c_int {
    let env: *const libc::c_char =
        getenv(b"ZSTD_CLEVEL\x00" as *const u8 as *const libc::c_char);
    if !env.is_null() {
        let mut ptr: *const libc::c_char = env;
        let mut sign: libc::c_int = 1i32;
        if *ptr as libc::c_int == '-' as i32 {
            sign = -1i32;
            ptr = ptr.offset(1isize)
        } else if *ptr as libc::c_int == '+' as i32 {
            ptr = ptr.offset(1isize)
        }
        if *ptr as libc::c_int >= '0' as i32 &&
               *ptr as libc::c_int <= '9' as i32 {
            let mut absLevel: libc::c_uint = 0;
            if 0 != readU32FromCharChecked(&mut ptr, &mut absLevel) {
                if g_displayLevel >= 2i32 {
                    fprintf(g_displayOut,
                            b"Ignore environment variable setting %s=%s: numeric value too large\n\x00"
                                as *const u8 as *const libc::c_char,
                            b"ZSTD_CLEVEL\x00" as *const u8 as
                                *const libc::c_char, env);
                }
                return 3i32
            } else {
                if *ptr as libc::c_int == 0i32 {
                    return (sign as libc::c_uint).wrapping_mul(absLevel) as
                               libc::c_int
                }
            }
        }
        if g_displayLevel >= 2i32 {
            fprintf(g_displayOut,
                    b"Ignore environment variable setting %s=%s: not a valid integer value\n\x00"
                        as *const u8 as *const libc::c_char,
                    b"ZSTD_CLEVEL\x00" as *const u8 as *const libc::c_char,
                    env);
        }
    }
    return 3i32;
}
unsafe fn main_0(mut argCount: libc::c_int,
                 mut argv: *mut *const libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut argNb: libc::c_int = 0;
    let mut followLinks: libc::c_int = 0i32;
    let mut forceStdout: libc::c_int = 0i32;
    let mut lastCommand: libc::c_int = 0i32;
    let mut ldmFlag: libc::c_int = 0i32;
    let mut main_pause: libc::c_int = 0i32;
    let mut nbWorkers: libc::c_int = 0i32;
    let mut adapt: libc::c_int = 0i32;
    let mut adaptMin: libc::c_int = ZSTD_minCLevel();
    let mut adaptMax: libc::c_int = ZSTD_maxCLevel();
    let mut rsyncable: libc::c_int = 0i32;
    let mut nextArgumentIsOutFileName: libc::c_int = 0i32;
    let mut nextArgumentIsMaxDict: libc::c_int = 0i32;
    let mut nextArgumentIsDictID: libc::c_int = 0i32;
    let mut nextArgumentsAreFiles: libc::c_int = 0i32;
    let mut nextEntryIsDictionary: libc::c_int = 0i32;
    let mut operationResult: libc::c_int = 0i32;
    let mut separateFiles: libc::c_int = 0i32;
    let mut setRealTimePrio: libc::c_int = 0i32;
    let mut singleThread: libc::c_int = 0i32;
    let mut ultra: libc::c_int = 0i32;
    let mut compressibility: libc::c_double = 0.5f64;
    /* would be better if this value was synchronized from bench */
    let mut bench_nbSeconds: libc::c_uint = 3i32 as libc::c_uint;
    let mut blockSize: size_t = 0i32 as size_t;
    let prefs: *mut FIO_prefs_t = FIO_createPreferences();
    let mut operation: zstd_operation_mode = zom_compress;
    let mut compressionParams: ZSTD_compressionParameters =
        ZSTD_compressionParameters{windowLog: 0,
                                   chainLog: 0,
                                   hashLog: 0,
                                   searchLog: 0,
                                   minMatch: 0,
                                   targetLength: 0,
                                   strategy: 0 as ZSTD_strategy,};
    let mut cLevel: libc::c_int = 0;
    let mut cLevelLast: libc::c_int = -1000000000i32;
    let mut recursive: libc::c_uint = 0i32 as libc::c_uint;
    let mut memLimit: libc::c_uint = 0i32 as libc::c_uint;
    /* argCount >= 1 */
    let mut filenameTable: *mut *const libc::c_char =
        malloc((argCount as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*const libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *const libc::c_char;
    let mut filenameIdx: libc::c_uint = 0i32 as libc::c_uint;
    let mut programName: *const libc::c_char = *argv.offset(0isize);
    let mut outFileName: *const libc::c_char = 0 as *const libc::c_char;
    let mut dictFileName: *const libc::c_char = 0 as *const libc::c_char;
    let mut suffix: *const libc::c_char =
        b".zst\x00" as *const u8 as *const libc::c_char;
    let mut maxDictSize: libc::c_uint = g_defaultMaxDictSize;
    let mut dictID: libc::c_uint = 0i32 as libc::c_uint;
    let mut dictCLevel: libc::c_int = g_defaultDictCLevel;
    let mut dictSelect: libc::c_uint = g_defaultSelectivityLevel;
    let mut extendedFileList: *mut *const libc::c_char =
        0 as *mut *const libc::c_char;
    let mut fileNamesBuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fileNamesNb: libc::c_uint = 0;
    let mut coverParams: ZDICT_cover_params_t = defaultCoverParams();
    let mut fastCoverParams: ZDICT_fastCover_params_t =
        defaultFastCoverParams();
    let mut dict: dictType = fastCover;
    let mut benchParams: BMK_advancedParams_t = BMK_initAdvancedParams();
    if filenameTable.is_null() {
        fprintf(g_displayOut,
                b"zstd: %s \n\x00" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()));
        exit(1i32);
    }
    let ref mut fresh0 = *filenameTable.offset(0isize);
    *fresh0 = b"/*stdin*\\\x00" as *const u8 as *const libc::c_char;
    g_displayOut = stderr;
    cLevel = init_cLevel();
    programName = lastNameFromPath(programName);
    nbWorkers = 1i32;
    if 0 !=
           exeNameMatch(programName,
                        b"zstdmt\x00" as *const u8 as *const libc::c_char) {
        nbWorkers = 0i32;
        singleThread = 0i32
    }
    if 0 !=
           exeNameMatch(programName,
                        b"unzstd\x00" as *const u8 as *const libc::c_char) {
        operation = zom_decompress
    }
    if 0 !=
           exeNameMatch(programName,
                        b"zstdcat\x00" as *const u8 as *const libc::c_char) {
        operation = zom_decompress;
        forceStdout = 1i32;
        FIO_overwriteMode(prefs);
        outFileName = b"/*stdout*\\\x00" as *const u8 as *const libc::c_char;
        g_displayLevel = 1i32
    }
    if 0 !=
           exeNameMatch(programName,
                        b"zcat\x00" as *const u8 as *const libc::c_char) {
        operation = zom_decompress;
        forceStdout = 1i32;
        FIO_overwriteMode(prefs);
        outFileName = b"/*stdout*\\\x00" as *const u8 as *const libc::c_char;
        g_displayLevel = 1i32
    }
    if 0 !=
           exeNameMatch(programName,
                        b"gzip\x00" as *const u8 as *const libc::c_char) {
        suffix = b".gz\x00" as *const u8 as *const libc::c_char;
        FIO_setCompressionType(prefs, FIO_gzipCompression);
        FIO_setRemoveSrcFile(prefs, 1i32 as libc::c_uint);
    }
    if 0 !=
           exeNameMatch(programName,
                        b"gunzip\x00" as *const u8 as *const libc::c_char) {
        operation = zom_decompress;
        FIO_setRemoveSrcFile(prefs, 1i32 as libc::c_uint);
    }
    if 0 !=
           exeNameMatch(programName,
                        b"gzcat\x00" as *const u8 as *const libc::c_char) {
        operation = zom_decompress;
        forceStdout = 1i32;
        FIO_overwriteMode(prefs);
        outFileName = b"/*stdout*\\\x00" as *const u8 as *const libc::c_char;
        g_displayLevel = 1i32
    }
    if 0 !=
           exeNameMatch(programName,
                        b"lzma\x00" as *const u8 as *const libc::c_char) {
        suffix = b".lzma\x00" as *const u8 as *const libc::c_char;
        FIO_setCompressionType(prefs, FIO_lzmaCompression);
        FIO_setRemoveSrcFile(prefs, 1i32 as libc::c_uint);
    }
    if 0 !=
           exeNameMatch(programName,
                        b"unlzma\x00" as *const u8 as *const libc::c_char) {
        operation = zom_decompress;
        FIO_setCompressionType(prefs, FIO_lzmaCompression);
        FIO_setRemoveSrcFile(prefs, 1i32 as libc::c_uint);
    }
    if 0 !=
           exeNameMatch(programName,
                        b"xz\x00" as *const u8 as *const libc::c_char) {
        suffix = b".xz\x00" as *const u8 as *const libc::c_char;
        FIO_setCompressionType(prefs, FIO_xzCompression);
        FIO_setRemoveSrcFile(prefs, 1i32 as libc::c_uint);
    }
    if 0 !=
           exeNameMatch(programName,
                        b"unxz\x00" as *const u8 as *const libc::c_char) {
        operation = zom_decompress;
        FIO_setCompressionType(prefs, FIO_xzCompression);
        FIO_setRemoveSrcFile(prefs, 1i32 as libc::c_uint);
    }
    if 0 !=
           exeNameMatch(programName,
                        b"lz4\x00" as *const u8 as *const libc::c_char) {
        suffix = b".lz4\x00" as *const u8 as *const libc::c_char;
        FIO_setCompressionType(prefs, FIO_lz4Compression);
    }
    if 0 !=
           exeNameMatch(programName,
                        b"unlz4\x00" as *const u8 as *const libc::c_char) {
        operation = zom_decompress;
        FIO_setCompressionType(prefs, FIO_lz4Compression);
    }
    memset(&mut compressionParams as *mut ZSTD_compressionParameters as
               *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ZSTD_compressionParameters>() as
               libc::c_ulong);
    FIO_addAbortHandler();
    /* command switches */
    argNb = 1i32;
    's_415:
        loop  {
            if !(argNb < argCount) {
                current_block = 15531972362839779146;
                break ;
            }
            let mut argument: *const libc::c_char =
                *argv.offset(argNb as isize);
            if !argument.is_null() {
                /* Protection if argument empty */
                if nextArgumentsAreFiles == 0i32 {
                    /* "-" means stdin/stdout */
                    if 0 ==
                           strcmp(argument,
                                  b"-\x00" as *const u8 as
                                      *const libc::c_char) {
                        if 0 == filenameIdx {
                            filenameIdx = 1i32 as libc::c_uint;
                            let ref mut fresh1 =
                                *filenameTable.offset(0isize);
                            *fresh1 =
                                b"/*stdin*\\\x00" as *const u8 as
                                    *const libc::c_char;
                            outFileName =
                                b"/*stdout*\\\x00" as *const u8 as
                                    *const libc::c_char;
                            g_displayLevel -=
                                (g_displayLevel == 2i32) as libc::c_int;
                            current_block = 14648606000749551097;
                        } else { current_block = 2616667235040759262; }
                    } else { current_block = 2616667235040759262; }
                    match current_block {
                        14648606000749551097 => { }
                        _ => {
                            /* Decode commands (note : aggregated commands are allowed) */
                            if *argument.offset(0isize) as libc::c_int ==
                                   '-' as i32 {
                                if *argument.offset(1isize) as libc::c_int ==
                                       '-' as i32 {
                                    /* long commands (--long-word) */
                                    if 0 ==
                                           strcmp(argument,
                                                  b"--\x00" as *const u8 as
                                                      *const libc::c_char) {
                                        nextArgumentsAreFiles = 1i32;
                                        /* only file names allowed from now on */
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--list\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        operation = zom_list;
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--compress\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        operation = zom_compress;
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--decompress\x00"
                                                             as *const u8 as
                                                             *const libc::c_char)
                                     {
                                        operation = zom_decompress;
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--uncompress\x00"
                                                             as *const u8 as
                                                             *const libc::c_char)
                                     {
                                        operation = zom_decompress;
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--force\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        FIO_overwriteMode(prefs);
                                        forceStdout = 1i32;
                                        followLinks = 1i32;
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--version\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        g_displayOut = stdout;
                                        fprintf(g_displayOut,
                                                b"*** %s %i-bits %s, by %s ***\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                b"zstd command line interface\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                (::std::mem::size_of::<size_t>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8i32
                                                                                     as
                                                                                     libc::c_ulong)
                                                    as libc::c_int,
                                                b"v1.3.8\x00" as *const u8 as
                                                    *const libc::c_char,
                                                b"Yann Collet\x00" as
                                                    *const u8 as
                                                    *const libc::c_char);
                                        operationResult = 0i32;
                                        current_block = 15272278224568454711;
                                        break ;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--help\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        g_displayOut = stdout;
                                        operationResult =
                                            usage_advanced(programName);
                                        current_block = 15272278224568454711;
                                        break ;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--verbose\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        g_displayLevel += 1;
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--quiet\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        g_displayLevel -= 1;
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--stdout\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        forceStdout = 1i32;
                                        outFileName =
                                            b"/*stdout*\\\x00" as *const u8 as
                                                *const libc::c_char;
                                        g_displayLevel -=
                                            (g_displayLevel == 2i32) as
                                                libc::c_int;
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--ultra\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        ultra = 1i32;
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--check\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        FIO_setChecksumFlag(prefs,
                                                            2i32 as
                                                                libc::c_uint);
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--no-check\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        FIO_setChecksumFlag(prefs,
                                                            0i32 as
                                                                libc::c_uint);
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--sparse\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        FIO_setSparseWrite(prefs,
                                                           2i32 as
                                                               libc::c_uint);
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--no-sparse\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        FIO_setSparseWrite(prefs,
                                                           0i32 as
                                                               libc::c_uint);
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--test\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        operation = zom_test;
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--train\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        operation = zom_train;
                                        if outFileName.is_null() {
                                            outFileName = g_defaultDictName
                                        }
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--maxdict\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        nextArgumentIsMaxDict = 1i32;
                                        lastCommand = 1i32;
                                        /* kept available for compatibility with old syntax ; will be removed one day */
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--dictID\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        nextArgumentIsDictID = 1i32;
                                        lastCommand = 1i32;
                                        /* kept available for compatibility with old syntax ; will be removed one day */
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--no-dictID\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        FIO_setDictIDFlag(prefs,
                                                          0i32 as
                                                              libc::c_uint);
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--keep\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        FIO_setRemoveSrcFile(prefs,
                                                             0i32 as
                                                                 libc::c_uint);
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--rm\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        FIO_setRemoveSrcFile(prefs,
                                                             1i32 as
                                                                 libc::c_uint);
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--priority=rt\x00"
                                                             as *const u8 as
                                                             *const libc::c_char)
                                     {
                                        setRealTimePrio = 1i32;
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--adapt\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        adapt = 1i32;
                                        current_block = 14648606000749551097;
                                    } else if 0 !=
                                                  longCommandWArg(&mut argument,
                                                                  b"--adapt=\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                     {
                                        adapt = 1i32;
                                        if 0 ==
                                               parseAdaptParameters(argument,
                                                                    &mut adaptMin,
                                                                    &mut adaptMax)
                                           {
                                            operationResult =
                                                badusage(programName);
                                            current_block =
                                                15272278224568454711;
                                            break ;
                                        } else {
                                            current_block =
                                                14648606000749551097;
                                        }
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--single-thread\x00"
                                                             as *const u8 as
                                                             *const libc::c_char)
                                     {
                                        nbWorkers = 0i32;
                                        singleThread = 1i32;
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--format=zstd\x00"
                                                             as *const u8 as
                                                             *const libc::c_char)
                                     {
                                        suffix =
                                            b".zst\x00" as *const u8 as
                                                *const libc::c_char;
                                        FIO_setCompressionType(prefs,
                                                               FIO_zstdCompression);
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--rsyncable\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                     {
                                        rsyncable = 1i32;
                                        current_block = 14648606000749551097;
                                    } else if 0 ==
                                                  strcmp(argument,
                                                         b"--no-progress\x00"
                                                             as *const u8 as
                                                             *const libc::c_char)
                                     {
                                        FIO_setNoProgress(1i32 as
                                                              libc::c_uint);
                                        current_block = 14648606000749551097;
                                    } else if 0 !=
                                                  longCommandWArg(&mut argument,
                                                                  b"--train-cover\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                     {
                                        operation = zom_train;
                                        if outFileName.is_null() {
                                            outFileName = g_defaultDictName
                                        }
                                        dict = cover;
                                        /* Allow optional arguments following an = */
                                        if *argument as libc::c_int == 0i32 {
                                            memset(&mut coverParams as
                                                       *mut ZDICT_cover_params_t
                                                       as *mut libc::c_void,
                                                   0i32,
                                                   ::std::mem::size_of::<ZDICT_cover_params_t>()
                                                       as libc::c_ulong);
                                        } else {
                                            let fresh2 = argument;
                                            argument = argument.offset(1);
                                            if *fresh2 as libc::c_int !=
                                                   '=' as i32 {
                                                operationResult =
                                                    badusage(programName);
                                                current_block =
                                                    15272278224568454711;
                                                break ;
                                            } else if 0 ==
                                                          parseCoverParameters(argument,
                                                                               &mut coverParams)
                                             {
                                                operationResult =
                                                    badusage(programName);
                                                current_block =
                                                    15272278224568454711;
                                                break ;
                                            }
                                        }
                                        current_block = 14648606000749551097;
                                    } else if 0 !=
                                                  longCommandWArg(&mut argument,
                                                                  b"--train-fastcover\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                     {
                                        operation = zom_train;
                                        if outFileName.is_null() {
                                            outFileName = g_defaultDictName
                                        }
                                        dict = fastCover;
                                        /* Allow optional arguments following an = */
                                        if *argument as libc::c_int == 0i32 {
                                            memset(&mut fastCoverParams as
                                                       *mut ZDICT_fastCover_params_t
                                                       as *mut libc::c_void,
                                                   0i32,
                                                   ::std::mem::size_of::<ZDICT_fastCover_params_t>()
                                                       as libc::c_ulong);
                                        } else {
                                            let fresh3 = argument;
                                            argument = argument.offset(1);
                                            if *fresh3 as libc::c_int !=
                                                   '=' as i32 {
                                                operationResult =
                                                    badusage(programName);
                                                current_block =
                                                    15272278224568454711;
                                                break ;
                                            } else if 0 ==
                                                          parseFastCoverParameters(argument,
                                                                                   &mut fastCoverParams)
                                             {
                                                operationResult =
                                                    badusage(programName);
                                                current_block =
                                                    15272278224568454711;
                                                break ;
                                            }
                                        }
                                        current_block = 14648606000749551097;
                                    } else if 0 !=
                                                  longCommandWArg(&mut argument,
                                                                  b"--train-legacy\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                     {
                                        operation = zom_train;
                                        if outFileName.is_null() {
                                            outFileName = g_defaultDictName
                                        }
                                        dict = legacy;
                                        /* Allow optional arguments following an = */
                                        if *argument as libc::c_int == 0i32 {
                                            current_block =
                                                14648606000749551097;
                                        } else {
                                            let fresh4 = argument;
                                            argument = argument.offset(1);
                                            if *fresh4 as libc::c_int !=
                                                   '=' as i32 {
                                                operationResult =
                                                    badusage(programName);
                                                current_block =
                                                    15272278224568454711;
                                                break ;
                                            } else if 0 ==
                                                          parseLegacyParameters(argument,
                                                                                &mut dictSelect)
                                             {
                                                operationResult =
                                                    badusage(programName);
                                                current_block =
                                                    15272278224568454711;
                                                break ;
                                            }
                                            current_block =
                                                14648606000749551097;
                                        }
                                    } else if 0 !=
                                                  longCommandWArg(&mut argument,
                                                                  b"--threads=\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                     {
                                        nbWorkers =
                                            readU32FromChar(&mut argument) as
                                                libc::c_int;
                                        current_block = 14648606000749551097;
                                    } else if 0 !=
                                                  longCommandWArg(&mut argument,
                                                                  b"--memlimit=\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                     {
                                        memLimit =
                                            readU32FromChar(&mut argument);
                                        current_block = 14648606000749551097;
                                    } else if 0 !=
                                                  longCommandWArg(&mut argument,
                                                                  b"--memory=\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                     {
                                        memLimit =
                                            readU32FromChar(&mut argument);
                                        current_block = 14648606000749551097;
                                    } else if 0 !=
                                                  longCommandWArg(&mut argument,
                                                                  b"--memlimit-decompress=\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                     {
                                        memLimit =
                                            readU32FromChar(&mut argument);
                                        current_block = 14648606000749551097;
                                    } else if 0 !=
                                                  longCommandWArg(&mut argument,
                                                                  b"--block-size=\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                     {
                                        blockSize =
                                            readU32FromChar(&mut argument) as
                                                size_t;
                                        current_block = 14648606000749551097;
                                    } else if 0 !=
                                                  longCommandWArg(&mut argument,
                                                                  b"--maxdict=\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                     {
                                        maxDictSize =
                                            readU32FromChar(&mut argument);
                                        current_block = 14648606000749551097;
                                    } else if 0 !=
                                                  longCommandWArg(&mut argument,
                                                                  b"--dictID=\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                     {
                                        dictID =
                                            readU32FromChar(&mut argument);
                                        current_block = 14648606000749551097;
                                    } else if 0 !=
                                                  longCommandWArg(&mut argument,
                                                                  b"--zstd=\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                     {
                                        if 0 ==
                                               parseCompressionParameters(argument,
                                                                          &mut compressionParams)
                                           {
                                            operationResult =
                                                badusage(programName);
                                            current_block =
                                                15272278224568454711;
                                            break ;
                                        } else {
                                            current_block =
                                                14648606000749551097;
                                        }
                                    } else if 0 !=
                                                  longCommandWArg(&mut argument,
                                                                  b"--long\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                     {
                                        let mut ldmWindowLog: libc::c_uint =
                                            0i32 as libc::c_uint;
                                        ldmFlag = 1i32;
                                        /* Parse optional window log */
                                        if *argument as libc::c_int ==
                                               '=' as i32 {
                                            argument =
                                                argument.offset(1isize);
                                            ldmWindowLog =
                                                readU32FromChar(&mut argument)
                                        } else if *argument as libc::c_int !=
                                                      0i32 {
                                            /* Invalid character following --long */
                                            operationResult =
                                                badusage(programName);
                                            current_block =
                                                15272278224568454711;
                                            break ;
                                        }
                                        if compressionParams.windowLog ==
                                               0i32 as libc::c_uint {
                                            compressionParams.windowLog =
                                                ldmWindowLog
                                        }
                                        current_block = 14648606000749551097;
                                    } else if 0 !=
                                                  longCommandWArg(&mut argument,
                                                                  b"--fast\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                     {
                                        /* Parse optional acceleration factor */
                                        if *argument as libc::c_int ==
                                               '=' as i32 {
                                            let maxFast: U32 =
                                                -ZSTD_minCLevel() as U32;
                                            let mut fastLevel: U32 = 0;
                                            argument =
                                                argument.offset(1isize);
                                            fastLevel =
                                                readU32FromChar(&mut argument);
                                            if fastLevel > maxFast {
                                                fastLevel = maxFast
                                            }
                                            if 0 != fastLevel {
                                                cLevel =
                                                    -(fastLevel as
                                                          libc::c_int);
                                                dictCLevel = cLevel
                                            } else {
                                                operationResult =
                                                    badusage(programName);
                                                current_block =
                                                    15272278224568454711;
                                                break ;
                                            }
                                        } else if *argument as libc::c_int !=
                                                      0i32 {
                                            /* Invalid character following --fast */
                                            operationResult =
                                                badusage(programName);
                                            current_block =
                                                15272278224568454711;
                                            break ;
                                        } else { cLevel = -1i32 }
                                        current_block = 14648606000749551097;
                                    } else {
                                        current_block = 10779579613334775377;
                                    }
                                } else {
                                    current_block = 10779579613334775377;
                                }
                                match current_block {
                                    14648606000749551097 => { }
                                    _ => {
                                        argument = argument.offset(1isize);
                                        while *argument.offset(0isize) as
                                                  libc::c_int != 0i32 {
                                            if 0 != lastCommand {
                                                fprintf(g_displayOut,
                                                        b"error : command must be followed by argument \n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char);
                                                operationResult = 1i32;
                                                current_block =
                                                    15272278224568454711;
                                                break 's_415 ;
                                            } else if *argument as libc::c_int
                                                          >= '0' as i32 &&
                                                          *argument as
                                                              libc::c_int <=
                                                              '9' as i32 {
                                                cLevel =
                                                    readU32FromChar(&mut argument)
                                                        as libc::c_int;
                                                dictCLevel = cLevel
                                            } else {
                                                match *argument.offset(0isize)
                                                          as libc::c_int {
                                                    86 => {
                                                        g_displayOut = stdout;
                                                        printVersion();
                                                        operationResult =
                                                            0i32;
                                                        current_block =
                                                            15272278224568454711;
                                                        break 's_415 ;
                                                    }
                                                    72 | 104 => {
                                                        g_displayOut = stdout;
                                                        operationResult =
                                                            usage_advanced(programName);
                                                        current_block =
                                                            15272278224568454711;
                                                        break 's_415 ;
                                                    }
                                                    122 => {
                                                        operation =
                                                            zom_compress;
                                                        argument =
                                                            argument.offset(1isize)
                                                    }
                                                    100 => {
                                                        benchParams.mode =
                                                            BMK_decodeOnly;
                                                        if operation as
                                                               libc::c_uint ==
                                                               zom_bench as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint
                                                           {
                                                            argument =
                                                                argument.offset(1isize)
                                                        } else {
                                                            /* benchmark decode (hidden option) */
                                                            operation =
                                                                zom_decompress;
                                                            argument =
                                                                argument.offset(1isize)
                                                        }
                                                    }
                                                    99 => {
                                                        forceStdout = 1i32;
                                                        outFileName =
                                                            b"/*stdout*\\\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char;
                                                        argument =
                                                            argument.offset(1isize)
                                                    }
                                                    68 => {
                                                        nextEntryIsDictionary
                                                            = 1i32;
                                                        lastCommand = 1i32;
                                                        argument =
                                                            argument.offset(1isize)
                                                    }
                                                    102 => {
                                                        FIO_overwriteMode(prefs);
                                                        forceStdout = 1i32;
                                                        followLinks = 1i32;
                                                        argument =
                                                            argument.offset(1isize)
                                                    }
                                                    118 => {
                                                        g_displayLevel += 1;
                                                        argument =
                                                            argument.offset(1isize)
                                                    }
                                                    113 => {
                                                        g_displayLevel -= 1;
                                                        argument =
                                                            argument.offset(1isize)
                                                    }
                                                    107 => {
                                                        FIO_setRemoveSrcFile(prefs,
                                                                             0i32
                                                                                 as
                                                                                 libc::c_uint);
                                                        argument =
                                                            argument.offset(1isize)
                                                    }
                                                    67 => {
                                                        FIO_setChecksumFlag(prefs,
                                                                            2i32
                                                                                as
                                                                                libc::c_uint);
                                                        argument =
                                                            argument.offset(1isize)
                                                    }
                                                    116 => {
                                                        operation = zom_test;
                                                        argument =
                                                            argument.offset(1isize)
                                                    }
                                                    111 => {
                                                        nextArgumentIsOutFileName
                                                            = 1i32;
                                                        lastCommand = 1i32;
                                                        argument =
                                                            argument.offset(1isize)
                                                    }
                                                    77 => {
                                                        argument =
                                                            argument.offset(1isize);
                                                        memLimit =
                                                            readU32FromChar(&mut argument)
                                                    }
                                                    108 => {
                                                        operation = zom_list;
                                                        argument =
                                                            argument.offset(1isize)
                                                    }
                                                    114 => {
                                                        recursive =
                                                            1i32 as
                                                                libc::c_uint;
                                                        argument =
                                                            argument.offset(1isize)
                                                    }
                                                    98 => {
                                                        operation = zom_bench;
                                                        argument =
                                                            argument.offset(1isize)
                                                    }
                                                    101 => {
                                                        argument =
                                                            argument.offset(1isize);
                                                        cLevelLast =
                                                            readU32FromChar(&mut argument)
                                                                as libc::c_int
                                                    }
                                                    105 => {
                                                        argument =
                                                            argument.offset(1isize);
                                                        bench_nbSeconds =
                                                            readU32FromChar(&mut argument)
                                                    }
                                                    66 => {
                                                        argument =
                                                            argument.offset(1isize);
                                                        blockSize =
                                                            readU32FromChar(&mut argument)
                                                                as size_t
                                                    }
                                                    83 => {
                                                        argument =
                                                            argument.offset(1isize);
                                                        separateFiles = 1i32
                                                    }
                                                    84 => {
                                                        argument =
                                                            argument.offset(1isize);
                                                        nbWorkers =
                                                            readU32FromChar(&mut argument)
                                                                as libc::c_int
                                                    }
                                                    115 => {
                                                        argument =
                                                            argument.offset(1isize);
                                                        dictSelect =
                                                            readU32FromChar(&mut argument)
                                                    }
                                                    112 => {
                                                        argument =
                                                            argument.offset(1isize);
                                                        if *argument as
                                                               libc::c_int >=
                                                               '0' as i32 &&
                                                               *argument as
                                                                   libc::c_int
                                                                   <=
                                                                   '9' as i32
                                                           {
                                                            benchParams.additionalParam
                                                                =
                                                                readU32FromChar(&mut argument)
                                                                    as
                                                                    libc::c_int
                                                        } else {
                                                            main_pause = 1i32
                                                        }
                                                    }
                                                    80 => {
                                                        argument =
                                                            argument.offset(1isize);
                                                        compressibility =
                                                            readU32FromChar(&mut argument)
                                                                as
                                                                libc::c_double
                                                                /
                                                                100i32 as
                                                                    libc::c_double
                                                    }
                                                    _ => {
                                                        operationResult =
                                                            badusage(programName);
                                                        current_block =
                                                            15272278224568454711;
                                                        break 's_415 ;
                                                    }
                                                }
                                            }
                                        }
                                        current_block = 14648606000749551097;
                                    }
                                }
                            } else if 0 != nextArgumentIsMaxDict {
                                /* kept available for compatibility with old syntax ; will be removed one day */
                                nextArgumentIsMaxDict = 0i32;
                                lastCommand = 0i32;
                                maxDictSize = readU32FromChar(&mut argument);
                                current_block = 14648606000749551097;
                            } else if 0 != nextArgumentIsDictID {
                                /* kept available for compatibility with old syntax ; will be removed one day */
                                nextArgumentIsDictID = 0i32;
                                lastCommand = 0i32;
                                dictID = readU32FromChar(&mut argument);
                                current_block = 14648606000749551097;
                            } else { current_block = 15895512938235629561; }
                        }
                    }
                } else { current_block = 15895512938235629561; }
                match current_block {
                    14648606000749551097 => { }
                    _ => {
                        /* if (nextArgumentIsAFile==0) */
                        if 0 != nextEntryIsDictionary {
                            nextEntryIsDictionary = 0i32;
                            lastCommand = 0i32;
                            dictFileName = argument
                        } else if 0 != nextArgumentIsOutFileName {
                            nextArgumentIsOutFileName = 0i32;
                            lastCommand = 0i32;
                            outFileName = argument;
                            if 0 ==
                                   strcmp(outFileName,
                                          b"-\x00" as *const u8 as
                                              *const libc::c_char) {
                                outFileName =
                                    b"/*stdout*\\\x00" as *const u8 as
                                        *const libc::c_char
                            }
                        } else {
                            let fresh5 = filenameIdx;
                            filenameIdx = filenameIdx.wrapping_add(1);
                            let ref mut fresh6 =
                                *filenameTable.offset(fresh5 as isize);
                            *fresh6 = argument
                        }
                    }
                }
            }
            argNb += 1
        }
    match current_block {
        15531972362839779146 => {
            if 0 != lastCommand {
                /* forgotten argument */
                fprintf(g_displayOut,
                        b"error : command must be followed by argument \n\x00"
                            as *const u8 as *const libc::c_char);
                operationResult = 1i32
            } else {
                if g_displayLevel >= 3i32 {
                    fprintf(g_displayOut,
                            b"*** %s %i-bits %s, by %s ***\n\x00" as *const u8
                                as *const libc::c_char,
                            b"zstd command line interface\x00" as *const u8 as
                                *const libc::c_char,
                            (::std::mem::size_of::<size_t>() as
                                 libc::c_ulong).wrapping_mul(8i32 as
                                                                 libc::c_ulong)
                                as libc::c_int,
                            b"v1.3.8\x00" as *const u8 as *const libc::c_char,
                            b"Yann Collet\x00" as *const u8 as
                                *const libc::c_char);
                }
                if nbWorkers == 0i32 && 0 == singleThread {
                    nbWorkers = UTIL_countPhysicalCores();
                    if g_displayLevel >= 3i32 {
                        fprintf(g_displayOut,
                                b"Note: %d physical core(s) detected \n\x00"
                                    as *const u8 as *const libc::c_char,
                                nbWorkers);
                    }
                }
                g_utilDisplayLevel = g_displayLevel;
                if 0 == followLinks {
                    let mut u: libc::c_uint = 0;
                    u = 0i32 as libc::c_uint;
                    fileNamesNb = 0i32 as libc::c_uint;
                    while u < filenameIdx {
                        if 0 != UTIL_isLink(*filenameTable.offset(u as isize))
                           {
                            if g_displayLevel >= 2i32 {
                                fprintf(g_displayOut,
                                        b"Warning : %s is a symbolic link, ignoring\n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        *filenameTable.offset(u as isize));
                            }
                        } else {
                            let fresh7 = fileNamesNb;
                            fileNamesNb = fileNamesNb.wrapping_add(1);
                            let ref mut fresh8 =
                                *filenameTable.offset(fresh7 as isize);
                            *fresh8 = *filenameTable.offset(u as isize)
                        }
                        u = u.wrapping_add(1)
                    }
                    filenameIdx = fileNamesNb
                }
                if 0 != recursive {
                    extendedFileList =
                        UTIL_createFileList(filenameTable, filenameIdx,
                                            &mut fileNamesBuf,
                                            &mut fileNamesNb, followLinks);
                    if !extendedFileList.is_null() {
                        let mut u_0: libc::c_uint = 0;
                        u_0 = 0i32 as libc::c_uint;
                        while u_0 < fileNamesNb {
                            if g_displayLevel >= 4i32 {
                                fprintf(g_displayOut,
                                        b"%u %s\n\x00" as *const u8 as
                                            *const libc::c_char, u_0,
                                        *extendedFileList.offset(u_0 as
                                                                     isize));
                            }
                            u_0 = u_0.wrapping_add(1)
                        }
                        free(filenameTable as *mut libc::c_void);
                        filenameTable = extendedFileList;
                        filenameIdx = fileNamesNb
                    }
                }
                if operation as libc::c_uint ==
                       zom_list as libc::c_int as libc::c_uint {
                    let ret: libc::c_int =
                        FIO_listMultipleFiles(filenameIdx, filenameTable,
                                              g_displayLevel);
                    operationResult = ret
                } else if operation as libc::c_uint ==
                              zom_bench as libc::c_int as libc::c_uint {
                    benchParams.blockSize = blockSize;
                    benchParams.nbWorkers = nbWorkers as libc::c_uint;
                    benchParams.realTime = setRealTimePrio as libc::c_uint;
                    benchParams.nbSeconds = bench_nbSeconds;
                    benchParams.ldmFlag = ldmFlag as libc::c_uint;
                    benchParams.ldmMinMatch = g_ldmMinMatch;
                    benchParams.ldmHashLog = g_ldmHashLog;
                    if g_ldmBucketSizeLog != 9999i32 as libc::c_uint {
                        benchParams.ldmBucketSizeLog = g_ldmBucketSizeLog
                    }
                    if g_ldmHashRateLog != 9999i32 as libc::c_uint {
                        benchParams.ldmHashRateLog = g_ldmHashRateLog
                    }
                    if cLevel > ZSTD_maxCLevel() { cLevel = ZSTD_maxCLevel() }
                    if cLevelLast > ZSTD_maxCLevel() {
                        cLevelLast = ZSTD_maxCLevel()
                    }
                    if cLevelLast < cLevel { cLevelLast = cLevel }
                    if cLevelLast > cLevel {
                        if g_displayLevel >= 3i32 {
                            fprintf(g_displayOut,
                                    b"Benchmarking levels from %d to %d\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    cLevel, cLevelLast);
                        }
                    }
                    if 0 != filenameIdx {
                        if 0 != separateFiles {
                            let mut i: libc::c_uint = 0;
                            i = 0i32 as libc::c_uint;
                            while i < filenameIdx {
                                let mut c: libc::c_int = 0;
                                if g_displayLevel >= 3i32 {
                                    fprintf(g_displayOut,
                                            b"Benchmarking %s \n\x00" as
                                                *const u8 as
                                                *const libc::c_char,
                                            *filenameTable.offset(i as
                                                                      isize));
                                }
                                c = cLevel;
                                while c <= cLevelLast {
                                    BMK_benchFilesAdvanced(&mut *filenameTable.offset(i
                                                                                          as
                                                                                          isize),
                                                           1i32 as
                                                               libc::c_uint,
                                                           dictFileName, c,
                                                           &mut compressionParams,
                                                           g_displayLevel,
                                                           &mut benchParams);
                                    c += 1
                                }
                                i = i.wrapping_add(1)
                            }
                        } else {
                            while cLevel <= cLevelLast {
                                BMK_benchFilesAdvanced(filenameTable,
                                                       filenameIdx,
                                                       dictFileName, cLevel,
                                                       &mut compressionParams,
                                                       g_displayLevel,
                                                       &mut benchParams);
                                cLevel += 1
                            }
                        }
                    } else {
                        while cLevel <= cLevelLast {
                            BMK_syntheticTest(cLevel, compressibility,
                                              &mut compressionParams,
                                              g_displayLevel,
                                              &mut benchParams);
                            cLevel += 1
                        }
                    }
                } else if operation as libc::c_uint ==
                              zom_train as libc::c_int as libc::c_uint {
                    let mut zParams: ZDICT_params_t =
                        ZDICT_params_t{compressionLevel: 0,
                                       notificationLevel: 0,
                                       dictID: 0,};
                    zParams.compressionLevel = dictCLevel;
                    zParams.notificationLevel =
                        g_displayLevel as libc::c_uint;
                    zParams.dictID = dictID;
                    if dict as libc::c_uint ==
                           cover as libc::c_int as libc::c_uint {
                        let optimize: libc::c_int =
                            (0 == coverParams.k || 0 == coverParams.d) as
                                libc::c_int;
                        coverParams.nbThreads = nbWorkers as libc::c_uint;
                        coverParams.zParams = zParams;
                        operationResult =
                            DiB_trainFromFiles(outFileName, maxDictSize,
                                               filenameTable, filenameIdx,
                                               blockSize,
                                               0 as
                                                   *mut ZDICT_legacy_params_t,
                                               &mut coverParams,
                                               0 as
                                                   *mut ZDICT_fastCover_params_t,
                                               optimize)
                    } else if dict as libc::c_uint ==
                                  fastCover as libc::c_int as libc::c_uint {
                        let optimize_0: libc::c_int =
                            (0 == fastCoverParams.k || 0 == fastCoverParams.d)
                                as libc::c_int;
                        fastCoverParams.nbThreads = nbWorkers as libc::c_uint;
                        fastCoverParams.zParams = zParams;
                        operationResult =
                            DiB_trainFromFiles(outFileName, maxDictSize,
                                               filenameTable, filenameIdx,
                                               blockSize,
                                               0 as
                                                   *mut ZDICT_legacy_params_t,
                                               0 as *mut ZDICT_cover_params_t,
                                               &mut fastCoverParams,
                                               optimize_0)
                    } else {
                        let mut dictParams: ZDICT_legacy_params_t =
                            ZDICT_legacy_params_t{selectivityLevel: 0,
                                                  zParams:
                                                      ZDICT_params_t{compressionLevel:
                                                                         0,
                                                                     notificationLevel:
                                                                         0,
                                                                     dictID:
                                                                         0,},};
                        memset(&mut dictParams as *mut ZDICT_legacy_params_t
                                   as *mut libc::c_void, 0i32,
                               ::std::mem::size_of::<ZDICT_legacy_params_t>()
                                   as libc::c_ulong);
                        dictParams.selectivityLevel = dictSelect;
                        dictParams.zParams = zParams;
                        operationResult =
                            DiB_trainFromFiles(outFileName, maxDictSize,
                                               filenameTable, filenameIdx,
                                               blockSize, &mut dictParams,
                                               0 as *mut ZDICT_cover_params_t,
                                               0 as
                                                   *mut ZDICT_fastCover_params_t,
                                               0i32)
                    }
                } else {
                    if operation as libc::c_uint ==
                           zom_test as libc::c_int as libc::c_uint {
                        outFileName =
                            b"/dev/null\x00" as *const u8 as
                                *const libc::c_char;
                        FIO_setRemoveSrcFile(prefs, 0i32 as libc::c_uint);
                    }
                    filenameIdx =
                        filenameIdx.wrapping_add((0 == filenameIdx) as
                                                     libc::c_int as
                                                     libc::c_uint);
                    if 0 ==
                           strcmp(*filenameTable.offset(0isize),
                                  b"/*stdin*\\\x00" as *const u8 as
                                      *const libc::c_char) &&
                           outFileName.is_null() {
                        outFileName =
                            b"/*stdout*\\\x00" as *const u8 as
                                *const libc::c_char
                    }
                    /* Check if input/output defined as console; trigger an error in this case */
                    if 0 ==
                           strcmp(*filenameTable.offset(0isize),
                                  b"/*stdin*\\\x00" as *const u8 as
                                      *const libc::c_char) &&
                           0 != isatty(fileno(stdin)) {
                        operationResult = badusage(programName)
                    } else if !outFileName.is_null() &&
                                  0 ==
                                      strcmp(outFileName,
                                             b"/*stdout*\\\x00" as *const u8
                                                 as *const libc::c_char) &&
                                  0 != isatty(fileno(stdout)) &&
                                  0 ==
                                      strcmp(*filenameTable.offset(0isize),
                                             b"/*stdin*\\\x00" as *const u8 as
                                                 *const libc::c_char) &&
                                  0 == forceStdout &&
                                  operation as libc::c_uint !=
                                      zom_decompress as libc::c_int as
                                          libc::c_uint {
                        operationResult = badusage(programName)
                    } else {
                        let maxCLevel: libc::c_int =
                            if 0 != ultra { ZSTD_maxCLevel() } else { 19i32 };
                        if cLevel > maxCLevel {
                            if g_displayLevel >= 2i32 {
                                fprintf(g_displayOut,
                                        b"Warning : compression level higher than max, reduced to %i \n\x00"
                                            as *const u8 as
                                            *const libc::c_char, maxCLevel);
                            }
                            cLevel = maxCLevel
                        }
                        if 0 ==
                               strcmp(*filenameTable.offset(0isize),
                                      b"/*stdin*\\\x00" as *const u8 as
                                          *const libc::c_char) &&
                               !outFileName.is_null() &&
                               0 ==
                                   strcmp(outFileName,
                                          b"/*stdout*\\\x00" as *const u8 as
                                              *const libc::c_char) &&
                               g_displayLevel == 2i32 {
                            g_displayLevel = 1i32
                        }
                        if 0 !=
                               (filenameIdx > 1i32 as libc::c_uint) as
                                   libc::c_int &
                                   (g_displayLevel == 2i32) as libc::c_int {
                            g_displayLevel = 1i32
                        }
                        FIO_setNotificationLevel(g_displayLevel as
                                                     libc::c_uint);
                        if operation as libc::c_uint ==
                               zom_compress as libc::c_int as libc::c_uint {
                            FIO_setNbWorkers(prefs,
                                             nbWorkers as libc::c_uint);
                            FIO_setBlockSize(prefs, blockSize as U32);
                            if g_overlapLog != 9999i32 as libc::c_uint {
                                FIO_setOverlapLog(prefs, g_overlapLog);
                            }
                            FIO_setLdmFlag(prefs, ldmFlag as libc::c_uint);
                            FIO_setLdmHashLog(prefs, g_ldmHashLog);
                            FIO_setLdmMinMatch(prefs, g_ldmMinMatch);
                            if g_ldmBucketSizeLog != 9999i32 as libc::c_uint {
                                FIO_setLdmBucketSizeLog(prefs,
                                                        g_ldmBucketSizeLog);
                            }
                            if g_ldmHashRateLog != 9999i32 as libc::c_uint {
                                FIO_setLdmHashRateLog(prefs,
                                                      g_ldmHashRateLog);
                            }
                            FIO_setAdaptiveMode(prefs, adapt as libc::c_uint);
                            FIO_setAdaptMin(prefs, adaptMin);
                            FIO_setAdaptMax(prefs, adaptMax);
                            FIO_setRsyncable(prefs,
                                             rsyncable as libc::c_uint);
                            if adaptMin > cLevel { cLevel = adaptMin }
                            if adaptMax < cLevel { cLevel = adaptMax }
                            if filenameIdx == 1i32 as libc::c_uint &&
                                   !outFileName.is_null() {
                                operationResult =
                                    FIO_compressFilename(prefs, outFileName,
                                                         *filenameTable.offset(0isize),
                                                         dictFileName, cLevel,
                                                         compressionParams)
                            } else {
                                operationResult =
                                    FIO_compressMultipleFilenames(prefs,
                                                                  filenameTable,
                                                                  filenameIdx,
                                                                  outFileName,
                                                                  suffix,
                                                                  dictFileName,
                                                                  cLevel,
                                                                  compressionParams)
                            }
                        } else {
                            if memLimit == 0i32 as libc::c_uint {
                                if compressionParams.windowLog ==
                                       0i32 as libc::c_uint {
                                    memLimit =
                                        (1i32 as U32) << g_defaultMaxWindowLog
                                } else {
                                    memLimit =
                                        (1i32 as U32) <<
                                            (compressionParams.windowLog &
                                                 31i32 as libc::c_uint)
                                }
                            }
                            FIO_setMemLimit(prefs, memLimit);
                            if filenameIdx == 1i32 as libc::c_uint &&
                                   !outFileName.is_null() {
                                operationResult =
                                    FIO_decompressFilename(prefs, outFileName,
                                                           *filenameTable.offset(0isize),
                                                           dictFileName)
                            } else {
                                operationResult =
                                    FIO_decompressMultipleFilenames(prefs,
                                                                    filenameTable,
                                                                    filenameIdx,
                                                                    outFileName,
                                                                    dictFileName)
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    FIO_freePreferences(prefs);
    if 0 != main_pause { waitEnter(); }
    if !extendedFileList.is_null() {
        UTIL_freeFileList(extendedFileList, fileNamesBuf);
    } else { free(filenameTable as *mut libc::c_void); }
    return operationResult;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *const libc::c_char) as i32)
    }
}
unsafe extern "C" fn run_static_initializers() {
    max =
        (-1i32 as
             libc::c_uint).wrapping_div(10i32 as
                                            libc::c_uint).wrapping_sub(1i32 as
                                                                           libc::c_uint)
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
