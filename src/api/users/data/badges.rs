#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Badges {
    // unused 0b0000_0000;
    EarlyModpackAdopter = 0b0000_0010,
    EarlyRespackAdopter = 0b0000_0100,
    EarlyPluginAdopter = 0b0000_1000,
    AlphaTester = 0b0001_0000,
    Contributor = 0b0010_0000,
    Translator = 0b0100_0000,
    // unused 0b0000_0000;
}

const ALL_BADGES: [Badges; 6] = [
    Badges::EarlyModpackAdopter,
    Badges::EarlyRespackAdopter,
    Badges::EarlyPluginAdopter,
    Badges::AlphaTester,
    Badges::Contributor,
    Badges::Translator,
];

pub struct BadgeList(Vec<Badges>);

impl BadgeList {
    /// Get the innner [Vec] of [Badges].
    pub fn vec(&self) -> &Vec<Badges> {
        &self.0
    }
}

impl std::fmt::Debug for BadgeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'de> serde::Deserialize<'de> for BadgeList {
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
