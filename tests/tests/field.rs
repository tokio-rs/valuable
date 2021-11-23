use valuable::NamedField;

fn assert_clone<T: Clone>() {}
fn assert_copy<T: Copy>() {}

#[test]
fn is_clone_copy() {
    assert_clone::<NamedField<'static>>();
    assert_copy::<NamedField<'static>>();
}
