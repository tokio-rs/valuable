use crate::{Definition, Field, Value};

use core::fmt;

pub trait Structable {
    fn definition(&self) -> Definition<'_>;

    fn get(&self, field: &Field<'_>) -> Option<Value<'_>>;

    fn with_iter_fn_mut(&self, f: &mut dyn FnMut(Iter<'_>));
}

type Iter<'a> = &'a mut dyn Iterator<Item = (Field<'a>, Value<'a>)>;

impl<'a> dyn Structable + 'a {
    fn with_iter<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Iter<'_>) -> R
    {
        let mut out = None;
        let mut f = Some(f);

        self.with_iter_fn_mut(&mut |iter| {
            out = Some(f.take().unwrap()(iter));
        });

        out.take().unwrap()
    }
}

pub(crate) fn debug(value: &dyn Structable, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    let def = value.definition();

    let mut f = fmt.debug_struct(def.name());

    value.with_iter(|iter| {
        for (field, value) in iter {
            f.field(field.name(), &value);
        }
    });

    f.finish()
}
