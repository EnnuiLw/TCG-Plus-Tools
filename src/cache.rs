use std::collections::HashMap;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use chrono::{prelude::*, Duration};
use bytes::Bytes;


const MAX_SIZE: usize = 100;
pub static MODEL_HASH: Lazy<Mutex<HashMap<String, Holder<Bytes>>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));


pub struct Holder<T> {
    target: T,
    expire_at: DateTime<Utc>
}

impl<T> Holder<T> where T: Clone {
    pub fn new(target: T, expire_at: DateTime<Utc>) -> Self {
        Self { target, expire_at }
    }

    fn get_target(&self, now: &DateTime<Utc>) -> Option<T> {
        if self.expire_at > *now {
            return Some(self.target.clone())
        }
        
        None
    }

    async fn extract_expire_data(target: &Mutex<HashMap<String, Holder<T>>>) -> crate::Result<Option<String>> {
        let hash = target.lock().await;
        let now = Utc::now();

        for (cache_key, data) in hash.keys().zip(hash.values()) {
            if data.expire_at <= now {
                return Ok(Some(cache_key.to_string()))
            }
        }

        Ok(None)
    }

    async fn inner_get(
        key: &str,
        now: &DateTime<Utc>,
        mutex: &Mutex<HashMap<String, Holder<T>>>,
    ) -> Option<T> {
        let hash = mutex.lock().await;
        if let Some(holder) = hash.get(key) {
            return holder.get_target(now);
        } 
        
        None
    }

    async fn inner_set(
        key: &str,
        target: T,
        mutex: &Mutex<HashMap<String, Holder<T>>>,
        expire_at: DateTime<Utc>
    ) -> crate::Result<()> {
        let mut hash = mutex.lock().await;
        if hash.capacity() > MAX_SIZE {
            Self::extract_expire_data(mutex)
                .await?
                .map(|k| hash.remove(&k));
        }

        hash.insert(key.to_owned(), Holder::new(target, expire_at));

        Ok(())
    }

    pub async fn get(
        key: &str,
        mutex: &Mutex<HashMap<String, Holder<T>>>,
    ) -> Option<T> {
        let now = Utc::now();
        if let Some(target) = Self::inner_get(key, &now, &mutex).await {
            return Some(target);
        }

        None
    }

    pub async fn set<Fut>(
        key: &str,
        mutex: &Mutex<HashMap<String, Holder<T>>>,
        cache_expire_second_count: u64,
        f: impl FnOnce() -> Fut,
    ) -> crate::Result<()>
    where
        Fut: Future<Output = crate::Result<T>>,
    {
        let now = Utc::now();
        if let Ok(target) = f().await {
            Self::inner_set(
                key, 
                target, 
                mutex, 
                now + Duration::seconds(cache_expire_second_count as i64)
            ).await?;
        }

        Ok(())
    }
}
