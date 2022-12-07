use anyhow::{bail, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let mut lines = input.lines();

    // Read the initial state. The input might not be a perfectly rectangular matrix.
    let mut initial_state = Vec::new();
    for line in &mut lines {
        let ascii = line.as_bytes();
        if ascii.is_empty() {
            break;
        }
        initial_state.push(ascii);
    }

    let line_length = initial_state.iter().map(|&s| s.len()).max().unwrap_or(0);
    let stacks_len = (line_length + 2) / 4;

    // The last line in the initial state input is just a list of "1 2 3 ..." indices.
    initial_state.pop();

    // Transpose the matrix into a list of stacks.
    let mut stacks9000 = Vec::with_capacity(stacks_len);
    for i in 0..stacks_len {
        let i = i * 4 + 1;
        let mut stack = Vec::new();
        for line in initial_state.iter().rev() {
            if i >= line.len() {
                break;
            }
            let ch = line[i];
            if !(b'A'..=b'Z').contains(&ch) {
                break;
            }
            stack.push(ch);
        }
        stacks9000.push(stack);
    }

    let mut stacks9001 = stacks9000.clone();

    // Process the "move N from A to B" instructions.
    for line in &mut lines {
        let mut it = line.split_ascii_whitespace();
        if it.next() != Some("move") {
            bail!("expected 'move'");
        }
        let amount = match it.next() {
            Some(s) => s.parse::<usize>()?,
            None => bail!("expected amount"),
        };
        if it.next() != Some("from") {
            bail!("expected 'from'");
        }
        let from = match it.next() {
            Some(s) => s.parse::<usize>()?.wrapping_sub(1),
            None => bail!("expected from"),
        };
        if it.next() != Some("to") {
            bail!("expected 'to'");
        }
        let to = match it.next() {
            Some(s) => s.parse::<usize>()?.wrapping_sub(1),
            None => bail!("expected to"),
        };

        if from == to || from >= stacks9000.len() || to >= stacks9000.len() {
            bail!("from/to are identical or out of bounds");
        }

        // CrateMover 9000
        {
            // Borrowing two different items of a `Vec` at the same time is not trivially possible.
            // While the use of `unsafe` in Rust is heavily discouraged, I personally don't feel like
            // the usual suggestion of abusing `split_at()` is any better. The above `if` condition
            // ensures that `from` and `to` are different indices and within the bounds of `stacks`.
            let from_stack = unsafe { &mut *(stacks9000.get_unchecked_mut(from) as *mut Vec<u8>) };
            let to_stack = unsafe { &mut *(stacks9000.get_unchecked_mut(to) as *mut Vec<u8>) };

            for _ in 0..amount {
                if let Some(ch) = from_stack.pop() {
                    to_stack.push(ch);
                }
            }
        }

        // CrateMover 9001
        {
            // Same as above.
            let from_stack = unsafe { &mut *(stacks9001.get_unchecked_mut(from) as *mut Vec<u8>) };
            let to_stack = unsafe { &mut *(stacks9001.get_unchecked_mut(to) as *mut Vec<u8>) };

            let offset = from_stack.len().saturating_sub(amount);
            to_stack.extend(from_stack.drain(offset..));
        }
    }

    let topmost_crates = |stacks: &Vec<Vec<u8>>| {
        stacks
            .iter()
            .map(|v| match v.last() {
                Some(b) => char::from(*b),
                None => ' ',
            })
            .collect::<String>()
    };

    println!("CrateMover9000: {}", topmost_crates(&stacks9000));
    println!("CrateMover9001: {}", topmost_crates(&stacks9001));
    Ok(())
}
