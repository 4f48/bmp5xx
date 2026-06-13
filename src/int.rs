// Copyright (C) 2026 Olivér Pirger
// SPDX-License-Identifier: GPL-3.0-or-later

//! Configuration options for interrupts.

use embedded_hal_async::{delay::DelayNs, i2c::I2c};

use crate::{
    Bmp5xx,
    error::{Error::WriteError, Result},
    register::{INT_CONFIG, INT_SOURCE},
};

/// Sensor interrupts configuration.
pub struct Interrupt {
    enabled: bool,
    source: IntSource,
    mode: IntMode,
    polarity: IntPolarity,
    int_pin: IntPin,
}

impl Default for Interrupt {
    fn default() -> Self {
        Self {
            enabled: false,
            source: IntSource::default(),
            mode: IntMode::Pulsed,
            polarity: IntPolarity::ActiveLow,
            int_pin: IntPin::PushPull,
        }
    }
}

impl Interrupt {
    /// Enable or disable interrupts.
    pub fn enable(mut self, enable: bool) -> Self {
        self.enabled = enable;
        self
    }

    /// Sets interrupt mode.
    pub fn mode(mut self, mode: IntMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets interrupt polarity.
    pub fn polarity(mut self, polarity: IntPolarity) -> Self {
        self.polarity = polarity;
        self
    }

    /// Sets interrupt pin behaviour.
    pub fn pin(mut self, pin: IntPin) -> Self {
        self.int_pin = pin;
        self
    }
}

/// Decide which events trigger an interrupt.
pub struct IntSource {
    data_ready: bool,
    fifo_full: bool,
    fifo_threshold: bool,
    pressure_oor: bool,
}

impl Default for IntSource {
    fn default() -> Self {
        Self {
            data_ready: true,
            fifo_full: false,
            fifo_threshold: false,
            pressure_oor: false,
        }
    }
}

impl IntSource {
    /// Whether new data triggers an interrupt.
    pub fn data_ready(mut self, data_ready: bool) -> Self {
        self.data_ready = data_ready;
        self
    }

    /// Whether the FIFO filling up triggers an interrupt.
    pub fn fifo_full(mut self, fifo_full: bool) -> Self {
        self.fifo_full = fifo_full;
        self
    }

    /// Whether the FIFO reaching a threshold triggers an interrupt.
    pub fn fifo_threshold(mut self, fifo_threshold: bool) -> Self {
        self.fifo_threshold = fifo_threshold;
        self
    }

    /// Whether an out-of-range pressure reading triggers an interrupt.
    pub fn pressure_oor(mut self, pressure_oor: bool) -> Self {
        self.pressure_oor = pressure_oor;
        self
    }
}

/// Whether the interrupt pin is pulsed or held until cleared.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum IntMode {
    /// Interrupt is asserted for a short duration.
    Pulsed = 0x00,
    /// Interrupt is held until cleared.
    Latched = 0x01,
}

/// Whether the interrupt pin is active low or high.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum IntPolarity {
    /// Interrupt is asserted by pulling the pin low.
    ActiveLow = 0x00,
    /// Interrupt is asserted by pulling the pin high.
    ActiveHigh = 0x01,
}

/// Whether the interrupt pin is push-pull or open-drain.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum IntPin {
    /// Pin is driven both high and low by the sensor.
    PushPull = 0x00,
    /// Pin is only pulled low. This mode requires an external pull-up.
    OpenDrain = 0x01,
}

impl<I2C, D> Bmp5xx<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    /// Configures interrupts for the sensor.
    pub async fn int(&mut self, conf: Interrupt) -> Result<()> {
        // set interrupt sources
        let source = conf.source;
        self.i2c
            .write(
                self.addr,
                &[
                    INT_SOURCE,
                    (source.pressure_oor as u8) << 3
                        | (source.fifo_threshold as u8) << 2
                        | (source.fifo_full as u8) << 1
                        | source.data_ready as u8,
                ],
            )
            .await
            .map_err(|_| WriteError)?;

        // configure interrupts
        self.i2c
            .write(
                self.addr,
                &[
                    INT_CONFIG,
                    (conf.enabled as u8) << 3
                        | (conf.int_pin as u8) << 2
                        | (conf.polarity as u8) << 1
                        | conf.mode as u8,
                ],
            )
            .await
            .map_err(|_| WriteError)
    }
}
