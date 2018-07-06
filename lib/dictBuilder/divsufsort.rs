#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( extern_types , libc , offset_to )]
extern crate libc;
extern "C" {
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    static mut _IO_2_1_stdin_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stdout_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stderr_: _IO_FILE_plus;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
}
pub type trbudget_t = _trbudget_t;
pub type __off64_t = libc::c_long;
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _trbudget_t {
    pub chance: libc::c_int,
    pub remain: libc::c_int,
    pub incval: libc::c_int,
    pub count: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub a: *mut libc::c_int,
    pub b: *mut libc::c_int,
    pub c: *mut libc::c_int,
    pub d: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub a: *mut libc::c_int,
    pub b: *mut libc::c_int,
    pub c: libc::c_int,
    pub d: libc::c_int,
}
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub a: *const libc::c_int,
    pub b: *mut libc::c_int,
    pub c: *mut libc::c_int,
    pub d: libc::c_int,
    pub e: libc::c_int,
}
pub type _IO_lock_t = ();
/* *
 * Constructs the suffix array of a given string.
 * @param T [0..n-1] The input string.
 * @param SA [0..n-1] The output array of suffixes.
 * @param n The length of the given string.
 * @param openMP enables OpenMP optimization.
 * @return 0 if no error occurred, -1 or -2 otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn divsufsort(mut T: *const libc::c_uchar,
                                    mut SA: *mut libc::c_int,
                                    mut n: libc::c_int,
                                    mut openMP: libc::c_int) -> libc::c_int {
    let mut bucket_A: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut bucket_B: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut err: libc::c_int = 0i32;
    if T.is_null() || SA.is_null() || n < 0i32 {
        return -1i32
    } else if n == 0i32 {
        return 0i32
    } else if n == 1i32 {
        *SA.offset(0isize) = 0i32;
        return 0i32
    } else if n == 2i32 {
        m =
            ((*T.offset(0isize) as libc::c_int) <
                 *T.offset(1isize) as libc::c_int) as libc::c_int;
        *SA.offset((m ^ 1i32) as isize) = 0i32;
        *SA.offset(m as isize) = 1i32;
        return 0i32
    } else {
        bucket_A =
            malloc((256i32 as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong)) as
                *mut libc::c_int;
        bucket_B =
            malloc(((256i32 * 256i32) as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong)) as
                *mut libc::c_int;
        if !bucket_A.is_null() && !bucket_B.is_null() {
            m = sort_typeBstar(T, SA, bucket_A, bucket_B, n, openMP);
            construct_SA(T, SA, bucket_A, bucket_B, n, m);
        } else { err = -2i32 }
        free(bucket_B as *mut libc::c_void);
        free(bucket_A as *mut libc::c_void);
        return err
    };
}
unsafe extern "C" fn construct_SA(mut T: *const libc::c_uchar,
                                  mut SA: *mut libc::c_int,
                                  mut bucket_A: *mut libc::c_int,
                                  mut bucket_B: *mut libc::c_int,
                                  mut n: libc::c_int, mut m: libc::c_int)
 -> () {
    let mut i: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut j: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut k: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut s: libc::c_int = 0;
    let mut c0: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    if 0i32 < m {
        c1 = 256i32 - 2i32;
        while 0i32 <= c1 {
            i =
                SA.offset(*bucket_B.offset((c1 << 8i32 | c1 + 1i32) as isize)
                              as isize);
            j =
                SA.offset(*bucket_A.offset((c1 + 1i32) as isize) as
                              isize).offset(-1isize);
            k = 0 as *mut libc::c_int;
            c2 = -1i32;
            while i <= j {
                s = *j;
                if 0i32 < s {
                    if *T.offset(s as isize) as libc::c_int == c1 {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                                &mut [libc::c_char; 11]>(b"T[s] == c1\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1630i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 72],
                                                                &[libc::c_char; 72]>(b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\x00")).as_ptr());
                    };
                    if s + 1i32 < n &&
                           *T.offset(s as isize) as libc::c_int <=
                               *T.offset((s + 1i32) as isize) as libc::c_int {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 36],
                                                                &mut [libc::c_char; 36]>(b"((s + 1) < n) && (T[s] <= T[s + 1])\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1631i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 72],
                                                                &[libc::c_char; 72]>(b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\x00")).as_ptr());
                    };
                    if *T.offset((s - 1i32) as isize) as libc::c_int <=
                           *T.offset(s as isize) as libc::c_int {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 17],
                                                                &mut [libc::c_char; 17]>(b"T[s - 1] <= T[s]\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1632i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 72],
                                                                &[libc::c_char; 72]>(b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\x00")).as_ptr());
                    };
                    *j = !s;
                    s -= 1;
                    c0 = *T.offset(s as isize) as libc::c_int;
                    if 0i32 < s &&
                           *T.offset((s - 1i32) as isize) as libc::c_int > c0
                       {
                        s = !s
                    }
                    if c0 != c2 {
                        if 0i32 <= c2 {
                            *bucket_B.offset((c1 << 8i32 | c2) as isize) =
                                SA.offset_to(k).expect("bad offset_to") as
                                    libc::c_long as libc::c_int
                        }
                        c2 = c0;
                        k =
                            SA.offset(*bucket_B.offset((c1 << 8i32 | c2) as
                                                           isize) as isize)
                    }
                    if k < j {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 6],
                                                                &mut [libc::c_char; 6]>(b"k < j\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1640i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 72],
                                                                &[libc::c_char; 72]>(b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\x00")).as_ptr());
                    };
                    let fresh0 = k;
                    k = k.offset(-1);
                    *fresh0 = s
                } else {
                    if s == 0i32 && *T.offset(s as isize) as libc::c_int == c1
                           || s < 0i32 {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 38],
                                                                &mut [libc::c_char; 38]>(b"((s == 0) && (T[s] == c1)) || (s < 0)\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1643i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 72],
                                                                &[libc::c_char; 72]>(b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\x00")).as_ptr());
                    };
                    *j = !s
                }
                j = j.offset(-1isize)
            }
            c1 -= 1
        }
    }
    c2 = *T.offset((n - 1i32) as isize) as libc::c_int;
    k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
    let fresh1 = k;
    k = k.offset(1);
    *fresh1 =
        if (*T.offset((n - 2i32) as isize) as libc::c_int) < c2 {
            !(n - 1i32)
        } else { n - 1i32 };
    i = SA;
    j = SA.offset(n as isize);
    while i < j {
        s = *i;
        if 0i32 < s {
            if *T.offset((s - 1i32) as isize) as libc::c_int >=
                   *T.offset(s as isize) as libc::c_int {
            } else {
                __assert_fail((*::std::mem::transmute::<&[u8; 17],
                                                        &mut [libc::c_char; 17]>(b"T[s - 1] >= T[s]\x00")).as_mut_ptr(),
                              (*::std::mem::transmute::<&[u8; 53],
                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                              1657i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 72],
                                                        &[libc::c_char; 72]>(b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\x00")).as_ptr());
            };
            s -= 1;
            c0 = *T.offset(s as isize) as libc::c_int;
            if s == 0i32 ||
                   (*T.offset((s - 1i32) as isize) as libc::c_int) < c0 {
                s = !s
            }
            if c0 != c2 {
                *bucket_A.offset(c2 as isize) =
                    SA.offset_to(k).expect("bad offset_to") as libc::c_long as
                        libc::c_int;
                c2 = c0;
                k = SA.offset(*bucket_A.offset(c2 as isize) as isize)
            }
            if i < k {
            } else {
                __assert_fail((*::std::mem::transmute::<&[u8; 6],
                                                        &mut [libc::c_char; 6]>(b"i < k\x00")).as_mut_ptr(),
                              (*::std::mem::transmute::<&[u8; 53],
                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                              1664i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 72],
                                                        &[libc::c_char; 72]>(b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\x00")).as_ptr());
            };
            let fresh2 = k;
            k = k.offset(1);
            *fresh2 = s
        } else {
            if s < 0i32 {
            } else {
                __assert_fail((*::std::mem::transmute::<&[u8; 6],
                                                        &mut [libc::c_char; 6]>(b"s < 0\x00")).as_mut_ptr(),
                              (*::std::mem::transmute::<&[u8; 53],
                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                              1667i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 72],
                                                        &[libc::c_char; 72]>(b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\x00")).as_ptr());
            };
            *i = !s
        }
        i = i.offset(1isize)
    };
}
unsafe extern "C" fn sort_typeBstar(mut T: *const libc::c_uchar,
                                    mut SA: *mut libc::c_int,
                                    mut bucket_A: *mut libc::c_int,
                                    mut bucket_B: *mut libc::c_int,
                                    mut n: libc::c_int,
                                    mut openMP: libc::c_int) -> libc::c_int {
    let mut PAb: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ISAb: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut buf: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut bufsize: libc::c_int = 0;
    let mut c0: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    i = 0i32;
    while i < 256i32 { *bucket_A.offset(i as isize) = 0i32; i += 1 }
    i = 0i32;
    while i < 256i32 * 256i32 { *bucket_B.offset(i as isize) = 0i32; i += 1 }
    i = n - 1i32;
    m = n;
    c0 = *T.offset((n - 1i32) as isize) as libc::c_int;
    while 0i32 <= i {
        loop  {
            c1 = c0;
            let ref mut fresh3 = *bucket_A.offset(c1 as isize);
            *fresh3 += 1;
            i -= 1;
            if !(0i32 <= i &&
                     { c0 = *T.offset(i as isize) as libc::c_int; c0 >= c1 })
               {
                break ;
            }
        }
        if !(0i32 <= i) { continue ; }
        let ref mut fresh4 = *bucket_B.offset((c0 << 8i32 | c1) as isize);
        *fresh4 += 1;
        m -= 1;
        *SA.offset(m as isize) = i;
        i -= 1;
        c1 = c0;
        while 0i32 <= i &&
                  { c0 = *T.offset(i as isize) as libc::c_int; c0 <= c1 } {
            let ref mut fresh5 = *bucket_B.offset((c1 << 8i32 | c0) as isize);
            *fresh5 += 1;
            i -= 1;
            c1 = c0
        }
    }
    m = n - m;
    c0 = 0i32;
    i = 0i32;
    j = 0i32;
    while c0 < 256i32 {
        t = i + *bucket_A.offset(c0 as isize);
        *bucket_A.offset(c0 as isize) = i + j;
        i = t + *bucket_B.offset((c0 << 8i32 | c0) as isize);
        c1 = c0 + 1i32;
        while c1 < 256i32 {
            j += *bucket_B.offset((c0 << 8i32 | c1) as isize);
            *bucket_B.offset((c0 << 8i32 | c1) as isize) = j;
            i += *bucket_B.offset((c1 << 8i32 | c0) as isize);
            c1 += 1
        }
        c0 += 1
    }
    if 0i32 < m {
        PAb = SA.offset(n as isize).offset(-(m as isize));
        ISAb = SA.offset(m as isize);
        i = m - 2i32;
        while 0i32 <= i {
            t = *PAb.offset(i as isize);
            c0 = *T.offset(t as isize) as libc::c_int;
            c1 = *T.offset((t + 1i32) as isize) as libc::c_int;
            let ref mut fresh6 = *bucket_B.offset((c0 << 8i32 | c1) as isize);
            *fresh6 -= 1;
            *SA.offset(*fresh6 as isize) = i;
            i -= 1
        }
        t = *PAb.offset((m - 1i32) as isize);
        c0 = *T.offset(t as isize) as libc::c_int;
        c1 = *T.offset((t + 1i32) as isize) as libc::c_int;
        let ref mut fresh7 = *bucket_B.offset((c0 << 8i32 | c1) as isize);
        *fresh7 -= 1;
        *SA.offset(*fresh7 as isize) = m - 1i32;
        buf = SA.offset(m as isize);
        bufsize = n - 2i32 * m;
        c0 = 256i32 - 2i32;
        j = m;
        while 0i32 < j {
            c1 = 256i32 - 1i32;
            while c0 < c1 {
                i = *bucket_B.offset((c0 << 8i32 | c1) as isize);
                if 1i32 < j - i {
                    sssort(T, PAb, SA.offset(i as isize),
                           SA.offset(j as isize), buf, bufsize, 2i32, n,
                           (*SA.offset(i as isize) == m - 1i32) as
                               libc::c_int);
                }
                j = i;
                c1 -= 1
            }
            c0 -= 1
        }
        i = m - 1i32;
        while 0i32 <= i {
            if 0i32 <= *SA.offset(i as isize) {
                j = i;
                loop  {
                    *ISAb.offset(*SA.offset(i as isize) as isize) = i;
                    i -= 1;
                    if !(0i32 <= i && 0i32 <= *SA.offset(i as isize)) {
                        break ;
                    }
                }
                *SA.offset((i + 1i32) as isize) = i - j;
                if i <= 0i32 { break ; }
            }
            j = i;
            loop  {
                let ref mut fresh8 = *SA.offset(i as isize);
                *fresh8 = !*SA.offset(i as isize);
                *ISAb.offset(*fresh8 as isize) = j;
                i -= 1;
                if !(*SA.offset(i as isize) < 0i32) { break ; }
            }
            *ISAb.offset(*SA.offset(i as isize) as isize) = j;
            i -= 1
        }
        trsort(ISAb, SA, m, 1i32);
        i = n - 1i32;
        j = m;
        c0 = *T.offset((n - 1i32) as isize) as libc::c_int;
        while 0i32 <= i {
            i -= 1;
            c1 = c0;
            while 0i32 <= i &&
                      { c0 = *T.offset(i as isize) as libc::c_int; c0 >= c1 }
                  {
                i -= 1;
                c1 = c0
            }
            if !(0i32 <= i) { continue ; }
            t = i;
            i -= 1;
            c1 = c0;
            while 0i32 <= i &&
                      { c0 = *T.offset(i as isize) as libc::c_int; c0 <= c1 }
                  {
                i -= 1;
                c1 = c0
            }
            j -= 1;
            *SA.offset(*ISAb.offset(j as isize) as isize) =
                if t == 0i32 || 1i32 < t - i { t } else { !t }
        }
        *bucket_B.offset((256i32 - 1i32 << 8i32 | 256i32 - 1i32) as isize) =
            n;
        c0 = 256i32 - 2i32;
        k = m - 1i32;
        while 0i32 <= c0 {
            i = *bucket_A.offset((c0 + 1i32) as isize) - 1i32;
            c1 = 256i32 - 1i32;
            while c0 < c1 {
                t = i - *bucket_B.offset((c1 << 8i32 | c0) as isize);
                *bucket_B.offset((c1 << 8i32 | c0) as isize) = i;
                i = t;
                j = *bucket_B.offset((c0 << 8i32 | c1) as isize);
                while j <= k {
                    *SA.offset(i as isize) = *SA.offset(k as isize);
                    i -= 1;
                    k -= 1
                }
                c1 -= 1
            }
            *bucket_B.offset((c0 << 8i32 | c0 + 1i32) as isize) =
                i - *bucket_B.offset((c0 << 8i32 | c0) as isize) + 1i32;
            *bucket_B.offset((c0 << 8i32 | c0) as isize) = i;
            c0 -= 1
        }
    }
    return m;
}
unsafe extern "C" fn trsort(mut ISA: *mut libc::c_int,
                            mut SA: *mut libc::c_int, mut n: libc::c_int,
                            mut depth: libc::c_int) -> () {
    let mut ISAd: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut first: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut last: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut budget: trbudget_t =
        _trbudget_t{chance: 0, remain: 0, incval: 0, count: 0,};
    let mut t: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    let mut unsorted: libc::c_int = 0;
    trbudget_init(&mut budget as *mut trbudget_t, tr_ilg(n) * 2i32 / 3i32, n);
    ISAd = ISA.offset(depth as isize);
    while -n < *SA {
        first = SA;
        skip = 0i32;
        unsorted = 0i32;
        loop  {
            t = *first;
            if t < 0i32 {
                first = first.offset(-(t as isize));
                skip += t
            } else {
                if skip != 0i32 {
                    *first.offset(skip as isize) = skip;
                    skip = 0i32
                }
                last =
                    SA.offset(*ISA.offset(t as isize) as
                                  isize).offset(1isize);
                if (1i32 as libc::c_long) <
                       first.offset_to(last).expect("bad offset_to") as
                           libc::c_long {
                    budget.count = 0i32;
                    tr_introsort(ISA, ISAd, SA, first, last,
                                 &mut budget as *mut trbudget_t);
                    if budget.count != 0i32 {
                        unsorted += budget.count
                    } else {
                        skip =
                            last.offset_to(first).expect("bad offset_to") as
                                libc::c_long as libc::c_int
                    }
                } else if first.offset_to(last).expect("bad offset_to") as
                              libc::c_long == 1i32 as libc::c_long {
                    skip = -1i32
                }
                first = last
            }
            if !(first < SA.offset(n as isize)) { break ; }
        }
        if skip != 0i32 { *first.offset(skip as isize) = skip }
        if unsorted == 0i32 { break ; }
        ISAd =
            ISAd.offset(ISA.offset_to(ISAd).expect("bad offset_to") as
                            libc::c_long as isize)
    };
}
unsafe extern "C" fn tr_introsort(mut ISA: *mut libc::c_int,
                                  mut ISAd: *const libc::c_int,
                                  mut SA: *mut libc::c_int,
                                  mut first: *mut libc::c_int,
                                  mut last: *mut libc::c_int,
                                  mut budget: *mut trbudget_t) -> () {
    let mut stack: [unnamed_1; 64] =
        [unnamed_1{a: 0 as *const libc::c_int,
                   b: 0 as *mut libc::c_int,
                   c: 0 as *mut libc::c_int,
                   d: 0,
                   e: 0,}; 64];
    let mut a: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut b: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut c: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut x: libc::c_int = 0i32;
    let mut incr: libc::c_int =
        ISA.offset_to(ISAd).expect("bad offset_to") as libc::c_long as
            libc::c_int;
    let mut limit: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut ssize: libc::c_int = 0;
    let mut trlink: libc::c_int = -1i32;
    ssize = 0i32;
    limit =
        tr_ilg(first.offset_to(last).expect("bad offset_to") as libc::c_long
                   as libc::c_int);
    loop  {
        if limit < 0i32 {
            if limit == -1i32 {
                tr_partition(ISAd.offset(-(incr as isize)), first, first,
                             last, &mut a as *mut *mut libc::c_int,
                             &mut b as *mut *mut libc::c_int,
                             (SA.offset_to(last).expect("bad offset_to") as
                                  libc::c_long - 1i32 as libc::c_long) as
                                 libc::c_int);
                if a < last {
                    c = first;
                    v =
                        (SA.offset_to(a).expect("bad offset_to") as
                             libc::c_long - 1i32 as libc::c_long) as
                            libc::c_int;
                    while c < a {
                        *ISA.offset(*c as isize) = v;
                        c = c.offset(1isize)
                    }
                }
                if b < last {
                    c = a;
                    v =
                        (SA.offset_to(b).expect("bad offset_to") as
                             libc::c_long - 1i32 as libc::c_long) as
                            libc::c_int;
                    while c < b {
                        *ISA.offset(*c as isize) = v;
                        c = c.offset(1isize)
                    }
                }
                if (1i32 as libc::c_long) <
                       a.offset_to(b).expect("bad offset_to") as libc::c_long
                   {
                    if ssize < 64i32 {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1204i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 73],
                                                                &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                    };
                    stack[ssize as usize].a = 0 as *const libc::c_int;
                    stack[ssize as usize].b = a;
                    stack[ssize as usize].c = b;
                    stack[ssize as usize].d = 0i32;
                    let fresh9 = ssize;
                    ssize = ssize + 1;
                    stack[fresh9 as usize].e = 0i32;
                    if ssize < 64i32 {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1205i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 73],
                                                                &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                    };
                    stack[ssize as usize].a = ISAd.offset(-(incr as isize));
                    stack[ssize as usize].b = first;
                    stack[ssize as usize].c = last;
                    stack[ssize as usize].d = -2i32;
                    let fresh10 = ssize;
                    ssize = ssize + 1;
                    stack[fresh10 as usize].e = trlink;
                    trlink = ssize - 2i32
                }
                if first.offset_to(a).expect("bad offset_to") as libc::c_long
                       <=
                       b.offset_to(last).expect("bad offset_to") as
                           libc::c_long {
                    if (1i32 as libc::c_long) <
                           first.offset_to(a).expect("bad offset_to") as
                               libc::c_long {
                        if ssize < 64i32 {
                        } else {
                            __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                    &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                          (*::std::mem::transmute::<&[u8; 53],
                                                                    &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                          1210i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 73],
                                                                    &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                        };
                        stack[ssize as usize].a = ISAd;
                        stack[ssize as usize].b = b;
                        stack[ssize as usize].c = last;
                        stack[ssize as usize].d =
                            tr_ilg(b.offset_to(last).expect("bad offset_to")
                                       as libc::c_long as libc::c_int);
                        let fresh11 = ssize;
                        ssize = ssize + 1;
                        stack[fresh11 as usize].e = trlink;
                        last = a;
                        limit =
                            tr_ilg(first.offset_to(a).expect("bad offset_to")
                                       as libc::c_long as libc::c_int)
                    } else if (1i32 as libc::c_long) <
                                  b.offset_to(last).expect("bad offset_to") as
                                      libc::c_long {
                        first = b;
                        limit =
                            tr_ilg(b.offset_to(last).expect("bad offset_to")
                                       as libc::c_long as libc::c_int)
                    } else {
                        if 0i32 <= ssize {
                        } else {
                            __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                                    &mut [libc::c_char; 11]>(b"0 <= ssize\x00")).as_mut_ptr(),
                                          (*::std::mem::transmute::<&[u8; 53],
                                                                    &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                          1215i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 73],
                                                                    &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                        };
                        if ssize == 0i32 {
                            return
                        } else {
                            ssize -= 1;
                            ISAd = stack[ssize as usize].a;
                            first = stack[ssize as usize].b;
                            last = stack[ssize as usize].c;
                            limit = stack[ssize as usize].d;
                            trlink = stack[ssize as usize].e
                        }
                    }
                } else if (1i32 as libc::c_long) <
                              b.offset_to(last).expect("bad offset_to") as
                                  libc::c_long {
                    if ssize < 64i32 {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1219i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 73],
                                                                &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                    };
                    stack[ssize as usize].a = ISAd;
                    stack[ssize as usize].b = first;
                    stack[ssize as usize].c = a;
                    stack[ssize as usize].d =
                        tr_ilg(first.offset_to(a).expect("bad offset_to") as
                                   libc::c_long as libc::c_int);
                    let fresh12 = ssize;
                    ssize = ssize + 1;
                    stack[fresh12 as usize].e = trlink;
                    first = b;
                    limit =
                        tr_ilg(b.offset_to(last).expect("bad offset_to") as
                                   libc::c_long as libc::c_int)
                } else if (1i32 as libc::c_long) <
                              first.offset_to(a).expect("bad offset_to") as
                                  libc::c_long {
                    last = a;
                    limit =
                        tr_ilg(first.offset_to(a).expect("bad offset_to") as
                                   libc::c_long as libc::c_int)
                } else {
                    if 0i32 <= ssize {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                                &mut [libc::c_char; 11]>(b"0 <= ssize\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1224i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 73],
                                                                &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                    };
                    if ssize == 0i32 {
                        return
                    } else {
                        ssize -= 1;
                        ISAd = stack[ssize as usize].a;
                        first = stack[ssize as usize].b;
                        last = stack[ssize as usize].c;
                        limit = stack[ssize as usize].d;
                        trlink = stack[ssize as usize].e
                    }
                }
            } else if limit == -2i32 {
                ssize -= 1;
                a = stack[ssize as usize].b;
                b = stack[ssize as usize].c;
                if stack[ssize as usize].d == 0i32 {
                    tr_copy(ISA, SA, first, a, b, last,
                            ISA.offset_to(ISAd).expect("bad offset_to") as
                                libc::c_long as libc::c_int);
                } else {
                    if 0i32 <= trlink { stack[trlink as usize].d = -1i32 }
                    tr_partialcopy(ISA, SA, first, a, b, last,
                                   ISA.offset_to(ISAd).expect("bad offset_to")
                                       as libc::c_long as libc::c_int);
                }
                if 0i32 <= ssize {
                } else {
                    __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                            &mut [libc::c_char; 11]>(b"0 <= ssize\x00")).as_mut_ptr(),
                                  (*::std::mem::transmute::<&[u8; 53],
                                                            &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                  1236i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 73],
                                                            &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                };
                if ssize == 0i32 {
                    return
                } else {
                    ssize -= 1;
                    ISAd = stack[ssize as usize].a;
                    first = stack[ssize as usize].b;
                    last = stack[ssize as usize].c;
                    limit = stack[ssize as usize].d;
                    trlink = stack[ssize as usize].e
                }
            } else {
                if 0i32 <= *first {
                    a = first;
                    loop  {
                        *ISA.offset(*a as isize) =
                            SA.offset_to(a).expect("bad offset_to") as
                                libc::c_long as libc::c_int;
                        a = a.offset(1isize);
                        if !(a < last && 0i32 <= *a) { break ; }
                    }
                    first = a
                }
                if first < last {
                    a = first;
                    loop  {
                        *a = !*a;
                        a = a.offset(1isize);
                        if !(*a < 0i32) { break ; }
                    }
                    next =
                        if *ISA.offset(*a as isize) !=
                               *ISAd.offset(*a as isize) {
                            tr_ilg((first.offset_to(a).expect("bad offset_to")
                                        as libc::c_long +
                                        1i32 as libc::c_long) as libc::c_int)
                        } else { -1i32 };
                    a = a.offset(1isize);
                    if a < last {
                        b = first;
                        v =
                            (SA.offset_to(a).expect("bad offset_to") as
                                 libc::c_long - 1i32 as libc::c_long) as
                                libc::c_int;
                        while b < a {
                            *ISA.offset(*b as isize) = v;
                            b = b.offset(1isize)
                        }
                    }
                    if 0 !=
                           trbudget_check(budget,
                                          first.offset_to(a).expect("bad offset_to")
                                              as libc::c_long as libc::c_int)
                       {
                        if first.offset_to(a).expect("bad offset_to") as
                               libc::c_long <=
                               a.offset_to(last).expect("bad offset_to") as
                                   libc::c_long {
                            if ssize < 64i32 {
                            } else {
                                __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                        &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                              (*::std::mem::transmute::<&[u8; 53],
                                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                              1252i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 73],
                                                                        &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                            };
                            stack[ssize as usize].a = ISAd;
                            stack[ssize as usize].b = a;
                            stack[ssize as usize].c = last;
                            stack[ssize as usize].d = -3i32;
                            let fresh13 = ssize;
                            ssize = ssize + 1;
                            stack[fresh13 as usize].e = trlink;
                            ISAd = ISAd.offset(incr as isize);
                            last = a;
                            limit = next
                        } else if (1i32 as libc::c_long) <
                                      a.offset_to(last).expect("bad offset_to")
                                          as libc::c_long {
                            if ssize < 64i32 {
                            } else {
                                __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                        &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                              (*::std::mem::transmute::<&[u8; 53],
                                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                              1256i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 73],
                                                                        &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                            };
                            stack[ssize as usize].a =
                                ISAd.offset(incr as isize);
                            stack[ssize as usize].b = first;
                            stack[ssize as usize].c = a;
                            stack[ssize as usize].d = next;
                            let fresh14 = ssize;
                            ssize = ssize + 1;
                            stack[fresh14 as usize].e = trlink;
                            first = a;
                            limit = -3i32
                        } else {
                            ISAd = ISAd.offset(incr as isize);
                            last = a;
                            limit = next
                        }
                    } else {
                        if 0i32 <= trlink { stack[trlink as usize].d = -1i32 }
                        if (1i32 as libc::c_long) <
                               a.offset_to(last).expect("bad offset_to") as
                                   libc::c_long {
                            first = a;
                            limit = -3i32
                        } else {
                            if 0i32 <= ssize {
                            } else {
                                __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                                        &mut [libc::c_char; 11]>(b"0 <= ssize\x00")).as_mut_ptr(),
                                              (*::std::mem::transmute::<&[u8; 53],
                                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                              1267i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 73],
                                                                        &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                            };
                            if ssize == 0i32 {
                                return
                            } else {
                                ssize -= 1;
                                ISAd = stack[ssize as usize].a;
                                first = stack[ssize as usize].b;
                                last = stack[ssize as usize].c;
                                limit = stack[ssize as usize].d;
                                trlink = stack[ssize as usize].e
                            }
                        }
                    }
                } else {
                    if 0i32 <= ssize {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                                &mut [libc::c_char; 11]>(b"0 <= ssize\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1271i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 73],
                                                                &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                    };
                    if ssize == 0i32 {
                        return
                    } else {
                        ssize -= 1;
                        ISAd = stack[ssize as usize].a;
                        first = stack[ssize as usize].b;
                        last = stack[ssize as usize].c;
                        limit = stack[ssize as usize].d;
                        trlink = stack[ssize as usize].e
                    }
                }
            }
        } else if first.offset_to(last).expect("bad offset_to") as
                      libc::c_long <= 8i32 as libc::c_long {
            tr_insertionsort(ISAd, first, last);
            limit = -3i32
        } else {
            let fresh15 = limit;
            limit = limit - 1;
            if fresh15 == 0i32 {
                tr_heapsort(ISAd, first,
                            first.offset_to(last).expect("bad offset_to") as
                                libc::c_long as libc::c_int);
                a = last.offset(-1isize);
                while first < a {
                    x = *ISAd.offset(*a as isize);
                    b = a.offset(-1isize);
                    while first <= b && *ISAd.offset(*b as isize) == x {
                        *b = !*b;
                        b = b.offset(-1isize)
                    }
                    a = b
                }
                limit = -3i32
            } else {
                a = tr_pivot(ISAd, first, last);
                t = *first;
                *first = *a;
                *a = t;
                v = *ISAd.offset(*first as isize);
                tr_partition(ISAd, first, first.offset(1isize), last,
                             &mut a as *mut *mut libc::c_int,
                             &mut b as *mut *mut libc::c_int, v);
                if first.offset_to(last).expect("bad offset_to") as
                       libc::c_long !=
                       a.offset_to(b).expect("bad offset_to") as libc::c_long
                   {
                    next =
                        if *ISA.offset(*a as isize) != v {
                            tr_ilg(a.offset_to(b).expect("bad offset_to") as
                                       libc::c_long as libc::c_int)
                        } else { -1i32 };
                    c = first;
                    v =
                        (SA.offset_to(a).expect("bad offset_to") as
                             libc::c_long - 1i32 as libc::c_long) as
                            libc::c_int;
                    while c < a {
                        *ISA.offset(*c as isize) = v;
                        c = c.offset(1isize)
                    }
                    if b < last {
                        c = a;
                        v =
                            (SA.offset_to(b).expect("bad offset_to") as
                                 libc::c_long - 1i32 as libc::c_long) as
                                libc::c_int;
                        while c < b {
                            *ISA.offset(*c as isize) = v;
                            c = c.offset(1isize)
                        }
                    }
                    if (1i32 as libc::c_long) <
                           a.offset_to(b).expect("bad offset_to") as
                               libc::c_long &&
                           0 !=
                               trbudget_check(budget,
                                              a.offset_to(b).expect("bad offset_to")
                                                  as libc::c_long as
                                                  libc::c_int) {
                        if first.offset_to(a).expect("bad offset_to") as
                               libc::c_long <=
                               b.offset_to(last).expect("bad offset_to") as
                                   libc::c_long {
                            if b.offset_to(last).expect("bad offset_to") as
                                   libc::c_long <=
                                   a.offset_to(b).expect("bad offset_to") as
                                       libc::c_long {
                                if (1i32 as libc::c_long) <
                                       first.offset_to(a).expect("bad offset_to")
                                           as libc::c_long {
                                    if ssize < 64i32 {
                                    } else {
                                        __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                                &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                                      (*::std::mem::transmute::<&[u8; 53],
                                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                      1311i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 73],
                                                                                &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                    };
                                    stack[ssize as usize].a =
                                        ISAd.offset(incr as isize);
                                    stack[ssize as usize].b = a;
                                    stack[ssize as usize].c = b;
                                    stack[ssize as usize].d = next;
                                    let fresh16 = ssize;
                                    ssize = ssize + 1;
                                    stack[fresh16 as usize].e = trlink;
                                    if ssize < 64i32 {
                                    } else {
                                        __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                                &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                                      (*::std::mem::transmute::<&[u8; 53],
                                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                      1312i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 73],
                                                                                &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                    };
                                    stack[ssize as usize].a = ISAd;
                                    stack[ssize as usize].b = b;
                                    stack[ssize as usize].c = last;
                                    stack[ssize as usize].d = limit;
                                    let fresh17 = ssize;
                                    ssize = ssize + 1;
                                    stack[fresh17 as usize].e = trlink;
                                    last = a
                                } else if (1i32 as libc::c_long) <
                                              b.offset_to(last).expect("bad offset_to")
                                                  as libc::c_long {
                                    if ssize < 64i32 {
                                    } else {
                                        __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                                &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                                      (*::std::mem::transmute::<&[u8; 53],
                                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                      1315i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 73],
                                                                                &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                    };
                                    stack[ssize as usize].a =
                                        ISAd.offset(incr as isize);
                                    stack[ssize as usize].b = a;
                                    stack[ssize as usize].c = b;
                                    stack[ssize as usize].d = next;
                                    let fresh18 = ssize;
                                    ssize = ssize + 1;
                                    stack[fresh18 as usize].e = trlink;
                                    first = b
                                } else {
                                    ISAd = ISAd.offset(incr as isize);
                                    first = a;
                                    last = b;
                                    limit = next
                                }
                            } else if first.offset_to(a).expect("bad offset_to")
                                          as libc::c_long <=
                                          a.offset_to(b).expect("bad offset_to")
                                              as libc::c_long {
                                if (1i32 as libc::c_long) <
                                       first.offset_to(a).expect("bad offset_to")
                                           as libc::c_long {
                                    if ssize < 64i32 {
                                    } else {
                                        __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                                &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                                      (*::std::mem::transmute::<&[u8; 53],
                                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                      1322i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 73],
                                                                                &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                    };
                                    stack[ssize as usize].a = ISAd;
                                    stack[ssize as usize].b = b;
                                    stack[ssize as usize].c = last;
                                    stack[ssize as usize].d = limit;
                                    let fresh19 = ssize;
                                    ssize = ssize + 1;
                                    stack[fresh19 as usize].e = trlink;
                                    if ssize < 64i32 {
                                    } else {
                                        __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                                &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                                      (*::std::mem::transmute::<&[u8; 53],
                                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                      1323i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 73],
                                                                                &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                    };
                                    stack[ssize as usize].a =
                                        ISAd.offset(incr as isize);
                                    stack[ssize as usize].b = a;
                                    stack[ssize as usize].c = b;
                                    stack[ssize as usize].d = next;
                                    let fresh20 = ssize;
                                    ssize = ssize + 1;
                                    stack[fresh20 as usize].e = trlink;
                                    last = a
                                } else {
                                    if ssize < 64i32 {
                                    } else {
                                        __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                                &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                                      (*::std::mem::transmute::<&[u8; 53],
                                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                      1326i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 73],
                                                                                &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                    };
                                    stack[ssize as usize].a = ISAd;
                                    stack[ssize as usize].b = b;
                                    stack[ssize as usize].c = last;
                                    stack[ssize as usize].d = limit;
                                    let fresh21 = ssize;
                                    ssize = ssize + 1;
                                    stack[fresh21 as usize].e = trlink;
                                    ISAd = ISAd.offset(incr as isize);
                                    first = a;
                                    last = b;
                                    limit = next
                                }
                            } else {
                                if ssize < 64i32 {
                                } else {
                                    __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                            &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                                  (*::std::mem::transmute::<&[u8; 53],
                                                                            &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                  1330i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 73],
                                                                            &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                };
                                stack[ssize as usize].a = ISAd;
                                stack[ssize as usize].b = b;
                                stack[ssize as usize].c = last;
                                stack[ssize as usize].d = limit;
                                let fresh22 = ssize;
                                ssize = ssize + 1;
                                stack[fresh22 as usize].e = trlink;
                                if ssize < 64i32 {
                                } else {
                                    __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                            &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                                  (*::std::mem::transmute::<&[u8; 53],
                                                                            &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                  1331i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 73],
                                                                            &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                };
                                stack[ssize as usize].a = ISAd;
                                stack[ssize as usize].b = first;
                                stack[ssize as usize].c = a;
                                stack[ssize as usize].d = limit;
                                let fresh23 = ssize;
                                ssize = ssize + 1;
                                stack[fresh23 as usize].e = trlink;
                                ISAd = ISAd.offset(incr as isize);
                                first = a;
                                last = b;
                                limit = next
                            }
                        } else if first.offset_to(a).expect("bad offset_to")
                                      as libc::c_long <=
                                      a.offset_to(b).expect("bad offset_to")
                                          as libc::c_long {
                            if (1i32 as libc::c_long) <
                                   b.offset_to(last).expect("bad offset_to")
                                       as libc::c_long {
                                if ssize < 64i32 {
                                } else {
                                    __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                            &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                                  (*::std::mem::transmute::<&[u8; 53],
                                                                            &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                  1337i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 73],
                                                                            &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                };
                                stack[ssize as usize].a =
                                    ISAd.offset(incr as isize);
                                stack[ssize as usize].b = a;
                                stack[ssize as usize].c = b;
                                stack[ssize as usize].d = next;
                                let fresh24 = ssize;
                                ssize = ssize + 1;
                                stack[fresh24 as usize].e = trlink;
                                if ssize < 64i32 {
                                } else {
                                    __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                            &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                                  (*::std::mem::transmute::<&[u8; 53],
                                                                            &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                  1338i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 73],
                                                                            &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                };
                                stack[ssize as usize].a = ISAd;
                                stack[ssize as usize].b = first;
                                stack[ssize as usize].c = a;
                                stack[ssize as usize].d = limit;
                                let fresh25 = ssize;
                                ssize = ssize + 1;
                                stack[fresh25 as usize].e = trlink;
                                first = b
                            } else if (1i32 as libc::c_long) <
                                          first.offset_to(a).expect("bad offset_to")
                                              as libc::c_long {
                                if ssize < 64i32 {
                                } else {
                                    __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                            &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                                  (*::std::mem::transmute::<&[u8; 53],
                                                                            &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                  1341i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 73],
                                                                            &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                };
                                stack[ssize as usize].a =
                                    ISAd.offset(incr as isize);
                                stack[ssize as usize].b = a;
                                stack[ssize as usize].c = b;
                                stack[ssize as usize].d = next;
                                let fresh26 = ssize;
                                ssize = ssize + 1;
                                stack[fresh26 as usize].e = trlink;
                                last = a
                            } else {
                                ISAd = ISAd.offset(incr as isize);
                                first = a;
                                last = b;
                                limit = next
                            }
                        } else if b.offset_to(last).expect("bad offset_to") as
                                      libc::c_long <=
                                      a.offset_to(b).expect("bad offset_to")
                                          as libc::c_long {
                            if (1i32 as libc::c_long) <
                                   b.offset_to(last).expect("bad offset_to")
                                       as libc::c_long {
                                if ssize < 64i32 {
                                } else {
                                    __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                            &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                                  (*::std::mem::transmute::<&[u8; 53],
                                                                            &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                  1348i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 73],
                                                                            &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                };
                                stack[ssize as usize].a = ISAd;
                                stack[ssize as usize].b = first;
                                stack[ssize as usize].c = a;
                                stack[ssize as usize].d = limit;
                                let fresh27 = ssize;
                                ssize = ssize + 1;
                                stack[fresh27 as usize].e = trlink;
                                if ssize < 64i32 {
                                } else {
                                    __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                            &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                                  (*::std::mem::transmute::<&[u8; 53],
                                                                            &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                  1349i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 73],
                                                                            &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                };
                                stack[ssize as usize].a =
                                    ISAd.offset(incr as isize);
                                stack[ssize as usize].b = a;
                                stack[ssize as usize].c = b;
                                stack[ssize as usize].d = next;
                                let fresh28 = ssize;
                                ssize = ssize + 1;
                                stack[fresh28 as usize].e = trlink;
                                first = b
                            } else {
                                if ssize < 64i32 {
                                } else {
                                    __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                            &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                                  (*::std::mem::transmute::<&[u8; 53],
                                                                            &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                  1352i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 73],
                                                                            &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                };
                                stack[ssize as usize].a = ISAd;
                                stack[ssize as usize].b = first;
                                stack[ssize as usize].c = a;
                                stack[ssize as usize].d = limit;
                                let fresh29 = ssize;
                                ssize = ssize + 1;
                                stack[fresh29 as usize].e = trlink;
                                ISAd = ISAd.offset(incr as isize);
                                first = a;
                                last = b;
                                limit = next
                            }
                        } else {
                            if ssize < 64i32 {
                            } else {
                                __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                        &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                              (*::std::mem::transmute::<&[u8; 53],
                                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                              1356i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 73],
                                                                        &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                            };
                            stack[ssize as usize].a = ISAd;
                            stack[ssize as usize].b = first;
                            stack[ssize as usize].c = a;
                            stack[ssize as usize].d = limit;
                            let fresh30 = ssize;
                            ssize = ssize + 1;
                            stack[fresh30 as usize].e = trlink;
                            if ssize < 64i32 {
                            } else {
                                __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                        &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                              (*::std::mem::transmute::<&[u8; 53],
                                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                              1357i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 73],
                                                                        &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                            };
                            stack[ssize as usize].a = ISAd;
                            stack[ssize as usize].b = b;
                            stack[ssize as usize].c = last;
                            stack[ssize as usize].d = limit;
                            let fresh31 = ssize;
                            ssize = ssize + 1;
                            stack[fresh31 as usize].e = trlink;
                            ISAd = ISAd.offset(incr as isize);
                            first = a;
                            last = b;
                            limit = next
                        }
                    } else {
                        if (1i32 as libc::c_long) <
                               a.offset_to(b).expect("bad offset_to") as
                                   libc::c_long && 0i32 <= trlink {
                            stack[trlink as usize].d = -1i32
                        }
                        if first.offset_to(a).expect("bad offset_to") as
                               libc::c_long <=
                               b.offset_to(last).expect("bad offset_to") as
                                   libc::c_long {
                            if (1i32 as libc::c_long) <
                                   first.offset_to(a).expect("bad offset_to")
                                       as libc::c_long {
                                if ssize < 64i32 {
                                } else {
                                    __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                            &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                                  (*::std::mem::transmute::<&[u8; 53],
                                                                            &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                  1365i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 73],
                                                                            &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                };
                                stack[ssize as usize].a = ISAd;
                                stack[ssize as usize].b = b;
                                stack[ssize as usize].c = last;
                                stack[ssize as usize].d = limit;
                                let fresh32 = ssize;
                                ssize = ssize + 1;
                                stack[fresh32 as usize].e = trlink;
                                last = a
                            } else if (1i32 as libc::c_long) <
                                          b.offset_to(last).expect("bad offset_to")
                                              as libc::c_long {
                                first = b
                            } else {
                                if 0i32 <= ssize {
                                } else {
                                    __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                                            &mut [libc::c_char; 11]>(b"0 <= ssize\x00")).as_mut_ptr(),
                                                  (*::std::mem::transmute::<&[u8; 53],
                                                                            &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                                  1370i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 73],
                                                                            &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                                };
                                if ssize == 0i32 {
                                    return
                                } else {
                                    ssize -= 1;
                                    ISAd = stack[ssize as usize].a;
                                    first = stack[ssize as usize].b;
                                    last = stack[ssize as usize].c;
                                    limit = stack[ssize as usize].d;
                                    trlink = stack[ssize as usize].e
                                }
                            }
                        } else if (1i32 as libc::c_long) <
                                      b.offset_to(last).expect("bad offset_to")
                                          as libc::c_long {
                            if ssize < 64i32 {
                            } else {
                                __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                        &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                              (*::std::mem::transmute::<&[u8; 53],
                                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                              1374i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 73],
                                                                        &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                            };
                            stack[ssize as usize].a = ISAd;
                            stack[ssize as usize].b = first;
                            stack[ssize as usize].c = a;
                            stack[ssize as usize].d = limit;
                            let fresh33 = ssize;
                            ssize = ssize + 1;
                            stack[fresh33 as usize].e = trlink;
                            first = b
                        } else if (1i32 as libc::c_long) <
                                      first.offset_to(a).expect("bad offset_to")
                                          as libc::c_long {
                            last = a
                        } else {
                            if 0i32 <= ssize {
                            } else {
                                __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                                        &mut [libc::c_char; 11]>(b"0 <= ssize\x00")).as_mut_ptr(),
                                              (*::std::mem::transmute::<&[u8; 53],
                                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                              1379i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 73],
                                                                        &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                            };
                            if ssize == 0i32 {
                                return
                            } else {
                                ssize -= 1;
                                ISAd = stack[ssize as usize].a;
                                first = stack[ssize as usize].b;
                                last = stack[ssize as usize].c;
                                limit = stack[ssize as usize].d;
                                trlink = stack[ssize as usize].e
                            }
                        }
                    }
                } else if 0 !=
                              trbudget_check(budget,
                                             first.offset_to(last).expect("bad offset_to")
                                                 as libc::c_long as
                                                 libc::c_int) {
                    limit =
                        tr_ilg(first.offset_to(last).expect("bad offset_to")
                                   as libc::c_long as libc::c_int);
                    ISAd = ISAd.offset(incr as isize)
                } else {
                    if 0i32 <= trlink { stack[trlink as usize].d = -1i32 }
                    if 0i32 <= ssize {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                                &mut [libc::c_char; 11]>(b"0 <= ssize\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1388i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 73],
                                                                &[libc::c_char; 73]>(b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\x00")).as_ptr());
                    };
                    if ssize == 0i32 {
                        return
                    } else {
                        ssize -= 1;
                        ISAd = stack[ssize as usize].a;
                        first = stack[ssize as usize].b;
                        last = stack[ssize as usize].c;
                        limit = stack[ssize as usize].d;
                        trlink = stack[ssize as usize].e
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn tr_ilg(mut n: libc::c_int) -> libc::c_int {
    return if 0 != n as libc::c_uint & 4294901760u32 {
               if 0 != n as libc::c_uint & 4278190080u32 {
                   24i32 + lg_table[(n >> 24i32 & 255i32) as usize]
               } else { 16i32 + lg_table[(n >> 16i32 & 255i32) as usize] }
           } else if 0 != n & 65280i32 {
               8i32 + lg_table[(n >> 8i32 & 255i32) as usize]
           } else { 0i32 + lg_table[(n >> 0i32 & 255i32) as usize] };
}
static mut lg_table: [libc::c_int; 256] =
    unsafe {
        [-1i32, 0i32, 1i32, 1i32, 2i32, 2i32, 2i32, 2i32, 3i32, 3i32, 3i32,
         3i32, 3i32, 3i32, 3i32, 3i32, 4i32, 4i32, 4i32, 4i32, 4i32, 4i32,
         4i32, 4i32, 4i32, 4i32, 4i32, 4i32, 4i32, 4i32, 4i32, 4i32, 5i32,
         5i32, 5i32, 5i32, 5i32, 5i32, 5i32, 5i32, 5i32, 5i32, 5i32, 5i32,
         5i32, 5i32, 5i32, 5i32, 5i32, 5i32, 5i32, 5i32, 5i32, 5i32, 5i32,
         5i32, 5i32, 5i32, 5i32, 5i32, 5i32, 5i32, 5i32, 5i32, 6i32, 6i32,
         6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32,
         6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32,
         6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32,
         6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32,
         6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32,
         6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 7i32, 7i32, 7i32, 7i32,
         7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32,
         7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32,
         7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32,
         7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32,
         7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32,
         7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32,
         7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32,
         7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32,
         7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32,
         7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32,
         7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32, 7i32,
         7i32, 7i32, 7i32]
    };
unsafe extern "C" fn trbudget_check(mut budget: *mut trbudget_t,
                                    mut size: libc::c_int) -> libc::c_int {
    if size <= (*budget).remain {
        (*budget).remain -= size;
        return 1i32
    } else if (*budget).chance == 0i32 {
        (*budget).count += size;
        return 0i32
    } else {
        (*budget).remain += (*budget).incval - size;
        (*budget).chance -= 1i32;
        return 1i32
    };
}
unsafe extern "C" fn tr_partition(mut ISAd: *const libc::c_int,
                                  mut first: *mut libc::c_int,
                                  mut middle: *mut libc::c_int,
                                  mut last: *mut libc::c_int,
                                  mut pa: *mut *mut libc::c_int,
                                  mut pb: *mut *mut libc::c_int,
                                  mut v: libc::c_int) -> () {
    let mut a: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut b: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut c: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut d: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut e: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut f: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut x: libc::c_int = 0i32;
    b = middle.offset(-1isize);
    loop  {
        b = b.offset(1isize);
        if !(b < last && { x = *ISAd.offset(*b as isize); x == v }) {
            break ;
        }
    }
    a = b;
    if a < last && x < v {
        loop  {
            b = b.offset(1isize);
            if !(b < last && { x = *ISAd.offset(*b as isize); x <= v }) {
                break ;
            }
            if !(x == v) { continue ; }
            t = *b;
            *b = *a;
            *a = t;
            a = a.offset(1isize)
        }
    }
    c = last;
    loop  {
        c = c.offset(-1isize);
        if !(b < c && { x = *ISAd.offset(*c as isize); x == v }) { break ; }
    }
    d = c;
    if b < d && x > v {
        loop  {
            c = c.offset(-1isize);
            if !(b < c && { x = *ISAd.offset(*c as isize); x >= v }) {
                break ;
            }
            if !(x == v) { continue ; }
            t = *c;
            *c = *d;
            *d = t;
            d = d.offset(-1isize)
        }
    }
    while b < c {
        t = *b;
        *b = *c;
        *c = t;
        loop  {
            b = b.offset(1isize);
            if !(b < c && { x = *ISAd.offset(*b as isize); x <= v }) {
                break ;
            }
            if !(x == v) { continue ; }
            t = *b;
            *b = *a;
            *a = t;
            a = a.offset(1isize)
        }
        loop  {
            c = c.offset(-1isize);
            if !(b < c && { x = *ISAd.offset(*c as isize); x >= v }) {
                break ;
            }
            if !(x == v) { continue ; }
            t = *c;
            *c = *d;
            *d = t;
            d = d.offset(-1isize)
        }
    }
    if a <= d {
        c = b.offset(-1isize);
        s =
            first.offset_to(a).expect("bad offset_to") as libc::c_long as
                libc::c_int;
        t =
            a.offset_to(b).expect("bad offset_to") as libc::c_long as
                libc::c_int;
        if s > t { s = t }
        e = first;
        f = b.offset(-(s as isize));
        while 0i32 < s {
            t = *e;
            *e = *f;
            *f = t;
            s -= 1;
            e = e.offset(1isize);
            f = f.offset(1isize)
        }
        s =
            c.offset_to(d).expect("bad offset_to") as libc::c_long as
                libc::c_int;
        t =
            (d.offset_to(last).expect("bad offset_to") as libc::c_long -
                 1i32 as libc::c_long) as libc::c_int;
        if s > t { s = t }
        e = b;
        f = last.offset(-(s as isize));
        while 0i32 < s {
            t = *e;
            *e = *f;
            *f = t;
            s -= 1;
            e = e.offset(1isize);
            f = f.offset(1isize)
        }
        first =
            first.offset(a.offset_to(b).expect("bad offset_to") as
                             libc::c_long as isize);
        last =
            last.offset(-(c.offset_to(d).expect("bad offset_to") as
                              libc::c_long as isize))
    }
    *pa = first;
    *pb = last;
}
unsafe extern "C" fn tr_pivot(mut ISAd: *const libc::c_int,
                              mut first: *mut libc::c_int,
                              mut last: *mut libc::c_int)
 -> *mut libc::c_int {
    let mut middle: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    t =
        first.offset_to(last).expect("bad offset_to") as libc::c_long as
            libc::c_int;
    middle = first.offset((t / 2i32) as isize);
    if t <= 512i32 {
        if t <= 32i32 {
            return tr_median3(ISAd, first, middle, last.offset(-1isize))
        } else {
            t >>= 2i32;
            return tr_median5(ISAd, first, first.offset(t as isize), middle,
                              last.offset(-1isize).offset(-(t as isize)),
                              last.offset(-1isize))
        }
    } else {
        t >>= 3i32;
        first =
            tr_median3(ISAd, first, first.offset(t as isize),
                       first.offset((t << 1i32) as isize));
        middle =
            tr_median3(ISAd, middle.offset(-(t as isize)), middle,
                       middle.offset(t as isize));
        last =
            tr_median3(ISAd,
                       last.offset(-1isize).offset(-((t << 1i32) as isize)),
                       last.offset(-1isize).offset(-(t as isize)),
                       last.offset(-1isize));
        return tr_median3(ISAd, first, middle, last)
    };
}
unsafe extern "C" fn tr_median3(mut ISAd: *const libc::c_int,
                                mut v1: *mut libc::c_int,
                                mut v2: *mut libc::c_int,
                                mut v3: *mut libc::c_int)
 -> *mut libc::c_int {
    let mut t: *mut libc::c_int = 0 as *mut libc::c_int;
    if *ISAd.offset(*v1 as isize) > *ISAd.offset(*v2 as isize) {
        t = v1;
        v1 = v2;
        v2 = t
    }
    if *ISAd.offset(*v2 as isize) > *ISAd.offset(*v3 as isize) {
        if *ISAd.offset(*v1 as isize) > *ISAd.offset(*v3 as isize) {
            return v1
        } else { return v3 }
    } else { return v2 };
}
unsafe extern "C" fn tr_median5(mut ISAd: *const libc::c_int,
                                mut v1: *mut libc::c_int,
                                mut v2: *mut libc::c_int,
                                mut v3: *mut libc::c_int,
                                mut v4: *mut libc::c_int,
                                mut v5: *mut libc::c_int)
 -> *mut libc::c_int {
    let mut t: *mut libc::c_int = 0 as *mut libc::c_int;
    if *ISAd.offset(*v2 as isize) > *ISAd.offset(*v3 as isize) {
        t = v2;
        v2 = v3;
        v3 = t
    }
    if *ISAd.offset(*v4 as isize) > *ISAd.offset(*v5 as isize) {
        t = v4;
        v4 = v5;
        v5 = t
    }
    if *ISAd.offset(*v2 as isize) > *ISAd.offset(*v4 as isize) {
        t = v2;
        v2 = v4;
        v4 = t;
        t = v3;
        v3 = v5;
        v5 = t
    }
    if *ISAd.offset(*v1 as isize) > *ISAd.offset(*v3 as isize) {
        t = v1;
        v1 = v3;
        v3 = t
    }
    if *ISAd.offset(*v1 as isize) > *ISAd.offset(*v4 as isize) {
        t = v1;
        v1 = v4;
        v4 = t;
        t = v3;
        v3 = v5;
        v5 = t
    }
    if *ISAd.offset(*v3 as isize) > *ISAd.offset(*v4 as isize) {
        return v4
    } else { return v3 };
}
unsafe extern "C" fn tr_heapsort(mut ISAd: *const libc::c_int,
                                 mut SA: *mut libc::c_int,
                                 mut size: libc::c_int) -> () {
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    m = size;
    if size % 2i32 == 0i32 {
        m -= 1;
        if *ISAd.offset(*SA.offset((m / 2i32) as isize) as isize) <
               *ISAd.offset(*SA.offset(m as isize) as isize) {
            t = *SA.offset(m as isize);
            *SA.offset(m as isize) = *SA.offset((m / 2i32) as isize);
            *SA.offset((m / 2i32) as isize) = t
        }
    }
    i = m / 2i32 - 1i32;
    while 0i32 <= i { tr_fixdown(ISAd, SA, i, m); i -= 1 }
    if size % 2i32 == 0i32 {
        t = *SA.offset(0isize);
        *SA.offset(0isize) = *SA.offset(m as isize);
        *SA.offset(m as isize) = t;
        tr_fixdown(ISAd, SA, 0i32, m);
    }
    i = m - 1i32;
    while 0i32 < i {
        t = *SA.offset(0isize);
        *SA.offset(0isize) = *SA.offset(i as isize);
        tr_fixdown(ISAd, SA, 0i32, i);
        *SA.offset(i as isize) = t;
        i -= 1
    };
}
unsafe extern "C" fn tr_fixdown(mut ISAd: *const libc::c_int,
                                mut SA: *mut libc::c_int, mut i: libc::c_int,
                                mut size: libc::c_int) -> () {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    v = *SA.offset(i as isize);
    c = *ISAd.offset(v as isize);
    loop  {
        j = 2i32 * i + 1i32;
        if !(j < size) { break ; }
        let fresh34 = j;
        j = j + 1;
        k = fresh34;
        d = *ISAd.offset(*SA.offset(k as isize) as isize);
        e = *ISAd.offset(*SA.offset(j as isize) as isize);
        if d < e { k = j; d = e }
        if d <= c { break ; }
        *SA.offset(i as isize) = *SA.offset(k as isize);
        i = k
    }
    *SA.offset(i as isize) = v;
}
unsafe extern "C" fn tr_insertionsort(mut ISAd: *const libc::c_int,
                                      mut first: *mut libc::c_int,
                                      mut last: *mut libc::c_int) -> () {
    let mut a: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut b: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    a = first.offset(1isize);
    while a < last {
        t = *a;
        b = a.offset(-1isize);
        loop  {
            r = *ISAd.offset(t as isize) - *ISAd.offset(*b as isize);
            if !(0i32 > r) { break ; }
            loop  {
                *b.offset(1isize) = *b;
                b = b.offset(-1isize);
                if !(first <= b && *b < 0i32) { break ; }
            }
            if b < first { break ; }
        }
        if r == 0i32 { *b = !*b }
        *b.offset(1isize) = t;
        a = a.offset(1isize)
    };
}
unsafe extern "C" fn tr_partialcopy(mut ISA: *mut libc::c_int,
                                    mut SA: *const libc::c_int,
                                    mut first: *mut libc::c_int,
                                    mut a: *mut libc::c_int,
                                    mut b: *mut libc::c_int,
                                    mut last: *mut libc::c_int,
                                    mut depth: libc::c_int) -> () {
    let mut c: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut d: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut e: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut s: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut rank: libc::c_int = 0;
    let mut lastrank: libc::c_int = 0;
    let mut newrank: libc::c_int = -1i32;
    v =
        (SA.offset_to(b).expect("bad offset_to") as libc::c_long -
             1i32 as libc::c_long) as libc::c_int;
    lastrank = -1i32;
    c = first;
    d = a.offset(-1isize);
    while c <= d {
        s = *c - depth;
        if 0i32 <= s && *ISA.offset(s as isize) == v {
            d = d.offset(1isize);
            *d = s;
            rank = *ISA.offset((s + depth) as isize);
            if lastrank != rank {
                lastrank = rank;
                newrank =
                    SA.offset_to(d).expect("bad offset_to") as libc::c_long as
                        libc::c_int
            }
            *ISA.offset(s as isize) = newrank
        }
        c = c.offset(1isize)
    }
    lastrank = -1i32;
    e = d;
    while first <= e {
        rank = *ISA.offset(*e as isize);
        if lastrank != rank {
            lastrank = rank;
            newrank =
                SA.offset_to(e).expect("bad offset_to") as libc::c_long as
                    libc::c_int
        }
        if newrank != rank { *ISA.offset(*e as isize) = newrank }
        e = e.offset(-1isize)
    }
    lastrank = -1i32;
    c = last.offset(-1isize);
    e = d.offset(1isize);
    d = b;
    while e < d {
        s = *c - depth;
        if 0i32 <= s && *ISA.offset(s as isize) == v {
            d = d.offset(-1isize);
            *d = s;
            rank = *ISA.offset((s + depth) as isize);
            if lastrank != rank {
                lastrank = rank;
                newrank =
                    SA.offset_to(d).expect("bad offset_to") as libc::c_long as
                        libc::c_int
            }
            *ISA.offset(s as isize) = newrank
        }
        c = c.offset(-1isize)
    };
}
unsafe extern "C" fn tr_copy(mut ISA: *mut libc::c_int,
                             mut SA: *const libc::c_int,
                             mut first: *mut libc::c_int,
                             mut a: *mut libc::c_int, mut b: *mut libc::c_int,
                             mut last: *mut libc::c_int,
                             mut depth: libc::c_int) -> () {
    let mut c: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut d: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut e: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut s: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    v =
        (SA.offset_to(b).expect("bad offset_to") as libc::c_long -
             1i32 as libc::c_long) as libc::c_int;
    c = first;
    d = a.offset(-1isize);
    while c <= d {
        s = *c - depth;
        if 0i32 <= s && *ISA.offset(s as isize) == v {
            d = d.offset(1isize);
            *d = s;
            *ISA.offset(s as isize) =
                SA.offset_to(d).expect("bad offset_to") as libc::c_long as
                    libc::c_int
        }
        c = c.offset(1isize)
    }
    c = last.offset(-1isize);
    e = d.offset(1isize);
    d = b;
    while e < d {
        s = *c - depth;
        if 0i32 <= s && *ISA.offset(s as isize) == v {
            d = d.offset(-1isize);
            *d = s;
            *ISA.offset(s as isize) =
                SA.offset_to(d).expect("bad offset_to") as libc::c_long as
                    libc::c_int
        }
        c = c.offset(-1isize)
    };
}
unsafe extern "C" fn trbudget_init(mut budget: *mut trbudget_t,
                                   mut chance: libc::c_int,
                                   mut incval: libc::c_int) -> () {
    (*budget).chance = chance;
    (*budget).incval = incval;
    (*budget).remain = (*budget).incval;
}
unsafe extern "C" fn sssort(mut T: *const libc::c_uchar,
                            mut PA: *const libc::c_int,
                            mut first: *mut libc::c_int,
                            mut last: *mut libc::c_int,
                            mut buf: *mut libc::c_int,
                            mut bufsize: libc::c_int, mut depth: libc::c_int,
                            mut n: libc::c_int, mut lastsuffix: libc::c_int)
 -> () {
    let mut a: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut b: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut middle: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut curbuf: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut curbufsize: libc::c_int = 0;
    let mut limit: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if lastsuffix != 0i32 { first = first.offset(1isize) }
    if bufsize < 1024i32 &&
           (bufsize as libc::c_long) <
               first.offset_to(last).expect("bad offset_to") as libc::c_long
           &&
           {
               limit =
                   ss_isqrt(first.offset_to(last).expect("bad offset_to") as
                                libc::c_long as libc::c_int);
               bufsize < limit
           } {
        if 1024i32 < limit { limit = 1024i32 }
        middle = last.offset(-(limit as isize));
        buf = middle;
        bufsize = limit
    } else { middle = last; limit = 0i32 }
    a = first;
    i = 0i32;
    while (1024i32 as libc::c_long) <
              a.offset_to(middle).expect("bad offset_to") as libc::c_long {
        ss_mintrosort(T, PA, a, a.offset(1024isize), depth);
        curbufsize =
            a.offset(1024isize).offset_to(last).expect("bad offset_to") as
                libc::c_long as libc::c_int;
        curbuf = a.offset(1024isize);
        if curbufsize <= bufsize { curbufsize = bufsize; curbuf = buf }
        b = a;
        k = 1024i32;
        j = i;
        while 0 != j & 1i32 {
            ss_swapmerge(T, PA, b.offset(-(k as isize)), b,
                         b.offset(k as isize), curbuf, curbufsize, depth);
            b = b.offset(-(k as isize));
            k <<= 1i32;
            j >>= 1i32
        }
        a = a.offset(1024isize);
        i += 1
    }
    ss_mintrosort(T, PA, a, middle, depth);
    k = 1024i32;
    while i != 0i32 {
        if 0 != i & 1i32 {
            ss_swapmerge(T, PA, a.offset(-(k as isize)), a, middle, buf,
                         bufsize, depth);
            a = a.offset(-(k as isize))
        }
        k <<= 1i32;
        i >>= 1i32
    }
    if limit != 0i32 {
        ss_mintrosort(T, PA, middle, last, depth);
        ss_inplacemerge(T, PA, first, middle, last, depth);
    }
    if lastsuffix != 0i32 {
        let mut PAi: [libc::c_int; 2] = [0; 2];
        PAi[0usize] = *PA.offset(*first.offset(-1isize) as isize);
        PAi[1usize] = n - 2i32;
        a = first;
        i = *first.offset(-1isize);
        while a < last &&
                  (*a < 0i32 ||
                       0i32 <
                           ss_compare(T, &mut PAi[0usize] as *mut libc::c_int,
                                      PA.offset(*a as isize), depth)) {
            *a.offset(-1isize) = *a;
            a = a.offset(1isize)
        }
        *a.offset(-1isize) = i
    };
}
unsafe extern "C" fn ss_compare(mut T: *const libc::c_uchar,
                                mut p1: *const libc::c_int,
                                mut p2: *const libc::c_int,
                                mut depth: libc::c_int) -> libc::c_int {
    let mut U1: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut U2: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut U1n: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut U2n: *const libc::c_uchar = 0 as *const libc::c_uchar;
    U1 = T.offset(depth as isize).offset(*p1 as isize);
    U2 = T.offset(depth as isize).offset(*p2 as isize);
    U1n = T.offset(*p1.offset(1isize) as isize).offset(2isize);
    U2n = T.offset(*p2.offset(1isize) as isize).offset(2isize);
    while U1 < U1n && U2 < U2n && *U1 as libc::c_int == *U2 as libc::c_int {
        U1 = U1.offset(1isize);
        U2 = U2.offset(1isize)
    }
    return if U1 < U1n {
               if U2 < U2n {
                   *U1 as libc::c_int - *U2 as libc::c_int
               } else { 1i32 }
           } else if U2 < U2n { -1i32 } else { 0i32 };
}
unsafe extern "C" fn ss_inplacemerge(mut T: *const libc::c_uchar,
                                     mut PA: *const libc::c_int,
                                     mut first: *mut libc::c_int,
                                     mut middle: *mut libc::c_int,
                                     mut last: *mut libc::c_int,
                                     mut depth: libc::c_int) -> () {
    let mut p: *const libc::c_int = 0 as *const libc::c_int;
    let mut a: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut b: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut len: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    loop  {
        if *last.offset(-1isize) < 0i32 {
            x = 1i32;
            p = PA.offset(!*last.offset(-1isize) as isize)
        } else { x = 0i32; p = PA.offset(*last.offset(-1isize) as isize) }
        a = first;
        len =
            first.offset_to(middle).expect("bad offset_to") as libc::c_long as
                libc::c_int;
        half = len >> 1i32;
        r = -1i32;
        while 0i32 < len {
            b = a.offset(half as isize);
            q =
                ss_compare(T,
                           PA.offset((if 0i32 <= *b { *b } else { !*b }) as
                                         isize), p, depth);
            if q < 0i32 {
                a = b.offset(1isize);
                half -= len & 1i32 ^ 1i32
            } else { r = q }
            len = half;
            half >>= 1i32
        }
        if a < middle {
            if r == 0i32 { *a = !*a }
            ss_rotate(a, middle, last);
            last =
                last.offset(-(a.offset_to(middle).expect("bad offset_to") as
                                  libc::c_long as isize));
            middle = a;
            if first == middle { break ; }
        }
        last = last.offset(-1isize);
        if x != 0i32 {
            loop  {
                last = last.offset(-1isize);
                if !(*last < 0i32) { break ; }
            }
        }
        if middle == last { break ; }
    };
}
unsafe extern "C" fn ss_rotate(mut first: *mut libc::c_int,
                               mut middle: *mut libc::c_int,
                               mut last: *mut libc::c_int) -> () {
    let mut a: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut b: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    l =
        first.offset_to(middle).expect("bad offset_to") as libc::c_long as
            libc::c_int;
    r =
        middle.offset_to(last).expect("bad offset_to") as libc::c_long as
            libc::c_int;
    while 0i32 < l && 0i32 < r {
        if l == r {
            ss_blockswap(first, middle, l);
            break ;
        } else if l < r {
            a = last.offset(-1isize);
            b = middle.offset(-1isize);
            t = *a;
            loop  {
                let fresh35 = a;
                a = a.offset(-1);
                *fresh35 = *b;
                let fresh36 = b;
                b = b.offset(-1);
                *fresh36 = *a;
                if !(b < first) { continue ; }
                *a = t;
                last = a;
                r -= l + 1i32;
                if r <= l { break ; }
                a = a.offset(-1isize);
                b = middle.offset(-1isize);
                t = *a
            }
        } else {
            a = first;
            b = middle;
            t = *a;
            loop  {
                let fresh37 = a;
                a = a.offset(1);
                *fresh37 = *b;
                let fresh38 = b;
                b = b.offset(1);
                *fresh38 = *a;
                if !(last <= b) { continue ; }
                *a = t;
                first = a.offset(1isize);
                l -= r + 1i32;
                if l <= r { break ; }
                a = a.offset(1isize);
                b = middle;
                t = *a
            }
        }
    };
}
unsafe extern "C" fn ss_blockswap(mut a: *mut libc::c_int,
                                  mut b: *mut libc::c_int, mut n: libc::c_int)
 -> () {
    let mut t: libc::c_int = 0;
    while 0i32 < n {
        t = *a;
        *a = *b;
        *b = t;
        n -= 1;
        a = a.offset(1isize);
        b = b.offset(1isize)
    };
}
unsafe extern "C" fn ss_mintrosort(mut T: *const libc::c_uchar,
                                   mut PA: *const libc::c_int,
                                   mut first: *mut libc::c_int,
                                   mut last: *mut libc::c_int,
                                   mut depth: libc::c_int) -> () {
    let mut stack: [unnamed_0; 16] =
        [unnamed_0{a: 0 as *mut libc::c_int,
                   b: 0 as *mut libc::c_int,
                   c: 0,
                   d: 0,}; 16];
    let mut Td: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut a: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut b: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut c: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut d: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut e: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut f: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut ssize: libc::c_int = 0;
    let mut limit: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut x: libc::c_int = 0i32;
    ssize = 0i32;
    limit =
        ss_ilg(first.offset_to(last).expect("bad offset_to") as libc::c_long
                   as libc::c_int);
    loop  {
        if first.offset_to(last).expect("bad offset_to") as libc::c_long <=
               8i32 as libc::c_long {
            if (1i32 as libc::c_long) <
                   first.offset_to(last).expect("bad offset_to") as
                       libc::c_long {
                ss_insertionsort(T, PA, first, last, depth);
            }
            if 0i32 <= ssize {
            } else {
                __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                        &mut [libc::c_char; 11]>(b"0 <= ssize\x00")).as_mut_ptr(),
                              (*::std::mem::transmute::<&[u8; 53],
                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                              418i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 74],
                                                        &[libc::c_char; 74]>(b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\x00")).as_ptr());
            };
            if ssize == 0i32 {
                return
            } else {
                ssize -= 1;
                first = stack[ssize as usize].a;
                last = stack[ssize as usize].b;
                depth = stack[ssize as usize].c;
                limit = stack[ssize as usize].d
            }
        } else {
            Td = T.offset(depth as isize);
            let fresh39 = limit;
            limit = limit - 1;
            if fresh39 == 0i32 {
                ss_heapsort(Td, PA, first,
                            first.offset_to(last).expect("bad offset_to") as
                                libc::c_long as libc::c_int);
            }
            if limit < 0i32 {
                a = first.offset(1isize);
                v =
                    *Td.offset(*PA.offset(*first as isize) as isize) as
                        libc::c_int;
                while a < last {
                    x =
                        *Td.offset(*PA.offset(*a as isize) as isize) as
                            libc::c_int;
                    if x != v {
                        if (1i32 as libc::c_long) <
                               first.offset_to(a).expect("bad offset_to") as
                                   libc::c_long {
                            break ;
                        }
                        v = x;
                        first = a
                    }
                    a = a.offset(1isize)
                }
                if (*Td.offset((*PA.offset(*first as isize) - 1i32) as isize)
                        as libc::c_int) < v {
                    first = ss_partition(PA, first, a, depth)
                }
                if first.offset_to(a).expect("bad offset_to") as libc::c_long
                       <=
                       a.offset_to(last).expect("bad offset_to") as
                           libc::c_long {
                    if (1i32 as libc::c_long) <
                           first.offset_to(a).expect("bad offset_to") as
                               libc::c_long {
                        if ssize < 16i32 {
                        } else {
                            __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                    &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                          (*::std::mem::transmute::<&[u8; 53],
                                                                    &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                          437i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 74],
                                                                    &[libc::c_char; 74]>(b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\x00")).as_ptr());
                        };
                        stack[ssize as usize].a = a;
                        stack[ssize as usize].b = last;
                        stack[ssize as usize].c = depth;
                        let fresh40 = ssize;
                        ssize = ssize + 1;
                        stack[fresh40 as usize].d = -1i32;
                        last = a;
                        depth += 1i32;
                        limit =
                            ss_ilg(first.offset_to(a).expect("bad offset_to")
                                       as libc::c_long as libc::c_int)
                    } else { first = a; limit = -1i32 }
                } else if (1i32 as libc::c_long) <
                              a.offset_to(last).expect("bad offset_to") as
                                  libc::c_long {
                    if ssize < 16i32 {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      444i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 74],
                                                                &[libc::c_char; 74]>(b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\x00")).as_ptr());
                    };
                    stack[ssize as usize].a = first;
                    stack[ssize as usize].b = a;
                    stack[ssize as usize].c = depth + 1i32;
                    let fresh41 = ssize;
                    ssize = ssize + 1;
                    stack[fresh41 as usize].d =
                        ss_ilg(first.offset_to(a).expect("bad offset_to") as
                                   libc::c_long as libc::c_int);
                    first = a;
                    limit = -1i32
                } else {
                    last = a;
                    depth += 1i32;
                    limit =
                        ss_ilg(first.offset_to(a).expect("bad offset_to") as
                                   libc::c_long as libc::c_int)
                }
            } else {
                a = ss_pivot(Td, PA, first, last);
                v =
                    *Td.offset(*PA.offset(*a as isize) as isize) as
                        libc::c_int;
                t = *first;
                *first = *a;
                *a = t;
                b = first;
                loop  {
                    b = b.offset(1isize);
                    if !(b < last &&
                             {
                                 x =
                                     *Td.offset(*PA.offset(*b as isize) as
                                                    isize) as libc::c_int;
                                 x == v
                             }) {
                        break ;
                    }
                }
                a = b;
                if a < last && x < v {
                    loop  {
                        b = b.offset(1isize);
                        if !(b < last &&
                                 {
                                     x =
                                         *Td.offset(*PA.offset(*b as isize) as
                                                        isize) as libc::c_int;
                                     x <= v
                                 }) {
                            break ;
                        }
                        if !(x == v) { continue ; }
                        t = *b;
                        *b = *a;
                        *a = t;
                        a = a.offset(1isize)
                    }
                }
                c = last;
                loop  {
                    c = c.offset(-1isize);
                    if !(b < c &&
                             {
                                 x =
                                     *Td.offset(*PA.offset(*c as isize) as
                                                    isize) as libc::c_int;
                                 x == v
                             }) {
                        break ;
                    }
                }
                d = c;
                if b < d && x > v {
                    loop  {
                        c = c.offset(-1isize);
                        if !(b < c &&
                                 {
                                     x =
                                         *Td.offset(*PA.offset(*c as isize) as
                                                        isize) as libc::c_int;
                                     x >= v
                                 }) {
                            break ;
                        }
                        if !(x == v) { continue ; }
                        t = *c;
                        *c = *d;
                        *d = t;
                        d = d.offset(-1isize)
                    }
                }
                while b < c {
                    t = *b;
                    *b = *c;
                    *c = t;
                    loop  {
                        b = b.offset(1isize);
                        if !(b < c &&
                                 {
                                     x =
                                         *Td.offset(*PA.offset(*b as isize) as
                                                        isize) as libc::c_int;
                                     x <= v
                                 }) {
                            break ;
                        }
                        if !(x == v) { continue ; }
                        t = *b;
                        *b = *a;
                        *a = t;
                        a = a.offset(1isize)
                    }
                    loop  {
                        c = c.offset(-1isize);
                        if !(b < c &&
                                 {
                                     x =
                                         *Td.offset(*PA.offset(*c as isize) as
                                                        isize) as libc::c_int;
                                     x >= v
                                 }) {
                            break ;
                        }
                        if !(x == v) { continue ; }
                        t = *c;
                        *c = *d;
                        *d = t;
                        d = d.offset(-1isize)
                    }
                }
                if a <= d {
                    c = b.offset(-1isize);
                    s =
                        first.offset_to(a).expect("bad offset_to") as
                            libc::c_long as libc::c_int;
                    t =
                        a.offset_to(b).expect("bad offset_to") as libc::c_long
                            as libc::c_int;
                    if s > t { s = t }
                    e = first;
                    f = b.offset(-(s as isize));
                    while 0i32 < s {
                        t = *e;
                        *e = *f;
                        *f = t;
                        s -= 1;
                        e = e.offset(1isize);
                        f = f.offset(1isize)
                    }
                    s =
                        c.offset_to(d).expect("bad offset_to") as libc::c_long
                            as libc::c_int;
                    t =
                        (d.offset_to(last).expect("bad offset_to") as
                             libc::c_long - 1i32 as libc::c_long) as
                            libc::c_int;
                    if s > t { s = t }
                    e = b;
                    f = last.offset(-(s as isize));
                    while 0i32 < s {
                        t = *e;
                        *e = *f;
                        *f = t;
                        s -= 1;
                        e = e.offset(1isize);
                        f = f.offset(1isize)
                    }
                    a =
                        first.offset(a.offset_to(b).expect("bad offset_to") as
                                         libc::c_long as isize);
                    c =
                        last.offset(-(c.offset_to(d).expect("bad offset_to")
                                          as libc::c_long as isize));
                    b =
                        if v <=
                               *Td.offset((*PA.offset(*a as isize) - 1i32) as
                                              isize) as libc::c_int {
                            a
                        } else { ss_partition(PA, a, c, depth) };
                    if first.offset_to(a).expect("bad offset_to") as
                           libc::c_long <=
                           c.offset_to(last).expect("bad offset_to") as
                               libc::c_long {
                        if c.offset_to(last).expect("bad offset_to") as
                               libc::c_long <=
                               b.offset_to(c).expect("bad offset_to") as
                                   libc::c_long {
                            if ssize < 16i32 {
                            } else {
                                __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                        &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                              (*::std::mem::transmute::<&[u8; 53],
                                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                              494i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 74],
                                                                        &[libc::c_char; 74]>(b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\x00")).as_ptr());
                            };
                            stack[ssize as usize].a = b;
                            stack[ssize as usize].b = c;
                            stack[ssize as usize].c = depth + 1i32;
                            let fresh42 = ssize;
                            ssize = ssize + 1;
                            stack[fresh42 as usize].d =
                                ss_ilg(b.offset_to(c).expect("bad offset_to")
                                           as libc::c_long as libc::c_int);
                            if ssize < 16i32 {
                            } else {
                                __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                        &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                              (*::std::mem::transmute::<&[u8; 53],
                                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                              495i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 74],
                                                                        &[libc::c_char; 74]>(b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\x00")).as_ptr());
                            };
                            stack[ssize as usize].a = c;
                            stack[ssize as usize].b = last;
                            stack[ssize as usize].c = depth;
                            let fresh43 = ssize;
                            ssize = ssize + 1;
                            stack[fresh43 as usize].d = limit;
                            last = a
                        } else if first.offset_to(a).expect("bad offset_to")
                                      as libc::c_long <=
                                      b.offset_to(c).expect("bad offset_to")
                                          as libc::c_long {
                            if ssize < 16i32 {
                            } else {
                                __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                        &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                              (*::std::mem::transmute::<&[u8; 53],
                                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                              498i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 74],
                                                                        &[libc::c_char; 74]>(b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\x00")).as_ptr());
                            };
                            stack[ssize as usize].a = c;
                            stack[ssize as usize].b = last;
                            stack[ssize as usize].c = depth;
                            let fresh44 = ssize;
                            ssize = ssize + 1;
                            stack[fresh44 as usize].d = limit;
                            if ssize < 16i32 {
                            } else {
                                __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                        &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                              (*::std::mem::transmute::<&[u8; 53],
                                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                              499i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 74],
                                                                        &[libc::c_char; 74]>(b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\x00")).as_ptr());
                            };
                            stack[ssize as usize].a = b;
                            stack[ssize as usize].b = c;
                            stack[ssize as usize].c = depth + 1i32;
                            let fresh45 = ssize;
                            ssize = ssize + 1;
                            stack[fresh45 as usize].d =
                                ss_ilg(b.offset_to(c).expect("bad offset_to")
                                           as libc::c_long as libc::c_int);
                            last = a
                        } else {
                            if ssize < 16i32 {
                            } else {
                                __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                        &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                              (*::std::mem::transmute::<&[u8; 53],
                                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                              502i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 74],
                                                                        &[libc::c_char; 74]>(b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\x00")).as_ptr());
                            };
                            stack[ssize as usize].a = c;
                            stack[ssize as usize].b = last;
                            stack[ssize as usize].c = depth;
                            let fresh46 = ssize;
                            ssize = ssize + 1;
                            stack[fresh46 as usize].d = limit;
                            if ssize < 16i32 {
                            } else {
                                __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                        &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                              (*::std::mem::transmute::<&[u8; 53],
                                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                              503i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 74],
                                                                        &[libc::c_char; 74]>(b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\x00")).as_ptr());
                            };
                            stack[ssize as usize].a = first;
                            stack[ssize as usize].b = a;
                            stack[ssize as usize].c = depth;
                            let fresh47 = ssize;
                            ssize = ssize + 1;
                            stack[fresh47 as usize].d = limit;
                            first = b;
                            last = c;
                            depth += 1i32;
                            limit =
                                ss_ilg(b.offset_to(c).expect("bad offset_to")
                                           as libc::c_long as libc::c_int)
                        }
                    } else if first.offset_to(a).expect("bad offset_to") as
                                  libc::c_long <=
                                  b.offset_to(c).expect("bad offset_to") as
                                      libc::c_long {
                        if ssize < 16i32 {
                        } else {
                            __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                    &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                          (*::std::mem::transmute::<&[u8; 53],
                                                                    &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                          508i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 74],
                                                                    &[libc::c_char; 74]>(b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\x00")).as_ptr());
                        };
                        stack[ssize as usize].a = b;
                        stack[ssize as usize].b = c;
                        stack[ssize as usize].c = depth + 1i32;
                        let fresh48 = ssize;
                        ssize = ssize + 1;
                        stack[fresh48 as usize].d =
                            ss_ilg(b.offset_to(c).expect("bad offset_to") as
                                       libc::c_long as libc::c_int);
                        if ssize < 16i32 {
                        } else {
                            __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                    &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                          (*::std::mem::transmute::<&[u8; 53],
                                                                    &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                          509i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 74],
                                                                    &[libc::c_char; 74]>(b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\x00")).as_ptr());
                        };
                        stack[ssize as usize].a = first;
                        stack[ssize as usize].b = a;
                        stack[ssize as usize].c = depth;
                        let fresh49 = ssize;
                        ssize = ssize + 1;
                        stack[fresh49 as usize].d = limit;
                        first = c
                    } else if c.offset_to(last).expect("bad offset_to") as
                                  libc::c_long <=
                                  b.offset_to(c).expect("bad offset_to") as
                                      libc::c_long {
                        if ssize < 16i32 {
                        } else {
                            __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                    &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                          (*::std::mem::transmute::<&[u8; 53],
                                                                    &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                          512i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 74],
                                                                    &[libc::c_char; 74]>(b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\x00")).as_ptr());
                        };
                        stack[ssize as usize].a = first;
                        stack[ssize as usize].b = a;
                        stack[ssize as usize].c = depth;
                        let fresh50 = ssize;
                        ssize = ssize + 1;
                        stack[fresh50 as usize].d = limit;
                        if ssize < 16i32 {
                        } else {
                            __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                    &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                          (*::std::mem::transmute::<&[u8; 53],
                                                                    &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                          513i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 74],
                                                                    &[libc::c_char; 74]>(b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\x00")).as_ptr());
                        };
                        stack[ssize as usize].a = b;
                        stack[ssize as usize].b = c;
                        stack[ssize as usize].c = depth + 1i32;
                        let fresh51 = ssize;
                        ssize = ssize + 1;
                        stack[fresh51 as usize].d =
                            ss_ilg(b.offset_to(c).expect("bad offset_to") as
                                       libc::c_long as libc::c_int);
                        first = c
                    } else {
                        if ssize < 16i32 {
                        } else {
                            __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                    &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                          (*::std::mem::transmute::<&[u8; 53],
                                                                    &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                          516i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 74],
                                                                    &[libc::c_char; 74]>(b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\x00")).as_ptr());
                        };
                        stack[ssize as usize].a = first;
                        stack[ssize as usize].b = a;
                        stack[ssize as usize].c = depth;
                        let fresh52 = ssize;
                        ssize = ssize + 1;
                        stack[fresh52 as usize].d = limit;
                        if ssize < 16i32 {
                        } else {
                            __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                    &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                          (*::std::mem::transmute::<&[u8; 53],
                                                                    &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                          517i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 74],
                                                                    &[libc::c_char; 74]>(b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\x00")).as_ptr());
                        };
                        stack[ssize as usize].a = c;
                        stack[ssize as usize].b = last;
                        stack[ssize as usize].c = depth;
                        let fresh53 = ssize;
                        ssize = ssize + 1;
                        stack[fresh53 as usize].d = limit;
                        first = b;
                        last = c;
                        depth += 1i32;
                        limit =
                            ss_ilg(b.offset_to(c).expect("bad offset_to") as
                                       libc::c_long as libc::c_int)
                    }
                } else {
                    limit += 1i32;
                    if (*Td.offset((*PA.offset(*first as isize) - 1i32) as
                                       isize) as libc::c_int) < v {
                        first = ss_partition(PA, first, last, depth);
                        limit =
                            ss_ilg(first.offset_to(last).expect("bad offset_to")
                                       as libc::c_long as libc::c_int)
                    }
                    depth += 1i32
                }
            }
        }
    };
}
unsafe extern "C" fn ss_ilg(mut n: libc::c_int) -> libc::c_int {
    return if 0 != n & 65280i32 {
               8i32 + lg_table[(n >> 8i32 & 255i32) as usize]
           } else { 0i32 + lg_table[(n >> 0i32 & 255i32) as usize] };
}
unsafe extern "C" fn ss_partition(mut PA: *const libc::c_int,
                                  mut first: *mut libc::c_int,
                                  mut last: *mut libc::c_int,
                                  mut depth: libc::c_int)
 -> *mut libc::c_int {
    let mut a: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut b: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    a = first.offset(-1isize);
    b = last;
    loop  {
        a = a.offset(1isize);
        if a < b &&
               *PA.offset(*a as isize) + depth >=
                   *PA.offset((*a + 1i32) as isize) + 1i32 {
            *a = !*a
        } else {
            loop  {
                b = b.offset(-1isize);
                if !(a < b &&
                         *PA.offset(*b as isize) + depth <
                             *PA.offset((*b + 1i32) as isize) + 1i32) {
                    break ;
                }
            }
            if b <= a { break ; }
            t = !*b;
            *b = *a;
            *a = t
        }
    }
    if first < a { *first = !*first }
    return a;
}
unsafe extern "C" fn ss_pivot(mut Td: *const libc::c_uchar,
                              mut PA: *const libc::c_int,
                              mut first: *mut libc::c_int,
                              mut last: *mut libc::c_int)
 -> *mut libc::c_int {
    let mut middle: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    t =
        first.offset_to(last).expect("bad offset_to") as libc::c_long as
            libc::c_int;
    middle = first.offset((t / 2i32) as isize);
    if t <= 512i32 {
        if t <= 32i32 {
            return ss_median3(Td, PA, first, middle, last.offset(-1isize))
        } else {
            t >>= 2i32;
            return ss_median5(Td, PA, first, first.offset(t as isize), middle,
                              last.offset(-1isize).offset(-(t as isize)),
                              last.offset(-1isize))
        }
    } else {
        t >>= 3i32;
        first =
            ss_median3(Td, PA, first, first.offset(t as isize),
                       first.offset((t << 1i32) as isize));
        middle =
            ss_median3(Td, PA, middle.offset(-(t as isize)), middle,
                       middle.offset(t as isize));
        last =
            ss_median3(Td, PA,
                       last.offset(-1isize).offset(-((t << 1i32) as isize)),
                       last.offset(-1isize).offset(-(t as isize)),
                       last.offset(-1isize));
        return ss_median3(Td, PA, first, middle, last)
    };
}
unsafe extern "C" fn ss_median3(mut Td: *const libc::c_uchar,
                                mut PA: *const libc::c_int,
                                mut v1: *mut libc::c_int,
                                mut v2: *mut libc::c_int,
                                mut v3: *mut libc::c_int)
 -> *mut libc::c_int {
    let mut t: *mut libc::c_int = 0 as *mut libc::c_int;
    if *Td.offset(*PA.offset(*v1 as isize) as isize) as libc::c_int >
           *Td.offset(*PA.offset(*v2 as isize) as isize) as libc::c_int {
        t = v1;
        v1 = v2;
        v2 = t
    }
    if *Td.offset(*PA.offset(*v2 as isize) as isize) as libc::c_int >
           *Td.offset(*PA.offset(*v3 as isize) as isize) as libc::c_int {
        if *Td.offset(*PA.offset(*v1 as isize) as isize) as libc::c_int >
               *Td.offset(*PA.offset(*v3 as isize) as isize) as libc::c_int {
            return v1
        } else { return v3 }
    } else { return v2 };
}
unsafe extern "C" fn ss_median5(mut Td: *const libc::c_uchar,
                                mut PA: *const libc::c_int,
                                mut v1: *mut libc::c_int,
                                mut v2: *mut libc::c_int,
                                mut v3: *mut libc::c_int,
                                mut v4: *mut libc::c_int,
                                mut v5: *mut libc::c_int)
 -> *mut libc::c_int {
    let mut t: *mut libc::c_int = 0 as *mut libc::c_int;
    if *Td.offset(*PA.offset(*v2 as isize) as isize) as libc::c_int >
           *Td.offset(*PA.offset(*v3 as isize) as isize) as libc::c_int {
        t = v2;
        v2 = v3;
        v3 = t
    }
    if *Td.offset(*PA.offset(*v4 as isize) as isize) as libc::c_int >
           *Td.offset(*PA.offset(*v5 as isize) as isize) as libc::c_int {
        t = v4;
        v4 = v5;
        v5 = t
    }
    if *Td.offset(*PA.offset(*v2 as isize) as isize) as libc::c_int >
           *Td.offset(*PA.offset(*v4 as isize) as isize) as libc::c_int {
        t = v2;
        v2 = v4;
        v4 = t;
        t = v3;
        v3 = v5;
        v5 = t
    }
    if *Td.offset(*PA.offset(*v1 as isize) as isize) as libc::c_int >
           *Td.offset(*PA.offset(*v3 as isize) as isize) as libc::c_int {
        t = v1;
        v1 = v3;
        v3 = t
    }
    if *Td.offset(*PA.offset(*v1 as isize) as isize) as libc::c_int >
           *Td.offset(*PA.offset(*v4 as isize) as isize) as libc::c_int {
        t = v1;
        v1 = v4;
        v4 = t;
        t = v3;
        v3 = v5;
        v5 = t
    }
    if *Td.offset(*PA.offset(*v3 as isize) as isize) as libc::c_int >
           *Td.offset(*PA.offset(*v4 as isize) as isize) as libc::c_int {
        return v4
    } else { return v3 };
}
unsafe extern "C" fn ss_heapsort(mut Td: *const libc::c_uchar,
                                 mut PA: *const libc::c_int,
                                 mut SA: *mut libc::c_int,
                                 mut size: libc::c_int) -> () {
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    m = size;
    if size % 2i32 == 0i32 {
        m -= 1;
        if (*Td.offset(*PA.offset(*SA.offset((m / 2i32) as isize) as isize) as
                           isize) as libc::c_int) <
               *Td.offset(*PA.offset(*SA.offset(m as isize) as isize) as
                              isize) as libc::c_int {
            t = *SA.offset(m as isize);
            *SA.offset(m as isize) = *SA.offset((m / 2i32) as isize);
            *SA.offset((m / 2i32) as isize) = t
        }
    }
    i = m / 2i32 - 1i32;
    while 0i32 <= i { ss_fixdown(Td, PA, SA, i, m); i -= 1 }
    if size % 2i32 == 0i32 {
        t = *SA.offset(0isize);
        *SA.offset(0isize) = *SA.offset(m as isize);
        *SA.offset(m as isize) = t;
        ss_fixdown(Td, PA, SA, 0i32, m);
    }
    i = m - 1i32;
    while 0i32 < i {
        t = *SA.offset(0isize);
        *SA.offset(0isize) = *SA.offset(i as isize);
        ss_fixdown(Td, PA, SA, 0i32, i);
        *SA.offset(i as isize) = t;
        i -= 1
    };
}
unsafe extern "C" fn ss_fixdown(mut Td: *const libc::c_uchar,
                                mut PA: *const libc::c_int,
                                mut SA: *mut libc::c_int, mut i: libc::c_int,
                                mut size: libc::c_int) -> () {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    v = *SA.offset(i as isize);
    c = *Td.offset(*PA.offset(v as isize) as isize) as libc::c_int;
    loop  {
        j = 2i32 * i + 1i32;
        if !(j < size) { break ; }
        let fresh54 = j;
        j = j + 1;
        k = fresh54;
        d =
            *Td.offset(*PA.offset(*SA.offset(k as isize) as isize) as isize)
                as libc::c_int;
        e =
            *Td.offset(*PA.offset(*SA.offset(j as isize) as isize) as isize)
                as libc::c_int;
        if d < e { k = j; d = e }
        if d <= c { break ; }
        *SA.offset(i as isize) = *SA.offset(k as isize);
        i = k
    }
    *SA.offset(i as isize) = v;
}
unsafe extern "C" fn ss_insertionsort(mut T: *const libc::c_uchar,
                                      mut PA: *const libc::c_int,
                                      mut first: *mut libc::c_int,
                                      mut last: *mut libc::c_int,
                                      mut depth: libc::c_int) -> () {
    let mut i: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut j: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    i = last.offset(-2isize);
    while first <= i {
        t = *i;
        j = i.offset(1isize);
        loop  {
            r =
                ss_compare(T, PA.offset(t as isize), PA.offset(*j as isize),
                           depth);
            if !(0i32 < r) { break ; }
            loop  {
                *j.offset(-1isize) = *j;
                j = j.offset(1isize);
                if !(j < last && *j < 0i32) { break ; }
            }
            if last <= j { break ; }
        }
        if r == 0i32 { *j = !*j }
        *j.offset(-1isize) = t;
        i = i.offset(-1isize)
    };
}
unsafe extern "C" fn ss_swapmerge(mut T: *const libc::c_uchar,
                                  mut PA: *const libc::c_int,
                                  mut first: *mut libc::c_int,
                                  mut middle: *mut libc::c_int,
                                  mut last: *mut libc::c_int,
                                  mut buf: *mut libc::c_int,
                                  mut bufsize: libc::c_int,
                                  mut depth: libc::c_int) -> () {
    let mut stack: [unnamed; 32] =
        [unnamed{a: 0 as *mut libc::c_int,
                 b: 0 as *mut libc::c_int,
                 c: 0 as *mut libc::c_int,
                 d: 0,}; 32];
    let mut l: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut r: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut lm: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rm: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut ssize: libc::c_int = 0;
    let mut check: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    check = 0i32;
    ssize = 0i32;
    loop  {
        if middle.offset_to(last).expect("bad offset_to") as libc::c_long <=
               bufsize as libc::c_long {
            if first < middle && middle < last {
                ss_mergebackward(T, PA, first, middle, last, buf, depth);
            }
            if 0 != check & 1i32 ||
                   0 != check & 2i32 &&
                       ss_compare(T,
                                  PA.offset((if 0i32 <= *first.offset(-1isize)
                                                {
                                                 *first.offset(-1isize)
                                             } else {
                                                 !*first.offset(-1isize)
                                             }) as isize),
                                  PA.offset(*first as isize), depth) == 0i32 {
                *first = !*first
            }
            if 0 != check & 4i32 &&
                   ss_compare(T,
                              PA.offset((if 0i32 <= *last.offset(-1isize) {
                                             *last.offset(-1isize)
                                         } else { !*last.offset(-1isize) }) as
                                            isize), PA.offset(*last as isize),
                              depth) == 0i32 {
                *last = !*last
            }
            if 0i32 <= ssize {
            } else {
                __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                        &mut [libc::c_char; 11]>(b"0 <= ssize\x00")).as_mut_ptr(),
                              (*::std::mem::transmute::<&[u8; 53],
                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                              771i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 92],
                                                        &[libc::c_char; 92]>(b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\x00")).as_ptr());
            };
            if ssize == 0i32 {
                return
            } else {
                ssize -= 1;
                first = stack[ssize as usize].a;
                middle = stack[ssize as usize].b;
                last = stack[ssize as usize].c;
                check = stack[ssize as usize].d
            }
        } else if first.offset_to(middle).expect("bad offset_to") as
                      libc::c_long <= bufsize as libc::c_long {
            if first < middle {
                ss_mergeforward(T, PA, first, middle, last, buf, depth);
            }
            if 0 != check & 1i32 ||
                   0 != check & 2i32 &&
                       ss_compare(T,
                                  PA.offset((if 0i32 <= *first.offset(-1isize)
                                                {
                                                 *first.offset(-1isize)
                                             } else {
                                                 !*first.offset(-1isize)
                                             }) as isize),
                                  PA.offset(*first as isize), depth) == 0i32 {
                *first = !*first
            }
            if 0 != check & 4i32 &&
                   ss_compare(T,
                              PA.offset((if 0i32 <= *last.offset(-1isize) {
                                             *last.offset(-1isize)
                                         } else { !*last.offset(-1isize) }) as
                                            isize), PA.offset(*last as isize),
                              depth) == 0i32 {
                *last = !*last
            }
            if 0i32 <= ssize {
            } else {
                __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                        &mut [libc::c_char; 11]>(b"0 <= ssize\x00")).as_mut_ptr(),
                              (*::std::mem::transmute::<&[u8; 53],
                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                              780i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 92],
                                                        &[libc::c_char; 92]>(b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\x00")).as_ptr());
            };
            if ssize == 0i32 {
                return
            } else {
                ssize -= 1;
                first = stack[ssize as usize].a;
                middle = stack[ssize as usize].b;
                last = stack[ssize as usize].c;
                check = stack[ssize as usize].d
            }
        } else {
            m = 0i32;
            len =
                (if (first.offset_to(middle).expect("bad offset_to") as
                         libc::c_long) <
                        middle.offset_to(last).expect("bad offset_to") as
                            libc::c_long {
                     first.offset_to(middle).expect("bad offset_to") as
                         libc::c_long
                 } else {
                     middle.offset_to(last).expect("bad offset_to") as
                         libc::c_long
                 }) as libc::c_int;
            half = len >> 1i32;
            while 0i32 < len {
                if ss_compare(T,
                              PA.offset((if 0i32 <=
                                                *middle.offset(m as
                                                                   isize).offset(half
                                                                                     as
                                                                                     isize)
                                            {
                                             *middle.offset(m as
                                                                isize).offset(half
                                                                                  as
                                                                                  isize)
                                         } else {
                                             !*middle.offset(m as
                                                                 isize).offset(half
                                                                                   as
                                                                                   isize)
                                         }) as isize),
                              PA.offset((if 0i32 <=
                                                *middle.offset(-(m as
                                                                     isize)).offset(-(half
                                                                                          as
                                                                                          isize)).offset(-1isize)
                                            {
                                             *middle.offset(-(m as
                                                                  isize)).offset(-(half
                                                                                       as
                                                                                       isize)).offset(-1isize)
                                         } else {
                                             !*middle.offset(-(m as
                                                                   isize)).offset(-(half
                                                                                        as
                                                                                        isize)).offset(-1isize)
                                         }) as isize), depth) < 0i32 {
                    m += half + 1i32;
                    half -= len & 1i32 ^ 1i32
                }
                len = half;
                half >>= 1i32
            }
            if 0i32 < m {
                lm = middle.offset(-(m as isize));
                rm = middle.offset(m as isize);
                ss_blockswap(lm, middle, m);
                r = middle;
                l = r;
                next = 0i32;
                if rm < last {
                    if *rm < 0i32 {
                        *rm = !*rm;
                        if first < lm {
                            loop  {
                                l = l.offset(-1isize);
                                if !(*l < 0i32) { break ; }
                            }
                            next |= 4i32
                        }
                        next |= 1i32
                    } else if first < lm {
                        while *r < 0i32 { r = r.offset(1isize) }
                        next |= 2i32
                    }
                }
                if first.offset_to(l).expect("bad offset_to") as libc::c_long
                       <=
                       r.offset_to(last).expect("bad offset_to") as
                           libc::c_long {
                    if ssize < 32i32 {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      810i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 92],
                                                                &[libc::c_char; 92]>(b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\x00")).as_ptr());
                    };
                    stack[ssize as usize].a = r;
                    stack[ssize as usize].b = rm;
                    stack[ssize as usize].c = last;
                    let fresh55 = ssize;
                    ssize = ssize + 1;
                    stack[fresh55 as usize].d = next & 3i32 | check & 4i32;
                    middle = lm;
                    last = l;
                    check = check & 3i32 | next & 4i32
                } else {
                    if 0 != next & 2i32 && r == middle { next ^= 6i32 }
                    if ssize < 32i32 {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 19],
                                                                &mut [libc::c_char; 19]>(b"ssize < STACK_SIZE\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      814i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 92],
                                                                &[libc::c_char; 92]>(b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\x00")).as_ptr());
                    };
                    stack[ssize as usize].a = first;
                    stack[ssize as usize].b = lm;
                    stack[ssize as usize].c = l;
                    let fresh56 = ssize;
                    ssize = ssize + 1;
                    stack[fresh56 as usize].d = check & 3i32 | next & 4i32;
                    first = r;
                    middle = rm;
                    check = next & 3i32 | check & 4i32
                }
            } else {
                if ss_compare(T,
                              PA.offset((if 0i32 <= *middle.offset(-1isize) {
                                             *middle.offset(-1isize)
                                         } else { !*middle.offset(-1isize) })
                                            as isize),
                              PA.offset(*middle as isize), depth) == 0i32 {
                    *middle = !*middle
                }
                if 0 != check & 1i32 ||
                       0 != check & 2i32 &&
                           ss_compare(T,
                                      PA.offset((if 0i32 <=
                                                        *first.offset(-1isize)
                                                    {
                                                     *first.offset(-1isize)
                                                 } else {
                                                     !*first.offset(-1isize)
                                                 }) as isize),
                                      PA.offset(*first as isize), depth) ==
                               0i32 {
                    *first = !*first
                }
                if 0 != check & 4i32 &&
                       ss_compare(T,
                                  PA.offset((if 0i32 <= *last.offset(-1isize)
                                                {
                                                 *last.offset(-1isize)
                                             } else {
                                                 !*last.offset(-1isize)
                                             }) as isize),
                                  PA.offset(*last as isize), depth) == 0i32 {
                    *last = !*last
                }
                if 0i32 <= ssize {
                } else {
                    __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                            &mut [libc::c_char; 11]>(b"0 <= ssize\x00")).as_mut_ptr(),
                                  (*::std::mem::transmute::<&[u8; 53],
                                                            &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                  822i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 92],
                                                            &[libc::c_char; 92]>(b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\x00")).as_ptr());
                };
                if ssize == 0i32 {
                    return
                } else {
                    ssize -= 1;
                    first = stack[ssize as usize].a;
                    middle = stack[ssize as usize].b;
                    last = stack[ssize as usize].c;
                    check = stack[ssize as usize].d
                }
            }
        }
    };
}
unsafe extern "C" fn ss_mergeforward(mut T: *const libc::c_uchar,
                                     mut PA: *const libc::c_int,
                                     mut first: *mut libc::c_int,
                                     mut middle: *mut libc::c_int,
                                     mut last: *mut libc::c_int,
                                     mut buf: *mut libc::c_int,
                                     mut depth: libc::c_int) -> () {
    let mut a: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut b: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut c: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut bufend: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    bufend =
        buf.offset(first.offset_to(middle).expect("bad offset_to") as
                       libc::c_long as isize).offset(-1isize);
    ss_blockswap(buf, first,
                 first.offset_to(middle).expect("bad offset_to") as
                     libc::c_long as libc::c_int);
    a = first;
    t = *a;
    b = buf;
    c = middle;
    loop  {
        r =
            ss_compare(T, PA.offset(*b as isize), PA.offset(*c as isize),
                       depth);
        if r < 0i32 {
            loop  {
                let fresh57 = a;
                a = a.offset(1);
                *fresh57 = *b;
                if bufend <= b {
                    *bufend = t;
                    return
                } else {
                    let fresh58 = b;
                    b = b.offset(1);
                    *fresh58 = *a;
                    if !(*b < 0i32) { break ; }
                }
            }
        } else if r > 0i32 {
            loop  {
                let fresh59 = a;
                a = a.offset(1);
                *fresh59 = *c;
                let fresh60 = c;
                c = c.offset(1);
                *fresh60 = *a;
                if last <= c {
                    while b < bufend {
                        let fresh61 = a;
                        a = a.offset(1);
                        *fresh61 = *b;
                        let fresh62 = b;
                        b = b.offset(1);
                        *fresh62 = *a
                    }
                    *a = *b;
                    *b = t;
                    return
                } else if !(*c < 0i32) { break ; }
            }
        } else {
            *c = !*c;
            loop  {
                let fresh63 = a;
                a = a.offset(1);
                *fresh63 = *b;
                if bufend <= b {
                    *bufend = t;
                    return
                } else {
                    let fresh64 = b;
                    b = b.offset(1);
                    *fresh64 = *a;
                    if !(*b < 0i32) { break ; }
                }
            }
            loop  {
                let fresh65 = a;
                a = a.offset(1);
                *fresh65 = *c;
                let fresh66 = c;
                c = c.offset(1);
                *fresh66 = *a;
                if last <= c {
                    while b < bufend {
                        let fresh67 = a;
                        a = a.offset(1);
                        *fresh67 = *b;
                        let fresh68 = b;
                        b = b.offset(1);
                        *fresh68 = *a
                    }
                    *a = *b;
                    *b = t;
                    return
                } else if !(*c < 0i32) { break ; }
            }
        }
    };
}
unsafe extern "C" fn ss_mergebackward(mut T: *const libc::c_uchar,
                                      mut PA: *const libc::c_int,
                                      mut first: *mut libc::c_int,
                                      mut middle: *mut libc::c_int,
                                      mut last: *mut libc::c_int,
                                      mut buf: *mut libc::c_int,
                                      mut depth: libc::c_int) -> () {
    let mut p1: *const libc::c_int = 0 as *const libc::c_int;
    let mut p2: *const libc::c_int = 0 as *const libc::c_int;
    let mut a: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut b: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut c: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut bufend: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut t: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    bufend =
        buf.offset(middle.offset_to(last).expect("bad offset_to") as
                       libc::c_long as isize).offset(-1isize);
    ss_blockswap(buf, middle,
                 middle.offset_to(last).expect("bad offset_to") as
                     libc::c_long as libc::c_int);
    x = 0i32;
    if *bufend < 0i32 {
        p1 = PA.offset(!*bufend as isize);
        x |= 1i32
    } else { p1 = PA.offset(*bufend as isize) }
    if *middle.offset(-1isize) < 0i32 {
        p2 = PA.offset(!*middle.offset(-1isize) as isize);
        x |= 2i32
    } else { p2 = PA.offset(*middle.offset(-1isize) as isize) }
    a = last.offset(-1isize);
    t = *a;
    b = bufend;
    c = middle.offset(-1isize);
    loop  {
        r = ss_compare(T, p1, p2, depth);
        if 0i32 < r {
            if 0 != x & 1i32 {
                loop  {
                    let fresh69 = a;
                    a = a.offset(-1);
                    *fresh69 = *b;
                    let fresh70 = b;
                    b = b.offset(-1);
                    *fresh70 = *a;
                    if !(*b < 0i32) { break ; }
                }
                x ^= 1i32
            }
            let fresh71 = a;
            a = a.offset(-1);
            *fresh71 = *b;
            if b <= buf {
                *buf = t;
                break ;
            } else {
                let fresh72 = b;
                b = b.offset(-1);
                *fresh72 = *a;
                if *b < 0i32 {
                    p1 = PA.offset(!*b as isize);
                    x |= 1i32
                } else { p1 = PA.offset(*b as isize) }
            }
        } else if r < 0i32 {
            if 0 != x & 2i32 {
                loop  {
                    let fresh73 = a;
                    a = a.offset(-1);
                    *fresh73 = *c;
                    let fresh74 = c;
                    c = c.offset(-1);
                    *fresh74 = *a;
                    if !(*c < 0i32) { break ; }
                }
                x ^= 2i32
            }
            let fresh75 = a;
            a = a.offset(-1);
            *fresh75 = *c;
            let fresh76 = c;
            c = c.offset(-1);
            *fresh76 = *a;
            if c < first {
                while buf < b {
                    let fresh77 = a;
                    a = a.offset(-1);
                    *fresh77 = *b;
                    let fresh78 = b;
                    b = b.offset(-1);
                    *fresh78 = *a
                }
                *a = *b;
                *b = t;
                break ;
            } else if *c < 0i32 {
                p2 = PA.offset(!*c as isize);
                x |= 2i32
            } else { p2 = PA.offset(*c as isize) }
        } else {
            if 0 != x & 1i32 {
                loop  {
                    let fresh79 = a;
                    a = a.offset(-1);
                    *fresh79 = *b;
                    let fresh80 = b;
                    b = b.offset(-1);
                    *fresh80 = *a;
                    if !(*b < 0i32) { break ; }
                }
                x ^= 1i32
            }
            let fresh81 = a;
            a = a.offset(-1);
            *fresh81 = !*b;
            if b <= buf {
                *buf = t;
                break ;
            } else {
                let fresh82 = b;
                b = b.offset(-1);
                *fresh82 = *a;
                if 0 != x & 2i32 {
                    loop  {
                        let fresh83 = a;
                        a = a.offset(-1);
                        *fresh83 = *c;
                        let fresh84 = c;
                        c = c.offset(-1);
                        *fresh84 = *a;
                        if !(*c < 0i32) { break ; }
                    }
                    x ^= 2i32
                }
                let fresh85 = a;
                a = a.offset(-1);
                *fresh85 = *c;
                let fresh86 = c;
                c = c.offset(-1);
                *fresh86 = *a;
                if c < first {
                    while buf < b {
                        let fresh87 = a;
                        a = a.offset(-1);
                        *fresh87 = *b;
                        let fresh88 = b;
                        b = b.offset(-1);
                        *fresh88 = *a
                    }
                    *a = *b;
                    *b = t;
                    break ;
                } else {
                    if *b < 0i32 {
                        p1 = PA.offset(!*b as isize);
                        x |= 1i32
                    } else { p1 = PA.offset(*b as isize) }
                    if *c < 0i32 {
                        p2 = PA.offset(!*c as isize);
                        x |= 2i32
                    } else { p2 = PA.offset(*c as isize) }
                }
            }
        }
    };
}
unsafe extern "C" fn ss_isqrt(mut x: libc::c_int) -> libc::c_int {
    let mut y: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    if x >= 1024i32 * 1024i32 {
        return 1024i32
    } else {
        e =
            if 0 != x as libc::c_uint & 4294901760u32 {
                if 0 != x as libc::c_uint & 4278190080u32 {
                    24i32 + lg_table[(x >> 24i32 & 255i32) as usize]
                } else { 16i32 + lg_table[(x >> 16i32 & 255i32) as usize] }
            } else if 0 != x & 65280i32 {
                8i32 + lg_table[(x >> 8i32 & 255i32) as usize]
            } else { 0i32 + lg_table[(x >> 0i32 & 255i32) as usize] };
        if e >= 16i32 {
            y =
                sqq_table[(x >> e - 6i32 - (e & 1i32)) as usize] <<
                    (e >> 1i32) - 7i32;
            if e >= 24i32 { y = y + 1i32 + x / y >> 1i32 }
            y = y + 1i32 + x / y >> 1i32
        } else if e >= 8i32 {
            y =
                (sqq_table[(x >> e - 6i32 - (e & 1i32)) as usize] >>
                     7i32 - (e >> 1i32)) + 1i32
        } else { return sqq_table[x as usize] >> 4i32 }
        return if x < y * y { y - 1i32 } else { y }
    };
}
static mut sqq_table: [libc::c_int; 256] =
    unsafe {
        [0i32, 16i32, 22i32, 27i32, 32i32, 35i32, 39i32, 42i32, 45i32, 48i32,
         50i32, 53i32, 55i32, 57i32, 59i32, 61i32, 64i32, 65i32, 67i32, 69i32,
         71i32, 73i32, 75i32, 76i32, 78i32, 80i32, 81i32, 83i32, 84i32, 86i32,
         87i32, 89i32, 90i32, 91i32, 93i32, 94i32, 96i32, 97i32, 98i32, 99i32,
         101i32, 102i32, 103i32, 104i32, 106i32, 107i32, 108i32, 109i32,
         110i32, 112i32, 113i32, 114i32, 115i32, 116i32, 117i32, 118i32,
         119i32, 120i32, 121i32, 122i32, 123i32, 124i32, 125i32, 126i32,
         128i32, 128i32, 129i32, 130i32, 131i32, 132i32, 133i32, 134i32,
         135i32, 136i32, 137i32, 138i32, 139i32, 140i32, 141i32, 142i32,
         143i32, 144i32, 144i32, 145i32, 146i32, 147i32, 148i32, 149i32,
         150i32, 150i32, 151i32, 152i32, 153i32, 154i32, 155i32, 155i32,
         156i32, 157i32, 158i32, 159i32, 160i32, 160i32, 161i32, 162i32,
         163i32, 163i32, 164i32, 165i32, 166i32, 167i32, 167i32, 168i32,
         169i32, 170i32, 170i32, 171i32, 172i32, 173i32, 173i32, 174i32,
         175i32, 176i32, 176i32, 177i32, 178i32, 178i32, 179i32, 180i32,
         181i32, 181i32, 182i32, 183i32, 183i32, 184i32, 185i32, 185i32,
         186i32, 187i32, 187i32, 188i32, 189i32, 189i32, 190i32, 191i32,
         192i32, 192i32, 193i32, 193i32, 194i32, 195i32, 195i32, 196i32,
         197i32, 197i32, 198i32, 199i32, 199i32, 200i32, 201i32, 201i32,
         202i32, 203i32, 203i32, 204i32, 204i32, 205i32, 206i32, 206i32,
         207i32, 208i32, 208i32, 209i32, 209i32, 210i32, 211i32, 211i32,
         212i32, 212i32, 213i32, 214i32, 214i32, 215i32, 215i32, 216i32,
         217i32, 217i32, 218i32, 218i32, 219i32, 219i32, 220i32, 221i32,
         221i32, 222i32, 222i32, 223i32, 224i32, 224i32, 225i32, 225i32,
         226i32, 226i32, 227i32, 227i32, 228i32, 229i32, 229i32, 230i32,
         230i32, 231i32, 231i32, 232i32, 232i32, 233i32, 234i32, 234i32,
         235i32, 235i32, 236i32, 236i32, 237i32, 237i32, 238i32, 238i32,
         239i32, 240i32, 240i32, 241i32, 241i32, 242i32, 242i32, 243i32,
         243i32, 244i32, 244i32, 245i32, 245i32, 246i32, 246i32, 247i32,
         247i32, 248i32, 248i32, 249i32, 249i32, 250i32, 250i32, 251i32,
         251i32, 252i32, 252i32, 253i32, 253i32, 254i32, 254i32, 255i32]
    };
/* *
 * Constructs the burrows-wheeler transformed string of a given string.
 * @param T [0..n-1] The input string.
 * @param U [0..n-1] The output string. (can be T)
 * @param A [0..n-1] The temporary array. (can be NULL)
 * @param n The length of the given string.
 * @param num_indexes The length of secondary indexes array. (can be NULL)
 * @param indexes The secondary indexes array. (can be NULL)
 * @param openMP enables OpenMP optimization.
 * @return The primary index if no error occurred, -1 or -2 otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn divbwt(mut T: *const libc::c_uchar,
                                mut U: *mut libc::c_uchar,
                                mut A: *mut libc::c_int, mut n: libc::c_int,
                                mut num_indexes: *mut libc::c_uchar,
                                mut indexes: *mut libc::c_int,
                                mut openMP: libc::c_int) -> libc::c_int {
    let mut B: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut bucket_A: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut bucket_B: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut m: libc::c_int = 0;
    let mut pidx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if T.is_null() || U.is_null() || n < 0i32 {
        return -1i32
    } else if n <= 1i32 {
        if n == 1i32 { *U.offset(0isize) = *T.offset(0isize) }
        return n
    } else {
        B = A;
        if B.is_null() {
            B =
                malloc(((n + 1i32) as
                            size_t).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                     as libc::c_ulong)) as
                    *mut libc::c_int
        }
        bucket_A =
            malloc((256i32 as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong)) as
                *mut libc::c_int;
        bucket_B =
            malloc(((256i32 * 256i32) as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                        as libc::c_ulong)) as
                *mut libc::c_int;
        if !B.is_null() && !bucket_A.is_null() && !bucket_B.is_null() {
            m = sort_typeBstar(T, B, bucket_A, bucket_B, n, openMP);
            if num_indexes.is_null() || indexes.is_null() {
                pidx = construct_BWT(T, B, bucket_A, bucket_B, n, m)
            } else {
                pidx =
                    construct_BWT_indexes(T, B, bucket_A, bucket_B, n, m,
                                          num_indexes, indexes)
            }
            *U.offset(0isize) = *T.offset((n - 1i32) as isize);
            i = 0i32;
            while i < pidx {
                *U.offset((i + 1i32) as isize) =
                    *B.offset(i as isize) as libc::c_uchar;
                i += 1
            }
            i += 1i32;
            while i < n {
                *U.offset(i as isize) =
                    *B.offset(i as isize) as libc::c_uchar;
                i += 1
            }
            pidx += 1i32
        } else { pidx = -2i32 }
        free(bucket_B as *mut libc::c_void);
        free(bucket_A as *mut libc::c_void);
        if A.is_null() { free(B as *mut libc::c_void); }
        return pidx
    };
}
unsafe extern "C" fn construct_BWT_indexes(mut T: *const libc::c_uchar,
                                           mut SA: *mut libc::c_int,
                                           mut bucket_A: *mut libc::c_int,
                                           mut bucket_B: *mut libc::c_int,
                                           mut n: libc::c_int,
                                           mut m: libc::c_int,
                                           mut num_indexes:
                                               *mut libc::c_uchar,
                                           mut indexes: *mut libc::c_int)
 -> libc::c_int {
    let mut i: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut j: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut k: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut orig: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut s: libc::c_int = 0;
    let mut c0: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut mod_0: libc::c_int = n / 8i32;
    mod_0 |= mod_0 >> 1i32;
    mod_0 |= mod_0 >> 2i32;
    mod_0 |= mod_0 >> 4i32;
    mod_0 |= mod_0 >> 8i32;
    mod_0 |= mod_0 >> 16i32;
    mod_0 >>= 1i32;
    *num_indexes = ((n - 1i32) / (mod_0 + 1i32)) as libc::c_uchar;
    if 0i32 < m {
        c1 = 256i32 - 2i32;
        while 0i32 <= c1 {
            i =
                SA.offset(*bucket_B.offset((c1 << 8i32 | c1 + 1i32) as isize)
                              as isize);
            j =
                SA.offset(*bucket_A.offset((c1 + 1i32) as isize) as
                              isize).offset(-1isize);
            k = 0 as *mut libc::c_int;
            c2 = -1i32;
            while i <= j {
                s = *j;
                if 0i32 < s {
                    if *T.offset(s as isize) as libc::c_int == c1 {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                                &mut [libc::c_char; 11]>(b"T[s] == c1\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1775i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 104],
                                                                &[libc::c_char; 104]>(b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\x00")).as_ptr());
                    };
                    if s + 1i32 < n &&
                           *T.offset(s as isize) as libc::c_int <=
                               *T.offset((s + 1i32) as isize) as libc::c_int {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 36],
                                                                &mut [libc::c_char; 36]>(b"((s + 1) < n) && (T[s] <= T[s + 1])\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1776i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 104],
                                                                &[libc::c_char; 104]>(b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\x00")).as_ptr());
                    };
                    if *T.offset((s - 1i32) as isize) as libc::c_int <=
                           *T.offset(s as isize) as libc::c_int {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 17],
                                                                &mut [libc::c_char; 17]>(b"T[s - 1] <= T[s]\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1777i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 104],
                                                                &[libc::c_char; 104]>(b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\x00")).as_ptr());
                    };
                    if s & mod_0 == 0i32 {
                        *indexes.offset((s / (mod_0 + 1i32) - 1i32) as isize)
                            =
                            SA.offset_to(j).expect("bad offset_to") as
                                libc::c_long as libc::c_int
                    }
                    s -= 1;
                    c0 = *T.offset(s as isize) as libc::c_int;
                    *j = !c0;
                    if 0i32 < s &&
                           *T.offset((s - 1i32) as isize) as libc::c_int > c0
                       {
                        s = !s
                    }
                    if c0 != c2 {
                        if 0i32 <= c2 {
                            *bucket_B.offset((c1 << 8i32 | c2) as isize) =
                                SA.offset_to(k).expect("bad offset_to") as
                                    libc::c_long as libc::c_int
                        }
                        c2 = c0;
                        k =
                            SA.offset(*bucket_B.offset((c1 << 8i32 | c2) as
                                                           isize) as isize)
                    }
                    if k < j {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 6],
                                                                &mut [libc::c_char; 6]>(b"k < j\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1788i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 104],
                                                                &[libc::c_char; 104]>(b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\x00")).as_ptr());
                    };
                    let fresh89 = k;
                    k = k.offset(-1);
                    *fresh89 = s
                } else if s != 0i32 {
                    *j = !s
                } else {
                    if *T.offset(s as isize) as libc::c_int == c1 {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                                &mut [libc::c_char; 11]>(b"T[s] == c1\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1794i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 104],
                                                                &[libc::c_char; 104]>(b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\x00")).as_ptr());
                    };
                }
                j = j.offset(-1isize)
            }
            c1 -= 1
        }
    }
    c2 = *T.offset((n - 1i32) as isize) as libc::c_int;
    k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
    if (*T.offset((n - 2i32) as isize) as libc::c_int) < c2 {
        if n - 1i32 & mod_0 == 0i32 {
            *indexes.offset(((n - 1i32) / (mod_0 + 1i32) - 1i32) as isize) =
                SA.offset_to(k).expect("bad offset_to") as libc::c_long as
                    libc::c_int
        }
        let fresh90 = k;
        k = k.offset(1);
        *fresh90 = !(*T.offset((n - 2i32) as isize) as libc::c_int)
    } else { let fresh91 = k; k = k.offset(1); *fresh91 = n - 1i32 }
    i = SA;
    j = SA.offset(n as isize);
    orig = SA;
    while i < j {
        s = *i;
        if 0i32 < s {
            if *T.offset((s - 1i32) as isize) as libc::c_int >=
                   *T.offset(s as isize) as libc::c_int {
            } else {
                __assert_fail((*::std::mem::transmute::<&[u8; 17],
                                                        &mut [libc::c_char; 17]>(b"T[s - 1] >= T[s]\x00")).as_mut_ptr(),
                              (*::std::mem::transmute::<&[u8; 53],
                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                              1815i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 104],
                                                        &[libc::c_char; 104]>(b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\x00")).as_ptr());
            };
            if s & mod_0 == 0i32 {
                *indexes.offset((s / (mod_0 + 1i32) - 1i32) as isize) =
                    SA.offset_to(i).expect("bad offset_to") as libc::c_long as
                        libc::c_int
            }
            s -= 1;
            c0 = *T.offset(s as isize) as libc::c_int;
            *i = c0;
            if c0 != c2 {
                *bucket_A.offset(c2 as isize) =
                    SA.offset_to(k).expect("bad offset_to") as libc::c_long as
                        libc::c_int;
                c2 = c0;
                k = SA.offset(*bucket_A.offset(c2 as isize) as isize)
            }
            if i < k {
            } else {
                __assert_fail((*::std::mem::transmute::<&[u8; 6],
                                                        &mut [libc::c_char; 6]>(b"i < k\x00")).as_mut_ptr(),
                              (*::std::mem::transmute::<&[u8; 53],
                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                              1825i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 104],
                                                        &[libc::c_char; 104]>(b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\x00")).as_ptr());
            };
            if 0i32 < s &&
                   (*T.offset((s - 1i32) as isize) as libc::c_int) < c0 {
                if s & mod_0 == 0i32 {
                    *indexes.offset((s / (mod_0 + 1i32) - 1i32) as isize) =
                        SA.offset_to(k).expect("bad offset_to") as
                            libc::c_long as libc::c_int
                }
                let fresh92 = k;
                k = k.offset(1);
                *fresh92 = !(*T.offset((s - 1i32) as isize) as libc::c_int)
            } else { let fresh93 = k; k = k.offset(1); *fresh93 = s }
        } else if s != 0i32 { *i = !s } else { orig = i }
        i = i.offset(1isize)
    }
    return SA.offset_to(orig).expect("bad offset_to") as libc::c_long as
               libc::c_int;
}
unsafe extern "C" fn construct_BWT(mut T: *const libc::c_uchar,
                                   mut SA: *mut libc::c_int,
                                   mut bucket_A: *mut libc::c_int,
                                   mut bucket_B: *mut libc::c_int,
                                   mut n: libc::c_int, mut m: libc::c_int)
 -> libc::c_int {
    let mut i: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut j: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut k: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut orig: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut s: libc::c_int = 0;
    let mut c0: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    if 0i32 < m {
        c1 = 256i32 - 2i32;
        while 0i32 <= c1 {
            i =
                SA.offset(*bucket_B.offset((c1 << 8i32 | c1 + 1i32) as isize)
                              as isize);
            j =
                SA.offset(*bucket_A.offset((c1 + 1i32) as isize) as
                              isize).offset(-1isize);
            k = 0 as *mut libc::c_int;
            c2 = -1i32;
            while i <= j {
                s = *j;
                if 0i32 < s {
                    if *T.offset(s as isize) as libc::c_int == c1 {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                                &mut [libc::c_char; 11]>(b"T[s] == c1\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1694i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 72],
                                                                &[libc::c_char; 72]>(b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\x00")).as_ptr());
                    };
                    if s + 1i32 < n &&
                           *T.offset(s as isize) as libc::c_int <=
                               *T.offset((s + 1i32) as isize) as libc::c_int {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 36],
                                                                &mut [libc::c_char; 36]>(b"((s + 1) < n) && (T[s] <= T[s + 1])\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1695i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 72],
                                                                &[libc::c_char; 72]>(b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\x00")).as_ptr());
                    };
                    if *T.offset((s - 1i32) as isize) as libc::c_int <=
                           *T.offset(s as isize) as libc::c_int {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 17],
                                                                &mut [libc::c_char; 17]>(b"T[s - 1] <= T[s]\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1696i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 72],
                                                                &[libc::c_char; 72]>(b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\x00")).as_ptr());
                    };
                    s -= 1;
                    c0 = *T.offset(s as isize) as libc::c_int;
                    *j = !c0;
                    if 0i32 < s &&
                           *T.offset((s - 1i32) as isize) as libc::c_int > c0
                       {
                        s = !s
                    }
                    if c0 != c2 {
                        if 0i32 <= c2 {
                            *bucket_B.offset((c1 << 8i32 | c2) as isize) =
                                SA.offset_to(k).expect("bad offset_to") as
                                    libc::c_long as libc::c_int
                        }
                        c2 = c0;
                        k =
                            SA.offset(*bucket_B.offset((c1 << 8i32 | c2) as
                                                           isize) as isize)
                    }
                    if k < j {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 6],
                                                                &mut [libc::c_char; 6]>(b"k < j\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1704i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 72],
                                                                &[libc::c_char; 72]>(b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\x00")).as_ptr());
                    };
                    let fresh94 = k;
                    k = k.offset(-1);
                    *fresh94 = s
                } else if s != 0i32 {
                    *j = !s
                } else {
                    if *T.offset(s as isize) as libc::c_int == c1 {
                    } else {
                        __assert_fail((*::std::mem::transmute::<&[u8; 11],
                                                                &mut [libc::c_char; 11]>(b"T[s] == c1\x00")).as_mut_ptr(),
                                      (*::std::mem::transmute::<&[u8; 53],
                                                                &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                                      1710i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 72],
                                                                &[libc::c_char; 72]>(b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\x00")).as_ptr());
                    };
                }
                j = j.offset(-1isize)
            }
            c1 -= 1
        }
    }
    c2 = *T.offset((n - 1i32) as isize) as libc::c_int;
    k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
    let fresh95 = k;
    k = k.offset(1);
    *fresh95 =
        if (*T.offset((n - 2i32) as isize) as libc::c_int) < c2 {
            !(*T.offset((n - 2i32) as isize) as libc::c_int)
        } else { n - 1i32 };
    i = SA;
    j = SA.offset(n as isize);
    orig = SA;
    while i < j {
        s = *i;
        if 0i32 < s {
            if *T.offset((s - 1i32) as isize) as libc::c_int >=
                   *T.offset(s as isize) as libc::c_int {
            } else {
                __assert_fail((*::std::mem::transmute::<&[u8; 17],
                                                        &mut [libc::c_char; 17]>(b"T[s - 1] >= T[s]\x00")).as_mut_ptr(),
                              (*::std::mem::transmute::<&[u8; 53],
                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                              1724i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 72],
                                                        &[libc::c_char; 72]>(b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\x00")).as_ptr());
            };
            s -= 1;
            c0 = *T.offset(s as isize) as libc::c_int;
            *i = c0;
            if 0i32 < s &&
                   (*T.offset((s - 1i32) as isize) as libc::c_int) < c0 {
                s = !(*T.offset((s - 1i32) as isize) as libc::c_int)
            }
            if c0 != c2 {
                *bucket_A.offset(c2 as isize) =
                    SA.offset_to(k).expect("bad offset_to") as libc::c_long as
                        libc::c_int;
                c2 = c0;
                k = SA.offset(*bucket_A.offset(c2 as isize) as isize)
            }
            if i < k {
            } else {
                __assert_fail((*::std::mem::transmute::<&[u8; 6],
                                                        &mut [libc::c_char; 6]>(b"i < k\x00")).as_mut_ptr(),
                              (*::std::mem::transmute::<&[u8; 53],
                                                        &mut [libc::c_char; 53]>(b"/home/danielrh/dev/zstd/lib/dictBuilder/divsufsort.c\x00")).as_mut_ptr(),
                              1732i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 72],
                                                        &[libc::c_char; 72]>(b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\x00")).as_ptr());
            };
            let fresh96 = k;
            k = k.offset(1);
            *fresh96 = s
        } else if s != 0i32 { *i = !s } else { orig = i }
        i = i.offset(1isize)
    }
    return SA.offset_to(orig).expect("bad offset_to") as libc::c_long as
               libc::c_int;
}
