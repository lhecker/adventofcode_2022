use adventofcode_2022::Measure;
use anyhow::Result;

fn main() -> Result<()> {
    let measure = Measure::new();

    let input = include_str!("input.txt");
    let mut accumulator = 0usize;
    let mut maximum = 0usize;

    for line in input.lines() {
        if line.is_empty() {
            maximum = maximum.max(accumulator);
            accumulator = 0;
        } else {
            accumulator += line.parse::<usize>().expect("integer");
        }
    }

    drop(measure);
    println!("{}", maximum);
    Ok(())
}
