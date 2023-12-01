use std::{
    fmt::Display,
    fmt::Formatter,
    fmt::Result,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub enum Solution {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    Str(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Solution::I8(x) => x.fmt(f),
            Solution::I16(x) => x.fmt(f),
            Solution::I32(x) => x.fmt(f),
            Solution::I64(x) => x.fmt(f),
            Solution::I128(x) => x.fmt(f),
            Solution::Isize(x) => x.fmt(f),
            Solution::U8(x) => x.fmt(f),
            Solution::U16(x) => x.fmt(f),
            Solution::U32(x) => x.fmt(f),
            Solution::U64(x) => x.fmt(f),
            Solution::U128(x) => x.fmt(f),
            Solution::Usize(x) => x.fmt(f),
            Solution::Str(x) => x.fmt(f),
        }
    }
}

#[macro_export]
macro_rules! return_sol {
    ($value:expr) => {
        utils::Solution::from($value)
    };
}

macro_rules! impl_from {
    ($type_:ident, $kind_:ident) => {
        impl From<$type_> for Solution {
            fn from(sol: $type_) -> Self {
                Self::$kind_(sol)
            }
        }
    };
}

impl From<&str> for Solution {
    fn from(sol: &str) -> Self {
        Self::Str(sol.to_owned())
    }
}

impl_from!(i8, I8);
impl_from!(i16, I16);
impl_from!(i32, I32);
impl_from!(i64, I64);
impl_from!(i128, I128);
impl_from!(isize, Isize);
impl_from!(u8, U8);
impl_from!(u16, U16);
impl_from!(u32, U32);
impl_from!(u64, U64);
impl_from!(u128, U128);
impl_from!(usize, Usize);

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let out: Vec<String> = buf.lines().map(|line| line.expect("error")).collect();
    out
}
