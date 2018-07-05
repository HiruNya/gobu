use super::{
    ScriptStep,
    values::quote,
};
use nom::types::CompleteStr;

named!(pub play(CompleteStr) -> ScriptStep,
    map!(
        preceded!(
            tag!("PLAY"),
            ws!(
                quote
            )
        ),
        |name| ScriptStep::Play(name.to_string())
    )
);