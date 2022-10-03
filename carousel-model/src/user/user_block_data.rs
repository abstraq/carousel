use serde::Deserialize;

/// Information about a user that has been blocked by the authenticated user.
/// Returned from the GET /users/blocks endpoint.
///
/// See https://dev.twitch.tv/docs/api/reference#get-users-blocks for more information.
#[derive(Debug, Deserialize)]
pub struct UserBlockData {
	/// The id of the blocked user.
	pub user_id: String,

	/// The login of the blocked user.
	pub user_login: String,

	/// The display name of the blocked user.
	pub display_name: String,
}
