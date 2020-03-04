extern crate distances;

use std::time::Instant;

use distances::numeric::euclidean;

fn main() {
    let n = 100_000_000;
    let x: Vec<f64> = vec![0.2; n];
    let y: Vec<f64> = vec![0.3; n];
    let start = Instant::now();
    let prod = euclidean(&x, &y);
    println!("{}", start.elapsed().as_secs());
    println!("{}", prod);
}
