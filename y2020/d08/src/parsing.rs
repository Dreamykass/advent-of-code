#![allow(dead_code)]

use std::borrow::Borrow;

#[derive(Debug, PartialEq, Clone)]
pub enum Code {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    pub code: Code,
    pub val: i32,
}

fn str_to_instruction(ss: &str) -> Instruction {
    let mut ss = ss.split(' ');
    let code = ss.next().unwrap();
    let val = ss.next().unwrap();

    let code = match code {
        "acc" => Code::Acc,
        "jmp" => Code::Jmp,
        "nop" => Code::Nop,
        _ => panic!("invalid code"),
    };

    let val = val.parse().unwrap();

    Instruction { code, val }
}

#[test]
fn str_to_instruction_test() {
    let i = str_to_instruction("jmp +392");
    assert_eq!(i.code, Code::Jmp);
    assert_eq!(i.val, 392);

    let i = str_to_instruction("acc -1");
    assert_eq!(i.code, Code::Acc);
    assert_eq!(i.val, -1);
}

pub fn get_instructions() -> Vec<Instruction> {
    include_str!("../input.txt")
        .lines()
        .map(|s| s.borrow())
        .map(str_to_instruction)
        .collect()
}
