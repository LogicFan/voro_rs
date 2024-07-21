//! Voronoi Cell

use glam::{DVec3, DVec4};
use std::collections::HashMap;

use crate::native::constant::*;

/// A class represent a Voronoi cell without neighbor information.
///
/// This class is an extension of the voronoicell_base class, in cases when
/// is not necessary to track the IDs of neighboring particles associated
/// with each face of the Voronoi cell.
#[derive(Debug)]
pub struct CellSgl {
    // tol = small_tolerance
    // tol_cu = small_tolerance^(3/2)
    small_tolerance: f64,
    // big_tol = large_tolerance
    large_tolerance: f64,

    // pts[i] = vertices[i]
    // current_vertices = vertices.capacity()
    // p = vertices.len()
    vertices: Vec<DVec3>,

    // ed[i][j] = edges[v2e[i].1][v2e[i][j].2 * (v2e[i].1 * 2 + 1)]
    // nu[i] = edges[i].capacity() * 2 + 1
    // current_vertices = edges.capacity()
    // p = vertices.len()
    // first value is order, second value is the index
    v2e: Vec<(usize, usize)>,

    // mep[i][j] = edges2[i][j]
    // mec[i] = edges2[i].len() * 2 + 1
    // mem[i] = edges2[i].capacity()
    // current_vertex_order = edges2.capacity()
    edges: HashMap<usize, Vec<usize>>,

    // mask[i] = mask[i]
    // current_vertices = mask.capacity()
    // maskc = mask.len()
    mask: Vec<u32>,

    // the index of a point in a cell for tracing routines.
    up: i32,

    // normal vector, the 4th one is the magnitude
    norm_vec: DVec4,

    // the first delete stack
    delete_stack: Vec<i32>,

    // the second delete stack
    delete_stack2: Vec<i32>,

    // the search stack
    search_stack: Vec<i32>,
}

impl CellSgl {
    fn nu(&self, i: usize) -> usize {
        self.v2e[i].0
    }

    fn ed(&self, i: usize, j: usize) -> usize {
        let m = self.nu(i);
        self.edges[&m][(2 * m + 1) * i + j]
    }
}

impl CellSgl {
    /// Create an empty cell
    ///
    /// - `max_len_sq`: the square of the maximum possible length for two cells
    /// in the container. This is important for tolerance calculation.
    pub(crate) fn new_empty(max_len_sq: f64) -> Self {
        Self {
            small_tolerance: max_len_sq * small_tolerance,
            large_tolerance: max_len_sq * large_tolerance,

            vertices: Vec::default(),
            v2e: Vec::default(),
            edges: HashMap::default(),
            mask: Vec::default(),

            up: 0,
            norm_vec: DVec4::default(),

            delete_stack: Vec::default(),
            delete_stack2: Vec::default(),
            search_stack: Vec::default(),
        }
    }

    pub fn new_cuboid(
        mut min: DVec3,
        mut max: DVec3,
        max_len_sq: f64,
    ) -> Self {
        let mut cell = Self::new_empty(max_len_sq);
        min *= 2.0;
        max *= 2.0;

        cell.up = 0;

        cell.vertices.push(DVec3::new(min.x, min.y, min.z));
        cell.vertices.push(DVec3::new(max.x, min.y, min.z));
        cell.vertices.push(DVec3::new(min.x, max.y, min.z));
        cell.vertices.push(DVec3::new(max.x, max.y, min.z));
        cell.vertices.push(DVec3::new(min.x, min.y, max.z));
        cell.vertices.push(DVec3::new(max.x, min.y, max.z));
        cell.vertices.push(DVec3::new(min.x, max.y, max.z));
        cell.vertices.push(DVec3::new(max.x, max.y, max.z));

        let edges2 =
            cell.edges.entry(3).or_insert(Vec::new());

        edges2.extend([1, 4, 2, 2, 1, 0, 0].iter());
        edges2.extend([3, 5, 0, 2, 1, 0, 1].iter());
        edges2.extend([0, 6, 3, 2, 1, 0, 2].iter());
        edges2.extend([2, 7, 1, 2, 1, 0, 3].iter());
        edges2.extend([6, 0, 5, 2, 1, 0, 4].iter());
        edges2.extend([4, 1, 7, 2, 1, 0, 5].iter());
        edges2.extend([7, 2, 4, 2, 1, 0, 6].iter());
        edges2.extend([5, 3, 6, 2, 1, 0, 7].iter());

        cell.v2e.push((3, 0));
        cell.v2e.push((3, 7));
        cell.v2e.push((3, 14));
        cell.v2e.push((3, 21));
        cell.v2e.push((3, 28));
        cell.v2e.push((3, 35));
        cell.v2e.push((3, 42));
        cell.v2e.push((3, 49));

        cell
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let cell = CellSgl::new_empty(1234.0);
        assert_eq!(
            cell.small_tolerance,
            2.74003042477488634e-12
        );
        assert_eq!(
            cell.large_tolerance,
            5.48006084954977268e-11
        );
    }

    #[test]
    fn new_cuboid() {
        let min = DVec3::new(1.57, 1.65, 1.31);
        let max = DVec3::new(1.86, 1.33, 2.54);
        let cell = CellSgl::new_cuboid(min, max, 1234.0);

        assert_eq!(
            cell.small_tolerance,
            2.74003042477488634e-12
        );
        assert_eq!(
            cell.large_tolerance,
            5.48006084954977268e-11
        );

        assert_eq!(
            cell.vertices[0],
            DVec3::new(3.14, 3.30, 2.62)
        );
        assert_eq!(
            cell.vertices[1],
            DVec3::new(3.72, 3.30, 2.62)
        );
        assert_eq!(
            cell.vertices[2],
            DVec3::new(3.14, 2.66, 2.62)
        );
        assert_eq!(
            cell.vertices[3],
            DVec3::new(3.72, 2.66, 2.62)
        );
        assert_eq!(
            cell.vertices[4],
            DVec3::new(3.14, 3.30, 5.08)
        );
        assert_eq!(
            cell.vertices[5],
            DVec3::new(3.72, 3.30, 5.08)
        );
        assert_eq!(
            cell.vertices[6],
            DVec3::new(3.14, 2.66, 5.08)
        );
        assert_eq!(
            cell.vertices[7],
            DVec3::new(3.72, 2.66, 5.08)
        );

        assert_eq!(
            (0..7)
                .into_iter()
                .map(|j| cell.ed(0, j))
                .collect::<Vec<_>>(),
            vec![1, 4, 2, 2, 1, 0, 0]
        );
        assert_eq!(
            (0..7)
                .into_iter()
                .map(|j| cell.ed(1, j))
                .collect::<Vec<_>>(),
            vec![3, 5, 0, 2, 1, 0, 1]
        );
        assert_eq!(
            (0..7)
                .into_iter()
                .map(|j| cell.ed(2, j))
                .collect::<Vec<_>>(),
            vec![0, 6, 3, 2, 1, 0, 2]
        );
        assert_eq!(
            (0..7)
                .into_iter()
                .map(|j| cell.ed(3, j))
                .collect::<Vec<_>>(),
            vec![2, 7, 1, 2, 1, 0, 3]
        );
        assert_eq!(
            (0..7)
                .into_iter()
                .map(|j| cell.ed(4, j))
                .collect::<Vec<_>>(),
            vec![6, 0, 5, 2, 1, 0, 4]
        );
        assert_eq!(
            (0..7)
                .into_iter()
                .map(|j| cell.ed(5, j))
                .collect::<Vec<_>>(),
            vec![4, 1, 7, 2, 1, 0, 5]
        );
        assert_eq!(
            (0..7)
                .into_iter()
                .map(|j| cell.ed(6, j))
                .collect::<Vec<_>>(),
            vec![7, 2, 4, 2, 1, 0, 6]
        );
        assert_eq!(
            (0..7)
                .into_iter()
                .map(|j| cell.ed(7, j))
                .collect::<Vec<_>>(),
            vec![5, 3, 6, 2, 1, 0, 7]
        );

        assert_eq!(cell.nu(0), 3);
        assert_eq!(cell.nu(3), 3);
        assert_eq!(cell.nu(5), 3);

        assert_eq!(
            cell.edges[&3],
            vec![
                1, 4, 2, 2, 1, 0, 0, 3, 5, 0, 2, 1, 0, 1,
                0, 6, 3, 2, 1, 0, 2, 2, 7, 1, 2, 1, 0, 3,
                6, 0, 5, 2, 1, 0, 4, 4, 1, 7, 2, 1, 0, 5,
                7, 2, 4, 2, 1, 0, 6, 5, 3, 6, 2, 1, 0, 7
            ]
        )
    }
}
