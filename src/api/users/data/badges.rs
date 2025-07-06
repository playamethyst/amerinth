use crate::helpers::enum_vec;
use strum::{EnumIter, IntoEnumIterator};

enum_vec! {
    /// A badge that a user can have on Modrinth.
    ///
    /// Badges are awarded for various contributions to the Modrinth community.
    /// These are currently unused and undisplayed, and as such are subject to change.
    #[repr(u8)]
    #[derive(Copy, PartialEq, EnumIter, Eq)]
    enum Badge {
        // unused 0b0000_0000;
        EarlyModpackAdopter = 0b0000_0010,
        EarlyRespackAdopter = 0b0000_0100,
        EarlyPluginAdopter = 0b0000_1000,
        AlphaTester = 0b0001_0000,
        Contributor = 0b0010_0000,
        Translator = 0b0100_0000,
        // unused 0b0000_0000;
    }
}

impl<'de> serde::Deserialize<'de> for Badges {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bitfield = u8::deserialize(deserializer)?;
        let mut badges = Vec::with_capacity(6);

        for badge in Badge::iter() {
            if bitfield & (badge as u8) != 0 {
                badges.push(badge);
            }
        }

        Ok(Self(badges))
    }
}
