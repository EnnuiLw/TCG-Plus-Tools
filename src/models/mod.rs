pub mod event_list;
pub mod event;
pub mod series;


pub mod event_models {
    pub(crate) use super::EventData;
    pub use super::event::{
        Success,
        FullEvent,
        PartialEvent
    };
}

pub mod event_list_models {
    pub(crate) use super::EventListData;
    pub use super::event_list::{
        Success,
        FullEvent,
        PartialEvent,
    };
}

pub(crate) type EventData<T> = AbstractEventData<event::Success<T>>;
pub(crate) type EventListData<T> = AbstractEventData<event_list::Success<T>>;


use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct AbstractEventData<S> {
    pub success: S
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceGeo {
    pub x: f64,
    pub y: f64,
}
