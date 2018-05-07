extern crate heliometer;

use heliometer::*;

fn main() {
    use std::io::Read;

    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file.bf>", args[0]);
        std::process::exit(1);
    }

    let mut file = std::fs::File::open(&args[1]).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    execute(&input, &mut std::io::stdin(), &mut std::io::stdout()).unwrap();
}
