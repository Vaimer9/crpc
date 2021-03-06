use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "crpc", about = "Discord Rich Presence CLI made in Rust")]
pub struct Args {
    #[structopt(subcommand)]
    pub commands: Option<Commands>
} 


#[derive(StructOpt, Debug)]
pub enum Commands {
    #[structopt(about = "Set Presence using the json file specified")]
    Manual {
        #[structopt(short = "f")]
        file: PathBuf 
    },
    
    #[structopt(about = "Set Presence using the json file in home directory")]
    Default,

    #[structopt(about = "Create required folder and files, only need to run once")]
    Init
}
