#[cxx::bridge(namespace = "voro")]
pub mod ffi {
    unsafe extern "C++" {
        include!("voro_rs/cpp/boilerplate.hh");

        type voronoicell = crate::cell::ffi::voronoicell;
        type voronoicell_neighbor =
            crate::cell::ffi::voronoicell_neighbor;

        type wall = crate::wall::ffi::wall;
        type wall_list = crate::wall_list::ffi::wall_list;

        type container;
        #[rust_name = "new_container"]
        fn construct(
            ax_: f64,
            bx_: f64,
            ay_: f64,
            by_: f64,
            az_: f64,
            bz_: f64,
            nx_: i32,
            ny_: i32,
            nz_: i32,
            xperiodic_: bool,
            yperiodic_: bool,
            zperiodic_: bool,
            init_mem: i32,
        ) -> UniquePtr<container>;

        unsafe fn add_wall(
            self: Pin<&mut container>,
            w: Pin<&mut wall>,
        );
        #[rust_name = "add_walls"]
        unsafe fn add_wall(
            self: Pin<&mut container>,
            w: Pin<&mut wall_list>,
        );
        fn point_inside_walls(
            self: Pin<&mut container>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "apply_walls_0"]
        fn apply_walls(
            self: Pin<&mut container>,
            c: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "apply_walls_1"]
        fn apply_walls(
            self: Pin<&mut container>,
            c: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;

        fn point_inside(
            self: Pin<&mut container>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        fn total_particles(
            self: Pin<&mut container>,
        ) -> i32;

        fn clear(self: Pin<&mut container>);
        fn put(
            self: Pin<&mut container>,
            n: i32,
            x: f64,
            y: f64,
            z: f64,
        );
        // TODO: put with particle_order
        fn compute_all_cells(self: Pin<&mut container>);
        fn sum_cell_volumes(
            self: Pin<&mut container>,
        ) -> f64;
        fn find_voronoi_cell(
            self: Pin<&mut container>,
            x: f64,
            y: f64,
            z: f64,
            rx: &mut f64,
            ry: &mut f64,
            rz: &mut f64,
            pid: &mut i32,
        ) -> bool;
        // TODO: compute_cell with c_loop
        #[rust_name = "compute_cell_with_index_0"]
        fn compute_cell(
            self: Pin<&mut container>,
            c: Pin<&mut voronoicell>,
            ijk: i32,
            q: i32,
        ) -> bool;
        #[rust_name = "compute_cell_with_index_1"]
        fn compute_cell(
            self: Pin<&mut container>,
            c: Pin<&mut voronoicell_neighbor>,
            ijk: i32,
            q: i32,
        ) -> bool;
        #[rust_name = "compute_ghost_0"]
        fn compute_ghost_cell(
            self: Pin<&mut container>,
            c: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "compute_ghost_1"]
        fn compute_ghost_cell(
            self: Pin<&mut container>,
            c: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
    }
}

use crate::cell::{VoroCellFactory, bridge::VoroCellMut};
use crate::wall::ffi::{
    wall_cone_to_wall, wall_cylinder_to_wall,
    wall_plane_to_wall, wall_sphere_to_wall,
};
use crate::wall::WallMut;
use crate::wall_list::{Walls, WallsMut};
use cxx::UniquePtr;
use std::marker::PhantomData;

type DVec3 = [f64; 3];
type IVec3 = [i32; 3];
type BVec3 = [bool; 3];

pub struct ContainerStd<'a> {
    pub(crate) inner: UniquePtr<ffi::container>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> ContainerStd<'a> {
    pub fn new(
        xyz_min: DVec3,
        xyz_max: DVec3,
        sub_grids: IVec3,
        is_periodic: BVec3,
        init_mem_alloc: i32,
    ) -> Self {
        Self {
            inner: ffi::new_container(
                xyz_min[0],
                xyz_max[0],
                xyz_min[1],
                xyz_max[1],
                xyz_min[2],
                xyz_max[2],
                sub_grids[0],
                sub_grids[1],
                sub_grids[2],
                is_periodic[0],
                is_periodic[1],
                is_periodic[2],
                init_mem_alloc,
            ),
            phantom: PhantomData,
        }
    }
}

impl<'a> Walls<'a> for ContainerStd<'a> {
    fn add_wall(
        &mut self,
        wall: impl Into<WallMut<'a>>,
    ) {
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

    fn point_inside_walls(&mut self, xyz: DVec3) -> bool {
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

pub trait Container {
    fn point_inside(&mut self, xyz: DVec3) -> bool;
    fn total_particles(&mut self) -> i32;

    fn clear(&mut self);
    fn put(&mut self, n: i32, xyz: DVec3);
    fn sum_cell_volumes(&mut self) -> f64;
    fn find_voronoi_cell(
        &mut self,
        xyz: DVec3,
    ) -> Option<(i32, DVec3)>;
    fn compute_cell_with_index<T>(
        &mut self,
        ijk: i32,
        q: i32,
    ) -> Option<T>
    where
        T: From<VoroCellFactory>;
    fn compute_ghost_cell<T>(
        &mut self,
        xyz: DVec3,
    ) -> Option<T>
    where
        T: From<VoroCellFactory>;
}

impl<'a> Container for ContainerStd<'a> {
    fn point_inside(&mut self, xyz: DVec3) -> bool {
        self.inner
            .pin_mut()
            .point_inside(xyz[0], xyz[1], xyz[2])
    }

    fn total_particles(&mut self) -> i32 {
        self.inner.pin_mut().total_particles()
    }

    fn clear(&mut self) {
        self.inner.pin_mut().clear()
    }

    fn put(&mut self, n: i32, xyz: DVec3) {
        self.inner.pin_mut().put(n, xyz[0], xyz[1], xyz[2])
    }

    fn sum_cell_volumes(&mut self) -> f64 {
        self.inner.pin_mut().sum_cell_volumes()
    }

    fn find_voronoi_cell(
        &mut self,
        xyz: DVec3,
    ) -> Option<(i32, DVec3)> {
        let mut pid = 0;
        let mut rx = 0.0;
        let mut ry = 0.0;
        let mut rz = 0.0;
        let b = self.inner.pin_mut().find_voronoi_cell(
            xyz[0], xyz[1], xyz[2], &mut rx, &mut ry,
            &mut rz, &mut pid,
        );
        if b {
            Some((pid, [rx, ry, rz]))
        } else {
            None
        }
    }

    fn compute_cell_with_index<T>(
        &mut self,
        ijk: i32,
        q: i32,
    ) -> Option<T>
    where
        VoroCellFactory: Into<T>,
    {
        todo!()
    }

    fn compute_ghost_cell<T>(
        &mut self,
        xyz: DVec3,
    ) -> Option<T>
    where
        T: From<VoroCellFactory>,
    {
        todo!()
    }
}
