#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![allow(unused_unsafe)]
#![allow(non_snake_case)] // suppress a trivial warning
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
use core::intrinsics::{volatile_store, volatile_load};

mod stm32;
use self::stm32::*;
mod startup;
pub use self::startup::*;
mod vector;
pub use self::vector::*;

pub fn ms_wait(ms: u16) {
    st!(SysTick.LOAD, 1000 - 1);
    st!(SysTick.VAL, 0);
    bset!(SysTick.CTRL, 0);
    for _ in 0..ms {
        while bitIsClr!(SysTick.CTRL, 16) {}
    }
    bclr!(SysTick.CTRL, 0);
}

#[no_mangle]
pub extern fn main() {
    bset!(RCC.AHBENR, 22);
    st!(GPIOF.MODER, 1);

    loop{
        ms_wait(100);
        st!(GPIOF.ODR, 1);
        ms_wait(500);
        st!(GPIOF.ODR, 0);
    }
}

#[panic_handler]
pub fn panic_fmt(_ : &core::panic::PanicInfo) ->! { loop{} }
