use criterion::{criterion_group, criterion_main, Criterion};
use std::fmt::Write;

fn dedent() -> String {
    let mut s = String::new();
    let mut f = code_formatter::Formatter::new(&mut s, "    ");
    write!(
        f,
        r#"
        struct Foo;

        impl Foo {{
            fn foo() {{
                todo!()
            }}
        }}
        "#,
    )
    .unwrap();
    assert_eq!(
        s,
        "struct Foo;\n\nimpl Foo {\n    fn foo() {\n        todo!()\n    }\n}\n"
    );
    s
}

fn dedent_benchmark(c: &mut Criterion) {
    c.bench_function("dedent", |b| b.iter(|| dedent()));
}

criterion_group!(benches, dedent_benchmark);
criterion_main!(benches);
