struct Instruction {
    direction: (i32, i32),
    distance: i32,
}

fn main() {
    let input: &str = include_str!("input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    let instructions = parse(input);
    let mut vertices = Vec::new();
    let mut start_ptr = (0, 0);
    let mut num_points = 0;
    instructions.iter().for_each(|inst| {
        vertices.push(start_ptr);
        let dir = inst.direction;
        let n = inst.distance;
        for _ in 0..n {
            start_ptr = (start_ptr.0 + dir.0, start_ptr.1 + dir.1);
            num_points += 1;
        }
    });
    vertices.push((0, 0));
    let shoelace_area: i32 = vertices
        .windows(2)
        .map(|item| {
            let (x1, y1) = item[0];
            let (x2, y2) = item[1];
            x1 * y2 - x2 * y1
        })
        .sum::<i32>()
        / 2;

    let num_interior = shoelace_area + 1 - num_points / 2;
    let area = num_interior + num_points;
    area.to_string()
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    input.lines().for_each(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        instructions.push(Instruction {
            direction: parts[0]
                .chars()
                .map(|c| match c {
                    'R' => (1, 0),
                    'L' => (-1, 0),
                    'U' => (0, -1),
                    'D' => (0, 1),
                    _ => panic!("Invalid direction"),
                })
                .collect::<Vec<(i32, i32)>>()[0],
            distance: parts[1].parse::<i32>().unwrap(),
        })
    });
    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let result: String = part1(test_input);
        assert_eq!(result, "62".to_string());
    }
}
