use rand::prelude::*;

pub mod constants;
use constants::*;
pub mod constraints;
pub mod error;
use error::*;
pub mod math;
use math::color::star_mass_to_rgb;
use math::luminosity::star_mass_to_luminosity;
use math::radius::star_mass_to_radius;
use math::satellite_zone::{get_approximate_innermost_orbit, get_approximate_outermost_orbit};
use math::spectral_class::star_mass_to_spectral_class;
use math::temperature::star_mass_to_temperature;
pub mod name;
use name::generate_star_name;

/// The `Star` type.
///
/// This is intended to encompass the most useful information we can generate
/// about main-sequence stars.  Other types will use different structs; it's
/// useful to view and treat these as the default sense of "star", given their
/// centrality to our purpose.
#[derive(Clone, Debug, PartialEq)]
pub struct Star {
  /// Type, Decile, Luminosity class.
  pub class: String,
  /// Measured in Msol.
  pub mass: f64,
  /// Measured in Kelvin.
  pub temperature: f64,
  /// Measured in Rsol.
  pub radius: f64,
  /// Measured in Lsol.
  pub luminosity: f64,
  /// Measured in Gyr.
  pub life_expectancy: f64,
  /// Measured in Gyr.
  pub current_age: f64,
  /// Measured in Dsol.
  pub density: f64,
  /// Habitable zone, measured in AU.
  pub habitable_zone: (f64, f64),
  /// Minimum and maximum sustainable distance for satellites, measured in AU.
  /// This is inferior to computing the Roche limit and Hill sphere, but we
  /// don't have enough information for that yet.
  pub satellite_zone: (f64, f64),
  /// The frost line, measured in AU.
  pub frost_line: f64,
  /// The absolute color of this star in SRGB.
  pub absolute_rgb: (u8, u8, u8),
  /// A generated name for this star.
  pub name: String,
}

/// Implementation of Star.
impl Star {
  /// Generate a random main-sequence star from a given mass.
  #[named]
  pub fn from_mass<R: Rng + ?Sized>(rng: &mut R, mass: f64) -> Result<Star, Error> {
    trace_enter!();
    trace_var!(mass);
    let temperature = star_mass_to_temperature(mass)?;
    trace_var!(temperature);
    let luminosity = star_mass_to_luminosity(mass)?;
    trace_var!(luminosity);
    let radius = star_mass_to_radius(mass)?;
    trace_var!(radius);
    let class = star_mass_to_spectral_class(mass)?;
    trace_var!(class);
    let life_expectancy = mass / luminosity * 10.0;
    trace_var!(life_expectancy);
    let lower_bound_age = 0.1 * life_expectancy;
    trace_var!(lower_bound_age);
    let upper_bound_age = 0.9 * life_expectancy;
    trace_var!(upper_bound_age);
    let current_age = rng.gen_range(lower_bound_age..upper_bound_age);
    trace_var!(current_age);
    let density = mass / radius.powf(3.0);
    trace_var!(density);
    let habitable_zone = ((luminosity / 1.1).sqrt(), (luminosity / 0.53).sqrt());
    trace_var!(habitable_zone);
    let satellite_inner_bound = get_approximate_innermost_orbit(mass);
    trace_var!(satellite_inner_bound);
    let satellite_outer_bound = get_approximate_outermost_orbit(mass);
    trace_var!(satellite_outer_bound);
    let satellite_zone = (satellite_inner_bound, satellite_outer_bound);
    let frost_line = 4.85 * luminosity.sqrt();
    trace_var!(frost_line);
    let absolute_rgb = star_mass_to_rgb(mass)?;
    trace_3u8!(absolute_rgb);
    let name = generate_star_name(rng);
    trace_var!(name);
    let result = Star {
      class,
      mass,
      luminosity,
      radius,
      temperature,
      life_expectancy,
      current_age,
      density,
      habitable_zone,
      satellite_zone,
      frost_line,
      absolute_rgb,
      name,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    trace_enter!();
    if self.mass < MINIMUM_HABITABLE_MASS {
      return Err(Error::MassTooLowToSupportLife);
    }
    if self.mass > MAXIMUM_HABITABLE_MASS {
      return Err(Error::MassTooHighToSupportLife);
    }
    if self.current_age < MINIMUM_HABITABLE_AGE {
      return Err(Error::TooYoungToSupportLife);
    }
    trace_exit!();
    Ok(())
  }

  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn is_habitable(&self) -> bool {
    trace_enter!();
    let result = match self.check_habitable() {
      Ok(()) => true,
      Err(_) => false,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::constraints::Constraints;
  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn get_random_main_sequence() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let star = Constraints::default().generate(&mut rng)?;
    trace_var!(star);
    print_var!(star);
    trace_exit!();
    Ok(())
  }
}
