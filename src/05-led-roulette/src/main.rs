#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let led_period = 50_u16;

    loop {
        for i in 0..=7 {
            let next = (i + 1) % 8;
            leds[next].on().ok();
            delay.delay_ms(led_period);

            leds[i].off().ok();
            delay.delay_ms(led_period);
        }

    }
}