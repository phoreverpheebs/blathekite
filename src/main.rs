#![allow(dead_code, unused_variables)]
use std::{
    // arch::asm,
    mem, ptr,
};

const HAI: &str = "hai";

fn main() {
    let scfunc = unsafe {
        let mut addr = mem::transmute::<fn(*mut u32, u32, u32, u32, &mut u8) -> u32, *mut u8>(int80);
        while ptr::read(addr) != 0x97 {
            addr = (addr as usize + 1) as *mut u8;
        }
        mem::transmute::<*mut u8, fn(usize) -> usize>(addr)
    };

    let hai = unsafe {
        mem::transmute::<&str, (usize, usize)>(HAI).0
    };

    not_syscall(4, 0, 1, hai, scfunc);
}

#[inline(never)]
fn not_syscall(
    eax: usize, 
    _ebx: usize, 
    edx: usize,
    ecx: usize, 
    sc: fn(usize) -> usize,
) -> usize {
    sc(eax);
    edx + ecx
}

pub fn int80(a: *mut u32, _b: u32, c: u32, d: u32, x: &mut u8) -> u32 {
    unsafe { *((a as usize + 0x05eb) as *mut u32) = c; }
    *x = 0xcd; // won't be optimised away
    let mut d = d as u8; /* makes sense to use the variable in ecx, because modr/m will give 0xf9
    which is a legal instruction for user-space, whereas using the variable in edx (c: u32) will give
    0xfa modr/m (cli), which would raise a #GP(0) */
    while d < 0xc2 { // an 8-bit comparison, to get an early ret
        d += 1;
    }
    d as u32
}
