mod directed_graph;
pub mod processing;
mod undirected_graph;

pub use directed_graph::{DirectedGraph, EdgeWeightedDigraph, FlowEdge, FlowNetwork};
pub use undirected_graph::UndirectedGraph;

/// This trait gives some basic information on vertices
pub trait VertexInfo {
    // fn vertex_edges(&self, v: &usize) -> &HashSet<usize>;
    fn vertex_edges(&self, v: &usize) -> Vec<&usize>;
    fn nb_vertices(&self) -> usize;
}

// Greatly inspired by :
// https://github.com/s1ck/graph/blob/main/crates/builder/src/index.rs
pub trait Weight:
    Copy
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + Ord
    + Eq
    + std::hash::Hash
    + PartialEq
    + std::fmt::Display
{
    fn zero() -> Self;
    fn max() -> Self;
}

macro_rules! impl_weight {
    ($TYPE:ty) => {
        impl Weight for $TYPE {
            fn max() -> Self {
                <$TYPE>::MAX
            }

            fn zero() -> Self {
                0 as $TYPE
            }
        }
    };
}

impl_weight!(u8);
impl_weight!(u16);
impl_weight!(u32);
impl_weight!(u64);
impl_weight!(usize);

impl_weight!(i8);
impl_weight!(i16);
impl_weight!(i32);
impl_weight!(i64);
impl_weight!(isize);
