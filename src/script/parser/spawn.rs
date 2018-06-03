use nom::types::CompleteStr;
use super::{
    super::ScriptStep,
    values::{quote, pos},
};

named!(pub spawn(CompleteStr) -> ScriptStep,
    map!(
        pair!(
            preceded!(
                tag!("SPAWN"),
                quote
            ),
            many0!(
                alt_complete!(
                    map!(
                        preceded!(
                            tag!("at"),
                            pos
                        ),
                        |(x, y)| {
                            Args::Pos(x, y)
                        }
                    )
                    | map!(
                        preceded!(
                            tag!("as"),
                            quote
                        ),
                        |name| Args::Name(name.to_string())
                    )
                )
            )
        ),
        |(character, list)| {
            let mut name = None;
            let mut pos = None;
            for item in list {
                match item {
                    Args::Name(n) => {
                        name = Some(n.to_string())
                    },
                    Args::Pos(x, y) => {
                        pos = Some((x, y))
                    }
                }
            }
            ScriptStep::Spawn(character.to_string(), name, pos)
        }
    )
);

enum Args {
    Name(String),
    Pos(f64, f64),
}

#[test]
fn parser_spawn() {
    println!("{:?}", spawn(CompleteStr("SPAWN 'Entity' as 'Death' at (4.0, 6.0) at (1., .2)")))
}