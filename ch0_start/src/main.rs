#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{
    delay::DelayNs, digital::StatefulOutputPin
};
use nrf52840_hal::{self as hal, gpio::Level, pac::Peripherals, Timer};

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use nrf52840_hal as _; // memory layout

#[entry]
fn main() -> ! {
 //   defmt::info!("Hello, World!");
    rtt_init_print!();
    let peripherals = Peripherals::take().unwrap();    
    let mut timer = Timer::new(peripherals.TIMER0);
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);
    let mut led = port0.p0_13.into_push_pull_output(Level::Low).degrade();

    loop {
        led.toggle().ok();
        timer.delay_ms(200);
    }
}
