//! Voronoi Cell

use crate::native::constant::*;
use glam::{DVec3, DVec4};

use super::{Edges, VoronoiCell};

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
    pub fn new_cuboid(mut min: DVec3, mut max: DVec3, max_len_sq: f64) -> Self {
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

        let edges = cell.edges.edges.entry(3).or_insert(Vec::new());

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

    /// Initializes the cell to be an octahedron with vertices at
    /// (l,0,0), (-l,0,0), (0,l,0), (0,-l,0), (0,0,l), and (0,0,-l).
    ///
    /// - `l`: a parameter setting the size of the octahedron.
    /// - `max_len_sq`: the square of the maximum possible length for two cells
    /// in the container. This is important for tolerance calculation.
    pub fn new_octahedron(mut l: f64, max_len_sq: f64) -> Self {
        let mut cell = Self::new_empty(max_len_sq);
        l *= 2.0;

        cell.vertices.push(DVec3::new(-l, 0.0, 0.0));
        cell.vertices.push(DVec3::new(l, 0.0, 0.0));
        cell.vertices.push(DVec3::new(0.0, -l, 0.0));
        cell.vertices.push(DVec3::new(0.0, l, 0.0));
        cell.vertices.push(DVec3::new(0.0, 0.0, -l));
        cell.vertices.push(DVec3::new(0.0, 0.0, l));

        let edges = cell.edges.edges.entry(4).or_insert(Vec::new());

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

    pub fn new_tetrahedron(mut a: DVec3, mut b: DVec3, mut c: DVec3, mut d: DVec3, max_len_sq: f64) -> Self {
        let mut cell = Self::new_empty(max_len_sq);
        a *= 2.0;
        b *= 2.0;
        c *= 2.0;
        d *= 2.0;

        cell.vertices.push(a);
        cell.vertices.push(b);
        cell.vertices.push(c);
        cell.vertices.push(d);

        let edges = cell.edges.edges.entry(3).or_insert(Vec::new());

        edges.extend([1, 3, 2, 0, 0, 0, 0].iter());
        edges.extend([0, 2, 3, 0, 2, 1, 1].iter());
        edges.extend([0, 3, 1, 2, 2, 1, 2].iter());
        edges.extend([0, 1, 2, 1, 2, 1, 3].iter());

        cell.edges.v2e.push((3, 0));
        cell.edges.v2e.push((3, 7));
        cell.edges.v2e.push((3, 14));
        cell.edges.v2e.push((3, 21));

        cell
    }
}

impl VoronoiCell for CellSgl {
    fn translate(&mut self, mut translation: DVec3) {
        translation *= 2.0;

        for v in &mut self.vertices {
            *v += translation;
        }
    }

    fn volume(&mut self) -> f64 {
        const FE: f64 = 1.0 / 48.0;
        let mut volume = 0.0;

        for i in 1..self.vertices.len() {
            let u = self.vertices[0] - self.vertices[i];
            for j in 0..self.edges.len(i) {
                let mut k = self.edges[i][j];
                if k >= 0 {
                    self.edges[i][j] = -1 - k;
                    let mut l = self.edges.ccw(self.edges[i][self.edges.len(i) + j], k);
                    let mut v = self.vertices[k as usize] - self.vertices[0];
                    let mut m = self.edges[k as usize][l as usize];
                    self.edges[k as usize][l as usize] = -1 - m;

                    while m != i as isize {
                        let n = self
                            .edges
                            .ccw(self.edges[k as usize][self.edges.len(k as usize) + l as usize], m);
                        let w = self.vertices[m as usize] - self.vertices[0];

                        volume += u.dot(v.cross(w));

                        k = m;
                        l = n;
                        v = w;
                        m = self.edges[k as usize][l as usize];
                        self.edges[k as usize][l as usize] = -1 - m as isize;
                    }
                }
            }
        }
        self.edges.reset();

        volume * FE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_float(left: f64, right: f64, epsilon: f64) {
        assert!((left - right).abs() <= epsilon, "{} != {}", left, right);
    }

    fn assert_vec3(left: DVec3, right: DVec3, epsilon: f64) {
        assert_float(left.x, right.x, epsilon);
        assert_float(left.y, right.y, epsilon);
        assert_float(left.z, right.z, epsilon);
    }

    #[test]
    fn new_empty() {
        let cell = CellSgl::new_empty(1234.0);
        cell.edges.assert();

        assert_float(cell.small_tolerance, 2.74003042477488634e-12, f64::EPSILON);
        assert_float(cell.large_tolerance, 5.48006084954977268e-11, f64::EPSILON);
    }

    #[test]
    fn new_cuboid() {
        let min = DVec3::new(1.57, 1.65, 1.31);
        let max = DVec3::new(1.86, 1.33, 2.54);
        let cell = CellSgl::new_cuboid(min, max, 3936.0);
        cell.edges.assert();

        assert_float(cell.small_tolerance, 8.73967564984923229e-12, f64::EPSILON);
        assert_float(cell.large_tolerance, 1.74793512996984646e-10, f64::EPSILON);

        assert_vec3(cell.vertices[0], DVec3::new(3.14, 3.30, 2.62), cell.small_tolerance);
        assert_vec3(cell.vertices[1], DVec3::new(3.72, 3.30, 2.62), cell.small_tolerance);
        assert_vec3(cell.vertices[2], DVec3::new(3.14, 2.66, 2.62), cell.small_tolerance);
        assert_vec3(cell.vertices[3], DVec3::new(3.72, 2.66, 2.62), cell.small_tolerance);
        assert_vec3(cell.vertices[4], DVec3::new(3.14, 3.30, 5.08), cell.small_tolerance);
        assert_vec3(cell.vertices[5], DVec3::new(3.72, 3.30, 5.08), cell.small_tolerance);
        assert_vec3(cell.vertices[6], DVec3::new(3.14, 2.66, 5.08), cell.small_tolerance);
        assert_vec3(cell.vertices[7], DVec3::new(3.72, 2.66, 5.08), cell.small_tolerance);

        assert_eq!(cell.edges[0], vec![1, 4, 2, 2, 1, 0, 0]);
        assert_eq!(cell.edges[1], vec![3, 5, 0, 2, 1, 0, 1]);
        assert_eq!(cell.edges[2], vec![0, 6, 3, 2, 1, 0, 2]);
        assert_eq!(cell.edges[3], vec![2, 7, 1, 2, 1, 0, 3]);
        assert_eq!(cell.edges[4], vec![6, 0, 5, 2, 1, 0, 4]);
        assert_eq!(cell.edges[5], vec![4, 1, 7, 2, 1, 0, 5]);
        assert_eq!(cell.edges[6], vec![7, 2, 4, 2, 1, 0, 6]);
        assert_eq!(cell.edges[7], vec![5, 3, 6, 2, 1, 0, 7]);

        assert_eq!(cell.edges.len(0), 3);
        assert_eq!(cell.edges.len(1), 3);
        assert_eq!(cell.edges.len(2), 3);
        assert_eq!(cell.edges.len(3), 3);
        assert_eq!(cell.edges.len(4), 3);
        assert_eq!(cell.edges.len(5), 3);
        assert_eq!(cell.edges.len(6), 3);
        assert_eq!(cell.edges.len(7), 3);

        assert_eq!(
            cell.edges.edges[&3],
            vec![
                1, 4, 2, 2, 1, 0, 0, 3, 5, 0, 2, 1, 0, 1, 0, 6, 3, 2, 1, 0, 2, 2, 7, 1, 2, 1, 0, 3, 6, 0, 5, 2, 1, 0,
                4, 4, 1, 7, 2, 1, 0, 5, 7, 2, 4, 2, 1, 0, 6, 5, 3, 6, 2, 1, 0, 7
            ]
        )
    }

    #[test]
    fn new_octahedron() {
        let l = 3.14;
        let cell = CellSgl::new_octahedron(l, 3134.0);
        cell.edges.assert();

        assert_float(cell.small_tolerance, 6.95887791835048120e-12, f64::EPSILON);
        assert_float(cell.large_tolerance, 1.39177558367009624e-10, f64::EPSILON);

        assert_vec3(cell.vertices[0], DVec3::new(-6.28, 0.00, 0.00), cell.small_tolerance);
        assert_vec3(cell.vertices[1], DVec3::new(6.28, 0.00, 0.00), cell.small_tolerance);
        assert_vec3(cell.vertices[2], DVec3::new(0.00, -6.28, 0.00), cell.small_tolerance);
        assert_vec3(cell.vertices[3], DVec3::new(0.00, 6.28, 0.00), cell.small_tolerance);
        assert_vec3(cell.vertices[4], DVec3::new(0.00, 0.00, -6.28), cell.small_tolerance);
        assert_vec3(cell.vertices[5], DVec3::new(0.00, 0.00, 6.28), cell.small_tolerance);

        assert_eq!(cell.edges[0], vec![2, 5, 3, 4, 0, 0, 0, 0, 0]);
        assert_eq!(cell.edges[1], vec![2, 4, 3, 5, 2, 2, 2, 2, 1]);
        assert_eq!(cell.edges[2], vec![0, 4, 1, 5, 0, 3, 0, 1, 2]);
        assert_eq!(cell.edges[3], vec![0, 5, 1, 4, 2, 3, 2, 1, 3]);
        assert_eq!(cell.edges[4], vec![0, 3, 1, 2, 3, 3, 1, 1, 4]);
        assert_eq!(cell.edges[5], vec![0, 2, 1, 3, 1, 3, 3, 1, 5]);

        assert_eq!(cell.edges.len(0), 4);
        assert_eq!(cell.edges.len(1), 4);
        assert_eq!(cell.edges.len(2), 4);
        assert_eq!(cell.edges.len(3), 4);
        assert_eq!(cell.edges.len(4), 4);
        assert_eq!(cell.edges.len(5), 4);

        assert_eq!(
            cell.edges.edges[&4],
            vec![
                2, 5, 3, 4, 0, 0, 0, 0, 0, 2, 4, 3, 5, 2, 2, 2, 2, 1, 0, 4, 1, 5, 0, 3, 0, 1, 2, 0, 5, 1, 4, 2, 3, 2,
                1, 3, 0, 3, 1, 2, 3, 3, 1, 1, 4, 0, 2, 1, 3, 1, 3, 3, 1, 5
            ]
        )
    }

    #[test]
    fn new_tetrahedron() {
        let a = DVec3::new(4.65, 4.51, 1.45);
        let b = DVec3::new(1.75, 2.14, 1.66);
        let c = DVec3::new(3.10, 3.38, 2.70);
        let d = DVec3::new(1.95, 2.64, 2.21);
        let cell = CellSgl::new_tetrahedron(a, b, c, d, 1818.0);
        cell.edges.assert();

        assert_float(cell.small_tolerance, 4.03677091753706918e-12, f64::EPSILON);
        assert_float(cell.large_tolerance, 8.07354183507413836e-11, f64::EPSILON);

        assert_vec3(cell.vertices[0], DVec3::new(9.30, 9.02, 2.90), cell.small_tolerance);
        assert_vec3(cell.vertices[1], DVec3::new(3.50, 4.28, 3.32), cell.small_tolerance);
        assert_vec3(cell.vertices[2], DVec3::new(6.20, 6.76, 5.40), cell.small_tolerance);
        assert_vec3(cell.vertices[3], DVec3::new(3.90, 5.28, 4.42), cell.small_tolerance);

        assert_eq!(cell.edges[0], vec![1, 3, 2, 0, 0, 0, 0]);
        assert_eq!(cell.edges[1], vec![0, 2, 3, 0, 2, 1, 1]);
        assert_eq!(cell.edges[2], vec![0, 3, 1, 2, 2, 1, 2]);
        assert_eq!(cell.edges[3], vec![0, 1, 2, 1, 2, 1, 3]);

        assert_eq!(cell.edges.len(0), 3);
        assert_eq!(cell.edges.len(1), 3);
        assert_eq!(cell.edges.len(2), 3);
        assert_eq!(cell.edges.len(3), 3);

        assert_eq!(
            cell.edges.edges[&3],
            vec![1, 3, 2, 0, 0, 0, 0, 0, 2, 3, 0, 2, 1, 1, 0, 3, 1, 2, 2, 1, 2, 0, 1, 2, 1, 2, 1, 3]
        )
    }

    #[test]
    fn translate() {
        let min = DVec3::new(1.57, 1.65, 1.31);
        let max = DVec3::new(1.86, 1.33, 2.54);
        let mut cell = CellSgl::new_cuboid(min, max, 3936.0);
        cell.translate(DVec3::new(3.14, 1.23, 3.13));

        assert_vec3(cell.vertices[0], DVec3::new(9.42, 5.76, 8.88), cell.small_tolerance);
        assert_vec3(cell.vertices[1], DVec3::new(10.00, 5.76, 8.88), cell.small_tolerance);
        assert_vec3(cell.vertices[2], DVec3::new(9.42, 5.12, 8.88), cell.small_tolerance);
        assert_vec3(cell.vertices[3], DVec3::new(10.00, 5.12, 8.88), cell.small_tolerance);
        assert_vec3(cell.vertices[4], DVec3::new(9.42, 5.76, 11.34), cell.small_tolerance);
        assert_vec3(cell.vertices[5], DVec3::new(10.00, 5.76, 11.34), cell.small_tolerance);
        assert_vec3(cell.vertices[6], DVec3::new(9.42, 5.12, 11.34), cell.small_tolerance);
        assert_vec3(cell.vertices[7], DVec3::new(10.00, 5.12, 11.34), cell.small_tolerance);
    }

    #[test]
    fn volume() {
        let min = DVec3::new(1.57, 1.33, 1.31);
        let max = DVec3::new(1.86, 1.65, 2.54);
        let mut cell = CellSgl::new_cuboid(min, max, 3936.0);
        assert_float(cell.volume(), 1.14143999999999940e-01, cell.small_tolerance);

        cell.translate(DVec3::new(3.14, 1.23, 3.13));
        assert_float(cell.volume(), 1.14143999999999940e-01, cell.small_tolerance);
    }
}