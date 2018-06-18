use nom::types::CompleteStr;
use super::super::ScriptStep;

named!(pub end(CompleteStr) -> ScriptStep,
    map!(
        tag!("END"),
        |_| ScriptStep::End
    )
);