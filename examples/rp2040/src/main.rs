// Copyright (C) 2026 Olivér Pirger
// SPDX-License-Identifier: GPL-3.0-or-later

#![no_std]
#![no_main]

use bmp5xx::Bmp5xx;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    i2c::{self, I2c},
    peripherals::{I2C1, USB},
    usb::{self, Driver},
};
use embassy_time::{Delay, Timer};
use log::info;
use panic_reset as _;

bind_interrupts!(struct Irqs {
    I2C1_IRQ => i2c::InterruptHandler<I2C1>;
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let usb_driver = Driver::new(p.USB, Irqs);
    spawner.spawn(usb_logger(usb_driver).unwrap());

    let i2c = I2c::new_async(p.I2C1, p.PIN_3, p.PIN_2, Irqs, Default::default());
    let mut bmp = Bmp5xx::new(i2c, Delay, 0x47);
    bmp.init().await.unwrap();

    loop {
        let pressure = bmp.meas_pres().await.unwrap();
        info!("{}", pressure);
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::task]
async fn usb_logger(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(512, log::LevelFilter::Info, driver);
}
