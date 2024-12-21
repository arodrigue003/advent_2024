use hashbrown::HashMap;
use itertools::Itertools;
use once_cell::sync::Lazy;
use petgraph::algo::all_simple_paths;
use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};
use std::iter::{IntoIterator, Iterator};
// use petgraph::dot::{Dot, Config::EdgeNoLabel};

static A: usize = 0;
static UP: usize = 1;
static LEFT: usize = 2;
static DOWN: usize = 3;
static RIGHT: usize = 4;

/// Represent every possible paths from two points the keypad + a pressure of the A button.
/// Staying at the same location is not represented here and the combined cost matrix must be
/// initialized with unit costs
static KEYPAD_PATHS: Lazy<Vec<((usize, usize), Vec<Vec<usize>>)>> = Lazy::new(|| {
    vec![
        // A <-> Others
        (
            (A, LEFT),
            vec![vec![A, DOWN, LEFT, LEFT, A], vec![A, LEFT, DOWN, LEFT, A]],
        ),
        (
            (LEFT, A),
            vec![vec![A, RIGHT, RIGHT, UP, A], vec![A, RIGHT, UP, RIGHT, A]],
        ),
        ((A, UP), vec![vec![A, LEFT, A], vec![A, DOWN, LEFT, UP, A]]),
        ((UP, A), vec![vec![A, RIGHT, A], vec![A, DOWN, RIGHT, UP, A]]),
        ((A, RIGHT), vec![vec![A, DOWN, A], vec![A, LEFT, DOWN, RIGHT, A]]),
        ((RIGHT, A), vec![vec![A, UP, A], vec![A, LEFT, UP, RIGHT, A]]),
        ((A, DOWN), vec![vec![A, DOWN, LEFT, A], vec![A, LEFT, DOWN, A]]),
        ((DOWN, A), vec![vec![A, UP, RIGHT, A], vec![A, RIGHT, UP, A]]),
        // LEFT <-> OTHERS
        (
            (LEFT, RIGHT),
            vec![vec![A, RIGHT, RIGHT, A], vec![A, RIGHT, UP, RIGHT, DOWN, A]],
        ),
        (
            (RIGHT, LEFT),
            vec![vec![A, LEFT, LEFT, A], vec![A, UP, LEFT, DOWN, LEFT, A]],
        ),
        (
            (LEFT, UP),
            vec![vec![A, RIGHT, UP, A], vec![A, RIGHT, RIGHT, UP, LEFT, A]],
        ),
        (
            (UP, LEFT),
            vec![vec![A, DOWN, LEFT, A], vec![A, RIGHT, DOWN, LEFT, LEFT, A]],
        ),
        ((LEFT, DOWN), vec![vec![A, RIGHT, A]]),
        ((DOWN, LEFT), vec![vec![A, LEFT, A]]),
        // UP <-> OTHERS
        ((UP, DOWN), vec![vec![A, DOWN, A], vec![A, RIGHT, DOWN, LEFT, A]]),
        ((DOWN, UP), vec![vec![A, UP, A], vec![A, RIGHT, UP, LEFT, A]]),
        ((UP, RIGHT), vec![vec![A, RIGHT, DOWN, A], vec![A, DOWN, RIGHT, A]]),
        ((RIGHT, UP), vec![vec![A, LEFT, UP, A], vec![A, UP, LEFT, A]]),
        // DOWN <-> OTHERS
        ((DOWN, RIGHT), vec![vec![A, RIGHT, A], vec![A, UP, RIGHT, DOWN, A]]),
        ((RIGHT, DOWN), vec![vec![A, LEFT, A], vec![A, UP, LEFT, DOWN, A]]),
    ]
});

static VERTICAL_MOVES_NUMPAD: Lazy<Vec<Vec<char>>> =
    Lazy::new(|| vec![vec!['7', '4', '1'], vec!['8', '5', '2', '0'], vec!['9', '6', '3', 'A']]);
static HORIZONTAL_MOVES_NUMPAD: Lazy<Vec<Vec<char>>> = Lazy::new(|| {
    vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['0', 'A'],
    ]
});

/// Allow mapping paths returned by petgraph all_simple_paths function to a sequence of button presses
static NUMPAD_PATH_MAPPING: Lazy<HashMap<(char, char), usize>> = Lazy::new(|| {
    let mut mapping = HashMap::new();

    // vertical moves
    for vertical_moves in VERTICAL_MOVES_NUMPAD.iter() {
        for (start, end) in vertical_moves.iter().tuple_windows() {
            mapping.insert((*start, *end), DOWN);
            mapping.insert((*end, *start), UP);
        }
    }

    // horizontal moves
    for horizontal_moves in HORIZONTAL_MOVES_NUMPAD.iter() {
        for (start, end) in horizontal_moves.iter().tuple_windows() {
            mapping.insert((*start, *end), RIGHT);
            mapping.insert((*end, *start), LEFT);
        }
    }

    mapping
});

/// Start from a matrix of cost and compos it to add a robot in the process
pub fn compose(costs: &[Vec<i32>]) -> Vec<Vec<i32>> {
    // Compose the matrix.
    // We init it with unit cost since X -> X cost is always 1 since every robot just have to press A.
    let mut combined_cost = vec![vec![1; 5]; 5];
    for ((start, end), possible_paths) in KEYPAD_PATHS.iter() {
        combined_cost[*start][*end] = possible_paths
            .iter()
            .map(|path| {
                path.iter()
                    .tuple_windows()
                    .map(|(start, end)| costs[*start][*end])
                    .sum()
            })
            .min()
            .unwrap();
    }

    combined_cost
}

pub fn solve_part_one(codes: &[String]) -> i32 {
    // Create a cost matrix from every point in the graph to every other points
    // This matrix indicate how many press are necessary to make the last robot make a movement
    // from the current position to the target position.

    // At the start every cost is 1 because the human can press every button whenever he wants
    let costs = vec![vec![1; 5]; 5];

    // Here we have every cost for the double robot setup
    let double_robot_cost = compose(&compose(&costs));

    // Build a graph corresponding to the numpad
    let mut graph = Graph::new_undirected();

    // Create the nodes
    let nodes: HashMap<_, _> = ['A', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        .into_iter()
        .map(|case| (case, graph.add_node(case)))
        .collect();
    // Create the reversed node hashmap
    let reversed_nodes: HashMap<_, _> = nodes.iter().map(|(value, node)| (*node, *value)).collect();

    // Create edges
    for vertical_moves in VERTICAL_MOVES_NUMPAD.iter() {
        for (start, end) in vertical_moves.iter().tuple_windows() {
            graph.add_edge(nodes[start], nodes[end], 1);
        }
    }
    for horizontal_moves in HORIZONTAL_MOVES_NUMPAD.iter() {
        for (start, end) in horizontal_moves.iter().tuple_windows() {
            graph.add_edge(nodes[start], nodes[end], 1);
        }
    }

    // For every digit in the target code, find the shortest cost associated with every path from
    // A to the digit value
    let mut total = 0;
    for code in codes {
        let score = compute_code_cost(code, &graph, &nodes, &reversed_nodes, &double_robot_cost);
        let numeric: i32 = code[0..3].parse().unwrap();
        total += score * numeric;
    }

    total
}

fn compute_code_cost(
    code: &String,
    graph: &Graph<char, i32, Undirected>,
    nodes: &HashMap<char, NodeIndex>,
    reversed_nodes: &HashMap<NodeIndex, char>,
    double_robot_cost: &[Vec<i32>],
) -> i32 {
    let mut score = 0;
    for (start, end) in std::iter::once('A').chain(code.chars()).tuple_windows() {
        let mut min_cost = i32::MAX;
        for simple_path in all_simple_paths::<Vec<_>, _>(&graph, nodes[&start], nodes[&end], 0, None) {
            // Build the associated sequence
            let sequence: Vec<_> = std::iter::once(A)
                .chain(
                    simple_path
                        .iter()
                        .tuple_windows()
                        .map(|(start, end)| NUMPAD_PATH_MAPPING[&(reversed_nodes[start], reversed_nodes[end])]),
                )
                .chain(std::iter::once(A))
                .collect();

            // Compute the cost
            let cost = get_cost(&double_robot_cost, &sequence);
            if cost < min_cost {
                min_cost = cost;
            }
        }
        score += min_cost;
    }
    score
}

fn get_cost(double_robot_cost: &[Vec<i32>], moves: &[usize]) -> i32 {
    moves
        .iter()
        .tuple_windows()
        .map(|(start, end)| double_robot_cost[*start][*end])
        .sum()
}

pub fn solve_part_two(codes: &[String]) -> u32 {
    0
}
