use day01::input_parser;
use day01::input_parser::{Instruction, Direction};

pub fn main() {
    const START_POSITION: u8 = 50;
    let input = std::fs::read_to_string("input/input.txt")
        .expect("Couldn't find input file!");
    let instructions = input_parser::parse(&input)
        .expect("Couldn't parse input file!");
    let null_positions = count_null_transitions(START_POSITION, instructions);
    println!("Null transitions found: {}", null_positions);
}

pub fn count_null_transitions(start_position: u8, instruction: Vec<Instruction>) -> usize {
    let mut position = start_position;
    instruction.iter().map(|&instr| {
        count_null_pos_on_rotation(&mut position, instr)
    }).sum()
}

fn count_null_pos_on_rotation(pos: &mut u8, instruction: Instruction) -> usize {
    let old_pos = *pos;
    if instruction.dir == Direction::Right {
        *pos = ((old_pos as usize + instruction.count) % 100) as u8;
        return (old_pos as usize + instruction.count) / 100
    }
    // First calculate negative position to get the amount of wraps
    let mut new_pos = old_pos as isize - instruction.count as isize;
    let wraps = match new_pos {
        ..0 => {
            let mut new_transitions = (-(new_pos / 100)) as usize;
            if old_pos > 0 { new_transitions += 1; }
            new_transitions
        },
        0 => 1,
        _ => 0,
    };

    // Modulo for real position
    new_pos = (new_pos % 100 + 100) % 100;
    *pos = new_pos as u8;
    wraps
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn check_correct_nullpositions() {
        const START_POSITION: u8 = 50;
        const EXPECTED_NULL_POSITIONS: usize = 6;
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
        assert_eq!(count_null_transitions(START_POSITION, test_instructions), EXPECTED_NULL_POSITIONS);
    }

}