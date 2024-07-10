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

use crate::wall::ffi::{
    wall_cone_to_wall, wall_cylinder_to_wall,
    wall_plane_to_wall, wall_sphere_to_wall,
};
use crate::wall::WallMut;
use cxx::UniquePtr;
use std::marker::PhantomData;

pub trait Walls<'a> {
    fn add_wall(&mut self, wall: impl Into<WallMut<'a>>);
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
            // `wall` based on the lifetime specifier `'a`.
            self.inner.pin_mut().add_wall(w0);
        }
    }
}

// TODO: add_wall and add_walls need to ensure lifetime.
