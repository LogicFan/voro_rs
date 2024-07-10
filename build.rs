fn main() {
    cxx_build::bridges([
        "src/cell.rs",
        "src/wall.rs",
        "src/container.rs",
    ])
    .file("cpp/voro++.cc")
    .cpp(true)
    .std("c++14")
    .warnings(false)
    .compile("voro++");
}
