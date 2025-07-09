/// Create an enum that has a [Vec] wrapper.
macro_rules! vec_enum {
    (
        $(#[$enum_meta:meta])*
        enum $enum:ident {
            $(
                $(#[$var_meta:meta])*
                $variant:ident $((
                    $($field_type:ty),* $(,)?
                ))? $(= $value:expr)?,
            )*
        }
        $(
            vec_meta(
                $(#[$vec_meta:meta])*
            )
        )?
    ) => {
        $(#[$enum_meta])*
        #[derive(Debug, Clone)]
        pub enum $enum {
            $(
                $(#[$var_meta])*
                $variant $((
                    $($field_type),*
                ))? $(= $value)?,
            )*
        }

        pastey::paste! {
            $($(#[$vec_meta])*)?
            #[derive(Clone)]
            pub struct [<$enum s>](Vec<$enum>);

            impl std::ops::Deref for [<$enum s>] {
                type Target = Vec<$enum>;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl std::ops::DerefMut for [<$enum s>] {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl std::fmt::Debug for [<$enum s>] {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }
        }
    };
}
pub(crate) use vec_enum;
