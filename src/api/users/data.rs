use chrono::{DateTime, Utc};
use serde::Deserialize;

mod badges;
pub use badges::*;

mod payout;
pub use payout::*;

#[derive(Debug, Deserialize)]
pub struct User {
    /// The user's username
    pub username: String,
    /// The user's display name
    pub name: Option<String>,
    /// The user's email (only displayed if requesting your own account).
    /// Requires `USER_READ_EMAIL` PAT scope.
    pub email: Option<String>,
    /// A description of the user
    pub bio: Option<String>,
    /// Various data relating to the user's payouts status (you can only see your own)
    pub payout_data: UserPayout,
    /// The user’s ID
    pub id: String,
    /// The user’s avatar url
    pub avatar_url: String,
    /// The time at which the user was created
    pub created: DateTime<Utc>,
    /// The user’s role
    pub role: UserRole,
    /// Any badges applicable to this user.
    /// These are currently unused and undisplayed, and as such are subject to change.
    pub badges: BadgeList,
    /// A list of authentication providers you have signed up for (only displayed if requesting your own account)
    pub auth_providers: Vec<AuthProviders>,
    /// Whether your email is verified (only displayed if requesting your own account)
    pub email_verified: Option<bool>,
    /// Whether you have a password associated with your account (only displayed if requesting your own account)
    pub has_password: Option<bool>,
    /// Whether you have TOTP two-factor authentication connected to your account (only displayed if requesting your own account)
    pub has_totp: Option<bool>,
    // ! don't even include github_id because it has been deprecated
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Moderator,
    Developer,
}

// see: https://github.com/modrinth/code/blob/bff26af4655587ebb619dfde477356cc6ca5bd4b/apps/labrinth/src/auth/mod.rs#L113
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AuthProviders {
    GitHub,
    Discord,
    Microsoft,
    GitLab,
    Google,
    Steam,
    PayPal,
}
