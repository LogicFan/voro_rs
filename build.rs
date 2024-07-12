fn main() {
    cxx_build::bridges([
        "src/cell.rs",
        "src/container.rs",
        "src/container_loop.rs",
        "src/wall.rs",
        "src/wall_list.rs",
        "src/particle_marker.rs",
    ])
    .file("cpp/voro++.cc")
    .cpp(true)
    .std("c++14")
    .warnings(false)
    .compile("voro++");
}
