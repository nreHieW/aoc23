use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
enum Tile {
    Rock,
    Empty,
}

struct State {
    actual_pos: (i64, i64),
    steps: i64,
}

fn main() {
    let input: &str = include_str!("input.txt");
    let result: String = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> String {
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();
    let (tiles, start) = parse(input);

    // 26501365 = 202300 * 131 + 65
    let offset = 65;
    let size = num_rows; // Since the map is a square
    let x = (26501365 - offset) / size;

    // for i in 0..3 {
    //     let goal = (x + i * size) as i64;
    //     let goals = get_goals(goal, &tiles, start, num_rows as i64, num_cols as i64);
    //     println!("Goal {}: {}", goal, goals);
    // }

    f(x as i64).to_string()
}

fn f(x: i64) -> i64 {
    // Goal 65: 3734
    // Goal 196: 33285
    // Goal 327: 92268
    // We can estimate any quadratic function with 3 point

    // per wolfram alpha:
    3734 + 14835 * x + 14716 * x.pow(2)
}

fn get_goals(
    goal: i64,
    tiles: &HashMap<(i64, i64), Tile>,
    start: (i64, i64),
    num_rows: i64,
    num_cols: i64,
) -> i64 {
    let directions = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut goal_tiles = HashSet::new();
    let mut visited = HashSet::new();

    let mut q = VecDeque::new();
    q.push_back(State {
        actual_pos: start,
        steps: 0,
    });

    while q.len() != 0 {
        let curr = q.pop_front().unwrap();
        let pos = curr.actual_pos;
        let steps = curr.steps;

        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        if steps == goal {
            goal_tiles.insert(pos);
            continue;
        }

        if (goal - steps) % 2 == 0 {
            goal_tiles.insert(pos);
        }

        for dir in &directions {
            let new_abs_pos = (pos.0 + dir.0, pos.1 + dir.1);
            let new_rel_pos = (
                new_abs_pos.0.rem_euclid(num_cols),
                new_abs_pos.1.rem_euclid(num_rows),
            );
            let tile = tiles.get(&new_rel_pos).unwrap();
            match tile {
                Tile::Rock => continue,
                Tile::Empty => {
                    q.push_back(State {
                        actual_pos: new_abs_pos,
                        steps: steps + 1,
                    });
                }
            }
        }
    }
    goal_tiles.len() as i64
}

fn parse(input: &str) -> (HashMap<(i64, i64), Tile>, (i64, i64)) {
    let mut tiles = HashMap::new();
    let mut start = (-1, -1);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let tile = match c {
                '#' => Tile::Rock,
                '.' => Tile::Empty,
                'S' => {
                    start = (x as i64, y as i64);
                    Tile::Empty
                }
                _ => panic!("Unknown tile: {}", c),
            };
            tiles.insert((x as i64, y as i64), tile);
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
        let result: String = part2(test_input);
        assert_eq!(result, "16733044".to_string());
    }
}
