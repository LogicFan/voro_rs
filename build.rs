fn main() {
    let mut voro = cc::Build::new();
    voro.cpp(true);
    voro.warnings(false);
    voro.file("voro/src/voro++.cc");
    #[cfg(not(target_os = "windows"))]
    voro.flag("-ansi");
    #[cfg(not(target_os = "windows"))]
    voro.flag("-pedantic");
    #[cfg(target_os = "macos")]
    voro.opt_level(0);
    voro.compile("voro++");

    let mut bridge = cxx_build::bridges([
        "src/cell.rs",
        "src/container.rs",
        "src/container_loop.rs",
        "src/particle_marker.rs",
        "src/pre_container.rs",
        "src/wall.rs",
        "src/wall_list.rs",
    ]);
    bridge.cpp(true);
    bridge.warnings(false);
    bridge.std("c++14");
    bridge.compile("bridge");
}
