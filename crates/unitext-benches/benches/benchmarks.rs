use criterion::{black_box, criterion_group, criterion_main, Criterion};
use unitext_string::UniString;
use unitext_security::visually_equal;

fn bench_uni_string_creation(c: &mut Criterion) {
    let text = "Hello 👨‍👩‍👧‍👦 Café";
    c.bench_function("UniString::new", |b| {
        b.iter(|| {
            let us = UniString::new(black_box(text));
            black_box(us.length());
        })
    });
}

fn bench_security_checks(c: &mut Criterion) {
    let safe_text = "apple.com";
    let unsafe_text = "аpple.com"; // Cyrillic 'a'

    c.bench_function("is_safe (clean)", |b| {
        b.iter(|| {
            let us = UniString::new(black_box(safe_text));
            black_box(us.is_safe());
        })
    });

    c.bench_function("is_safe (unsafe)", |b| {
        b.iter(|| {
            let us = UniString::new(black_box(unsafe_text));
            black_box(us.is_safe());
        })
    });
}

fn bench_visually_equal(c: &mut Criterion) {
    let safe_text = "apple.com";
    let unsafe_text = "аpple.com"; // Cyrillic 'a'

    c.bench_function("visually_equal", |b| {
        b.iter(|| {
            black_box(visually_equal(black_box(safe_text), black_box(unsafe_text)));
        })
    });
}

fn bench_to_ascii(c: &mut Criterion) {
    let text = "Héllo Café 👨‍👩‍👧‍👦";

    c.bench_function("to_ascii", |b| {
        b.iter(|| {
            let us = UniString::new(black_box(text));
            black_box(us.to_ascii());
        })
    });
}

criterion_group!(
    benches,
    bench_uni_string_creation,
    bench_security_checks,
    bench_visually_equal,
    bench_to_ascii
);
criterion_main!(benches);
