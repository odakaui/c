use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "counter")]
#[clap(about = "A simple project counter.", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(arg_required_else_help = true, name = "add")]
    Add {
        #[clap(value_parser)]
        project_name: String,
    },

    #[clap(arg_required_else_help = true, name = "count")]
    Count {
        #[clap(value_parser)]
        project_name: String,
    },

    #[clap(arg_required_else_help = true, name = "list")]
    List {
        #[clap(value_parser)]
        project_name: String,
    },

    #[clap(name = "list-projects")]
    ListProjects,

    #[clap(arg_required_else_help = true, name = "rm")]
    Remove {
        #[clap(value_parser)]
        hash: String,
    },
}
