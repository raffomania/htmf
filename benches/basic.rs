use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use htmb::{self as h, Element};

fn run(c: &mut Criterion) {
    let mut group = c.benchmark_group("run");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.bench_function("run", |b| {
        b.iter(|| {
            h::a(
                [h::href("https://www.rafa.ee")],
                [Element::Text("My Site".into())],
            )
            .to_html();
        });
    });
    group.finish();
}

criterion_group!(benches, run);
criterion_main!(benches);
