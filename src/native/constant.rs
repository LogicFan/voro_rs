#![allow(non_upper_case_globals)]
use super::Float;

pub const small_tolerance: Float = 10.0 * Float::EPSILON;
pub const large_tolerance: Float = 20.0 * small_tolerance;
pub const large_number: Float = Float::MAX;

// const double tolerance=10.*std::numeric_limits<double>::epsilon();

// const double big_tolerance_fac=20.;

// const double default_length=1000.;

// /** A large number that is used in the computation. */
// const double large_number=std::numeric_limits<double>::max();

// /** A radius to use as a placeholder when no other information is available. */
// const double default_radius=0.5;

// /** The maximum number of shells of periodic images to test over. */
// const int max_unit_voro_shells=10;

// /** A guess for the optimal number of particles per block, used to set up the
//  * container grid. */
// const double optimal_particles=5.6;
