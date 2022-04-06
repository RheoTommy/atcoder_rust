#![allow(dead_code)]

pub mod solver {
    use std::fmt::{Debug, Display};
    use std::io::*;
    use std::str::{Chars, FromStr, SplitAsciiWhitespace};

    pub struct Solver {
        iter: SplitAsciiWhitespace<'static>,
        reader: BufReader<StdinLock<'static>>,
        writer: BufWriter<StdoutLock<'static>>,
    }

    impl Default for Solver {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Solver {
        pub fn new() -> Self {
            let stdout = Box::new(stdout());
            let stdin = Box::new(stdin());
            Self {
                iter: "".split_ascii_whitespace(),
                reader: BufReader::new(Box::leak(stdin).lock()),
                writer: BufWriter::new(Box::leak(stdout).lock()),
            }
        }

        fn input_line(&mut self) -> SplitAsciiWhitespace<'static> {
            let mut str = String::new();
            self.reader.read_line(&mut str).expect("No Input!");
            let str = Box::leak(str.into_boxed_str());
            str.split_ascii_whitespace()
        }

        fn read_str(&mut self) -> &str {
            if let Some(str) = self.iter.next() {
                str
            } else {
                self.iter = self.input_line();
                self.read_str()
            }
        }

        fn read<T: FromStr>(&mut self) -> T
            where
                <T as FromStr>::Err: Debug,
        {
            self.read_str().parse().expect("Cannot Parse!")
        }

        fn read_usize(&mut self) -> usize {
            self.read()
        }

        fn read_isize(&mut self) -> isize {
            self.read()
        }

        fn read_uint(&mut self) -> u64 {
            self.read()
        }

        fn read_int(&mut self) -> i64 {
            self.read()
        }

        fn read_float(&mut self) -> f64 {
            self.read()
        }

        fn read_chars(&mut self) -> Chars {
            self.read_str().chars()
        }

        fn read_vec<T: FromStr>(&mut self, n: usize) -> Vec<T>
            where
                <T as FromStr>::Err: Debug,
        {
            (0..n).map(|_| self.read()).collect::<Vec<_>>()
        }

        fn print<T: Display>(&mut self, t: T) {
            write!(self.writer, "{}", t).expect("Cannot Print!");
        }

        fn println<T: Display>(&mut self, t: T) {
            writeln!(self.writer, "{}", t).expect("Cannot Print!");
        }

        fn print_iter<T: Display, I: Iterator<Item=T>>(&mut self, mut iter: I, sep: &str) {
            if let Some(t) = iter.next() {
                self.print(t);
                for it in iter {
                    self.print(sep);
                    self.print(it);
                }
            }
            self.print("\n");
        }

        fn flush(&mut self) {
            self.writer.flush().expect("Cannot Flush!");
        }

        pub fn solve(&mut self) {

        }
    }
}

use solver::*;

fn main() {
    let mut solver = Solver::new();
    solver.solve();
}
