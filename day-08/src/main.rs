#[derive(Debug, PartialEq, Eq)]
enum StatusCode {
    InfiniteLoop,
    UnknownOpCodeException,
    InvalidInstructionPointerException,
    Ok,
}

#[derive(Debug)]
struct EvalResult {
    status: StatusCode,
    accumulator: isize,
    instruction_pointer: usize,
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let instructions: Vec<&str> = INPUT.lines().collect();
    let result = evaluate(&instructions);

    // Part 1
    println!("Part 1: {}", result.accumulator);

    // Part 2
    // Ugly brute force time! TODO: maybe do something smarter...
    for i in 0..instructions.len() {
        let mut new_instructions = instructions.clone();
        let replacement = new_instructions.get_mut(i).unwrap().replace("jmp", "nop");
        *new_instructions.get_mut(i).unwrap() = &replacement;
        let result = evaluate(&new_instructions);
        if result.status == StatusCode::Ok {
            println!("Part 2: {}", result.accumulator);
            return;
        }
    }
    // If we didn't find with the other replace, try the other way :D D:
    for i in 0..instructions.len() {
        let mut new_instructions = instructions.clone();
        let replacement = new_instructions.get(i).unwrap().replace("nop", "jmp");
        *new_instructions.get_mut(i).unwrap() = &replacement;
        let result = evaluate(&new_instructions);
        if result.status == StatusCode::Ok {
            println!("Part 2: {}", result.accumulator);
            break;
        }
    }
}

fn evaluate(instructions: &Vec<&str>) -> EvalResult {
    let mut instruction_pointer = 0;
    let mut accumulator = 0;
    let mut instruction_pointer_history: Vec<usize> = Vec::new();

    let total_instructions = instructions.len();

    loop {
        // dbg!(instruction_pointer);
        // dbg!(accumulator);
        match instructions.get(instruction_pointer) {
            Some(instruction) => {
                let keyval: Vec<&str> = instruction.split(" ").collect();
                let instruction = keyval[0];
                let number: isize = keyval[1].parse().unwrap();
                // dbg!(instruction, number);

                // If we go into an infinite loop, break away
                if instruction_pointer_history.contains(&instruction_pointer) {
                    return EvalResult {
                        status: StatusCode::InfiniteLoop,
                        accumulator,
                        instruction_pointer,
                    };
                }
                instruction_pointer_history.push(instruction_pointer);

                // Main instruction evaluator
                match instruction {
                    "nop" => {}
                    "jmp" => {
                        // TODO: fix, this is ugly.
                        // we shouldn't get negative numbers here...
                        instruction_pointer = (instruction_pointer as isize + number) as usize;
                        continue;
                    }
                    "acc" => {
                        accumulator += number;
                    }
                    _ => {
                        return EvalResult {
                            status: StatusCode::UnknownOpCodeException,
                            accumulator,
                            instruction_pointer,
                        }
                    }
                }
                instruction_pointer += 1;
            }
            None => {
                // If we get into _exactly_ the position of last instruction plus one, the execution is successful
                if instruction_pointer == total_instructions {
                    return EvalResult {
                        status: StatusCode::Ok,
                        accumulator,
                        instruction_pointer,
                    };
                } else {
                    return EvalResult {
                        status: StatusCode::InvalidInstructionPointerException,
                        accumulator,
                        instruction_pointer,
                    };
                }
            }
        }
    }
}
