use anyhow::Result;
use std::io::Read;

fn find_marker(input: &[u8], length: usize) -> usize {
    // Count the number of `duplicates` by keeping a table of each character's frequency in the
    // window of the last `length` characters. We can detect a duplicate character if the
    // corresponding slot in `frequencies` is 2 ore more. This only works for ASCII text of course.
    let mut frequencies = [0u8; 256];
    let mut duplicates = 0usize;
    let mut distance = 0usize;

    for &ch in input {
        // While this detects duplicate characters as described above...
        frequencies[ch as usize] += 1;
        if frequencies[ch as usize] > 1 {
            duplicates += 1;
        }

        // ...this is the inverse and removes characters that have
        // now fallen outside of the window of the last `length` characters.
        if distance >= length {
            let pch = input[distance - length];
            if frequencies[pch as usize] > 1 {
                duplicates -= 1;
            }
            frequencies[pch as usize] -= 1;
        }

        distance += 1;
        if distance >= 4 && duplicates == 0 {
            break;
        }
    }

    distance
}

fn main() -> Result<()> {
    let mut input = Vec::new();
    std::io::stdin().lock().read_to_end(&mut input)?;
    println!("start-of-packet: {}", find_marker(&input, 4));
    println!("start-of-message: {}", find_marker(&input, 14));
    Ok(())
}
