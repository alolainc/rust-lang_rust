#![feature(const_generics)]

// All of these three items must be in `lib2` to reproduce the error

pub trait TypeFn {
    type Output;
}

pub struct GenericType<const B: i8>;

// Removing the braces around `42` resolves the crash
impl TypeFn for GenericType<{ 42 }> {
    type Output = ();
}
