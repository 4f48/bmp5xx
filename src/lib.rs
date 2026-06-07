// Copyright (C) 2026 Olivér Pirger
// SPDX-License-Identifier: GPL-3.0-or-later

#![no_std]

pub mod error;
pub(crate) mod register;

use embedded_hal_async::i2c::I2c;

use crate::error::{Error, Result};

pub struct Bmp58x<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C> Bmp58x<I2C>
where
    I2C: I2c,
{
    pub async fn new(i2c: I2C, addr: u8) -> Result<Self> {
        let mut bmp58x = Self { i2c, addr };
        let mut id_buf = [0u8; 1];
        bmp58x
            .i2c
            .write_read(addr, &[0x01], &mut id_buf)
            .await
            .map_err(|_| Error::ReadError)?;
        match id_buf[0] {
            0x50 | 0x51 => (),
            _ => return Err(Error::InvalidId),
        }

        Ok(bmp58x)
    }

    pub async fn reset(&mut self) -> Result<()> {
        self.i2c
            .write(self.addr, &[0x7E, 0xB6])
            .await
            .map_err(|_| Error::WriteError)?;

        Ok(())
    }
}
