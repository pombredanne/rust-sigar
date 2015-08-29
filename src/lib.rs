extern crate libc;

use std::default::Default;

mod ffi;

#[derive(Debug)]
pub struct Memory {
    pub ram: u64,
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub actual_used: u64,
    pub actual_free: u64,

    pub used_percent: f64,
    pub free_percent: f64
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

#[derive(Debug)]
pub struct CPU {
    pub user: u64,
    pub sys: u64,
    pub nice: u64,
    pub idle: u64,
    pub wait: u64,
    pub irq: u64,
    pub soft_irq: u64,
    pub stolen: u64,
    pub total: u64,
}

pub fn cpu() -> Result<CPU, String> {
    let mut sigar_ptr : *mut ffi::sigar_t = std::ptr::null_mut();

    let res = unsafe { ffi::sigar_open(&mut sigar_ptr) };
    if res != 0 {
        return Err(ffi::error(sigar_ptr, res))
    }

    let mut cpu: ffi::sigar_cpu_t = Default::default();

    let res = unsafe { ffi::sigar_cpu_get(sigar_ptr, &mut cpu) };
    if res != 0 {
        return Err(ffi::error(sigar_ptr, res))
    }

    let res = unsafe { ffi::sigar_close(sigar_ptr) };
    if res != 0 {
        Err("failed to close sigar".to_string())
    } else {
        Ok(CPU{
            user: cpu.user,
            sys: cpu.sys,
            nice: cpu.nice,
            idle: cpu.idle,
            wait: cpu.wait,
            irq: cpu.irq,
            soft_irq: cpu.soft_irq,
            stolen: cpu.stolen,
            total: cpu.total,
        })
    }
}

#[derive(Debug)]
pub struct Swap {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub page_in: u64,
    pub page_out: u64,
}

pub fn swap() -> Result<Swap, String> {
    let mut sigar_ptr : *mut ffi::sigar_t = std::ptr::null_mut();

    let res = unsafe { ffi::sigar_open(&mut sigar_ptr) };
    if res != 0 {
        return Err(ffi::error(sigar_ptr, res))
    }

    let mut swap: ffi::sigar_swap_t = Default::default();

    let res = unsafe { ffi::sigar_swap_get(sigar_ptr, &mut swap) };
    if res != 0 {
        return Err(ffi::error(sigar_ptr, res))
    }

    let res = unsafe { ffi::sigar_close(sigar_ptr) };
    if res != 0 {
        Err("failed to close sigar".to_string())
    } else {
        Ok(Swap{
            total: swap.total,
            used: swap.used,
            free: swap.free,
            page_in: swap.page_in,
            page_out: swap.page_out,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::cpu;

    #[test]
    fn it_collects_cpu_information() {
        let cpu = super::cpu().ok().expect("failed to get cpu information");

        assert!(cpu.user > 0);
        assert!(cpu.sys > 0);
        assert!(cpu.idle > 0);
        assert!(cpu.total > 0);

        assert_eq!(cpu.user + cpu.sys + cpu.idle, cpu.total);
    }

    #[test]
    fn it_collects_memory_information() {
        let memory = super::memory().ok().expect("failed to get memory information");

        assert!(memory.free > 0);
        assert!(memory.used > 0);
        assert!(memory.total > 0);

        assert_eq!(memory.free + memory.used, memory.total);
    }
}
