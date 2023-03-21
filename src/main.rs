#![allow(dead_code, unused_variables)]
use std::{
    // arch::asm,
    mem, ptr,
};

const HAI: &str = "hai";

fn main() {
    let volebx = HAI.as_ptr(); // the first variable declared is in ebx

    let scfunc = unsafe {
        let mut addr = mem::transmute::<fn(u32, u32) -> u32, *mut u8>(sc);
        while ptr::read(addr) != 0x0f {
            addr = (addr as usize + 1) as *mut u8;
        }
        mem::transmute::<usize, fn(usize) -> usize>(addr as usize + 3)
    };

    not_syscall(1, 0, 1, 1, scfunc);

    _ = volebx;
}

fn void(a: usize) -> usize {
    a
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
    eax
}

#[inline(never)]
fn sc(a: u32, b: u32) -> u32 {
    let mut r = a + b;
    r &= 0xf;
    r + 0xfaeb97c3
}
