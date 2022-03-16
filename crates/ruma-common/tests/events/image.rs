#![cfg(feature = "unstable-msc3552")]

use assign::assign;
use js_int::uint;
use matches::assert_matches;
use ruma_common::{
    event_id,
    events::{
        file::{EncryptedContentInit, FileContent, FileContentInfo},
        image::{
            Captions, ImageContent, ImageEventContent, ThumbnailContent, ThumbnailFileContent,
            ThumbnailFileContentInfo, Thumbnails,
        },
        message::MessageContent,
        room::{
            message::{InReplyTo, Relation},
            JsonWebKeyInit,
        },
        AnyMessageLikeEvent, MessageLikeEvent, MessageLikeUnsigned,
    },
    mxc_uri, room_id,
    serde::Base64,
    user_id, MilliSecondsSinceUnixEpoch,
};
use serde_json::{from_value as from_json_value, json, to_value as to_json_value};

#[test]
fn plain_content_serialization() {
    let event_content = ImageEventContent::plain(
        "Upload: my_image.jpg",
        FileContent::plain(mxc_uri!("mxc://notareal.hs/abcdef").to_owned(), None),
    );

    assert_eq!(
        to_json_value(&event_content).unwrap(),
        json!({
            "org.matrix.msc1767.text": "Upload: my_image.jpg",
            "org.matrix.msc1767.file": {
                "url": "mxc://notareal.hs/abcdef",
            },
            "org.matrix.msc1767.image": {}
        })
    );
}

#[test]
fn encrypted_content_serialization() {
    let event_content = ImageEventContent::plain(
        "Upload: my_image.jpg",
        FileContent::encrypted(
            mxc_uri!("mxc://notareal.hs/abcdef").to_owned(),
            EncryptedContentInit {
                key: JsonWebKeyInit {
                    kty: "oct".to_owned(),
                    key_ops: vec!["encrypt".to_owned(), "decrypt".to_owned()],
                    alg: "A256CTR".to_owned(),
                    k: Base64::parse("TLlG_OpX807zzQuuwv4QZGJ21_u7weemFGYJFszMn9A").unwrap(),
                    ext: true,
                }
                .into(),
                iv: Base64::parse("S22dq3NAX8wAAAAAAAAAAA").unwrap(),
                hashes: [(
                    "sha256".to_owned(),
                    Base64::parse("aWOHudBnDkJ9IwaR1Nd8XKoI7DOrqDTwt6xDPfVGN6Q").unwrap(),
                )]
                .into(),
                v: "v2".to_owned(),
            }
            .into(),
            None,
        ),
    );

    assert_eq!(
        to_json_value(&event_content).unwrap(),
        json!({
            "org.matrix.msc1767.text": "Upload: my_image.jpg",
            "org.matrix.msc1767.file": {
                "url": "mxc://notareal.hs/abcdef",
                "key": {
                    "kty": "oct",
                    "key_ops": ["encrypt", "decrypt"],
                    "alg": "A256CTR",
                    "k": "TLlG_OpX807zzQuuwv4QZGJ21_u7weemFGYJFszMn9A",
                    "ext": true
                },
                "iv": "S22dq3NAX8wAAAAAAAAAAA",
                "hashes": {
                    "sha256": "aWOHudBnDkJ9IwaR1Nd8XKoI7DOrqDTwt6xDPfVGN6Q"
                },
                "v": "v2"
            },
            "org.matrix.msc1767.image": {}
        })
    );
}

#[test]
fn image_event_serialization() {
    let event = MessageLikeEvent {
        content: assign!(
            ImageEventContent::with_message(
                MessageContent::html(
                    "Upload: my_house.jpg",
                    "Upload: <strong>my_house.jpg</strong>",
                ),
                FileContent::plain(
                    mxc_uri!("mxc://notareal.hs/abcdef").to_owned(),
                    Some(Box::new(assign!(
                        FileContentInfo::new(),
                        {
                            name: Some("my_house.jpg".to_owned()),
                            mimetype: Some("image/jpeg".to_owned()),
                            size: Some(uint!(897_774)),
                        }
                    ))),
                )
            ),
            {
                image: Box::new(ImageContent::with_size(uint!(1920), uint!(1080))),
                thumbnail: Thumbnails::new(&[ThumbnailContent::new(
                    ThumbnailFileContent::plain(
                        mxc_uri!("mxc://notareal.hs/thumbnail").to_owned(),
                        Some(Box::new(assign!(ThumbnailFileContentInfo::new(), {
                            mimetype: Some("image/jpeg".to_owned()),
                            size: Some(uint!(334_593)),
                        })))
                    ),
                    None
                )]),
                caption: Captions::plain("This is my house"),
                relates_to: Some(Relation::Reply {
                    in_reply_to: InReplyTo::new(event_id!("$replyevent:example.com").to_owned()),
                }),
            }
        ),
        event_id: event_id!("$event:notareal.hs").to_owned(),
        sender: user_id!("@user:notareal.hs").to_owned(),
        origin_server_ts: MilliSecondsSinceUnixEpoch(uint!(134_829_848)),
        room_id: room_id!("!roomid:notareal.hs").to_owned(),
        unsigned: MessageLikeUnsigned::default(),
    };

    assert_eq!(
        to_json_value(&event).unwrap(),
        json!({
            "content": {
                "org.matrix.msc1767.message": [
                    { "body": "Upload: <strong>my_house.jpg</strong>", "mimetype": "text/html"},
                    { "body": "Upload: my_house.jpg", "mimetype": "text/plain"},
                ],
                "org.matrix.msc1767.file": {
                    "url": "mxc://notareal.hs/abcdef",
                    "name": "my_house.jpg",
                    "mimetype": "image/jpeg",
                    "size": 897_774,
                },
                "org.matrix.msc1767.image": {
                    "width": 1920,
                    "height": 1080,
                },
                "org.matrix.msc1767.thumbnail": [
                    {
                        "url": "mxc://notareal.hs/thumbnail",
                        "mimetype": "image/jpeg",
                        "size": 334_593,
                    }
                ],
                "org.matrix.msc1767.caption": [
                    {
                        "body": "This is my house",
                        "mimetype": "text/plain",
                    }
                ],
                "m.relates_to": {
                    "m.in_reply_to": {
                        "event_id": "$replyevent:example.com"
                    }
                }
            },
            "event_id": "$event:notareal.hs",
            "origin_server_ts": 134_829_848,
            "room_id": "!roomid:notareal.hs",
            "sender": "@user:notareal.hs",
            "type": "m.image",
        })
    );
}

#[test]
fn plain_content_deserialization() {
    let json_data = json!({
        "org.matrix.msc1767.text": "Upload: my_cat.png",
        "org.matrix.msc1767.file": {
            "url": "mxc://notareal.hs/abcdef",
        },
        "org.matrix.msc1767.image": {
            "width": 668,
        },
        "org.matrix.msc1767.caption": [
            {
                "body": "Look at my cat!",
            }
        ]
    });

    assert_matches!(
        from_json_value::<ImageEventContent>(json_data)
            .unwrap(),
        ImageEventContent { message, file, image, thumbnail, caption, .. }
        if message.find_plain() == Some("Upload: my_cat.png")
            && message.find_html().is_none()
            && file.url == "mxc://notareal.hs/abcdef"
            && image.width == Some(uint!(668))
            && image.height.is_none()
            && thumbnail.is_empty()
            && caption.find_plain() == Some("Look at my cat!")
    );
}

#[test]
fn encrypted_content_deserialization() {
    let json_data = json!({
        "org.matrix.msc1767.text": "Upload: my_file.txt",
        "org.matrix.msc1767.file": {
            "url": "mxc://notareal.hs/abcdef",
            "key": {
                "kty": "oct",
                "key_ops": ["encrypt", "decrypt"],
                "alg": "A256CTR",
                "k": "TLlG_OpX807zzQuuwv4QZGJ21_u7weemFGYJFszMn9A",
                "ext": true
            },
            "iv": "S22dq3NAX8wAAAAAAAAAAA",
            "hashes": {
                "sha256": "aWOHudBnDkJ9IwaR1Nd8XKoI7DOrqDTwt6xDPfVGN6Q"
            },
            "v": "v2"
        },
        "org.matrix.msc1767.image": {},
        "org.matrix.msc1767.thumbnail": [
            {
                "url": "mxc://notareal.hs/thumbnail",
            }
        ]
    });

    assert_matches!(
        from_json_value::<ImageEventContent>(json_data)
            .unwrap(),
        ImageEventContent { message, file, image, thumbnail, caption, .. }
        if message.find_plain() == Some("Upload: my_file.txt")
            && message.find_html().is_none()
            && file.url == "mxc://notareal.hs/abcdef"
            && file.encryption_info.is_some()
            && image.width.is_none()
            && image.height.is_none()
            && thumbnail.thumbnails()[0].file.url == "mxc://notareal.hs/thumbnail"
            && caption.is_empty()
    );
}

#[test]
fn message_event_deserialization() {
    let json_data = json!({
        "content": {
            "org.matrix.msc1767.text": "Upload: my_gnome.webp",
            "org.matrix.msc1767.file": {
                "url": "mxc://notareal.hs/abcdef",
                "name": "my_gnome.webp",
                "mimetype": "image/webp",
                "size": 123_774,
            },
            "org.matrix.msc1767.image": {
                "width": 1300,
                "height": 837,
            }
        },
        "event_id": "$event:notareal.hs",
        "origin_server_ts": 134_829_848,
        "room_id": "!roomid:notareal.hs",
        "sender": "@user:notareal.hs",
        "type": "m.image",
    });

    assert_matches!(
        from_json_value::<AnyMessageLikeEvent>(json_data).unwrap(),
        AnyMessageLikeEvent::Image(MessageLikeEvent {
            content: ImageEventContent {
                message,
                file: FileContent {
                    url,
                    info: Some(info),
                    ..
                },
                image,
                thumbnail,
                caption,
                ..
            },
            event_id,
            origin_server_ts,
            room_id,
            sender,
            unsigned
        }) if event_id == event_id!("$event:notareal.hs")
            && message.find_plain() == Some("Upload: my_gnome.webp")
            && message.find_html().is_none()
            && url == "mxc://notareal.hs/abcdef"
            && info.name.as_deref() == Some("my_gnome.webp")
            && info.mimetype.as_deref() == Some("image/webp")
            && info.size == Some(uint!(123_774))
            && image.width == Some(uint!(1300))
            && image.height == Some(uint!(837))
            && thumbnail.is_empty()
            && caption.is_empty()
            && origin_server_ts == MilliSecondsSinceUnixEpoch(uint!(134_829_848))
            && room_id == room_id!("!roomid:notareal.hs")
            && sender == user_id!("@user:notareal.hs")
            && unsigned.is_empty()
    );
}