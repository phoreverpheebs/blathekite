use std::{
    mem::transmute as trans,
};

#[inline(never)]
pub fn incrementjmp(di: u64, si: u64) -> u64 {
    if si + 0xc3d1ff00c88c9837 > 0x3ca316d0 {
        si
    } else {
        di
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

#[inline(never)]
pub fn modify(_: u32, si: &mut usize, dx: usize) -> u64 {
    *si -= 7;
    let n = unsafe { /* It will never actually return to here (compiler doesn't know that) */
        trans::<_, fn() -> u64>( trans::<fn(_, _) -> _, usize>(incrementjmp) + dx*5)()
    };
    n + continue_from_here(n as usize)
}

#[inline(never)]
pub fn continue_from_here(mut di: usize) -> u64 {
    if di | 5 < 523 {
        (di as u64) << 2
    } else {
        modify(523, &mut di, 82535) /* These numbers are random (for now) */
    }
    /* Extend this function */
}
