use std::{ io, path::PathBuf, time::Instant, fs };

fn get_input(query: &str) -> io::Result<String> {
    let mut buffer = String::new();
    println!("{}", query);
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn organize_dir(dir_path: PathBuf) {
    let entries = dir_path.read_dir().unwrap_or_else(|e| panic!("{}", e));

    for entry in entries {
        let entry = entry.exist();
        if entry.path().is_dir() {
            continue;
        }

        let extension = entry
            .path()
            .extension()
            .unwrap_or_else(|| panic!("Can't get extension."));
        let extension_dir = dir_path.join(extension);
        if !extension_dir.exists() {
            fs::create_dir(&extension_dir).unwrap();
        }

        fs::rename(entry.path(), extension_dir.join(entry.file_name())).unwrap();
    }
}

fn main() {
    loop {
        let dir_path = get_input("Enter a path").unwrap();
        organize_dir(PathBuf::from(dir_path));
    }
}
