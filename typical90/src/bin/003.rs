use std::collections::VecDeque;

use proconio::{fastout, input};


#[derive(Debug)]
struct Node {
    edge: Vec<usize>,
}
#[derive(Debug)]
struct Graph {
    node: Vec<Node>,
}
impl Graph {
    fn new(n: usize) -> Graph {
        let nodes: Vec<Node> = (0..n).map(|_| Node { edge: vec![] }).collect();
        Graph { node: nodes }
    }
    fn add_edge(&mut self, a: usize, b: usize) {
        self.node[a].edge.push(b);
        self.node[b].edge.push(a);
    }
}

fn bfs(g: &Graph, start: usize) -> Vec<isize> {
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);
    let mut distance = vec![-1 as isize; g.node.len()];
    distance[start] = 0;
    while let Some(node_index) = queue.pop_front() {
        let node = &g.node[node_index];

        for &next_node in node.edge.iter() {
            if distance[next_node] != -1 {
                continue;
            } else {
                distance[next_node] = distance[node_index] + 1;
            }
            queue.push_back(next_node);
        }
    }
    distance
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [(usize, usize);n-1]
    }
    let mut g = Graph::new(n);
    a.iter().for_each(|&(x, y)| {
        g.add_edge(x - 1, y - 1);
    });
    let bfs_result = bfs(&g, 0);
    let max = bfs_result.iter().enumerate().max_by(|x, y| {
        x.1.cmp(y.1)
    }).unwrap();

    println!("{:?}", bfs(&g, max.0).iter().max().unwrap()+1);
}
