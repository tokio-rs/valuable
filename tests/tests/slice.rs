use valuable::*;

#[test]
fn test_iter() {
    let slice = Slice::I32(&[1, 2, 3]);

    let iter = slice.iter();
    assert_eq!(iter.len(), 3);
    let v: Vec<_> = iter.map(|v| v.as_i32().unwrap()).collect();
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
fn test_iter_rev() {
    let slice = Slice::I32(&[1, 2, 3]);

    let iter = slice.iter().rev();
    assert_eq!(iter.len(), 3);
    let v: Vec<_> = iter.map(|v| v.as_i32().unwrap()).collect();
    assert_eq!(v, vec![3, 2, 1]);
}
