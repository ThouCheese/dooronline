use std::fs::File;
use std::io::prelude::*;

fn minify_css(mut source_file: String) -> std::io::Result<()> {
    let mut content = String::new();
    {
        let mut beautiful = File::open(&source_file)?;
        beautiful.read_to_string(&mut content)?;
    }
    content = content.replace("    ", "");
    content = content.replace("\n", "");
    content = content.replace(": ", ":");
    let new_length = source_file.len() - 3;
    source_file.truncate(new_length);
    let mut small = File::create(source_file + "min.css")?;
    small.write_all((&content).as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    minify_css(String::from("static/style.css"))
}
