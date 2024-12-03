use regex::Regex;

fn solve(input: &str) -> u64 {
    let re = Regex::new(r"mul\(([0-9]{0,3}),([0-9]{0,3})\)").unwrap();

    re.captures_iter(input).map(|c| c.extract()).map(|(_, [a,b])| {
        let ai = a.parse::<u64>().unwrap();
        let bi = b.parse::<u64>().unwrap();
        ai * bi
    }).sum::<u64>()
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve(input)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut text = &input[..];
    let mut solution = 0u64;
    while !text.is_empty() {
        let end = text.find("don't()").unwrap_or(text.len());
        solution += solve(&text[..end]);
        text = &text[end..];

        let start = text.find("do()").unwrap_or(text.len());
        text = &text[start..];
    }

    return solution;
}