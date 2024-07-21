//! Voronoi Cell

use super::{constant, Float, Float3};

/// A class represent a Voronoi cell without neighbor information.
///
/// This class is an extension of the voronoicell_base class, in cases when
/// is not necessary to track the IDs of neighboring particles associated
/// with each face of the Voronoi cell.
#[derive(Debug)]
pub struct CellSgl {
    /// tol = small_tolerance
    /// tol_cu = small_tolerance^(3/2)
    small_tolerance: Float,
    /// big_tol = large_tolerance
    large_tolerance: Float,

    /// pts[i] = vertices[i]
    /// current_vertices = vertices.capacity()
    /// p = vertices.len()
    vertices: Vec<Float3>,

    /// ed[i][j] = edges[i][j]
    /// nu[i] = edges[i].capacity() 
    /// current_vertices = edges.capacity()
    edges: Vec<Vec<usize>>,

    /// mep[i][j] = edges2[i][j]
    /// mec[i] = edges2[i].len()
    /// mem[i] = edges2[i].capacity()
    /// current_vertex_order = edges2.capacity()
    edges2: Vec<Vec<usize>>,

    mask: Vec<u32>,

    up: i32,

    delete_stack: Vec<i32>,
    delete_stack2: Vec<i32>,
    search_stack: Vec<i32>,
}

impl CellSgl {
    /// Create an empty cell
    /// 
    /// - `max_len_sq`: the square of the maximum possible length for two cells 
    /// in the container. This is important for tolerance calculation.
    pub(crate) fn new_empty(max_len_sq: Float) -> Self {
        Self {
            small_tolerance: max_len_sq * constant::small_tolerance,
            large_tolerance: max_len_sq * constant::large_tolerance,

            vertices: Vec::new(),
            edges: Vec::new(),
            edges2: Vec::new(),
            mask: Vec::new(),
            up: 0,
            delete_stack: Vec::new(),
            delete_stack2: Vec::new(),
            search_stack: Vec::new()
        }
    }

    pub fn new_cuboid(mut xyz_min: Float3, mut xyz_max: Float3, max_len_sq: Float) -> Self {
        let mut cell = Self::new_empty(max_len_sq);
        xyz_min *= 2.0;
        xyz_max *= 2.0;
        
        cell.up = 0;

        cell.vertices.resize(8, Float3::ZERO);
        cell.vertices.push(Float3::new(xyz_min.x, xyz_min.y, xyz_min.z));
        cell.vertices.push(Float3::new(xyz_max.x, xyz_min.y, xyz_min.z));
        cell.vertices.push(Float3::new(xyz_min.x, xyz_max.y, xyz_min.z));
        cell.vertices.push(Float3::new(xyz_max.x, xyz_max.y, xyz_min.z));
        cell.vertices.push(Float3::new(xyz_min.x, xyz_min.y, xyz_max.z));
        cell.vertices.push(Float3::new(xyz_max.x, xyz_min.y, xyz_max.z));
        cell.vertices.push(Float3::new(xyz_min.x, xyz_max.y, xyz_max.z));
        cell.vertices.push(Float3::new(xyz_max.x, xyz_max.y, xyz_max.z));

        cell.edges2.resize(, value)


        cell.edges2[3].push(1);
        cell.edges2[3].push(4);
        cell.edges2[3].push(2);
        cell.edges2[3].push(2);
        cell.edges2[3].push(1);
        cell.edges2[3].push(0);
        cell.edges2[3].push(1);
        cell.edges2[3].push(1);
        cell.edges2[3].push(1);
        cell.edges2[3].push(1);
        cell.edges2[3].push(1);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn invariant()

    #[test]
    fn container_std_test() {
        let xyz_min = [-10.0, -10.0, -10.0];
        let xyz_max = [10.0, 10.0, 10.0];
        let is_periodic = [false, false, false];

        let mut pc = PreContainerStd::new(
            xyz_min,
            xyz_max,
            is_periodic,
        );
        let grids = pc.optimal_grids();

        pc.put(0, [0.0, 0.0, 0.0], 0.0);
        pc.put(1, [1.0, 0.0, 0.0], 0.0);
        pc.put(2, [2.0, 0.0, 0.0], 0.0);
        pc.put(3, [3.0, 0.0, 0.0], 0.0);
        pc.put(4, [4.0, 0.0, 0.0], 0.0);
        pc.put(5, [4.0, 1.0, 0.0], 0.0);
        pc.put(6, [4.0, 2.0, 0.0], 0.0);
        pc.put(7, [4.0, 3.0, 0.0], 0.0);
        pc.put(8, [4.0, 4.0, 0.0], 0.0);
        assert_eq!(pc.total_particles(), 9);

        let mut con = ContainerStd::new(
            xyz_min,
            xyz_max,
            grids,
            is_periodic,
        );
        pc.setup(&mut con);
        assert_eq!(con.total_particles(), 9);
        assert_eq!(con.sum_cell_volumes(), 8000.0);

        let c = con.find_voronoi_cell([4.0, 4.0, 0.0]);
        assert!(c.is_some());
        assert_eq!(c.unwrap().0, 8);
    }
}
