use adventofcode_2022::Measure;
use anyhow::{bail, Result};

fn main() -> Result<()> {
    let measure = Measure::new();

    let input = include_str!("input.txt");
    let mut stack: Vec<usize> = Vec::new();
    let mut dir_sizes: Vec<usize> = Vec::new();

    for line in input.lines() {
        let ascii = line.as_bytes();

        // Out of the input only 3 things are of interest:
        // 1. "cd .."
        // 2. "cd <directory>"
        // 3. "123456 <filename>"
        if let Some(suffix) = line.strip_prefix("$ cd ") {
            if suffix == ".." {
                // 1. "cd .."
                let size = stack.pop().unwrap_or(0);
                dir_sizes.push(size);
                // Add the child directory's size to the parent directory.
                if let Some(last) = stack.last_mut() {
                    *last += size;
                } else {
                    bail!("empty stack");
                }
            } else {
                // 2. "cd <directory>"
                stack.push(0);
            }
        } else if !ascii.is_empty() && (b'0'..=b'9').contains(&ascii[0]) {
            // 3. "123456 <filename>"
            let Some((prefix, _)) = line.split_once(' ') else {
                bail!("expected '<filesize> <filename>'");
            };
            let Some(last) = stack.last_mut() else {
                bail!("empty stack");
            };
            // Add the file's size to the current directory.
            *last += prefix.parse::<usize>()?;
        }
    }

    // After processing the input we might not have "cd .."'d back to the
    // root directory. This loop will do just that by simulating "cd .."s.
    // It'll also net us the root dir size, which is equal to the current disk usage.
    let root_size = {
        let mut size = 0usize;
        while let Some(last) = stack.pop() {
            size += last;
            dir_sizes.push(size);
        }
        size
    };

    let max_disk_usage = 40000000usize;
    let Some(min_deletion_size) = root_size.checked_sub(max_disk_usage) else {
        bail!("root dir is small and no cleanup is needed");
    };

    let mut small_dir_size_total = 0usize;
    let mut min_dir_size = usize::MAX;
    for size in dir_sizes {
        if size <= 100000 {
            small_dir_size_total += size;
        }
        if size >= min_deletion_size && size < min_dir_size {
            min_dir_size = size;
        }
    }

    drop(measure);
    println!(
        "Total sum of directories with a size of <100000: {}",
        small_dir_size_total
    );
    println!(
        "Smallest directory that restores a disk space of 30000000: {}",
        min_dir_size
    );
    Ok(())
}
