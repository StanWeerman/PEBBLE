use std::{
    fmt::Display,
    ops::{Add, BitAnd, BitXor, Deref, DerefMut, Not},
};

#[derive(Debug, Clone, Copy)]
pub struct Lane<const WIDTH: usize>(pub [bool; WIDTH]);

impl<const WIDTH: usize> Lane<WIDTH> {
    pub fn new() -> Self {
        Self([false; WIDTH])
    }
    pub fn rotate(&mut self, count: usize) -> [bool; WIDTH] {
        let mut ret = self.clone();
        for i in 0..WIDTH {
            ret[i] = self[(i + count) % WIDTH];
        }
        ret.0
    }
}

impl<const WIDTH: usize> Deref for Lane<WIDTH> {
    type Target = [bool; WIDTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const WIDTH: usize> DerefMut for Lane<WIDTH> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const WIDTH: usize> BitAnd for Lane<WIDTH> {
    type Output = [bool; WIDTH];

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut a = [false; WIDTH];
        for i in 0..WIDTH {
            a[i] = self[i] & rhs[i];
        }
        return a;
    }
}

impl<const WIDTH: usize> BitXor for Lane<WIDTH> {
    type Output = [bool; WIDTH];

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut a = [false; WIDTH];
        for i in 0..WIDTH {
            a[i] = self[i] ^ rhs[i];
        }
        return a;
    }
}

impl<const WIDTH: usize> Not for Lane<WIDTH> {
    type Output = [bool; WIDTH];

    fn not(self) -> Self::Output {
        self.map(|bit| !bit)
    }
}
