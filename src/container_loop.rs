//! The loop classes for containers or periodic containers.

#[cxx::bridge(namespace = "voro")]
pub mod ffi {
    unsafe extern "C++" {
        include!("voro_rs/cpp/boilerplate.hh");

        type container = crate::container::ffi::container;
        type container_poly =
            crate::container::ffi::container_poly;
        type particle_order =
            crate::particle_marker::ffi::particle_order;

        type c_loop_all;
        #[rust_name = "new_c_loop_all_0"]
        fn construct(
            con: Pin<&mut container>,
        ) -> UniquePtr<c_loop_all>;
        #[rust_name = "new_c_loop_all_1"]
        fn construct(
            con: Pin<&mut container_poly>,
        ) -> UniquePtr<c_loop_all>;
        fn x(self: Pin<&mut c_loop_all>) -> f64;
        fn y(self: Pin<&mut c_loop_all>) -> f64;
        fn z(self: Pin<&mut c_loop_all>) -> f64;
        fn pid(self: Pin<&mut c_loop_all>) -> i32;
        fn start(self: Pin<&mut c_loop_all>) -> bool;
        fn inc(self: Pin<&mut c_loop_all>) -> bool;

        type c_loop_subset;
        #[rust_name = "new_c_loop_subset_0"]
        fn construct(
            con: Pin<&mut container>,
        ) -> UniquePtr<c_loop_subset>;
        #[rust_name = "new_c_loop_subset_1"]
        fn construct(
            con: Pin<&mut container_poly>,
        ) -> UniquePtr<c_loop_subset>;
        fn x(self: Pin<&mut c_loop_subset>) -> f64;
        fn y(self: Pin<&mut c_loop_subset>) -> f64;
        fn z(self: Pin<&mut c_loop_subset>) -> f64;
        fn pid(self: Pin<&mut c_loop_subset>) -> i32;
        fn start(self: Pin<&mut c_loop_subset>) -> bool;
        fn inc(self: Pin<&mut c_loop_subset>) -> bool;

        fn setup_sphere(
            self: Pin<&mut c_loop_subset>,
            vx: f64,
            vy: f64,
            vz: f64,
            r: f64,
            bounds_test: bool,
        );
        fn setup_box(
            self: Pin<&mut c_loop_subset>,
            xmin: f64,
            xmax: f64,
            ymin: f64,
            ymax: f64,
            zmin: f64,
            zmax: f64,
            bounds_test: bool,
        );
        fn setup_intbox(
            self: Pin<&mut c_loop_subset>,
            ai_: i32,
            bi_: i32,
            aj_: i32,
            bj_: i32,
            ak_: i32,
            bk_: i32,
        );

        type c_loop_order;
        #[rust_name = "new_c_loop_order_0"]
        fn construct(
            con: Pin<&mut container>,
            vo: Pin<&mut particle_order>,
        ) -> UniquePtr<c_loop_order>;
        #[rust_name = "new_c_loop_order_1"]
        fn construct(
            con: Pin<&mut container_poly>,
            vo: Pin<&mut particle_order>,
        ) -> UniquePtr<c_loop_order>;
        fn x(self: Pin<&mut c_loop_order>) -> f64;
        fn y(self: Pin<&mut c_loop_order>) -> f64;
        fn z(self: Pin<&mut c_loop_order>) -> f64;
        fn pid(self: Pin<&mut c_loop_order>) -> i32;
        fn start(self: Pin<&mut c_loop_order>) -> bool;
        fn inc(self: Pin<&mut c_loop_order>) -> bool;
    }
}

use crate::{
    container::{ContainerRad, ContainerStd},
    particle_marker::ParticleMarker,
};
use cxx::UniquePtr;

type DVec3 = [f64; 3];
type IVec3 = [i32; 3];

pub struct LoopAll {
    pub(crate) inner: UniquePtr<ffi::c_loop_all>,
}

impl LoopAll {
    /// The constructor copies several necessary constants from the
    /// base container class.
    pub fn of_container_std(
        container: &mut ContainerStd,
    ) -> Self {
        Self {
            inner: ffi::new_c_loop_all_0(
                container.inner.pin_mut(),
            ),
        }
    }

    /// The constructor copies several necessary constants from the
    /// base container class.
    pub fn of_container_rad(
        container: &mut ContainerRad,
    ) -> Self {
        Self {
            inner: ffi::new_c_loop_all_1(
                container.inner.pin_mut(),
            ),
        }
    }
}

pub struct LoopSubset {
    pub(crate) inner: UniquePtr<ffi::c_loop_subset>,
}

impl LoopSubset {
    /// The constructor copies several necessary constants from the
    /// base container class.
    pub fn of_container_std(
        container: &mut ContainerStd,
    ) -> Self {
        Self {
            inner: ffi::new_c_loop_subset_0(
                container.inner.pin_mut(),
            ),
        }
    }

    /// The constructor copies several necessary constants from the
    /// base container class.
    pub fn of_container_rad(
        container: &mut ContainerRad,
    ) -> Self {
        Self {
            inner: ffi::new_c_loop_subset_1(
                container.inner.pin_mut(),
            ),
        }
    }

    /// Setup a `LoopSubset` object to scan over all particles within a
    /// given sphere.
    ///
    /// * `v`: the position vector of the center of the sphere.
    /// * `r`: the radius of the sphere.
    /// * `bounds_test`: whether to do detailed bounds checking. If this is
    /// false then the class will loop over all particles in
    /// blocks that overlap the given sphere. If it is true,
    /// the particle will only loop over the particles which
    /// actually lie within the sphere.
    pub fn setup_sphere(
        &mut self,
        v: DVec3,
        r: f64,
        bounds_test: bool,
    ) {
        self.inner.pin_mut().setup_sphere(
            v[0],
            v[1],
            v[2],
            r,
            bounds_test,
        );
    }

    /// Setup a `LoopSubset` object to loop over all particles in a rectangular box.
    ///
    /// * `xyz_min`: the minimum coordinates of the box.
    /// * `xyz_max`: the maximum coordinates of the box.
    /// * `bounds_test`: whether to do detailed bounds checking. If this is
    /// false then the class will loop over all particles in
    /// blocks that overlap the given box. If it is true, the
    /// particle will only loop over the particles which
    /// actually lie within the box.
    pub fn setup_box(
        &mut self,
        xyz_min: DVec3,
        xyz_max: DVec3,
        bounds_test: bool,
    ) {
        self.inner.pin_mut().setup_box(
            xyz_min[0],
            xyz_max[0],
            xyz_min[1],
            xyz_max[1],
            xyz_min[2],
            xyz_max[2],
            bounds_test,
        );
    }

    /// Setup a `LoopSubset` object to loop over all particles in a
    /// rectangular subgrid of blocks.
    ///
    /// * `a`: the start of sub-grid range.
    /// * `b`: the end of sub-grid range.
    pub fn setup_grid(&mut self, a: IVec3, b: IVec3) {
        self.inner.pin_mut().setup_intbox(
            a[0], b[0], a[1], b[1], a[2], b[2],
        );
    }
}

pub struct LoopMarked {
    pub(crate) inner: UniquePtr<ffi::c_loop_order>,
}

impl LoopMarked {
    /// The constructor copies several necessary constants from the
    /// base container class.
    pub fn with_container_std(
        container: &mut ContainerStd,
        marker: &mut ParticleMarker,
    ) -> Self {
        Self {
            inner: ffi::new_c_loop_order_0(
                container.inner.pin_mut(),
                marker.inner.pin_mut(),
            ),
        }
    }

    /// The constructor copies several necessary constants from the
    /// base container class.
    pub fn with_container_rad(
        container: &mut ContainerRad,
        marker: &mut ParticleMarker,
    ) -> Self {
        Self {
            inner: ffi::new_c_loop_order_1(
                container.inner.pin_mut(),
                marker.inner.pin_mut(),
            ),
        }
    }
}

pub trait ContainerLoop {
    /// Returns the x position of the particle currently being
    /// considered by the loop.
    fn x(&mut self) -> f64;

    /// Returns the y position of the particle currently being
    /// considered by the loop.
    fn y(&mut self) -> f64;

    /// Returns the z position of the particle currently being
    /// considered by the loop.
    fn z(&mut self) -> f64;

    /// Returns the position vector of the particle currently being
    /// considered by the loop.
    fn position(&mut self) -> DVec3 {
        [self.x(), self.y(), self.z()]
    }

    /// Returns the ID of the particle currently being considered
    /// by the loop.
    fn particle_id(&mut self) -> i32;

    /// Sets the class to consider the first particle.
    ///
    /// Return true if there is any particle to consider, false
    /// otherwise.
    fn start(&mut self) -> bool;

    /// Finds the next particle to test.
    ///
    /// Return true if there is another particle, false if no more
    /// particles are available.
    fn inc(&mut self) -> bool;
}

impl ContainerLoop for LoopAll {
    fn x(&mut self) -> f64 {
        self.inner.pin_mut().x()
    }

    fn y(&mut self) -> f64 {
        self.inner.pin_mut().y()
    }

    fn z(&mut self) -> f64 {
        self.inner.pin_mut().z()
    }

    fn particle_id(&mut self) -> i32 {
        self.inner.pin_mut().pid()
    }

    fn start(&mut self) -> bool {
        self.inner.pin_mut().start()
    }

    fn inc(&mut self) -> bool {
        self.inner.pin_mut().inc()
    }
}

impl ContainerLoop for LoopSubset {
    fn x(&mut self) -> f64 {
        self.inner.pin_mut().x()
    }

    fn y(&mut self) -> f64 {
        self.inner.pin_mut().y()
    }

    fn z(&mut self) -> f64 {
        self.inner.pin_mut().z()
    }

    fn particle_id(&mut self) -> i32 {
        self.inner.pin_mut().pid()
    }

    fn start(&mut self) -> bool {
        self.inner.pin_mut().start()
    }

    fn inc(&mut self) -> bool {
        self.inner.pin_mut().inc()
    }
}

impl ContainerLoop for LoopMarked {
    fn x(&mut self) -> f64 {
        self.inner.pin_mut().x()
    }

    fn y(&mut self) -> f64 {
        self.inner.pin_mut().y()
    }

    fn z(&mut self) -> f64 {
        self.inner.pin_mut().z()
    }

    fn particle_id(&mut self) -> i32 {
        self.inner.pin_mut().pid()
    }

    fn start(&mut self) -> bool {
        self.inner.pin_mut().start()
    }

    fn inc(&mut self) -> bool {
        self.inner.pin_mut().inc()
    }
}
