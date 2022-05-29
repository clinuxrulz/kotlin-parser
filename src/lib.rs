#[macro_use] extern crate lalrpop_util;

pub mod ast;

#[cfg(test)]
mod test;

lalrpop_mod!(pub kotlin); // synthesized by LALRPOP

#[test]
fn kotlin1() {
    assert!(kotlin::TermParser::new().parse("22").is_ok());
    assert!(kotlin::TermParser::new().parse("(22)").is_ok());
    assert!(kotlin::TermParser::new().parse("((((22))))").is_ok());
    assert!(kotlin::TermParser::new().parse("((22)").is_err());
    //
    println!("{:?}", kotlin::IdentifierParser::new().parse("part1.part2.part3"));
}

