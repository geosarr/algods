use rayon::iter;
use rayon::prelude::*;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Array<T> {
    vec: Vec<T>,
}
impl<T> Array<T> {
    pub fn from_vec(vector: Vec<T>) -> Self {
        Self { vec: vector }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
        }
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn coordinate(&self, i: usize) -> &T {
        &self.vec[i]
    }
    pub fn get_mut_coordinate(&mut self, i: usize) -> &mut T {
        &mut self.vec[i]
    }
}
impl<T: Add<Output = T> + Copy + std::iter::Sum> Array<T> {
    pub fn sum(&self) -> T {
        self.vec.iter().map(|e| *e).sum()
    }
}
impl<
        'a,
        T: Mul<Output = T>
            + Copy
            + std::iter::Sum
            + std::marker::Send
            + std::marker::Sync
            + std::iter::Sum<<Vec<T> as rayon::iter::IntoParallelRefIterator<'a>>::Item>,
    > Array<T>
where
    Vec<T>: iter::IntoParallelRefIterator<'a>,
{
    pub fn into_par_dot(self, other: Self) -> T {
        self.vec
            .into_par_iter()
            .enumerate()
            .map(|(pos, value)| value * other.vec[pos])
            .sum()
    }
}

impl<'a, T: 'a + Mul<Output = T> + Copy + std::iter::Sum> Array<T> {
    pub fn dot<'b>(&'a self, other: &'b Self) -> T {
        self.vec
            .iter()
            .enumerate()
            .map(|(pos, value)| *value * other.vec[pos])
            .sum()
    }
    pub fn dot_ref<'b>(&'a self, other: Array<&'b T>) -> T {
        self.vec
            .iter()
            .enumerate()
            .map(|(pos, value)| *value * *(other.vec[pos]))
            .sum()
    }
}
impl<T: Add<Output = T> + Copy> Add for Array<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            vec: self
                .vec
                .iter()
                .enumerate()
                .map(|(pos, e)| *e + other.vec[pos])
                .collect(),
        }
    }
}

impl<'a, 'b, T: Copy + Add> Add<&'b Array<T>> for &'a Array<T>
where
    T: Copy + Add,
    Vec<T>: FromIterator<<T as Add>::Output>,
{
    type Output = Array<T>;

    fn add(self, other: &'b Array<T>) -> Self::Output {
        Array {
            vec: self
                .vec
                .iter()
                .enumerate()
                .map(|(pos, e)| *e + other.vec[pos])
                .collect(),
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub for Array<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            vec: self
                .vec
                .iter()
                .enumerate()
                .map(|(pos, e)| *e - other.vec[pos])
                .collect(),
        }
    }
}

impl<'a, 'b, T: Copy + Sub> Sub<&'b Array<T>> for &'a Array<T>
where
    T: Copy + Sub,
    Vec<T>: FromIterator<<T as Sub>::Output>,
{
    type Output = Array<T>;

    fn sub(self, other: &'b Array<T>) -> Self::Output {
        Array {
            vec: self
                .vec
                .iter()
                .enumerate()
                .map(|(pos, e)| *e - other.vec[pos])
                .collect(),
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul for Array<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            vec: self
                .vec
                .iter()
                .enumerate()
                .map(|(pos, e)| *e * other.vec[pos])
                .collect(),
        }
    }
}

impl<'a, 'b, T: Copy + Mul> Mul<&'b Array<T>> for &'a Array<T>
where
    T: Copy + Mul,
    Vec<T>: FromIterator<<T as Mul>::Output>,
{
    type Output = Array<T>;

    fn mul(self, other: &'b Array<T>) -> Self::Output {
        Array {
            vec: self
                .vec
                .iter()
                .enumerate()
                .map(|(pos, e)| *e * other.vec[pos])
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    vec: Vec<Array<T>>,
}

impl<T> Matrix<T> {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }
    pub fn with_capacity(n: usize, m: usize) -> Self {
        let mut _vec = Vec::with_capacity(n); // n rows
        for _ in 0..n {
            _vec.push(Array::with_capacity(m)); // m columns
        }
        Self { vec: _vec }
    }
    pub fn add_row(&mut self, row: Vec<T>) {
        if !self.vec.is_empty() {
            let row_len = row.len();
            let nb_col = self.vec[0].len();
            if row_len != nb_col {
                panic!("row length = {row_len} != {nb_col} = number of columns");
            }
        }
        self.vec.push(Array { vec: row });
    }
    pub fn coef(&self, i: usize, j: usize) -> &T {
        self.vec[i].coordinate(j)
    }
    pub fn row(&self, i: usize) -> &Array<T> {
        &self.vec[i]
    }
    pub fn col(&self, j: usize) -> Array<&T> {
        let vec: Vec<&T> = self.vec.iter().map(|arr| arr.coordinate(j)).collect();
        Array::from_vec(vec)
    }
    pub fn get_mut_coef(&mut self, i: usize, j: usize) -> &mut T {
        self.vec[i].get_mut_coordinate(j)
    }
}

impl<'a, T: 'a + Mul<Output = T> + Copy + Zero + std::iter::Sum> Matrix<T> {
    pub fn dot(self, other: Self) -> Self {
        let nb_row_lhs = self.vec.len();
        let nb_col_lhs = self.vec[0].len();
        let nb_row_rhs = other.vec.len();
        let nb_col_rhs = other.vec[0].len();
        if self.vec.is_empty() || other.vec.is_empty() {
            panic!("can not multiply empty matrices");
        } else if self.vec[0].len() != other.vec.len() {
            panic!("can not multiply matrices with shapes ({nb_row_lhs},{nb_col_lhs}) and ({nb_row_rhs},{nb_col_rhs})");
        } else {
            let mut mat = Self::zero(nb_row_lhs, nb_col_rhs);
            // mat.row(0).dot_ref(mat.col(1));
            for i in 0..nb_row_lhs {
                for j in 0..nb_col_rhs {
                    *mat.get_mut_coef(i, j) = self.row(i).dot_ref(other.col(j));
                }
            }
            mat
        }
    }
}

impl<T: Zero + Copy> Matrix<T> {
    pub fn zero(n: usize, m: usize) -> Self {
        let mut vec = Vec::with_capacity(n);
        let zero = T::zero();
        for _ in 0..n {
            let row_zeros = vec![zero; m];
            vec.push(Array::from_vec(row_zeros));
        }
        Self { vec: vec }
    }
}

impl<T: One + Copy> Matrix<T> {
    pub fn one(n: usize, m: usize) -> Self {
        let mut vec = Vec::with_capacity(n);
        let one = T::one();
        for _ in 0..n {
            let row_ones = vec![one; m];
            vec.push(Array::from_vec(row_ones));
        }
        Self { vec: vec }
    }
}

trait Zero {
    fn zero() -> Self;
}

trait One {
    fn one() -> Self;
}

macro_rules! impl_zero_one {
    ($TYPE:ty) => {
        impl Zero for $TYPE {
            fn zero() -> Self {
                0 as $TYPE
            }
        }
        impl One for $TYPE {
            fn one() -> Self {
                1 as $TYPE
            }
        }
    };
}

impl_zero_one!(u8);
impl_zero_one!(u16);
impl_zero_one!(u32);
impl_zero_one!(u64);
impl_zero_one!(usize);

impl_zero_one!(i8);
impl_zero_one!(i16);
impl_zero_one!(i32);
impl_zero_one!(i64);
impl_zero_one!(isize);

impl_zero_one!(f32);
impl_zero_one!(f64);

use std::fmt;

impl<T: std::fmt::Debug> fmt::Display for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.vec)
    }
}

impl<T: std::fmt::Debug> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.vec.len();
        for k in 0..len {
            write!(f, "{}\n", self.vec[k]);
        }
        Ok(())
    }
}
