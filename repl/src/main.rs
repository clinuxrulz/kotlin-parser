use kotlin_parser::kotlin;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    loop {
        std::io::stdin().read_line(&mut buffer)?;
        if buffer == "quit" {
            break;
        }
    }
    Ok(())
}
