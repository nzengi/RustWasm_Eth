use rustwasm_eth_backend::Executor;
use rustwasm_eth_sandbox::Sandbox;

fn main() {
    let wasm_code: &[u8] = &[0, 1, 2, 3];

    let mut executor = Executor::new(1024);
    let mut sandbox = Sandbox::new(1024, 10000, 1);

    executor.execute(wasm_code).expect("Execution failed");
    sandbox.execute(wasm_code).expect("Sandbox execution failed");

    println!("WASM code executed successfully in both backend and sandbox.");
}
