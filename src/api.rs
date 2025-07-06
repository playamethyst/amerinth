pub mod projects {
    #[cfg(any(feature = "projects", feature = "tags"))]
    mod project_type;
    #[cfg(any(feature = "projects", feature = "tags"))]
    pub use project_type::*;
}

#[cfg(feature = "users")]
pub mod users;

#[cfg(feature = "tags")]
pub mod tags;

#[cfg(feature = "misc")]
pub mod misc {
    mod forge;
    pub use forge::*;

    mod statistics;
    pub use statistics::*;
}
