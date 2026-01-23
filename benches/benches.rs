use criterion::{criterion_main, criterion_group, Criterion};
use std::hint::black_box;

use easypls::expr::Expr;

pub fn small_sat() {
    let prop = "not (((a or (b and c) <-> d) xor a or (b and c) <-> d) nand e) or not (not f)";
    let expr = Expr::parse(prop.as_bytes()).unwrap();
    let cnf = expr.tseitin(false);
    let _ = cnf.find_evidence();
}

pub fn small_sat_bench(c: &mut Criterion) {
    c.bench_function("small_sat", |b| b.iter(|| black_box(small_sat())));
}

criterion_group!(benches, small_sat_bench);
criterion_main!(benches);
