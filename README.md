# chakracore-rs

[![Discord chat](https://img.shields.io/discord/359930650330923008?logo=discord)](https://discord.gg/XKG5CjtXEj?utm_source=catswords)

**chakracore-rs** provides minimal ChakraCore bindings for Rust. (Experimental)

This project aims to keep all `unsafe` code strictly inside the low-level FFI crate, while offering a small, ergonomic, and predictable API surface for embedding ChakraCore in Rust applications.

Inspired by [@darfink’s chakracore-rs implementation](https://github.com/darfink/chakracore-rs?utm_source=catswords), which is no longer maintained. This workspace is a clean reimplementation.

---

## Workspace layout

This repository is a Cargo workspace with the following crates:

* **`chakracore-sys`**

  * Raw FFI bindings to ChakraCore
  * All `unsafe` code lives here
  * Thin, mostly mechanical bindings to the C API

* **`chakracore`**

  * Safe-ish ergonomic wrapper
  * API inspired by ChakraCore samples
  * Focused on correctness and minimal abstraction

* **`chakracore-examples`**

  * Runnable example binaries
  * Used to validate real execution paths

---

## High-level API overview

The `chakracore` crate exposes a minimal, explicit API:

* `Runtime::new()`
* `Context::new(&runtime)`
* `context.make_current() -> Guard`
* `script::eval(&guard, "...")`
* `value::Function::new(&guard, closure)`
* `Function::call(&guard, &[&Value])`

The `Guard` type enforces context lifetime and helps prevent common misuse patterns.

---

## Requirements (Windows)

You need ChakraCore headers and libraries available on your system.

### Required files

* `ChakraCore.h` (header)
* `ChakraCore.lib` (import library)
* `ChakraCore.dll` (runtime)

### Environment variables

Set these so `chakracore-sys` can locate ChakraCore:

```powershell
$env:CHAKRACORE_INCLUDE_DIR="C:\path\to\ChakraCore\include"
$env:CHAKRACORE_LIB_DIR="C:\path\to\ChakraCore\lib"
```

### Build

```powershell
cargo build
```

### Runtime notes

At runtime, `ChakraCore.dll` must be discoverable:

* Place it next to the produced `.exe`, **or**
* Add its directory to `PATH`

---

## Requirements (Linux)

ChakraCore must be available as a shared library.

### Option A: Use a system-installed ChakraCore

If your distribution provides ChakraCore:

* `ChakraCore.h` must be available
* `libChakraCore.so` must be linkable

Set environment variables if needed:

```bash
export CHAKRACORE_INCLUDE_DIR="/usr/include"
export CHAKRACORE_LIB_DIR="/usr/lib"
```

Build:

```bash
cargo build
```

If the library is not in a default loader path, set:

```bash
export LD_LIBRARY_PATH="$CHAKRACORE_LIB_DIR:$LD_LIBRARY_PATH"
```

---

### Option B: Build ChakraCore from source

If ChakraCore is not available via your package manager:

1. Build ChakraCore from source

2. Locate:

   * `ChakraCore.h`
   * `libChakraCore.so`

3. Export paths:

```bash
export CHAKRACORE_INCLUDE_DIR="/path/to/ChakraCore/include"
export CHAKRACORE_LIB_DIR="/path/to/ChakraCore/build/lib"
```

4. Ensure runtime loader can find the library:

```bash
export LD_LIBRARY_PATH="$CHAKRACORE_LIB_DIR:$LD_LIBRARY_PATH"
```

5. Build this workspace:

```bash
cargo build
```

---

## Running the examples

All runnable examples live in the **`chakracore-examples`** crate and are built as binaries.

### Example binaries

* `hello_world`
* `multiply`

### Example console messages

```text
> cargo run -p chakracore-examples --bin multiply
cargo run -p chakracore-examples --bin multiply
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target\debug\multiply.exe`
191 * 7 = 1337
multiply(191, 7) = 1337
```

---

### Windows

```powershell
cargo run -p chakracore-examples --bin hello_world
cargo run -p chakracore-examples --bin multiply
```

If you encounter a runtime error related to missing `ChakraCore.dll`, ensure it is either:

* In the same directory as the executable, or
* In a directory listed in `PATH`

---

### Linux

If `libChakraCore.so` is not in a default loader path:

```bash
export LD_LIBRARY_PATH="$CHAKRACORE_LIB_DIR:$LD_LIBRARY_PATH"
```

Then run:

```bash
cargo run -p chakracore-examples --bin hello_world
cargo run -p chakracore-examples --bin multiply
```

---

## Example: Hello World

**Binary:** `chakracore-examples --bin hello_world`

```rust
extern crate chakracore as js;

fn main() {
    let runtime = js::Runtime::new().unwrap();
    let context = js::Context::new(&runtime).unwrap();
    let guard = context.make_current().unwrap();

    let result = js::script::eval(&guard, "5 + 5").unwrap();
    assert_eq!(result.to_integer(&guard).unwrap(), 10);
}
```

Run:

```bash
cargo run -p chakracore-examples --bin hello_world
```

---

## Example: Function – Multiply

**Binary:** `chakracore-examples --bin multiply`

```rust
extern crate chakracore as js;

fn main() {
    let runtime = js::Runtime::new().unwrap();
    let context = js::Context::new(&runtime).unwrap();
    let guard = context.make_current().unwrap();

    let multiply = js::value::Function::new(&guard, Box::new(|guard, info| {
        let result =
            info.arguments[0].to_integer(guard).unwrap()
            * info.arguments[1].to_integer(guard).unwrap();

        Ok(js::value::Number::new(guard, result).into())
    }));

    let a: js::value::Value = js::value::Number::new(&guard, 191).into();
    let b: js::value::Value = js::value::Number::new(&guard, 7).into();

    let result = multiply.call(&guard, &[&a, &b]).unwrap();
    assert_eq!(result.to_integer(&guard).unwrap(), 1337);
}
```

Run:

```bash
cargo run -p chakracore-examples --bin multiply
```

---

## Join the community
I am always open. Collaboration, opportunities, and community activities are all welcome.

* ActivityPub [@catswords_oss@catswords.social](https://catswords.social/@catswords_oss?utm_source=catswords)
* XMPP [catswords@conference.omemo.id](xmpp:catswords@conference.omemo.id?join)
* [Join Catswords OSS on Microsoft Teams (teams.live.com)](https://teams.live.com/l/community/FEACHncAhq8ldnojAI?utm_source=catswords)
* [Join Catswords OSS #chakracore-rs on Discord (discord.gg)](https://discord.gg/jVvhB8N7tb?utm_source=catswords)
