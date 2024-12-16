use crate::day16::models::{Map, Tile};
use petgraph::algo::astar;
// use petgraph::dot::Dot;
use petgraph::graph::Graph;

pub fn solve_part_one(map: &Map) -> u32 {
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

    // Compute the shortest path
    let path = astar(
        &graph,
        hor_nodes[map.start.0][map.start.1],
        |finish| {
            let weight = &graph[finish];
            weight.0 == map.end.0 && weight.1 == map.end.1
        },
        |e| *e.weight(),
        |_| 0,
    ).unwrap();

    path.0
}

pub fn solve_part_two(map: &Map) -> u32 {
    0
}
