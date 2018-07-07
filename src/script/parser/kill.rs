use nom::types::CompleteStr;
use super::{
    super::ScriptStep,
    values::quote,
};

named!(pub kill(CompleteStr) -> ScriptStep,
    map!(
        preceded!(
            tag!("KILL"),
            pair!(
                quote,
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
        |(entity, trans)| {
            let trans = {
                if let Some(t) = trans {
                    Some(t.to_string())
                } else {None}
            };
            ScriptStep::Kill(entity.to_string(), trans)
        }
    )
);

#[test]
fn parser_kill() {
    println!("{:?}", kill(CompleteStr("KILL 'character'")))
}