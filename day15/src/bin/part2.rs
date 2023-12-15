#[derive(Debug, PartialEq, Eq, Clone)]
struct Lens {
    label: String,
    focal_length: i32,
}

#[derive(Debug)]
struct Box {
    idx: usize,
    lenses: Vec<Lens>,
}

impl Box {
    fn contains_label(&self, other_label: &str) -> bool {
        self.lenses.iter().any(|l| l.label == other_label)
    }

    fn remove_lens(&mut self, to_remove_label: &str) {
        if self.contains_label(to_remove_label) {
            self.lenses = self
                .lenses
                .iter()
                .filter(|l| l.label != to_remove_label)
                .cloned()
                .collect::<Vec<Lens>>();
        }
    }

    fn add_lens(&mut self, to_add: Lens) {
        if !self.contains_label(&to_add.label) {
            self.lenses.push(to_add);
        } else {
            let idx = self
                .lenses
                .iter()
                .position(|l| l.label == to_add.label)
                .unwrap();
            self.lenses[idx] = to_add;
        }
    }

    fn calc_power(&self) -> i32 {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, l)| (1 + self.idx as i32) * (i as i32 + 1) * l.focal_length)
            .sum()
    }
}

fn main() {
    let input: &str = include_str!("./input.txt");
    let result: String = part2(input);
    println!("Result: {}", result);
}
fn part2(input: &str) -> String {
    let binding = input.replace("\n", "");
    let segments: Vec<&str> = binding.split(',').collect();
    let mut boxes = (0..255)
        .map(|i| Box {
            idx: i,
            lenses: Vec::new(),
        })
        .collect::<Vec<Box>>();
    segments.iter().for_each(|s| process_segment(&mut boxes, s));
    boxes
        .iter()
        .map(|b| b.calc_power())
        .sum::<i32>()
        .to_string()
}

fn process_segment(boxes: &mut Vec<Box>, input: &str) {
    if input.contains("-") {
        let input = input.split("-").collect::<Vec<&str>>();
        let label = input[0];
        let box_id = hash_algo(label) as usize;
        let curr_box = &mut boxes[box_id];
        curr_box.remove_lens(label);
    } else if input.contains("=") {
        let input = input.split("=").collect::<Vec<&str>>();
        let label = input[0];
        let focal_length = input[1].parse::<i32>().expect("Invalid focal length");
        let curr_lens = Lens {
            label: label.to_string(),
            focal_length,
        };
        let box_id = hash_algo(label) as usize;
        let curr_box = &mut boxes[box_id];
        curr_box.add_lens(curr_lens);
    } else {
        panic!("Invalid segment: {}", input);
    }
}

fn hash_algo(input: &str) -> i32 {
    let mut curr = 0;
    input.chars().for_each(|c| {
        let ascii = c as i32;
        curr += ascii;
        curr *= 17;
        curr = curr % 256;
    });
    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result: String = part2(test_input);
        assert_eq!(result, "145".to_string());
    }
}
