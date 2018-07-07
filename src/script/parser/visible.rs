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
            pair!(
                opt!(
                    ws!(
                        preceded!(
                            tag!("~"),
                            quote
                        )
                    )
                ),
                opt!(
                    ws!(
                        preceded!(
                            tag!("with"),
                            quote
                        )
                    )
                )
            )
        ),
        |(image, (state, trans))| {
            let state = {
                if let Some(s) = state {
                    Some(s.to_string())
                } else {None}
            };
            let trans = {
                if let Some(t) = trans {
                    Some(t.to_string())
                } else {None}
            };
            ScriptStep::Show(image.to_string(), state, trans)
        }
    )
);

named!(pub hide(CompleteStr) -> ScriptStep,
    map!(
        preceded!(
            tag!("HIDE"),
            pair!(
                quote,
                opt!(
                    preceded!(
                        tag!("with"),
                        quote
                    )
                )
            )
        ),
        |(image, trans)| {
            let trans = {
                if let Some(t) = trans{
                    Some(t.to_string())
                } else {None}
            };
            ScriptStep::Hide(image.to_string(), trans)
        }
    )
);

#[test]
pub fn test_parser_visible() {
    println!("{:?}", show(CompleteStr("SHOW 'cat girl'")));
    println!("{:?}", show(CompleteStr("SHOW 'cat girl'~'second'")));
    println!("{:?}", hide(CompleteStr("HIDE 'cat girl'")));
}