use crate::parsing::{get_instructions, Code, Instruction};

mod parsing;

// Ok() with acc_val on termination
// Err() with acc_val just before a crash or looping
fn execution_check(instructions: Vec<Instruction>) -> Result<i32, i32> {
    let mut instructions = instructions
        .into_iter()
        .map(|i| (i, false))
        .collect::<Vec<(Instruction, bool)>>();

    let mut acc_val = 0i32;
    let mut seek = 0usize;

    while seek != instructions.len() {
        if instructions.get(seek).ok_or(acc_val)?.1 {
            return Err(acc_val);
        };

        let (instruction, visited) = instructions.get_mut(seek).ok_or(acc_val)?;

        // println!(
        //     "seek: {}, code: {:?}, val: {}, visited: {}, acc: {}",
        //     seek, instruction.code, instruction.val, visited, acc_val
        // );

        *visited = true;
        match &instruction.code {
            Code::Acc => {
                acc_val += instruction.val;
                seek += 1;
            }
            Code::Jmp => {
                // seek += instruction.val as usize;
                let mut seek_temp = seek as isize;
                seek_temp += instruction.val as isize;
                seek = seek_temp as usize;
            }
            Code::Nop => {
                seek += 1;
            }
        }
    }

    Ok(acc_val)
}

fn task_one() {
    println!(
        "part one, acc value: {}",
        execution_check(get_instructions()).unwrap_err()
    );
}

fn task_two() {
    let instructions = get_instructions();
    instructions
        .iter()
        .enumerate()
        .filter(|(_, i)| i.code != Code::Acc)
        .map(|(e, _)| e)
        .for_each(|e| {
            let mut instructions = instructions.clone();
            if instructions[e].code == Code::Jmp {
                instructions[e].code = Code::Nop;
            } else {
                instructions[e].code = Code::Jmp;
            }

            if let Ok(acc_val) = execution_check(instructions) {
                println!("part two, acc value: {}", acc_val);
            }
        });
}

fn main() {
    task_one();
    task_two();
}
