//! Voronoi Cell

use crate::native::*;
use glam::{DVec3, DVec4, Vec4Swizzles};

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

    pt: Vec<DVec4>,
    ed: Edges,

    // mask[i] = mask[i]
    // current_vertices = mask.capacity()
    // maskc
    mask: Vec<usize>,
    mask_c: usize,

    // the index of a point in a cell for tracing routines.
    up: usize,

    // normal vector, the 4th one is the magnitude
    norm_vec: DVec4,

    // the first delete stack
    delete_stack: Vec<usize>,

    // the second delete stack
    delete_stack2: Vec<usize>,

    // the search stack
    search_stack: Vec<usize>,
}

impl CellSgl {
    fn nu(&self, v: usize) -> usize {
        self.ed.vertex_order(v)
    }

    fn reset_mask(&mut self) {
        for m in &mut self.mask {
            *m = 0;
        }
        self.mask_c = 4;
    }

    fn m_calc(&mut self, n: usize, ans: &mut f64) -> usize {
        *ans = self.pt[n].xyz().dot(self.norm_vec.xyz());
        self.pt[n].w = *ans;
        let mask_r = if *ans < -self.small_tolerance {
            0
        } else if *ans > self.small_tolerance {
            2
        } else {
            1
        };
        self.mask[n] = self.mask_c | mask_r;
        mask_r
    }

    fn m_test(&mut self, n: usize, ans: &mut f64) -> usize {
        if self.mask[n] > self.mask_c {
            *ans = self.pt[n].w;
            self.mask[n] & 3
        } else {
            self.m_calc(n, ans)
        }
    }

    fn flip(&mut self, tp: usize) {
        let j = self.nu(tp) * 2;
        self.ed[tp][j] = -1 - self.ed[tp][j]
    }

    fn definite_min(&mut self, lp: &mut usize, us: &mut usize, l: &mut f64, u: &mut f64, lw: &mut usize) -> bool {
        let mut tp = self.up;
        let mut qp = 0;
        let mut q = 0.0;
        let mut ts = 0;

        // Check to see whether point up is a well-defined maximum. Otherwise
        // any neighboring vertices of up that are marginal need to be
        // followed, to see if they lead to a better maximum.
        while ts < self.nu(tp) {
            qp = self.ed[tp][ts] as usize;
            self.m_test(qp, &mut q);
            if q < *u + self.large_tolerance {
                break;
            }
            ts += 1;
        }
        if ts == self.nu(tp) {
            return true;
        }

        // The point tp is marginal, so it will be necessary to do the
        // flood-fill search. Mark the point tp and the point qp, and search
        // any remaining neighbors of the point tp.
        self.flip(self.up);
        self.flip(qp);
        self.delete_stack.push(qp);
        ts += 1;
        while ts < self.nu(tp) {
            qp = self.ed[tp][ts] as usize;
            self.m_test(qp as usize, &mut q);
            if q < *u + self.large_tolerance {
                self.delete_stack.push(*lp);
                self.flip(*lp as usize);
            }
            ts += 1;
        }

        // Consider additional marginal points, starting with the original
        // point qp
        let mut qw;
        let mut i = 0;
        while i < self.delete_stack.len() {
            tp = self.delete_stack[i];
            i += 1;
            ts = 0;
            while ts < self.nu(tp) {
                qp = self.ed[tp][ts] as usize;
                // Skip the point if it's already marked
                if self.ed[qp][self.nu(qp) * 2] < 0 {
                    continue;
                }
                qw = self.m_test(qp, &mut q);
                // This point is a better minimum. Reset markers and
                // return true.
                if q < *u {
                    self.flip(self.up);
                    self.up = tp;
                    *us = ts;
                    self.m_calc(self.up, u);
                    *lp = qp;
                    *lw = qw;
                    *l = q;
                    if !self.delete_stack.is_empty() {
                        let tmp = self.delete_stack.pop().unwrap();
                        self.flip(tmp);
                    }
                    return false;
                }
                // The point is marginal and therefore must also be
                // considered
                if q < *u + self.large_tolerance {
                    self.delete_stack.push(qp);
                    self.flip(qp);
                }
                ts += 1;
            }
        }
        // Reset markers and return false
        self.flip(self.up);
        if !self.delete_stack.is_empty() {
            let tmp = self.delete_stack.pop().unwrap();
            self.flip(tmp);
        }
        true
    }

    fn search_downward(
        &mut self,
        lw: &mut usize,
        lp: &mut usize,
        ls: &mut usize,
        us: &mut usize,
        l: &mut f64,
        u: &mut f64,
    ) -> bool {
        let mut vs;

        *us = 0;
        while *us < self.nu(self.up) {
            *lp = self.ed[self.up][*us] as usize;
            *lw = self.m_test(*lp, l);
            *us += 1
        }
        if *us == self.nu(self.up) {
            if self.definite_min(lp, us, l, u, lw) {
                return false;
            }
        }

        while *lw == 2 {
            // Test all the neighbors of the current point
            // and find the one which is closest to the
            // plane
            vs = self.ed[self.up][self.nu(self.up) + *us] as usize;
            self.up = *lp;
            *u = *l;
            *us = 0;
            while *us < self.nu(self.up) {
                if *us == vs {
                    continue;
                }
                *lp = self.ed[self.up][*us] as usize;
                *lw = self.m_test(*lp, l);
                if *u > *l {
                    break;
                }
                *us += 1;
            }
            if *us == self.nu(self.up) && self.definite_min(lp, us, l, u, lw) {
                return false;
            }
        }
        *ls = self.ed[self.up][self.nu(self.up) + *us] as usize;

        true
    }

    #[allow(unused)]
    fn n_plane_impl(&mut self, norm_vec: DVec3, rsq: f64, pid: u32) -> bool {
        let lp = self.up;
        let us = 0;
        let ls = 0;
        let l = 0.0;
        let mut u = 0.0;

        self.up = 0;
        self.norm_vec.x = norm_vec.x;
        self.norm_vec.y = norm_vec.y;
        self.norm_vec.z = norm_vec.z;
        self.norm_vec.w = rsq;

        self.mask_c += 4;
        if self.mask_c < 4 {
            self.reset_mask();
        }
        let uw = self.m_test(self.up, &mut u);
        todo!()

        // uw=m_test(up,u);
        // if(uw==2) {
        // 	if(!search_downward(lw,lp,ls,us,l,u)) return false;
        // 	if(lw==1) {up=lp;lp=-1;}
        // } else if(uw==0) {
        // 	if(!search_upward(uw,lp,ls,us,l,u)) return true;
        // 	if(uw==1) lp=-1;
        // } else {
        // 	lp=-1;
        // }
    }
}

impl CellSgl {
    /// Create an empty cell
    ///
    /// - `max_len_sq`: the square of the maximum possible length for two cells
    /// in the container. This is important for tolerance calculation.
    pub(crate) fn new_empty(max_len_sq: f64) -> Self {
        Self {
            small_tolerance: max_len_sq * constant::small_tolerance,
            large_tolerance: max_len_sq * constant::large_tolerance,

            pt: Vec::default(),
            ed: Edges::default(),

            mask: Vec::default(),
            mask_c: 0,

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

        cell.pt.push(DVec4::new(min.x, min.y, min.z, 0.0));
        cell.pt.push(DVec4::new(max.x, min.y, min.z, 0.0));
        cell.pt.push(DVec4::new(min.x, max.y, min.z, 0.0));
        cell.pt.push(DVec4::new(max.x, max.y, min.z, 0.0));
        cell.pt.push(DVec4::new(min.x, min.y, max.z, 0.0));
        cell.pt.push(DVec4::new(max.x, min.y, max.z, 0.0));
        cell.pt.push(DVec4::new(min.x, max.y, max.z, 0.0));
        cell.pt.push(DVec4::new(max.x, max.y, max.z, 0.0));
        cell.mask.resize(cell.pt.len(), 0);

        let edges = cell.ed.edges.entry(3).or_insert(Vec::new());

        edges.extend([1, 4, 2, 2, 1, 0, 0].iter());
        edges.extend([3, 5, 0, 2, 1, 0, 1].iter());
        edges.extend([0, 6, 3, 2, 1, 0, 2].iter());
        edges.extend([2, 7, 1, 2, 1, 0, 3].iter());
        edges.extend([6, 0, 5, 2, 1, 0, 4].iter());
        edges.extend([4, 1, 7, 2, 1, 0, 5].iter());
        edges.extend([7, 2, 4, 2, 1, 0, 6].iter());
        edges.extend([5, 3, 6, 2, 1, 0, 7].iter());

        cell.ed.v2e.push((3, 0));
        cell.ed.v2e.push((3, 7));
        cell.ed.v2e.push((3, 14));
        cell.ed.v2e.push((3, 21));
        cell.ed.v2e.push((3, 28));
        cell.ed.v2e.push((3, 35));
        cell.ed.v2e.push((3, 42));
        cell.ed.v2e.push((3, 49));

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

        cell.pt.push(DVec4::new(-l, 0.0, 0.0, 0.0));
        cell.pt.push(DVec4::new(l, 0.0, 0.0, 0.0));
        cell.pt.push(DVec4::new(0.0, -l, 0.0, 0.0));
        cell.pt.push(DVec4::new(0.0, l, 0.0, 0.0));
        cell.pt.push(DVec4::new(0.0, 0.0, -l, 0.0));
        cell.pt.push(DVec4::new(0.0, 0.0, l, 0.0));
        cell.mask.resize(cell.pt.len(), 0);

        let edges = cell.ed.edges.entry(4).or_insert(Vec::new());

        edges.extend([2, 5, 3, 4, 0, 0, 0, 0, 0].iter());
        edges.extend([2, 4, 3, 5, 2, 2, 2, 2, 1].iter());
        edges.extend([0, 4, 1, 5, 0, 3, 0, 1, 2].iter());
        edges.extend([0, 5, 1, 4, 2, 3, 2, 1, 3].iter());
        edges.extend([0, 3, 1, 2, 3, 3, 1, 1, 4].iter());
        edges.extend([0, 2, 1, 3, 1, 3, 3, 1, 5].iter());

        cell.ed.v2e.push((4, 0));
        cell.ed.v2e.push((4, 9));
        cell.ed.v2e.push((4, 18));
        cell.ed.v2e.push((4, 27));
        cell.ed.v2e.push((4, 36));
        cell.ed.v2e.push((4, 45));

        cell
    }

    pub fn new_tetrahedron(mut a: DVec3, mut b: DVec3, mut c: DVec3, mut d: DVec3, max_len_sq: f64) -> Self {
        let mut cell = Self::new_empty(max_len_sq);
        a *= 2.0;
        b *= 2.0;
        c *= 2.0;
        d *= 2.0;

        cell.pt.push(DVec4::new(a.x, a.y, a.z, 0.0));
        cell.pt.push(DVec4::new(b.x, b.y, b.z, 0.0));
        cell.pt.push(DVec4::new(c.x, c.y, c.z, 0.0));
        cell.pt.push(DVec4::new(d.x, d.y, d.z, 0.0));
        cell.mask.resize(cell.pt.len(), 0);

        let edges = cell.ed.edges.entry(3).or_insert(Vec::new());

        edges.extend([1, 3, 2, 0, 0, 0, 0].iter());
        edges.extend([0, 2, 3, 0, 2, 1, 1].iter());
        edges.extend([0, 3, 1, 2, 2, 1, 2].iter());
        edges.extend([0, 1, 2, 1, 2, 1, 3].iter());

        cell.ed.v2e.push((3, 0));
        cell.ed.v2e.push((3, 7));
        cell.ed.v2e.push((3, 14));
        cell.ed.v2e.push((3, 21));

        cell
    }

    fn max_radius_sq(&self) -> f64 {
        self.pt
            .iter()
            .map(|v| v.length_squared())
            .max_by(|x, y| x.total_cmp(y))
            .unwrap()
    }
}

impl VoronoiCell for CellSgl {
    fn translate(&mut self, mut translation: DVec3) {
        translation *= 2.0;

        for v in &mut self.pt {
            v.x += translation.x;
            v.y += translation.y;
            v.z += translation.z;
        }
    }

    fn volume(&mut self) -> f64 {
        const FE: f64 = 1.0 / 48.0;
        let mut volume = 0.0;

        for i in 1..self.pt.len() {
            let u = (self.pt[0] - self.pt[i]).xyz();
            for j in 0..self.nu(i) {
                let mut k = self.ed[i][j];
                if k >= 0 {
                    self.ed[i][j] = -1 - k;
                    let mut l = self.ed.ccw(self.ed[i][self.nu(i) + j], k);
                    let mut v = (self.pt[k as usize] - self.pt[0]).xyz();
                    let mut m = self.ed[k as usize][l as usize];
                    self.ed[k as usize][l as usize] = -1 - m;

                    while m != i as isize {
                        let n = self.ed.ccw(self.ed[k as usize][self.nu(k as usize) + l as usize], m);
                        let w = (self.pt[m as usize] - self.pt[0]).xyz();

                        volume += u.dot(v.cross(w));

                        k = m;
                        l = n;
                        v = w;
                        m = self.ed[k as usize][l as usize];
                        self.ed[k as usize][l as usize] = -1 - m as isize;
                    }
                }
            }
        }
        self.ed.reset();

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
        cell.ed.assert();

        assert_float(cell.small_tolerance, 2.74003042477488634e-12, f64::EPSILON);
        assert_float(cell.large_tolerance, 5.48006084954977268e-11, f64::EPSILON);
    }

    #[test]
    fn new_cuboid() {
        let min = DVec3::new(1.57, 1.65, 1.31);
        let max = DVec3::new(1.86, 1.33, 2.54);
        let cell = CellSgl::new_cuboid(min, max, 3936.0);
        cell.ed.assert();

        assert_float(cell.small_tolerance, 8.73967564984923229e-12, f64::EPSILON);
        assert_float(cell.large_tolerance, 1.74793512996984646e-10, f64::EPSILON);

        assert_vec3(cell.pt[0].xyz(), DVec3::new(3.14, 3.30, 2.62), cell.small_tolerance);
        assert_vec3(cell.pt[1].xyz(), DVec3::new(3.72, 3.30, 2.62), cell.small_tolerance);
        assert_vec3(cell.pt[2].xyz(), DVec3::new(3.14, 2.66, 2.62), cell.small_tolerance);
        assert_vec3(cell.pt[3].xyz(), DVec3::new(3.72, 2.66, 2.62), cell.small_tolerance);
        assert_vec3(cell.pt[4].xyz(), DVec3::new(3.14, 3.30, 5.08), cell.small_tolerance);
        assert_vec3(cell.pt[5].xyz(), DVec3::new(3.72, 3.30, 5.08), cell.small_tolerance);
        assert_vec3(cell.pt[6].xyz(), DVec3::new(3.14, 2.66, 5.08), cell.small_tolerance);
        assert_vec3(cell.pt[7].xyz(), DVec3::new(3.72, 2.66, 5.08), cell.small_tolerance);

        assert_eq!(cell.ed[0], vec![1, 4, 2, 2, 1, 0, 0]);
        assert_eq!(cell.ed[1], vec![3, 5, 0, 2, 1, 0, 1]);
        assert_eq!(cell.ed[2], vec![0, 6, 3, 2, 1, 0, 2]);
        assert_eq!(cell.ed[3], vec![2, 7, 1, 2, 1, 0, 3]);
        assert_eq!(cell.ed[4], vec![6, 0, 5, 2, 1, 0, 4]);
        assert_eq!(cell.ed[5], vec![4, 1, 7, 2, 1, 0, 5]);
        assert_eq!(cell.ed[6], vec![7, 2, 4, 2, 1, 0, 6]);
        assert_eq!(cell.ed[7], vec![5, 3, 6, 2, 1, 0, 7]);

        assert_eq!(cell.nu(0), 3);
        assert_eq!(cell.nu(1), 3);
        assert_eq!(cell.nu(2), 3);
        assert_eq!(cell.nu(3), 3);
        assert_eq!(cell.nu(4), 3);
        assert_eq!(cell.nu(5), 3);
        assert_eq!(cell.nu(6), 3);
        assert_eq!(cell.nu(7), 3);

        assert_eq!(
            cell.ed.edges[&3],
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
        cell.ed.assert();

        assert_float(cell.small_tolerance, 6.95887791835048120e-12, f64::EPSILON);
        assert_float(cell.large_tolerance, 1.39177558367009624e-10, f64::EPSILON);

        assert_vec3(cell.pt[0].xyz(), DVec3::new(-6.28, 0.00, 0.00), cell.small_tolerance);
        assert_vec3(cell.pt[1].xyz(), DVec3::new(6.28, 0.00, 0.00), cell.small_tolerance);
        assert_vec3(cell.pt[2].xyz(), DVec3::new(0.00, -6.28, 0.00), cell.small_tolerance);
        assert_vec3(cell.pt[3].xyz(), DVec3::new(0.00, 6.28, 0.00), cell.small_tolerance);
        assert_vec3(cell.pt[4].xyz(), DVec3::new(0.00, 0.00, -6.28), cell.small_tolerance);
        assert_vec3(cell.pt[5].xyz(), DVec3::new(0.00, 0.00, 6.28), cell.small_tolerance);

        assert_eq!(cell.ed[0], vec![2, 5, 3, 4, 0, 0, 0, 0, 0]);
        assert_eq!(cell.ed[1], vec![2, 4, 3, 5, 2, 2, 2, 2, 1]);
        assert_eq!(cell.ed[2], vec![0, 4, 1, 5, 0, 3, 0, 1, 2]);
        assert_eq!(cell.ed[3], vec![0, 5, 1, 4, 2, 3, 2, 1, 3]);
        assert_eq!(cell.ed[4], vec![0, 3, 1, 2, 3, 3, 1, 1, 4]);
        assert_eq!(cell.ed[5], vec![0, 2, 1, 3, 1, 3, 3, 1, 5]);

        assert_eq!(cell.nu(0), 4);
        assert_eq!(cell.nu(1), 4);
        assert_eq!(cell.nu(2), 4);
        assert_eq!(cell.nu(3), 4);
        assert_eq!(cell.nu(4), 4);
        assert_eq!(cell.nu(5), 4);

        assert_eq!(
            cell.ed.edges[&4],
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
        cell.ed.assert();

        assert_float(cell.small_tolerance, 4.03677091753706918e-12, f64::EPSILON);
        assert_float(cell.large_tolerance, 8.07354183507413836e-11, f64::EPSILON);

        assert_vec3(cell.pt[0].xyz(), DVec3::new(9.30, 9.02, 2.90), cell.small_tolerance);
        assert_vec3(cell.pt[1].xyz(), DVec3::new(3.50, 4.28, 3.32), cell.small_tolerance);
        assert_vec3(cell.pt[2].xyz(), DVec3::new(6.20, 6.76, 5.40), cell.small_tolerance);
        assert_vec3(cell.pt[3].xyz(), DVec3::new(3.90, 5.28, 4.42), cell.small_tolerance);

        assert_eq!(cell.ed[0], vec![1, 3, 2, 0, 0, 0, 0]);
        assert_eq!(cell.ed[1], vec![0, 2, 3, 0, 2, 1, 1]);
        assert_eq!(cell.ed[2], vec![0, 3, 1, 2, 2, 1, 2]);
        assert_eq!(cell.ed[3], vec![0, 1, 2, 1, 2, 1, 3]);

        assert_eq!(cell.nu(0), 3);
        assert_eq!(cell.nu(1), 3);
        assert_eq!(cell.nu(2), 3);
        assert_eq!(cell.nu(3), 3);

        assert_eq!(
            cell.ed.edges[&3],
            vec![1, 3, 2, 0, 0, 0, 0, 0, 2, 3, 0, 2, 1, 1, 0, 3, 1, 2, 2, 1, 2, 0, 1, 2, 1, 2, 1, 3]
        )
    }

    #[test]
    fn translate() {
        let min = DVec3::new(1.57, 1.65, 1.31);
        let max = DVec3::new(1.86, 1.33, 2.54);
        let mut cell = CellSgl::new_cuboid(min, max, 3936.0);
        cell.translate(DVec3::new(3.14, 1.23, 3.13));

        assert_vec3(cell.pt[0].xyz(), DVec3::new(9.42, 5.76, 8.88), cell.small_tolerance);
        assert_vec3(cell.pt[1].xyz(), DVec3::new(10.00, 5.76, 8.88), cell.small_tolerance);
        assert_vec3(cell.pt[2].xyz(), DVec3::new(9.42, 5.12, 8.88), cell.small_tolerance);
        assert_vec3(cell.pt[3].xyz(), DVec3::new(10.00, 5.12, 8.88), cell.small_tolerance);
        assert_vec3(cell.pt[4].xyz(), DVec3::new(9.42, 5.76, 11.34), cell.small_tolerance);
        assert_vec3(cell.pt[5].xyz(), DVec3::new(10.00, 5.76, 11.34), cell.small_tolerance);
        assert_vec3(cell.pt[6].xyz(), DVec3::new(9.42, 5.12, 11.34), cell.small_tolerance);
        assert_vec3(cell.pt[7].xyz(), DVec3::new(10.00, 5.12, 11.34), cell.small_tolerance);
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

    #[test]
    fn max_radius_sq() {
        let min = DVec3::new(1.57, 1.33, 1.31);
        let max = DVec3::new(1.86, 1.65, 2.54);
        let mut cell = CellSgl::new_cuboid(min, max, 3936.0);
        assert_float(cell.max_radius_sq(), 5.05348000000000042e+01, cell.small_tolerance);

        cell.translate(DVec3::new(3.14, 1.23, 3.13));
        assert_float(cell.max_radius_sq(), 2.61773199999999974e+02, cell.small_tolerance);
    }
}
