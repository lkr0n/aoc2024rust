use aoc_speed::day1;
use dotenv;

fn main() -> () {
    dotenv::dotenv().ok();
    aoc_macro::execute!(day1); 
}
