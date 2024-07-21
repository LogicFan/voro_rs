//! For store edge information in the Voronoi cells.

use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

/// a structure to store the edge information
/// for voronoi cells.
#[derive(Clone, Debug, Default)]
pub(super) struct Edges {
    /// The actual data for edges.
    ///
    /// `edges[p]` is the edge information for all the
    /// vertices of order `p`, with each vertex holding
    /// `2*p+1` integers.
    ///
    /// The first `p` elements hold the neighboring edges, and
    /// remaining hold a table of relations which is redundant but
    /// helps speed up the computation.
    pub edges: HashMap<usize, Vec<usize>>,

    /// The mapping from vertices to edges.
    ///
    /// `v2e[i]` store the mapping for the i-th vertex.
    /// The first element is the order, the second element
    /// is the index of the edge.
    pub v2e: Vec<(usize, usize)>,
}

impl Edges {
    /// Get the number of edges connected to this vertex.
    pub fn vertex_order(&self, v: usize) -> usize {
        self.v2e[v].0
    }

    /// shorthand for [Self::vertex_order].
    pub fn o(&self, v: usize) -> usize {
        self.vertex_order(v)
    }

    /// The number of vertex.
    pub fn len(&self) -> usize {
        self.v2e.len()
    }

    /// Ensure invariant, will panic if fails. Should only
    /// be used in unit test.
    #[allow(dead_code)]
    pub fn assert(&self) {
        // ensure size agrees
        let mut count = 0;
        for (m, e) in &self.edges {
            assert_eq!(e.len() % (2 * m + 1), 0);
            count += e.len() / (2 * m + 1);
        }
        assert_eq!(self.len(), count);
        assert_eq!(self.len(), self.v2e.len());

        // ensure consistency for redundant data.
        for i in 0..self.len() {
            for j in 0..self.o(i) {
                assert_eq!(
                    self[self[i][j]]
                        [self[i][self.o(i) + j]],
                    i
                );
            }
        }

        // ensure no duplication.
        for i in 0..self.len() {
            for j in 0..self.o(i) {
                for k in 0..j {
                    assert_ne!(self[i][j], self[i][k]);
                }
            }
        }
    }
}

impl Index<usize> for Edges {
    type Output = [usize];

    fn index(&self, index: usize) -> &Self::Output {
        let m = self.o(index);
        let start = (2 * m + 1) * index;
        let end = (2 * m + 1) * (index + 1);
        &self.edges[&m].as_slice()[start..end]
    }
}

impl IndexMut<usize> for Edges {
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output {
        let m = self.o(index);
        let start = (2 * m + 1) * index;
        let end = (2 * m + 1) * (index + 1);
        &mut self.edges.get_mut(&m).unwrap().as_mut_slice()
            [start..end]
    }
}
