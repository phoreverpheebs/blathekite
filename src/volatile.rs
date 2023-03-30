use std::{
    mem::transmute as trans,
};

#[inline(never)]
pub fn incrementjmp(di: u64, si: u64) -> u64 {
    if si + 0xc3d1ff00a68c9837 > 0x3ca316d0 {
        si
    } else {
        di
    }
}

#[inline(never)]
pub fn modify(_: u32, si: &mut usize, dx: usize) -> u64 {
    *si -= 7;
    let n = unsafe { /* It will never actually return to here (compiler doesn't know that) */
        trans::<_, fn() -> u64>( trans::<fn(_, _) -> _, usize>(incrementjmp) + dx*5)()
    };
    n + force_call(n)
}

/* Without this function, the above `incrementjmp + dx*5` call will get optimised
 * to a jmp, causing the return address to not be on the stack. */
#[inline(never)]
fn force_call(n: u64) -> u64 {
    let (mut a, mut b, mut e, mut f) = (n, n, n, n);
    let mut c = a;
    let mut d = 0x24041f0f;
    unsafe {
        trans::<fn(
            &mut u64, &mut u64, &mut u64, &mut u64, &mut u64, &mut u64, &mut u64,
        ) -> u64, fn(
            &mut u64, &mut u64, &mut u64, &mut u64, &mut u64, &mut u64,
        )>(morefunc)(&mut a, &mut b, &mut c, &mut d, &mut e, &mut f);
    }
    a + b + c + d + e + f + 0x7deb9066 /* Setting the sign bit moves the compiled insn up */
}

#[used]
static gaslight_the_compiler: fn(
    &mut u64, &mut u64, &mut u64, &mut u64, &mut u64, &mut u64, &mut u64, 
) -> u64 = morefunc;
/* -----------------------------------------------------------------------
 * An odd nuance about function existence:
 *  the compiler deems 'morefunc' as unused either if it is
 *  only used in this function or if it is only declared as a static
 *  variable with #[used]. However, if both of these methods are used
 *  the function compiles.
 */

#[inline(never)]
fn morefunc(
    a: &mut u64, b: &mut u64, c: &mut u64, 
    d: &mut u64, e: &mut u64, f: &mut u64,
    g: &mut u64,
) -> u64 {
    *a += 1701536116;
    *b += 1701994851;
    *c += 1920296559;
    *d += 1718379891;
    *e += 15180358;
    *f += 560036206;
    *g += 1259;
    *a >>= 2;
    *b >>= 3;
    *c >>= 5;
    *d >>= 2;
    *e <<= 2;
    *f >>= 6;
    *g <<= 8;

    let mut n = *g;
    n += *e;
    n %= 0x7e8fdf;
    n
}

