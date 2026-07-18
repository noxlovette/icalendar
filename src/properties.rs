/// Section 3.7
mod calendar;
/// Section 3.8
mod component;

pub use calendar::*;
pub use component::*;

use crate::params::PropertyParams;

pub trait Property<T> {
    fn get_params(&self) -> &[PropertyParams];
    fn get_value(&self) -> &T;
}
