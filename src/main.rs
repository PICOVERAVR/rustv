use rustv::*;

use std::env;
use std::fs;

const DMEM_SIZE: usize = 512;

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: rustv [binary]");
        return Ok(())
    }

    let binpath = &args[1];
    let iv = fs::read(binpath)?;
    let mut dv = vec![0; DMEM_SIZE];

    let s = State::new(0);

    run(iv, s, &mut dv);

    Ok(())
}
