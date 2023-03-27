#![allow(non_camel_case_types, non_upper_case_globals)]
use std::{mem::transmute as trans, ptr::read as slay};
use std::{ptr, mem};

/* Imported functions will live at a lower address */
mod volatile;

/* 6 usize arguments to fill up the registers and 2 stack arguments.
 * If we only use 1 stack argument, it'll be optimised to a mov [rsp], $1
 * however two stack arguments force a push, giving us a 32-bit push instruction */
type she = fn(usize, usize, usize, usize, usize, isize, usize, u32) -> usize;
type he = fn(&mut u32, u8, &mut u32, &mut u16, &mut u32, isize, usize) -> u8;
type any = *mut u8;

const done_questioning: u8 = 0x84;

#[inline(always)]
unsafe fn until(mut me: *mut u8, byte: u8) -> *mut u8 {
    while slay(me) != byte {
        me = (me as usize + 1) as *mut u8;
    }
    me
}

fn main() {
    let mut jmp_difference: isize = unsafe {
        trans::<fn(_, _, _) -> _, isize>(volatile::modify)
    };
    let chosenname = unsafe {
        let me = trans::<he, any>(deadname);
        let out = trans::<any, she>(until(me, done_questioning));
        jmp_difference -= until(me, 0xc3) as isize - 0x24;
        out
    };
    printit(unsafe { &(*trans::<fn(_) -> _, *const u8>(rexw) | 0x20) }, chosenname, jmp_difference);
}

fn deadname(
    di: &mut u32, 
    si: u8, 
    dx: &mut u32, 
    cx: &mut u16, 
    _: &mut u32,
    r9: isize, /* amount to increment return address by */
    stk0: usize, /* placeholder to get return address */
) -> u8 {
    /* This initial addition is put here, to get rid of the compiler
     * using a lea instead of an add instruction. */
    let mut acc = *di as u8 + si;
    if acc > 0x90 {
        /* Without runtime conditional it'd get optimised */
        *dx += 0x8b49c084;
        acc <<= 3;
        *cx += 0x050f; /* This feels like cheating */
    }

    /* The following mem::size_of is unnecessary as this implementation would fail on
     * non-64-bit architectures. */
    unsafe {
        (*ptr::slice_from_raw_parts_mut(
            ((&stk0 as *const usize) as usize - mem::size_of::<usize>() as usize) as *mut isize,
            1,
        ))[0] += r9;
    }
    acc
}

#[inline(always)]
fn printit(address: *const u8, chosenname: she, jmp: isize) -> usize {
    chosenname(
        1, address as usize, 1, 0, 1, jmp, /* register values */
        0, 0x6cc3, /* stack values */
    )
}

#[inline]
fn rexw(p: *const u64) -> u64 {
    unsafe { *p }
}
