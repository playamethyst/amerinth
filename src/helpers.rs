mod endpoint;
pub(crate) use endpoint::*;

mod enum_vec;
pub(crate) use enum_vec::*;

/// A macro to use all items from a module.
macro_rules! use_all {
    ($vis:vis $mod:ident) => {
        mod $mod;
        $vis use $mod::*;
    };
}
pub(crate) use use_all;

/// Deserialize an enum with a default "Other" variant for unknown cases.
macro_rules! deserialize_other {
    ($enum:ident) => {
        impl<'de> serde::Deserialize<'de> for $enum {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                Ok(s.parse().unwrap_or($enum::Other(s)))
            }
        }
    };
}
pub(crate) use deserialize_other;
