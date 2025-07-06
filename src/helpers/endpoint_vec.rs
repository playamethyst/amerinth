use std::fmt::Debug;

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
