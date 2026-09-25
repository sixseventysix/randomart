use crate::op::Op;
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, weighted::WeightedAliasIndex};

#[derive(Clone, Copy)]
pub enum Nonterminal {
    C,
    A,
}

#[derive(Clone, Copy)]
pub enum Symbol {
    Terminal(Op),
    Random,
    Nonterminal(Nonterminal),
}

use Nonterminal::{A, C};
use Symbol::{Nonterminal as N, Random, Terminal as T};

type Table = &'static [(f32, &'static [Symbol])];

const C_TABLE: Table = &[
    (3.0, &[T(Op::Sin), N(C)]),
    (3.0, &[T(Op::Cos), N(C)]),
    (1.0, &[T(Op::Exp), N(C)]),
    (1.0, &[T(Op::Sqrt), N(C)]),
    (1.0, &[T(Op::Add), N(C), N(C)]),
    (1.0, &[T(Op::Mult), N(C), N(C)]),
    (1.0, &[T(Op::Div), N(C), N(C)]),
    (1.0, &[T(Op::Mix), N(C), N(C), N(C), N(C)]),
    (1.0, &[N(A)]),
];

const A_TABLE: Table = &[
    (1.0, &[T(Op::X)]),
    (1.0, &[T(Op::Y)]),
    (1.0, &[Random]),
];

fn dice(table: Table) -> WeightedAliasIndex<f32> {
    WeightedAliasIndex::new(table.iter().map(|(weight, _)| *weight).collect()).unwrap()
}

pub struct Grammar {
    c_dice: WeightedAliasIndex<f32>,
    a_dice: WeightedAliasIndex<f32>,
    rng: ChaCha8Rng,
    todo: Vec<(Symbol, u32)>,
}

impl Grammar {
    pub fn new(seed: u64, depth: u32) -> Self {
        Self {
            c_dice: dice(C_TABLE),
            a_dice: dice(A_TABLE),
            rng: ChaCha8Rng::seed_from_u64(seed),
            todo: vec![(N(C), depth)],
        }
    }

    fn expand(&mut self, n: Nonterminal, depth: u32) {
        let n = if depth == 0 { A } else { n };
        let (table, dice) = match n {
            C => (C_TABLE, &self.c_dice),
            A => (A_TABLE, &self.a_dice),
        };
        let rhs = table[dice.sample(&mut self.rng)].1;
        for &symbol in rhs.iter().rev() {
            self.todo.push((symbol, depth.saturating_sub(1)));
        }
    }
}

impl Iterator for Grammar {
    type Item = Op;

    fn next(&mut self) -> Option<Op> {
        loop {
            let (symbol, depth) = self.todo.pop()?;
            match symbol {
                T(op) => return Some(op),
                Random => return Some(Op::Const(self.rng.random_range(-1.0..1.0))),
                N(n) => self.expand(n, depth),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate(seed: u64, depth: u32) -> Vec<Op> {
        Grammar::new(seed, depth).collect()
    }

    #[test]
    fn same_seed_same_tree() {
        for seed in 0..100 {
            assert_eq!(generate(seed, 8), generate(seed, 8));
        }
    }

    #[test]
    fn every_tree_is_one_complete_expression() {
        for seed in 0..100 {
            let mut open = 1;
            for op in generate(seed, 8) {
                assert!(open > 0);
                open = open - 1 + op.arity();
            }
            assert_eq!(open, 0);
        }
    }

    #[test]
    fn depth_zero_is_one_leaf() {
        for seed in 0..100 {
            let tree = generate(seed, 0);
            assert_eq!(tree.len(), 1);
            assert_eq!(tree[0].arity(), 0);
        }
    }
}
