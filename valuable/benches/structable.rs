use valuable::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Default)]
struct HelloWorld {
    one: usize,
    two: usize,
    three: usize,
    four: usize,
    five: usize,
    six: usize,
}

static FIELDS: &[StaticField] = &[
    StaticField::new("one"),
    StaticField::new("two"),
    StaticField::new("three"),
    StaticField::new("four"),
    StaticField::new("five"),
    StaticField::new("six"),
];

type Iter<'a> = &'a mut dyn Iterator<Item = (Field<'a>, Value<'a>)>;

impl Structable for HelloWorld {
    fn definition(&self) -> Definition<'_> {
        Definition {
            name: "HelloWorld",
            static_fields: FIELDS,
            is_dynamic: false,
        }
    }

    fn get(&self, field: &Field<'_>) -> Option<Value<'_>> {
        match field {
            Field::Static(field) => {
                if *field == &FIELDS[0] {
                    Some(Value::Usize(self.one))
                } else if *field == &FIELDS[1] {
                    Some(Value::Usize(self.two))
                } else if *field == &FIELDS[2] {
                    Some(Value::Usize(self.three))
                } else if *field == &FIELDS[3] {
                    Some(Value::Usize(self.four))
                } else if *field == &FIELDS[4] {
                    Some(Value::Usize(self.five))
                } else if *field == &&FIELDS[5] {
                    Some(Value::Usize(self.six))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn with_iter_fn_mut(&self, f: &mut dyn FnMut(Iter<'_>)) {
        todo!()
    }
}

trait Visitor {
    fn visit(&mut self, field: usize, val: usize);
}

impl HelloWorld {
    fn visit(&self, v: &mut dyn Visitor) {
        v.visit(0, self.one);
        v.visit(1, self.two);
        v.visit(2, self.three);
        v.visit(3, self.four);
        v.visit(4, self.five);
        v.visit(5, self.six);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    const NUM: usize = 50;

    let hello_world = HelloWorld::default();
    let structable = &hello_world as &dyn Structable;
    let f = &structable.definition().static_fields()[5];

    struct Sum(usize);

    impl Visitor for Sum {
        fn visit(&mut self, field: usize, val: usize) {
            if field == 5 {
                self.0 += val;
            }
        }
    }

    c.bench_function("struct", |b| {
        b.iter(|| {
            let mut num = 0;

            for _ in 0..NUM {
                num += black_box(hello_world.six);
            }

            black_box(num);
        })
    });

    c.bench_function("structable", |b| {
        b.iter(|| {
            let mut num = 0;

            let f = black_box(Field::Static(f));

            for _ in 0..NUM {
                match structable.get(&f) {
                    Some(Value::Usize(n)) => num += n,
                    _ => panic!(),
                }
            }

            black_box(num);
        })
    });

    c.bench_function("visit", |b| {
        b.iter(|| {
            let mut num = 0;

            let mut v = Sum(black_box(0));

            for _ in 0..NUM {
                hello_world.visit(&mut v);
            }

            black_box(num);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);