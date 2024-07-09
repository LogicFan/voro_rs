#[cxx::bridge(namespace = "voro")]
pub mod ffi {
    unsafe extern "C++" {
        include!("voro_rs/cpp/voro++.hh");
        include!("voro_rs/cpp/boilerplate.hh");

        type voronoicell;
        fn construct() -> UniquePtr<voronoicell>;
        fn init_base(
            self: Pin<&mut voronoicell>,
            xmin: f64,
            xmax: f64,
            ymin: f64,
            ymax: f64,
            zmin: f64,
            zmax: f64,
        );
        fn init_octahedron_base(
            self: Pin<&mut voronoicell>,
            l: f64,
        );
        fn init_tetrahedron_base(
            self: Pin<&mut voronoicell>,
            x0: f64,
            y0: f64,
            z0: f64,
            x1: f64,
            y1: f64,
            z1: f64,
            x2: f64,
            y2: f64,
            z2: f64,
            x3: f64,
            y3: f64,
            z3: f64,
        );
        fn translate(
            self: Pin<&mut voronoicell>,
            x: f64,
            y: f64,
            z: f64,
        );
        fn volume(self: Pin<&mut voronoicell>) -> f64;
        fn max_radius_squared(
            self: Pin<&mut voronoicell>,
        ) -> f64;
        fn total_edge_distance(
            self: Pin<&mut voronoicell>,
        ) -> f64;
        fn surface_area(self: Pin<&mut voronoicell>)
            -> f64;
        fn centroid(
            self: Pin<&mut voronoicell>,
            cx: &mut f64,
            cy: &mut f64,
            cz: &mut f64,
        );
        fn number_of_faces(
            self: Pin<&mut voronoicell>,
        ) -> i32;
        fn number_of_edges(
            self: Pin<&mut voronoicell>,
        ) -> i32;
        fn vertex_orders(
            self: Pin<&mut voronoicell>,
            v: Pin<&mut CxxVector<i32>>,
        );
    }
}

use cxx::{CxxVector, UniquePtr};

type Vec3 = [f64; 3];

pub trait VoronoiCell {
    fn translate(&mut self, xyz: Vec3);
    fn volume(&mut self) -> f64;
    fn max_radius_squared(&mut self) -> f64;
    fn total_edge_distance(&mut self) -> f64;
    fn surface_area(&mut self) -> f64;

    fn centroid(&mut self) -> Vec3;
    fn number_of_faces(&mut self) -> i32;
    fn number_of_edges(&mut self) -> i32;
    fn vertex_orders(&mut self) -> Vec<i32>;
}

pub struct VoronoiCellNoNeighbor {
    inner: UniquePtr<ffi::voronoicell>,
}

impl VoronoiCellNoNeighbor {
    pub fn new(xyz_min: Vec3, xyz_max: Vec3) -> Self {
        let mut val = Self {
            inner: ffi::construct(),
        };
        val.inner.pin_mut().init_base(
            xyz_min[0], xyz_max[0], xyz_min[1], xyz_max[1],
            xyz_min[2], xyz_max[2],
        );
        val
    }

    pub fn new_octahedron(l: f64) -> Self {
        let mut val = Self {
            inner: ffi::construct(),
        };
        val.inner.pin_mut().init_octahedron_base(l);
        val
    }

    pub fn new_tetrahedron(
        xyz0: Vec3,
        xyz1: Vec3,
        xyz2: Vec3,
        xyz3: Vec3,
    ) -> Self {
        let mut val = Self {
            inner: ffi::construct(),
        };
        val.inner.pin_mut().init_tetrahedron_base(
            xyz0[0], xyz0[1], xyz0[2], xyz1[0], xyz1[1],
            xyz1[2], xyz2[0], xyz2[1], xyz2[2], xyz3[0],
            xyz3[1], xyz3[2],
        );
        val
    }
}

impl VoronoiCell for VoronoiCellNoNeighbor {
    fn translate(&mut self, xyz: Vec3) {
        self.inner
            .pin_mut()
            .translate(xyz[0], xyz[1], xyz[2]);
    }

    fn volume(&mut self) -> f64 {
        self.inner.pin_mut().volume()
    }

    fn max_radius_squared(&mut self) -> f64 {
        self.inner.pin_mut().max_radius_squared()
    }

    fn total_edge_distance(&mut self) -> f64 {
        self.inner.pin_mut().total_edge_distance()
    }

    fn surface_area(&mut self) -> f64 {
        self.inner.pin_mut().surface_area()
    }

    fn centroid(&mut self) -> Vec3 {
        let mut cx = 0.0;
        let mut cy = 0.0;
        let mut cz = 0.0;
        self.inner
            .pin_mut()
            .centroid(&mut cx, &mut cy, &mut cz);
        [cx, cy, cz]
    }

    fn number_of_faces(&mut self) -> i32 {
        self.inner.pin_mut().number_of_faces()
    }

    fn number_of_edges(&mut self) -> i32 {
        self.inner.pin_mut().number_of_edges()
    }

    fn vertex_orders(&mut self) -> Vec<i32> {
        let mut v = CxxVector::new();
        self.inner.pin_mut().vertex_orders(v.pin_mut());
        v.into_iter().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_new() {
        VoronoiCellNoNeighbor::new(
            [1.0, 1.0, 1.0],
            [2.0, 2.0, 2.0],
        );
    }
}
