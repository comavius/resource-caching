pub mod custom_resource;
pub mod code_runner;
use std::sync::Arc;
use crate::code_runner::runner::Runner;
use std::env;

#[tokio::main]
async fn main() {
    // get cache path from args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide cache path as an argument");
    }
    let path = args[1].to_owned();
    // exec config
    let code = "#include <bits/stdc++.h>\nint main() { sleep(10) ;std::cout << \"Executed successfully!!\" << std::endl; return 0; }";
    let execute_count = 10;
    // Add to cache asynchronously
    let runner = Arc::new(Runner::new(path));
    let identifier = uuid::Uuid::new_v4().to_string();
    let mut handles = Vec::new();
    for i in 0..execute_count {
        let cloned_runner = runner.clone();
        let identifier = identifier.clone();
        let handle = tokio::spawn(async move {
            println!("Starting async job yield in loop: {}", i);
            let runner = &(*cloned_runner);
            let result = runner.run(identifier, code.to_owned()).await;
            match result {
                Ok(output) => println!("Output: {}", output),
                Err(e) => println!("Error: {}", e),
            }
        });
        handles.push(handle);
    }
    let cache_clear_handle = tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        println!("Clearing cache");
        runner.clear_cache().await;
    });
    for handle in handles {
        handle.await.unwrap();
    }
    cache_clear_handle.await.unwrap();
}
