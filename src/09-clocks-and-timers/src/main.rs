#![no_main]
#![no_std]

use aux9::{entry, switch_hal::OutputSwitch, tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // Set the auto reload register to ms value
    tim6.arr.write(|w| w.arr().bits(ms));
    // enable the counter
    tim6.cr1.modify(|_,w| w.cen().set_bit());
    // Busy waiting for UIF bit to be set
    while !tim6.sr.read().uif().bit_is_set() {}
    // Clear the update event flag
    tim6.sr.modify(|_,w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = aux9::init();
    let mut leds = leds.into_array();

    // Add power to the TIM6 timer
    rcc.apb1enr.modify(|_,w|w.tim6en().set_bit());

    // One-pulse mode, keep counter disabled
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());

    // Configure the timer Prescaler value for 1 Khz
    tim6.psc.write(|w| w.psc().bits(7999));

    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().unwrap();
            delay(tim6, ms);
            leds[curr].off().unwrap();
            delay(tim6, ms);
        }
    }
}
