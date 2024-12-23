use pyo3::prelude::*;
use std::thread::sleep;
use std::time::Duration;

#[pyclass]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

// #[pyclass]
pub struct Node {
    index: usize,
    label: String,
}

// #[pyclass]
pub struct Edge {
    source: usize,
    target: usize,
}

#[pymethods]
impl Graph {
    #[new]
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn add_node(&mut self, label: String) {
        self.nodes.push(Node {
            index: self.nodes.len(),
            label,
        });
    }

    fn add_edge(&mut self, source: usize, target: usize) {
        self.edges.push(Edge { source, target });
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn edge_count(&self) -> usize {
        self.edges.len()
    }

    fn get_node_label(&self, index: usize) -> String {
        if let Some(node) = self.nodes.get(index) {
            assert!(node.index == index);
            node.label.clone()
        } else {
            "ERROR: No node with such index".to_string()
        }
    }

    fn dfs(&self, py: Python, start_index: usize) -> PyResult<Vec<usize>> {
        let mut visited = vec![false; self.node_count()];
        let mut stack = vec![start_index];
        let mut result = Vec::new();

        while let Some(node_index) = stack.pop() {
            println!(
                "Visiting node {} with label {}",
                node_index,
                self.get_node_label(node_index)
            );

            if visited[node_index] {
                println!("Node {} already visited", node_index);
                continue;
            }

            visited[node_index] = true;
            result.push(node_index);

            // Simulate some work
            sleep(Duration::from_secs(2));

            if let Err(err) = py.check_signals() {
                println!("Signal received: {:?}", err);
                println!("Returning partial result");
                return Ok(result);
            }

            for edge in &self.edges {
                if edge.source == node_index {
                    stack.push(edge.target);
                }
            }
        }

        Ok(result)
    }
}

#[pymodule]
fn aeon_demo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Graph>()?;
    // m.add_class::<Node>()?;
    // m.add_class::<Edge>()?;
    Ok(())
}
