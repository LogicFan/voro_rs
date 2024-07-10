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

pub trait Walls<'a> {
    fn add_wall(&mut self, wall: impl Into<WallMut<'a>>);
    fn add_walls(&mut self, walls: impl Into<WallsMut<'a>>);
    fn apply_walls<'b>(
        &mut self,
        cell: impl Into<VoroCellMut<'b>>,
        xyz: DVec3,
    ) -> bool;
}

pub enum WallsMut<'a> {
    WallList(&'a mut WallList<'a>),
}

pub struct WallList<'a> {
    inner: UniquePtr<ffi::wall_list>,
    phantom: PhantomData<&'a mut ffi::wall_list>,
}

impl<'a> WallList<'a> {
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
    use crate::{prelude::{VoroCell, VoroCellSgl}, wall::{Wall, WallSphere}};
    use super::*;

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

        let mut w0 = WallSphere::new([0.0, 0.0, 100.0], 100.0);
        let mut w1 = WallSphere::new([0.0, 100.0, 0.0], 100.0);
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

