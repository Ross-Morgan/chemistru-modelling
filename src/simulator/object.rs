use crate::molecule::CovalentMolecule;

use super::position::Position;

pub struct SimObject {
    inner: Object,
    position: Position,
    born: usize,
    age: usize,
}

pub enum Object {
    Molecule(CovalentMolecule),
    Atom,
}
