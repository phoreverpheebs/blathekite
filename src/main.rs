#![allow(non_camel_case_types, non_upper_case_globals)]
use std::{mem::transmute as trans, ptr::read as slay};

const hai: &str = "hai world <3";

type he = fn(&mut u32, u8, &mut u32, &mut u16) -> u8;
type she = fn(usize, usize, usize, usize, usize) -> usize;
type any = *mut u8;

const done_questioning: u8 = 0x84;

fn main() {
    let chosenname = unsafe {
        let mut queer = trans::<he, any>(deadname);
        while slay(queer) != done_questioning {
            queer = (queer as usize + 1) as *mut u8;
        }
        trans::<any, she>(queer)
    };
    chosenname(
        1,
        unsafe { trans::<_, (usize, usize)>(hai).0 },
        hai.len(),
        0,
        1,
    );
}

fn deadname(edi: &mut u32, sil: u8, edx: &mut u32, ecx: &mut u16) -> u8 {
    let mut acc = *edi as u8 + sil;
    if acc > 0x90 {
        /* Without runtime conditional it'd get optimised */
        *edx += 0x8b49c084;
        acc <<= 3;
        *ecx += 0x050f; /* This feels like cheating */
    }
    acc
}
