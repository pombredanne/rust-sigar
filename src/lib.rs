extern crate libc;

use std::default::Default;

mod ffi;

#[derive(Debug)]
pub struct Memory {
    ram: u64,
    total: u64,
    used: u64,
    free: u64,
    actual_used: u64,
    actual_free: u64,

    used_percent: f64,
    free_percent: f64
}

pub fn memory() -> Result<Memory, String> {
    let mut sigar_ptr : *mut ffi::sigar_t = std::ptr::null_mut();

    let res = unsafe { ffi::sigar_open(&mut sigar_ptr) };
    if res != 0 {
        return Err(ffi::error(sigar_ptr, res))
    }

    let mut mem: ffi::sigar_mem_t = Default::default();

    let res = unsafe { ffi::sigar_mem_get(sigar_ptr, &mut mem) };
    if res != 0 {
        return Err(ffi::error(sigar_ptr, res))
    }

    let res = unsafe { ffi::sigar_close(sigar_ptr) };
    if res != 0 {
        Err("failed to close sigar".to_string())
    } else {
        Ok(Memory{
            ram: mem.ram,
            total: mem.total,
            used: mem.used,
            free: mem.free,
            actual_used: mem.actual_used,
            actual_free: mem.actual_free,

            used_percent: mem.used_percent,
            free_percent: mem.free_percent,
        })
    }
}

// unsafe {
//     let mut sigar_ptr : *mut sigar_t = std::ptr::null_mut();
//
//     let res = sigar_open(&mut sigar_ptr);
//     if res != 0 {
//         println!("{}", error(sigar_ptr, res))
//     }
//
//     let mut mem = sigar_mem_t {
//         ram: 0,
//         total: 0,
//         used: 0,
//         free: 0,
//         actual_used: 0,
//         actual_free: 0,
//
//         used_percent: 0.0,
//         free_percent: 0.0
//     };
//
//     let res = sigar_mem_get(sigar_ptr, &mut mem);
//     if res != 0 {
//         println!("{}", error(sigar_ptr, res))
//     }
//     println!("{:?}", mem);
//
//     let mut cpu = sigar_cpu_t {
//         user: 0,
//         sys: 0,
//         nice: 0,
//         idle: 0,
//         wait: 0,
//         irq: 0,
//         soft_irq: 0,
//         stolen: 0,
//         total: 0,
//     };
//
//     let res = sigar_cpu_get(sigar_ptr, &mut cpu);
//     if res != 0 {
//         println!("{}", error(sigar_ptr, res))
//     }
//     println!("{:?}", cpu);
//
//     let res = sigar_close(sigar_ptr);
//     if res != 0 {
//         println!("{}", error(sigar_ptr, res))
//     }
// };
