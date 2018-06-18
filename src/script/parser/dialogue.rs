use nom::types::CompleteStr;
use super::{
    super::ScriptStep,
    values::speech,
};

named!(pub dialogue_continue(CompleteStr) -> ScriptStep,
    map!(
        speech,
        |text|{ScriptStep::DialogueContinue(text.to_string())}
    )
);

named!(pub dialogue(CompleteStr) -> ScriptStep,
    map!(
        pair!(
            speech,
            ws!(
                preceded!(
                    tag!(":"),
                    ws!(
                        delimited!(
                            tag!("\""),
                            take_until!("\""),
                            tag!("\"")
                        )
                    )
                )
            )
        ),
        |(speaker, content)| ScriptStep::Dialogue(speaker.to_string(), content.to_string())
    )
);

#[test]
fn test_dialogue() {
    println!("{:?}", dialogue_continue(CompleteStr("\"hi there\"")));
    println!("{:?}", dialogue(CompleteStr(r#""Main Character" : "Hi there""#)))
}