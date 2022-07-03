use dankgine::core::engine::{add_body, update};
use criterion::{black_box, Criterion, criterion_main, criterion_group};


pub fn update_many(times: usize) {
    for _ in 0..times {
        black_box(
            add_body(
                black_box(300.0), 
                black_box(50.0), 
                black_box(5.0)
            )
        );
        black_box(update());
    }
}


pub fn bench_updates(c: &mut Criterion) {
    c.bench_function(
        "add body 10",
        |b| b.iter(|| update_many(black_box(10)))
    );
}

criterion_group!(benches, bench_updates);
criterion_main!(benches);
