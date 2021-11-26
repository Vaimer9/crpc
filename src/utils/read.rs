use crate::utils::presence;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Formatted {
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

pub fn get_data(Path: PathBuf) -> Result<String> {
    let mut data = String::new();
    let mut file = File::open(Path)?;
    file.read_to_string(&mut data)?;
    Ok(data)
}


pub fn get_json(data: String) ->  Result<Formatted>{
    let data: Formatted = serde_json::from_str(&data)?;
    Ok(data)
}

pub fn give_example()  {
    let example = r#"
    {
        id: "",
        status: "",
        details: "",
        large: false,
        small: false,
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

    match File::create("./example.json") {
        Ok(mut e) => e.write_all(data.as_bytes()).expect("Can't write to file"),
        Err(_) => {
            let mut x = File::create("./data_example.json").expect("File already named example.json and dat_example.json");
            x.write_all(data.as_bytes()).expect("Can't write to file");
        }
    }
}
