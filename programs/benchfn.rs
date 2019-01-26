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
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* relies on standard C (note : clock_t measurements can be wrong when using multi-threading) */
    #[no_mangle]
    fn UTIL_getTime() -> UTIL_time_t;
    /* returns time span in microseconds */
    #[no_mangle]
    fn UTIL_clockSpanMicro(clockStart: UTIL_time_t) -> U64;
    /* returns time span in microseconds */
    #[no_mangle]
    fn UTIL_clockSpanNano(clockStart: UTIL_time_t) -> U64;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
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
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* benchfn :
 * benchmark any function on a set of input
 * providing result in nanoSecPerRun
 * or detecting and returning an error
 */
/* ===  Dependencies  === */
/* size_t */
/* ====  Benchmark any function, iterated on a set of blocks  ==== */
/* BMK_runTime_t: valid result return type */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BMK_runTime_t {
    pub nanoSecPerRun: libc::c_ulonglong,
    pub sumOfReturn: size_t,
}
/* BMK_runOutcome_t:
 * type expressing the outcome of a benchmark run by BMK_benchFunction(),
 * which can be either valid or invalid.
 * benchmark outcome can be invalid if errorFn is provided.
 * BMK_runOutcome_t must be considered "opaque" : never access its members directly.
 * Instead, use its assigned methods :
 * BMK_isSuccessful_runOutcome, BMK_extract_runTime, BMK_extract_errorResult.
 * The structure is only described here to allow its allocation on stack. */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BMK_runOutcome_t {
    pub internal_never_ever_use_directly: BMK_runTime_t,
    pub error_result_never_ever_use_directly: size_t,
    pub error_tag_never_ever_use_directly: libc::c_int,
}
/* prototypes for benchmarked functions */
pub type BMK_benchFn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: size_t,
                                _: *mut libc::c_void, _: size_t,
                                _: *mut libc::c_void) -> size_t>;
pub type BMK_initFn_t
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> size_t>;
pub type BMK_errorFn_t
    =
    Option<unsafe extern "C" fn(_: size_t) -> libc::c_uint>;
/* BMK_benchFunction() parameters are provided through following structure.
 * This is preferable for readability,
 * as the number of parameters required is pretty large.
 * No initializer is provided, because it doesn't make sense to provide some "default" :
 * all parameters should be specified by the caller */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BMK_benchParams_t {
    pub benchFn: BMK_benchFn_t,
    pub benchPayload: *mut libc::c_void,
    pub initFn: BMK_initFn_t,
    pub initPayload: *mut libc::c_void,
    pub errorFn: BMK_errorFn_t,
    pub blockCount: size_t,
    pub srcBuffers: *const *const libc::c_void,
    pub srcSizes: *const size_t,
    pub dstBuffers: *const *mut libc::c_void,
    pub dstCapacities: *const size_t,
    pub blockResults: *mut size_t,
}
/* ====  Benchmarking any function, providing intermediate results  ==== */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct BMK_timedFnState_s {
    pub timeSpent_ns: U64,
    pub timeBudget_ns: U64,
    pub runBudget_ns: U64,
    pub fastestRun: BMK_runTime_t,
    pub nbLoops: libc::c_uint,
    pub coolTime: UTIL_time_t,
}
/* ====  Benchmark any function, returning intermediate results  ==== */
/* state information tracking benchmark session */
pub type BMK_timedFnState_t = BMK_timedFnState_s;
/* BMK_benchFunction() :
 * This function benchmarks benchFn and initFn, providing a result.
 *
 * params : see description of BMK_benchParams_t above.
 * nbLoops: defines number of times benchFn is run over the full set of blocks.
 *          Minimum value is 1. A 0 is interpreted as a 1.
 *
 * @return: can express either an error or a successful result.
 *          Use BMK_isSuccessful_runOutcome() to check if benchmark was successful.
 *          If yes, extract the result with BMK_extract_runTime(),
 *          it will contain :
 *              .sumOfReturn : the sum of all return values of benchFn through all of blocks
 *              .nanoSecPerRun : time per run of benchFn + (time for initFn / nbLoops)
 *          .sumOfReturn is generally intended for functions which return a # of bytes written into dstBuffer,
 *              in which case, this value will be the total amount of bytes written into dstBuffer.
 *
 * blockResults : when provided (!= NULL), and when benchmark is successful,
 *                params.blockResults contains all return values of `benchFn` over all blocks.
 *                when provided (!= NULL), and when benchmark failed,
 *                params.blockResults contains return values of `benchFn` over all blocks preceding and including the failed block.
 */
#[no_mangle]
pub unsafe extern "C" fn BMK_benchFunction(mut p: BMK_benchParams_t,
                                           mut nbLoops: libc::c_uint)
 -> BMK_runOutcome_t {
    let mut dstSize: size_t = 0i32 as size_t;
    nbLoops =
        nbLoops.wrapping_add((0 == nbLoops) as libc::c_int as libc::c_uint);
    let mut i: size_t = 0;
    i = 0i32 as size_t;
    while i < p.blockCount {
        memset(*p.dstBuffers.offset(i as isize), 0xe5i32,
               *p.dstCapacities.offset(i as isize));
        i = i.wrapping_add(1)
    }
    let clockStart: UTIL_time_t = UTIL_getTime();
    let mut loopNb: libc::c_uint = 0;
    let mut blockNb: libc::c_uint = 0;
    if p.initFn.is_some() {
        p.initFn.expect("non-null function pointer")(p.initPayload);
    }
    loopNb = 0i32 as libc::c_uint;
    while loopNb < nbLoops {
        blockNb = 0i32 as libc::c_uint;
        while (blockNb as libc::c_ulong) < p.blockCount {
            let res: size_t =
                p.benchFn.expect("non-null function pointer")(*p.srcBuffers.offset(blockNb
                                                                                       as
                                                                                       isize),
                                                              *p.srcSizes.offset(blockNb
                                                                                     as
                                                                                     isize),
                                                              *p.dstBuffers.offset(blockNb
                                                                                       as
                                                                                       isize),
                                                              *p.dstCapacities.offset(blockNb
                                                                                          as
                                                                                          isize),
                                                              p.benchPayload);
            if loopNb == 0i32 as libc::c_uint {
                if !p.blockResults.is_null() {
                    *p.blockResults.offset(blockNb as isize) = res
                }
                if p.errorFn.is_some() &&
                       0 != p.errorFn.expect("non-null function pointer")(res)
                   {
                    return BMK_runOutcome_error(res)
                }
                dstSize =
                    (dstSize as libc::c_ulong).wrapping_add(res) as size_t as
                        size_t
            }
            blockNb = blockNb.wrapping_add(1)
        }
        loopNb = loopNb.wrapping_add(1)
    }
    let totalTime: U64 = UTIL_clockSpanNano(clockStart);
    let mut rt: BMK_runTime_t =
        BMK_runTime_t{nanoSecPerRun: 0, sumOfReturn: 0,};
    rt.nanoSecPerRun =
        totalTime.wrapping_div(nbLoops as libc::c_ulong) as libc::c_ulonglong;
    rt.sumOfReturn = dstSize;
    return BMK_setValid_runTime(rt);
}
unsafe extern "C" fn BMK_setValid_runTime(mut runTime: BMK_runTime_t)
 -> BMK_runOutcome_t {
    let mut outcome: BMK_runOutcome_t =
        BMK_runOutcome_t{internal_never_ever_use_directly:
                             BMK_runTime_t{nanoSecPerRun: 0, sumOfReturn: 0,},
                         error_result_never_ever_use_directly: 0,
                         error_tag_never_ever_use_directly: 0,};
    outcome.error_tag_never_ever_use_directly = 0i32;
    outcome.internal_never_ever_use_directly = runTime;
    return outcome;
}
unsafe extern "C" fn BMK_runOutcome_error(mut errorResult: size_t)
 -> BMK_runOutcome_t {
    let mut b: BMK_runOutcome_t =
        BMK_runOutcome_t{internal_never_ever_use_directly:
                             BMK_runTime_t{nanoSecPerRun: 0, sumOfReturn: 0,},
                         error_result_never_ever_use_directly: 0,
                         error_tag_never_ever_use_directly: 0,};
    memset(&mut b as *mut BMK_runOutcome_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<BMK_runOutcome_t>() as libc::c_ulong);
    b.error_tag_never_ever_use_directly = 1i32;
    b.error_result_never_ever_use_directly = errorResult;
    return b;
}
/* check first if the benchmark was successful or not */
#[no_mangle]
pub unsafe extern "C" fn BMK_isSuccessful_runOutcome(mut outcome:
                                                         BMK_runOutcome_t)
 -> libc::c_int {
    return (outcome.error_tag_never_ever_use_directly == 0i32) as libc::c_int;
}
/* If the benchmark was successful, extract the result.
 * note : this function will abort() program execution if benchmark failed !
 *        always check if benchmark was successful first !
 */
#[no_mangle]
pub unsafe extern "C" fn BMK_extract_runTime(mut outcome: BMK_runOutcome_t)
 -> BMK_runTime_t {
    if outcome.error_tag_never_ever_use_directly == 0i32 {
    } else {
        __assert_fail(b"outcome.error_tag_never_ever_use_directly == 0\x00" as
                          *const u8 as *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/benchfn.c\x00"
                          as *const u8 as *const libc::c_char,
                      74i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 52],
                                                &[libc::c_char; 52]>(b"BMK_runTime_t BMK_extract_runTime(BMK_runOutcome_t)\x00")).as_ptr());
    };
    return outcome.internal_never_ever_use_directly;
}
/* when benchmark failed, it means one invocation of `benchFn` failed.
 * The failure was detected by `errorFn`, operating on return values of `benchFn`.
 * Returns the faulty return value.
 * note : this function will abort() program execution if benchmark did not failed.
 *        always check if benchmark failed first !
 */
#[no_mangle]
pub unsafe extern "C" fn BMK_extract_errorResult(mut outcome:
                                                     BMK_runOutcome_t)
 -> size_t {
    if outcome.error_tag_never_ever_use_directly != 0i32 {
    } else {
        __assert_fail(b"outcome.error_tag_never_ever_use_directly != 0\x00" as
                          *const u8 as *const libc::c_char,
                      b"/home/danielrh/dev/zstd-c2rust/programs/benchfn.c\x00"
                          as *const u8 as *const libc::c_char,
                      80i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"size_t BMK_extract_errorResult(BMK_runOutcome_t)\x00")).as_ptr());
    };
    return outcome.error_result_never_ever_use_directly;
}
/* BMK_benchTimedFn() :
 * Similar to BMK_benchFunction(), most arguments being identical.
 * Automatically determines `nbLoops` so that each result is regularly produced at interval of about run_ms.
 * Note : minimum `nbLoops` is 1, therefore a run may last more than run_ms, and possibly even more than total_ms.
 * Usage - initialize timedFnState, select benchmark duration (total_ms) and each measurement duration (run_ms)
 *         call BMK_benchTimedFn() repetitively, each measurement is supposed to last about run_ms
 *         Check if total time budget is spent or exceeded, using BMK_isCompleted_TimedFn()
 */
#[no_mangle]
pub unsafe extern "C" fn BMK_benchTimedFn(mut cont: *mut BMK_timedFnState_t,
                                          mut p: BMK_benchParams_t)
 -> BMK_runOutcome_t {
    let runBudget_ns: U64 = (*cont).runBudget_ns;
    let runTimeMin_ns: U64 = runBudget_ns.wrapping_div(2i32 as libc::c_ulong);
    let mut completed: libc::c_int = 0i32;
    let mut bestRunTime: BMK_runTime_t = (*cont).fastestRun;
    while 0 == completed {
        let mut runResult: BMK_runOutcome_t =
            BMK_runOutcome_t{internal_never_ever_use_directly:
                                 BMK_runTime_t{nanoSecPerRun: 0,
                                               sumOfReturn: 0,},
                             error_result_never_ever_use_directly: 0,
                             error_tag_never_ever_use_directly: 0,};
        if UTIL_clockSpanMicro((*cont).coolTime) as libc::c_ulonglong >
               (70i32 as
                    libc::c_ulonglong).wrapping_mul((1i32 as
                                                         libc::c_ulonglong).wrapping_mul(1000000u64))
           {
            sleep(10i32 as libc::c_uint);
            (*cont).coolTime = UTIL_getTime()
        }
        runResult = BMK_benchFunction(p, (*cont).nbLoops);
        if 0 == BMK_isSuccessful_runOutcome(runResult) { return runResult }
        let newRunTime: BMK_runTime_t = BMK_extract_runTime(runResult);
        let loopDuration_ns: U64 =
            newRunTime.nanoSecPerRun.wrapping_mul((*cont).nbLoops as
                                                      libc::c_ulonglong) as
                U64;
        (*cont).timeSpent_ns =
            ((*cont).timeSpent_ns as
                 libc::c_ulong).wrapping_add(loopDuration_ns) as U64 as U64;
        if loopDuration_ns > runBudget_ns.wrapping_div(50i32 as libc::c_ulong)
           {
            let fastestRun_ns: U64 =
                (if bestRunTime.nanoSecPerRun < newRunTime.nanoSecPerRun {
                     bestRunTime.nanoSecPerRun
                 } else { newRunTime.nanoSecPerRun }) as U64;
            (*cont).nbLoops =
                (runBudget_ns.wrapping_div(fastestRun_ns) as
                     U32).wrapping_add(1i32 as libc::c_uint)
        } else {
            let multiplier: libc::c_uint = 10i32 as libc::c_uint;
            if (*cont).nbLoops <
                   (-1i32 as libc::c_uint).wrapping_div(multiplier) {
            } else {
                __assert_fail(b"cont->nbLoops < ((unsigned)-1) / multiplier\x00"
                                  as *const u8 as *const libc::c_char,
                              b"/home/danielrh/dev/zstd-c2rust/programs/benchfn.c\x00"
                                  as *const u8 as *const libc::c_char,
                              245i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 75],
                                                        &[libc::c_char; 75]>(b"BMK_runOutcome_t BMK_benchTimedFn(BMK_timedFnState_t *, BMK_benchParams_t)\x00")).as_ptr());
            };
            (*cont).nbLoops = (*cont).nbLoops.wrapping_mul(multiplier)
        }
        if loopDuration_ns < runTimeMin_ns {
            if completed == 0i32 {
            } else {
                __assert_fail(b"completed == 0\x00" as *const u8 as
                                  *const libc::c_char,
                              b"/home/danielrh/dev/zstd-c2rust/programs/benchfn.c\x00"
                                  as *const u8 as *const libc::c_char,
                              251i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 75],
                                                        &[libc::c_char; 75]>(b"BMK_runOutcome_t BMK_benchTimedFn(BMK_timedFnState_t *, BMK_benchParams_t)\x00")).as_ptr());
            };
        } else {
            if newRunTime.nanoSecPerRun < bestRunTime.nanoSecPerRun {
                bestRunTime = newRunTime
            }
            completed = 1i32
        }
    }
    return BMK_setValid_runTime(bestRunTime);
}
/* Tells if duration of all benchmark runs has exceeded total_ms
 */
#[no_mangle]
pub unsafe extern "C" fn BMK_isCompleted_TimedFn(mut timedFnState:
                                                     *const BMK_timedFnState_t)
 -> libc::c_int {
    return ((*timedFnState).timeSpent_ns >= (*timedFnState).timeBudget_ns) as
               libc::c_int;
}
/* BMK_createTimedFnState() and BMK_resetTimedFnState() :
 * Create/Set BMK_timedFnState_t for next benchmark session,
 * which shall last a minimum of total_ms milliseconds,
 * producing intermediate results, paced at interval of (approximately) run_ms.
 */
#[no_mangle]
pub unsafe extern "C" fn BMK_createTimedFnState(mut total_ms: libc::c_uint,
                                                mut run_ms: libc::c_uint)
 -> *mut BMK_timedFnState_t {
    let r: *mut BMK_timedFnState_t =
        malloc(::std::mem::size_of::<BMK_timedFnState_t>() as libc::c_ulong)
            as *mut BMK_timedFnState_t;
    if r.is_null() { return 0 as *mut BMK_timedFnState_t }
    BMK_resetTimedFnState(r, total_ms, run_ms);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_resetTimedFnState(mut timedFnState:
                                                   *mut BMK_timedFnState_t,
                                               mut total_ms: libc::c_uint,
                                               mut run_ms: libc::c_uint) {
    if 0 == total_ms { total_ms = 1i32 as libc::c_uint }
    if 0 == run_ms { run_ms = 1i32 as libc::c_uint }
    if run_ms > total_ms { run_ms = total_ms }
    (*timedFnState).timeSpent_ns = 0i32 as U64;
    (*timedFnState).timeBudget_ns =
        (total_ms as U64 as
             libc::c_ulonglong).wrapping_mul((1i32 as
                                                  libc::c_ulonglong).wrapping_mul(1000000000u64)).wrapping_div(1000i32
                                                                                                                   as
                                                                                                                   libc::c_ulonglong)
            as U64;
    (*timedFnState).runBudget_ns =
        (run_ms as U64 as
             libc::c_ulonglong).wrapping_mul((1i32 as
                                                  libc::c_ulonglong).wrapping_mul(1000000000u64)).wrapping_div(1000i32
                                                                                                                   as
                                                                                                                   libc::c_ulonglong)
            as U64;
    (*timedFnState).fastestRun.nanoSecPerRun =
        -1i64 as U64 as libc::c_ulonglong;
    (*timedFnState).fastestRun.sumOfReturn = -1i64 as size_t;
    (*timedFnState).nbLoops = 1i32 as libc::c_uint;
    (*timedFnState).coolTime = UTIL_getTime();
}
#[no_mangle]
pub unsafe extern "C" fn BMK_freeTimedFnState(mut state:
                                                  *mut BMK_timedFnState_t) {
    free(state as *mut libc::c_void);
}