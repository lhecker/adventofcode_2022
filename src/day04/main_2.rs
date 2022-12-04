use anyhow::{bail, Result};
use std::io::BufRead;
use std::ops::RangeInclusive;

fn parse_range(s: &str) -> Result<RangeInclusive<u8>> {
    let (left, right) = match s.split_once('-') {
        Some(v) => v,
        None => bail!("expected - separator"),
    };

    let from = left.parse()?;
    let to = right.parse()?;
    Ok(from..=to)
}

fn overlap_range(a: &RangeInclusive<u8>, b: &RangeInclusive<u8>) -> RangeInclusive<u8> {
    let from = a.start().max(b.start());
    let to = a.end().min(b.end());
    *from..=*to
}

fn main() -> Result<()> {
    let stdin = std::io::stdin().lock();
    let mut total_overlaps = 0usize;

    for line in stdin.lines() {
        let line = line?;

        let (left, right) = match line.split_once(',') {
            Some(v) => v,
            None => bail!("expected , separator"),
        };

        let r1 = parse_range(left)?;
        let r2 = parse_range(right)?;
        let r = overlap_range(&r1, &r2);

        if !r.is_empty() {
            total_overlaps += 1;
        }
    }

    println!("{}", total_overlaps);
    Ok(())
}
