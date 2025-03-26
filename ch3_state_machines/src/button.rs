use embedded_hal::digital::InputPin;
use fugit::ExtU64;
//use microbit::hal::gpio::PullUp;
//use microbit::hal::gpio::{Floating, Input, Pin};
use nrf52840_hal::gpio::PullUp;
use nrf52840_hal::gpio::Input;
use nrf52840_hal::gpio::Pin;

#[cfg(target_os = "none")] // embedded target
use defmt::debug;

use crate::{
    channel::Sender,
    time::{Ticker, Timer},
};

#[derive(Clone, Copy)]
pub enum ButtonDirection {
    Left,
    Right,
}

enum ButtonState<'a> {
    WaitForPress,
    Debounce(Timer<'a>),
}

pub struct ButtonTask<'a> {
    pin: Pin<Input<PullUp>>,
    ticker: &'a Ticker,
    direction: ButtonDirection,
    state: ButtonState<'a>,
    sender: Sender<'a, ButtonDirection>,
}

impl<'a> ButtonTask<'a> {
    pub fn new(
        pin: Pin<Input<PullUp>>,
        ticker: &'a Ticker,
        direction: ButtonDirection,
        sender: Sender<'a, ButtonDirection>,
    ) -> Self {
        Self {
            pin,
            ticker,
            direction,
            state: ButtonState::WaitForPress,
            sender,
        }
    }

    pub fn poll(&mut self) {
        match self.state {
            ButtonState::WaitForPress => {
                if self.pin.is_low().unwrap() {
                    debug!("Press!");
                    self.sender.send(self.direction);
                    self.state = ButtonState::Debounce(Timer::new(100.millis(), &self.ticker));
                    debug!("Debounced!");
                }
            }
            ButtonState::Debounce(ref timer) => {
                if timer.is_ready() && self.pin.is_high().unwrap() {
                    self.state = ButtonState::WaitForPress;
                }
            }
        }
    }
}
