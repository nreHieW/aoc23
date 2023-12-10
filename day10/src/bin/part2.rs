use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Any,
    NA,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
struct Cell {
    x: i32,
    y: i32,
    val: char,
    directions: [Direction; 2],
}

impl Cell {
    fn get_neighbour_coords(&self) -> Vec<(i32, i32)> {
        let directions = &self.directions;
        let mut coords = HashSet::new();

        directions.iter().for_each(|x| match x {
            Direction::Up => _ = coords.insert((self.x, self.y - 1)),
            Direction::Down => _ = coords.insert((self.x, self.y + 1)),
            Direction::Left => _ = coords.insert((self.x - 1, self.y)),
            Direction::Right => _ = coords.insert((self.x + 1, self.y)),
            Direction::Any => {
                _ = coords.insert((self.x, self.y - 1));
                _ = coords.insert((self.x, self.y + 1));
                _ = coords.insert((self.x - 1, self.y));
                _ = coords.insert((self.x + 1, self.y));
            }
            Direction::NA => (),
        });
        coords.into_iter().collect::<Vec<(i32, i32)>>()
    }
}

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> String {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let (map, starting_cell) = parse(input);
    let mut visited: HashSet<Cell> = HashSet::new();
    let mut predescessors: HashMap<Cell, Cell> = HashMap::new();
    let mut queue: VecDeque<_> = [(0, starting_cell)].into();

    let mut furthest_dist = 0;
    let mut last = starting_cell;
    while !queue.is_empty() {
        let (dist, curr) = queue.pop_front().expect("queue not empty");
        visited.insert(curr);

        if curr.val == '.' {
            continue;
        }
        last = curr;

        let neighbours = curr.get_neighbour_coords();
        neighbours.iter().for_each(|x| {
            let next_cell = map.get(&(x.0, x.1));
            if next_cell.is_some() && !visited.contains(&next_cell.unwrap()) {
                queue.push_back((dist + 1, *next_cell.unwrap()));
                predescessors.insert(*next_cell.unwrap(), curr);
            }
        });

        furthest_dist = furthest_dist.max(dist);
    }

    let mut path = Vec::new();
    while last.val != 'S' {
        let p = predescessors.get(&last).unwrap().clone();
        path.push(p);
        last = p;
    }
    path = path.into_iter().rev().collect::<Vec<_>>();

    loop {
        if path.iter().filter(|x| x.val == 'S').count() == 2 {
            break;
        }
        let last_item = path.last().unwrap();
        for neighbour in last_item.get_neighbour_coords() {
            let next_cell = map.get(&(neighbour.0, neighbour.1));

            if next_cell.is_some() && next_cell.unwrap().val == 'S' {
                path.push(*next_cell.unwrap());
                break;
            }

            if next_cell.is_some()
                && !path.contains(&next_cell.unwrap())
                && next_cell.unwrap().val != '.'
            {
                path.push(*next_cell.unwrap());
            }
        }
    }
    let loop_cells = path.iter().collect::<HashSet<_>>();
    // println!("{:?}", loop_cells);
    let mut total_count = 0;

    for j in 0..height {
        for i in 0..width {
            let curr = map.get(&(i as i32, j as i32));
            let mut winding_rule_count = 0;

            if curr.is_none() || loop_cells.contains(&curr.unwrap()) {
                continue;
            }

            for k in i..width {
                let next = map.get(&(k as i32, j as i32));
                if next.is_none() {
                    continue;
                }

                if loop_cells.contains(&next.unwrap())
                    && (next.unwrap().directions.contains(&Direction::Up))
                {
                    winding_rule_count += 1;
                }
            }

            if winding_rule_count % 2 == 1 {
                total_count += 1;
            }
        }
    }
    total_count.to_string()
}

fn parse(input: &str) -> (HashMap<(i32, i32), Cell>, Cell) {
    use Direction::*;
    let pipe_map = HashMap::from([
        ('|', [Up, Down]),
        ('-', [Left, Right]),
        ('L', [Up, Right]),
        ('J', [Up, Left]),
        ('7', [Down, Left]),
        ('F', [Down, Right]),
        ('.', [NA, NA]),
        ('S', [Any, Any]),
    ]);
    let binding = input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, val)| Cell {
                    x: x as i32,
                    y: y as i32,
                    val,
                    directions: pipe_map.get(&val).expect("valid char").clone(),
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();
    let cells = binding.iter().flatten().collect::<Vec<&Cell>>();

    let mut map = HashMap::new();
    cells.iter().for_each(|x| {
        map.insert((x.x, x.y), (*x).clone());
    });

    let start_cell_orig = cells.iter().find(|x| x.val == 'S').expect("start cell");
    map.remove(&(start_cell_orig.x, start_cell_orig.y));

    let mut actual = Vec::new();
    let (start_x, start_y) = (start_cell_orig.x, start_cell_orig.y);

    if map.get(&(start_x, start_y - 1)).is_some()
        && pipe_map
            .get(&map.get(&(start_x, start_y - 1)).unwrap().val)
            .unwrap()
            .contains(&Down)
    {
        actual.push(Up);
    }

    if map.get(&(start_x, start_y + 1)).is_some()
        && pipe_map
            .get(&map.get(&(start_x, start_y + 1)).unwrap().val)
            .unwrap()
            .contains(&Up)
    {
        actual.push(Down);
    }

    if map.get(&(start_x - 1, start_y)).is_some()
        && pipe_map
            .get(&map.get(&(start_x - 1, start_y)).unwrap().val)
            .unwrap()
            .contains(&Right)
    {
        actual.push(Left);
    }

    if map.get(&(start_x + 1, start_y)).is_some()
        && pipe_map
            .get(&map.get(&(start_x + 1, start_y)).unwrap().val)
            .unwrap()
            .contains(&Left)
    {
        actual.push(Right);
    }

    let start_cell = Cell {
        x: start_x,
        y: start_y,
        val: 'S',
        directions: [actual[0], actual[1]],
    };

    map.insert((start_x, start_y), start_cell.clone());

    (map, start_cell)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let result: String = part2(test_input);
        assert_eq!(result, "4".to_string());
    }

    #[test]
    fn test2() {
        let test_input: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let result: String = part2(test_input);
        assert_eq!(result, "8".to_string());
    }

    #[test]
    fn test3() {
        let test_input: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let result: String = part2(test_input);
        assert_eq!(result, "10".to_string());
    }
}
