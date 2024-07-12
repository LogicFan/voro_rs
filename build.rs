fn main() {
    let mut voro = cc::Build::new();
    voro.cpp(true);
    voro.warnings(false);
    voro.file("cpp/voro++.cc");
    #[cfg(not(target_os = "windows"))]
    voro.flag("-ansi");
    #[cfg(not(target_os = "windows"))]
    voro.flag("-pedantic");
    voro.compile("voro++");

    let mut bridge = cxx_build::bridges([
        "src/cell.rs",
        "src/container.rs",
        "src/container_loop.rs",
        "src/wall.rs",
        "src/wall_list.rs",
        "src/particle_marker.rs",
    ]);
    bridge.cpp(true);
    bridge.warnings(false);
    bridge.compile("bridge");
}
