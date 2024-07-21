//! Voronoi Cell

use glam::{DVec3, DVec4};
use std::collections::HashMap;

use crate::native::constant::*;

use super::Edges;

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

    edges: Edges,

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
    /// Create an empty cell
    ///
    /// - `max_len_sq`: the square of the maximum possible length for two cells
    /// in the container. This is important for tolerance calculation.
    pub(crate) fn new_empty(max_len_sq: f64) -> Self {
        Self {
            small_tolerance: max_len_sq * small_tolerance,
            large_tolerance: max_len_sq * large_tolerance,

            vertices: Vec::default(),
            edges: Edges::default(),
            mask: Vec::default(),

            up: 0,
            norm_vec: DVec4::default(),

            delete_stack: Vec::default(),
            delete_stack2: Vec::default(),
            search_stack: Vec::default(),
        }
    }

    /// Initializes a Voronoi cell as a rectangular box with the given dimensions.
    ///
    /// - `min`: the minimum coordinates.
    /// - `max`: the maximum coordinates.
    /// - `max_len_sq`: the square of the maximum possible length for two cells
    /// in the container. This is important for tolerance calculation.
    pub fn new_cuboid(
        mut min: DVec3,
        mut max: DVec3,
        max_len_sq: f64,
    ) -> Self {
        let mut cell = Self::new_empty(max_len_sq);
        min *= 2.0;
        max *= 2.0;

        cell.vertices.push(DVec3::new(min.x, min.y, min.z));
        cell.vertices.push(DVec3::new(max.x, min.y, min.z));
        cell.vertices.push(DVec3::new(min.x, max.y, min.z));
        cell.vertices.push(DVec3::new(max.x, max.y, min.z));
        cell.vertices.push(DVec3::new(min.x, min.y, max.z));
        cell.vertices.push(DVec3::new(max.x, min.y, max.z));
        cell.vertices.push(DVec3::new(min.x, max.y, max.z));
        cell.vertices.push(DVec3::new(max.x, max.y, max.z));

        let edges =
            cell.edges.edges.entry(3).or_insert(Vec::new());

        edges.extend([1, 4, 2, 2, 1, 0, 0].iter());
        edges.extend([3, 5, 0, 2, 1, 0, 1].iter());
        edges.extend([0, 6, 3, 2, 1, 0, 2].iter());
        edges.extend([2, 7, 1, 2, 1, 0, 3].iter());
        edges.extend([6, 0, 5, 2, 1, 0, 4].iter());
        edges.extend([4, 1, 7, 2, 1, 0, 5].iter());
        edges.extend([7, 2, 4, 2, 1, 0, 6].iter());
        edges.extend([5, 3, 6, 2, 1, 0, 7].iter());

        cell.edges.v2e.push((3, 0));
        cell.edges.v2e.push((3, 7));
        cell.edges.v2e.push((3, 14));
        cell.edges.v2e.push((3, 21));
        cell.edges.v2e.push((3, 28));
        cell.edges.v2e.push((3, 35));
        cell.edges.v2e.push((3, 42));
        cell.edges.v2e.push((3, 49));

        cell
    }

    pub fn new_octahedron(
        mut l: f64,
        max_len_sq: f64,
    ) -> Self {
        let mut cell = Self::new_empty(max_len_sq);
        l *= 2.0;

        cell.vertices.push(DVec3::new(-l, 0.0, 0.0));
        cell.vertices.push(DVec3::new(l, 0.0, 0.0));
        cell.vertices.push(DVec3::new(0.0, -l, 0.0));
        cell.vertices.push(DVec3::new(0.0, l, 0.0));
        cell.vertices.push(DVec3::new(0.0, 0.0, -l));
        cell.vertices.push(DVec3::new(0.0, 0.0, l));

        let edges =
            cell.edges.edges.entry(4).or_insert(Vec::new());

        edges.extend([2, 5, 3, 4, 0, 0, 0, 0, 0].iter());
        edges.extend([2, 4, 3, 5, 2, 2, 2, 2, 1].iter());
        edges.extend([0, 4, 1, 5, 0, 3, 0, 1, 2].iter());
        edges.extend([0, 5, 1, 4, 2, 3, 2, 1, 3].iter());
        edges.extend([0, 3, 1, 2, 3, 3, 1, 1, 4].iter());
        edges.extend([0, 2, 1, 3, 1, 3, 3, 1, 5].iter());

        cell.edges.v2e.push((4, 0));
        cell.edges.v2e.push((4, 9));
        cell.edges.v2e.push((4, 18));
        cell.edges.v2e.push((4, 27));
        cell.edges.v2e.push((4, 36));
        cell.edges.v2e.push((4, 45));

        cell
    }
}

impl CellSgl {
    pub fn translate(&mut self, mut translation: DVec3) {
        translation *= 2.0;

        for v in &mut self.vertices {
            *v += translation;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let cell = CellSgl::new_empty(1234.0);
        cell.edges.assert();

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
        let cell = CellSgl::new_cuboid(min, max, 3936.0);
        cell.edges.assert();

        assert_eq!(
            cell.small_tolerance,
            8.73967564984923229e-12
        );
        assert_eq!(
            cell.large_tolerance,
            1.74793512996984646e-10
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
            cell.edges[0],
            vec![1, 4, 2, 2, 1, 0, 0]
        );
        assert_eq!(
            cell.edges[1],
            vec![3, 5, 0, 2, 1, 0, 1]
        );
        assert_eq!(
            cell.edges[2],
            vec![0, 6, 3, 2, 1, 0, 2]
        );
        assert_eq!(
            cell.edges[3],
            vec![2, 7, 1, 2, 1, 0, 3]
        );
        assert_eq!(
            cell.edges[4],
            vec![6, 0, 5, 2, 1, 0, 4]
        );
        assert_eq!(
            cell.edges[5],
            vec![4, 1, 7, 2, 1, 0, 5]
        );
        assert_eq!(
            cell.edges[6],
            vec![7, 2, 4, 2, 1, 0, 6]
        );
        assert_eq!(
            cell.edges[7],
            vec![5, 3, 6, 2, 1, 0, 7]
        );

        assert_eq!(cell.edges.o(0), 3);
        assert_eq!(cell.edges.o(1), 3);
        assert_eq!(cell.edges.o(2), 3);
        assert_eq!(cell.edges.o(3), 3);
        assert_eq!(cell.edges.o(4), 3);
        assert_eq!(cell.edges.o(5), 3);
        assert_eq!(cell.edges.o(6), 3);
        assert_eq!(cell.edges.o(7), 3);

        assert_eq!(
            cell.edges.edges[&3],
            vec![
                1, 4, 2, 2, 1, 0, 0, 3, 5, 0, 2, 1, 0, 1,
                0, 6, 3, 2, 1, 0, 2, 2, 7, 1, 2, 1, 0, 3,
                6, 0, 5, 2, 1, 0, 4, 4, 1, 7, 2, 1, 0, 5,
                7, 2, 4, 2, 1, 0, 6, 5, 3, 6, 2, 1, 0, 7
            ]
        )
    }

    #[test]
    fn new_octahedron() {
        let l = 3.14;
        let cell = CellSgl::new_octahedron(l, 3134.0);
        cell.edges.assert();
    }
}
