use petgraph::algo::kosaraju_scc;
use petgraph::visit::{IntoNodeIdentifiers, depth_first_search};
use petgraph::visit::{DfsEvent};
use proconio::{fastout, input};
use std::fs;
use std::collections::HashMap;
use std::io::Write;
use petgraph::{Graph, dot::Dot};
// #[fastout]
fn main() {
    input!{
        n: u32,
    };
    input!{
        s: [u32; n]
    };
    let mut graph = Graph::<u32, u32>::new();
    let mut map = HashMap::new();
    for i in 0..n {
        let from = *map.entry(i+1).or_insert_with(|| graph.add_node(i+1));
        let to = *map.entry(s[i as usize]).or_insert_with(|| graph.add_node(s[i as usize]));
        graph.add_edge(from, to, 100);
    }

    let scc = kosaraju_scc(&graph);

    let mut ec = 0;
    let mut cycles = Vec::new();
    for cycle in scc {
        let mut edge_count = 0;
        for node in &cycle {
            let edges = graph.edges(*node);
            edge_count += edges.count();
        }
        if &cycle.len() > &1 {
            cycles = cycle;
            ec = edge_count;
        }
        // println!("{}", &cycle.iter().map(|x| graph.node_weight(*x).unwrap().to_string()).collect::<Vec<String>>().join(" "));
    }
    // println!("{}", scc[&scc.len() - 1].len());


    // let mut f = fs::File::create("graph.dot").unwrap();
    // let dot_str = format!("{:?}", Dot::with_config(&graph, &[]));
    // f.write(dot_str.as_bytes()).unwrap();
    println!("{}", ec);
    println!("{}", cycles.iter().map(|x| graph.node_weight(*x).unwrap().to_string()).collect::<Vec<String>>().join(" "));
}
