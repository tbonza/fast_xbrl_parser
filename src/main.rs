use std::io;
use std::io::{Read,Write};

fn main() -> io::Result<()> {
    let mut buffer = "".to_string();

    io::stdin().read_to_string(&mut buffer)?;
    io::stdout().write_all(buffer.as_bytes())?;

    Ok(())
}
