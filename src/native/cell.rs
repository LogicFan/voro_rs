//! Voronoi Cell

use super::Float3;

/// A class represent a Voronoi cell without neighbor information.
///
/// This class is an extension of the voronoicell_base class, in cases when
/// is not necessary to track the IDs of neighboring particles associated
/// with each face of the Voronoi cell.
#[derive(Debug)]
pub struct CellSgl {
    /// represent `ed`, `nu`, and `current_vertices`.
    /// `edges[i][j] = ed[i][j]`
    /// `edges.len() = current_vertices`
    /// `edges[i].len()` = nu[i]
    pub(crate) ed: Vec<Vec<usize>>,

    /// 
    pub(crate) pts: Vec<Float3>,
}
