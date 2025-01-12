use criterion::{criterion_group, criterion_main, Criterion};
use htmf::declare::*;

fn run(c: &mut Criterion) {
    let mut group = c.benchmark_group("run");
    group.bench_function("basic", |b| {
        b.iter(|| {
            html(class("w-full h-full"))
                .with([
                    head([]).with([
                        link(rel("stylesheet").href("/assets/preflight.css")),
                        link(rel("stylesheet").href("/assets/railwind.css")),
                        script(src("/assets/htmx.1.9.9.js")),
                        meta(name("color-scheme").content("dark")),
                        meta(name("viewport").content("width=device-width,initial-scale=1")),
                    ]),
                    body(class("w-full h-full text-gray-200 bg-neutral-800")),
                ])
                .to_html();
        });
    });

    group.bench_function("wide", |b| {
        b.iter(|| {
            let children = (0..10_000).map(|_| div([])).collect::<Vec<_>>();
            html([]).with(body([]).with(children)).to_html();
        });
    });

    group.bench_function("deep", |b| {
        b.iter(|| {
            let mut elem = div([]);
            for _ in 0..10_000 {
                elem = div([]).with(elem);
            }
            html([]).with(body([]).with(elem)).to_html();
        });
    });

    #[cfg(feature = "unstable-builder")]
    group.bench_function("builder", |b| {
        b.iter(|| {
            html(class("w-full h-full"))
                .with([head([]).with([
                    link(rel("stylesheet").href("/assets/preflight.css")),
                    link(rel("stylesheet").href("/assets/railwind.css")),
                    script(src("/assets/htmx.1.9.9.js")),
                    meta(name("color-scheme").content("dark")),
                    meta(name("viewport").content("width=device-width,initial-scale=1")),
                ])])
                .body(class("w-full h-full text-gray-200 bg-neutral-800"))
                .to_html();
        });
    });
    group.finish();
}

criterion_group!(benches, run);
criterion_main!(benches);
