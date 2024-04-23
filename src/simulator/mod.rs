use itertools::Itertools;

use crate::{data::first_ionisation_energy, forces::dispersion_force};

use self::position::Position;

pub mod object;
pub mod position;
pub mod simulator;

pub struct SimSpace {
    scenario: Scenario,
    objects: Vec<Position>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scenario {
    Empty,
    Atoms,
    HydrogenHalide,
    Halogenoalkane,
}

impl SimSpace {
    pub fn simulate(&mut self) {

    }

    pub fn simulate_atoms(&mut self) {
        let atomic_number = 42;
        
        let objects = self.objects.as_slice()
            .iter()
            .tuple_combinations::<(_, _)>()
            .collect::<Vec<_>>();

        objects
            .iter()
            .for_each(|&(lhs, rhs)| {
            
                let distance = lhs.distance_from(rhs);
                let forces = calculate_forces(atomic_number, distance);
            })
    }
    
    pub fn simulate_hydrogen_halides(&mut self) {}
    pub fn simulate_halogenoalkanes(&mut self) {} 
}

fn calculate_forces(atomic_number: u8, distance: f64) -> Vec<f64> {
    let polarisability = todo!();

    let dispersion = dispersion_force(polarisability, first_ionisation_energy(atomic_number.try_into().unwrap()), distance.into());

    vec![]
}