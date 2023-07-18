// use std::fs::*;
use std::io::{stdout, BufWriter};
use ferris_says::say;

fn main() {
    // Read the contents of the `Cargo.toml` file
    // let toml: Result<String, Error> = read_to_string("Cargo.toml");
    // // Pattern match on the `Result` to get the `String` or the `Error`
    // let file_contents: String = match toml {
    //     Ok(toml) => toml,
    //     Err(_e) => "Didn't work".to_string(),
    // };
    // println!("{}", file_contents);
    let mut s = "❤️🧡💛💚💙💜";
    println!("{}", s);
    s = "💜💙💚💛🧡❤️";
    println!("{}", s);
    let stdout = stdout();
    let message = String::from("Hey there!");
    let width = message.chars().count();

    let mut writer: BufWriter<std::io::StdoutLock<'_>> = BufWriter::new(stdout.lock());

    let _ = say(message.as_bytes(), width, &mut writer).unwrap();
}
