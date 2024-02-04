use std::{
    fmt::Debug,
    ops::{Add, Div /* AddAssign, SubAssign, MulAssign, DivAssign */, Mul, Sub},
};

#[derive(Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct StandardForm(f64, i32);

impl Debug for StandardForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:.5}x10{}", self.0, to_superscript(self.1)))
    }
}

impl Add for StandardForm {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self.1.cmp(&rhs.1) {
            std::cmp::Ordering::Equal => Self::new(self.0 + rhs.0, self.1),
            std::cmp::Ordering::Greater => Self::new(
                self.0 + (rhs.0 * 10.0f64.powf((self.1 - rhs.1) as f64).recip()),
                self.1,
            ),
            std::cmp::Ordering::Less => Self::new(
                rhs.0 + (self.0 * 10.0f64.powf((rhs.1 - self.1) as f64).recip()),
                rhs.1,
            ),
        }
    }
}

impl Add<StandardForm> for f64 {
    type Output = StandardForm;

    fn add(self, rhs: StandardForm) -> Self::Output {
        rhs + StandardForm::from(self)
    }
}

impl Add<f64> for StandardForm {
    type Output = StandardForm;

    fn add(self, rhs: f64) -> Self::Output {
        self + StandardForm::from(rhs)
    }
}

impl Sub for StandardForm {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let order_diff = rhs.1 - self.1;

        match order_diff {
            0 => Self::new(self.0 - rhs.0, self.1),
            d @ i32::MIN..=-1 => {
                // Self has higher order
                let modified = rhs.0 * 10.0f64.powi(d);
                Self::new(self.0 - modified, self.1)
            }
            d @ 0.. => {
                // rhs has higher order
                let modified = self.0 * 10.0f64.powi(-d);
                Self::new(modified - rhs.0, rhs.1)
            }
        }
    }
}

impl Sub<StandardForm> for f64 {
    type Output = StandardForm;

    fn sub(self, rhs: StandardForm) -> Self::Output {
        StandardForm::from(self) - rhs
    }
}

impl<T> Mul<T> for StandardForm
where
    StandardForm: From<T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let other = StandardForm::from(rhs);

        Self::new(self.0 * other.0, self.1 + other.1)
    }
}

impl<T> Div<T> for StandardForm
where
    StandardForm: From<T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let other = StandardForm::from(rhs);

        Self::new(self.0 / other.0, self.1 / other.1)
    }
}

impl Mul<StandardForm> for f64 {
    type Output = StandardForm;

    fn mul(self, rhs: StandardForm) -> Self::Output {
        rhs * self
    }
}

impl From<f64> for StandardForm {
    fn from(value: f64) -> Self {
        let power = value.log10().floor();

        Self(value / power, power as i32)
    }
}

impl StandardForm {
    pub const fn new_const(num: f64, order: i32) -> Self {
        Self(num, order)
    }

    pub fn new(mut num: f64, mut order: i32) -> Self {
        while num.abs() >= 10.0 {
            num /= 10.0;
            order += 1;
        }

        while num.abs() <= 0.1 {
            num *= 10.0;
            order -= 1;
        }

        Self(num, order)
    }

    pub fn powf(self, power: f64) -> Self {
        let new_power = power * (self.1 as f64);
        let int_part = new_power.floor();
        let frac_part = new_power % 1.0;

        dbg!(self);
        dbg!(power);
        dbg!(int_part);
        dbg!(frac_part);

        Self::new(
            self.0.powf(power) * 10.0f64.powf(frac_part),
            int_part as i32,
        )
    }
}

#[macro_export]
macro_rules! sf {
    ($number:literal e $pow:literal) => {
        $crate::standard_form::StandardForm::new($number as f64, $pow)
    };
}

fn to_superscript(n: i32) -> String {
    n.to_string()
        .chars()
        .map(|c| match c {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            _ => panic!("Invalid char"),
        })
        .collect::<String>()
}
