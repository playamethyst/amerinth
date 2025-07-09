/// Create a tag endpoint that retuns a
/// [HashMap](std::collections::HashMap) of items.
macro_rules! tag {
    (
        $endpoint:literal;

        $(#[$fn_meta:meta])*
        fn $fn:ident() -> $tag:ident;

        $(#[$struct_meta:meta])*
        [$key:literal: $key_ty:ty] -> {
            $(
                $(#[$field_meta:meta])*
                $field:ident: $field_ty:ty
            ),*
        }
    ) => {
        $(#[$struct_meta])*
        #[derive(Debug, Clone, serde::Deserialize)]
        pub struct $tag {
            $(
                $(#[$field_meta])*
                pub $field: $field_ty
            ),*
        }

        $crate::helpers::endpoint! {
            "GET" $endpoint -> "ijson::IValue";
            $(#[$fn_meta])*
            fn $fn() -> std::collections::HashMap<$key_ty, $tag> {
                |res| match res {
                    Ok(res) => {
                        let mut map = std::collections::HashMap::new();

                        if let Some(values) = res.parse()?.as_array() {
                            for value in values {
                                if let Some(obj) = value.as_object() {
                                    if let Some(key_value) = obj.get($key) {
                                        if let Some(key_str) = key_value.as_string() {
                                            let data: $tag = ijson::from_value(value).map_err(|source|
                                                rustify::errors::ClientError::DataParseError { source: source.into() }
                                            )?;
                                            println!("{:?}", key_str);
                                            map.insert(key_str.to_string().parse()?, data);
                                        }
                                    }
                                }
                            }
                        }

                        Ok(map)
                    }
                    Err(err) => Err(err.into())
                }
            }
        }
    };
}
pub(crate) use tag;

/// Create a tag endpoint that returns a [Vec] of items.
macro_rules! tag_vec {
    (
        $(#[$fn_meta:meta])*
        $fn:ident, $tag:ident ($vec_tag:literal), $endpoint:literal;
    ) => {
        $crate::helpers::endpoint! {
            "GET" $endpoint -> $vec_tag;

            $(#[$fn_meta])*
            fn $fn() -> Vec<$tag>
        }
    };
}
pub(crate) use tag_vec;
