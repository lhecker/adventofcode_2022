use anyhow::{bail, Result};

struct Puzzle1<'a> {
    input: &'a [u8],
    visited: Vec<bool>,
    scenic_scores: Vec<usize>,
    part1: usize,
    part2: usize,
}

impl<'a> Puzzle1<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            visited: vec![false; input.len()],
            scenic_scores: vec![1; input.len()],
            part1: 0,
            part2: 0,
        }
    }

    fn process<I: Iterator<Item = usize>>(&mut self, it: I) {
        let mut dist = [0usize; 10];
        let mut max_h = 0;

        for (i, offset) in it.enumerate() {
            let h = self.input[offset];

            // Part 1
            if h > max_h {
                if !self.visited[offset] {
                    self.visited[offset] = true;
                    self.part1 += 1;
                }
                max_h = h;
            }

            // Part 2
            let idx = (h - b'0') as usize;
            self.scenic_scores[offset] *= i - dist[idx];
            self.part2 = self.part2.max(self.scenic_scores[offset]);
            dist[..=idx].fill(i);
        }
    }

    fn result(&self) -> (usize, usize) {
        (self.part1, self.part2)
    }
}

pub fn day08() -> Result<(usize, usize)> {
    let input = include_bytes!("input.txt");
    let Some(width) = input.iter().position(|&ch| ch == b'\n') else {
        bail!("failed to determine row width");
    };
    let stride = width + 1;
    if width * stride != input.len() {
        bail!("expected input to be square with trailing newlines");
    }

    let mut p = Puzzle1::new(input);

    // left -> right
    (0..width)
        .map(|i| i * stride)
        .map(|offset| offset..(offset + width))
        .for_each(|r| p.process(r));
    // right -> left
    (0..width)
        .map(|i| i * stride)
        .map(|offset| (offset..(offset + width)).rev())
        .for_each(|r| p.process(r));
    // top -> bottom
    (0..width)
        .map(|x| (0..width).map(move |y| y * stride + x))
        .for_each(|r| p.process(r));
    // bottom -> top
    (0..width)
        .map(|x| (0..width).rev().map(move |y| y * stride + x))
        .for_each(|r| p.process(r));

    Ok(p.result())
}
