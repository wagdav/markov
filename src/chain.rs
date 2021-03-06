use rand::seq::IteratorRandom;
use std::collections::{HashMap, HashSet, VecDeque};

type Item = Option<String>;
type Prefix = VecDeque<Item>;
type Suffix = HashSet<Item>;

#[derive(Debug)]
pub struct Chain {
    state_tab: HashMap<Prefix, Suffix>,
    n_prefix: usize,
}

#[derive(Debug)]
pub struct ChainBuilder {
    state_tab: HashMap<Prefix, Suffix>,
    current: VecDeque<Item>,
    n_prefix: usize,
}

impl ChainBuilder {
    pub fn new(n_prefix: usize) -> Self {
        Self {
            n_prefix,
            state_tab: HashMap::new(),
            current: vec![None; n_prefix].into_iter().collect(),
        }
    }
    pub fn add(&mut self, word: &str) {
        self.add_item(Some(word.to_owned()))
    }

    pub fn build(mut self) -> Chain {
        self.add_item(None);
        Chain {
            n_prefix: self.n_prefix,
            state_tab: self.state_tab,
        }
    }

    fn add_item(&mut self, item: Item) {
        if self.current.len() == self.n_prefix {
            self.state_tab
                .entry(self.current.to_owned())
                .or_insert_with(HashSet::new)
                .insert(item.to_owned());
            self.current.pop_front();
        }
        self.current.push_back(item);
    }
}

impl Chain {
    /// Generate at most `n_word` random words.
    pub fn generate(&self, n_words: usize) -> Vec<String> {
        let mut result = vec![];
        let mut cur = vec![None; self.n_prefix].into_iter().collect();
        for _ in 0..n_words {
            let next = self.state_tab[&cur]
                .iter()
                .choose(&mut rand::thread_rng())
                .unwrap();

            if next.is_none() {
                break;
            }

            let word = next.as_ref().unwrap();

            result.push(word.to_string());

            // Advance chain
            cur.pop_front();
            cur.push_back(Some(word.to_string()));
        }

        result
    }
}
