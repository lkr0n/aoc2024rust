use aoc_speed::day1;
use aoc_speed::day2;
use aoc_speed::day3;
use dotenv;

fn main() -> () {
    dotenv::dotenv().ok();
    aoc_macro::execute!(day1);
    aoc_macro::execute!(day2);
    aoc_macro::execute!(day3);
}
