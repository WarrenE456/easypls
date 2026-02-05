use criterion::{criterion_main, criterion_group, Criterion};
use std::hint::black_box;

use easypls::expr::Expr;

pub fn small_sat() {
    let prop = "not (((a or (b and c) <-> d) xor a or (b and c) <-> d) nand e) or not (not f)";

    let expr = Expr::parse(prop.as_bytes()).unwrap();
    let mut cnf = expr.tseitin(false);
    let _ = cnf.find_evidence();
}

pub fn large_sat() {
    let prop = "not (
  (
    (
      (a or (b and c) <-> d)
      xor
      ((e and f) or g <-> h)
    )
    nand
    (
      (i or j)
      xor
      (k and (l or m) <-> n)
    )
  )
  or
  (
    (
      (o and p)
      or
      (q xor r <-> s)
    )
    nand
    (
      not (t)
      or
      (u and (v or w) <-> x)
    )
  )
)
or
not (
  (
    (y or z)
    and
    (
      (a xor e)
      or
      (i <-> o)
    )
  )
  nand
  (
    (
      (b and f)
      xor
      (j or p <-> t)
    )
    or
    not (
      (c or g)
      and
      (k <-> q)
    )
  )
)
or
(
  (
    not (d)
    xor
    (h and l)
  )
  nand
  (
    (m or r)
    <-> 
    (s and w)
  )
)
or
not (not f)
 
";

    let mut cnf = Expr::parse(prop.as_bytes()).unwrap().tseitin(false);
    let _ = cnf.find_evidence();
}

pub fn small_sat_bench(c: &mut Criterion) {
    c.bench_function("small_sat", |b| b.iter(|| black_box(small_sat())));
}

pub fn large_sat_bench(c: &mut Criterion) {
    c.bench_function("large_sat", |b| b.iter(|| black_box(large_sat())));
}


criterion_group!(benches, small_sat_bench, large_sat_bench);
criterion_main!(benches);
