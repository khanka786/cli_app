use clap::{Arg, Command};
use std::process;
use kashan786_cli_app::{make_directory, make_file, echo, list_directory, remove, view_file, 
    match_pattern, remove_directory, clear_terminal, copy_file, directory_path, date, zip_directory, rename};

fn main() {
    let matches = Command::new("kashan_cli_application") 
        .arg(
            Arg::new("make_directory")
                .long("make_directory")   
                .value_parser(clap::value_parser!(String)) 
                .help("Creates a new directory in the current working directory"),
        )
        .arg(
            Arg::new("make_file")
                .long("make_file")   
                .value_parser(clap::value_parser!(String))
                .help("Creates a new file in the current working directory"),
        )
        .arg(
            Arg::new("echo")
                .long("echo")          
                .value_parser(clap::value_parser!(String))
                .num_args(1..=2)
                .help("Text to print or save to a file; specify optional filename as second argument"),
        )
        .arg(
            Arg::new("list")
                .long("list") 
                .action(clap::ArgAction::SetTrue)
                .help("Lists all files and directories in the current directory"),
        )
        .arg(
            Arg::new("remove")
                .long("remove")           
                .value_parser(clap::value_parser!(String))
                .help("Removes a given file in the current directory"),
        )
        .arg(
            Arg::new("view_file")
                .long("view_file")           
                .value_parser(clap::value_parser!(String))
                .help("Views a file in the current directory"),
        )
        .arg(
            Arg::new("match_pattern")
                .long("match_pattern")           
                .value_parser(clap::value_parser!(String))
                .num_args(2) // Expect exactly 2 arguments
                .help("Matches a given pattern with a given file and displays the lines containing the pattern"),
        )
        .arg(
            Arg::new("remove_directory")
                .long("remove_directory")           
                .value_parser(clap::value_parser!(String))
                .help("Removes a given directory"),
        )
        .arg(
            Arg::new("clear_terminal")
                .long("clear_terminal")
                .action(clap::ArgAction::SetTrue)
                .help("Clears the terminal"),
        )
        .arg(
            Arg::new("copy_file")
                .long("copy_file")
                .value_parser(clap::value_parser!(String))
                .num_args(2) // Expect exactly 2 arguments
                .help("Copies a specified file"),
        )
        .arg(
            Arg::new("directory_path")
                .long("directory_path")
                .action(clap::ArgAction::SetTrue)
                .help("Shows the path of the current directory"),
        )
        .arg(
            Arg::new("date")
                .long("date")
                .action(clap::ArgAction::SetTrue)
                .help("Shows the current date and time"),
        )
        .arg(
            Arg::new("zip_directory")
                .long("zip_directory")
                .value_parser(clap::value_parser!(String))
                .num_args(2) // Expect exactly 2 arguments
                .help("Compresses a directory "),
        )
        .arg(
            Arg::new("rename")
                .long("rename")   
                .value_parser(clap::value_parser!(String)) 
                .num_args(2) // Expect exactly 2 arguments
                .help("Changes the name of a file to a new given name"),
        )
        .get_matches();

    if let Err(e) = run(&matches) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {

  
    if let Some(directory_name) = matches.get_one::<String>("make_directory") {
        make_directory(directory_name)?;
    }

 
    if let Some(filename) = matches.get_one::<String>("make_file") {
        make_file(filename)?;
    }


    if let Some(values) = matches.get_many::<String>("echo") {
        // Collect arguments as a Vec<String>
        let args: Vec<_> = values.collect();
        
        let text = &args[0];  
        let filename = args.get(1).map(|s| s.as_str());  // Optional filename
        echo(text, filename)?;
    }


    if matches.get_flag("list") {
        list_directory()?;  
    }


    if let Some(filename) = matches.get_one::<String>("remove") {
        remove(filename)?;
    }


    if let Some(filename) = matches.get_one::<String>("view_file") {
        view_file(filename)?; 
    }
    

    if let Some(values) = matches.get_many::<String>("match_pattern") {
        let args: Vec<_> = values.collect();
        let pattern = &args[0];
        let filename = &args[1];
        match_pattern(pattern, filename)?;
    }


    if let Some(directoryname) = matches.get_one::<String>("remove_directory") {
        remove_directory(directoryname)?;
    }


    if matches.get_flag("clear_terminal") {
        clear_terminal()?;
    }


    if let Some(values) = matches.get_many::<String>("copy_file") {
        let args: Vec<_> = values.collect();
        let filename = &args[0];
        let destination = &args[1];
        copy_file(filename, destination)?;
    }


    if matches.get_flag("directory_path") {
        directory_path()?;
    }

    
    if matches.get_flag("date"){
        date()?;
    }


    if let Some(values) = matches.get_many::<String>("zip_directory") {
        let args: Vec<_> = values.collect();
        let directory_name = &args[0];
        let output = &args[1];
        zip_directory(directory_name, output)?;
    }
    

    if let Some(values) = matches.get_many::<String>("rename") {
        let args: Vec<_> = values.collect();
        let filename = &args[0];
        let new_filename = &args[1];
        rename(filename, new_filename)?;
    }

    Ok(())
}
