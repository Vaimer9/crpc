use serde::{Deserialize, Serialize};
use directories::ProjectDirs;

use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::{Read, Write};
use std::collections::HashMap;

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
    id: String,
    status: String,
    details: String,
    large: bool,
    small: bool,
    want_buttons: bool,
    large_image: String,
    small_image: String,
    large_tool: String,
    small_tool: String,
    buttons: HashMap<String, String>
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

pub fn get_json(data: String) ->  std::io::Result<Formatted>{
    let data: Formatted = serde_json::from_str(&data)?;
    Ok(data)
}
