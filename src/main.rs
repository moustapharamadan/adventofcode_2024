mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    println!("Day1 Part1: {:?}", day1::part1("data/input/day1.txt"));
    println!("Day1 Part2: {:?}", day1::part2("data/input/day1.txt"));

    println!("Day2 Part1: {:?}", day2::part1("data/input/day2.txt"));
    println!("Day2 Part2: {:?}", day2::part2("data/input/day2.txt"));

    println!("Day3 Part1: {:?}", day3::part1("data/input/day3.txt"));
    println!("Day3 Part1: {:?}", day3::part2("data/input/day3.txt"));

    println!("Day4 Part1: {:?}", day4::part1("data/input/day4.txt"));
    println!("Day4 Part2: {:?}", day4::part2("data/input/day4.txt"));

    println!("Day5 Part1: {:?}", day5::part1("data/input/day5.txt"));
    println!(
        "Day5 Part1 (New): {:?}",
        day5::part1_new("data/input/day5.txt")
    );
    println!("Day5 Part2: {:?}", day5::part2("data/input/day5.txt"));

    println!("Day6 Part1: {:?}", day6::part1("data/input/day6.txt"));
    // println!("Day6 Part2: {:?}", day6::part2("data/input/day6.txt"));

    println!("Day7 Part1: {:?}", day7::part1("data/input/day7.txt"));
    println!("Day7 Part2: {:?}", day7::part2("data/input/day7.txt"));

    println!("Day8 Part1: {:?}", day8::part1("data/input/day8.txt"));
    println!("Day8 Part2: {:?}", day8::part2("data/input/day8.txt"));

    println!("Day9 Part1: {:?}", day9::part1("data/input/day9.txt"));
    println!("Day9 Part2: {:?}", day9::part2("data/input/day9.txt"));
}
