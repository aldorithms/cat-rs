
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // Parse command line arguments
    let mut matches = cmd().get_matches();

    // Retrieve the list of file paths from the command line arguments
    let paths: Vec<_> = matches
        .remove_many::<std::path::PathBuf>("filename")
        .unwrap()
        .collect();

    // Iterate over each file path
    for path in paths {
        println!();

        // Check if the file exists
        if !path.exists() {
            // Open the file and read its contents
            let mut file = std::fs::File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            
            // If the "-n" or "--number" flag was provided, display the contents of the file with each line numbered
            if *matches.get_one::<bool>("number").unwrap() {
                for (i, line) in contents.lines().enumerate() {
                    println!("{}: {}", i + 1, line);
                }
            } else {
                // Otherwise, simply display the contents of the file
                println!("{}", contents);
            }

        } else {
            eprintln!("{:}: No such file or directory", path.display());
            continue;
        }   
    }

    Ok(())
}

pub fn cmd() -> clap::Command {
    // Define the command line arguments
    clap::Command::new("cat").args(&[
        clap::arg!(filename: <FILE> ... "File to output")
            .required(true)
            .value_parser(
                clap::value_parser!(std::path::PathBuf)
            ),
        clap::arg!(number: -n --number "display the contents of a file with each line numbered"),
    ])
}
//
//This code defines a command line tool called "cat" that reads the contents of one or more files and prints them to the standard output. The "-n" or "--number" flag can be used to display the contents of the file with each line numbered..</s>