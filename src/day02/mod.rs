use anyhow::{bail, Result};

pub fn day02() -> Result<(usize, usize)> {
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
    const GAME_MATRIX1: [[u8; 3]; 3] = [
        [3, 6, 0],
        [0, 3, 6],
        [6, 0, 3],
    ];

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
    const GAME_MATRIX2: [[u8; 3]; 3] = [
        [3, 1, 2],
        [1, 2, 3],
        [2, 3, 1],
    ];

    let input = include_str!("input.txt");
    let mut score1 = 0usize;
    let mut score2 = 0usize;

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

        score1 += GAME_MATRIX1[them][me] as usize;
        score1 += me + 1;

        score2 += GAME_MATRIX2[them][me] as usize;
        score2 += me * 3;
    }

    Ok((score1, score2))
}
