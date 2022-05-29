#[macro_use] extern crate lalrpop_util;

pub mod ast;

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

#[test]
fn test_kotlin_import() {
    println!("{:?}", kotlin::ImportHeaderParser::new().parse("import java.lang.String;"));
    println!("{:?}", kotlin::ImportHeaderParser::new().parse("import java.lang.*;"));
    println!("{:?}", kotlin::ImportHeaderParser::new().parse("import java.lang.String as JString;"));
}
