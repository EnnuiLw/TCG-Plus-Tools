pub mod clients;
pub mod models;
pub mod county;
pub mod error;
mod cache;

// Use for the Cache system.
// Max count is `100`, expired at 24h.
pub(crate) use cache::{Holder, MODEL_HASH};

pub use clients::Client;
pub use clients::events;
pub use clients::URL;
pub use error::Error;
pub use crate::clients::events::{EventTrait, EventUtilTrait};
pub use crate::clients::CaptureMode;
pub use crate::county::Country;

pub type Result<T> = std::result::Result<T, Error>;

pub mod events_prelude {
    pub use crate::models::event_models as event;
    pub use crate::models::event_list_models as event_list;
    pub(crate) use event::EventData;
    pub(crate) use event_list::EventListData;
}
