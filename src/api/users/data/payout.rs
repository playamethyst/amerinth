use serde::Deserialize;

// see: https://github.com/modrinth/code/blob/bff26af4655587ebb619dfde477356cc6ca5bd4b/apps/labrinth/src/models/v3/users.rs#L58
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct UserPayout {
    /// The payout balance available for the user to withdraw (note, you cannot modify this in a `PATCH` request)
    pub balance: f64,
    // ! these are undocumented in v2
    /// The user's PayPal email address, if they have one
    pub paypal_address: Option<String>,
    /// The user's Venmo phone number, if they have one
    pub paypal_country: Option<String>,
    /// The user's Venmo handle, if they have one
    pub venmo_handle: Option<String>,
}
