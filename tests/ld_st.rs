use rustv::{State, run};

#[test]
fn st() {

    // write 0xffffffff, then 0x0000, then 0xff to the same memory location
    let iv = vec![
        0x93, 0x00, 0x40, 0x00, // addi x1, x0, 4
        0x13, 0x01, 0xf0, 0xff, // addi x2, x0, -1
        0x23, 0xa0, 0x20, 0x00, // sw x2, 0(x1)
        0x13, 0x01, 0x00, 0x00, // addi x2, x0, 0
        0x23, 0x90, 0x20, 0x00, // sh x2, 0(x1)
        0x13, 0x01, 0xf0, 0x0f, // addi x2, x0, 255
        0x23, 0x80, 0x20, 0x00, // sb x2, 0(x1)
    ];

    let s = State::new(0);

    let mut dv = vec![0; 8];

    run(iv, s, &mut dv).gprs();

    assert_eq!(dv[0], 0);
    assert_eq!(dv[1], 0);
    assert_eq!(dv[2], 0);
    assert_eq!(dv[3], 0);

    assert_eq!(dv[4], 0xff);
    assert_eq!(dv[5], 0);
    assert_eq!(dv[6], 0xff);
    assert_eq!(dv[7], 0xff);
}

#[test]
fn ld() {
    // read a word, then a halfword, then a byte from the same memory location
    let iv = vec![
        0x83, 0x20, 0x00, 0x00, // lw x1, 0(x0)
        0x03, 0x11, 0x20, 0x00, // lh x2, 2(x0)
        0x83, 0x01, 0x30, 0x00, // lb x3, 3(x0)
    ];

    let s = State::new(0);

    let mut dv = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let regs = run(iv, s, &mut dv).gprs();

    println!("x1: {:x}", regs[1]);
    println!("x2: {:x}", regs[2]);
    println!("x3: {:x}", regs[3]);

    assert_eq!(regs[0], 0);
    assert_eq!(regs[1], 0x04030201);
    assert_eq!(regs[2], 0x0403);
    assert_eq!(regs[3], 0x04);
    assert_eq!(regs[4], 0);
}
