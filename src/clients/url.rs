use std::str::FromStr;
use regex::Regex;
use once_cell::sync::Lazy;


#[allow(unused)]
pub(crate) const BASE: &str = "https://www.bandai-tcg-plus.com";
pub(crate) const BASE_API: &str = "https://api.bandai-tcg-plus.com/api/user";

pub(crate) const EVENT: &str = "event";
pub(crate) const EVENT_LIST: &str = "event/list";

#[allow(unused)]
pub(crate) const SERIES: &str = "series";
// pub(crate) const SERIES_API: &str = "eventseries";

// const EVENT_REGEX: &str = r"^https://www\.bandai-tcg-plus\.com/event/event(?:/(\d+))?";
// const EVENT_LIST_REGEX: &str = r"^https://www\.bandai-tcg-plus\.com/event\?";
const PARAMS_REGEX: &str = r"([a-zA-Z0-9_\[\]]+)=([^&]*)";

#[allow(unused)]
const API_EVENT_REGEX: &str = r"^https://api\.bandai-tcg-plus\.com/api/user/event/(\d{6,8})";
#[allow(unused)]
const API_EVENT_LIST_REGEX: &str = r"^https://api\.bandai-tcg-plus\.com/api/user/event/list\?";

const EVENT_API_BOTH_REGEX: &str = r"^https://(?:www|api)\.bandai-tcg-plus\.com(?:/api/user)?/event(?:/(?<id>\d+))?$";
// const EVENT_API_BOTH_REGEX: &str = r"^https://(?:www|api)\.bandai-tcg-plus\.com(?:/api/user)?/event/(\d*)";
const EVENT_LIST_API_BOTH_REGEX: &str = r"^https://(?:www|api)\.bandai-tcg-plus\.com(?:/api/user)?/event/list\?([a-zA-Z0-9_\[\]]+)=([^&]*)";


// const ALL_MATCH: &str = r"^https://(?:www|api)\.bandai-tcg-plus\.com(?:(?:/event(?:/\d+)?)(?:\?(?P<params1>[A-Za-z0-9_\[\]%=&\-]*))?|(?:/api/user/event/(?:list|\d+))(?:\?(?P<params2>[A-Za-z0-9_\[\]%=&\-]*))?)$";
// static RE_EVENT: Lazy<Regex> = Lazy::new(|| Regex::new(EVENT_REGEX).unwrap());
// static RE_EVENT_LIST: Lazy<Regex> = Lazy::new(|| Regex::new(EVENT_LIST_REGEX).unwrap());
static RE_PARAMS: Lazy<Regex> = Lazy::new(|| Regex::new(PARAMS_REGEX).unwrap());

#[allow(unused)]
static RE_API_EVENT: Lazy<Regex> = Lazy::new(|| Regex::new(API_EVENT_REGEX).unwrap());
#[allow(unused)]
static RE_API_EVENT_LIST: Lazy<Regex> = Lazy::new(|| Regex::new(API_EVENT_LIST_REGEX).unwrap());

static RE_SUPER_EVENT: Lazy<Regex> = Lazy::new(|| Regex::new(EVENT_API_BOTH_REGEX).unwrap());
static RE_SUPER_EVENT_LIST: Lazy<Regex> = Lazy::new(|| Regex::new(EVENT_LIST_API_BOTH_REGEX).unwrap());

// static RE_COUNTRY: Lazy<Regex> = Lazy::new(|| Regex::new(r"country_code\[\]=([^&]*)").unwrap());
// static RE_GAME_ID: Lazy<Regex> = Lazy::new(|| Regex::new(r"game_title_id=([^&]*)").unwrap());
// static RE_LIMIT: Lazy<Regex> = Lazy::new(|| Regex::new(r"limit=([^&]*)").unwrap());
// static RE_DATE: Lazy<Regex> = Lazy::new(|| Regex::new(r"start_date=([^&]*)").unwrap());
// static RE_PREF: Lazy<Regex> = Lazy::new(|| Regex::new(r"params_pref\[\]=([^&]*)").unwrap());


pub struct URL<'a> {
    material: &'a str,
    capture: CaptureMode
}

impl<'a> URL<'a> {
    pub fn capture(&self) -> &CaptureMode {
        &self.capture
    }

    pub fn new(material: &'a str) -> crate::Result<Self> {
        let capture = CaptureMode::from_str(material)?;

        Ok(Self {
            material,
            capture,
        })
    }

    pub fn get_material(&self) -> &'a str {
        self.material
    }

    pub fn get_api(&self) -> String {
        match &self.capture {
            CaptureMode::Detail(id) => {
                let id = if !id.eq("")  { format!("/{id}")  } else { String::new() };
                format!("{BASE_API}/{EVENT}{id}")
            }
            CaptureMode::List(params) => 
                format!("{BASE_API}/{EVENT_LIST}?{}", params.join("&"))
        }
    }
}


/// return the event ID (ex: 3679885) from url
/// 
/// like:
/// let url = "https://www.bandai-tcg-plus.com/event/3679885";
/// let id = CaptureMode::from_str(url).unwrap();
/// 
///   
pub enum CaptureMode {
    Detail(String),
    List(Vec<String>),
}

impl std::str::FromStr for CaptureMode {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cap) = RE_SUPER_EVENT.captures(s) {
            let id = if let Some(id) = cap.name("id") { id.as_str().to_string() } else { String::new() };
            return Ok(Self::Detail(id));
        }

        if RE_SUPER_EVENT_LIST.is_match(s) {
            let params = RE_PARAMS.captures_iter(s)
                .map(|c| format!("{}={}", c[1].to_string(), c[2].to_string()))
                .collect::<Vec<_>>();
            return Ok(Self::List(params));
        }
    
        Err(crate::Error::InvalidURL)
    }
}

pub fn gen_cache_key(capture: &CaptureMode) -> String {
    match capture {
            CaptureMode::Detail(id) => 
                id.to_string(),
            CaptureMode::List(params) => 
                format!("{EVENT_LIST}?{}", params.join("&"))
        }
}
