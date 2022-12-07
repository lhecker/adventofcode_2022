use adventofcode_2022::Measure;
use anyhow::Result;

fn main() -> Result<()> {
    let measure = Measure::new();

    let mut priorities = [0u8; 128];
    for ch in b'a'..=b'z' {
        priorities[ch as usize] = ch - b'a' + 1;
    }
    for ch in b'A'..=b'Z' {
        priorities[ch as usize] = ch - b'A' + 27;
    }

    let input = include_str!("input.txt");
    let mut duplicates = [0u8; 256];
    let mut score = 0usize;
    let mut index = 1u8;

    for line in input.lines() {
        for &ch in line.as_bytes() {
            duplicates[ch as usize] |= index;
        }

        index <<= 1;
        if index == 0b1000 {
            let idx = duplicates.iter().position(|&ch| ch == 0b111).unwrap_or(0);
            score += priorities[idx] as usize;
            duplicates.fill(0);
            index = 1;
        }
    }

    drop(measure);
    println!("{}", score);
    Ok(())
}
