use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PayoutWallet {
    Paypal,
    Venmo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PayoutWalletType {
    Email,
    Phone,
    UserHandle,
}

#[derive(Debug, Deserialize)]
pub struct UserPayout {
    /// The payout balance available for the user to withdraw (note, you cannot modify this in a `PATCH` request)
    pub balance: f32,
    /// The wallet that the user has selected
    #[serde(rename = "payout_wallet")]
    pub wallet: Option<PayoutWallet>,
    /// The type of the user’s wallet
    #[serde(rename = "payout_wallet_type")]
    pub wallet_type: Option<PayoutWalletType>,
    /// The user’s payout address
    #[serde(rename = "payout_address")]
    pub address: Option<String>,
}
