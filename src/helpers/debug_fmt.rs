use std::fmt::Debug;

/// A wrapper around a type that implements [Debug] to format it as a [str].
///
/// This formats [Vec<T>] in a way that the Modrinth API expects.
pub(crate) struct DebugFmt<T: Debug>(pub(crate) T);

impl<T: Debug> std::ops::Deref for DebugFmt<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Debug> std::ops::DerefMut for DebugFmt<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Debug> serde::Serialize for DebugFmt<T> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:?}", self.0))
    }
}

impl<T: Debug> From<T> for DebugFmt<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
