mod args;
mod build;
mod config;
mod init;
mod new;
mod setup_project;

use args::FiskurArgs;
use clap::Parser;

fn main() {
    let args = FiskurArgs::parse();

    match args.command {
        args::Cmd::New(commans) => {
            let res = new::new_project(commans);
            match res {
                Err(error) => {
                    eprintln!(
                        "something went wrong whilest making your project! {:?}",
                        error
                    );
                }
                Ok(_file) => {}
            }
        }
        args::Cmd::Init => {
            let res = init::init();
            match res {
                Err(error) => {
                    eprintln!(
                        "something went wrong whilest initializing project {:?}",
                        error
                    );
                }
                Ok(_file) => {}
            }
        }
        args::Cmd::Build => {
            let res = build::build();
            match res {
                Err(error) => {
                    eprintln!("Error whilest building! {:?}", error)
                }
                Ok(_) => {}
            }
        }
    }
}
