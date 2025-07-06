use crate::prelude::*;

/// A report type supported by Modrinth.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReportType {
    /// The project is spam or contains spam content.
    Spam,
    /// The project violates copyright laws or contains copyrighted material without permission.
    Copyright,
    /// The project is inappropriate or contains offensive content.
    Inappropriate,
    /// The project is malicious or contains harmful content.
    Malicious,
    /// The project is [cybersquatting](https://en.wikipedia.org/wiki/Cybersquatting) the name.
    #[serde(rename = "name-squatting")]
    NameSquatting,
    /// The project has a poor description.
    #[serde(rename = "poor description")]
    PoorDescription,
    /// The project has invalid metadata, such as missing or incorrect fields.
    #[serde(rename = "invalid metadata")]
    InvalidMetadata,
    /// There is another issue with the project not covered by the other report types.
    Other,
}

/// ### Get a list of report types
///
/// Gets an array of valid report types.
///
/// See the [Modrinth API docs](https://docs.modrinth.com/api/operations/reporttypelist/) for more details.
pub async fn report_types<Auth: AuthState>(
    modrinth: &Modrinth<Auth>,
) -> Result<Vec<ReportType>, ModrinthError> {
    #[derive(Endpoint)]
    #[endpoint(
        method = "GET",
        path = "v2/tag/report_type",
        response = "Vec<ReportType>"
    )]
    struct GetReportTypes;

    Ok(exec!(GetReportTypes, modrinth)?.parse()?)
}
