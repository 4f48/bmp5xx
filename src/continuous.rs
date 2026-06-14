// Copyright (C) 2026 Olivér Pirger
// SPDX-License-Identifier: GPL-3.0-or-later

//! Continuous mode operations for BMP580/BMP581/BMP585 sensors.
//! Continuous mode does repeated measurements without any delay, outputting data as fast as possible.

use embedded_hal_async::{delay::DelayNs, i2c::I2c};

use crate::{
    Bmp5xx,
    error::{Error::WriteError, Result},
    register::ODR_CONFIG,
};

impl<I2C, D> Bmp5xx<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    /// Puts the sensor into continuous mode, making new measurements non-stop, without any delay.
    /// Use [`read_temp`](Self::read_temp) and [`read_pres`](Self::read_pres) to get the results.
    /// Use [`stop`](Self::stop) to stop the measurements.
    pub async fn start_continuous(&mut self, press_en: bool) -> Result<()> {
        self.stop().await?;

        self.conf_osr(press_en).await?;

        // switch to continuous mode
        self.i2c
            .write(self.addr, &[ODR_CONFIG, 0x03])
            .await
            .map_err(|_| WriteError)
    }
}
