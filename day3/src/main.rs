use std::io::Read;

fn part1(input: &str) -> u32 {
    let m = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    m.captures_iter(input).map(move |capture| {
        capture[1].parse::<u32>().unwrap() * capture[2].parse::<u32>().unwrap()
    }).sum()
}

fn part2(input: &str) -> u32 {
    let m = regex::Regex::new(r"(?<doverb>do)\(\)|(?<dontverb>don't)\(\)|(?<mulverb>mul)\((?<v1>\d{1,3}),(?<v2>\d{1,3})\)").unwrap();
    let mut total = 0u32;
    let mut enabled = true;
    for capture in m.captures_iter(input) {
        if capture.name("doverb").is_some() {
            enabled = true;
        } else if capture.name("dontverb").is_some() {
            enabled = false;
        } else if capture.name("mulverb").is_some() {
            if enabled {
                let p = capture["v1"].parse::<u32>().unwrap() * capture["v2"].parse::<u32>().unwrap();
                total += p;
            }
        }
    }
    total
}

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let part1_total = part1(&input);
    println!("Total (part 1): {part1_total}");
    let part2_total = part2(&input);
    println!("Total (part 2): {part2_total}");
    Ok(())
}
