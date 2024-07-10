//! Walls trait and related classes.

#[cxx::bridge(namespace = "voro")]
pub mod ffi {
    unsafe extern "C++" {
        include!("voro_rs/cpp/boilerplate.hh");

        type voronoicell = crate::cell::ffi::voronoicell;
        type voronoicell_neighbor =
            crate::cell::ffi::voronoicell_neighbor;

        type wall_sphere = crate::wall::ffi::wall_sphere;
        type wall_plane = crate::wall::ffi::wall_plane;
        type wall_cylinder =
            crate::wall::ffi::wall_cylinder;
        type wall_cone = crate::wall::ffi::wall_cone;
        type wall = crate::wall::ffi::wall;

        type wall_list;
        #[rust_name = "new_wall_list"]
        fn construct() -> UniquePtr<wall_list>;
        unsafe fn add_wall(
            self: Pin<&mut wall_list>,
            w: Pin<&mut wall>,
        );
        #[rust_name = "add_walls"]
        unsafe fn add_wall(
            self: Pin<&mut wall_list>,
            w: Pin<&mut wall_list>,
        );
        fn point_inside_walls(
            self: Pin<&mut wall_list>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "apply_walls_0"]
        fn apply_walls(
            self: Pin<&mut wall_list>,
            c: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "apply_walls_1"]
        fn apply_walls(
            self: Pin<&mut wall_list>,
            c: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
    }
}

use crate::cell::VoroCellMut;
use crate::wall::ffi::{
    wall_cone_to_wall, wall_cylinder_to_wall,
    wall_plane_to_wall, wall_sphere_to_wall,
};
use crate::wall::WallMut;
use cxx::UniquePtr;
use std::marker::PhantomData;

type DVec3 = [f64; 3];

/// The interface of `wall_list` class in voro++.
///
/// This trait contains several simple routines that make use of the wall
/// classes (such as telling whether a
/// given position is inside all of the walls or not).
pub trait Walls<'a> {
    /// Adds a wall to the list.
    ///
    /// * `wall`: a reference to the wall to add. Since this method will not
    /// take ownership nor copy the wall, the `wall`
    /// need to outlive the struct holding it.
    fn add_wall(&mut self, wall: impl Into<WallMut<'a>>);

    /// Adds all of the walls on another wall_list to this class.
    ///
    /// * `walls`: a reference to the `Walls` struct. Since this method
    /// will not take ownership nor copy the wall, the `walls` need to
    /// outlive the struct holding it.
    fn add_walls(&mut self, walls: impl Into<WallsMut<'a>>);

    /// Determines whether a given position is inside all of the
    /// walls on the list.
    ///
    /// * `xyz`: the position to test.
    ///
    /// Return true if it is inside, false if it is outside.
    fn inside_walls(&mut self, xyz: DVec3) -> bool;

    /// Cuts a Voronoi cell by all of the walls currently on the list.
    ///
    /// * `cell`: a reference to the Voronoi cell class.
    /// * `xyz`: the position of the cell.
    ///
    /// Return true if the cell still exists, false if the cell is
    /// deleted.
    fn apply_walls<'b>(
        &mut self,
        cell: impl Into<VoroCellMut<'b>>,
        xyz: DVec3,
    ) -> bool;
}

/// A enum to store mutable reference of any `Walls`. This is
/// to mimic the override in C++.
pub enum WallsMut<'a> {
    WallList(&'a mut WallList<'a>),
}

/// `wall_list` class in voro++.
///
/// A class for storing a list of pointers to walls.
///
/// This class does not implement `Clone` trait because there is no
/// well-defined copy constructor in the original voro++ code.
///
/// There is a lifetime specifier because it does not take the ownership
/// of wall inside the list. All walls added into this struct must outlive
/// this struct.
pub struct WallList<'a> {
    pub(crate) inner: UniquePtr<ffi::wall_list>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> WallList<'a> {
    /// Create an empty `WallList```
    pub fn new() -> Self {
        Self {
            inner: ffi::new_wall_list(),
            phantom: PhantomData,
        }
    }
}

impl<'a> Walls<'a> for WallList<'a> {
    fn add_wall(&mut self, wall: impl Into<WallMut<'a>>) {
        let w0 = match wall.into() {
            WallMut::Sphere(w) => {
                wall_sphere_to_wall(w.inner.pin_mut())
            }
            WallMut::Plane(w) => {
                wall_plane_to_wall(w.inner.pin_mut())
            }
            WallMut::Cylinder(w) => {
                wall_cylinder_to_wall(w.inner.pin_mut())
            }
            WallMut::Cone(w) => {
                wall_cone_to_wall(w.inner.pin_mut())
            }
        };
        unsafe {
            // ensure the lifetime of `self` is within the lifetime of
            // `wall` using the lifetime specifier `'a`.
            self.inner.pin_mut().add_wall(w0);
        }
    }

    fn add_walls(
        &mut self,
        walls: impl Into<WallsMut<'a>>,
    ) {
        unsafe {
            // ensure the lifetime of `self` is within the lifetime of
            // `wall` using the lifetime specifier `'a`.
            match walls.into() {
                WallsMut::WallList(w) => self
                    .inner
                    .pin_mut()
                    .add_walls(w.inner.pin_mut()),
            }
        }
    }

    fn inside_walls(&mut self, xyz: DVec3) -> bool {
        self.inner
            .pin_mut()
            .point_inside_walls(xyz[0], xyz[1], xyz[2])
    }

    fn apply_walls<'b>(
        &mut self,
        cell: impl Into<VoroCellMut<'b>>,
        xyz: DVec3,
    ) -> bool {
        match cell.into() {
            VoroCellMut::Sgl(c) => {
                self.inner.pin_mut().apply_walls_0(
                    c.inner.pin_mut(),
                    xyz[0],
                    xyz[1],
                    xyz[2],
                )
            }
            VoroCellMut::Nbr(c) => {
                self.inner.pin_mut().apply_walls_1(
                    c.inner.pin_mut(),
                    xyz[0],
                    xyz[1],
                    xyz[2],
                )
            }
        }
    }
}

impl<'a> Into<WallsMut<'a>> for &'a mut WallList<'a> {
    fn into(self) -> WallsMut<'a> {
        WallsMut::WallList(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        prelude::{VoroCell, VoroCellSgl},
        wall::{Wall, WallSphere},
    };

    #[test]
    fn basic_test() {
        let mut c0 = VoroCellSgl::new(
            [-1.0, -1.0, -1.0],
            [1.0, 1.0, 1.0],
        );
        let mut c1 = c0.clone();
        let mut c2 = c0.clone();
        let mut c3 = c0.clone();
        assert_eq!(c0.volume(), 8.0);
        assert_eq!(c1.volume(), 8.0);
        assert_eq!(c2.volume(), 8.0);
        assert_eq!(c3.volume(), 8.0);

        let mut w0 =
            WallSphere::new([0.0, 0.0, 100.0], 100.0);
        let mut w1 =
            WallSphere::new([0.0, 100.0, 0.0], 100.0);
        w0.cut_cell(&mut c0, [0.0, 0.0, 0.0]);
        assert_eq!(c0.volume(), 4.0);
        w1.cut_cell(&mut c0, [0.0, 0.0, 0.0]);
        assert_eq!(c0.volume(), 2.0);

        let mut wl = WallList::new();
        wl.add_wall(&mut w0);
        wl.apply_walls(&mut c1, [0.0, 0.0, 0.0]);
        assert_eq!(c1.volume(), 4.0);
        wl.add_wall(&mut w1);
        wl.apply_walls(&mut c1, [0.0, 0.0, 0.0]);
        wl.apply_walls(&mut c2, [0.0, 0.0, 0.0]);
        assert_eq!(c1.volume(), 2.0);
        assert_eq!(c2.volume(), 2.0);

        let mut wl2 = WallList::new();
        wl2.add_walls(&mut wl);
        wl2.apply_walls(&mut c3, [0.0, 0.0, 0.0]);
        assert_eq!(c3.volume(), 2.0);
    }
}
