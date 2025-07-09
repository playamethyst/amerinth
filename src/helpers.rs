mod endpoint;
pub(crate) use endpoint::*;

mod vec_enum;
pub(crate) use vec_enum::*;
mod deserialize_other;
pub(crate) use deserialize_other::*;

/// A macro to use all items from a module.
macro_rules! use_all {
    ($vis:vis $mod:ident) => {
        mod $mod;
        #[allow(unused_imports)]
        $vis use $mod::*;
    };
}
pub(crate) use use_all;
