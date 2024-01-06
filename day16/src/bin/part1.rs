use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Copy)]
struct Tile {
    x: i32,
    y: i32,
    is_energized: bool,
    num_beams: i32,
    recent_beam_direction: Direction,
    cell_type: CellType,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
struct Beam {
    x: i32,
    y: i32,
    x_max: i32,
    y_max: i32,
    direction: Direction,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    NA,
}

#[derive(Clone, Debug, Copy, PartialEq)]
enum CellType {
    LeftMirror,  // /
    RightMirror, // \
    HorizontalSplitter,
    VerticalSplitter,
    Normal,
}

impl Beam {
    fn step(&mut self) -> (i32, i32) {
        match self.direction {
            Direction::Up => self.y = self.y - 1,
            Direction::Down => self.y = self.y + 1,
            Direction::Left => self.x = self.x - 1,
            Direction::Right => self.x = self.x + 1,
            Direction::NA => {
                panic!("NA direction")
            }
        }
        (self.x, self.y)
    }
}

impl Tile {
    fn set_energized(&mut self, direction: Direction) {
        self.is_energized = true;
        if self.recent_beam_direction != direction {
            self.num_beams += 1;
        }
        self.recent_beam_direction = direction;
    }

    fn to_string(&self) -> String {
        if self.is_energized {
            return "#".to_string();
        } else {
            return ".".to_string();
        }
        // if self.cell_type != CellType::Normal {
        //     match self.cell_type {
        //         CellType::LeftMirror => "/".to_string(),
        //         CellType::RightMirror => "\\".to_string(),
        //         CellType::HorizontalSplitter => "-".to_string(),
        //         CellType::VerticalSplitter => "|".to_string(),
        //         CellType::Normal => ".".to_string(),
        //     }
        // } else if self.is_energized {
        //     if self.num_beams > 1 {
        //         self.num_beams.to_string()
        //     } else {
        //         match self.recent_beam_direction {
        //             Direction::Up => "^",
        //             Direction::Down => "v",
        //             Direction::Left => "<",
        //             Direction::Right => ">",
        //             Direction::NA => panic!("NA direction but is energized"),
        //         }
        //         .to_string()
        //     }
        // } else {
        //     ".".to_string()
        // }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    x: i32,
    y: i32,
    direction: Direction,
}

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}
fn part1(input: &str) -> String {
    use CellType::*;
    let (cells, mut tiles_map) = parse(input);
    let mut tiles;
    let x_max = cells[0].len() as i32 - 1;
    let y_max = cells.len() as i32 - 1;
    let first_beam = Beam {
        x: -1,
        y: 0,
        x_max,
        y_max,
        direction: Direction::Right,
    };
    let mut beams = vec![first_beam];
    let mut curr_energised = 0;
    let mut break_flag = false;
    let mut states: HashSet<State> = HashSet::new();
    while beams.len() > 0 {
        let mut new_beams = Vec::new();
        tiles = tiles_map.values().cloned().collect::<Vec<Tile>>();
        let num_energized = tiles
            .clone()
            .iter()
            .filter(|tile| tile.is_energized)
            .count();
        if num_energized == curr_energised && curr_energised > 0 {
            if !break_flag {
                break_flag = true;
            }
        } else {
            curr_energised = num_energized;
        }

        for beam in beams.iter_mut() {
            let (x, y) = beam.step();
            if (x < 0 || x > x_max) || (y < 0 || y > y_max) {
                continue;
            }
            let curr_cell = &cells[y as usize][x as usize];
            match curr_cell.cell_type {
                LeftMirror => {
                    match beam.direction {
                        Direction::Up => {
                            new_beams.push(Beam {
                                x,
                                y,
                                x_max,
                                y_max,
                                direction: Direction::Right,
                            });
                        }
                        Direction::Down => {
                            new_beams.push(Beam {
                                x,
                                y,
                                x_max,
                                y_max,
                                direction: Direction::Left,
                            });
                        }
                        Direction::Left => {
                            new_beams.push(Beam {
                                x,
                                y,
                                x_max,
                                y_max,
                                direction: Direction::Down,
                            });
                        }
                        Direction::Right => {
                            new_beams.push(Beam {
                                x,
                                y,
                                x_max,
                                y_max,
                                direction: Direction::Up,
                            });
                        }
                        Direction::NA => {
                            panic!("NA direction")
                        }
                    };
                }
                RightMirror => {
                    match beam.direction {
                        Direction::Up => {
                            new_beams.push(Beam {
                                x,
                                y,
                                x_max,
                                y_max,
                                direction: Direction::Left,
                            });
                        }
                        Direction::Down => {
                            new_beams.push(Beam {
                                x,
                                y,
                                x_max,
                                y_max,
                                direction: Direction::Right,
                            });
                        }
                        Direction::Left => {
                            new_beams.push(Beam {
                                x,
                                y,
                                x_max,
                                y_max,
                                direction: Direction::Up,
                            });
                        }
                        Direction::Right => {
                            new_beams.push(Beam {
                                x,
                                y,
                                x_max,
                                y_max,
                                direction: Direction::Down,
                            });
                        }
                        Direction::NA => {
                            panic!("NA direction")
                        }
                    };
                }
                HorizontalSplitter => match beam.direction {
                    Direction::Up | Direction::Down => {
                        new_beams.push(Beam {
                            x,
                            y,
                            x_max,
                            y_max,
                            direction: Direction::Right,
                        });
                        new_beams.push(Beam {
                            x,
                            y,
                            x_max,
                            y_max,
                            direction: Direction::Left,
                        });
                    }

                    _ => {
                        new_beams.push(Beam {
                            x,
                            y,
                            x_max,
                            y_max,
                            direction: beam.direction,
                        });
                    }
                },
                VerticalSplitter => match beam.direction {
                    Direction::Left | Direction::Right => {
                        new_beams.push(Beam {
                            x,
                            y,
                            x_max,
                            y_max,
                            direction: Direction::Down,
                        });
                        new_beams.push(Beam {
                            x,
                            y,
                            x_max,
                            y_max,
                            direction: Direction::Up,
                        });
                    }

                    _ => {
                        new_beams.push(Beam {
                            x,
                            y,
                            x_max,
                            y_max,
                            direction: beam.direction,
                        });
                    }
                },
                Normal => {
                    new_beams.push(Beam {
                        x,
                        y,
                        x_max,
                        y_max,
                        direction: beam.direction,
                    });
                }
            }
            let mut tile = *(tiles_map.get(&(x, y)).unwrap());
            tiles_map.remove(&(x, y));
            tile.set_energized(beam.direction);
            tiles_map.insert((x, y), tile);
        }
        beams = new_beams
            .into_iter()
            .unique()
            .filter(|beam| {
                !states.contains(&State {
                    x: beam.x,
                    y: beam.y,
                    direction: beam.direction,
                })
            })
            .collect::<Vec<Beam>>();
        beams.iter().for_each(|beam| {
            states.insert(State {
                x: beam.x,
                y: beam.y,
                direction: beam.direction,
            });
        });
    }
    tiles = tiles_map.values().cloned().collect::<Vec<Tile>>();
    tiles
        .clone()
        .iter()
        .filter(|tile| tile.is_energized)
        .count()
        .to_string()
}

fn string_repr(tiles: &Vec<Tile>) -> String {
    let mut tiles = tiles.clone();
    tiles.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
    let num_cols = tiles.iter().map(|tile| tile.x).max().unwrap() + 1;
    tiles
        .chunks(num_cols as usize)
        .map(|row| {
            row.iter()
                .map(|tile| tile.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, HashMap<(i32, i32), Tile>) {
    let mut cells = Vec::new();
    let mut tiles = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        let mut row = Vec::new();
        line.chars().enumerate().for_each(|(x, c)| {
            let cell_type = match c {
                '/' => CellType::LeftMirror,
                '\\' => CellType::RightMirror,
                '-' => CellType::HorizontalSplitter,
                '|' => CellType::VerticalSplitter,
                '.' => CellType::Normal,
                _ => panic!("Invalid cell type"),
            };
            let cell = Tile {
                x: x as i32,
                y: y as i32,
                is_energized: false,
                num_beams: 0,
                recent_beam_direction: Direction::NA,
                cell_type,
            };
            tiles.insert((x as i32, y as i32), cell);
            row.push(cell);
        });
        cells.push(row);
    });
    (cells, tiles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let result: String = part1(test_input);
        assert_eq!(result, "46".to_string());
    }
    #[test]
    fn test2() {
        let test_input: &str = r"....\....
.........
../.-.\..
.........
..\...\..
.........
.........";
        let result: String = part1(test_input);
        assert_eq!(result, "22".to_string());
    }
}
