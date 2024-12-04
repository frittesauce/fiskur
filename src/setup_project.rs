use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn setup_project(project_dir: &Path) ->  io::Result<()>{

    let project_name = project_dir.file_name().unwrap();

    let config_content = 
format!(r#"[config]
name = "{:?}"
version = "0.1.0"
edition = "2024"
src = "src/src/"
main = "notmain"
"#, project_name);
    let config_path = project_dir.join("fiskur.toml");
    let mut config_file = File::create(config_path)?;
    config_file.write_all(config_content.as_bytes())?;    
    
    let src_dir = project_dir.join("src");
    fs::create_dir(&src_dir)?;

    let kty_content = 
r#"fn main() {
    chirp("hello world!");
}
"#;
    let kty_path = src_dir.join("main.kty");
    let mut kty_file = File::create(kty_path)?;
    kty_file.write_all(kty_content.as_bytes())?;

    println!("finished creating project!");
    
    Ok(())
}
