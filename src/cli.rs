use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "tgit", author, version)]
#[command(about = "A simply git ui, writing with Rust.", long_about = None)]
pub struct Args {
    /// work root directory
    pub work_dir: Option<String>,
}

impl Args {
    pub fn path_parser(&self) -> anyhow::Result<&Path> {
        match &self.work_dir {
            Some(s) => {
                let path = Path::new(s);
                match path.try_exists() {
                    Ok(b) => if b {
                        if path.is_dir() {
                            return Ok(path);
                        } else {
                            return Err(anyhow::anyhow!("'{}' is not a directory", s));
                        }
                    } else {
                        return Err(anyhow::anyhow!("can not access:{}", s));
                    },

                    Err(e) => Err(anyhow::anyhow!("{}", e)),
                }
            },

            // default
            None => Ok(Path::new("./")),
        }
    }
}
