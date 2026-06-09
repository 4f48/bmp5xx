// Copyright (C) 2026 Olivér Pirger
// SPDX-License-Identifier: GPL-3.0-or-later

#![no_std]

pub mod error;
pub(crate) mod register;

use byteorder::{ByteOrder, LittleEndian};
use embedded_hal_async::{delay::DelayNs, i2c::I2c};

use crate::{
    error::{
        Error::{self, WriteError},
        Result,
    },
    register::{
        CHIP_ID, CMD, INT_STATUS, ODR_CONFIG, OSR_CONFIG, PRESS_DATA_XLSB, STATUS, TEMP_DATA_XLSB,
    },
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Oversampling {
    X1 = 0x00,
    X2 = 0x01,
    X4 = 0x02,
    X8 = 0x03,
    X16 = 0x04,
    X32 = 0x05,
    X64 = 0x06,
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

pub struct Bmp5xx<I2C, D> {
    i2c: I2C,
    delay: D,
    addr: u8,
    osr_t: Oversampling,
    osr_p: Oversampling,
}

impl<I2C, D> Bmp5xx<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    pub async fn new(i2c: I2C, delay: D, addr: u8) -> Result<Self> {
        let mut bmp58x = Self {
            i2c,
            delay,
            addr,
            osr_t: Oversampling::X1,
            osr_p: Oversampling::X1,
        };

        let mut id_buf = [0u8; 1];
        bmp58x
            .i2c
            .write_read(addr, &[CHIP_ID], &mut id_buf)
            .await
            .map_err(|_| Error::ReadError)?;
        match id_buf[0] {
            0x50 | 0x51 => (),
            _ => return Err(Error::InvalidId),
        }

        bmp58x.reset().await?;

        let mut status_buf = [0u8; 1];
        bmp58x
            .i2c
            .write_read(bmp58x.addr, &[STATUS], &mut status_buf)
            .await
            .map_err(|_| Error::ReadError)?;
        if status_buf[0] & 0x02 == 0 || status_buf[0] & 0x04 != 0 {
            return Err(Error::NotReady);
        }

        let mut int_status_buf = [0u8; 1];
        bmp58x
            .i2c
            .write_read(bmp58x.addr, &[INT_STATUS], &mut int_status_buf)
            .await
            .map_err(|_| Error::ReadError)?;
        if int_status_buf[0] & (1 << 4) == 0 {
            return Err(Error::NotReady);
        }

        // switch to standby & disable deep standby
        bmp58x
            .i2c
            .write(bmp58x.addr, &[ODR_CONFIG, 0x80])
            .await
            .map_err(|_| Error::WriteError)?;

        Ok(bmp58x)
    }

    pub async fn reset(&mut self) -> Result<()> {
        // trigger reset
        self.i2c
            .write(self.addr, &[CMD, 0xB6])
            .await
            .map_err(|_| Error::WriteError)?;

        // t_soft_res
        self.delay.delay_ms(2).await;
        Ok(())
    }

    pub fn temperature_oversampling(&mut self, oversampling: Oversampling) {
        self.osr_t = oversampling;
    }

    pub fn pressure_oversampling(&mut self, oversampling: Oversampling) {
        self.osr_p = oversampling;
    }

    pub async fn temperature(&mut self) -> Result<f32> {
        // switch to standby & disable deep standby
        self.i2c
            .write(self.addr, &[ODR_CONFIG, 0x80])
            .await
            .map_err(|_| Error::WriteError)?;

        // configure oversampling
        self.i2c
            .write(self.addr, &[OSR_CONFIG, self.osr_t as u8])
            .await
            .map_err(|_| Error::WriteError)?;

        // switch to forced & disable deep standby
        self.i2c
            .write(self.addr, &[ODR_CONFIG, 0x82])
            .await
            .map_err(|_| WriteError)?;

        // t_conv_t
        self.delay
            .delay_us((self.osr_t.t_conv_t() * 1050.0) as u32)
            .await;

        let mut temp_buf = [0u8; 3];
        self.i2c
            .write_read(self.addr, &[TEMP_DATA_XLSB], &mut temp_buf)
            .await
            .map_err(|_| Error::ReadError)?;
        let raw_temp = LittleEndian::read_i24(&temp_buf);
        Ok(raw_temp as f32 / 65536.0)
    }

    pub async fn pressure(&mut self) -> Result<f32> {
        // switch to standby & disable deep standby
        self.i2c
            .write(self.addr, &[ODR_CONFIG, 0x80])
            .await
            .map_err(|_| Error::WriteError)?;

        // configure oversampling, enable pressure measurement
        self.i2c
            .write(
                self.addr,
                &[
                    OSR_CONFIG,
                    (1 << 6) | ((self.osr_p as u8) << 3) | self.osr_t as u8,
                ],
            )
            .await
            .map_err(|_| Error::WriteError)?;

        // switch to forced & disable deep standby
        self.i2c
            .write(self.addr, &[ODR_CONFIG, 0x82])
            .await
            .map_err(|_| WriteError)?;

        // t_conv_p
        self.delay
            .delay_us((self.osr_p.t_conv_p() * 1050.0) as u32)
            .await;

        let mut press_buf = [0u8; 3];
        self.i2c
            .write_read(self.addr, &[PRESS_DATA_XLSB], &mut press_buf)
            .await
            .map_err(|_| Error::ReadError)?;
        let raw_press = LittleEndian::read_i24(&press_buf);
        Ok(raw_press as f32 / 64.0 / 100.0)
    }
}
