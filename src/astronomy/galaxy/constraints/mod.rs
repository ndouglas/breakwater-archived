use rand::prelude::*;

use crate::astronomy::galaxy::error::*;
use crate::astronomy::galaxy::Galaxy;
use crate::astronomy::stellar_neighborhood::constraints::Constraints as StellarNeighborhoodConstraints;

/// Constraints for creating a galaxy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// Any constraints placed on the various neighbors.
  pub stellar_neighborhood_constraints: Option<StellarNeighborhoodConstraints>,
}

impl Constraints {
  /// Generate a habitable galaxy.
  pub fn habitable() -> Self {
    let stellar_neighborhood_constraints = Some(StellarNeighborhoodConstraints::habitable());
    Self {
      stellar_neighborhood_constraints,
      ..Constraints::default()
    }
  }

  /// Generate a random stellar neighborhood with the specified constraints.
  ///
  /// This may or may not be habitable.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<Galaxy, Error> {
    trace_enter!();
    let stellar_neighborhood_constraints = self
      .stellar_neighborhood_constraints
      .unwrap_or(StellarNeighborhoodConstraints::default());
    trace_var!(stellar_neighborhood_constraints);
    let stellar_neighborhood = stellar_neighborhood_constraints.generate(rng)?;
    let result = Galaxy { stellar_neighborhood };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let stellar_neighborhood_constraints = Some(StellarNeighborhoodConstraints::default());
    Self {
      stellar_neighborhood_constraints,
    }
  }
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn get_random() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let constraints = Constraints::habitable();
    let galaxy = constraints.generate(&mut rng)?;
    info_var!(galaxy);
    print_var!(galaxy);
    trace_exit!();
    Ok(())
  }
}
