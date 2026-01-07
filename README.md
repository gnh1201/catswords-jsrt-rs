# chakracore-rs

[![Discord chat](https://img.shields.io/discord/359930650330923008?logo=discord)](https://discord.gg/XKG5CjtXEj?utm_source=catswords)

chakracore-rs provides minimal ChakraCore bindings for Rust. (Experimental)

Inspired by [@darfink’s chakracore-rs implementation](https://github.com/darfink/chakracore-rs?utm_source=catswords). However, it is outdated and no longer maintained, so we rewrote the code.

This workspace contains:

- `chakracore-sys`: raw FFI bindings to ChakraCore (all `unsafe` stays here)
- `chakracore`: safe-ish ergonomic wrapper matching the sample API:
  - `Runtime::new()`
  - `Context::new(&runtime)`
  - `context.make_current() -> Guard`
  - `script::eval(&guard, "...")`
  - `value::Function::new(&guard, closure)`
  - `Function::call(&guard, &[&Value])`

## Requirements (Windows)

You need ChakraCore headers and import library:

- `ChakraCore.h` in an include directory
- `ChakraCore.lib` in a library directory
- `ChakraCore.dll` available at runtime (PATH or next to your exe)

Set environment variables:

PowerShell:

```powershell
$env:CHAKRACORE_INCLUDE_DIR="C:\path\to\ChakraCore\include"
$env:CHAKRACORE_LIB_DIR="C:\path\to\ChakraCore\lib"
```

Build:

```powershell
cargo build -p chakracore
```

## Example: Hello World

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

## Example: Function - Multiply

```rust
extern crate chakracore as js;

fn main() {
  let runtime = js::Runtime::new().unwrap();
  let context = js::Context::new(&runtime).unwrap();
  let guard = context.make_current().unwrap();

  let multiply = js::value::Function::new(&guard, Box::new(|guard, info| {
      let result = info.arguments[0].to_integer(guard).unwrap()
                 * info.arguments[1].to_integer(guard).unwrap();
      Ok(js::value::Number::new(guard, result).into())
  }));

  let a: js::value::Value = js::value::Number::new(&guard, 191).into();
  let b: js::value::Value = js::value::Number::new(&guard, 7).into();

  let result = multiply.call(&guard, &[&a, &b]).unwrap();
  assert_eq!(result.to_integer(&guard).unwrap(), 1337);
}
```

## Join the community
I am always open. Collaboration, opportunities, and community activities are all welcome.

* ActivityPub [@catswords_oss@catswords.social](https://catswords.social/@catswords_oss?utm_source=catswords)
* XMPP [catswords@conference.omemo.id](xmpp:catswords@conference.omemo.id?join)
* [Join Catswords OSS on Microsoft Teams (teams.live.com)](https://teams.live.com/l/community/FEACHncAhq8ldnojAI?utm_source=catswords)
* [Join Catswords OSS #chakracore-rs on Discord (discord.gg)](https://discord.gg/jVvhB8N7tb?utm_source=catswords)
