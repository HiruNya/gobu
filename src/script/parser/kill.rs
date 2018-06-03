use nom::types::CompleteStr;
use super::{
    super::ScriptStep,
    values::quote,
};

named!(pub kill(CompleteStr) -> ScriptStep,
    map!(
        preceded!(
            tag!("KILL"),
            ws!(
                quote
            )
        ),
        |entity| ScriptStep::Kill(entity.to_string())
    )
);

#[test]
fn parser_kill() {
    println!("{:?}", kill(CompleteStr("KILL 'character'")))
}