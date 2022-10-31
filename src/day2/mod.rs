use std::{fs, time::SystemTime};

enum Instructions {
    Forward(i32),
    Up(i32),
    Down(i32),
}

pub fn day2() {
    println!("--------------DAY 2--------------");
    let start_time = SystemTime::now();

    let raw_instructions = fs::read_to_string("src/day2/input.txt").expect("msg");
    let raw_instructions = raw_instructions.split('\n').collect::<Vec<&str>>();
    let result = process_instructions(raw_instructions);

    println!("result = {}", result);
    println!(
        "calculated in: {} secs",
        start_time.elapsed().unwrap().as_secs_f32()
    );
}

fn parse_instruction(raw_instruction: &str) -> Option<Instructions> {
    let value = raw_instruction.chars().last().unwrap();

    match raw_instruction.chars().next().unwrap() {
        'f' => Some(Instructions::Forward(value.to_digit(10).unwrap() as i32)),
        'u' => Some(Instructions::Up(value.to_digit(10).unwrap() as i32)),
        'd' => Some(Instructions::Down(value.to_digit(10).unwrap() as i32)),
        _ => None,
    }
}

fn process_instructions(instructions: Vec<&str>) -> i32 {
    let mut sub = Submarine {
        horizontal_position: 0,
        depth: 0,
        aim: 0,
    };

    instructions.iter().for_each(|instruction| {
        let command = parse_instruction(instruction);
        match command.unwrap() {
            Instructions::Forward(value) => {
                sub.horizontal_position += value;
                sub.depth += sub.aim * value;
            }
            Instructions::Down(value) => sub.aim += value,
            Instructions::Up(value) => sub.aim -= value,
        }
    });
    sub.depth * sub.horizontal_position
}

struct Submarine {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
}

#[cfg(test)]
mod tests {
    use crate::day2::process_instructions;

    #[test]
    fn process_instructions_test() {
        let instructions = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        let result = process_instructions(instructions);

        assert_eq!(result, 900);
    }
}
