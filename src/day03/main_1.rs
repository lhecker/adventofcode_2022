use anyhow::{bail, Result};
use std::io::BufRead;

fn main() -> Result<()> {
    let mut priorities = [0u8; 128];
    for ch in b'a'..=b'z' {
        priorities[ch as usize] = ch - b'a' + 1;
    }
    for ch in b'A'..=b'Z' {
        priorities[ch as usize] = ch - b'A' + 27;
    }

    let stdin = std::io::stdin().lock();
    let mut duplicates = [0u8; 128];
    let mut score = 0usize;

    for line in stdin.lines() {
        let line = line?;
        let ascii = line.as_bytes();
        if ascii.len() & 1 != 0 {
            bail!("expected line of even length");
        }

        duplicates.fill(0);

        let (left, right) = ascii.split_at(ascii.len() / 2);
        for ch in left {
            duplicates[(*ch & 0x7f) as usize] |= 0b01;
        }
        for ch in right {
            duplicates[(*ch & 0x7f) as usize] |= 0b10;
        }

        let idx = duplicates.iter().position(|&ch| ch == 0b11).unwrap_or(0);
        score += priorities[idx] as usize;
    }

    println!("{}", score);
    Ok(())
}
