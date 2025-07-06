/// Create an enum that has a [Vec] wrapper.
macro_rules! enum_vec {
    (
        $(#[$meta:meta])*
        enum $name:ident {
            $(
                $(#[$var_meta:meta])*
                $variant:ident $((
                    $($field_type:ty),* $(,)?
                ))? $(= $value:expr)?,
            )*
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone)]
        pub enum $name {
            $(
                $(#[$var_meta])*
                $variant $((
                    $($field_type),*
                ))? $(= $value)?,
            )*
        }

        pastey::paste! {
            #[derive(Clone)]
            pub struct [<$name s>](Vec<$name>);

            impl std::ops::Deref for [<$name s>] {
                type Target = Vec<$name>;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl std::ops::DerefMut for [<$name s>] {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl std::fmt::Debug for [<$name s>] {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }
        }
    };
}
pub(crate) use enum_vec;
