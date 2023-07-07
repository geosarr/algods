mod directed_graph;

/// This module collects some graph processing algorithms
pub mod processing;
mod undirected_graph;

pub use directed_graph::{DiGraph, EdgeWeightedDiGraph}; //, , FlowEdge, FlowNetwork};
pub use undirected_graph::Graph;

use std::cmp::Ord;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub};

/// This trait gives some basic information on vertices
pub trait VertexInfo<N>
where
    N: Index,
{
    fn vertex_edges(&self, vertex: &N) -> Vec<&N>;
    fn nb_vertices(&self) -> usize;
}
pub trait Max {
    fn maximum() -> Self;
}
pub trait Base: Ord + Hash + Sub<Output = Self> + Add<Output = Self> + Copy {}
pub trait Convert: std::convert::From<bool> + Copy + AddAssign {
    fn to_usize(self) -> usize;
    fn to_vertex(nb: usize) -> Self;
}
pub trait Index: Base + Convert + Max {}
// Greatly inspired by :
// https://github.com/s1ck/graph/blob/main/crates/builder/src/index.rs
pub trait Weight: Hash + Clone + Copy + Eq + PartialEq + std::fmt::Display {}
macro_rules! impl_index {
    ($TYPE:ty) => {
        impl Base for $TYPE {}
        impl Max for $TYPE {
            fn maximum() -> Self {
                <$TYPE>::MAX
            }
        }
        impl Convert for $TYPE {
            fn to_usize(self) -> usize {
                self as usize
            }
            fn to_vertex(nb: usize) -> Self {
                nb as Self
            }
        }
        impl Index for $TYPE {}
    };
}
macro_rules! impl_weight {
    ($TYPE:ty) => {
        impl Weight for $TYPE {}
    };
}
impl_index!(u8);
impl_index!(u16);
impl_index!(u32);
impl_index!(u64);
impl_index!(u128);
impl_index!(usize);
impl_weight!(u8);
impl_weight!(u16);
impl_weight!(u32);
impl_weight!(u64);
impl_weight!(u128);
impl_weight!(usize);
impl_weight!(i8);
impl_weight!(i16);
impl_weight!(i32);
impl_weight!(i64);
impl_weight!(i128);
impl_weight!(isize);
