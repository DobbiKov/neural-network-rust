use std::{cmp::max, f64::consts::E};

#[derive(Clone)]
pub struct  Activation<'a>{
    pub function: &'a dyn Fn(f64) -> f64,
    pub derivative: &'a dyn Fn(f64) -> f64,
}

pub const SIGMOID: Activation = Activation{
    function: &|x| 1.0/(1.0 + E.powf(-x)),
    derivative: &|x| x * (1.0 - x),
};

pub const RELU: Activation = Activation{
    function: &|x| if x >= 0.0 { x } else { 0.0 },
    derivative: &|x| if x <= 0.0 { 0.0 } else { 1.0 }
};