macro_rules! n_tuple_component_count {
    () => { 0usize };
    ($component:ident $(, $components:ident)*) => {
        1usize + $crate::math::macros::n_tuple_component_count!($($components),*)
    };
}

macro_rules! n_tuple_nested_call {
    ($f:expr, $s:expr, $component:ident) => {
        $s.$component
    };
    ($f:expr, $s:expr, $component:ident, $($components:ident),*) => {
        $f($s.$component, $crate::math::macros::n_tuple_nested_call!($f, $s, $($components),*))
    };
}

macro_rules! n_tuple_permute_impl {
    ($s:expr; $p:ident; $($components:tt)*) => {
        $crate::math::macros::n_tuple_permute_impl!(@(0usize; $s; $p; $($components)*,))
    };
    (@($idx:expr; $s:expr; $p:ident; $component:tt, $($components:tt)*) $($inits:tt)*) => {
        $crate::math::macros::n_tuple_permute_impl!(
            @(1usize + $idx; $s; $p; $($components)*)
            $($inits)*
            $component: $s[$p[$idx]],
        )
    };
    (@($idx:expr; $s:expr; $p:ident; $(,)?) $($inits:tt)*) => {
        Self {
            $($inits)*
        }
    }
}

macro_rules! n_tuple_index_impl {
    ($e:expr; [$($s:tt)+]; $($components:tt)*) => {
        $crate::math::macros::n_tuple_index_impl!(@(0usize; $e; [$($s)+]; $($components)*,))
    };
    (@($idx:expr; $e:expr; [$($s:tt)+]; $component:tt, $($components:tt)*) $($arms:tt)*) => {
        $crate::math::macros::n_tuple_index_impl!(
            @(1usize + $idx; $e; [$($s)+]; $($components)*)
            $($arms)*
            x if x == $idx => $($s)+.$component,
        )
    };
    (@($idx:expr; $e:expr; [$($s:tt)+]; $(,)?) $($arms:tt)*) => {
        match $e {
            $($arms)*
            _ => panic!("index out of bounds")
        }
    }
}

macro_rules! n_tuple_scalar_mul_lhs_impl {
    ($typ:ty, $name:ident, $($components:ident),+) => {
        impl std::ops::Mul<$name<$typ>> for $typ {
            type Output = $name<$typ>;

            fn mul(self, rhs: $name<$typ>) -> Self::Output {
                Self::Output {
                    $($components: self * rhs.$components),+
                }
            }
        }
    };
}

macro_rules! n_tuple_fmt_string {
    ($name:ident, $component:ident, $($components:ident),*) => {
        concat!(stringify!($name), "(", stringify!({$component}) $(, ", ", stringify!({$components}))*, ")")
    };
}

macro_rules! n_tuple_from_impl {
    ($from:ty, $to:ty, $name:ident, $($components:ident),+) => {
        impl std::convert::From<$name<$from>> for $name<$to> {
            fn from(value: $name<$from>) -> Self {
                Self {
                    $($components: value.$components.into()),+
                }
            }
        }
    };
}

macro_rules! n_tuple_impl {
    ($name:ident, $($components:ident),+) => {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
        pub struct $name<T> {
            $(pub $components: T),
            +
        }

        impl<T> $name<T> {
            pub const DIMENSIONS: usize = $crate::math::macros::n_tuple_component_count!($($components),+);
        }

        impl<T: $crate::math::number_traits::Number> $name<T> {
            pub fn new($($components: T),+) -> Self {
                let v = Self { $($components),+ };
                debug_assert!(!v.has_nan());
                v
            }

            pub fn has_nan(&self) -> bool {
                $(self.$components.is_nan()) ||+
            }
        }

        impl<T: $crate::math::number_traits::Signed> $name<T>
        {
            pub fn abs(self) -> Self {
                Self {
                    $($components: self.$components.abs()),+
                }
            }
        }

        impl<T: $crate::math::number_traits::Float> $name<T> {
            pub fn floor(self) -> Self {
                Self {
                    $($components: self.$components.floor()),+
                }
            }

            pub fn ceil(self) -> Self {
                Self {
                    $($components: self.$components.ceil()),+
                }
            }
        }

        impl<T> $name<T>
        where
            T: Copy + std::ops::Sub<Output = T> + std::ops::Mul<Self, Output = Self> + $crate::math::number_traits::Number,
            Self: std::ops::Add<Output = Self>
        {
            pub fn lerp(self, b: Self, t: T) -> Self {
                (T::ONE - t) * self + t * b
            }
        }

        impl<T: $crate::math::number_traits::Number> $name<T>
        {
            pub fn min(self, b: Self) -> Self {
                Self {
                    $($components: self.$components.min(b.$components)),+
                }
            }

            pub fn max(self, b: Self) -> Self {
                Self {
                    $($components: self.$components.max(b.$components)),+
                }
            }

            pub fn min_value(self) -> T {
                $crate::math::macros::n_tuple_nested_call!($crate::math::number_traits::Number::min, self, $($components),*)
            }

            pub fn max_value(self) -> T {
                $crate::math::macros::n_tuple_nested_call!($crate::math::number_traits::Number::max, self, $($components),*)
            }
        }

        impl<T> $name<T>
        where
            T: Copy + PartialOrd + $crate::math::number_traits::Number
        {
            pub fn min_index(&self) -> usize {
                let mut min = T::MAX;
                let mut min_index = 0;
                for (index, component) in [$(self.$components),+].into_iter().enumerate() {
                    if component < min {
                        min = component;
                        min_index = index;
                    }
                }
                min_index
            }

            pub fn max_index(&self) -> usize {
                let mut max = T::MIN;
                let mut max_index = 0;
                for (index, component) in [$(self.$components),+].into_iter().enumerate() {
                    if component > max {
                        max = component;
                        max_index = index;
                    }
                }
                max_index
            }
        }

        impl<T> $name<T>
        where
            T: std::ops::Add<Output = T> + std::ops::Mul<Output = T>
        {
            pub fn mul_add(self, a: Self, b: Self) -> Self {
                Self {
                    $($components: (self.$components * a.$components) + b.$components),+
                }
            }
        }

        impl<T: Copy> $name<T> {
            pub fn permute(self, permutation: &[usize; $crate::math::macros::n_tuple_component_count!($($components),+)]) -> Self {
                 $crate::math::macros::n_tuple_permute_impl!(self; permutation; $($components),+)
            }
        }

        impl<T> $name<T>
        where
            T: std::ops::Mul<Output = T>
        {
            pub fn h_prod(self) -> T {
                $crate::math::macros::n_tuple_nested_call!(T::mul, self, $($components),+)
            }
        }

        impl<T> std::ops::Index<usize> for $name<T> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                $crate::math::macros::n_tuple_index_impl!(index; [&self]; $($components),+)
            }

        }

        impl<T> std::ops::IndexMut<usize> for $name<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                $crate::math::macros::n_tuple_index_impl!(index; [&mut self]; $($components),+)
            }
        }

        impl<T> std::ops::Add for $name<T>
        where
            T: std::ops::Add<Output = T> + $crate::math::number_traits::Number
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                debug_assert!(!rhs.has_nan());
                Self::Output {
                    $($components: self.$components + rhs.$components,)+
                }
            }
        }

        impl<T> std::ops::AddAssign for $name<T>
        where
            T: std::ops::AddAssign + $crate::math::number_traits::Number
        {
            fn add_assign(&mut self, rhs: Self) {
                debug_assert!(!rhs.has_nan());
                $(self.$components += rhs.$components;)+
            }
        }

        impl<T> std::ops::Sub for $name<T>
        where
            T: std::ops::Sub<Output = T> + $crate::math::number_traits::Number
        {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                debug_assert!(!rhs.has_nan());
                Self::Output {
                    $($components: self.$components - rhs.$components,)+
                }
            }
        }

        impl<T> std::ops::SubAssign for $name<T>
        where
            T: std::ops::SubAssign + $crate::math::number_traits::Number
        {
            fn sub_assign(&mut self, rhs: Self) {
                debug_assert!(!rhs.has_nan());
                $(self.$components -= rhs.$components;)+
            }
        }

        impl<T> std::ops::Mul<T> for $name<T>
        where
            T: Copy + std::ops::Mul<Output = T> + $crate::math::number_traits::Number
        {
            type Output = Self;

            fn mul(self, rhs: T) -> Self::Output {
                debug_assert!(!rhs.is_nan());
                Self::Output {
                    $($components: self.$components * rhs,)+
                }
            }
        }

        impl<T> std::ops::MulAssign<T> for $name<T>
        where
            T: Copy + std::ops::MulAssign + $crate::math::number_traits::Number
        {
            fn mul_assign(&mut self, rhs: T) {
                debug_assert!(!rhs.is_nan());
                $(self.$components *= rhs;)+
            }
        }

        $crate::math::macros::n_tuple_scalar_mul_lhs_impl!(i8, $name, $($components),+);
        $crate::math::macros::n_tuple_scalar_mul_lhs_impl!(i16, $name, $($components),+);
        $crate::math::macros::n_tuple_scalar_mul_lhs_impl!(i32, $name, $($components),+);
        $crate::math::macros::n_tuple_scalar_mul_lhs_impl!(i64, $name, $($components),+);
        $crate::math::macros::n_tuple_scalar_mul_lhs_impl!(i128, $name, $($components),+);
        $crate::math::macros::n_tuple_scalar_mul_lhs_impl!(isize, $name, $($components),+);
        $crate::math::macros::n_tuple_scalar_mul_lhs_impl!(u8, $name, $($components),+);
        $crate::math::macros::n_tuple_scalar_mul_lhs_impl!(u16, $name, $($components),+);
        $crate::math::macros::n_tuple_scalar_mul_lhs_impl!(u32, $name, $($components),+);
        $crate::math::macros::n_tuple_scalar_mul_lhs_impl!(u64, $name, $($components),+);
        $crate::math::macros::n_tuple_scalar_mul_lhs_impl!(u128, $name, $($components),+);
        $crate::math::macros::n_tuple_scalar_mul_lhs_impl!(usize, $name, $($components),+);
        $crate::math::macros::n_tuple_scalar_mul_lhs_impl!(f32, $name, $($components),+);
        $crate::math::macros::n_tuple_scalar_mul_lhs_impl!(f64, $name, $($components),+);

        impl<T> std::ops::Div<T> for $name<T>
        where
            T: Copy + std::ops::Div<Output = T> + $crate::math::number_traits::Number
        {
            type Output = Self;

            fn div(self, rhs: T) -> Self::Output {
                debug_assert!(!rhs.is_nan());
                Self::Output {
                    $($components: self.$components / rhs,)+
                }
            }
        }

        impl<T> std::ops::DivAssign<T> for $name<T>
        where
            T: Copy + std::ops::DivAssign + $crate::math::number_traits::Number
        {
            fn div_assign(&mut self, rhs: T) {
                debug_assert!(!rhs.is_nan());
                $(self.$components /= rhs;)+
            }
        }

        impl<T: std::ops::Neg<Output = T>> std::ops::Neg for $name<T> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::Output {
                    $($components: -self.$components),+
                }
            }
        }

        impl<T: std::fmt::Display> std::fmt::Display for $name<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $crate::math::macros::n_tuple_fmt_string!($name, $($components),+), $($components = self.$components),+)
            }
        }

        $crate::math::macros::n_tuple_from_impl!(i8, i16, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i8, i32, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i8, i64, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i8, i128, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i8, isize, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i8, f32, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i8, f64, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i16, i32, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i16, i64, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i16, i128, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i16, isize, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i16, f32, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i16, f64, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i32, i64, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i32, i128, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i32, f64, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(i64, i128, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u8, i16, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u8, i32, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u8, i64, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u8, i128, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u8, isize, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u8, u16, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u8, u32, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u8, u64, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u8, u128, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u8, usize, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u8, f32, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u8, f64, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u16, i32, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u16, i64, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u16, i128, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u16, u32, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u16, u64, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u16, u128, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u16, usize, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u16, f32, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u16, f64, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u32, i64, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u32, i128, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u32, u64, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u32, u128, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u32, f64, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u64, i128, $name, $($components),+);
        $crate::math::macros::n_tuple_from_impl!(u64, u128, $name, $($components),+);
    };
}

#[allow(unused)]
pub(crate) use n_tuple_component_count;
#[allow(unused)]
pub(crate) use n_tuple_fmt_string;
#[allow(unused)]
pub(crate) use n_tuple_from_impl;
pub(crate) use n_tuple_impl;
#[allow(unused)]
pub(crate) use n_tuple_index_impl;
#[allow(unused)]
pub(crate) use n_tuple_nested_call;
#[allow(unused)]
pub(crate) use n_tuple_permute_impl;
#[allow(unused)]
pub(crate) use n_tuple_scalar_mul_lhs_impl;
