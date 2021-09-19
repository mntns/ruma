//! Matrix room identifiers.

use std::convert::TryInto;

use crate::ServerName;

/// A Matrix room ID.
///
/// A `RoomId` is generated randomly or converted from a string slice, and can be converted back
/// into a string as needed.
///
/// ```
/// # use std::convert::TryFrom;
/// # use ruma_identifiers::RoomId;
/// assert_eq!(
///     <&RoomId>::try_from("!n8f893n9:example.com").unwrap().as_ref(),
///     "!n8f893n9:example.com"
/// );
/// ```

#[repr(transparent)]
pub struct RoomId(str);

opaque_identifier_validated!(RoomId, ruma_identifiers_validation::room_id::validate);

impl RoomId {
    /// Attempts to generate a `RoomId` for the given origin server with a localpart consisting of
    /// 18 random ASCII characters.
    ///
    /// Fails if the given homeserver cannot be parsed as a valid host.
    #[cfg(feature = "rand")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
    pub fn new(server_name: &ServerName) -> Box<Self> {
        Self::from_owned(format!("!{}:{}", crate::generate_localpart(18), server_name).into())
    }
}

impl RoomId {
    /// Returns the rooms's unique ID.
    pub fn localpart(&self) -> &str {
        &self.as_str()[1..self.colon_idx()]
    }

    /// Returns the server name of the room ID.
    pub fn server_name(&self) -> &ServerName {
        self.as_str()[self.colon_idx() + 1..].try_into().unwrap()
    }

    fn colon_idx(&self) -> usize {
        self.as_str().find(':').unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::RoomId;
    use crate::Error;

    #[test]
    fn valid_room_id() {
        assert_eq!(
            <&RoomId>::try_from("!29fhd83h92h0:example.com")
                .expect("Failed to create RoomId.")
                .as_ref(),
            "!29fhd83h92h0:example.com"
        );
    }

    #[test]
    fn empty_localpart() {
        assert_eq!(
            <&RoomId>::try_from("!:example.com").expect("Failed to create RoomId.").as_ref(),
            "!:example.com"
        );
    }

    #[cfg(feature = "rand")]
    #[test]
    fn generate_random_valid_room_id() {
        use crate::ServerName;

        let server_name =
            <&ServerName>::try_from("example.com").expect("Failed to parse ServerName");
        let room_id = RoomId::new(server_name);
        let id_str = room_id.as_str();

        assert!(id_str.starts_with('!'));
        assert_eq!(id_str.len(), 31);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_valid_room_id() {
        assert_eq!(
            serde_json::to_string(
                <&RoomId>::try_from("!29fhd83h92h0:example.com").expect("Failed to create RoomId.")
            )
            .expect("Failed to convert RoomId to JSON."),
            r#""!29fhd83h92h0:example.com""#
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize_valid_room_id() {
        assert_eq!(
            serde_json::from_str::<Box<RoomId>>(r#""!29fhd83h92h0:example.com""#)
                .expect("Failed to convert JSON to RoomId"),
            <&RoomId>::try_from("!29fhd83h92h0:example.com").expect("Failed to create RoomId.")
        );
    }

    #[test]
    fn valid_room_id_with_explicit_standard_port() {
        assert_eq!(
            <&RoomId>::try_from("!29fhd83h92h0:example.com:443")
                .expect("Failed to create RoomId.")
                .as_ref(),
            "!29fhd83h92h0:example.com:443"
        );
    }

    #[test]
    fn valid_room_id_with_non_standard_port() {
        assert_eq!(
            <&RoomId>::try_from("!29fhd83h92h0:example.com:5000")
                .expect("Failed to create RoomId.")
                .as_ref(),
            "!29fhd83h92h0:example.com:5000"
        );
    }

    #[test]
    fn missing_room_id_sigil() {
        assert_eq!(
            <&RoomId>::try_from("carl:example.com").unwrap_err(),
            Error::MissingLeadingSigil
        );
    }

    #[test]
    fn missing_room_id_delimiter() {
        assert_eq!(<&RoomId>::try_from("!29fhd83h92h0").unwrap_err(), Error::MissingDelimiter);
    }

    #[test]
    fn invalid_room_id_host() {
        assert_eq!(<&RoomId>::try_from("!29fhd83h92h0:/").unwrap_err(), Error::InvalidServerName);
    }

    #[test]
    fn invalid_room_id_port() {
        assert_eq!(
            <&RoomId>::try_from("!29fhd83h92h0:example.com:notaport").unwrap_err(),
            Error::InvalidServerName
        );
    }
}
