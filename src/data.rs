use std::num::NonZeroU8;

use crate::standard_form::StandardForm;

#[inline]
pub const fn first_ionisation_energy(element: NonZeroU8) -> StandardForm {
    match element.get() {
        1 => StandardForm::new_const(1.3120, 3),
        2 => StandardForm::new_const(2.3723, 3),
        3 => StandardForm::new_const(5.202, 2),
        4 => StandardForm::new_const(8.995, 2),
        5 => StandardForm::new_const(8.006, 2),
        6 => StandardForm::new_const(1.0865, 3),
        7 => StandardForm::new_const(1.4023, 3),
        8 => StandardForm::new_const(1.3139, 3),
        9 => StandardForm::new_const(1.6810, 3),
        _ => StandardForm::new_const(0.0001, 0),
    }
}
