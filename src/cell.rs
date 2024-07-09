#[cxx::bridge(namespace = "voro")]
pub mod ffi {
    unsafe extern "C++" {
        include!("voro_rs/cpp/voro++.hh");
        include!("voro_rs/cpp/boilerplate.hh");

        type voronoicell;
        #[rust_name = "new_voronoicell"]
        fn construct() -> UniquePtr<voronoicell>;
        fn init_base(
            self: Pin<&mut voronoicell>,
            xmin: f64,
            xmax: f64,
            ymin: f64,
            ymax: f64,
            zmin: f64,
            zmax: f64,
        );
        fn init_octahedron_base(
            self: Pin<&mut voronoicell>,
            l: f64,
        );
        fn init_tetrahedron_base(
            self: Pin<&mut voronoicell>,
            x0: f64,
            y0: f64,
            z0: f64,
            x1: f64,
            y1: f64,
            z1: f64,
            x2: f64,
            y2: f64,
            z2: f64,
            x3: f64,
            y3: f64,
            z3: f64,
        );
        fn translate(
            self: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
        );
        fn volume(self: Pin<&mut voronoicell>) -> f64;
        fn max_radius_squared(
            self: Pin<&mut voronoicell>,
        ) -> f64;
        fn total_edge_distance(
            self: Pin<&mut voronoicell>,
        ) -> f64;
        fn surface_area(self: Pin<&mut voronoicell>)
            -> f64;
        fn centroid(
            self: Pin<&mut voronoicell>,
            cx: &mut f64,
            cy: &mut f64,
            cz: &mut f64,
        );
        fn number_of_faces(
            self: Pin<&mut voronoicell>,
        ) -> i32;
        fn number_of_edges(
            self: Pin<&mut voronoicell>,
        ) -> i32;
        fn vertex_orders(
            self: Pin<&mut voronoicell>,
            v: Pin<&mut CxxVector<i32>>,
        );
        #[rust_name = "vertices_local"]
        fn vertices(
            self: Pin<&mut voronoicell>,
            v: Pin<&mut CxxVector<f64>>,
        );
        #[rust_name = "vertices_global"]
        fn vertices(
            self: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
            v: Pin<&mut CxxVector<f64>>,
        );
        fn face_areas(
            self: Pin<&mut voronoicell>,
            v: Pin<&mut CxxVector<f64>>,
        );
        fn face_orders(
            self: Pin<&mut voronoicell>,
            v: Pin<&mut CxxVector<i32>>,
        );
        fn face_freq_table(
            self: Pin<&mut voronoicell>,
            v: Pin<&mut CxxVector<i32>>,
        );
        fn face_vertices(
            self: Pin<&mut voronoicell>,
            v: Pin<&mut CxxVector<i32>>,
        );
        fn face_perimeters(
            self: Pin<&mut voronoicell>,
            v: Pin<&mut CxxVector<f64>>,
        );
        fn normals(
            self: Pin<&mut voronoicell>,
            v: Pin<&mut CxxVector<f64>>,
        );
        fn plane_intersects(
            self: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
            rsq: f64,
        ) -> bool;
        fn plane_intersects_guess(
            self: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
            rsq: f64,
        ) -> bool;

        type voronoicell_neighbor;
        #[rust_name = "new_voronoicell_neighbor"]
        fn construct() -> UniquePtr<voronoicell_neighbor>;
        fn init_base(
            self: Pin<&mut voronoicell_neighbor>,
            xmin: f64,
            xmax: f64,
            ymin: f64,
            ymax: f64,
            zmin: f64,
            zmax: f64,
        );
        fn init_octahedron_base(
            self: Pin<&mut voronoicell_neighbor>,
            l: f64,
        );
        fn init_tetrahedron_base(
            self: Pin<&mut voronoicell_neighbor>,
            x0: f64,
            y0: f64,
            z0: f64,
            x1: f64,
            y1: f64,
            z1: f64,
            x2: f64,
            y2: f64,
            z2: f64,
            x3: f64,
            y3: f64,
            z3: f64,
        );
        fn translate(
            self: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
        );
        fn volume(self: Pin<&mut voronoicell_neighbor>) -> f64;
        fn max_radius_squared(
            self: Pin<&mut voronoicell_neighbor>,
        ) -> f64;
        fn total_edge_distance(
            self: Pin<&mut voronoicell_neighbor>,
        ) -> f64;
        fn surface_area(self: Pin<&mut voronoicell_neighbor>)
            -> f64;
        fn centroid(
            self: Pin<&mut voronoicell_neighbor>,
            cx: &mut f64,
            cy: &mut f64,
            cz: &mut f64,
        );
        fn number_of_faces(
            self: Pin<&mut voronoicell_neighbor>,
        ) -> i32;
        fn number_of_edges(
            self: Pin<&mut voronoicell_neighbor>,
        ) -> i32;
        fn vertex_orders(
            self: Pin<&mut voronoicell_neighbor>,
            v: Pin<&mut CxxVector<i32>>,
        );
        #[rust_name = "vertices_local"]
        fn vertices(
            self: Pin<&mut voronoicell_neighbor>,
            v: Pin<&mut CxxVector<f64>>,
        );
        #[rust_name = "vertices_global"]
        fn vertices(
            self: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
            v: Pin<&mut CxxVector<f64>>,
        );
        fn face_areas(
            self: Pin<&mut voronoicell_neighbor>,
            v: Pin<&mut CxxVector<f64>>,
        );
        fn face_orders(
            self: Pin<&mut voronoicell_neighbor>,
            v: Pin<&mut CxxVector<i32>>,
        );
        fn face_freq_table(
            self: Pin<&mut voronoicell_neighbor>,
            v: Pin<&mut CxxVector<i32>>,
        );
        fn face_vertices(
            self: Pin<&mut voronoicell_neighbor>,
            v: Pin<&mut CxxVector<i32>>,
        );
        fn face_perimeters(
            self: Pin<&mut voronoicell_neighbor>,
            v: Pin<&mut CxxVector<f64>>,
        );
        fn normals(
            self: Pin<&mut voronoicell_neighbor>,
            v: Pin<&mut CxxVector<f64>>,
        );
        fn plane_intersects(
            self: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
            rsq: f64,
        ) -> bool;
        fn plane_intersects_guess(
            self: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
            rsq: f64,
        ) -> bool;
    }
}

use cxx::{CxxVector, UniquePtr};

type Vec3 = [f64; 3];

/// `voronoicell_base` abstract class in voro++.
///
/// A trait representing a single Voronoi cell.
///
/// This trait represents a single Voronoi cell, as a collection of vertices
/// that are connected by edges. The class contains routines for initializing
/// the Voronoi cell to be simple shapes such as a box, tetrahedron, or octahedron.
/// It the contains routines for recomputing the cell based on cutting it
/// by a plane, which forms the key routine for the Voronoi cell computation.
/// It contains numerous routine for computing statistics about the Voronoi cell,
/// and it can output the cell in several formats.
pub trait VoronoiCell {
    /// Translates the vertices of the Voronoi cell by a given vector.
    ///
    /// * `xyz`: the coordinates of the vector.
    fn translate(&mut self, xyz: Vec3);

    /// Calculates the volume of the Voronoi cell, by decomposing the cell into
    /// tetrahedra extending outward from the zeroth vertex, whose volumes are
    /// evaluated using a scalar triple product.
    ///
    /// Return a floating point number holding the calculated volume.
    fn volume(&mut self) -> f64;

    /// Computes the maximum radius squared of a vertex from the center of the
    /// cell. It can be used to determine when enough particles have been testing an
    /// all planes that could cut the cell have been considered.
    ///
    /// Return the maximum radius squared of a vertex.
    fn max_radius_squared(&mut self) -> f64;

    /// Calculates the total edge distance of the Voronoi cell.
    ///
    /// Return a floating point number holding the calculated distance.
    fn total_edge_distance(&mut self) -> f64;

    /// Calculates the total surface area of the Voronoi cell.
    ///
    /// Return the computed area.
    fn surface_area(&mut self) -> f64;

    /// Calculates the centroid of the Voronoi cell, by decomposing the cell into
    /// tetrahedra extending outward from the zeroth vertex.
    ///
    /// Return the centroid vector.
    fn centroid(&mut self) -> Vec3;

    /// Returns the number of faces of a computed Voronoi cell.
    fn number_of_faces(&mut self) -> i32;

    /// Counts the number of edges of the Voronoi cell.
    fn number_of_edges(&mut self) -> i32;

    /// Returns a vector of the vertex orders.
    fn vertex_orders(&mut self) -> Vec<i32>;
}

/// `voronoicell` class in voro++.
///
/// A class represent a Voronoi cell without neighbor information.
///
/// This class is an extension of the voronoicell_base class, in cases when
/// is not necessary to track the IDs of neighboring particles associated
/// with each face of the Voronoi cell.
pub struct VoronoiCellNoNeighbor {
    inner: UniquePtr<ffi::voronoicell>,
}

impl VoronoiCellNoNeighbor {
    /// Initializes the Voronoi cell to be rectangular box with the
    /// given dimensions.
    ///
    /// * `xyz_min`: the minimum xyz coordinates.
    /// * `xyz_max`: the maximum xyz coordinates.
    pub fn new(xyz_min: Vec3, xyz_max: Vec3) -> Self {
        let mut val = Self {
            inner: ffi::new_voronoicell(),
        };
        val.inner.pin_mut().init_base(
            xyz_min[0], xyz_max[0], xyz_min[1], xyz_max[1],
            xyz_min[2], xyz_max[2],
        );
        val
    }

    /// Initializes the cell to be an octahedron with vertices at
    /// (l,0,0), (-l,0,0), (0,l,0), (0,-l,0), (0,0,l), and (0,0,-l).
    ///
    /// * `l`: a parameter setting the size of the octahedron.
    pub fn new_octahedron(l: f64) -> Self {
        let mut val = Self {
            inner: ffi::new_voronoicell(),
        };
        val.inner.pin_mut().init_octahedron_base(l);
        val
    }

    /// Initializes the cell to be a tetrahedron.
    ///
    /// * `xyz0`: the coordinates of the first vertex.
    /// * `xyz1`: the coordinates of the second vertex.
    /// * `xyz2`: the coordinates of the third vertex.
    /// * `xyz3`: the coordinates of the fourth vertex.
    pub fn new_tetrahedron(
        xyz0: Vec3,
        xyz1: Vec3,
        xyz2: Vec3,
        xyz3: Vec3,
    ) -> Self {
        let mut val = Self {
            inner: ffi::new_voronoicell(),
        };
        val.inner.pin_mut().init_tetrahedron_base(
            xyz0[0], xyz0[1], xyz0[2], xyz1[0], xyz1[1],
            xyz1[2], xyz2[0], xyz2[1], xyz2[2], xyz3[0],
            xyz3[1], xyz3[2],
        );
        val
    }
}

impl VoronoiCell for VoronoiCellNoNeighbor {
    fn translate(&mut self, xyz: Vec3) {
        self.inner
            .pin_mut()
            .translate(xyz[0], xyz[1], xyz[2]);
    }

    fn volume(&mut self) -> f64 {
        self.inner.pin_mut().volume()
    }

    fn max_radius_squared(&mut self) -> f64 {
        self.inner.pin_mut().max_radius_squared()
    }

    fn total_edge_distance(&mut self) -> f64 {
        self.inner.pin_mut().total_edge_distance()
    }

    fn surface_area(&mut self) -> f64 {
        self.inner.pin_mut().surface_area()
    }

    fn centroid(&mut self) -> Vec3 {
        let mut cx = 0.0;
        let mut cy = 0.0;
        let mut cz = 0.0;
        self.inner
            .pin_mut()
            .centroid(&mut cx, &mut cy, &mut cz);
        [cx, cy, cz]
    }

    fn number_of_faces(&mut self) -> i32 {
        self.inner.pin_mut().number_of_faces()
    }

    fn number_of_edges(&mut self) -> i32 {
        self.inner.pin_mut().number_of_edges()
    }

    fn vertex_orders(&mut self) -> Vec<i32> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().vertex_orders(v.pin_mut());
        v.into_iter().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_new() {
        VoronoiCellNoNeighbor::new(
            [1.0, 1.0, 1.0],
            [2.0, 2.0, 2.0],
        );
    }
}
