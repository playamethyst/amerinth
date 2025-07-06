use crate::prelude::*;

/// A report type supported by Modrinth.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReportType {
    Spam,
    Copyright,
    Inappropriate,
    Malicious,
    #[serde(rename = "name-squatting")]
    NameSquatting,
    #[serde(rename = "poor description")]
    PoorDescription,
    #[serde(rename = "invalid metadata")]
    InvalidMetadata,
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
    struct Request;

    Ok(exec!(Request, modrinth)?.parse()?)
}
