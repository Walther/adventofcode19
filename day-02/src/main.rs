use intcode::compute;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    // Parse entire input into a list
    let input: Vec<usize> = INPUT
        .split(",")
        .map(|number| number.parse().unwrap())
        .collect();

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<usize>) {
    let mut punchcard = input.clone();
    // "Before running the program, replace position 1 with the value 12 and replace position 2 with the value 2."
    punchcard[1] = 12;
    punchcard[2] = 2;
    let result = compute(&punchcard);
    println!("Part 1: {}", result);
}

fn part2(input: &Vec<usize>) {
    let orig_punchcard = input.clone();
    // Find pair of inputs that give target result
    let mut result_noun = 0;
    let mut result_verb = 0;
    let target_result = 19690720;
    'outer_loop: for noun in 0..=99 {
        for verb in 0..=99 {
            let mut punchcard = orig_punchcard.clone();
            punchcard[1] = noun;
            punchcard[2] = verb;
            let result = compute(&punchcard);
            if result == target_result {
                result_noun = noun;
                result_verb = verb;
                break 'outer_loop;
            }
        }
    }
    println!("Part 2: {}", 100 * result_noun + result_verb);
}
