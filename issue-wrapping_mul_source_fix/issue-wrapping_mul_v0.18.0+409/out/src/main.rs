#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn set_rand_seed(s: uint32_t);
    fn get_rand_seed() -> uint32_t;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
unsafe fn main_0() -> libc::c_int {
    set_rand_seed(1234 as libc::c_int as uint32_t);
    printf(
        b"get_rand_seed: 0x%08X\n\0" as *const u8 as *const libc::c_char,
        get_rand_seed(),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
