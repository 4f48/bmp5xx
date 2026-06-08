// Copyright (C) 2026 Olivér Pirger
// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug)]
pub enum Error {
    ReadError,
    WriteError,
    InvalidId,
    NotReady,
    Timeout,
}

pub type Result<T> = core::result::Result<T, Error>;
