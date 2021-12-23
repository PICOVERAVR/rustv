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

    let regs = run(iv, s, &mut dv).gprs();

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
    // TODO
    panic!()
}
