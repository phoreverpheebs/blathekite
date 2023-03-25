use std::{
    mem::transmute as trans,
};

#[inline(never)]
pub fn incrementjmp(di: u64, si: u64) -> u64 {
    if si + 0xc3d23c000000 > 0x3cf3167f {
        si
    } else {
        di
    }
}

#[inline(never)]
pub fn modify(_: u32, si: &mut usize) -> u64 {
    *si -= 7;
    unsafe {
        trans::<_, fn() -> u64>( trans::<fn(_, _) -> _, usize>(incrementjmp) + 5)()
    }
}

/*
 * #[used]
 * static gaslight_the_compiler: fn(u64, u64) -> u64 = volatile::incrementjmp;
---------------------------------------------------------------------------
 * An odd nuance about function existence:
 *  the compiler deems 'incrementjmp' as unused either if it is
 *  only used in this function or if it is only declared as a static
 *  variable with #[used]. However, if both of these methods are used
 *  the function compiles.
 *
 * The above no longer happens since the outcome of 'modify' depends on the
 * address of 'incrementjmp', however if it ended with (where 'a' is the 
 * first _: u32 parameter):
 *
 *          incrementjmp(a as u64, (a as u64) << 5)
 *
 * 'gaslight_the_compiler' would have to exist for 'incrementjmp' to be
 * compiled as a function.
 */
