use std::io;
use std::io::BufRead;
use std::env;

fn main()->io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = String::new();

}