use std::{ mem, ptr };

const HAI: &str = "hai world <3";

fn main() {
    let mut addr: *mut u8;
    let notsyscall = unsafe {
        addr = mem::transmute::<fn(&mut u32, u8, &mut u32, &mut u16) -> u8, *mut u8>(syscall);
        while ptr::read(addr) != 0x84 {
            addr = (addr as usize + 1) as *mut u8;
        }
        mem::transmute::<*mut u8, fn(usize, usize, usize, usize, usize) -> usize>(addr) 
            // we can manipulate certain register states using function parameters
            //                        edi    esi    edx    ecx    r8
    };

    notsyscall(1, HAI.as_ptr() as usize, HAI.len(), 0, 1);
}

fn syscall(edi: &mut u32, sil: u8, edx: &mut u32, ecx: &mut u16) -> u8 {
    let mut acc = *edi as u8 + sil;
    if acc > 0x90 { /* Without runtime conditional it'd get optimised */
        *edx += 0x8b49c084;
        acc <<= 3;
        *ecx += 0x050f; /* This feels like cheating */
    }
    acc
}

