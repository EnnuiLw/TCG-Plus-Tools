#[cfg(test)]
mod tests {
    use anyhow::Result;
    use tcg_plus_tools::{
        county::Country, 
        Client, 
        EventTrait,
        events::{EventUtilTrait, SearchConditions}, 
    };

    const URL: &str = "https://www.bandai-tcg-plus.com/event/3679885";


    #[tokio::test]
    async fn get_event_info() -> Result<()> {
        let client = Client::default();
        let event = client.get_event_info(URL).await?;

        let title = "スタンダードバトル（6月開催）";
        assert_eq!(title, &event.title);

        let street_addr = "神奈川県横浜市戸塚区品濃町548-12";
        assert_eq!(street_addr, event.detail_location().1);
        
        Ok(())
    }

    #[tokio::test]
    async fn event_list_work() {
        let client = Client::default();
        let search = SearchConditions::from_country(Country::JP(None));
        let is_ok = client
            .get_event_list_info(search)
            .await
            .is_ok();
        assert!(is_ok);
    }
}
