#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(const_raw_ptr_deref)]
#![feature(linkage)]
#![allow(non_snake_case)] // suppress a trivial warning
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused)]
pub use crate::main;
extern crate r0;

extern "C" {
    static mut _sidata: u32;
    static mut _sdata: u32;
    static mut _edata: u32;
    static mut _sbss: u32;
    static mut _ebss: u32;
}

#[no_mangle]
pub extern fn Reset_Handler() {
    unsafe {
        r0::init_data(&mut _sdata, &mut _edata, &mut _sidata);
        r0::zero_bss(&mut _sbss, &mut _ebss);
    }
    main();
    loop{}
}
