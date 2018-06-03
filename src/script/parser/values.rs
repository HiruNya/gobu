use nom::types::CompleteStr;
use std::str::FromStr;

named!(pub quote(CompleteStr) -> CompleteStr,
    ws!(
        delimited!(
            tag!("'"),
            take_until!("'"),
            tag!("'")
        )
    )
);

named!(pub speech(CompleteStr) -> CompleteStr,
    delimited!(
        tag!("\""),
        take_until!("\""),
        tag!("\"")
    )
);

named!(pub pos(CompleteStr) -> (f64, f64),
    ws!(
        delimited!(
            tag!("("),
            pair!(
                ws!(
                    map!(
                        take_until!(","),
                        |num| f64::from_str(&num).unwrap_or(0.)
                    )
                ),
                ws!(
                    preceded!(
                        tag!(","),
                        ws!(
                            map!(
                                take_until!(")"),
                                |num| f64::from_str(&num).unwrap_or(0.)
                            )
                        )
                    )
                )
            ),
            tag!(")")
        )
    )
);

#[test]
fn parser_value_pos() {
    println!("{:?}", pos(CompleteStr("(5, 2)")))
}