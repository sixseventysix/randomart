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
    pub x_only_subtree_depths: Vec<usize>,
    pub y_only_subtree_depths: Vec<usize>,
}

impl TreeStats {
    pub fn from_node(node: &Node) -> Self {
        fn helper(node: &Node, depth: usize, stats: &mut TreeStats) -> Dependency {
            use crate::node::Node::*;

            stats.total_nodes += 1;
            stats.max_depth = stats.max_depth.max(depth);
            let mut child_deps = vec![];

            match node {
                X => {
                    stats.leaf_nodes += 1;
                    stats.leaf_depths.push(depth);
                    return Dependency::X;
                }
                Y => {
                    stats.leaf_nodes += 1;
                    stats.leaf_depths.push(depth);
                    return Dependency::Y;
                }
                Number(_) => {
                    stats.leaf_nodes += 1;
                    stats.leaf_depths.push(depth);
                    return Dependency::NO;
                }

                Add(a, b) => {
                    *stats.op_counts.entry("Add".into()).or_default() += 1;
                    stats.total_ops += 1;
                    child_deps.push(helper(a, depth + 1, stats));
                    child_deps.push(helper(b, depth + 1, stats));
                }
                Mult(a, b) => {
                    *stats.op_counts.entry("Mult".into()).or_default() += 1;
                    stats.total_ops += 1;
                    child_deps.push(helper(a, depth + 1, stats));
                    child_deps.push(helper(b, depth + 1, stats));
                }
                Div(a, b) => {
                    *stats.op_counts.entry("Div".into()).or_default() += 1;
                    stats.total_ops += 1;
                    child_deps.push(helper(a, depth + 1, stats));
                    child_deps.push(helper(b, depth + 1, stats));
                }
                Modulo(a, b) => {
                    *stats.op_counts.entry("Modulo".into()).or_default() += 1;
                    stats.total_ops += 1;
                    child_deps.push(helper(a, depth + 1, stats));
                    child_deps.push(helper(b, depth + 1, stats));
                }

                Sin(a) => {
                    *stats.op_counts.entry("Sin".into()).or_default() += 1;
                    stats.total_ops += 1;
                    child_deps.push(helper(a, depth + 1, stats));
                }
                Cos(a) => {
                    *stats.op_counts.entry("Cos".into()).or_default() += 1;
                    stats.total_ops += 1;
                    child_deps.push(helper(a, depth + 1, stats));
                }
                Exp(a) => {
                    *stats.op_counts.entry("Exp".into()).or_default() += 1;
                    stats.total_ops += 1;
                    child_deps.push(helper(a, depth + 1, stats));
                }
                Sqrt(a) => {
                    *stats.op_counts.entry("Sqrt".into()).or_default() += 1;
                    stats.total_ops += 1;
                    child_deps.push(helper(a, depth + 1, stats));
                }

                Mix(a, b, c, d) => {
                    *stats.op_counts.entry("Mix".into()).or_default() += 1;
                    stats.total_ops += 1;
                    child_deps.push(helper(a, depth + 1, stats));
                    child_deps.push(helper(b, depth + 1, stats));
                    child_deps.push(helper(c, depth + 1, stats));
                    child_deps.push(helper(d, depth + 1, stats));
                }
                MixUnbounded(a, b, c, d) => {
                    *stats.op_counts.entry("MixUnbounded".into()).or_default() += 1;
                    stats.total_ops += 1;
                    child_deps.push(helper(a, depth + 1, stats));
                    child_deps.push(helper(b, depth + 1, stats));
                    child_deps.push(helper(c, depth + 1, stats));
                    child_deps.push(helper(d, depth + 1, stats));
                }

                Rule(_) | Random | Triple(_, _, _) => {
                    unreachable!("Rule, Random, and Triple should not appear in simplified trees")
                }
            }

            let unified = unify_deps(&child_deps);
            match unified {
                Dependency::X => {
                    stats.x_only_subtrees += 1;
                    stats.x_only_subtree_depths.push(depth);
                }
                Dependency::Y => {
                    stats.y_only_subtrees += 1;
                    stats.y_only_subtree_depths.push(depth);
                }
                _ => {}
            }

            unified
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

    pub fn histogram(data: &[usize]) -> BTreeMap<usize, usize> {
        let mut map = BTreeMap::new();
        for &value in data {
            *map.entry(value).or_default() += 1;
        }
        map
    }

    fn leaf_depth_histogram(&self) -> BTreeMap<usize, usize> {
        Self::histogram(&self.leaf_depths)
    }

    fn x_only_subtree_depth_histogram(&self) -> BTreeMap<usize, usize> {
        Self::histogram(&self.x_only_subtree_depths)
    }

    fn y_only_subtree_depth_histogram(&self) -> BTreeMap<usize, usize> {
        Self::histogram(&self.y_only_subtree_depths)
    }

    fn print_all_histograms(&self) {
        println!("Leaf Depth Histogram: {:?}", self.leaf_depth_histogram());
        println!("X-only Subtree Depth Histogram: {:?}", self.x_only_subtree_depth_histogram());
        println!("Y-only Subtree Depth Histogram: {:?}", self.y_only_subtree_depth_histogram());
    }

    fn summary(&self) {
        println!("Total Nodes: {}", self.total_nodes);
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