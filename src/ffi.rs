use std::default::Default;
use std::ffi::CStr;
use std::str;

use libc::{c_int, c_double, c_char, uint64_t};

#[link(name = "sigar")]
extern {
    // SIGAR_DECLARE(int) sigar_open(sigar_t **sigar);
    pub fn sigar_open(sigar: *mut *mut sigar_t) -> c_int;

    // SIGAR_DECLARE(int) sigar_close(sigar_t *sigar);
    pub fn sigar_close(sigar: *mut sigar_t) -> c_int;

    // SIGAR_DECLARE(char *) sigar_strerror(sigar_t *sigar, int err);
    fn sigar_strerror(sigar: *mut sigar_t, code: c_int) -> &mut c_char;


    // SIGAR_DECLARE(int) sigar_mem_get(sigar_t *sigar, sigar_mem_t *mem);
    pub fn sigar_mem_get(sigar: *mut sigar_t, mem: *mut sigar_mem_t) -> c_int;

    // SIGAR_DECLARE(int) sigar_cpu_get(sigar_t *sigar, sigar_cpu_t *cpu);
    pub fn sigar_cpu_get(sigar: *mut sigar_t, mem: *mut sigar_cpu_t) -> c_int;

    // SIGAR_DECLARE(int) sigar_swap_get(sigar_t *sigar, sigar_swap_t *swap);
    pub fn sigar_swap_get(sigar: *mut sigar_t, swap: *mut sigar_swap_t) -> c_int;
}

#[repr(C)]
pub struct sigar_t;

#[repr(C)]
#[derive(Debug)]
pub struct sigar_mem_t {
    pub ram: uint64_t,
    pub total: uint64_t,
    pub used: uint64_t,
    pub free: uint64_t,
    pub actual_used: uint64_t,
    pub actual_free: uint64_t,

    pub used_percent: c_double,
    pub free_percent: c_double
}

impl Default for sigar_mem_t {
    fn default() -> sigar_mem_t {
        sigar_mem_t {
            ram: 0,
            total: 0,
            used: 0,
            free: 0,
            actual_used: 0,
            actual_free: 0,

            used_percent: 0.0,
            free_percent: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct sigar_cpu_t {
    pub user: uint64_t,
    pub sys: uint64_t,
    pub nice: uint64_t,
    pub idle: uint64_t,
    pub wait: uint64_t,
    pub irq: uint64_t,
    pub soft_irq: uint64_t,
    pub stolen: uint64_t,
    pub total: uint64_t,
}

impl Default for sigar_cpu_t {
    fn default() -> sigar_cpu_t {
        sigar_cpu_t {
            user: 0,
            sys: 0,
            nice: 0,
            idle: 0,
            wait: 0,
            irq: 0,
            soft_irq: 0,
            stolen: 0,
            total: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct sigar_swap_t {
    pub total: uint64_t,
    pub used: uint64_t,
    pub free: uint64_t,
    pub page_in: uint64_t,
    pub page_out: uint64_t,
}

impl Default for sigar_swap_t {
    fn default() -> sigar_swap_t {
        sigar_swap_t {
            total: 0,
            used: 0,
            free: 0,
            page_in: 0,
            page_out: 0,
        }
    }
}

pub fn error(sigar: *mut sigar_t, code: c_int) -> String {
    unsafe {
        let ptr = sigar_strerror(sigar, code);
        let bytes = CStr::from_ptr(ptr).to_bytes();
        str::from_utf8(bytes).ok().expect("Invalid UTF8 string").to_string()
    }
}
