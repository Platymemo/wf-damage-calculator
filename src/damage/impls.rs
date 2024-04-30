use std::ops::Index;

use super::{DamageArray, DamageType};

impl DamageArray {
    pub fn new() -> Self {
        DamageArray([0.0f64; 20])
    }

    pub fn from(values: [f64; 20]) -> Self {
        DamageArray(values)
    }
}

impl std::ops::Add<f64> for DamageArray {
    type Output = DamageArray;

    fn add(self, rhs: f64) -> Self::Output {
        let mut arr = [0.0f64; 20];
        for (idx, val) in self.0.iter().enumerate() {
            arr[idx] = val + rhs;
        }

        DamageArray(arr)
    }
}

impl std::ops::Add<DamageArray> for f64 {
    type Output = DamageArray;

    fn add(self, rhs: DamageArray) -> Self::Output {
        rhs + self
    }
}

impl std::ops::Add<DamageArray> for DamageArray {
    type Output = DamageArray;

    fn add(self, rhs: DamageArray) -> Self::Output {
        let mut arr = [0.0f64; 20];
        for (idx, (left, right)) in self.0.iter().zip(rhs.0).enumerate() {
            arr[idx] = left + right;
        }

        DamageArray(arr)
    }
}

impl std::ops::AddAssign<f64> for DamageArray {
    fn add_assign(&mut self, rhs: f64) {
        for val in self.0.iter_mut() {
            *val += rhs;
        }
    }
}

impl std::ops::AddAssign<DamageArray> for DamageArray {
    fn add_assign(&mut self, rhs: DamageArray) {
        for (idx, val) in self.0.iter_mut().enumerate() {
            *val += rhs[idx];
        }
    }
}

impl std::ops::Sub<f64> for DamageArray {
    type Output = DamageArray;

    fn sub(self, rhs: f64) -> Self::Output {
        let mut arr = [0.0f64; 20];
        for (idx, val) in self.0.iter().enumerate() {
            arr[idx] = val - rhs;
        }

        DamageArray(arr)
    }
}

impl std::ops::Sub<DamageArray> for f64 {
    type Output = DamageArray;

    fn sub(self, rhs: DamageArray) -> Self::Output {
        rhs + (-self)
    }
}

impl std::ops::Sub<DamageArray> for DamageArray {
    type Output = DamageArray;

    fn sub(self, rhs: DamageArray) -> Self::Output {
        let mut arr = [0.0f64; 20];
        for (idx, (left, right)) in self.0.iter().zip(rhs.0).enumerate() {
            arr[idx] = left - right;
        }

        DamageArray(arr)
    }
}

impl std::ops::SubAssign<f64> for DamageArray {
    fn sub_assign(&mut self, rhs: f64) {
        for val in self.0.iter_mut() {
            *val -= rhs;
        }
    }
}

impl std::ops::SubAssign<DamageArray> for DamageArray {
    fn sub_assign(&mut self, rhs: DamageArray) {
        for (idx, val) in self.0.iter_mut().enumerate() {
            *val -= rhs[idx];
        }
    }
}

impl std::ops::Mul<f64> for DamageArray {
    type Output = DamageArray;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut arr = [0.0f64; 20];
        for (idx, val) in self.0.iter().enumerate() {
            arr[idx] = val * rhs;
        }

        DamageArray(arr)
    }
}

impl std::ops::Mul<DamageArray> for f64 {
    type Output = DamageArray;

    fn mul(self, rhs: DamageArray) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<DamageArray> for DamageArray {
    type Output = DamageArray;

    fn mul(self, rhs: DamageArray) -> Self::Output {
        let mut arr = [0.0f64; 20];
        for (idx, (left, right)) in self.0.iter().zip(rhs.0).enumerate() {
            arr[idx] = left * right;
        }

        DamageArray(arr)
    }
}

impl std::ops::MulAssign<f64> for DamageArray {
    fn mul_assign(&mut self, rhs: f64) {
        for val in self.0.iter_mut() {
            *val *= rhs;
        }
    }
}

impl std::ops::MulAssign<DamageArray> for DamageArray {
    fn mul_assign(&mut self, rhs: DamageArray) {
        for (idx, val) in self.0.iter_mut().enumerate() {
            *val *= rhs[idx];
        }
    }
}

impl std::ops::Div<f64> for DamageArray {
    type Output = DamageArray;

    fn div(self, rhs: f64) -> Self::Output {
        let mut arr = [0.0f64; 20];
        for (idx, val) in self.0.iter().enumerate() {
            arr[idx] = val / rhs;
        }

        DamageArray(arr)
    }
}

impl std::ops::Div<DamageArray> for f64 {
    type Output = DamageArray;

    fn div(self, rhs: DamageArray) -> Self::Output {
        let mut arr = [0.0f64; 20];
        for (idx, val) in rhs.0.iter().enumerate() {
            arr[idx] = self / val;
        }

        DamageArray(arr)
    }
}

impl std::ops::Div<DamageArray> for DamageArray {
    type Output = DamageArray;

    fn div(self, rhs: DamageArray) -> Self::Output {
        let mut arr = [0.0f64; 20];
        for (idx, (left, right)) in self.0.iter().zip(rhs.0).enumerate() {
            arr[idx] = left / right;
        }

        DamageArray(arr)
    }
}

impl std::ops::DivAssign<f64> for DamageArray {
    fn div_assign(&mut self, rhs: f64) {
        for val in self.0.iter_mut() {
            *val /= rhs;
        }
    }
}

impl std::ops::DivAssign<DamageArray> for DamageArray {
    fn div_assign(&mut self, rhs: DamageArray) {
        for (idx, val) in self.0.iter_mut().enumerate() {
            *val /= rhs[idx];
        }
    }
}

impl std::ops::Rem<f64> for DamageArray {
    type Output = DamageArray;

    fn rem(self, rhs: f64) -> Self::Output {
        let mut arr = [0.0f64; 20];
        for (idx, val) in self.0.iter().enumerate() {
            arr[idx] = val % rhs;
        }

        DamageArray(arr)
    }
}

impl std::ops::Rem<DamageArray> for f64 {
    type Output = DamageArray;

    fn rem(self, rhs: DamageArray) -> Self::Output {
        let mut arr = [0.0f64; 20];
        for (idx, val) in rhs.0.iter().enumerate() {
            arr[idx] = self % val;
        }

        DamageArray(arr)
    }
}

impl std::ops::Rem<DamageArray> for DamageArray {
    type Output = DamageArray;

    fn rem(self, rhs: DamageArray) -> Self::Output {
        let mut arr = [0.0f64; 20];
        for (idx, (left, right)) in self.0.iter().zip(rhs.0).enumerate() {
            arr[idx] = left % right;
        }

        DamageArray(arr)
    }
}

impl std::ops::RemAssign<f64> for DamageArray {
    fn rem_assign(&mut self, rhs: f64) {
        for val in self.0.iter_mut() {
            *val %= rhs;
        }
    }
}

impl std::ops::RemAssign<DamageArray> for DamageArray {
    fn rem_assign(&mut self, rhs: DamageArray) {
        for (idx, val) in self.0.iter_mut().enumerate() {
            *val %= rhs[idx];
        }
    }
}

impl std::ops::Neg for DamageArray {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut arr = [0.0f64; 20];
        for (idx, val) in self.0.iter().enumerate() {
            arr[idx] = -val;
        }

        DamageArray(arr)
    }
}

impl Index<DamageType> for DamageArray {
    type Output = f64;

    fn index(&self, index: DamageType) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl Index<usize> for DamageArray {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl DamageType {
    /// Returns true if the damage type is considered physical damage, otherwise false.
    pub fn is_physical(&self) -> bool {
        match self {
            Self::Impact | Self::Puncture | Self::Slash => true,
            _ => false,
        }
    }

    /// Returns true if the damage type is a primary element, otherwise false.
    pub fn is_primary_elemental(&self) -> bool {
        match self {
            Self::Cold | Self::Electricity | Self::Heat | Self::Toxin => true,
            _ => false,
        }
    }

    /// Returns true if the damage type is a secondary element, otherwise false.
    /// A secondary element is a combination of two primary elements.
    pub fn is_secondary_elemental(&self) -> bool {
        match self {
            Self::Blast
            | Self::Radiation
            | Self::Gas
            | Self::Magnetic
            | Self::Viral
            | Self::Corrosive => true,
            _ => false,
        }
    }

    /// Returns true if the damage type is elemental, otherwise false.
    pub fn is_elemental(&self) -> bool {
        self.is_primary_elemental() || self.is_secondary_elemental()
    }

    /// Combines two primary elements into their secondary variant.
    /// Panics if parameters are not primary elements.
    pub fn combine(&self, other: &Self) -> Self {
        self.combine_checked(other).unwrap()
    }

    /// Combines two primary elements into their secondary variant.
    /// Fails if either damage type is not a primary element.
    pub fn combine_checked(&self, other: &Self) -> Option<Self> {
        if !self.is_primary_elemental() || !other.is_primary_elemental() {
            return None;
        }

        match (self.min(other), self.max(other)) {
            (Self::Heat, Self::Cold) => Some(Self::Blast),
            (Self::Heat, Self::Electricity) => Some(Self::Radiation),
            (Self::Heat, Self::Toxin) => Some(Self::Gas),
            (Self::Cold, Self::Electricity) => Some(Self::Magnetic),
            (Self::Cold, Self::Toxin) => Some(Self::Viral),
            (Self::Electricity, Self::Toxin) => Some(Self::Corrosive),
            (_, _) => unreachable!(),
        }
    }
}
