//! PreContainer and related classes

#[cxx::bridge(namespace = "voro")]
pub mod ffi {
    unsafe extern "C++" {
        include!("voro_rs/src/boilerplate.hh");

        type container = crate::container::ffi::container;
        type container_poly =
            crate::container::ffi::container_poly;

        type particle_order =
            crate::particle_marker::ffi::particle_order;

        type pre_container;
        #[rust_name = "new_pre_container"]
        fn construct(
            ax_: f64,
            bx_: f64,
            ay_: f64,
            by_: f64,
            az_: f64,
            bz_: f64,
            xperiodic_: bool,
            yperiodic_: bool,
            zperiodic_: bool,
        ) -> UniquePtr<pre_container>;
        fn guess_optimal(
            self: Pin<&mut pre_container>,
            nx: &mut i32,
            ny: &mut i32,
            nz: &mut i32,
        );
        fn total_particles(
            self: Pin<&mut pre_container>,
        ) -> i32;
        fn put(
            self: Pin<&mut pre_container>,
            n: i32,
            x: f64,
            y: f64,
            z: f64,
        );
        fn setup(
            self: Pin<&mut pre_container>,
            con: Pin<&mut container>,
        );
        #[rust_name = "setup_with_particle_order"]
        fn setup(
            self: Pin<&mut pre_container>,
            vo: Pin<&mut particle_order>,
            con: Pin<&mut container>,
        );

        type pre_container_poly;
        #[rust_name = "new_pre_container_poly"]
        fn construct(
            ax_: f64,
            bx_: f64,
            ay_: f64,
            by_: f64,
            az_: f64,
            bz_: f64,
            xperiodic_: bool,
            yperiodic_: bool,
            zperiodic_: bool,
        ) -> UniquePtr<pre_container_poly>;
        fn guess_optimal(
            self: Pin<&mut pre_container_poly>,
            nx: &mut i32,
            ny: &mut i32,
            nz: &mut i32,
        );
        fn total_particles(
            self: Pin<&mut pre_container_poly>,
        ) -> i32;
        fn put(
            self: Pin<&mut pre_container_poly>,
            n: i32,
            x: f64,
            y: f64,
            z: f64,
            r: f64,
        );
        fn setup(
            self: Pin<&mut pre_container_poly>,
            con: Pin<&mut container_poly>,
        );
        #[rust_name = "setup_with_particle_order"]
        fn setup(
            self: Pin<&mut pre_container_poly>,
            vo: Pin<&mut particle_order>,
            con: Pin<&mut container_poly>,
        );
    }
}

use crate::prelude::{
    ContainerRad, ContainerStd, ParticleMarker,
};
use cxx::UniquePtr;

type DVec3 = [f64; 3];
type IVec3 = [i32; 3];
type BVec3 = [bool; 3];

/// A class for storing an arbitrary number of particles without radius
/// information, prior to setting up a container geometry.
pub struct PreContainerStd {
    pub(crate) inner: UniquePtr<ffi::pre_container>,
}

impl PreContainerStd {
    /// The class constructor sets up the geometry of container,
    /// initializing the minimum and maximum coordinates in each
    /// direction.
    ///
    /// * `xyz_min`: the minimum coordinates.
    /// * `xyz_max`: the maximum coordinates.
    /// * `is_periodic`: flags setting whether the container is periodic in
    /// each coordinate direction.
    pub fn new(
        xyz_min: DVec3,
        xyz_max: DVec3,
        is_periodic: BVec3,
    ) -> Self {
        Self {
            inner: ffi::new_pre_container(
                xyz_min[0],
                xyz_max[0],
                xyz_min[1],
                xyz_max[1],
                xyz_min[2],
                xyz_max[2],
                is_periodic[0],
                is_periodic[1],
                is_periodic[2],
            ),
        }
    }
}

/// A class for storing an arbitrary number of particles with radius
/// information, prior to setting up a container geometry.
pub struct PreContainerRad {
    pub(crate) inner: UniquePtr<ffi::pre_container_poly>,
}

impl PreContainerRad {
    /// The class constructor sets up the geometry of container,
    /// initializing the minimum and maximum coordinates in each
    /// direction.
    ///
    /// * `xyz_min`: the minimum coordinates.
    /// * `xyz_max`: the maximum coordinates.
    /// * `is_periodic`: flags setting whether the container is periodic in
    /// each coordinate direction.
    pub fn new(
        xyz_min: DVec3,
        xyz_max: DVec3,
        is_periodic: BVec3,
    ) -> Self {
        Self {
            inner: ffi::new_pre_container_poly(
                xyz_min[0],
                xyz_max[0],
                xyz_min[1],
                xyz_max[1],
                xyz_min[2],
                xyz_max[2],
                is_periodic[0],
                is_periodic[1],
                is_periodic[2],
            ),
        }
    }
}

pub trait PreContainer<T> {
    /// Makes a guess at the optimal grid of blocks to use.
    ///
    /// Return the number of grids to use.
    fn optimal_grids(&mut self) -> IVec3;

    /// Calculates and returns the total number of particles stored
    /// within the class.
    ///
    /// Return The number of particles.
    fn total_particles(&mut self) -> i32;

    /// Stores a particle ID and position, allocating a new memory chunk if necessary.
    ///
    /// * `n`: the numerical ID of the inserted particle.
    /// * `xyz`: the position vector of the inserted particle.
    /// * `r`: the radius of the particle. This is ignored for `PreContainerStd`.
    fn put(&mut self, n: i32, xyz: DVec3, r: f64);

    /// Transfers the particles stored within the class to a container class.
    ///
    /// * `container`: the container class to transfer to.
    fn setup(&mut self, container: &mut T);

    /// Transfers the particles stored within the class to a container class, also
    /// recording the order in which particles were stored.
    ///
    /// * `marker`: the ordering class to use.
    /// * `container`: the container class to transfer to.
    fn setup_with_marker(
        &mut self,
        marker: &mut ParticleMarker,
        container: &mut T,
    );
}

impl<'a> PreContainer<ContainerStd<'a>>
    for PreContainerStd
{
    fn optimal_grids(&mut self) -> IVec3 {
        let mut nx = 0;
        let mut ny = 0;
        let mut nz = 0;
        self.inner
            .pin_mut()
            .guess_optimal(&mut nx, &mut ny, &mut nz);
        [nx, ny, nz]
    }

    fn total_particles(&mut self) -> i32 {
        self.inner.pin_mut().total_particles()
    }

    fn put(&mut self, n: i32, xyz: DVec3, _: f64) {
        self.inner.pin_mut().put(n, xyz[0], xyz[1], xyz[2]);
    }

    fn setup(&mut self, container: &mut ContainerStd) {
        self.inner
            .pin_mut()
            .setup(container.inner.pin_mut());
    }

    fn setup_with_marker(
        &mut self,
        marker: &mut ParticleMarker,
        container: &mut ContainerStd,
    ) {
        self.inner.pin_mut().setup_with_particle_order(
            marker.inner.pin_mut(),
            container.inner.pin_mut(),
        );
    }
}

impl<'a> PreContainer<ContainerRad<'a>>
    for PreContainerRad
{
    fn optimal_grids(&mut self) -> IVec3 {
        let mut nx = 0;
        let mut ny = 0;
        let mut nz = 0;
        self.inner
            .pin_mut()
            .guess_optimal(&mut nx, &mut ny, &mut nz);
        [nx, ny, nz]
    }

    fn total_particles(&mut self) -> i32 {
        self.inner.pin_mut().total_particles()
    }

    fn put(&mut self, n: i32, xyz: DVec3, r: f64) {
        self.inner
            .pin_mut()
            .put(n, xyz[0], xyz[1], xyz[2], r);
    }

    fn setup(&mut self, container: &mut ContainerRad) {
        self.inner
            .pin_mut()
            .setup(container.inner.pin_mut());
    }

    fn setup_with_marker(
        &mut self,
        marker: &mut ParticleMarker,
        container: &mut ContainerRad,
    ) {
        self.inner.pin_mut().setup_with_particle_order(
            marker.inner.pin_mut(),
            container.inner.pin_mut(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Container0;

    #[test]
    fn container_std_test() {
        let xyz_min = [-10.0, -10.0, -10.0];
        let xyz_max = [10.0, 10.0, 10.0];
        let is_periodic = [false, false, false];

        let mut pc = PreContainerStd::new(
            xyz_min,
            xyz_max,
            is_periodic,
        );
        let grids = pc.optimal_grids();

        pc.put(0, [0.0, 0.0, 0.0], 0.0);
        pc.put(1, [1.0, 0.0, 0.0], 0.0);
        pc.put(2, [2.0, 0.0, 0.0], 0.0);
        pc.put(3, [3.0, 0.0, 0.0], 0.0);
        pc.put(4, [4.0, 0.0, 0.0], 0.0);
        pc.put(5, [4.0, 1.0, 0.0], 0.0);
        pc.put(6, [4.0, 2.0, 0.0], 0.0);
        pc.put(7, [4.0, 3.0, 0.0], 0.0);
        pc.put(8, [4.0, 4.0, 0.0], 0.0);
        assert_eq!(pc.total_particles(), 9);

        let mut con = ContainerStd::new(
            xyz_min,
            xyz_max,
            grids,
            is_periodic,
        );
        pc.setup(&mut con);
        assert_eq!(con.total_particles(), 9);
        assert_eq!(con.sum_cell_volumes(), 8000.0);

        let c = con.find_voronoi_cell([4.0, 4.0, 0.0]);
        assert!(c.is_some());
        assert_eq!(c.unwrap().0, 8);
    }
}
