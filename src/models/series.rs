use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct FullEventSeries {
    pub code: i64,
    pub event_series_id: i64,
    pub event_series_title: String,
    pub user_excerpt: String,
    pub apply_start_date: String,
    pub apply_end_date: String,
    pub online_flag: i64,
    pub game_title_id: i64,
    pub display_apply_period: String,
    pub files: Vec<File>,
    pub terms: Vec<Term>,
    pub series_type: i64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialEventSeries {
    pub event_series_id: i64,
    pub event_series_title: String,
    pub user_excerpt: String,
    pub apply_start_date: String,
    pub apply_end_date: String,
    pub game_title_id: i64,
    pub display_apply_period: String,
    pub files: Vec<File>,
    pub terms: Vec<Term>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub id: i64,
    pub file_type: i64,
    pub file_name: String,
    pub file_url: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Term {
    pub apply_start_date: String,
    pub apply_end_date: String
}
