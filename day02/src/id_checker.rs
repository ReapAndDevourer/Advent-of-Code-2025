pub mod pt01 {

    pub fn sum_illegal_ids(ranges: Vec<(usize, usize)>) -> usize {
        ranges
            .iter()
            .map(|&(start, end)| (start..=end).filter(|&id| id_illegal(id)).sum::<usize>())
            .sum()
    }

    fn id_illegal(id: usize) -> bool {
        let number_string = id.to_string();
        if number_string.len() % 2 != 0 {
            return false;
        }
        let (lower_part, upper_part) = number_string.split_at(number_string.len() / 2);
        lower_part == upper_part
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        pub fn test_sum_illegal_ids() {
            let input_ranges: Vec<(usize, usize)> = vec![
                (11, 22),
                (95, 115),
                (998, 1012),
                (1188511880, 1188511890),
                (222220, 222224),
                (1698522, 1698528),
                (446443, 446449),
                (38593856, 38593862),
            ];
            const EXPECTED_SUM: usize = 1227775554;
            let calculated_sum = sum_illegal_ids(input_ranges);
            assert_eq!(calculated_sum, EXPECTED_SUM);
        }
    }
}

pub mod pt02 {

    pub fn sum_illegal_ids(ranges: Vec<(usize, usize)>) -> usize {
        ranges
            .iter()
            .map(|&(start, end)| (start..=end).filter(|&id| id_illegal(id)).sum::<usize>())
            .sum()
    }

    fn id_illegal(id: usize) -> bool {
        let number_string = id.to_string();
        for parts in 2..=number_string.len() {
            if number_string.len() % parts != 0 {
                continue;
            }
            let chunk_size = number_string.len() / parts;
            let snippets: Vec<&str> = (0..parts)
                .map(|part| &number_string[part * chunk_size..(part + 1) * chunk_size])
                .collect();
            if snippets.windows(2).all(|w| w[0] == w[1]) {
                return true;
            }
        }
        return false;
    }

    
    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        pub fn test_sum_illegal_ids() {
            let input_ranges: Vec<(usize, usize)> = vec![
                (11, 22),
                (95, 115),
                (998, 1012),
                (1188511880, 1188511890),
                (222220, 222224),
                (1698522, 1698528),
                (446443, 446449),
                (38593856, 38593862),
                (565653, 565659),
                (824824821, 824824827),
                (2121212118, 2121212124),
            ];
            const EXPECTED_SUM: usize = 4174379265;
            let calculated_sum = sum_illegal_ids(input_ranges);
            assert_eq!(calculated_sum, EXPECTED_SUM);
        }
    }
}
