use rustv::*;

use std::{env, fs};

const DMEM_SIZE: usize = 1048576;
const START_ADDR: usize = 0x200;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: rustv [binary]");
        return Ok(());
    }

    let binpath = &args[1];
    let iv = fs::read(binpath)?;
    let mut dv = vec![0; DMEM_SIZE];

    let s = State::new(START_ADDR, START_ADDR, vec![Ext::All]);

    println!("starting execution");
    run(iv, s, &mut dv);

    Ok(())
}
