#![deny(clippy::all)]
use intcode::compute;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    // Parse entire input into a list
    let input: Vec<usize> = INPUT
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    part1(input.clone());
    part2(input.clone());
}

fn part1(mut memory: Vec<usize>) {
    // "Before running the program, replace position 1 with the value 12 and replace position 2 with the value 2."
    memory[1] = 12;
    memory[2] = 2;
    let result = compute(memory);
    println!("Part 1: {}", result);
}

fn part2(input: Vec<usize>) {
    // Find pair of inputs that give target result
    let mut result_noun = 0;
    let mut result_verb = 0;
    let target_result = 1969_0720;
    'outer_loop: for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = input.clone();
            memory[1] = noun;
            memory[2] = verb;
            let result = compute(memory);
            if result == target_result {
                result_noun = noun;
                result_verb = verb;
                break 'outer_loop;
            }
        }
    }
    println!("Part 2: {}", 100 * result_noun + result_verb);
}
