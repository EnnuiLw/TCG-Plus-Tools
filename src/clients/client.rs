use bytes::Bytes;
use serde::de::DeserializeOwned;

use crate::{MODEL_HASH, Holder};

// 24h
const CACHE_EXPIRE_TIME: u64 = 86400; 


#[derive(Default)]
pub struct Client(pub(crate) reqwest::Client);

impl Client {
    pub async fn connect(
        &self, 
        endpoint: &str,
    ) -> crate::Result<reqwest::Response> {
        Ok(self.0
            .get(endpoint)
            .send()
            .await
            .map_err(|e| crate::Error::RequestError(e))?)
    } 

    async fn _connect(
        &self,
        endpoint: &str,
        cache_key: Option<&str>
    ) -> crate::Result<Bytes> {
        if let Some(cache_key) = cache_key {
            if let Some(data) = Holder::get(cache_key, &MODEL_HASH).await {
                return Ok(data.clone());
            }

            let req = self
                .connect(endpoint)
                .await?
                .bytes()
                .await?;

            Holder::set(
                cache_key, 
                &MODEL_HASH, 
                CACHE_EXPIRE_TIME, 
                || async { Ok(req.clone()) })
                .await?;
        }

        let req = self
            .connect(endpoint)
            .await?
            .bytes()
            .await?;
        
        Ok(req)
    }

    pub async fn connect_with_json<T>(
        &self, 
        endpoint: &str, 
        cache_key: Option<&str>
    ) -> crate::Result<T> 
    where 
        T: DeserializeOwned 
    {
        let req = self._connect(endpoint, cache_key).await?;
        Ok(serde_json::from_slice(&req)
            .map_err(|e| crate::Error::SerdeError(e))?)
    }
}
