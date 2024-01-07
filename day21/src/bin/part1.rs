use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
enum Tile {
    Rock,
    Empty,
}

struct State {
    pos: (i32, i32),
    steps: i32,
}

fn main() {
    let input: &str = include_str!("input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();
    let (tiles, start) = parse(input);
    let directions = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

    let goal = 64;
    let mut goal_tiles = HashSet::new();

    let mut q = VecDeque::new();
    q.push_back(State {
        pos: start,
        steps: 0,
    });

    while q.len() != 0 {
        let curr = q.pop_front().unwrap();
        let pos = curr.pos;
        if goal_tiles.contains(&pos) {
            continue;
        }
        let steps = curr.steps;

        if steps == goal {
            goal_tiles.insert(pos);
            continue;
        }

        if (goal - steps) % 2 == 0 {
            goal_tiles.insert(pos);
        }

        for dir in &directions {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if new_pos.0 < 0
                || new_pos.0 >= num_cols as i32
                || new_pos.1 < 0
                || new_pos.1 >= num_rows as i32
            {
                continue;
            }
            let tile = tiles.get(&new_pos).unwrap();
            match tile {
                Tile::Rock => continue,
                Tile::Empty => {
                    q.push_back(State {
                        pos: new_pos,
                        steps: steps + 1,
                    });
                }
            }
        }
    }
    goal_tiles.len().to_string()
}

fn parse(input: &str) -> (HashMap<(i32, i32), Tile>, (i32, i32)) {
    let mut tiles = HashMap::new();
    let mut start = (-1, -1);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let tile = match c {
                '#' => Tile::Rock,
                '.' => Tile::Empty,
                'S' => {
                    start = (x as i32, y as i32);
                    Tile::Empty
                }
                _ => panic!("Unknown tile: {}", c),
            };
            tiles.insert((x as i32, y as i32), tile);
        });
    });
    if start == (-1, -1) {
        panic!("No start found");
    }
    (tiles, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let result: String = part1(test_input);
        assert_eq!(result, "16".to_string());
    }
}
