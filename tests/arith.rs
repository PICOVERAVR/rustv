
use rustv::{State, run};

#[test]
fn addi() {
    let iv = vec![
        0x93, 0x00, 0xf0, 0x0f, // addi x1, x0, 255
        0x13, 0x01, 0xf0, 0xff, // addi x2, x0, -1
        0x93, 0x01, 0xf0, 0x0f, // addi x3, x0, 255
        0x13, 0x02, 0xf0, 0x0f, // addi x4, x0, 255
    ];

    let s = State::new(0);

    let regs = run(iv, s, &mut vec![]).gprs();

    assert_eq!(regs[0], 0);
    assert_eq!(regs[1], 255);
    assert_eq!(regs[2], 0xffffffff);
    assert_eq!(regs[3], 255);
    assert_eq!(regs[4], 255);
}
