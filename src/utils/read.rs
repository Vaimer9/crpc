use serde::{Deserialize, Serialize};
use directories::ProjectDirs;

use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::{Read, Write};

pub const JSON: &str = r#"
{
	id: "",
	status: "",
	details: "",
	large: false,
	smalle: false,
	want_buttons: false,
	large_image: "",
	small_image: "",
	large_tool: "",
	small_tool: "",
	buttons: {
		name: "",
		link: "",
		name2: "",
		link2: ""
	}
}
"#;

#[derive(Serialize, Deserialize)]
pub struct Formatted {
    pub id: String,
    pub status: String,
    pub details: String,
    pub large: bool,
    pub small: bool,
    pub want_buttons: bool,
    pub button_numbers: u8,
    pub large_image: String,
    pub small_image: String,
    pub large_tool: String,
    pub small_tool: String,
    pub buttons: Vec<String> 
}

pub fn install() -> std::io::Result<()> {
    if let Some(x) = ProjectDirs::from("crpc", "Vaimer9", "crpc") {
        match Path::new(x.config_dir()).is_dir() {
            true => {
                fs::create_dir_all(x.config_dir())?;
                let mut file = File::create(x.config_dir().join("config.json"))?;
                file.write_all(JSON.as_bytes())?;
            }
            false => {
                println!("You have already Initialized! No need to run again")
            }
        }
    }

    Ok(())
}

pub fn get_data() -> std::io::Result<String> {
    let mut data = String::new();
    if let Some(x) = ProjectDirs::from("crpc", "Vaimer9", "crpc") {
        match Path::new(x.config_dir()).is_dir() {
            true => {
                let mut file = File::open(Path::new(x.config_dir()).join("config.json"))?;
                file.read_to_string(&mut data)?;
            }
            false => println!("Please run `crpc init`, as you have not Initialized the application!")
        }
    }
    Ok(data)
}

pub fn get_local_data(path: &Path) -> std::io::Result<String> {
    let mut data = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut data);
    Ok(data)
}

pub fn get_json(data: String) ->  std::io::Result<Formatted>{
    let data: Formatted = serde_json::from_str(&data)?;
    Ok(data)
}


