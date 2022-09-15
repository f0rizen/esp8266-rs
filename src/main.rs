#![no_std]
#![no_main]

use panic_halt as _;

use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = dp.GPIO.split();

    let mut led = pins.gpio2.into_push_pull_output();
    let (mut timer, _) = dp.TIMER.timers();

    led.set_high().unwrap();

    loop {
        timer.delay_ms(1000);
        led.toggle().unwrap();
    }
}
