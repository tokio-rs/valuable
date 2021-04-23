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
    StaticField::new(0, "one"),
    StaticField::new(1, "two"),
    StaticField::new(2, "three"),
    StaticField::new(3, "four"),
    StaticField::new(4, "five"),
    StaticField::new(5, "six"),
];

impl Structable for HelloWorld {
    fn definition(&self) -> Definition<'_> {
        Definition {
            name: "HelloWorld",
            static_fields: FIELDS,
            is_dynamic: false,
        }
    }

    fn visit(&self, v: &mut dyn Visit) {
        let definition = self.definition();
        v.visit_struct(&Record::new(
            &definition,
            &[
                Value::Usize(self.one),
                Value::Usize(self.two),
                Value::Usize(self.three),
                Value::Usize(self.four),
                Value::Usize(self.five),
                Value::Usize(self.six),
            ],
        ));
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    const NUM: usize = 50;

    let hello_world = black_box(HelloWorld::default());
    let structable = &hello_world as &dyn Structable;
    let definition = structable.definition();
    let f = &structable.definition().static_fields()[5];

    struct Sum(usize, &'static StaticField);

    impl Visit for Sum {
        fn visit_struct(&mut self, record: &Record<'_>) {
            self.0 += match record.get_static_unchecked(self.1) {
                Value::Usize(v) => v,
                _ => return,
            }
        }
    }

    c.bench_function("struct", |b| {
        b.iter(|| {
            let mut num = 0;
            for _ in 0..NUM {
                let hello_world = black_box(HelloWorld::default());
                num += hello_world.six;
            }

            black_box(num);
        })
    });

    c.bench_function("valuable", |b| {
        b.iter(|| {
            let mut v = Sum(black_box(0), f);

            for _ in 0..NUM {
                v.visit_struct(&Record::new(
                    &definition,
                    &[
                        Value::Usize(0),
                        Value::Usize(0),
                        Value::Usize(0),
                        Value::Usize(0),
                        Value::Usize(0),
                        Value::Usize(0),
                    ],
                ));
                /*
                v.visit_struct(&Record::new(
                    &definition,
                    &[
                        Value::Usize(hello_world.one),
                        Value::Usize(hello_world.two),
                        Value::Usize(hello_world.three),
                        Value::Usize(hello_world.four),
                        Value::Usize(hello_world.five),
                        Value::Usize(hello_world.six),
                    ]
                ));
                */
                // hello_world.visit(&mut v);
            }

            black_box(v.0);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
