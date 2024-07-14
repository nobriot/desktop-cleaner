use clap::Parser;
use dirs::home_dir;
use std::process::ExitCode;
use std::{fs, path::PathBuf, thread, time::Duration};
use trash;

#[cfg(windows)]
use winapi::um::fileapi::GetFileAttributesW;
#[cfg(windows)]
use winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN;

/// desktop-cleaner set of arguments
#[derive(Parser, Debug)]
#[command(name = "desktop-cleaner")]
#[command(about = "A simple program that moves files from Desktop to the trash bin", long_about = None)]
struct Args {
    /// Interval in seconds between checks - Defaults to 10 minutes
    #[arg(short, long, default_value_t = 600)]
    interval: u64,

    /// Optional home directory to clean
    #[arg(short, long)]
    home_dir: Option<String>,

    /// Dry-run mode, does not actually delete files
    #[arg(short, long)]
    dry_run: bool,
}

/// Main function that continuously cleans up the Desktop directory by moving
/// files to the trash bin based on safe extensions and a specified interval.
fn main() -> ExitCode {
    // Parse our arguments
    let args = Args::parse();

    // Extensions that we will not delete
    let safe_extensions = vec!["desktop", "exe", "lnk", "url"];

    // Main loop - never stops
    loop {
        if let Err(message) = clean_desktop(&safe_extensions, &args) {
            eprintln!("Error: {message}");
        }
        // Sleep a little bit before the next sweep.
        println!("Waiting {} seconds before the next sweep", args.interval);
        thread::sleep(Duration::from_secs(args.interval));
    }

    // std::process::ExitCode::SUCCESS
}

/// Cleans up the Desktop directory by moving files to the trash bin based
/// on safe extensions and dry-run mode.
///
/// ### Arguments
/// - `safe_extensions`: A slice of safe file extensions that should not be deleted.
/// - `args`: A reference to the `Args` struct containing program arguments.
///
/// ### Returns
/// Result<(), String> indicating success or an error message if any operation fails.
///
/// ### Example
/// ```
/// let safe_extensions = ["desktop", "exe", "lnk", "url"];
/// let args = Args { interval: 600, home_dir: None, dry_run: false };
/// let result = clean_desktop(&safe_extensions, &args);
/// ```
///
fn clean_desktop(safe_extensions: &[&str], args: &Args) -> Result<(), String> {
    // Find the Desktop, if any.
    let mut desktop_directory = match &args.home_dir {
        Some(dir) => PathBuf::from(dir),
        None => home_dir().expect("Could not find the home directory"),
    };
    desktop_directory.push("Desktop");

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
        if !is_hidden(&path) && !is_symlink(&path) {
            // Check if the file will survive based on its extension.
            let mut delete: bool = true;

            // Directories just get deleted.
            if path.is_file() {
                let file_extension_option = path.extension();

                if let Some(file_extension) = file_extension_option {
                    let file_extension = file_extension.to_str().unwrap_or("").to_lowercase();

                    for extension in safe_extensions {
                        if file_extension == *extension {
                            delete = false;
                            break;
                        }
                    }
                }
            }

            if delete {
                if args.dry_run {
                    println!("Would move {:?} to trash", path);
                } else {
                    if let Err(e) = trash::delete(&path) {
                        eprintln!("Failed to move file to trash: {}", e);
                    } else {
                        println!("Moved {:?} to trash", path);
                    }
                }
            }
        }
    }

    Ok(())
}

/// Checks if a file is hidden based on the file path and the operating system.
///
/// On Unix-like systems, it checks if the file name starts with a dot.
///
/// On Windows, it checks the file attributes for the hidden attribute using WinAPI.
///
/// Returns true if the file is hidden, false otherwise.
///
/// # Arguments
/// - `file_path`: A reference to the file path to check for hidden status.
///
/// # Returns
/// A boolean value indicating whether the file is hidden or not.
///
/// # Platform Specific
/// - Unix: Checks if the file name starts with a dot.
/// - Windows: Uses WinAPI to check file attributes for hidden status.
///
/// # Examples
/// ```
/// use std::path::PathBuf;
/// let file_path = PathBuf::from("/path/to/hidden/file.txt");
/// let is_hidden = is_hidden(&file_path);
/// assert_eq!(is_hidden, true);
///
fn is_hidden(file_path: &PathBuf) -> bool {
    // Unix-like systems: Check if the file name starts with a dot
    #[cfg(unix)]
    {
        if let Some(file_name) = file_path.file_name() {
            return file_name
                .to_str()
                .map_or(false, |name| name.starts_with('.'));
        }
        false
    }

    // Windows: Check file attributes for the hidden attribute
    #[cfg(windows)]
    {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;

        let wide_path: Vec<u16> = OsStr::new(file_path).encode_wide().chain(Some(0)).collect();
        let attributes = unsafe { GetFileAttributesW(wide_path.as_ptr()) };
        if attributes == u32::MAX {
            return false;
        }
        (attributes & FILE_ATTRIBUTE_HIDDEN) != 0
    }
}

/// Checks if the given file path is a symbolic link.
fn is_symlink(file_path: &PathBuf) -> bool {
    fs::read_link(file_path).is_ok()
}
