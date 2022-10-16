use crate::astronomy::terrestrial_planet::TerrestrialPlanet;

pub mod constants;
pub mod constraints;
pub mod error;
use error::Error;
pub mod math;

/// The `Planet` class.  This will get complicated.
#[derive(Clone, Debug, PartialEq)]
pub enum Planet {
  /// Gas Giant Planet.
  /// Terrestrial Planet.
  TerrestrialPlanet(TerrestrialPlanet),
}

impl Planet {
  /// Indicate whether this planet is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    trace_enter!();
    use Planet::*;
    match &self {
      TerrestrialPlanet(terrestrial_planet) => terrestrial_planet.check_habitable()?,
    }
    let result = Ok(());
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this planet is capable of supporting conventional life.
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
