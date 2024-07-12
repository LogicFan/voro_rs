//! Walls trait and related classes.

#[cxx::bridge(namespace = "voro")]
pub mod ffi {
    unsafe extern "C++" {
        include!("voro_rs/src/boilerplate.hh");

        type voronoicell = crate::cell::ffi::voronoicell;
        type voronoicell_neighbor =
            crate::cell::ffi::voronoicell_neighbor;

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

use crate::cell::{VoroCell, VoroCellNbr, VoroCellSgl};
use crate::container::ffi::{
    container_poly_to_wall_list, container_to_wall_list,
};
use crate::container::{ContainerRad, ContainerStd};
use crate::wall::ffi::{
    wall_cone_to_wall, wall_cylinder_to_wall,
    wall_plane_to_wall, wall_sphere_to_wall,
};
use crate::wall::{
    Wall, WallCone, WallCylinder, WallPlane, WallSphere,
};
use cxx::UniquePtr;
use std::marker::PhantomData;

type DVec3 = [f64; 3];

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
    /// Create an empty `WallList`
    pub fn new() -> Self {
        Self {
            inner: ffi::new_wall_list(),
            phantom: PhantomData,
        }
    }
}

/// A part of trait `Wall` whose parameter does not depends any type.
pub trait Walls0 {
    /// Determines whether a given position is inside all of the
    /// walls on the list.
    ///
    /// * `xyz`: the position to test.
    ///
    /// Return true if it is inside, false if it is outside.
    fn point_inside_walls(&mut self, xyz: DVec3) -> bool;
}

impl<'a> Walls0 for WallList<'a> {
    fn point_inside_walls(&mut self, xyz: DVec3) -> bool {
        self.inner
            .pin_mut()
            .point_inside_walls(xyz[0], xyz[1], xyz[2])
    }
}

/// A part of trait `Wall` whose parameter depends cell type.
pub trait Walls1<T: VoroCell> {
    /// Cuts a Voronoi cell by all of the walls currently on the list.
    ///
    /// * `cell`: a reference to the Voronoi cell class.
    /// * `xyz`: the position of the cell.
    ///
    /// Return true if the cell still exists, false if the cell is
    /// deleted.
    fn apply_walls(
        &mut self,
        cell: &mut T,
        xyz: DVec3,
    ) -> bool;
}

impl<'a> Walls1<VoroCellSgl> for WallList<'a> {
    fn apply_walls(
        &mut self,
        cell: &mut VoroCellSgl,
        xyz: DVec3,
    ) -> bool {
        self.inner.pin_mut().apply_walls_0(
            cell.inner.pin_mut(),
            xyz[0],
            xyz[1],
            xyz[2],
        )
    }
}

impl<'a> Walls1<VoroCellNbr> for WallList<'a> {
    fn apply_walls(
        &mut self,
        cell: &mut VoroCellNbr,
        xyz: DVec3,
    ) -> bool {
        self.inner.pin_mut().apply_walls_1(
            cell.inner.pin_mut(),
            xyz[0],
            xyz[1],
            xyz[2],
        )
    }
}

/// A part of trait `Wall` whose parameter depends wall type.
pub trait Walls2<'a, T: Wall> {
    /// Adds a wall to the list.
    ///
    /// * `wall`: a reference to the wall to add. Since this method will not
    /// take ownership nor copy the wall, the `wall`
    /// need to outlive the struct holding it.
    fn add_wall(&mut self, wall: &'a mut T);
}

impl<'a> Walls2<'a, WallSphere> for WallList<'a> {
    fn add_wall(&mut self, wall: &'a mut WallSphere) {
        let w0 = wall_sphere_to_wall(wall.inner.pin_mut());
        unsafe {
            // ensure the lifetime of `self` is within the lifetime of
            // `wall` using the lifetime specifier `'a`.
            self.inner.pin_mut().add_wall(w0);
        }
    }
}

impl<'a> Walls2<'a, WallPlane> for WallList<'a> {
    fn add_wall(&mut self, wall: &'a mut WallPlane) {
        let w0 = wall_plane_to_wall(wall.inner.pin_mut());
        unsafe {
            // ensure the lifetime of `self` is within the lifetime of
            // `wall` using the lifetime specifier `'a`.
            self.inner.pin_mut().add_wall(w0);
        }
    }
}

impl<'a> Walls2<'a, WallCylinder> for WallList<'a> {
    fn add_wall(&mut self, wall: &'a mut WallCylinder) {
        let w0 =
            wall_cylinder_to_wall(wall.inner.pin_mut());
        unsafe {
            // ensure the lifetime of `self` is within the lifetime of
            // `wall` using the lifetime specifier `'a`.
            self.inner.pin_mut().add_wall(w0);
        }
    }
}

impl<'a> Walls2<'a, WallCone> for WallList<'a> {
    fn add_wall(&mut self, wall: &'a mut WallCone) {
        let w0 = wall_cone_to_wall(wall.inner.pin_mut());
        unsafe {
            // ensure the lifetime of `self` is within the lifetime of
            // `wall` using the lifetime specifier `'a`.
            self.inner.pin_mut().add_wall(w0);
        }
    }
}

/// A part of trait `Wall` whose parameter depends walls type.
pub trait Walls3<'a, T: Walls<'a>> {
    /// Adds all of the walls on another wall_list to this class.
    ///
    /// * `walls`: a reference to the `Walls` struct. Since this method
    /// will not take ownership nor copy the wall, the `walls` need to
    /// outlive the struct holding it.
    fn add_walls(&mut self, walls: &mut T);
}

impl<'a> Walls3<'a, WallList<'a>> for WallList<'a> {
    fn add_walls(&mut self, walls: &mut WallList<'a>) {
        unsafe {
            // ensure the lifetime of `self` is within the lifetime of
            // `wall` using the lifetime specifier `'a`.self
            self.inner
                .pin_mut()
                .add_walls(walls.inner.pin_mut())
        }
    }
}

impl<'a> Walls3<'a, ContainerStd<'a>> for WallList<'a> {
    fn add_walls(&mut self, walls: &mut ContainerStd<'a>) {
        unsafe {
            // ensure the lifetime of `self` is within the lifetime of
            // `wall` using the lifetime specifier `'a`.self
            self.inner.pin_mut().add_walls(
                container_to_wall_list(
                    walls.inner.pin_mut(),
                ),
            )
        }
    }
}

impl<'a> Walls3<'a, ContainerRad<'a>> for WallList<'a> {
    fn add_walls(&mut self, walls: &mut ContainerRad<'a>) {
        unsafe {
            // ensure the lifetime of `self` is within the lifetime of
            // `wall` using the lifetime specifier `'a`.self
            self.inner.pin_mut().add_walls(
                container_poly_to_wall_list(
                    walls.inner.pin_mut(),
                ),
            )
        }
    }
}

/// The interface of `wall_list` class in voro++.
///
/// This trait contains several simple routines that make use of the wall
/// classes (such as telling whether a
/// given position is inside all of the walls or not).
pub trait Walls<'a>:
    Walls0
    + Walls1<VoroCellSgl>
    + Walls1<VoroCellNbr>
    + Walls2<'a, WallSphere>
    + Walls2<'a, WallPlane>
    + Walls2<'a, WallCylinder>
    + Walls2<'a, WallCone>
    + Walls3<'a, WallList<'a>>
    + Walls3<'a, ContainerStd<'a>>
    + Walls3<'a, ContainerRad<'a>>
{
}

impl<'a> Walls<'a> for WallList<'a> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cell::{VoroCell, VoroCellSgl},
        wall::{Wall1, WallSphere},
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
