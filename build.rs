fn main() {
    cxx_build::bridges([
        "src/cell.rs",
        "src/container.rs",
        "src/wall.rs",
        "src/wall_list.rs",
    ])
    .file("cpp/voro++.cc")
    .cpp(true)
    .std("c++14")
    .warnings(false)
    .compile("voro++");
}
