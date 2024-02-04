use std::f64::consts::PI;

use crate::standard_form::StandardForm;

const R: StandardForm = StandardForm::new_const(8.31446261815324, 0);
const BOLTZMANN: StandardForm = StandardForm::new_const(1.380649, -23);

pub(crate) fn dispersion_force(
    polarisability: StandardForm,
    first_ionisation_energy: StandardForm,
    distance: StandardForm,
) -> StandardForm {
    -1.0 * (StandardForm::from(3.0) * polarisability.powf(2.0) * first_ionisation_energy) / (StandardForm::from(4.0) * distance.powf(6.0))
}

pub(crate) fn dipole_dipole_force(
    dipole_moment_a: StandardForm,
    dipole_moment_b: StandardForm,
    permittivity: StandardForm,
    temperature: StandardForm,
    distance: StandardForm,
) -> StandardForm {
    -1.0 * (2.0 * dipole_moment_a.powf(2.0) * dipole_moment_b.powf(2.0))
        / (3.0 * (4.0 * PI * permittivity).powf(2.0) * distance.powf(6.0) * BOLTZMANN * temperature)
}

pub(crate) fn ion_dipole_force(ion_charge: StandardForm, dipole_moment: StandardForm, permittivity: StandardForm, distance: StandardForm) -> StandardForm {
    (-1.0) * (ion_charge * dipole_moment) / ((4.0 * PI * permittivity).powf(2.0) * distance.powf(2.0))
}

pub(crate) fn molar_refractivity(
    temperature: StandardForm,
    pressure: StandardForm,
    refractive_index: StandardForm,
) -> StandardForm {
    0.0 - (R * temperature) / pressure * (refractive_index.powf(2.0) - StandardForm::from(1.0))
        / (refractive_index.powf(2.0) + 2.0)
}
