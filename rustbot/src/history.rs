
use std::collections::VecDeque;
use std::fmt;

#[derive(Clone, Debug)]
pub struct History {
    data: VecDeque<String>,
    size: usize,
}

impl History {
    pub fn new(size: usize) -> Self {
        History {
            data: VecDeque::with_capacity(size),
            size,
        }
    }

    pub fn add(&mut self, element: String) {
        if self.data.len() == self.size {
            self.data.pop_front();
        }
        self.data.push_back(element);
    }

    pub fn string(&self) -> String {
        let mut s = String::new();
        for string in &self.data {
            s.push_str(string);