use crate::ModrinthError;
use chrono::{DateTime, Utc};

mod endpoint_vec;
pub(crate) use endpoint_vec::*;

mod enum_vec;
pub(crate) use enum_vec::*;

/// A shortcut for a serializable Date.
pub type Date = DateTime<Utc>;

/// A [Result] wrapper that uses [ModrinthError] as the default error type.
pub type Result<T, E = ModrinthError> = std::result::Result<T, E>;
