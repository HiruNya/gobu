pub mod dialogue;
pub mod visible;
pub mod spawn;
pub mod values;
pub mod kill;
pub mod move_p;

use nom::types::CompleteStr;
use self::{
    dialogue::{dialogue, dialogue_continue},
    visible::{show, hide},
    spawn::spawn,
    kill::kill,
    move_p::move_p,
};
use super::ScriptStep;

named!(translate_to_step(CompleteStr) -> Vec<ScriptStep>,
    many0!(
        ws!(
            alt_complete!(
                dialogue
                | dialogue_continue
                | show | hide
                | spawn | kill
                | move_p
            )
        )
    )
);

pub fn translate(text: &str) -> Result<Vec<ScriptStep>, String> {
    match translate_to_step(CompleteStr(text)) {
        Result::Ok((_, list)) => Ok(list),
        Result::Err(e) => Err(e.to_string()),
    }
}

#[test]
pub fn try_translate() {
    println!("{:?}", translate(r#" "Test" "Wow it works" "Main Character" : "You're right, it does work!""#))
}