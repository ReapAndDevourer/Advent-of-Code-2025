pub fn power_score_sum<const BATTERY_SLOTS: usize>(power_lines: Vec<Vec<u8>>) -> u64 {
    power_lines
        .iter()
        .map(|line| max_power_score::<BATTERY_SLOTS>(line))
        .sum()
}

fn max_power_score<const N_BATTERIES: usize>(battery_levels: &Vec<u8>) -> u64 {
    let mut max_levels: [u8; N_BATTERIES] = [0; N_BATTERIES];
    for &battery in battery_levels {
        // Create temporary slice with size one bigger that contains new battery
        let mut temp = max_levels.to_vec();
        temp.push(battery);
        // check if a higher number is created if the bytes rotate
        for i in 0..(temp.len() - 1) {
            let window = &temp[i..i + 2];
            if window[1] > window[0] {
                temp[i..].rotate_left(1);
                max_levels.copy_from_slice(&temp[..N_BATTERIES]);
                break;
            }
        }
    }
    convert_slice_to_number(max_levels)
}

fn convert_slice_to_number<const N_BATTERIES: usize>(levels: [u8; N_BATTERIES]) -> u64 {
    let mut number: u64 = 0;
    for b in levels {
        number = number * 10 + b as u64;
    }
    number
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 98)]
    #[case([8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 89)]
    #[case([2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 78)]
    #[case([8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 92)]
    pub fn test_max_power_score_pt01<const N: usize>(#[case] input: [u8; N], #[case] expected: u64) {
        assert_eq!(max_power_score::<2>(&input.to_vec()), expected);
    }

    #[test]
    pub fn test_power_score_sum_pt01() {
        let test_input: Vec<Vec<u8>> = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
        const EXPECTED_SUM: u64 = 357;
        assert_eq!(power_score_sum::<2>(test_input), EXPECTED_SUM)
    }

        #[rstest]
    #[case([9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 987654321111)]
    #[case([8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 811111111119)]
    #[case([2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 434234234278)]
    #[case([8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 888911112111)]
    pub fn test_max_power_score_pt02<const N: usize>(#[case] input: [u8; N], #[case] expected: u64) {
        assert_eq!(max_power_score::<12>(&input.to_vec()), expected);
    }

    #[test]
    pub fn test_power_score_sum_pt02() {
        let test_input: Vec<Vec<u8>> = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
        const EXPECTED_SUM: u64 = 3121910778619;
        assert_eq!(power_score_sum::<12>(test_input), EXPECTED_SUM)
    }
}
