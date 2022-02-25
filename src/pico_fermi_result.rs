use std::collections::VecDeque;

pub struct CheckResult {
    result: VecDeque<char>,
}

impl CheckResult {
    pub fn check_with_final_list(user_list: &Vec<i16>, final_list: &Vec<i16>) -> Self {
        // TODO Check that user_list is equal to final_list; maybe this check should occur when handling user_input
        let mut result: VecDeque<char> = VecDeque::with_capacity(final_list.len());
        // TODO Don't assume unique values in {user,final}_list
        // TODO Don't use raw characters; should use some enum for future GUI work
        for i in 0..user_list.len() {
            // Fermi values should be in the front
            if user_list[i] == final_list[i] {
                result.push_front('F');
            }
            // Pico values should be after the Fermi values
            else if user_list[i] != final_list[i] && final_list.contains(&user_list[i]) {
                result.push_back('P');
            }
        }
        // Everything after the Fermi values should be Null
        while result.len() != final_list.len() {
            result.push_back('-');
        }
        return CheckResult {
            result,
        };
    }

    pub fn get_result(&self) -> VecDeque<char> {
        self.result.clone()
    }
}
