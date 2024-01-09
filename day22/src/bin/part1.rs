use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Brick {
    first_end: (i32, i32, i32),
    second_end: (i32, i32, i32),
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let smaller_z = self.first_end.2.min(self.second_end.2);
        let other_smaller_z = other.first_end.2.min(other.second_end.2);
        smaller_z.cmp(&other_smaller_z)
    }
}
impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Brick {
    fn gen_xy_coord(&self) -> Vec<(i32, i32)> {
        let x_range =
            self.first_end.0.min(self.second_end.0)..=self.first_end.0.max(self.second_end.0);
        let y_range =
            self.first_end.1.min(self.second_end.1)..=self.first_end.1.max(self.second_end.1);
        x_range.cartesian_product(y_range).collect::<Vec<_>>()
    }
}

fn main() {
    let input: &str = include_str!("input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    let mut bricks = parse(input);
    bricks.sort();

    let mut mapping: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new(); // This mapping represents the active z coordinates and tte brick index at a given x,y coordinate
    bricks.iter().enumerate().for_each(|(i, x)| {
        let first_xy = (x.first_end.0, x.first_end.1);
        let second_xy = (x.second_end.0, x.second_end.1);

        if first_xy == second_xy {
            // This is a vertical brick
            let height = (x.first_end.2 - x.second_end.2).abs();
            if let Some(v) = mapping.get_mut(&first_xy) {
                let last_val = v.last().unwrap().1;
                let new_bottom = last_val + 1;
                for z in new_bottom..=new_bottom + height {
                    v.push((i as i32, z));
                }
            } else {
                let new_bottom = 1;
                for z in new_bottom..=new_bottom + height {
                    mapping.insert(first_xy, vec![(i as i32, z)]);
                }
            }
        } else {
            // This is a horizontal brick so the new bottom should be the same for all coordinates
            let coords = x.gen_xy_coord();
            let mut new_bottom = 1;
            coords.iter().for_each(|coord| {
                if let Some(v) = mapping.get(&coord) {
                    let last_val = v.last().unwrap().1;
                    new_bottom = new_bottom.max(last_val + 1);
                }
            });

            for coord in coords {
                if let Some(v) = mapping.get_mut(&coord) {
                    v.push((i as i32, new_bottom));
                } else {
                    mapping.insert(coord, vec![(i as i32, new_bottom)]);
                }
            }
        }
    });
    let mut edges = HashSet::new(); // (Source, Destination)
    mapping.iter().for_each(|(_k, v)| {
        let idxs = v.iter().map(|(idx, _val)| idx).collect::<Vec<_>>();
        let vals = v.iter().map(|(_idx, val)| val).collect::<Vec<_>>();
        idxs.iter()
            .zip(vals.iter())
            .tuple_windows()
            .for_each(|((a, a_val), (b, b_val))| {
                if a != b && (*a_val - *b_val).abs() == 1 {
                    edges.insert((*a, *b));
                }
            });
    });
    let num_disintegrate = (0..bricks.len())
        .filter_map(|x| {
            let sources = edges
                .iter()
                .filter(|(_src, dest)| **dest == (x as i32))
                .collect::<Vec<_>>();
            if sources.len() == 1 {
                Some(sources[0].0)
            } else {
                None
            }
        })
        .unique()
        .collect::<Vec<_>>()
        .len();

    (bricks.len() - num_disintegrate).to_string()
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let tmp = line.split('~').collect::<Vec<_>>();
            let first_end = tmp[0]
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let second_end = tmp[1]
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            Brick {
                first_end: (first_end[0], first_end[1], first_end[2]),
                second_end: (second_end[0], second_end[1], second_end[2]),
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        let result: String = part1(test_input);
        assert_eq!(result, "5".to_string());
    }
}
