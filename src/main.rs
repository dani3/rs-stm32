#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate stm32f1;

pub use cortex_m_rt::entry;
pub use panic_itm;

pub use stm32f1xx_hal::prelude::*;

pub use cortex_m::{iprintln, peripheral::ITM};

#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();

    let mut itm = p.ITM;

    iprintln!(&mut itm.stim[0], "Hello, world!");

    // infinite loop; just so we don't leave this stack frame
    loop {}
}
