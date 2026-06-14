// Copyright (C) 2026 Olivér Pirger
// SPDX-License-Identifier: GPL-3.0-or-later

//! Normal mode operations for BMP580/BMP581/BMP585 sensors. Normal mode does periodic measurements with a specified data output rate.

use embedded_hal_async::{delay::DelayNs, i2c::I2c};

use crate::{
    Bmp5xx,
    error::{
        Error::{OdrInvalid, ReadError, WriteError},
        Result,
    },
    register::{ODR_CONFIG, OSR_EFF},
};

/// How many measurements should be made per second. Values are in Hertz (Hz; 1/s).
/// Check out the documentation for each variant for more details.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum OutputDataRate {
    /// 240 Hz
    Hz240 = 0x00,
    /// 218.537 Hz (0.67 error)
    Hz218 = 0x01,
    /// 199.111 Hz (0.44 error)
    Hz199 = 0x02,
    /// 179.2 Hz (0.44 error)
    Hz179 = 0x03,
    /// 160 Hz
    Hz160 = 0x04,
    /// 149.333 Hz (0.44 error)
    Hz149 = 0x05,
    /// 140 Hz
    Hz140 = 0x06,
    /// 129.855 Hz (0.11 error)
    Hz129 = 0x07,
    /// 120 Hz
    Hz120 = 0x08,
    /// 110.164 Hz (0.15 error)
    Hz110 = 0x09,
    /// 100.299 Hz (0.30 error)
    Hz100 = 0x0A,
    /// 89.6 Hz (0.44 error)
    Hz089 = 0x0B,
    /// 80 Hz
    Hz080 = 0x0C,
    /// 70 Hz
    Hz070 = 0x0D,
    /// 60 Hz
    Hz060 = 0x0E,
    /// 50.056 Hz (0.11 error)
    Hz050 = 0x0F,
    /// 45.025 Hz (0.06 error)
    Hz045 = 0x10,
    /// 40 Hz
    Hz040 = 0x11,
    /// 35 Hz
    Hz035 = 0x12,
    /// 30 Hz
    Hz030 = 0x13,
    /// 25.005 Hz (0.02 error)
    Hz025 = 0x14,
    /// 20 Hz
    Hz020 = 0x15,
    /// 15 Hz
    Hz015 = 0x16,
    /// 10 Hz
    Hz010 = 0x17,
    /// 5 Hz
    Hz005 = 0x18,
    /// 4 Hz
    Hz004 = 0x19,
    /// 3 Hz
    Hz003 = 0x1A,
    /// 2 Hz
    Hz002 = 0x1B,
    /// 1 Hz
    Hz001 = 0x1C,
    /// 0.5 Hz
    Hz000_5 = 0x1D,
    /// 0.25 Hz
    Hz000_25 = 0x1E,
    /// 0.125 Hz
    Hz000_125 = 0x1F,
}

impl<I2C, D> Bmp5xx<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    /// Puts the sensor into normal mode, making new measurements periodically.
    /// Use [`read_temp`](Self::read_temp) and [`read_pres`](Self::read_pres) to get the results.
    /// Use [`stop`](Self::stop) to stop the measurements.
    ///
    /// Warning: this returns an error if the oversampling configuration is incompatible with the output data rate given.
    ///
    /// Note: using interrupts is highly recommended for normal mode.
    pub async fn start_normal(&mut self, press_en: bool, odr: OutputDataRate) -> Result<()> {
        self.stop().await?;

        self.conf_osr(press_en).await?;

        // set output data rate and switch to normal mode
        self.i2c
            .write(self.addr, &[ODR_CONFIG, (odr as u8) << 2 | 0x01])
            .await
            .map_err(|_| WriteError)?;

        let mut osr_eff_buf = [0u8; 1];
        self.i2c
            .write_read(self.addr, &[OSR_EFF], &mut osr_eff_buf)
            .await
            .map_err(|_| ReadError)?;
        if osr_eff_buf[0] & 0x80 == 0 {
            return Err(OdrInvalid);
        }

        Ok(())
    }
}
