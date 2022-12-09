use adventofcode_2022::*;
use anyhow::{bail, Result};

macro_rules! gen_executor {
    ($($day:ident,)*) => {
        [
            $(
                || -> Result<()> {
                    let (p1, p2) = $day()?;
                    println!("{}", p1);
                    println!("{}", p2);
                    Ok(())
                },
            )*
        ]
    };
}

fn main() -> Result<()> {
    let days = foreach_day!(gen_executor);

    if let Some(sel) = std::env::args().nth(1) {
        let idx: usize = sel.parse()?;
        if let Some(f) = idx.checked_sub(1).and_then(|i| days.get(i)) {
            f()?;
        } else {
            bail!("expected day between 1 and {}", days.len() + 1);
        }
    } else {
        for (i, f) in days.iter().enumerate() {
            if i != 0 {
                println!();
            }
            println!("# day {}", i + 1);
            f()?;
        }
    }

    Ok(())
}
