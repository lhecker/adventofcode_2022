use adventofcode_2022::Measure;
use anyhow::{bail, Result};

fn main() -> Result<()> {
    let measure = Measure::new();

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
        // indices ↓ them:
        //   0 Rock
        //   1 Paper
        //   2 Scissor
        // indices → me:
        //   0 lose
        //   1 draw
        //   2 win
        // results:
        //   1 Rock
        //   2 Paper
        //   3 Scissor
        #[rustfmt::skip]
        const GAME_MATRIX: [[u8; 3]; 3] = [
            [3, 1, 2],
            [1, 2, 3],
            [2, 3, 1],
        ];

        score_total += GAME_MATRIX[them][me] as usize;
        score_total += me * 3;
    }

    drop(measure);
    println!("{}", score_total);
    Ok(())
}
