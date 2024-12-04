use crate::setup_project;

use std::{fs, io};
use std::path::Path;

use crate::args;

pub fn new_project(commands: args::NewCommands) -> io::Result<()>{
    let project_name: String = commands.name;
    println!("making project {}", project_name);

    let project_dir = Path::new(&project_name); 

    if project_dir.exists() {
        eprintln!("Project directory already exists! consider renaming or moving to another folder!");
        return Ok(());
    }
    fs::create_dir(project_dir)?;
   
    setup_project::setup_project(project_dir)
}
