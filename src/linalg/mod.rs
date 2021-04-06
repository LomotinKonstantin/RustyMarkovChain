mod wgraph;

pub use wgraph::WeightedGraph;

pub fn float_cmp(a: f64, b: f64, precision: usize) -> bool {
    let p = 10f64.powi(-(precision as i32));
    (a - b).abs() < p
}