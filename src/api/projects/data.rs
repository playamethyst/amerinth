use crate::helpers::use_all;

#[cfg(any(feature = "projects", feature = "tags"))]
use_all!(pub ptype);

#[cfg(any(feature = "projects", feature = "tags"))]
#[derive(Clone, Debug, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectSide {
    /// The project is required to function.
    Required,
    /// The project is not required, but may enhance the experience.
    Optional,
    /// The project is unsupported, meaning it is not recommended for use.
    Unsupported,
    /// It is unknown whether the project will work.
    Unknown,
}
