pub mod event;
pub mod series;

pub use event::EventTrait;
pub use event::EventUtilTrait;
use chrono::{DateTime, Utc};
use crate::Country;
use crate::clients::url::{BASE_API, EVENT_LIST};

pub const LIMIT: u32 = 20;


pub struct SearchConditions {
    // pub event_type: EventType,
    pub date: DateTime<Utc>,
    pub country: Country,
    pub max_size: Option<u32>
}

#[derive(Default)]
#[non_exhaustive]
pub enum EventType {
    #[default]
    Standard,
    FlagShip,
    FivePack,
}


impl SearchConditions {
    pub fn new(
        country: Country,
        // event_type: Option<EventType>,
        date: Option<DateTime<Utc>>,
        max_size: Option<u32>
    ) -> Self {
        Self {
            // event_type: event_type.unwrap_or_default(),
            date: date.unwrap_or_default(),
            country,
            max_size
        }
    }

    pub fn from_country(country: Country) -> Self {
        Self {
            country,
            // event_type: EventType::default(),
            date: Utc::now(),
            max_size: None
        }
    }

    pub fn to_url(&self) -> String {
        let max_size = self.max_size.unwrap_or(LIMIT).min(LIMIT);

        format!("{BASE_API}/{EVENT_LIST}?\
            application_open_flg=0&\
            country_code[]={}&\
            favorite=0&\
            game_title_id=8&\
            limit={max_size}&\
            offset=0&\
            order=1&\
            series_type[]=1&\
            start_date={}&\
            params_pref[]={}",
            self.country.to_string(),
            self.date.format("%Y-%m-%d"),
            self.country.subnational_division_code())
    }
}
