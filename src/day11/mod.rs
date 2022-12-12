use anyhow::{bail, Result};

#[derive(Clone, Copy, PartialEq, Eq)]
enum ParseState {
    MonkeyID,
    StartingItems,
    Operation,
    Test,
    TestTrue,
    TestFalse,
}

#[derive(Clone, Copy)]
enum Operation {
    Add(i64),
    Multiply(i64),
    MultiplyOld,
}

impl Operation {
    fn apply(&self, old: i64) -> i64 {
        match self {
            Operation::Add(num) => old + num,
            Operation::Multiply(num) => old * num,
            Operation::MultiplyOld => old * old,
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test_divisor: i64,
    targets: [usize; 2],
    inspected_items: usize,
}

fn product_of_top2(monkeys: &[Monkey]) -> usize {
    let mut top0 = 0;
    let mut top1 = 0;

    for m in monkeys {
        top1 = top1.max(m.inspected_items);
        if top1 > top0 {
            std::mem::swap(&mut top0, &mut top1);
        }
    }

    top0 * top1
}

pub fn day11() -> Result<(usize, usize)> {
    let input = include_str!("input.txt");

    let mut monkeys = Vec::with_capacity(8);
    {
        let mut parser_state = ParseState::MonkeyID;
        let mut items: Vec<i64> = Vec::new();
        let mut operation = Operation::Add(0);
        let mut test_divisor = 0;
        let mut targets = [0usize; 2];

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            const PREFIXES: [&str; 6] = [
                "Monkey ",
                "  Starting items: ",
                "  Operation: new = old ",
                "  Test: divisible by ",
                "    If true: throw to monkey ",
                "    If false: throw to monkey ",
            ];
            let prefix = PREFIXES[parser_state as usize];
            let Some(s) = line.strip_prefix(prefix) else {
                bail!("expected prefix '{}', got: '{}'", prefix, line);
            };

            match parser_state {
                ParseState::MonkeyID => {
                    parser_state = ParseState::StartingItems;
                }
                ParseState::StartingItems => {
                    items = s
                        .split(", ")
                        .map(|s| s.parse::<i64>())
                        .collect::<std::result::Result<Vec<_>, _>>()?;
                    parser_state = ParseState::Operation;
                }
                ParseState::Operation => {
                    if let Some(s) = s.strip_prefix("* ") {
                        if s == "old" {
                            operation = Operation::MultiplyOld;
                        } else {
                            operation = Operation::Multiply(s.parse()?);
                        }
                    } else if let Some(s) = s.strip_prefix("+ ") {
                        operation = Operation::Add(s.parse()?);
                    } else {
                        bail!("expected */+ operation, got '{}'", s);
                    }
                    parser_state = ParseState::Test;
                }
                ParseState::Test => {
                    test_divisor = s.parse()?;
                    parser_state = ParseState::TestTrue;
                }
                ParseState::TestTrue => {
                    targets[1] = s.parse()?;
                    parser_state = ParseState::TestFalse;
                }
                ParseState::TestFalse => {
                    targets[0] = s.parse()?;
                    monkeys.push(Monkey {
                        items: std::mem::take(&mut items),
                        operation,
                        test_divisor,
                        targets,
                        inspected_items: 0,
                    });
                    parser_state = ParseState::MonkeyID;
                }
            }
        }

        if parser_state != ParseState::MonkeyID {
            bail!("expected input blocks of 6 lines each");
        }
    }

    // Part 1
    let total1 = {
        let mut monkeys = monkeys.clone();

        // I'm not sure what the best pattern is to efficiently move items between nested `Vec`s like so:
        // ```
        // let v: Vec<Vec<_>> = ...;
        // v.extend(v.drain(..));
        // ```
        // ...because that's how our `monkeys` vector works. The traditional option is to just use
        // indices and no `drain()`, but that hurts performance, so what I did instead is to unsafely
        // create another mutable slice that points to `monkeys` and use that as the `extend()` target.
        // To restore some level of sanity the code below contains an `assert_ne!()` to ensure we don't
        // drain `Vec`s into themselves, which would be like truly having two mutable borrows to it.
        // This approach improves throughput by >100%.
        let monkey_targets =
            unsafe { std::slice::from_raw_parts_mut(monkeys.as_mut_ptr(), monkeys.len()) };

        for _ in 0..20 {
            for (i, m) in monkeys.iter_mut().enumerate() {
                m.inspected_items += m.items.len();

                for worry_level in m.items.drain(..) {
                    // Worry level is multiplied / increases by
                    let mut worry_level = m.operation.apply(worry_level);
                    // Monkey gets bored with item
                    worry_level /= 3;
                    // Worry level test
                    let b = worry_level % m.test_divisor == 0;
                    let target = m.targets[b as usize];
                    // Item is thrown to another monkey
                    assert_ne!(i, target);
                    monkey_targets[target].items.push(worry_level);
                }
            }
        }

        product_of_top2(&monkeys)
    };

    // This code is pretty much identical to "Part 1" above (including the unsafe code), but the
    // `/= 3` has been replaced with a modular "reduction" of the `worry_level` via `common_divisor`.
    let total2 = {
        let common_divisor = monkeys.iter().fold(1, |acc, m| acc * m.test_divisor);
        let monkey_targets =
            unsafe { std::slice::from_raw_parts_mut(monkeys.as_mut_ptr(), monkeys.len()) };

        for _ in 0..10000 {
            for (i, m) in monkeys.iter_mut().enumerate() {
                m.inspected_items += m.items.len();

                for worry_level in m.items.drain(..) {
                    // Worry level is multiplied / increases by
                    let mut worry_level = m.operation.apply(worry_level);
                    // Prevent the worry_level from exceeding the maximum.
                    worry_level %= common_divisor;
                    // Worry level test
                    let b = worry_level % m.test_divisor == 0;
                    let target = m.targets[b as usize];
                    // Item is thrown to another monkey
                    assert_ne!(i, target);
                    monkey_targets[target].items.push(worry_level);
                }
            }
        }

        product_of_top2(&monkeys)
    };

    Ok((total1, total2))
}
