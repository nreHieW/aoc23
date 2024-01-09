use core::panic;
use std::{collections::HashMap, fmt};
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Hail {
    initial_pos: (i64, i64, i64),
    x_velocity: i64,
    y_velocity: i64,
    z_velocity: i64,
}

impl fmt::Display for Hail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {} @ {}, {}, {}",
            self.initial_pos.0,
            self.initial_pos.1,
            self.initial_pos.2,
            self.x_velocity,
            self.y_velocity,
            self.z_velocity
        )
    }
}

fn main() {
    let input: &str = include_str!("input.txt");
    let result: String = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> String {
    let hails = parse(input);
    // solve_z3(&hails)
    solve_linear_alg(&hails)
}

fn solve_linear_alg(hails: &Vec<Hail>) -> String {
    use itertools::Itertools;
    // Given some hail in the form u + Av and let the rock be of the form p + Kq
    // At some time t where they intersect
    // u + tv = p + tq
    // u + (tp - tq) = p
    // u + t(p - q) = p
    // This implies that if we set the rock as stationary we can do brute force by altering the hail's movement vector since q is the same across all hailstones
    // The location where the adjusted hailstones intersect is the location of the rock

    // let mut min_vel = hails
    //     .iter()
    //     .map(|x| (x.x_velocity.min(x.y_velocity)).min(x.z_velocity))
    //     .min()
    //     .unwrap();
    // let mut max_vel = hails
    //     .iter()
    //     .map(|x| (x.x_velocity.max(x.y_velocity)).max(x.z_velocity))
    //     .max()
    //     .unwrap();
    // min_vel = min_vel.min(-1 * max_vel);
    // max_vel = max_vel.max(-1 * min_vel);
    let min_vel = -200;
    let max_vel = 200;

    let mut possible_solutions = HashMap::new();
    for i in min_vel..=max_vel {
        for j in min_vel..=max_vel {
            let adj = (i, j);
            let adjusted_hails = hails
                .clone()
                .iter()
                .map(|x| Hail {
                    initial_pos: x.initial_pos,
                    x_velocity: x.x_velocity + adj.0,
                    y_velocity: x.y_velocity + adj.1,
                    z_velocity: x.z_velocity,
                })
                .collect::<Vec<Hail>>();

            // Optimization taken from https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/ker0hj8/?utm_source=share&utm_medium=web2x&context=3
            let first = adjusted_hails[0].clone();
            let first_check = adjusted_hails
                .iter()
                .skip(1)
                .all(|x| find_intersection(first, *x).is_some());

            if !first_check {
                continue;
            }
            let hail_pairs = adjusted_hails.iter().combinations(2);
            let intersections = hail_pairs
                .filter_map(|x| find_intersection(x[0].clone(), x[1].clone()))
                .collect::<Vec<(f64, f64)>>();
            let biggest_x_diff = intersections
                .iter()
                .map(|x| (x.0 - intersections[0].0).abs())
                .fold(0.0, |acc, x| f64::max(acc, x));
            let biggest_y_diff = intersections
                .iter()
                .map(|x| (x.1 - intersections[0].1).abs())
                .fold(0.0, |acc, x| f64::max(acc, x));
            possible_solutions.insert(
                (biggest_x_diff + biggest_y_diff) as i64,
                (adj, intersections[0]),
            );
        }
    }
    let smallest_key = possible_solutions.keys().min().unwrap();
    let (adj, intersection) = possible_solutions.get(smallest_key).unwrap();
    let (x, y) = intersection.clone();
    let (direction_x, direction_y) = (adj.0 as i64 * -1, adj.1 as i64 * -1);
    let mut z: Option<i64> = None;
    for item in hails.windows(2) {
        let first_hail = item[0];
        let second_hail = item[1];

        if (direction_x - first_hail.x_velocity) == 0
            || (direction_y - first_hail.y_velocity) == 0
            || (direction_x - second_hail.x_velocity) == 0
            || (direction_y - second_hail.y_velocity) == 0
        {
            continue;
        }

        let t =
            (first_hail.initial_pos.0 as f64 - x) as i64 / (direction_x - first_hail.x_velocity);
        let check_t =
            (first_hail.initial_pos.1 as f64 - y) as i64 / (direction_y - first_hail.y_velocity);

        if t != check_t {
            panic!("t = {}, check t = {}", t, check_t);
        }
        let pz_tqz = first_hail.initial_pos.2 + t * first_hail.z_velocity;

        let t1 =
            (second_hail.initial_pos.0 as f64 - x) as i64 / (direction_x - second_hail.x_velocity);
        let check_t1 =
            (second_hail.initial_pos.1 as f64 - y) as i64 / (direction_y - second_hail.y_velocity);

        if t1 != check_t1 {
            panic!("t = {}, check t = {}", t1, check_t1);
        }
        let pz_t1qz = second_hail.initial_pos.2 + t1 * second_hail.z_velocity;
        if t1 == t {
            continue;
        }
        let qz = (pz_tqz - pz_t1qz) / (t - t1);
        z = Some(pz_tqz - t * qz);
        break;
    }
    if z.is_none() {
        panic!("No solution found");
    }
    (x as i64 + y as i64 + z.unwrap()).to_string()
}

fn find_intersection(first: Hail, second: Hail) -> Option<(f64, f64)> {
    let a = (first.initial_pos.0 as f64, first.initial_pos.1 as f64);
    let b = (second.initial_pos.0 as f64, second.initial_pos.1 as f64);
    let ad = (first.x_velocity as f64, first.y_velocity as f64);
    let bd = (second.x_velocity as f64, second.y_velocity as f64);

    // https://stackoverflow.com/questions/2931573/determining-if-two-rays-intersect
    let det = bd.0 * ad.1 - bd.1 * ad.0;

    if det == 0.0 {
        // They are parallel
        return None;
    }
    let u = ((b.1 - a.1) * bd.0 - (b.0 - a.0) * bd.1) / det;
    let v = ((b.1 - a.1) * ad.0 - (b.0 - a.0) * ad.1) / det;

    if u >= 0.0 && v >= 0.0 {
        let intersect = (a.0 + u * ad.0, a.1 + u * ad.1);
        return Some(intersect);
    }
    None
}

#[allow(dead_code)]
fn solve_z3(hails: &Vec<Hail>) -> String {
    let _ = hails.clone();
    //     use z3::ast::Ast;
    //     use z3::{ast, Config, Context, SatResult, Solver};
    //     // References: https://gist.github.com/icub3d/bc414304b0c336a009658c5b84f455f7 and ChatGPT 4

    //     // Create a Z3 configuration
    //     let cfg = Config::new();

    //     // Create a Z3 context
    //     let ctx = Context::new(&cfg);

    //     // Create a Z3 solver
    //     let solver = Solver::new(&ctx);

    //     let xr = ast::Int::new_const(&ctx, "xr");
    //     let yr = ast::Int::new_const(&ctx, "yr");
    //     let zr = ast::Int::new_const(&ctx, "zr");
    //     let xv = ast::Int::new_const(&ctx, "xv");
    //     let yv = ast::Int::new_const(&ctx, "yv");
    //     let zv = ast::Int::new_const(&ctx, "zv");

    //     for i in 0..3 {
    //         let curr = hails[i];
    //         let t = ast::Int::new_const(&ctx, format!("t{}", i).as_str());

    //         let xh = ast::Int::from_i64(&ctx, curr.initial_pos.0);
    //         let yh = ast::Int::from_i64(&ctx, curr.initial_pos.1);
    //         let zh = ast::Int::from_i64(&ctx, curr.initial_pos.2);
    //         let xvh = ast::Int::from_i64(&ctx, curr.x_velocity);
    //         let yvh = ast::Int::from_i64(&ctx, curr.y_velocity);
    //         let zvh = ast::Int::from_i64(&ctx, curr.z_velocity);

    //         solver.assert(&(&xh + &xvh * &t)._eq(&(&xr + &xv * &t)));
    //         solver.assert(&(&yh + &yvh * &t)._eq(&(&yr + &yv * &t)));
    //         solver.assert(&(&zh + &zvh * &t)._eq(&(&zr + &zv * &t)));
    //     }

    //     match solver.check() {
    //         SatResult::Sat => {
    //             // Get the model
    //             let model = solver.get_model().unwrap();

    //             let xr = model.eval(&xr, true).unwrap().as_i64().unwrap();
    //             let yr = model.eval(&yr, true).unwrap().as_i64().unwrap();
    //             let zr = model.eval(&zr, true).unwrap().as_i64().unwrap();

    //             xr + yr + zr
    //         }
    //         _ => -1,
    //     }
    //     .to_string()
    todo!()
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
        let result: String = part2(test_input);
        assert_eq!(result, "47".to_string());
    }
}
