use std::fs::File;
use std::io::{prelude::*, ErrorKind};

pub struct Compiled {
    dir_path: String,
}

impl Compiled {
    pub async fn new(dir_path: &str) -> Result<Compiled, std::io::Error> {
        let instance = Compiled {
            dir_path: dir_path.to_string(),
        };
        Ok(instance)
    }

    pub async fn initialize(&self, code: &str) -> Result<(), std::io::Error> {
        self.allocate(code).await?;
        self.compile().await?;
        println!("Compiled successfully at path: {}", self.dir_path);
        Ok(())
    }

    pub async fn run(&self) -> Result<String, std::io::Error> {
        println!("Start running at path: {}", self.dir_path);
        let output = std::process::Command::new(format!("{}/src", self.dir_path))
            .output()?;
        println!("Completed running at path: {}", self.dir_path);
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    async fn allocate(&self, code: &str) -> Result<(), std::io::Error> {
        std::fs::create_dir(self.dir_path.to_owned())?;
        let path = format!("{}/src.cpp", self.dir_path);
        let mut src = File::create(&path)?;
        let bytes = code.as_bytes();
        src.write_all(bytes)?;
        Ok(())
    }

    async fn compile(&self) -> Result<(), std::io::Error> {
        let compile_str = format!("g++ {}/src.cpp -o {}/src", self.dir_path, self.dir_path);
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
    }
}
