use rand::prelude::*;
use std::default::Default;

use crate::astronomy::distant_binary_star::constraints::Constraints as DistantBinaryStarConstraints;
use crate::astronomy::planetary_system::constraints::Constraints as PlanetarySystemConstraints;
use crate::astronomy::star_subsystem::error::Error;
use crate::astronomy::star_subsystem::Subsystem;

/// Constraints for creating a main-sequence star subsystem.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {}

impl Constraints {
  /// Generate a habitable star subsystem.
  #[named]
  pub fn habitable() -> Self {
    trace_enter!();
    let result = Self {
      ..Constraints::default()
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Generate.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<Subsystem, Error> {
    trace_enter!();
    use Subsystem::*;
    let generate_planetary_system: bool = rng.gen();
    let result;
    if generate_planetary_system {
      let constraints = PlanetarySystemConstraints::default();
      result = PlanetarySystem(constraints.generate(rng)?);
    } else {
      let constraints = DistantBinaryStarConstraints::default();
      result = DistantBinaryStar(constraints.generate(rng)?);
    }
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    Self {}
  }
}
