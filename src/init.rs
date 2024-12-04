use std::{env, io};

use crate::setup_project;

pub fn init() -> io::Result<()> {
    println!("initinializing project!");

    let project_dir = env::current_dir()?;

    setup_project::setup_project(&project_dir)
}
