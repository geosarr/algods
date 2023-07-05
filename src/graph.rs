mod directed_graph;

/// This module collects some graph processing algorithms
// pub mod processing;
mod undirected_graph;

pub use directed_graph::DiGraph; //, EdgeWeightDiGraph, FlowEdge, FlowNetwork};
pub use undirected_graph::Graph;

/// This trait gives some basic information on vertices
pub trait VertexInfo<N>
where
    N: Index,
{
    fn vertex_edges(&self, vertex: &N) -> Vec<&N>;
    fn nb_vertices(&self) -> usize;
}
pub trait Zero {
    fn zero() -> Self;
}
pub trait Index:
    Zero
    + std::cmp::Ord
    + std::hash::Hash
    + std::convert::From<bool>
    + std::ops::Add<Output = Self>
    + Copy
    + std::ops::AddAssign
{
    fn to_usize(self) -> usize;
    fn maximum() -> Self;
    fn to_index(nb: usize) -> Self;
}
// Greatly inspired by :
// https://github.com/s1ck/graph/blob/main/crates/builder/src/index.rs
pub trait Weight:
    Copy
    + Index
    + Zero
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + Ord
    + Eq
    + std::hash::Hash
    + PartialEq
    + std::fmt::Display
{
}
macro_rules! impl_traits {
    ($TYPE:ty) => {
        impl Zero for $TYPE {
            fn zero() -> Self {
                0 as $TYPE
            }
        }
        impl Index for $TYPE {
            fn to_usize(self) -> usize {
                self as usize
            }
            fn maximum() -> Self {
                <$TYPE>::MAX
            }
            fn to_index(nb: usize) -> Self {
                nb as Self
            }
        }
        impl Weight for $TYPE {}
    };
}
impl_traits!(u8);
impl_traits!(u16);
impl_traits!(u32);
impl_traits!(u64);
impl_traits!(u128);
impl_traits!(usize);
// impl_traits!(i8);
// impl_traits!(i16);
// impl_traits!(i32);
// impl_traits!(i64);
// impl_traits!(i128);
// impl_traits!(isize);
