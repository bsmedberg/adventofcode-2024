use std::collections::HashMap;
use std::io::Read;

/// Given a slice, for each element in the slice iterate (element, slice of all following)
struct SliceFollowingIterator<'a, T> {
    slice: &'a [T],
    current: usize,
}

impl<'a, T> Iterator for SliceFollowingIterator<'a, T> {
    type Item = (&'a T, &'a [T]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.slice.len() {
            let el = &self.slice[self.current];
            let previous = &self.slice[self.current + 1..];
            self.current += 1;
            Some((el, previous))
        } else {
            None
        }
    }
}

fn iter_slice_with_following<T>(slice: &[T]) -> impl Iterator<Item = (&T, &[T])> {
    SliceFollowingIterator { slice, current: 0 }
}

struct Puzzle {
    /// The key must be printed before any of the values.
    pub ordering_rules: HashMap<usize, Vec<usize>>,

    pub updates: Vec<Vec<usize>>,
}

impl Puzzle {
    fn new<'a>(mut lines: impl Iterator<Item = &'a str>) -> Puzzle {
        let mut ordering_rules: HashMap<usize, Vec<usize>> = HashMap::new();
        for line in lines.by_ref() {
            if line.is_empty() { break; }

            let numbers: Vec<usize> = line.split("|").map(|n| n.parse().unwrap()).collect();
            assert!(numbers.len() == 2);
            ordering_rules.entry(numbers[0]).and_modify(|l| l.push(numbers[1])).or_insert_with(|| vec![numbers[1]]);
        }

        let updates: Vec<Vec<usize>> = lines.map(|line| line.split(",").map(|n| n.parse().unwrap()).collect()).collect();

        Puzzle {
            ordering_rules,
            updates,
        }
    }

    fn is_correctly_ordered(&self, update: &[usize]) -> bool {
        for (el, following) in iter_slice_with_following(update) {
            if let Some(following_required_list) = self.ordering_rules.get(el) {
                for following_required in following_required_list {
                    if !following.contains(following_required) && update.contains(following_required) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn get_middle_number(update: &[usize]) -> usize {
        let len = update.len();
        assert!(len % 2 == 1);
        let idx = len / 2;
        update[idx]
    }

    fn add_correctly_ordered_middle_numbers(&self) -> usize {
        self.updates.iter().filter(|update| self.is_correctly_ordered(update)).map(|u| Self::get_middle_number(&u)).sum()
    }
}

fn main() {
    let path = std::env::args().skip(1).next().unwrap();
    let mut fd = std::fs::File::open(path).unwrap();
    let mut buf = String::new();
    fd.read_to_string(&mut buf).unwrap();
    let p = Puzzle::new(buf.lines());
    let s = p.add_correctly_ordered_middle_numbers();
    println!("Sum of correctly-ordered middle numbers: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &'static str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_is_correctly_ordered() {
        let p = Puzzle::new(TEST_DATA.lines());
        assert_eq!(p.is_correctly_ordered(&p.updates[0]), true, "is_correctly_ordered #1");
        assert_eq!(p.is_correctly_ordered(&p.updates[1]), true, "is_correctly_ordered #2");
        assert_eq!(p.is_correctly_ordered(&p.updates[2]), true, "is_correctly_ordered #3");
        assert_eq!(p.is_correctly_ordered(&p.updates[3]), false, "is_correctly_ordered #4");
        assert_eq!(p.is_correctly_ordered(&p.updates[4]), false, "is_correctly_ordered #5");
        assert_eq!(p.is_correctly_ordered(&p.updates[5]), false, "is_correctly_ordered #6");
    }

    #[test]
    fn test_get_middle_number() {
        let p = Puzzle::new(TEST_DATA.lines());
        assert_eq!(Puzzle::get_middle_number(&p.updates[0]), 61);
    }

    #[test]
    fn test_sum() {
        let p = Puzzle::new(TEST_DATA.lines());
        assert_eq!(p.add_correctly_ordered_middle_numbers(), 143);
    }
}
