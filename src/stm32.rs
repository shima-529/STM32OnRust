use core::intrinsics::{volatile_store, volatile_load};

pub struct st_rcc {
    pub CR: u32,
    pub CFGR: u32,
    pub CIR: u32,
    pub APB2STR: u32,
    pub APB1STR: u32,
    pub AHBENR: u32,
    pub APB2ENR: u32,
    pub APB1ENR: u32,
    pub BDCR: u32,
    pub CSR: u32,
    pub AHBSTR: u32,
    pub CFGR2: u32,
    pub CFGR3: u32
}

pub struct st_gpio {
    pub MODER: u32,
    pub OTYPER: u32,
    pub OSPEEDR: u32,
    pub PUPDR: u32,
    pub IDR: u32,
    pub ODR: u32,
    pub BSRR: u32,
    pub LCKR: u32,
    pub AFRL: u32,
    pub AFRH: u32,
    pub BRR: u32
}

pub struct st_systick {
    pub CTRL: u32,
    pub LOAD: u32,
    pub VAL: u32
}

pub const RCC: *mut st_rcc = 0x4002_1000 as *mut st_rcc;
pub const GPIOF: *mut st_gpio = 0x4800_1400 as *mut st_gpio;
pub const SysTick: *mut st_systick = 0xE000_E010 as *mut st_systick;

#[macro_export]
macro_rules! reg {
    ($x:ident) => {
        (&mut *$x)
    }
}

#[macro_export]
macro_rules! st {
    ($x:ident.$y:ident, $z:expr) => {
        unsafe {
            volatile_store(&mut reg!($x).$y, $z)
        }
    };
    ($x:expr, $y:expr) => {
        unsafe {
            volatile_store($x, $y)
        }
    };
}

#[macro_export]
macro_rules! ld {
    ($x:ident.$y:ident) => {
        (|| {
            unsafe {
                volatile_load(&mut reg!($x).$y)
            }
        })()
    };
    ($x:expr) => {
        (|| {
            unsafe {
                volatile_load($x)
            }
        })()
    };
}

#[macro_export]
macro_rules! bset {
    ($x:ident.$y:ident, $z:expr) => {
        st!($x.$y, ld!($x.$y) | (1 << $z))
    }
}

#[macro_export]
macro_rules! bclr {
    ($x:ident.$y:ident, $z:expr) => {
        st!($x.$y, ld!($x.$y) & !(1 << $z))
    }
}

#[macro_export]
macro_rules! bitIsSet {
    ($x:ident.$y:ident, $z:expr) => {
        (ld!($x.$y) & (1 << $z)) != 0
    }
}

#[macro_export]
macro_rules! bitIsClr {
    ($x:ident.$y:ident, $z:expr) => {
        !bitIsSet!($x.$y, $z)
    }
}
