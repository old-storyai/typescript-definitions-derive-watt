static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("typescript-definitions-derive.wasm");
#[proc_macro_derive(TypeScriptDefinition, attributes(ts))]
pub fn derive_typescript_definition(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(derive_typescript_definition), input)
}
#[proc_macro_derive(TypeScriptify, attributes(ts))]
pub fn derive_type_script_ify(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(derive_type_script_ify), input)
}
