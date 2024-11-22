macro_rules! impl_tuple3_from {
    ($name:ident, $from:ty, $($to:ty),+) => {
        $(impl From<$name<$from>> for $name<$to> {
            fn from(value: $name<$from>) -> Self {
                Self {
                    x: value.x as $to,
                    y: value.y as $to,
                    z: value.z as $to,
                }
            }
        })+
    };
}

macro_rules! impl_tuple3_lhs_scalar_mul {
    ($name:ident, $scalar:ty) => {
        impl std::ops::Mul<$name<$scalar>> for $scalar {
            type Output = $name<$scalar>;

            fn mul(self, rhs: $name<$scalar>) -> $name<$scalar> {
                rhs * self
            }
        }
    };
}

macro_rules! impl_tuple3 {
    ($name:ident) => {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
        pub struct $name<T> {
            pub x: T,
            pub y: T,
            pub z: T,
        }

        impl<T> $name<T> {
            pub fn new(x: T, y: T, z: T) -> Self {
                Self { x, y, z }
            }

            pub fn abs(self) -> Self
            where
                T: num_traits::Signed,
            {
                Self {
                    x: self.x.abs(),
                    y: self.y.abs(),
                    z: self.z.abs(),
                }
            }
        }

        impl<T: num_traits::Float> $name<T> {
            pub fn has_nan(self) -> bool {
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

            pub fn permute(self, indices: &[usize; 3]) -> Self {
                Self {
                    x: self[indices[0]],
                    y: self[indices[1]],
                    z: self[indices[2]],
                }
            }
        }

        impl<T: $crate::math::PartialPreOrd> $name<T> {
            pub fn min(self, other: Self) -> Self {
                Self {
                    x: self.x.min(other.x),
                    y: self.y.min(other.y),
                    z: self.z.min(other.z),
                }
            }

            pub fn max(self, other: Self) -> Self {
                Self {
                    x: self.x.max(other.x),
                    y: self.y.max(other.y),
                    z: self.z.max(other.z),
                }
            }

            pub fn min_component_value(self) -> T {
                self.x.min(self.y.min(self.z))
            }

            pub fn max_component_value(self) -> T {
                self.x.max(self.y.max(self.z))
            }

            pub fn min_component_index(self) -> usize {
                if self.x < self.y {
                    if self.x < self.z {
                        0
                    } else {
                        2
                    }
                } else {
                    if self.y < self.z {
                        1
                    } else {
                        2
                    }
                }
            }

            pub fn max_component_index(self) -> usize {
                if self.x > self.y {
                    if self.x > self.z {
                        0
                    } else {
                        2
                    }
                } else {
                    if self.y > self.z {
                        1
                    } else {
                        2
                    }
                }
            }
        }

        impl<T: std::fmt::Display> std::fmt::Display for $name<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(
                    f,
                    "{}({}, {}, {})",
                    stringify!($name),
                    self.x,
                    self.y,
                    self.z
                )
            }
        }

        impl<T> std::ops::Index<usize> for $name<T> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
                    _ => panic!(
                        "index out of bounds: the len is 3 but the index is {}",
                        index
                    ),
                }
            }
        }

        impl<T> std::ops::IndexMut<usize> for $name<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    2 => &mut self.z,
                    _ => panic!(
                        "index out of bounds: the len is 3 but the index is {}",
                        index
                    ),
                }
            }
        }

        impl<T: std::ops::Add<T, Output = T>> std::ops::Add for $name<T> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                }
            }
        }

        impl<T: std::ops::AddAssign<T>> std::ops::AddAssign<$name<T>> for $name<T> {
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
            }
        }

        impl<T: std::ops::Sub<T, Output = T>> std::ops::Sub for $name<T> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                }
            }
        }

        impl<T: std::ops::SubAssign<T>> std::ops::SubAssign<$name<T>> for $name<T> {
            fn sub_assign(&mut self, rhs: Self) {
                self.x -= rhs.x;
                self.y -= rhs.y;
                self.z -= rhs.z;
            }
        }

        impl<T: std::ops::Neg<Output = T>> std::ops::Neg for $name<T> {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                }
            }
        }

        impl<T> std::ops::Mul<T> for $name<T>
        where
            T: Clone + std::ops::Mul<T, Output = T>,
        {
            type Output = Self;

            fn mul(self, rhs: T) -> Self {
                Self {
                    x: self.x * rhs.clone(),
                    y: self.y * rhs.clone(),
                    z: self.z * rhs.clone(),
                }
            }
        }

        impl_tuple3_lhs_scalar_mul!($name, f32);
        impl_tuple3_lhs_scalar_mul!($name, f64);
        impl_tuple3_lhs_scalar_mul!($name, i8);
        impl_tuple3_lhs_scalar_mul!($name, i16);
        impl_tuple3_lhs_scalar_mul!($name, i32);
        impl_tuple3_lhs_scalar_mul!($name, i64);
        impl_tuple3_lhs_scalar_mul!($name, i128);
        impl_tuple3_lhs_scalar_mul!($name, isize);
        impl_tuple3_lhs_scalar_mul!($name, u8);
        impl_tuple3_lhs_scalar_mul!($name, u16);
        impl_tuple3_lhs_scalar_mul!($name, u32);
        impl_tuple3_lhs_scalar_mul!($name, u64);
        impl_tuple3_lhs_scalar_mul!($name, u128);
        impl_tuple3_lhs_scalar_mul!($name, usize);

        impl<T> std::ops::Div<T> for $name<T>
        where
            T: Copy + std::ops::Div<T, Output = T>,
        {
            type Output = Self;

            fn div(self, rhs: T) -> Self {
                Self {
                    x: self.x / rhs.clone(),
                    y: self.y / rhs.clone(),
                    z: self.z / rhs.clone(),
                }
            }
        }

        impl_tuple3_from!(
            $name, bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple3_from!(
            $name, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple3_from!(
            $name, f64, f32, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple3_from!(
            $name, i8, f32, f64, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple3_from!(
            $name, i16, f32, f64, i8, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple3_from!(
            $name, i32, f32, f64, i8, i16, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple3_from!(
            $name, i64, f32, f64, i8, i16, i32, i128, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple3_from!(
            $name, i128, f32, f64, i8, i16, i32, i64, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple3_from!(
            $name, isize, f32, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize
        );
        impl_tuple3_from!(
            $name, u8, f32, f64, i8, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize
        );
        impl_tuple3_from!(
            $name, u16, f32, f64, i8, i16, i32, i64, i128, isize, u8, u32, u64, u128, usize
        );
        impl_tuple3_from!(
            $name, u32, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u64, u128, usize
        );
        impl_tuple3_from!(
            $name, u64, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u128, usize
        );
        impl_tuple3_from!(
            $name, u128, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, usize
        );
        impl_tuple3_from!(
            $name, usize, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128
        );
    };
}

macro_rules! impl_tuple2_from {
    ($name:ident, $from:ty, $($to:ty),+) => {
        $(impl From<$name<$from>> for $name<$to> {
            fn from(value: $name<$from>) -> Self {
                Self {
                    x: value.x as $to,
                    y: value.y as $to,
                }
            }
        })+
    };
}

macro_rules! impl_tuple2_lhs_scalar_mul {
    ($name:ident, $scalar:ty) => {
        impl std::ops::Mul<$name<$scalar>> for $scalar {
            type Output = $name<$scalar>;

            fn mul(self, rhs: $name<$scalar>) -> $name<$scalar> {
                rhs * self
            }
        }
    };
}

macro_rules! impl_tuple2 {
    ($name:ident) => {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
        pub struct $name<T> {
            pub x: T,
            pub y: T,
        }

        impl<T> $name<T> {
            pub fn new(x: T, y: T) -> Self {
                Self { x, y }
            }

            pub fn abs(self) -> Self
            where
                T: num_traits::Signed,
            {
                Self {
                    x: self.x.abs(),
                    y: self.y.abs(),
                }
            }
        }

        impl<T: num_traits::Float> $name<T> {
            pub fn has_nan(self) -> bool {
                self.x.is_nan() || self.y.is_nan()
            }

            pub fn ceil(self) -> Self {
                Self {
                    x: self.x.ceil(),
                    y: self.y.ceil(),
                }
            }

            pub fn floor(self) -> Self {
                Self {
                    x: self.x.floor(),
                    y: self.y.floor(),
                }
            }

            pub fn permute(self, indices: &[usize; 2]) -> Self {
                Self {
                    x: self[indices[0]],
                    y: self[indices[1]],
                }
            }
        }

        impl<T: $crate::math::PartialPreOrd> $name<T> {
            pub fn min(self, other: Self) -> Self {
                Self {
                    x: self.x.min(other.x),
                    y: self.y.min(other.y),
                }
            }

            pub fn max(self, other: Self) -> Self {
                Self {
                    x: self.x.max(other.x),
                    y: self.y.max(other.y),
                }
            }

            pub fn min_component_value(self) -> T {
                self.x.min(self.y)
            }

            pub fn max_component_value(self) -> T {
                self.x.max(self.y)
            }

            pub fn min_component_index(self) -> usize {
                if self.x < self.y {
                    0
                } else {
                    1
                }
            }

            pub fn max_component_index(self) -> usize {
                if self.x > self.y {
                    0
                } else {
                    1
                }
            }
        }

        impl<T: std::fmt::Display> std::fmt::Display for $name<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "{}({}, {})", stringify!($name), self.x, self.y,)
            }
        }

        impl<T> std::ops::Index<usize> for $name<T> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    _ => panic!(
                        "index out of bounds: the len is 2 but the index is {}",
                        index
                    ),
                }
            }
        }

        impl<T> std::ops::IndexMut<usize> for $name<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    _ => panic!(
                        "index out of bounds: the len is 2 but the index is {}",
                        index
                    ),
                }
            }
        }

        impl<T: std::ops::Add<T, Output = T>> std::ops::Add for $name<T> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl<T: std::ops::AddAssign<T>> std::ops::AddAssign<$name<T>> for $name<T> {
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }

        impl<T: std::ops::Sub<T, Output = T>> std::ops::Sub for $name<T> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl<T: std::ops::SubAssign<T>> std::ops::SubAssign<$name<T>> for $name<T> {
            fn sub_assign(&mut self, rhs: Self) {
                self.x -= rhs.x;
                self.y -= rhs.y;
            }
        }

        impl<T: std::ops::Neg<Output = T>> std::ops::Neg for $name<T> {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }

        impl<T> std::ops::Mul<T> for $name<T>
        where
            T: Clone + std::ops::Mul<T, Output = T>,
        {
            type Output = Self;

            fn mul(self, rhs: T) -> Self {
                Self {
                    x: self.x * rhs.clone(),
                    y: self.y * rhs.clone(),
                }
            }
        }

        impl_tuple2_lhs_scalar_mul!($name, f32);
        impl_tuple2_lhs_scalar_mul!($name, f64);
        impl_tuple2_lhs_scalar_mul!($name, i8);
        impl_tuple2_lhs_scalar_mul!($name, i16);
        impl_tuple2_lhs_scalar_mul!($name, i32);
        impl_tuple2_lhs_scalar_mul!($name, i64);
        impl_tuple2_lhs_scalar_mul!($name, i128);
        impl_tuple2_lhs_scalar_mul!($name, isize);
        impl_tuple2_lhs_scalar_mul!($name, u8);
        impl_tuple2_lhs_scalar_mul!($name, u16);
        impl_tuple2_lhs_scalar_mul!($name, u32);
        impl_tuple2_lhs_scalar_mul!($name, u64);
        impl_tuple2_lhs_scalar_mul!($name, u128);
        impl_tuple2_lhs_scalar_mul!($name, usize);

        impl<T> std::ops::Div<T> for $name<T>
        where
            T: Copy + std::ops::Div<T, Output = T>,
        {
            type Output = Self;

            fn div(self, rhs: T) -> Self {
                Self {
                    x: self.x / rhs.clone(),
                    y: self.y / rhs.clone(),
                }
            }
        }

        impl_tuple2_from!(
            $name, bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple2_from!(
            $name, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple2_from!(
            $name, f64, f32, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple2_from!(
            $name, i8, f32, f64, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple2_from!(
            $name, i16, f32, f64, i8, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple2_from!(
            $name, i32, f32, f64, i8, i16, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple2_from!(
            $name, i64, f32, f64, i8, i16, i32, i128, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple2_from!(
            $name, i128, f32, f64, i8, i16, i32, i64, isize, u8, u16, u32, u64, u128, usize
        );
        impl_tuple2_from!(
            $name, isize, f32, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize
        );
        impl_tuple2_from!(
            $name, u8, f32, f64, i8, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize
        );
        impl_tuple2_from!(
            $name, u16, f32, f64, i8, i16, i32, i64, i128, isize, u8, u32, u64, u128, usize
        );
        impl_tuple2_from!(
            $name, u32, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u64, u128, usize
        );
        impl_tuple2_from!(
            $name, u64, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u128, usize
        );
        impl_tuple2_from!(
            $name, u128, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, usize
        );
        impl_tuple2_from!(
            $name, usize, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128
        );
    };
}
