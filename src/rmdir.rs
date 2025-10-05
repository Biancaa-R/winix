use std::fs;
use std::path::Path;

pub fn run(args: &[String]) {
    if args.is_empty() {
        eprintln!("rmdir: missing operand");
        return;
    }

    let mut recursive = false;
    let mut dirs = Vec::new();

    for arg in args {
        if arg == "-r" {
            recursive = true;
        } else {
            dirs.push(arg);
        }
    }

    for dir in dirs {
        let path = Path::new(dir);
        let result = if recursive {
            fs::remove_dir_all(path)
        } else {
            fs::remove_dir(path)
        };

        if let Err(e) = result {
            eprintln!("rmdir: failed to remove '{}': {}", dir, e);
        }
    }
}
