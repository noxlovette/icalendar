use crate::{
    Calendar,
    components::{Event, FreeBusy, Timezone},
};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Node {
    Calendar(Calendar),
    Event(Event),
    FreeBusy(FreeBusy),
    Timezone(Timezone),
}
