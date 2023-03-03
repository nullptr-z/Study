fn main() {
  // 初始化
    init();
    // 创建 create an isolate,每个窗口都是一个 isolate
    let params = CreateParams::default();
    let isolate = v8::new_isolate::new();
    // 创建 handel scope
     let handle scope=&mut handelScope::new(isolate);
     // 创建 context
      let context=v8::Context::new(handle_scope);
     // 创建 context scope
     let context_scope=&mut v8::ContextScope::new(handle_scope,context);

// javascript code
let soource=r#"
  function hello(){
    return "Hello World";
  }
  hello();
"#;

let source=v8::String::new(context_scope,source).unwrap();
let script=Script::compile(context_scope,source,None).unwrap();
let result=script.run(context_scope).unwrap();
let value:String=serde_v8::from_v8(context_scope,result).unwrap();
println!("Result is: {value}");
}

fn init() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::initialize_platform(platform);
    v8::initialize();
}
