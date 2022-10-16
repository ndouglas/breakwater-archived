use rand::prelude::*;

use crate::astronomy::planet::error::Error;
use crate::astronomy::planet::Planet;
use crate::astronomy::terrestrial_planet::constraints::Constraints as TerrestrialPlanetConstraints;

/// Constraints for creating a planet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// Terrestrial planet constraints.
  pub terrestrial_planet_constraints: Option<TerrestrialPlanetConstraints>,
}

impl Constraints {
  /// Generate.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<Planet, Error> {
    trace_enter!();
    let constraints = self
      .terrestrial_planet_constraints
      .unwrap_or(TerrestrialPlanetConstraints::default());
    trace_var!(constraints);
    let result = { Planet::TerrestrialPlanet(constraints.generate(rng)?) };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let terrestrial_planet_constraints = None;
    Self {
      terrestrial_planet_constraints,
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
  pub fn test_generate() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let planet = &Constraints::default().generate(&mut rng)?;
    trace_var!(planet);
    print_var!(planet);
    trace_exit!();
    Ok(())
  }
}
