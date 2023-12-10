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
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    let (map, starting_cell) = parse(input);
    let mut visited: HashSet<Cell> = HashSet::new();
    let mut queue: VecDeque<_> = [(0, starting_cell)].into();

    let mut c = 0;

    while !queue.is_empty() {
        let (dist, curr) = queue.pop_front().expect("queue not empty");
        visited.insert(curr);

        if curr.val == '.' {
            continue;
        }

        let neighbours = curr.get_neighbour_coords();
        neighbours.iter().for_each(|x| {
            let next_cell = map.get(&(x.0, x.1));
            if next_cell.is_some() && !visited.contains(&next_cell.unwrap()) {
                queue.push_back((dist + 1, *next_cell.unwrap()));
            }
        });

        c = c.max(dist);
    }
    c.to_string()
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
        let test_input: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
        let result: String = part1(test_input);
        assert_eq!(result, "4".to_string());
    }

    #[test]
    fn test2() {
        let test_input: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let result: String = part1(test_input);
        assert_eq!(result, "8".to_string());
    }
}
