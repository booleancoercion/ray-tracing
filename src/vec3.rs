use rand::distributions::{Distribution, Standard};
use rand::Rng;

use std::fmt::Display;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3(pub [f64; 3]);

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }

    #[inline(always)]
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    #[inline(always)]
    pub fn y(&self) -> f64 {
        self.0[1]
    }

    #[inline(always)]
    pub fn z(&self) -> f64 {
        self.0[2]
    }

    #[inline(always)]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    pub fn length_squared(&self) -> f64 {
        self.dot(&self)
    }

    #[inline(always)]
    pub fn dot(&self, other: &Self) -> f64 {
        self.0[0] * other.0[0] + self.0[1] * other.0[1] + self.0[2] * other.0[2]
    }

    #[inline(always)]
    pub fn cross(&self, other: &Self) -> Self {
        Self([
            self.0[1] * other.0[2] - self.0[2] * other.0[1],
            self.0[2] * other.0[0] - self.0[0] * other.0[2],
            self.0[0] * other.0[1] - self.0[1] * other.0[0],
        ])
    }

    #[inline(always)]
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn random_in_unit_sphere<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        let r: f64 = rng.gen();

        r * Self::random_unit_vec(rng)
    }

    pub fn random_unit_vec<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        let z: f64 = rng.gen_range(-1.0..1.0);

        let sin_theta = (1.0 - z * z).sqrt();
        let phi: f64 = rng.gen_range(0.0..std::f64::consts::TAU);

        let cos_phi = phi.cos();

        let x = sin_theta * cos_phi;
        let y = sin_theta * (1.0 - cos_phi * cos_phi).sqrt();

        Self([x, y, z])
    }

    pub fn random_in_unit_disk<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        let x: f64 = rng.gen_range(-1.0..1.0);
        let y = (1.0 - x * x).sqrt();
        let r: f64 = rng.gen();

        Self([r * x, r * y, 0.0])
    }

    #[inline(always)]
    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        (self.0[0].abs() < eps) && (self.0[1].abs() < eps) && (self.0[2].abs() < eps)
    }
}

impl Distribution<Vec3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3(rng.gen())
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0[0], self.0[1], self.0[2])
    }
}

impl Default for Vec3 {
    #[inline(always)]
    fn default() -> Self {
        Self([0.0, 0.0, 0.0])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self([-self.0[0], -self.0[1], -self.0[2]])
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl AddAssign for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Self::Output {
        Self([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}

impl MulAssign<f64> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f64) {
        self.0[0] *= rhs;
        self.0[1] *= rhs;
        self.0[2] *= rhs;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        ])
    }
}

impl DivAssign<f64> for Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl SubAssign for Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs
    }
}
