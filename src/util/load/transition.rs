use toml::from_str;
use std::{
    fs::File,
    io::Read,
    path::Path,
    collections::HashMap,
};
use super::super::super::{
    animation::{
        Animation,
        CharacterTransition,
    },
    error::ConfigImportError,
};

/// Load transitions from a TOML file
pub fn load_transitions_from_file<P: AsRef<Path>>(path: P) -> Result<Animation, ConfigImportError> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;
    load_transitions_from_str(&buffer)
}

/// Load transitions from a TOML file
pub fn load_transitions_from_str(text: &str) -> Result<Animation, ConfigImportError> {
    let anim: AnimationFromFile = from_str(text)?;
    let char_trans = {
        let mut trans_map = HashMap::new();
        if let Some(ct) = anim.chara_trans {
            for (k, v) in ct.iter() {
                trans_map.insert(k.to_string(), v.to_transition());
            }
        }
        trans_map
    };
    Ok(Animation {
        char_trans,
    })
}

#[derive(Deserialize)]
struct AnimationFromFile {
    #[serde(rename = "CharacterTransition")]
    chara_trans: Option<HashMap<String, Transition>>
}

#[derive(Deserialize)]
#[serde(tag = "type", content = "time")]
enum Transition {
    FadeIn(f32),
    FadeOut(f32),
}
impl Transition {
    fn to_transition(&self) -> Box<dyn CharacterTransition> {
        use self::Transition::*;
        use super::super::super::animation::premade::{
            FadeIn,
            FadeOut,
        };
        match *self {
            FadeIn(t) => FadeIn::new(t),
            FadeOut(t) => FadeOut::new(t),
        }
    }
}