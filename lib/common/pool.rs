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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* custom memory allocation functions */
    #[no_mangle]
    fn ZSTD_malloc(size: size_t, customMem: ZSTD_customMem)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ZSTD_calloc(size: size_t, customMem: ZSTD_customMem)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ZSTD_free(ptr: *mut libc::c_void, customMem: ZSTD_customMem);
    #[no_mangle]
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_wait(__cond: *mut pthread_cond_t,
                         __mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_create(__newthread: *mut pthread_t,
                      __attr: *const pthread_attr_t,
                      __start_routine:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> *mut libc::c_void>,
                      __arg: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_init(__cond: *mut pthread_cond_t,
                         __cond_attr: *const pthread_condattr_t)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_init(__mutex: *mut pthread_mutex_t,
                          __mutexattr: *const pthread_mutexattr_t)
     -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct POOL_ctx_s {
    pub customMem: ZSTD_customMem,
    pub threads: *mut pthread_t,
    pub threadCapacity: size_t,
    pub threadLimit: size_t,
    pub queue: *mut POOL_job,
    pub queueHead: size_t,
    pub queueTail: size_t,
    pub queueSize: size_t,
    pub numThreadsBusy: size_t,
    pub queueEmpty: libc::c_int,
    pub queueMutex: pthread_mutex_t,
    pub queuePushCond: pthread_cond_t,
    pub queuePopCond: pthread_cond_t,
    pub shutdown: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __pthread_cond_s {
    pub unnamed: unnamed_1,
    pub unnamed_0: unnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: unnamed_2,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type POOL_job = POOL_job_s;
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* ======   Dependencies   ======= */
/* size_t */
/* ======   Compiler specifics   ====== */
/* A job is a function and an opaque argument */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct POOL_job_s {
    pub function: POOL_function,
    pub opaque: *mut libc::c_void,
}
/* ! POOL_function :
 *  The function type that can be added to a thread pool.
 */
pub type POOL_function
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type pthread_t = libc::c_ulong;
/*
 * Copyright (c) 2016-present, Yann Collet, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */
/* size_t */
/* ZSTD_customMem */
pub type POOL_ctx = POOL_ctx_s;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
/* *< this constant defers to stdlib's functions */
static mut ZSTD_defaultCMem: ZSTD_customMem =
    ZSTD_customMem{customAlloc: None,
                   customFree: None,
                   opaque: 0 as *const libc::c_void as *mut libc::c_void,};
/* ! POOL_create() :
 *  Create a thread pool with at most `numThreads` threads.
 * `numThreads` must be at least 1.
 *  The maximum number of queued jobs before blocking is `queueSize`.
 * @return : POOL_ctx pointer on success, else NULL.
*/
#[no_mangle]
pub unsafe extern "C" fn POOL_create(mut numThreads: size_t,
                                     mut queueSize: size_t) -> *mut POOL_ctx {
    return POOL_create_advanced(numThreads, queueSize, ZSTD_defaultCMem);
}
#[no_mangle]
pub unsafe extern "C" fn POOL_create_advanced(mut numThreads: size_t,
                                              mut queueSize: size_t,
                                              mut customMem: ZSTD_customMem)
 -> *mut POOL_ctx {
    let mut ctx: *mut POOL_ctx = 0 as *mut POOL_ctx;
    if 0 == numThreads { return 0 as *mut POOL_ctx }
    ctx =
        ZSTD_calloc(::std::mem::size_of::<POOL_ctx>() as libc::c_ulong,
                    customMem) as *mut POOL_ctx;
    if ctx.is_null() { return 0 as *mut POOL_ctx }
    (*ctx).queueSize = queueSize.wrapping_add(1i32 as libc::c_ulong);
    (*ctx).queue =
        ZSTD_malloc((*ctx).queueSize.wrapping_mul(::std::mem::size_of::<POOL_job>()
                                                      as libc::c_ulong),
                    customMem) as *mut POOL_job;
    (*ctx).queueHead = 0i32 as size_t;
    (*ctx).queueTail = 0i32 as size_t;
    (*ctx).numThreadsBusy = 0i32 as size_t;
    (*ctx).queueEmpty = 1i32;
    pthread_mutex_init(&mut (*ctx).queueMutex,
                       0 as *const pthread_mutexattr_t);
    pthread_cond_init(&mut (*ctx).queuePushCond,
                      0 as *const pthread_condattr_t);
    pthread_cond_init(&mut (*ctx).queuePopCond,
                      0 as *const pthread_condattr_t);
    (*ctx).shutdown = 0i32;
    (*ctx).threads =
        ZSTD_malloc(numThreads.wrapping_mul(::std::mem::size_of::<pthread_t>()
                                                as libc::c_ulong), customMem)
            as *mut pthread_t;
    (*ctx).threadCapacity = 0i32 as size_t;
    (*ctx).customMem = customMem;
    if (*ctx).threads.is_null() || (*ctx).queue.is_null() {
        POOL_free(ctx);
        return 0 as *mut POOL_ctx
    }
    let mut i: size_t = 0;
    i = 0i32 as size_t;
    while i < numThreads {
        if 0 !=
               pthread_create(&mut *(*ctx).threads.offset(i as isize),
                              0 as *const pthread_attr_t, Some(POOL_thread),
                              ctx as *mut libc::c_void) {
            (*ctx).threadCapacity = i;
            POOL_free(ctx);
            return 0 as *mut POOL_ctx
        }
        i = i.wrapping_add(1)
    }
    (*ctx).threadCapacity = numThreads;
    (*ctx).threadLimit = numThreads;
    return ctx;
}
/* ! POOL_free() :
 *  Free a thread pool returned by POOL_create().
 */
#[no_mangle]
pub unsafe extern "C" fn POOL_free(mut ctx: *mut POOL_ctx) {
    if ctx.is_null() { return }
    POOL_join(ctx);
    pthread_mutex_destroy(&mut (*ctx).queueMutex);
    pthread_cond_destroy(&mut (*ctx).queuePushCond);
    pthread_cond_destroy(&mut (*ctx).queuePopCond);
    ZSTD_free((*ctx).queue as *mut libc::c_void, (*ctx).customMem);
    ZSTD_free((*ctx).threads as *mut libc::c_void, (*ctx).customMem);
    ZSTD_free(ctx as *mut libc::c_void, (*ctx).customMem);
}
/* ! POOL_join() :
    Shutdown the queue, wake any sleeping threads, and join all of the threads.
*/
unsafe extern "C" fn POOL_join(mut ctx: *mut POOL_ctx) {
    pthread_mutex_lock(&mut (*ctx).queueMutex);
    (*ctx).shutdown = 1i32;
    pthread_mutex_unlock(&mut (*ctx).queueMutex);
    pthread_cond_broadcast(&mut (*ctx).queuePushCond);
    pthread_cond_broadcast(&mut (*ctx).queuePopCond);
    let mut i: size_t = 0;
    i = 0i32 as size_t;
    while i < (*ctx).threadCapacity {
        pthread_join(*(*ctx).threads.offset(i as isize),
                     0 as *mut *mut libc::c_void);
        i = i.wrapping_add(1)
    };
}
/* POOL_thread() :
 * Work thread for the thread pool.
 * Waits for jobs and executes them.
 * @returns : NULL on failure else non-null.
 */
unsafe extern "C" fn POOL_thread(mut opaque: *mut libc::c_void)
 -> *mut libc::c_void {
    let ctx: *mut POOL_ctx = opaque as *mut POOL_ctx;
    if ctx.is_null() { return 0 as *mut libc::c_void }
    loop  {
        pthread_mutex_lock(&mut (*ctx).queueMutex);
        while 0 != (*ctx).queueEmpty ||
                  (*ctx).numThreadsBusy >= (*ctx).threadLimit {
            if 0 != (*ctx).shutdown {
                pthread_mutex_unlock(&mut (*ctx).queueMutex);
                return opaque
            }
            pthread_cond_wait(&mut (*ctx).queuePopCond,
                              &mut (*ctx).queueMutex);
        }
        let job: POOL_job = *(*ctx).queue.offset((*ctx).queueHead as isize);
        (*ctx).queueHead =
            (*ctx).queueHead.wrapping_add(1i32 as
                                              libc::c_ulong).wrapping_rem((*ctx).queueSize);
        (*ctx).numThreadsBusy = (*ctx).numThreadsBusy.wrapping_add(1);
        (*ctx).queueEmpty =
            ((*ctx).queueHead == (*ctx).queueTail) as libc::c_int;
        pthread_cond_signal(&mut (*ctx).queuePushCond);
        pthread_mutex_unlock(&mut (*ctx).queueMutex);
        job.function.expect("non-null function pointer")(job.opaque);
        pthread_mutex_lock(&mut (*ctx).queueMutex);
        (*ctx).numThreadsBusy = (*ctx).numThreadsBusy.wrapping_sub(1);
        if (*ctx).queueSize == 1i32 as libc::c_ulong {
            pthread_cond_signal(&mut (*ctx).queuePushCond);
        }
        pthread_mutex_unlock(&mut (*ctx).queueMutex);
    };
}
/* ! POOL_resize() :
 *  Expands or shrinks pool's number of threads.
 *  This is more efficient than releasing + creating a new context,
 *  since it tries to preserve and re-use existing threads.
 * `numThreads` must be at least 1.
 * @return : 0 when resize was successful,
 *           !0 (typically 1) if there is an error.
 *    note : only numThreads can be resized, queueSize remains unchanged.
 */
#[no_mangle]
pub unsafe extern "C" fn POOL_resize(mut ctx: *mut POOL_ctx,
                                     mut numThreads: size_t) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if ctx.is_null() { return 1i32 }
    pthread_mutex_lock(&mut (*ctx).queueMutex);
    result = POOL_resize_internal(ctx, numThreads);
    pthread_cond_broadcast(&mut (*ctx).queuePopCond);
    pthread_mutex_unlock(&mut (*ctx).queueMutex);
    return result;
}
/* @return : 0 on success, 1 on error */
unsafe extern "C" fn POOL_resize_internal(mut ctx: *mut POOL_ctx,
                                          mut numThreads: size_t)
 -> libc::c_int {
    if numThreads <= (*ctx).threadCapacity {
        if 0 == numThreads { return 1i32 }
        (*ctx).threadLimit = numThreads;
        return 0i32
    }
    let threadPool: *mut pthread_t =
        ZSTD_malloc(numThreads.wrapping_mul(::std::mem::size_of::<pthread_t>()
                                                as libc::c_ulong),
                    (*ctx).customMem) as *mut pthread_t;
    if threadPool.is_null() { return 1i32 }
    memcpy(threadPool as *mut libc::c_void,
           (*ctx).threads as *const libc::c_void,
           (*ctx).threadCapacity.wrapping_mul(::std::mem::size_of::<pthread_t>()
                                                  as libc::c_ulong));
    ZSTD_free((*ctx).threads as *mut libc::c_void, (*ctx).customMem);
    (*ctx).threads = threadPool;
    let mut threadId: size_t = 0;
    threadId = (*ctx).threadCapacity;
    while threadId < numThreads {
        if 0 !=
               pthread_create(&mut *threadPool.offset(threadId as isize),
                              0 as *const pthread_attr_t, Some(POOL_thread),
                              ctx as *mut libc::c_void) {
            (*ctx).threadCapacity = threadId;
            return 1i32
        }
        threadId = threadId.wrapping_add(1)
    }
    (*ctx).threadCapacity = numThreads;
    (*ctx).threadLimit = numThreads;
    return 0i32;
}
/* ! POOL_sizeof() :
 * @return threadpool memory usage
 *  note : compatible with NULL (returns 0 in this case)
 */
#[no_mangle]
pub unsafe extern "C" fn POOL_sizeof(mut ctx: *mut POOL_ctx) -> size_t {
    if ctx.is_null() { return 0i32 as size_t }
    return (::std::mem::size_of::<POOL_ctx>() as
                libc::c_ulong).wrapping_add((*ctx).queueSize.wrapping_mul(::std::mem::size_of::<POOL_job>()
                                                                              as
                                                                              libc::c_ulong)).wrapping_add((*ctx).threadCapacity.wrapping_mul(::std::mem::size_of::<pthread_t>()
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong));
}
/* ! POOL_add() :
 *  Add the job `function(opaque)` to the thread pool. `ctx` must be valid.
 *  Possibly blocks until there is room in the queue.
 *  Note : The function may be executed asynchronously,
 *         therefore, `opaque` must live until function has been completed.
 */
#[no_mangle]
pub unsafe extern "C" fn POOL_add(mut ctx: *mut POOL_ctx,
                                  mut function: POOL_function,
                                  mut opaque: *mut libc::c_void) {
    pthread_mutex_lock(&mut (*ctx).queueMutex);
    while 0 != isQueueFull(ctx) && 0 == (*ctx).shutdown {
        pthread_cond_wait(&mut (*ctx).queuePushCond, &mut (*ctx).queueMutex);
    }
    POOL_add_internal(ctx, function, opaque);
    pthread_mutex_unlock(&mut (*ctx).queueMutex);
}
unsafe extern "C" fn POOL_add_internal(mut ctx: *mut POOL_ctx,
                                       mut function: POOL_function,
                                       mut opaque: *mut libc::c_void) {
    let job: POOL_job = POOL_job_s{function: function, opaque: opaque,};
    if 0 != (*ctx).shutdown { return }
    (*ctx).queueEmpty = 0i32;
    *(*ctx).queue.offset((*ctx).queueTail as isize) = job;
    (*ctx).queueTail =
        (*ctx).queueTail.wrapping_add(1i32 as
                                          libc::c_ulong).wrapping_rem((*ctx).queueSize);
    pthread_cond_signal(&mut (*ctx).queuePopCond);
}
/* *
 * Returns 1 if the queue is full and 0 otherwise.
 *
 * When queueSize is 1 (pool was created with an intended queueSize of 0),
 * then a queue is empty if there is a thread free _and_ no job is waiting.
 */
unsafe extern "C" fn isQueueFull(mut ctx: *const POOL_ctx) -> libc::c_int {
    if (*ctx).queueSize > 1i32 as libc::c_ulong {
        return ((*ctx).queueHead ==
                    (*ctx).queueTail.wrapping_add(1i32 as
                                                      libc::c_ulong).wrapping_rem((*ctx).queueSize))
                   as libc::c_int
    } else {
        return ((*ctx).numThreadsBusy == (*ctx).threadLimit ||
                    0 == (*ctx).queueEmpty) as libc::c_int
    };
}
/* ! POOL_tryAdd() :
 *  Add the job `function(opaque)` to thread pool _if_ a worker is available.
 *  Returns immediately even if not (does not block).
 * @return : 1 if successful, 0 if not.
 */
#[no_mangle]
pub unsafe extern "C" fn POOL_tryAdd(mut ctx: *mut POOL_ctx,
                                     mut function: POOL_function,
                                     mut opaque: *mut libc::c_void)
 -> libc::c_int {
    pthread_mutex_lock(&mut (*ctx).queueMutex);
    if 0 != isQueueFull(ctx) {
        pthread_mutex_unlock(&mut (*ctx).queueMutex);
        return 0i32
    }
    POOL_add_internal(ctx, function, opaque);
    pthread_mutex_unlock(&mut (*ctx).queueMutex);
    return 1i32;
}