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
    let n = unsafe {
        /* It will never actually return to here (compiler doesn't know that) */
        std::mem::transmute::<_, fn() -> u64>(std::mem::transmute::<fn(_, _) -> _, usize>(incrementjmp) + dx * 5)()
    };
    n + force_call(n)
}

#[inline(never)]
pub fn force_call(n: u64) -> u64 {
    let (mut a, mut b, mut d, e, f) = (n, n, n, n, n);
    let mut c = a;
    unsafe {
        std::mem::transmute::<
            fn(&mut u64, &mut u64, &mut u64, &mut u64, &mut u64, &mut u64, &mut u64) -> u64,
            fn(&mut u64, &mut u64, &mut u64, &mut u64),
        >(morefunc)(&mut a, &mut b, &mut c, &mut d);
    }
    a + b + d + e + f + 0x80005beb /* set sign bit, first 3 bytes are under our control */
}

#[used]
static gaslight_the_compiler: fn(
    &mut u64,
    &mut u64,
    &mut u64,
    &mut u64,
    &mut u64,
    &mut u64,
    &mut u64,
) -> u64 = morefunc;
/* -----------------------------------------------------------------------
 * An odd nuance about function inlining:
 *  the compiler deems 'morefunc' as unused either if it is
 *  only used in this function or if it is only declared as a static
 *  variable with #[used]. However, if both of these methods are used
 *  the function compiles.
 */

#[inline(never)]
fn morefunc(
    a: &mut u64,
    b: &mut u64,
    c: &mut u64,
    d: &mut u64,
    e: &mut u64,
    f: &mut u64,
    g: &mut u64,
) -> u64 {
    *a += 1701536116;
    *b += 1701994851;
    *c += 1920296559;
    *d += 1718379891;
    *e += 15180358;
    *f += 560036206;
    *a >>= 2;
    *g <<= 8;

    let mut n = *g;
    n %= 0x15959f76458e3241;
    catch_from_modulo(n)
}

#[inline(never)]
fn catch_from_modulo(a: u64) -> u64 {
    let a = a + 0x77;
    if a >= 0x0feb51480ae98091 {
        let mut pls = Please {
            a: a,
            b: 0x10ebce8948d1ff + (a >> 2),
        };
        (a >> 2) + another(&mut pls)
    } else {
        a - 0xd514ba117cb70e78
    }
}

struct Please {
    a: u64,
    b: u64,
}

#[inline(never)]
fn another(a: &mut Please) -> u64 {
    /* blackboxing both values instead of blackboxing the expression
     * as a whole allows us to keep the compiler from precalculating it */
    (std::hint::black_box(a.a) + std::hint::black_box(a.b)) / keep_going(std::hint::black_box(0x69ebd1fff9498d48))
    /* note for division magic number: we want to subtract 7 from ecx (80/83 e9 07) to fix
     * where it points and then we call ecx (ff d1) */
}

#[inline(never)]
fn keep_going(a: u64) -> u64 {
    a + std::hint::black_box(0x14eb902824748d48) + std::hint::black_box(0x34ebd1ff07e98348)
        >> just_keep_working(std::hint::black_box(a), a)
}

#[inline(never)]
fn just_keep_working(a: u64, b: u64) -> u8 {
    (a + (b / 0x90d1ff5959183681)) as u8
}
