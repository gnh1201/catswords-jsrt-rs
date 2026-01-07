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

ChakraCore must be available as a shared library. If your distribution provides ChakraCore:

* `ChakraCore.h` must be available
* `libChakraCore.so` must be linkable

Set environment variables if needed:

```bash
export CHAKRACORE_INCLUDE_DIR="/usr/include"
export CHAKRACORE_LIB_DIR="/usr/lib"
#export CHAKRACORE_INCLUDE_DIR="/path/to/ChakraCore/include"  # build from source
#export CHAKRACORE_LIB_DIR="/path/to/ChakraCore/build/lib"  # build from source
#export LD_LIBRARY_PATH="$CHAKRACORE_LIB_DIR:$LD_LIBRARY_PATH"  # build from source
```

Build:

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target\debug\multiply.exe`
direct call: 191 * 7 = 1337
global eval: multiply(191, 7) = 1337
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = js::Runtime::new()?;
    let context = js::Context::new(&runtime)?;
    let guard = context.make_current()?;
    let result = js::script::eval(&guard, "5 + 5")?;
    let value = result.to_integer(&guard)?;
    assert_eq!(value, 10);
    println!("5 + 5 = {}", value);
    Ok(())
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

type AnyResult<T> = Result<T, Box<dyn std::error::Error>>;

fn make_multiply(guard: &js::Guard) -> js::value::Function {
    js::value::Function::new(guard, Box::new(|guard, info| {
        if info.arguments.len() != 2 {
            return Err(js::err_msg(
                js::JsErrorCode::JsErrorInvalidArgument,
                format!("multiply expects 2 arguments, got {}", info.arguments.len()),
            ));
        }

        let a = info.arguments[0].to_integer(guard)?;
        let b = info.arguments[1].to_integer(guard)?;
        Ok(js::value::Number::new(guard, a * b).into())
    }))
}

fn scenario_direct_call(guard: &js::Guard, multiply: &js::value::Function) -> AnyResult<()> {
    let a: js::value::Value = js::value::Number::new(guard, 191).into();
    let b: js::value::Value = js::value::Number::new(guard, 7).into();

    let result = multiply.call(guard, &[&a, &b])?;
    let value = result.to_integer(guard)?;

    assert_eq!(value, 1337);
    println!("direct call: 191 * 7 = {}", value);
    Ok(())
}

fn scenario_global_eval(
    context: &js::Context,
    guard: &js::Guard,
    multiply: js::value::Function,
) -> AnyResult<()> {
    let fval: js::value::Value = multiply.into();
    context.set_global(guard, "multiply", &fval)?;

    let result = js::script::eval(guard, "multiply(191, 7)")?;
    let value = result.to_integer(guard)?;

    assert_eq!(value, 1337);
    println!("global eval: multiply(191, 7) = {}", value);
    Ok(())
}

fn main() -> AnyResult<()> {
    let runtime = js::Runtime::new()?;
    let context = js::Context::new(&runtime)?;
    let guard = context.make_current()?;

    let multiply = make_multiply(&guard);

    scenario_direct_call(&guard, &multiply)?;
    scenario_global_eval(&context, &guard, multiply)?;

    Ok(())
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
