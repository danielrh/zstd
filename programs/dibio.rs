#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(libc)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
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
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t,
              __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    /* relies on standard C (note : clock_t measurements can be wrong when using multi-threading) */
    #[no_mangle]
    fn UTIL_getTime() -> UTIL_time_t;
    /* returns time span in microseconds */
    #[no_mangle]
    fn UTIL_clockSpanMicro(clockStart: UTIL_time_t) -> U64;
    #[no_mangle]
    fn UTIL_getFileSize(infilename: *const libc::c_char) -> U64;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn ZDICT_isError(errorCode: size_t) -> libc::c_uint;
    #[no_mangle]
    fn ZDICT_getErrorName(errorCode: size_t) -> *const libc::c_char;
    /* ! ZDICT_trainFromBuffer_cover():
 *  Train a dictionary from an array of samples using the COVER algorithm.
 *  Samples must be stored concatenated in a single flat buffer `samplesBuffer`,
 *  supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order.
 *  The resulting dictionary will be saved into `dictBuffer`.
 * @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)
 *          or an error code, which can be tested with ZDICT_isError().
 *  Note: ZDICT_trainFromBuffer_cover() requires about 9 bytes of memory for each input byte.
 *  Tips: In general, a reasonable dictionary has a size of ~ 100 KB.
 *        It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`.
 *        In general, it's recommended to provide a few thousands samples, though this can vary a lot.
 *        It's recommended that total size of all samples be about ~x100 times the target size of dictionary.
 */
    #[no_mangle]
    fn ZDICT_trainFromBuffer_cover(dictBuffer: *mut libc::c_void,
                                   dictBufferCapacity: size_t,
                                   samplesBuffer: *const libc::c_void,
                                   samplesSizes: *const size_t,
                                   nbSamples: libc::c_uint,
                                   parameters: ZDICT_cover_params_t)
     -> size_t;
    /* ! ZDICT_optimizeTrainFromBuffer_cover():
 * The same requirements as above hold for all the parameters except `parameters`.
 * This function tries many parameter combinations and picks the best parameters.
 * `*parameters` is filled with the best parameters found,
 * dictionary constructed with those parameters is stored in `dictBuffer`.
 *
 * All of the parameters d, k, steps are optional.
 * If d is non-zero then we don't check multiple values of d, otherwise we check d = {6, 8}.
 * if steps is zero it defaults to its default value.
 * If k is non-zero then we don't check multiple values of k, otherwise we check steps values in [50, 2000].
 *
 * @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)
 *           or an error code, which can be tested with ZDICT_isError().
 *           On success `*parameters` contains the parameters selected.
 * Note: ZDICT_optimizeTrainFromBuffer_cover() requires about 8 bytes of memory for each input byte and additionally another 5 bytes of memory for each byte of memory for each thread.
 */
    #[no_mangle]
    fn ZDICT_optimizeTrainFromBuffer_cover(dictBuffer: *mut libc::c_void,
                                           dictBufferCapacity: size_t,
                                           samplesBuffer: *const libc::c_void,
                                           samplesSizes: *const size_t,
                                           nbSamples: libc::c_uint,
                                           parameters:
                                               *mut ZDICT_cover_params_t)
     -> size_t;
    /* ! ZDICT_trainFromBuffer_fastCover():
 *  Train a dictionary from an array of samples using a modified version of COVER algorithm.
 *  Samples must be stored concatenated in a single flat buffer `samplesBuffer`,
 *  supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order.
 *  d and k are required.
 *  All other parameters are optional, will use default values if not provided
 *  The resulting dictionary will be saved into `dictBuffer`.
 * @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)
 *          or an error code, which can be tested with ZDICT_isError().
 *  Note: ZDICT_trainFromBuffer_fastCover() requires about 1 bytes of memory for each input byte and additionally another 6 * 2^f bytes of memory .
 *  Tips: In general, a reasonable dictionary has a size of ~ 100 KB.
 *        It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`.
 *        In general, it's recommended to provide a few thousands samples, though this can vary a lot.
 *        It's recommended that total size of all samples be about ~x100 times the target size of dictionary.
 */
    #[no_mangle]
    fn ZDICT_trainFromBuffer_fastCover(dictBuffer: *mut libc::c_void,
                                       dictBufferCapacity: size_t,
                                       samplesBuffer: *const libc::c_void,
                                       samplesSizes: *const size_t,
                                       nbSamples: libc::c_uint,
                                       parameters: ZDICT_fastCover_params_t)
     -> size_t;
    /* ! ZDICT_optimizeTrainFromBuffer_fastCover():
 * The same requirements as above hold for all the parameters except `parameters`.
 * This function tries many parameter combinations (specifically, k and d combinations)
 * and picks the best parameters. `*parameters` is filled with the best parameters found,
 * dictionary constructed with those parameters is stored in `dictBuffer`.
 * All of the parameters d, k, steps, f, and accel are optional.
 * If d is non-zero then we don't check multiple values of d, otherwise we check d = {6, 8}.
 * if steps is zero it defaults to its default value.
 * If k is non-zero then we don't check multiple values of k, otherwise we check steps values in [50, 2000].
 * If f is zero, default value of 20 is used.
 * If accel is zero, default value of 1 is used.
 *
 * @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)
 *           or an error code, which can be tested with ZDICT_isError().
 *           On success `*parameters` contains the parameters selected.
 * Note: ZDICT_optimizeTrainFromBuffer_fastCover() requires about 1 byte of memory for each input byte and additionally another 6 * 2^f bytes of memory for each thread.
 */
    #[no_mangle]
    fn ZDICT_optimizeTrainFromBuffer_fastCover(dictBuffer: *mut libc::c_void,
                                               dictBufferCapacity: size_t,
                                               samplesBuffer:
                                                   *const libc::c_void,
                                               samplesSizes: *const size_t,
                                               nbSamples: libc::c_uint,
                                               parameters:
                                                   *mut ZDICT_fastCover_params_t)
     -> size_t;
    /* ! ZDICT_trainFromBuffer_unsafe_legacy() :
    Strictly Internal use only !!
    Same as ZDICT_trainFromBuffer_legacy(), but does not control `samplesBuffer`.
    `samplesBuffer` must be followed by noisy guard band to avoid out-of-buffer reads.
    @return : size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)
              or an error code.
*/
    #[no_mangle]
    fn ZDICT_trainFromBuffer_unsafe_legacy(dictBuffer: *mut libc::c_void,
                                           dictBufferCapacity: size_t,
                                           samplesBuffer: *const libc::c_void,
                                           samplesSizes: *const size_t,
                                           nbSamples: libc::c_uint,
                                           parameters: ZDICT_legacy_params_t)
     -> size_t;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type UTIL_time_t = timespec;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct fileStats {
    pub totalSizeToLoad: U64,
    pub oneSampleTooLarge: libc::c_uint,
    pub nbSamples: libc::c_uint,
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
/* This library is designed for a single-threaded console application.
*  It exit() and printf() into stderr when it encounters an error condition. */
/*-*************************************
*  Dependencies
***************************************/
/*-*************************************
*  Public functions
***************************************/
/*! DiB_trainFromFiles() :
    Train a dictionary from a set of files provided by `fileNamesTable`.
    Resulting dictionary is written into file `dictFileName`.
    `parameters` is optional and can be provided with values set to 0, meaning "default".
    @return : 0 == ok. Any other : error.
*/
#[no_mangle]
pub unsafe extern "C" fn DiB_trainFromFiles(mut dictFileName:
                                                *const libc::c_char,
                                            mut maxDictSize: libc::c_uint,
                                            mut fileNamesTable:
                                                *mut *const libc::c_char,
                                            mut nbFiles: libc::c_uint,
                                            mut chunkSize: size_t,
                                            mut params:
                                                *mut ZDICT_legacy_params_t,
                                            mut coverParams:
                                                *mut ZDICT_cover_params_t,
                                            mut fastCoverParams:
                                                *mut ZDICT_fastCover_params_t,
                                            mut optimize: libc::c_int)
 -> libc::c_int {
    let displayLevel: libc::c_uint =
        if !params.is_null() {
            (*params).zParams.notificationLevel
        } else if !coverParams.is_null() {
            (*coverParams).zParams.notificationLevel
        } else if !fastCoverParams.is_null() {
            (*fastCoverParams).zParams.notificationLevel
        } else { 0i32 as libc::c_uint };
    /* should never happen */
    let dictBuffer: *mut libc::c_void = malloc(maxDictSize as libc::c_ulong);
    let fs: fileStats =
        DiB_fileStats(fileNamesTable, nbFiles, chunkSize, displayLevel);
    let sampleSizes: *mut size_t =
        malloc((fs.nbSamples as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>()
                                                    as libc::c_ulong)) as
            *mut size_t;
    let memMult: size_t =
        (if !params.is_null() {
             11i32
         } else if !coverParams.is_null() { 9i32 } else { 1i32 }) as size_t;
    let maxMem: size_t =
        DiB_findMaxMem(fs.totalSizeToLoad.wrapping_mul(memMult) as
                           libc::c_ulonglong).wrapping_div(memMult);
    let mut loadedSize: size_t =
        (if (maxMem as libc::c_ulonglong) <
                fs.totalSizeToLoad as libc::c_ulonglong {
             maxMem as libc::c_ulonglong
         } else { fs.totalSizeToLoad as libc::c_ulonglong }) as size_t;
    let srcBuffer: *mut libc::c_void =
        malloc(loadedSize.wrapping_add(32i32 as libc::c_ulong));
    let mut result: libc::c_int = 0i32;
    if sampleSizes.is_null() || srcBuffer.is_null() || dictBuffer.is_null() {
        fprintf(stderr,
                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                12i32);
        fprintf(stderr,
                b"not enough memory for DiB_trainFiles\x00" as *const u8 as
                    *const libc::c_char);
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        exit(12i32);
    }
    if 0 != fs.oneSampleTooLarge {
        if displayLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    b"!  Warning : some sample(s) are very large \n\x00" as
                        *const u8 as *const libc::c_char);
        }
        if displayLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    b"!  Note that dictionary is only useful for small samples. \n\x00"
                        as *const u8 as *const libc::c_char);
        }
        if displayLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    b"!  As a consequence, only the first %u bytes of each sample are loaded \n\x00"
                        as *const u8 as *const libc::c_char,
                    128i32 * (1i32 << 10i32));
        }
    }
    if fs.nbSamples < 5i32 as libc::c_uint {
        if displayLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    b"!  Warning : nb of samples too low for proper processing ! \n\x00"
                        as *const u8 as *const libc::c_char);
        }
        if displayLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    b"!  Please provide _one file per sample_. \n\x00" as
                        *const u8 as *const libc::c_char);
        }
        if displayLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    b"!  Alternatively, split files into fixed-size blocks representative of samples, with -B# \n\x00"
                        as *const u8 as *const libc::c_char);
        }
        fprintf(stderr,
                b"Error %i : \x00" as *const u8 as *const libc::c_char,
                14i32);
        fprintf(stderr,
                b"nb of samples too low\x00" as *const u8 as
                    *const libc::c_char);
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        exit(14i32);
    }
    if (fs.totalSizeToLoad as libc::c_ulonglong) <
           (8i32 as libc::c_uint).wrapping_mul(maxDictSize) as
               libc::c_ulonglong {
        if displayLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    b"!  Warning : data size of samples too small for target dictionary size \n\x00"
                        as *const u8 as *const libc::c_char);
        }
        if displayLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    b"!  Samples should be about 100x larger than target dictionary size \n\x00"
                        as *const u8 as *const libc::c_char);
        }
    }
    if loadedSize < fs.totalSizeToLoad {
        if displayLevel >= 1i32 as libc::c_uint {
            fprintf(stderr,
                    b"Not enough memory; training on %u MB only...\n\x00" as
                        *const u8 as *const libc::c_char,
                    (loadedSize >> 20i32) as libc::c_uint);
        }
    }
    if displayLevel >= 3i32 as libc::c_uint {
        fprintf(stderr,
                b"Shuffling input files\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    DiB_shuffle(fileNamesTable, nbFiles);
    DiB_loadFiles(srcBuffer, &mut loadedSize, sampleSizes, fs.nbSamples,
                  fileNamesTable, nbFiles, chunkSize, displayLevel);
    let mut dictSize: size_t = 0;
    if !params.is_null() {
        DiB_fillNoise((srcBuffer as
                           *mut libc::c_char).offset(loadedSize as isize) as
                          *mut libc::c_void, 32i32 as size_t);
        dictSize =
            ZDICT_trainFromBuffer_unsafe_legacy(dictBuffer,
                                                maxDictSize as size_t,
                                                srcBuffer, sampleSizes,
                                                fs.nbSamples, *params)
    } else if !coverParams.is_null() {
        if 0 != optimize {
            dictSize =
                ZDICT_optimizeTrainFromBuffer_cover(dictBuffer,
                                                    maxDictSize as size_t,
                                                    srcBuffer, sampleSizes,
                                                    fs.nbSamples,
                                                    coverParams);
            if 0 == ZDICT_isError(dictSize) {
                let mut splitPercentage: libc::c_uint =
                    ((*coverParams).splitPoint * 100i32 as libc::c_double) as
                        libc::c_uint;
                if displayLevel >= 2i32 as libc::c_uint {
                    fprintf(stderr,
                            b"k=%u\nd=%u\nsteps=%u\nsplit=%u\n\x00" as
                                *const u8 as *const libc::c_char,
                            (*coverParams).k, (*coverParams).d,
                            (*coverParams).steps, splitPercentage);
                }
            }
        } else {
            dictSize =
                ZDICT_trainFromBuffer_cover(dictBuffer, maxDictSize as size_t,
                                            srcBuffer, sampleSizes,
                                            fs.nbSamples, *coverParams)
        }
    } else {
        if !fastCoverParams.is_null() {
        } else {
            __assert_fail(b"fastCoverParams != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"/home/danielrh/dev/zstd-c2rust/programs/dibio.c\x00"
                              as *const u8 as *const libc::c_char,
                          339i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 170],
                                                    &[libc::c_char; 170]>(b"int DiB_trainFromFiles(const char *, unsigned int, const char **, unsigned int, size_t, ZDICT_legacy_params_t *, ZDICT_cover_params_t *, ZDICT_fastCover_params_t *, int)\x00")).as_ptr());
        };
        if 0 != optimize {
            dictSize =
                ZDICT_optimizeTrainFromBuffer_fastCover(dictBuffer,
                                                        maxDictSize as size_t,
                                                        srcBuffer,
                                                        sampleSizes,
                                                        fs.nbSamples,
                                                        fastCoverParams);
            if 0 == ZDICT_isError(dictSize) {
                let mut splitPercentage_0: libc::c_uint =
                    ((*fastCoverParams).splitPoint * 100i32 as libc::c_double)
                        as libc::c_uint;
                if displayLevel >= 2i32 as libc::c_uint {
                    fprintf(stderr,
                            b"k=%u\nd=%u\nf=%u\nsteps=%u\nsplit=%u\naccel=%u\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*fastCoverParams).k, (*fastCoverParams).d,
                            (*fastCoverParams).f, (*fastCoverParams).steps,
                            splitPercentage_0, (*fastCoverParams).accel);
                }
            }
        } else {
            dictSize =
                ZDICT_trainFromBuffer_fastCover(dictBuffer,
                                                maxDictSize as size_t,
                                                srcBuffer, sampleSizes,
                                                fs.nbSamples,
                                                *fastCoverParams)
        }
    }
    if 0 != ZDICT_isError(dictSize) {
        if displayLevel >= 1i32 as libc::c_uint {
            fprintf(stderr,
                    b"dictionary training failed : %s \n\x00" as *const u8 as
                        *const libc::c_char, ZDICT_getErrorName(dictSize));
        }
        result = 1i32
    } else {
        if displayLevel >= 2i32 as libc::c_uint {
            fprintf(stderr,
                    b"Save dictionary of size %u into file %s \n\x00" as
                        *const u8 as *const libc::c_char,
                    dictSize as libc::c_uint, dictFileName);
        }
        DiB_saveDict(dictFileName, dictBuffer, dictSize);
    }
    /* clean up */
    free(srcBuffer);
    free(sampleSizes as *mut libc::c_void);
    free(dictBuffer);
    return result;
}
/* ! DiB_fileStats() :
 *  Given a list of files, and a chunkSize (0 == no chunk, whole files)
 *  provides the amount of data to be loaded and the resulting nb of samples.
 *  This is useful primarily for allocation purpose => sample buffer, and sample sizes table.
 */
unsafe extern "C" fn DiB_fileStats(mut fileNamesTable:
                                       *mut *const libc::c_char,
                                   mut nbFiles: libc::c_uint,
                                   mut chunkSize: size_t,
                                   mut displayLevel: libc::c_uint)
 -> fileStats {
    let mut fs: fileStats =
        fileStats{totalSizeToLoad: 0, oneSampleTooLarge: 0, nbSamples: 0,};
    let mut n: libc::c_uint = 0;
    memset(&mut fs as *mut fileStats as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<fileStats>() as libc::c_ulong);
    n = 0i32 as libc::c_uint;
    while n < nbFiles {
        let fileSize: U64 =
            UTIL_getFileSize(*fileNamesTable.offset(n as isize));
        let srcSize: U64 =
            if fileSize == -1i32 as U64 {
                0i32 as libc::c_ulong
            } else { fileSize };
        let nbSamples: U32 =
            (if 0 != chunkSize {
                 srcSize.wrapping_add(chunkSize.wrapping_sub(1i32 as
                                                                 libc::c_ulong)).wrapping_div(chunkSize)
             } else { 1i32 as libc::c_ulong }) as U32;
        let chunkToLoad: U64 =
            if 0 != chunkSize {
                if chunkSize < srcSize { chunkSize } else { srcSize }
            } else { srcSize };
        let cappedChunkSize: size_t =
            if chunkToLoad < (128i32 * (1i32 << 10i32)) as libc::c_ulong {
                chunkToLoad
            } else { (128i32 * (1i32 << 10i32)) as libc::c_ulong };
        fs.totalSizeToLoad =
            (fs.totalSizeToLoad as
                 libc::c_ulong).wrapping_add(cappedChunkSize.wrapping_mul(nbSamples
                                                                              as
                                                                              libc::c_ulong))
                as U64 as U64;
        fs.oneSampleTooLarge |=
            (chunkSize > (2i32 * (128i32 * (1i32 << 10i32))) as libc::c_ulong)
                as libc::c_int as libc::c_uint;
        fs.nbSamples = fs.nbSamples.wrapping_add(nbSamples);
        n = n.wrapping_add(1)
    }
    if displayLevel >= 4i32 as libc::c_uint {
        fprintf(stderr,
                b"Preparing to load : %u KB \n\x00" as *const u8 as
                    *const libc::c_char,
                (fs.totalSizeToLoad >> 10i32) as libc::c_uint);
    }
    return fs;
}
/*-********************************************************
*  Dictionary training functions
**********************************************************/
unsafe extern "C" fn DiB_findMaxMem(mut requiredMem: libc::c_ulonglong)
 -> size_t {
    let mut g_maxMemory: size_t =
        if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
               4i32 as libc::c_ulong {
            (2i32 as
                 libc::c_uint).wrapping_mul(1u32 <<
                                                30i32).wrapping_sub((64i32 *
                                                                         (1i32
                                                                              <<
                                                                              20i32))
                                                                        as
                                                                        libc::c_uint)
                as libc::c_ulong
        } else {
            ((512i32 * (1i32 << 20i32)) as size_t) <<
                ::std::mem::size_of::<size_t>() as libc::c_ulong
        };
    let step: size_t = (8i32 * (1i32 << 20i32)) as size_t;
    let mut testmem: *mut libc::c_void = 0 as *mut libc::c_void;
    requiredMem =
        (requiredMem >> 23i32).wrapping_add(1i32 as libc::c_ulonglong) <<
            23i32;
    requiredMem = requiredMem.wrapping_add(step as libc::c_ulonglong);
    if requiredMem > g_maxMemory as libc::c_ulonglong {
        requiredMem = g_maxMemory as libc::c_ulonglong
    }
    while testmem.is_null() {
        testmem = malloc(requiredMem as size_t);
        requiredMem = requiredMem.wrapping_sub(step as libc::c_ulonglong)
    }
    free(testmem);
    return requiredMem as size_t;
}
unsafe extern "C" fn DiB_saveDict(mut dictFileName: *const libc::c_char,
                                  mut buff: *const libc::c_void,
                                  mut buffSize: size_t) {
    let f: *mut FILE =
        fopen(dictFileName, b"wb\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        fprintf(stderr,
                b"Error %i : \x00" as *const u8 as *const libc::c_char, 3i32);
        fprintf(stderr,
                b"cannot open %s \x00" as *const u8 as *const libc::c_char,
                dictFileName);
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        exit(3i32);
    }
    let n: size_t = fwrite(buff, 1i32 as size_t, buffSize, f);
    if n != buffSize {
        fprintf(stderr,
                b"Error %i : \x00" as *const u8 as *const libc::c_char, 4i32);
        fprintf(stderr,
                b"%s : write error\x00" as *const u8 as *const libc::c_char,
                dictFileName);
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        exit(4i32);
    }
    let n_0: size_t = fclose(f) as size_t;
    if n_0 != 0i32 as libc::c_ulong {
        fprintf(stderr,
                b"Error %i : \x00" as *const u8 as *const libc::c_char, 5i32);
        fprintf(stderr,
                b"%s : flush error\x00" as *const u8 as *const libc::c_char,
                dictFileName);
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        exit(5i32);
    };
}
unsafe extern "C" fn DiB_fillNoise(mut buffer: *mut libc::c_void,
                                   mut length: size_t) {
    let prime1: libc::c_uint = 2654435761u32;
    let prime2: libc::c_uint = 2246822519u32;
    let mut acc: libc::c_uint = prime1;
    let mut p: size_t = 0i32 as size_t;
    p = 0i32 as size_t;
    while p < length {
        acc = acc.wrapping_mul(prime2);
        *(buffer as *mut libc::c_uchar).offset(p as isize) =
            (acc >> 21i32) as libc::c_uchar;
        p = p.wrapping_add(1)
    };
}
/*-*************************************
*  Exceptions
***************************************/
/* ********************************************************
*  Helper functions
**********************************************************/
/* ********************************************************
*  File related operations
**********************************************************/
/** DiB_loadFiles() :
 *  load samples from files listed in fileNamesTable into buffer.
 *  works even if buffer is too small to load all samples.
 *  Also provides the size of each sample into sampleSizes table
 *  which must be sized correctly, using DiB_fileStats().
 * @return : nb of samples effectively loaded into `buffer`
 * *bufferSizePtr is modified, it provides the amount data loaded within buffer.
 *  sampleSizes is filled with the size of each sample.
 */
unsafe extern "C" fn DiB_loadFiles(mut buffer: *mut libc::c_void,
                                   mut bufferSizePtr: *mut size_t,
                                   mut sampleSizes: *mut size_t,
                                   mut sstSize: libc::c_uint,
                                   mut fileNamesTable:
                                       *mut *const libc::c_char,
                                   mut nbFiles: libc::c_uint,
                                   mut targetChunkSize: size_t,
                                   mut displayLevel: libc::c_uint)
 -> libc::c_uint {
    let buff: *mut libc::c_char = buffer as *mut libc::c_char;
    let mut pos: size_t = 0i32 as size_t;
    let mut nbLoadedChunks: libc::c_uint = 0i32 as libc::c_uint;
    let mut fileIndex: libc::c_uint = 0;
    fileIndex = 0i32 as libc::c_uint;
    while fileIndex < nbFiles {
        let fileName: *const libc::c_char =
            *fileNamesTable.offset(fileIndex as isize);
        let fs64: libc::c_ulonglong =
            UTIL_getFileSize(fileName) as libc::c_ulonglong;
        let mut remainingToLoad: libc::c_ulonglong =
            if fs64 == -1i32 as U64 as libc::c_ulonglong {
                0i32 as libc::c_ulonglong
            } else { fs64 };
        let nbChunks: U32 =
            if 0 != targetChunkSize {
                fs64.wrapping_add(targetChunkSize.wrapping_sub(1i32 as
                                                                   libc::c_ulong)
                                      as
                                      libc::c_ulonglong).wrapping_div(targetChunkSize
                                                                          as
                                                                          libc::c_ulonglong)
                    as U32
            } else { 1i32 as libc::c_uint };
        let chunkSize: U64 =
            (if 0 != targetChunkSize {
                 if (targetChunkSize as libc::c_ulonglong) < fs64 {
                     targetChunkSize as libc::c_ulonglong
                 } else { fs64 }
             } else { fs64 }) as U64;
        let maxChunkSize: size_t =
            if chunkSize < (128i32 * (1i32 << 10i32)) as libc::c_ulong {
                chunkSize
            } else { (128i32 * (1i32 << 10i32)) as libc::c_ulong };
        let mut cnb: U32 = 0;
        let f: *mut FILE =
            fopen(fileName, b"rb\x00" as *const u8 as *const libc::c_char);
        if f.is_null() {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    10i32);
            fprintf(stderr,
                    b"zstd: dictBuilder: %s %s \x00" as *const u8 as
                        *const libc::c_char, fileName,
                    strerror(*__errno_location()));
            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
            exit(10i32);
        }
        if displayLevel >= 2i32 as libc::c_uint {
            if UTIL_clockSpanMicro(g_displayClock) > g_refreshRate ||
                   displayLevel >= 4i32 as libc::c_uint {
                g_displayClock = UTIL_getTime();
                fprintf(stderr,
                        b"Loading %s...       \r\x00" as *const u8 as
                            *const libc::c_char, fileName);
                if displayLevel >= 4i32 as libc::c_uint { fflush(stderr); }
            }
        }
        cnb = 0i32 as U32;
        while cnb < nbChunks {
            let toLoad: size_t =
                (if (maxChunkSize as libc::c_ulonglong) < remainingToLoad {
                     maxChunkSize as libc::c_ulonglong
                 } else { remainingToLoad }) as size_t;
            if toLoad > (*bufferSizePtr).wrapping_sub(pos) { break ; }
            let readSize: size_t =
                fread(buff.offset(pos as isize) as *mut libc::c_void,
                      1i32 as size_t, toLoad, f);
            if readSize != toLoad {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 11i32);
                fprintf(stderr,
                        b"Pb reading %s\x00" as *const u8 as
                            *const libc::c_char, fileName);
                fprintf(stderr,
                        b"\n\x00" as *const u8 as *const libc::c_char);
                exit(11i32);
            }
            pos =
                (pos as libc::c_ulong).wrapping_add(readSize) as size_t as
                    size_t;
            let fresh0 = nbLoadedChunks;
            nbLoadedChunks = nbLoadedChunks.wrapping_add(1);
            *sampleSizes.offset(fresh0 as isize) = toLoad;
            remainingToLoad =
                remainingToLoad.wrapping_sub(targetChunkSize as
                                                 libc::c_ulonglong);
            if nbLoadedChunks == sstSize {
                /* no more space left in sampleSizes table */
                fileIndex = nbFiles;
                break ;
            } else {
                if toLoad < targetChunkSize {
                    fseek(f,
                          targetChunkSize.wrapping_sub(toLoad) as
                              libc::c_long, 1i32);
                }
                cnb = cnb.wrapping_add(1)
            }
        }
        fclose(f);
        fileIndex = fileIndex.wrapping_add(1)
    }
    if displayLevel >= 2i32 as libc::c_uint {
        fprintf(stderr, b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char);
    }
    *bufferSizePtr = pos;
    if displayLevel >= 4i32 as libc::c_uint {
        fprintf(stderr,
                b"loaded : %u KB \n\x00" as *const u8 as *const libc::c_char,
                (pos >> 10i32) as libc::c_uint);
    }
    return nbLoadedChunks;
}
static mut g_displayClock: UTIL_time_t =
    timespec{tv_sec: 0i32 as __time_t, tv_nsec: 0i32 as __syscall_slong_t,};
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* **************************************
*  Compiler Warnings
****************************************/
/*-*************************************
*  Includes
***************************************/
/* malloc, free */
/* memset */
/* fprintf, fopen, ftello64 */
/* errno */
/* read */
/*-*************************************
*  Constants
***************************************/
/* rough estimation : memory cost to analyze 1 byte of sample */
/* rough estimation : memory cost to analyze 1 byte of sample */
/* rough estimation : memory cost to analyze 1 byte of sample */
/*-*************************************
*  Console display
***************************************/
static mut g_refreshRate: U64 = (1000000i32 / 6i32) as U64;
/* DiB_shuffle() :
 * shuffle a table of file names in a semi-random way
 * It improves dictionary quality by reducing "locality" impact, so if sample set is very large,
 * it will load random elements from it, instead of just the first ones. */
unsafe extern "C" fn DiB_shuffle(mut fileNamesTable: *mut *const libc::c_char,
                                 mut nbFiles: libc::c_uint) {
    let mut seed: U32 = 0xfd2fb528u32;
    let mut i: libc::c_uint = 0;
    if nbFiles >= 1i32 as libc::c_uint {
    } else {
        __assert_fail(b"nbFiles >= 1\x00" as *const u8 as *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/dibio.c\x00"
                          as *const u8 as *const libc::c_char,
                      165i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 46],
                                                &[libc::c_char; 46]>(b"void DiB_shuffle(const char **, unsigned int)\x00")).as_ptr());
    };
    i = nbFiles.wrapping_sub(1i32 as libc::c_uint);
    while i > 0i32 as libc::c_uint {
        let j: libc::c_uint =
            DiB_rand(&mut seed).wrapping_rem(i.wrapping_add(1i32 as
                                                                libc::c_uint));
        let tmp: *const libc::c_char = *fileNamesTable.offset(j as isize);
        let ref mut fresh1 = *fileNamesTable.offset(j as isize);
        *fresh1 = *fileNamesTable.offset(i as isize);
        let ref mut fresh2 = *fileNamesTable.offset(i as isize);
        *fresh2 = tmp;
        i = i.wrapping_sub(1)
    };
}
unsafe extern "C" fn DiB_rand(mut src: *mut U32) -> U32 {
    static mut prime1: U32 = 2654435761u32;
    static mut prime2: U32 = 2246822519u32;
    let mut rand32: U32 = *src;
    rand32 = (rand32 as libc::c_uint).wrapping_mul(prime1) as U32 as U32;
    rand32 ^= prime2;
    rand32 = rand32 << 13i32 | rand32 >> 32i32 - 13i32;
    *src = rand32;
    return rand32 >> 5i32;
}