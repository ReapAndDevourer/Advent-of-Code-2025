use day02::id_checker;
use day02::input_parser;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read input file!");
    let ranges = input_parser::string_to_ranges(input);
    print!(
        "Sum of illegal IDs: {}",
        id_checker::pt02::sum_illegal_ids(ranges)
    );
}
