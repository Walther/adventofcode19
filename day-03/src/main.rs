use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    //  We know there's two lines. Might as well be explicit
    let mut lines = INPUT.lines();
    let wire1: Vec<&str> = lines.next().unwrap().split(',').collect();
    let wire2: Vec<&str> = lines.next().unwrap().split(',').collect();

    // dbg!(wire1);
    // dbg!(wire2);

    let traces1: Vec<Trace> = parse_trace(wire1);
    let traces2: Vec<Trace> = parse_trace(wire2);

    let mut board: Board = HashMap::new();
    board = draw_trace(board, traces1);
    board = draw_trace(board, traces2);

    // Part 1
    let crossings: Vec<(&(isize, isize), &usize)> = board
        .iter()
        .filter(|&(_coord, visits)| *visits >= 2)
        .collect();
    dbg!(crossings.clone());
    let mut distances: Vec<((isize, isize), usize)> = crossings
        .iter()
        .map(|&(coord, _visits)| (*coord, manhattan((0, 0), *coord)))
        .collect();
    distances.sort_by_key(|(_coord, distance)| *distance);
    println!("Part 1: {}", (distances[0].1))
}

// Hashmap of key: (x,y) value: number of visits
type Board = HashMap<(isize, isize), usize>;

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

fn draw_trace(mut board: Board, traces: Vec<Trace>) -> Board {
    let mut x: isize = 0;
    let mut y: isize = 0;
    for trace in traces {
        match trace.direction {
            Direction::Up => {
                for _ in 0..trace.length {
                    y += trace.length;
                    mark_board(&mut board, x, y);
                }
            }
            Direction::Down => {
                for _ in 0..trace.length {
                    y -= trace.length;
                    mark_board(&mut board, x, y);
                }
            }
            Direction::Left => {
                for _ in 0..trace.length {
                    x -= trace.length;
                    mark_board(&mut board, x, y);
                }
            }
            Direction::Right => {
                for _ in 0..trace.length {
                    x += trace.length;
                    mark_board(&mut board, x, y);
                }
            }
        }
    }
    board
}

fn mark_board(board: &mut Board, x: isize, y: isize) {
    board
        .entry((x, y))
        .and_modify(|visits| *visits += 1)
        .or_insert(1);
}

fn manhattan((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> usize {
    (x2 - x1).abs() as usize + (y2 - y1).abs() as usize
}
