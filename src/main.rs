use std::collections::{HashMap, HashSet};
use std::cmp;

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
            let destinations = adjacency_list
                .entry(source)
                .or_insert_with(|| Vec::new());

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
    let mut edges: HashMap<usize, Vec<(usize, usize)>> = g.edges.clone();
    let mut nodes_unexplored = g.nodes.clone();
    let num_nodes = g.nodes.len();

    let mut nodes_dists: HashMap<usize, usize> = HashMap::with_capacity(num_nodes);
    nodes_dists.entry(start).or_insert(0);

    let exploring = start;
    nodes_unexplored.remove(&exploring);
    let edge_explore = edges.get(&exploring).unwrap();
    let mut val : usize = 20;
    
    for edg in edge_explore {
        let temp_val = *nodes_dists.entry(edg.0)
            .and_modify(|val| { if *val > edg.1 {
                *val = edg.1;
            }})
            .or_insert(edg.1);
        if temp_val < val {
            println!("{}", temp_val);
            val = temp_val;
        }
    }
    

    println!("{:?}", edge_explore);
    println!("{:?}", nodes_dists);

    None
}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = shortest_path(
            &g, 1000, 9000) {
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
