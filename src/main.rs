mod utils;

fn start_cli() -> std::io::Result<()> {
    let args = utils::args::Cli::from_args();
    
    match args.commands {
        Some(Manual {file} ) => {
            let data = get_json(get_local_data(file)?)?;
            let pres = utils::presence::Presence::from_json(&data)?;
            utils::presence::set_presence(&pres)?;
        }

        Some(Default) => {
            let data = get_json(get_data()?)?;
            let pres = utils::presence::Presence::from_json(&data)?;
            utils::presence::set_presence(&pres)?;
        }

        Some(Init) => {
            utils::presence::install()?;
        }
    }
}

fn main() {
    // Empty for now
}
