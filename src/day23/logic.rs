use crate::day23::models::ConnectionMap;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
// use petgraph::dot::{Config::EdgeNoLabel, Dot};
use petgraph::prelude::NodeIndex;
use petgraph::{Graph, Undirected};

fn bron_kerbosch_1(
    graph: &Graph<String, i32, Undirected>,
    r: HashSet<NodeIndex>,
    mut p: HashSet<NodeIndex>,
    mut x: HashSet<NodeIndex>,
    max_clique: &mut HashSet<NodeIndex>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            let _ = std::mem::replace(max_clique, r.clone());
        }
    }
    if p.is_empty() {
        return;
    }

    for node in p.clone() {
        // Add the vertex to r
        let mut r = r.clone();
        r.insert(node);
        // Get the neighbor set of the node
        let neighbors: HashSet<_> = graph.neighbors(node).into_iter().collect();
        bron_kerbosch_1(
            graph,
            r,
            p.intersection(&neighbors).cloned().collect(),
            x.intersection(&neighbors).cloned().collect(),
            max_clique
        );
        p.remove(&node);
        x.insert(node);
    }
}

pub fn solve_part_one(connection_map: &ConnectionMap) -> usize {
    // Create a graph for representation purposes
    let mut graph = Graph::new_undirected();

    // Create nodes
    let mut nodes = HashMap::new();
    for connection in &connection_map.connections {
        let start = *nodes
            .entry(connection.0.clone())
            .or_insert_with(|| graph.add_node(connection.0.clone()));
        let end = *nodes
            .entry(connection.1.clone())
            .or_insert_with(|| graph.add_node(connection.1.clone()));
        graph.add_edge(start, end, 1);
    }

    // Store found set
    let mut found_sets: HashSet<Vec<&String>> = HashSet::new();

    // For every node in the graph, look for two friends
    for node in graph.node_indices() {
        for neighbor in graph.neighbors(node) {
            // For every neighbor of node
            for neighbor_2 in graph.neighbors(neighbor) {
                // if neighbor_2 has node as a neighbor, then it form a set of three computers with
                // node
                if graph.contains_edge(node, neighbor_2) {
                    let n1 = graph.node_weight(node).unwrap();
                    let n2 = graph.node_weight(neighbor).unwrap();
                    let n3 = graph.node_weight(neighbor_2).unwrap();
                    if n1.starts_with("t") || n2.starts_with("t") || n3.starts_with("t") {
                        let mut nodes = vec![n1, n2, n3];
                        nodes.sort();
                        found_sets.insert(nodes);
                    }
                }
            }
        }
    }

    let mut max_clique = HashSet::new();
    bron_kerbosch_1(
        &graph,
        HashSet::new(),
        graph.node_indices().into_iter().collect(),
        HashSet::new(),
        &mut max_clique
    );

    // get the weight of the max clique, sort it and display it
    let mut max_clique_weight: Vec<_> = max_clique.iter().map(|node|graph.node_weight(*node).unwrap()).collect();
    max_clique_weight.sort();
    println!("{}", max_clique_weight.iter().join(","));

    // Display the graph
    // println!("{:?}", Dot::with_config(&graph, &[EdgeNoLabel]));

    found_sets.len()
}

pub fn solve_part_two(_connection_map: &ConnectionMap) -> u32 {
    0
}
