use serde::{Deserialize, Serialize};
use ts_rs::TS;
use twilight_model::id::Id;
use twilight_validate::channel::ChannelValidationError;

use crate::{
    discord::channel::{ChannelType, PermissionOverwrite, ThreadMetadata, VideoQualityMode},
    internal::member::Member,
    util::NotBigU64,
};

use super::messages::{Message, OpCreateMessageFields};

#[derive(Clone, Debug, Serialize, TS)]
#[serde(untagged)]
#[ts(export, rename = "InternalGuildChannel")]
#[ts(export_to = "bindings/internal/GuildChannel.ts")]
pub enum GuildChannel {
    Category(CategoryChannel),
    NewsThread(Box<NewsThread>),
    PrivateThread(Box<PrivateThread>),
    PublicThread(Box<PublicThread>),
    Text(TextChannel),
    Voice(VoiceChannel),
    Stage(VoiceChannel),
    GuildDirectory(TextChannel),
    Forum(TextChannel),
    Unknown(UnknownChannel),
}

impl From<twilight_model::channel::Channel> for GuildChannel {
    fn from(v: twilight_model::channel::Channel) -> Self {
        match v.kind {
            twilight_model::channel::ChannelType::GuildCategory => Self::Category(v.into()),
            twilight_model::channel::ChannelType::AnnouncementThread => {
                Self::NewsThread(Box::new(v.into()))
            }
            twilight_model::channel::ChannelType::PrivateThread => {
                Self::PrivateThread(Box::new(v.into()))
            }
            twilight_model::channel::ChannelType::PublicThread => {
                Self::PublicThread(Box::new(v.into()))
            }

            twilight_model::channel::ChannelType::GuildText
            | twilight_model::channel::ChannelType::GuildAnnouncement => Self::Text(v.into()),

            twilight_model::channel::ChannelType::GuildVoice => Self::Voice(v.into()),
            twilight_model::channel::ChannelType::GuildStageVoice => Self::Stage(v.into()),

            twilight_model::channel::ChannelType::Private => {
                panic!("Bot does not support private channels, we should never reach this path")
            }
            twilight_model::channel::ChannelType::Group => {
                panic!("Bot does not support private channels, we should never reach this path")
            }
            twilight_model::channel::ChannelType::GuildDirectory => Self::GuildDirectory(v.into()),
            twilight_model::channel::ChannelType::GuildForum => Self::Forum(v.into()),
            _ => Self::Unknown(UnknownChannel {
                id: v.id.to_string(),
                kind: v.kind.into(),
                unknown_kind_id: u8::from(v.kind),
            }),
        }
    }
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, rename = "IUnknownChannel")]
#[ts(export_to = "bindings/internal/UnknownChannel.ts")]
#[serde(rename_all = "camelCase")]
pub struct UnknownChannel {
    pub id: String,
    #[ts(type = "{Unknown: number}")]
    pub kind: ChannelType,
    pub unknown_kind_id: u8,
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, rename = "IVoiceChannel")]
#[ts(export_to = "bindings/internal/VoiceChannel.ts")]
#[serde(rename_all = "camelCase")]
pub struct VoiceChannel {
    pub bitrate: u32,
    pub id: String,
    #[ts(type = "'Voice'|'StageVoice'")]
    pub kind: ChannelType,
    pub name: String,
    pub parent_id: Option<String>,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub position: i32,
    pub rtc_region: Option<String>,
    pub user_limit: Option<u32>,
    pub video_quality_mode: Option<VideoQualityMode>,
}

impl From<twilight_model::channel::Channel> for VoiceChannel {
    fn from(v: twilight_model::channel::Channel) -> Self {
        Self {
            bitrate: v.bitrate.unwrap_or_default(),
            id: v.id.to_string(),
            kind: v.kind.into(),
            name: v.name.unwrap_or_default(),
            parent_id: v.parent_id.as_ref().map(ToString::to_string),
            permission_overwrites: v
                .permission_overwrites
                .unwrap_or_default()
                .into_iter()
                .map(Into::into)
                .collect(),
            position: v.position.unwrap_or_default(),
            rtc_region: v.rtc_region,
            user_limit: v.user_limit,
            video_quality_mode: v.video_quality_mode.map(Into::into),
        }
    }
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, rename = "ITextChannel")]
#[ts(export_to = "bindings/internal/TextChannel.ts")]
#[serde(rename_all = "camelCase")]
pub struct TextChannel {
    pub id: String,
    #[ts(type = "'Text'|'News'|'Forum'|'GuildDirectory'")]
    pub kind: ChannelType,
    pub last_pin_timestamp: Option<NotBigU64>,
    pub name: String,
    pub nsfw: bool,
    pub parent_id: Option<String>,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub position: i32,
    pub rate_limit_per_user: Option<u16>,
    pub topic: Option<String>,
}

impl From<twilight_model::channel::Channel> for TextChannel {
    fn from(v: twilight_model::channel::Channel) -> Self {
        Self {
            id: v.id.to_string(),
            kind: v.kind.into(),
            last_pin_timestamp: v
                .last_pin_timestamp
                .map(|e| NotBigU64(e.as_micros() as u64 / 1000)),
            name: v.name.unwrap_or_default(),
            nsfw: v.nsfw.unwrap_or_default(),
            parent_id: v.parent_id.as_ref().map(ToString::to_string),
            permission_overwrites: v
                .permission_overwrites
                .unwrap_or_default()
                .into_iter()
                .map(Into::into)
                .collect(),
            position: v.position.unwrap_or_default(),
            rate_limit_per_user: v.rate_limit_per_user,
            topic: v.topic,
        }
    }
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, rename = "IPublicThread")]
#[ts(export_to = "bindings/internal/PublicThread.ts")]
#[serde(rename_all = "camelCase")]
pub struct PublicThread {
    pub default_auto_archive_duration_minutes: Option<u32>,
    pub id: String,
    #[ts(type = "'PublicThread'")]
    pub kind: ChannelType,
    pub member: Option<SelfThreadMember>,
    pub member_count: i8,
    pub message_count: u32,
    pub name: String,
    pub owner_id: Option<String>,
    pub parent_id: Option<String>,
    pub rate_limit_per_user: Option<u16>,
    pub thread_metadata: ThreadMetadata,
}

impl From<twilight_model::channel::Channel> for PublicThread {
    fn from(v: twilight_model::channel::Channel) -> Self {
        Self {
            default_auto_archive_duration_minutes: v
                .default_auto_archive_duration
                .map(|v| v.number() as u32),
            id: v.id.to_string(),
            kind: v.kind.into(),
            member: v.member.map(Into::into),
            member_count: v.member_count.unwrap_or_default(),
            message_count: v.message_count.unwrap_or_default(),
            name: v.name.unwrap_or_default(),
            owner_id: v.owner_id.as_ref().map(ToString::to_string),
            parent_id: v.parent_id.as_ref().map(ToString::to_string),
            rate_limit_per_user: v.rate_limit_per_user,
            thread_metadata: v.thread_metadata.unwrap_or_else(empty_thread_meta).into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, rename = "IPrivateThread")]
#[ts(export_to = "bindings/internal/PrivateThread.ts")]
#[serde(rename_all = "camelCase")]
pub struct PrivateThread {
    pub default_auto_archive_duration_minutes: Option<u32>,
    pub id: String,
    pub invitable: Option<bool>,
    #[ts(type = "'PrivateThread'")]
    pub kind: ChannelType,
    pub member: Option<SelfThreadMember>,
    pub member_count: i8,
    pub message_count: u32,
    pub name: String,
    pub owner_id: Option<String>,
    pub parent_id: Option<String>,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub rate_limit_per_user: Option<u16>,
    pub thread_metadata: ThreadMetadata,
}

impl From<twilight_model::channel::Channel> for PrivateThread {
    fn from(v: twilight_model::channel::Channel) -> Self {
        Self {
            default_auto_archive_duration_minutes: v
                .default_auto_archive_duration
                .map(|v| v.number() as u32),
            id: v.id.to_string(),
            kind: v.kind.into(),
            member: v.member.map(Into::into),
            member_count: v.member_count.unwrap_or_default(),
            message_count: v.message_count.unwrap_or_default(),
            name: v.name.unwrap_or_default(),
            owner_id: v.owner_id.as_ref().map(ToString::to_string),
            parent_id: v.parent_id.as_ref().map(ToString::to_string),
            rate_limit_per_user: v.rate_limit_per_user,
            thread_metadata: v.thread_metadata.unwrap_or_else(empty_thread_meta).into(),
            permission_overwrites: v
                .permission_overwrites
                .unwrap_or_default()
                .into_iter()
                .map(Into::into)
                .collect(),
            invitable: v.invitable,
        }
    }
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, rename = "INewsThread")]
#[ts(export_to = "bindings/internal/NewsThread.ts")]
#[serde(rename_all = "camelCase")]
pub struct NewsThread {
    pub default_auto_archive_duration_minutes: Option<u32>,
    pub id: String,
    #[ts(type = "'NewsThread'")]
    pub kind: ChannelType,
    pub member: Option<SelfThreadMember>,
    pub member_count: i8,
    pub message_count: u32,
    pub name: String,
    pub owner_id: Option<String>,
    pub parent_id: Option<String>,
    pub rate_limit_per_user: Option<u16>,
    pub thread_metadata: ThreadMetadata,
}

impl From<twilight_model::channel::Channel> for NewsThread {
    fn from(v: twilight_model::channel::Channel) -> Self {
        Self {
            default_auto_archive_duration_minutes: v
                .default_auto_archive_duration
                .map(|v| v.number() as u32),
            id: v.id.to_string(),
            kind: v.kind.into(),
            member: v.member.map(Into::into),
            member_count: v.member_count.unwrap_or_default(),
            message_count: v.message_count.unwrap_or_default(),
            name: v.name.unwrap_or_default(),
            owner_id: v.owner_id.as_ref().map(ToString::to_string),
            parent_id: v.parent_id.as_ref().map(ToString::to_string),
            rate_limit_per_user: v.rate_limit_per_user,
            thread_metadata: v.thread_metadata.unwrap_or_else(empty_thread_meta).into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, rename = "IThreadMember")]
#[ts(export_to = "bindings/internal/ThreadMember.ts")]
#[serde(rename_all = "camelCase")]
pub struct ThreadMember {
    // Removed as the values aren't documented anywhere and i want to make a proper
    // abstraction for this similar to UserFlags and the like.
    // pub flags: NotBigU64,
    pub id: Option<String>,
    pub join_timestamp: NotBigU64,
    pub member: Option<Member>,

    // Unsure if presence is provided without presence intent
    // pub presence: Option<Presence>,
    pub user_id: Option<String>,
}

impl From<twilight_model::channel::thread::ThreadMember> for ThreadMember {
    fn from(v: twilight_model::channel::thread::ThreadMember) -> Self {
        Self {
            // flags: NotBigU64(v.flags),
            id: v.id.as_ref().map(ToString::to_string),
            join_timestamp: NotBigU64(v.join_timestamp.as_micros() as u64 / 1000),
            member: v.member.map(Into::into),
            user_id: v.user_id.as_ref().map(ToString::to_string),
        }
    }
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, rename = "ISelfThreadMember")]
#[ts(export_to = "bindings/internal/ISelfThreadMember.ts")]
#[serde(rename_all = "camelCase")]
pub struct SelfThreadMember {
    // Removed as the values aren't documented anywhere and i want to make a proper
    // abstraction for this similar to UserFlags and the like.
    // pub flags: NotBigU64,
    pub join_timestamp: NotBigU64,
}

impl From<twilight_model::channel::thread::ThreadMember> for SelfThreadMember {
    fn from(v: twilight_model::channel::thread::ThreadMember) -> Self {
        Self {
            // flags: NotBigU64(v.flags),
            join_timestamp: NotBigU64(v.join_timestamp.as_micros() as u64 / 1000),
        }
    }
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, rename = "ICategoryChannel")]
#[ts(export_to = "bindings/internal/CategoryChannel.ts")]
#[serde(rename_all = "camelCase")]
pub struct CategoryChannel {
    pub id: String,
    #[ts(type = "'Category'")]
    pub kind: ChannelType,
    pub name: String,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub position: i32,
}

impl From<twilight_model::channel::Channel> for CategoryChannel {
    fn from(v: twilight_model::channel::Channel) -> Self {
        Self {
            kind: v.kind.into(),
            id: v.id.to_string(),
            name: v.name.unwrap_or_default(),
            position: v.position.unwrap_or_default(),
            permission_overwrites: v
                .permission_overwrites
                .unwrap_or_default()
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

fn empty_thread_meta() -> twilight_model::channel::thread::ThreadMetadata {
    twilight_model::channel::thread::ThreadMetadata {
        archived: false,
        auto_archive_duration: 60u16.into(),
        archive_timestamp: twilight_model::util::Timestamp::from_secs(0).unwrap(),
        create_timestamp: None,
        invitable: None,
        locked: false,
    }
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(
    export,
    rename = "IEditChannel",
    export_to = "bindings/internal/EditChannel.ts"
)]
#[serde(rename_all = "camelCase")]
pub struct EditChannel {
    #[ts(optional)]
    #[serde(default)]
    bitrate: Option<u32>,

    #[ts(optional)]
    #[serde(default)]
    name: Option<String>,

    #[ts(optional)]
    #[serde(default)]
    nsfw: Option<bool>,

    #[ts(optional)]
    #[serde(deserialize_with = "crate::deserialize_undefined_null_optional_field")]
    parent_id: Option<Option<String>>,

    #[ts(optional)]
    #[serde(default)]
    permission_overwrites: Option<Vec<PermissionOverwrite>>,

    #[ts(optional)]
    #[serde(default)]
    position: Option<NotBigU64>,

    #[ts(optional)]
    #[serde(default)]
    rate_limit_per_user: Option<u16>,

    #[ts(optional)]
    #[serde(default)]
    topic: Option<String>,

    #[ts(optional)]
    #[serde(default)]
    user_limit: Option<u16>,

    #[ts(optional)]
    #[serde(default)]
    video_quality_mode: Option<VideoQualityMode>,
}

impl EditChannel {
    pub fn apply<'a, 'b, 'c>(
        &'a self,
        perms_buf: &'b mut Vec<twilight_model::channel::permission_overwrite::PermissionOverwrite>,
        mut req: twilight_http::request::channel::UpdateChannel<'c>,
    ) -> Result<twilight_http::request::channel::UpdateChannel<'c>, ChannelValidationError>
    where
        'a: 'c,
        'b: 'c,
    {
        if let Some(bitrate) = &self.bitrate {
            req = req.bitrate(*bitrate)?;
        }

        if let Some(name) = &self.name {
            req = req.name(name)?;
        }

        if let Some(nsfw) = &self.nsfw {
            req = req.nsfw(*nsfw);
        }

        if let Some(parent_id) = &self.parent_id {
            // TODO: Should we error on invalid ID's?
            let parent_id = parent_id
                .as_ref()
                .and_then(|s| Id::new_checked(s.parse().ok()?));

            req = req.parent_id(parent_id);
        }

        if let Some(permission_overwrites) = &self.permission_overwrites {
            // TODO: should we error on bad overwrites instead of throwing them away?
            perms_buf.extend(
                permission_overwrites
                    .clone()
                    .into_iter()
                    .filter_map(|v| v.try_into().ok()),
            );

            req = req.permission_overwrites(perms_buf);
        }

        if let Some(position) = &self.position {
            req = req.position(position.0);
        }

        if let Some(rate_limit_per_user) = &self.rate_limit_per_user {
            req = req.rate_limit_per_user(*rate_limit_per_user)?;
        }

        if let Some(topic) = &self.topic {
            req = req.topic(topic)?;
        }

        if let Some(user_limit) = &self.user_limit {
            req = req.user_limit(*user_limit)?;
        }

        if let Some(video_quality_mode) = &self.video_quality_mode {
            req = req.video_quality_mode((*video_quality_mode).into());
        }

        Ok(req)
    }
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(
    export,
    rename = "IUpdateThread",
    export_to = "bindings/internal/IUpdateThread.ts"
)]
#[serde(rename_all = "camelCase")]
pub struct UpdateThread {
    pub channel_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration_minutes: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u16>,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(
    export,
    rename = "ICreateChannel",
    export_to = "bindings/internal/ICreateChannel.ts"
)]
#[serde(rename_all = "camelCase")]
pub struct CreateChannel {
    pub name: String,

    #[ts(optional)]
    #[serde(default)]
    pub kind: Option<ChannelType>,

    #[ts(optional)]
    #[serde(default)]
    bitrate: Option<u32>,

    #[ts(optional)]
    #[serde(default)]
    nsfw: Option<bool>,

    #[ts(optional)]
    #[serde(default)]
    parent_id: Option<String>,

    #[ts(optional)]
    #[serde(default)]
    permission_overwrites: Option<Vec<PermissionOverwrite>>,

    #[ts(optional)]
    #[serde(default)]
    position: Option<NotBigU64>,

    #[ts(optional)]
    #[serde(default)]
    rate_limit_per_user: Option<u16>,

    #[ts(optional)]
    #[serde(default)]
    topic: Option<String>,

    #[ts(optional)]
    #[serde(default)]
    user_limit: Option<u16>,
}

impl CreateChannel {
    pub fn apply<'a, 'b, 'c>(
        &'a self,
        perms_buf: &'b mut Vec<twilight_model::channel::permission_overwrite::PermissionOverwrite>,
        mut req: twilight_http::request::guild::CreateGuildChannel<'c>,
    ) -> Result<twilight_http::request::guild::CreateGuildChannel<'c>, ChannelValidationError>
    where
        'a: 'c,
        'b: 'c,
    {
        if let Some(bitrate) = &self.bitrate {
            req = req.bitrate(*bitrate)?;
        }

        if let Some(nsfw) = &self.nsfw {
            req = req.nsfw(*nsfw);
        }

        if let Some(parent_id) = &self.parent_id {
            // TODO: Should we error on invalid ID's?
            if let Ok(parsed) = parent_id.parse() {
                if let Some(id) = Id::new_checked(parsed) {
                    req = req.parent_id(id);
                }
            }
        }

        if let Some(permission_overwrites) = &self.permission_overwrites {
            // TODO: should we error on bad overwrites instead of throwing them away?
            perms_buf.extend(
                permission_overwrites
                    .clone()
                    .into_iter()
                    .filter_map(|v| v.try_into().ok()),
            );

            req = req.permission_overwrites(perms_buf);
        }

        if let Some(position) = &self.position {
            req = req.position(position.0);
        }

        if let Some(rate_limit_per_user) = &self.rate_limit_per_user {
            req = req.rate_limit_per_user(*rate_limit_per_user)?;
        }

        if let Some(topic) = &self.topic {
            req = req.topic(topic)?;
        }

        if let Some(user_limit) = &self.user_limit {
            req = req.user_limit(*user_limit);
        }

        if let Some(kind) = self.kind {
            req = req.kind(kind.into())
        }

        Ok(req)
    }
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(
    export,
    rename = "ICreateThread",
    export_to = "bindings/internal/ICreateThread.ts"
)]
#[serde(rename_all = "camelCase")]
pub struct CreateThread {
    pub channel_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration_minutes: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
    pub kind: ChannelType,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(
    export,
    rename = "ICreateThreadFromMessage",
    export_to = "bindings/internal/ICreateThreadFromMessage.ts"
)]
#[serde(rename_all = "camelCase")]
pub struct CreateThreadFromMessage {
    pub channel_id: String,
    pub message_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration_minutes: Option<u16>,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(
    export,
    rename = "ICreateForumThread",
    export_to = "bindings/internal/ICreateForumThread.ts"
)]
#[serde(rename_all = "camelCase")]
pub struct CreateForumThread {
    pub channel_id: String,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration_minutes: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u16>,

    pub message: OpCreateMessageFields,
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(
    export,
    rename = "IThreadsListing",
    export_to = "bindings/internal/IThreadsListing.ts"
)]
#[serde(rename_all = "camelCase")]
pub struct ThreadsListing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    pub members: Vec<ThreadMember>,
    pub threads: Vec<GuildChannel>,
}

impl From<twilight_model::channel::thread::ThreadsListing> for ThreadsListing {
    fn from(value: twilight_model::channel::thread::ThreadsListing) -> Self {
        Self {
            has_more: value.has_more,
            members: value.members.into_iter().map(Into::into).collect(),
            threads: value.threads.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(
    export,
    rename = "IListThreadsRequest",
    export_to = "bindings/internal/IListThreadsRequest.ts"
)]
#[serde(rename_all = "camelCase")]
pub struct ListThreadsRequest {
    pub channel_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<NotBigU64>,
}

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(
    export,
    rename = "IListThreadMembersRequest",
    export_to = "bindings/internal/IListThreadMembersRequest.ts"
)]
#[serde(rename_all = "camelCase")]
pub struct ListThreadMembersRequest {
    pub channel_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_member: Option<bool>,
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(
    export,
    rename = "IForumThreadResponse",
    export_to = "bindings/internal/IForumThreadResponse.ts"
)]
#[serde(rename_all = "camelCase")]
pub struct ForumThreadResponse {
    pub message: Message,
    pub channel: GuildChannel,
}
