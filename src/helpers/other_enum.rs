/// Create an enum with a variant that can hold any non-variant.
macro_rules! other_enum {
    (
        $(#[$enum_meta:meta])*
        $vis:vis enum $enum:ident {
            $(
                $(#[$var_meta:meta])*
                $variant:ident
            ),* $(,)?
        }

        $(#[$other_meta:meta])*
        $other:ident($other_ty:ty)
    ) => {
        #[derive(Debug, Clone, strum::EnumString)]
        $(#[$enum_meta])*
        $vis enum $enum {
            $(
                $(#[$var_meta])*
                $variant
            ),*,
            $(#[$other_meta])*
            #[strum(disabled)]
            $other($other_ty)
        }

        impl<'de> serde::Deserialize<'de> for $enum {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let v = <$other_ty>::deserialize(deserializer)?;
                Ok(v.parse().unwrap_or(Self::$other(v)))
            }
        }
    };
}
pub(crate) use other_enum;
