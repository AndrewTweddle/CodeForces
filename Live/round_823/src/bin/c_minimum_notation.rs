use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let mut line_iter = stdin_lock.lines();
    let t = line_iter
        .next()
        .unwrap()
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    for _ in 0..t {
        // read string s and convert its UTF-8 digits to bytes
        let s = line_iter.next().unwrap().unwrap();
        let mut digits: Vec<u8> = s.chars().map(|ch| ch as u8 - b'0').collect();

        sort_digits_lexicographically(&mut digits);

        // Convert bytes back to UTF-8 string
        for d in &mut digits {
            *d += b'0'
        }
        let output = String::from_utf8(digits).unwrap();

        println!("{}", output);
    }
}

fn sort_digits_lexicographically(digits: &mut Vec<u8>) {
    let mut num_sorted = 0;
    for next_digit in 0_u8..9 {
        num_sorted += sort_slice_by_digit(&mut digits[num_sorted..], next_digit);

        if num_sorted == digits.len() {
            return;
        }
    }
}

// this returns the number of digits in the slice that are sorted
fn sort_slice_by_digit(slice: &mut [u8], digit: u8) -> usize {
    let mut digits_found = 0;
    let mut sub_slice = &mut slice[0..];

    while let Some(subslice_digit_ix) = sub_slice.iter().position(|d| *d == digit) {
        if subslice_digit_ix != 0 {
            // All digits up to (but not including) the digit in
            // position d_ix will be moved after that digit
            move_digits_before_pos_after_it(sub_slice, subslice_digit_ix);

            // if we were at the final digit, then there is nothing more to sort (for any digits)
            if subslice_digit_ix == sub_slice.len() - 1 {
                // indicate that all digits in the slice have been sorted...
                return slice.len() - 1;
            }
        }

        digits_found += 1;
        sub_slice = &mut slice[digits_found..];
    }

    digits_found
}

fn move_digits_before_pos_after_it(slice: &mut [u8], mut pos: usize) {
    // Note that pos points to the first digit in the slice
    // that should not be moved later in the sequence.
    // All digits before it will be moved somewhere after it.

    // Bump the values of all the digits (except 9) that will be moved
    slice.iter_mut().take(pos).for_each(|digit| {
        if *digit != 9 {
            *digit += 1;
        }
    });

    // Sort them now. This will make it easier to move them in batches
    slice[0..pos].sort();

    // Track the first position at which we might want to push the next block
    let mut last_lower_pos = pos;

    // Keep moving blocks of equal digits after pos, sliding pos forward each time
    while pos != 0 {
        // Calculate the next block of identical digits
        let block_digit = slice[0];
        let mut block_size = 0;
        while slice[block_size] == block_digit {
            block_size += 1;
        }

        // Find the index of the last later digit that is less than the block's digit.
        let rel_last_lower_pos = slice[last_lower_pos..]
            .iter()
            .rposition(|&lower_digit| lower_digit < block_digit)
            .unwrap();
        last_lower_pos += rel_last_lower_pos;

        // rotate the block into position
        slice[0..=last_lower_pos].rotate_left(block_size);
        pos -= block_size;
    }
}
