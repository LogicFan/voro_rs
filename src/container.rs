#[cxx::bridge(namespace = "voro")]
pub mod ffi {
    unsafe extern "C++" {
        include!("voro_rs/cpp/boilerplate.hh");

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
        // TODO: method from container_base
        fn clear(
            self: Pin<&mut container>,
        );
        fn put(
            self: Pin<&mut container>,
            n: i32,
            x: f64,
            y: f64,
            z: f64
        );
        // TODO: put with particle_order
        fn compute_all_cells(
            self: Pin<&mut container>,
        );
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
            pid: &mut i32
        ) -> bool;
        // TODO: compute_cell with c_loop
    }
}
