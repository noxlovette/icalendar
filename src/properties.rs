/// Section 3.7
mod calendar;
/// Section 3.8
mod component;
pub use calendar::*;
pub use component::*;
use std::fmt::Debug;

#[derive(Debug)]
/// X Property
pub struct Xprop;
#[derive(Debug)]
/// IANA Propery
pub struct Iana;

use crate::{
    ast::parser::ParseError,
    params::{Altrep, Language},
    values::Text,
};

/// This trait ensures that all parameters as used in properties have iana and
/// x-name params 100% of the time
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

impl TryFrom<&[u8]> for SharedParams {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Params for SharedParams {
    fn get_iana(&self) -> &[Text] {
        &self.iana
    }

    fn get_xname(&self) -> &[Text] {
        &self.xname
    }
}

/// Shared + Altrep + Language
///
/// These params are shared by multiple properties:
///
/// Summary, Resources, Description, Location, Contact, etc.
struct AltrepLanguageParams {
    shared: SharedParams,
    altrep: Option<Altrep>,
    language: Option<Language>,
}
