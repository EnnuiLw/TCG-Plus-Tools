use serde::de::DeserializeOwned;
use crate::{events_prelude::*, URL};

#[async_trait::async_trait]
pub trait EventTrait {
    async fn inner_connect<T>(
        &self, 
        id: &str
    ) -> crate::Result<T> where T: DeserializeOwned;

    async fn get_event_info(
        &self, 
        id: &str
    ) -> crate::Result<event::PartialEvent> {
        Ok(self
            .inner_connect::<EventData<event::PartialEvent>>(id)
            .await?
            .success
            .event)
    }

    async fn get_event_full_info(
        &self, 
        id: &str
    ) -> crate::Result<event::FullEvent> {
        Ok(self
            .inner_connect::<EventData<event::FullEvent>>(id)
            .await?
            .success
            .event)
    }

    async fn get_event_list_info(
        &self, 
        search: super::SearchConditions
    ) -> crate::Result<Vec<event_list::PartialEvent>> {
        let url = search.to_url();
        Ok(self
            .inner_connect::<EventListData<event_list::PartialEvent>>(&url)
            .await?
            .success
            .event_list)
    }

    async fn get_event_list_full_info(
        &self, 
        search: super::SearchConditions
    ) -> crate::Result<Vec<event_list::FullEvent>> {
        let url = search.to_url();
        Ok(self
            .inner_connect::<EventListData<event_list::FullEvent>>(&url)
            .await?
            .success
            .event_list)
    }
}


#[async_trait::async_trait]
impl EventTrait for crate::Client {
    async fn inner_connect<T>(&self, id: &str) -> crate::Result<T>
    where
        T: DeserializeOwned
    {
        let url = URL::new(id)?;
        let cache_key = crate::clients::url::gen_cache_key(url.capture());
        Ok(self
            .connect_with_json::<T>(&url.get_api(), Some(&cache_key))
            .await?)
    }
}


pub trait EventUtilTrait {
    /// Return Detail Address
    fn detail_location(&self) -> (String, String);
}


impl EventUtilTrait for event::FullEvent {
    fn detail_location(&self) -> (String, String) {
        (self.postcode.to_string(),
        format!("{}{}{}", 
            self.pref_name.to_string(),
            self.city_address.to_string(),
            self.street_address.to_string())
        )
    }
}

impl EventUtilTrait for event::PartialEvent {
    fn detail_location(&self) -> (String, String) {
        (self.postcode.to_string(),
        format!("{}{}{}", 
            self.pref_name.to_string(),
            self.city_address.to_string(),
            self.street_address.to_string())
        )
    }
}
