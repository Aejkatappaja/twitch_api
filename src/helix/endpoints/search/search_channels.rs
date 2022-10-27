//! Returns a list of channels (users who have streamed within the past 6 months) that match the query via channel name or description either entirely or partially.
//! [`search-channels`](https://dev.twitch.tv/docs/api/reference#search-channels)
//!
//! # Accessing the endpoint
//!
//! ## Request: [SearchChannelsRequest]
//!
//! To use this endpoint, construct a [`SearchChannelsRequest`] with the [`SearchChannelsRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::search::search_channels;
//! let request = search_channels::SearchChannelsRequest::builder()
//!     .query("hello")
//!     .build();
//! ```
//!
//! ## Response: [Channel]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, search::search_channels};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = search_channels::SearchChannelsRequest::builder()
//!     .query("hello")
//!     .build();
//! let response: Vec<search_channels::Channel> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`SearchChannelsRequest::parse_response(None, &request.get_uri(), response)`](SearchChannelsRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Search Channels](super::search_channels)
///
/// [`search-channels`](https://dev.twitch.tv/docs/api/reference#search-channels)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct SearchChannelsRequest<'a> {
    /// URL encoded search query
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[serde(borrow)]
    pub query: &'a str,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub after: Option<helix::Cursor>,
    /// Maximum number of objects to return. Maximum: 100 Default: 20
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    // FIXME: No setter because int
    pub first: Option<usize>,
    /// Filter results for live streams only. Default: false
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub live_only: Option<bool>,
}

impl<'a> SearchChannelsRequest<'a> {
    /// Search channels with the following query.
    pub fn query(query: impl Into<&'a str>) -> Self {
        Self {
            query: query.into(),
            after: None,
            first: None,
            live_only: None,
        }
    }

    /// Get live streams only
    pub fn live_only(mut self, live_only: bool) -> Self {
        self.live_only = Some(live_only);
        self
    }

    /// Set amount of results returned per page.
    pub fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }
}

/// Return Values for [Search Channels](super::search_channels)
///
/// [`search-channels`](https://dev.twitch.tv/docs/api/reference#search-channels)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Channel {
    /// ID of the game being played on the stream
    pub game_id: types::CategoryId,
    /// Name of the game being played on the stream.
    pub game_name: String,
    /// Channel ID
    pub id: types::UserId,
    /// Display name corresponding to user_id
    pub display_name: types::DisplayName,
    /// Channel language (Broadcaster Language field from the [Channels service][crate::helix::channels])
    pub broadcaster_language: String,
    /// Login of the broadcaster.
    pub broadcaster_login: types::UserName,
    /// channel title
    pub title: String,
    /// Thumbnail URL of the stream. All image URLs have variable width and height. You can replace {width} and {height} with any values to get that size image.
    pub thumbnail_url: String,
    /// Live status
    pub is_live: bool,
    /// UTC timestamp. (live only)
    #[serde(
        default,
        deserialize_with = "helix::deserialize_none_from_empty_string"
    )]
    pub started_at: Option<types::Timestamp>,
    // FIXME: Twitch doc say tag_ids
    /// Shows tag IDs that apply to the stream (live only).See <https://www.twitch.tv/directory/all/tags> for tag types
    pub tag_ids: Vec<types::TagId>,
}

impl Request for SearchChannelsRequest<'_> {
    type Response = Vec<Channel>;

    const PATH: &'static str = "search/channels";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for SearchChannelsRequest<'_> {}

impl helix::Paginated for SearchChannelsRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = SearchChannelsRequest::query("fort");

    // From twitch docs
    let data = br#"
    {
        "data": [
          {
            "broadcaster_language": "en",
            "broadcaster_login": "a_seagull",
            "display_name": "A_Seagull",
            "game_id": "506442",
            "game_name": "DOOM Eternal",
            "id": "19070311",
            "is_live": true,
            "tag_ids": [
              "6ea6bca4-4712-4ab9-a906-e3336a9d8039"
            ],
            "thumbnail_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/a_seagull-profile_image-4d2d235688c7dc66-300x300.png",
            "title": "a_seagull",
            "started_at": "2020-03-18T17:56:00Z"
          }
        ],
        "pagination": {}
      }
"#
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/search/channels?query=fort"
    );

    dbg!(SearchChannelsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}
