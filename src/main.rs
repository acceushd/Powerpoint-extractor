use itertools::Itertools;
use std::collections::HashMap;
#[allow(unused_imports)]
use std::{env, fs};
use std::thread::sleep;
use std::time::Duration;
use tempfile::tempdir;
use zip::ZipArchive;

#[allow(unused_parens)]
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, {}", args[1]);
    let content = extract_zip_archive(args[1].clone());
    content.keys().sorted();
}

fn extract_zip_archive(zip_file: String) -> HashMap<String, String> {
    let powerpoint_file = zip_file;
    let powerpoint_reader =
        fs::File::open(&powerpoint_file).expect("Failed to open PowerPoint file");
    let mut zip_archive = ZipArchive::new(powerpoint_reader).expect("Failed to open zip file");
    let tmp_dir = tempdir();
    return match tmp_dir {
        Ok(dir) => {
            println!("Created temporary directory: {}", dir.path().display());

            ZipArchive::extract(&mut zip_archive, &dir).expect("Failed to extract zip file");
            let files = fs::read_dir(&dir).expect("Failed to read tmp directory");
            let mut content_of_files: HashMap<String, String> = HashMap::new();
            let mut found = false;
            for file in files {
                match file {
                    Ok(file) => {
                        found = true;
                        let file_path = file.path();
                        println!("Found file: {}.", file_path.display());
                        content_of_files.insert(
                            file_path.display().to_string(),
                            fs::read_to_string(file_path).expect("Failed to read file"),
                        );
                    }
                    Err(e) => {
                        eprintln!("Error reading file: {}", e);
                    }
                }
            }
            if !found {
                println!("No files found in tmp directory");
            }
            sleep(Duration::from_millis(2000));
            dir.close().expect("Failed to delete temporary directory");
            content_of_files
        }
        Err(e) => {
            eprintln!("Error creating temporary directory: {}", e);
            HashMap::new()
        }
    }
}
