use nom::{
    alphanumeric,
    types::CompleteStr,
};
use super::super::ScriptStep;

named!(pub goto(CompleteStr) -> ScriptStep,
    map!(
        preceded!(
            tag!("->"),
            ws!(
                separated_pair!(
                    opt!(
                        alphanumeric
                    ),
                    tag!(":"),
                    opt!(
                        alphanumeric
                    )
                )
            )
        ),
        |(name, anchor)| {
            let name = {
                if let Some(n) = name {
                    Some(n.to_string())
                } else { None }
            };
            let anchor = {
                if let Some(a) = anchor {
                    Some(a.to_string())
                } else { None }
            };
            ScriptStep::GoTo(name, anchor)
        }
    )
);

#[test]
fn test_parser_goto() {
    println!("{:?}", goto(CompleteStr(r#"-> name:anchor"#)));
    println!("{:?}", goto(CompleteStr(r#"-> name:"#)));
    println!("{:?}", goto(CompleteStr(r#"-> :anchor"#)));
}