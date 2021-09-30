pub struct IO{
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
    pub fn next_usize(&mut self) -> usize {
        self.read()
    }
    pub fn next_uint(&mut self) -> u64 {
        self.read()
    }
    pub fn next_int(&mut self) -> i64 {
        self.read()
    }
    pub fn next_float(&mut self) -> f64 {
        self.read()
    }
    pub fn next_chars(&mut self) -> std::str::Chars {
        self.next_str().chars()
    }
    pub fn next_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T>
        where
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.read()).collect::<Vec<_>>()
    }
    pub fn print<T: std::fmt::Display>(&mut self, t: T) {
        use std::io::Write;
        write!(self.buf, "{}", t).unwrap();
    }
    pub fn println<T: std::fmt::Display>(&mut self, t: T) {
        self.print(t);
        self.print("\n");
    }
    pub fn print_iter<T: std::fmt::Display, I: Iterator<Item=T>>(
        &mut self,
        mut iter: I,
        sep: &str,
    ) {
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