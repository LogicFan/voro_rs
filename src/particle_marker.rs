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

/// A class for storing ordering information when particles are added to
/// a container.
///
/// When particles are added to a container class, they are sorted into an
/// internal computational grid of blocks. The `ParticleMarker` class provides a
/// mechanism for remembering which block particles were sorted into. The import
/// and put routines in the container class have variants that also take a
/// `ParticleMarker` class. Each time they are called, they will store the block
/// that the particle was sorted into, plus the position of the particle within
/// the block. The `ParticleMarker` class can used by the `LoopMarked` class to
/// specifically loop over the particles that have their information stored
/// within it.
pub struct ParticleMarker {
    pub(crate) inner: UniquePtr<ffi::particle_order>,
}

impl ParticleMarker {
    /// Create a new `ParticleMarker`.
    pub fn new() -> Self {
        Self::new_with_memory(16)
    }

    /// The struct allocates memory to store the
    /// ordering information.
    /// * `initial_memory`: the initial amount of memory to allocate,
    /// in terms of particle count.
    pub fn new_with_memory(initial_memory: i32) -> Self {
        Self {
            inner: ffi::new_particle_order(initial_memory),
        }
    }
}
