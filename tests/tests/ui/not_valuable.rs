use valuable::Valuable;

struct S;

#[derive(Valuable)]
struct Struct {
    f: Option<S>,
}

#[derive(Valuable)]
struct Tuple(Option<S>);

#[derive(Valuable)]
enum Enum {
    Struct { f: Option<S> },
    Tuple(Option<S>),
}

fn main() {}
