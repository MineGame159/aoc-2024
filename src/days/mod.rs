macro_rules! reexport {
    ($x:ident) => {
        mod $x;
        pub use $x::*;
    };
}

reexport!(day1);
reexport!(day2);
reexport!(day3);
reexport!(day4);
reexport!(day5);
reexport!(day6);
reexport!(day7);
reexport!(day8);
reexport!(day9);
reexport!(day10);
reexport!(day11);
