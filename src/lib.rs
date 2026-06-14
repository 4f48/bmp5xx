// Copyright (C) 2026 Olivér Pirger
// SPDX-License-Identifier: GPL-3.0-or-later

#![no_std]
#![warn(missing_docs)]

//! True-to-spec async I2C driver for the BMP580/BMP581/BMP585 barometric pressure sensors.
//!
//! ## Features
//! - `no_std` compatible, works without an allocator
//! - Widely compatible, generic over [`embedded_hal_async`] I2C traits
//! - 100% documentation coverage
//! - Based on Bosch Sensortec BMP581 datasheet, written by a human
//! - Fully tested on real hardware: compatible with [Adafruit BMP581](https://www.adafruit.com/product/6407) development board
//!
//! ## Usage
//!
//! Getting started is easy:
//! ```rs
//! // initialize the sensor
//! let mut bmp5 = Bmp5xx::new(i2c, Delay, 0x47);
//! bmp5.init().await.unwrap();
//!
//! // new pressure measurement
//! let pressure = bmp5.meas_pres().await.unwrap();
//! ```
//!
//! Advanced operations:
//! ```rs
//! // change oversampling rate
//! bmp5.osr_temp(Oversampling::X8);
//! bmp5.osr_pres(Oversampling::X128);
//!
//! // set up interrupts
//! bmp5.int(Interrupt::default().enable(true)).await.unwrap();
//!
//! // start continuous measurement
//! bmp5.start_continuous(true).await.unwrap();
//! ```

use byteorder::{ByteOrder, LittleEndian};
use embedded_hal_async::{delay::DelayNs, i2c::I2c};

use crate::{
    error::{
        Error::{InvalidId, NotReady, ReadError, WriteError},
        Result,
    },
    osr::Oversampling,
    register::{CHIP_ID, CMD, INT_STATUS, ODR_CONFIG, PRESS_DATA_XLSB, STATUS, TEMP_DATA_XLSB},
};

mod continuous;
pub mod error;
mod forced;
pub mod iir;
pub mod int;
pub mod normal;
pub mod oor;
pub mod osr;
mod register;

/// Async I2C driver compatible with BMP580/BMP581/BMP585 barometric pressure sensors.
pub struct Bmp5xx<I2C, D> {
    i2c: I2C,
    delay: D,
    addr: u8,

    osr_t: Oversampling,
    osr_p: Oversampling,
}

// Basic functions and operations.
impl<I2C, D> Bmp5xx<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    /// Creates a new driver instance.
    pub fn new(i2c: I2C, delay: D, addr: u8) -> Self {
        Self {
            i2c,
            delay,
            addr,

            osr_t: Oversampling::X1,
            osr_p: Oversampling::X1,
        }
    }

    /// Triggers a software reset on the sensor. Takes about 2 ms.
    pub async fn reset(&mut self) -> Result<()> {
        // trigger reset
        self.i2c
            .write(self.addr, &[CMD, 0xB6])
            .await
            .map_err(|_| WriteError)?;

        // t_soft_res
        self.delay.delay_ms(2).await;

        Ok(())
    }

    /// Initializes the sensor and readies it for further operations.
    /// Run this before starting other operations, otherwise the sensor may behave unexpectedly.
    /// You only need to initialize the sensor once.
    pub async fn init(&mut self) -> Result<()> {
        let mut id_buf = [0u8; 1];
        self.i2c
            .write_read(self.addr, &[CHIP_ID], &mut id_buf)
            .await
            .map_err(|_| ReadError)?;
        match id_buf[0] {
            0x50 | 0x51 => (),
            _ => return Err(InvalidId),
        }

        self.reset().await?;

        let mut status_buf = [0u8; 1];
        self.i2c
            .write_read(self.addr, &[STATUS], &mut status_buf)
            .await
            .map_err(|_| ReadError)?;
        if status_buf[0] & 0x02 == 0 || status_buf[0] & 0x04 != 0 {
            return Err(NotReady);
        }

        let mut int_status_buf = [0u8; 1];
        self.i2c
            .write_read(self.addr, &[INT_STATUS], &mut int_status_buf)
            .await
            .map_err(|_| ReadError)?;
        if int_status_buf[0] & 0x10 == 0 {
            return Err(NotReady);
        }

        Ok(())
    }
}

// Generic functions and operations.
impl<I2C, D> Bmp5xx<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    /// Reads the latest temperature reading in degrees Celsius (°C), useful for normal and continuous mode operation.
    ///
    /// This doesn't initialize a new measurement, use [`meas_temp`](Self::meas_temp) for that.
    pub async fn read_temp(&mut self) -> Result<f32> {
        let mut temp_buf = [0u8; 3];
        self.i2c
            .write_read(self.addr, &[TEMP_DATA_XLSB], &mut temp_buf)
            .await
            .map_err(|_| ReadError)?;
        Ok(LittleEndian::read_i24(&temp_buf) as f32 / 65536.0)
    }

    /// Reads the latest pressure reading in hectopascals (hPa), useful for normal and continuous mode operation.
    ///
    /// This doesn't initialize a new measurement, use [`meas_pres`](Self::meas_pres) for that.
    pub async fn read_pres(&mut self) -> Result<f32> {
        let mut pres_buf = [0u8; 3];
        self.i2c
            .write_read(self.addr, &[PRESS_DATA_XLSB], &mut pres_buf)
            .await
            .map_err(|_| ReadError)?;
        Ok(LittleEndian::read_i24(&pres_buf) as f32 / 64.0 / 100.0)
    }

    /// Stop measurement and return the sensor to standby mode.
    pub async fn stop(&mut self) -> Result<()> {
        self.i2c
            .write(self.addr, &[ODR_CONFIG, 0x00])
            .await
            .map_err(|_| WriteError)
    }
}
