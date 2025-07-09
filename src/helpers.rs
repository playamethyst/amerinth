use_all!(pub(crate) debug_fmt);
use_all!(pub(crate) endpoint);
use_all!(pub(crate) other_enum);
use_all!(pub(crate) vec_enum);

/// A macro to use all items from a module.
macro_rules! use_all {
    ($vis:vis $mod:ident) => {
        mod $mod;
        #[allow(unused_imports)]
        $vis use $mod::*;
    };
}
pub(crate) use use_all;
