extern crate time;
use time::PreciseTime;
extern crate distance;
use distance::cosine;

	
fn main() {
	let n = 100000000;
    let x: Vec<f64> = vec![0.2; n];
    let y: Vec<f64> = vec![0.3; n];
    let start = PreciseTime::now();
    let prod = cosine(&x,&y);
    let end = PreciseTime::now();
    println!("{}", start.to(end));
    println!("{}", prod);
    // assert!(approx_eq!(f64, prod, 6000000.0, ulps=2));
}