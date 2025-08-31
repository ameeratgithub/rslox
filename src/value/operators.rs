use std::ops::{Add, Div, Mul, Neg, Not, Sub};

use crate::value::Value;

/// Overrides '+' operator for custom type `Value`
/// It's like operator overloading in C++
/// Only works if `Value` is of type number
impl Add for Value {
    type Output = self::Value;
    fn add(self, rhs: Self) -> Self::Output {
        if self.is_number() && rhs.is_number() {
            // Convert both operands into numbers.
            let a: f64 = self.into();
            let b: f64 = rhs.into();
            // Convert both numbers into value
            return (a + b).into();
        }

        // This should be unreachable, types should be checked in compiler for proper
        // error handling
        unreachable!()
    }
}

/// Overrides '-' operator for custom type `Value`
/// It's like operator overloading in C++
/// Only works if `Value` is of type number
impl Sub for Value {
    type Output = self::Value;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_number() && rhs.is_number() {
            let a: f64 = self.into();
            let b: f64 = rhs.into();
            return (a - b).into();
        }

        // This should be unreachable, types should be checked in compiler for proper
        // error handling
        unreachable!()
    }
}

/// Overrides '*' operator for custom type `Value`
/// It's like operator overloading in C++
/// Only works if `Value` is of type number
impl Mul for Value {
    type Output = self::Value;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_number() && rhs.is_number() {
            let a: f64 = self.into();
            let b: f64 = rhs.into();
            return (a * b).into();
        }

        // This should be unreachable, types should be checked in compiler for proper
        // error handling
        unreachable!()
    }
}

/// Overrides '/' operator for custom type `Value`
/// It's like operator overloading in C++
/// Only works if `Value` is of type number
impl Div for Value {
    type Output = self::Value;
    fn div(self, rhs: Self) -> Self::Output {
        if self.is_number() && rhs.is_number() {
            let a: f64 = self.into();
            let b: f64 = rhs.into();
            return (a / b).into();
        }

        // This should be unreachable, types should be checked in compiler for proper
        // error handling
        unreachable!()
    }
}

/// Overrides '-' (negation, which is unary) operator, for custom type `Value`
/// It's like operator overloading in C++
/// Only works if `Value` is of type number
impl Neg for Value {
    type Output = self::Value;

    fn neg(self) -> Self::Output {
        if self.is_number() {
            let a: f64 = self.into();
            return (-a).into();
        }
        // This code shouldn't be reached
        unreachable!()
    }
}
/// Overrides '!' operator for custom type `Value`, only works if value is Bool
/// It's like operator overloading in C++
impl Not for Value {
    type Output = self::Value;
    fn not(self) -> Self::Output {
        if self.is_bool() {
            let b: bool = self.into();
            return (!b).into();
        }

        // This code shouldn't be reached
        unreachable!();
    }
}
