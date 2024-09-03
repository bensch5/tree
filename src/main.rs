use std::{any, env::args, fs::read_dir, io, path::Path};

use colored::Colorize;

fn main() {
    let path = match args().skip(1).next() {
        Some(arg) => arg,
        None => ".".to_string(),
    };
    let max_level = match args().skip(2).next() {
        Some(arg) => arg.parse().unwrap(),
        None => 3,
    };
    println!("{path}");
    rec(Path::new(&path), 1, max_level)
}

fn rec(path: &Path, level: usize, max_level: usize) {
    // unwrap files
    let files: Vec<_> = read_dir(path).unwrap().map(|e|e.unwrap()).collect();
    for (i, file) in files.iter().enumerate() {
        let file_str = file.file_name().into_string().unwrap();
        let seperator = match i {
            any if any == files.len() - 1 => "└────── ",
            _ => "├────── ",
        };
        if file.file_type().unwrap().is_dir() {
            println!("{}{}", "\t".repeat(level - 1) + seperator, file_str.blue());
            if level < max_level {
                rec(file.path().as_path(), level + 1, max_level);
            }
        } else {
            println!("{}{}", "\t".repeat(level - 1) + seperator, file_str);
        }
    }
}