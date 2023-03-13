use std::mem;

fn main() {
    let adder: fn(i32, i32) -> i32 = unsafe { 
        mem::transmute::<usize, fn(i32, i32) -> i32>(
            mem::transmute::<fn(i32, i32) -> i32, usize>(add) + 3)
    };
    println!("0x{:x}", adder(5, 5));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
