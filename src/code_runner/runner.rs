use crate::code_runner::cache::Cache;

pub struct Runner {
    cache: Cache,
}

impl Runner {
    pub fn new(path: String) -> Runner {
        Runner {
            cache: Cache::new(path.as_str()),
        }
    }

    pub async fn run(&self, id: String, code: String) -> Result<String, std::io::Error> {
        let wrapped_compiled_resource = self.cache.instantiate_and_get(id.as_str(), code.as_str()).await?;
        println!("Got compiled resource");
        // Try to get read lock in every 1 second
        let compiled_resource;
        loop {
            println!("Trying to get read lock");
            let result = wrapped_compiled_resource.try_read();
            match result {
                Ok(resource) => {
                    compiled_resource = resource;
                    break;
                }
                Err(_) => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
        println!("Got read lock");
        return Ok(compiled_resource.run().await?);
    }
}