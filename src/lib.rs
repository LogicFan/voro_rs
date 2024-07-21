//! # voro_rs
//!
//! A Rust binding for voro++ library.
//!
//! This binding has been tested on Windows, Linux and MacOS.

pub mod native;

pub mod cell;
pub mod container;
pub mod container_loop;
pub mod particle_marker;
pub mod pre_container;
pub mod wall;
pub mod wall_list;

pub mod prelude {
    pub use crate::cell::{
        VoroCell, VoroCellNbr, VoroCellSgl,
    };
    pub use crate::container::{
        Container, Container0, Container1, Container2,
        ContainerRad, ContainerStd,
    };
    pub use crate::container_loop::{
        ContainerLoop, LoopAll, LoopMarked, LoopSubset,
    };
    pub use crate::particle_marker::ParticleMarker;
    pub use crate::wall::{
        Wall, Wall0, Wall1, WallCone, WallCylinder,
        WallPlane, WallSphere,
    };
    pub use crate::wall_list::{
        WallList, Walls, Walls0, Walls1, Walls2, Walls3,
    };
}
