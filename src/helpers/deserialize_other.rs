/// Deserialize an enum with a default "Other" variant for unknown cases.
/// Requires the enum to implement [strum::EnumString].
macro_rules! deserialize_other {
    ($enum:ident) => {
        impl<'de> serde::Deserialize<'de> for $enum {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                Ok(s.parse().unwrap_or(Self::Other(s)))
            }
        }
    };
}
pub(crate) use deserialize_other;
