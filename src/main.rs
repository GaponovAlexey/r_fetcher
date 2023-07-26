use std::{ io::{self, Write}, path::PathBuf, fs, fmt::format };

fn get_input(query: &str) -> io::Result<String> {
    println!("{:?}", query);
    io::stdout().flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn organize_dir(dir_path: PathBuf) {
    let entries = dir_path.read_dir().unwrap_or_else(|e| panic!("{}", e));

    for entry in entries {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            continue;
        }

        match entry.path().extension() {
            Some(extension) => {
                let extension_dir = dir_path.join(
                    format!("format - {}", extension.to_string_lossy())
                );
                if !extension_dir.exists() {
                    fs::create_dir(&extension_dir).unwrap();
                }
                fs::rename(entry.path(), extension_dir.join(entry.file_name())).unwrap();
            }
            None => println!("File {} has no extension.", entry.path().display()),
        }
    }
}

fn main() {
    loop {
        let dir_path = get_input("Enter a path").unwrap();
        organize_dir(PathBuf::from(dir_path));
    }
}

// use std::{ io::{ self, Write }, path::PathBuf, time::Instant, fs };

// fn get_input(query: &str) -> io::Result<String> {
//     println!("{:?}", query);
//     io::stdout().flush()?;

//     let mut buffer = String::new();
//     io::stdin().read_line(&mut buffer)?;
//     Ok(buffer.trim().to_string())
// }

// fn organize_dir(dir_path: PathBuf) {
//     if !dir_path.exists() {
//         println!("Doesn't exist {:?}", dir_path.display())
//     } else {
//         let dir_files = match dir_path.read_dir() {
//             Ok(r) => r,
//             Err(e) => {
//                 eprintln!("{:?}", e);
//                 return;
//             }
//         };
//         for f in dir_files {
//             if let Ok(f) = f {
//                 if f.path().is_dir() {
//                     println!("path{} is a dir, skip", f.path().display());
//                     continue;
//                 }

//                 let file_extension = match f.path().extension() {
//                     Some(extension) =>
//                         match extension.to_str() {
//                             Some(extension) => extension.to_lowercase(),
//                             None => {
//                                 continue;
//                             }
//                         }
//                     None => {
//                         println!("{:?}", f.path().display());
//                         continue;
//                     }
//                 };
//                 let extension_dir = PathBuf::from(dir_path.join(file_extension));
//                 create_dir_if_not_exist(&extension_dir);

//                 move_file(&f.path(), &extension_dir.join(f.file_name()));
//             }
//         }
//     }
// }

// fn create_dir_if_not_exist(dir_path: &PathBuf) {
//     if !dir_path.exists() {
//         if let Err(err) = fs::create_dir(dir_path) {
//             println!("Err{:?}", err)
//         }
//     }
// }

// fn move_file(from: &PathBuf, to: &PathBuf) {
//     if let Err(e) = fs::rename(from, to) {
//         println!("Err moving file{}{}", from.display(), to.display())
//     }
// }

// fn main() {
//     loop {
//         let dir_path = match get_input("Enter a path") {
//             Ok(res) => res,
//             Err(err) => {
//                 eprintln!("{:?}", err);
//                 continue;
//             }
//         };
//         let now = Instant::now();
//         organize_dir(PathBuf::from(dir_path));
//         println!("Время выполнения:{:?}", now.elapsed().as_secs_f64());
//     }
// }
