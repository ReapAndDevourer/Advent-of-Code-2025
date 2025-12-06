pub fn get_battery_level_rows(input: String) -> Vec<Vec<u8>> {
    let mut lines = Vec::<Vec<u8>>::new();
    for line in input.lines() {
        lines.push(
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("Provided char input doesnt match a number!")
                        as u8
                })
                .collect::<Vec<u8>>(),
        );
    }
    lines
}
