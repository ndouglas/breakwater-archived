use rand::prelude::*;
use rand_distr::{Distribution, LogNormal};

use crate::astronomy::gas_giant_planet::constants::*;
use crate::astronomy::gas_giant_planet::error::Error;
use crate::astronomy::gas_giant_planet::GasGiantPlanet;
use crate::astronomy::host_star::HostStar;

/// Constraints for creating a planet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The minimum mass.
  pub minimum_mass: Option<f64>,
  /// The maximum mass.
  pub maximum_mass: Option<f64>,
}

impl Constraints {
  /// Generate.
  #[named]
  pub fn generate<R: Rng + ?Sized>(
    &self,
    rng: &mut R,
    _host_star: &HostStar,
    distance: f64,
  ) -> Result<GasGiantPlanet, Error> {
    trace_enter!();
    let minimum_mass = self.minimum_mass.unwrap_or(MINIMUM_MASS);
    trace_var!(minimum_mass);
    let maximum_mass = self.maximum_mass.unwrap_or(MAXIMUM_MASS);
    trace_var!(maximum_mass);
    let log_normal = LogNormal::new(0.2, 0.5).unwrap();
    let mass = log_normal.sample(rng);
    trace_var!(mass);
    let mut result = GasGiantPlanet::from_mass(mass)?;
    result.semi_major_axis = distance;
    let orbital_eccentricity = 0.0167;
    result.orbital_eccentricity = orbital_eccentricity;
    trace_var!(orbital_eccentricity);
    let perihelion = (1.0 - orbital_eccentricity) * distance;
    result.perihelion = perihelion;
    trace_var!(perihelion);
    let aphelion = (1.0 + orbital_eccentricity) * distance;
    result.aphelion = aphelion;
    trace_var!(aphelion);
    let orbital_period = distance.powf(3.0).sqrt();
    result.orbital_period = orbital_period;
    trace_var!(orbital_period);
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let minimum_mass = None;
    let maximum_mass = None;
    Self {
      minimum_mass,
      maximum_mass,
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
    let host_star_constraints = HostStarConstraints::habitable();
    let mut host_star = host_star_constraints.generate(&mut rng)?;
    trace_var!(host_star);
    let mut is_habitable = !host_star.is_habitable();
    let mut counter = 0;
    while !is_habitable && counter < 50 {
      host_star = host_star_constraints.generate(&mut rng)?;
      is_habitable = !host_star.is_habitable();
      counter += 1;
    }
    let habitable_zone = host_star.get_habitable_zone();
    trace_var!(habitable_zone);
    let distance = rng.gen_range(habitable_zone.0..habitable_zone.1);
    trace_var!(distance);
    let planet = &Constraints::default().generate(&mut rng, &host_star, distance)?;
    trace_var!(planet);
    print_var!(planet);
    trace_exit!();
    Ok(())
  }
}
