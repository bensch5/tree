use std::{env::args, fs::read_dir, path::Path};

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
    // keep track of all the currently open directories
    let open_dirs = vec![false; max_level];
    rec(Path::new(&path), 0, max_level - 1, open_dirs)
}

fn rec(path: &Path, level: usize, max_level: usize, mut open_dirs: Vec<bool>) {
    // set current directory to open
    open_dirs[level] = true;
    // unwrap files
    let files: Vec<_> = read_dir(path).unwrap().map(|e| e.unwrap()).collect();
    for (i, file) in files.iter().enumerate() {
        let file_str = file.file_name().into_string().unwrap();
        let mut separator = "├────── ";
        if i == files.len() - 1 {
            // set current directory to closed
            open_dirs[level] = false;
            separator = "└────── ";
        }
        for j in 0..level {
            if open_dirs[j] {
                print!("│\t");
            } else {
                print!("\t");
            }
        }
        print!("{separator}");
        if file.file_type().unwrap().is_dir() {
            println!("{}", file_str.blue());
            if level < max_level {
                rec(
                    file.path().as_path(),
                    level + 1,
                    max_level,
                    open_dirs.to_vec(),
                );
            }
        } else {
            println!("{file_str}");
        }
    }
}
