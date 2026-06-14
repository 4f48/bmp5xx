// Copyright (C) 2026 Olivér Pirger
// SPDX-License-Identifier: GPL-3.0-or-later

//! IIR filtering configuration for BMP580/BMP581/BMP585 sensors.

use embedded_hal_async::{delay::DelayNs, i2c::I2c};

use crate::{
    Bmp5xx,
    error::{Error::WriteError, Result},
    register::{DSP_CONFIG, DSP_IIR},
};

/// IIR low-pass filtering configuration.
pub struct Iir {
    enabled: bool,
    iir_t: Filter,
    iir_p: Filter,
}

impl Default for Iir {
    fn default() -> Self {
        Self {
            enabled: false,
            iir_t: Filter::Bypass,
            iir_p: Filter::Bypass,
        }
    }
}

impl Iir {
    /// Enable or disable filtering.
    pub fn enable(mut self, enable: bool) -> Self {
        self.enabled = enable;
        self
    }

    /// Set filtering for temperature measurements.
    pub fn temp(mut self, filter: Filter) -> Self {
        self.iir_t = filter;
        self
    }

    /// Set filtering for pressure measurements.
    pub fn pres(mut self, filter: Filter) -> Self {
        self.iir_p = filter;
        self
    }
}

/// Selection of filter coefficients for temperature and pressure readings.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Filter {
    /// Bypass filtering
    Bypass = 0x00,
    /// Filter coefficient: 1
    Coeff1 = 0x01,
    /// Filter coefficient: 3
    Coeff3 = 0x02,
    /// Filter coefficient: 7
    Coeff7 = 0x03,
    /// Filter coefficient: 15
    Coeff15 = 0x04,
    /// Filter coefficient: 31
    Coeff31 = 0x05,
    /// Filter coefficient: 63
    Coeff63 = 0x06,
    /// Filter coefficient: 127
    Coeff127 = 0x07,
}

impl<I2C, D> Bmp5xx<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    /// Configure IIR low-pass filtering for measurements. This affects all outputs by the sensor, even out-of-range measurement detection.
    pub async fn iir(&mut self, iir: Iir) -> Result<()> {
        // configure coefficients
        self.i2c
            .write(
                self.addr,
                &[DSP_IIR, (iir.iir_p as u8) << 3 | iir.iir_t as u8],
            )
            .await
            .map_err(|_| WriteError)?;
        // enable/disable outputting filtered values
        // overwrites comp_pt_en compensation setting
        self.i2c
            .write(
                self.addr,
                &[DSP_CONFIG, if iir.enabled { 0xFF } else { 0x03 }],
            )
            .await
            .map_err(|_| WriteError)
    }
}
