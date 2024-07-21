mod edges;
mod sgl;

use edges::Edges;
use glam::DVec3;

pub trait VoronoiCell {
    fn translate(&mut self, translation: DVec3);

    fn volume(&mut self) -> f64;
}
