use fastrand::u8;
use std::ops::RangeBounds;

mod final_list;
use final_list::build_final_list;
mod pico_fermi_result;
use pico_fermi_result::CheckResult;


const RED: &str = "\u{1F534}";
const YELLOW: &str = "\u{1F7E1}";
const GREEN: &str = "\u{1F7E2}";

fn main() {
    let final_list = build_final_list(4, 0, 9, true);
    let user_list = build_final_list(4, 0, 9, true);
    println!("final_list: {:?}", final_list);
    print!("user_list: {:?}", user_list);
    let result = CheckResult::check_with_final_list(&user_list, &final_list);
    print!(" -==- ");
    print!("{:?}", result.get_result());
}
