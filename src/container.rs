//!  Container and related classes.

#[cxx::bridge(namespace = "voro")]
pub mod ffi {
    unsafe extern "C++" {
        include!("voro_rs/cpp/boilerplate.hh");

        type voronoicell = crate::cell::ffi::voronoicell;
        type voronoicell_neighbor =
            crate::cell::ffi::voronoicell_neighbor;

        type wall = crate::wall::ffi::wall;
        type wall_list = crate::wall_list::ffi::wall_list;

        type particle_order =
            crate::particle_marker::ffi::particle_order;

        fn container_to_wall_list(
            value: Pin<&mut container>,
        ) -> Pin<&mut wall_list>;
        fn container_poly_to_wall_list(
            value: Pin<&mut container_poly>,
        ) -> Pin<&mut wall_list>;

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
        #[rust_name = "put_with_particle_order"]
        fn put(
            self: Pin<&mut container>,
            vo: Pin<&mut particle_order>,
            n: i32,
            x: f64,
            y: f64,
            z: f64,
        );
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

        type container_poly;
        #[rust_name = "new_container_poly"]
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
        ) -> UniquePtr<container_poly>;

        unsafe fn add_wall(
            self: Pin<&mut container_poly>,
            w: Pin<&mut wall>,
        );
        #[rust_name = "add_walls"]
        unsafe fn add_wall(
            self: Pin<&mut container_poly>,
            w: Pin<&mut wall_list>,
        );
        fn point_inside_walls(
            self: Pin<&mut container_poly>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "apply_walls_0"]
        fn apply_walls(
            self: Pin<&mut container_poly>,
            c: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "apply_walls_1"]
        fn apply_walls(
            self: Pin<&mut container_poly>,
            c: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;

        fn point_inside(
            self: Pin<&mut container_poly>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        fn total_particles(
            self: Pin<&mut container_poly>,
        ) -> i32;

        fn clear(self: Pin<&mut container_poly>);
        fn put(
            self: Pin<&mut container_poly>,
            n: i32,
            x: f64,
            y: f64,
            z: f64,
            r: f64,
        );
        #[rust_name = "put_with_particle_order"]
        fn put(
            self: Pin<&mut container_poly>,
            vo: Pin<&mut particle_order>,
            n: i32,
            x: f64,
            y: f64,
            z: f64,
            r: f64,
        );
        fn compute_all_cells(
            self: Pin<&mut container_poly>,
        );
        fn sum_cell_volumes(
            self: Pin<&mut container_poly>,
        ) -> f64;
        fn find_voronoi_cell(
            self: Pin<&mut container_poly>,
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
            self: Pin<&mut container_poly>,
            c: Pin<&mut voronoicell>,
            ijk: i32,
            q: i32,
        ) -> bool;
        #[rust_name = "compute_cell_with_index_1"]
        fn compute_cell(
            self: Pin<&mut container_poly>,
            c: Pin<&mut voronoicell_neighbor>,
            ijk: i32,
            q: i32,
        ) -> bool;
        #[rust_name = "compute_ghost_0"]
        fn compute_ghost_cell(
            self: Pin<&mut container_poly>,
            c: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
            r: f64,
        ) -> bool;
        #[rust_name = "compute_ghost_1"]
        fn compute_ghost_cell(
            self: Pin<&mut container_poly>,
            c: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
            r: f64,
        ) -> bool;
    }
}

use crate::cell::{VoroCellNbr, VoroCellSgl};
use crate::particle_marker::ParticleMarker;
use crate::wall::ffi::{
    wall_cone_to_wall, wall_cylinder_to_wall,
    wall_plane_to_wall, wall_sphere_to_wall,
};
use crate::wall::{
    WallCone, WallCylinder, WallPlane, WallSphere,
};
use crate::wall_list::{
    WallList, Walls, Walls0, Walls1, Walls2, Walls3,
};
use cxx::UniquePtr;
use ffi::{
    container_poly_to_wall_list, container_to_wall_list,
};
use std::marker::PhantomData;

type DVec3 = [f64; 3];
type IVec3 = [i32; 3];
type BVec3 = [bool; 3];

/// A class for computing regular Voronoi tessellations.
///
/// A class that has routines
/// specifically for computing the regular Voronoi tessellation with no
/// dependence on particle radii.
pub struct ContainerStd<'a> {
    pub(crate) inner: UniquePtr<ffi::container>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> ContainerStd<'a> {
    /// The class constructor sets up the geometry of container.
    ///
    /// * `xyz_min`: the minimum coordinates.
    /// * `xyz_max`: the maximum coordinates.
    /// * `sub_grids`: the number of grid blocks in each of the
    /// three coordinate directions.
    /// * `is_periodic`: flags setting whether the container is
    /// periodic in each coordinate direction.
    pub fn new(
        xyz_min: DVec3,
        xyz_max: DVec3,
        sub_grids: IVec3,
        is_periodic: BVec3,
    ) -> Self {
        Self::new_with_memory(
            xyz_min,
            xyz_max,
            sub_grids,
            is_periodic,
            16,
        )
    }

    /// The class constructor sets up the geometry of container.
    ///
    /// * `xyz_min`: the minimum coordinates.
    /// * `xyz_max`: the maximum coordinates.
    /// * `sub_grids`: the number of grid blocks in each of the
    /// three coordinate directions.
    /// * `is_periodic`: flags setting whether the container is
    /// periodic in each coordinate direction.
    /// * `initial_memory`: the initial memory allocation for each sub-grid,
    /// in terms of particle count.
    pub fn new_with_memory(
        xyz_min: DVec3,
        xyz_max: DVec3,
        sub_grids: IVec3,
        is_periodic: BVec3,
        initial_memory: i32,
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
                initial_memory,
            ),
            phantom: PhantomData,
        }
    }
}

pub struct ContainerRad<'a> {
    pub(crate) inner: UniquePtr<ffi::container_poly>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> ContainerRad<'a> {
    /// The class constructor sets up the geometry of container.
    ///
    /// * `xyz_min`: the minimum coordinates.
    /// * `xyz_max`: the maximum coordinates.
    /// * `sub_grids`: the number of grid blocks in each of the three
    /// coordinate directions.
    /// * `is_periodic`: flags setting whether the container is
    /// periodic in each coordinate direction.
    pub fn new(
        xyz_min: DVec3,
        xyz_max: DVec3,
        sub_grids: IVec3,
        is_periodic: BVec3,
    ) -> Self {
        Self::new_with_memory(
            xyz_min,
            xyz_max,
            sub_grids,
            is_periodic,
            16,
        )
    }

    /// The class constructor sets up the geometry of container.
    ///
    /// * `xyz_min`: the minimum coordinates.
    /// * `xyz_max`: the maximum coordinates.
    /// * `sub_grids`: the number of grid blocks in each of the three
    /// coordinate directions.
    /// * `is_periodic`: flags setting whether the container is
    /// periodic in each coordinate direction.
    /// * `initial_memory`: the initial memory allocation for each sub-grid,
    /// in terms of particle count.
    pub fn new_with_memory(
        xyz_min: DVec3,
        xyz_max: DVec3,
        sub_grids: IVec3,
        is_periodic: BVec3,
        initial_memory: i32,
    ) -> Self {
        Self {
            inner: ffi::new_container_poly(
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
                initial_memory,
            ),
            phantom: PhantomData,
        }
    }
}

impl<'a> Walls0 for ContainerStd<'a> {
    fn point_inside_walls(&mut self, xyz: DVec3) -> bool {
        self.inner
            .pin_mut()
            .point_inside_walls(xyz[0], xyz[1], xyz[2])
    }
}

impl<'a> Walls0 for ContainerRad<'a> {
    fn point_inside_walls(&mut self, xyz: DVec3) -> bool {
        self.inner
            .pin_mut()
            .point_inside_walls(xyz[0], xyz[1], xyz[2])
    }
}

impl<'a> Walls1<VoroCellSgl> for ContainerStd<'a> {
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

impl<'a> Walls1<VoroCellNbr> for ContainerStd<'a> {
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

impl<'a> Walls1<VoroCellSgl> for ContainerRad<'a> {
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

impl<'a> Walls1<VoroCellNbr> for ContainerRad<'a> {
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

impl<'a> Walls2<'a, WallSphere> for ContainerStd<'a> {
    fn add_wall(&mut self, wall: &'a mut WallSphere) {
        let w0 = wall_sphere_to_wall(wall.inner.pin_mut());
        unsafe {
            // ensure the lifetime of `self` is within the lifetime of
            // `wall` using the lifetime specifier `'a`.
            self.inner.pin_mut().add_wall(w0);
        }
    }
}

impl<'a> Walls2<'a, WallPlane> for ContainerStd<'a> {
    fn add_wall(&mut self, wall: &'a mut WallPlane) {
        let w0 = wall_plane_to_wall(wall.inner.pin_mut());
        unsafe {
            // ensure the lifetime of `self` is within the lifetime of
            // `wall` using the lifetime specifier `'a`.
            self.inner.pin_mut().add_wall(w0);
        }
    }
}

impl<'a> Walls2<'a, WallCylinder> for ContainerStd<'a> {
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

impl<'a> Walls2<'a, WallCone> for ContainerStd<'a> {
    fn add_wall(&mut self, wall: &'a mut WallCone) {
        let w0 = wall_cone_to_wall(wall.inner.pin_mut());
        unsafe {
            // ensure the lifetime of `self` is within the lifetime of
            // `wall` using the lifetime specifier `'a`.
            self.inner.pin_mut().add_wall(w0);
        }
    }
}

impl<'a> Walls2<'a, WallSphere> for ContainerRad<'a> {
    fn add_wall(&mut self, wall: &'a mut WallSphere) {
        let w0 = wall_sphere_to_wall(wall.inner.pin_mut());
        unsafe {
            // ensure the lifetime of `self` is within the lifetime of
            // `wall` using the lifetime specifier `'a`.
            self.inner.pin_mut().add_wall(w0);
        }
    }
}

impl<'a> Walls2<'a, WallPlane> for ContainerRad<'a> {
    fn add_wall(&mut self, wall: &'a mut WallPlane) {
        let w0 = wall_plane_to_wall(wall.inner.pin_mut());
        unsafe {
            // ensure the lifetime of `self` is within the lifetime of
            // `wall` using the lifetime specifier `'a`.
            self.inner.pin_mut().add_wall(w0);
        }
    }
}

impl<'a> Walls2<'a, WallCylinder> for ContainerRad<'a> {
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

impl<'a> Walls2<'a, WallCone> for ContainerRad<'a> {
    fn add_wall(&mut self, wall: &'a mut WallCone) {
        let w0 = wall_cone_to_wall(wall.inner.pin_mut());
        unsafe {
            // ensure the lifetime of `self` is within the lifetime of
            // `wall` using the lifetime specifier `'a`.
            self.inner.pin_mut().add_wall(w0);
        }
    }
}

impl<'a> Walls3<'a, WallList<'a>> for ContainerStd<'a> {
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

impl<'a> Walls3<'a, ContainerStd<'a>> for ContainerStd<'a> {
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

impl<'a> Walls3<'a, ContainerRad<'a>> for ContainerStd<'a> {
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

impl<'a> Walls3<'a, WallList<'a>> for ContainerRad<'a> {
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

impl<'a> Walls3<'a, ContainerStd<'a>> for ContainerRad<'a> {
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

impl<'a> Walls3<'a, ContainerRad<'a>> for ContainerRad<'a> {
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

impl<'a> Walls<'a> for ContainerStd<'a> {}
impl<'a> Walls<'a> for ContainerRad<'a> {}

impl<'a> ContainerStd<'a> {
    /// Put a particle into the correct region of the container.
    ///
    /// * `n`: the numerical ID of the inserted particle.
    /// * `xyz`: the position vector of the inserted particle.
    pub fn put(&mut self, n: i32, xyz: DVec3) {
        self.inner.pin_mut().put(n, xyz[0], xyz[1], xyz[2])
    }

    /// Put a particle into the correct region of the container, also recording
    /// into which region it was stored.
    /// 
    /// * `marker`: the marker class in which to record the region.
    /// * `n`: the numerical ID of the inserted particle.
    /// * `xyz`: the position vector of the inserted particle.
    pub fn put_with_marker(
        &mut self,
        marker: &mut ParticleMarker,
        n: i32,
        xyz: DVec3,
    ) {
        self.inner.pin_mut().put_with_particle_order(
            marker.inner.pin_mut(),
            n,
            xyz[0],
            xyz[1],
            xyz[2],
        )
    }
}

impl<'a> ContainerRad<'a> {
    /// Put a particle into the correct region of the container.
    /// 
    /// * `n`: the numerical ID of the inserted particle.
    /// * `xyz`: the position vector of the inserted particle.
    /// * `r`: the radius of the particle.
    pub fn put(&mut self, n: i32, xyz: DVec3, r: f64) {
        self.inner
            .pin_mut()
            .put(n, xyz[0], xyz[1], xyz[2], r)
    }

    /// Put a particle into the correct region of the container, also recording
    /// into which region it was stored.
    /// 
    /// * `marker`: the marker class in which to record the region.
    /// * `n`: the numerical ID of the inserted particle.
    /// * `xyz`: the position vector of the inserted particle.
    /// * `r`: the radius of the particle.
    pub fn put_with_marker(
        &mut self,
        marker: &mut ParticleMarker,
        n: i32,
        xyz: DVec3,
        r: f64,
    ) {
        self.inner.pin_mut().put_with_particle_order(
            marker.inner.pin_mut(),
            n,
            xyz[0],
            xyz[1],
            xyz[2],
            r,
        );
    }
}

/// A part of trait `Container` whose parameter does not depends any type.
pub trait Container0 {
    fn point_inside(&mut self, xyz: DVec3) -> bool;
    fn total_particles(&mut self) -> i32;
    fn clear(&mut self);
    fn sum_cell_volumes(&mut self) -> f64;
    fn find_voronoi_cell(
        &mut self,
        xyz: DVec3,
    ) -> Option<(i32, DVec3)>;
}

impl<'a> Container0 for ContainerStd<'a> {
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
}

impl<'a> Container0 for ContainerRad<'a> {
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
}

// pub trait Container {
//     fn compute_cell_with_index<T>(
//         &mut self,
//         ijk: i32,
//         q: i32,
//     ) -> Option<T>
//     where
//         T: VoroCell;
//     fn compute_ghost_cell<T>(
//         &mut self,
//         xyz: DVec3,
//     ) -> Option<T>;
// }

// impl<'a> Container for ContainerStd<'a> {
//     fn sum_cell_volumes(&mut self) -> f64 {
//         self.inner.pin_mut().sum_cell_volumes()
//     }
//     fn compute_cell_with_index<T>(
//         &mut self,
//         ijk: i32,
//         q: i32,
//     ) -> Option<T>
//     where
//         T: VoroCell,
//     {
//         todo!()
//     }

//     fn compute_ghost_cell<T>(
//         &mut self,
//         xyz: DVec3,
//     ) -> Option<T> {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cell::{VoroCell, VoroCellNbr},
        wall::WallSphere,
    };

    #[test]
    fn walls_test() {
        let mut c0 = VoroCellNbr::new(
            [-1.0, -1.0, -1.0],
            [1.0, 1.0, 1.0],
        );
        let mut c1 = c0.clone();
        let mut c2 = c0.clone();
        assert_eq!(c0.volume(), 8.0);
        assert_eq!(c1.volume(), 8.0);
        assert_eq!(c2.volume(), 8.0);
        let mut w0 =
            WallSphere::new([0.0, 0.0, 100.0], 100.0);
        let mut w1 =
            WallSphere::new([0.0, 100.0, 0.0], 100.0);

        let mut wl0 = WallList::new();
        let mut wl1 = ContainerStd::new(
            [-1.0, -1.0, -1.0],
            [1.0, 1.0, 1.0],
            [1, 1, 1],
            [false, false, false],
        );
        let mut wl2 = ContainerRad::new(
            [-1.0, -1.0, -1.0],
            [1.0, 1.0, 1.0],
            [1, 1, 1],
            [false, false, false],
        );

        wl0.add_wall(&mut w0);
        wl0.add_wall(&mut w1);
        wl0.apply_walls(&mut c0, [0.0, 0.0, 0.0]);
        assert_eq!(c0.volume(), 2.0);

        wl1.add_walls(&mut wl0);
        wl1.apply_walls(&mut c1, [0.0, 0.0, 0.0]);
        assert_eq!(c1.volume(), 2.0);

        wl2.add_walls(&mut wl1);
        wl2.apply_walls(&mut c2, [0.0, 0.0, 0.0]);
        assert_eq!(c2.volume(), 2.0);
    }
}
