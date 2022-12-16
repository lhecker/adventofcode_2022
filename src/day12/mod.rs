use anyhow::{bail, Result};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    pub x: i32,
    pub y: i32,
}

pub fn day12() -> Result<(usize, usize)> {
    let input = include_bytes!("input.txt");

    let Some(width) = input.iter().position(|&ch| ch == b'\n') else {
        bail!("failed to determine row width");
    };
    let width = width;
    let stride = width + 1;
    let height = input.len() / stride;

    let Some(start_pos) = input.iter().position(|&ch | ch == b'S') else {
        bail!("failed to find S");
    };
    let Some(end_pos) = input.iter().position(|&ch | ch == b'E') else {
        bail!("failed to find E");
    };

    let mut visited = vec![false; width * height];
    let mut stack = VecDeque::with_capacity(width + height);

    let width = width as i32;
    let stride = stride as i32;
    let height = height as i32;
    let start_pos = start_pos as i32;
    let end_pos = end_pos as i32;
    let start = Coord {
        x: start_pos % stride,
        y: start_pos / stride,
    };
    let end = Coord {
        x: end_pos % stride,
        y: end_pos / stride,
    };

    let mut visit_at =
        |pos: Coord| std::mem::replace(&mut visited[(pos.y * width + pos.x) as usize], true);
    let height_at = |pos: Coord| {
        if !(0..width).contains(&pos.x) || !(0..height).contains(&pos.y) {
            return i32::MIN;
        }
        match input[(pos.y * stride + pos.x) as usize] {
            b'S' => 0,
            b'E' => 25,
            ch => (ch - b'a') as i32,
        }
    };

    visit_at(end);
    stack.push_back((end, 0));

    let mut remaining_goals = 2;
    let mut distance_s = 0;
    let mut distance_a = 0;

    loop {
        let Some((pos, distance)) = stack.pop_front() else {
            bail!("empty stack");
        };

        let pos_h = height_at(pos);

        if pos == start {
            distance_s = distance;
            remaining_goals -= 1;
        }
        if pos_h == 0 && distance_a == 0 {
            distance_a = distance;
            remaining_goals -= 1;
        }
        if remaining_goals == 0 {
            break;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let c = Coord {
                x: pos.x + dx,
                y: pos.y + dy,
            };
            let h = height_at(c);
            if h >= pos_h - 1 && !visit_at(c) {
                stack.push_back((c, distance + 1));
            }
        }
    }

    Ok((distance_s, distance_a))
}
