#![allow(unused_imports)]

use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{IOPin, PinDriver},
    peripherals::Peripherals,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;
// use log::*;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let mut led_pin = PinDriver::output(peripherals.pins.gpio8).unwrap();

    loop {
        led_pin.set_low().unwrap();
        println!("LED ON");
        FreeRtos::delay_ms(1000);
        led_pin.set_high().unwrap();
        println!("LED OFF");
        FreeRtos::delay_ms(1000);
    }
}
