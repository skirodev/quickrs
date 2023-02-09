mod hand_written;

use hand_written::NativeModule;

rquickjs::module_init!(NativeModule);

fn print(msg: String) {
    println!("{}", msg);
}

pub fn main() {
    use rquickjs::{
        BuiltinResolver, Context, FileResolver, Func, ModuleLoader, Runtime, ScriptLoader,
    };
    let resolver = (
        BuiltinResolver::default()
            .with_module("bundle/script_module")
            .with_module("bundle/native_module"),
        FileResolver::default()
            .with_path("./")
            .with_path("../../target/debug")
            .with_native(),
    );
    let loader = (
        //BuiltinLoader::default().with_module("bundle/script_module", SCRIPT_MODULE),
        ModuleLoader::default().with_module("bundle/native_module", NativeModule),
        ScriptLoader::default(),
    );
    let rt = Runtime::new().unwrap();
    let ctx = Context::full(&rt).unwrap();
    rt.set_loader(resolver, loader);
    ctx.with(|ctx| {
        let global = ctx.globals();
        global.set("print", Func::new("print", print)).unwrap();

        println!("Hello from js engine inside wasm.");
        match ctx.compile(
            "test",
            r#"
            import { n, s, f } from "bundle/native_module";
print(`n = ${n}`);
print(`s = "${s}"`);

"#,
        ) {
            Ok(_) => println!("Compiled with no errors"),
            Err(err) => println!("ERROR: {}", err),
        }
    });
}