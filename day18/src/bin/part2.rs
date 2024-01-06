struct Instruction {
    direction: (i64, i64),
    distance: i64,
}

fn main() {
    let input: &str = include_str!("input.txt");
    let result: String = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> String {
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
    let shoelace_area: i64 = vertices
        .windows(2)
        .map(|item| {
            let (x1, y1) = item[0];
            let (x2, y2) = item[1];
            x1 * y2 - x2 * y1
        })
        .sum::<i64>()
        / 2;

    let num_interior = shoelace_area + 1 - num_points / 2;
    let area = num_interior + num_points;
    area.to_string()
}

fn color_to_insn(input: &str) -> (char, i64) {
    let dist = i64::from_str_radix(&input[..5], 16).unwrap();
    let direction = match input
        .chars()
        .nth(5)
        .expect(format!("Invalid color: {}", input).as_str())
    {
        '0' => 'R',
        '1' => 'D',
        '2' => 'L',
        '3' => 'U',
        _ => panic!("Invalid direction"),
    };
    (direction, dist)
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    input.lines().for_each(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let hexadecimal = parts[2].to_string().replace("(#", "").replace(")", "");
        let (direction, dist) = color_to_insn(&hexadecimal);
        instructions.push(Instruction {
            direction: match direction {
                'R' => (1, 0),
                'L' => (-1, 0),
                'U' => (0, -1),
                'D' => (0, 1),
                _ => panic!("Invalid direction"),
            },
            distance: dist,
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
        let result: String = part2(test_input);
        assert_eq!(result, "952408144115".to_string());
    }
}
