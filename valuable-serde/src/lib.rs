//! [`serde::Serialize`] implementation for [`valuable::Value`].
//!
//! # Examples
//!
//! ```
//! use valuable::Valuable;
//! use valuable_serde::Serializable;
//!
//! #[derive(Valuable)]
//! struct Point {
//!     x: i32,
//!     y: i32,
//! }
//!
//! let point = Point { x: 1, y: 2 };
//!
//! let value = Serializable::new(&point);
//!
//! assert_eq!(
//!     serde_json::to_string(&value).unwrap(),
//!     r#"{"x":1,"y":2}"#,
//! );
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs, rust_2018_idioms)]

use core::{fmt, mem};

use serde::ser::{
    Error, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{Serialize, Serializer};
use valuable::field::Fields;
use valuable::{EnumDef, NamedValues, StructDef, Valuable, Value, Variant, VariantDef, Visit};

/// A wrapper around [`Value`] that implements [`Serialize`].
pub struct Serializable<V>(V);

impl<V> Serializable<V>
where
    V: Valuable,
{
    /// Creates a new `Serializable`.
    pub fn new(v: V) -> Self {
        Self(v)
    }

    /// Returns a reference to the underlying value.
    pub fn get_ref(&self) -> &V {
        &self.0
    }

    /// Returns a mutable reference to the underlying value.
    pub fn get_mut(&mut self) -> &mut V {
        &mut self.0
    }

    /// Unwraps this `Serializable`, returning the underlying value.
    pub fn into_inner(self) -> V {
        self.0
    }
}

impl<V> fmt::Debug for Serializable<V>
where
    V: Valuable,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_value(), f)
    }
}

impl<V> Valuable for Serializable<V>
where
    V: Valuable,
{
    fn as_value(&self) -> Value<'_> {
        self.0.as_value()
    }

    fn visit(&self, visit: &mut dyn Visit) {
        self.0.visit(visit);
    }
}

impl<V> Serialize for Serializable<V>
where
    V: Valuable,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0.as_value() {
            Value::Bool(b) => serializer.serialize_bool(b),
            Value::I8(n) => serializer.serialize_i8(n),
            Value::I16(n) => serializer.serialize_i16(n),
            Value::I32(n) => serializer.serialize_i32(n),
            Value::I64(n) => serializer.serialize_i64(n),
            Value::I128(n) => serializer.serialize_i128(n),
            Value::Isize(n) => serializer.serialize_i64(n as _),
            Value::U8(n) => serializer.serialize_u8(n),
            Value::U16(n) => serializer.serialize_u16(n),
            Value::U32(n) => serializer.serialize_u32(n),
            Value::U64(n) => serializer.serialize_u64(n),
            Value::U128(n) => serializer.serialize_u128(n),
            Value::Usize(n) => serializer.serialize_u64(n as _),
            Value::F32(n) => serializer.serialize_f32(n),
            Value::F64(n) => serializer.serialize_f64(n),
            Value::Char(c) => serializer.serialize_char(c),
            Value::String(s) => serializer.serialize_str(s),
            Value::Unit => serializer.serialize_none(),
            Value::Listable(l) => {
                let size_hint = l.size_hint();
                let mut ser = serializer.serialize_seq(Some(size_hint.1.unwrap_or(size_hint.0)))?;
                let mut visitor = VisitList::<S>::Serializer(&mut ser);
                l.visit(&mut visitor);
                if let VisitList::Error(e) = visitor {
                    return Err(e);
                }
                ser.end()
            }
            Value::Mappable(m) => {
                let size_hint = m.size_hint();
                let mut ser = serializer.serialize_map(size_hint.1)?;
                let mut visitor = VisitMap::<S>::Serializer(&mut ser);
                m.visit(&mut visitor);
                if let VisitMap::Error(e) = visitor {
                    return Err(e);
                }
                ser.end()
            }
            Value::Structable(s) => match s.definition() {
                StructDef::Static { name, fields, .. } => {
                    let mut visitor = VisitStaticStruct::Start {
                        name,
                        fields,
                        serializer,
                    };
                    s.visit(&mut visitor);
                    match visitor {
                        VisitStaticStruct::End(res) => res,
                        _ => unreachable!(),
                    }
                }
                StructDef::Dynamic { fields, .. } => {
                    if fields.is_named() {
                        // TODO: size_hint?
                        let mut ser = serializer.serialize_map(None)?;
                        let mut visitor = VisitDynamic::<S>::NamedFields(&mut ser);
                        s.visit(&mut visitor);
                        if let VisitDynamic::Error(e) = visitor {
                            return Err(e);
                        }
                        ser.end()
                    } else {
                        // TODO: size_hint?
                        let mut ser = serializer.serialize_seq(None)?;
                        let mut visitor = VisitDynamic::<S>::UnnamedFields(&mut ser);
                        s.visit(&mut visitor);
                        if let VisitDynamic::Error(e) = visitor {
                            return Err(e);
                        }
                        ser.end()
                    }
                }
                _ => unreachable!(),
            },
            Value::Enumerable(e) => match (e.definition(), e.variant()) {
                (
                    EnumDef::Static {
                        name,
                        variants: def,
                        ..
                    },
                    Variant::Static(variant),
                ) => {
                    let mut visitor = VisitStaticEnum::Start {
                        name,
                        def,
                        variant,
                        serializer,
                    };
                    e.visit(&mut visitor);
                    match visitor {
                        VisitStaticEnum::End(res) => res,
                        _ => unreachable!(),
                    }
                }
                (EnumDef::Dynamic { .. }, variant) => {
                    if variant.is_named_fields() {
                        // TODO: size_hint?
                        let mut ser = serializer.serialize_map(None)?;
                        let mut visitor = VisitDynamic::<S>::NamedFields(&mut ser);
                        e.visit(&mut visitor);
                        if let VisitDynamic::Error(e) = visitor {
                            return Err(e);
                        }
                        ser.end()
                    } else {
                        // TODO: size_hint?
                        let mut ser = serializer.serialize_seq(None)?;
                        let mut visitor = VisitDynamic::<S>::UnnamedFields(&mut ser);
                        e.visit(&mut visitor);
                        if let VisitDynamic::Error(e) = visitor {
                            return Err(e);
                        }
                        ser.end()
                    }
                }
                (EnumDef::Static { .. }, Variant::Dynamic(..)) => {
                    Err(S::Error::custom("dynamic variant in static enum"))
                }
                _ => unreachable!(),
            },
            #[cfg(feature = "std")]
            Value::Path(p) => Serialize::serialize(p, serializer),
            #[cfg(feature = "std")]
            Value::Error(e) => Err(S::Error::custom(e)),

            v => unimplemented!("{:?}", v),
        }
    }
}

enum VisitList<'a, S: Serializer> {
    Serializer(&'a mut S::SerializeSeq),
    Error(S::Error),
}

impl<S: Serializer> Visit for VisitList<'_, S> {
    fn visit_value(&mut self, value: Value<'_>) {
        if let Self::Serializer(ser) = self {
            if let Err(e) = ser.serialize_element(&Serializable(value)) {
                *self = Self::Error(e);
            }
        }
    }
}

enum VisitMap<'a, S: Serializer> {
    Serializer(&'a mut S::SerializeMap),
    Error(S::Error),
}

impl<S: Serializer> Visit for VisitMap<'_, S> {
    fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
        if let Self::Serializer(ser) = self {
            if let Err(e) = ser.serialize_entry(&Serializable(key), &Serializable(value)) {
                *self = Self::Error(e);
            }
        }
    }
}

enum VisitStaticStruct<S: Serializer> {
    Start {
        name: &'static str,
        fields: Fields<'static>,
        serializer: S,
    },
    End(Result<S::Ok, S::Error>),
    Tmp,
}

impl<S: Serializer> Visit for VisitStaticStruct<S> {
    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        let (name, fields, serializer) = match mem::replace(self, Self::Tmp) {
            Self::Start {
                name,
                fields: Fields::Named(fields),
                serializer,
            } => (name, fields, serializer),
            Self::End(..) => {
                *self = Self::End(Err(S::Error::custom(
                    "visit_named_fields called multiple times in static struct",
                )));
                return;
            }
            _ => unreachable!(),
        };
        let mut ser = match serializer.serialize_struct(name, named_values.len()) {
            Ok(ser) => ser,
            Err(e) => {
                *self = Self::End(Err(e));
                return;
            }
        };
        for (i, (_, v)) in named_values.entries().enumerate() {
            if let Err(e) = ser.serialize_field(fields[i].name(), &Serializable(v.as_value())) {
                *self = Self::End(Err(e));
                return;
            }
        }
        *self = Self::End(ser.end());
    }

    fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
        let (name, serializer) = match mem::replace(self, Self::Tmp) {
            Self::Start {
                name,
                fields: Fields::Unnamed,
                serializer,
            } => (name, serializer),
            Self::End(..) => {
                *self = Self::End(Err(S::Error::custom(
                    "visit_unnamed_fields called multiple times in static struct",
                )));
                return;
            }
            _ => unreachable!(),
        };
        if values.len() == 1 {
            *self = Self::End(serializer.serialize_newtype_struct(name, &Serializable(values[0])));
            return;
        }
        let mut ser = match serializer.serialize_tuple_struct(name, values.len()) {
            Ok(ser) => ser,
            Err(e) => {
                *self = Self::End(Err(e));
                return;
            }
        };
        for v in values {
            if let Err(e) = ser.serialize_field(&Serializable(v.as_value())) {
                *self = Self::End(Err(e));
                return;
            }
        }
        *self = Self::End(ser.end());
    }
}

enum VisitStaticEnum<S: Serializer> {
    Start {
        name: &'static str,
        def: &'static [VariantDef<'static>],
        variant: &'static VariantDef<'static>,
        serializer: S,
    },
    End(Result<S::Ok, S::Error>),
    Tmp,
}

impl<S: Serializer> Visit for VisitStaticEnum<S> {
    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        let (name, def, variant, serializer) = match mem::replace(self, Self::Tmp) {
            Self::Start {
                name,
                def,
                variant,
                serializer,
            } => (name, def, variant, serializer),
            Self::End(..) => {
                *self = Self::End(Err(S::Error::custom(
                    "visit_named_fields called multiple times in static enum",
                )));
                return;
            }
            _ => unreachable!(),
        };
        let variant_name = variant.name();
        let variant_index = def.iter().position(|v| v.name() == variant_name).unwrap();
        assert!(variant_index <= u32::MAX as usize);
        let mut ser = match serializer.serialize_struct_variant(
            name,
            variant_index as _,
            variant_name,
            named_values.len(),
        ) {
            Ok(ser) => ser,
            Err(e) => {
                *self = Self::End(Err(e));
                return;
            }
        };
        let fields = match variant.fields() {
            Fields::Named(fields) => fields,
            Fields::Unnamed => unreachable!(),
        };
        for (i, (_, v)) in named_values.entries().enumerate() {
            if let Err(e) = ser.serialize_field(fields[i].name(), &Serializable(v.as_value())) {
                *self = Self::End(Err(e));
                return;
            }
        }
        *self = Self::End(ser.end());
    }

    fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
        let (name, def, variant, serializer) = match mem::replace(self, Self::Tmp) {
            Self::Start {
                name,
                def,
                variant,
                serializer,
            } => (name, def, variant, serializer),
            Self::End(..) => {
                *self = Self::End(Err(S::Error::custom(
                    "visit_unnamed_fields called multiple times in static enum",
                )));
                return;
            }
            _ => unreachable!(),
        };
        let variant_name = variant.name();
        let variant_index = def.iter().position(|v| v.name() == variant_name).unwrap();
        assert!(variant_index <= u32::MAX as usize);
        if values.len() == 1 {
            *self = Self::End(serializer.serialize_newtype_variant(
                name,
                variant_index as _,
                variant_name,
                &Serializable(values[0]),
            ));
            return;
        }
        let mut ser = match serializer.serialize_tuple_variant(
            name,
            variant_index as _,
            variant_name,
            values.len(),
        ) {
            Ok(ser) => ser,
            Err(e) => {
                *self = Self::End(Err(e));
                return;
            }
        };
        for v in values {
            if let Err(e) = ser.serialize_field(&Serializable(v.as_value())) {
                *self = Self::End(Err(e));
                return;
            }
        }
        *self = Self::End(ser.end());
    }
}

// Dynamic struct and variant of dynamic enum will be serialized as map or seq.
enum VisitDynamic<'a, S: Serializer> {
    NamedFields(&'a mut S::SerializeMap),
    UnnamedFields(&'a mut S::SerializeSeq),
    Error(S::Error),
}

impl<S: Serializer> Visit for VisitDynamic<'_, S> {
    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        let ser = match self {
            Self::NamedFields(ser) => ser,
            Self::Error(..) => return,
            _ => unreachable!(),
        };
        for (f, v) in named_values.entries() {
            if let Err(e) = ser.serialize_entry(f.name(), &Serializable(v.as_value())) {
                *self = Self::Error(e);
                return;
            }
        }
    }

    fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
        let ser = match self {
            Self::UnnamedFields(ser) => ser,
            Self::Error(..) => return,
            _ => unreachable!(),
        };
        for v in values {
            if let Err(e) = ser.serialize_element(&Serializable(v.as_value())) {
                *self = Self::Error(e);
                return;
            }
        }
    }
}
