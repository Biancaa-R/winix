use std::fs;
use std::path::Path;

pub fn run(args: &[String]) {
    let root = if args.is_empty() { "." } else { &args[0] };
    let path = Path::new(root);

    if !path.exists() {
        eprintln!("tree: '{}' does not exist", root);
        return;
    }

    print_tree(path, 0);
}

fn print_tree(path: &Path, depth: usize) {
    if let Some(name) = path.file_name() {
        println!("{}{}", " ".repeat(depth * 2), name.to_string_lossy());
    }

    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                print_tree(&entry.path(), depth + 1);
            }
        }
    }
}
