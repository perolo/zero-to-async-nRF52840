#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{
    delay::DelayNs, digital::StatefulOutputPin
};
use nrf52840_hal::{self as hal, gpio::Level, pac::Peripherals, Timer};
use embedded_hal::digital::InputPin;

use panic_rtt_target as _;
use defmt_rtt as _;

use nrf52840_hal as _; // memory layout

#[cfg(target_os = "none")] // embedded target
use defmt::{info, debug};

#[entry]
fn main() -> ! {
    info!("Hello, ch1_setup!");
    let peripherals = Peripherals::take().unwrap();    
    let mut timer = Timer::new(peripherals.TIMER0);
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);
    let mut led = port0.p0_13.into_push_pull_output(Level::High).degrade();
    let mut button_l = port0.p0_11.into_pullup_input().degrade();
    let mut button_r = port0.p0_12.into_pullup_input().degrade();

    loop {
        led.toggle().ok();
        // blocking here:
        timer.delay_ms(500);
        // will prevent timely detection & response to these:
        if button_l.is_low().unwrap() {
            debug!("button l low!");
            //..
        }
        if button_r.is_low().unwrap() {
            debug!("button r low!");
            //..
        }
    }
}
