#![feature(lang_items)]
#![no_std]

#![feature(plugin)]
#![plugin(bytestool)]
#[macro_use(sized_bytes)]

extern crate sizedbytes;
use sizedbytes::SizedBytes;

#[no_mangle]
pub extern fn kmain() -> ! {
    const MESG : SizedBytes = sized_bytes!(b"Hello World");

    let color_byte = 0x1f;

    let mut message = [color_byte; MESG.size * 2 ];
    for (i, char_byte) in MESG.bytes.into_iter().enumerate() {
      message[i * 2] = *char_byte;
    }

    let buffer_ptr = (0xb8000  +1988) as *mut _;

    unsafe { *buffer_ptr = message };

    loop{}

}

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn rust_begin_panic() -> ! {
    loop {}
}
