//! Particle order class

#[cxx::bridge(namespace = "voro")]
pub mod ffi {
    unsafe extern "C++" {
        include!("voro_rs/cpp/boilerplate.hh");

        type particle_order;
        #[rust_name = "new_particle_order"]
        fn construct(
            init_size: i32,
        ) -> UniquePtr<particle_order>;
    }
}

use cxx::UniquePtr;

pub struct ParticleMarker {
    pub(crate) inner: UniquePtr<ffi::particle_order>,
}

impl ParticleMarker {
    pub fn new() -> Self {
        Self::new_with_memory(16)
    }

    pub fn new_with_memory(initial_memory: i32) -> Self {
        Self {
            inner: ffi::new_particle_order(initial_memory),
        }
    }
}
