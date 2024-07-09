//! Wall and related classes.

#[cxx::bridge(namespace = "voro")]
pub mod ffi {
    unsafe extern "C++" {
        include!("voro_rs/cpp/voro++.hh");
        include!("voro_rs/cpp/boilerplate.hh");

        type voronoicell = crate::cell::ffi::voronoicell;
        type voronoicell_neighbor =
            crate::cell::ffi::voronoicell_neighbor;

        type wall_sphere;
        #[rust_name = "new_wall_sphere"]
        fn construct(
            xc_: f64,
            yc_: f64,
            zc_: f64,
            rc_: f64,
            w_id_: i32,
        ) -> UniquePtr<wall_sphere>;
        fn point_inside(
            self: Pin<&mut wall_sphere>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "cut_cell_0"]
        fn cut_cell(
            self: Pin<&mut wall_sphere>,
            c: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "cut_cell_1"]
        fn cut_cell(
            self: Pin<&mut wall_sphere>,
            c: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;

        type wall_plane;
        #[rust_name = "new_wall_plane"]
        fn construct(
            xc_: f64,
            yc_: f64,
            zc_: f64,
            rc_: f64,
            w_id_: i32,
        ) -> UniquePtr<wall_plane>;
        fn point_inside(
            self: Pin<&mut wall_plane>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "cut_cell_0"]
        fn cut_cell(
            self: Pin<&mut wall_plane>,
            c: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "cut_cell_1"]
        fn cut_cell(
            self: Pin<&mut wall_plane>,
            c: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;

        type wall_cylinder;
        #[rust_name = "new_wall_cylinder"]
        fn construct(
            xc_: f64,
            yc_: f64,
            zc_: f64,
            xa_: f64,
            ya_: f64,
            za_: f64,
            rc_: f64,
            w_id_: i32,
        ) -> UniquePtr<wall_cylinder>;
        fn point_inside(
            self: Pin<&mut wall_cylinder>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "cut_cell_0"]
        fn cut_cell(
            self: Pin<&mut wall_cylinder>,
            c: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "cut_cell_1"]
        fn cut_cell(
            self: Pin<&mut wall_cylinder>,
            c: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;

        type wall_cone;
        #[rust_name = "new_wall_cone"]
        fn construct(
            xc_: f64,
            yc_: f64,
            zc_: f64,
            xa_: f64,
            ya_: f64,
            za_: f64,
            ang: f64,
            w_id_: i32,
        ) -> UniquePtr<wall_cone>;
        fn point_inside(
            self: Pin<&mut wall_cone>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "cut_cell_0"]
        fn cut_cell(
            self: Pin<&mut wall_cone>,
            c: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "cut_cell_1"]
        fn cut_cell(
            self: Pin<&mut wall_cone>,
            c: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
    }
}

use crate::cell::VoroCellMut;
use cxx::UniquePtr;

type DVec3 = [f64; 3];

/// This is a trait for a generic wall object. A wall object
/// can be specified by deriving a new struct from this and specifying the
/// functions.
pub trait Wall {
    /// Tests to see whether a point is inside the sphere wall object.
    ///
    /// * `x,y,z`: the vector to test.
    ///
    /// Return true if the point is inside, false if the point is outside.
    fn point_inside(&mut self, xyz: DVec3) -> bool;

    /// Cuts a cell by the sphere wall object. The spherical wall is approximated by
    /// a single plane applied at the point on the sphere which is closest to the center
    /// of the cell. This works well for particle arrangements that are packed against
    /// the wall, but loses accuracy for sparse particle distributions.
    ///
    /// * `cell`: the Voronoi cell to be cut.
    /// * `xyz`: the location of the Voronoi cell.
    ///
    /// Return true if the cell still exists, false if the cell is deleted.
    fn cut_cell<'a>(
        &mut self,
        cell: impl Into<VoroCellMut<'a>>,
        xyz: DVec3,
    ) -> bool;
}

/// A class representing a spherical wall object.
pub struct WallSphere {
    inner: UniquePtr<ffi::wall_sphere>,
}

impl WallSphere {
    /// Constructs a spherical wall object.
    ///
    /// * `c`: a position vector for the sphere's center.
    /// * `r`: the radius of the sphere.
    pub fn new(c: DVec3, r: f64) -> Self {
        Self {
            inner: ffi::new_wall_sphere(
                c[0], c[1], c[2], r, -99,
            ),
        }
    }

    /// Constructs a spherical wall object.
    ///
    /// * `c`: a position vector for the sphere's center.
    /// * `r`: the radius of the sphere.
    /// * `id`: an ID number to associate with the wall for neighbor tracking.
    pub fn new_with_id(c: DVec3, r: f64, id: i32) -> Self {
        Self {
            inner: ffi::new_wall_sphere(
                c[0], c[1], c[2], r, id,
            ),
        }
    }
}

impl Wall for WallSphere {
    fn point_inside(&mut self, xyz: DVec3) -> bool {
        self.inner
            .pin_mut()
            .point_inside(xyz[0], xyz[1], xyz[2])
    }

    fn cut_cell<'a>(
        &mut self,
        cell: impl Into<VoroCellMut<'a>>,
        xyz: DVec3,
    ) -> bool {
        match cell.into() {
            VoroCellMut::Standalone(c) => {
                self.inner.pin_mut().cut_cell_0(
                    c.inner.pin_mut(),
                    xyz[0],
                    xyz[1],
                    xyz[2],
                )
            }
            VoroCellMut::WithNeighbor(c) => {
                self.inner.pin_mut().cut_cell_1(
                    c.inner.pin_mut(),
                    xyz[0],
                    xyz[1],
                    xyz[2],
                )
            }
        }
    }
}

/// A class representing a plane wall object.
pub struct WallPlane {
    inner: UniquePtr<ffi::wall_plane>,
}

impl WallPlane {
    /// Constructs a plane wall object.
    ///
    /// * `c`: a normal vector to the plane.
    /// * `a`: a displacement along the normal vector.
    pub fn new(c: DVec3, a: f64) -> Self {
        Self {
            inner: ffi::new_wall_plane(
                c[0], c[1], c[2], a, -99,
            ),
        }
    }

    /// Constructs a plane wall object.
    ///
    /// * `c`: a normal vector to the plane.
    /// * `a`: a displacement along the normal vector.
    /// * `id`: an ID number to associate with the wall for neighbor tracking.
    pub fn new_with_id(c: DVec3, a: f64, id: i32) -> Self {
        Self {
            inner: ffi::new_wall_plane(
                c[0], c[1], c[2], a, id,
            ),
        }
    }
}

impl Wall for WallPlane {
    fn point_inside(&mut self, xyz: DVec3) -> bool {
        self.inner
            .pin_mut()
            .point_inside(xyz[0], xyz[1], xyz[2])
    }

    fn cut_cell<'a>(
        &mut self,
        cell: impl Into<VoroCellMut<'a>>,
        xyz: DVec3,
    ) -> bool {
        match cell.into() {
            VoroCellMut::Standalone(c) => {
                self.inner.pin_mut().cut_cell_0(
                    c.inner.pin_mut(),
                    xyz[0],
                    xyz[1],
                    xyz[2],
                )
            }
            VoroCellMut::WithNeighbor(c) => {
                self.inner.pin_mut().cut_cell_1(
                    c.inner.pin_mut(),
                    xyz[0],
                    xyz[1],
                    xyz[2],
                )
            }
        }
    }
}

/// A class representing a cylindrical wall object.
pub struct WallCylinder {
    inner: UniquePtr<ffi::wall_cylinder>,
}

impl WallCylinder {
    /// Constructs a cylinder wall object.
    ///
    /// * `c`: a point on the axis of the cylinder.
    /// * `a`: a vector pointing along the direction of the cylinder.
    /// * `r`: the radius of the cylinder.
    pub fn new(c: DVec3, a: DVec3, r: f64) -> Self {
        Self {
            inner: ffi::new_wall_cylinder(
                c[0], c[1], c[2], a[0], a[1], a[2], r, -99,
            ),
        }
    }

    /// Constructs a cylinder wall object.
    ///
    /// * `c`: a point on the axis of the cylinder.
    /// * `a`: a vector pointing along the direction of the cylinder.
    /// * `r`: the radius of the cylinder.
    /// * `id`: an ID number to associate with the wall for neighbor tracking.
    pub fn new_with_id(
        c: DVec3,
        a: DVec3,
        r: f64,
        id: i32,
    ) -> Self {
        Self {
            inner: ffi::new_wall_cylinder(
                c[0], c[1], c[2], a[0], a[1], a[2], r, id,
            ),
        }
    }
}

impl Wall for WallCylinder {
    fn point_inside(&mut self, xyz: DVec3) -> bool {
        self.inner
            .pin_mut()
            .point_inside(xyz[0], xyz[1], xyz[2])
    }

    fn cut_cell<'a>(
        &mut self,
        cell: impl Into<VoroCellMut<'a>>,
        xyz: DVec3,
    ) -> bool {
        match cell.into() {
            VoroCellMut::Standalone(c) => {
                self.inner.pin_mut().cut_cell_0(
                    c.inner.pin_mut(),
                    xyz[0],
                    xyz[1],
                    xyz[2],
                )
            }
            VoroCellMut::WithNeighbor(c) => {
                self.inner.pin_mut().cut_cell_1(
                    c.inner.pin_mut(),
                    xyz[0],
                    xyz[1],
                    xyz[2],
                )
            }
        }
    }
}

/// A class representing a conical wall object.
pub struct WallCone {
    inner: UniquePtr<ffi::wall_cone>,
}

impl WallCone {
    /// Constructs a cone wall object.
    ///
    /// * `c`: the apex of the cone.
    /// * `a`: a vector pointing along the axis of the cone.
    /// * `ang`: the angle (in radians) of the cone, measured from the axis.
    pub fn new(c: DVec3, a: DVec3, ang: f64) -> Self {
        Self {
            inner: ffi::new_wall_cone(
                c[0], c[1], c[2], a[0], a[1], a[2], ang,
                -99,
            ),
        }
    }

    /// Constructs a cone wall object.
    ///
    /// * `c`: the apex of the cone.
    /// * `a`: a vector pointing along the axis of the cone.
    /// * `ang`: the angle (in radians) of the cone, measured from the axis.
    /// * `id`: an ID number to associate with the wall for neighbor tracking.
    pub fn new_with_id(
        c: DVec3,
        a: DVec3,
        ang: f64,
        id: i32,
    ) -> Self {
        Self {
            inner: ffi::new_wall_cone(
                c[0], c[1], c[2], a[0], a[1], a[2], ang, id,
            ),
        }
    }
}

impl Wall for WallCone {
    fn point_inside(&mut self, xyz: DVec3) -> bool {
        self.inner
            .pin_mut()
            .point_inside(xyz[0], xyz[1], xyz[2])
    }

    fn cut_cell<'a>(
        &mut self,
        cell: impl Into<VoroCellMut<'a>>,
        xyz: DVec3,
    ) -> bool {
        match cell.into() {
            VoroCellMut::Standalone(c) => {
                self.inner.pin_mut().cut_cell_0(
                    c.inner.pin_mut(),
                    xyz[0],
                    xyz[1],
                    xyz[2],
                )
            }
            VoroCellMut::WithNeighbor(c) => {
                self.inner.pin_mut().cut_cell_1(
                    c.inner.pin_mut(),
                    xyz[0],
                    xyz[1],
                    xyz[2],
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::VoroCellSgl;

    #[test]
    fn basic_test() {
        let mut w = WallSphere::new([0.0, 0.0, 0.0], 1.0);
        let mut c = VoroCellSgl::new(
            [1.0, 1.0, 1.0],
            [2.0, 2.0, 2.0],
        );

        let _ = w.cut_cell(&mut c, [0.0, 0.0, 0.0]);
    }
}
