use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Success<T> {
    /// Status Code
    pub code: i64,
    /// Event Information
    pub event: T,
    pub count_applicants: i64,
    pub within_two_hours_events: Vec<String>,
    pub event_change_log: Vec<String>,
    pub counts_of_applied_events_this_month: i64,
    pub fair_play_status: i64,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct PartialEvent {
    /// Event ID
    pub id: u64,
    /// The title of the event
    #[serde(rename = "event_series_title")]
    pub title: String,
    /// Event series Id
    pub event_series_id: String,
    pub series_header_img_url: String,
    pub event_series_type: i64,
    pub organizer_id: u64,
    pub organizer_name: String,
    pub organizer_url: String,
    pub organizer_sns_url: String,
    /// Prefecture code
    pub pref_code: String,
    /// Prefecture name
    pub pref_name: String,
    pub postcode: String,
    pub city_address: String,
    pub street_address: String,
    /// Start date/time of this event
    pub start_datetime: String,
    /// important notices for participation
    pub user_excerpt: String,
    /// Participation capacity
    pub max_join_count: i64,
    /// team capacity
    pub team_count: i64,
    /// Probably it means the start time for participation
    pub apply_start_datetime: String,
    /// Display the application period
    pub display_apply_period: String,
    pub event_application_face_recognition_setting: i64,
    /// the country code of the event location
    pub country_code: String,
    /// entry requirements
    #[serde(rename = "participation_condition_by_admin")]
    pub participation_condition: String,
    /// The ID of the game
    pub game_title_id: String,
    /// The title of the game 
    pub game_title: String,
    pub cancelable_by_user: i64,
    pub appliable_by_user: i64,
    /// Whether it needs face recognition
    pub checkin_face_recognition_setting: i64,
    /// Whether the Winners will be selected on a first-come, first-served basis
    pub is_arriving_first: bool,    
    /// Whether the event has been canceled
    pub is_canceled: bool,
    /// Whether the capacity is hidden
    pub is_capacity_hidden: bool,
    /// whether it's over capacity
    pub is_over_max_join_count: bool,
    /// Whether the event is a championship
    pub is_area_championship: bool,
    /// 
    pub is_select_pref_for_points: bool,
    /// 
    pub is_top_tournaments_right: bool,
    /// 
    pub no_use_tcg_plus: bool,
    /// 
    pub is_moala_face_guide_display: bool,
    /// 
    pub japan_only_flag: bool,
    /// 
    pub is_cancelable_by_user: bool,
    /// 
    pub is_appliable_by_user: bool,
    /// 
    pub is_judge_user_disable_event: bool,
    /// 
    pub count_waiting_cancel: i64,
    /// 
    pub count_can_apply: i64,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct FullEvent {
     /// Event ID
    pub id: u64,
    /// The title of the event
    #[serde(rename = "event_series_title")]
    pub title: String,
    /// Event series Id
    pub event_series_id: String,
    /// Start date of this event page for publish
    #[serde(rename = "webReleaseStartDate")]
    pub web_release_start_date: String,
    pub series_header_img_url: String,
    pub event_series_type: i64,
    pub organizer_id: u64,
    pub organizer_name: String,
    pub organizer_url: String,
    pub organizer_sns_url: String,
    pub organizer_type: i64,
    pub online_flag: String,
    /// Prefecture code
    pub pref_code: String,
    /// Prefecture name
    pub pref_name: String,
    pub postcode: String,
    pub city_address: String,
    pub street_address: String,
    /// x,y coordinates using Google Maps Platform
    pub place_geo: super::PlaceGeo,
    /// Start date/time of this event
    pub start_datetime: String,
    pub timezone: String,
    /// important notices for participation
    pub user_excerpt: String,
    pub entry_type: i64,
    pub entry_fee: String,
    pub entry_fee_currency_code: String,
    /// Participation capacity
    pub max_join_count: i64,
    /// team capacity
    pub team_count: i64,
    /// ?
    pub status_id: i64,
    /// ?
    pub flgs: i64,
    /// Probably it means the start time for participation
    pub apply_start_datetime: String,
    /// ?
    #[serde(rename = "eventSeriesFlgs")]
    pub event_series_flgs: String,
    /// Display the application period
    pub display_apply_period: String,
    
    /// user post questionnaire due date
    pub user_post_questionnaire_due_date: String,
    /// ?
    pub event_application_face_recognition_setting: i64,
    /// the country code of the event location
    pub country_code: String,
    /// entry requirements
    #[serde(rename = "participation_condition_by_admin")]
    pub participation_condition: String,
    /// The ID of the game
    pub game_title_id: String,
    /// The title of the game 
    pub game_title: String,
    pub cancelable_by_user: i64,
    pub appliable_by_user: i64,
    /// ?
    pub game_kind_id: i64,
    /// Whether it needs face recognition
    pub checkin_face_recognition_setting: i64,
    /// ?
    pub event_format: i64,
    /// ?
    pub is_two_hour_exclusion_enabled: bool,
    /// IDK
    #[serde(rename = "eventSeriesExFlgs")]
    pub event_series_ex_flgs: String,
    /// Whether the Winners will be selected on a first-come, first-served basis
    pub is_arriving_first: bool,    
    /// Whether the event has been canceled
    pub is_canceled: bool,
    /// Whether the capacity is hidden
    pub is_capacity_hidden: bool,
    /// ?
    pub is_after_lottery: bool,
    /// ?
    pub hidden_entry_fee: bool,
    /// whether it's over capacity
    pub is_over_max_join_count: bool,
    /// Whether the event is a championship
    pub is_area_championship: bool,
    /// 
    pub is_select_pref_for_points: bool,
    /// 
    pub is_top_tournaments_right: bool,
    /// 
    pub no_use_tcg_plus: bool,
    /// 
    pub is_moala_face_guide_display: bool,
    /// 
    pub japan_only_flag: bool,
    /// 
    pub is_cancelable_by_user: bool,
    /// 
    pub is_appliable_by_user: bool,
    /// 
    pub is_judge_user_disable_event: bool,
    /// 
    pub count_waiting_cancel: i64,
    /// 
    pub count_can_apply: i64,
    /// 
    pub is_penalty: bool,
    /// 
    pub game_format_ids: Vec<i64>
}
