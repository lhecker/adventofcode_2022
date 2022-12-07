use anyhow::{bail, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let mut score_total = 0usize;

    for line in input.lines() {
        let bytes = line.as_bytes();
        if bytes.len() != 3 {
            continue;
        }

        if bytes[1] != b' ' {
            bail!("expected space separator");
        }

        let them = match bytes[0] {
            b'A' => 0,
            b'B' => 1,
            b'C' => 2,
            _ => bail!("expected ABC"),
        };
        let me = match bytes[2] {
            b'X' => 0,
            b'Y' => 1,
            b'Z' => 2,
            _ => bail!("expected XYZ"),
        };

        // WIN_MATRIX[them][me]
        // ↓ them
        // → me
        // indices:
        //   0 Rock
        //   1 Paper
        //   2 Scissor
        // results:
        //   0 lose
        //   3 draw
        //   6 win
        #[rustfmt::skip]
        const GAME_MATRIX: [[u8; 3]; 3] = [
            [3, 6, 0],
            [0, 3, 6],
            [6, 0, 3],
        ];

        score_total += GAME_MATRIX[them][me] as usize;
        score_total += me + 1;
    }

    println!("{}", score_total);
    Ok(())
}
