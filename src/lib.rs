use log::info;
use std::collections::HashMap;
use std::fmt;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3_log::{Caching, Logger};

use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::Graph;

#[derive(Debug)]
enum NodeType {
    Branch = 0,
    Leaf = 1,
}

struct Node {
    index: i32,
    ntype: NodeType,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}_{})", self.ntype, self.index)
    }
}

#[pyclass]
struct Tree {
    tree: Graph<Node, ()>,
    tree_branch_index: HashMap<i32, NodeIndex>,
}

#[pymethods]
impl Tree {
    #[new]
    fn new() -> Self {
        Tree {
            tree: Graph::<Node, _>::new(),
            tree_branch_index: HashMap::new(),
        }
    }

    fn add_branch(&mut self, index: i32, parent_branch: Option<i32>) -> PyResult<()> {
        // Add branch to the tree
        let node_index = self.tree.add_node(Node {
            index: index,
            ntype: NodeType::Branch,
        });

        // Add itself to the mapping
        self.tree_branch_index.insert(index, node_index);

        // If parent branch is given, add edge
        if let Some(x) = parent_branch {
            match self.tree_branch_index.get(&x) {
                Some(parent_index) => {
                    self.tree.add_edge(*parent_index, node_index, ());
                    Ok(())
                }
                None => Err(PyValueError::new_err(format!("Branch_{x} doesn't exist!"))),
            }
        } else {
            Ok(())
        }
    }

    fn add_leaf(&mut self, index: i32, parent_branch: i32) -> PyResult<()> {
        // Add leaf to the tree
        let node_index = self.tree.add_node(Node {
            index: index,
            ntype: NodeType::Leaf,
        });

        // Add edge from leaf to tree
        match self.tree_branch_index.get(&parent_branch) {
            Some(parent_index) => {
                self.tree.add_edge(*parent_index, node_index, ());
                Ok(())
            }
            None => Err(PyValueError::new_err(format!(
                "Branch_{parent_branch} doesn't exist!"
            ))),
        }
    }

    fn __repr__(&self) -> String {
        self.to_string()
    }

    #[getter(num_nodes)]
    fn num_nodes(&self) -> usize {
        self.tree.node_count()
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            Dot::with_config(&self.tree, &[Config::EdgeIndexLabel])
        )
    }
}

#[pyfunction]
fn error_function() -> PyResult<PyErr> {
    Err(PyValueError::new_err("This always breaks!"))
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_demo(py: Python, m: &PyModule) -> PyResult<()> {
    let _ = Logger::new(py, Caching::LoggersAndLevels)?.install();

    m.add_class::<Tree>()?;

    m.add_function(wrap_pyfunction!(error_function, m)?)?;
    Ok(())
}
