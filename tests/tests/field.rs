use valuable::NamedField;

fn assert_clone<T: Clone>() {}

#[test]
fn is_clone_copy() {
    assert_clone::<NamedField<'static>>();
}
