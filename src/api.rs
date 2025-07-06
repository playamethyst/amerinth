pub mod projects {
    use crate::helpers::use_all;

    #[cfg(any(feature = "projects", feature = "tags"))]
    use_all!(pub project_type);
}

#[cfg(feature = "users")]
pub mod users;

/// Tags are common and reusable lists of metadata types such as categories or versions.
#[cfg(feature = "tags")]
pub mod tags;

/// Miscellaneous endpoints.
#[cfg(feature = "misc")]
pub mod misc {
    use crate::helpers::use_all;

    use_all!(pub forge);
    use_all!(pub statistics);
}
