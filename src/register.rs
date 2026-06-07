// Copyright (C) 2026 Olivér Pirger
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(dead_code)]

pub(crate) const CHIP_ID: u8 = 0x01;
pub(crate) const REV_ID: u8 = 0x02;

pub(crate) const CHIP_STATUS: u8 = 0x11;

pub(crate) const DRIVE_CONFIG: u8 = 0x13;
pub(crate) const INT_CONFIG: u8 = 0x14;
pub(crate) const INT_SOURCE: u8 = 0x15;
pub(crate) const FIFO_CONFIG: u8 = 0x16;
pub(crate) const FIFO_COUNT: u8 = 0x17;
pub(crate) const FIFO_SEL: u8 = 0x18;

pub(crate) const RESERVED_REG_0: u8 = 0x1C;
pub(crate) const TEMP_DATA_XLSB: u8 = 0x1D;
pub(crate) const TEMP_DATA_LSB: u8 = 0x1E;
pub(crate) const TEMP_DATA_MSB: u8 = 0x1F;
pub(crate) const PRESS_DATA_XLSB: u8 = 0x20;
pub(crate) const PRESS_DATA_LSB: u8 = 0x21;
pub(crate) const PRESS_DATA_MSB: u8 = 0x22;
pub(crate) const RESERVED_REG1: u8 = 0x23;
pub(crate) const RESERVED_REG2: u8 = 0x24;
pub(crate) const RESERVED_REG3: u8 = 0x25;
pub(crate) const RESERVED_REG4: u8 = 0x26;
pub(crate) const INT_STATUS: u8 = 0x27;
pub(crate) const STATUS: u8 = 0x28;
pub(crate) const FIFO_DATA: u8 = 0x29;

pub(crate) const NVM_ADDR: u8 = 0x2B;
pub(crate) const NVM_DATA_LSB: u8 = 0x2C;
pub(crate) const NVM_DATA_MSB: u8 = 0x2D;

pub(crate) const DSP_CONFIG: u8 = 0x30;
pub(crate) const DSP_IIR: u8 = 0x31;
pub(crate) const OOR_THR_P_LSB: u8 = 0x32;
pub(crate) const OOR_THR_P_MSB: u8 = 0x33;
pub(crate) const OOR_RANGE: u8 = 0x34;
pub(crate) const OOR_CONFIG: u8 = 0x35;
pub(crate) const OSR_CONFIG: u8 = 0x36;
pub(crate) const ODR_CONFIG: u8 = 0x37;
pub(crate) const OSR_EFF: u8 = 0x38;

pub(crate) const CMD: u8 = 0x7E;
