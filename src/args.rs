use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct FiskurArgs{
    #[clap(subcommand)]
    pub command: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// build the current project
    Build,

    /// creates a project inside the current folder
    Init,

    /// make a new project
    New(NewCommands)
}

#[derive(Debug, Args)]
pub struct NewCommands {
    /// the name of the project
    pub name: String,
}

