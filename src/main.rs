//! Author: Giovanni D'Andrea
//! License: MIT
//!
//! Release date:
use std::fs::{DirEntry, File};
use std::io::{Read, Write};
use std::path::Path;
use std::{
    env,
    env::VarError,
    fmt,
    fmt::{Display, Formatter},
    fs,
    io::{Error, ErrorKind},
    path::{self, PathBuf},
    process,
    str::FromStr,
};
use zip::write::FileOptions;
use zip::ZipWriter;

mod utils;

static KB_DEF: &str = "Kb";
static MB_DEF: &str = "Mb";
static GB_DEF: &str = "Gb";

/// Represents command-line options.
#[derive(Debug)]
enum Options {
    /// Option to specify the output.
    Output,
    /// Option to specify the output file name.
    FileName,
    /// Option to exclude certain items.
    Exclude,
    /// Option to set the level of redundancy.
    Redundancy,
    /// Option to display help information.
    Helper,
}

#[derive(Debug)]
enum ErrorOptions {
    NotValidOption,
}

/// Implementing the `FromStr` trait for `Options` convert a String to an Options.
impl FromStr for Options {
    type Err = ErrorOptions;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // TODO: Switch -o with -r (target dir and filename)
            "-o" => Ok(Options::Output),
            "-O" => Ok(Options::FileName),
            "-e" => Ok(Options::Exclude),
            "-R" => Ok(Options::Redundancy),
            "-h" => Ok(Options::Helper),
            _ => Err(ErrorOptions::NotValidOption),
        }
    }
}

/// Implementing the `Display` trait for `Options` to enable formatted printing.
impl Display for Options {
    /// Formats the `Options` enum into a string representation suitable for command-line usage.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Options::Output => "-o",
                Options::FileName => "-O",
                Options::Exclude => "-e",
                Options::Redundancy => "-R",
                Options::Helper => "-h",
            }
        )
    }
}

/// A struct representing command-line arguments.
///
/// This struct is like a suitcase, holding all the essentials for your trip through the CLI.
#[derive(Debug)]
struct CliArgs {
    // FIXME: Review those arguments
    // The output file name.
    filename_out: String,
    // The "where to?" of our journey.
    target: String,
    // How many extra socks you're packing.
    redundancy: String,
    // Things you're leaving behind (like pineapple pizza).
    excluded: String,
}

impl CliArgs {
    /// Constructs a new, empty `CliArgs`.
    fn new() -> Self {
        Self {
            filename_out: "".to_string(),
            target: "".to_string(),
            redundancy: "".to_string(),
            excluded: "".to_string(),
        }
    }
}

impl Display for CliArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Target: {}, Redundancy: {}, Excluded: {}",
            self.target, self.redundancy, self.excluded
        )
    }
}

/// An enumeration of file types.
///
/// Just like in real life, it's either a folder or a file.
#[derive(Debug, Clone)]
enum FileType {
    Directory,
    // The digital equivalent of a drawer.
    File, // A single piece of paper in that drawer.
}

/// A struct representing the command-line interface.
///
/// Think of it as the control panel of your spaceship.
#[derive(Debug)]
struct Cli {
    arguments: CliArgs, // The dials and switches on your control panel.
}

impl Cli {
    /// Creates a new `Cli` with the given `CliArgs`.
    ///
    /// Like turning the key in your spaceship's ignition.
    fn new(config: CliArgs) -> Self {
        Self { arguments: config }
    }

    fn parse_relative_directory(&mut self) -> Result<(), Error> {
        return if self.arguments.target.starts_with("~") {
            let home: String =
                env::var("HOME").map_err(|e: VarError| Error::new(ErrorKind::NotFound, e))?;
            self.arguments.target = format!("{}{}", home, self.arguments.target.replace('~', ""));
            Ok(())
        } else {
            Ok(())
        };
    }

    /// Converts bytes to a human-readable format.
    ///
    /// This method is like a translator for computers, turning bytes into something we can understand.
    fn as_human_read(&self, bytes: u64) -> (f64, &str) {
        match bytes {
            0..=999 => (bytes as f64, KB_DEF),
            1000..=1_048_575 => ((bytes as f64) / 1024.0, KB_DEF),
            1_048_576..=1_073_741_823 => ((bytes as f64) / (1024.0 * 1024.0), MB_DEF),
            _ => ((bytes as f64) / (1024.0 * 1024.0 * 1024.0), GB_DEF),
        }
    }

    /// Prepares directories for... well, something important. (To be implemented)
    fn prepare_exclude_directories(&self) -> Vec<PathBuf> {
        let exclude_dirs: Vec<&str> = self.arguments.excluded.split(",").collect();
        let exclude_dirs_buf: Vec<PathBuf> = exclude_dirs.iter().map(|path| PathBuf::from(path)).collect();

        return exclude_dirs_buf;
    }

    /// Predicts how big the zip file will be. (It's like fortune-telling for files)
    fn how_is_big(&self) {
        /* TODO: To implement */
        // From the array of directories got in input (i.e. directories: Vec<Something>)
        // predict the size of the zip file (No fucking idea how to do this yet)
        // return the predicted size of the zip file
        todo!("Predict the zip file size")
    }

    /// Prints a preview of what's going to happen. (Everyone loves a sneak peek)
    fn print_preview(&self) {
        /* TODO: To implement */
        // Get all the details about:
        // - the zip file size
        // - the directory zip
        // - the directories to avoid
        todo!("Print the preview before continue the program")
    }

    fn configure_output_name(&self) -> &Path {
        // TODO: use datetime to format the filename (tmp_<datetime>.zip)
        if self.arguments.filename_out == "" {
            return Path::new("tmp.zip");
        }

        return Path::new(&self.arguments.filename_out);
    }

    fn is_excluded_path(&self, path: &DirEntry) -> bool {
        // FIXME: parse the excluded path names correctly
        let exclude_paths: Vec<PathBuf> = self.prepare_exclude_directories();

        return exclude_paths.contains(&path.path());
    }

    /// Zip the target directory excluding the choosen ones
    fn zip_files(&self) -> Result<(), Error> {
        // TODO: Refactor this function
        // TODO: Set the zip file name using cli argument
        let path = self.configure_output_name();
        let file = File::create(path)?;

        let mut stack: Vec<PathBuf> = vec![PathBuf::from(&self.arguments.target)];
        let mut zip = ZipWriter::new(file);
        let mut file_counter: usize = 0;

        while let Some(current_path) = stack.pop() {
            let relative_path = current_path
                .strip_prefix(&self.arguments.target)
                .unwrap_or(&current_path);

            if current_path.is_dir() {
                for entry in fs::read_dir(&current_path)? {
                    let entry = entry?;
                    // if entry is not an excluded
                    if !self.is_excluded_path(&entry) {
                        stack.push(entry.path());
                    }
                    continue;
                }
                if relative_path != Path::new("") {
                    let mut dir_path = relative_path.to_str().unwrap().to_owned();
                    dir_path.push('/'); // add a '/' to the end of the path

                    zip.add_directory(&dir_path, FileOptions::default())?;
                    println!("Directory {} added to zip file!", dir_path);
                }
            } else {
                zip.start_file(relative_path.to_str().unwrap(), FileOptions::default())?;
                let mut f = File::open(&current_path)?;

                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;

                file_counter += 1;
                println!("File {:?} zipped!", current_path.file_name().unwrap());
            }
        }

        zip.finish()?;
        println!("Zip file created successfully!");
        println!("Total files zipped: {}", file_counter);

        Ok(())
    }

    /// Calculates the total size of the target directory.
    ///
    /// It's like weighing your suitcase before a flight to avoid extra fees.
    fn calculate_target_size(&self) -> Result<u64, Error> {
        let path = path::Path::new(self.arguments.target.as_str());
        let mut total_size = 0;
        let mut stack: Vec<PathBuf> = vec![path.to_path_buf()];

        while let Some(current_path) = stack.pop() {
            if current_path.is_dir() {
                for entry in fs::read_dir(&current_path)? {
                    let entry = entry?;
                    let entry_path = entry.path();
                    stack.push(entry_path);
                }
            } else {
                total_size += fs::metadata(&current_path)?.len();
            }
        }
        return Ok(total_size);
    }

    /// Runs the tool and zips the files.
    fn run(&mut self) {
        self.parse_relative_directory()
            .unwrap_or_else(|er| eprintln!("[EROR] {}", er));
        println!("[INFO] parsed: {:#?}", self);

        match self.calculate_target_size() {
            Ok(size) => {
                let (s, t) = self.as_human_read(size);
                println!("{}{}", s as usize, t);
                self.zip_files()
                    .unwrap_or_else(|er| eprintln!("[EROR] {}", er));
            }
            Err(er) => eprintln!("[EROR] {}", er),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Please provide a valid argument! See -h to help");
        process::exit(1);
    }

    let mut cli_config = CliArgs::new();

    for (i, arg) in args.iter().enumerate() {
        match Options::from_str(arg.as_str()).ok() {
            Some(Options::Output) => {
                cli_config.target = args[i + 1].clone();
            }
            Some(Options::FileName) => {
                cli_config.filename_out = args[i + 1].clone();
            }
            Some(Options::Exclude) => {
                cli_config.excluded = args[i + 1].clone();
            }
            Some(Options::Redundancy) => {
                cli_config.redundancy = args[i + 1].clone();
            }
            Some(Options::Helper) => {
                utils::print_helper(None);
                process::exit(1);
            }
            None => {}
        }
    }

    println!("{cli_config:?}");
    let mut cli = Cli::new(cli_config);
    cli.run();
}
