use proconio::{fastout, input};

#[derive(Debug)]
struct Node {
    edge: Vec<usize>,
    parent: Option<usize>,
}

#[derive(Debug)]
struct Graph {
    node: Vec<Node>,
}
impl Graph {
    fn new(n: usize) -> Graph {
        let nodes: Vec<Node> = (0..n).map(|_| Node { edge: vec![], parent: None }).collect();
        Graph { node: nodes }
    }
    fn add_edge(&mut self, a: usize, b: usize) {
        self.node[a].edge.push(b);
        self.node[b].parent = Some(a);
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut g = Graph::new(n);
    for (a, b) in ab {
        g.add_edge(a-1, b-1);
    }
    let mut parent_less = vec![];
    for (i, node) in g.node.iter().enumerate() {
        if node.parent.is_none() {
            parent_less.push(i);
        }
    }
    if parent_less.len() != 1 {
        println!("-1");
        return;
    }
    println!("{}", parent_less[0] + 1);
}
