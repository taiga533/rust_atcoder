use proconio::{fastout, input};

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        self.parent[x] = self.root(self.parent[x]);
        self.parent[x]
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);
        if parent == child {
            return false;
        }
        if self.size[parent] < self.size[child] {
            std::mem::swap(&mut parent, &mut child);
        }
        self.size[parent] += self.size[child];
        self.parent[child] = parent;

        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.size[root]
    }
}

#[fastout]
fn main() {
input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut query: Vec<Vec<usize>> = vec![];

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                mut q: [usize; 2],
            }
            q.insert(0, t);
            query.push(q);
            continue;
        } else {
            input! {
                mut q: [usize; 4],
            }
            q.insert(0, t);
            query.push(q);
            continue;
        }
    }

    let mut uf = UnionFind::new(h * w);
    let mut grid = vec![vec![false; w]; h];

    for q in query.iter() {
        if q[0] == 1 {
            let (_, x, y) = (q[0], q[1] - 1, q[2] - 1);
            grid[x][y] = true;
            if x > 0 && grid[x - 1][y] {
                uf.unite(x * w + y, (x - 1) * w + y);
            }
            if x < grid.len() - 1 && grid[x + 1][y] {
                uf.unite(x * w + y, (x + 1) * w + y);
            }
            if y > 0 && grid[x][y - 1] {
                uf.unite(x * w + y, x * w + y - 1);
            }
            if y < grid[x].len() - 1 && grid[x][y + 1] {
                uf.unite(x * w + y, x * w + y + 1);
            }
        } else {
            let (_, xa, ya, xb, yb) = (q[0], q[1] - 1, q[2] - 1, q[3] - 1, q[4] - 1);
            if grid[xa][ya] && grid[xb][yb] && uf.is_same(xa * w + ya, xb * w + yb) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
