use rand::prelude::*;
use std::default::Default;

use crate::astronomy::star::Star;

use crate::astronomy::close_binary_star::constants::MAXIMUM_AVERAGE_SEPARATION;
use crate::astronomy::close_binary_star::constants::MAXIMUM_COMBINED_MASS;
use crate::astronomy::close_binary_star::constants::MAXIMUM_HABITABLE_COMBINED_MASS;
use crate::astronomy::close_binary_star::constants::MAXIMUM_HABITABLE_INDIVIDUAL_MASS;
use crate::astronomy::close_binary_star::constants::MAXIMUM_HABITABLE_SEPARATION;
use crate::astronomy::close_binary_star::constants::MAXIMUM_INDIVIDUAL_MASS;
use crate::astronomy::close_binary_star::constants::MAXIMUM_ORBITAL_ECCENTRICITY;
use crate::astronomy::close_binary_star::constants::MINIMUM_AVERAGE_SEPARATION;
use crate::astronomy::close_binary_star::constants::MINIMUM_COMBINED_MASS;
use crate::astronomy::close_binary_star::constants::MINIMUM_HABITABLE_AGE;
use crate::astronomy::close_binary_star::constants::MINIMUM_HABITABLE_COMBINED_MASS;
use crate::astronomy::close_binary_star::constants::MINIMUM_HABITABLE_INDIVIDUAL_MASS;
use crate::astronomy::close_binary_star::constants::MINIMUM_INDIVIDUAL_MASS;
use crate::astronomy::close_binary_star::constants::MINIMUM_MAIN_SEQUENCE_STAR_MASS;
use crate::astronomy::close_binary_star::constants::MINIMUM_ORBITAL_ECCENTRICITY;

use crate::astronomy::close_binary_star::error::Error;
use crate::astronomy::close_binary_star::CloseBinaryStar;

/// Constraints for creating a main-sequence star.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The minimum combined mass of the stars, in Msol.
  pub minimum_combined_mass: Option<f64>,
  /// The maximum combined mass of the stars, in Msol.
  pub maximum_combined_mass: Option<f64>,
  /// The minimum individual mass of the stars, in Msol.
  pub minimum_individual_mass: Option<f64>,
  /// The maximum individual mass of the stars, in Msol.
  pub maximum_individual_mass: Option<f64>,
  /// The minimum separation between the stars, in Msol.
  pub minimum_average_separation: Option<f64>,
  /// The maximum separation between the stars, in Msol.
  pub maximum_average_separation: Option<f64>,
  /// The minimum orbital eccentricity.
  pub minimum_orbital_eccentricity: Option<f64>,
  /// The maximum orbital_eccentricity.
  pub maximum_orbital_eccentricity: Option<f64>,
  /// The minimum age of the stars, in Gyr.
  pub minimum_age: Option<f64>,
  /// The maximum age of the stars, in Gyr.
  pub maximum_age: Option<f64>,
  /// Enforce habitability.
  pub enforce_habitability: bool,
}

impl Constraints {
  /// Generate a habitable binary star.
  #[named]
  pub fn habitable() -> Self {
    trace_enter!();
    let minimum_combined_mass = Some(MINIMUM_HABITABLE_COMBINED_MASS);
    let maximum_combined_mass = Some(MAXIMUM_HABITABLE_COMBINED_MASS);
    let minimum_individual_mass = Some(MINIMUM_HABITABLE_INDIVIDUAL_MASS);
    let maximum_individual_mass = Some(MAXIMUM_HABITABLE_INDIVIDUAL_MASS);
    let maximum_average_separation = Some(MAXIMUM_HABITABLE_SEPARATION);
    let minimum_age = Some(MINIMUM_HABITABLE_AGE);
    let enforce_habitability = true;
    let result = Self {
      minimum_combined_mass,
      maximum_combined_mass,
      minimum_individual_mass,
      maximum_individual_mass,
      maximum_average_separation,
      minimum_age,
      enforce_habitability,
      ..Constraints::default()
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Generate a binary star from our constraints.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<CloseBinaryStar, Error> {
    trace_enter!();
    let mut minimum_combined_mass = self.minimum_combined_mass.unwrap_or(MINIMUM_COMBINED_MASS);
    trace_var!(minimum_combined_mass);
    let maximum_combined_mass = self.maximum_combined_mass.unwrap_or(MAXIMUM_COMBINED_MASS);
    trace_var!(maximum_combined_mass);
    let minimum_individual_mass = self.minimum_individual_mass.unwrap_or(MINIMUM_INDIVIDUAL_MASS);
    trace_var!(minimum_individual_mass);
    let maximum_individual_mass = self.maximum_individual_mass.unwrap_or(MAXIMUM_INDIVIDUAL_MASS);
    trace_var!(maximum_individual_mass);
    let minimum_orbital_eccentricity = self
      .minimum_orbital_eccentricity
      .unwrap_or(MINIMUM_ORBITAL_ECCENTRICITY);
    trace_var!(minimum_orbital_eccentricity);
    let maximum_orbital_eccentricity = self
      .maximum_orbital_eccentricity
      .unwrap_or(MAXIMUM_ORBITAL_ECCENTRICITY);
    trace_var!(maximum_orbital_eccentricity);
    let minimum_average_separation = self.minimum_average_separation.unwrap_or(MINIMUM_AVERAGE_SEPARATION);
    trace_var!(minimum_average_separation);
    let maximum_average_separation = self.maximum_average_separation.unwrap_or(MAXIMUM_AVERAGE_SEPARATION);
    trace_var!(maximum_average_separation);
    let orbital_eccentricity = rng.gen_range(minimum_orbital_eccentricity..maximum_orbital_eccentricity);
    trace_var!(orbital_eccentricity);
    let average_separation = rng.gen_range(minimum_average_separation..maximum_average_separation);
    trace_var!(average_separation);
    let combined_mass;
    let primary_mass;
    let secondary_mass;
    if self.enforce_habitability {
      minimum_combined_mass =
        (1.1 * (4.0 * maximum_average_separation * (1.0 + orbital_eccentricity)).powf(2.0)).powf(1.0 / 4.0);
    }
    let (primary, secondary) = {
      combined_mass = rng.gen_range(minimum_combined_mass..maximum_combined_mass);
      let half = combined_mass / 2.0;
      let top = combined_mass - MINIMUM_MAIN_SEQUENCE_STAR_MASS;
      primary_mass = rng.gen_range(half..top);
      secondary_mass = combined_mass - primary_mass;
      let primary = Star::from_mass(rng, primary_mass)?;
      let secondary = Star::from_mass(rng, secondary_mass)?;
      (primary, secondary)
    };
    trace_var!(primary);
    trace_var!(secondary);
    let result = CloseBinaryStar::from_stars(rng, primary, secondary, average_separation, orbital_eccentricity)?;
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  #[named]
  fn default() -> Self {
    trace_enter!();
    let minimum_combined_mass = Some(MINIMUM_COMBINED_MASS);
    let maximum_combined_mass = Some(MAXIMUM_COMBINED_MASS);
    let minimum_individual_mass = Some(MINIMUM_INDIVIDUAL_MASS);
    let maximum_individual_mass = Some(MAXIMUM_INDIVIDUAL_MASS);
    let minimum_average_separation = None;
    let maximum_average_separation = None;
    let minimum_orbital_eccentricity = Some(MINIMUM_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_ORBITAL_ECCENTRICITY);
    let minimum_age = None;
    let maximum_age = None;
    let enforce_habitability = false;
    let result = Self {
      minimum_combined_mass,
      maximum_combined_mass,
      minimum_individual_mass,
      maximum_individual_mass,
      minimum_average_separation,
      maximum_average_separation,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      minimum_age,
      maximum_age,
      enforce_habitability,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_habitable() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let binary = &Constraints::habitable().generate(&mut rng)?;
    trace_var!(binary);
    print_var!(binary);
    trace_exit!();
    Ok(())
  }

  #[named]
  #[test]
  pub fn test_habitable_bulk() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let mut success = 0;
    let trials = 10;
    let mut counter = 0;
    loop {
      match &Constraints::habitable().generate(&mut rng) {
        Ok(_binary) => success += 1,
        Err(error) => println!("ERROR: {:#?}", error),
      }
      counter += 1;
      if counter >= trials {
        break;
      }
    }
    assert_eq!(success, trials);
    trace_exit!();
    Ok(())
  }
}