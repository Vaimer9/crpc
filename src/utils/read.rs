use crate::utils::presence;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

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

pub fn get_data(Path: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut data = String::new();
    let mut file = File::open(Path)?;
    file.read_to_string(&mut data)?;
    Ok(data)
}


pub fn get_json(data: String) ->  Result<Formatted, Box<dyn std::error::Error>>{
    let data: Formatted = serde_json::from_str(&data)?;
    Ok(data)
}
