use crate::{tensor::ops::*, Distribution};
use half::f16;
use num_traits::ToPrimitive;
use rand::prelude::StdRng;

pub trait Element:
    Zeros<Self>
    + ToPrimitive
    + ElementRandom<Self>
    + ElementConversion
    + ElementPrecision
    + ElementValue
    + Ones<Self>
    + std::ops::Mul<Self, Output = Self>
    + std::fmt::Debug
    + Default
    + 'static
    + Send
    + Sync
    + Copy
    + std::cmp::PartialOrd<Self>
{
}

#[cfg(feature = "tch")]
pub(crate) trait TchElement: Element + tch::kind::Element {}

pub(crate) trait ExpElement {
    fn exp_elem(self) -> Self;
    fn log_elem(self) -> Self;
    fn pow_elem(self, value: f32) -> Self;
}

pub trait ElementConversion {
    fn from_elem<E: ToPrimitive>(elem: E) -> Self;
    fn to_elem<E: Element>(&self) -> E;
}

pub trait ElementRandom<T> {
    fn random(distribution: Distribution<T>, rng: &mut StdRng) -> T;
}

pub trait ElementValue {
    fn inf() -> Self;
    fn inf_neg() -> Self;
    fn nan() -> Self;
    fn zero() -> Self;
    fn one() -> Self;
}

#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub enum Precision {
    Double,
    Full,
    Half,
    Other,
}

pub trait ElementPrecision {
    fn precision() -> Precision;
}

#[cfg(feature = "ndarray")]
pub(crate) trait NdArrayElement:
    Element + ndarray::LinalgScalar + ndarray::ScalarOperand + ExpElement + num_traits::FromPrimitive
{
}

macro_rules! ad_items {
    (
        ty $float:ident $precision:expr,
        zero $zero:expr,
        one $one:expr,
        convert $convert:expr,
        random $random:expr

    ) => {
        impl Element for $float {}

        impl Zeros<$float> for $float {
            fn zeros(&self) -> $float {
                $zero
            }
        }

        impl ElementConversion for $float {
            fn from_elem<E: ToPrimitive>(elem: E) -> Self {
                $convert(&elem)
            }
            fn to_elem<E: Element>(&self) -> E {
                E::from_elem(*self)
            }
        }

        impl ElementValue for $float {
            fn inf() -> Self {
                Self::from_elem(f64::INFINITY)
            }
            fn inf_neg() -> Self {
                Self::from_elem(std::ops::Neg::neg(f64::INFINITY))
            }
            fn nan() -> Self {
                Self::from_elem(f64::NAN)
            }
            fn zero() -> Self {
                $zero
            }
            fn one() -> Self {
                $one
            }
        }

        impl ElementPrecision for $float {
            fn precision() -> Precision {
                $precision
            }
        }

        impl ElementRandom<$float> for $float {
            fn random(distribution: Distribution<$float>, rng: &mut StdRng) -> $float {
                $random(distribution, rng)
            }
        }

        impl Ones<$float> for $float {
            fn ones(&self) -> $float {
                $one
            }
        }
    };
    (
        float $float:ident $precision:expr,
        convert $convert:expr,
        random $random:expr
    ) => {
        ad_items!(
            ty $float $precision,
            zero 0.0,
            one 1.0,
            convert $convert,
            random $random
        );
    };
    (
        int $int:ident $precision:expr,
        convert $convert:expr,
        random $random:expr
    ) => {
        ad_items!(
            ty $int $precision,
            zero 0,
            one 1,
            convert $convert,
            random $random
        );
    };
}

ad_items!(
    float f64 Precision::Double,
    convert |elem: &dyn ToPrimitive| elem.to_f64().unwrap(),
    random |distribution: Distribution<f64>, rng: &mut StdRng| distribution.sampler(rng).sample()
);

ad_items!(
    ty f16 Precision::Half,
    zero <f16 as num_traits::Zero>::zero(),
    one <f16 as num_traits::One>::one(),
    convert |elem: &dyn ToPrimitive| f16::from_f32(elem.to_f32().unwrap()),
    random |distribution: Distribution<f16>, rng: &mut StdRng| {
        let distribution: Distribution<f32> = distribution.convert();
        let sample = distribution.sampler(rng).sample();
        f16::from_elem(sample)
    }
);
ad_items!(
    float f32 Precision::Full,
    convert |elem: &dyn ToPrimitive| elem.to_f32().unwrap(),
    random |distribution: Distribution<f32>, rng: &mut StdRng| distribution.sampler(rng).sample()
);

ad_items!(
    int i64 Precision::Double,
    convert |elem: &dyn ToPrimitive| elem.to_i64().unwrap(),
    random |distribution: Distribution<i64>, rng: &mut StdRng| distribution.sampler(rng).sample()
);
ad_items!(
    int i32 Precision::Full,
    convert |elem: &dyn ToPrimitive| elem.to_i32().unwrap(),
    random |distribution: Distribution<i32>, rng: &mut StdRng| distribution.sampler(rng).sample()
);
ad_items!(
    int i16 Precision::Half,
    convert |elem: &dyn ToPrimitive| elem.to_i16().unwrap(),
    random |distribution: Distribution<i16>, rng: &mut StdRng| distribution.sampler(rng).sample()
);
ad_items!(
    int i8 Precision::Other,
    convert |elem: &dyn ToPrimitive| elem.to_i8().unwrap(),
    random |distribution: Distribution<i8>, rng: &mut StdRng| distribution.sampler(rng).sample()
);

ad_items!(
    int u8 Precision::Other,
    convert |elem: &dyn ToPrimitive| elem.to_u8().unwrap(),
    random |distribution: Distribution<u8>, rng: &mut StdRng| distribution.sampler(rng).sample()
);

#[cfg(feature = "tch")]
mod tch_elem {
    use super::*;

    impl TchElement for f64 {}
    impl TchElement for f32 {}
    impl TchElement for f16 {}

    impl TchElement for i64 {}
    impl TchElement for i32 {}
    impl TchElement for i16 {}

    impl TchElement for u8 {}
}

#[cfg(feature = "ndarray")]
mod ndarray_elem {
    use super::*;

    macro_rules! impl_exp_elem {
        ($elem:ident) => {
            impl ExpElement for $elem {
                fn exp_elem(self) -> Self {
                    $elem::exp(self)
                }
                fn log_elem(self) -> Self {
                    $elem::ln(self)
                }
                fn pow_elem(self, value: f32) -> Self {
                    $elem::powf(self, value.into())
                }
            }
        };
        ($elem:ident, $tmp:ident) => {
            impl ExpElement for $elem {
                fn exp_elem(self) -> Self {
                    let tmp = $tmp::exp(self as $tmp);
                    tmp as $elem
                }
                fn log_elem(self) -> Self {
                    let tmp = $tmp::ln(self as $tmp);
                    tmp as $elem
                }
                fn pow_elem(self, value: f32) -> Self {
                    let tmp = $tmp::powf(self as $tmp, value as $tmp);
                    tmp as $elem
                }
            }
        };
    }

    impl NdArrayElement for f64 {}
    impl_exp_elem!(f64);

    impl NdArrayElement for f32 {}
    impl_exp_elem!(f32);

    impl NdArrayElement for i64 {}
    impl_exp_elem!(i64, f64);

    impl NdArrayElement for i32 {}
    impl_exp_elem!(i32, f32);

    impl NdArrayElement for i16 {}
    impl_exp_elem!(i16, f32);

    impl NdArrayElement for u8 {}
    impl_exp_elem!(u8, f32);
}
