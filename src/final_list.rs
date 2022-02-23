#[cfg(test)]
#[path = "./test_final_list.rs"]
mod test_final_list;


pub fn build_final_list(list_size: usize, low_range: i16, high_range: i16, unique_nums: bool) -> Vec<i16> {
    let mut final_list = Vec::with_capacity(list_size);
    // Check if the range is nonsensical
    if low_range >= high_range {
        panic!("The lower range is less than or equal to the higher range, low: {}; high: {}", low_range, high_range);
    }
    // Check if the range is less than the list_size
    // If the numbers need to be unique, this could be a problem
    if unique_nums && (high_range - low_range) < list_size as i16 {
        panic!("If the numbers are unique, the range cannot be less than the size of the final list");
    }
    for _ in 0..list_size {
        let mut temp = fastrand::i16(low_range..high_range);
        if unique_nums {
            while final_list.contains(&temp) {
                temp = fastrand::i16(low_range..high_range);
            }
        }
        final_list.push(temp);
    }

    return final_list;
}
