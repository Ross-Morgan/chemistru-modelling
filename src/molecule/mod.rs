pub mod graph;
pub mod id;

use std::{num::NonZeroU8, collections::HashMap};

use chemistru_elements::element::Element;
use graph::UnGraph;

use self::id::{NodeID, EdgeID};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CovalentBond(NonZeroU8);

impl CovalentBond {
    pub const fn new(v: NonZeroU8) -> Self {
        Self(v)
    }

    pub fn new_primitive(v: u8) -> Self {
        Self(v.try_into().expect("Covalent bond cannot have order of zero"))
    }

    pub const fn order(self) -> u8 {
        self.0.get()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CovalentMolecule {
    inner: UnGraph<Element, CovalentBond>,
}

impl CovalentMolecule {
    pub fn new() -> Self {
        Self { inner: UnGraph::new() }
    }

    pub fn add_atom(&mut self, atom: Element) -> NodeID {
        self.inner.add_node(atom)
    }

    pub fn add_bond(&mut self, a: NodeID, b: NodeID, bond: CovalentBond) -> EdgeID {
        self.inner.add_edge(a, b, bond)
    }
}

impl CovalentMolecule {
    pub fn formula(&self) -> String {
        let mut buf = HashMap::<_, usize>::new();

        self.inner
            .nodes
            .values()
            .copied()
            .for_each(|e| {
                buf
                    .entry(e.symbol())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            });

        let mut v = buf
            .into_iter()
            .collect::<Vec<_>>();
        
        v.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        
        v
            .into_iter()
            .map(|(sym, count)| format!("{sym}{}", match count {
                1 => String::new(),
                c => to_subscript(c),
            }))
            .collect()
    }
}

fn to_subscript<T: ToString>(s: T) -> String {
    s
        .to_string()
        .chars()
        .map(|c| match c {
            '0' => '0',
            '1' => '₁',
            '2' => '₂',
            '3' => '₃',
            '4' => '₄',
            '5' => '₅',
            '6' => '₆',
            '7' => '₇',
            '8' => '₈',
            '9' => '₉',
            c => c,
        })
        .fold(String::new(), |mut acc, c| { acc.push(c); acc })
}

pub trait LonePairs {
    fn lone_pairs(&self) -> usize;
}
pub trait BondPairs {
    fn bond_pairs(&self) -> usize;
}