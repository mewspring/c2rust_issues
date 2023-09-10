use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
#[no_mangle]
pub static mut cur_rand_seed: int32_t = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn set_rand_seed(mut s: int32_t) {
    cur_rand_seed = s;
}
#[no_mangle]
pub unsafe extern "C" fn get_rand_seed() -> int32_t {
    let INCREMENT: uint32_t = 1 as libc::c_int as uint32_t;
    let MULTIPLIER: uint32_t = 0x15a4e35 as libc::c_int as uint32_t;
    cur_rand_seed = (MULTIPLIER * cur_rand_seed as uint32_t).wrapping_add(INCREMENT)
        as int32_t;
    let mut ret: int32_t = abs(cur_rand_seed);
    return ret;
}
