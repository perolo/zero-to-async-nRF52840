#![no_std]
#![no_main]

mod button;
mod channel;
mod led;
mod time;

use button::{ButtonDirection, ButtonTask};
use channel::Channel;

use cortex_m_rt::entry;
//use embedded_hal::digital::OutputPin;
use led::LedTask;
//use microbit::Board;
//use panic_rtt_target as _;
//use rtt_target::rtt_init_print;
use nrf52840_hal as _; // memory layout

//use cortex_m::Peripherals;
use nrf52840_hal::{self as hal, gpio::Level, pac::Peripherals};

use time::Ticker;
//use time::Timer;

#[cfg(target_os = "none")] // embedded target
use defmt::info;

use panic_rtt_target as _;
use defmt_rtt as _;


#[entry]
fn main() -> ! {
    //rtt_init_print!();
    //let board = Board::take().unwrap();
    //let ticker = Ticker::new(board.RTC0);
    //let (col, mut row) = board.display_pins.degrade();
    //row[0].set_high().ok();
    //let button_l = board.buttons.button_a.degrade();
    //let button_r = board.buttons.button_b.degrade();
    info!("Hello, ch3_state_machines!");
    let peripherals = Peripherals::take().unwrap();    
    //let mut timer = Timer::new(peripherals.TIMER0);
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);
    let led = [
        port0.p0_13.into_push_pull_output(Level::High).degrade(),
        port0.p0_14.into_push_pull_output(Level::High).degrade(),
        port0.p0_15.into_push_pull_output(Level::High).degrade(),
        port0.p0_16.into_push_pull_output(Level::High).degrade(),
    ];
    let  button_l = port0.p0_11.into_pullup_input().degrade();
    let  button_r = port0.p0_12.into_pullup_input().degrade();
    let ticker = Ticker::new(peripherals.RTC0);

    let channel: Channel<ButtonDirection> = Channel::new();
    let mut led_task = LedTask::new(led, &ticker, channel.get_receiver());
    let mut button_l_task = ButtonTask::new(
        button_l,
        &ticker,
        ButtonDirection::Left,
        channel.get_sender(),
    );
    let mut button_r_task = ButtonTask::new(
        button_r,
        &ticker,
        ButtonDirection::Right,
        channel.get_sender(),
    );

    loop {
        led_task.poll();
        button_l_task.poll();
        button_r_task.poll();
    }
}
