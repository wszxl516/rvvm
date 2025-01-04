#![allow(dead_code)]

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Register<T: Sized, const N: usize>([T; N]);
impl <T: Sized + Default + Copy, const N: usize>Register<T, N>{
    pub fn new()-> Self{
        Self([T::default(); N])
    }
    pub fn get<R: Into<usize>>(&self, r: R)-> T{
        self.0[r.into()]
    }
    pub fn set<R: Into<usize>>(&mut self, r: R, value: T){
        self.0[r.into()] = value
    }
}
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Generic {
    zero = 0, 
    ra, 
    sp, 
    gp, 
    tp, 
    t0, 
    t1, 
    t2,
    s0, 
    s1, 
    a0, 
    a1, 
    a2, 
    a3, 
    a4, 
    a5,
    a6, 
    a7, 
    s2, 
    s3, 
    s4, 
    s5, 
    s6, 
    s7,
    s8, 
    s9, 
    s10, 
    s11, 
    t3, 
    t4, 
    t5, 
    t6
}

impl From<u8> for Generic {
    fn from(value: u8) -> Self{
        unsafe {core::mem::transmute(value)}
    }
}
impl From<Generic> for u8 {
    fn from(value: Generic) -> Self{
        unsafe {core::mem::transmute(value)}
    }
}

impl From<usize> for Generic {
    fn from(value: usize) -> Self{
        Generic::from(value as u8)
    }
}
impl From<Generic> for usize {
    fn from(value: Generic) -> Self{
        u8::from(value) as _
    }
}


#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fcsr {
    f0 = 0, 
    f1, 
    f2, 
    f3, 
    f4, 
    f5, 
    f6, 
    f7,
    f8, 
    f9, 
    f10, 
    f11, 
    f12, 
    f13, 
    f14, 
    f15,
    f16, 
    f17, 
    f18, 
    f19, 
    f20, 
    f21, 
    f22, 
    f23,
    f24, 
    f25, 
    f26, 
    f27, 
    f28, 
    f29, 
    f30, 
    f31
}

impl From<u8> for Fcsr {
    fn from(value: u8) -> Self{
        unsafe {core::mem::transmute(value)}
    }
}
impl From<Fcsr> for u8 {
    fn from(value: Fcsr) -> Self{
        unsafe {core::mem::transmute(value)}
    }
}

impl From<usize> for Fcsr {
    fn from(value: usize) -> Self{
        Fcsr::from(value as u8)
    }
}
impl From<Fcsr> for usize {
    fn from(value: Fcsr) -> Self{
        u8::from(value) as _
    }
}