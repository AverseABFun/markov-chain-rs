use std::ops::{Index, IndexMut};

use crate::util::compare_const_strs;
use regex::Regex;

type MarkovNodeID = usize;

#[derive(Clone, Debug)]
pub struct Map<K: PartialEq + Clone, V: Clone> {
    keys: Vec<K>,
    values: Vec<V>,
    iterator_idx: usize,
}

pub fn map_create<K: PartialEq + Clone, V: Clone>() -> Map<K, V> {
    Map {
        keys: [].to_vec(),
        values: [].to_vec(),
        iterator_idx: 0,
    }
}

pub fn map_from<K: PartialEq + Clone, V: Clone>(from: &[(K, V)]) -> Map<K, V> {
    let mut out = map_create();
    for (key, value) in from {
        out.insert(key.clone(), value.clone());
    }
    out
}

impl<K: PartialEq + Clone, V: Clone> Map<K, V> {
    #[doc = "The insert function adds the key to the [Map] with the provided value."]
    pub fn insert(&mut self, key: K, value: V) {
        self.keys.push(key);
        self.values.push(value);
    }
    #[doc = "The set function changes the keys value to the provided value"]
    #[doc = "and returns if it found the key or not (true=key found, false=key"]
    #[doc = "not found)"]
    pub fn set(&mut self, key: K, value: V) -> bool {
        for i in 0..self.keys.len() {
            if self.keys[i] == key {
                self.values[i] = value;
                return true;
            }
        }
        false
    }
    #[doc = "The add function is different from the"]
    #[doc = "[Map::insert] and [Map::set] functions in that it"]
    #[doc = "automatically calls either one depending"]
    #[doc = "on if the key exists in the [Map]."]
    pub fn add(&mut self, key: K, value: V) {
        if !self.set(key.clone(), value.clone()) {
            self.insert(key, value);
        }
    }
    #[doc = "The get function simply returns the value in the [Map]"]
    #[doc = "if it found the key, and [None] if it didn't."]
    pub fn get(&self, key: K) -> Option<V> {
        for i in 0..self.keys.len() {
            if self.keys[i] == key {
                return Some(self.values[i].clone());
            }
        }
        None
    }
    fn get_or_panic(&self, key: K) -> &V {
        for i in 0..self.keys.len() {
            if self.keys[i] == key {
                return &self.values[i];
            }
        }
        panic!("cannot find key in [Map]")
    }
    fn get_idx(&self, key: K) -> Option<usize> {
        for i in 0..self.keys.len() {
            if self.keys[i] == key {
                return Some(i);
            }
        }
        None
    }
    #[doc = "The has function returns if it found the provided key in the [Map]."]
    pub fn has(&self, key: K) -> bool {
        for i in 0..self.keys.len() {
            if self.keys[i] == key {
                return true;
            }
        }
        false
    }
}

impl<K: PartialEq + Clone, V: Clone> Iterator for Map<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator_idx += 1;
        if self.keys.len() <= self.iterator_idx - 1 {
            return None;
        }
        Some((
            self.keys[self.iterator_idx - 1].clone(),
            self.values[self.iterator_idx - 1].clone(),
        ))
    }
}

impl<K: PartialEq + Clone, V: Clone> Index<K> for Map<K, V> {
    type Output = V;
    fn index(&self, index: K) -> &Self::Output {
        self.get_or_panic(index)
    }
}

impl<K: PartialEq + Clone, V: Clone> IndexMut<K> for Map<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        if !self.has(index.clone()) {
            panic!("cannot find key in [Map]")
        }
        let idx = self
            .get_idx(index)
            .expect("somehow there is no index found for the provided key");
        &mut self.values[idx]
    }
}

#[derive(Clone, Debug)]
pub struct MarkovNode {
    pub data: *const str,
    pub id: MarkovNodeID,
    pub links: Map<MarkovNodeID, u64>, // key is a MarkovNodeID, value is the number of "portions"
}

#[derive(Debug)]
pub struct MarkovChain {
    pub root: MarkovNode,
    pub nodes: Vec<MarkovNode>,                   // excluding the root
    pub nodes_map: Map<MarkovNodeID, MarkovNode>, // excluding the root
    all_portions: u64,
    newest_id: MarkovNodeID,
}

pub fn create_markov_chain() -> MarkovChain {
    MarkovChain {
        root: MarkovNode {
            data: "",
            id: 0,
            links: map_create(),
        },
        nodes: [].to_vec(),
        nodes_map: map_create(),
        all_portions: 0,
        newest_id: 0,
    }
}

impl MarkovChain {
    pub fn train_text(&mut self, text: String) {
        let mut text = text.clone();
        text = text.to_lowercase();
        let regex = Regex::new(r"(?m)[^\w\s]").unwrap();
        text = regex.replace_all(&text, "").to_string();
    }
    pub fn train_word(&mut self, from_word: *const str, to_word: *const str) {
        let mut i = 0;
        for val in self.nodes.clone() {
            if compare_const_strs(val.data, from_word) {
                for (val2, portions) in val.links.clone() {
                    if compare_const_strs(self.nodes_map[val2].data, to_word) {
                        self.nodes[i].links[val2] = portions + 1;
                        self.all_portions += 1;
                        return;
                    }
                }
                let mut i2 = 0;
                for val in self.nodes.clone() {
                    if compare_const_strs(val.data, to_word) {
                        self.nodes[i].links.add(i2, 1);
                        return;
                    }
                    i2 += 1;
                }
                self.newest_id += 1;
                let value = MarkovNode {
                    data: to_word,
                    id: self.newest_id,
                    links: map_create(),
                };
                self.nodes.push(value);
                self.nodes[i].links.add(self.newest_id, 1);
            }
            i += 1;
        }
    }
}
