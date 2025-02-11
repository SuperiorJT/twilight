//! Gateway event payload when an invite is created.

use crate::{
    id::{ChannelId, GuildId, UserId},
    invite::TargetType,
    user::User,
};
use serde::{Deserialize, Serialize};

/// A new [`Invite`] has been created.
///
/// [`Invite`]: crate::invite::Invite
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InviteCreate {
    /// ID of the channel invited users will first see.
    pub channel_id: ChannelId,
    /// Unique code.
    pub code: String,
    /// When the invite was created.
    ///
    /// This is in an ISO 8601 timestamp format.
    pub created_at: String,
    /// ID of the guild being invited to.
    pub guild_id: GuildId,
    /// Information about the user who created the invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter: Option<User>,
    /// Maximum age before the invite expires.
    ///
    /// This is in seconds.
    pub max_age: u64,
    /// Maximum number of uses before the invite expires.
    pub max_uses: u64,
    /// Target of the invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_type: Option<TargetType>,
    /// User whose stream to display for this voice channel stream invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user: Option<PartialUser>,
    /// Whether the invite is temporary.
    ///
    /// Invited users will be kicked when they are disconnected from an audio
    /// channel unless they're assigned a role.
    pub temporary: bool,
    /// Number of times the invite has been used.
    ///
    /// This will always be zero.
    pub uses: u8,
}

/// Information about the user whose stream to display for a voice channel
/// stream invite.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialUser {
    /// Hash of the user's avatar.
    pub avatar: Option<String>,
    /// Discriminator used to differentiate people with the same [`username`].
    ///
    /// [`username`]: Self::username
    pub discriminator: String,
    /// ID of the user.
    pub id: UserId,
    /// Username of the user.
    pub username: String,
}

#[cfg(test)]
mod tests {
    use super::{InviteCreate, PartialUser};
    use crate::id::{ChannelId, GuildId, UserId};
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        InviteCreate: channel_id,
        code,
        created_at,
        guild_id,
        inviter,
        max_age,
        max_uses,
        target_user_type,
        target_user,
        temporary,
        uses
    );
    assert_fields!(PartialUser: avatar, discriminator, id, username);
    assert_impl_all!(
        InviteCreate: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
    assert_impl_all!(
        PartialUser: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn test_invite_create() {
        let value = InviteCreate {
            channel_id: ChannelId(1),
            code: "a".repeat(7),
            created_at: "2021-01-01T00:00:00+00:00".to_owned(),
            guild_id: GuildId(2),
            inviter: None,
            max_age: 3600,
            max_uses: 5,
            target_user_type: None,
            target_user: None,
            temporary: false,
            uses: 0,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InviteCreate",
                    len: 8,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("code"),
                Token::Str("aaaaaaa"),
                Token::Str("created_at"),
                Token::Str("2021-01-01T00:00:00+00:00"),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("max_age"),
                Token::U64(3600),
                Token::Str("max_uses"),
                Token::U64(5),
                Token::Str("temporary"),
                Token::Bool(false),
                Token::Str("uses"),
                Token::U8(0),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_partial_user() {
        let value = PartialUser {
            avatar: Some("a".repeat(32)),
            discriminator: "123".to_owned(),
            id: UserId(1),
            username: "twilight".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PartialUser",
                    len: 4,
                },
                Token::Str("avatar"),
                Token::Some,
                Token::Str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                Token::Str("discriminator"),
                Token::Str("123"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("1"),
                Token::Str("username"),
                Token::Str("twilight"),
                Token::StructEnd,
            ],
        );
    }
}
