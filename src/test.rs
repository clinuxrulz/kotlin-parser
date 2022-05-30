use crate::kotlin;

#[test]
fn test_kotlin_simple_identifier() {
    println!("{:?}", kotlin::SimpleIdentifierParser::new().parse("abstract"));
}

#[test]
fn test_kotlin_import() {
    println!("{:?}", kotlin::ImportHeaderParser::new().parse("import java.lang.String;"));
    println!("{:?}", kotlin::ImportHeaderParser::new().parse("import java.lang.*;"));
    println!("{:?}", kotlin::ImportHeaderParser::new().parse("import java.lang.String as JString;"));
}

#[test]
fn test_kotlin_import_list() {
    println!("{:?}",
        kotlin::ImportListParser::new().parse(
            r#"
                import java.lang.Object;
                import java.lang.String as JString;
                import javax.swing.*;
            "#
        )
    );
}
