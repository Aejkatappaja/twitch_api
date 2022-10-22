//! Convenience functions for [HelixClient]

use crate::helix::{self, ClientRequestError, HelixClient};
use crate::types;
use twitch_oauth2::TwitchToken;

type ClientError<'a, C> = ClientRequestError<<C as crate::HttpClient<'a>>::Error>;

// TODO: Consider moving these into the specific modules where the request is defined. Preferably backed by a macro

impl<'a, C: crate::HttpClient<'a> + Sync> HelixClient<'a, C> {
    /// Get [User](helix::users::User) from user login
    pub async fn get_user_from_login<T>(
        &'a self,
        login: impl Into<types::UserName>,
        token: &T,
    ) -> Result<Option<helix::users::User>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        self.req_get(helix::users::GetUsersRequest::login(login), token)
            .await
            .map(|response| response.first())
    }

    /// Get [User](helix::users::User) from user id
    pub async fn get_user_from_id<T>(
        &'a self,
        id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<Option<helix::users::User>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        self.req_get(helix::users::GetUsersRequest::id(id), token)
            .await
            .map(|response| response.first())
    }

    /// Get [ChannelInformation](helix::channels::ChannelInformation) from a broadcasters login
    pub async fn get_channel_from_login<T>(
        &'a self,
        login: impl Into<types::UserName>,
        token: &T,
    ) -> Result<Option<helix::channels::ChannelInformation>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        if let Some(user) = self.get_user_from_login(login, token).await? {
            self.get_channel_from_id(user.id, token).await
        } else {
            Ok(None)
        }
    }

    /// Get [ChannelInformation](helix::channels::ChannelInformation) from a broadcasters id
    pub async fn get_channel_from_id<T>(
        &'a self,
        id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<Option<helix::channels::ChannelInformation>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        self.req_get(
            helix::channels::GetChannelInformationRequest::broadcaster_id(id),
            token,
        )
        .await
        .map(|response| response.first())
    }

    /// Get chatters in a stream [Chatter][helix::chat::Chatter]
    ///
    /// `batch_size` sets the amount of chatters to retrieve per api call, max 1000, defaults to 100.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let chatters: Vec<helix::chat::Chatter> = client.get_chatters("1234", "4321", 1000, &token).try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn get_chatters<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        moderator_id: impl Into<types::UserId>,
        batch_size: impl Into<Option<usize>>,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<dyn futures::Stream<Item = Result<helix::chat::Chatter, ClientError<'a, C>>> + 'a>,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::chat::GetChattersRequest {
            first: batch_size.into(),
            ..helix::chat::GetChattersRequest::new(broadcaster_id, moderator_id)
        };

        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Search [Categories](helix::search::Category)
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let categories: Vec<helix::search::Category> = client.search_categories("Fortnite", &token).try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn search_categories<T>(
        &'a self,
        query: impl Into<String>,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<dyn futures::Stream<Item = Result<helix::search::Category, ClientError<'a, C>>> + 'a>,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::search::SearchCategoriesRequest::query(query).first(100);
        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Search [Channels](helix::search::Channel) via channel name or description
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let channel: Vec<helix::search::Channel> = client.search_channels("twitchdev", false, &token).try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn search_channels<T>(
        &'a self,
        query: impl Into<String>,
        live_only: bool,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<dyn futures::Stream<Item = Result<helix::search::Channel, ClientError<'a, C>>> + 'a>,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::search::SearchChannelsRequest::query(query.into()).live_only(live_only);
        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Get information on a [follow relationship](helix::users::FollowRelationship)
    ///
    /// Can be used to see if X follows Y
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::{types, helix};
    /// use futures::TryStreamExt;
    ///
    /// // Get the followers of channel "1234"
    /// let followers: Vec<helix::users::FollowRelationship> = client.get_follow_relationships(types::UserId::from("1234"), None, &token).try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn get_follow_relationships<T>(
        &'a self,
        to_id: impl Into<Option<types::UserId>>,
        from_id: impl Into<Option<types::UserId>>,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<
            dyn futures::Stream<Item = Result<helix::users::FollowRelationship, ClientError<'a, C>>>
                + Send
                + 'a,
        >,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let mut req = helix::users::GetUsersFollowsRequest::empty();
        req.to_id = to_id.into();
        req.from_id = from_id.into();

        make_stream(req, token, self, |s| {
            std::collections::VecDeque::from(s.follow_relationships)
        })
    }

    /// Get authenticated users' followed [streams](helix::streams::Stream)
    ///
    /// Requires token with scope [`user:read:follows`](twitch_oauth2::Scope::UserReadFollows).
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let channels: Vec<helix::streams::Stream> = client.get_followed_streams(&token).try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn get_followed_streams<T>(
        &'a self,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<dyn futures::Stream<Item = Result<helix::streams::Stream, ClientError<'a, C>>> + 'a>,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        use futures::StreamExt;

        let user_id = match token
            .user_id()
            .ok_or_else(|| ClientRequestError::Custom("no user_id found on token".into()))
        {
            Ok(t) => t,
            Err(e) => return futures::stream::once(async { Err(e) }).boxed(),
        };
        let req = helix::streams::GetFollowedStreamsRequest::user_id(user_id);
        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Get authenticated broadcasters' [subscribers](helix::subscriptions::BroadcasterSubscription)
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let subs: Vec<helix::subscriptions::BroadcasterSubscription> = client.get_broadcaster_subscriptions(&token).try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn get_broadcaster_subscriptions<T>(
        &'a self,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<
            dyn futures::Stream<
                    Item = Result<
                        helix::subscriptions::BroadcasterSubscription,
                        ClientError<'a, C>,
                    >,
                > + 'a,
        >,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        use futures::StreamExt;

        let user_id = match token
            .user_id()
            .ok_or_else(|| ClientRequestError::Custom("no user_id found on token".into()))
        {
            Ok(t) => t,
            Err(e) => return futures::stream::once(async { Err(e) }).boxed(),
        };
        let req = helix::subscriptions::GetBroadcasterSubscriptionsRequest::broadcaster_id(user_id);
        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Get all moderators in a channel [Get Moderators](helix::moderation::GetModeratorsRequest)
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let moderators: Vec<helix::moderation::Moderator> = client.get_moderators_in_channel_from_id("twitchdev", &token).try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn get_moderators_in_channel_from_id<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<
            dyn futures::Stream<Item = Result<helix::moderation::Moderator, ClientError<'a, C>>>
                + 'a,
        >,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::moderation::GetModeratorsRequest::broadcaster_id(broadcaster_id);

        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Get all banned users in a channel [Get Banned Users](helix::moderation::GetBannedUsersRequest)
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let moderators: Vec<helix::moderation::BannedUser> = client.get_banned_users_in_channel_from_id("twitchdev", &token).try_collect().await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn get_banned_users_in_channel_from_id<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<
            dyn futures::Stream<Item = Result<helix::moderation::BannedUser, ClientError<'a, C>>>
                + 'a,
        >,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::moderation::GetBannedUsersRequest::broadcaster_id(broadcaster_id);

        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Get a users, with login, follow count
    pub async fn get_total_followers_from_login<T>(
        &'a self,
        login: impl Into<types::UserName>,
        token: &T,
    ) -> Result<Option<i64>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        if let Some(user) = self.get_user_from_login(login, token).await? {
            self.get_total_followers_from_id(user.id, token)
                .await
                .map(Some)
        } else {
            Ok(None)
        }
    }

    /// Get a users, with id, follow count
    ///
    /// # Notes
    ///
    /// This returns zero if the user doesn't exist
    pub async fn get_total_followers_from_id<T>(
        &'a self,
        to_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<i64, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let resp = self
            .req_get(
                helix::users::GetUsersFollowsRequest::followers(to_id),
                token,
            )
            .await?;

        Ok(resp.data.total)
    }

    /// Get games by ID. Can only be at max 100 ids.
    pub async fn get_games_by_id<T>(
        &'a self,
        ids: impl IntoIterator<Item = types::CategoryId>,
        token: &T,
    ) -> Result<std::collections::HashMap<types::CategoryId, helix::games::Game>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let ids: Vec<_> = ids.into_iter().take(101).collect();
        if ids.len() > 100 {
            return Err(ClientRequestError::Custom("too many IDs, max 100".into()));
        }

        let resp = self
            .req_get(helix::games::GetGamesRequest::ids(ids), token)
            .await?;

        Ok(resp
            .data
            .into_iter()
            .map(|g: helix::games::Game| (g.id.clone(), g))
            .collect())
    }

    /// Block a user
    pub async fn block_user<T>(
        &'a self,
        target_user_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<helix::users::BlockUser, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        Ok(self
            .req_put(
                helix::users::BlockUserRequest::block_user(target_user_id),
                helix::EmptyBody,
                token,
            )
            .await?
            .data)
    }

    /// Unblock a user
    pub async fn unblock_user<T>(
        &'a self,
        target_user_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<helix::users::UnblockUser, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        Ok(self
            .req_delete(
                helix::users::UnblockUserRequest::unblock_user(target_user_id),
                token,
            )
            .await?
            .data)
    }

    /// Ban a user
    pub async fn ban_user<T>(
        &'a self,
        target_user_id: impl Into<types::UserId>,
        reason: impl std::fmt::Display,
        duration: impl Into<Option<u32>>,
        broadcaster_id: impl Into<types::UserId>,
        moderator_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<helix::moderation::BanUser, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        Ok(self
            .req_post(
                helix::moderation::BanUserRequest::new(broadcaster_id, moderator_id),
                helix::moderation::BanUserBody::new(target_user_id, reason.to_string(), duration),
                token,
            )
            .await?
            .data)
    }

    /// Unban a user
    pub async fn unban_user<T>(
        &'a self,
        target_user_id: impl Into<types::UserId>,
        broadcaster_id: impl Into<types::UserId>,
        moderator_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<helix::moderation::UnbanUserResponse, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        Ok(self
            .req_delete(
                helix::moderation::UnbanUserRequest::new(
                    broadcaster_id,
                    moderator_id,
                    target_user_id,
                ),
                token,
            )
            .await?
            .data)
    }

    // FIXME: Example should use https://github.com/twitch-rs/twitch_api/issues/162
    /// Get all scheduled streams in a channel.
    ///
    /// # Notes
    ///
    /// Make sure to limit the data here using [`try_take_while`](futures::stream::TryStreamExt::try_take_while), otherwise this will never end on recurring scheduled streams.
    ///
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// use twitch_api::helix;
    /// use futures::TryStreamExt;
    ///
    /// let schedule: Vec<helix::schedule::Segment> = client
    ///     .get_channel_schedule("twitchdev", &token)
    ///     .try_take_while(|s| {
    ///         futures::future::ready(Ok(!s.start_time.as_str().starts_with("2021-10")))
    ///     })
    ///     .try_collect()
    ///     .await?;
    ///
    /// # Ok(()) }
    /// ```
    pub fn get_channel_schedule<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<dyn futures::Stream<Item = Result<helix::schedule::Segment, ClientError<'a, C>>> + 'a>,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::schedule::GetChannelStreamScheduleRequest::broadcaster_id(broadcaster_id);

        make_stream(req, token, self, |broadcasts| broadcasts.segments.into())
    }

    /// Get all global emotes
    pub async fn get_global_emotes<T>(
        &'a self,
        token: &T,
    ) -> Result<Vec<helix::chat::GlobalEmote>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::chat::GetGlobalEmotesRequest::new();
        Ok(self.req_get(req, token).await?.data)
    }

    /// Get channel emotes in channel with user id
    pub async fn get_channel_emotes_from_id<T>(
        &'a self,
        user_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<Vec<helix::chat::ChannelEmote>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::chat::GetChannelEmotesRequest::broadcaster_id(user_id);
        Ok(self.req_get(req, token).await?.data)
    }

    /// Get channel emotes in channel with user login
    pub async fn get_channel_emotes_from_login<T>(
        &'a self,
        login: impl Into<types::UserName>,
        token: &T,
    ) -> Result<Option<Vec<helix::chat::ChannelEmote>>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        if let Some(user) = self.get_user_from_login(login, token).await? {
            self.get_channel_emotes_from_id(user.id, token)
                .await
                .map(Some)
        } else {
            Ok(None)
        }
    }

    /// Get emotes in emote set
    pub async fn get_emote_sets<T>(
        &'a self,
        emote_sets: impl IntoIterator<Item = types::EmoteSetId>,
        token: &T,
    ) -> Result<Vec<helix::chat::get_emote_sets::Emote>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::chat::GetEmoteSetsRequest::emote_set_ids(emote_sets);
        Ok(self.req_get(req, token).await?.data)
    }

    /// Get a broadcaster's chat settings
    pub async fn get_chat_settings<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        moderator_id: impl Into<Option<types::UserId>>,
        token: &T,
    ) -> Result<helix::chat::ChatSettings, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let mut req = helix::chat::GetChatSettingsRequest::new(broadcaster_id);
        if let Some(moderator_id) = moderator_id.into() {
            req = req.moderator_id(moderator_id);
        }
        Ok(self.req_get(req, token).await?.data)
    }

    /// Send a chat announcement
    pub async fn send_chat_announcement<T, E>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        moderator_id: impl Into<types::UserId>,
        message: impl std::fmt::Display,
        color: impl std::convert::TryInto<helix::chat::AnnouncementColor, Error = E>,
        token: &T,
    ) -> Result<helix::chat::SendChatAnnouncementResponse, ClientExtError<'a, C, E>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::chat::SendChatAnnouncementRequest::new(broadcaster_id, moderator_id);
        let body = helix::chat::SendChatAnnouncementBody::new(message.to_string(), color)?;
        Ok(self
            .req_post(req, body, token)
            .await
            .map_err(ClientExtError::ClientError)?
            .data)
    }

    /// Delete a specific chat message
    pub async fn delete_chat_message<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        moderator_id: impl Into<types::UserId>,
        message_id: impl Into<types::MsgId>,
        token: &T,
    ) -> Result<helix::moderation::DeleteChatMessagesResponse, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::moderation::DeleteChatMessagesRequest::new(broadcaster_id, moderator_id)
            .message_id(message_id);

        Ok(self.req_delete(req, token).await?.data)
    }

    /// Delete all chat messages in a broadcasters chat room
    pub async fn delete_all_chat_message<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        moderator_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<helix::moderation::DeleteChatMessagesResponse, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::moderation::DeleteChatMessagesRequest::new(broadcaster_id, moderator_id);
        Ok(self.req_delete(req, token).await?.data)
    }

    /// Start a raid
    pub async fn start_a_raid<T>(
        &'a self,
        from_broadcaster_id: impl Into<types::UserId>,
        to_broadcaster_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<helix::raids::StartARaidResponse, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::raids::StartARaidRequest::new(from_broadcaster_id, to_broadcaster_id);
        Ok(self.req_post(req, helix::EmptyBody, token).await?.data)
    }

    /// Cancel a raid
    pub async fn cancel_a_raid<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<helix::raids::CancelARaidResponse, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::raids::CancelARaidRequest::new(broadcaster_id);
        Ok(self.req_delete(req, token).await?.data)
    }

    /// Get a users chat color
    pub async fn get_user_chat_color<T>(
        &'a self,
        user_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<Option<helix::chat::UserChatColor>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::chat::GetUserChatColorRequest {
            user_id: vec![user_id.into()],
        };

        Ok(self.req_get(req, token).await?.first())
    }

    /// Get a users chat color
    pub async fn update_user_chat_color<T>(
        &'a self,
        user_id: impl Into<types::UserId>,
        color: impl Into<types::NamedUserColor<'static>>,
        token: &T,
    ) -> Result<helix::chat::UpdateUserChatColorResponse, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::chat::UpdateUserChatColorRequest {
            user_id: user_id.into(),
            color: color.into(),
        };

        Ok(self.req_put(req, helix::EmptyBody, token).await?.data)
    }

    /// Get multiple users chat colors
    pub async fn get_users_chat_colors<T>(
        &'a self,
        user_ids: impl IntoIterator<Item = types::UserId>,
        token: &T,
    ) -> Result<Vec<helix::chat::UserChatColor>, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::chat::GetUserChatColorRequest {
            user_id: user_ids.into_iter().map(Into::into).collect(),
        };

        Ok(self.req_get(req, token).await?.data)
    }

    /// Add a channel moderator
    pub async fn add_channel_moderator<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        moderator_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<helix::moderation::AddChannelModeratorResponse, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::moderation::AddChannelModeratorRequest {
            broadcaster_id: broadcaster_id.into(),
            moderator_id: moderator_id.into(),
        };

        Ok(self.req_post(req, helix::EmptyBody, token).await?.data)
    }

    /// Remove a channel moderator
    pub async fn remove_channel_moderator<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        moderator_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<helix::moderation::RemoveChannelModeratorResponse, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::moderation::RemoveChannelModeratorRequest {
            broadcaster_id: broadcaster_id.into(),
            moderator_id: moderator_id.into(),
        };

        Ok(self.req_delete(req, token).await?.data)
    }

    /// Get channel VIPs
    pub fn get_vips_in_channel<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        token: &'a T,
    ) -> std::pin::Pin<
        Box<dyn futures::Stream<Item = Result<helix::channels::Vip, ClientError<'a, C>>> + 'a>,
    >
    where
        T: TwitchToken + Send + Sync + ?Sized,
    {
        let req = helix::channels::GetVipsRequest::new(broadcaster_id);

        make_stream(req, token, self, std::collections::VecDeque::from)
    }

    /// Add a channel vip
    pub async fn add_channel_vip<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        user_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<helix::channels::AddChannelVipResponse, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::channels::AddChannelVipRequest {
            broadcaster_id: broadcaster_id.into(),
            user_id: user_id.into(),
        };

        Ok(self.req_post(req, helix::EmptyBody, token).await?.data)
    }

    /// Remove a channel vip
    pub async fn remove_channel_vip<T>(
        &'a self,
        broadcaster_id: impl Into<types::UserId>,
        user_id: impl Into<types::UserId>,
        token: &T,
    ) -> Result<helix::channels::RemoveChannelVipResponse, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::channels::RemoveChannelVipRequest {
            broadcaster_id: broadcaster_id.into(),
            user_id: user_id.into(),
        };

        Ok(self.req_delete(req, token).await?.data)
    }

    /// Send a whisper
    pub async fn send_whisper<T>(
        &'a self,
        from: impl Into<types::UserId>,
        to: impl Into<types::UserId>,
        message: impl std::fmt::Display,
        token: &T,
    ) -> Result<helix::whispers::SendWhisperResponse, ClientError<'a, C>>
    where
        T: TwitchToken + ?Sized,
    {
        let req = helix::whispers::SendWhisperRequest::new(from, to);
        let body = helix::whispers::SendWhisperBody::new(message);

        Ok(self.req_post(req, body, token).await?.data)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClientExtError<'a, C: crate::HttpClient<'a>, E> {
    #[error(transparent)]
    ClientError(ClientError<'a, C>),
    #[error(transparent)]
    Other(#[from] E),
}

/// Make a paginate-able request into a stream
///
/// # Examples
///
/// ```rust, no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, twitch_api::client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
/// use twitch_api::helix;
/// use futures::TryStreamExt;
///
/// let req = helix::moderation::GetModeratorsRequest::broadcaster_id("1234");
///
/// helix::make_stream(req, &token, &client, std::collections::VecDeque::from).try_collect::<Vec<_>>().await?
/// # ;
/// # Ok(())
/// # }
/// ```
pub fn make_stream<
    'a,
    C: crate::HttpClient<'a> + Send + Sync,
    T: TwitchToken + ?Sized + Send + Sync,
    // FIXME: Why does this have to be clone and debug?
    Req: super::Request
        + super::RequestGet
        + super::Paginated
        + Clone
        + std::fmt::Debug
        + Send
        + Sync
        + 'a,
    // FIXME: this 'a seems suspicious
    Item: Send + 'a,
>(
    req: Req,
    token: &'a T,
    client: &'a super::HelixClient<'a, C>,
    fun: impl Fn(<Req as super::Request>::Response) -> std::collections::VecDeque<Item>
        + Send
        + Sync
        + Copy
        + 'static,
) -> std::pin::Pin<Box<dyn futures::Stream<Item = Result<Item, ClientError<'a, C>>> + 'a + Send>>
where
    // FIXME: This clone is bad. I want to be able to return the data, but not in a way that limits the response to be Default
    // I also want to keep allocations low, so std::mem::take is perfect, but that makes get_next not work optimally.
    <Req as super::Request>::Response: Send + Sync + std::fmt::Debug + Clone,
{
    use futures::StreamExt;
    enum StateMode<Req: super::Request + super::RequestGet, Item> {
        /// A request needs to be done.
        Req(Option<Req>),
        /// We have made a request, now working through the data
        Cont(
            super::Response<Req, <Req as super::Request>::Response>,
            std::collections::VecDeque<Item>,
        ),
        Next(Option<super::Response<Req, <Req as super::Request>::Response>>),
        /// The operation failed, allowing no further processing
        Failed,
    }

    impl<Req: super::Request + super::RequestGet, Item> StateMode<Req, Item> {
        fn take_initial(&mut self) -> Req {
            match self {
                StateMode::Req(ref mut r) if r.is_some() => std::mem::take(r).expect("oops"),
                _ => todo!("hmmm"),
            }
        }

        fn take_next(&mut self) -> super::Response<Req, <Req as super::Request>::Response> {
            match self {
                StateMode::Next(ref mut r) if r.is_some() => std::mem::take(r).expect("oops"),
                _ => todo!("hmmm"),
            }
        }
    }

    struct State<
        'a,
        C: crate::HttpClient<'a>,
        T: TwitchToken + ?Sized,
        Req: super::Request + super::RequestGet,
        Item,
    > {
        mode: StateMode<Req, Item>,
        client: &'a HelixClient<'a, C>,
        token: &'a T,
    }

    impl<
            'a,
            C: crate::HttpClient<'a>,
            T: TwitchToken + ?Sized,
            Req: super::Request + super::RequestGet + super::Paginated,
            Item,
        > State<'a, C, T, Req, Item>
    {
        /// Process a request, with a given deq
        fn process(
            mut self,
            r: super::Response<Req, <Req as super::Request>::Response>,
            d: std::collections::VecDeque<Item>,
        ) -> Self {
            self.mode = StateMode::Cont(r, d);
            self
        }

        fn failed(mut self) -> Self {
            self.mode = StateMode::Failed;
            self
        }

        /// get the next
        fn get_next(mut self) -> Self {
            match self.mode {
                StateMode::Cont(r, d) => {
                    assert!(d.is_empty());
                    self.mode = StateMode::Next(Some(r));
                    self
                }
                _ => panic!("oops"),
            }
        }
    }
    let statemode = StateMode::Req(Some(req));
    let state = State {
        mode: statemode,
        client,
        token,
    };
    futures::stream::unfold(state, move |mut state: State<_, _, _, _>| async move {
        match state.mode {
            StateMode::Req(Some(_)) => {
                let req = state.mode.take_initial();
                let f = state.client.req_get(req, state.token);
                let resp = match f.await {
                    Ok(resp) => resp,
                    Err(e) => return Some((Err(e), state.failed())),
                };
                let mut deq = fun(resp.data.clone());
                deq.pop_front().map(|d| (Ok(d), state.process(resp, deq)))
            }
            StateMode::Cont(_, ref mut deq) => {
                if let Some(d) = deq.pop_front() {
                    if deq.is_empty() {
                        Some((Ok(d), state.get_next()))
                    } else {
                        Some((Ok(d), state))
                    }
                } else {
                    // New request returned empty.
                    None
                }
            }
            StateMode::Next(Some(_)) => {
                let resp = state.mode.take_next();
                let f = resp.get_next(state.client, state.token);
                let resp = match f.await {
                    Ok(Some(resp)) => resp,
                    Ok(None) => return None,
                    Err(e) => return Some((Err(e), state.failed())),
                };
                let mut deq = fun(resp.data.clone());
                deq.pop_front().map(|d| (Ok(d), state.process(resp, deq)))
            }
            _ => todo!("failed to process request"),
        }
    })
    .boxed()
}
