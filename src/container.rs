#[cxx::bridge(namespace = "voro")]
pub mod ffi {
    unsafe extern "C++" {
        include!("voro_rs/cpp/voro++.hh");
        include!("voro_rs/cpp/boilerplate.hh");

        type container;
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
    }
}
