use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

use super::float_cmp;

#[derive(Default)]
pub struct WeightedGraph<T: Hash + Eq + Clone + Debug> {
    weights: Vec<f64>,
    vertices: HashMap<T, usize>,
}

impl<T> WeightedGraph<T> where T: Hash + Eq + Clone + Debug {

    pub fn from_vertices(v: &[T]) -> Self {
        let n = v.len();
        debug_assert!(n != 0, "Empty vertice array is not allowed!");
        WeightedGraph {
            weights: vec!(0f64; n * n),
            vertices: v.to_vec().into_iter().zip(0..n).collect(),
        }
    }

    pub fn set_weight(&mut self, from: &T, to: &T, w: f64) {
        debug_assert!(w >= 0., "The weight cannot be negative!");
        let abs_idx = self.calc_abs_idx(from, to);
        self.weights[abs_idx] = w;
    }

    pub fn set_all_weights(&mut self, new_weights: Vec<f64>) {
        self.weights = new_weights;
    }

    pub fn get_weight(&self, from: &T, to: &T) -> f64 {
        // Indices are checked, safe
        unsafe {*self.weights.get_unchecked(self.calc_abs_idx(from, to))}
    }

    pub fn get_weights_for(&self, c: T) -> &[f64] {
        let idx = self.vertices[&c];
        let n = self.n_vertices();
        &self.weights[n * idx .. n * idx + n]
    }

    pub fn n_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn get_vertices(&self) -> Vec<&T> {
        let mut pairs: Vec<_> = self.vertices.iter().collect();
        pairs.sort_by(|(_, idx1), (_, idx2)| idx1.cmp(idx2) );
        let (keys, _): (Vec<&T>, Vec<&usize>) = pairs.iter().cloned().unzip();
        keys
    }

    pub fn get_all_weights(&self) -> &Vec<f64> {
        &self.weights
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
            let sum = row.iter().sum::<f64>();
            debug_assert!(float_cmp(sum, 1., 8) || float_cmp(sum, 0., 8));
        }
    }

    fn calc_abs_idx(&self, from: &T, to: &T) -> usize {
        let idx_from = self.vertices[from];
        let idx_to = self.vertices[to];
        
        let n = self.vertices.len();
        let abs_idx = n * idx_from + idx_to;
        debug_assert!(abs_idx < self.weights.len());
        abs_idx
    }
}

impl<T> std::fmt::Debug for WeightedGraph<T> 
where T: Hash + Eq + Clone + Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let chars = self.get_vertices();
        let n = chars.len();
        writeln!(f, "\n   {:?}", chars)?;
        for (from, c) in chars.iter().enumerate() {
            let slice = &self.weights[n * from .. n * from + n];
            writeln!(f, "{:?} {:?}", c, slice)?;
        }
        Ok(())
    }
}