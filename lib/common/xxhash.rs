#![allow
( dead_code , mutable_transmutes , non_camel_case_types , non_snake_case ,
non_upper_case_globals , unused_mut )]
#![feature ( libc , offset_to )]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
}
/* !
XXH32() :
    Calculate the 32-bits hash of sequence "length" bytes stored at memory address "input".
    The memory between input & input+length must be valid (allocated and read-accessible).
    "seed" can be used to alter the result predictably.
    Speed on Core 2 Duo @ 3 GHz (single thread, SMHasher benchmark) : 5.4 GB/s
XXH64() :
    Calculate the 64-bits hash of sequence of length "len" stored at memory address "input".
    "seed" can be used to alter the result predictably.
    This function runs 2x faster on 64-bits systems, but slower on 32-bits systems (see benchmark).
*/
pub type XXH32_state_t = XXH32_state_s;
pub type BYTE = uint8_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct XXH32_state_s {
    pub total_len_32: libc::c_uint,
    pub large_len: libc::c_uint,
    pub v1: libc::c_uint,
    pub v2: libc::c_uint,
    pub v3: libc::c_uint,
    pub v4: libc::c_uint,
    pub mem32: [libc::c_uint; 4],
    pub memsize: libc::c_uint,
    pub reserved: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct XXH64_canonical_t {
    pub digest: [libc::c_uchar; 8],
}
pub const XXH_OK: XXH_errorcode = 0;
pub type XXH32_hash_t = libc::c_uint;
pub type XXH_errorcode = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct XXH64_state_s {
    pub total_len: libc::c_ulonglong,
    pub v1: libc::c_ulonglong,
    pub v2: libc::c_ulonglong,
    pub v3: libc::c_ulonglong,
    pub v4: libc::c_ulonglong,
    pub mem64: [libc::c_ulonglong; 4],
    pub memsize: libc::c_uint,
    pub reserved: [libc::c_uint; 2],
}
pub type XXH_alignment = libc::c_uint;
pub const XXH_static_assert: unnamed_0 = 1;
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct XXH32_canonical_t {
    pub digest: [libc::c_uchar; 4],
}
pub type unnamed = libc::c_uint;
pub type uint32_t = libc::c_uint;
pub const XXH_littleEndian: XXH_endianess = 1;
pub type U32 = uint32_t;
pub type XXH_endianess = libc::c_uint;
pub type U64 = uint64_t;
pub type XXH64_state_t = XXH64_state_s;
pub type unnamed_0 = libc::c_uint;
pub const XXH_bigEndian: XXH_endianess = 0;
pub const XXH_aligned: XXH_alignment = 0;
pub const XXH_unaligned: XXH_alignment = 1;
pub type uint64_t = libc::c_ulong;
pub const XXH_ERROR: XXH_errorcode = 1;
pub type uint8_t = libc::c_uchar;
pub const XXH_static_assert_0: unnamed = 1;
pub type XXH64_hash_t = libc::c_ulonglong;
/* !XXH_FORCE_ALIGN_CHECK :
 * This is a minor performance trick, only useful with lots of very small keys.
 * It means : check for aligned/unaligned input.
 * The check costs one initial branch per hash; set to 0 when the input data
 * is guaranteed to be aligned.
 */
/* !XXH_FORCE_NATIVE_FORMAT :
 * By default, xxHash library provides endian-independant Hash values, based on little-endian convention.
 * Results are therefore identical for little-endian and big-endian CPU.
 * This comes at a performance cost for big-endian CPU, since some swapping is required to emulate little-endian format.
 * Should endian-independance be of no importance for your application, you may set the #define below to 1,
 * to improve speed for Big-endian CPU.
 * This option has no impact on Little_Endian CPU.
 */
/* !XXH_ACCEPT_NULL_INPUT_POINTER :
 * If the input pointer is a null pointer, xxHash default behavior is to trigger a memory access error, since it is a bad pointer.
 * When this option is enabled, xxHash output for null input pointers will be the same as a null-length input.
 * By default, this option is disabled. To enable it, uncomment below define :
 */
/* !XXH_FORCE_MEMORY_ACCESS :
 * By default, access to unaligned memory is controlled by `memcpy()`, which is safe and portable.
 * Unfortunately, on some target/compiler combinations, the generated assembly is sub-optimal.
 * The below switch allow to select different access method for improved performance.
 * Method 0 (default) : use `memcpy()`. Safe and portable.
 * Method 1 : `__packed` statement. It depends on compiler extension (ie, not portable).
 *            This method is safe if your compiler supports it, and *generally* as fast or faster than `memcpy`.
 * Method 2 : direct access. This method doesn't depend on compiler but violate C standard.
 *            It can generate buggy code on targets which do not support unaligned memory accesses.
 *            But in some circumstances, it's the only known way to get the most performance (ie GCC + ARMv6)
 * See http://stackoverflow.com/a/32095106/646947 for details.
 * Prefer these methods in priority order (0 > 1 > 2)
 */
unsafe extern "C" fn XXH_malloc(mut s: size_t) -> *mut libc::c_void {
    return malloc(s);
}
unsafe extern "C" fn XXH_free(mut p: *mut libc::c_void) -> () { free(p); }
unsafe extern "C" fn XXH_memcpy(mut dest: *mut libc::c_void,
                                mut src: *const libc::c_void,
                                mut size: size_t) -> *mut libc::c_void {
    return memcpy(dest, src, size);
}
/* !XXH_NAMESPACE, aka Namespace Emulation :

If you want to include _and expose_ xxHash functions from within your own library,
but also want to avoid symbol collisions with another library which also includes xxHash,

you can use XXH_NAMESPACE, to automatically prefix any public symbol from xxhash library
with the value of XXH_NAMESPACE (so avoid to keep it NULL and avoid numeric values).

Note that no change is required within the calling program as long as it includes `xxhash.h` :
regular symbol name will be automatically translated by this header.
*/
/* * XXH_PRIVATE_API
*   This is useful if you want to include xxhash functions in `static` mode
*   in order to inline them, and remove their symbol from the public list.
*   Methodology :
*     #define XXH_PRIVATE_API
*     #include "xxhash.h"
*   `xxhash.c` is automatically included.
*   It's not useful to compile and link it as a separate module anymore.
*/
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH_versionNumber() -> libc::c_uint {
    return (0i32 * 100i32 * 100i32 + 6i32 * 100i32 + 2i32) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32(mut input: *const libc::c_void,
                                    mut len: size_t, mut seed: libc::c_uint)
 -> XXH32_hash_t {
    let mut endian_detected: XXH_endianess =
        *(&g_one as *const libc::c_int as *const libc::c_char) as
            XXH_endianess;
    if endian_detected as libc::c_uint ==
           XXH_littleEndian as libc::c_int as libc::c_uint || 0 != 0i32 {
        return XXH32_endian_align(input, len, seed, XXH_littleEndian,
                                  XXH_unaligned)
    } else {
        return XXH32_endian_align(input, len, seed, XXH_bigEndian,
                                  XXH_unaligned)
    };
}
unsafe extern "C" fn XXH32_endian_align(mut input: *const libc::c_void,
                                        mut len: size_t, mut seed: U32,
                                        mut endian: XXH_endianess,
                                        mut align: XXH_alignment) -> U32 {
    let mut p: *const BYTE = input as *const BYTE;
    let mut bEnd: *const BYTE = p.offset(len as isize);
    let mut h32: U32 = 0;
    if len >= 16i32 as libc::c_ulong {
        let limit: *const BYTE = bEnd.offset(-16isize);
        let mut v1: U32 =
            seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2);
        let mut v2: U32 = seed.wrapping_add(PRIME32_2);
        let mut v3: U32 = seed.wrapping_add(0i32 as libc::c_uint);
        let mut v4: U32 = seed.wrapping_sub(PRIME32_1);
        loop  {
            v1 =
                XXH32_round(v1,
                            XXH_readLE32_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(4isize);
            v2 =
                XXH32_round(v2,
                            XXH_readLE32_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(4isize);
            v3 =
                XXH32_round(v3,
                            XXH_readLE32_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(4isize);
            v4 =
                XXH32_round(v4,
                            XXH_readLE32_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(4isize);
            if !(p <= limit) { break ; }
        }
        h32 =
            (v1 << 1i32 |
                 v1 >>
                     32i32 -
                         1i32).wrapping_add(v2 << 7i32 |
                                                v2 >>
                                                    32i32 -
                                                        7i32).wrapping_add(v3
                                                                               <<
                                                                               12i32
                                                                               |
                                                                               v3
                                                                                   >>
                                                                                   32i32
                                                                                       -
                                                                                       12i32).wrapping_add(v4
                                                                                                               <<
                                                                                                               18i32
                                                                                                               |
                                                                                                               v4
                                                                                                                   >>
                                                                                                                   32i32
                                                                                                                       -
                                                                                                                       18i32)
    } else { h32 = seed.wrapping_add(PRIME32_5) }
    h32 = (h32 as libc::c_uint).wrapping_add(len as U32) as U32 as U32;
    while p.offset(4isize) <= bEnd {
        h32 =
            (h32 as
                 libc::c_uint).wrapping_add(XXH_readLE32_align(p as
                                                                   *const libc::c_void,
                                                               endian,
                                                               align).wrapping_mul(PRIME32_3))
                as U32 as U32;
        h32 = (h32 << 17i32 | h32 >> 32i32 - 17i32).wrapping_mul(PRIME32_4);
        p = p.offset(4isize)
    }
    while p < bEnd {
        h32 =
            (h32 as
                 libc::c_uint).wrapping_add((*p as
                                                 libc::c_uint).wrapping_mul(PRIME32_5))
                as U32 as U32;
        h32 = (h32 << 11i32 | h32 >> 32i32 - 11i32).wrapping_mul(PRIME32_1);
        p = p.offset(1isize)
    }
    h32 ^= h32 >> 15i32;
    h32 = (h32 as libc::c_uint).wrapping_mul(PRIME32_2) as U32 as U32;
    h32 ^= h32 >> 13i32;
    h32 = (h32 as libc::c_uint).wrapping_mul(PRIME32_3) as U32 as U32;
    h32 ^= h32 >> 16i32;
    return h32;
}
static mut PRIME32_3: U32 = unsafe { 3266489917u32 };
static mut PRIME32_2: U32 = unsafe { 2246822519u32 };
static mut PRIME32_1: U32 = unsafe { 2654435761u32 };
static mut PRIME32_5: U32 = unsafe { 374761393u32 };
static mut PRIME32_4: U32 = unsafe { 668265263u32 };
unsafe extern "C" fn XXH_readLE32_align(mut ptr: *const libc::c_void,
                                        mut endian: XXH_endianess,
                                        mut align: XXH_alignment) -> U32 {
    if align as libc::c_uint == XXH_unaligned as libc::c_int as libc::c_uint {
        return if endian as libc::c_uint ==
                      XXH_littleEndian as libc::c_int as libc::c_uint {
                   XXH_read32(ptr)
               } else { XXH_swap32(XXH_read32(ptr)) }
    } else {
        return if endian as libc::c_uint ==
                      XXH_littleEndian as libc::c_int as libc::c_uint {
                   *(ptr as *const U32)
               } else { XXH_swap32(*(ptr as *const U32)) }
    };
}
unsafe extern "C" fn XXH_swap32(mut x: U32) -> U32 {
    return x << 24i32 & 4278190080u32 |
               x << 8i32 & 16711680i32 as libc::c_uint |
               x >> 8i32 & 65280i32 as libc::c_uint |
               x >> 24i32 & 255i32 as libc::c_uint;
}
unsafe extern "C" fn XXH_read32(mut memPtr: *const libc::c_void) -> U32 {
    let mut val: U32 = 0;
    memcpy(&mut val as *mut U32 as *mut libc::c_void, memPtr,
           ::std::mem::size_of::<U32>() as libc::c_ulong);
    return val;
}
unsafe extern "C" fn XXH32_round(mut seed: U32, mut input: U32) -> U32 {
    seed =
        (seed as libc::c_uint).wrapping_add(input.wrapping_mul(PRIME32_2)) as
            U32 as U32;
    seed = seed << 13i32 | seed >> 32i32 - 13i32;
    seed = (seed as libc::c_uint).wrapping_mul(PRIME32_1) as U32 as U32;
    return seed;
}
static mut g_one: libc::c_int = unsafe { 1i32 };
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64(mut input: *const libc::c_void,
                                    mut len: size_t,
                                    mut seed: libc::c_ulonglong)
 -> XXH64_hash_t {
    let mut endian_detected: XXH_endianess =
        *(&g_one as *const libc::c_int as *const libc::c_char) as
            XXH_endianess;
    if endian_detected as libc::c_uint ==
           XXH_littleEndian as libc::c_int as libc::c_uint || 0 != 0i32 {
        return XXH64_endian_align(input, len, seed as U64, XXH_littleEndian,
                                  XXH_unaligned) as XXH64_hash_t
    } else {
        return XXH64_endian_align(input, len, seed as U64, XXH_bigEndian,
                                  XXH_unaligned) as XXH64_hash_t
    };
}
unsafe extern "C" fn XXH64_endian_align(mut input: *const libc::c_void,
                                        mut len: size_t, mut seed: U64,
                                        mut endian: XXH_endianess,
                                        mut align: XXH_alignment) -> U64 {
    let mut k1: U64 = 0;
    let mut p: *const BYTE = input as *const BYTE;
    let bEnd: *const BYTE = p.offset(len as isize);
    let mut h64: U64 = 0;
    if len >= 32i32 as libc::c_ulong {
        let limit: *const BYTE = bEnd.offset(-32isize);
        let mut v1: U64 =
            seed.wrapping_add(PRIME64_1).wrapping_add(PRIME64_2);
        let mut v2: U64 = seed.wrapping_add(PRIME64_2);
        let mut v3: U64 = seed.wrapping_add(0i32 as libc::c_ulong);
        let mut v4: U64 = seed.wrapping_sub(PRIME64_1);
        loop  {
            v1 =
                XXH64_round(v1,
                            XXH_readLE64_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(8isize);
            v2 =
                XXH64_round(v2,
                            XXH_readLE64_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(8isize);
            v3 =
                XXH64_round(v3,
                            XXH_readLE64_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(8isize);
            v4 =
                XXH64_round(v4,
                            XXH_readLE64_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(8isize);
            if !(p <= limit) { break ; }
        }
        h64 =
            (v1 << 1i32 |
                 v1 >>
                     64i32 -
                         1i32).wrapping_add(v2 << 7i32 |
                                                v2 >>
                                                    64i32 -
                                                        7i32).wrapping_add(v3
                                                                               <<
                                                                               12i32
                                                                               |
                                                                               v3
                                                                                   >>
                                                                                   64i32
                                                                                       -
                                                                                       12i32).wrapping_add(v4
                                                                                                               <<
                                                                                                               18i32
                                                                                                               |
                                                                                                               v4
                                                                                                                   >>
                                                                                                                   64i32
                                                                                                                       -
                                                                                                                       18i32);
        h64 = XXH64_mergeRound(h64, v1);
        h64 = XXH64_mergeRound(h64, v2);
        h64 = XXH64_mergeRound(h64, v3);
        h64 = XXH64_mergeRound(h64, v4)
    } else { h64 = seed.wrapping_add(PRIME64_5) }
    h64 = (h64 as libc::c_ulong).wrapping_add(len) as U64 as U64;
    while p.offset(8isize) <= bEnd {
        k1 =
            XXH64_round(0i32 as U64,
                        XXH_readLE64_align(p as *const libc::c_void, endian,
                                           align));
        h64 ^= k1;
        h64 =
            (h64 << 27i32 |
                 h64 >>
                     64i32 -
                         27i32).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
        p = p.offset(8isize)
    }
    if p.offset(4isize) <= bEnd {
        h64 ^=
            (XXH_readLE32_align(p as *const libc::c_void, endian, align) as
                 U64).wrapping_mul(PRIME64_1);
        h64 =
            (h64 << 23i32 |
                 h64 >>
                     64i32 -
                         23i32).wrapping_mul(PRIME64_2).wrapping_add(PRIME64_3);
        p = p.offset(4isize)
    }
    while p < bEnd {
        h64 ^= (*p as libc::c_ulong).wrapping_mul(PRIME64_5);
        h64 = (h64 << 11i32 | h64 >> 64i32 - 11i32).wrapping_mul(PRIME64_1);
        p = p.offset(1isize)
    }
    h64 ^= h64 >> 33i32;
    h64 = (h64 as libc::c_ulong).wrapping_mul(PRIME64_2) as U64 as U64;
    h64 ^= h64 >> 29i32;
    h64 = (h64 as libc::c_ulong).wrapping_mul(PRIME64_3) as U64 as U64;
    h64 ^= h64 >> 32i32;
    return h64;
}
static mut PRIME64_3: U64 = unsafe { 1609587929392839161u64 as U64 };
static mut PRIME64_2: U64 = unsafe { 14029467366897019727u64 as U64 };
static mut PRIME64_1: U64 = unsafe { 11400714785074694791u64 as U64 };
static mut PRIME64_5: U64 = unsafe { 2870177450012600261u64 as U64 };
static mut PRIME64_4: U64 = unsafe { 9650029242287828579u64 as U64 };
unsafe extern "C" fn XXH_readLE64_align(mut ptr: *const libc::c_void,
                                        mut endian: XXH_endianess,
                                        mut align: XXH_alignment) -> U64 {
    if align as libc::c_uint == XXH_unaligned as libc::c_int as libc::c_uint {
        return if endian as libc::c_uint ==
                      XXH_littleEndian as libc::c_int as libc::c_uint {
                   XXH_read64(ptr)
               } else { XXH_swap64(XXH_read64(ptr)) }
    } else {
        return if endian as libc::c_uint ==
                      XXH_littleEndian as libc::c_int as libc::c_uint {
                   *(ptr as *const U64)
               } else { XXH_swap64(*(ptr as *const U64)) }
    };
}
unsafe extern "C" fn XXH_swap64(mut x: U64) -> U64 {
    return ((x << 56i32) as libc::c_ulonglong & 18374686479671623680u64 |
                (x << 40i32) as libc::c_ulonglong & 71776119061217280u64 |
                (x << 24i32) as libc::c_ulonglong & 280375465082880u64 |
                (x << 8i32) as libc::c_ulonglong & 1095216660480u64 |
                (x >> 8i32) as libc::c_ulonglong & 4278190080u64 |
                (x >> 24i32) as libc::c_ulonglong & 16711680u64 |
                (x >> 40i32) as libc::c_ulonglong & 65280u64 |
                (x >> 56i32) as libc::c_ulonglong & 255u64) as U64;
}
unsafe extern "C" fn XXH_read64(mut memPtr: *const libc::c_void) -> U64 {
    let mut val: U64 = 0;
    memcpy(&mut val as *mut U64 as *mut libc::c_void, memPtr,
           ::std::mem::size_of::<U64>() as libc::c_ulong);
    return val;
}
unsafe extern "C" fn XXH64_round(mut acc: U64, mut input: U64) -> U64 {
    acc =
        (acc as libc::c_ulong).wrapping_add(input.wrapping_mul(PRIME64_2)) as
            U64 as U64;
    acc = acc << 31i32 | acc >> 64i32 - 31i32;
    acc = (acc as libc::c_ulong).wrapping_mul(PRIME64_1) as U64 as U64;
    return acc;
}
unsafe extern "C" fn XXH64_mergeRound(mut acc: U64, mut val: U64) -> U64 {
    val = XXH64_round(0i32 as U64, val);
    acc ^= val;
    acc = acc.wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
    return acc;
}
/* ! State allocation, compatible with dynamic libraries */
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32_createState() -> *mut XXH32_state_t {
    return XXH_malloc(::std::mem::size_of::<XXH32_state_t>() as libc::c_ulong)
               as *mut XXH32_state_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32_freeState(mut statePtr:
                                                  *mut XXH32_state_t)
 -> XXH_errorcode {
    XXH_free(statePtr as *mut libc::c_void);
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64_createState() -> *mut XXH64_state_t {
    return XXH_malloc(::std::mem::size_of::<XXH64_state_t>() as libc::c_ulong)
               as *mut XXH64_state_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64_freeState(mut statePtr:
                                                  *mut XXH64_state_t)
 -> XXH_errorcode {
    XXH_free(statePtr as *mut libc::c_void);
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32_reset(mut statePtr: *mut XXH32_state_t,
                                          mut seed: libc::c_uint)
 -> XXH_errorcode {
    let mut state: XXH32_state_t =
        XXH32_state_s{total_len_32: 0,
                      large_len: 0,
                      v1: 0,
                      v2: 0,
                      v3: 0,
                      v4: 0,
                      mem32: [0; 4],
                      memsize: 0,
                      reserved: 0,};
    memset(&mut state as *mut XXH32_state_t as *mut libc::c_void, 0i32,
           (::std::mem::size_of::<XXH32_state_t>() as
                libc::c_ulong).wrapping_sub(4i32 as libc::c_ulong));
    state.v1 = seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2);
    state.v2 = seed.wrapping_add(PRIME32_2);
    state.v3 = seed.wrapping_add(0i32 as libc::c_uint);
    state.v4 = seed.wrapping_sub(PRIME32_1);
    memcpy(statePtr as *mut libc::c_void,
           &mut state as *mut XXH32_state_t as *const libc::c_void,
           ::std::mem::size_of::<XXH32_state_t>() as libc::c_ulong);
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32_update(mut state_in: *mut XXH32_state_t,
                                           mut input: *const libc::c_void,
                                           mut len: size_t) -> XXH_errorcode {
    let mut endian_detected: XXH_endianess =
        *(&g_one as *const libc::c_int as *const libc::c_char) as
            XXH_endianess;
    if endian_detected as libc::c_uint ==
           XXH_littleEndian as libc::c_int as libc::c_uint || 0 != 0i32 {
        return XXH32_update_endian(state_in, input, len, XXH_littleEndian)
    } else {
        return XXH32_update_endian(state_in, input, len, XXH_bigEndian)
    };
}
unsafe extern "C" fn XXH32_update_endian(mut state: *mut XXH32_state_t,
                                         mut input: *const libc::c_void,
                                         mut len: size_t,
                                         mut endian: XXH_endianess)
 -> XXH_errorcode {
    let mut p: *const BYTE = input as *const BYTE;
    let bEnd: *const BYTE = p.offset(len as isize);
    (*state).total_len_32 =
        (*state).total_len_32.wrapping_add(len as libc::c_uint);
    (*state).large_len |=
        ((len >= 16i32 as libc::c_ulong) as libc::c_int |
             ((*state).total_len_32 >= 16i32 as libc::c_uint) as libc::c_int)
            as libc::c_uint;
    if ((*state).memsize as libc::c_ulong).wrapping_add(len) <
           16i32 as libc::c_ulong {
        XXH_memcpy(((*state).mem32.as_mut_ptr() as
                        *mut BYTE).offset((*state).memsize as isize) as
                       *mut libc::c_void, input, len);
        (*state).memsize = (*state).memsize.wrapping_add(len as libc::c_uint);
        return XXH_OK
    } else {
        if 0 != (*state).memsize {
            XXH_memcpy(((*state).mem32.as_mut_ptr() as
                            *mut BYTE).offset((*state).memsize as isize) as
                           *mut libc::c_void, input,
                       (16i32 as libc::c_uint).wrapping_sub((*state).memsize)
                           as size_t);
            let mut p32: *const U32 = (*state).mem32.as_mut_ptr();
            (*state).v1 =
                XXH32_round((*state).v1,
                            XXH_readLE32(p32 as *const libc::c_void, endian));
            p32 = p32.offset(1isize);
            (*state).v2 =
                XXH32_round((*state).v2,
                            XXH_readLE32(p32 as *const libc::c_void, endian));
            p32 = p32.offset(1isize);
            (*state).v3 =
                XXH32_round((*state).v3,
                            XXH_readLE32(p32 as *const libc::c_void, endian));
            p32 = p32.offset(1isize);
            (*state).v4 =
                XXH32_round((*state).v4,
                            XXH_readLE32(p32 as *const libc::c_void, endian));
            p32 = p32.offset(1isize);
            p =
                p.offset((16i32 as
                              libc::c_uint).wrapping_sub((*state).memsize) as
                             isize);
            (*state).memsize = 0i32 as libc::c_uint
        }
        if p <= bEnd.offset(-16isize) {
            let limit: *const BYTE = bEnd.offset(-16isize);
            let mut v1: U32 = (*state).v1;
            let mut v2: U32 = (*state).v2;
            let mut v3: U32 = (*state).v3;
            let mut v4: U32 = (*state).v4;
            loop  {
                v1 =
                    XXH32_round(v1,
                                XXH_readLE32(p as *const libc::c_void,
                                             endian));
                p = p.offset(4isize);
                v2 =
                    XXH32_round(v2,
                                XXH_readLE32(p as *const libc::c_void,
                                             endian));
                p = p.offset(4isize);
                v3 =
                    XXH32_round(v3,
                                XXH_readLE32(p as *const libc::c_void,
                                             endian));
                p = p.offset(4isize);
                v4 =
                    XXH32_round(v4,
                                XXH_readLE32(p as *const libc::c_void,
                                             endian));
                p = p.offset(4isize);
                if !(p <= limit) { break ; }
            }
            (*state).v1 = v1;
            (*state).v2 = v2;
            (*state).v3 = v3;
            (*state).v4 = v4
        }
        if p < bEnd {
            XXH_memcpy((*state).mem32.as_mut_ptr() as *mut libc::c_void,
                       p as *const libc::c_void,
                       p.offset_to(bEnd).expect("bad offset_to") as
                           libc::c_long as size_t);
            (*state).memsize =
                p.offset_to(bEnd).expect("bad offset_to") as libc::c_long as
                    libc::c_uint
        }
        return XXH_OK
    };
}
unsafe extern "C" fn XXH_readLE32(mut ptr: *const libc::c_void,
                                  mut endian: XXH_endianess) -> U32 {
    return XXH_readLE32_align(ptr, endian, XXH_unaligned);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32_digest(mut state_in: *const XXH32_state_t)
 -> XXH32_hash_t {
    let mut endian_detected: XXH_endianess =
        *(&g_one as *const libc::c_int as *const libc::c_char) as
            XXH_endianess;
    if endian_detected as libc::c_uint ==
           XXH_littleEndian as libc::c_int as libc::c_uint || 0 != 0i32 {
        return XXH32_digest_endian(state_in, XXH_littleEndian)
    } else { return XXH32_digest_endian(state_in, XXH_bigEndian) };
}
unsafe extern "C" fn XXH32_digest_endian(mut state: *const XXH32_state_t,
                                         mut endian: XXH_endianess) -> U32 {
    let mut p: *const BYTE = (*state).mem32.as_ptr() as *const BYTE;
    let bEnd: *const BYTE =
        ((*state).mem32.as_ptr() as
             *const BYTE).offset((*state).memsize as isize);
    let mut h32: U32 = 0;
    if 0 != (*state).large_len {
        h32 =
            ((*state).v1 << 1i32 |
                 (*state).v1 >>
                     32i32 -
                         1i32).wrapping_add((*state).v2 << 7i32 |
                                                (*state).v2 >>
                                                    32i32 -
                                                        7i32).wrapping_add((*state).v3
                                                                               <<
                                                                               12i32
                                                                               |
                                                                               (*state).v3
                                                                                   >>
                                                                                   32i32
                                                                                       -
                                                                                       12i32).wrapping_add((*state).v4
                                                                                                               <<
                                                                                                               18i32
                                                                                                               |
                                                                                                               (*state).v4
                                                                                                                   >>
                                                                                                                   32i32
                                                                                                                       -
                                                                                                                       18i32)
    } else { h32 = (*state).v3.wrapping_add(PRIME32_5) }
    h32 =
        (h32 as libc::c_uint).wrapping_add((*state).total_len_32) as U32 as
            U32;
    while p.offset(4isize) <= bEnd {
        h32 =
            (h32 as
                 libc::c_uint).wrapping_add(XXH_readLE32(p as
                                                             *const libc::c_void,
                                                         endian).wrapping_mul(PRIME32_3))
                as U32 as U32;
        h32 = (h32 << 17i32 | h32 >> 32i32 - 17i32).wrapping_mul(PRIME32_4);
        p = p.offset(4isize)
    }
    while p < bEnd {
        h32 =
            (h32 as
                 libc::c_uint).wrapping_add((*p as
                                                 libc::c_uint).wrapping_mul(PRIME32_5))
                as U32 as U32;
        h32 = (h32 << 11i32 | h32 >> 32i32 - 11i32).wrapping_mul(PRIME32_1);
        p = p.offset(1isize)
    }
    h32 ^= h32 >> 15i32;
    h32 = (h32 as libc::c_uint).wrapping_mul(PRIME32_2) as U32 as U32;
    h32 ^= h32 >> 13i32;
    h32 = (h32 as libc::c_uint).wrapping_mul(PRIME32_3) as U32 as U32;
    h32 ^= h32 >> 16i32;
    return h32;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64_reset(mut statePtr: *mut XXH64_state_t,
                                          mut seed: libc::c_ulonglong)
 -> XXH_errorcode {
    let mut state: XXH64_state_t =
        XXH64_state_s{total_len: 0,
                      v1: 0,
                      v2: 0,
                      v3: 0,
                      v4: 0,
                      mem64: [0; 4],
                      memsize: 0,
                      reserved: [0; 2],};
    memset(&mut state as *mut XXH64_state_t as *mut libc::c_void, 0i32,
           (::std::mem::size_of::<XXH64_state_t>() as
                libc::c_ulong).wrapping_sub(8i32 as libc::c_ulong));
    state.v1 =
        seed.wrapping_add(PRIME64_1 as
                              libc::c_ulonglong).wrapping_add(PRIME64_2 as
                                                                  libc::c_ulonglong);
    state.v2 = seed.wrapping_add(PRIME64_2 as libc::c_ulonglong);
    state.v3 = seed.wrapping_add(0i32 as libc::c_ulonglong);
    state.v4 = seed.wrapping_sub(PRIME64_1 as libc::c_ulonglong);
    memcpy(statePtr as *mut libc::c_void,
           &mut state as *mut XXH64_state_t as *const libc::c_void,
           ::std::mem::size_of::<XXH64_state_t>() as libc::c_ulong);
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64_update(mut state_in: *mut XXH64_state_t,
                                           mut input: *const libc::c_void,
                                           mut len: size_t) -> XXH_errorcode {
    let mut endian_detected: XXH_endianess =
        *(&g_one as *const libc::c_int as *const libc::c_char) as
            XXH_endianess;
    if endian_detected as libc::c_uint ==
           XXH_littleEndian as libc::c_int as libc::c_uint || 0 != 0i32 {
        return XXH64_update_endian(state_in, input, len, XXH_littleEndian)
    } else {
        return XXH64_update_endian(state_in, input, len, XXH_bigEndian)
    };
}
unsafe extern "C" fn XXH64_update_endian(mut state: *mut XXH64_state_t,
                                         mut input: *const libc::c_void,
                                         mut len: size_t,
                                         mut endian: XXH_endianess)
 -> XXH_errorcode {
    let mut p: *const BYTE = input as *const BYTE;
    let bEnd: *const BYTE = p.offset(len as isize);
    (*state).total_len =
        (*state).total_len.wrapping_add(len as libc::c_ulonglong);
    if ((*state).memsize as libc::c_ulong).wrapping_add(len) <
           32i32 as libc::c_ulong {
        XXH_memcpy(((*state).mem64.as_mut_ptr() as
                        *mut BYTE).offset((*state).memsize as isize) as
                       *mut libc::c_void, input, len);
        (*state).memsize = (*state).memsize.wrapping_add(len as U32);
        return XXH_OK
    } else {
        if 0 != (*state).memsize {
            XXH_memcpy(((*state).mem64.as_mut_ptr() as
                            *mut BYTE).offset((*state).memsize as isize) as
                           *mut libc::c_void, input,
                       (32i32 as libc::c_uint).wrapping_sub((*state).memsize)
                           as size_t);
            (*state).v1 =
                XXH64_round((*state).v1 as U64,
                            XXH_readLE64((*state).mem64.as_mut_ptr().offset(0isize)
                                             as *const libc::c_void, endian))
                    as libc::c_ulonglong;
            (*state).v2 =
                XXH64_round((*state).v2 as U64,
                            XXH_readLE64((*state).mem64.as_mut_ptr().offset(1isize)
                                             as *const libc::c_void, endian))
                    as libc::c_ulonglong;
            (*state).v3 =
                XXH64_round((*state).v3 as U64,
                            XXH_readLE64((*state).mem64.as_mut_ptr().offset(2isize)
                                             as *const libc::c_void, endian))
                    as libc::c_ulonglong;
            (*state).v4 =
                XXH64_round((*state).v4 as U64,
                            XXH_readLE64((*state).mem64.as_mut_ptr().offset(3isize)
                                             as *const libc::c_void, endian))
                    as libc::c_ulonglong;
            p =
                p.offset((32i32 as
                              libc::c_uint).wrapping_sub((*state).memsize) as
                             isize);
            (*state).memsize = 0i32 as libc::c_uint
        }
        if p.offset(32isize) <= bEnd {
            let limit: *const BYTE = bEnd.offset(-32isize);
            let mut v1: U64 = (*state).v1 as U64;
            let mut v2: U64 = (*state).v2 as U64;
            let mut v3: U64 = (*state).v3 as U64;
            let mut v4: U64 = (*state).v4 as U64;
            loop  {
                v1 =
                    XXH64_round(v1,
                                XXH_readLE64(p as *const libc::c_void,
                                             endian));
                p = p.offset(8isize);
                v2 =
                    XXH64_round(v2,
                                XXH_readLE64(p as *const libc::c_void,
                                             endian));
                p = p.offset(8isize);
                v3 =
                    XXH64_round(v3,
                                XXH_readLE64(p as *const libc::c_void,
                                             endian));
                p = p.offset(8isize);
                v4 =
                    XXH64_round(v4,
                                XXH_readLE64(p as *const libc::c_void,
                                             endian));
                p = p.offset(8isize);
                if !(p <= limit) { break ; }
            }
            (*state).v1 = v1 as libc::c_ulonglong;
            (*state).v2 = v2 as libc::c_ulonglong;
            (*state).v3 = v3 as libc::c_ulonglong;
            (*state).v4 = v4 as libc::c_ulonglong
        }
        if p < bEnd {
            XXH_memcpy((*state).mem64.as_mut_ptr() as *mut libc::c_void,
                       p as *const libc::c_void,
                       p.offset_to(bEnd).expect("bad offset_to") as
                           libc::c_long as size_t);
            (*state).memsize =
                p.offset_to(bEnd).expect("bad offset_to") as libc::c_long as
                    libc::c_uint
        }
        return XXH_OK
    };
}
unsafe extern "C" fn XXH_readLE64(mut ptr: *const libc::c_void,
                                  mut endian: XXH_endianess) -> U64 {
    return XXH_readLE64_align(ptr, endian, XXH_unaligned);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64_digest(mut state_in: *const XXH64_state_t)
 -> XXH64_hash_t {
    let mut endian_detected: XXH_endianess =
        *(&g_one as *const libc::c_int as *const libc::c_char) as
            XXH_endianess;
    if endian_detected as libc::c_uint ==
           XXH_littleEndian as libc::c_int as libc::c_uint || 0 != 0i32 {
        return XXH64_digest_endian(state_in, XXH_littleEndian) as XXH64_hash_t
    } else {
        return XXH64_digest_endian(state_in, XXH_bigEndian) as XXH64_hash_t
    };
}
unsafe extern "C" fn XXH64_digest_endian(mut state: *const XXH64_state_t,
                                         mut endian: XXH_endianess) -> U64 {
    let mut v1: U64 = 0;
    let mut v3: U64 = 0;
    let mut k1: U64 = 0;
    let mut v2: U64 = 0;
    let mut v4: U64 = 0;
    let mut p: *const BYTE = (*state).mem64.as_ptr() as *const BYTE;
    let bEnd: *const BYTE =
        ((*state).mem64.as_ptr() as
             *const BYTE).offset((*state).memsize as isize);
    let mut h64: U64 = 0;
    if (*state).total_len >= 32i32 as libc::c_ulonglong {
        v1 = (*state).v1 as U64;
        v2 = (*state).v2 as U64;
        v3 = (*state).v3 as U64;
        v4 = (*state).v4 as U64;
        h64 =
            (v1 << 1i32 |
                 v1 >>
                     64i32 -
                         1i32).wrapping_add(v2 << 7i32 |
                                                v2 >>
                                                    64i32 -
                                                        7i32).wrapping_add(v3
                                                                               <<
                                                                               12i32
                                                                               |
                                                                               v3
                                                                                   >>
                                                                                   64i32
                                                                                       -
                                                                                       12i32).wrapping_add(v4
                                                                                                               <<
                                                                                                               18i32
                                                                                                               |
                                                                                                               v4
                                                                                                                   >>
                                                                                                                   64i32
                                                                                                                       -
                                                                                                                       18i32);
        h64 = XXH64_mergeRound(h64, v1);
        h64 = XXH64_mergeRound(h64, v2);
        h64 = XXH64_mergeRound(h64, v3);
        h64 = XXH64_mergeRound(h64, v4)
    } else {
        h64 = (*state).v3.wrapping_add(PRIME64_5 as libc::c_ulonglong) as U64
    }
    h64 =
        (h64 as libc::c_ulong).wrapping_add((*state).total_len as U64) as U64
            as U64;
    while p.offset(8isize) <= bEnd {
        k1 =
            XXH64_round(0i32 as U64,
                        XXH_readLE64(p as *const libc::c_void, endian));
        h64 ^= k1;
        h64 =
            (h64 << 27i32 |
                 h64 >>
                     64i32 -
                         27i32).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
        p = p.offset(8isize)
    }
    if p.offset(4isize) <= bEnd {
        h64 ^=
            (XXH_readLE32(p as *const libc::c_void, endian) as
                 U64).wrapping_mul(PRIME64_1);
        h64 =
            (h64 << 23i32 |
                 h64 >>
                     64i32 -
                         23i32).wrapping_mul(PRIME64_2).wrapping_add(PRIME64_3);
        p = p.offset(4isize)
    }
    while p < bEnd {
        h64 ^= (*p as libc::c_ulong).wrapping_mul(PRIME64_5);
        h64 = (h64 << 11i32 | h64 >> 64i32 - 11i32).wrapping_mul(PRIME64_1);
        p = p.offset(1isize)
    }
    h64 ^= h64 >> 33i32;
    h64 = (h64 as libc::c_ulong).wrapping_mul(PRIME64_2) as U64 as U64;
    h64 ^= h64 >> 29i32;
    h64 = (h64 as libc::c_ulong).wrapping_mul(PRIME64_3) as U64 as U64;
    h64 ^= h64 >> 32i32;
    return h64;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32_copyState(mut dstState:
                                                  *mut XXH32_state_t,
                                              mut srcState:
                                                  *const XXH32_state_t)
 -> () {
    memcpy(dstState as *mut libc::c_void, srcState as *const libc::c_void,
           ::std::mem::size_of::<XXH32_state_t>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64_copyState(mut dstState:
                                                  *mut XXH64_state_t,
                                              mut srcState:
                                                  *const XXH64_state_t)
 -> () {
    memcpy(dstState as *mut libc::c_void, srcState as *const libc::c_void,
           ::std::mem::size_of::<XXH64_state_t>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32_canonicalFromHash(mut dst:
                                                          *mut XXH32_canonical_t,
                                                      mut hash: XXH32_hash_t)
 -> () {
    if 0 != *(&g_one as *const libc::c_int as *const libc::c_char) {
        hash = XXH_swap32(hash)
    }
    memcpy(dst as *mut libc::c_void,
           &mut hash as *mut XXH32_hash_t as *const libc::c_void,
           ::std::mem::size_of::<XXH32_canonical_t>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64_canonicalFromHash(mut dst:
                                                          *mut XXH64_canonical_t,
                                                      mut hash: XXH64_hash_t)
 -> () {
    if 0 != *(&g_one as *const libc::c_int as *const libc::c_char) {
        hash = XXH_swap64(hash as U64) as XXH64_hash_t
    }
    memcpy(dst as *mut libc::c_void,
           &mut hash as *mut XXH64_hash_t as *const libc::c_void,
           ::std::mem::size_of::<XXH64_canonical_t>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH32_hashFromCanonical(mut src:
                                                          *const XXH32_canonical_t)
 -> XXH32_hash_t {
    return XXH_readBE32(src as *const libc::c_void);
}
unsafe extern "C" fn XXH_readBE32(mut ptr: *const libc::c_void) -> U32 {
    return if 0 !=
                  *(&g_one as *const libc::c_int as *const libc::c_char) as
                      libc::c_int {
               XXH_swap32(XXH_read32(ptr))
           } else { XXH_read32(ptr) };
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_XXH64_hashFromCanonical(mut src:
                                                          *const XXH64_canonical_t)
 -> XXH64_hash_t {
    return XXH_readBE64(src as *const libc::c_void) as XXH64_hash_t;
}
unsafe extern "C" fn XXH_readBE64(mut ptr: *const libc::c_void) -> U64 {
    return if 0 !=
                  *(&g_one as *const libc::c_int as *const libc::c_char) as
                      libc::c_int {
               XXH_swap64(XXH_read64(ptr))
           } else { XXH_read64(ptr) };
}
