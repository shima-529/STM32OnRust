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
