use crate::linalg::WeightedGraph;

pub struct TextMarkovChain {
    graph: WeightedGraph<char>,
}

impl TextMarkovChain {
    pub fn new(symbols: &[char]) -> Self {
        TextMarkovChain {
            graph: WeightedGraph::from_vertices(symbols),
        }
    }

    pub fn fit(&mut self, data: &[String]) {
        for item in data {
            let mut chars = item.chars();
            let mut prev_char = chars.next().unwrap();
            for curr_char in chars {
                self.graph.incr(&prev_char, &curr_char);
                prev_char = curr_char;
            }
        }
        self.graph.normalize();
    }
}