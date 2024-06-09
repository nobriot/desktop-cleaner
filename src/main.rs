use clap::Parser;
use dirs::home_dir;
use std::process::ExitCode;
use std::{fs, thread, time::Duration};
use trash::delete;

#[derive(Parser, Debug)]
#[command(name = "desktop_cleaner")]
#[command(about = "A simple program that moves files from Desktop to the trash bin", long_about = None)]
struct Args {
    /// Interval in seconds between checks - Defaults to 10 minutes
    #[arg(short, long, default_value_t = 600)]
    interval: u64,
}

fn main() -> ExitCode {
    // Parse our arguments
    let args = Args::parse();

    // Extensions that we will not delete
    let safe_extensions = vec![".desktop", ".exe"];

    // Main loop
    loop {
        if let Err(message) = clean_desktop(&safe_extensions) {
            eprintln!("Error: {message}");
        }
        // Sleep a little bit before the next sweep.
        println!("Waiting {} seconds before the next sweep", args.interval);
        thread::sleep(Duration::from_secs(args.interval));
    }

    std::process::ExitCode::SUCCESS
}

fn clean_desktop(safe_extensions: &[&str]) -> Result<(), String> {
    // Find the Desktop, if any.
    let desktop_directory = home_dir().map(|mut path| {
        path.push("Desktop");
        path
    });
    if desktop_directory.is_none() {
        return Err(String::from("Could not find the Desktop directory."));
    }
    let desktop_directory = desktop_directory.unwrap();

    // List the files
    let files = fs::read_dir(&desktop_directory);
    if files.is_err() {
        return Err(format!(
            "Could not read content of the {:?} directory.",
            desktop_directory
        ));
    }
    let files = files.unwrap();

    // Check for all of them if we want to delete.
    for file in files {
        if file.is_err() {
            eprintln!("Error with file: {:?}", file);
            continue;
        }

        let path = file.unwrap().path();
        if path.is_file() {
            // Check if the file will survive.
            let mut delete: bool = true;
            for extension in safe_extensions {
                if path.ends_with(extension) {
                    delete = false;
                    break;
                }
            }

            if delete {
                println!("Moving file: {:?} to trash", path);
                /*if let Err(e) = delete(&path) {
                    eprintln!("Failed to move file to trash: {}", e);
                } else {
                    println!("Moved {:?} to trash", path);
                }*/
            }
        }
    }

    Ok(())
}
