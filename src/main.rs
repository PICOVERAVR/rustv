use rustv::*;

use std::env;
use std::fs;

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: rustv <binary>");
        panic!()
    }

    let binpath = &args[1];
    let bv = fs::read(binpath)?;

    let s = State::new(0);

    run(bv, s, 512);

    Ok(())
}
