use anyhow::Result;

fn main() -> Result<()> {
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

    println!("{}", maximum);
    Ok(())
}
