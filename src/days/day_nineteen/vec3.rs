use std::cmp::PartialEq;
use std::convert::From;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Vec3 {
    co: [i32; 3],
}

impl Vec3 {
    pub fn rotate(&self, axis: Vec3, second_axis: Vec3) -> Self {
        let third_axis = axis.cross(second_axis.clone());
        [self.dot(axis), self.dot(second_axis), self.dot(third_axis)].into()
    }

    pub fn dot(&self, other: Vec3) -> i32 {
        return self[0] * other[0] + self[1] * other[1] + self[2] * other[2];
    }

    pub fn abs(&self) -> Vec3 {
        return [self[0].abs(), self[1].abs(), self[2].abs()].into();
    }

    pub fn sum(&self) -> i32 {
        self.co[0] + self.co[1] + self.co[2]
    }

    pub fn cross(&self, axis: Vec3) -> Self {
        [
            self[1] * axis[2] - self[2] * axis[1],
            self[2] * axis[0] - self[0] * axis[2],
            self[0] * axis[1] - self[1] * axis[0],
        ]
        .into()
    }

    pub fn zero() -> Self {
        [0, 0, 0].into()
    }

    pub fn new(x: [i32; 3]) -> Self {
        Self { co: x }
    }
}

// Convenience construction

impl From<[i32; 3]> for Vec3 {
    fn from(item: [i32; 3]) -> Self {
        Vec3 { co: item }
    }
}

// Equality check

// impl PartialEq for Vec3 {
//     fn eq(&self, other: &Self) -> bool {
//         self.co == other.co
//     }
// }
//
// impl Ord for Vec3 {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.co.cmp(&other.co)
//     }
// }
//
// impl PartialOrd for Vec3 {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// Operators

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]].into()
    }
}
impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]].into()
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]].into()
    }
}
impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]].into()
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]].into()
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        [self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2]].into()
    }
}

impl Index<usize> for Vec3 {
    type Output = i32;
    fn index(&self, i: usize) -> &Self::Output {
        &self.co[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.co[index]
    }
}
