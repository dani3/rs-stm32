#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate stm32f1;

pub use cortex_m_rt::entry;
pub use panic_itm;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;

    // infinite loop; just so we don't leave this stack frame
    loop {}
}
