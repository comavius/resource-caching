use crate::custom_resource::compiled::Compiled;
use std::collections::HashMap;
use tokio::sync::{Mutex, RwLock};
use std::sync::Arc;

pub struct Cache {
    path: String,
    cache: Mutex<HashMap<String, Arc<RwLock<Compiled>>>>,
}

impl Cache {
    pub fn new(dir_path: &str) -> Cache {
        Cache {
            path: dir_path.to_string(),
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub async fn instantiate_and_get(&self, external_id: &str, code: &str) -> Result<Arc<RwLock<Compiled>>, std::io::Error> {
        // If key already exists, return corresponding value.
        let path = format!("{}/{}", self.path, external_id);
        let mut cache_content;
        loop {
            let result = self.cache.try_lock();
            match result {
                Ok(content) => {
                    cache_content = content;
                    break;
                }
                Err(_) => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
        let wrapped_resource_to_instantiate: Arc<RwLock<Compiled>> = Arc::new(RwLock::new(Compiled::new(path.as_str()).await?));
        if cache_content.contains_key(external_id) {
            return Ok(cache_content[external_id].clone());
        }
        else {
            cache_content.insert(external_id.to_string(), wrapped_resource_to_instantiate.clone());
        }
        // Get write lock of resource's RwLock before dropping lock of cache's Mutex.
        let resource_to_instantiate = (*wrapped_resource_to_instantiate).try_write().unwrap();
        std::mem::drop(cache_content);
        // Instantiate and drop
        resource_to_instantiate.initialize(code).await?;
        std::mem::drop(resource_to_instantiate);
        Ok(wrapped_resource_to_instantiate)
    }
}
