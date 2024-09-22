use valuable::{Renderable, Valuable};

#[derive(Debug)]
struct NotTuplable<'a> {
    s: &'a str,
    i: usize,
}

impl<'a> core::fmt::Display for NotTuplable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The string is \"{}\", and the integer is {}.",
            self.s, self.i
        )
    }
}

#[test]
fn test_renderable_struct_from_debug() {
    let s_owned = "Hello, Valuable World".to_owned();
    let s = NotTuplable { s: &s_owned, i: 42 };

    let r = Renderable::Debug(&s);
    let v = r.as_value();

    // Rendering should produce the debug output for the struct
    assert_eq!(r.render_to_string(), format!("{s:?}"));

    // Writing the value itself as `Debug` should print the same debug output
    // as the struct
    assert_eq!(format!("{v:?}"), format!("{s:?}"));
    assert_eq!(format!("{v:#?}"), format!("{s:#?}"));
}

#[test]
fn test_renderable_struct_from_display() {
    let s_owned = "Hello, Valuable World".to_owned();
    let s = NotTuplable { s: &s_owned, i: 42 };

    let r = Renderable::Display(&s);
    let v = r.as_value();

    // Rendering should produce the display output for the struct
    assert_eq!(r.render_to_string(), format!("{s}"));

    // Just to make sure, the display output should be different for the debug
    // output
    assert_ne!(r.render_to_string(), format!("{s:?}"));

    // Writing the value itself as `Debug` should print the same display output
    // as the struct
    assert_eq!(format!("{v:?}"), format!("{s}"));
    assert_eq!(format!("{v:#?}"), format!("{s:#}"));
}
