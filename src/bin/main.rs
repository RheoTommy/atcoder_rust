#![allow(unused_macros)]
#![allow(unused_imports)]

use crate::lib::*;

pub mod lib {
    #[macro_export]
    macro_rules! deb {
        ($($x:expr),* $(,)*) => {{
            let mut s = String::new();
            $(
                s.push_str(&format!("{:?} ", $x));
            )*
            eprintln!("{}", s.trim());
        }};
    }

    #[macro_export]
    macro_rules! prn {
        ($io:expr, $($x:expr),* $(,)*) => {{
            let mut s = String::new();
            $(
                s.push_str(&format!("{} ", $x));
            )*
            $io.print(s.trim());
        }};
    }

    #[macro_export]
    macro_rules! pri {
        ($io:expr, $($x:expr),* $(,)*) => {{
            let mut s = String::new();
            $(
                s.push_str(&format!("{} ", $x));
            )*
            $io.print(s.trim());
            $io.print("\n");
        }};
    }

    #[macro_export]
    macro_rules! int {
        ($io:expr, $($x:ident),* $(,)*) => {
            $(
                let $x: i64 = $io.read();
            )*
        };
    }

    #[macro_export]
    macro_rules! uint {
        ($io:expr, $($x:ident),* $(,)*) => {
            $(
                let $x: u64 = $io.read();
            )*
        };
    }

    #[macro_export]
    macro_rules! usize {
        ($io:expr, $($x:ident),* $(,)*) => {
            $(
                let $x: usize = $io.read();
            )*
        };
    }

    #[macro_export]
    macro_rules! str {
        ($io:expr, $($x:ident),* $(,)*) => {
            $(
                let $x: String = $io.read();
            )*
        };
    }

    #[macro_export]
    macro_rules! char {
        ($io:expr, $($x:ident),* $(,)*) => {
            $(
                let $x: char = $io.read();
            )*
        };
    }

    #[macro_export]
    macro_rules! float {
        ($io:expr, $($x:ident),* $(,)*) => {
            $(
                let $x: f64 = $io.read();
            )*
        };
    }

    #[macro_export]
    macro_rules! chars {
        ($io:expr, $($x:ident),* $(,)*) => {
            $(
                let $x: Vec<char> = $io.next_str().chars().collect::<Vec<char>>();
            )*
        };
    }

    #[macro_export]
    macro_rules! tuple {
        ($io:expr, $x:ident, $($t:ty),* $(,)*) => {
            let $x = ( $( $io.read::<$t>(), )* );
        }
    }

    #[macro_export]
    macro_rules! read_tuple {
        ($io:expr, $($t:ty),* $(,)*) => {
            ( $( $io.read::<$t>(), )* )
        }
    }

    pub const U_INF: u64 = (1 << 60) + (1 << 30);
    pub const I_INF: i64 = (1 << 60) + (1 << 30);
    pub const INF: usize = (1 << 60) + (1 << 30);

    pub struct IO {
        iter: std::str::SplitAsciiWhitespace<'static>,
        buf: std::io::BufWriter<std::io::StdoutLock<'static>>,
    }

    impl Default for IO {
        fn default() -> Self {
            Self::new()
        }
    }

    impl IO {
        pub fn new() -> Self {
            use std::io::*;
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            let input = Box::leak(input.into_boxed_str());
            let out = Box::new(stdout());
            IO {
                iter: input.split_ascii_whitespace(),
                buf: BufWriter::new(Box::leak(out).lock()),
            }
        }
        pub fn next_str(&mut self) -> &str {
            self.iter.next().unwrap()
        }
        pub fn read<T: std::str::FromStr>(&mut self) -> T
        where
            <T as std::str::FromStr>::Err: std::fmt::Debug,
        {
            self.iter.next().unwrap().parse().unwrap()
        }

        pub fn print<T: std::fmt::Display>(&mut self, t: T) {
            use std::io::Write;
            write!(self.buf, "{}", t).unwrap();
        }

        pub fn print_iter<T: std::fmt::Display, I: IntoIterator<Item = T>>(
            &mut self,
            iter: I,
            sep: &str,
        ) {
            let mut iter = iter.into_iter();
            if let Some(v) = iter.next() {
                self.print(v);
                for vi in iter {
                    self.print(sep);
                    self.print(vi);
                }
            }
            self.print("\n");
        }

        pub fn flush(&mut self) {
            use std::io::Write;
            self.buf.flush().unwrap();
        }
    }
}

fn main() {
    let mut io = IO::new();
    int!(io, a, b, c);
    for k in 1.. {
        if c * k < a {
            continue;
        }
        if c * k <= b {
            pri!(io, c * k);
        } else {
            pri!(io, -1);
        }
        break;
    }
}
