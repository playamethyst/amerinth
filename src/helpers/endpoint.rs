use std::fmt::Debug;

/// Create an endpoint.
macro_rules! endpoint {
    (@code $res:expr;) => {
        match $res {
            Ok(res) => Ok(res.parse()?),
            Err(err) => Err(err.into()),
        }
    };
    (@code $res:expr; $($code:tt)*) => {
        $($code)*
    };
    // build the body of an endpoint function
    (@body
        $modrinth:expr; $($res:ident;)? $response:literal;
        code: $([$($code:tt)*])?
        $method:literal, $path:literal: $(
            [$(
                $(#[$field_meta:meta])*
                $end_arg:ident: $end_ty:ty [$($end_val:tt)*]
            ),* $(,)?]
        )?
    ) => {
        #[allow(redundant_semicolons)]
        {
            #[derive(rustify_derive::Endpoint)]
            #[endpoint(method = $method, path = $path, response = $response)]
            struct Request $({
                $(
                    $(#[$field_meta])*
                    $end_arg: $end_ty
                ),*
            })?;
            let middleware = $crate::client::AuthMiddleware($modrinth);
            let endpoint = Request $({$($end_arg: $($end_val)*),*})?
                .with_middleware(&middleware);
            #[cfg(not(feature = "blocking"))]
            let res = endpoint.exec(&$modrinth.client).await;
            #[cfg(feature = "blocking")]
            let res = endpoint.exec_block(&$modrinth.client);
            $(let $res = res;)?

            endpoint!(@code res; $($($code)*)?)
        }
    };
    // the main macro entry point
    (
        $method:literal $path:literal $({
            $(
                $(#[$field_meta:meta])*
                $end_arg:ident: $end_ty:ty [$($end_val:tt)*]
            ),* $(,)?
        })? -> $response:literal $([$auth:ident])?;
        $(#[$fn_meta:meta])*
        $vis:vis fn $name:ident$(<
            $($lifetime:lifetime),* $(,)?
            $($generic:ident $(: [$($bounds:tt)+])?),*
        >)?(
            $($arg:ident: $arg_ty:ty),*
        ) -> $return_ty:ty $({
            |$res:ident| $($code:tt)*
        })?
    ) => {
        $(#[$fn_meta])*
        #[cfg(not(feature = "blocking"))]
        $vis async fn $name<
            $($($lifetime,)*)?
            Auth: $crate::client::AuthState $(+ $crate::client::$auth)?,
            $($(
                $generic $(: $($bounds)+)?
            ),*)?
        >(
            modrinth: &$crate::Modrinth<Auth>,
            $($arg: $arg_ty),*
        ) -> Result<$return_ty, $crate::ModrinthError> {
            endpoint!(@body
                modrinth; $($res;)? $response;
                code: $([$($code)*])?
                $method, $path: $(
                    [$(
                        $(#[$field_meta])*
                        $end_arg: $end_ty [$($end_val)*]
                    ),*]
                )?
            )
        }

        $(#[$fn_meta])*
        #[cfg(feature = "blocking")]
        $vis fn $name<
            $($($lifetime,)*)?
            Auth: $crate::client::AuthState $(+ $crate::client::$auth)?,
            $($(
                $generic $(: Send + $($bounds)+)?
            ),*)?
        >(
            modrinth: &$crate::Modrinth<Auth>,
            $($arg: $arg_ty),*
        ) -> Result<$return_ty, $crate::ModrinthError> {
            endpoint!(@body
                modrinth; $($res;)? $response;
                code: $([$($code)*])?
                $method, $path: $(
                    [$(
                        $(#[$field_meta])*
                        $end_arg: $end_ty [$($end_val)*]
                    ),*]
                )?
            )
        }
    };
}
pub(crate) use endpoint;

/// A [Vec] wrapper that (de)serializes a list of items in a way the Modrinth API expects.
pub(crate) struct EndpointVec<T: Debug>(pub(crate) Vec<T>);

impl<T: Debug> std::ops::Deref for EndpointVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Debug> std::ops::DerefMut for EndpointVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Debug> serde::Serialize for EndpointVec<T> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:?}", self.0))
    }
}
