#![no_std]

use num_traits as num;

use core::convert::{TryFrom, TryInto};
use core::num::TryFromIntError;
use core::ops;

#[inline]
fn try_from_int_error() -> TryFromIntError {
    // XXX: Literally the dumbest thing I've ever had to write.
    u8::try_from(256).unwrap_err()
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct U256 {
    high: u128,
    low: u128,
}

impl<T> From<T> for U256
where
    T: Into<u128>,
{
    #[inline]
    fn from(t: T) -> Self {
        Self {
            high: 0,
            low: t.into(),
        }
    }
}

macro_rules! impl_try_from_u256 {
    ($($int:ty, )+) => {
        $(
        impl TryFrom<U256> for $int {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(u: U256) -> Result<Self, Self::Error> {
                if u.high == 0 {
                    Self::try_from(u.low)
                } else {
                    Err(try_from_int_error())
                }
            }
        }

        impl TryFrom<&U256> for $int {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(u: &U256) -> Result<Self, Self::Error> {
                if u.high == 0 {
                    Self::try_from(u.low)
                } else {
                    Err(try_from_int_error())
                }
            }
        }
        )+
    }
}

impl_try_from_u256!(i8, u8, i16, u16, i32, u32, i64, u64, i128, isize, usize,);

impl TryFrom<U256> for u128 {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(u: U256) -> Result<Self, Self::Error> {
        if u.high == 0 {
            Ok(u.low)
        } else {
            Err(try_from_int_error())
        }
    }
}

impl TryFrom<&U256> for u128 {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(u: &U256) -> Result<Self, Self::Error> {
        if u.high == 0 {
            Ok(u.low)
        } else {
            Err(try_from_int_error())
        }
    }
}

impl U256 {
    #[inline]
    pub const fn new(low: u128) -> Self {
        Self { low, high: 0 }
    }

    #[inline]
    pub const fn with_high_order(high: u128, low: u128) -> Self {
        Self { high, low }
    }

    #[inline]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        let (low, low_overflow) = self.low.overflowing_add(rhs.low);
        let high = self.high.wrapping_add(rhs.high);
        U256 {
            low,
            high: if low_overflow {
                high.wrapping_add(1)
            } else {
                high
            },
        }
    }

    #[inline]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        // TODO: Optimize this.

        let (mid, mid_overflow) = self.high.overflowing_add(rhs.high);

        if mid_overflow {
            return None;
        }

        let (low, low_overflow) = self.low.overflowing_add(rhs.low);

        if !low_overflow {
            return Some(Self { high: mid, low });
        }

        let (high, high_overflow) = mid.overflowing_add(1);

        if high_overflow {
            None
        } else {
            Some(Self { high, low })
        }
    }

    #[inline]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        match self.checked_add(rhs) {
            Some(v) => v,
            None => Self::max_value(),
        }
    }

    #[inline]
    pub const fn wrapping_mul(self, _rhs: Self) -> Self {
        // TODO
        self
    }

    #[inline]
    pub const fn checked_mul(self, _rhs: Self) -> Option<Self> {
        // TODO
        None
    }

    #[inline]
    pub const fn saturating_mul(self, _rhs: Self) -> Self {
        // TODO
        self
    }

    #[inline]
    pub const fn wrapping_sub(self, _rhs: Self) -> Self {
        // TODO
        self
    }

    #[inline]
    pub const fn checked_sub(self, _rhs: Self) -> Option<Self> {
        // TODO
        None
    }

    #[inline]
    pub const fn saturating_sub(self, _rhs: Self) -> Self {
        // TODO
        self
    }

    #[inline]
    pub const fn wrapping_div(self, _rhs: Self) -> Self {
        // TODO
        self
    }

    #[inline]
    pub const fn checked_div(self, _rhs: Self) -> Option<Self> {
        // TODO
        None
    }

    #[inline]
    pub const fn wrapping_rem(self, _rhs: Self) -> Self {
        // TODO
        self
    }

    #[inline]
    pub const fn checked_rem(self, _rhs: Self) -> Option<Self> {
        // TODO
        None
    }

    #[inline]
    pub const fn checked_shr(self, _cnt: Self) -> Option<Self> {
        // TODO
        None
    }

    #[inline]
    pub const fn wrapping_shr(self, _cnt: Self) -> Self {
        // TODO
        self
    }

    #[inline]
    pub const fn checked_shl(self, _cnt: Self) -> Option<Self> {
        // TODO
        None
    }

    #[inline]
    pub const fn wrapping_shl(self, _cnt: Self) -> Self {
        // TODO
        self
    }

    #[inline]
    pub const fn signed_shr(self, _cnt: Self) -> Self {
        // TODO
        self
    }

    #[inline]
    pub const fn signed_shl(self, _cnt: Self) -> Self {
        // TODO
        self
    }

    #[inline]
    pub const fn swap_bytes(self) -> Self {
        Self {
            high: self.low.swap_bytes(),
            low: self.high.swap_bytes(),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    pub const fn from_be(v: Self) -> Self {
        // TODO
        v
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    pub const fn from_le(v: Self) -> Self {
        // TODO
        v
    }

    #[inline]
    pub const fn to_be(self) -> Self {
        // TODO
        self
    }

    #[inline]
    pub const fn to_le(self) -> Self {
        // TODO
        self
    }

    #[inline]
    pub fn pow(self, exp: Self) -> Self {
        #[cfg(debug_assertions)]
        let result = self.checked_pow(exp).unwrap();

        #[cfg(not(debug_assertions))]
        let result = self.wrapping_pow(exp);

        result
    }

    #[inline]
    pub const fn wrapping_pow(self, _: Self) -> Self {
        // TODO
        self
    }

    #[inline]
    pub const fn checked_pow(self, _: Self) -> Option<Self> {
        // TODO
        None
    }

    #[inline]
    pub const fn saturating_pow(self, _: Self) -> Self {
        // TODO
        self
    }

    #[inline]
    pub const fn min_value() -> Self {
        Self { high: 0, low: 0 }
    }

    #[inline]
    pub const fn max_value() -> Self {
        Self {
            high: u128::max_value(),
            low: u128::max_value(),
        }
    }

    #[inline]
    pub const fn count_ones(self) -> u32 {
        self.low.count_ones() + self.high.count_ones()
    }

    #[inline]
    pub const fn count_zeros(self) -> u32 {
        self.low.count_zeros() + self.high.count_zeros()
    }

    #[inline]
    pub const fn leading_zeros(self) -> u32 {
        if self.high == 0 {
            128 + self.low.leading_zeros()
        } else {
            self.high.leading_zeros()
        }
    }

    #[inline]
    pub const fn trailing_zeros(self) -> u32 {
        if self.low == 0 {
            128 + self.high.trailing_zeros()
        } else {
            self.low.trailing_zeros()
        }
    }

    #[inline]
    pub const fn rotate_left(self, _cnt: u32) -> Self {
        // TODO
        self
    }

    #[inline]
    pub const fn rotate_right(self, _cnt: u32) -> Self {
        // TODO
        self
    }

    #[inline]
    pub const fn to_le_bytes(self) -> [u8; 32] {
        // TODO
        [0; 32]
    }

    #[inline]
    pub const fn to_be_bytes(self) -> [u8; 32] {
        // TODO
        [0; 32]
    }

    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; 32] {
        // TODO
        [0; 32]
    }

    #[inline]
    pub const fn from_be_bytes(_: [u8; 32]) -> Self {
        // TODO
        Self { high: 0, low: 0 }
    }

    #[inline]
    pub const fn from_le_bytes(_: [u8; 32]) -> Self {
        // TODO
        Self { high: 0, low: 0 }
    }

    #[inline]
    pub const fn from_ne_bytes(_: [u8; 32]) -> Self {
        // TODO
        Self { high: 0, low: 0 }
    }
}

impl ops::BitAnd for U256 {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            high: self.high & rhs.high,
            low: self.low & rhs.low,
        }
    }
}

impl ops::BitOr for U256 {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            high: self.high | rhs.high,
            low: self.low | rhs.low,
        }
    }
}

impl ops::BitXor for U256 {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            high: self.high ^ rhs.high,
            low: self.low ^ rhs.low,
        }
    }
}

impl ops::Not for U256 {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self {
            high: !self.high,
            low: !self.low,
        }
    }
}

impl ops::Rem<&Self> for U256 {
    type Output = Self;

    #[inline]
    fn rem(self, _rhs: &Self) -> Self::Output {
        // TODO
        self
    }
}

impl ops::Rem for U256 {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        let result = self.checked_rem(rhs).unwrap();

        #[cfg(not(debug_assertions))]
        let result = self.wrapping_rem(rhs);

        result
    }
}

impl ops::Div<&Self> for U256 {
    type Output = Self;

    #[inline]
    fn div(self, _rhs: &Self) -> Self::Output {
        // TODO
        self
    }
}

impl ops::Div for U256 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        let result = self.checked_div(rhs).unwrap();

        #[cfg(not(debug_assertions))]
        let result = self.wrapping_div(rhs);

        result
    }
}

impl ops::Sub<&Self> for U256 {
    type Output = Self;

    #[inline]
    fn sub(self, _rhs: &Self) -> Self::Output {
        // TODO
        self
    }
}

impl ops::Sub for U256 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        let result = self.checked_sub(rhs).unwrap();

        #[cfg(not(debug_assertions))]
        let result = self.wrapping_sub(rhs);

        result
    }
}

impl ops::Mul<&Self> for U256 {
    type Output = Self;

    #[inline]
    fn mul(self, _rhs: &Self) -> Self::Output {
        // TODO
        self
    }
}

impl ops::Mul for U256 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        let result = self.checked_mul(rhs).unwrap();

        #[cfg(not(debug_assertions))]
        let result = self.wrapping_mul(rhs);

        result
    }
}

impl ops::Add<&Self> for U256 {
    type Output = Self;

    #[inline]
    fn add(self, _rhs: &Self) -> Self::Output {
        // TODO
        self
    }
}

impl ops::Add for U256 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        let result = self.checked_add(rhs).unwrap();

        #[cfg(not(debug_assertions))]
        let result = self.wrapping_add(rhs);

        result
    }
}

impl ops::AddAssign<&Self> for U256 {
    #[inline]
    fn add_assign(&mut self, _rhs: &Self) {
        // TODO
    }
}

impl ops::AddAssign for U256 {
    #[inline]
    fn add_assign(&mut self, _rhs: Self) {
        // TODO
    }
}

impl ops::SubAssign<&Self> for U256 {
    #[inline]
    fn sub_assign(&mut self, _rhs: &Self) {
        // TODO
    }
}

impl ops::SubAssign for U256 {
    #[inline]
    fn sub_assign(&mut self, _rhs: Self) {
        // TODO
    }
}

impl ops::MulAssign<&Self> for U256 {
    #[inline]
    fn mul_assign(&mut self, _rhs: &Self) {
        // TODO
    }
}

impl ops::MulAssign for U256 {
    #[inline]
    fn mul_assign(&mut self, _rhs: Self) {
        // TODO
    }
}

impl ops::DivAssign<&Self> for U256 {
    #[inline]
    fn div_assign(&mut self, _rhs: &Self) {
        // TODO
    }
}

impl ops::DivAssign for U256 {
    #[inline]
    fn div_assign(&mut self, _rhs: Self) {
        // TODO
    }
}

impl ops::RemAssign<&Self> for U256 {
    #[inline]
    fn rem_assign(&mut self, _rhs: &Self) {
        // TODO
    }
}

impl ops::RemAssign for U256 {
    #[inline]
    fn rem_assign(&mut self, _rhs: Self) {
        // TODO
    }
}

macro_rules! impl_shl {
    ($($int:ty, )+) => {
        $(
        impl ops::Shl<$int> for U256 {
            type Output = Self;

            #[inline]
            fn shl(self, rhs: $int) -> Self::Output {
                let low: u128 = rhs.try_into().unwrap();
                ops::Shl::<Self>::shl(self, Self { high: 0, low })
            }
        }
        )+
    }
}

macro_rules! impl_shr {
    ($($int:ty, )+) => {
        $(
        impl ops::Shr<$int> for U256 {
            type Output = Self;

            #[inline]
            fn shr(self, rhs: $int) -> Self::Output {
                let low: u128 = rhs.try_into().unwrap();
                ops::Shr::<Self>::shr(self, Self { high: 0, low })
            }
        }
        )+
    }
}

impl_shr!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,);
impl_shl!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize,);

impl ops::Shl for U256 {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        let result = self.checked_shl(rhs).unwrap();

        #[cfg(not(debug_assertions))]
        let result = self.wrapping_shl(rhs);

        result
    }
}

impl ops::Shr for U256 {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        let result = self.checked_shr(rhs).unwrap();

        #[cfg(not(debug_assertions))]
        let result = self.wrapping_shr(rhs);

        result
    }
}

impl num::One for U256 {
    #[inline]
    fn one() -> Self {
        Self { high: 0, low: 1 }
    }
}

impl num::Zero for U256 {
    #[inline]
    fn zero() -> Self {
        Self { high: 0, low: 0 }
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.high == 0 && self.low == 0
    }
}

impl num::Bounded for U256 {
    #[inline]
    fn min_value() -> Self {
        U256::min_value()
    }

    #[inline]
    fn max_value() -> Self {
        U256::max_value()
    }
}

impl num::Saturating for U256 {
    #[inline]
    fn saturating_add(self, rhs: Self) -> Self {
        U256::saturating_add(self, rhs)
    }

    #[inline]
    fn saturating_sub(self, rhs: Self) -> Self {
        U256::saturating_sub(rhs, rhs)
    }
}

impl num::SaturatingMul for U256 {
    #[inline]
    fn saturating_mul(&self, rhs: &Self) -> Self {
        U256::saturating_mul(*self, *rhs)
    }
}

impl num::SaturatingSub for U256 {
    #[inline]
    fn saturating_sub(&self, rhs: &Self) -> Self {
        U256::saturating_sub(*self, *rhs)
    }
}

impl num::SaturatingAdd for U256 {
    #[inline]
    fn saturating_add(&self, rhs: &Self) -> Self {
        U256::saturating_add(*self, *rhs)
    }
}

impl<T> num::Pow<T> for U256
where
    T: Into<Self>,
{
    type Output = Self;

    #[inline]
    fn pow(self, rhs: T) -> Self {
        Self::pow(self, Into::<Self>::into(rhs))
    }
}

impl num::ToPrimitive for U256 {
    #[inline]
    fn to_u64(&self) -> Option<u64> {
        TryFrom::try_from(self).ok()
    }

    #[inline]
    fn to_i64(&self) -> Option<i64> {
        TryFrom::try_from(self).ok()
    }

    #[inline]
    fn to_isize(&self) -> Option<isize> {
        TryFrom::try_from(self).ok()
    }

    #[inline]
    fn to_i8(&self) -> Option<i8> {
        TryFrom::try_from(self).ok()
    }

    #[inline]
    fn to_i16(&self) -> Option<i16> {
        TryFrom::try_from(self).ok()
    }

    #[inline]
    fn to_i32(&self) -> Option<i32> {
        TryFrom::try_from(self).ok()
    }

    #[inline]
    fn to_i128(&self) -> Option<i128> {
        TryFrom::try_from(self).ok()
    }

    #[inline]
    fn to_usize(&self) -> Option<usize> {
        TryFrom::try_from(self).ok()
    }

    #[inline]
    fn to_u8(&self) -> Option<u8> {
        TryFrom::try_from(self).ok()
    }

    #[inline]
    fn to_u16(&self) -> Option<u16> {
        TryFrom::try_from(self).ok()
    }

    #[inline]
    fn to_u32(&self) -> Option<u32> {
        TryFrom::try_from(self).ok()
    }

    #[inline]
    fn to_u128(&self) -> Option<u128> {
        TryFrom::try_from(self).ok()
    }
}

impl num::NumCast for U256 {
    #[inline]
    fn from<T>(n: T) -> Option<Self>
    where
        T: num::ToPrimitive,
    {
        n.to_u128().map(Self::new)
    }
}

impl num::PrimInt for U256 {
    #[inline]
    fn count_ones(self) -> u32 {
        Self::count_ones(self)
    }

    #[inline]
    fn count_zeros(self) -> u32 {
        Self::count_zeros(self)
    }

    #[inline]
    fn leading_zeros(self) -> u32 {
        Self::leading_zeros(self)
    }

    #[inline]
    fn trailing_zeros(self) -> u32 {
        Self::trailing_zeros(self)
    }

    #[inline]
    fn rotate_left(self, cnt: u32) -> Self {
        Self::rotate_left(self, cnt)
    }

    #[inline]
    fn rotate_right(self, cnt: u32) -> Self {
        Self::rotate_right(self, cnt)
    }

    #[inline]
    fn signed_shr(self, cnt: u32) -> Self {
        Self::signed_shr(self, Self::from(cnt))
    }

    #[inline]
    fn unsigned_shr(self, cnt: u32) -> Self {
        ops::Shr::shr(self, cnt)
    }

    #[inline]
    fn signed_shl(self, cnt: u32) -> Self {
        Self::signed_shl(self, Self::from(cnt))
    }

    #[inline]
    fn unsigned_shl(self, cnt: u32) -> Self {
        ops::Shl::shl(self, cnt)
    }

    #[inline]
    fn swap_bytes(self) -> Self {
        Self::swap_bytes(self)
    }

    #[inline]
    fn from_be(v: Self) -> Self {
        Self::from_be(v)
    }

    #[inline]
    fn from_le(v: Self) -> Self {
        Self::from_le(v)
    }

    #[inline]
    fn to_be(self) -> Self {
        Self::to_be(self)
    }

    #[inline]
    fn to_le(self) -> Self {
        Self::to_le(self)
    }

    #[inline]
    fn pow(self, exp: u32) -> Self {
        Self::pow(self, Self::from(exp))
    }
}

impl num::Num for U256 {
    type FromStrRadixErr = (); // TODO

    fn from_str_radix(_: &str, _: u32) -> Result<Self, Self::FromStrRadixErr> {
        Err(()) // TODO
    }
}

impl num::CheckedAdd for U256 {
    #[inline]
    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        U256::checked_add(*self, *rhs)
    }
}

impl num::CheckedSub for U256 {
    #[inline]
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        U256::checked_sub(*self, *rhs)
    }
}

impl num::CheckedMul for U256 {
    #[inline]
    fn checked_mul(&self, rhs: &Self) -> Option<Self> {
        U256::checked_mul(*self, *rhs)
    }
}

impl num::CheckedDiv for U256 {
    #[inline]
    fn checked_div(&self, rhs: &Self) -> Option<Self> {
        U256::checked_div(*self, *rhs)
    }
}

impl num::CheckedRem for U256 {
    #[inline]
    fn checked_rem(&self, rhs: &Self) -> Option<Self> {
        U256::checked_rem(*self, *rhs)
    }
}

impl num::CheckedShl for U256 {
    #[inline]
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        U256::checked_shl(*self, rhs.into())
    }
}

impl num::CheckedShr for U256 {
    #[inline]
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        U256::checked_shr(*self, rhs.into())
    }
}

impl num::WrappingAdd for U256 {
    #[inline]
    fn wrapping_add(&self, rhs: &Self) -> Self {
        U256::wrapping_add(*self, *rhs)
    }
}

impl num::WrappingSub for U256 {
    #[inline]
    fn wrapping_sub(&self, rhs: &Self) -> Self {
        U256::wrapping_sub(*self, *rhs)
    }
}

impl num::WrappingMul for U256 {
    #[inline]
    fn wrapping_mul(&self, rhs: &Self) -> Self {
        U256::wrapping_mul(*self, *rhs)
    }
}

impl num::WrappingShl for U256 {
    #[inline]
    fn wrapping_shl(&self, rhs: u32) -> Self {
        U256::wrapping_shl(*self, rhs.into())
    }
}

impl num::WrappingShr for U256 {
    #[inline]
    fn wrapping_shr(&self, rhs: u32) -> Self {
        U256::wrapping_shr(*self, rhs.into())
    }
}

impl num::Unsigned for U256 {}