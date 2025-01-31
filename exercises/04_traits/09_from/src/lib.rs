// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

#[allow(dead_code)]
pub struct WrappingU32 {
    value: u32,
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}

impl std::convert::From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        Self { value }
    }
}
