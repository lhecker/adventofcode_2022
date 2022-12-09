use anyhow::Result;

pub fn day04() -> Result<(usize, usize)> {
    let input = include_str!("input.txt");
    let mut buffer = [0i32; 4];
    let mut accumulator = 0;
    let mut total1 = 0usize;
    let mut total2 = 0usize;

    for line in input.split_inclusive('\n') {
        // Not using .split() and .parse() here roughly doubles throughput.
        let mut i = 0;
        for &ch in line.as_bytes() {
            if ch >= b'0' && ch <= b'9' {
                accumulator = accumulator * 10 + (ch - b'0') as i32;
            } else {
                buffer[i] = accumulator;
                accumulator = 0;
                i += 1;
            }
        }

        // Not using RangeInclusive improves throughput by roughly 10%.
        // But all of these tuples are still basically RangeInclusive instances.
        let left = (buffer[0], buffer[1]);
        let right = (buffer[2], buffer[3]);
        let overlap = (left.0.max(right.0), left.1.min(right.1));

        if overlap == left || overlap == right {
            total1 += 1;
        }
        // Since the ranges are inclusive we need to use <= to check if the range is non-empty.
        if overlap.0 <= overlap.1 {
            total2 += 1;
        }
    }

    Ok((total1, total2))
}
