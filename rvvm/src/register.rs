#![allow(dead_code)]

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Register<T: Sized, const N: usize>([T; N]);
impl<T: Sized + Default + Copy, const N: usize> Register<T, N> {
    pub fn new() -> Self {
        Self([T::default(); N])
    }
    pub fn get<R: Into<usize>>(&self, r: R) -> T {
        self.0[r.into()]
    }
    pub fn set<R: Into<usize>>(&mut self, r: R, value: T) {
        self.0[r.into()] = value
    }
}

crate::impl_numeric_enum! {
    u8,
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub Generic [
        zero = 0,
        ra = 1,
        sp = 2,
        gp = 3,
        tp = 4,
        t0 = 5,
        t1 = 6,
        t2 = 7,
        s0 = 8,
        s1 = 9,
        a0 = 10,
        a1 = 11,
        a2 = 12,
        a3 = 13,
        a4 = 14,
        a5 = 15,
        a6 = 16,
        a7 = 17,
        s2 = 18,
        s3 = 19,
        s4 = 20,
        s5 = 21,
        s6 = 22,
        s7 = 23,
        s8 = 24,
        s9 = 25,
        s10 = 26,
        s11 = 27,
        t3 = 28,
        t4 = 29,
        t5 = 30,
        t6 = 31
    ]
}


crate::impl_numeric_enum! {
    u8,
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub Float [
        f0 = 0,
        f1 = 1,
        f2 = 2,
        f3 = 3,
        f4 = 4,
        f5 = 5,
        f6 = 6,
        f7 = 7,
        f8 = 8,
        f9 = 9,
        f10 = 10,
        f11 = 11,
        f12 = 12,
        f13 = 13,
        f14 = 14,
        f15 = 15,
        f16 = 16,
        f17 = 17,
        f18 = 18,
        f19 = 19,
        f20 = 20,
        f21 = 21,
        f22 = 22,
        f23 = 23,
        f24 = 24,
        f25 = 25,
        f26 = 26,
        f27 = 27,
        f28 = 28,
        f29 = 29,
        f30 = 30,
        f31 = 31
    ]
}
