use anyhow::{bail, Result};

pub fn day09() -> Result<(usize, usize)> {
    let input = include_str!("input.txt");

    const WIDTH: i32 = 768;
    const HEIGHT: i32 = 512;

    const N: i32 = 0;
    const L: i32 = -1;
    const R: i32 = 1;
    const U: i32 = -1;
    const D: i32 = 1;
    const MOVES: [[(i32, i32); 5]; 5] = [
        [(L, U), (L, U), (N, U), (R, U), (R, U)],
        [(L, U), (N, N), (N, N), (N, N), (R, U)],
        [(L, N), (N, N), (N, N), (N, N), (R, N)],
        [(L, D), (N, N), (N, N), (N, N), (R, D)],
        [(L, D), (L, D), (N, D), (R, D), (R, D)],
    ];

    let mut total1 = 1usize;
    let mut total2 = 1usize;
    let mut knots = [(WIDTH / 2, HEIGHT / 2); 10];
    let mut visited = vec![0u8; (WIDTH * HEIGHT) as usize];
    let mut visit = |pos: (i32, i32), flags: u8| {
        let offset = (pos.1 * WIDTH + pos.0) as usize;
        if visited[offset] & flags == 0 {
            visited[offset] |= flags;
            1
        } else {
            0
        }
    };

    // Visit the initial center
    visit(knots[0], 3);

    for line in input.lines() {
        let ascii = line.as_bytes();
        if ascii.len() < 3 {
            bail!("invalid input: {}", line);
        }

        let d = match ascii[0] {
            b'L' => (L, 0),
            b'R' => (R, 0),
            b'U' => (0, U),
            b'D' => (0, D),
            _ => bail!("dir"),
        };
        let steps: u32 = line[2..].parse()?;

        for _ in 0..steps {
            knots[0].0 += d.0;
            knots[0].1 += d.1;

            if !(0..WIDTH).contains(&knots[0].0) || !(0..HEIGHT).contains(&knots[0].1) {
                bail!("map too small - location {} {}", knots[0].0, knots[0].1);
            }

            for i in 1..knots.len() {
                let h = knots[i - 1];
                let t = knots[i];
                // Using unchecked access here makes it run roughly 15% faster.
                let d = unsafe {
                    MOVES
                        .get_unchecked((h.1 - t.1 + 2) as usize)
                        .get_unchecked((h.0 - t.0 + 2) as usize)
                };
                knots[i].0 += d.0;
                knots[i].1 += d.1;
            }

            total1 += visit(knots[1], 1);
            total2 += visit(knots[9], 2);
        }
    }

    Ok((total1, total2))
}
