#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};
use heapless::Vec;

#[entry]
unsafe fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();


    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8,32> = Vec::new();

    loop {
        // clear the buffer before receiving data
        buffer.clear();

        loop {
            // Wait for data
            while usart1.isr.read().rxne().bit_is_clear() {}
            // Retrieve the data and push into buffer

            let byte = usart1.rdr.read().rdr().bits() as u8;
            if buffer.push(byte).is_err() {
                for byte in b"error: buffer full\n\r" {
                    // Wait for transmit
                    while usart1.isr.read().txe().bit_is_clear() {} // <- NEW!
                    usart1
                        .tdr
                        .write(|w| w.tdr().bits(u16::from(*byte) ));
                }
            }

            // Carriage return
            if byte == 13 {
                // Respond
                for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                    while usart1.isr.read().txe().bit_is_clear() {}
                    usart1
                        .tdr
                        .write(|w| w.tdr().bits(u16::from(*byte)));
                }
                break;
            }


        }
    }
}

