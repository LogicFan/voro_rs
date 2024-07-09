#[cxx::bridge(namespace="voro")]
pub mod ffi {
    unsafe extern "C++" {
        include!("voro_rs/cpp/voro++.hh");
        include!("voro_rs/cpp/boilerplate.hh");

        type voronoicell;
        fn construct() -> UniquePtr<voronoicell>;
    }
}