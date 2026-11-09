use crate::aliases::Traits;

pub trait Float: Traits {
    fn is_sign_negative(self) -> bool;

    fn is_sign_positive(self) -> bool;

    fn is_infinite(self) -> bool;

    fn is_finite(self) -> bool;

    fn is_nan(self) -> bool;
}

inherent!(f32, f64);

pub type DefaultFloat = f64;

macro_rules! inherent {
    ($($type: ty),+ $(,)?) => {
        $(
            impl $crate::float::Float for $type {
                fn is_sign_negative(self) -> bool {
                    self.is_sign_negative()
                }

                fn is_sign_positive(self) -> bool {
                    self.is_sign_positive()
                }

                fn is_infinite(self) -> bool {
                    self.is_infinite()
                }

                fn is_finite(self) -> bool {
                    self.is_finite()
                }

                fn is_nan(self) -> bool {
                    self.is_nan()
                }
            }
        )+
    }
}

pub(crate) use inherent;
