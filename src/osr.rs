// Copyright (C) 2026 Olivér Pirger
// SPDX-License-Identifier: GPL-3.0-or-later

//! Oversampling configuration for BMP580/BMP581/BMP585 sensors.

use embedded_hal_async::{delay::DelayNs, i2c::I2c};

use crate::{
    Bmp5xx,
    error::{Error::WriteError, Result},
    register::OSR_CONFIG,
};

/// Selection of oversampling rates for temperature and pressure readings.
///
/// Note that not all OSR and ODR combinations are valid.
/// You should check that your oversampling configuration's measurement time
/// fits in your output data rate cycle.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Oversampling {
    /// 1x oversampling (no oversampling)
    X1 = 0x00,
    /// 2x oversampling
    X2 = 0x01,
    /// 4x oversampling
    X4 = 0x02,
    /// 8x oversampling
    X8 = 0x03,
    /// 16x oversampling
    X16 = 0x04,
    /// 32x oversampling
    X32 = 0x05,
    /// 64x oversampling
    X64 = 0x06,
    /// 128x oversampling
    X128 = 0x07,
}

impl Oversampling {
    pub(crate) fn t_conv_t(&self) -> f32 {
        match self {
            Oversampling::X1 => 1.0,
            Oversampling::X2 => 1.1,
            Oversampling::X4 => 1.5,
            Oversampling::X8 => 2.1,
            Oversampling::X16 => 3.3,
            Oversampling::X32 => 5.8,
            Oversampling::X64 => 10.8,
            Oversampling::X128 => 20.8,
        }
    }

    pub(crate) fn t_conv_p(&self) -> f32 {
        match self {
            Oversampling::X1 => 1.0,
            Oversampling::X2 => 1.7,
            Oversampling::X4 => 2.9,
            Oversampling::X8 => 5.4,
            Oversampling::X16 => 10.4,
            Oversampling::X32 => 20.4,
            Oversampling::X64 => 40.4,
            Oversampling::X128 => 80.4,
        }
    }
}

impl<I2C, D> Bmp5xx<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    /// Sets the oversampling rate for temperature readings.
    pub fn osr_temp(&mut self, osr: Oversampling) {
        self.osr_t = osr;
    }

    /// Sets the oversampling rate for pressure readings.
    pub fn osr_pres(&mut self, osr: Oversampling) {
        self.osr_p = osr;
    }
}

impl<I2C, D> Bmp5xx<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    pub(crate) async fn conf_osr(&mut self, press_en: bool) -> Result<()> {
        self.i2c
            .write(
                self.addr,
                &[
                    OSR_CONFIG,
                    (press_en as u8) << 6 | (self.osr_p as u8) << 3 | self.osr_t as u8,
                ],
            )
            .await
            .map_err(|_| WriteError)
    }
}
