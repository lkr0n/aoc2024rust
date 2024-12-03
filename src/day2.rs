fn is_safe(levels: &Vec<i64>) -> bool {
    let mut is_decreasing = true;
    let mut is_increasing = true;
    for (a,b) in levels.iter().zip(&levels[1..]) {
        let delta = (a - b).abs();
        if delta > 3 {
            return false;
        } 

        is_decreasing = is_decreasing && a < b;
        is_increasing = is_increasing && a > b;
    }
    return is_increasing || is_decreasing;
}


fn is_safe_dampened(levels: &Vec<i64>) -> bool {

    if is_safe(levels) {
       return true; 
    }

    for i in 0..levels.len() {
        let corrective = [&levels[..i], &levels[i+1..]].concat();
        if is_safe(&corrective) {
            return true;
        }
    }
    return false;
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    
    input.lines().filter(|line| {
        let levels =  line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>())
            .flatten()
            .collect();
        is_safe(&levels)
    }
    ).count()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    input.lines().filter(|line| {
        let levels =  line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>())
            .flatten()
            .collect();
        is_safe_dampened(&levels)
    }
    ).count()
}
