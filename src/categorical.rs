use num::FromPrimitive;
use rayon::prelude::*;

pub trait Categorical: PartialEq + Sync + Send + FromPrimitive {}

impl Categorical for i128 {}
impl Categorical for i64 {}
impl Categorical for i32 {}
impl Categorical for i16 {}
impl Categorical for i8 {}

pub fn hamming<T: Categorical>(x: &[T], y: &[T]) -> T {
    FromPrimitive::from_usize(
        x.par_iter()
            .zip(y.par_iter())
            .filter(|(a, b)| a != b)
            .count()
    ).unwrap()
}
