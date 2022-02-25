use std::io;

pub(crate) fn get_guess(guess_size: usize, low_range: i16, high_range: i16) -> Vec<i16> {
    let mut guess_list = Vec::with_capacity(guess_size);

    // TODO Implement parsing better. Maybe split by ascii whitespace?
    let mut str_guess = String::new();

    io::stdin()
        .read_line(&mut str_guess)
        .expect("Couldn't read line");

    let mut list_str_guess = str_guess.split_whitespace();
    // TODO Don't use panic!s
    while let Some(next_string) = list_str_guess.next() {
        if guess_list.len() > guess_size {
            panic!("Too many numbers inputted");
        }
        let guess: i16 = match next_string.parse() {
            Ok(num) => num,
            Err(err) => panic!("Couldn't read number in guess: {}", err),
        };
        if guess < low_range || guess > high_range {
            panic!("Number ({}) given is not between the proper range ({} - {})", guess, low_range, high_range);
        }
        guess_list.push(guess);
    }

    return guess_list;
}