use fugit::{Duration, Instant};
use nrf52840_hal::Rtc;
use nrf52840_hal::pac::RTC0;


type TickInstant = Instant<u64, 1, 32768>;
type TickDuration = Duration<u64, 1, 32768>;

pub struct Timer<'a> {
    end_time: TickInstant,
    ticker: &'a Ticker,
}

impl<'a> Timer<'a> {
    pub fn new(duration: TickDuration, ticker: &'a Ticker) -> Self {
        Self {
            end_time: ticker.now() + duration,
            ticker,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.ticker.now() >= self.end_time
    }
}

/// Keeps track of time for the system using RTC0, which ticks away at a rate
/// of 32,768/sec using a low-power oscillator that runs even when the core is
/// powered down.
///
/// RTC0's counter is only 24-bits wide, which means there will be an overflow
/// every ~8min, which we do not account for: this will be fixed in chapter 4.
pub struct Ticker {
    rtc: Rtc<RTC0>,
}

impl Ticker {
    /// Create on startup to get RTC0 going.
    pub fn new(rtc0: RTC0) -> Self {
        let rtc = Rtc::new(rtc0, 0).unwrap();
        rtc.enable_counter();
        Self { rtc }
    }

    pub fn now(&self) -> TickInstant {
        TickInstant::from_ticks(self.rtc.get_counter() as u64)
    }
}
