use crate::node::Node;
use std::collections::BTreeMap;

#[derive(PartialEq)]
enum Dependency {
    X,
    Y,
    XY,
    NO
}

#[derive(Debug, Default)]
pub struct TreeStats {
    pub total_nodes: usize,
    pub leaf_nodes: usize,
    pub max_depth: usize,
    pub x_only_subtrees: usize,
    pub y_only_subtrees: usize,
    pub total_ops: usize,
    pub op_counts: std::collections::HashMap<&'static str, usize>,
    pub leaf_depths: Vec<usize>,
    pub x_only_subtree_op_counts: Vec<usize>,
    pub y_only_subtree_op_counts: Vec<usize>,
}

impl TreeStats {
    pub fn from_node(node: &Node) -> Self {
        fn helper(node: &Node, depth: usize, stats: &mut TreeStats) -> (Dependency, usize) {
            use crate::node::Node::*;

            stats.total_nodes += 1;
            stats.max_depth = stats.max_depth.max(depth);
            let mut child_deps = vec![];
            let mut child_op_count = 0;

            let _ = match node {
                X => {
                    stats.leaf_nodes += 1;
                    stats.leaf_depths.push(depth);
                    return (Dependency::X, 0);
                }
                Y => {
                    stats.leaf_nodes += 1;
                    stats.leaf_depths.push(depth);
                    return (Dependency::Y, 0);
                }
                Number(_) => {
                    stats.leaf_nodes += 1;
                    stats.leaf_depths.push(depth);
                    return (Dependency::NO, 0);
                }

                Add(a, b) => {
                    *stats.op_counts.entry("Add".into()).or_default() += 1;
                    stats.total_ops += 1;
                    let (d1, o1) = helper(a, depth + 1, stats);
                    let (d2, o2) = helper(b, depth + 1, stats);
                    child_deps.extend([d1, d2]);
                    child_op_count += o1 + o2;
                }
                Mult(a, b) => {
                    *stats.op_counts.entry("Mult".into()).or_default() += 1;
                    stats.total_ops += 1;
                    let (d1, o1) = helper(a, depth + 1, stats);
                    let (d2, o2) = helper(b, depth + 1, stats);
                    child_deps.extend([d1, d2]);
                    child_op_count += o1 + o2;
                }
                Div(a, b) => {
                    *stats.op_counts.entry("Div".into()).or_default() += 1;
                    stats.total_ops += 1;
                    let (d1, o1) = helper(a, depth + 1, stats);
                    let (d2, o2) = helper(b, depth + 1, stats);
                    child_deps.extend([d1, d2]);
                    child_op_count += o1 + o2;
                }
                Modulo(a, b) => {
                    *stats.op_counts.entry("Modulo".into()).or_default() += 1;
                    stats.total_ops += 1;
                    let (d1, o1) = helper(a, depth + 1, stats);
                    let (d2, o2) = helper(b, depth + 1, stats);
                    child_deps.extend([d1, d2]);
                    child_op_count += o1 + o2;
                }

                Sin(a) => {
                    *stats.op_counts.entry("Sin".into()).or_default() += 1;
                    stats.total_ops += 1;
                    let (d, o) = helper(a, depth + 1, stats);
                    child_deps.push(d);
                    child_op_count += o;
                }
                Cos(a) => {
                    *stats.op_counts.entry("Cos".into()).or_default() += 1;
                    stats.total_ops += 1;
                    let (d, o) = helper(a, depth + 1, stats);
                    child_deps.push(d);
                    child_op_count += o;
                }
                Exp(a) => {
                    *stats.op_counts.entry("Exp".into()).or_default() += 1;
                    stats.total_ops += 1;
                    let (d, o) = helper(a, depth + 1, stats);
                    child_deps.push(d);
                    child_op_count += o;
                }
                Sqrt(a) => {
                    *stats.op_counts.entry("Sqrt".into()).or_default() += 1;
                    stats.total_ops += 1;
                    let (d, o) = helper(a, depth + 1, stats);
                    child_deps.push(d);
                    child_op_count += o;
                }

                Mix(a, b, c, d) => {
                    *stats.op_counts.entry("Mix".into()).or_default() += 1;
                    stats.total_ops += 1;
                    let (d1, o1) = helper(a, depth + 1, stats);
                    let (d2, o2) = helper(b, depth + 1, stats);
                    let (d3, o3) = helper(c, depth + 1, stats);
                    let (d4, o4) = helper(d, depth + 1, stats);
                    child_deps.extend([d1, d2, d3, d4]);
                    child_op_count += o1 + o2 + o3 + o4;
                }
                MixUnbounded(a, b, c, d) => {
                    *stats.op_counts.entry("MixUnbounded".into()).or_default() += 1;
                    stats.total_ops += 1;
                    let (d1, o1) = helper(a, depth + 1, stats);
                    let (d2, o2) = helper(b, depth + 1, stats);
                    let (d3, o3) = helper(c, depth + 1, stats);
                    let (d4, o4) = helper(d, depth + 1, stats);
                    child_deps.extend([d1, d2, d3, d4]);
                    child_op_count += o1 + o2 + o3 + o4;
                }

                Rule(_) | Random | Triple(_, _, _) => unreachable!(),
            };

            let unified = unify_deps(&child_deps);

            match unified {
                Dependency::X => {
                    stats.x_only_subtrees += 1;
                    stats.x_only_subtree_op_counts.push(child_op_count + 1);
                }
                Dependency::Y => {
                    stats.y_only_subtrees += 1;
                    stats.y_only_subtree_op_counts.push(child_op_count + 1);
                }
                _ => {}
            }

            (unified, child_op_count + 1)
        }

        fn unify_deps(deps: &[Dependency]) -> Dependency {
            use Dependency::*;
            if deps.is_empty() {
                return NO;
            }
            if deps.iter().all(|d| *d == X || *d == NO) {
                X
            } else if deps.iter().all(|d| *d == Y || *d == NO) {
                Y
            } else if deps.iter().all(|d| *d == NO) {
                NO
            } else {
                XY
            }
        }

        let mut stats = TreeStats::default();
        helper(node, 0, &mut stats);
        stats
    }

    fn histogram(data: &[usize]) -> BTreeMap<usize, usize> {
        let mut map = BTreeMap::new();
        for &value in data {
            *map.entry(value).or_default() += 1;
        }
        map
    }

    fn leaf_depth_histogram(&self) -> BTreeMap<usize, usize> {
        Self::histogram(&self.leaf_depths)
    }

    fn x_only_subtree_op_counts_histogram(&self) -> BTreeMap<usize, usize> {
        Self::exclusive_histogram(&self.x_only_subtree_op_counts)
    }

    fn y_only_subtree_op_counts_histogram(&self) -> BTreeMap<usize, usize> {
        Self::exclusive_histogram(&self.y_only_subtree_op_counts)
    }

    fn exclusive_histogram(data: &[usize]) -> BTreeMap<usize, usize> {
        let mut cumulative: BTreeMap<usize, usize> = BTreeMap::new();

        for &val in data {
            *cumulative.entry(val).or_insert(0) += 1;
        }

        let mut exclusive = BTreeMap::new();
        let mut acc: usize = 0;

        for (&depth, &count) in cumulative.iter().rev() {
            let exclusive_count = count.saturating_sub(acc);
            exclusive.insert(depth, exclusive_count);
            acc += exclusive_count;
        }

        exclusive.retain(|_, &mut v| v != 0);
        exclusive
    }

    fn print_all_histograms(&self) {
        println!("Leaf Depth Histogram: {:?}", self.leaf_depth_histogram());
        println!("X-only Subtree Op Counts Histogram: {:?}", self.x_only_subtree_op_counts_histogram());
        println!("Y-only Subtree Op Counts Histogram: {:?}", self.y_only_subtree_op_counts_histogram());
    }

    fn summary(&self) {
        println!("Total Nodes: {}", self.total_nodes);
        println!("Size in memory: {} bytes", self.total_nodes * std::mem::size_of::<Node>());
        println!("Total Ops: {}", self.total_ops);
        println!("Op Counts: {:?}", self.op_counts);
        println!("Leaf Nodes: {}", self.leaf_nodes);
        println!("Max Depth: {}", self.max_depth);
        println!("X-only Subtrees: {}", self.x_only_subtrees);
        println!("Y-only Subtrees: {}", self.y_only_subtrees);
    }

    pub fn report(&self) {
        self.summary();
        self.print_all_histograms();
    }
}