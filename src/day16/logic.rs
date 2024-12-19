// use colored::Colorize;
use hashbrown::HashSet;
use petgraph::graph::Graph;

// use petgraph::dot::Dot;
use crate::day16::astar::multi_astar;
use crate::day16::models::{Map, Tile};

pub fn prepare_data(map: &Map) -> (i32, HashSet<(usize, usize)>) {
    let width = map.grid[0].len();
    let height = map.grid.len();

    // Create the graph
    let mut graph = Graph::new_undirected();

    // Create graph nodes from the grid for both horizontal and vertical nodes
    let hor_nodes: Vec<Vec<_>> = (0..height)
        .map(|line| (0..width).map(|column| graph.add_node((line, column, 'H'))).collect())
        .collect();
    let ver_nodes: Vec<Vec<_>> = (0..height)
        .map(|line| (0..width).map(|column| graph.add_node((line, column, 'V'))).collect())
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

    // Compute the shortest path and every nodes in it
    let shortest_paths = multi_astar(
        &graph,
        start,
        |finish| finish == hor_end || finish == ver_end,
        |e| *e.weight(),
    )
    .unwrap();

    // Display the result
    // for (i, line) in map.grid.iter().enumerate() {
    //     for (j, block) in line.iter().enumerate() {
    //         if shortest_paths.1.contains(&hor_nodes[i][j]) || shortest_paths.1.contains(&ver_nodes[i][j]) {
    //             print!("{}", "O".green().bold());
    //         } else {
    //             match block {
    //                 Tile::Floor => print!("."),
    //                 Tile::Wall => print!("#")
    //             }
    //         }
    //     }
    //     println!();
    // }

    let mut paths = HashSet::new();
    for node in &shortest_paths.1 {
        let cost = &graph[*node];
        paths.insert((cost.0, cost.1));
    }

    (shortest_paths.0, paths)
}
