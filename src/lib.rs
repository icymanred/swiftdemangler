use std::ptr::read;
type Res<T> = Result<T,&'static str>;
pub mod reader;
pub mod demangler;
pub mod nodes;
pub fn demangle_symbol_to_string(mangled:String)  -> Res<String>{
    if !reader::prefixes::is_swift_symbol(mangled.as_str()) {
        return Err("Symbol passed in is not a swift symbol")
    }  
    Ok("unimplemented".to_string())

    
}