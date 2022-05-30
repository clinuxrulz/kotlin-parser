use crate::kotlin;

#[test]
fn test_kotlin_simple_identifier() {
    assert!(kotlin::SimpleIdentifierParser::new().parse("abstract").is_ok());
}

#[test]
fn test_kotlin_import() {
    assert!(kotlin::ImportHeaderParser::new().parse("import java.lang.String;").is_ok());
    assert!(kotlin::ImportHeaderParser::new().parse("import java.lang.*;").is_ok());
    assert!(kotlin::ImportHeaderParser::new().parse("import java.lang.String as JString;").is_ok());
}

#[test]
fn test_kotlin_import_list() {
    assert!(
        kotlin::ImportListParser::new()
            .parse(r#"
                import java.lang.Object;
                import java.lang.String as JString;
                import javax.swing.*;
            "#)
            .is_ok()
    );
}
