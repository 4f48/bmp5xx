// Copyright (C) 2026 Olivér Pirger
// SPDX-License-Identifier: GPL-3.0-or-later

//! Out-of-range feature configuration for BMP580/BMP581/BMP585 sensors.

use embedded_hal_async::{delay::DelayNs, i2c::I2c};

use crate::{
    Bmp5xx,
    error::{
        Error::{BadInput, InvalidMinMax, WriteError},
        Result,
    },
    register::OOR_THR_P_LSB,
};

/// After how many measurements should the out-of-range feature trigger.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum CountLimit {
    /// Trigger immediately.
    Limit1 = 0x00,
    /// Trigger after 3 consecutive out-of-range measurements.
    Limit3 = 0x01,
    /// Trigger after 7 consecutive out-of-range measurements.
    Limit7 = 0x02,
    /// Trigger after 15 consecutive out-of-range measurements.
    Limit15 = 0x03,
}

impl<I2C, D> Bmp5xx<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    /// Configure the out-of-range feature. It's useful to enable the out-of-range interrupt source for this.
    /// The `min` and `max` values should be pressure in hectopascals (hPa).
    pub async fn oor(&mut self, min: f32, max: f32, count: CountLimit) -> Result<()> {
        if min > max {
            return Err(InvalidMinMax);
        }
        let reference = (min + max) / 2.0;
        let window = (max - min) / 2.0;

        let reference = reference * 100.0 + 0.5;
        let window = window * 100.0 + 0.5;

        if reference > 0x1FFFF as f32 || window > u8::MAX as f32 {
            return Err(BadInput);
        }

        let reference = reference as u32;
        let window = window as u8;

        self.i2c
            .write(
                self.addr,
                &[
                    OOR_THR_P_LSB,
                    (reference & 0xFF) as u8,
                    ((reference >> 8) & 0xFF) as u8,
                    window,
                    (count as u8) << 6 | ((reference >> 16) & 0x01) as u8,
                ],
            )
            .await
            .map_err(|_| WriteError)
    }
}
