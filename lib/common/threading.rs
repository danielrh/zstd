#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(libc)]
extern crate libc;
/* *
 * Copyright (c) 2016 Tino Reichardt
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 *
 * You can contact the author at:
 * - zstdmt source repository: https://github.com/mcmilk/zstdmt
 */
/* *
 * This file will hold wrapper for systems, which do not support pthreads
 */
/* create fake symbol to avoid empty trnaslation unit warning */
#[no_mangle]
pub static mut g_ZSTD_threading_useles_symbol: libc::c_int = 0;