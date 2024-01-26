use std::path::PathBuf;

use crate::TreeCollectionWithLCA;
use ogcat::ogtree::TreeCollection;
use pyo3::prelude::*;

#[pyclass]
pub struct RMQ {
    inner: crate::rmq::RMQ,
}

#[pymethods]
impl RMQ {
    #[new]
    pub fn new(values: Vec<u32>) -> Self {
        Self {
            inner: crate::rmq::RMQ::new(values),
        }
    }

    pub fn __len__(&self) -> usize {
        self.inner.values.len()
    }

    pub fn argmin(&self, l: u32, r: u32) -> usize {
        let (l, r) = (std::cmp::min(l, r), std::cmp::max(l, r));
        self.inner.argmin(l, r)
    }

    pub fn values(&self) -> Vec<u32> {
        self.inner.values.clone()
    }

    pub fn min(&self, l: u32, r: u32) -> u32 {
        let (l, r) = (std::cmp::min(l, r), std::cmp::max(l, r));
        self.inner.min(l, r)
    }
}

#[pyclass]
pub struct TreeSet {
    data: TreeCollectionWithLCA,
}

#[pymethods]
impl TreeSet {
    #[new]
    pub fn new(path: PathBuf) -> PyResult<Self> {
        let tc = TreeCollection::from_newick(path).expect("Failed to load tree collection");
        let wrapped = TreeCollectionWithLCA::from_tree_collection(tc);
        Ok(Self { data: wrapped })
    }

    pub fn __len__(&self) -> usize {
        self.data.collection.trees.len()
    }

    pub fn tally_single_quintet(&self, names: (&str, &str, &str, &str, &str)) -> Vec<usize> {
        let mut res = vec![0usize; 15];
        let transl = self.data.translate_taxon_names(names);
        for (i, lca) in self.data.lca.iter().enumerate() {
            let quintet = [
                lca.rev[transl.0],
                lca.rev[transl.1],
                lca.rev[transl.2],
                lca.rev[transl.3],
                lca.rev[transl.4],
            ];
            if quintet.iter().any(|it| *it == 0) {
                continue;
            }
            // println!("> {:?}", i);
            if let Some(t) = lca.retrieve_topology(&quintet) {
                res[t as usize] += 1;
            }
        }
        res
    }
}
