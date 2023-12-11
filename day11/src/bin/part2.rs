#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cell {
    x: i64,
    y: i64,
    val: char,
    id: i64,
}

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> String {
    let (cells, galaxies) = parse(input);
    let (blank_rows, blank_cols) = get_blank(cells);

    let mut res = 0;

    galaxies.iter().enumerate().for_each(|(i, c)| {
        galaxies.iter().enumerate().for_each(|(j, other)| {
            let mut indiv = 0;

            if j <= i {
                return;
            }
            let start_x = c.x;
            let start_y = c.y;
            let end_x = other.x;
            let end_y = other.y;

            let x_range = if start_x < end_x {
                start_x..end_x
            } else {
                end_x..start_x
            };

            let y_range = if start_y < end_y {
                start_y..end_y
            } else {
                end_y..start_y
            };

            indiv += x_range.end - x_range.start;
            indiv += y_range.end - y_range.start;

            let increment = 1000000 - 1;

            blank_rows.iter().for_each(|r| {
                if y_range.contains(r) {
                    indiv += increment;
                }
            });

            blank_cols.iter().for_each(|c| {
                if x_range.contains(c) {
                    indiv += increment;
                }
            });
            res += indiv;
        })
    });

    res.to_string()
}

fn parse(input: &str) -> (Vec<Vec<Cell>>, Vec<Cell>) {
    let mut cells = Vec::new();
    let mut galaxies = Vec::new();

    let mut curr = 1;

    input.lines().enumerate().for_each(|(y, line)| {
        cells.push(Vec::new());
        line.chars().enumerate().for_each(|(x, val)| {
            let c = Cell {
                x: x as i64,
                y: y as i64,
                val: val,
                id: if val == '#' { curr } else { 0 },
            };

            cells[y].push(c);

            if val == '#' {
                galaxies.push(c);
                curr += 1;
            }
        });
    });

    (cells, galaxies)
}

fn get_blank(cells: Vec<Vec<Cell>>) -> (Vec<i64>, Vec<i64>) {
    let mut blank_rows = Vec::new();
    let mut blank_cols = Vec::new();

    let height = cells.len();
    let width = cells[0].len();

    for i in 0..height {
        let row = cells[i].clone();

        if row.iter().all(|c| c.val == '.') {
            blank_rows.push(i as i64);
        }
    }

    for i in 0..width {
        let mut col = Vec::new();

        for j in 0..height {
            col.push(cells[j][i]);
        }

        if col.iter().all(|c| c.val == '.') {
            blank_cols.push(i as i64);
        }
    }

    (blank_rows, blank_cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result: String = part2(test_input);
        assert_eq!(result, "374".to_string());
    }
}
