//! Parses script files and turns them into [`ScriptStep`]s

mod dialogue;
mod visible;
mod spawn;
mod values;
mod kill;
mod move_p;
mod stage;
mod end;
mod goto;

use indexmap::IndexMap;
use nom::types::CompleteStr;
use self::{
    dialogue::{dialogue, dialogue_continue},
    visible::{show, hide},
    spawn::spawn,
    kill::kill,
    move_p::move_p,
    stage::stage,
    end::end,
    goto::goto,
};
use super::ScriptStep;
use super::super::error::ScriptImportError;

named!(translate_to_step(CompleteStr) -> Vec<ScriptStep>,
    many0!(
        ws!(
            alt_complete!(
                dialogue
                | dialogue_continue
                | show | hide
                | spawn | kill
                | move_p
                | stage
                | end
                | goto
            )
        )
    )
);

named!(translate_script(CompleteStr) -> IndexMap<String, Vec<ScriptStep>>,
    map!(
        many0!(
            ws!(
                pair!(
                    opt!(
                        preceded!(
                            tag!(":"),
                            take_until_either!(" \r\n")
                        )
                    ),
                    terminated!(
                        translate_to_step,
                        peek!(
                            alt_complete!(
                                tag!(":")
                                | eof!()
                            )
                        )
                    )
                )
            )
        ),
        |list| {
            list.iter()
                .fold(IndexMap::new(), |mut list, (k, v)| {
                    let name = k.unwrap_or(CompleteStr("main"));
                    list.insert(name.to_string(), v.clone());
                    list
                })
        }
    )
);

/// Translates a [`str`] into a [`Vec`] of [`ScriptSteps`].
/// Returns an error if it fails.
pub fn translate(text: &str)
    -> Result<IndexMap<String, Vec<ScriptStep>>, ScriptImportError> {
    match translate_script(CompleteStr(text)) {
        Ok((_, map)) => Ok(map),
        Err(e) => Err(ScriptImportError::Nom(e.into_error_kind())),
    }
}

#[test]
pub fn try_translate() {
    println!("{:?}", translate_script(CompleteStr(r#"
        "wow"
        "Much wow indeed"
        :anchor2
        "That's a different anchor"
        "I'm impressed"
    "#)));
}