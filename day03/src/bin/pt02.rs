use day03::input_parser;
use day03::power_calculator;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read input file!");
    let battery_packs = input_parser::get_battery_level_rows(input);
    print!(
        "Sum of highest battery scores: {}",
        power_calculator::power_score_sum::<12>(battery_packs)
    );
}
