use std::collections::{HashMap, HashSet};

type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list.entry(source).or_insert_with(|| Vec::new());

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    let edges: HashMap<usize, Vec<(usize, usize)>> = g.edges.clone();
    let mut temp_node_dists: HashMap<usize, usize> = HashMap::with_capacity(g.nodes.len());
    let mut final_node_dists: HashMap<usize, usize> = HashMap::with_capacity(g.nodes.len());
    let mut next_search = (&start, &(0));

    while final_node_dists.len() < g.nodes.len() {
        let exploring = *next_search.0;
        let dist_to_start = *next_search.1;
        let edge_explore = edges.get(&exploring).unwrap();
        final_node_dists.entry(*next_search.0).or_insert(*next_search.1);
        temp_node_dists.remove_entry(&exploring);
        if exploring == goal {
            return Some((vec![start, goal], dist_to_start));
        }
        for edg in edge_explore {
            if final_node_dists.contains_key(&edg.0) {
                continue;
            } else {
                temp_node_dists
                    .entry(edg.0)
                    .and_modify(|val| {
                        if *val > (edg.1 + dist_to_start) {
                            *val = edg.1 + dist_to_start;
                        }
                    })
                    .or_insert(edg.1 + dist_to_start);
            }
        }
        next_search = temp_node_dists.iter().min_by_key(|entry| entry.1).unwrap();
    }
    None
}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = shortest_path(&g, 1000, 9000) {
        println!("1000->9000, {:?} {}", path, cost);
    };
}

#[test]
fn large_graph() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1000, 9000);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 24);
}
