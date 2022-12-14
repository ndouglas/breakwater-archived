use rand::prelude::*;

use crate::astronomy::gas_giant_planet::constraints::Constraints as GasGiantPlanetConstraints;
use crate::astronomy::host_star::HostStar;
use crate::astronomy::planet::error::Error;
use crate::astronomy::planet::Planet;
use crate::astronomy::terrestrial_planet::constraints::Constraints as TerrestrialPlanetConstraints;

/// Constraints for creating a planet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// Gas Giant planet constraints.
  pub gas_giant_planet_constraints: Option<GasGiantPlanetConstraints>,
  /// Terrestrial planet constraints.
  pub terrestrial_planet_constraints: Option<TerrestrialPlanetConstraints>,
}

impl Constraints {
  /// Aim for a habitable planet.
  pub fn habitable() -> Self {
    let terrestrial_planet_constraints = Some(TerrestrialPlanetConstraints::habitable());
    Self {
      terrestrial_planet_constraints,
      ..Constraints::default()
    }
  }

  /// Generate.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R, host_star: &HostStar, distance: f64) -> Result<Planet, Error> {
    trace_enter!();
    use Planet::*;
    let result = {
      if distance >= host_star.get_frost_line() {
        let constraints = self
          .gas_giant_planet_constraints
          .unwrap_or(GasGiantPlanetConstraints::default());
        trace_var!(constraints);
        GasGiantPlanet(constraints.generate(rng, host_star, distance)?)
      } else {
        let constraints = self
          .terrestrial_planet_constraints
          .unwrap_or(TerrestrialPlanetConstraints::default());
        trace_var!(constraints);
        TerrestrialPlanet(constraints.generate(rng, host_star, distance)?)
      }
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let gas_giant_planet_constraints = None;
    let terrestrial_planet_constraints = None;
    Self {
      gas_giant_planet_constraints,
      terrestrial_planet_constraints,
    }
  }
}

#[cfg(test)]
pub mod test {

  use crate::astronomy::host_star::constraints::Constraints as HostStarConstraints;
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
    let host_star = &HostStarConstraints::default().generate(&mut rng)?;
    let habitable_zone = host_star.get_habitable_zone();
    let distance = rng.gen_range(habitable_zone.0..habitable_zone.1);
    let planet = &Constraints::default().generate(&mut rng, &host_star, distance)?;
    trace_var!(planet);
    print_var!(planet);
    trace_exit!();
    Ok(())
  }

  #[named]
  #[test]
  pub fn test_habitable() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let host_star = &HostStarConstraints::habitable().generate(&mut rng)?;
    let habitable_zone = host_star.get_habitable_zone();
    let distance = rng.gen_range(habitable_zone.0..habitable_zone.1);
    let planet = &Constraints::habitable().generate(&mut rng, &host_star, distance)?;
    trace_var!(planet);
    print_var!(planet);
    trace_exit!();
    Ok(())
  }
}
