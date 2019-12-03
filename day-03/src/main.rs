#![deny(clippy::all)]
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //  We know there's two lines. Might as well be explicit
    let mut lines = INPUT.lines();
    let wire1: Vec<&str> = lines.next().unwrap().split(',').collect();
    let wire2: Vec<&str> = lines.next().unwrap().split(',').collect();

    let traces1 = draw_trace(parse_trace(wire1));
    let traces2 = draw_trace(parse_trace(wire2));

    let coords1: HashSet<_> = traces1.keys().collect();
    let coords2: HashSet<_> = traces2.keys().collect();
    let crossings: HashSet<_> = coords1.intersection(&coords2).collect();

    // Part 1
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
type Coords = HashMap<(i32, i32), u32>;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Trace {
    direction: Direction,
    length: i32,
}

// TODO: serde?
fn parse_trace(instructions: Vec<&str>) -> Vec<Trace> {
    instructions
        .iter()
        .map(|instruction| {
            // Parsing the instruction
            let mut chars = instruction.chars();
            let dir = chars.next().unwrap();
            let length = chars.collect::<String>().parse::<i32>().unwrap();
            Trace {
                direction: match dir {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Unsupported direction: {}", dir),
                },
                length,
            }
        })
        .collect()
}

fn draw_trace(traces: Vec<Trace>) -> Coords {
    let mut coords: Coords = HashMap::new();

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut step = 0;
    for trace in traces {
        for _ in 0..trace.length {
            step += 1;
            match trace.direction {
                Direction::Up => {
                    y += 1;
                    coords.entry((x, y)).or_insert(step);
                }
                Direction::Down => {
                    y -= 1;
                    coords.entry((x, y)).or_insert(step);
                }
                Direction::Left => {
                    x -= 1;
                    coords.entry((x, y)).or_insert(step);
                }
                Direction::Right => {
                    x += 1;
                    coords.entry((x, y)).or_insert(step);
                }
            }
        }
    }
    coords
}

fn manhattan((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> u32 {
    (x2 - x1).abs() as u32 + (y2 - y1).abs() as u32
}
