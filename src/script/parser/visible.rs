use nom::types::CompleteStr;
use super::{
    super::ScriptStep,
    values::quote,
};

named!(pub show(CompleteStr) -> ScriptStep,
    map!(
        pair!(
            preceded!(
                tag!("SHOW"),
                quote
            ),
            opt!(
                ws!(
                    preceded!(
                        tag!("~"),
                        quote
                    )
                )
            )
        ),
        |(image, state)| {
            let state = {
                if let Some(s) = state {
                    Some(s.to_string())
                } else {None}
            };
            ScriptStep::Show(image.to_string(), state)
        }
    )
);

named!(pub hide(CompleteStr) -> ScriptStep,
    map!(
        preceded!(
            tag!("HIDE"),
            quote
        ),
        |image| ScriptStep::Hide(image.to_string())
    )
);

#[test]
pub fn test_parser_visible() {
    println!("{:?}", show(CompleteStr("SHOW 'cat girl'")));
    println!("{:?}", show(CompleteStr("SHOW 'cat girl'~'second'")));
    println!("{:?}", hide(CompleteStr("HIDE 'cat girl'")));
}