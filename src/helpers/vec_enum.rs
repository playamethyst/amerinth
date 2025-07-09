/// Create a [Vec] wrapper for an enum.
macro_rules! vec_enum {
    (
        $vis:vis $enum:ident;
        $(vec($(#[$vec_meta:meta])*))?
    ) => {
        pastey::paste! {
            $($(#[$vec_meta])*)?
            #[derive(Clone)]
            $vis struct [<$enum s>](Vec<$enum>);

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
