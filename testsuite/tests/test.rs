#![no_std]
#![no_main]

use defmt_rtt as _; // global logger
use panic_probe as _;

use microbit::hal::nrf51 as _;

#[defmt_test::tests]
mod tests {
   #[test]
   fn assert_true() {
       assert!(true)
   }

   #[test]
   fn assert_false() {
       assert!(false)
   }
}
