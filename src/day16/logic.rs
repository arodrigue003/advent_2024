use crate::day16::models::{Map, PreparedData, Tile};
use hashbrown::HashSet;
use itertools::Itertools;
use petgraph::algo::{all_simple_paths, astar};
// use petgraph::dot::Dot;
use petgraph::graph::{Graph};

pub fn prepare_data(map: &Map) -> PreparedData {
    let width = map.grid[0].len();
    let height = map.grid.len();

    // Create the graph
    let mut graph = Graph::new_undirected();

    // Create graph nodes from the grid for both horizontal and vertical nodes
    let hor_nodes: Vec<Vec<_>> = (0..height)
        .into_iter()
        .map(|line| {
            (0..width)
                .into_iter()
                .map(|column| graph.add_node((line, column, 'H')))
                .collect()
        })
        .collect();
    let ver_nodes: Vec<Vec<_>> = (0..height)
        .into_iter()
        .map(|line| {
            (0..width)
                .into_iter()
                .map(|column| graph.add_node((line, column, 'V')))
                .collect()
        })
        .collect();

    // Create graph edges if possible
    for line in 1..height - 1 {
        for column in 1..width - 1 {
            if map.grid[line][column] == Tile::Floor {
                // Create the turn
                graph.add_edge(ver_nodes[line][column], hor_nodes[line][column], 1000);

                // Two nodes following each others horizontally
                if map.grid[line][column + 1] == Tile::Floor {
                    graph.add_edge(hor_nodes[line][column], hor_nodes[line][column + 1], 1);
                }
                // Two nodes following each other vertically
                if map.grid[line + 1][column] == Tile::Floor {
                    graph.add_edge(ver_nodes[line][column], ver_nodes[line + 1][column], 1);
                }
            }
        }
    }

    // Display the graph
    // println!("{:?}", Dot::with_config(&graph, &[]));

    // Get starting en ending node
    let start = hor_nodes[map.start.0][map.start.1];
    let hor_end = hor_nodes[map.end.0][map.end.1];
    let ver_end = ver_nodes[map.end.0][map.end.1];

    PreparedData {
        graph,
        start,
        hor_end,
        ver_end,
    }
}

pub fn solve_part_one(prepared_data: &PreparedData) -> u32 {
    // Compute the shortest path
    let path = astar(
        &prepared_data.graph,
        prepared_data.start,
        |finish| finish == prepared_data.hor_end || finish == prepared_data.ver_end,
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap();

    path.0
}

pub fn solve_part_two(prepared_data: &PreparedData) -> usize {
    // Compute the shortest path
    let shortest_path = astar(
        &prepared_data.graph,
        prepared_data.start,
        |finish| finish == prepared_data.hor_end || finish == prepared_data.ver_end,
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap();

    // Determinate the length of the path
    let path_length = shortest_path.0 / 1000 + shortest_path.0 % 1000;
    let intermediate_node_length = path_length as usize - 1;

    println!("{path_length}");

    // Store best places
    let mut best_places = HashSet::new();

    // Determinate every path from start to hor_finish or ver_finish
    for path in all_simple_paths::<Vec<_>, _>(
        &prepared_data.graph,
        prepared_data.start,
        prepared_data.ver_end,
        intermediate_node_length,
        Some(intermediate_node_length),
    ) {
        // Check that we have a successful path
        let score: u32 = path.iter().tuple_windows().map(|(left, right)| {
            *prepared_data.graph.edge_weight(prepared_data.graph.find_edge(*left, *right).unwrap()).unwrap()
        }).sum();

        // Skip this iteration if the score is not right
        if score != shortest_path.0 {
            continue
        }

        for node in path {
            let weight = &prepared_data.graph[node];
            best_places.insert((weight.0, weight.1));
        }
    }
    for path in all_simple_paths::<Vec<_>, _>(
        &prepared_data.graph,
        prepared_data.start,
        prepared_data.hor_end,
        intermediate_node_length,
        Some(intermediate_node_length),
    ) {
        // Check that we have a successful path
        let score: u32 = path.iter().tuple_windows().map(|(left, right)| {
            *prepared_data.graph.edge_weight(prepared_data.graph.find_edge(*left, *right).unwrap()).unwrap()
        }).sum();

        // Skip this iteration if the score is not right
        if score != shortest_path.0 {
            continue
        }

        for node in path {
            let weight = &prepared_data.graph[node];
            best_places.insert((weight.0, weight.1));
        }
    }

    best_places.len()
}
