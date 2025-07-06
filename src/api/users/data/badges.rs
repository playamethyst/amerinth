use crate::helpers::enum_vec;

const ALL_BADGES: [Badge; 6] = [
    Badge::EarlyModpackAdopter,
    Badge::EarlyRespackAdopter,
    Badge::EarlyPluginAdopter,
    Badge::AlphaTester,
    Badge::Contributor,
    Badge::Translator,
];

enum_vec! {
    /// A Modrinth user badge.
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
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
    vec_derive(Clone)
}

impl<'de> serde::Deserialize<'de> for Badges {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bitfield = u8::deserialize(deserializer)?;
        let mut badges = Vec::with_capacity(6);

        for badge in ALL_BADGES {
            if bitfield & (badge as u8) != 0 {
                badges.push(badge);
            }
        }

        Ok(Self(badges))
    }
}
