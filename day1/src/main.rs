use std::{collections::{BinaryHeap, HashMap}, io::BufRead};


struct Counter(HashMap<u32, u32>);

impl Counter {
    pub fn increment(&mut self, v: u32) {
        self.0.entry(v).and_modify(|ov| *ov += 1).or_insert(1);
    }

    pub fn get(&self, v: u32) -> u32 {
        self.0.get(&v).copied().unwrap_or(0)
    }
}

impl FromIterator<u32> for Counter
{
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut s = Counter(HashMap::new());
        for v in iter {
            s.increment(v);
        }
        s
    }
}

fn read_lines(fd: impl std::io::Read) -> impl Iterator<Item = (u32, u32)> {
    std::io::BufReader::new(fd).lines().map(|line| {
        let line = line.unwrap();
        let values: Vec<&str> = line.split_whitespace().collect();
        assert!(values.len() == 2);
        (values[0].parse::<u32>().unwrap(), values[1].parse::<u32>().unwrap())
    })
}

fn main() {
    let (mut h1, mut h2): (BinaryHeap<u32>, BinaryHeap<u32>) = read_lines(std::io::stdin()).collect();
    let h2_counter: Counter = h2.iter().copied().collect();
    assert!(h1.len() == h2.len());
    let mut cum_distance: u32 = 0;
    let mut cum_similarity: u32 = 0;
    while let Some(v1) = h1.pop() {
        let v2 = h2.pop().unwrap();
        let distance = v1.abs_diff(v2);
        cum_distance += distance;

        let similarity = v1 * h2_counter.get(v1);
        cum_similarity += similarity;
    }
    println!("distance: {cum_distance}");
    println!("similary: {cum_similarity}")
}
