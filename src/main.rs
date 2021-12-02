mod utils;
use structopt::StructOpt;
use crate::utils::read::{get_json, get_local_data, install};
use crate::utils::args::Commands;

fn start_cli() -> std::io::Result<()> {
    let args = utils::args::Args::from_args();
    
    match args.commands {
        Some(Commands::Manual {file} ) => {
            let data = get_json(get_local_data(&file)?)?;
            let pres = utils::presence::Data::from_json(data);
            utils::presence::start(pres).unwrap();
        }

        Some(Commands::Default) | None => {
            let data = utils::read::get_json(utils::read::get_data()?)?;
            let pres = utils::presence::Data::from_json(data);
            utils::presence::start(pres).unwrap();
        }

        Some(Commands::Init) => {
            utils::read::install()?;
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    start_cli()?;
    Ok(())
}
