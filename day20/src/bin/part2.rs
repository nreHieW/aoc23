use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
struct FlipFlop {
    state: bool, // either on or off
    destinations: Vec<String>,
}

impl FlipFlop {
    fn process(&mut self, input: Pulse) -> Option<Pulse> {
        if input == Pulse::High {
            None
        } else {
            match self.state {
                true => {
                    self.state = false;
                    Some(Pulse::Low)
                }
                false => {
                    self.state = true;
                    Some(Pulse::High)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    state: bool, // is it all high?
    destinations: Vec<String>,
    past_inputs: HashMap<String, Pulse>,
}

impl Conjunction {
    fn process(&mut self, input: Pulse, source: String) -> Option<Pulse> {
        self.past_inputs.insert(source, input);
        self.update();
        if self.state {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn update(&mut self) {
        self.state = self.past_inputs.values().all(|x| *x == Pulse::High);
    }

    fn update_connections(&mut self, connections: Vec<String>) {
        connections.iter().for_each(|c| {
            self.past_inputs.insert(c.to_string(), Pulse::Low);
        });
    }
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

impl Module {
    fn process(&mut self, input: Pulse, source: String) -> Option<Pulse> {
        match self {
            Module::FlipFlop(f) => f.process(input),
            Module::Conjunction(c) => c.process(input, source),
        }
    }

    fn get_destinations(&self) -> Vec<String> {
        match self {
            Module::FlipFlop(f) => f.destinations.clone(),
            Module::Conjunction(c) => c.destinations.clone(),
        }
    }

    fn update_connections(&mut self, connections: Vec<String>) {
        match self {
            Module::FlipFlop(_) => {}
            Module::Conjunction(c) => c.update_connections(connections),
        }
    }

    fn get_type(&self) -> String {
        match self {
            Module::FlipFlop(_) => "FlipFlop".to_string(),
            Module::Conjunction(_) => "Conjunction".to_string(),
        }
    }
}

struct State {
    curr_module: String,
    input_pulse: Pulse,
    source: String, // Source of the input pulse
}

fn main() {
    let input: &str = include_str!("input.txt");
    let result: String = part2(input);
    println!("Result: {}", result);
}

fn part2(input: &str) -> String {
    let (mut modules, broadcaster) = parse(input);
    let mut q = VecDeque::new();

    // Hard coded
    // let direct_child = "bn"; // has to be low
    let indirect_childs = vec!["pl", "mz", "lz", "zm"]; // has to be high
    let mut periods: HashMap<String, Vec<i64>> = HashMap::new();
    for i in 0..20_000 {
        // single button push
        broadcaster.iter().for_each(|d| {
            q.push_back(State {
                curr_module: d.to_string(),
                input_pulse: Pulse::Low,
                source: "button".to_string(),
            });
        });

        while q.len() != 0 {
            let curr_state = q.pop_front().unwrap();

            if curr_state.curr_module == "rx" && curr_state.input_pulse == Pulse::Low {
                return i.to_string();
            }

            if modules.get(&curr_state.curr_module).is_none() {
                continue;
            }

            let mut curr_module = modules
                .get(&curr_state.curr_module)
                .expect(format!("Invalid module: {}", curr_state.curr_module).as_str())
                .clone();
            modules.remove(&curr_state.curr_module);
            let out = curr_module.process(curr_state.input_pulse, curr_state.source);
            if out.is_some() {
                let out_pulse = out.unwrap();

                if out_pulse == Pulse::High
                    && indirect_childs.contains(&curr_state.curr_module.as_str())
                {
                    if periods.get(&curr_state.curr_module).is_none() {
                        periods.insert(curr_state.curr_module.to_string(), vec![i]);
                    } else {
                        let mut tmp = periods
                            .get(&curr_state.curr_module)
                            .expect("Invalid module")
                            .clone();
                        tmp.push(i);
                        periods.insert(curr_state.curr_module.to_string(), tmp);
                    }
                }

                let destinations = curr_module.get_destinations();
                destinations.iter().for_each(|d| {
                    q.push_back(State {
                        curr_module: d.to_string(),
                        input_pulse: out_pulse,
                        source: curr_state.curr_module.to_string(),
                    });
                });
            }

            modules.insert(curr_state.curr_module, curr_module);
        }
    }
    // periods.iter().for_each(|(k, v)| {
    //     let increments = v.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i64>>();
    //     println!("{}: {:?}", k, increments);
    // });

    periods
        .iter()
        .map(|(_, v)| {
            let increments = v.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i64>>();
            increments[0]
        })
        .fold(1, |acc, x| acc * x)
        .to_string()
}

fn parse(input: &str) -> (HashMap<String, Module>, Vec<String>) {
    let mut modules = HashMap::new();
    let mut broadcaster = Vec::new();
    let mut tmp = Vec::new();
    input.lines().for_each(|line| {
        let split_vals = line.split(" -> ").collect::<Vec<&str>>();
        let curr = split_vals[0];
        let dests = split_vals[1].split(", ").map(|x| x.to_string()).collect();

        if curr.starts_with("broadcaster") {
            broadcaster = dests;
        } else if curr.starts_with("%") {
            let new_mod = Module::FlipFlop(FlipFlop {
                state: false,
                destinations: dests,
            });
            tmp.push((curr[1..].to_string(), new_mod));
        } else if curr.starts_with("&") {
            let new_mod = Module::Conjunction(Conjunction {
                state: false,
                destinations: dests,
                past_inputs: HashMap::new(),
            });
            tmp.push((curr[1..].to_string(), new_mod));
        } else {
            panic!("Invalid input");
        }
    });
    tmp.iter().for_each(|(name, module)| {
        let mut connections = Vec::new();
        if module.get_type() == "Conjunction" {
            tmp.iter().for_each(|(name2, module2)| {
                if module2.get_destinations().contains(name) {
                    connections.push(name2.to_string());
                }
            });
        }
        let mut new_module = module.clone();
        new_module.update_connections(connections);
        modules.insert(name.to_string(), new_module);
    });
    (modules, broadcaster)
}
