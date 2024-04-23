pub mod molecule;
pub mod simulator;
pub mod standard_form;

mod data;

#[rustfmt::skip]
#[allow(unused)]
mod forces;

const PERMEABILITY: StandardForm<f64> = StandardForm(4 * std::f64::consts::PI, -7);
const PERMITTIVITY: StandardForm<f64> = StandardForm(8.85, -12);

struct Object;

struct Simulator {
    object_pool: Vec<Object>,
    time: u64,
}

impl Simulator {
    fn new() -> Self {
        Self {
            object_pool: vec![],
            time: 0,
        }
    }

    fn run_indefinitely(&mut self) -> ! {
        loop {
            &self.simulate_tick();
            self.time += 1
        }
    }

    fn simulate_tick(&mut self) {

    }
}