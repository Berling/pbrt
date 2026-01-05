macro_rules! number_impl {
    ($typ:ty) => {
        impl Number for $typ {
            const ONE: Self = 1;
            const MIN: Self = <$typ>::MIN;
            const MAX: Self = <$typ>::MAX;

            fn min(self, b: Self) -> Self {
                Ord::min(self, b)
            }

            fn max(self, b: Self) -> Self {
                Ord::max(self, b)
            }
        }
    };
}

macro_rules! signed_impl {
    ($typ:ty) => {
        impl Signed for $typ {
            fn abs(self) -> Self {
                self.abs()
            }
        }
    };
}

macro_rules! float_impl {
    ($typ:ty) => {
        impl Number for $typ {
            const ONE: Self = 1.0;
            const MIN: Self = <$typ>::MIN;
            const MAX: Self = <$typ>::MAX;

            fn is_nan(&self) -> bool {
                <$typ>::is_nan(*self)
            }

            fn min(self, b: Self) -> Self {
                <$typ>::min(self, b)
            }

            fn max(self, b: Self) -> Self {
                <$typ>::max(self, b)
            }
        }

        impl Float for $typ {
            fn floor(self) -> Self {
                self.floor()
            }

            fn ceil(self) -> Self {
                self.ceil()
            }
        }
    };
}

pub trait Number {
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;
    fn is_nan(&self) -> bool {
        false
    }

    fn min(self, b: Self) -> Self;
    fn max(self, b: Self) -> Self;
}

pub trait Signed: Number {
    fn abs(self) -> Self;
}

pub trait Float: Number {
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
}

number_impl!(i8);
signed_impl!(i8);
number_impl!(i16);
signed_impl!(i16);
number_impl!(i32);
signed_impl!(i32);
number_impl!(i64);
signed_impl!(i64);
number_impl!(i128);
signed_impl!(i128);
number_impl!(isize);
signed_impl!(isize);
number_impl!(u8);
number_impl!(u16);
number_impl!(u32);
number_impl!(u64);
number_impl!(u128);
number_impl!(usize);
float_impl!(f32);
signed_impl!(f32);
float_impl!(f64);
signed_impl!(f64);
