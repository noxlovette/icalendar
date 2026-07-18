/// Section 3.7
mod calendar;
/// Section 3.8
mod component;

pub use calendar::*;
pub use component::*;

use std::fmt::Debug;

use crate::values::Text;

/// This trait ensures that all parameters as used in properties have iana and x-name params 100% of the time
pub trait Params: Default + Debug {
    fn get_iana(&self) -> &[Text];
    fn get_xname(&self) -> &[Text];
}

/// The params that every property has
///
/// That is, the IANA and non-standard property parameters
#[derive(Default, Debug)]
struct SharedParams {
    iana: Vec<Text>,
    xname: Vec<Text>,
}

impl Params for SharedParams {
    fn get_iana(&self) -> &[Text] {
        &self.iana
    }
    fn get_xname(&self) -> &[Text] {
        &self.xname
    }
}
