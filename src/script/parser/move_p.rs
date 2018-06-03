use super::{
    ScriptStep,
    values::{quote, pos},
};
use nom::types::CompleteStr;

named!(pub move_p(CompleteStr) -> ScriptStep,
    map!(
        preceded!(
            tag!("MOVE"),
            pair!(
                ws!(
                    quote
                ),
                pos
            )
        ),
        |(name, pos)| ScriptStep::Move(name.to_string(), pos)
    )
);

#[test]
fn parser_move() {
    println!("{:?}", move_p(CompleteStr("MOVE 'character' ( 5. , 3. )")))
}