use petgraph::dot::Dot;
use petgraph::graph::UnGraph;
use petgraph::prelude::*;
use std::fmt::Debug;
use std::fmt::Display;
use std::fs;
use std::io::Result;

#[derive(Clone, Debug)]
pub struct NetGraph<T> {
    pub graph: UnGraph<T, f32>,
}

impl<T> NetGraph<T> {
    pub fn new(graph: UnGraph<T, f32>) -> Self {
        Self { graph }
    }

    pub fn to_dot(&self) -> String
    where
        T: Display,
    {
        format!("{}", Dot::new(&self.graph))
    }

    pub fn save_dot(&self, path: &str) -> Result<()>
    where
        T: Display,
    {
        let dot = self.to_dot();
        fs::write(path, dot)
    }

    pub fn add_node(&mut self, node: T) -> NodeIndex {
        self.graph.add_node(node)
    }

    pub fn add_edge(&mut self, first: NodeIndex, second: NodeIndex, w: f32) -> EdgeIndex {
        self.graph.add_edge(first, second, w)
    }

    pub fn plot(&self)
    where
        T: Debug,
    {
        println!("Hello from plot!");
        println!("{:?}", self.graph);
    }
}
