use chess_backend::Colour;
use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Eval {
    Numeric(f32),
    Mate(usize, Colour),
    Infinity,
    NegInfinity,
}
impl Eval {
    pub fn max(self, other: Self) -> Self {
        if self >= other {
            self
        } else {
            other
        }
    }

    pub fn min(self, other: Self) -> Self {
        if self <= other {
            self
        } else {
            other
        }
    }
}
impl Add for Eval {
    type Output = Option<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        if let Self::Numeric(n1) = self {
            if let Self::Numeric(n2) = rhs {
                return Some(Self::Numeric(n1 + n2));
            }
        }
        None
    }
}
impl Add<f32> for Eval {
    type Output = Option<Self>;
    fn add(self, rhs: f32) -> Self::Output {
        self + Self::Numeric(rhs)
    }
}

impl Sub for Eval {
    type Output = Option<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        if let Self::Numeric(n1) = self {
            if let Self::Numeric(n2) = rhs {
                return Some(Self::Numeric(n1 - n2));
            }
        }
        None
    }
}
impl Sub<f32> for Eval {
    type Output = Option<Self>;
    fn sub(self, rhs: f32) -> Self::Output {
        self - Self::Numeric(rhs)
    }
}

impl Mul<f32> for Eval {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        if let Self::Numeric(inner) = self {
            Self::Numeric(inner * rhs)
        } else {
            self
        }
    }
}

impl PartialOrd for Eval {
    fn lt(&self, other: &Self) -> bool {
        !self.ge(other)
    }
    fn le(&self, other: &Self) -> bool {
        !self.gt(other)
    }
    fn gt(&self, other: &Self) -> bool {
        match self {
            Self::Numeric(val_self) => match other {
                Self::Numeric(val_other) => val_self > val_other,
                Self::Mate(_, c) => match c {
                    Colour::White => false,
                    Colour::Black => true,
                },
                Self::Infinity => false,
                Self::NegInfinity => true,
            },
            Self::Mate(mate_self, c1) => match other {
                Self::Numeric(_) => match c1 {
                    Colour::White => true,
                    Colour::Black => false,
                },
                Self::Mate(mate_other, c2) => match c1 {
                    Colour::White => match c2 {
                        Colour::White => mate_self < mate_other,
                        Colour::Black => true,
                    },
                    Colour::Black => match c2 {
                        Colour::White => false,
                        Colour::Black => mate_self > mate_other,
                    },
                },
                Self::Infinity => false,
                Self::NegInfinity => true,
            },
            Self::Infinity => true,
            Self::NegInfinity => false,
        }
    }
    fn ge(&self, other: &Self) -> bool {
        match self {
            Self::Numeric(val_self) => match other {
                Self::Numeric(val_other) => val_self >= val_other,
                Self::Mate(_, c) => match c {
                    Colour::White => false,
                    Colour::Black => true,
                },
                Self::Infinity => false,
                Self::NegInfinity => true,
            },
            Self::Mate(mate_self, c1) => match other {
                Self::Numeric(_) => match c1 {
                    Colour::White => true,
                    Colour::Black => false,
                },
                Self::Mate(mate_other, c2) => match c1 {
                    Colour::White => match c2 {
                        Colour::White => mate_self <= mate_other,
                        Colour::Black => true,
                    },
                    Colour::Black => match c2 {
                        Colour::White => false,
                        Colour::Black => mate_self >= mate_other,
                    },
                },
                Self::Infinity => false,
                Self::NegInfinity => true,
            },
            Self::Infinity => true,
            Self::NegInfinity => false,
        }
    }
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self > other {
            Some(Ordering::Greater)
        } else if self < other {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}
