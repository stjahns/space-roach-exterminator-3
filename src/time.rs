///
/// Temporarily pulled from https://github.com/rust-lang/time
/// as cheap workaround for wacky miniz linking confict
///
use libc;

mod imp {
    use libc::{c_int, timespec};

    #[cfg(all(not(target_os = "android"),
              not(target_os = "bitrig"),
              not(target_os = "nacl"),
              not(target_os = "openbsd")))]
    #[link(name = "rt")]
    extern {}

    extern {
        pub fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    }

}

fn os_precise_time_ns() -> u64 {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    unsafe {
        imp::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts);
    }
    return (ts.tv_sec as u64) * 1000000000 + (ts.tv_nsec as u64)
}

pub fn precise_time_s() -> f64 {
    return (os_precise_time_ns() as f64) / 1000000000.;
}
