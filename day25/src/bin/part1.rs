use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input: &str = include_str!("input.txt");
    let result: String = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    let (edges, graph, all_nodes) = parse(input);
    let mut paths = HashMap::new();
    all_nodes.iter().for_each(|node| {
        let path = bfs(graph.clone(), node.clone());
        paths.insert(node.clone(), path);
    });
    let mut edge_counts = HashMap::new();

    all_nodes.iter().combinations(2).for_each(|x| {
        let first = x[0].clone();
        let second = x[1].clone();
        let p = paths.get(&first).unwrap().get(&second).unwrap();
        p.windows(2).for_each(|w| {
            let f = w[0].clone();
            let s = w[1].clone();
            let curr_edge = if edges.contains(&(f.clone(), s.clone())) {
                (f, s)
            } else {
                (s, f)
            };
            if edge_counts.contains_key(&curr_edge) {
                let count = edge_counts.get(&curr_edge).unwrap();
                edge_counts.insert(curr_edge, count + 1);
            } else {
                edge_counts.insert(curr_edge, 1);
            }
        })
    });
    let mut flattend = edge_counts.iter().collect::<Vec<_>>();
    flattend.sort_by(|a, b| (a.1.cmp(b.1)).reverse()); // Sort by descending order
    let top3 = flattend.iter().take(3).collect::<Vec<_>>();
    let mut new_edges = edges.clone();

    // heuristic to remove edges
    top3.iter().for_each(|x| {
        let edge = x.0;
        new_edges.retain(|e| e != edge);
    });
    let (new_graph, _) = edge_to_graph(new_edges);

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let start = all_nodes[0].clone();
    queue.push_back(start.clone());

    while let Some(curr_node) = queue.pop_front() {
        if visited.contains(&curr_node) {
            continue;
        }
        visited.insert(curr_node.clone());
        let neighbours = new_graph.get(&curr_node).unwrap();
        neighbours.iter().for_each(|neighbour| {
            queue.push_back(neighbour.clone());
        });
    }

    let total = all_nodes.len();
    let visited = visited.len();

    (visited * (total - visited)).to_string()
}

fn bfs(graph: HashMap<String, Vec<String>>, start: String) -> HashMap<String, Vec<String>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(vec![start]);

    let mut paths = HashMap::new();

    while let Some(curr_path) = queue.pop_front() {
        let curr_node = curr_path.last().unwrap().clone();
        if visited.contains(&curr_node) {
            continue;
        }
        visited.insert(curr_node.clone());
        paths.insert(curr_node.clone(), curr_path.clone());
        let neighbours = graph.get(&curr_node).unwrap();
        neighbours.iter().for_each(|neighbour| {
            let mut new_path = curr_path.clone();
            new_path.push(neighbour.clone());
            queue.push_back(new_path);
        });
    }

    paths
}

fn parse(
    input: &str,
) -> (
    Vec<(String, String)>,
    HashMap<String, Vec<String>>,
    Vec<String>,
) {
    let mut edges = Vec::new();
    input.lines().for_each(|x| {
        let tmp = x.split(":").collect::<Vec<&str>>();
        let curr = tmp[0].trim().to_string();
        let children = tmp[1]
            .trim()
            .split(" ")
            .map(|x| x.trim().to_string())
            .collect::<Vec<String>>();
        children.iter().for_each(|child| {
            let edge = (curr.clone(), child.clone());
            edges.push(edge);
        })
    });

    let (graph, all_nodes) = edge_to_graph(edges.clone());
    (edges, graph, all_nodes)
}

fn edge_to_graph(edges: Vec<(String, String)>) -> (HashMap<String, Vec<String>>, Vec<String>) {
    let mut graph = HashMap::new();
    let all_nodes = edges
        .iter()
        .flat_map(|(x, y)| vec![x.clone(), y.clone()])
        .unique()
        .collect::<Vec<String>>();
    all_nodes.iter().for_each(|node| {
        let neighbours = edges
            .iter()
            .filter(|(x, y)| x == node || y == node)
            .map(|(x, y)| if x == node { y.clone() } else { x.clone() })
            .collect::<Vec<String>>();
        graph.insert(node.clone(), neighbours);
    });
    (graph, all_nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_input: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        let result: String = part1(test_input);
        assert_eq!(result, "54".to_string());
    }
}
