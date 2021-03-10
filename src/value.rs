use crate::{Listable, Mappable, Valuable};

#[non_exhaustive]
pub enum Value<'a> {
    U8(u8),
    // More here
    Listable(&'a dyn Listable),
    Mappable(&'a dyn Mappable),
    Valuable(&'a dyn Valuable),
}