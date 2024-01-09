use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
enum Tile {
    Path,
    Forest,
    UpSlope,
    DownSlope,
    LeftSlope,
    RightSlope,
}
fn main() {
    let input: &str = include_str!("input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    let (tiles, start, goal) = parse(input);
    let mut paths = Vec::new();

    let mut stack = Vec::new();
    stack.push(vec![start]);

    while let Some(curr_path) = stack.pop() {
        let curr_pos = *curr_path.last().unwrap();
        if !tiles.contains_key(&curr_pos) || tiles.get(&curr_pos) == Some(&Tile::Forest) {
            // not a valid path
            continue;
        }

        if curr_pos == goal {
            paths.push(curr_path);
            continue;
        }

        let tile = tiles.get(&curr_pos).unwrap();
        let directions = match tile {
            Tile::Path => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
            Tile::UpSlope => vec![(0, -1)],
            Tile::DownSlope => vec![(0, 1)],
            Tile::LeftSlope => vec![(-1, 0)],
            Tile::RightSlope => vec![(1, 0)],
            _ => panic!("Unknown tile"),
        };

        for (dx, dy) in directions {
            let new_pos = (curr_pos.0 + dx, curr_pos.1 + dy);

            // check if we've already visited this position
            if curr_path.contains(&new_pos) {
                continue;
            }

            let mut new_path = curr_path.clone();
            new_path.push(new_pos);
            stack.push(new_path);
        }
    }
    // Do not count the start position
    paths.iter().map(|x| x.len() - 1).max().unwrap().to_string()
}

fn parse(input: &str) -> (HashMap<(i32, i32), Tile>, (i32, i32), (i32, i32)) {
    let mut tiles = HashMap::new();
    let mut start = (-1, -1);
    let mut goal = (-1, -1);

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            match c {
                '.' => {
                    if y == 0 {
                        start = (x as i32, y as i32);
                    }
                    if y == input.lines().count() - 1 {
                        goal = (x as i32, y as i32);
                    }
                    tiles.insert((x as i32, y as i32), Tile::Path)
                }
                '#' => tiles.insert((x as i32, y as i32), Tile::Forest),
                '>' => tiles.insert((x as i32, y as i32), Tile::RightSlope),
                '<' => tiles.insert((x as i32, y as i32), Tile::LeftSlope),
                '^' => tiles.insert((x as i32, y as i32), Tile::UpSlope),
                'v' => tiles.insert((x as i32, y as i32), Tile::DownSlope),
                _ => panic!("Unknown tile"),
            };
        })
    });
    if start == (-1, -1) {
        panic!("No start found");
    }
    if goal == (-1, -1) {
        panic!("No goal found");
    }
    (tiles, start, goal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        let result: String = part1(test_input);
        assert_eq!(result, "94".to_string());
    }
}
