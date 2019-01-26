#![feature(libc)]
#![feature(ptr_wrapping_offset_from)]
#![feature(used)]
#![feature(extern_types)]
extern crate libc;
pub mod decompress;
pub mod compress;
pub mod common;
pub mod dictBuilder;
#[no_mangle]
pub extern "C" fn ZSTD_cpuid() {
}
extern "C" fn ZSTD_cpuid_bmi2(_cpuid: ()) -> libc::c_int {
    is_x86_feature_detected!("bmi2") as libc::c_int
}

