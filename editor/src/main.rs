use std::default::Default;

struct Foo {
    bar: String,
}

fn main() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(isolate);

    let context = v8::Context::new(scope, Default::default());
    let res = context.set_slot(Foo { bar: "blah".to_string() });
    dbg!(res);

    let scope = &mut v8::ContextScope::new(scope, context);

    let code = v8::String::new(scope, "foo()").unwrap();

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap().to_rust_string_lossy(scope);
    dbg!(result);
}
