extern crate crypto_hash;
extern crate serde;
extern crate ssb_legacy_msg_data;
extern crate ssb_multiformats;
extern crate strtod;

use ssb_legacy_msg_data::LegacyF64;
use ssb_multiformats::{
    multibox::Multibox,
    multihash::Multihash,
    multikey::{Multikey, Multisig},
};

pub mod json;
pub mod verify;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Content<T> {
    Encrypted(Multibox),
    Plain(T),
}

/// A complete ssb message, signed and all.
///
/// This does not check whether the `content` value is valid.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Message<T> {
    pub previous: Option<Multihash>,
    pub author: Multikey,
    pub sequence: u64,
    pub timestamp: LegacyF64,
    pub content: Content<T>,
    pub swapped: bool,
    pub signature: Option<Multisig>,
}

impl<T> Message<T> {
    /// Return whether the content of this message is encrypted.
    pub fn is_encrypted(&self) -> bool {
        match self.content {
            Content::Encrypted(..) => true,
            Content::Plain(..) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use ssb_legacy_msg_data::value::Value;
    use std::fs::read_to_string;

    #[test]
    fn parse() {
        // js-produced signing encoding (methinks)
        let s = read_to_string("./test-data/alice/about-value-from-log.json").unwrap();
        dbg!(&s);

        let (msg, _rest) = json::from_legacy::<Value>(&s.as_bytes()).unwrap();

        eprintln!("msg: {:?}", msg);

        assert!(msg.previous.is_none());

        // TODO: struct SigningEncoding
        // let reencoded = json::to_legacy_vec(&msg, false).unwrap();
        // assert_eq!(&s.as_bytes(), &reencoded.as_slice());
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Post {
        text: String,
        mentions: Vec<String>, //TODO: not String, MultiWhatever
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag = "type")] //
    pub enum MessageContent {
        #[serde(rename = "post")]
        Post(Post),
    }

    #[test]
    fn round_trip_a_post_message() {
        let s = read_to_string("./test-data/alice/post-valid.json").unwrap();
        dbg!(&s);

        let (msg, _rest) = json::from_legacy::<MessageContent>(&s.as_bytes()).unwrap();
        let sign_json = json::to_legacy_vec(&msg, false).unwrap();
        let (msg, _rest) = json::from_legacy::<MessageContent>(&sign_json).unwrap();

        eprintln!("msg: {:?}", msg);

        match msg.content {
            Content::Plain(MessageContent::Post(msg)) => assert_eq!(msg.text, "Bob?"),
            _ => panic!(),
        }
    }
}
