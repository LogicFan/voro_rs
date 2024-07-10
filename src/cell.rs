//! Voronoicell and related classes.

#[cxx::bridge(namespace = "voro")]
pub mod ffi {
    unsafe extern "C++" {
        include!("voro_rs/cpp/boilerplate.hh");

        type voronoicell;
        #[rust_name = "new_voronoicell"]
        fn construct() -> UniquePtr<voronoicell>;
        fn clone_voronoicell(
            value: &UniquePtr<voronoicell>,
        ) -> UniquePtr<voronoicell>;
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
        #[rust_name = "nplane_rsq"]
        fn nplane(
            self: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
            rsq: f64,
            p_id: i32,
        ) -> bool;
        fn nplane(
            self: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
            p_id: i32,
        ) -> bool;
        #[rust_name = "plane_rsq"]
        fn plane(
            self: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
            rsq: f64,
        ) -> bool;
        fn plane(
            self: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;

        type voronoicell_neighbor;
        #[rust_name = "new_voronoicell_neighbor"]
        fn construct() -> UniquePtr<voronoicell_neighbor>;
        #[rust_name = "clone_voronoicell_neighbor"]
        fn clone_voronoicell(
            value: &UniquePtr<voronoicell_neighbor>,
        ) -> UniquePtr<voronoicell_neighbor>;
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
        fn volume(
            self: Pin<&mut voronoicell_neighbor>,
        ) -> f64;
        fn max_radius_squared(
            self: Pin<&mut voronoicell_neighbor>,
        ) -> f64;
        fn total_edge_distance(
            self: Pin<&mut voronoicell_neighbor>,
        ) -> f64;
        fn surface_area(
            self: Pin<&mut voronoicell_neighbor>,
        ) -> f64;
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
        #[rust_name = "nplane_rsq"]
        fn nplane(
            self: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
            rsq: f64,
            p_id: i32,
        ) -> bool;
        fn nplane(
            self: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
            p_id: i32,
        ) -> bool;
        #[rust_name = "plane_rsq"]
        fn plane(
            self: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
            rsq: f64,
        ) -> bool;
        fn plane(
            self: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        fn neighbors(
            self: Pin<&mut voronoicell_neighbor>,
            v: Pin<&mut CxxVector<i32>>,
        );
    }
}

use cxx::{CxxVector, UniquePtr};

type DVec3 = [f64; 3];

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
pub trait VoroCell {
    /// Translates the vertices of the Voronoi cell by a given vector.
    ///
    /// * `xyz`: the coordinates of the vector.
    fn translate(&mut self, xyz: DVec3);

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
    fn centroid(&mut self) -> DVec3;

    /// Returns the number of faces of a computed Voronoi cell.
    fn number_of_faces(&mut self) -> i32;

    /// Counts the number of edges of the Voronoi cell.
    fn number_of_edges(&mut self) -> i32;

    /// Returns a vector of the vertex orders.
    fn vertex_orders(&mut self) -> Vec<i32>;

    /// Returns a vector of the vertex vectors using the local coordinate system.
    fn vertices_local(&mut self) -> Vec<f64>;

    /// Returns a vector of the vertex vectors in the global coordinate system.
    /// * `xyz`: the position vector of the particle in the global coordinate system.
    fn vertices_global(&mut self, xyz: DVec3) -> Vec<f64>;

    /// Calculates the areas of each face of the Voronoi cell and prints the
    /// results to an output vector.
    fn face_areas(&mut self) -> Vec<f64>;

    /// Outputs a list of the number of edges in each face.
    fn face_orders(&mut self) -> Vec<i32>;

    /// Computes the number of edges that each face has and outputs a frequency
    /// table of the results.
    fn face_freq_table(&mut self) -> Vec<i32>;

    /// For each face, this routine outputs a bracketed sequence of numbers
    /// containing a list of all the vertices that make up that face.
    fn face_vertices(&mut self) -> Vec<i32>;

    /// This routine returns the perimeters of each face.
    fn face_perimeters(&mut self) -> Vec<f64>;

    /// For each face of the Voronoi cell, this routine prints the out the normal
    /// vector of the face, and scales it to the distance from the cell center to
    /// that plane.
    fn normals(&mut self) -> Vec<f64>;

    /// This routine tests to see whether the cell intersects a plane by starting
    /// from the guess point up. If up intersects, then it immediately returns true.
    /// Otherwise, it calls the plane_intersects_track() routine.
    ///
    /// * `xyz`: the normal vector to the plane.
    /// * `rsq`: the distance along this vector of the plane.
    ///
    /// Return false if the plane does not intersect the plane, true if it does.
    fn plane_intersects(
        &mut self,
        xyz: DVec3,
        rsq: f64,
    ) -> bool;

    /// This routine tests to see if a cell intersects a plane. It first tests a
    /// random sample of approximately sqrt(p)/4 points. If any of those are
    /// intersect, then it immediately returns true. Otherwise, it takes the closest
    /// point and passes that to plane_intersect_track() routine.
    ///
    /// * `xyz`: the normal vector to the plane.
    /// * `rsq`: the distance along this vector of the plane.
    ///
    /// Return false if the plane does not intersect the plane, true if it does. */
    fn plane_intersects_guess(
        &mut self,
        xyz: DVec3,
        rsq: f64,
    ) -> bool;

    /// Cuts the Voronoi cell by a particle whose center is at a
    /// separation of (x,y,z) from the cell center. The value of rsq
    /// should be initially set to $x^2+y^2+z^2$.
    ///
    /// * `xyz`: the normal vector to the plane.
    /// * `rsq`: the distance along this vector of the plane.
    /// * `p_id`: the plane ID (for neighbor tracking only).
    ///
    /// Return false if the plane cut deleted the cell entirely,
    /// true otherwise.
    fn nplane_rsq(
        &mut self,
        xyz: DVec3,
        rsq: f64,
        p_id: i32,
    ) -> bool;

    /// This routine calculates the modulus squared of the vector
    /// before passing it to the main nplane() routine with full
    /// arguments.
    ///
    /// * `xyz`: the vector to cut the cell by.
    /// * `p_id`: the plane ID (for neighbor tracking only).
    ///
    /// Return false if the plane cut deleted the cell entirely,
    /// true otherwise.
    fn nplane(&mut self, xyz: DVec3, p_id: i32) -> bool;

    /// This version of the plane routine just makes up the plane
    /// ID to be zero. It will only be referenced if neighbor
    /// tracking is enabled.
    ///
    /// * `xyz`: the vector to cut the cell by.
    /// * `rsq`: the modulus squared of the vector.
    ///
    /// Return false if the plane cut deleted the cell entirely,
    /// true otherwise.
    fn plane_rsq(&mut self, xyz: DVec3, rsq: f64) -> bool;

    /// Cuts a Voronoi cell using the influence of a particle at
    /// (x,y,z), first calculating the modulus squared of this
    /// vector before passing it to the main nplane() routine. Zero
    /// is supplied as the plane ID, which will be ignored unless
    /// neighbor tracking is enabled.
    ///
    /// * `xyz`: the vector to cut the cell by.
    ///
    /// Return false if the plane cut deleted the cell entirely,
    /// true otherwise.
    fn plane(&mut self, xyz: DVec3) -> bool;
}

/// A enum to store mutable reference of any `VoroCell`. This is 
/// to mimic the override in C++.
pub enum VoroCellMut<'a> {
    Sgl(&'a mut VoroCellSgl),
    Nbr(&'a mut VoroCellNbr),
}

/// `voronoicell` class in voro++.
///
/// A class represent a Voronoi cell without neighbor information.
///
/// This class is an extension of the voronoicell_base class, in cases when
/// is not necessary to track the IDs of neighboring particles associated
/// with each face of the Voronoi cell.
pub struct VoroCellSgl {
    pub(crate) inner: UniquePtr<ffi::voronoicell>,
}

impl VoroCellSgl {
    /// Initializes the Voronoi cell to be rectangular box with the
    /// given dimensions.
    ///
    /// * `xyz_min`: the minimum xyz coordinates.
    /// * `xyz_max`: the maximum xyz coordinates.
    pub fn new(xyz_min: DVec3, xyz_max: DVec3) -> Self {
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
        xyz0: DVec3,
        xyz1: DVec3,
        xyz2: DVec3,
        xyz3: DVec3,
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

impl Clone for VoroCellSgl {
    fn clone(&self) -> Self {
        Self {
            inner: ffi::clone_voronoicell(&self.inner),
        }
    }
}

impl VoroCell for VoroCellSgl {
    fn translate(&mut self, xyz: DVec3) {
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

    fn centroid(&mut self) -> DVec3 {
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

    fn vertices_local(&mut self) -> Vec<f64> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().vertices_local(v.pin_mut());
        v.into_iter().copied().collect()
    }

    fn vertices_global(&mut self, xyz: DVec3) -> Vec<f64> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().vertices_global(
            xyz[0],
            xyz[1],
            xyz[2],
            v.pin_mut(),
        );
        v.into_iter().copied().collect()
    }

    fn face_areas(&mut self) -> Vec<f64> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().face_areas(v.pin_mut());
        v.into_iter().copied().collect()
    }

    fn face_orders(&mut self) -> Vec<i32> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().face_orders(v.pin_mut());
        v.into_iter().copied().collect()
    }

    fn face_freq_table(&mut self) -> Vec<i32> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().face_freq_table(v.pin_mut());
        v.into_iter().copied().collect()
    }

    fn face_vertices(&mut self) -> Vec<i32> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().face_vertices(v.pin_mut());
        v.into_iter().copied().collect()
    }

    fn face_perimeters(&mut self) -> Vec<f64> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().face_perimeters(v.pin_mut());
        v.into_iter().copied().collect()
    }

    fn normals(&mut self) -> Vec<f64> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().normals(v.pin_mut());
        v.into_iter().copied().collect()
    }

    fn plane_intersects(
        &mut self,
        xyz: DVec3,
        rsq: f64,
    ) -> bool {
        self.inner
            .pin_mut()
            .plane_intersects(xyz[0], xyz[1], xyz[2], rsq)
    }

    fn plane_intersects_guess(
        &mut self,
        xyz: DVec3,
        rsq: f64,
    ) -> bool {
        self.inner.pin_mut().plane_intersects_guess(
            xyz[0], xyz[1], xyz[2], rsq,
        )
    }

    fn nplane_rsq(
        &mut self,
        xyz: DVec3,
        rsq: f64,
        p_id: i32,
    ) -> bool {
        self.inner
            .pin_mut()
            .nplane_rsq(xyz[0], xyz[1], xyz[2], rsq, p_id)
    }

    fn nplane(&mut self, xyz: DVec3, p_id: i32) -> bool {
        self.inner
            .pin_mut()
            .nplane(xyz[0], xyz[1], xyz[2], p_id)
    }

    fn plane_rsq(&mut self, xyz: DVec3, rsq: f64) -> bool {
        self.inner
            .pin_mut()
            .plane_rsq(xyz[0], xyz[1], xyz[2], rsq)
    }

    fn plane(&mut self, xyz: DVec3) -> bool {
        self.inner.pin_mut().plane(xyz[0], xyz[1], xyz[2])
    }
}

impl<'a> Into<VoroCellMut<'a>> for &'a mut VoroCellSgl {
    fn into(self) -> VoroCellMut<'a> {
        VoroCellMut::Sgl(self)
    }
}

/// `voronoicell_neighbor` class in voro++.
/// A class to represent a Voronoi cell with neighbor information, in cases when the
///  IDs of neighboring particles associated with each face of the Voronoi cell.
///  It contains additional data structures for storing this
///  information.
pub struct VoroCellNbr {
    pub(crate) inner: UniquePtr<ffi::voronoicell_neighbor>,
}

impl VoroCellNbr {
    /// Initializes the Voronoi cell to be rectangular box with the
    /// given dimensions.
    ///
    /// * `xyz_min`: the minimum xyz coordinates.
    /// * `xyz_max`: the maximum xyz coordinates.
    pub fn new(xyz_min: DVec3, xyz_max: DVec3) -> Self {
        let mut val = Self {
            inner: ffi::new_voronoicell_neighbor(),
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
            inner: ffi::new_voronoicell_neighbor(),
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
        xyz0: DVec3,
        xyz1: DVec3,
        xyz2: DVec3,
        xyz3: DVec3,
    ) -> Self {
        let mut val = Self {
            inner: ffi::new_voronoicell_neighbor(),
        };
        val.inner.pin_mut().init_tetrahedron_base(
            xyz0[0], xyz0[1], xyz0[2], xyz1[0], xyz1[1],
            xyz1[2], xyz2[0], xyz2[1], xyz2[2], xyz3[0],
            xyz3[1], xyz3[2],
        );
        val
    }

    pub fn neighbors(&mut self) -> Vec<i32> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().neighbors(v.pin_mut());
        v.into_iter().copied().collect()
    }
}

impl Clone for VoroCellNbr {
    fn clone(&self) -> Self {
        Self {
            inner: ffi::clone_voronoicell_neighbor(
                &self.inner,
            ),
        }
    }
}

impl VoroCell for VoroCellNbr {
    fn translate(&mut self, xyz: DVec3) {
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

    fn centroid(&mut self) -> DVec3 {
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

    fn vertices_local(&mut self) -> Vec<f64> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().vertices_local(v.pin_mut());
        v.into_iter().copied().collect()
    }

    fn vertices_global(&mut self, xyz: DVec3) -> Vec<f64> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().vertices_global(
            xyz[0],
            xyz[1],
            xyz[2],
            v.pin_mut(),
        );
        v.into_iter().copied().collect()
    }

    fn face_areas(&mut self) -> Vec<f64> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().face_areas(v.pin_mut());
        v.into_iter().copied().collect()
    }

    fn face_orders(&mut self) -> Vec<i32> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().face_orders(v.pin_mut());
        v.into_iter().copied().collect()
    }

    fn face_freq_table(&mut self) -> Vec<i32> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().face_freq_table(v.pin_mut());
        v.into_iter().copied().collect()
    }

    fn face_vertices(&mut self) -> Vec<i32> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().face_vertices(v.pin_mut());
        v.into_iter().copied().collect()
    }

    fn face_perimeters(&mut self) -> Vec<f64> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().face_perimeters(v.pin_mut());
        v.into_iter().copied().collect()
    }

    fn normals(&mut self) -> Vec<f64> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().normals(v.pin_mut());
        v.into_iter().copied().collect()
    }

    fn plane_intersects(
        &mut self,
        xyz: DVec3,
        rsq: f64,
    ) -> bool {
        self.inner
            .pin_mut()
            .plane_intersects(xyz[0], xyz[1], xyz[2], rsq)
    }

    fn plane_intersects_guess(
        &mut self,
        xyz: DVec3,
        rsq: f64,
    ) -> bool {
        self.inner.pin_mut().plane_intersects_guess(
            xyz[0], xyz[1], xyz[2], rsq,
        )
    }

    fn nplane_rsq(
        &mut self,
        xyz: DVec3,
        rsq: f64,
        p_id: i32,
    ) -> bool {
        self.inner
            .pin_mut()
            .nplane_rsq(xyz[0], xyz[1], xyz[2], rsq, p_id)
    }

    fn nplane(&mut self, xyz: DVec3, p_id: i32) -> bool {
        self.inner
            .pin_mut()
            .nplane(xyz[0], xyz[1], xyz[2], p_id)
    }

    fn plane_rsq(&mut self, xyz: DVec3, rsq: f64) -> bool {
        self.inner
            .pin_mut()
            .plane_rsq(xyz[0], xyz[1], xyz[2], rsq)
    }

    fn plane(&mut self, xyz: DVec3) -> bool {
        self.inner.pin_mut().plane(xyz[0], xyz[1], xyz[2])
    }
}

impl<'a> Into<VoroCellMut<'a>> for &'a mut VoroCellNbr {
    fn into(self) -> VoroCellMut<'a> {
        VoroCellMut::Nbr(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut c0 = VoroCellSgl::new(
            [0.0, 1.0, 1.0],
            [2.0, 2.0, 2.0],
        );
        assert_eq!(c0.volume(), 2.0);
        assert_eq!(c0.volume(), 2.0);
        let mut c1 = c0.clone();
        assert_eq!(c0.volume(), 2.0);
        assert_eq!(c1.volume(), 2.0);

        let mut c2 = VoroCellNbr::new(
            [1.0, 1.0, 1.0],
            [2.0, 2.0, 4.0],
        );
        assert_eq!(c2.volume(), 3.0);
        assert_eq!(c2.volume(), 3.0);
        let mut c3 = c2.clone();
        assert_eq!(c2.volume(), 3.0);
        assert_eq!(c3.volume(), 3.0);

        assert_eq!(c0.centroid(), [1.0, 1.5, 1.5]);
    }
}
