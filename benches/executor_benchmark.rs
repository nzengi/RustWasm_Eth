use criterion::{criterion_group, criterion_main, Criterion};
use rustwasm_eth_backend::Executor;
use rustwasm_eth_sandbox::Sandbox;

fn benchmark_executor(c: &mut Criterion) {
    let wasm_code: &[u8] = &[0x00, 0x61, 0x62, 0x63];
    let mut executor = Executor::new(1024);
    c.bench_function("Executor WASM Execution", |b| {
        b.iter(|| {
            executor.execute(wasm_code).expect("Execution failed");
        });
    });
}

fn benchmark_sandbox(c: &mut Criterion) {
    let wasm_code: &[u8] = &[0x00, 0x61, 0x62, 0x63];
    let mut sandbox = Sandbox::new(1024, 10000, 1);
    c.bench_function("Sandbox WASM Execution", |b| {
        b.iter(|| {
            sandbox.execute(wasm_code).expect("Sandbox execution failed");
        });
    });
}

criterion_group!(benches, benchmark_executor, benchmark_sandbox);
criterion_main!(benches);
