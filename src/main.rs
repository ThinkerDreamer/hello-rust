use std::fs::*;
use std::io::{Error, stdout};
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
    let _ = say(b"Hey there", 80, &mut stdout().lock());
}
