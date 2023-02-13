use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;
use template_benchmarks_rs::{
    askama_bench, fomat, handlebars, horrorshow_bench, liquid, markup_bench, maud_bench, minijinja,
    mustache, ramhorns, ructe, sailfish, std_write, tera,
};

// The measurement_time increase from the default of 5s to 6s allows some more leeway
// for the runtime templates.
const MEASUREMENT_TIME: Duration = Duration::from_secs(6);

macro_rules! compiled_templates {
    ($c:expr, name = $name:literal, size = $size:expr, bench = $fun:ident) => {
        let mut group = $c.benchmark_group(concat!("Compiled - ", $name));
        group.measurement_time(MEASUREMENT_TIME);

        group.bench_with_input("Askama", &$size, |b, i| askama_bench::$fun(b, i));
        group.bench_with_input("fomat", &$size, |b, i| fomat::$fun(b, i));
        group.bench_with_input("Horrorshow", &$size, |b, i| horrorshow_bench::$fun(b, i));
        group.bench_with_input("Markup", &$size, |b, i| markup_bench::$fun(b, i));
        group.bench_with_input("Maud", &$size, |b, i| maud_bench::$fun(b, i));
        group.bench_with_input("Ructe", &$size, |b, i| ructe::$fun(b, i));
        group.bench_with_input("Sailfish", &$size, |b, i| sailfish::$fun(b, i));
        group.bench_with_input("write", &$size, |b, i| std_write::$fun(b, i));

        group.finish();
    };
}

macro_rules! runtime_templates {
    ($c:expr, name = $name:literal, size = $size:expr, bench = $fun:ident) => {
        let mut group = $c.benchmark_group(concat!("Runtime - ", $name));
        group.measurement_time(MEASUREMENT_TIME);

        group.bench_with_input("Handlebars", &$size, |b, i| handlebars::$fun(b, i));
        group.bench_with_input("Liquid", &$size, |b, i| liquid::$fun(b, i));
        group.bench_with_input("Minijinja", &$size, |b, i| minijinja::$fun(b, i));
        group.bench_with_input("Mustache", &$size, |b, i| mustache::$fun(b, i));
        group.bench_with_input("Ramhorns", &$size, |b, i| ramhorns::$fun(b, i));
        group.bench_with_input("Tera", &$size, |b, i| tera::$fun(b, i));

        group.finish();
    };
}

const BIG_TABLE_SIZE: usize = 100;
const TEAMS_SIZE: usize = 0;

fn compiled_big_table(c: &mut Criterion) {
    compiled_templates! {
       c,
       name = "Big Table",
       size = BIG_TABLE_SIZE,
       bench = big_table
    }
}

fn compiled_teams(c: &mut Criterion) {
    compiled_templates! {
        c,
        name = "Teams",
        size = TEAMS_SIZE,
        bench = teams
    }
}

fn runtime_big_table(c: &mut Criterion) {
    runtime_templates! {
        c,
        name = "Big Table",
        size = BIG_TABLE_SIZE,
        bench = big_table
    }
}

fn runtime_teams(c: &mut Criterion) {
    runtime_templates! {
        c,
        name = "Teams",
        size = TEAMS_SIZE,
        bench = teams
    }
}

criterion_group!(
    benches,
    compiled_big_table,
    compiled_teams,
    runtime_big_table,
    runtime_teams
);

criterion_main!(benches);
