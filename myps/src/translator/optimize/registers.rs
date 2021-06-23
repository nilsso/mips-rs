use std::collections::HashMap;

use itertools::Itertools;
use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::Undirected;

type Graph<T> = StableGraph<T, (), Undirected, usize>;
type Stack = Vec<(usize, Vec<usize>)>;

// Try to do the interference graph simplification with a specific n.
fn try_n_simplify(n: usize, mut inter_graph: Graph<usize>) -> Option<Stack> {
    let mut stack = Stack::new();
    while inter_graph.node_count() > 0 {
        let id_opt = inter_graph
            .node_indices()
            .find(|id| inter_graph.neighbors(*id).count() < n);
        let id = {
            if id_opt.is_none() {
                return None;
            } else {
                id_opt.unwrap()
            }
        };
        let neighbors = inter_graph
            .neighbors(id)
            .map(|id| *inter_graph.node_weight(id).unwrap())
            .collect();
        let var = *inter_graph.node_weight(id).unwrap();
        inter_graph.remove_node(id);
        stack.push((var, neighbors));
    }
    Some(stack)
}

// Do the interference graph simplification.
fn simplify(inter_graph_init: Graph<usize>) -> (usize, Stack) {
    for n in 1.. {
        if let Some(stack) = try_n_simplify(n, inter_graph_init.clone()) {
            return (n, stack);
        }
    }
    unreachable!();
}

// Construct a var to register map which optimizes register re-use via graph coloring.
pub fn var_to_reg_optimizer_map(
    num_vars: usize,
    lifetimes: &Vec<(usize, usize)>,
    colors_fixed: &Vec<usize>,
) -> HashMap<usize, usize> {
    let num_fixed = colors_fixed.len();
    let num_free = lifetimes.len() - num_fixed;

    let mut inter_graph = Graph::<usize>::default();
    let vars = (0..)
        .filter(|i| !colors_fixed.contains(&i))
        .take(num_free)
        .collect::<Vec<_>>();
    let nodes = vars
        .iter()
        .map(|i| inter_graph.add_node(*i))
        .collect::<Vec<_>>();
    let mut num_edges = 0;
    for (i, j) in vars.iter().combinations(2).map(|v| (*v[0], *v[1])) {
        let (i_s, i_e) = lifetimes[i];
        let (j_s, j_e) = lifetimes[j];
        if i_s < j_e && j_s < i_e {
            inter_graph.add_edge(nodes[i - num_fixed], nodes[j - num_fixed], ());
            num_edges += 1;
        }
    }

    let (n, mut stack) = simplify(inter_graph);

    let colors = (0..)
        .filter(|n| !colors_fixed.contains(n))
        .take(n)
        .collect::<Vec<_>>();
    let mut color_graph = Graph::<(usize, usize)>::with_capacity(num_vars, num_edges);
    let mut lookup = HashMap::<usize, NodeIndex<usize>>::new();


    while let Some((from, edges_to)) = stack.pop() {
        // Get color
        let mut taken_colors = edges_to
            .iter()
            .map(|to| {
                let id = lookup[&to];
                let (_, color) = color_graph.node_weight(id).copied().unwrap();
                color
            })
            .collect::<Vec<_>>();
        taken_colors.sort();
        let color = *colors
            .iter()
            .filter(|c| !taken_colors.contains(c))
            .next()
            .unwrap();

        // Add node
        let id = color_graph.add_node((from, color));
        lookup.insert(from, id);

        for to in edges_to {
            let to = lookup[&to];
            color_graph.add_edge(id, to, ());
        }
    }

    // let m = n + colors_fixed.len();
    let map = color_graph
        .node_indices()
        // .map(|id| color_graph.node_weight(id))
        // .flatten()
        // .copied()
        .map(|id| {
            let (var, reg) = color_graph.node_weight(id).unwrap();
            // if num_fixed == 0 {
            //     (*var, n - reg - 1)
            // } else {
                (*var, *reg)
            // }
        })
        .chain(colors_fixed.iter().map(|&i| (i, i)))
        .collect();
    map
}
