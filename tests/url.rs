#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use anyhow::Result;
    use tcg_plus_tools::URL;

    const URL: &str = "https://www.bandai-tcg-plus.com/event/3679885";
    const EVENT_LIST_URL: &str = "https://api.bandai-tcg-plus.com/api/user/event/list?application_open_flg=0&country_code[]=JP&favorite=0&game_title_id=8&limit=20&offset=0&order=1&series_type[]=1&start_date=2025-07-22&params_pref[]=";

    #[tokio::test]
    async fn regex_works() {
        let all_event = "https://www.bandai-tcg-plus.com/event";
        let invalid_url = "https://www.google.com/";
        assert!(URL::new(URL).is_ok());
        assert!(URL::new(all_event).is_ok());
        assert!(URL::new(invalid_url).is_err());
    }
    
    #[tokio::test]
    async fn url_to_api_url_works() {
        let correct_event_api_url = "https://api.bandai-tcg-plus.com/api/user/event/3679885";
        assert_eq!(
            correct_event_api_url, 
            URL::new(URL).unwrap().get_api()
        );
    }


    #[tokio::test]
    async fn event_url_regex_works() -> Result<()> {
        use tcg_plus_tools::CaptureMode;

        // Event ID regex checker
        let event_id = "3679885";
        let CaptureMode::Detail(id) = CaptureMode::from_str(URL)? else { panic!("Internal Error") };
        assert_eq!(event_id, id);

        // Event List `Param` regex checker
        let params = vec![
            "application_open_flg=0",
            "country_code[]=JP",
            "favorite=0",
            "game_title_id=8",
            "limit=20",
            "offset=0",
            "order=1",
            "series_type[]=1",
            "start_date=2025-07-22",
            "params_pref[]="
        ];
        let CaptureMode::List(params_2) = CaptureMode::from_str(EVENT_LIST_URL)? else { panic!("Internal Error") };

        for (param, ref param_from_url) in params.iter().zip(params_2) {
            assert_eq!(param, param_from_url);
        }

        Ok(())

    }

}
