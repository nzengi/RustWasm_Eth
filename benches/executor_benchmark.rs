use criterion::{criterion_group, criterion_main, Criterion};
use rustwasm_eth_backend::Executor;
use rustwasm_eth_sandbox::Sandbox;

/// Benchmarks the performance of the WASM execution using the Executor.
/// This measures how efficiently the Executor can handle a basic WASM bytecode.
fn benchmark_executor(c: &mut Criterion) {
    // Simulated WASM bytecode for benchmarking
    let wasm_code: &[u8] = &[0x00, 0x61, 0x62, 0x63];

    // Create an Executor instance with a defined memory size
    let mut executor = Executor::new(1024);

    // Criterion benchmark for the Executor's WASM execution
    c.bench_function("Executor WASM Execution", |b| {
        b.iter(|| {
            // Execute the WASM code and ensure no errors occur during execution
            executor.execute(wasm_code).expect("Execution failed");
        });
    });
}

/// Benchmarks the performance of the WASM execution in the secure Sandbox environment.
/// This helps to analyze how well the sandbox handles code execution with security measures.
fn benchmark_sandbox(c: &mut Criterion) {
    // Simulated WASM bytecode for benchmarking
    let wasm_code: &[u8] = &[0x00, 0x61, 0x62, 0x63];

    // Create a Sandbox instance with a defined memory size, gas limit, and security level
    let mut sandbox = Sandbox::new(1024, 10000, 1);

    // Criterion benchmark for the Sandbox's WASM execution
    c.bench_function("Sandbox WASM Execution", |b| {
        b.iter(|| {
            // Execute the WASM code inside the secure sandbox
            sandbox.execute(wasm_code).expect("Sandbox execution failed");
        });
    });
}

// Criterion groups to organize the benchmarks for Executor and Sandbox
criterion_group!(benches, benchmark_executor, benchmark_sandbox);

// Define the main entry point for Criterion to run the benchmarks
criterion_main!(benches);
