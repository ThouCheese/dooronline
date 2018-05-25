use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn minify_css(source_file: &Path) -> std::io::Result<()> {
    let mut content = String::new();
    {
        let mut beautiful = File::open(source_file.to_str().unwrap())?;
        beautiful.read_to_string(&mut content)?;
    }
    content = content.replace("    ", "");
    content = content.replace("\n", "");
    content = content.replace(": ", ":");
    let mut small = File::create(source_file.with_extension("min.css"))?;
    println!("{:?}", small);
    small.write_all((&content).as_bytes())?;
    Ok(())
}

fn minify_html(source_file: &Path) -> std::io::Result<()> {
    let mut content = String::new();
    {
        let mut beautiful = File::open(source_file.to_str().unwrap())?;
        beautiful.read_to_string(&mut content)?;
    }
    content = content.replace("    ", "");
    content = content.replace("\n", "");
    let mut source_path = String::from(source_file.to_str().unwrap());
    let target_length = source_path.len() - 8;
    source_path.truncate(target_length);
    let mut small = File::create(
        format!("{}{}", source_path.as_str(), "min.html.hbs")
    )?;
    println!("{:?}", small);
    small.write_all((&content).as_bytes())?;
    Ok(())

}

fn add_subdirs(path: PathBuf, result: &mut Vec<PathBuf>) {
    let paths = fs::read_dir(path.to_str().expect("1")).expect("2");
    for dir_entry in paths {
        let sub_path = dir_entry.expect("2").path();
        if sub_path.is_dir() {
            add_subdirs(sub_path, result)
        } else {
            result.push(sub_path);
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut result = Vec::<PathBuf>::new();
    add_subdirs(Path::new("./").to_path_buf(), &mut result);
    for path in result {
        if String::from(path.to_str().unwrap()).contains(".min.") {
            continue
        }
        let extension = path.extension().map(|os_str| os_str.to_str().unwrap());
        if extension == Some("css") {
            minify_css(&path)?;
        }
        if extension == Some("hbs") {
            minify_html(&path)?;
        }
    }
    Ok(())
}
