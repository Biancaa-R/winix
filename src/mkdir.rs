use std::fs;
use std::io;
use std::path::Path;

/// Run the mkdir command
/// `args` should be the arguments passed to mkdir, e.g., ["-p", "dir1", "dir2"]
pub fn run(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        return Ok(()); // Don't fail, just print message
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
        let path = Path::new(dir);
        let result = if recursive {
            fs::create_dir_all(path)
        } else {
            fs::create_dir(path)
        };

        if let Err(e) = result {
            eprintln!("mkdir: cannot create directory '{}': {}", dir, e);
        }
    }
    Ok(())
}
