// Copyright (C) 2026 Olivér Pirger
// SPDX-License-Identifier: GPL-3.0-or-later

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
    /// Triggers a new temperature measurement and returns it in degrees Celsius (°C) as soon as it's complete.
    pub async fn meas_temp(&mut self) -> Result<f32> {
        self.stop().await?;

        self.conf_osr(false).await?;

        // switch to forced mode
        self.i2c
            .write(self.addr, &[ODR_CONFIG, 0x02])
            .await
            .map_err(|_| WriteError)?;

        self.delay
            .delay_us((self.osr_t.t_conv_t() * 1050.0) as u32)
            .await;

        self.read_temp().await
    }

    /// Triggers a new pressure measurement and returns it in hectopascals (hPa) as soon as it's complete.
    pub async fn meas_pres(&mut self) -> Result<f32> {
        self.stop().await?;

        self.conf_osr(true).await?;

        self.i2c
            .write(self.addr, &[ODR_CONFIG, 0x02])
            .await
            .map_err(|_| WriteError)?;

        self.delay
            .delay_us((self.osr_p.t_conv_p() * 1050.0) as u32)
            .await;

        self.read_pres().await
    }
}
