use adventofcode_2022::Measure;
use anyhow::{bail, Result};

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
    let mut duplicates = [false; 256];
    let mut score = 0usize;

    for line in input.lines() {
        let ascii = line.as_bytes();
        if ascii.len() & 1 != 0 {
            bail!("expected line of even length");
        }

        let (left, right) = ascii.split_at(ascii.len() / 2);
        for &ch in left {
            duplicates[ch as usize] = true;
        }

        score += right
            .iter()
            .find(|&&ch| duplicates[ch as usize])
            .map(|&ch| priorities[ch as usize] as usize)
            .unwrap_or(0);
    }

    drop(measure);
    println!("{}", score);
    Ok(())
}
