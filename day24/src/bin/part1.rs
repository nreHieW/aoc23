#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Hail {
    initial_pos: (i64, i64, i64),
    x_velocity: i64,
    y_velocity: i64,
    z_velocity: i64,
}
fn main() {
    let input: &str = include_str!("input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    let start: i64 = 200000000000000;
    let end: i64 = 400000000000000;
    // let start = 7;
    // let end = 27;
    let test_area = (start as f64)..=(end as f64);
    let hails = parse(input);

    hails
        .iter()
        .enumerate()
        .map(|(i, first)| {
            let c = hails.clone();
            let mut count = 0;
            for (j, second) in c.iter().enumerate() {
                if i <= j {
                    continue;
                }
                let a = (first.initial_pos.0 as f64, first.initial_pos.1 as f64);
                let b = (second.initial_pos.0 as f64, second.initial_pos.1 as f64);
                let ad = (first.x_velocity as f64, first.y_velocity as f64);
                let bd = (second.x_velocity as f64, second.y_velocity as f64);

                // https://stackoverflow.com/questions/2931573/determining-if-two-rays-intersect
                let det = bd.0 * ad.1 - bd.1 * ad.0;

                if det == 0.0 {
                    // They are parallel
                    continue;
                }
                let u = ((b.1 - a.1) * bd.0 - (b.0 - a.0) * bd.1) / det;
                let v = ((b.1 - a.1) * ad.0 - (b.0 - a.0) * ad.1) / det;

                if u >= 0.0 && v >= 0.0 {
                    let intersect = (a.0 + u * ad.0, a.1 + u * ad.1);
                    if test_area.contains(&intersect.0) && test_area.contains(&intersect.1) {
                        count += 1;
                    }
                }
            }

            count
        })
        .sum::<usize>()
        .to_string()
}

fn parse(input: &str) -> Vec<Hail> {
    let mut hails = Vec::new();
    input.lines().for_each(|line| {
        let tmp = line.split("@").collect::<Vec<&str>>();
        let pos = tmp[0]
            .trim()
            .split(",")
            .map(|x| {
                x.trim()
                    .parse::<i64>()
                    .expect(format!("Failed to parse {}", x).as_str())
            })
            .collect::<Vec<i64>>();
        let vel = tmp[1]
            .trim()
            .split(",")
            .map(|x| {
                x.trim()
                    .parse::<i64>()
                    .expect(format!("Failed to parse {}", x).as_str())
            })
            .collect::<Vec<i64>>();
        hails.push(Hail {
            initial_pos: (pos[0], pos[1], pos[2]),
            x_velocity: vel[0],
            y_velocity: vel[1],
            z_velocity: vel[2],
        })
    });
    hails
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        let result: String = part1(test_input);
        assert_eq!(result, "2".to_string());
    }
}
