// My poor imitation of X macros, coming from C/C++. I wouldn't say I'm happy with this.
#[macro_export]
macro_rules! foreach_day {
    ($x:ident $(,$e:expr)*) => {
        $x!(
            $($e,)*
            day01,
            day02,
            day03,
            day04,
            day05,
            day06,
            day07,
            day08,
            day09,
            day10,
            day11,
            day12,
        );
    };
}

macro_rules! reexport {
    ($($day:ident,)*) => {
        $(
            mod $day;
            pub use $day::$day;
        )*
    };
}

foreach_day!(reexport);
