const NAMES: [&str; 8] = [
    "_T0",
    "$S",
    "_$S",
    "$s",
    "_$s",
    "$e",
    "_$e",
    "@__swiftmacro_",
];
pub fn is_old_function_type_mangling(mangled: &str) -> bool {
    mangled.starts_with("_T")
}
pub fn get_prefix_length(mangled: &str) -> usize {
    if mangled.is_empty() {
        return 0;
    }
    for nam in NAMES {
        if mangled.starts_with(nam) {
            return nam.len();
        }
    }
    0
}
pub fn is_swift_symbol(mangled: &str) -> bool {
    #[cfg(feature="oldmangling")]
    if is_old_function_type_mangling(mangled) {
        return true;
    }
    get_prefix_length(mangled) != 0
}
