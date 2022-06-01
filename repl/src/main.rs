use kotlin_parser::kotlin;
use std::io::Write;

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    let parser = kotlin::SimpleUserTypeParser::new();
    loop {
        print!("> ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut buffer)?;
        trim_newline(&mut buffer);
        if buffer == "quit" || buffer == "exit" {
            break;
        }
        println!("{:?}", parser.parse(&buffer));
        buffer.clear();
    }
    Ok(())
}
