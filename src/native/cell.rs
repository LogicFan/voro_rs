//! Voronoi Cell

use super::{constant, Float, Float3};

/// A class represent a Voronoi cell without neighbor information.
///
/// This class is an extension of the voronoicell_base class, in cases when
/// is not necessary to track the IDs of neighboring particles associated
/// with each face of the Voronoi cell.
#[derive(Debug)]
pub struct CellSgl {
    /// `tol`, and `tol_cu = tol^(3/2)`
    δ: Float,
    /// big_tol
    Δ: Float,

    /// `pts`, and `vertices.len() = current_vertices`
    vertices: Vec<Float3>,

    /// represent `ed`, `nu`, and `current_vertices`.
    /// `edges[i][j] = ed[i][j]`
    /// `edges.len() = current_vertices`
    /// `edges[i].len()` = nu[i]
    ed: Vec<Vec<usize>>,

    mep: Vec<Vec<usize>>,
}

impl CellSgl {
    /// Create an empty cell
    /// 
    /// - `max_len`: the maximum possible length for two cells 
    /// in the container. This is important for tolerance calculation.
    pub(crate) fn new_empty(max_len: Float) -> Self {
        Self {
            δ: max_len.powi(2) * constant::δ,
            Δ: max_len.powi(2) * constant::Δ,

            vertices: Vec::new(),
            ed: Vec::new(),
            mep: Vec::new(),
        }
    }
}
