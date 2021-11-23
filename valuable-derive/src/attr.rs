use std::{cell::RefCell, thread};

use proc_macro2::Span;
use syn::{spanned::Spanned, Error, Fields, Ident, Lit, Meta, NestedMeta};

// All #[valuable] attributes.
static ATTRS: &[&AttrDef] = &[&RENAME, &TRANSPARENT, &SKIP];
// #[valuable(rename = "...")]
static RENAME: AttrDef = AttrDef {
    name: "rename",
    conflicts_with: &[],
    position: &[
        Position::Struct,
        Position::Enum,
        Position::Variant,
        Position::NamedField,
    ],
    style: &[MetaStyle::NamedValue],
};
// #[valuable(transparent)]
static TRANSPARENT: AttrDef = AttrDef {
    name: "transparent",
    conflicts_with: &["rename"],
    position: &[Position::Struct],
    style: &[MetaStyle::Ident],
};
// #[valuable(skip)]
static SKIP: AttrDef = AttrDef {
    name: "skip",
    conflicts_with: &["rename"],
    position: &[
        // TODO: How do we implement Enumerable::variant and Valuable::as_value?
        // Position::Variant,
        Position::NamedField,
        Position::UnnamedField,
    ],
    style: &[MetaStyle::Ident],
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Position {
    // TODO: kind: struct, tuple, or unit
    Struct,
    Enum,
    // TODO: kind of variants: struct, tuple, or unit
    Variant,
    NamedField,
    UnnamedField,
}

impl Position {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Struct => "struct",
            Self::Enum => "enum",
            Self::Variant => "variant",
            Self::NamedField => "named field",
            Self::UnnamedField => "unnamed field",
        }
    }

    fn is_field(self) -> bool {
        matches!(self, Self::NamedField | Self::UnnamedField)
    }
}

impl From<&Fields> for Position {
    fn from(meta: &Fields) -> Self {
        match meta {
            Fields::Named(..) => Self::NamedField,
            Fields::Unnamed(..) | Fields::Unit => Self::UnnamedField,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MetaStyle {
    // #[valuable(<name>)]
    Ident,
    // #[valuable(<name> = ...)]
    NamedValue,
    // #[valuable(<name>(...))]
    List,
}

impl MetaStyle {
    fn format(self, name: &str) -> String {
        match self {
            MetaStyle::Ident => name.to_string(),
            MetaStyle::List => format!("{}(...)", name),
            MetaStyle::NamedValue => format!("{} = \"...\"", name),
        }
    }
}

impl From<&Meta> for MetaStyle {
    fn from(meta: &Meta) -> Self {
        match meta {
            Meta::Path(..) => Self::Ident,
            Meta::List(..) => Self::List,
            Meta::NameValue(..) => Self::NamedValue,
        }
    }
}

#[derive(Debug)]
struct AttrDef {
    name: &'static str,
    conflicts_with: &'static [&'static str],
    // allowed positions.
    position: &'static [Position],
    // allowed styles.
    style: &'static [MetaStyle],
}

impl AttrDef {
    /// Performs checks that can be performed without parsing other attributes,
    /// and returns `true` if at least one error occurs.
    fn early_check(&self, cx: &Context, pos: Position, meta: &Meta) -> bool {
        let mut has_error = false;
        if let Err(msg) = self.check_position(pos) {
            cx.error(Error::new_spanned(meta, msg));
            has_error = true;
        }
        if let Err(msg) = self.check_style(meta) {
            cx.error(Error::new_spanned(meta, msg));
            has_error = true;
        }
        has_error
    }

    fn check_position(&self, pos: Position) -> Result<(), String> {
        if self.position.contains(&pos) {
            return Ok(());
        }
        let mut msg = format!("#[valuable({})] may only be used on ", self.name);
        // TODO: simplify if possible
        // len == 1: a
        // len == 2: a and b
        // len > 2: a, b, and c
        let position = if self.position.contains(&Position::NamedField)
            && self.position.contains(&Position::UnnamedField)
        {
            let mut position: Vec<_> = self
                .position
                .iter()
                .filter(|p| !p.is_field())
                .map(Position::as_str)
                .collect();
            position.push("field");
            position
        } else {
            self.position.iter().map(Position::as_str).collect()
        };
        let len = position.len();
        for (i, p) in position.iter().enumerate() {
            if i != 0 {
                if len == 2 {
                    msg.push_str(" and ");
                } else {
                    msg.push_str(", ");
                    if i == len - 1 {
                        msg.push_str("and ");
                    }
                }
            }
            msg.push_str(p);
            msg.push('s');
        }
        Err(msg)
    }

    fn check_style(&self, meta: &Meta) -> Result<(), String> {
        let meta = MetaStyle::from(meta);
        if self.style.contains(&meta) {
            return Ok(());
        }
        let mut msg = "expected ".to_string();
        let mut first = true;
        for style in self.style {
            if first {
                first = false;
            } else {
                msg.push_str(" or ");
            }
            msg.push_str(&format!("`#[valuable({})]`", style.format(self.name)));
        }
        msg.push_str(", found ");
        msg.push_str(&format!("`#[valuable({})]`", meta.format(self.name)));
        Err(msg)
    }

    /// Performs checks that can be performed after parsing all attributes in
    /// the same scope and parent scopes, and returns `true` if at least one
    /// error occurs.
    fn late_check(&self, cx: &Context, meta: &Meta) -> bool {
        let mut has_error = false;
        if let Err(msg) = self.check_conflicts(cx, meta) {
            cx.error(Error::new_spanned(meta.path(), msg));
            has_error = true;
        }
        has_error
    }

    fn check_conflicts(&self, cx: &Context, meta: &Meta) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct Context {
    // - `None`: during checking.
    // - `Some(None)`: there are no errors.
    // - `Some(Some)`: there are errors.
    error: RefCell<Option<Option<Error>>>,
}

impl Context {
    pub(crate) fn error(&self, e: Error) {
        match self.error.borrow_mut().as_mut().unwrap() {
            Some(base) => base.combine(e),
            error @ None => *error = Some(e),
        }
    }

    pub(crate) fn check(self) -> Result<(), Error> {
        match self.error.borrow_mut().take().unwrap() {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            error: RefCell::new(Some(None)),
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if !thread::panicking() && self.error.borrow().is_some() {
            panic!("context need to be checked");
        }
    }
}

pub(crate) struct Attrs {
    rename: Option<(syn::MetaNameValue, syn::LitStr)>,
    transparent: Option<Span>,
    skip: Option<Span>,
}

impl Attrs {
    pub(crate) fn rename(&self, original: &Ident) -> syn::LitStr {
        self.rename.as_ref().map_or_else(
            || syn::LitStr::new(&original.to_string(), original.span()),
            |(_, l)| l.clone(),
        )
    }

    pub(crate) fn transparent(&self) -> bool {
        self.transparent.is_some()
    }

    pub(crate) fn skip(&self) -> bool {
        self.skip.is_some()
    }
}

pub(crate) fn parse_attrs(cx: &Context, attrs: &[syn::Attribute], pos: Position) -> Attrs {
    let mut rename = None;
    let mut transparent = None;
    let mut skip = None;
    for (def, meta) in filter_valuable_attrs(cx, attrs) {
        if def.early_check(cx, pos, &meta) {
            continue;
        }
        match def.name {
            // #[valuable(rename = "...")]
            "rename" => {
                if rename.is_some() {
                    cx.error(Error::new_spanned(
                        meta,
                        "duplicate #[valuable(rename)] attribute",
                    ));
                    continue;
                }
                let m = match meta {
                    Meta::NameValue(m) => m,
                    _ => unreachable!(),
                };
                let lit = match &m.lit {
                    Lit::Str(l) => l.clone(),
                    l => {
                        cx.error(Error::new_spanned(l, "expected string literal"));
                        continue;
                    }
                };
                rename = Some((m, lit));
            }

            // #[valuable(transparent)]
            "transparent" => {
                if transparent.replace(meta.span()).is_some() {
                    cx.error(Error::new_spanned(
                        meta,
                        "duplicate #[valuable(transparent)] attribute",
                    ));
                    continue;
                }
            }

            // #[valuable(skip)]
            "skip" => {
                if skip.replace(meta.span()).is_some() {
                    cx.error(Error::new_spanned(
                        meta,
                        "duplicate #[valuable(skip)] attribute",
                    ));
                    continue;
                }
            }

            _ => unreachable!("{}", def.name),
        }
    }

    Attrs {
        rename,
        transparent,
        skip,
    }
}

fn filter_valuable_attrs<'a>(
    cx: &'a Context,
    attrs: &'a [syn::Attribute],
) -> impl Iterator<Item = (&'static AttrDef, Meta)> + 'a {
    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("valuable"))
        .filter_map(move |attr| match attr.parse_meta() {
            Ok(Meta::List(l)) => Some(l.nested),
            Ok(m) => {
                cx.error(Error::new_spanned(m, "expected `#[valuable(...)]`"));
                None
            }
            Err(e) => {
                cx.error(e);
                None
            }
        })
        .flatten()
        .filter_map(move |m| match m {
            NestedMeta::Lit(l) => {
                cx.error(Error::new_spanned(l, "expected identifier, found literal"));
                None
            }
            NestedMeta::Meta(m) => match m.path().get_ident() {
                Some(p) => match ATTRS.iter().position(|a| p == a.name) {
                    Some(pos) => Some((ATTRS[pos], m)),
                    None => {
                        cx.error(Error::new_spanned(
                            p,
                            format!("unknown valuable attribute `{}`", p),
                        ));
                        None
                    }
                },
                None => {
                    cx.error(Error::new_spanned(m, "expected identifier, found path"));
                    None
                }
            },
        })
}
