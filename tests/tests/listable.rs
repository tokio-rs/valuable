use tests::*;
use valuable::*;

struct VisitHello(u32);

impl Visit for VisitHello {
    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        let id = &HELLO_WORLD_FIELDS[0];
        assert_eq!("id", id.name());
        assert_eq!(Some(self.0), named_values.get(id).unwrap().as_u32());
    }
}

struct VisitList(u32);

impl Visit for VisitList {
    fn visit_slice(&mut self, slice: Slice<'_>) {
        match slice {
            Slice::Value(slice) => {
                for value in slice {
                    match *value {
                        Value::Structable(value) => {
                            unimplemented!()
                        }
                        _ => panic!(),
                    }
                }
            }
            _ => panic!(),
        }
    }
}

#[test]
#[ignore]
fn test_default_visit_slice_empty() {
    let empty: Vec<HelloWorld> = vec![];
    unimplemented!()
}

#[test]
#[ignore]
fn test_default_visit_slice_small() {
    unimplemented!()
}

#[test]
#[ignore]
fn test_default_visit_slice_big() {
    unimplemented!()
}

#[test]
#[ignore]
fn test_visit_slice_primitive() {
    unimplemented!()
}
