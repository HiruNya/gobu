use std::{
    path::Path,
    fs::File,
    io::Read,
    collections::{
        HashMap,
        HashSet,
    },
};
use toml::from_str;
use piston_window::{
    Key,
    MouseButton,
    Button,
};
use super::super::super::{
    input::GameInput,
    error::ConfigImportError,
};

pub fn load_input_from_file<P: AsRef<Path>>(path: P)
    -> Result<GameInput, ConfigImportError> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;
    load_input_from_str(&buffer)
}

pub fn load_input_from_str(text: &str)
    -> Result<GameInput, ConfigImportError> {
    let map: HashMap<String, Vec<InputFromFile>> = from_str(text)?;
    let mut cont = None;
    for (k, v) in map.iter() {
        match k.to_lowercase().as_str() {
            "continue" => {
                let list: HashSet<Button> = v
                    .iter()
                    .map(|item| item.to_button())
                    .collect();
                cont = Some(list);
            },
            _ => {},
        }
    }
    Ok(GameInput {
        continue_: cont.unwrap_or(HashSet::new()).clone()
    })
}

#[test]
fn test_input_from_str() {
    println!("{:?}", load_input_from_str(r#"
        Continue = [
            "Space",
            "Left",
        ]
    "#))
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum InputFromFile {
    Mouse(MouseButton),
    Keyboard(Key),
}

impl InputFromFile {
    fn to_button(&self) -> Button {
        match *self {
            InputFromFile::Keyboard(key) => Button::Keyboard(key),
            InputFromFile::Mouse(key) => Button::Mouse(key),
        }
    }
}