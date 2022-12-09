use anyhow::Result;

fn sort<T: Ord + Copy>(s: &mut [T], a: usize, b: usize) {
    let va = s[a];
    let vb = s[b];
    s[a] = va.min(vb);
    s[b] = va.max(vb);
}

pub fn day01() -> Result<(usize, usize)> {
    let input = include_str!("input.txt");
    let mut accumulator = 0usize;
    let mut maximas = [0usize; 3];

    for line in input.lines() {
        if line.is_empty() {
            maximas[2] = maximas[2].max(accumulator);
            sort(&mut maximas, 2, 1);
            sort(&mut maximas, 1, 0);
            accumulator = 0;
        } else {
            accumulator += line.parse::<usize>().expect("integer");
        }
    }

    Ok((maximas[0], maximas.iter().sum::<usize>()))
}
