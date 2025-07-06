use std::fmt::Debug;

/// Create the functions for an endpoint.
macro_rules! endpoint_fn {
    (
        $endpoint:ident {
            $($end_arg:ident: $($end_val:tt)*),*
        };
        $(#[$meta:meta])*
        fn $name:ident(
            $($arg:ident: $arg_ty:ty),*
        ) -> $return_ty:ty {
            $($code:tt)*
        }
    ) => {
        pastey::paste! {
            $(#[$meta])*
            #[cfg(not(feature = "blocking"))]
            pub async fn $name<Auth: $crate::client::AuthState>(
                modrinth: &$crate::Modrinth<Auth>,
                $($arg: $arg_ty),*
            ) -> Result<$return_ty, $crate::ModrinthError> {
                $(let $arg = $arg.into();)*
                let res = $endpoint {
                    $($end_arg: $($end_val)*),*
                }
                    .with_middleware(&$crate::client::AuthMiddleware(modrinth))
                    .exec(&modrinth.client)
                    .await;
                match res {
                    $($code)*
                }
            }

            $(#[$meta])*
            #[cfg(feature = "blocking")]
            pub fn $name<Auth: $crate::client::AuthState>(
                modrinth: &$crate::Modrinth<Auth>,
                $($arg: $arg_ty),*
            ) -> Result<$return_ty, $crate::ModrinthError> {
                $(let $arg = $arg.into();)*
                let res = $endpoint {
                    $($end_arg: $($end_val)*),*
                }
                    .with_middleware(&$crate::client::AuthMiddleware(modrinth))
                    .exec_block(&modrinth.client);
                match res {
                    $($code)*
                }
            }
        }
    };
}
pub(crate) use endpoint_fn;

/// A [Vec] wrapper that (de)serializes a list of items in a way the Modrinth API expects.
pub(crate) struct EndpointVec<T: Debug>(Vec<T>);

impl<T: Debug> serde::Serialize for EndpointVec<T> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:?}", self.0))
    }
}

impl<T: Debug> FromIterator<T> for EndpointVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        EndpointVec(iter.into_iter().collect())
    }
}
