mod day1;
mod day2;
mod day3;

fn main() {
    println!("Day1 Part1: {:?}", day1::part1("data/input/day1.txt"));
    println!("Day1 Part2: {:?}", day1::part2("data/input/day1.txt"));

    println!("Day2 Part1: {:?}", day2::part1("data/input/day2.txt"));
    println!("Day2 Part2: {:?}", day2::part2("data/input/day2.txt"));

    println!("Day3 Part1: {:?}", day3::part1("data/input/day3.txt"));
    println!("Day3 Part1: {:?}", day3::part2("data/input/day3.txt"));
}
