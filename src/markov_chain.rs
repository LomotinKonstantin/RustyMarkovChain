use std::iter::FromIterator;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};
use std::collections::HashSet;

use rand::{
    distributions::{WeightedIndex, Distribution},
    seq::IteratorRandom,
};

use crate::linalg::WeightedGraph;

const F64_SIZE: usize = std::mem::size_of::<f64>();
const U64_SIZE: usize = std::mem::size_of::<u64>();
const TOP_PROBAS: f64 = 0.35;
const CANNOT_START_FROM: &str = "'- ";

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
        for _ in (0..n_weights * U64_SIZE).step_by(U64_SIZE) {
            reader.read_exact(&mut weight_buf).unwrap();
            let w = u64::from_le_bytes(weight_buf);
            weights.push(w);
        }
        graph.set_all_weights(weights);
        TextMarkovChain {
            graph
        }
    }   

    pub fn fit(data: &[&str]) -> Self {
        let mut char_set = TextMarkovChain::unique_chars(&data);
        if !char_set.contains(&' ') {
            char_set.insert(' ');
        }
        let mut unique_chars: Vec<char> = char_set.into_iter().collect();
        unique_chars.sort_unstable();
        let mut mc = TextMarkovChain::new(&unique_chars);
        for item in data {
            let mut chars = item.chars();
            let mut prev_char = chars.next().unwrap();
            for curr_char in chars {
                mc.graph.incr(&prev_char, &curr_char);
                prev_char = curr_char;
            }
            mc.graph.incr(&prev_char, &' ');
        }
        mc
    }

    pub fn gen(&self, min_len: usize) -> String {
        let all_chars = self.graph.get_vertices();
        let initial_char = loop {
            let c = **TextMarkovChain::choice(&all_chars);
            if !CANNOT_START_FROM.contains(c) { break c; }
        };
        let mut result = vec!(initial_char);
        let mut curr_char = initial_char;
        loop {
            let probas = self.graph.get_weights_for(curr_char);
            let is_dead_end = self.check_dead_end(probas, all_chars.as_slice());
            let next_char = loop {
                let c = **TextMarkovChain::choice_from_top(&all_chars, probas, TOP_PROBAS);
                if c == ' ' && !is_dead_end && result.len() < min_len { continue; }
                break c;
            };
            // Space stands for the end of the word
            if next_char == ' ' { break; }
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

    fn choice_with_proba<'a, T>(options: &'a [T], probas: &[u64]) -> &'a T {
        assert_eq!(options.len(), probas.len());
        let mut rng = rand::thread_rng();
        let dist = WeightedIndex::new(probas).unwrap();
        &options[dist.sample(&mut rng)]
    }

    fn choice_from_top<'a, T>(options: &'a [T], probas: &[u64], top_percent: f64) -> &'a T 
    where T: std::fmt::Debug
    {
        // Sorting
        let mut common: Vec<(&u64, &T)> = probas.iter().zip(options).collect();
        common.sort_by(|(p1, _), (p2, _)| p2.partial_cmp(p1).unwrap() );
        // Calculating absolute number to choose from
        let n = (options.len() as f64 * top_percent) as usize;
        let (new_probas, new_options): (Vec<_>, Vec<&T>) = common[0..n].iter().cloned().unzip();
        TextMarkovChain::choice_with_proba::<&T>(new_options.as_slice(), &new_probas)
    }

    fn check_dead_end(&self, probas: &[u64], all_chars: &[&char]) -> bool {
        let possible: Vec<_> = probas.iter().zip(all_chars).filter(|(proba, _)| **proba > 0).collect();
        possible.len() == 1 && **possible[0].1 == ' '
    }

    fn unique_chars(words: &[&str]) -> HashSet<char> {
        let mut cntr = HashSet::new();
        for w in words.iter() {
            cntr.extend(w.chars());
        };
        cntr
    }
}

#[cfg(test)]
mod markov_chain_test {

    use super::TextMarkovChain;

    #[test]
    fn fit_test() {
        let data = [
            "aa", "ar", "rc", "cr", "bo"
        ];
        let mc = TextMarkovChain::fit(&data);
        let expected = [
            0, 0, 0, 0, 0, 0, 
            1, 1, 0, 0, 0, 1, 
            0, 0, 0, 0, 1, 0, 
            1, 0, 0, 0, 0, 1, 
            1, 0, 0, 0, 0, 0, 
            2, 0, 0, 1, 0, 0
        ];
        assert_eq!(mc.graph.get_all_weights().as_slice(), expected);
    }

}