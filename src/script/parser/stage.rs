use nom::types::CompleteStr;
use super::{
    super::ScriptStep,
    values::quote,
};

named!(pub stage(CompleteStr) -> ScriptStep,
    map!(
        preceded!(
            tag!("STAGE"),
            ws!(
                quote
            )
        ),
        |bg| ScriptStep::Stage(bg.to_string())
    )
);

#[test]
fn test_parser_stage() {
    println!("{:?}", stage(CompleteStr(r#"STAGE 'background2'"#)))
}