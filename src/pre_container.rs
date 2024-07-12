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

use cxx::UniquePtr;

type DVec3 = [f64; 3];

pub struct PreContainerStd {
    pub(crate) inner: UniquePtr<ffi::pre_container>,
}