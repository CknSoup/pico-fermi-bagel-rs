use fastrand::u8;
use std::ops::RangeBounds;

mod final_list;
use final_list::build_final_list;
mod pico_fermi_result;
use pico_fermi_result::CheckResult;
mod user_input;
use user_input::get_guess;


const RED: &str = "\u{1F534}";
const YELLOW: &str = "\u{1F7E1}";
const GREEN: &str = "\u{1F7E2}";

const LIST_SIZE: usize = 4;
const LOW_RANGE: i16 = 0;
const HIGH_RANGE: i16 = 9;
const UNIQUE_NUMS: bool = true;
const GUESSES: usize = 8;

fn main() {
    let final_list = build_final_list(LIST_SIZE, LOW_RANGE, HIGH_RANGE, UNIQUE_NUMS);
    let guesses: usize = GUESSES;

    let mut i = 0;
    while i < guesses {
        let user_guess = get_guess(LIST_SIZE, LOW_RANGE, HIGH_RANGE);
        let result = CheckResult::check_with_final_list(&user_guess, &final_list);
        // print!(" -=- ");
        println!("{:?}", result.get_result());
        if format!("{:?}", result.get_result()) == "['F', 'F', 'F', 'F']" {
            println!("\n{:?}\nYou Win!", final_list);
            return;
        }
        i += 1;
    }

    println!("\n{:?}\nYou Lose", final_list);
}
