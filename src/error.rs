// Copyright (C) 2026 Olivér Pirger
// SPDX-License-Identifier: GPL-3.0-or-later

//! Error types for this crate.
//!
//! All fallible operations return [`Result`] in this crate,
//! which is an alias for [`core::result::Result`] with [`Error`] as the error type.

/// Errors that can occur when using this crate.
#[derive(Debug)]
pub enum Error {
    /// The driver initiated a read operation that has failed.
    ReadError,
    /// The driver initiated a write operation that has failed.
    WriteError,
    /// The sensor's ID was invalid during initialization.
    InvalidId,
    /// The sensor was not ready during initialization.
    NotReady,
    /// The operation took too long to complete.
    Timeout,
    /// The set output data rate is incompatible with the oversampling settings.
    OdrInvalid,
}

/// A specialised [`Result`](core::result::Result) type for this crate.
pub type Result<T> = core::result::Result<T, Error>;
