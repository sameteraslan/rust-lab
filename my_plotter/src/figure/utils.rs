use std::convert::TryFrom;

fn add<T, U, R>(a: T, b: U) -> R
where
    T: Into<R>,
    U: Into<R>,
    R: std::ops::Add<Output = R>,
{
    let a_converted: R = a.into();
    let b_converted: R = b.into();
    a_converted + b_converted
}

fn sub<T, U, R>(a: T, b: U) -> R
where
    T: Into<R>,
    U: Into<R>,
    R: std::ops::Sub<Output = R>,
{
    let a_converted: R = a.into();
    let b_converted: R = b.into();
    a_converted - b_converted
}
