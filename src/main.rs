use std::env;
use std::process::Command;
fn main() {
    let args: Vec<String> = env::args().collect();
    mv(args);
}

fn mv(args: Vec<String>) {
    match args.len() {
        0..=1 => eprintln!("provide files to move"),
        2.. => mv_more_than_one(args),
    }
}

fn mv_more_than_one(files: Vec<String>) {
    for i in files.iter().skip(1) {
        let _ = Command::new("mv")
            .arg(i)
            .arg(".")
            .output();
    }
}
