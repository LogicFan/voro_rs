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
    pub edges: HashMap<usize, Vec<isize>>,

    /// The mapping from vertices to edges.
    ///
    /// `v2e[i]` store the mapping for the i-th vertex.
    /// The first element is the order, the second element
    /// is the index of the edge of the current vertex
    pub v2e: Vec<(usize, usize)>,
}

impl Edges {
    /// Get the number of edges connected to this vertex.
    pub fn vertex_order(&self, v: usize) -> usize {
        self.v2e[v].0
    }

    /// shorthand for [Self::vertex_order].
    pub fn len(&self, v: usize) -> usize {
        self.vertex_order(v)
    }

    /// Pick out the index
    /// of the next edge counterclockwise at the current vertex.
    ///
    /// - `a`: the index of an edge of the current vertex.
    /// - `v`: the index of vertex.
    pub fn ccw(&self, a: isize, v: isize) -> isize {
        if a == self.len(v as usize) as isize - 1 {
            0
        } else {
            a + 1
        }
    }

    /// Pick out the index
    /// of the next edge clockwise at the current vertex.
    ///
    /// - `a`: the index of an edge of the current vertex.
    /// - `v`: the index of vertex.
    pub fn cw(&self, a: isize, v: isize) -> isize {
        if a == 0 {
            self.len(v as usize) as isize - 1
        } else {
            a - 1
        }
    }

    pub fn reset(&mut self) {
        for i in 0..self.v2e.len() {
            for j in 0..self.len(i) {
                assert!(self[i][j] < 0, "edge reset routine found a previously untested edge");
                self[i][j] = -1 - self[i][j];
            }
        }
    }
}

impl Edges {
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
        assert_eq!(self.v2e.len(), count);

        // ensure consistency for redundant data.
        for i in 0..self.v2e.len() {
            for j in 0..self.len(i) {
                assert_eq!(self[self[i][j] as usize][self[i][self.len(i) + j] as usize], i as isize);
            }
        }

        // ensure no duplication.
        for i in 0..self.v2e.len() {
            for j in 0..self.len(i) {
                for k in 0..j {
                    assert_ne!(self[i][j], self[i][k]);
                }
            }
        }
    }
}

impl Index<usize> for Edges {
    type Output = [isize];

    fn index(&self, index: usize) -> &Self::Output {
        let m = self.len(index);
        let start = (2 * m + 1) * index;
        let end = (2 * m + 1) * (index + 1);
        &self.edges[&m].as_slice()[start..end]
    }
}

impl IndexMut<usize> for Edges {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let m = self.len(index);
        let start = (2 * m + 1) * index;
        let end = (2 * m + 1) * (index + 1);
        &mut self.edges.get_mut(&m).unwrap().as_mut_slice()[start..end]
    }
}
