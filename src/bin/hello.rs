#![no_main]
#![no_std]

use knurling_test as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    knurling_test::exit()
}
