pub fn string_to_ranges(input: String) -> Vec<(usize, usize)> {
    let line = input
        .lines()
        .next()
        .expect("Input didn't contain single line!");
    let line_elements = line.split(',');
    line_elements
        .map(|range_string| {
            let (s_lower, s_higher) = range_string
                .split_once('-')
                .expect("Line contained element with incorrect format!");
            (
                s_lower
                    .parse()
                    .expect("Lower bound string is not a number!"),
                s_higher
                    .parse()
                    .expect("Higher bound string ist not a number!"),
            )
        })
        .collect::<Vec<(usize, usize)>>()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn input_parse() {
        let input_line: &str = concat!(
            "11-22,95-115,998-1012,",
            "1188511880-1188511890,222220-222224,1698522-1698528,",
            "446443-446449,38593856-38593862"
        );
        let expected_ranges: Vec<(usize, usize)> = vec![
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
        ];
        let created_ranges = string_to_ranges(input_line.to_string());
        assert_eq!(created_ranges, expected_ranges);
    }
}
