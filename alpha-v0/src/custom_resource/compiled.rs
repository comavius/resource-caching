use std::fs::File;
use std::io::{prelude::*, ErrorKind};

pub struct Compiled {
    dir_path: Option<String>
}

impl Compiled {
    pub fn new() -> Compiled {
        Compiled {
            dir_path: None,
        }
    }

    pub async fn initialize(&mut self, code: &str, path: &str) -> Result<(), std::io::Error> {
        println!("Initializing compiled resource at path: {}", path);
        self.dir_path = Some(path.to_string());
        println!("Allocating resources");
        self.allocate(code).await?;
        println!("Compiling resources");
        self.compile().await?; 
        println!("Completed initialization");
        Ok(())
    }

    pub async fn run(&self) -> Result<String, std::io::Error> {
        match &self.dir_path {
            None => return Err(std::io::Error::new(ErrorKind::Other, "Path not set")),
            Some(path) => {
                let output = std::process::Command::new(format!("{}/src", path))
                    .output()?;
                println!("Completed running at path: {}", path);
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            }
        }
    }
    
    async fn allocate(&self, code: &str) -> Result<(), std::io::Error> {
        match &self.dir_path {
            None => return Err(std::io::Error::new(ErrorKind::Other, "Path not set")),
            Some(path) => {
                std::fs::create_dir(path)?;
                let mut src = File::create(format!("{}/src.cpp", path))?;
                let bytes = code.as_bytes();
                src.write_all(bytes)?;
                Ok(())
            }
        }
    }

    async fn compile(&self) -> Result<(), std::io::Error> {
        match &self.dir_path {
            None => return Err(std::io::Error::new(ErrorKind::Other, "Path not set")),
            Some(path) => {
                let compile_str = format!("g++ {}/src.cpp -o {}/src", path, path);
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(compile_str)
                    .output()?;
                if !output.status.success() {
                    return Err(std::io::Error::new(
                        ErrorKind::Other,
                        format!(
                            "Failed to compile code: {}",
                            String::from_utf8_lossy(&output.stderr)
                        ),
                    ));
                }
                Ok(())
            },
        }
    }
}

impl Drop for Compiled {
    fn drop(&mut self) {
        match &self.dir_path {
            None => (),
            Some(path) => {
                std::fs::remove_dir_all(path).unwrap();
            }
        }
    }
}