use std::io;
use std::fs;
use std::path::Path;

pub fn run(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        return Ok(()); // don't fail
    }

    let mut recursive = false;
    let mut dirs = Vec::new();

    for arg in args {
        if arg == "-p" {
            recursive = true;
        } else {
            dirs.push(arg);
        }
    }

    for dir in dirs {
        let path = std::path::Path::new(dir);
        let result = if recursive {
            std::fs::create_dir_all(path)
        } else {
            std::fs::create_dir(path)
        };

        if let Err(e) = result {
            eprintln!("mkdir: cannot create directory '{}': {}", dir, e);
        }
    }
    Ok(())
}
