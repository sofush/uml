use static_files::resource_dir;

fn main() -> std::io::Result<()> {
    let mut r#static = resource_dir("./static");
    r#static.with_generated_fn("gen_static_files");
    r#static.build()?;

    let mut wasm = resource_dir("../uml-wasm/wasm");
    wasm.with_generated_fn("gen_wasm_files");
    wasm.build()
}
