struct Data {
    xs: Vec<i64>,
    ys: Vec<i64>,
}

fn parse(input: &str) -> Data {
    let mut xs = vec!();
    let mut ys = vec!();
    for line in input.lines() {
        let mut it = line.split_ascii_whitespace();
        let x = it.next().unwrap().parse::<i64>().unwrap();
        let y = it.next().unwrap().parse::<i64>().unwrap();
        xs.push(x);
        ys.push(y);
    }

    return Data { xs, ys };
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut data = parse(input);
    data.xs.sort();
    data.ys.sort();

    data.xs.into_iter().zip(data.ys).map( | (x,y) | (x - y).abs()).sum::<i64>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let data = parse(input);
    data.xs.iter().map(|x| x * data.ys.iter().filter(|&y| y==x).count() as i64)
        .sum::<i64>()
}
