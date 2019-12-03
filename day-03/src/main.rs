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
    let crossings: HashSet<_> = traces1.intersection(&traces2).collect();
    // dbg!(crossings.clone());
    let mut distances: Vec<((isize, isize), usize)> = crossings
        .iter()
        .map(|&coord| (*coord, manhattan((0, 0), *coord)))
        .collect();
    distances.sort_by_key(|(_coord, distance)| *distance);
    println!("Part 1: {}", (distances[0].1))
}

// HashSet of visited coordinates
type Coords = HashSet<(isize, isize)>;

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
    let mut coords: Coords = HashSet::new();

    let mut x: isize = 0;
    let mut y: isize = 0;
    for trace in traces {
        match trace.direction {
            Direction::Up => {
                for _ in 0..trace.length {
                    y += 1;
                    coords.insert((x, y));
                }
            }
            Direction::Down => {
                for _ in 0..trace.length {
                    y -= 1;
                    coords.insert((x, y));
                }
            }
            Direction::Left => {
                for _ in 0..trace.length {
                    x -= 1;
                    coords.insert((x, y));
                }
            }
            Direction::Right => {
                for _ in 0..trace.length {
                    x += 1;
                    coords.insert((x, y));
                }
            }
        }
    }
    coords
}

fn manhattan((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> usize {
    (x2 - x1).abs() as usize + (y2 - y1).abs() as usize
}
