//! Endpoints regarding EventSub

use crate::{helix, types};
use serde::{Deserialize, Serialize};

pub mod create_eventsub_subscription;

#[doc(inline)]
pub use create_eventsub_subscription::{
    CreateEventSubSubscription, CreateEventSubSubscriptionBody, CreateEventSubSubscriptionRequest,
};
