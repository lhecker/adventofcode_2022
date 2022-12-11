use anyhow::Result;

pub fn day10() -> Result<(i32, String)> {
    let input = include_str!("input.txt");
    let mut cycle = 0i32;
    let mut value = 1i32;
    let mut total1 = 0i32;
    let mut screen = Vec::from(concat!(
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
        "                                        \n",
    ));

    for line in input.lines() {
        let mut pending = 0;
        let mut latency = 1;

        if let Some(s) = line.strip_prefix("addx ") {
            pending = s.parse::<i32>()?;
            latency = 2;
        }

        for _ in 0..latency {
            // Part 2
            let x = cycle % 40;
            if (value - 1..=value + 1).contains(&x) {
                let y = cycle / 40;
                screen[(y * 41 + x) as usize] = b'#';
            }

            cycle += 1;

            // Part 1
            if cycle >= 20 && (cycle - 20) % 40 == 0 {
                total1 += cycle * value;
            }
        }

        value += pending;
    }

    // We know we're dealing with just 3 characters: '\n', ' ' and '#'.
    let screen = unsafe { String::from_utf8_unchecked(screen) };
    Ok((total1, screen))
}
