use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;

use super::float_cmp;

pub struct WeightedGraph<T: Hash + Eq + Clone> {
    weights: Vec<f64>,
    vertices: HashMap<T, usize>,
}

impl<T> WeightedGraph<T> where T: Hash + Eq + Clone {
    pub fn new() -> Self {
        WeightedGraph {
            weights: Vec::new(),
            vertices: HashMap::new(),
        }
    }

    pub fn from_vertices(v: &[T]) -> Self {
        let n = v.len();
        debug_assert!(n != 0, "Empty vertice array is not allowed!");
        WeightedGraph {
            weights: vec!(0f64; n * n),
            vertices: HashMap::from_iter(v.to_vec().into_iter().zip(0..n)),
        }
    }

    pub fn set_weight(&mut self, from: &T, to: &T, w: f64) {
        debug_assert!(w >= 0., "The weight cannot be negative!");
        let abs_idx = self.calc_abs_idx(from, to);
        self.weights[abs_idx] = w;
    }

    pub fn get_weight(&self, from: &T, to: &T) -> f64 {
        // Indices are checked, safe
        unsafe {*self.weights.get_unchecked(self.calc_abs_idx(from, to))}
    }

    pub fn n_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn get_vertices(&self) -> Vec<&T> {
        self.vertices.keys().collect()
    }

    pub fn incr(&mut self, from: &T, to: &T) {
        let idx = self.calc_abs_idx(from, to);
        self.weights[idx] += 1.;
    }

    pub fn normalize(&mut self) {
        let n = self.n_vertices();
        for row in self.weights.chunks_mut(n) {
            let mut sum: f64 = row.iter().sum();
            if sum == 0. {
                sum = 1.;
            }
            for el in row.iter_mut() {
                *el /= sum;
            }

            debug_assert!(float_cmp(row.iter().sum::<f64>(), 1., 8));
        }
    }

    fn calc_abs_idx(&self, from: &T, to: &T) -> usize {
        let idx_from = self.vertices.get(from).unwrap();
        let idx_to = self.vertices.get(to).unwrap();
        
        let n = self.vertices.len();
        let abs_idx = n * idx_from + idx_to;
        debug_assert!(abs_idx < self.weights.len());
        abs_idx
    }
}