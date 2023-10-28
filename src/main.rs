use std::io;
use std::fs;
use clap::{arg, command, Arg, ArgAction};

fn main() {
    let matches = command!()
        .arg(
            Arg::new("file_path").action(ArgAction::Append)
            .required(false)
        )
        .arg(
            arg!(
                -n --number<NUMBER> "print the first N lines per line"
            )
            .required(false)
        )
        .arg(
            arg!(
                -c --count <NUMBER> "print the first N lines of the file"
            )
            .required(false)
        )
        .get_matches();

    let mut contents: String = String::new();
    let file_paths = matches
        .get_many::<String>("file_path")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();
    if file_paths.len() > 0 {
        // read the file given by the file path argument
        for file_path in file_paths {
            contents.push_str(
                &fs::read_to_string(file_path)
                    .expect("Should have been able to read the file\n")
            );
        }
    } else {
        // read the standard input as text input
        io::stdin()
            .read_line(&mut contents)
            .expect("Failed to read line\n");
    }

    println!("{contents}");
}
