use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "crpc", about = "Discord Rich Presence CLI made in Rust")]
pub enum Args {
    set {
        #[structopt(short = "s")]
        status: String,

        #[structopt(short = "d")]
        details: String,

        #[structopt(short = "li")]
        large_image: String,

        #[structopt(short = "si")]
        small_image: String,
        
        #[structopt(short = "lit")]
        large_image_text: String,

        #[structopt(short = "sit")]
        small_image_text: String,

    },

    set_dir {
        #[structopt(short = "f")]
        file: PathBuf
    },

    set_conf 
}
