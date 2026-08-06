use std::fs;
use std::path::{Path, PathBuf};

const TEMPLATE: &str = r#"
use std::fs::File;
use std::io::prelude::*;

pub fn run() -> Result<(), String> {
    let mut file = File::open("src/day01/input.txt").map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;

    Ok(())
}
"#;

pub fn scaffold_day(day_number: &str) -> Result<(), String> {
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let src_root = crate_root.join("src");
    scaffold_dir(&src_root, day_number)
}

fn scaffold_dir(root: &Path, day_number: &str) -> Result<(), String> {
    let day_dir = root.join(format!("day{day_number}"));
    fs::create_dir_all(&day_dir).map_err(|e| e.to_string())?;

    let render = TEMPLATE.trim().replace("01", day_number);

    let files = [
        ("mod.rs", "pub mod a;\npub mod b;\n"),
        (
            "a.rs",
            render.as_str(),
        ),
        (
            "b.rs",
            render.as_str(),
        ),
        ("input.txt", "TODO: add input\n"),
    ];

    for (name, content) in files {
        let file_path = day_dir.join(name);
        if !file_path.exists() {
            fs::write(file_path, content).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}
