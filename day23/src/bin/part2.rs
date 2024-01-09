use std::collections::{HashMap, VecDeque};

#[derive(Eq, PartialEq, Debug)]
enum Tile {
    Path,
    Forest,
}

#[derive(Debug)]
struct Intersection {
    adj_list: Vec<((i32, i32), i32)>, // (pos of next intersection), distance
}

fn main() {
    let input: &str = include_str!("input.txt");
    let result: String = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> String {
    let (tiles, start, goal) = parse(input);
    let intersections = get_intersections(tiles, start, goal);
    let mut paths = Vec::new();

    let mut stack = Vec::new();
    stack.push((vec![start], 0));

    while let Some((curr_path, dist_so_far)) = stack.pop() {
        let curr_pos = *curr_path.last().unwrap();

        if curr_pos == goal {
            paths.push((curr_path, dist_so_far));
            continue;
        }

        let tile = intersections.get(&curr_pos).unwrap();
        for (next_pos, next_dist) in tile.adj_list.iter() {
            if curr_path.contains(next_pos) {
                continue;
            }
            let mut new_path = curr_path.clone();
            new_path.push(*next_pos);
            stack.push((new_path, dist_so_far + next_dist));
        }
    }
    // Do not count the start position
    paths.iter().map(|x| x.1).max().unwrap().to_string()
}

fn get_intersections(
    tiles: HashMap<(i32, i32), Tile>,
    start: (i32, i32),
    end: (i32, i32),
) -> HashMap<(i32, i32), Intersection> {
    let mut intersections = Vec::new();
    for (pos, tile) in tiles.iter() {
        if *tile == Tile::Path {
            let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            let mut count = 0;
            for (dx, dy) in directions {
                let new_pos = (pos.0 + dx, pos.1 + dy);
                if tiles.contains_key(&new_pos) && tiles.get(&new_pos) == Some(&Tile::Path) {
                    count += 1;
                }
            }
            if count > 2 {
                intersections.push(*pos);
            }
        }
    }
    intersections.push(start);
    intersections.push(end);
    // for each intersection, do a bfs to find adjacent intersections
    let mut res = HashMap::new();
    intersections.iter().for_each(|pos| {
        let mut adj_list = Vec::new();
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((*pos, 0));

        while let Some((curr_pos, curr_dist)) = queue.pop_front() {
            if visited.contains_key(&curr_pos) {
                continue;
            }
            visited.insert(curr_pos, curr_dist);

            if intersections.contains(&curr_pos) && curr_pos != *pos {
                adj_list.push((curr_pos, curr_dist));
                continue;
            }

            let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

            for (dx, dy) in directions {
                let new_pos = (curr_pos.0 + dx, curr_pos.1 + dy);
                if tiles.contains_key(&new_pos) && tiles.get(&new_pos) == Some(&Tile::Path) {
                    queue.push_back((new_pos, curr_dist + 1));
                }
            }
        }
        res.insert(*pos, Intersection { adj_list });
    });
    res
}

fn parse(input: &str) -> (HashMap<(i32, i32), Tile>, (i32, i32), (i32, i32)) {
    let mut tiles = HashMap::new();
    let mut start = (-1, -1);
    let mut goal = (-1, -1);

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            match c {
                '#' => tiles.insert((x as i32, y as i32), Tile::Forest),
                _ => {
                    if y == 0 {
                        start = (x as i32, y as i32);
                    }
                    if y == input.lines().count() - 1 {
                        goal = (x as i32, y as i32);
                    }
                    tiles.insert((x as i32, y as i32), Tile::Path)
                }
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
        let result: String = part2(test_input);
        assert_eq!(result, "154".to_string());
    }
}
