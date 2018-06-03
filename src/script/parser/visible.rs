use nom::types::CompleteStr;
use super::{
    super::ScriptStep,
    values::quote,
};

named!(pub show(CompleteStr) -> ScriptStep,
    map!(
        preceded!(
            tag!("SHOW"),
            quote
        ),
        |image| ScriptStep::Show(image.to_string())
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
pub fn visible() {
    println!("{:?}", show(CompleteStr("SHOW 'cat girl'")));
    println!("{:?}", hide(CompleteStr("HIDE 'cat girl'")));
}