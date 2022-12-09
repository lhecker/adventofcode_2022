use anyhow::{bail, Result};

pub fn day03() -> Result<(usize, usize)> {
    let mut priorities = [0u8; 128];
    for ch in b'a'..=b'z' {
        priorities[ch as usize] = ch - b'a' + 1;
    }
    for ch in b'A'..=b'Z' {
        priorities[ch as usize] = ch - b'A' + 27;
    }

    let input = include_str!("input.txt");
    let mut duplicates1 = [false; 256];
    let mut duplicates2 = [0u8; 256];
    let mut score1 = 0usize;
    let mut score2 = 0usize;
    let mut index = 1u8;

    for line in input.lines() {
        let ascii = line.as_bytes();
        if ascii.len() & 1 != 0 {
            bail!("expected line of even length");
        }

        // Part 1
        {
            let (left, right) = ascii.split_at(ascii.len() / 2);
            for &ch in left {
                duplicates1[ch as usize] = true;
            }

            score1 += right
                .iter()
                .find(|&&ch| duplicates1[ch as usize])
                .map(|&ch| priorities[ch as usize] as usize)
                .unwrap_or(0);

            duplicates1.fill(false);
        }

        // Part 2
        {
            for &ch in line.as_bytes() {
                duplicates2[ch as usize] |= index;
            }

            index <<= 1;
            if index == 0b1000 {
                score2 += duplicates2
                    .iter()
                    .position(|&ch| ch == 0b111)
                    .map(|ch| priorities[ch] as usize)
                    .unwrap_or(0);
                duplicates2.fill(0);
                index = 1;
            }
        }
    }

    Ok((score1, score2))
}
