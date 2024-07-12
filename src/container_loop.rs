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
        fn start(self: Pin<&mut c_loop_subset>) -> bool;
        fn inc(self: Pin<&mut c_loop_subset>) -> bool;

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
        fn start(self: Pin<&mut c_loop_order>) -> bool;
        fn inc(self: Pin<&mut c_loop_order>) -> bool;
    }
}

use crate::{
    container::{ContainerRad, ContainerStd},
    particle_marker::ParticleMarker,
};
use cxx::UniquePtr;

pub struct LoopAll {
    pub(crate) inner: UniquePtr<ffi::c_loop_all>,
}

impl LoopAll {
    pub fn of_container_std(
        container: &mut ContainerStd,
    ) -> Self {
        Self {
            inner: ffi::new_c_loop_all_0(
                container.inner.pin_mut(),
            ),
        }
    }

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
    pub fn of_container_std(
        container: &mut ContainerStd,
    ) -> Self {
        Self {
            inner: ffi::new_c_loop_subset_0(
                container.inner.pin_mut(),
            ),
        }
    }

    pub fn of_container_rad(
        container: &mut ContainerRad,
    ) -> Self {
        Self {
            inner: ffi::new_c_loop_subset_1(
                container.inner.pin_mut(),
            ),
        }
    }
}

pub struct LoopMarked {
    pub(crate) inner: UniquePtr<ffi::c_loop_order>,
}

impl LoopMarked {
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
    fn start(&mut self) -> bool;
    fn inc(&mut self) -> bool;
}

impl ContainerLoop for LoopAll {
    fn start(&mut self) -> bool {
        self.inner.pin_mut().start()
    }

    fn inc(&mut self) -> bool {
        self.inner.pin_mut().inc()
    }
}

impl ContainerLoop for LoopSubset {
    fn start(&mut self) -> bool {
        self.inner.pin_mut().start()
    }

    fn inc(&mut self) -> bool {
        self.inner.pin_mut().inc()
    }
}

impl ContainerLoop for LoopMarked {
    fn start(&mut self) -> bool {
        self.inner.pin_mut().start()
    }

    fn inc(&mut self) -> bool {
        self.inner.pin_mut().inc()
    }
}
