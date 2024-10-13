use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num_traits::{Float, One, Signed};

#[derive(Clone, Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Copy> Vector3<T> {
    pub fn permute(self, perm: [usize; 3]) -> Self {
        Self {
            x: self[perm[0]],
            y: self[perm[1]],
            z: self[perm[2]],
        }
    }
}

impl<T: Mul<Output = T>> Vector3<T> {
    pub fn horizontal_product(self) -> T {
        self.x * self.y * self.z
    }
}

impl<T: Signed> Vector3<T> {
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
}

macro_rules! impl_min_max {
    ($type:ty) => {
        impl Vector3<$type> {
            pub fn min(self, rhs: Self) -> Self {
                Self {
                    x: self.x.min(rhs.x),
                    y: self.y.min(rhs.y),
                    z: self.z.min(rhs.z),
                }
            }

            pub fn max(self, rhs: Self) -> Self {
                Self {
                    x: self.x.max(rhs.x),
                    y: self.y.max(rhs.y),
                    z: self.z.max(rhs.z),
                }
            }

            pub fn min_component_value(&self) -> $type {
                self.x.min(self.y.min(self.z))
            }

            pub fn max_component_value(&self) -> $type {
                self.x.max(self.y.max(self.z))
            }
        }
    };
}

impl_min_max!(i8);
impl_min_max!(i16);
impl_min_max!(i32);
impl_min_max!(i64);
impl_min_max!(i128);
impl_min_max!(u8);
impl_min_max!(u16);
impl_min_max!(u32);
impl_min_max!(u64);
impl_min_max!(u128);
impl_min_max!(f32);
impl_min_max!(f64);

impl<T: PartialOrd> Vector3<T> {
    pub fn min_component_index(&self) -> usize {
        if self.x < self.y {
            if self.x < self.z {
                0
            } else {
                2
            }
        } else if self.y < self.z {
            1
        } else {
            2
        }
    }

    pub fn max_component_index(&self) -> usize {
        if self.x > self.y {
            if self.x > self.z {
                0
            } else {
                2
            }
        } else if self.y > self.z {
            1
        } else {
            2
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Vector3<T> {
    pub fn fma(self, b: Self, c: Self) -> Self {
        Self {
            x: self.x * b.x + c.x,
            y: self.y * b.y + c.y,
            z: self.z * b.z + c.z,
        }
    }
}

impl<T: One + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy> Vector3<T> {
    pub fn lerp(self, rhs: Self, t: T) -> Self {
        self * (T::one() - t) + rhs * t
    }
}

impl<T: Float> Vector3<T> {
    pub fn has_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }
}

impl<T: Copy> Copy for Vector3<T> {}

impl<T: PartialEq> PartialEq for Vector3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: AbsDiffEq> AbsDiffEq for Vector3<T>
where
    T::Epsilon: Copy,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        T::abs_diff_eq(&self.x, &other.x, epsilon)
            && T::abs_diff_eq(&self.y, &other.y, epsilon)
            && T::abs_diff_eq(&self.z, &other.z, epsilon)
    }
}

impl<T: RelativeEq> RelativeEq for Vector3<T>
where
    T::Epsilon: Copy,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        T::relative_eq(&self.x, &other.x, epsilon, max_relative)
            && T::relative_eq(&self.y, &other.y, epsilon, max_relative)
            && T::relative_eq(&self.z, &other.z, epsilon, max_relative)
    }
}

impl<T: UlpsEq> UlpsEq for Vector3<T>
where
    T::Epsilon: Copy,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
            && T::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
            && T::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
    }
}

impl<T> Index<usize> for Vector3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range"),
        }
    }
}

impl<T> IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range"),
        }
    }
}

impl<T: Add<Output = T>> Add for Vector3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: AddAssign> AddAssign for Vector3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Sub<Output = T>> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: SubAssign> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Neg<Output = T>> Neg for Vector3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vector3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::Vector3;

    #[test]
    fn vector3_operations() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert!(!v.has_nan());
        assert_relative_eq!(v[0], 1.0);
        assert_relative_eq!(v[1], 2.0);
        assert_relative_eq!(v[2], 3.0);
        assert!(std::panic::catch_unwind(|| v[4]).is_err());

        let mut v = Vector3::new(1.0, 2.0, 3.0);
        v[0] = 19.0;
        v[1] = 7.4;
        v[2] = -42.0;
        assert_relative_eq!(v[0], 19.0);
        assert_relative_eq!(v[1], 7.4);
        assert_relative_eq!(v[2], -42.0);
        assert!(std::panic::catch_unwind(|| v[20]).is_err());

        let mut v1 = Vector3::new(0.5, -1.5, 2.3);
        let v2 = Vector3::new(1.0, 2.0, -3.0);
        assert_relative_eq!(v1 + v2, Vector3::new(1.5, 0.5, -0.7));

        v1 += v2;
        assert_relative_eq!(v1, Vector3::new(1.5, 0.5, -0.7));

        let mut v1 = Vector3::new(0.5, -1.5, 2.3);
        let v2 = Vector3::new(1.0, 2.0, -3.0);
        assert_relative_eq!(v1 - v2, Vector3::new(-0.5, -3.5, 5.3));

        v1 -= v2;
        assert_relative_eq!(v1, Vector3::new(-0.5, -3.5, 5.3));

        let v = Vector3::new(1.5, -0.8, 0.0);
        assert_relative_eq!(-v, Vector3::new(-1.5, 0.8, 0.0));

        let mut v = Vector3::new(0.2, -1.4, 2.3);
        assert_relative_eq!(v * 1.5, Vector3::new(0.3, -2.1, 3.45));

        v *= 1.5;
        assert_relative_eq!(v, Vector3::new(0.3, -2.1, 3.45));

        let mut v = Vector3::new(1.0, 2.0, 3.0);
        assert_relative_eq!(v / 2.0, Vector3::new(0.5, 1.0, 1.5));

        v /= 2.0;
        assert_relative_eq!(v, Vector3::new(0.5, 1.0, 1.5));

        let v = Vector3::new(-1.0, -2.3, -0.5);
        assert_relative_eq!(v.abs(), Vector3::new(1.0, 2.3, 0.5));

        let v = Vector3::new(2.3, -0.2, 1.4);
        assert_relative_eq!(v.ceil(), Vector3::new(3.0, 0.0, 2.0));

        assert_relative_eq!(v.floor(), Vector3::new(2.0, -1.0, 1.0));

        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(2.0, 4.0, 6.0);
        assert_relative_eq!(v1.lerp(v2, 0.5), Vector3::new(1.5, 3.0, 4.5));

        let v1 = Vector3::new(1.5, -0.5, 1.2);
        let v2 = Vector3::new(2.0, -3.0, 3.0);
        let v3 = Vector3::new(-0.2, 1.4, 0.8);
        assert_relative_eq!(v1.fma(v2, v3), Vector3::new(2.8, 2.9, 4.4));

        let v1 = Vector3::<f32>::new(1.0, -0.5, 2.3);
        let v2 = Vector3::<f32>::new(0.2, 1.5, 2.1);
        assert_relative_eq!(v1.min(v2), Vector3::new(0.2, -0.5, 2.1));

        assert_relative_eq!(v1.max(v2), Vector3::new(1.0, 1.5, 2.3));

        let v = Vector3::new(0.5f32, 0.1f32, 0.8f32);
        assert_relative_eq!(v.min_component_value(), 0.1);

        assert_relative_eq!(v.max_component_value(), 0.8);

        assert_eq!(v.min_component_index(), 1);

        assert_eq!(v.max_component_index(), 2);

        assert_relative_eq!(v.permute([1, 2, 0]), Vector3::new(0.1, 0.8, 0.5));

        assert_relative_eq!(v.horizontal_product(), 0.04);
    }
}
