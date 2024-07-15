mod constant;
mod cell;

#[cfg(feature = "f32")]
type Float = f32;
#[cfg(feature = "f64")]
type Float = f64;

#[cfg(feature = "f32")]
type Float3 = glam::Vec3A;
#[cfg(feature = "f64")]
type Float3 = glam::DVec3;

type Bool3 = glam::BVec3;
type Int3 = glam::IVec3;
