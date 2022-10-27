//! Create a single scheduled broadcast or a recurring scheduled broadcast for a channel’s [stream schedule](https://help.twitch.tv/s/article/channel-page-setup#Schedule).
//! [`create-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#create-channel-stream-schedule-segment)
//!
//! # Accessing the endpoint
//!
//! ## Request: [CreateChannelStreamScheduleSegmentRequest]
//!
//! To use this endpoint, construct a [`CreateChannelStreamScheduleSegmentRequest`] with the [`CreateChannelStreamScheduleSegmentRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::schedule::create_channel_stream_schedule_segment;
//! let request =
//!     create_channel_stream_schedule_segment::CreateChannelStreamScheduleSegmentRequest::builder(
//!     )
//!     .broadcaster_id("141981764")
//!     .build();
//! ```
//!
//! ## Body: [CreateChannelStreamScheduleSegmentBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use std::convert::TryFrom;
//! # use twitch_api::helix::schedule::create_channel_stream_schedule_segment;
//! let body =
//!     create_channel_stream_schedule_segment::CreateChannelStreamScheduleSegmentBody::builder()
//!         .start_time(twitch_api::types::Timestamp::try_from("2021-07-01T18:00:00Z").unwrap())
//!         .timezone("America/New_York")
//!         .is_recurring(false)
//!         .duration("60".to_string())
//!         .category_id(Some("509670".into()))
//!         .title("TwitchDev Monthly Update // July 1, 2021".to_string())
//!         .build();
//! ```
//!
//! ## Response: [ScheduledBroadcasts]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, schedule::create_channel_stream_schedule_segment};
//! # use twitch_api::client;
//! # use std::convert::TryFrom;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = create_channel_stream_schedule_segment::CreateChannelStreamScheduleSegmentRequest::builder()
//!     .broadcaster_id("141981764")
//!     .build();
//! let body = create_channel_stream_schedule_segment::CreateChannelStreamScheduleSegmentBody::builder()
//!     .start_time(twitch_api::types::Timestamp::try_from("2021-07-01T18:00:00Z")?)
//!     .timezone("America/New_York")
//!     .is_recurring(false)
//!     .duration("60".to_string())
//!     .category_id(Some("509670".into()))
//!     .title("TwitchDev Monthly Update // July 1, 2021".to_string())
//!     .build();
//! let response: create_channel_stream_schedule_segment::CreateChannelStreamScheduleSegmentResponse = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&body, &token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`CreateChannelStreamScheduleSegmentRequest::parse_response(None, &request.get_uri(), response)`](CreateChannelStreamScheduleSegmentRequest::parse_response)

use super::*;
use helix::RequestPost;
/// Query Parameters for [Create Channel Stream Schedule Segment](super::create_channel_stream_schedule_segment)
///
/// [`create-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#create-channel-stream-schedule-segment)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct CreateChannelStreamScheduleSegmentRequest<'a> {
    /// User ID of the broadcaster who owns the channel streaming schedule. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[serde(borrow)]
    pub broadcaster_id: &'a types::UserIdRef,
}

impl<'a> CreateChannelStreamScheduleSegmentRequest<'a> {
    /// Create a single scheduled broadcast or a recurring scheduled broadcast for a channel’s [stream schedule](https://help.twitch.tv/s/article/channel-page-setup#Schedule).
    pub fn broadcaster_id(broadcaster_id: impl Into<&'a types::UserIdRef>) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into(),
        }
    }
}

/// Body Parameters for [Create Channel Stream Schedule Segment](super::create_channel_stream_schedule_segment)
///
/// [`create-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#create-channel-stream-schedule-segment)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct CreateChannelStreamScheduleSegmentBody<'a> {
    /// Start time for the scheduled broadcast specified in RFC3339 format.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[serde(borrow)]
    pub start_time: &'a types::TimestampRef,
    // FIXME: specific braid?
    /// The timezone of the application creating the scheduled broadcast using the IANA time zone database format.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[serde(borrow)]
    pub timezone: &'a str,
    /// Indicates if the scheduled broadcast is recurring weekly.
    pub is_recurring: bool,
    /// Duration of the scheduled broadcast in minutes from the start_time. Default: 240.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none", borrow)]
    pub duration: Option<&'a str>,
    /// Game/Category ID for the scheduled broadcast.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none", borrow)]
    pub category_id: Option<&'a types::CategoryIdRef>,
    /// Title for the scheduled broadcast. Maximum: 140 characters.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[serde(skip_serializing_if = "Option::is_none", borrow)]
    pub title: Option<&'a str>,
}

impl<'a> CreateChannelStreamScheduleSegmentBody<'a> {
    /// Create a single scheduled broadcast or a recurring scheduled broadcast for a channel’s [stream schedule](https://help.twitch.tv/s/article/channel-page-setup#Schedule).
    pub fn new(
        start_time: impl Into<&'a types::TimestampRef>,
        timezone: impl Into<&'a str>,
        is_recurring: bool,
    ) -> Self {
        Self {
            start_time: start_time.into(),
            timezone: timezone.into(),
            is_recurring,
            duration: Default::default(),
            category_id: Default::default(),
            title: Default::default(),
        }
    }
}

impl helix::private::SealedSerialize for CreateChannelStreamScheduleSegmentBody<'_> {}

/// Return Values for [Create Channel Stream Schedule Segment](super::create_channel_stream_schedule_segment)
///
/// [`create-channel-stream-schedule-segment`](https://dev.twitch.tv/docs/api/reference#create-channel-stream-schedule-segment)
pub type CreateChannelStreamScheduleSegmentResponse = ScheduledBroadcasts;

impl Request for CreateChannelStreamScheduleSegmentRequest<'_> {
    type Response = CreateChannelStreamScheduleSegmentResponse;

    const PATH: &'static str = "schedule/segment";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelManageSchedule];
}

impl<'a> RequestPost for CreateChannelStreamScheduleSegmentRequest<'a> {
    type Body = CreateChannelStreamScheduleSegmentBody<'a>;
}

#[cfg(test)]
#[test]
fn test_request() {
    use std::convert::TryFrom;

    use helix::*;
    let req = CreateChannelStreamScheduleSegmentRequest::broadcaster_id("141981764");

    let ts = types::Timestamp::try_from("2021-07-01T18:00:00Z").unwrap();
    let body = CreateChannelStreamScheduleSegmentBody {
        duration: Some("60"),
        category_id: Some("509670".into()),
        title: Some("TwitchDev Monthly Update // July 1, 2021"),
        ..CreateChannelStreamScheduleSegmentBody::new(&*ts, "America/New_York", false)
    };

    dbg!(req.create_request(body, "token", "clientid").unwrap());

    // From twitch docs
    let data = br#"
    {
        "data": {
          "segments": [
            {
              "id": "eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0=",
              "start_time": "2021-07-01T18:00:00Z",
              "end_time": "2021-07-01T19:00:00Z",
              "title": "TwitchDev Monthly Update // July 1, 2021",
              "canceled_until": null,
              "category": {
                  "id": "509670",
                  "name": "Science & Technology"
              },
              "is_recurring": false
            }
          ],
          "broadcaster_id": "141981764",
          "broadcaster_name": "TwitchDev",
          "broadcaster_login": "twitchdev",
          "vacation": null
        }
      }
    "#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/schedule/segment?broadcaster_id=141981764"
    );

    dbg!(
        CreateChannelStreamScheduleSegmentRequest::parse_response(Some(req), &uri, http_response)
            .unwrap()
    );
}
