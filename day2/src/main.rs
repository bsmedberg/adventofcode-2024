use std::io::{BufRead, BufReader};

struct IterPairs<I>
where I: Iterator<Item = u32>
{
    it: I,
    last: Option<u32>,
}

fn iter_pairs(mut it: impl Iterator<Item = u32>) -> impl Iterator<Item = (u32, u32)> {
    let next = it.next();
    IterPairs {
        it,
        last: next,
    }
}

impl<I> Iterator for IterPairs<I>
where I: Iterator<Item = u32>
{
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.last {
            None => None,
            Some(last) => {
                self.last = self.it.next();
                match self.last {
                    None => None,
                    Some(next) => Some((last, next)),
                }
            }
        }
    }
}

/// Yield an iterator of iterators: each item will be an iterator that skips one element
fn iter_skips(list: &[u32]) -> impl Iterator<Item = impl Iterator<Item = u32> + use<'_>> {
    (0..list.len()).map(|skip_idx| list.iter().enumerate().filter_map(move |(idx, v)| if skip_idx == idx { None } else {Some(v)}).copied())
}

struct Report(Vec<u32>);

impl Report {
    pub fn new(data: Vec<u32>) -> Report {
        Report(data)
    }

    fn is_safe_internal(it: impl Iterator<Item = u32>) -> bool {
        let mut pairs = iter_pairs(it).peekable();
        let is_increasing = match pairs.peek() {
            None => true,
            Some((v1, v2)) => v2 > v1,
        };
        for (v1, v2) in pairs {
            let pair_is_increasing = v2 > v1;
            if pair_is_increasing != is_increasing {
                return false;
            }
            let delta = v2.abs_diff(v1);
            if delta < 1 || delta > 3 {
                return false;
            }
        }
        true
    }

    pub fn is_safe(&self) -> bool {
        Report::is_safe_internal(self.0.iter().copied())
    }

    pub fn is_safe_with_skips(&self) -> bool {
        self.is_safe() || iter_skips(&self.0).any(Report::is_safe_internal)
    }
}

fn main() {
    let (count_safe, count_safe_with_skips) = BufReader::new(std::io::stdin()).lines().map(|line| {
        let line = line.unwrap();
        let sequence: Vec<u32> = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        let r = Report::new(sequence);
        (r.is_safe(), r.is_safe_with_skips())
    }).fold((0u32, 0u32), |(count_safe, count_safe_with_skips), (is_safe, is_safe_with_skips)| (count_safe + if is_safe { 1 } else { 0 }, count_safe_with_skips + if is_safe_with_skips { 1 } else { 0 }));
    println!("Count of initially-safe reports: {count_safe}");
    println!("Count of safe after skipping: {count_safe_with_skips}")
}
