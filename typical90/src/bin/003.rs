use petgraph::{Directed, Graph, visit::Dfs};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [(usize, usize);n-1]
    }
    let graph = Graph::<usize, (), Directed, usize>::from_edges(a.iter().map(|(x, y)| {
        (x-1, y-1)
    }).collect::<Vec<(usize, usize)>>());

    let mut dfs = Dfs::new(&graph, graph.getNodeIndex(0).unwrap());
    while let Some(nx) = dfs.next(&graph) {
        println!("{}", nx + 1);
    }
}
