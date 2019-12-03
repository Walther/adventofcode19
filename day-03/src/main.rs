use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //  We know there's two lines. Might as well be explicit
    let mut lines = INPUT.lines();
    let wire1: Vec<&str> = lines.next().unwrap().split(',').collect();
    let wire2: Vec<&str> = lines.next().unwrap().split(',').collect();

    // dbg!(wire1);
    // dbg!(wire2);
    let traces1 = draw_trace(parse_trace(wire1));
    let traces2 = draw_trace(parse_trace(wire2));

    // Part 1
    let coords1: HashSet<_> = traces1.keys().collect();
    let coords2: HashSet<_> = traces2.keys().collect();
    let crossings: HashSet<_> = coords1.intersection(&coords2).collect();
    // dbg!(crossings.clone());
    let mut distances: Vec<_> = crossings
        .iter()
        .map(|&coord| (*coord, manhattan((0, 0), **coord)))
        .collect();
    distances.sort_by_key(|(_coord, distance)| *distance);
    println!("Part 1: {}", (distances[0].1));

    // Part 2
    let minimum_steps = crossings
        .iter()
        .map(|&coord| {
            let steps1 = traces1.get(*coord).unwrap();
            let steps2 = traces2.get(*coord).unwrap();
            steps1 + steps2
        })
        .min()
        .unwrap();

    println!("Part 2: {}", minimum_steps);
}

// HashMap of visited coordinates & earliest visit step
type Coords = HashMap<(isize, isize), usize>;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Trace {
    direction: Direction,
    length: isize,
}

// TODO: serde?
fn parse_trace(instructions: Vec<&str>) -> Vec<Trace> {
    instructions
        .iter()
        .map(|instruction| {
            // Parsing the instruction
            let mut chars = instruction.chars();
            let dir = chars.next().unwrap();
            let length = chars.collect::<String>().parse::<isize>().unwrap();
            Trace {
                direction: match dir {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Unsupported direction: {}", dir),
                },
                length: length,
            }
        })
        .collect()
}

fn draw_trace(traces: Vec<Trace>) -> Coords {
    let mut coords: Coords = HashMap::new();

    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut step = 0;
    for trace in traces {
        match trace.direction {
            Direction::Up => {
                for _ in 0..trace.length {
                    step += 1;
                    y += 1;
                    coords.entry((x, y)).or_insert(step);
                }
            }
            Direction::Down => {
                for _ in 0..trace.length {
                    step += 1;
                    y -= 1;
                    coords.entry((x, y)).or_insert(step);
                }
            }
            Direction::Left => {
                for _ in 0..trace.length {
                    step += 1;
                    x -= 1;
                    coords.entry((x, y)).or_insert(step);
                }
            }
            Direction::Right => {
                for _ in 0..trace.length {
                    step += 1;
                    x += 1;
                    coords.entry((x, y)).or_insert(step);
                }
            }
        }
    }
    coords
}

fn manhattan((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> usize {
    (x2 - x1).abs() as usize + (y2 - y1).abs() as usize
}
