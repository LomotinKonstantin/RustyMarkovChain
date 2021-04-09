use std::iter::FromIterator;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};

use rand::{
    distributions::{WeightedIndex, Distribution},
    seq::IteratorRandom,
};

use crate::linalg::WeightedGraph;

const F64_SIZE: usize = std::mem::size_of::<f64>();
const U64_SIZE: usize = std::mem::size_of::<u64>();

pub struct TextMarkovChain {
    graph: WeightedGraph<char>,
}

impl TextMarkovChain {
    pub fn new(symbols: &[char]) -> Self {
        assert!(symbols.contains(&' '));
        TextMarkovChain {
            graph: WeightedGraph::from_vertices(symbols),
        }
    }

    pub fn load(path: &str) -> TextMarkovChain {
        let f = File::open(path).expect("No 'weights.bin' file found here. Run -fit command to train the markov chain");
        let mut reader = BufReader::new(f);
        // Reading number of bytes
        let mut size = [0u8; U64_SIZE];
        reader.read_exact(&mut size).unwrap();
        let size = (u64::from_le_bytes(size)) as usize;
        let mut str_buf = vec![0u8; size];
        // Reading vertices
        reader.read_exact(&mut str_buf).unwrap();
        let vertices: Vec<char> = String::from_utf8(str_buf).unwrap().chars().collect();
        let mut graph = WeightedGraph::from_vertices(&vertices);
        // Reading weigths
        let mut weight_buf = [0u8; F64_SIZE];
        let n_weights = vertices.len();
        let n_weights = n_weights * n_weights;
        let mut weights = Vec::with_capacity(n_weights);
        for _ in (0..n_weights * F64_SIZE).step_by(F64_SIZE) {
            reader.read_exact(&mut weight_buf).unwrap();
            let w = f64::from_le_bytes(weight_buf);
            weights.push(w);
        }
        graph.set_all_weights(weights);
        TextMarkovChain {
            graph
        }
    }   

    pub fn fit(&mut self, data: &[&str]) {
        for item in data {
            let mut chars = item.chars();
            let mut prev_char = chars.next().unwrap();
            for curr_char in chars {
                self.graph.incr(&prev_char, &curr_char);
                prev_char = curr_char;
            }
            self.graph.incr(&prev_char, &' ');
        }
        self.graph.normalize();
    }

    pub fn gen(&self, len: usize) -> String {
        let all_chars = self.graph.get_vertices();
        let initial_char = **TextMarkovChain::choice(&all_chars);
        let mut result = vec!(initial_char);
        let mut curr_char = initial_char;
        for _ in 1..len {
            let probas = self.graph.get_weights_for(curr_char);
            let next_char = **TextMarkovChain::choice_with_proba(&all_chars, probas);
            // Space stands for the end of the word
            if next_char == ' ' {
                break;
            }
            result.push(next_char);
            curr_char = next_char;
        }
        String::from_iter(result.iter())
    }

    pub fn dump(&self, path: &str) {
        let f = File::create(path).unwrap();
        let mut writer = BufWriter::new(f);
        
        // Saving vertices
        let vertices = self.graph.get_vertices();
        let vert_str: String = vertices.into_iter().collect();
        // Size
        let n_bytes = vert_str.len() as u64;
        writer.write_all(&n_bytes.to_le_bytes()).unwrap();
        // Values
        writer.write_all(&vert_str.into_bytes()).expect("Serialization failed");
        // Saving weigths
        let weights = self.graph.get_all_weights();
        for w in weights {
            writer.write_all(&w.to_le_bytes()).unwrap();
        }
        writer.flush().unwrap();
    }

    // Private

    fn choice<T>(options: &[T]) -> &T
    {
        let mut rng = rand::thread_rng();
        let idx = (0..options.len()).choose(&mut rng).unwrap();
        &options[idx]
    }

    fn choice_with_proba<'a, T>(options: &'a [T], probas: &[f64]) -> &'a T {
        let mut rng = rand::thread_rng();
        let dist = WeightedIndex::new(probas).unwrap();
        &options[dist.sample(&mut rng)]
    }
}