extern crate chakracore as js;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = js::Runtime::new()?;
    let context = js::Context::new(&runtime)?;
    let guard = context.make_current()?;

    // Create a JS function backed by a Rust closure
    let multiply = js::value::Function::new(&runtime, &guard, Box::new(|guard, info| {
        if info.arguments.len() != 2 {
            return Err(js::err_msg(
                js::JsErrorCode::JsErrorInvalidArgument,
                format!("multiply expects 2 arguments, got {}", info.arguments.len()),
            ));
        }

        let a = info.arguments[0].to_integer(guard)?;
        let b = info.arguments[1].to_integer(guard)?;
        let result = a * b;

        Ok(js::value::Number::new(guard, result).into())
    }));

    let a: js::value::Value = js::value::Number::new(&guard, 191).into();
    let b: js::value::Value = js::value::Number::new(&guard, 7).into();

    let result = multiply.call(&guard, &[&a, &b])?;
    let value = result.to_integer(&guard)?;

    assert_eq!(value, 1337);
    println!("191 * 7 = {}", value);
	
    // Register as global function: global.multiply = <function>
    let fval: js::value::Value = multiply.into();
    context.set_global(&guard, "multiply", &fval)?;

    // Now JS code can call it
    let result2 = js::script::eval(&guard, "multiply(191, 7)")?;
    let value = result2.to_integer(&guard)?;

    assert_eq!(value, 1337);
    println!("multiply(191, 7) = {}", value);

    Ok(())
}
