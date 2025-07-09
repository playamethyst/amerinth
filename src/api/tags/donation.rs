use crate::prelude::*;

endpoint! {
    "GET" "v2/tag/donation_platform" -> "ijson::IValue";

    /// ### Get a list of donation platforms
    ///
    /// Gets an array of donation platforms and information about them.
    ///
    /// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/donationplatformlist/) for more details.
    pub fn donation_platforms() -> Vec<DonationPlatform> {
        |res| match res {
            Ok(res) => {
                let mut platforms = Vec::new();

                if let Some(values) = res.parse()?.as_array() {
                    for value in values {
                        if let Some(obj) = value.as_object() {
                            let data: DonationPlatform = ijson::from_value(&obj.clone().into()).map_err(|source|
                                rustify::errors::ClientError::DataParseError { source: source.into() }
                            )?;
                            platforms.push(data);
                        }
                    }
                }

                Ok(platforms)
            },
            Err(err) => Err(err.into())
        }
    }
}

#[derive(Clone, Debug, strum::Display, EnumString, PartialEq, Eq)]
pub enum DonationPlatform {
    #[strum(serialize = "bmac", to_string = "Buy Me a Coffee")]
    BuyMeACoffee,
    #[strum(serialize = "github", to_string = "GitHub Sponsors")]
    GitHub,
    #[strum(serialize = "ko-fi", to_string = "Ko-fi")]
    Kofi,
    #[strum(serialize = "patreon", to_string = "Patreon")]
    Patreon,
    #[strum(serialize = "paypal", to_string = "PayPal")]
    PayPal,
    #[strum(serialize = "other", to_string = "Other")]
    Other,
    #[strum(serialize = "{short}", to_string = "{name}")]
    Undocumented { short: String, name: String },
}

impl<'de> Deserialize<'de> for DonationPlatform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Entry {
            short: String,
            name: String,
        }

        let e = Entry::deserialize(deserializer)?;
        Ok(e.short.parse().unwrap_or(Self::Undocumented {
            short: e.short,
            name: e.name,
        }))
    }
}
