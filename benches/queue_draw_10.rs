#![recursion_limit = "256"]

mod queue_draw_helpers;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use queue_draw_helpers::*;

fn run(c: &mut Criterion) {
    let (n, screen_as_st_queue) = st_queue_screen::new_screen_10();
    let screen_as_trait_objects = trait_objects_screen::new_screen_10();
    let screen_as_enums = enum_screen::new_screen_10();

    assert_eq!(screen_as_trait_objects.len(), n);
    assert_eq!(screen_as_enums.len(), n);
    assert_eq!(n, 10);

    let mut group = c.benchmark_group("queue_draw_10");

    group.bench_with_input(BenchmarkId::new("st_queue_screen", n), &n, |b, _| {
        use st_queue_components::Draw;
        b.iter(|| screen_as_st_queue.draw())
    });

    group.bench_with_input(BenchmarkId::new("trait_objects_screen", n), &n, |b, _| {
        b.iter(|| {
            for component in &screen_as_trait_objects {
                component.draw();
            }
        })
    });

    group.bench_with_input(BenchmarkId::new("enum_screen", n), &n, |b, _| {
        b.iter(|| {
            use enum_components::Draw;
            for component in &screen_as_enums {
                component.draw();
            }
        })
    });

    group.finish();
}

criterion_group!(benches, run);
criterion_main!(benches);
