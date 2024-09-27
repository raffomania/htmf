use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use htmf::{declare::*, Element};

fn run(c: &mut Criterion) {
    let mut group = c.benchmark_group("run");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.bench_function("basic", |b| {
        b.iter(|| {
            document()
                .with([html().class("w-full h-full").with([
                    head().with([
                        link().rel("stylesheet").href("/assets/preflight.css"),
                        link().rel("stylesheet").href("/assets/railwind.css"),
                        script().src("/assets/htmx.1.9.9.js"),
                        meta().name("color-scheme").content("dark"),
                        meta()
                            .name("viewport")
                            .content("width=device-width,initial-scale=1"),
                    ]),
                    body().class("w-full h-full text-gray-200 bg-neutral-800"),
                ])])
                .to_html();
        });
    });
    group.bench_function("builder", |b| {
        b.iter(|| {
            let doc = document()
                .nest()
                .html()
                .class("w-full h-full")
                .with([head().with([
                    link().rel("stylesheet").href("/assets/preflight.css"),
                    link().rel("stylesheet").href("/assets/railwind.css"),
                    script().src("/assets/htmx.1.9.9.js"),
                    meta().name("color-scheme").content("dark"),
                    meta()
                        .name("viewport")
                        .content("width=device-width,initial-scale=1"),
                ])])
                .body()
                .class("w-full h-full text-gray-200 bg-neutral-800");
            Element::from(doc).to_html();
        });
    });
    group.finish();
}

criterion_group!(benches, run);
criterion_main!(benches);
