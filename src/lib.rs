use tera::Number;

pub fn register_f64_math_functions(tera: &mut tera::Tera) {
    tera.register_function("neg", f64_functions(|x| -x));
    tera.register_function("abs", f64_functions(|x| x.abs()));
    tera.register_function("sqrt", f64_functions(|x| x.sqrt()));
    tera.register_function("floor", f64_functions(|x| x.floor()));
    tera.register_function("ceil", f64_functions(|x| x.ceil()));
    tera.register_function("round", f64_functions(|x| x.round()));
    tera.register_function("trunc", f64_functions(|x| x.trunc()));
    tera.register_function("fract", f64_functions(|x| x.fract()));
    tera.register_function("recip", f64_functions(|x| x.recip()));
    tera.register_function("to_deg", f64_functions(|x| x.to_degrees()));
    tera.register_function("to_rad", f64_functions(|x| x.to_radians()));
    tera.register_function("sin", f64_functions(|x| x.sin()));
    tera.register_function("cos", f64_functions(|x| x.cos()));
    tera.register_function("tan", f64_functions(|x| x.tan()));
    tera.register_function("asin", f64_functions(|x| x.asin()));
    tera.register_function("acos", f64_functions(|x| x.acos()));
    tera.register_function("atan", f64_functions(|x| x.atan()));
    tera.register_function("sinh", f64_functions(|x| x.sinh()));
    tera.register_function("cosh", f64_functions(|x| x.cosh()));
    tera.register_function("tanh", f64_functions(|x| x.tanh()));
}

pub fn register_f64_math_filters(tera: &mut tera::Tera) {
    tera.register_filter("neg", f64_filters(|x| -x));
    tera.register_filter("abs", f64_filters(|x| x.abs()));
    tera.register_filter("sqrt", f64_filters(|x| x.sqrt()));
    tera.register_filter("floor", f64_filters(|x| x.floor()));
    tera.register_filter("ceil", f64_filters(|x| x.ceil()));
    tera.register_filter("round", f64_filters(|x| x.round()));
    tera.register_filter("trunc", f64_filters(|x| x.trunc()));
    tera.register_filter("fract", f64_filters(|x| x.fract()));
    tera.register_filter("recip", f64_filters(|x| x.recip()));
    tera.register_filter("to_deg", f64_filters(|x| x.to_degrees()));
    tera.register_filter("to_rad", f64_filters(|x| x.to_radians()));
    tera.register_filter("sin", f64_filters(|x| x.sin()));
    tera.register_filter("cos", f64_filters(|x| x.cos()));
    tera.register_filter("tan", f64_filters(|x| x.tan()));
    tera.register_filter("asin", f64_filters(|x| x.asin()));
    tera.register_filter("acos", f64_filters(|x| x.acos()));
    tera.register_filter("atan", f64_filters(|x| x.atan()));
    tera.register_filter("sinh", f64_filters(|x| x.sinh()));
    tera.register_filter("cosh", f64_filters(|x| x.cosh()));
    tera.register_filter("tanh", f64_filters(|x| x.tanh()));
}

pub fn f64_functions<F: Fn(f64) -> f64>(f: F) -> impl tera::Function {
    Box::new(
        |args: &std::collections::HashMap<String, tera::Value>| -> tera::Result<tera::Value> {
            let num = Number::from_f64(
                args.get("num")
                    .and_then(|v| match v {
                        tera::Value::Number(n) => n.as_f64(),
                        _ => None,
                    })
                    .ok_or("Not a number")?,
            )
            .ok_or("Not a number")?;
            Ok(tera::Value::Number(num))
        },
    )
}

pub fn f64_filters<F: Fn(f64) -> f64 + Send + Sync>(f: F) -> impl tera::Filter {
    Box::new(
        move |value: &tera::Value,
              _: &std::collections::HashMap<String, tera::Value>|
              -> tera::Result<tera::Value> {
            match value {
                tera::Value::Number(n) => {
                    let num = Number::from_f64(f(n.as_f64().unwrap())).unwrap();
                    Ok(tera::Value::Number(num))
                }
                _ => Err("Not a number".into()),
            }
        },
    )
}
