use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "hello.wat")?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    let answer = instance
        .get_func(&mut store, "answer")
        .expect("`answer` was not an exported function");
    let answer = answer.typed::<(), i32, _>(&store)?;
    let result = answer.call(&mut store, ())?;
    println!("Answer: {:?}", result);
    Ok(())
}
