use rand::prelude::*;

use crate::astronomy::RADIUS_OF_STELLAR_NEIGHBORHOOD;
use crate::astronomy::get_random_point_in_sphere;
use crate::astronomy::AstronomicalError;
use crate::astronomy::Star;
use crate::astronomy::StarSystem;
use crate::astronomy::StarSystemStars;

pub mod constraints;
pub use constraints::*;

/// The `StellarNeighbor` class.
///
/// No, not someone who brings you brownies when you move into the area.
///
/// This is just a combination of a fully-fledged star system and a set of 3-D
/// coordinates so that we can place it relative to our primary star system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StellarNeighbor {
  /// Each coordinate (x,y,z) is a distance (in light years) from the origin.
  pub coordinates: (f64, f64, f64),
  /// The details of this particular star system.
  pub star_system: StarSystem,
}

impl StellarNeighbor {

  /// Generate a random stellar neighborhood with the specified constraints.
  ///
  /// This may or may not be habitable.
  #[named]
  pub fn get_random_constrained<R: Rng + ?Sized>(
    rng: &mut R,
    constraints: &StellarNeighborConstraints,
  ) -> Result<StellarNeighbor, AstronomicalError> {
    trace_enter!();
    let radius = constraints.radius.unwrap_or(RADIUS_OF_STELLAR_NEIGHBORHOOD);
    trace_var!(radius);
    let raw_coordinates = get_random_point_in_sphere(rng);
    trace_var!(raw_coordinates);
    let x = raw_coordinates.0 * radius;
    trace_var!(x);
    let y = raw_coordinates.1 * radius;
    trace_var!(y);
    let z = raw_coordinates.2 * radius;
    trace_var!(z);
    let coordinates = (x, y, z);
    trace_var!(coordinates);
    let star_system = StarSystem {
      stars: StarSystemStars::Solitary(Star::get_random_habitable(rng)?),
    };
    trace_var!(star_system);
    let result = StellarNeighbor {
      coordinates,
      star_system,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn get_random() -> Result<(), AstronomicalError> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let constraints = StellarNeighborConstraints {
      radius: Some(RADIUS_OF_STELLAR_NEIGHBORHOOD),
    };
    let stellar_neighbor = StellarNeighbor::get_random_constrained(&mut rng, &constraints)?;
    trace_var!(stellar_neighbor);
    println!("{:#?}", stellar_neighbor);
    trace_exit!();
    Ok(())
  }

}
