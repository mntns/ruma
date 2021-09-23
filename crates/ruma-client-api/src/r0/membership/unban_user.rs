//! [POST /_matrix/client/r0/rooms/{roomId}/unban](https://matrix.org/docs/spec/client_server/r0.6.0#post-matrix-client-r0-rooms-roomid-unban)

use ruma_api::ruma_api;
use ruma_identifiers::{RoomId, UserId};

ruma_api! {
    metadata: {
        description: "Unban a user from a room.",
        method: POST,
        name: "unban_user",
        path: "/_matrix/client/r0/rooms/:room_id/unban",
        rate_limited: false,
        authentication: AccessToken,
    }

    request: {
        /// The room to unban the user from.
        #[ruma_api(path)]
        pub room_id: &'a RoomId,

        /// The user to unban.
        pub user_id: &'a UserId,

        /// Optional reason for unbanning the user.
        #[cfg(feature = "unstable-pre-spec")]
        #[cfg_attr(docsrs, doc(cfg(feature = "unstable-pre-spec")))]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reason: Option<&'a str>,
    }

    #[derive(Default)]
    response: {}

    error: crate::Error
}

impl<'a> Request<'a> {
    /// Creates a new `Request` with the given room id and room id.
    pub fn new(room_id: &'a RoomId, user_id: &'a UserId) -> Self {
        Self {
            room_id,
            user_id,
            #[cfg(feature = "unstable-pre-spec")]
            reason: None,
        }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}
