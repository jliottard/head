use std::env;
use std::io;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut contents: String = String::new();
    if args.len() == 1 {
        // read the standard input as text input
        io::stdin()
            .read_line(&mut contents)
            .expect("Failed to read line\n");
    } else {
        // read the file given by the file path argument
        let file_path = &args[1];
        contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file\n");
    }
    println!("{contents}");
}
