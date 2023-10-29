use std::io;
use std::fs;
use clap::builder::ValueParserFactory;
use clap::{arg, command, Arg, ArgAction, value_parser};

fn main() {
    let matches = command!()
        .arg(
            Arg::new("file_path").action(ArgAction::Append)
            .required(false)
        )
        .arg(
            arg!(
                -n --number <N> "print the first N lines of the text"
            )
            .value_parser(value_parser!(u32))
            .required(false)
        )
        .arg(
            arg!(
                -c --count <N> "print the first N bytes of the lines"
            )
            .value_parser(value_parser!(u32))
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
            let text = &fs::read_to_string(file_path)
                .expect("Should have been able to read the file\n");
            if let Some(bytes_count) = matches.get_one::<u32>("count") {
                if *bytes_count <= text.len().try_into().unwrap() {
                    // contents.push_str(&contents[..*bytes_count as usize]);
                    contents = text[..*bytes_count as usize].to_string();
                }
            } else {
                let lines_number = matches.get_one::<u32>("number");
                let mut line_index = 0;
                for line in text.lines() {
                    match lines_number {
                        Some(end_line) => {
                            if &line_index == end_line {
                                break;
                            }
                        },
                        None => (),
                    }
                    contents.push_str(line);
                    contents.push('\n');
                    line_index = line_index + 1;
                }
            }
        }
    } else {
        // read the standard input as text input
        io::stdin()
            .read_line(&mut contents)
            .expect("Failed to read line\n");
    }

    println!("{contents}");
}
