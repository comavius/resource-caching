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
        let mut cache_content = self.cache.lock().await;
        let must_instantiate = !cache_content.contains_key(external_id);
        if must_instantiate {
            cache_content.insert(
                external_id.to_string(),
                Arc::new(RwLock::new(Compiled::new()))
            );
            let cloned_resource_ref = cache_content[external_id].clone();
            let mut write_locked_resource = cloned_resource_ref.try_write().unwrap();
            std::mem::drop(cache_content);
            write_locked_resource.initialize(code, path.as_str()).await?;
            let cache_content = self.cache.lock().await;
            let cloned_resource_ref = cache_content[external_id].clone();
            return Ok(cloned_resource_ref)
        }
        else {
            let cloned_resource_ref = cache_content[external_id].clone();
            return Ok(cloned_resource_ref);
        }
    }

    pub async fn clear(&self) {
        let mut cache_content = self.cache.lock().await;
        cache_content.clear();
    }
}
