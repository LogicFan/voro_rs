//! Wall and related classes.

#[cxx::bridge(namespace = "voro")]
pub mod ffi {
    unsafe extern "C++" {
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
        #[rust_name = "clone_wall_sphere"]
        fn clone_wall(
            value: &UniquePtr<wall_sphere>,
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
        #[rust_name = "clone_wall_plane"]
        fn clone_wall(
            value: &UniquePtr<wall_plane>,
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
        #[rust_name = "clone_wall_cylinder"]
        fn clone_wall(
            value: &UniquePtr<wall_cylinder>,
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
        #[rust_name = "clone_wall_cone"]
        fn clone_wall(
            value: &UniquePtr<wall_cone>,
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

        type wall;
        fn wall_sphere_to_wall<'a>(
            w: Pin<&'a mut wall_sphere>,
        ) -> Pin<&'a mut wall>;
        fn wall_plane_to_wall<'a>(
            w: Pin<&'a mut wall_plane>,
        ) -> Pin<&'a mut wall>;
        fn wall_cylinder_to_wall<'a>(
            w: Pin<&'a mut wall_cylinder>,
        ) -> Pin<&'a mut wall>;
        fn wall_cone_to_wall<'a>(
            w: Pin<&'a mut wall_cone>,
        ) -> Pin<&'a mut wall>;
    }
}

use crate::cell::bridge::VoroCellMut;
use cxx::UniquePtr;

type DVec3 = [f64; 3];

/// `wall` abstract class in voro++.
///
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

/// A enum to store mutable reference of any `Wall`. This is
/// to mimic the override in C++.
pub enum WallMut<'a> {
    Sphere(&'a mut WallSphere),
    Plane(&'a mut WallPlane),
    Cylinder(&'a mut WallCylinder),
    Cone(&'a mut WallCone),
}

/// `wall_sphere` class in voro++.
///
/// A class representing a spherical wall object.
pub struct WallSphere {
    pub(crate) inner: UniquePtr<ffi::wall_sphere>,
}

impl WallSphere {
    /// Constructs a spherical wall object.
    ///
    /// * `c`: a position vector for the sphere's center.
    /// * `r`: the radius of the sphere.
    pub fn new(c: DVec3, r: f64) -> Self {
        Self::new_with_id(c, r, -99)
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

impl Clone for WallSphere {
    fn clone(&self) -> Self {
        Self {
            inner: ffi::clone_wall_sphere(&self.inner),
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
            VoroCellMut::Sgl(c) => {
                self.inner.pin_mut().cut_cell_0(
                    c.inner.pin_mut(),
                    xyz[0],
                    xyz[1],
                    xyz[2],
                )
            }
            VoroCellMut::Nbr(c) => {
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

impl<'a> Into<WallMut<'a>> for &'a mut WallSphere {
    fn into(self) -> WallMut<'a> {
        WallMut::Sphere(self)
    }
}

/// `wall_plane` class in voro++.
///
/// A class representing a plane wall object.
pub struct WallPlane {
    pub(crate) inner: UniquePtr<ffi::wall_plane>,
}

impl WallPlane {
    /// Constructs a plane wall object.
    ///
    /// * `c`: a normal vector to the plane.
    /// * `a`: a displacement along the normal vector.
    pub fn new(c: DVec3, a: f64) -> Self {
        Self::new_with_id(c, a, -99)
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

impl Clone for WallPlane {
    fn clone(&self) -> Self {
        Self {
            inner: ffi::clone_wall_plane(&self.inner),
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
            VoroCellMut::Sgl(c) => {
                self.inner.pin_mut().cut_cell_0(
                    c.inner.pin_mut(),
                    xyz[0],
                    xyz[1],
                    xyz[2],
                )
            }
            VoroCellMut::Nbr(c) => {
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

impl<'a> Into<WallMut<'a>> for &'a mut WallPlane {
    fn into(self) -> WallMut<'a> {
        WallMut::Plane(self)
    }
}

/// `wall_cylinder` class in voro++.
///
/// A class representing a cylindrical wall object.
pub struct WallCylinder {
    pub(crate) inner: UniquePtr<ffi::wall_cylinder>,
}

impl WallCylinder {
    /// Constructs a cylinder wall object.
    ///
    /// * `c`: a point on the axis of the cylinder.
    /// * `a`: a vector pointing along the direction of the cylinder.
    /// * `r`: the radius of the cylinder.
    pub fn new(c: DVec3, a: DVec3, r: f64) -> Self {
        Self::new_with_id(c, a, r, -99)
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

impl Clone for WallCylinder {
    fn clone(&self) -> Self {
        Self {
            inner: ffi::clone_wall_cylinder(&self.inner),
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
            VoroCellMut::Sgl(c) => {
                self.inner.pin_mut().cut_cell_0(
                    c.inner.pin_mut(),
                    xyz[0],
                    xyz[1],
                    xyz[2],
                )
            }
            VoroCellMut::Nbr(c) => {
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

impl<'a> Into<WallMut<'a>> for &'a mut WallCylinder {
    fn into(self) -> WallMut<'a> {
        WallMut::Cylinder(self)
    }
}

/// `wall_cone` class in voro++.
///
/// A class representing a conical wall object.
pub struct WallCone {
    pub(crate) inner: UniquePtr<ffi::wall_cone>,
}

impl WallCone {
    /// Constructs a cone wall object.
    ///
    /// * `c`: the apex of the cone.
    /// * `a`: a vector pointing along the axis of the cone.
    /// * `ang`: the angle (in radians) of the cone, measured from the axis.
    pub fn new(c: DVec3, a: DVec3, ang: f64) -> Self {
        Self::new_with_id(c, a, ang, -99)
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

impl Clone for WallCone {
    fn clone(&self) -> Self {
        Self {
            inner: ffi::clone_wall_cone(&self.inner),
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
            VoroCellMut::Sgl(c) => {
                self.inner.pin_mut().cut_cell_0(
                    c.inner.pin_mut(),
                    xyz[0],
                    xyz[1],
                    xyz[2],
                )
            }
            VoroCellMut::Nbr(c) => {
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

impl<'a> Into<WallMut<'a>> for &'a mut WallCone {
    fn into(self) -> WallMut<'a> {
        WallMut::Cone(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{VoroCell, VoroCellSgl};

    #[test]
    fn basic_test() {
        let mut w0 =
            WallSphere::new([10.0, 0.0, 0.0], 10.0);
        let mut c0 = VoroCellSgl::new(
            [-1.0, -1.0, -1.0],
            [1.0, 1.0, 1.0],
        );

        assert_eq!(c0.volume(), 8.0);

        let mut c1 = c0.clone();
        assert_eq!(c1.volume(), 8.0);

        let b = w0.cut_cell(&mut c1, [0.0, 0.0, 0.0]);
        assert_eq!(c0.volume(), 8.0);
        assert_eq!(c1.volume(), 4.0);
        assert_eq!(b, true);

        let mut w1 = w0.clone();
        let c = w1.cut_cell(&mut c0, [0.0, 0.0, 0.0]);
        assert_eq!(c0.volume(), 4.0);
        assert_eq!(c, true);
    }
}
