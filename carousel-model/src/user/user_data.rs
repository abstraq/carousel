use serde::Deserialize;
use time::OffsetDateTime;

/// Information about the user returned from the twitch GET /users endpoint.
///
/// The `type` field in the response is renamed to `role` to avoid a conflict
/// with the `type` keyword.
///
/// The `email` field is an optional field as it is only returned when the
/// application has the `user:read:email` scope.
///
/// See https://dev.twitch.tv/docs/api/reference#get-users for more information.
#[derive(Debug, Deserialize)]
pub struct UserData {
	/// The current contract the user has with Twitch.
	pub broadcaster_type: BroadcasterType,

	/// The user's channel description.
	pub description: String,

	/// The user's display name.
	pub display_name: String,

	/// The user's unique identifier. This should be preferred over the
	/// `login` when possible.
	pub id: String,

	/// The user's login name.
	pub login: String,

	/// The image displayed when the user is offline.
	pub offline_image_url: String,

	/// The user's profile image.
	pub profile_image_url: String,

	/// The user's global role on Twitch.
	#[serde(rename = "type")]
	pub role: UserRole,

	/// The user's email address. This is only returned when the application
	/// has the `user:read:email` scope.
	pub email: Option<String>,

	/// The date the user's account was created.
	#[serde(with = "time::serde::iso8601")]
	pub created_at: OffsetDateTime,
}

/// Represents the different broadcaster types a user can be.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BroadcasterType {
	/// Represents a user who is a Twitch partner.
	Partner,
	/// Represents a user who is a Twitch affiliate.
	Affiliate,
	/// Represents a user who is neither a partner nor an affiliate.
	#[serde(rename = "")]
	None,
}

/// Represents the different global roles a user can have on Twitch.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
	/// Represents a user who is a Twitch staff member.
	Staff,
	/// Represents a user who is a Twitch admin.
	Admin,
	/// Represents a user who is a Twitch global moderator.
	GlobalMod,
	/// Represents a user who has no global role on Twitch.
	#[serde(rename = "")]
	None,
}
