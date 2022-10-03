use serde::Deserialize;
use time::OffsetDateTime;

/// Information about a follow relationship between two users returned
/// from the GET /users/follows endpoint.
///
/// See https://dev.twitch.tv/docs/api/reference#get-users-follows for
/// more information.
#[derive(Debug, Deserialize)]
pub struct UserFollowData {
	/// The id of the user being followed by the user specified by `from_id`.
	pub to_id: String,

	/// The login of the user being followed by the user specified by `from_id`.
	pub to_login: String,

	/// The display name of the user being followed by the user specified by `from_id`.
	pub to_name: String,

	/// The id of the user following the user specified by `to_id`.
	pub from_id: String,

	/// The login of the user following the user specified by `to_id`.
	pub from_login: String,

	/// The display name of the user following the user specified by `to_id`.
	pub from_name: String,

	/// The date the user started following the user specified by `to_id`.
	#[serde(with = "time::serde::iso8601")]
	pub followed_at: OffsetDateTime,
}
