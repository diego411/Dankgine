extern crate test;

use crate::core::engine::{add_body, update};

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_five_updates(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..5 {
                black_box(add_body(300.0, 50.0, 5.0));
                black_box(update());
            }
        });
    }
}
