macro_rules! impl_try_from_bytes {
    ($ty:ident) => {
        impl_try_from_bytes!($ty, Text);
    };
    ($ty:ident, $value_ty:ty) => {
        impl_try_from_bytes!($ty, $value_ty, SharedParams);
    };
    ($ty:ident, $value_ty:ty, $param_ty:ty) => {
        impl TryFrom<&[u8]> for $ty {
            type Error = crate::ast::parser::ParseError;
            fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
                if let Some(param_start) = memchr::memchr(b';', v) {
                    let value = <$value_ty>::try_from(&v[0..param_start])?;
                    let params = <$param_ty>::try_from(&v[param_start..])?;
                    Ok(Self { value, params })
                } else {
                    Ok(Self {
                        value: v.try_into()?,
                        params: <$param_ty>::default(),
                    })
                }
            }
        }
    };
}

/// Section 3.7
mod calendar;
/// Section 3.8
mod component;
pub use calendar::*;
pub use component::*;
use std::fmt::Debug;

#[derive(Debug)]
/// X Property
pub struct Xprop(Text);
#[derive(Debug)]
/// IANA Propery
pub struct Iana(Text);

use crate::{
    ast::parser::ParseError,
    params::{Altrep, Language},
    values::Text,
};

/// This trait ensures that all parameters as used in properties have iana and
/// x-name params 100% of the time
pub trait Params<'a>: Default + Debug + TryFrom<&'a [u8]> {
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

impl<'a> Params<'a> for SharedParams {
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
#[derive(Debug)]
struct AltrepLanguageParams {
    shared: SharedParams,
    altrep: Option<Altrep>,
    language: Option<Language>,
}
