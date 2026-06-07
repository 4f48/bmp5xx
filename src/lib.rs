// Copyright (C) 2026 Olivér Pirger
// SPDX-License-Identifier: GPL-3.0-or-later

#![no_std]

pub mod error;
pub(crate) mod register;

use embedded_hal_async::{delay::DelayNs, i2c::I2c};

use crate::{
    error::{Error, Result},
    register::{CHIP_ID, CMD, STATUS},
};

pub struct Bmp58x<I2C, D> {
    i2c: I2C,
    delay: D,
    addr: u8,
}

impl<I2C, D> Bmp58x<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    pub async fn new(i2c: I2C, delay: D, addr: u8) -> Result<Self> {
        let mut bmp58x = Self { i2c, delay, addr };

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

        Ok(bmp58x)
    }

    pub async fn reset(&mut self) -> Result<()> {
        self.i2c
            .write(self.addr, &[CMD, 0xB6])
            .await
            .map_err(|_| Error::WriteError)?;

        let mut timeout = 0;
        let mut status_buf = [0u8; 1];
        loop {
            #[allow(clippy::collapsible_if)]
            if self
                .i2c
                .write_read(self.addr, &[STATUS], &mut status_buf)
                .await
                .is_ok()
            {
                if (status_buf[0] & (1 << 1)) != 0 {
                    break;
                }
            };

            if timeout >= 100 {
                return Err(Error::Timeout);
            }
            timeout += 1;

            self.delay.delay_ms(2).await;
        }

        Ok(())
    }
}
