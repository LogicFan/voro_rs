# voro_rs

A Rust binding for voro++ library.

This binding has been tested on Windows, Linux and MacOS.

## Progress

Here is the list of classes/functions plan to have a high-level binding in rust.

- [x] `voronoicell`/`voronoicell_neighbor`
- [x] `wall_sphere`/`wall_plane`/`wall_cylinder`/`wall_cone`
- [x] `wall_list`
- [ ] `container`/`container_poly`
- [ ] `c_loop_all`/`c_loop_subset`/`c_loop_order`
- [ ] `particle_order`
- [ ] `container_periodic`/`container_periodic_poly`
- [ ] `c_loop_all_periodic`/`c_loop_order_periodic`
- [ ] `pre_container`/`pre_container_poly`

## About voro++

Voro++ is a open source software library for the computation of the Voronoi diagram, a widely-used tessellation that has applications in many scientific fields. Read more details [here](https://math.lbl.gov/voro++/about.html).
