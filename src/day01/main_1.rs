use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin().lock();
    let mut accumulator = 0usize;
    let mut maximum = 0usize;

    for line in stdin.lines() {
        let line = line?;
        if line.is_empty() {
            maximum = maximum.max(accumulator);
            accumulator = 0;
        } else {
            accumulator += line.parse::<usize>().expect("integer");
        }
    }

    println!("{}", maximum);
    Ok(())
}
