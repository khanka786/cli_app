use std::{env, fs};
use std::error::Error;
use std::io::{Write, BufReader, BufRead, self};
use std::path::Path;
use chrono::Local;
use zip::ZipWriter;
use zip::write::FileOptions;
use walkdir::WalkDir;



pub fn make_directory(directory_name: &str) -> Result<(), Box<dyn Error>> {
    let current_dir = std::env::current_dir()?;
    let new_directory = current_dir.join(directory_name);
    fs::create_dir_all(&new_directory)?;  // Creates the directory if it doesn't exist
    println!("\n{:?} directory was created successfully\n", new_directory);

    Ok(())
}


pub fn make_file(filename: &str) -> Result<(), Box<dyn Error>>{
    let current_dir = std::env::current_dir()?;
    let file_path = current_dir.join(filename);
    fs::File::create(file_path)?;
    println!("\n{:?} file was created successfully\n", filename);

    Ok(())
}


pub fn echo(text: &str, filename: Option<&str>) -> Result<(), Box<dyn Error>> {
    if let Some(filename) = filename {
        let mut file = fs::File::create(filename)?;
        writeln!(file, "{}", text)?;
        println!("\n{:?} file was created successfully with content: {}\n", filename, text);
    } else {
        println!("{}", text);
    }

    Ok(())
}


pub fn list_directory() -> Result<(), Box<dyn Error>> {
    let current_dir = std::env::current_dir()?;
    println!("\nListing contents of the current directory: {:?}\n", current_dir);

    let entries = fs::read_dir(current_dir)?;
    println!("\nDirectory contents:\n");
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }

    Ok(())
}


pub fn remove(filename: &str) -> Result<(), Box<dyn Error>> {
    fs::remove_file(filename)?;
    println!("\n{:?} file was removed successfully\n", filename);

    Ok(())
}

 
pub fn view_file(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    println!("\nFile contents:\n{}", contents);

    Ok(())
}


pub fn match_pattern(pattern: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}


pub fn remove_directory(directoryname: &str) -> Result<(), Box<dyn Error>> {
    fs::remove_dir_all(directoryname)?;
    println!("\n{:?} directory was removed successfully\n", directoryname);

    Ok(())
}


pub fn clear_terminal() -> Result<(), Box<dyn Error>> {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush()?; // Flushes the stdout to ensure the terminal is cleared

    Ok(())
}


pub fn copy_file(filename: &str, destination: &str) -> Result<(), Box<dyn Error>> {
    fs::copy(filename, destination)?;
    println!("\n{:?} has been copied to {:?}\n", filename, destination);

    Ok(())
}


pub fn directory_path() -> Result<(), Box<dyn Error>> {
    let path = env::current_dir().unwrap(); // unwrap makes it so that Ok() does not print with the path
    println!("\nCurrent Directory Path: {:?}\n", path);

    Ok(())
}


pub fn date() -> Result<(), Box<dyn Error>> {
    let now = Local::now();  
    println!("The current date and time is: {}", now.format("%Y-%m-%d %H:%M:%S")); 

    Ok(())
}


pub fn zip_directory(directory_name: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(output);
    let file = fs::File::create(&path)?;
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default()
    .compression_method(zip::CompressionMethod::Deflated) 
    .unix_permissions(0o755); 

    let base_path = Path::new(directory_name);

    // Walk through the directory recursively
    for entry in WalkDir::new(base_path).into_iter().filter_map(Result::ok) {
        let entry_path = entry.path();
        let name = entry_path.strip_prefix(Path::new(directory_name))?;

        if entry_path.is_file() {
            let mut f = fs::File::open(entry_path)?;
            zip.start_file(name.to_string_lossy(), options)?;
            io::copy(&mut f, &mut zip)?;
        } else if entry_path.is_dir() {
            zip.add_directory(name.to_string_lossy(), options)?;
        }
    }

    zip.finish()?;
    println!("Successfully created zip file at {}", output);

    Ok(())
}


pub fn rename(filename: &str, new_filename: &str) -> Result<(), Box<dyn Error>> {
    fs::rename(filename, new_filename)?;
    println!("\nSuccessfully renamed {:?} to {:?}\n", filename, new_filename);

    Ok(())
}