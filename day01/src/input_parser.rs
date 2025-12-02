#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Instruction {
    pub dir: Direction,
    pub count: usize,
}

pub fn parse(input: &str) -> Result<Vec<Instruction>, std::io::Error> {
    let instructions = input.lines()
        .filter_map(|line| {
            if line.len() < 2 {
                return None
            }
            let (dir_string, count_string) = line.split_at(1);
            let dir = match dir_string {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => return None,
            };
            let count: usize = match count_string.trim().parse() {
                Ok(count) => count,
                Err(_) => return None,
            };
            // Here als skip Instruction creation for current row if parse not possible
            Some(Instruction { dir, count })
    }).collect();
    Ok(instructions)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_input_parse() {
        const TEST_INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        let expected_instructions = vec![
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
        let found_instructions = parse(TEST_INPUT)
            .expect("Error on parsing test input!");
        assert_eq!(found_instructions, expected_instructions);
    }

}