extern crate catswords_jsrt as js;

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

fn scenario_global_eval(guard: &js::Guard, multiply: js::value::Function) -> AnyResult<()> {
    let context = guard.context();

    let fval: js::value::Value = multiply.into();
    context.set_global("multiply", &fval)?;

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
    scenario_global_eval(&guard, multiply)?;

    Ok(())
}
