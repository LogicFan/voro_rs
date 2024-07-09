//! Wall and related classes.

#[cxx::bridge(namespace = "voro")]
pub mod ffi {
    unsafe extern "C++" {
        include!("voro_rs/cpp/voro++.hh");
        include!("voro_rs/cpp/boilerplate.hh");

        type voronoicell = crate::cell::ffi::voronoicell;
        type voronoicell_neighbor =
            crate::cell::ffi::voronoicell_neighbor;

        type wall_sphere;
        #[rust_name = "new_wall_sphere"]
        fn construct(
            xc_: f64,
            yc_: f64,
            zc_: f64,
            rc_: f64,
            w_id_: i32,
        ) -> UniquePtr<wall_sphere>;
        fn point_inside(
            self: Pin<&mut wall_sphere>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        fn cut_cell(
            self: Pin<&mut wall_sphere>,
            c: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "cut_cell_neighbor"]
        fn cut_cell(
            self: Pin<&mut wall_sphere>,
            c: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;

        type wall_plane;
        #[rust_name = "new_wall_plane"]
        fn construct(
            xc_: f64,
            yc_: f64,
            zc_: f64,
            rc_: f64,
            w_id_: i32,
        ) -> UniquePtr<wall_plane>;
        fn point_inside(
            self: Pin<&mut wall_plane>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        fn cut_cell(
            self: Pin<&mut wall_plane>,
            c: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "cut_cell_neighbor"]
        fn cut_cell(
            self: Pin<&mut wall_plane>,
            c: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;

        type wall_cylinder;
        #[rust_name = "new_wall_cylinder"]
        fn construct(
            xc_: f64,
            yc_: f64,
            zc_: f64,
            rc_: f64,
            w_id_: i32,
        ) -> UniquePtr<wall_cylinder>;
        fn point_inside(
            self: Pin<&mut wall_cylinder>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        fn cut_cell(
            self: Pin<&mut wall_cylinder>,
            c: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "cut_cell_neighbor"]
        fn cut_cell(
            self: Pin<&mut wall_cylinder>,
            c: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;

        type wall_cone;
        #[rust_name = "new_wall_cone"]
        fn construct(
            xc_: f64,
            yc_: f64,
            zc_: f64,
            rc_: f64,
            w_id_: i32,
        ) -> UniquePtr<wall_cone>;
        fn point_inside(
            self: Pin<&mut wall_cone>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        fn cut_cell(
            self: Pin<&mut wall_cone>,
            c: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
        #[rust_name = "cut_cell_neighbor"]
        fn cut_cell(
            self: Pin<&mut wall_cone>,
            c: Pin<&mut voronoicell_neighbor>,
            x: f64,
            y: f64,
            z: f64,
        ) -> bool;
    }
}

use crate::prelude::{VoronoiCell, VoronoiCellNeighbor};
use cxx::UniquePtr;

type DVec3 = [f64; 3];

pub trait Wall {
    fn point_inside(&mut self, xyz: DVec3) -> bool;
    fn cut_cell(
        &mut self,
        cell: &mut VoronoiCell,
        xyz: DVec3,
    ) -> bool;
    fn cut_cell_neighbor(
        &mut self,
        cell: &mut VoronoiCellNeighbor,
        xyz: DVec3,
    ) -> bool;
}

pub struct WallSphere {
    inner: UniquePtr<ffi::wall_sphere>,
}

impl WallSphere {
    pub fn new(center: DVec3, radius: f64) -> Self {
        Self {
            inner: ffi::new_wall_sphere(
                center[0], center[1], center[2], radius,
                -99,
            ),
        }
    }

    pub fn new_with_id(
        center: DVec3,
        radius: f64,
        id: i32,
    ) -> Self {
        Self {
            inner: ffi::new_wall_sphere(
                center[0], center[1], center[2], radius, id,
            ),
        }
    }
}

impl Wall for WallSphere {
    fn point_inside(&mut self, xyz: DVec3) -> bool {
        self.inner
            .pin_mut()
            .point_inside(xyz[0], xyz[1], xyz[2])
    }

    fn cut_cell(
        &mut self,
        cell: &mut VoronoiCell,
        xyz: DVec3,
    ) -> bool {
        self.inner.pin_mut().cut_cell(
            cell.inner.pin_mut(),
            xyz[0],
            xyz[1],
            xyz[2],
        )
    }

    fn cut_cell_neighbor(
        &mut self,
        cell: &mut VoronoiCellNeighbor,
        xyz: DVec3,
    ) -> bool {
        self.inner.pin_mut().cut_cell_neighbor(
            cell.inner.pin_mut(),
            xyz[0],
            xyz[1],
            xyz[2],
        )
    }
}

pub struct WallPlane {
    inner: UniquePtr<ffi::wall_plane>,
}

impl WallPlane {
    pub fn new(center: DVec3, radius: f64) -> Self {
        Self {
            inner: ffi::new_wall_plane(
                center[0], center[1], center[2], radius,
                -99,
            ),
        }
    }

    pub fn new_with_id(
        center: DVec3,
        radius: f64,
        id: i32,
    ) -> Self {
        Self {
            inner: ffi::new_wall_plane(
                center[0], center[1], center[2], radius, id,
            ),
        }
    }
}

impl Wall for WallPlane {
    fn point_inside(&mut self, xyz: DVec3) -> bool {
        self.inner
            .pin_mut()
            .point_inside(xyz[0], xyz[1], xyz[2])
    }

    fn cut_cell(
        &mut self,
        cell: &mut VoronoiCell,
        xyz: DVec3,
    ) -> bool {
        self.inner.pin_mut().cut_cell(
            cell.inner.pin_mut(),
            xyz[0],
            xyz[1],
            xyz[2],
        )
    }

    fn cut_cell_neighbor(
        &mut self,
        cell: &mut VoronoiCellNeighbor,
        xyz: DVec3,
    ) -> bool {
        self.inner.pin_mut().cut_cell_neighbor(
            cell.inner.pin_mut(),
            xyz[0],
            xyz[1],
            xyz[2],
        )
    }
}

pub struct WallCylinder {
    inner: UniquePtr<ffi::wall_cylinder>,
}

impl WallCylinder {
    pub fn new(center: DVec3, radius: f64) -> Self {
        Self {
            inner: ffi::new_wall_cylinder(
                center[0], center[1], center[2], radius,
                -99,
            ),
        }
    }

    pub fn new_with_id(
        center: DVec3,
        radius: f64,
        id: i32,
    ) -> Self {
        Self {
            inner: ffi::new_wall_cylinder(
                center[0], center[1], center[2], radius, id,
            ),
        }
    }
}

impl Wall for WallCylinder {
    fn point_inside(&mut self, xyz: DVec3) -> bool {
        self.inner
            .pin_mut()
            .point_inside(xyz[0], xyz[1], xyz[2])
    }

    fn cut_cell(
        &mut self,
        cell: &mut VoronoiCell,
        xyz: DVec3,
    ) -> bool {
        self.inner.pin_mut().cut_cell(
            cell.inner.pin_mut(),
            xyz[0],
            xyz[1],
            xyz[2],
        )
    }

    fn cut_cell_neighbor(
        &mut self,
        cell: &mut VoronoiCellNeighbor,
        xyz: DVec3,
    ) -> bool {
        self.inner.pin_mut().cut_cell_neighbor(
            cell.inner.pin_mut(),
            xyz[0],
            xyz[1],
            xyz[2],
        )
    }
}

pub struct WallCone {
    inner: UniquePtr<ffi::wall_cone>,
}

impl WallCone {
    pub fn new(center: DVec3, radius: f64) -> Self {
        Self {
            inner: ffi::new_wall_cone(
                center[0], center[1], center[2], radius,
                -99,
            ),
        }
    }

    pub fn new_with_id(
        center: DVec3,
        radius: f64,
        id: i32,
    ) -> Self {
        Self {
            inner: ffi::new_wall_cone(
                center[0], center[1], center[2], radius, id,
            ),
        }
    }
}

impl Wall for WallCone {
    fn point_inside(&mut self, xyz: DVec3) -> bool {
        self.inner
            .pin_mut()
            .point_inside(xyz[0], xyz[1], xyz[2])
    }

    fn cut_cell(
        &mut self,
        cell: &mut VoronoiCell,
        xyz: DVec3,
    ) -> bool {
        self.inner.pin_mut().cut_cell(
            cell.inner.pin_mut(),
            xyz[0],
            xyz[1],
            xyz[2],
        )
    }

    fn cut_cell_neighbor(
        &mut self,
        cell: &mut VoronoiCellNeighbor,
        xyz: DVec3,
    ) -> bool {
        self.inner.pin_mut().cut_cell_neighbor(
            cell.inner.pin_mut(),
            xyz[0],
            xyz[1],
            xyz[2],
        )
    }
}
