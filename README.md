# Practicing WASM

This repository is for practice only!

Custom Node-RED node using WASM and LDK.

## Structure

```bash
├── decode-invoice          // Node-RED custom node project
│   ├── invoiceAmount.html
│   ├── invoiceAmount.js
│   ├── package.json
│   └── package-lock.json
├── src                     // Rust source code
│   └── lib.rs
├── target                  // Compiled WASM first goes here
├── Cargo.toml              // Rust project config file
├── ldkwasm_test.js         // Javascript bindings for WASM
├── ldkwasm_test_bg.wasm    // WASM file that goes with bindings
└── README.md
```

## Steps to reproduce

Add the wasm target to rustup:

```
$ rustup target add wasm32-unknown-unknown
```

Specify release target as WASM:

```
$ cargo build --release --target wasm32-unknown-unknown
```

Which creates a WASM binary at `target/wasm32-unknown-unknown/release/ldkwasm_test.wasm`.

Get `wasm-bindgen-cli` if you haven't already:

```
$ cargo install wasm-bindgen-cli
```

Next we must generate the JS bindings for this WASM, as follows:

```
$ wasm-bindgen target/wasm32-unknown-unknown/release/ldkwasm_test.wasm --target nodejs --out-dir . --no-typescript
```

The resulting files in project root `ldkwasm_test_bg.wasm` and `ldkwasm_test.js` are respectively the final WASM and bindings.

## Using in Node-RED

The decode-invoice directory contains a basic custom Node-RED node.

It should be simple enough to read without documentation.

Load the wasm to use:

```js
// Loads the bindings and wasm as a module
const hellowasm = require('./hellowasm);
const result = hellowasm.add(100, 23);
console.log(result);
```

## Using in Node-RED custom node

From Node-RED home directory (usually ~/.node-red):

```bash
npm install [path-to-this-repo/decode-invoice]
```

Restart Node-RED and you will likely find this custom node!

![Custom Node-RED node](https://imgproxy.snort.social/FQ6qtqEYTrBFsnjo9dnzlBtb9VLBG1DG-a7S0JX_rfQ//aHR0cHM6Ly92b2lkLmNhdC9kL05BcWQ2aGQ3WmJkNmtYblJhZ0ZuSnYud2VicA)

It takes any BOLT11 invoice, and returns the amount in millisatoshis. (As a BigInt!)

## Pitfalls

If your Rust code takes i64s (rather than i32, f64, f32) your JS code will need to pass it BigInt()s. Your WASM will also return BigInt() if it returns an i64.