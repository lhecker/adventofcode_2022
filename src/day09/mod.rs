use anyhow::{bail, Result};

pub fn day09() -> Result<(usize, usize)> {
    let input = include_str!("input.txt");

    const WIDTH: i32 = 768;
    const HEIGHT: i32 = 512;

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
            b'L' => (-1, 0),
            b'R' => (1, 0),
            b'U' => (0, -1),
            b'D' => (0, 1),
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
                let dx = h.0 - t.0;
                let dy = h.1 - t.1;
                // Normally a `dx.clamp(-1, 1)` (or `dy...`) would be sufficient,
                // but I found that a LUT is significantly faster.
                // Using unchecked access here makes it run roughly 5-10% faster. Rust doesn't
                // know nor understand the the game fundamentally doesn't allow a dx/dy of >2.
                const LUT: [i32; 5] = [-1, -1, 0, 1, 1];
                let dx_clamped = unsafe { *LUT.get_unchecked((dx + 2) as usize) };
                let dy_clamped = unsafe { *LUT.get_unchecked((dy + 2) as usize) };
                // Equivalent to `if dx.abs() > 1 || dy.abs() > 1` but slightly faster.
                if dx != dx_clamped || dy != dy_clamped {
                    knots[i].0 += dx_clamped;
                    knots[i].1 += dy_clamped;
                }
            }

            total1 += visit(knots[1], 1);
            total2 += visit(knots[9], 2);
        }
    }

    Ok((total1, total2))
}
