use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    x: i32,
    y: i32,
    direction: (i32, i32),
    steps_in_direction: i32,
}

fn part1(input: &str) -> String {
    let start_x = 0;
    let start_y = 0;
    let num_lines = input.lines().count();
    let num_columns = input.lines().next().unwrap().chars().count();
    let cells = parse(input);
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut pq = PriorityQueue::new();
    pq.push(
        State {
            x: start_x,
            y: start_y,
            direction: (0, 0),
            steps_in_direction: 0,
        },
        Reverse(0),
    );
    let mut visited = HashSet::new();
    while let Some(curr) = pq.pop() {
        let curr_state = curr.0;
        let heatloss = curr.1 .0;

        // Does not matter if we have multiple of the same states with different priority, we are guranteed to early circuit the best possible path
        if curr_state.x == num_columns as i32 - 1 && curr_state.y == num_lines as i32 - 1 {
            return heatloss.to_string();
        }

        if visited.contains(&curr_state) {
            continue;
        }
        visited.insert(curr_state);
        directions.iter().for_each(|direction| {
            let new_x = curr_state.x + direction.0;
            let new_y = curr_state.y + direction.1;

            if new_x < 0 || new_y < 0 || new_x >= num_columns as i32 || new_y >= num_lines as i32 {
                return;
            }
            let new_heatloss = Reverse(heatloss + cells[&(new_x, new_y)]);

            // Same direction: Check if less than 2 steps taken then increment
            if *direction == curr_state.direction {
                if curr_state.steps_in_direction < 3 {
                    pq.push_increase(
                        State {
                            x: new_x,
                            y: new_y,
                            direction: *direction,
                            steps_in_direction: curr_state.steps_in_direction + 1,
                        },
                        new_heatloss,
                    );
                }
            } else if *direction != (-curr_state.direction.0, -curr_state.direction.1) {
                pq.push_increase(
                    State {
                        x: new_x,
                        y: new_y,
                        direction: *direction,
                        steps_in_direction: 1,
                    },
                    new_heatloss,
                );
            }
        })
    }
    "-1".to_string()
}

fn parse(input: &str) -> HashMap<(i32, i32), i32> {
    let mut cells: HashMap<(i32, i32), i32> = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            cells.insert((x as i32, y as i32), c.to_digit(10).unwrap() as i32);
        })
    });
    cells
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let result: String = part1(test_input);
        assert_eq!(result, "102".to_string());
    }
    #[test]
    fn test2() {
        let test_input = "112999
911111";
        let result: String = part1(test_input);
        assert_eq!(result, "7".to_string());
    }
}
