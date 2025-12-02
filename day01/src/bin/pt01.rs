use day01::input_parser;
use day01::input_parser::{Instruction, Direction};

pub fn main() {
    const START_POSITION: u8 = 50;
    let input = std::fs::read_to_string("input/input.txt")
        .expect("Couldn't find input file!");
    let instructions = input_parser::parse(&input)
        .expect("Couldn't parse input file!");
    let null_positions = count_null_positions(START_POSITION, instructions);
    println!("Null positions found: {}", null_positions);
}

pub fn count_null_positions(start_position: u8, instruction: Vec<Instruction>) -> usize {
    let mut position = start_position;
    instruction.iter().filter_map(|&instr| {
        position = rotate(position, instr);
        match position {
            0 => Some(()),
            _ => None,
        }
    }).count()
}

fn rotate(pos: u8, instruction: Instruction) -> u8 {
    if instruction.dir == Direction::Right {
        return ((pos as usize + instruction.count) % 100) as u8;
    }
    (pos + 100 - (instruction.count % 100) as u8) % 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn check_correct_nullpositions() {
        const START_POSITION: u8 = 50;
        const EXPECTED_NULL_POSITIONS: usize = 3;
        let test_instructions = vec![
            Instruction{ dir: Direction::Left, count: 68 },
            Instruction{ dir: Direction::Left, count: 30 },
            Instruction{ dir: Direction::Right, count: 48 },
            Instruction{ dir: Direction::Left, count: 5 },
            Instruction{ dir: Direction::Right, count: 60 },
            Instruction{ dir: Direction::Left, count: 55 },
            Instruction{ dir: Direction::Left, count: 1 },
            Instruction{ dir: Direction::Left, count: 99 },
            Instruction{ dir: Direction::Right, count: 14 },
            Instruction{ dir: Direction::Left, count: 82 },
        ];
        assert_eq!(count_null_positions(START_POSITION, test_instructions), EXPECTED_NULL_POSITIONS);
    }

}



