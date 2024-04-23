pub struct Position(f64, f64, f64);

impl Position {
    #[inline]
    pub fn distance_from(self, rhs: &Self) -> f64 {
        ((rhs.0 - self.0).powi(2) + (rhs.1 - self.1).powi(2) + (rhs.2 - self.2).powi(2)).sqrt()
    }
}