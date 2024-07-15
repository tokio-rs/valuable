use valuable::*;

#[derive(Valuable)]
struct Rename1 {
    #[valuable(rename = b)]
    f: (),
}

#[derive(Valuable)]
struct Transparent1 {
    #[valuable(transparent)]
    f: (),
}
#[derive(Valuable)]
#[valuable(transparent)]
enum Transparent2 {
    V(()),
}
#[derive(Valuable)]
#[valuable(transparent)]
struct Transparent3 {
    f1: (),
    f2: (),
}
#[derive(Valuable)]
#[valuable(transparent, rename = "a")]
struct Transparent4 {
    f: (),
}

#[derive(Valuable)]
#[valuable(skip)]
struct Skip1 {
    f: (),
}
#[derive(Valuable)]
#[valuable(skip)]
enum Skip2 {
    #[valuable(skip)]
    V(()),
}
#[derive(Valuable)]
struct Skip3 {
    #[valuable(skip, rename = "a")]
    f: (),
}

fn main() {}
